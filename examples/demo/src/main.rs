//! Dioxus Three Demo
//! 
//! A demo showing the ThreeView component with interactive controls
//! and support for loading various 3D model formats.

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder, LogicalSize};
use dioxus_three::{ThreeView, ModelFormat, ShaderPreset};

fn main() {
    tracing_subscriber::fmt::init();
    
    dioxus_desktop::launch::launch_virtual_dom_blocking(
        VirtualDom::new(app),
        Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_title("Dioxus Three Demo - 3D Model Viewer")
                    .with_inner_size(LogicalSize::new(1400, 900))
            )
    );
}

fn app() -> Element {
    // Model URL
    let mut model_url = use_signal(|| "".to_string());
    let mut format = use_signal(|| ModelFormat::Cube);
    
    // Transform state
    let mut pos_x = use_signal(|| 0.0f32);
    let mut pos_y = use_signal(|| 0.0f32);
    let mut pos_z = use_signal(|| 0.0f32);
    let mut rot_x = use_signal(|| 0.0f32);
    let mut rot_y = use_signal(|| 0.0f32);
    let mut rot_z = use_signal(|| 0.0f32);
    let mut scale = use_signal(|| 1.0f32);
    let mut color = use_signal(|| "#ff6b6b".to_string());
    
    // Camera state
    let mut cam_x = use_signal(|| 5.0f32);
    let mut cam_y = use_signal(|| 5.0f32);
    let mut cam_z = use_signal(|| 5.0f32);
    
    // Options
    let mut auto_rotate = use_signal(|| true);
    let mut rot_speed = use_signal(|| 1.0f32);
    let mut auto_center = use_signal(|| true);
    let mut auto_scale = use_signal(|| false);
    let mut show_grid = use_signal(|| true);
    let mut show_axes = use_signal(|| true);
    let mut wireframe = use_signal(|| false);
    let mut shader = use_signal(|| ShaderPreset::None);
    
    // Preset model URLs - verified working models from Three.js examples
    let preset_models = vec![
        ("Cube (Default)", "", ModelFormat::Cube),
        ("Male Character (OBJ)", "https://threejs.org/examples/models/obj/male02/male02.obj", ModelFormat::Obj),
        ("Damaged Helmet (glTF)", "https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf", ModelFormat::Gltf),
        ("Duck (glTF)", "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Duck/glTF/Duck.gltf", ModelFormat::Gltf),
        ("Samba Dancing (FBX)", "https://threejs.org/examples/models/fbx/Samba%20Dancing.fbx", ModelFormat::Fbx),
        ("Slotted Disk (STL)", "https://threejs.org/examples/models/stl/ascii/slotted_disk.stl", ModelFormat::Stl),
        ("Dolphins (PLY)", "https://threejs.org/examples/models/ply/ascii/dolphins.ply", ModelFormat::Ply),
        ("ABB Robot (DAE)", "https://threejs.org/examples/models/collada/abb_irb52_7_120.dae", ModelFormat::Dae),
    ];
    
    rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" }
        
        div { class: "flex h-screen bg-gray-900 text-white font-sans overflow-hidden",
            // Left Control Panel
            div { class: "w-96 bg-gray-800 p-4 overflow-y-auto shadow-2xl flex-shrink-0",
                h1 { class: "text-xl font-bold mb-4 text-pink-500", "🎮 3D Model Viewer" }
                
                // Model Loading Section
                ControlGroup { title: "📁 Model",
                    // Format selector
                    div { class: "mb-3",
                        label { class: "block text-xs text-gray-400 mb-1", "Format" }
                        select {
                            class: "w-full bg-gray-700 rounded px-2 py-1 text-sm",
                            value: "{format().as_str()}",
                            onchange: move |e| {
                                let new_format = match e.value().as_str() {
                                    "obj" => ModelFormat::Obj,
                                    "fbx" => ModelFormat::Fbx,
                                    "gltf" => ModelFormat::Gltf,
                                    "glb" => ModelFormat::Glb,
                                    "stl" => ModelFormat::Stl,
                                    "ply" => ModelFormat::Ply,
                                    "dae" => ModelFormat::Dae,
                                    _ => ModelFormat::Cube,
                                };
                                format.set(new_format);
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
                    }
                    
                    // URL input
                    div { class: "mb-3",
                        label { class: "block text-xs text-gray-400 mb-1", "Model URL" }
                        input {
                            r#type: "text",
                            value: "{model_url()}",
                            placeholder: "https://example.com/model.obj",
                            oninput: move |e| model_url.set(e.value()),
                            class: "w-full bg-gray-700 rounded px-2 py-1 text-sm"
                        }
                    }
                    
                    // Preset models
                    div { class: "mb-2",
                        label { class: "block text-xs text-gray-400 mb-1", "Presets" }
                        div { class: "grid grid-cols-1 gap-1",
                            for (name , url , fmt) in preset_models.clone() {
                                button {
                                    class: "bg-gray-600 hover:bg-gray-500 px-2 py-1 rounded text-xs text-left transition",
                                    onclick: move |_| {
                                        model_url.set(url.to_string());
                                        format.set(fmt.clone());
                                        // Reset transform for new models
                                        pos_x.set(0.0); pos_y.set(0.0); pos_z.set(0.0);
                                        rot_x.set(0.0); rot_y.set(0.0); rot_z.set(0.0);
                                        scale.set(1.0);
                                    },
                                    "{name}"
                                }
                            }
                        }
                    }
                }
                
                // Transform Section
                ControlGroup { title: "🔧 Transform",
                    Slider { label: "Pos X", value: pos_x, min: -10.0, max: 10.0, step: 0.1 }
                    Slider { label: "Pos Y", value: pos_y, min: -10.0, max: 10.0, step: 0.1 }
                    Slider { label: "Pos Z", value: pos_z, min: -10.0, max: 10.0, step: 0.1 }
                    
                    div { class: "my-2 border-t border-gray-600" }
                    
                    Slider { label: "Rot X", value: rot_x, min: 0.0, max: 360.0, step: 1.0, suffix: "°" }
                    Slider { label: "Rot Y", value: rot_y, min: 0.0, max: 360.0, step: 1.0, suffix: "°" }
                    Slider { label: "Rot Z", value: rot_z, min: 0.0, max: 360.0, step: 1.0, suffix: "°" }
                    
                    div { class: "my-2 border-t border-gray-600" }
                    
                    Slider { label: "Scale", value: scale, min: 0.01, max: 5.0, step: 0.01 }
                }
                
                // Appearance Section
                ControlGroup { title: "🎨 Appearance",
                    div { class: "flex items-center gap-3 mb-3",
                        input {
                            r#type: "color",
                            value: "{color()}",
                            oninput: move |e| color.set(e.value()),
                            class: "w-12 h-8 rounded cursor-pointer"
                        }
                        span { class: "text-sm text-gray-400", "{color()}" }
                    }
                    
                    Toggle { label: "Wireframe", value: wireframe }
                    
                    // Shader selector
                    div { class: "mb-3",
                        label { class: "block text-xs text-gray-400 mb-1", "Shader Effect" }
                        select {
                            class: "w-full bg-gray-700 rounded px-2 py-1 text-sm",
                            onchange: move |e| {
                                let new_shader = match e.value().as_str() {
                                    "gradient" => ShaderPreset::Gradient,
                                    "water" => ShaderPreset::Water,
                                    "hologram" => ShaderPreset::Hologram,
                                    "toon" => ShaderPreset::Toon,
                                    "heatmap" => ShaderPreset::Heatmap,
                                    _ => ShaderPreset::None,
                                };
                                shader.set(new_shader);
                            },
                            option { value: "none", "None (Standard Material)" }
                            option { value: "gradient", "🌈 Animated Gradient" }
                            option { value: "water", "🌊 Water/Waves" }
                            option { value: "hologram", "✨ Hologram" }
                            option { value: "toon", "🎨 Toon/Cel Shading" }
                            option { value: "heatmap", "🔥 Heat Map" }
                        }
                    }
                    
                    Toggle { label: "Auto Center", value: auto_center }
                    Toggle { label: "Auto Scale", value: auto_scale }
                }
                
                // Camera Section
                ControlGroup { title: "📷 Camera",
                    Slider { label: "Cam X", value: cam_x, min: -20.0, max: 20.0, step: 0.1 }
                    Slider { label: "Cam Y", value: cam_y, min: -20.0, max: 20.0, step: 0.1 }
                    Slider { label: "Cam Z", value: cam_z, min: -20.0, max: 20.0, step: 0.1 }
                }
                
                // Animation Section
                ControlGroup { title: "🔄 Animation",
                    Toggle { label: "Auto Rotate", value: auto_rotate }
                    if auto_rotate() {
                        Slider { label: "Speed", value: rot_speed, min: 0.0, max: 5.0, step: 0.1, suffix: "x" }
                    }
                }
                
                // Display Section
                ControlGroup { title: "👁️ Display",
                    Toggle { label: "Show Grid", value: show_grid }
                    Toggle { label: "Show Axes", value: show_axes }
                }
                
                // Actions
                ControlGroup { title: "📍 Actions",
                    div { class: "grid grid-cols-2 gap-2",
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 px-2 py-1 rounded text-xs transition",
                            onclick: move |_| { cam_x.set(0.0); cam_y.set(10.0); cam_z.set(0.01); },
                            "Top"
                        }
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 px-2 py-1 rounded text-xs transition",
                            onclick: move |_| { cam_x.set(10.0); cam_y.set(0.0); cam_z.set(0.0); },
                            "Side"
                        }
                        button {
                            class: "bg-green-600 hover:bg-green-700 px-2 py-1 rounded text-xs transition",
                            onclick: move |_| { cam_x.set(5.0); cam_y.set(5.0); cam_z.set(5.0); },
                            "Iso"
                        }
                        button {
                            class: "bg-red-600 hover:bg-red-700 px-2 py-1 rounded text-xs transition",
                            onclick: move |_| {
                                pos_x.set(0.0); pos_y.set(0.0); pos_z.set(0.0);
                                rot_x.set(0.0); rot_y.set(0.0); rot_z.set(0.0);
                                scale.set(1.0);
                                color.set("#ff6b6b".to_string());
                                cam_x.set(5.0); cam_y.set(5.0); cam_z.set(5.0);
                                auto_rotate.set(true);
                                wireframe.set(false);
                                shader.set(ShaderPreset::None);
                            },
                            "Reset"
                        }
                    }
                }
                
                // Info
                div { class: "mt-4 p-3 bg-gray-900 rounded text-xs text-gray-400",
                    p { "Dioxus Three - 3D Model Viewer" }
                    p { "Supports: OBJ, FBX, GLTF, GLB, STL, PLY, DAE" }
                }
            }
            
            // Right - Three.js View
            div { class: "flex-1 relative bg-black",
                ThreeView {
                    model_url: if model_url().is_empty() {{ None }} else {{ Some(model_url()) }},
                    format: format(),
                    pos_x: pos_x(),
                    pos_y: pos_y(),
                    pos_z: pos_z(),
                    rot_x: rot_x(),
                    rot_y: rot_y(),
                    rot_z: rot_z(),
                    scale: scale(),
                    color: color(),
                    auto_center: auto_center(),
                    auto_scale: auto_scale(),
                    cam_x: cam_x(),
                    cam_y: cam_y(),
                    cam_z: cam_z(),
                    auto_rotate: auto_rotate(),
                    rot_speed: rot_speed(),
                    show_grid: show_grid(),
                    show_axes: show_axes(),
                    wireframe: wireframe(),
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
fn Slider(
    label: String,
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    suffix: Option<String>,
) -> Element {
    let suffix = suffix.unwrap_or_default();
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
            span { class: "w-12 text-right text-xs font-mono text-pink-400",
                "{value():.1}{suffix}"
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
