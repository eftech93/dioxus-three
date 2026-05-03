//! Web/WASM implementation of ThreeView using canvas element
//!
//! Enhanced with Phase 1 features: Raycasting, Selection, and Transform Gizmos

use crate::ThreeViewProps;
use crate::input::{PointerEvent, PointerDragEvent, HitInfo, Vector2, Vector3, MouseButton};
use crate::EntityId;
use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

// Global callback storage
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static CALLBACK_STORAGE: RefCell<HashMap<String, CallbackStorage>> = RefCell::new(HashMap::new());
}

/// Stores callbacks for a specific canvas instance
struct CallbackStorage {
    on_pointer_down: Option<Callback<PointerEvent>>,
    on_pointer_up: Option<Callback<PointerEvent>>,
    on_pointer_move: Option<Callback<PointerEvent>>,
    on_pointer_drag: Option<Callback<PointerDragEvent>>,
    on_gizmo_drag: Option<Callback<crate::GizmoEvent>>,
}

impl CallbackStorage {
    fn new() -> Self {
        Self {
            on_pointer_down: None,
            on_pointer_up: None,
            on_pointer_move: None,
            on_pointer_drag: None,
            on_gizmo_drag: None,
        }
    }
}

/// Register callbacks for a canvas instance
fn register_callbacks(
    canvas_id: String,
    on_pointer_down: Option<Callback<PointerEvent>>,
    on_pointer_up: Option<Callback<PointerEvent>>,
    on_pointer_move: Option<Callback<PointerEvent>>,
    on_pointer_drag: Option<Callback<PointerDragEvent>>,
    on_gizmo_drag: Option<Callback<crate::GizmoEvent>>,
) {
    CALLBACK_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let callbacks = storage.entry(canvas_id.clone()).or_insert_with(CallbackStorage::new);
        callbacks.on_pointer_down = on_pointer_down;
        callbacks.on_pointer_up = on_pointer_up;
        callbacks.on_pointer_move = on_pointer_move;
        callbacks.on_pointer_drag = on_pointer_drag;
        callbacks.on_gizmo_drag = on_gizmo_drag;
    });
    
    // Set up JavaScript bridge
    setup_js_bridge();
    
    let js_code = r#"
        (function() {
            if (!window.dioxusThreeBridge) {
                window.dioxusThreeBridge = {
                    onPointerDown: function(canvasId, eventJson) {
                        if (window.dioxusThreeRustBridge && window.dioxusThreeRustBridge.on_pointer_down) {
                            window.dioxusThreeRustBridge.on_pointer_down(canvasId, eventJson);
                        }
                    },
                    onPointerUp: function(canvasId, eventJson) {
                        if (window.dioxusThreeRustBridge && window.dioxusThreeRustBridge.on_pointer_up) {
                            window.dioxusThreeRustBridge.on_pointer_up(canvasId, eventJson);
                        }
                    },
                    onPointerMove: function(canvasId, eventJson) {
                        if (window.dioxusThreeRustBridge && window.dioxusThreeRustBridge.on_pointer_move) {
                            window.dioxusThreeRustBridge.on_pointer_move(canvasId, eventJson);
                        }
                    },
                    onPointerDrag: function(canvasId, eventJson) {
                        if (window.dioxusThreeRustBridge && window.dioxusThreeRustBridge.on_pointer_drag) {
                            window.dioxusThreeRustBridge.on_pointer_drag(canvasId, eventJson);
                        }
                    },
                    onGizmoDrag: function(canvasId, eventJson) {
                        if (window.dioxusThreeRustBridge && window.dioxusThreeRustBridge.on_gizmo_drag) {
                            window.dioxusThreeRustBridge.on_gizmo_drag(canvasId, eventJson);
                        }
                    }
                };
            }
        })();
    "#;
    
    let _ = js_sys::eval(js_code);
}

/// Set up the JS -> Rust bridge using wasm_bindgen closures
fn setup_js_bridge() {
    let on_pointer_down = Closure::wrap(Box::new(|canvas_id: JsValue, event_json: JsValue| {
        if let (Some(id), Some(json)) = (canvas_id.as_string(), event_json.as_string()) {
            invoke_pointer_down(&id, &json);
        }
    }) as Box<dyn FnMut(JsValue, JsValue)>);
    
    let on_pointer_up = Closure::wrap(Box::new(|canvas_id: JsValue, event_json: JsValue| {
        if let (Some(id), Some(json)) = (canvas_id.as_string(), event_json.as_string()) {
            invoke_pointer_up(&id, &json);
        }
    }) as Box<dyn FnMut(JsValue, JsValue)>);
    
    let on_pointer_move = Closure::wrap(Box::new(|canvas_id: JsValue, event_json: JsValue| {
        if let (Some(id), Some(json)) = (canvas_id.as_string(), event_json.as_string()) {
            invoke_pointer_move(&id, &json);
        }
    }) as Box<dyn FnMut(JsValue, JsValue)>);
    
    let on_pointer_drag = Closure::wrap(Box::new(|canvas_id: JsValue, event_json: JsValue| {
        if let (Some(id), Some(json)) = (canvas_id.as_string(), event_json.as_string()) {
            invoke_pointer_drag(&id, &json);
        }
    }) as Box<dyn FnMut(JsValue, JsValue)>);
    
    let on_gizmo_drag = Closure::wrap(Box::new(|canvas_id: JsValue, event_json: JsValue| {
        if let (Some(id), Some(json)) = (canvas_id.as_string(), event_json.as_string()) {
            invoke_gizmo_drag(&id, &json);
        }
    }) as Box<dyn FnMut(JsValue, JsValue)>);
    
    let bridge = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&bridge, &"on_pointer_down".into(), on_pointer_down.as_ref().unchecked_ref());
    let _ = js_sys::Reflect::set(&bridge, &"on_pointer_up".into(), on_pointer_up.as_ref().unchecked_ref());
    let _ = js_sys::Reflect::set(&bridge, &"on_pointer_move".into(), on_pointer_move.as_ref().unchecked_ref());
    let _ = js_sys::Reflect::set(&bridge, &"on_pointer_drag".into(), on_pointer_drag.as_ref().unchecked_ref());
    let _ = js_sys::Reflect::set(&bridge, &"on_gizmo_drag".into(), on_gizmo_drag.as_ref().unchecked_ref());
    
    if let Some(window) = web_sys::window() {
        let _ = js_sys::Reflect::set(&window, &"dioxusThreeRustBridge".into(), &bridge);
    }
    
    on_pointer_down.forget();
    on_pointer_up.forget();
    on_pointer_move.forget();
    on_pointer_drag.forget();
    on_gizmo_drag.forget();
}

/// Invoke a pointer down callback
fn invoke_pointer_down(canvas_id: &str, event_json: &str) {
    if let Some(event) = parse_pointer_event(event_json) {
        CALLBACK_STORAGE.with(|storage| {
            if let Some(callbacks) = storage.borrow().get(canvas_id) {
                if let Some(cb) = &callbacks.on_pointer_down {
                    cb.call(event);
                }
            }
        });
    }
}

/// Invoke a pointer up callback
fn invoke_pointer_up(canvas_id: &str, event_json: &str) {
    if let Some(event) = parse_pointer_event(event_json) {
        CALLBACK_STORAGE.with(|storage| {
            if let Some(callbacks) = storage.borrow().get(canvas_id) {
                if let Some(cb) = &callbacks.on_pointer_up {
                    cb.call(event);
                }
            }
        });
    }
}

/// Invoke a pointer move callback
fn invoke_pointer_move(canvas_id: &str, event_json: &str) {
    if let Some(event) = parse_pointer_event(event_json) {
        CALLBACK_STORAGE.with(|storage| {
            if let Some(callbacks) = storage.borrow().get(canvas_id) {
                if let Some(cb) = &callbacks.on_pointer_move {
                    cb.call(event);
                }
            }
        });
    }
}

/// Invoke a pointer drag callback
fn invoke_pointer_drag(canvas_id: &str, event_json: &str) {
    if let Some(event) = parse_drag_event(event_json) {
        CALLBACK_STORAGE.with(|storage| {
            if let Some(callbacks) = storage.borrow().get(canvas_id) {
                if let Some(cb) = &callbacks.on_pointer_drag {
                    cb.call(event);
                }
            }
        });
    }
}

fn get_f32_from_js(obj: &js_sys::Object, key: &str) -> Option<f32> {
    let val = js_sys::Reflect::get(obj, &key.into()).ok()?;
    val.as_f64().map(|v| v as f32)
}

fn get_bool_from_js(obj: &js_sys::Object, key: &str) -> bool {
    js_sys::Reflect::get(obj, &key.into())
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

fn get_string_from_js(obj: &js_sys::Object, key: &str) -> Option<String> {
    js_sys::Reflect::get(obj, &key.into())
        .ok()
        .and_then(|v| v.as_string())
}

fn get_vec2_from_js(obj: &js_sys::Object, key: &str) -> Option<Vector2> {
    let obj = js_sys::Reflect::get(obj, &key.into()).ok()?;
    let inner = obj.dyn_ref::<js_sys::Object>()?;
    let x = get_f32_from_js(inner, "x")?;
    let y = get_f32_from_js(inner, "y")?;
    Some(Vector2::new(x, y))
}

fn get_vec3_from_js(obj: &js_sys::Object, key: &str) -> Option<Vector3> {
    let obj = js_sys::Reflect::get(obj, &key.into()).ok()?;
    let inner = obj.dyn_ref::<js_sys::Object>()?;
    let x = get_f32_from_js(inner, "x")?;
    let y = get_f32_from_js(inner, "y")?;
    let z = get_f32_from_js(inner, "z")?;
    Some(Vector3::new(x, y, z))
}

fn parse_hit_info(json_val: &js_sys::Object) -> Option<HitInfo> {
    let hit = js_sys::Reflect::get(json_val, &"hit".into()).ok()?;
    if hit.is_null() || hit.is_undefined() {
        return None;
    }
    let hit_obj = hit.dyn_ref::<js_sys::Object>()?;
    let entity_id = get_f32_from_js(hit_obj, "entityId")? as usize;
    let point = get_vec3_from_js(hit_obj, "point")?;
    let normal = get_vec3_from_js(hit_obj, "normal")?;
    let uv = get_vec2_from_js(hit_obj, "uv");
    let distance = get_f32_from_js(hit_obj, "distance")?;
    let face_index = js_sys::Reflect::get(hit_obj, &"faceIndex".into()).ok().and_then(|v| v.as_f64()).map(|v| v as usize);
    let instance_id = js_sys::Reflect::get(hit_obj, &"instanceId".into()).ok().and_then(|v| v.as_f64()).map(|v| v as usize);
    
    Some(HitInfo {
        entity_id: EntityId(entity_id),
        point,
        normal,
        uv,
        distance,
        face_index,
        instance_id,
    })
}

/// Parse a JSON pointer event
fn parse_pointer_event(json: &str) -> Option<PointerEvent> {
    let json_val = js_sys::JSON::parse(json).ok()?;
    let json_obj = json_val.dyn_ref::<js_sys::Object>()?;
    
    Some(PointerEvent {
        hit: parse_hit_info(json_obj),
        screen_position: get_vec2_from_js(json_obj, "screenPosition").unwrap_or(Vector2::new(0.0, 0.0)),
        ndc_position: get_vec2_from_js(json_obj, "ndcPosition").unwrap_or(Vector2::new(0.0, 0.0)),
        button: get_string_from_js(json_obj, "button").and_then(|s| match s.as_str() {
            "Left" => Some(MouseButton::Left),
            "Right" => Some(MouseButton::Right),
            "Middle" => Some(MouseButton::Middle),
            _ => None,
        }),
        shift_key: get_bool_from_js(json_obj, "shiftKey"),
        ctrl_key: get_bool_from_js(json_obj, "ctrlKey"),
        alt_key: get_bool_from_js(json_obj, "altKey"),
    })
}

/// Parse a JSON drag event
fn parse_drag_event(json: &str) -> Option<PointerDragEvent> {
    let json_val = js_sys::JSON::parse(json).ok()?;
    let json_obj = json_val.dyn_ref::<js_sys::Object>()?;
    
    let hit = parse_hit_info(json_obj);
    let screen_position = get_vec2_from_js(json_obj, "screenPosition").unwrap_or(Vector2::new(0.0, 0.0));
    let delta = get_vec2_from_js(json_obj, "delta").unwrap_or(Vector2::new(0.0, 0.0));
    
    Some(PointerDragEvent {
        hit,
        start_hit: None,
        screen_position,
        start_screen_position: Vector2::new(0.0, 0.0),
        world_position: Vector3::ZERO,
        start_world_position: Vector3::ZERO,
        delta,
        total_delta: delta,
        button: MouseButton::Left,
    })
}

/// Parse a JSON gizmo event
fn parse_gizmo_event(json: &str) -> Option<crate::GizmoEvent> {
    let json_val = js_sys::JSON::parse(json).ok()?;
    let json_obj = json_val.dyn_ref::<js_sys::Object>()?;
    
    let target = get_f32_from_js(json_obj, "target")? as usize;
    let mode = get_string_from_js(json_obj, "mode").map(|s| match s.to_lowercase().as_str() {
        "translate" => crate::GizmoMode::Translate,
        "rotate" => crate::GizmoMode::Rotate,
        "scale" => crate::GizmoMode::Scale,
        _ => crate::GizmoMode::Translate,
    }).unwrap_or(crate::GizmoMode::Translate);
    
    let space = get_string_from_js(json_obj, "space").map(|s| match s.to_lowercase().as_str() {
        "local" => crate::GizmoSpace::Local,
        _ => crate::GizmoSpace::World,
    }).unwrap_or(crate::GizmoSpace::World);
    
    let transform_obj = js_sys::Reflect::get(json_obj, &"transform".into()).ok()?.dyn_ref::<js_sys::Object>()?.clone();
    let position = get_vec3_from_js(&transform_obj, "position").unwrap_or(Vector3::ZERO);
    let rotation = get_vec3_from_js(&transform_obj, "rotation").unwrap_or(Vector3::ZERO);
    let scale = get_vec3_from_js(&transform_obj, "scale").unwrap_or(Vector3::new(1.0, 1.0, 1.0));
    let is_finished = get_bool_from_js(json_obj, "isFinished");
    
    Some(crate::GizmoEvent {
        target: EntityId(target),
        mode,
        space,
        transform: crate::GizmoTransform { position, rotation, scale },
        is_finished,
    })
}

/// Invoke a gizmo drag callback
fn invoke_gizmo_drag(canvas_id: &str, event_json: &str) {
    if let Some(event) = parse_gizmo_event(event_json) {
        CALLBACK_STORAGE.with(|storage| {
            if let Some(callbacks) = storage.borrow().get(canvas_id) {
                if let Some(cb) = &callbacks.on_gizmo_drag {
                    cb.call(event);
                }
            }
        });
    }
}

/// A Three.js 3D viewer component for Dioxus Web
///
/// Uses a native canvas element with Three.js injected via script tags.
/// Supports raycasting, selection, and transform gizmos (Phase 1).
#[component]
pub fn ThreeView(props: ThreeViewProps) -> Element {
    let canvas_id = use_signal(|| format!("three-canvas-{}", js_sys::Math::random()));
    let class = props.class.clone();
    let _view_id = props.id.clone().unwrap_or_else(|| canvas_id());

    // Store props in individual signals so we can track changes
    let mut cam_x = use_signal(|| props.cam_x);
    let mut cam_y = use_signal(|| props.cam_y);
    let mut cam_z = use_signal(|| props.cam_z);
    let mut target_x = use_signal(|| props.target_x);
    let mut target_y = use_signal(|| props.target_y);
    let mut target_z = use_signal(|| props.target_z);
    let mut auto_rotate = use_signal(|| props.auto_rotate);
    let mut rot_speed = use_signal(|| props.rot_speed);
    let mut rot_x = use_signal(|| props.rot_x);
    let mut rot_y = use_signal(|| props.rot_y);
    let mut rot_z = use_signal(|| props.rot_z);
    let mut scale = use_signal(|| props.scale);
    let mut color = use_signal(|| props.color.clone());
    let mut background = use_signal(|| props.background.clone());
    let mut show_grid = use_signal(|| props.show_grid);
    let mut show_axes = use_signal(|| props.show_axes);
    let mut wireframe = use_signal(|| props.wireframe);
    let mut models = use_signal(|| props.models.clone());
    let mut prev_models = use_signal(|| props.models.clone());
    
    // Phase 1: Selection and gizmo state
    let mut selection = use_signal(|| props.selection.clone());
    let mut gizmo = use_signal(|| props.gizmo.clone());
    let mut selection_style = use_signal(|| props.selection_style.clone());
    
    // Store callbacks
    let canvas_id_for_callbacks = canvas_id();
    let _ = use_hook(|| {
        register_callbacks(
            canvas_id_for_callbacks.clone(),
            props.on_pointer_down.clone(),
            props.on_pointer_up.clone(),
            props.on_pointer_move.clone(),
            props.on_pointer_drag.clone(),
            props.on_gizmo_drag.clone(),
        );
    });

    // Update signals when props change
    use_effect(use_reactive((&props,), move |(new_props,)| {
        cam_x.set(new_props.cam_x);
        cam_y.set(new_props.cam_y);
        cam_z.set(new_props.cam_z);
        target_x.set(new_props.target_x);
        target_y.set(new_props.target_y);
        target_z.set(new_props.target_z);
        auto_rotate.set(new_props.auto_rotate);
        rot_speed.set(new_props.rot_speed);
        rot_x.set(new_props.rot_x);
        rot_y.set(new_props.rot_y);
        rot_z.set(new_props.rot_z);
        scale.set(new_props.scale);
        color.set(new_props.color.clone());
        background.set(new_props.background.clone());
        show_grid.set(new_props.show_grid);
        show_axes.set(new_props.show_axes);
        wireframe.set(new_props.wireframe);
        // Only update models signal if structurally changed (avoids reload during gizmo drag)
        let current_models = prev_models();
        if new_props.models != current_models {
            models.set(new_props.models.clone());
            prev_models.set(new_props.models.clone());
        }
        
        selection.set(new_props.selection.clone());
        gizmo.set(new_props.gizmo.clone());
        selection_style.set(new_props.selection_style.clone());
    }));

    // Effect that runs when models change - reload the scene models
    use_effect(move || {
        let id = canvas_id();
        let mds = models();

        web_sys::console::log_1(&format!("ThreeView models changed: {} models", mds.len()).into());

        wasm_bindgen_futures::spawn_local(async move {
            load_required_loaders(&mds).await;
            wait_ms(500).await;

            let model_js = build_models_js(&mds);

            let js_code = format!(
                r#"
                (function() {{
                    const canvas = document.getElementById('{}');
                    if (!canvas || !canvas.dioxusThreeState) {{
                        console.log('Canvas or state not found for model update');
                        return;
                    }}
                    
                    const {{ scene, modelContainer, state }} = canvas.dioxusThreeState;
                    const THREE = window.THREE;
                    
                    // Clear existing models
                    while(modelContainer.children.length > 0){{
                        modelContainer.remove(modelContainer.children[0]);
                    }}
                    
                    // Reset entity counter and map
                    canvas.dioxusThreeState.entityCounter = 0;
                    canvas.dioxusThreeState.entityMap = new Map();
                    
                    // Load new models
                    {}
                    
                    console.log('Models reloaded: {} models');
                }})();
                "#,
                id,
                model_js,
                mds.len()
            );

            if let Err(e) = js_sys::eval(&js_code) {
                web_sys::console::error_1(&format!("Failed to update models: {:?}", e).into());
            }
        });
    });

    // Effect that runs when any of the transform/camera values change
    use_effect(move || {
        let id = canvas_id();
        let cx = cam_x();
        let cy = cam_y();
        let cz = cam_z();
        let tx = target_x();
        let ty = target_y();
        let tz = target_z();
        let ar = auto_rotate();
        let rs = rot_speed();
        let rx = rot_x();
        let ry = rot_y();
        let rz = rot_z();
        let sc = scale();
        let col = color();
        let bg = background();
        let sg = show_grid();
        let sa = show_axes();
        let wf = wireframe();
        let sel = selection();
        let gz = gizmo();
        let ss = selection_style();

        let rot_x_rad = rx.to_radians();
        let rot_y_rad = ry.to_radians();
        let rot_z_rad = rz.to_radians();

        let selection_json = match sel {
            Some(s) => {
                let ids: Vec<String> = s.iter().map(|id| id.0.to_string()).collect();
                format!("[{}]", ids.join(","))
            }
            None => "[]".to_string(),
        };

        let gizmo_json = match gz {
            Some(g) => format!(
                r#"{{"target": {}, "mode": "{:?}", "space": "{:?}", "size": {}}}"#,
                g.target.0,
                g.mode,
                g.space,
                g.size
            ),
            None => "null".to_string(),
        };

        let selection_color = ss.outline_color.clone();

        let js_code = format!(
            r#"
            (function() {{
                const canvas = document.getElementById('{}');
                if (!canvas || !canvas.dioxusThreeState) {{
                    console.log('Canvas or state not found');
                    return;
                }}
                
                const {{ state, camera }} = canvas.dioxusThreeState;
                
                state.camX = {};
                state.camY = {};
                state.camZ = {};
                state.targetX = {};
                state.targetY = {};
                state.targetZ = {};
                state.autoRotate = {};
                state.rotSpeed = {};
                state.rotX = {};
                state.rotY = {};
                state.rotZ = {};
                state.scale = {};
                state.color = '{}';
                state.background = '{}';
                state.showGrid = {};
                state.showAxes = {};
                state.wireframe = {};
                state.selection = {};
                state.gizmo = {};
                state.selectionColor = '{}';
                
                // Update camera position immediately
                if (camera) {{
                    camera.position.set(state.camX, state.camY, state.camZ);
                    camera.lookAt(state.targetX, state.targetY, state.targetZ);
                }}
                
                // Update selection visualization
                if (window.updateSelectionVisualization) {{
                    window.updateSelectionVisualization(canvas, state.selection, state.selectionColor);
                }}
                
                // Update gizmo
                if (window.updateGizmo) {{
                    window.updateGizmo(canvas, state.gizmo);
                }}
                
                console.log('JS state updated:', {{ camX: state.camX, camY: state.camY, camZ: state.camZ }});
            }})();
            "#,
            id,
            cx,
            cy,
            cz,
            tx,
            ty,
            tz,
            ar.to_string().to_lowercase(),
            rs,
            rot_x_rad,
            rot_y_rad,
            rot_z_rad,
            sc,
            col,
            bg,
            sg.to_string().to_lowercase(),
            sa.to_string().to_lowercase(),
            wf.to_string().to_lowercase(),
            selection_json,
            gizmo_json,
            selection_color
        );

        if let Err(e) = js_sys::eval(&js_code) {
            web_sys::console::error_1(&format!("Failed to update scene: {:?}", e).into());
        }
    });

    rsx! {
        canvas {
            id: canvas_id(),
            class: class.as_str(),
            style: "width: 100%; height: 100%; display: block; touch-action: none; user-select: none; -webkit-user-select: none;",
            onmounted: move |_element| {
                let id = canvas_id();
                wasm_bindgen_futures::spawn_local(async move {
                    init_three_js(&id).await;
                });
            },
        }
    }
}

/// Wait for a specified number of milliseconds
async fn wait_ms(ms: u32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(window) = web_sys::window() {
            let _ =
                window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms as i32);
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// Initialize Three.js on the canvas with Phase 1 features
async fn init_three_js(canvas_id: &str) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            web_sys::console::error_1(&"No global window".into());
            return;
        }
    };

    let document = match window.document() {
        Some(d) => d,
        None => {
            web_sys::console::error_1(&"No document".into());
            return;
        }
    };

    let canvas = match document.get_element_by_id(canvas_id) {
        Some(el) => el,
        None => {
            web_sys::console::error_1(&format!("Canvas element {} not found", canvas_id).into());
            return;
        }
    };

    // Inject Three.js if not already loaded
    if !is_three_js_loaded(&document) {
        web_sys::console::log_1(&"Loading Three.js...".into());
        load_script(
            &document,
            "https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js",
        )
        .await;
    }

    // Wait for Three.js to be available
    let mut attempts = 0;
    while !is_three_js_loaded(&document) && attempts < 50 {
        wait_ms(200).await;
        attempts += 1;
    }

    if !is_three_js_loaded(&document) {
        web_sys::console::error_1(&"Failed to load Three.js after waiting".into());
        return;
    }

    web_sys::console::log_1(&"Three.js loaded successfully!".into());
    create_scene(&canvas);
}

/// Check if Three.js is already loaded
fn is_three_js_loaded(_document: &web_sys::Document) -> bool {
    if let Some(window) = web_sys::window() {
        let result = js_sys::Reflect::get(&window, &"THREE".into());
        return result.map(|v| !v.is_undefined()).unwrap_or(false);
    }
    false
}

/// Check if a specific script is loaded
fn is_script_loaded(document: &web_sys::Document, url: &str) -> bool {
    let scripts = document.get_elements_by_tag_name("script");
    for i in 0..scripts.length() {
        if let Some(script) = scripts.item(i) {
            if let Some(src) = script.get_attribute("src") {
                if src.contains(url) {
                    return true;
                }
            }
        }
    }
    false
}

/// Load a script dynamically and wait for it to load
async fn load_script(document: &web_sys::Document, url: &str) {
    if is_script_loaded(document, url) {
        return;
    }

    web_sys::console::log_1(&format!("Loading script: {}", url).into());

    let script = match document.create_element("script") {
        Ok(s) => s,
        Err(_) => return,
    };

    let _ = script.set_attribute("src", url);
    let _ = script.set_attribute("async", "false");

    let head = match document.head() {
        Some(h) => h,
        None => return,
    };

    let _ = head.append_child(&script);

    let mut attempts = 0;
    while !is_script_loaded(document, url) && attempts < 50 {
        wait_ms(100).await;
        attempts += 1;
    }

    wait_ms(500).await;

    web_sys::console::log_1(&format!("Script loaded: {}", url).into());
}

/// Create the Three.js scene with Phase 1 features
fn create_scene(canvas: &web_sys::Element) {
    let js_code = format!(
        r#"
        (function() {{
            const canvas = document.getElementById('{}');
            if (!canvas) {{
                console.error('Canvas not found');
                return;
            }}
            
            function initScene() {{
                if (typeof window.THREE === 'undefined') {{
                    setTimeout(initScene, 100);
                    return;
                }}
                createScene();
            }}
            
            function createScene() {{
                const THREE = window.THREE;
                
                const scene = new THREE.Scene();
                scene.background = new THREE.Color('#1a1a2e');
                
                if (canvas.clientWidth === 0 || canvas.clientHeight === 0) {{
                    canvas.style.width = '100%';
                    canvas.style.height = '100%';
                    canvas.width = canvas.parentElement ? canvas.parentElement.clientWidth : 800;
                    canvas.height = canvas.parentElement ? canvas.parentElement.clientHeight : 600;
                }}
                
                const width = canvas.clientWidth || canvas.width || 800;
                const height = canvas.clientHeight || canvas.height || 600;
                const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
                camera.position.set(8, 8, 8);
                camera.lookAt(0, 0, 0);
                
                const renderer = new THREE.WebGLRenderer({{ canvas: canvas, antialias: true }});
                renderer.setSize(width, height);
                renderer.setPixelRatio(window.devicePixelRatio);
                renderer.shadowMap.enabled = true;
                
                const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
                scene.add(ambientLight);
                
                const dirLight = new THREE.DirectionalLight(0xffffff, 1.2);
                dirLight.position.set(10, 20, 10);
                dirLight.castShadow = true;
                scene.add(dirLight);
                
                const fillLight = new THREE.DirectionalLight(0xffffff, 0.4);
                fillLight.position.set(-10, 10, -10);
                scene.add(fillLight);
                
                let gridHelper = new THREE.GridHelper(20, 20, 0x444444, 0x222222);
                scene.add(gridHelper);
                
                let axesHelper = new THREE.AxesHelper(2);
                scene.add(axesHelper);
                
                let modelContainer = new THREE.Group();
                scene.add(modelContainer);
                
                // Selection outline group
                let outlineGroup = new THREE.Group();
                scene.add(outlineGroup);
                
                // Gizmo group
                let gizmoGroup = new THREE.Group();
                scene.add(gizmoGroup);
                
                // Raycaster for picking
                const raycaster = new THREE.Raycaster();
                const mouse = new THREE.Vector2();
                
                // Entity tracking
                let entityCounter = 0;
                let entityMap = new Map();
                
                const state = {{
                    rotX: 0,
                    rotY: 0,
                    rotZ: 0,
                    autoRotate: true,
                    rotSpeed: 1.0,
                    scale: 1.0,
                    color: '#ff6b6b',
                    camX: 8,
                    camY: 8,
                    camZ: 8,
                    targetX: 0,
                    targetY: 0,
                    targetZ: 0,
                    showGrid: true,
                    showAxes: true,
                    wireframe: false,
                    background: '#1a1a2e',
                    selection: [],
                    gizmo: null,
                    dragging: false,
                    dragMode: null,
                }};
                
                // Gizmo drag state
                let gizmoDragState = {{
                    isDragging: false,
                    draggedAxis: null,
                    dragMode: null,
                    dragStartObjectWorldPos: null,
                    dragStartObjectPos: null,
                    dragStartObjectRot: null,
                    dragStartObjectScale: null,
                    dragStartNdcY: 0,
                    dragAxisDir: null,
                    dragStartDepth: 0,
                    rotBasisU: null,
                    rotBasisV: null,
                    dragStartAngle: 0,
                    targetEntityId: null,
                    targetObject: null,
                    gizmoMode: 'translate',
                    gizmoSpace: 'world'
                }};
                
                // Map from outline group to source mesh for tracking
                let outlineToMeshMap = new Map();
                
                const dragRaycaster = new THREE.Raycaster();
                
                let autoRotY = 0;
                
                function animate() {{
                    requestAnimationFrame(animate);
                    
                    camera.position.set(state.camX, state.camY, state.camZ);
                    camera.lookAt(state.targetX, state.targetY, state.targetZ);
                    
                    scene.background = new THREE.Color(state.background);
                    
                    if (state.showGrid && !gridHelper.parent) {{
                        scene.add(gridHelper);
                    }} else if (!state.showGrid && gridHelper.parent) {{
                        scene.remove(gridHelper);
                    }}
                    
                    if (state.showAxes && !axesHelper.parent) {{
                        scene.add(axesHelper);
                    }} else if (!state.showAxes && axesHelper.parent) {{
                        scene.remove(axesHelper);
                    }}
                    
                    modelContainer.traverse(function(child) {{
                        if (child.isMesh && child.material) {{
                            if (Array.isArray(child.material)) {{
                                child.material.forEach(m => m.wireframe = state.wireframe);
                            }} else {{
                                child.material.wireframe = state.wireframe;
                            }}
                        }}
                    }});
                    
                    modelContainer.scale.setScalar(state.scale);
                    
                    if (state.autoRotate) {{
                        autoRotY += state.rotSpeed * 0.01;
                        modelContainer.rotation.y = state.rotY + autoRotY;
                        modelContainer.rotation.x = state.rotX;
                        modelContainer.rotation.z = state.rotZ;
                    }} else {{
                        modelContainer.rotation.set(state.rotX, state.rotY, state.rotZ);
                    }}
                    
                    updateGizmoPosition();
                    
                    // Update selection outlines to track their source meshes
                    for (const [outlineGroup, sourceMesh] of outlineToMeshMap) {{
                        if (sourceMesh.parent) {{
                            const box = new THREE.Box3().setFromObject(sourceMesh);
                            const center = box.getCenter(new THREE.Vector3());
                            const currentSize = box.getSize(new THREE.Vector3());
                            const originalSize = outlineGroup.userData.originalSize;
                            
                            outlineGroup.position.copy(center);
                            outlineGroup.rotation.copy(sourceMesh.rotation);
                            
                            // Scale outline to match object's current bounding box
                            if (originalSize && originalSize.x > 0 && originalSize.y > 0 && originalSize.z > 0) {{
                                outlineGroup.scale.set(
                                    currentSize.x / originalSize.x,
                                    currentSize.y / originalSize.y,
                                    currentSize.z / originalSize.z
                                );
                            }}
                        }}
                    }}
                    
                    renderer.render(scene, camera);
                }}
                animate();
                
                window.addEventListener('resize', () => {{
                    const w = canvas.clientWidth || canvas.width || 800;
                    const h = canvas.clientHeight || canvas.height || 600;
                    renderer.setSize(w, h);
                    camera.aspect = w / h;
                    camera.updateProjectionMatrix();
                }});
                
                // Pointer event handlers for raycasting
                function getPointerPosition(event) {{
                    const rect = canvas.getBoundingClientRect();
                    const clientX = event.clientX !== undefined ? event.clientX : (event.touches && event.touches[0] ? event.touches[0].clientX : 0);
                    const clientY = event.clientY !== undefined ? event.clientY : (event.touches && event.touches[0] ? event.touches[0].clientY : 0);
                    
                    return {{
                        x: ((clientX - rect.left) / rect.width) * 2 - 1,
                        y: -((clientY - rect.top) / rect.height) * 2 + 1,
                        screenX: clientX - rect.left,
                        screenY: clientY - rect.top,
                    }};
                }}
                
                function castRay(ndcX, ndcY) {{
                    raycaster.setFromCamera({{ x: ndcX, y: ndcY }}, camera);
                    const intersects = raycaster.intersectObjects(modelContainer.children, true);
                    
                    if (intersects.length > 0) {{
                        const hit = intersects[0];
                        const entityId = (hit.object.userData && hit.object.userData.entityId !== undefined)
                            ? hit.object.userData.entityId
                            : undefined;
                        
                        if (entityId === undefined) return null;
                        
                        return {{
                            entityId: entityId,
                            point: {{ x: hit.point.x, y: hit.point.y, z: hit.point.z }},
                            normal: hit.face ? {{ x: hit.face.normal.x, y: hit.face.normal.y, z: hit.face.normal.z }} : {{ x: 0, y: 1, z: 0 }},
                            uv: hit.uv ? {{ x: hit.uv.x, y: hit.uv.y }} : null,
                            distance: hit.distance,
                            faceIndex: hit.faceIndex,
                            instanceId: hit.instanceId,
                        }};
                    }}
                    return null;
                }}
                
                // Update selection visualization with enhanced border
                window.updateSelectionVisualization = function(canvas, selectionIds, colorHex) {{
                    // Clear existing outlines
                    while(outlineGroup.children.length > 0) {{
                        outlineGroup.remove(outlineGroup.children[0]);
                    }}
                    outlineToMeshMap.clear();
                    
                    const outlineColor = colorHex || '#FFD700';
                    const outlineColorNum = new THREE.Color(outlineColor).getHex();
                    
                    // Add outlines for selected objects
                    modelContainer.traverse(function(child) {{
                        if (child.isMesh && child.userData.entityId !== undefined) {{
                            const isSelected = selectionIds.includes(child.userData.entityId);
                            
                            if (isSelected) {{
                                // Get object bounds
                                const box = new THREE.Box3().setFromObject(child);
                                const size = box.getSize(new THREE.Vector3());
                                const center = box.getCenter(new THREE.Vector3());
                                const maxDim = Math.max(size.x, size.y, size.z);
                                
                                // Create outline group for this object
                                const objOutlineGroup = new THREE.Group();
                                
                                // Main wireframe box
                                const outlineGeometry = new THREE.BoxGeometry(size.x * 1.1, size.y * 1.1, size.z * 1.1);
                                const outlineMaterial = new THREE.MeshBasicMaterial({{
                                    color: outlineColorNum,
                                    wireframe: true,
                                    transparent: true,
                                    opacity: 1.0
                                }});
                                const outlineMesh = new THREE.Mesh(outlineGeometry, outlineMaterial);
                                objOutlineGroup.add(outlineMesh);
                                
                                // Inner glow effect
                                const glowGeometry = new THREE.BoxGeometry(size.x * 1.05, size.y * 1.05, size.z * 1.05);
                                const glowMaterial = new THREE.MeshBasicMaterial({{
                                    color: outlineColorNum,
                                    transparent: true,
                                    opacity: 0.2,
                                    side: THREE.BackSide
                                }});
                                const glowMesh = new THREE.Mesh(glowGeometry, glowMaterial);
                                objOutlineGroup.add(glowMesh);
                                
                                // Position and rotate to match object
                                objOutlineGroup.position.copy(center);
                                objOutlineGroup.rotation.copy(child.rotation);
                                
                                // Store original size for scale tracking
                                objOutlineGroup.userData = {{ originalSize: size.clone() }};
                                
                                outlineGroup.add(objOutlineGroup);
                                outlineToMeshMap.set(objOutlineGroup, child);
                            }}
                        }}
                    }});
                }};
                
                // Update gizmo
                window.updateGizmo = function(canvas, gizmoConfig) {{
                    // Clear existing gizmo
                    while(gizmoGroup.children.length > 0) {{
                        gizmoGroup.remove(gizmoGroup.children[0]);
                    }}
                    
                    if (!gizmoConfig) {{
                        gizmoDragState.targetEntityId = null;
                        gizmoDragState.targetObject = null;
                        return;
                    }}
                    
                    // Use the live entityMap from canvas state (not the stale closure variable)
                    const liveState = canvas.dioxusThreeState;
                    let targetObj = liveState.entityMap.get(gizmoConfig.target);
                    if (!targetObj) {{
                        // Fallback: traverse modelContainer
                        liveState.modelContainer.traverse(function(child) {{
                            if (child.isMesh && child.userData.entityId === gizmoConfig.target) {{
                                targetObj = child;
                            }}
                        }});
                    }}
                    
                    if (!targetObj) return;
                    
                    gizmoDragState.targetEntityId = gizmoConfig.target;
                    gizmoDragState.targetObject = targetObj;
                    gizmoDragState.gizmoMode = (gizmoConfig.mode || 'Translate').toLowerCase();
                    gizmoDragState.gizmoSpace = (gizmoConfig.space || 'World').toLowerCase();
                    
                    // Create gizmo based on mode
                    const gizmoSize = gizmoConfig.size || 1.5;
                    const mode = gizmoDragState.gizmoMode;
                    
                    if (mode === 'translate') {{
                        // Create translation gizmo (arrows)
                        const arrowGeometry = new THREE.ConeGeometry(0.12 * gizmoSize, 0.35 * gizmoSize, 8);
                        const lineGeometry = new THREE.BufferGeometry().setFromPoints([
                            new THREE.Vector3(0, 0, 0),
                            new THREE.Vector3(1, 0, 0)
                        ]);
                        const hitGeometry = new THREE.CylinderGeometry(0.18 * gizmoSize, 0.18 * gizmoSize, 1.2 * gizmoSize, 8);
                        const hitMaterial = new THREE.MeshBasicMaterial({{ visible: false, transparent: true, opacity: 0 }});
                        
                        // X axis (red)
                        const xArrow = new THREE.Mesh(arrowGeometry, new THREE.MeshBasicMaterial({{ color: 0xff0000 }}));
                        xArrow.rotation.z = -Math.PI / 2;
                        xArrow.position.x = 1 * gizmoSize;
                        xArrow.userData = {{ axis: 'x', type: 'gizmo', gizmoMode: 'translate' }};
                        gizmoGroup.add(xArrow);
                        
                        const xLine = new THREE.Line(lineGeometry, new THREE.LineBasicMaterial({{ color: 0xff0000 }}));
                        xLine.scale.setScalar(gizmoSize);
                        xLine.userData = {{ axis: 'x', type: 'gizmo', gizmoMode: 'translate' }};
                        gizmoGroup.add(xLine);
                        
                        const xHit = new THREE.Mesh(hitGeometry, hitMaterial);
                        xHit.rotation.z = -Math.PI / 2;
                        xHit.position.x = 0.6 * gizmoSize;
                        xHit.userData = {{ axis: 'x', type: 'gizmo', gizmoMode: 'translate' }};
                        gizmoGroup.add(xHit);
                        
                        // Y axis (green)
                        const yArrow = new THREE.Mesh(arrowGeometry, new THREE.MeshBasicMaterial({{ color: 0x00ff00 }}));
                        yArrow.position.y = 1 * gizmoSize;
                        yArrow.userData = {{ axis: 'y', type: 'gizmo', gizmoMode: 'translate' }};
                        gizmoGroup.add(yArrow);
                        
                        const yLine = new THREE.Line(lineGeometry, new THREE.LineBasicMaterial({{ color: 0x00ff00 }}));
                        yLine.rotation.z = Math.PI / 2;
                        yLine.scale.setScalar(gizmoSize);
                        yLine.userData = {{ axis: 'y', type: 'gizmo', gizmoMode: 'translate' }};
                        gizmoGroup.add(yLine);
                        
                        const yHit = new THREE.Mesh(hitGeometry, hitMaterial);
                        yHit.position.y = 0.6 * gizmoSize;
                        yHit.userData = {{ axis: 'y', type: 'gizmo', gizmoMode: 'translate' }};
                        gizmoGroup.add(yHit);
                        
                        // Z axis (blue)
                        const zArrow = new THREE.Mesh(arrowGeometry, new THREE.MeshBasicMaterial({{ color: 0x0000ff }}));
                        zArrow.rotation.x = Math.PI / 2;
                        zArrow.position.z = 1 * gizmoSize;
                        zArrow.userData = {{ axis: 'z', type: 'gizmo', gizmoMode: 'translate' }};
                        gizmoGroup.add(zArrow);
                        
                        const zLine = new THREE.Line(lineGeometry, new THREE.LineBasicMaterial({{ color: 0x0000ff }}));
                        zLine.rotation.y = -Math.PI / 2;
                        zLine.scale.setScalar(gizmoSize);
                        zLine.userData = {{ axis: 'z', type: 'gizmo', gizmoMode: 'translate' }};
                        gizmoGroup.add(zLine);
                        
                        const zHit = new THREE.Mesh(hitGeometry, hitMaterial);
                        zHit.rotation.x = Math.PI / 2;
                        zHit.position.z = 0.6 * gizmoSize;
                        zHit.userData = {{ axis: 'z', type: 'gizmo', gizmoMode: 'translate' }};
                        gizmoGroup.add(zHit);
                    }} else if (mode === 'rotate') {{
                        // Create rotation gizmo (tori)
                        const torusGeometry = new THREE.TorusGeometry(1 * gizmoSize, 0.04 * gizmoSize, 8, 32);
                        const hitTorusGeometry = new THREE.TorusGeometry(1 * gizmoSize, 0.1 * gizmoSize, 8, 32);
                        const hitMaterial = new THREE.MeshBasicMaterial({{ visible: false, transparent: true, opacity: 0 }});
                        
                        // X axis (red) - rotate around X
                        const xRing = new THREE.Mesh(torusGeometry, new THREE.MeshBasicMaterial({{ color: 0xff0000 }}));
                        xRing.rotation.y = Math.PI / 2;
                        xRing.userData = {{ axis: 'x', type: 'gizmo', gizmoMode: 'rotate' }};
                        gizmoGroup.add(xRing);
                        
                        const xHit = new THREE.Mesh(hitTorusGeometry, hitMaterial);
                        xHit.rotation.y = Math.PI / 2;
                        xHit.userData = {{ axis: 'x', type: 'gizmo', gizmoMode: 'rotate' }};
                        gizmoGroup.add(xHit);
                        
                        // Y axis (green) - rotate around Y
                        const yRing = new THREE.Mesh(torusGeometry, new THREE.MeshBasicMaterial({{ color: 0x00ff00 }}));
                        yRing.rotation.x = Math.PI / 2;
                        yRing.userData = {{ axis: 'y', type: 'gizmo', gizmoMode: 'rotate' }};
                        gizmoGroup.add(yRing);
                        
                        const yHit = new THREE.Mesh(hitTorusGeometry, hitMaterial);
                        yHit.rotation.x = Math.PI / 2;
                        yHit.userData = {{ axis: 'y', type: 'gizmo', gizmoMode: 'rotate' }};
                        gizmoGroup.add(yHit);
                        
                        // Z axis (blue) - rotate around Z
                        const zRing = new THREE.Mesh(torusGeometry, new THREE.MeshBasicMaterial({{ color: 0x0000ff }}));
                        zRing.userData = {{ axis: 'z', type: 'gizmo', gizmoMode: 'rotate' }};
                        gizmoGroup.add(zRing);
                        
                        const zHit = new THREE.Mesh(hitTorusGeometry, hitMaterial);
                        zHit.userData = {{ axis: 'z', type: 'gizmo', gizmoMode: 'rotate' }};
                        gizmoGroup.add(zHit);
                    }} else if (mode === 'scale') {{
                        // Create scale gizmo (boxes at ends)
                        const boxGeometry = new THREE.BoxGeometry(0.22 * gizmoSize, 0.22 * gizmoSize, 0.22 * gizmoSize);
                        const lineGeometry = new THREE.BufferGeometry().setFromPoints([
                            new THREE.Vector3(0, 0, 0),
                            new THREE.Vector3(1, 0, 0)
                        ]);
                        
                        // X axis (red)
                        const xBox = new THREE.Mesh(boxGeometry, new THREE.MeshBasicMaterial({{ color: 0xff0000 }}));
                        xBox.position.x = 1 * gizmoSize;
                        xBox.userData = {{ axis: 'x', type: 'gizmo', gizmoMode: 'scale' }};
                        gizmoGroup.add(xBox);
                        
                        const xLine = new THREE.Line(lineGeometry, new THREE.LineBasicMaterial({{ color: 0xff0000 }}));
                        xLine.scale.setScalar(gizmoSize);
                        gizmoGroup.add(xLine);
                        
                        // Y axis (green)
                        const yBox = new THREE.Mesh(boxGeometry, new THREE.MeshBasicMaterial({{ color: 0x00ff00 }}));
                        yBox.position.y = 1 * gizmoSize;
                        yBox.userData = {{ axis: 'y', type: 'gizmo', gizmoMode: 'scale' }};
                        gizmoGroup.add(yBox);
                        
                        const yLine = new THREE.Line(lineGeometry, new THREE.LineBasicMaterial({{ color: 0x00ff00 }}));
                        yLine.rotation.z = Math.PI / 2;
                        yLine.scale.setScalar(gizmoSize);
                        gizmoGroup.add(yLine);
                        
                        // Z axis (blue)
                        const zBox = new THREE.Mesh(boxGeometry, new THREE.MeshBasicMaterial({{ color: 0x0000ff }}));
                        zBox.position.z = 1 * gizmoSize;
                        zBox.userData = {{ axis: 'z', type: 'gizmo', gizmoMode: 'scale' }};
                        gizmoGroup.add(zBox);
                        
                        const zLine = new THREE.Line(lineGeometry, new THREE.LineBasicMaterial({{ color: 0x0000ff }}));
                        zLine.rotation.y = -Math.PI / 2;
                        zLine.scale.setScalar(gizmoSize);
                        gizmoGroup.add(zLine);
                        
                        // Center box (white) for uniform scale
                        const centerBox = new THREE.Mesh(
                            new THREE.BoxGeometry(0.15 * gizmoSize, 0.15 * gizmoSize, 0.15 * gizmoSize),
                            new THREE.MeshBasicMaterial({{ color: 0xffffff }})
                        );
                        centerBox.userData = {{ axis: 'all', type: 'gizmo', gizmoMode: 'scale' }};
                        gizmoGroup.add(centerBox);
                    }}
                    
                    // Position gizmo at target
                    updateGizmoPosition();
                }};
                
                // Update gizmo position to follow target
                function updateGizmoPosition() {{
                    if (!gizmoDragState.targetObject) return;
                    
                    const targetObj = gizmoDragState.targetObject;
                    const worldPos = new THREE.Vector3();
                    targetObj.getWorldPosition(worldPos);
                    gizmoGroup.position.copy(worldPos);
                    
                    // Update rotation based on space mode
                    if (gizmoDragState.gizmoSpace === 'local') {{
                        gizmoGroup.rotation.copy(targetObj.rotation);
                    }} else {{
                        gizmoGroup.rotation.set(0, 0, 0);
                    }}
                }}
                
                // Raycast against gizmo handles
                function raycastGizmo(ndcX, ndcY) {{
                    if (gizmoGroup.children.length === 0) return null;
                    
                    dragRaycaster.setFromCamera({{ x: ndcX, y: ndcY }}, camera);
                    const intersects = dragRaycaster.intersectObjects(gizmoGroup.children, false);
                    
                    for (let i = 0; i < intersects.length; i++) {{
                        const hit = intersects[i];
                        if (hit.object.userData && hit.object.userData.type === 'gizmo') {{
                            return {{
                                axis: hit.object.userData.axis,
                                mode: hit.object.userData.gizmoMode,
                                point: hit.point
                            }};
                        }}
                    }}
                    return null;
                }}
                
                // Closest point on an infinite line to a ray (robust, no planes needed)
                function closestPointOnLineToRay(lineOrigin, lineDir, rayOrigin, rayDir) {{
                    const w = new THREE.Vector3().subVectors(lineOrigin, rayOrigin);
                    const a = lineDir.dot(lineDir);
                    const b = lineDir.dot(rayDir);
                    const c = rayDir.dot(rayDir);
                    const d = lineDir.dot(w);
                    const e = rayDir.dot(w);
                    const det = a * c - b * b;
                    if (Math.abs(det) < 0.0001) {{
                        // Nearly parallel - return line origin
                        return lineOrigin.clone();
                    }}
                    const t = (b * e - c * d) / det;
                    return lineOrigin.clone().add(lineDir.clone().multiplyScalar(t));
                }}
                
                // Start gizmo drag
                function startGizmoDrag(axis, mode, ndcPos) {{
                    if (!gizmoDragState.targetObject) {{
                        console.warn('Dioxus Three: Cannot start gizmo drag - no target object');
                        return false;
                    }}
                    
                    gizmoDragState.isDragging = true;
                    gizmoDragState.draggedAxis = axis;
                    gizmoDragState.dragMode = mode;
                    gizmoDragState.dragStartNdcY = ndcPos.y;
                    
                    const targetObj = gizmoDragState.targetObject;
                    console.log('[GIZMO-START] axis:', axis, 'mode:', mode, 'target:', gizmoDragState.targetEntityId, 'parent:', targetObj.parent ? targetObj.parent.type : 'null');
                    
                    // Store starting transforms
                    const worldPos = new THREE.Vector3();
                    targetObj.getWorldPosition(worldPos);
                    gizmoDragState.dragStartObjectWorldPos = worldPos.clone();
                    gizmoDragState.dragStartObjectPos = targetObj.position.clone();
                    gizmoDragState.dragStartObjectRot = targetObj.rotation.clone();
                    gizmoDragState.dragStartObjectScale = targetObj.scale.clone();
                    
                    if (mode === 'translate') {{
                        // Build the drag axis in world space
                        const axisVector = new THREE.Vector3(
                            axis === 'x' ? 1 : 0,
                            axis === 'y' ? 1 : 0,
                            axis === 'z' ? 1 : 0
                        );
                        if (gizmoDragState.gizmoSpace === 'local') {{
                            axisVector.applyEuler(targetObj.rotation);
                        }}
                        gizmoDragState.dragAxisDir = axisVector.normalize();
                        
                        // Create a plane that CONTAINS the drag axis and faces the camera.
                        // Plane normal = component of camera direction perpendicular to axis
                        const cameraDir = new THREE.Vector3().subVectors(camera.position, worldPos).normalize();
                        let planeNormal = cameraDir.clone();
                        // Remove component along the axis so the plane contains the axis
                        planeNormal.sub(axisVector.clone().multiplyScalar(planeNormal.dot(axisVector)));
                        
                        if (planeNormal.lengthSq() < 0.0001) {{
                            // Axis is parallel to camera direction — use camera up as fallback
                            planeNormal.crossVectors(axisVector, camera.up).normalize();
                        }} else {{
                            planeNormal.normalize();
                        }}
                        
                        gizmoDragState.dragPlane = new THREE.Plane().setFromNormalAndCoplanarPoint(planeNormal, worldPos);
                        
                        // Store initial plane hit for offset calculation
                        dragRaycaster.setFromCamera({{ x: ndcPos.x, y: ndcPos.y }}, camera);
                        const hit = dragRaycaster.ray.intersectPlane(gizmoDragState.dragPlane, new THREE.Vector3());
                        gizmoDragState.dragPlaneHit = hit ? hit.clone() : worldPos.clone();
                        
                        console.log('[GIZMO-START] translate axis:', axis, 'planeNormal:', '{{x:'+planeNormal.x.toFixed(2)+',y:'+planeNormal.y.toFixed(2)+',z:'+planeNormal.z.toFixed(2)+'}}', 'hit:', hit ? '{{x:'+hit.x.toFixed(2)+',y:'+hit.y.toFixed(2)+',z:'+hit.z.toFixed(2)+'}}' : 'null');
                    }} else if (mode === 'rotate') {{
                        const axisVector = new THREE.Vector3(
                            axis === 'x' ? 1 : 0,
                            axis === 'y' ? 1 : 0,
                            axis === 'z' ? 1 : 0
                        );
                        if (gizmoDragState.gizmoSpace === 'local') {{
                            axisVector.applyEuler(targetObj.rotation);
                        }}
                        gizmoDragState.dragAxisDir = axisVector.normalize();
                        
                        // Build orthonormal basis in the rotation plane
                        const normal = gizmoDragState.dragAxisDir;
                        let u = new THREE.Vector3().crossVectors(normal, camera.up).normalize();
                        if (u.lengthSq() < 0.001) {{
                            u = new THREE.Vector3().crossVectors(normal, new THREE.Vector3(1,0,0)).normalize();
                        }}
                        const v = new THREE.Vector3().crossVectors(normal, u).normalize();
                        gizmoDragState.rotBasisU = u;
                        gizmoDragState.rotBasisV = v;
                        
                        // Remember initial angle
                        dragRaycaster.setFromCamera({{ x: ndcPos.x, y: ndcPos.y }}, camera);
                        const plane = new THREE.Plane().setFromNormalAndCoplanarPoint(normal, worldPos);
                        const hit = dragRaycaster.ray.intersectPlane(plane, new THREE.Vector3());
                        if (hit !== null) {{
                            const toHit = new THREE.Vector3().subVectors(hit, worldPos);
                            gizmoDragState.dragStartAngle = Math.atan2(
                                toHit.dot(v), toHit.dot(u)
                            );
                        }} else {{
                            gizmoDragState.dragStartAngle = 0;
                        }}
                    }} else if (mode === 'scale') {{
                        console.log('[GIZMO-START] scale startNdcY:', ndcPos.y, 'startScale:', targetObj.scale.x.toFixed(3), targetObj.scale.y.toFixed(3), targetObj.scale.z.toFixed(3));
                    }}
                    
                    canvas.style.cursor = axis === 'all' ? 'move' : (mode === 'rotate' ? 'ew-resize' : 'move');
                    return true;
                }}
                
                // Update gizmo drag
                function updateGizmoDrag(ndcX, ndcY) {{
                    if (!gizmoDragState.isDragging || !gizmoDragState.targetObject) return;
                    
                    const targetObj = gizmoDragState.targetObject;
                    const axis = gizmoDragState.draggedAxis;
                    const mode = gizmoDragState.dragMode;
                    
                    dragRaycaster.setFromCamera({{ x: ndcX, y: ndcY }}, camera);
                    
                    try {{
                        if (mode === 'translate') {{
                            if (!gizmoDragState.dragPlane || !gizmoDragState.dragPlaneHit) {{
                                console.warn('[GIZMO-DRAG] translate missing plane data');
                                return;
                            }}
                            
                            // Intersect mouse ray with the drag plane
                            const hit = dragRaycaster.ray.intersectPlane(gizmoDragState.dragPlane, new THREE.Vector3());
                            if (!hit) {{
                                console.warn('[GIZMO-DRAG] translate ray missed plane');
                                return;
                            }}
                            
                            // Delta from initial hit, projected onto drag axis
                            const toHit = new THREE.Vector3().subVectors(hit, gizmoDragState.dragPlaneHit);
                            const axisDelta = toHit.dot(gizmoDragState.dragAxisDir);
                            
                            const newWorldPos = gizmoDragState.dragStartObjectWorldPos.clone().add(
                                gizmoDragState.dragAxisDir.clone().multiplyScalar(axisDelta)
                            );
                            
                            const localPos = newWorldPos.clone();
                            if (targetObj.parent) {{
                                targetObj.parent.worldToLocal(localPos);
                            }}
                            targetObj.position.copy(localPos);
                            
                            console.log('[GIZMO-DRAG] translate axisDelta:', axisDelta.toFixed(3), 'newPos:', '{{x:'+targetObj.position.x.toFixed(2)+',y:'+targetObj.position.y.toFixed(2)+',z:'+targetObj.position.z.toFixed(2)+'}}');
                            
                            updateGizmoPosition();
                            notifyGizmoDrag(false);
                        }} else if (mode === 'rotate') {{
                            const center = gizmoDragState.dragStartObjectWorldPos;
                            const normal = gizmoDragState.dragAxisDir;
                            const plane = new THREE.Plane().setFromNormalAndCoplanarPoint(normal, center);
                            const hit = dragRaycaster.ray.intersectPlane(plane, new THREE.Vector3());
                            
                            if (hit !== null && gizmoDragState.rotBasisU) {{
                                const toHit = new THREE.Vector3().subVectors(hit, center);
                                const currentAngle = Math.atan2(
                                    toHit.dot(gizmoDragState.rotBasisV),
                                    toHit.dot(gizmoDragState.rotBasisU)
                                );
                                const angle = currentAngle - gizmoDragState.dragStartAngle;
                                
                                targetObj.rotation.copy(gizmoDragState.dragStartObjectRot);
                                if (axis === 'x') targetObj.rotation.x += angle;
                                if (axis === 'y') targetObj.rotation.y += angle;
                                if (axis === 'z') targetObj.rotation.z += angle;
                                
                                updateGizmoPosition();
                                notifyGizmoDrag(false);
                            }}
                        }} else if (mode === 'scale') {{
                            const deltaY = (gizmoDragState.dragStartNdcY - ndcY) * 4;  // increased sensitivity
                            const scaleFactor = Math.max(0.1, 1 + deltaY);
                            
                            const newScale = gizmoDragState.dragStartObjectScale.clone();
                            if (axis === 'x' || axis === 'all') newScale.x = gizmoDragState.dragStartObjectScale.x * scaleFactor;
                            if (axis === 'y' || axis === 'all') newScale.y = gizmoDragState.dragStartObjectScale.y * scaleFactor;
                            if (axis === 'z' || axis === 'all') newScale.z = gizmoDragState.dragStartObjectScale.z * scaleFactor;
                            
                            targetObj.scale.copy(newScale);
                            
                            console.log('[GIZMO-DRAG] scale deltaY:', deltaY.toFixed(3), 'factor:', scaleFactor.toFixed(3), 'newScale:', '{{x:'+newScale.x.toFixed(2)+',y:'+newScale.y.toFixed(2)+',z:'+newScale.z.toFixed(2)+'}}');
                            
                            notifyGizmoDrag(false);
                        }}
                    }} catch (err) {{
                        console.error('[GIZMO-DRAG] error in', mode, err);
                    }}
                }}
                
                // End gizmo drag
                function endGizmoDrag() {{
                    console.log('Dioxus Three: Gizmo drag ended');
                    if (gizmoDragState.isDragging) {{
                        notifyGizmoDrag(true);
                    }}
                    gizmoDragState.isDragging = false;
                    gizmoDragState.draggedAxis = null;
                    gizmoDragState.dragMode = null;
                    gizmoDragState.dragStartObjectWorldPos = null;
                    gizmoDragState.dragAxisDir = null;
                    gizmoDragState.dragStartDepth = 0;
                    gizmoDragState.rotBasisU = null;
                    gizmoDragState.rotBasisV = null;
                    gizmoDragState.dragStartAngle = 0;
                    canvas.style.cursor = 'default';
                }}
                
                // Notify about gizmo drag
                function notifyGizmoDrag(isFinished) {{
                    if (!gizmoDragState.targetObject) return;
                    
                    const obj = gizmoDragState.targetObject;
                    if (window.dioxusThreeBridge && window.dioxusThreeBridge.onGizmoDrag) {{
                        window.dioxusThreeBridge.onGizmoDrag(canvas.id, JSON.stringify({{
                            target: gizmoDragState.targetEntityId,
                            mode: gizmoDragState.gizmoMode,
                            space: gizmoDragState.gizmoSpace,
                            transform: {{
                                position: {{ x: obj.position.x, y: obj.position.y, z: obj.position.z }},
                                rotation: {{ x: obj.rotation.x, y: obj.rotation.y, z: obj.rotation.z }},
                                scale: {{ x: obj.scale.x, y: obj.scale.y, z: obj.scale.z }}
                            }},
                            isFinished: isFinished
                        }}));
                    }}
                }}
                
                // Pointer events
                canvas.addEventListener('pointerdown', (e) => {{
                    try {{
                        const pos = getPointerPosition(e);
                        
                        // Check if clicking on gizmo first
                        const gizmoHit = raycastGizmo(pos.x, pos.y);
                        if (gizmoHit && e.button === 0) {{
                            console.log('Dioxus Three: Gizmo hit on pointerdown', gizmoHit);
                            canvas.setPointerCapture(e.pointerId);
                            startGizmoDrag(gizmoHit.axis, gizmoHit.mode, pos);
                            return;
                        }}
                        
                        const hit = castRay(pos.x, pos.y);
                        
                        // Send to Rust via bridge
                        if (window.dioxusThreeBridge && window.dioxusThreeBridge.onPointerDown) {{
                            window.dioxusThreeBridge.onPointerDown(canvas.id, JSON.stringify({{
                                hit: hit,
                                screenPosition: {{ x: pos.screenX, y: pos.screenY }},
                                ndcPosition: {{ x: pos.x, y: pos.y }},
                                button: e.button === 0 ? 'Left' : e.button === 2 ? 'Right' : 'Middle',
                                shiftKey: e.shiftKey,
                                ctrlKey: e.ctrlKey,
                                altKey: e.altKey,
                            }}));
                        }}
                        
                        state.dragging = true;
                        state.dragStart = {{ x: pos.x, y: pos.y }};
                    }} catch (err) {{
                        console.error('Dioxus Three: pointerdown error', err);
                    }}
                }});
                
                canvas.addEventListener('pointermove', (e) => {{
                    try {{
                        const pos = getPointerPosition(e);
                        
                        // Handle gizmo drag
                        if (gizmoDragState.isDragging) {{
                            updateGizmoDrag(pos.x, pos.y);
                            return;
                        }}
                        
                        // Check for gizmo hover
                        const gizmoHit = raycastGizmo(pos.x, pos.y);
                        if (gizmoHit) {{
                            canvas.style.cursor = gizmoHit.mode === 'rotate' ? 'ew-resize' : 'move';
                            return;
                        }}
                        
                        if (state.dragging) {{
                            // Drag event
                            if (window.dioxusThreeBridge && window.dioxusThreeBridge.onPointerDrag) {{
                                window.dioxusThreeBridge.onPointerDrag(canvas.id, JSON.stringify({{
                                    hit: castRay(pos.x, pos.y),
                                    screenPosition: {{ x: pos.screenX, y: pos.screenY }},
                                    delta: {{ x: pos.x - state.dragStart.x, y: pos.y - state.dragStart.y }},
                                }}));
                            }}
                        }} else {{
                            // Move/hover event
                            const hit = castRay(pos.x, pos.y);
                            
                            if (window.dioxusThreeBridge && window.dioxusThreeBridge.onPointerMove) {{
                                window.dioxusThreeBridge.onPointerMove(canvas.id, JSON.stringify({{
                                    hit: hit,
                                    screenPosition: {{ x: pos.screenX, y: pos.screenY }},
                                    ndcPosition: {{ x: pos.x, y: pos.y }},
                                    shiftKey: e.shiftKey,
                                    ctrlKey: e.ctrlKey,
                                    altKey: e.altKey,
                                }}));
                            }}
                            
                            // Update cursor
                            if (hit) {{
                                canvas.style.cursor = 'pointer';
                            }} else {{
                                canvas.style.cursor = 'default';
                            }}
                        }}
                    }} catch (err) {{
                        console.error('Dioxus Three: pointermove error', err);
                    }}
                }});
                
                canvas.addEventListener('pointerup', (e) => {{
                    try {{
                        // End gizmo drag if active
                        if (gizmoDragState.isDragging) {{
                            if (canvas.hasPointerCapture && canvas.hasPointerCapture(e.pointerId)) {{
                                canvas.releasePointerCapture(e.pointerId);
                            }}
                            endGizmoDrag();
                            return;
                        }}
                        
                        state.dragging = false;
                        
                        const pos = getPointerPosition(e);
                        const hit = castRay(pos.x, pos.y);
                        
                        if (window.dioxusThreeBridge && window.dioxusThreeBridge.onPointerUp) {{
                            window.dioxusThreeBridge.onPointerUp(canvas.id, JSON.stringify({{
                                hit: hit,
                                screenPosition: {{ x: pos.screenX, y: pos.screenY }},
                                ndcPosition: {{ x: pos.x, y: pos.y }},
                                button: e.button === 0 ? 'Left' : e.button === 2 ? 'Right' : 'Middle',
                            }}));
                        }}
                    }} catch (err) {{
                        console.error('Dioxus Three: pointerup error', err);
                    }}
                }});
                
                canvas.dioxusThreeState = {{ 
                    scene, 
                    camera, 
                    modelContainer, 
                    outlineGroup,
                    gizmoGroup,
                    state,
                    raycaster,
                    entityCounter,
                    entityMap,
                }};
                
                console.log('Dioxus Three: Scene initialized with Phase 1 features (raycasting, selection, gizmos)');
            }}
            
            initScene();
        }})();
        "#,
        canvas.id(),
    );

    if let Err(e) = js_sys::eval(&js_code) {
        web_sys::console::error_1(&format!("Failed to create scene: {:?}", e).into());
    }
}

/// Build JavaScript code to load models with entity IDs
fn build_models_js(models: &[crate::ModelConfig]) -> String {
    if models.is_empty() {
        // Default cube
        return r#"
            const geometry = new THREE.BoxGeometry(1, 1, 1);
            const material = new THREE.MeshStandardMaterial({ 
                color: '#ff6b6b', 
                roughness: 0.5, 
                metalness: 0.3
            });
            const model = new THREE.Mesh(geometry, material);
            model.castShadow = true;
            model.receiveShadow = true;
            
            // Assign entity ID - use 0 for default
            const entityId = 0;
            model.userData = { entityId: entityId };
            canvas.dioxusThreeState.entityMap.set(entityId, model);
            
            modelContainer.add(model);
        "#
        .to_string();
    }

    let mut model_code = String::new();

    for (idx, model) in models.iter().enumerate() {
        let pos_x = model.pos_x;
        let pos_y = model.pos_y;
        let pos_z = model.pos_z;
        let rot_x = model.rot_x.to_radians();
        let rot_y = model.rot_y.to_radians();
        let rot_z = model.rot_z.to_radians();
        let scl = model.scale;
        let color = &model.color;
        let url = &model.url;

        let code = if model.format == crate::ModelFormat::Cube || model.url.is_empty() {
            format!(
                r#"
                // Model {}: Cube
                (function() {{
                    const geometry = new THREE.BoxGeometry(1, 1, 1);
                    const material = new THREE.MeshStandardMaterial({{ 
                        color: '{}', 
                        roughness: 0.5, 
                        metalness: 0.3 
                    }});
                    const mesh = new THREE.Mesh(geometry, material);
                    mesh.position.set({}, {}, {});
                    mesh.rotation.set({}, {}, {});
                    mesh.scale.setScalar({});
                    mesh.castShadow = true;
                    mesh.receiveShadow = true;
                    
                    // Assign entity ID - use array index to match Rust Selection
                    const entityId = {};
                    mesh.userData = {{ entityId: entityId }};
                    canvas.dioxusThreeState.entityMap.set(entityId, mesh);
                    
                    modelContainer.add(mesh);
                }})();
                "#,
                idx, color, pos_x, pos_y, pos_z, rot_x, rot_y, rot_z, scl, idx
            )
        } else {
            // Load external model using appropriate loader
            let loader_class = model.format.loader_js();
            let is_geometry_loader = matches!(
                model.format,
                crate::ModelFormat::Stl | crate::ModelFormat::Ply
            );

            if is_geometry_loader {
                format!(
                    r#"
                    // Model {}: {} 
                    (function() {{
                        if (typeof THREE.{loader} === 'undefined') {{
                            console.warn('Loader {loader} not available');
                            const geometry = new THREE.BoxGeometry(1, 1, 1);
                            const material = new THREE.MeshStandardMaterial({{ color: '{color}' }});
                            const mesh = new THREE.Mesh(geometry, material);
                            mesh.position.set({pos_x}, {pos_y}, {pos_z});
                            mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                            mesh.scale.setScalar({scl});
                            
                            const entityId = {idx};
                            mesh.userData = {{ entityId: entityId }};
                            canvas.dioxusThreeState.entityMap.set(entityId, mesh);
                            
                            modelContainer.add(mesh);
                            return;
                        }}
                        
                        const loader = new THREE.{loader}();
                        loader.load(
                            '{url}',
                            function(geometry) {{
                                const material = new THREE.MeshStandardMaterial({{ 
                                    color: '{color}', 
                                    roughness: 0.5, 
                                    metalness: 0.1 
                                }});
                                const mesh = new THREE.Mesh(geometry, material);
                                mesh.position.set({pos_x}, {pos_y}, {pos_z});
                                mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                                mesh.scale.setScalar({scl});
                                mesh.castShadow = true;
                                mesh.receiveShadow = true;
                                
                                const entityId = {idx};
                                mesh.userData = {{ entityId: entityId }};
                                canvas.dioxusThreeState.entityMap.set(entityId, mesh);
                                
                                modelContainer.add(mesh);
                                console.log('Loaded model {idx}: {url}');
                            }},
                            undefined,
                            function(error) {{
                                console.error('Failed to load model {idx}:', error);
                                const geometry = new THREE.BoxGeometry(1, 1, 1);
                                const material = new THREE.MeshStandardMaterial({{ color: '{color}' }});
                                const mesh = new THREE.Mesh(geometry, material);
                                mesh.position.set({pos_x}, {pos_y}, {pos_z});
                                mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                                mesh.scale.setScalar({scl});
                                
                                const entityId = {idx};
                                mesh.userData = {{ entityId: entityId }};
                                canvas.dioxusThreeState.entityMap.set(entityId, mesh);
                                
                                modelContainer.add(mesh);
                            }}
                        );
                    }})();
                    "#,
                    idx,
                    model.format.as_str(),
                    loader = loader_class,
                    url = url.replace("'", "\\'"),
                    color = color,
                    pos_x = pos_x,
                    pos_y = pos_y,
                    pos_z = pos_z,
                    rot_x = rot_x,
                    rot_y = rot_y,
                    rot_z = rot_z,
                    scl = scl,
                    idx = idx
                )
            } else {
                format!(
                    r#"
                    // Model {}: {}
                    (function() {{
                        if (typeof THREE.{loader} === 'undefined') {{
                            console.warn('Loader {loader} not available');
                            const geometry = new THREE.BoxGeometry(1, 1, 1);
                            const material = new THREE.MeshStandardMaterial({{ color: '{color}' }});
                            const mesh = new THREE.Mesh(geometry, material);
                            mesh.position.set({pos_x}, {pos_y}, {pos_z});
                            mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                            mesh.scale.setScalar({scl});
                            
                            const entityId = {idx};
                            mesh.userData = {{ entityId: entityId }};
                            canvas.dioxusThreeState.entityMap.set(entityId, mesh);
                            
                            modelContainer.add(mesh);
                            return;
                        }}
                        
                        const loader = new THREE.{loader}();
                        loader.load(
                            '{url}',
                            function(object) {{
                                let model = object.scene || object;
                                model.position.set({pos_x}, {pos_y}, {pos_z});
                                model.rotation.set({rot_x}, {rot_y}, {rot_z});
                                model.scale.setScalar({scl});
                                
                                // Assign entity ID to the root model
                                const entityId = {idx};
                                model.userData = {{ entityId: entityId }};
                                
                                model.traverse(function(child) {{
                                    if (child.isMesh) {{
                                        child.castShadow = true;
                                        child.receiveShadow = true;
                                        // Also assign to child meshes for raycasting
                                        child.userData = {{ entityId: entityId }};
                                    }}
                                }});
                                canvas.dioxusThreeState.entityMap.set(entityId, model);
                                modelContainer.add(model);
                                console.log('Loaded model {idx}: {url}');
                            }},
                            function(xhr) {{
                                console.log('Model {idx} loading: ' + (xhr.loaded / xhr.total * 100) + '%');
                            }},
                            function(error) {{
                                console.error('Failed to load model {idx}:', error);
                                const geometry = new THREE.BoxGeometry(1, 1, 1);
                                const material = new THREE.MeshStandardMaterial({{ color: '{color}' }});
                                const mesh = new THREE.Mesh(geometry, material);
                                mesh.position.set({pos_x}, {pos_y}, {pos_z});
                                mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                                mesh.scale.setScalar({scl});
                                
                                const entityId = {idx};
                                mesh.userData = {{ entityId: entityId }};
                                canvas.dioxusThreeState.entityMap.set(entityId, mesh);
                                
                                modelContainer.add(mesh);
                            }}
                        );
                    }})();
                    "#,
                    idx,
                    model.format.as_str(),
                    loader = loader_class,
                    url = url.replace("'", "\\'"),
                    color = color,
                    pos_x = pos_x,
                    pos_y = pos_y,
                    pos_z = pos_z,
                    rot_x = rot_x,
                    rot_y = rot_y,
                    rot_z = rot_z,
                    scl = scl,
                    idx = idx
                )
            }
        };

        model_code.push_str(&code);
    }

    model_code
}

/// Load all required loaders for the given models
async fn load_required_loaders(models: &[crate::ModelConfig]) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    // Collect unique formats that need loaders
    let mut unique_formats: Vec<crate::ModelFormat> = vec![];
    for model in models {
        if model.format != crate::ModelFormat::Cube
            && !model.url.is_empty()
            && !unique_formats.contains(&model.format)
        {
            unique_formats.push(model.format.clone());
        }
    }

    // Load each required loader
    for format in &unique_formats {
        let loader_url = get_loader_url(format);
        if !loader_url.is_empty() {
            load_script(&document, loader_url).await;

            // Load extra dependencies (like fflate for FBX)
            for extra in format.extra_scripts() {
                load_script(&document, extra).await;
            }
        }
    }
}

/// Get the loader URL for a format
fn get_loader_url(format: &crate::ModelFormat) -> &'static str {
    match format {
        crate::ModelFormat::Obj => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/OBJLoader.js"
        }
        crate::ModelFormat::Fbx => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/FBXLoader.js"
        }
        crate::ModelFormat::Gltf | crate::ModelFormat::Glb => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/GLTFLoader.js"
        }
        crate::ModelFormat::Stl => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/STLLoader.js"
        }
        crate::ModelFormat::Ply => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/PLYLoader.js"
        }
        crate::ModelFormat::Dae => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/ColladaLoader.js"
        }
        crate::ModelFormat::Json => "",
        crate::ModelFormat::Cube => "",
    }
}
