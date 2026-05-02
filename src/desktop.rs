//! Desktop implementation of ThreeView using WebView iframe
//!
//! Enhanced with eval bridge for iframe -> Rust event communication.

use crate::{
    generate_three_js_html, EntityId, GizmoEvent, GizmoMode, GizmoSpace, GizmoTransform,
    HitInfo, PointerEvent, ThreeViewProps, Vector2, Vector3,
};
use dioxus::document::eval;
use dioxus::prelude::*;

/// Parse a hit info from serde_json::Value
fn parse_hit_info(val: &serde_json::Value) -> Option<HitInfo> {
    let hit = val.get("hit")?;
    if hit.is_null() {
        return None;
    }
    let entity_id = hit.get("entityId")?.as_u64()? as usize;
    let point = parse_vec3(hit.get("point")?)?;
    let normal = parse_vec3(hit.get("normal")?)?;
    let uv = hit.get("uv").and_then(|uv| {
        if uv.is_null() {
            None
        } else {
            Some(Vector2::new(
                uv.get("x")?.as_f64()? as f32,
                uv.get("y")?.as_f64()? as f32,
            ))
        }
    });
    let distance = hit.get("distance")?.as_f64()? as f32;
    let face_index = hit
        .get("faceIndex")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);
    let instance_id = hit
        .get("instanceId")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);

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

fn parse_vec2(val: &serde_json::Value) -> Option<Vector2> {
    Some(Vector2::new(
        val.get("x")?.as_f64()? as f32,
        val.get("y")?.as_f64()? as f32,
    ))
}

fn parse_vec3(val: &serde_json::Value) -> Option<Vector3> {
    Some(Vector3::new(
        val.get("x")?.as_f64()? as f32,
        val.get("y")?.as_f64()? as f32,
        val.get("z")?.as_f64()? as f32,
    ))
}

fn parse_pointer_event(data: &serde_json::Value) -> Option<PointerEvent> {
    let hit = parse_hit_info(data);
    let screen_position = parse_vec2(data.get("screenPosition")?)?;
    let ndc_position = parse_vec2(data.get("ndcPosition")?)?;
    let button = data.get("button").and_then(|b| match b.as_str()? {
        "Left" => Some(crate::MouseButton::Left),
        "Right" => Some(crate::MouseButton::Right),
        "Middle" => Some(crate::MouseButton::Middle),
        _ => None,
    });
    let shift_key = data.get("shiftKey").and_then(|v| v.as_bool()).unwrap_or(false);
    let ctrl_key = data.get("ctrlKey").and_then(|v| v.as_bool()).unwrap_or(false);
    let alt_key = data.get("altKey").and_then(|v| v.as_bool()).unwrap_or(false);

    Some(PointerEvent {
        hit,
        screen_position,
        ndc_position,
        button,
        shift_key,
        ctrl_key,
        alt_key,
    })
}

fn parse_gizmo_event(data: &serde_json::Value) -> Option<GizmoEvent> {
    let target = data.get("target")?.as_u64()? as usize;
    let mode = data
        .get("mode")
        .and_then(|m| match m.as_str()?.to_lowercase().as_str() {
            "translate" => Some(GizmoMode::Translate),
            "rotate" => Some(GizmoMode::Rotate),
            "scale" => Some(GizmoMode::Scale),
            _ => Some(GizmoMode::Translate),
        })
        .unwrap_or(GizmoMode::Translate);
    let space = data
        .get("space")
        .and_then(|s| match s.as_str()?.to_lowercase().as_str() {
            "local" => Some(GizmoSpace::Local),
            _ => Some(GizmoSpace::World),
        })
        .unwrap_or(GizmoSpace::World);

    let transform_obj = data.get("transform")?;
    let position = parse_vec3(transform_obj.get("position")?)?;
    let rotation = parse_vec3(transform_obj.get("rotation")?)?;
    let scale = parse_vec3(transform_obj.get("scale")?)?;
    let is_finished = data
        .get("isFinished")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    Some(GizmoEvent {
        target: EntityId(target),
        mode,
        space,
        transform: GizmoTransform {
            position,
            rotation,
            scale,
        },
        is_finished,
    })
}

/// A Three.js 3D viewer component for Dioxus Desktop
///
/// Uses a WebView iframe to render the Three.js scene.
/// Supports all features including multiple models, shaders, animations,
/// raycasting, selection, and transform gizmos.
#[component]
pub fn ThreeView(mut props: ThreeViewProps) -> Element {
    // Generate initial HTML once - subsequent updates go via postMessage
    let mut html_signal = use_signal(|| generate_three_js_html(&props));
    let mut prev_models_len = use_signal(|| props.models.len());

    // Take callbacks for event handling
    let on_pointer_down = props.on_pointer_down.take();
    let on_pointer_up = props.on_pointer_up.take();
    let on_pointer_move = props.on_pointer_move.take();
    let on_gizmo_drag = props.on_gizmo_drag.take();
    let on_selection_change = props.on_selection_change.take();

    // Set up eval bridge for iframe events
    let _ = use_hook(move || {
        spawn(async move {
            let mut eval = eval(
                r#"
                window._dioxusThreeEvents = [];
                window.addEventListener('message', function(e) {
                    if (e.data && (
                        e.data.type === 'gizmo-drag' || 
                        e.data.type === 'pointer-down' ||
                        e.data.type === 'pointer-up' ||
                        e.data.type === 'pointer-move' ||
                        e.data.type === 'selection-change'
                    )) {
                        window._dioxusThreeEvents.push(e.data);
                        dioxus.send(e.data);
                    }
                });
                
                while (true) {
                    await dioxus.recv();
                }
                "#,
            );

            loop {
                match eval.recv::<serde_json::Value>().await {
                    Ok(event) => {
                        let event_type = event.get("type").and_then(|v| v.as_str());
                        let data = event.get("data");

                        match event_type {
                            Some("gizmo-drag") => {
                                if let (Some(cb), Some(data)) = (&on_gizmo_drag, data) {
                                    if let Some(gizmo_event) = parse_gizmo_event(data) {
                                        cb.call(gizmo_event);
                                    }
                                }
                            }
                            Some("pointer-down") => {
                                if let (Some(cb), Some(data)) = (&on_pointer_down, data) {
                                    if let Some(ptr_event) = parse_pointer_event(data) {
                                        cb.call(ptr_event);
                                    }
                                }
                            }
                            Some("pointer-up") => {
                                if let (Some(cb), Some(data)) = (&on_pointer_up, data) {
                                    if let Some(ptr_event) = parse_pointer_event(data) {
                                        cb.call(ptr_event);
                                    }
                                }
                            }
                            Some("pointer-move") => {
                                if let (Some(cb), Some(data)) = (&on_pointer_move, data) {
                                    if let Some(ptr_event) = parse_pointer_event(data) {
                                        cb.call(ptr_event);
                                    }
                                }
                            }
                            Some("selection-change") => {
                                if let (Some(cb), Some(data)) = (&on_selection_change, data) {
                                    if let Some(selection_ids) = data.get("selection") {
                                        let ids: Vec<EntityId> = selection_ids
                                            .as_array()
                                            .unwrap_or(&vec![])
                                            .iter()
                                            .filter_map(|v| v.as_u64().map(|id| EntityId(id as usize)))
                                            .collect();
                                        let mut selection = crate::Selection::with_mode(props.selection_mode);
                                        for id in ids {
                                            selection.select(id);
                                        }
                                        cb.call(selection);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {
                        // Eval finished or error, break the loop
                        break;
                    }
                }
            }
        });
    });

    // Send state updates to iframe via postMessage (avoids full reloads)
    use_effect(use_reactive((&props,), move |(new_props,)| {
        let old_len = prev_models_len();
        let new_len = new_props.models.len();
        if old_len != new_len {
            html_signal.set(generate_three_js_html(&new_props));
            prev_models_len.set(new_len);
            return;
        }
        
        spawn(async move {
            let selection_json = match &new_props.selection {
                Some(s) => format!("[{}]", s.iter().map(|e| e.0.to_string()).collect::<Vec<_>>().join(",")),
                None => "[]".to_string(),
            };
            
            let gizmo_json = match &new_props.gizmo {
                Some(g) => format!(
                    r#"{{"target":{},"mode":"{:?}","space":"{:?}"}}"#,
                    g.target.0, g.mode, g.space
                ),
                None => "null".to_string(),
            };
            
            println!("[DESKTOP] Sending postMessage - selection: {}, gizmo: {}", selection_json, gizmo_json);
            
            let js = format!(r#"
                (function() {{
                    const iframe = document.querySelector('iframe[srcdoc]');
                    if (iframe && iframe.contentWindow) {{
                        console.log('[POSTMSG] sending update-state');
                        iframe.contentWindow.postMessage({{
                            type: 'update-state',
                            camX: {},
                            camY: {},
                            camZ: {},
                            targetX: {},
                            targetY: {},
                            targetZ: {},
                            autoRotate: {},
                            rotSpeed: {},
                            scale: {},
                            color: '{}',
                            background: '{}',
                            showGrid: {},
                            showAxes: {},
                            wireframe: {},
                            selection: {},
                            gizmo: {}
                        }}, '*');
                    }} else {{
                        console.warn('[POSTMSG] iframe not found');
                    }}
                }})();
            "#,
                new_props.cam_x,
                new_props.cam_y,
                new_props.cam_z,
                new_props.target_x,
                new_props.target_y,
                new_props.target_z,
                new_props.auto_rotate.to_string().to_lowercase(),
                new_props.rot_speed,
                new_props.scale,
                new_props.color.replace('\\', "\\\\").replace('\'', "\\'"),
                new_props.background.replace('\\', "\\\\").replace('\'', "\\'"),
                new_props.show_grid.to_string().to_lowercase(),
                new_props.show_axes.to_string().to_lowercase(),
                new_props.wireframe.to_string().to_lowercase(),
                selection_json,
                gizmo_json
            );
            let _ = eval(&js).await;
            println!("[DESKTOP] postMessage sent");
        });
    }));

    rsx! {
        iframe {
            class: "{props.class}",
            style: "width: 100%; height: 100%; border: none;",
            srcdoc: "{html_signal}",
        }
    }
}
