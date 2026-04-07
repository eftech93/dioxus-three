//! Dioxus Three Demo - Multi-Model Support
//!
//! A demo showing the ThreeView component with multi-model support
//! and independent controls for each model.

use dioxus::prelude::*;
use dioxus_three::{ModelConfig, ModelFormat, ShaderPreset};

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
    // Scene models list
    let mut models = use_signal(|| {
        vec![SceneModel {
            id: 0,
            name: "Cube".to_string(),
            config: ModelConfig::new("", ModelFormat::Cube).with_color("#ff6b6b"),
        }]
    });
    let mut next_id = use_signal(|| 1usize);
    let mut selected_model = use_signal(|| 0usize);

    // Camera state
    let mut cam_x = use_signal(|| 8.0f32);
    let mut cam_y = use_signal(|| 8.0f32);
    let mut cam_z = use_signal(|| 8.0f32);

    // Global options
    let mut auto_rotate = use_signal(|| true);
    let mut rot_speed = use_signal(|| 1.0f32);
    let mut show_grid = use_signal(|| true);
    let mut show_axes = use_signal(|| true);
    let mut shader = use_signal(|| ShaderPreset::None);

    // New model form
    let mut new_url = use_signal(|| "".to_string());
    let mut new_format = use_signal(|| ModelFormat::Obj);
    let mut new_name = use_signal(|| "".to_string());

    // Preset models
    let preset_models = vec![
        ("Cube", "", ModelFormat::Cube),
        ("Male Character (OBJ)", "https://threejs.org/examples/models/obj/male02/male02.obj", ModelFormat::Obj),
        ("Damaged Helmet (glTF)", "https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf", ModelFormat::Gltf),
        ("Duck (glTF)", "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Duck/glTF/Duck.gltf", ModelFormat::Gltf),
    ];

    // Build ModelConfig list for ThreeView
    let model_configs: Vec<ModelConfig> = models.read().iter().map(|m| m.config.clone()).collect();

    rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" }

        div { class: "flex h-screen bg-gray-900 text-white font-sans overflow-hidden",
            // Left Control Panel
            div { class: "w-[450px] bg-gray-800 p-4 overflow-y-auto shadow-2xl flex-shrink-0",
                h1 { class: "text-xl font-bold mb-4 text-yellow-500", "🎮 Multi-Model 3D Viewer" }

                // Models List Section
                ControlGroup { title: "📦 Scene Models",
                    // Model list
                    div { class: "mb-3 max-h-40 overflow-y-auto",
                        for (idx, model) in models().iter().enumerate() {
                            div {
                                class: if idx == selected_model() { "flex items-center justify-between p-2 bg-yellow-600 rounded mb-1 cursor-pointer" } else { "flex items-center justify-between p-2 bg-gray-700 rounded mb-1 cursor-pointer hover:bg-gray-600" },
                                onclick: move |_| selected_model.set(idx),
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
                                            if selected_model() >= current.len() {
                                                selected_model.set(current.len() - 1);
                                            }
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
                                    "fbx" => ModelFormat::Fbx,
                                    "gltf" => ModelFormat::Gltf,
                                    "glb" => ModelFormat::Glb,
                                    "stl" => ModelFormat::Stl,
                                    "ply" => ModelFormat::Ply,
                                    "dae" => ModelFormat::Dae,
                                    _ => ModelFormat::Cube,
                                });
                            },
                            option { value: "cube", "Cube (Default)" }
                            option { value: "obj", "OBJ (Wavefront)" }
                            option { value: "fbx", "FBX (Autodesk)" }
                            option { value: "gltf", "glTF 2.0" }
                            option { value: "glb", "GLB (Binary)" }
                            option { value: "stl", "STL" }
                            option { value: "ply", "PLY (Stanford)" }
                            option { value: "dae", "Collada (DAE)" }
                        }

                        input {
                            r#type: "text",
                            value: "{new_url()}",
                            placeholder: "https://example.com/model.obj (optional for presets)",
                            oninput: move |e| new_url.set(e.value()),
                            class: "w-full bg-gray-700 rounded px-2 py-1 text-sm mb-2"
                        }

                        // Quick preset buttons
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
                                let url = new_url();
                                let name = if new_name().is_empty() {
                                    format!("Model {}", next_id())
                                } else {
                                    new_name()
                                };
                                let config = ModelConfig::new(url.clone(), new_format())
                                    .with_color("#ff6b6b");

                                models.write().push(SceneModel {
                                    id: next_id(),
                                    name,
                                    config,
                                });
                                next_id += 1;
                                selected_model.set(models().len() - 1);
                                new_url.set("".to_string());
                                new_name.set("".to_string());
                            },
                            "➕ Add Model to Scene"
                        }
                    }
                }

                // Selected Model Transform Section
                SelectedModelControls {
                    models: models,
                    selected_idx: selected_model(),
                }

                // Global Settings
                ControlGroup { title: "🌍 Global Settings",
                    // Shader selector
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
                            option { value: "none", "None (Standard Material)" }
                            option { value: "gradient", "🌈 Animated Gradient" }
                            option { value: "water", "🌊 Water/Waves" }
                            option { value: "hologram", "✨ Hologram" }
                            option { value: "toon", "🎨 Toon/Cel Shading" }
                            option { value: "heatmap", "🔥 Heat Map" }
                        }
                    }

                    Toggle { label: "Auto Rotate All", value: auto_rotate }
                    if auto_rotate() {
                        div { class: "mb-2",
                            label { class: "w-12 text-xs text-gray-400", "Speed" }
                            input {
                                r#type: "range",
                                min: "0",
                                max: "5",
                                step: "0.1",
                                value: "{rot_speed()}",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<f32>() {
                                        rot_speed.set(v);
                                    }
                                },
                                class: "flex-1 h-1.5 bg-gray-600 rounded-lg appearance-none cursor-pointer"
                            }
                        }
                    }

                    Toggle { label: "Show Grid", value: show_grid }
                    Toggle { label: "Show Axes", value: show_axes }
                }

                // Camera Section
                ControlGroup { title: "📷 Camera",
                    Slider { label: "Cam X", value: cam_x, min: -30.0, max: 30.0, step: 0.1 }
                    Slider { label: "Cam Y", value: cam_y, min: -30.0, max: 30.0, step: 0.1 }
                    Slider { label: "Cam Z", value: cam_z, min: -30.0, max: 30.0, step: 0.1 }

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
                ControlGroup { title: "📍 Scene Actions",
                    button {
                        class: "w-full bg-red-600 hover:bg-red-700 px-3 py-2 rounded text-sm font-medium transition mb-2",
                        onclick: move |_| {
                            models.set(vec![
                                SceneModel {
                                    id: 0,
                                    name: "Cube".to_string(),
                                    config: ModelConfig::new("", ModelFormat::Cube)
                                        .with_color("#ff6b6b"),
                                }
                            ]);
                            selected_model.set(0);
                            cam_x.set(8.0); cam_y.set(8.0); cam_z.set(8.0);
                            auto_rotate.set(true);
                            shader.set(ShaderPreset::None);
                        },
                        "🗑️ Clear Scene"
                    }
                }

                // Info
                div { class: "mt-4 p-3 bg-gray-900 rounded text-xs text-gray-400",
                    p { "Dioxus Three - Multi-Model 3D Viewer" }
                    p { "Models in scene: {models().len()}" }
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
fn SelectedModelControls(models: Signal<Vec<SceneModel>>, selected_idx: usize) -> Element {
    let model = models.read().get(selected_idx).cloned();

    let model_name = model.as_ref().map(|m| m.name.clone()).unwrap_or_default();
    let pos_x = model.as_ref().map(|m| m.config.pos_x).unwrap_or(0.0);
    let pos_y = model.as_ref().map(|m| m.config.pos_y).unwrap_or(0.0);
    let pos_z = model.as_ref().map(|m| m.config.pos_z).unwrap_or(0.0);
    let rot_x = model.as_ref().map(|m| m.config.rot_x).unwrap_or(0.0);
    let rot_y = model.as_ref().map(|m| m.config.rot_y).unwrap_or(0.0);
    let rot_z = model.as_ref().map(|m| m.config.rot_z).unwrap_or(0.0);
    let scale = model.as_ref().map(|m| m.config.scale).unwrap_or(1.0);
    let color = model
        .as_ref()
        .map(|m| m.config.color.clone())
        .unwrap_or_else(|| "#ff6b6b".to_string());

    let title = format!("🔧 Transform: {}", model_name);

    rsx! {
        ControlGroup { title: title,
            // Position
            div { class: "mb-2",
                label { class: "block text-xs text-gray-400 mb-1", "Position" }
                div { class: "grid grid-cols-3 gap-2",
                    input {
                        r#type: "number",
                        value: "{pos_x}",
                        step: "0.1",
                        onchange: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                let mut m = models.write();
                                if let Some(model) = m.get_mut(selected_idx) {
                                    model.config.pos_x = v;
                                }
                            }
                        },
                        class: "w-full bg-gray-600 rounded px-2 py-1 text-sm"
                    }
                    input {
                        r#type: "number",
                        value: "{pos_y}",
                        step: "0.1",
                        onchange: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                let mut m = models.write();
                                if let Some(model) = m.get_mut(selected_idx) {
                                    model.config.pos_y = v;
                                }
                            }
                        },
                        class: "w-full bg-gray-600 rounded px-2 py-1 text-sm"
                    }
                    input {
                        r#type: "number",
                        value: "{pos_z}",
                        step: "0.1",
                        onchange: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                let mut m = models.write();
                                if let Some(model) = m.get_mut(selected_idx) {
                                    model.config.pos_z = v;
                                }
                            }
                        },
                        class: "w-full bg-gray-600 rounded px-2 py-1 text-sm"
                    }
                }
            }

            // Rotation
            div { class: "mb-2",
                label { class: "block text-xs text-gray-400 mb-1", "Rotation (degrees)" }
                div { class: "grid grid-cols-3 gap-2",
                    input {
                        r#type: "number",
                        value: "{rot_x:.0}",
                        step: "1",
                        onchange: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                let mut m = models.write();
                                if let Some(model) = m.get_mut(selected_idx) {
                                    model.config.rot_x = v;
                                }
                            }
                        },
                        class: "w-full bg-gray-600 rounded px-2 py-1 text-sm"
                    }
                    input {
                        r#type: "number",
                        value: "{rot_y:.0}",
                        step: "1",
                        onchange: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                let mut m = models.write();
                                if let Some(model) = m.get_mut(selected_idx) {
                                    model.config.rot_y = v;
                                }
                            }
                        },
                        class: "w-full bg-gray-600 rounded px-2 py-1 text-sm"
                    }
                    input {
                        r#type: "number",
                        value: "{rot_z:.0}",
                        step: "1",
                        onchange: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                let mut m = models.write();
                                if let Some(model) = m.get_mut(selected_idx) {
                                    model.config.rot_z = v;
                                }
                            }
                        },
                        class: "w-full bg-gray-600 rounded px-2 py-1 text-sm"
                    }
                }
            }

            // Scale
            div { class: "mb-2",
                label { class: "block text-xs text-gray-400 mb-1", "Scale: {scale:.2}" }
                input {
                    r#type: "range",
                    min: "0.1",
                    max: "5.0",
                    step: "0.1",
                    value: "{scale}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f32>() {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.scale = v;
                            }
                        }
                    },
                    class: "w-full h-1.5 bg-gray-600 rounded-lg appearance-none cursor-pointer"
                }
            }

            // Color
            div { class: "flex items-center gap-3",
                label { class: "text-xs text-gray-400", "Color" }
                input {
                    r#type: "color",
                    value: "{color}",
                    oninput: move |e| {
                        let c = e.value();
                        let mut m = models.write();
                        if let Some(model) = m.get_mut(selected_idx) {
                            model.config.color = c;
                        }
                    },
                    class: "w-12 h-8 rounded cursor-pointer"
                }
                span { class: "text-sm text-gray-400", "{color}" }
            }
        }
    }
}
