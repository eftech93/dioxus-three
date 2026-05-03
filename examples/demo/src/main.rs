//! Dioxus Three - Desktop Demo
//!
//! A comprehensive demo showing all features:
//! - Multi-model support with external model loading
//! - Camera controls with presets
//! - Shader effects
//! - Phase 1: Raycasting, Selection, Transform Gizmos

use dioxus::prelude::*;
use dioxus_three::{EntityId, Gizmo, GizmoMode, GizmoSpace, GizmoTransform, Selection};
use dioxus_three::{ModelConfig, ModelFormat, ShaderPreset};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
struct SceneModel {
    id: usize,
    name: String,
    config: ModelConfig,
}

fn main() {
    tracing_subscriber::fmt::init();
    dioxus_desktop::launch::launch(app, vec![], vec![Box::new(dioxus_desktop::Config::new())]);
}

fn app() -> Element {
    // Scene models - start with 3 colored cubes
    let mut models = use_signal(|| {
        vec![
            SceneModel {
                id: 0,
                name: "Red Cube".to_string(),
                config: ModelConfig::new("", ModelFormat::Cube)
                    .with_position(-2.0, 0.0, 0.0)
                    .with_color("#ff6b6b"),
            },
            SceneModel {
                id: 1,
                name: "Green Cube".to_string(),
                config: ModelConfig::new("", ModelFormat::Cube)
                    .with_position(0.0, 0.0, 0.0)
                    .with_color("#6bcf7f"),
            },
            SceneModel {
                id: 2,
                name: "Blue Cube".to_string(),
                config: ModelConfig::new("", ModelFormat::Cube)
                    .with_position(2.0, 0.0, 0.0)
                    .with_color("#4dabf7"),
            },
        ]
    });
    let mut next_id = use_signal(|| 3usize);

    // Camera state
    let mut cam_x = use_signal(|| 8.0f32);
    let mut cam_y = use_signal(|| 8.0f32);
    let mut cam_z = use_signal(|| 8.0f32);

    // Phase 1: Selection and Gizmo state
    let mut selection = use_signal(Selection::new);
    let mut gizmo_mode = use_signal(|| GizmoMode::Translate);
    let mut gizmo_space = use_signal(|| GizmoSpace::World);
    let show_gizmo = use_signal(|| true);

    // Track transform overrides from gizmo drags
    let mut transform_overrides = use_signal(HashMap::<usize, GizmoTransform>::new);

    // Global options
    let mut auto_rotate = use_signal(|| false);
    let rot_speed = use_signal(|| 1.0f32);
    let show_grid = use_signal(|| true);
    let show_axes = use_signal(|| true);
    let mut shader = use_signal(|| ShaderPreset::None);
    let wireframe = use_signal(|| false);

    // New model form
    let mut new_url = use_signal(|| "".to_string());
    let mut new_format = use_signal(|| ModelFormat::Obj);
    let mut new_name = use_signal(|| "".to_string());

    // Preset models
    let preset_models = vec![
        ("Cube", "", ModelFormat::Cube),
        ("Male (OBJ)", "https://threejs.org/examples/models/obj/male02/male02.obj", ModelFormat::Obj),
        ("Helmet (glTF)", "https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf", ModelFormat::Gltf),
        ("Duck (glTF)", "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Duck/glTF/Duck.gltf", ModelFormat::Gltf),
    ];

    // NOTE: We do NOT bake transform_overrides into model_configs passed to ThreeView.
    // The gizmo directly manipulates objects in the Three.js scene. Baking overrides here
    // would cause model reloads on every drag frame, killing fluidity.
    let model_configs: Vec<ModelConfig> = models.read().iter().map(|m| m.config.clone()).collect();

    rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" }

        div { class: "flex h-screen bg-gray-900 text-white font-sans overflow-hidden",
            // Left Control Panel
            div { class: "w-[450px] bg-gray-800 p-4 overflow-y-auto shadow-2xl flex-shrink-0",
                h1 { class: "text-xl font-bold mb-4 text-yellow-500", "🎮 Dioxus Three Demo" }
                p { class: "text-xs text-gray-400 mb-4", "v0.0.3 - Phase 1: Raycasting, Selection & Gizmos" }

                // Phase 1: Selection Section
                ControlGroup { title: "🎯 Selection (Phase 1)",
                    div { class: "flex gap-2 mb-3",
                        button {
                            class: "flex-1 bg-blue-600 hover:bg-blue-700 px-3 py-2 rounded text-sm font-medium transition",
                            onclick: move |_| selection.write().clear(),
                            "Clear Selection"
                        }
                        button {
                            class: "flex-1 bg-green-600 hover:bg-green-700 px-3 py-2 rounded text-sm font-medium transition",
                            onclick: move |_| {
                                for i in 0..models().len() {
                                    selection.write().select(EntityId(i));
                                }
                            },
                            "Select All"
                        }
                    }

                    p { class: "text-sm text-gray-400 mb-2",
                        "Selected: {selection().count()} object(s)"
                    }

                    // Selected items list with model names
                    for id in selection().iter() {
                        {
                            let model_name = models().get(id.0).map(|m| m.name.clone()).unwrap_or_else(|| format!("Object {}", id.0));
                            let is_primary = selection().primary() == Some(id);
                            rsx! {
                                div {
                                    class: if is_primary {
                                        "bg-yellow-600 rounded px-3 py-2 mb-1 text-sm flex items-center gap-2 border-2 border-yellow-400"
                                    } else {
                                        "bg-gray-600 rounded px-3 py-2 mb-1 text-sm flex items-center gap-2 border border-gray-500"
                                    },
                                    div { class: "flex items-center gap-2 flex-1",
                                        span { class: "text-yellow-300 text-lg", "★" }
                                        div { class: "flex flex-col",
                                            span { class: "font-medium", "{model_name}" }
                                            span { class: "text-xs text-gray-300", "Entity {id}" }
                                        }
                                    }
                                    if is_primary {
                                        span { class: "text-xs bg-yellow-700 px-2 py-1 rounded", "Primary" }
                                    }
                                }
                            }
                        }
                    }

                    if !selection().has_selection() {
                        div { class: "text-xs text-gray-500 italic bg-gray-700 rounded p-3",
                            p { "💡 Click objects to select" }
                            p { "💡 Shift+click for multi-select" }
                            p { class: "mt-1", "💡 Yellow border appears around selected items" }
                        }
                    } else {
                        div { class: "text-xs text-gray-400 mt-2 bg-gray-700 rounded p-2",
                            p { "🎯 Primary selection (★) controls the gizmo" }
                            p { "📦 Selected items show yellow border + corners in 3D view" }
                        }

                        // Transform readout for primary selection
                        if let Some(primary_id) = selection().primary() {
                            {
                                let idx = primary_id.0;
                                let transform = transform_overrides.read().get(&idx).cloned();
                                let model = models.read().get(idx).cloned();
                                if let Some(model) = model {
                                    let pos = transform.map(|t| t.position)
                                        .unwrap_or_else(|| dioxus_three::Vector3::new(model.config.pos_x, model.config.pos_y, model.config.pos_z));
                                    let rot = transform.map(|t| t.rotation)
                                        .unwrap_or_else(|| dioxus_three::Vector3::new(model.config.rot_x.to_radians(), model.config.rot_y.to_radians(), model.config.rot_z.to_radians()));
                                    let scl = transform.map(|t| t.scale)
                                        .unwrap_or_else(|| dioxus_three::Vector3::new(model.config.scale, model.config.scale, model.config.scale));

                                    rsx! {
                                        div { class: "mt-2 bg-gray-900 rounded p-2 text-xs font-mono space-y-1",
                                            p { class: "text-gray-400 font-medium", "📐 Transform" }
                                            p { class: "text-green-400", "Pos: {pos.x:.2}, {pos.y:.2}, {pos.z:.2}" }
                                            p { class: "text-blue-400", "Rot: {rot.x:.2}, {rot.y:.2}, {rot.z:.2}" }
                                            p { class: "text-red-400", "Scl: {scl.x:.2}, {scl.y:.2}, {scl.z:.2}" }
                                        }
                                    }
                                } else {
                                    rsx! {}
                                }
                            }
                        }
                    }
                }

                // Phase 1: Gizmo Section
                ControlGroup { title: "🔧 Transform Gizmo (Phase 1)",
                    Toggle { label: "Show Gizmo", value: show_gizmo }

                    if show_gizmo() {
                        div { class: "mt-3 space-y-3",
                            div {
                                label { class: "block text-xs text-gray-400 mb-1", "Mode" }
                                div { class: "grid grid-cols-3 gap-2",
                                    GizmoModeButton {
                                        label: "Move",
                                        active: matches!(gizmo_mode(), GizmoMode::Translate),
                                        onclick: move |_| gizmo_mode.set(GizmoMode::Translate)
                                    }
                                    GizmoModeButton {
                                        label: "Rotate",
                                        active: matches!(gizmo_mode(), GizmoMode::Rotate),
                                        onclick: move |_| gizmo_mode.set(GizmoMode::Rotate)
                                    }
                                    GizmoModeButton {
                                        label: "Scale",
                                        active: matches!(gizmo_mode(), GizmoMode::Scale),
                                        onclick: move |_| gizmo_mode.set(GizmoMode::Scale)
                                    }
                                }
                            }

                            div {
                                label { class: "block text-xs text-gray-400 mb-1", "Space" }
                                div { class: "grid grid-cols-2 gap-2",
                                    button {
                                        class: if matches!(gizmo_space(), GizmoSpace::World) {
                                            "bg-yellow-600 hover:bg-yellow-700 px-3 py-2 rounded text-sm transition"
                                        } else {
                                            "bg-gray-600 hover:bg-gray-500 px-3 py-2 rounded text-sm transition"
                                        },
                                        onclick: move |_| gizmo_space.set(GizmoSpace::World),
                                        "World"
                                    }
                                    button {
                                        class: if matches!(gizmo_space(), GizmoSpace::Local) {
                                            "bg-yellow-600 hover:bg-yellow-700 px-3 py-2 rounded text-sm transition"
                                        } else {
                                            "bg-gray-600 hover:bg-gray-500 px-3 py-2 rounded text-sm transition"
                                        },
                                        onclick: move |_| gizmo_space.set(GizmoSpace::Local),
                                        "Local"
                                    }
                                }
                            }
                        }
                    }
                }

                // Models Section
                ControlGroup { title: "📦 Scene Models",
                    div { class: "mb-3 max-h-32 overflow-y-auto",
                        for (idx, model) in models().iter().enumerate() {
                            div {
                                class: if selection().is_selected(EntityId(idx)) {
                                    "flex items-center justify-between p-2 bg-yellow-600 rounded mb-1 cursor-pointer"
                                } else {
                                    "flex items-center justify-between p-2 bg-gray-700 rounded mb-1 cursor-pointer hover:bg-gray-600"
                                },
                                onclick: move |_| {
                                    selection.write().toggle(EntityId(idx));
                                },
                                div { class: "flex items-center gap-2",
                                    span { class: "text-sm font-medium", "{model.name}" }
                                    span { class: "text-xs text-gray-400", "({model.config.format.as_str()})" }
                                }
                                button {
                                    class: "text-red-400 hover:text-red-300 text-xs",
                                    onclick: move |e: Event<MouseData>| {
                                        e.stop_propagation();
                                        let mut current = models.write();
                                        if current.len() > 1 {
                                            current.remove(idx);
                                            selection.write().deselect(EntityId(idx));
                                        }
                                    },
                                    "✕"
                                }
                            }
                        }
                    }

                    // Add Model Form
                    div { class: "border-t border-gray-600 pt-3 mt-3",
                        h4 { class: "text-xs font-medium text-gray-400 mb-2", "Add New Model" }

                        input {
                            r#type: "text",
                            value: "{new_name()}",
                            placeholder: "Model name",
                            oninput: move |e| new_name.set(e.value()),
                            class: "w-full bg-gray-700 rounded px-2 py-1 text-sm mb-2"
                        }

                        select {
                            class: "w-full bg-gray-700 rounded px-2 py-1 text-sm mb-2",
                            onchange: move |e| {
                                new_format.set(match e.value().as_str() {
                                    "obj" => ModelFormat::Obj,
                                    "gltf" => ModelFormat::Gltf,
                                    "glb" => ModelFormat::Glb,
                                    "fbx" => ModelFormat::Fbx,
                                    "stl" => ModelFormat::Stl,
                                    _ => ModelFormat::Cube,
                                });
                            },
                            option { value: "cube", "Cube (Default)" }
                            option { value: "obj", "OBJ" }
                            option { value: "gltf", "glTF" }
                            option { value: "glb", "GLB" }
                            option { value: "fbx", "FBX" }
                            option { value: "stl", "STL" }
                        }

                        input {
                            r#type: "text",
                            value: "{new_url()}",
                            placeholder: "https://example.com/model.obj (optional)",
                            oninput: move |e| new_url.set(e.value()),
                            class: "w-full bg-gray-700 rounded px-2 py-1 text-sm mb-2"
                        }

                        div { class: "grid grid-cols-2 gap-1 mb-2",
                            for (name, url, fmt) in preset_models.clone().into_iter().take(4) {
                                button {
                                    class: "bg-gray-600 hover:bg-gray-500 px-2 py-1 rounded text-xs text-left transition",
                                    onclick: move |_| {
                                        new_name.set(name.to_string());
                                        new_url.set(url.to_string());
                                        new_format.set(fmt.clone());
                                    },
                                    "{name}"
                                }
                            }
                        }

                        button {
                            class: "w-full bg-green-600 hover:bg-green-700 px-3 py-2 rounded text-sm font-medium transition",
                            onclick: move |_| {
                                let id = next_id();
                                let name = if new_name().is_empty() {
                                    format!("Model {}", id)
                                } else {
                                    new_name()
                                };
                                let url = new_url();
                                let fmt = new_format();

                                models.write().push(SceneModel {
                                    id,
                                    name,
                                    config: ModelConfig::new(url, fmt).with_color("#ff6b6b"),
                                });
                                next_id += 1;
                                new_name.set("".to_string());
                                new_url.set("".to_string());
                            },
                            "➕ Add Model"
                        }
                    }
                }

                // Global Settings
                ControlGroup { title: "🌍 Global Settings",
                    div { class: "mb-3",
                        label { class: "block text-xs text-gray-400 mb-1", "Shader Effect" }
                        select {
                            class: "w-full bg-gray-700 rounded px-2 py-1 text-sm",
                            onchange: move |e| {
                                shader.set(match e.value().as_str() {
                                    "gradient" => ShaderPreset::Gradient,
                                    "water" => ShaderPreset::Water,
                                    "hologram" => ShaderPreset::Hologram,
                                    "toon" => ShaderPreset::Toon,
                                    "heatmap" => ShaderPreset::Heatmap,
                                    _ => ShaderPreset::None,
                                });
                            },
                            option { value: "none", "None (Standard)" }
                            option { value: "gradient", "🌈 Gradient" }
                            option { value: "water", "🌊 Water" }
                            option { value: "hologram", "✨ Hologram" }
                            option { value: "toon", "🎨 Toon" }
                            option { value: "heatmap", "🔥 Heatmap" }
                        }
                    }

                    Toggle { label: "Auto Rotate", value: auto_rotate }
                    if auto_rotate() {
                        Slider { label: "Speed", value: rot_speed, min: 0.0, max: 5.0, step: 0.1 }
                    }
                    Toggle { label: "Show Grid", value: show_grid }
                    Toggle { label: "Show Axes", value: show_axes }
                    Toggle { label: "Wireframe", value: wireframe }
                }

                // Camera Section
                ControlGroup { title: "📷 Camera",
                    Slider { label: "X", value: cam_x, min: -30.0, max: 30.0, step: 0.1 }
                    Slider { label: "Y", value: cam_y, min: -30.0, max: 30.0, step: 0.1 }
                    Slider { label: "Z", value: cam_z, min: -30.0, max: 30.0, step: 0.1 }

                    div { class: "grid grid-cols-3 gap-2 mt-2",
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 px-2 py-1 rounded text-xs transition",
                            onclick: move |_| { cam_x.set(0.0); cam_y.set(15.0); cam_z.set(0.01); },
                            "Top"
                        }
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 px-2 py-1 rounded text-xs transition",
                            onclick: move |_| { cam_x.set(15.0); cam_y.set(0.0); cam_z.set(0.0); },
                            "Side"
                        }
                        button {
                            class: "bg-green-600 hover:bg-green-700 px-2 py-1 rounded text-xs transition",
                            onclick: move |_| { cam_x.set(10.0); cam_y.set(10.0); cam_z.set(10.0); },
                            "Iso"
                        }
                    }
                }

                // Actions
                ControlGroup { title: "📍 Actions",
                    button {
                        class: "w-full bg-red-600 hover:bg-red-700 px-3 py-2 rounded text-sm font-medium transition mb-2",
                        onclick: move |_| {
                            models.set(vec![
                                SceneModel {
                                    id: 0,
                                    name: "Cube".to_string(),
                                    config: ModelConfig::new("", ModelFormat::Cube).with_color("#ff6b6b"),
                                }
                            ]);
                            selection.write().clear();
                            next_id.set(1);
                            cam_x.set(8.0); cam_y.set(8.0); cam_z.set(8.0);
                            auto_rotate.set(false);
                            shader.set(ShaderPreset::None);
                        },
                        "🗑️ Reset Scene"
                    }
                }

                // Info
                div { class: "mt-4 p-3 bg-gray-900 rounded text-xs text-gray-400",
                    p { "Dioxus Three v0.0.3" }
                    p { "Models: {models().len()} | Selected: {selection().count()}" }
                }
            }

            // Right - Three.js View
            div { class: "flex-1 relative bg-black",
                dioxus_three::ThreeView {
                    models: model_configs,
                    cam_x: cam_x(),
                    cam_y: cam_y(),
                    cam_z: cam_z(),
                    auto_rotate: auto_rotate(),
                    rot_speed: rot_speed(),
                    show_grid: show_grid(),
                    show_axes: show_axes(),
                    shader: shader(),
                    wireframe: wireframe(),

                    // Phase 1: Raycasting
                    raycast: dioxus_three::RaycastConfig::default(),

                    // Phase 1: Pointer events
                    on_pointer_down: move |event: dioxus_three::PointerEvent| {
                        if let Some(hit) = event.hit {
                            if event.shift_key {
                                selection.write().toggle(hit.entity_id);
                            } else {
                                selection.write().select(hit.entity_id);
                            }
                        } else if !event.shift_key {
                            selection.write().clear();
                        }
                    },

                    // Phase 1: Selection
                    selection: Some(selection()),
                    selection_mode: dioxus_three::SelectionMode::Multiple,
                    selection_style: dioxus_three::SelectionStyle::default(),

                    // Phase 1: Gizmo
                    gizmo: if show_gizmo() {
                        selection().primary().map(|id| {
                            Gizmo::new(id)
                                .with_mode(gizmo_mode())
                                .with_space(gizmo_space())
                        })
                    } else {
                        None
                    },

                    on_gizmo_drag: move |event: dioxus_three::GizmoEvent| {
                        println!("[DEMO] gizmo_drag - entity: {}, mode: {:?}, finished: {}, scale: {:?}",
                            event.target.0, event.mode, event.is_finished, event.transform.scale);
                        transform_overrides.write().insert(event.target.0, event.transform);
                        if event.is_finished {
                            println!("[DEMO] Gizmo finished: entity {} -> pos={:?}",
                                event.target.0, event.transform.position);
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn ControlGroup(title: String, children: Element) -> Element {
    rsx! {
        div { class: "bg-gray-700 rounded p-3 mb-3",
            h3 { class: "text-sm font-medium text-gray-300 mb-2", "{title}" }
            {children}
        }
    }
}

#[component]
fn Slider(label: String, value: Signal<f32>, min: f32, max: f32, step: f32) -> Element {
    rsx! {
        div { class: "flex items-center gap-2 mb-2",
            label { class: "w-12 text-xs text-gray-400", "{label}" }
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value()}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<f32>() {
                        value.set(v);
                    }
                },
                class: "flex-1 h-1.5 bg-gray-600 rounded-lg appearance-none cursor-pointer"
            }
            span { class: "w-12 text-right text-xs font-mono text-yellow-400",
                "{value():.1}"
            }
        }
    }
}

#[component]
fn Toggle(label: String, value: Signal<bool>) -> Element {
    rsx! {
        div { class: "flex items-center gap-3 mb-2",
            input {
                r#type: "checkbox",
                checked: "{value()}",
                onchange: move |e| value.set(e.checked()),
                class: "w-4 h-4 rounded"
            }
            label { class: "text-sm text-gray-300", "{label}" }
        }
    }
}

#[component]
fn GizmoModeButton(label: String, active: bool, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: if active {
                "bg-yellow-600 hover:bg-yellow-700 px-3 py-2 rounded text-sm font-medium transition"
            } else {
                "bg-gray-600 hover:bg-gray-500 px-3 py-2 rounded text-sm transition"
            },
            onclick: move |_| onclick.call(()),
            "{label}"
        }
    }
}
