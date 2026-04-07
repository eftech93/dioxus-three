//! Dioxus Three - Web Demo
//!
//! A web-optimized demo showing the ThreeView component with Dioxus Web.
//! Run with: `dx serve --platform web`

use dioxus::prelude::*;
use dioxus_three::{ModelConfig, ModelFormat, ShaderPreset};

/// Wrapper component that takes signals and subscribes to them properly
/// This ensures the ThreeView re-renders when any signal changes
#[component]
fn ThreeViewWrapper(
    models: Signal<Vec<SceneModel>>,
    cam_x: Signal<f32>,
    cam_y: Signal<f32>,
    cam_z: Signal<f32>,
    auto_rotate: Signal<bool>,
    rot_speed: Signal<f32>,
    show_grid: Signal<bool>,
    show_axes: Signal<bool>,
    shader: Signal<ShaderPreset>,
) -> Element {
    // Build model configs from signal - this will re-run when models changes
    let model_configs: Vec<ModelConfig> = models.read().iter().map(|m| m.config.clone()).collect();
    
    // Reading all signals here ensures this component re-renders when they change
    let cx = cam_x();
    let cy = cam_y();
    let cz = cam_z();
    let ar = auto_rotate();
    let rs = rot_speed();
    let sg = show_grid();
    let sa = show_axes();
    let _sh = shader();
    
    rsx! {
        dioxus_three::ThreeView {
            models: model_configs,
            cam_x: cx,
            cam_y: cy,
            cam_z: cz,
            auto_rotate: ar,
            rot_speed: rs,
            show_grid: sg,
            show_axes: sa,
            shader: _sh,
        }
    }
}

fn main() {
    // Initialize console error panic hook for better debugging
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    launch(app);
}

fn app() -> Element {
    // Scene state
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
    let mut sidebar_open = use_signal(|| true);

    rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" }

        div { class: "flex flex-col h-screen w-screen bg-gray-900 text-white font-sans overflow-hidden",
            // Header
            header { class: "bg-gray-800 p-3 flex items-center justify-between shadow-lg flex-shrink-0 h-14",
                h1 { class: "text-lg font-bold text-yellow-500", "🎮 Dioxus Three - Web Demo" }
                button {
                    class: "md:hidden bg-gray-700 hover:bg-gray-600 px-3 py-1 rounded text-sm",
                    onclick: move |_| sidebar_open.set(!sidebar_open()),
                    if sidebar_open() { "✕ Hide" } else { "☰ Show" }
                }
            }

            // Main content - side by side layout
            div { class: "flex flex-1 overflow-hidden w-full",
                // LEFT: Control Panel - Fixed width, always visible on desktop
                aside {
                    class: if sidebar_open() {
                        "w-80 flex-shrink-0 bg-gray-800 p-4 overflow-y-auto shadow-xl h-full"
                    } else {
                        "hidden w-80 flex-shrink-0 bg-gray-800 p-4 overflow-y-auto shadow-xl h-full md:block"
                    },
                    // Models Section
                    ControlGroup { title: "📦 Scene Models",
                        // Model list
                        div { class: "mb-3 max-h-32 overflow-y-auto",
                            for (idx, model) in models().iter().enumerate() {
                                div {
                                    class: if idx == selected_model() {
                                        "flex items-center justify-between p-2 bg-yellow-600 rounded mb-1 cursor-pointer"
                                    } else {
                                        "flex items-center justify-between p-2 bg-gray-700 rounded mb-1 cursor-pointer hover:bg-gray-600"
                                    },
                                    onclick: move |_| selected_model.set(idx),
                                    div { class: "flex items-center gap-2",
                                        span { class: "text-sm font-medium", "{model.name}" }
                                        span { class: "text-xs text-gray-300", "({model.config.format.as_str()})" }
                                    }
                                    button {
                                        class: "text-red-300 hover:text-red-200 text-xs",
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

                        // Quick preset models
                        div { class: "grid grid-cols-2 gap-2",
                            PresetButton { name: "Cube", format: ModelFormat::Cube, url: "", models, next_id, selected_model }
                            PresetButton { name: "Male (OBJ)", format: ModelFormat::Obj, url: "https://threejs.org/examples/models/obj/male02/male02.obj", models, next_id, selected_model }
                            PresetButton { name: "Helmet (glTF)", format: ModelFormat::Gltf, url: "https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf", models, next_id, selected_model }
                            PresetButton { name: "Duck (glTF)", format: ModelFormat::Gltf, url: "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Duck/glTF/Duck.gltf", models, next_id, selected_model }
                        }
                    }

                    // Selected Model Controls
                    SelectedModelControls { models, selected_idx: selected_model() }

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
                    }

                    // Camera Section
                    ControlGroup { title: "📷 Camera",
                        Slider { label: "X", value: cam_x, min: -30.0, max: 30.0, step: 0.1 }
                        Slider { label: "Y", value: cam_y, min: -30.0, max: 30.0, step: 0.1 }
                        Slider { label: "Z", value: cam_z, min: -30.0, max: 30.0, step: 0.1 }

                        div { class: "grid grid-cols-3 gap-2 mt-2",
                            button {
                                class: "bg-blue-600 hover:bg-blue-700 px-2 py-1 rounded text-xs",
                                onclick: move |_| { cam_x.set(0.0); cam_y.set(15.0); cam_z.set(0.01); },
                                "Top"
                            }
                            button {
                                class: "bg-blue-600 hover:bg-blue-700 px-2 py-1 rounded text-xs",
                                onclick: move |_| { cam_x.set(15.0); cam_y.set(0.0); cam_z.set(0.0); },
                                "Side"
                            }
                            button {
                                class: "bg-green-600 hover:bg-green-700 px-2 py-1 rounded text-xs",
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
                                        config: ModelConfig::new("", ModelFormat::Cube)
                                            .with_color("#ff6b6b"),
                                    }
                                ]);
                                selected_model.set(0);
                                cam_x.set(8.0); cam_y.set(8.0); cam_z.set(8.0);
                                auto_rotate.set(true);
                                shader.set(ShaderPreset::None);
                            },
                            "🗑️ Reset Scene"
                        }
                    }

                    // Info
                    div { class: "mt-4 p-3 bg-gray-900 rounded text-xs text-gray-400",
                        p { "Dioxus Three - Web Demo" }
                        p { "Models: {models().len()}" }
                    }
                }

                // RIGHT: 3D Renderer (Three.js Canvas) - Takes remaining space
                main { class: "flex-1 relative bg-black h-full w-full min-w-0",
                    div { class: "absolute inset-0 w-full h-full",
                        // Key: Pass signals directly so component subscribes to them
                        ThreeViewWrapper {
                            models: models,
                            cam_x: cam_x,
                            cam_y: cam_y,
                            cam_z: cam_z,
                            auto_rotate: auto_rotate,
                            rot_speed: rot_speed,
                            show_grid: show_grid,
                            show_axes: show_axes,
                            shader: shader,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct SceneModel {
    id: usize,
    name: String,
    config: ModelConfig,
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
fn PresetButton(
    name: String,
    format: ModelFormat,
    url: String,
    models: Signal<Vec<SceneModel>>,
    next_id: Signal<usize>,
    selected_model: Signal<usize>,
) -> Element {
    rsx! {
        button {
            class: "bg-gray-600 hover:bg-gray-500 px-2 py-2 rounded text-xs text-left transition",
            onclick: move |_| {
                let id = next_id();
                let config = ModelConfig::new(url.clone(), format.clone())
                    .with_color("#ff6b6b");

                models.write().push(SceneModel {
                    id,
                    name: name.clone(),
                    config,
                });
                next_id += 1;
                selected_model.set(models().len() - 1);
            },
            "{name}"
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

    let title = format!("🔧 {}", model_name);

    rsx! {
        ControlGroup { title: title,
            // Position
            div { class: "mb-2",
                label { class: "block text-xs text-gray-400 mb-1", "Position" }
                div { class: "grid grid-cols-3 gap-2",
                    NumberInput {
                        value: pos_x,
                        onchange: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.pos_x = v;
                            }
                        }
                    }
                    NumberInput {
                        value: pos_y,
                        onchange: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.pos_y = v;
                            }
                        }
                    }
                    NumberInput {
                        value: pos_z,
                        onchange: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.pos_z = v;
                            }
                        }
                    }
                }
            }

            // Rotation
            div { class: "mb-2",
                label { class: "block text-xs text-gray-400 mb-1", "Rotation (°)" }
                div { class: "grid grid-cols-3 gap-2",
                    NumberInput {
                        value: rot_x,
                        onchange: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.rot_x = v;
                            }
                        }
                    }
                    NumberInput {
                        value: rot_y,
                        onchange: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.rot_y = v;
                            }
                        }
                    }
                    NumberInput {
                        value: rot_z,
                        onchange: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.rot_z = v;
                            }
                        }
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

#[component]
fn NumberInput(value: f32, onchange: EventHandler<f32>) -> Element {
    rsx! {
        input {
            r#type: "number",
            value: "{value:.1}",
            step: "0.1",
            onchange: move |e| {
                if let Ok(v) = e.value().parse::<f32>() {
                    onchange.call(v);
                }
            },
            class: "w-full bg-gray-600 rounded px-2 py-1 text-sm"
        }
    }
}
