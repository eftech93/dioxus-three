//! Dioxus Three - Mobile Demo
//!
//! A mobile-optimized demo showing the ThreeView component with Dioxus Mobile.
//! Supports iOS and Android platforms.

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
    shader: Signal<ShaderPreset>,
) -> Element {
    // Build model configs from signal - this will re-run when models changes
    let model_configs: Vec<ModelConfig> = models.read().iter().map(|m| m.config.clone()).collect();
    
    // Reading all signals here ensures this component re-renders when they change
    let cx = cam_x();
    let cy = cam_y();
    let cz = cam_z();
    let ar = auto_rotate();
    let sh = shader();
    
    rsx! {
        dioxus_three::ThreeView {
            models: model_configs,
            cam_x: cx,
            cam_y: cy,
            cam_z: cz,
            auto_rotate: ar,
            rot_speed: 1.0,
            show_grid: true,
            show_axes: true,
            shader: sh,
        }
    }
}

fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug),
    );

    dioxus::mobile::launch(app);
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
    let mut shader = use_signal(|| ShaderPreset::None);
    let mut show_controls = use_signal(|| false);

    rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" }

        div { class: "flex flex-col h-screen bg-gray-900 text-white overflow-hidden",
            // Header
            header { class: "bg-gray-800 p-3 flex items-center justify-between shadow-lg z-20",
                h1 { class: "text-base font-bold text-yellow-500", "🎮 Dioxus 3D" }
                button {
                    class: "bg-gray-700 hover:bg-gray-600 px-3 py-1 rounded text-sm",
                    onclick: move |_| show_controls.set(!show_controls()),
                    if show_controls() { "✕ Close" } else { "☰ Controls" }
                }
            }

            // Main content - 3D View takes full screen
            div { class: "flex-1 relative bg-black",
                ThreeViewWrapper {
                    models: models,
                    cam_x: cam_x,
                    cam_y: cam_y,
                    cam_z: cam_z,
                    auto_rotate: auto_rotate,
                    shader: shader,
                }

                // Floating quick controls (when main controls are hidden)
                if !show_controls() {
                    div { class: "absolute bottom-4 left-4 right-4 flex justify-center gap-2",
                        QuickButton { 
                            icon: "⟲", 
                            onclick: move |_| auto_rotate.set(!auto_rotate()),
                            active: auto_rotate()
                        }
                        QuickButton { 
                            icon: "🎨", 
                            onclick: move |_| {
                                shader.set(match shader() {
                                    ShaderPreset::None => ShaderPreset::Gradient,
                                    ShaderPreset::Gradient => ShaderPreset::Water,
                                    ShaderPreset::Water => ShaderPreset::Hologram,
                                    ShaderPreset::Hologram => ShaderPreset::Toon,
                                    ShaderPreset::Toon => ShaderPreset::Heatmap,
                                    _ => ShaderPreset::None,
                                });
                            },
                            active: shader() != ShaderPreset::None
                        }
                        QuickButton { 
                            icon: "📷", 
                            onclick: move |_| { 
                                cam_x.set(10.0); cam_y.set(10.0); cam_z.set(10.0); 
                            },
                            active: false
                        }
                    }
                }
            }

            // Bottom sheet controls
            if show_controls() {
                div { class: "absolute inset-0 bg-black/50 z-10",
                    onclick: move |_| show_controls.set(false),
                }
                
                div { class: "absolute bottom-0 left-0 right-0 bg-gray-800 rounded-t-2xl shadow-2xl z-20 max-h-[70vh] overflow-y-auto",
                    // Drag handle
                    div { class: "flex justify-center pt-2 pb-1",
                        div { class: "w-10 h-1 bg-gray-600 rounded-full" }
                    }

                    div { class: "p-4",
                        // Models Section
                        ControlSection { title: "📦 Models",
                            // Model list
                            div { class: "flex gap-2 overflow-x-auto pb-2 mb-3",
                                for (idx, model) in models().iter().enumerate() {
                                    ModelCard { 
                                        name: model.name.clone(),
                                        format: model.config.format.as_str().to_string(),
                                        selected: idx == selected_model(),
                                        onclick: move |_| selected_model.set(idx)
                                    }
                                }
                            }

                            // Add model buttons
                            div { class: "grid grid-cols-2 gap-2",
                                AddModelButton { name: "Cube", format: ModelFormat::Cube, url: "", models, next_id, selected_model }
                                AddModelButton { name: "Helmet", format: ModelFormat::Gltf, url: "https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf", models, next_id, selected_model }
                                AddModelButton { name: "Duck", format: ModelFormat::Gltf, url: "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Duck/glTF/Duck.gltf", models, next_id, selected_model }
                                AddModelButton { name: "Male", format: ModelFormat::Obj, url: "https://threejs.org/examples/models/obj/male02/male02.obj", models, next_id, selected_model }
                            }
                        }

                        // Selected Model Controls
                        if let Some(model) = models.read().get(selected_model()).cloned() {
                            ControlSection { title: "🔧 Transform",
                                // Position
                                div { class: "mb-3",
                                    label { class: "block text-xs text-gray-400 mb-1", "Position" }
                                    Vec3Input {
                                        x: model.config.pos_x,
                                        y: model.config.pos_y,
                                        z: model.config.pos_z,
                                        onchange: move |(x, y, z)| {
                                            let mut m = models.write();
                                            if let Some(model) = m.get_mut(selected_model()) {
                                                model.config.pos_x = x;
                                                model.config.pos_y = y;
                                                model.config.pos_z = z;
                                            }
                                        }
                                    }
                                }

                                // Rotation
                                div { class: "mb-3",
                                    label { class: "block text-xs text-gray-400 mb-1", "Rotation" }
                                    Vec3Input {
                                        x: model.config.rot_x,
                                        y: model.config.rot_y,
                                        z: model.config.rot_z,
                                        onchange: move |(x, y, z)| {
                                            let mut m = models.write();
                                            if let Some(model) = m.get_mut(selected_model()) {
                                                model.config.rot_x = x;
                                                model.config.rot_y = y;
                                                model.config.rot_z = z;
                                            }
                                        }
                                    }
                                }

                                // Scale
                                div { class: "mb-3",
                                    label { class: "block text-xs text-gray-400 mb-1", 
                                        "Scale: {model.config.scale:.2}"
                                    }
                                    input {
                                        r#type: "range",
                                        min: "0.1",
                                        max: "3.0",
                                        step: "0.1",
                                        value: "{model.config.scale}",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse::<f32>() {
                                                let mut m = models.write();
                                                if let Some(model) = m.get_mut(selected_model()) {
                                                    model.config.scale = v;
                                                }
                                            }
                                        },
                                        class: "w-full h-2 bg-gray-600 rounded-lg appearance-none"
                                    }
                                }

                                // Color
                                div { class: "flex items-center gap-3",
                                    label { class: "text-xs text-gray-400", "Color" }
                                    input {
                                        r#type: "color",
                                        value: "{model.config.color}",
                                        oninput: move |e| {
                                            let c = e.value();
                                            let mut m = models.write();
                                            if let Some(model) = m.get_mut(selected_model()) {
                                                model.config.color = c;
                                            }
                                        },
                                        class: "w-12 h-10 rounded cursor-pointer"
                                    }
                                }
                            }
                        }

                        // Shader Section
                        ControlSection { title: "🎨 Shader",
                            div { class: "grid grid-cols-3 gap-2",
                                ShaderButton { name: "None", preset: ShaderPreset::None, shader }
                                ShaderButton { name: "Gradient", preset: ShaderPreset::Gradient, shader }
                                ShaderButton { name: "Water", preset: ShaderPreset::Water, shader }
                                ShaderButton { name: "Hologram", preset: ShaderPreset::Hologram, shader }
                                ShaderButton { name: "Toon", preset: ShaderPreset::Toon, shader }
                                ShaderButton { name: "Heatmap", preset: ShaderPreset::Heatmap, shader }
                            }
                        }

                        // Camera Section
                        ControlSection { title: "📷 Camera",
                            div { class: "grid grid-cols-3 gap-2",
                                CameraButton { name: "Top", x: 0.0, y: 15.0, z: 0.01, cam_x, cam_y, cam_z }
                                CameraButton { name: "Side", x: 15.0, y: 0.0, z: 0.0, cam_x, cam_y, cam_z }
                                CameraButton { name: "Iso", x: 10.0, y: 10.0, z: 10.0, cam_x, cam_y, cam_z }
                            }
                        }

                        // Actions
                        button {
                            class: "w-full bg-red-600 hover:bg-red-700 px-4 py-3 rounded-lg text-sm font-medium transition",
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
fn ControlSection(title: String, children: Element) -> Element {
    rsx! {
        div { class: "bg-gray-700/50 rounded-lg p-3 mb-3",
            h3 { class: "text-sm font-medium text-gray-300 mb-2", "{title}" }
            {children}
        }
    }
}

#[component]
fn QuickButton(icon: String, onclick: EventHandler<MouseEvent>, active: bool) -> Element {
    rsx! {
        button {
            class: if active {
                "w-12 h-12 bg-yellow-600 rounded-full text-xl shadow-lg flex items-center justify-center"
            } else {
                "w-12 h-12 bg-gray-800/80 rounded-full text-xl shadow-lg flex items-center justify-center"
            },
            onclick: move |e| onclick.call(e),
            "{icon}"
        }
    }
}

#[component]
fn ModelCard(name: String, format: String, selected: bool, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div {
            class: if selected {
                "flex-shrink-0 p-3 bg-yellow-600 rounded-lg min-w-[80px] cursor-pointer"
            } else {
                "flex-shrink-0 p-3 bg-gray-700 rounded-lg min-w-[80px] cursor-pointer"
            },
            onclick: move |e| onclick.call(e),
            p { class: "text-sm font-medium truncate", "{name}" }
            p { class: "text-xs text-gray-300", "{format}" }
        }
    }
}

#[component]
fn AddModelButton(
    name: String,
    format: ModelFormat,
    url: String,
    models: Signal<Vec<SceneModel>>,
    next_id: Signal<usize>,
    selected_model: Signal<usize>,
) -> Element {
    rsx! {
        button {
            class: "bg-gray-600 hover:bg-gray-500 px-3 py-2 rounded text-sm transition",
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
            "➕ {name}"
        }
    }
}

#[component]
fn Vec3Input(x: f32, y: f32, z: f32, onchange: EventHandler<(f32, f32, f32)>) -> Element {
    rsx! {
        div { class: "grid grid-cols-3 gap-2",
            input {
                r#type: "number",
                value: "{x:.1}",
                step: "0.1",
                onchange: move |e| {
                    if let Ok(v) = e.value().parse::<f32>() {
                        onchange.call((v, y, z));
                    }
                },
                class: "w-full bg-gray-600 rounded px-2 py-2 text-sm"
            }
            input {
                r#type: "number",
                value: "{y:.1}",
                step: "0.1",
                onchange: move |e| {
                    if let Ok(v) = e.value().parse::<f32>() {
                        onchange.call((x, v, z));
                    }
                },
                class: "w-full bg-gray-600 rounded px-2 py-2 text-sm"
            }
            input {
                r#type: "number",
                value: "{z:.1}",
                step: "0.1",
                onchange: move |e| {
                    if let Ok(v) = e.value().parse::<f32>() {
                        onchange.call((x, y, v));
                    }
                },
                class: "w-full bg-gray-600 rounded px-2 py-2 text-sm"
            }
        }
    }
}

#[component]
fn ShaderButton(name: String, preset: ShaderPreset, shader: Signal<ShaderPreset>) -> Element {
    let is_active = shader() == preset;
    rsx! {
        button {
            class: if is_active {
                "bg-yellow-600 hover:bg-yellow-700 px-2 py-2 rounded text-xs font-medium transition"
            } else {
                "bg-gray-600 hover:bg-gray-500 px-2 py-2 rounded text-xs transition"
            },
            onclick: move |_| {
                shader.set(preset.clone());
            },
            "{name}"
        }
    }
}

#[component]
fn CameraButton(
    name: String,
    x: f32,
    y: f32,
    z: f32,
    cam_x: Signal<f32>,
    cam_y: Signal<f32>,
    cam_z: Signal<f32>,
) -> Element {
    rsx! {
        button {
            class: "bg-blue-600 hover:bg-blue-700 px-2 py-2 rounded text-xs transition",
            onclick: move |_| {
                cam_x.set(x);
                cam_y.set(y);
                cam_z.set(z);
            },
            "{name}"
        }
    }
}
