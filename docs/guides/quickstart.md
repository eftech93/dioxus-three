# Quick Start

Let's build a simple 3D model viewer step by step.

## 1. Basic Cube (Hello World)

The simplest example - a rotating colored cube:

```rust
use dioxus::prelude::*;
use dioxus_three::ThreeView;

fn main() {
    dioxus_desktop::launch(app);
}

fn app() -> Element {
    rsx! {
        div { style: "width: 100vw; height: 100vh;",
            ThreeView {
                auto_rotate: true,
                color: "#00ff00".to_string(),
                scale: 1.5,
            }
        }
    }
}
```

## 2. Loading Your First Model

Load a 3D model from a URL:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelFormat};

fn app() -> Element {
    rsx! {
        ThreeView {
            model_url: Some("https://threejs.org/examples/models/obj/male02/male02.obj".to_string()),
            format: ModelFormat::Obj,
            auto_center: true,
            auto_scale: true,
            show_grid: true,
        }
    }
}
```

## 3. Interactive Controls

Add controls to transform the model:

```rust
use dioxus::prelude::*;
use dioxus_three::ThreeView;

fn app() -> Element {
    let mut scale = use_signal(|| 1.0f32);
    let mut rot_y = use_signal(|| 0.0f32);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Control panel
            div { style: "width: 300px; padding: 20px; background: #333; color: white;",
                h2 { "Controls" }
                
                div { style: "margin: 10px 0;",
                    label { "Scale: {scale():.1}" }
                    input {
                        r#type: "range",
                        min: "0.1",
                        max: "3.0",
                        step: "0.1",
                        value: "{scale()}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                scale.set(v);
                            }
                        }
                    }
                }
                
                div { style: "margin: 10px 0;",
                    label { "Rotation Y: {rot_y():.0}°" }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "360",
                        step: "1",
                        value: "{rot_y()}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                rot_y.set(v);
                            }
                        }
                    }
                }
            }
            
            // 3D View
            div { style: "flex: 1;",
                ThreeView {
                    scale: scale(),
                    rot_y: rot_y(),
                    auto_rotate: false,
                }
            }
        }
    }
}
```

## 4. Adding Shader Effects

Apply a built-in shader effect:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ShaderPreset};

fn app() -> Element {
    rsx! {
        ThreeView {
            shader: ShaderPreset::Water,
            auto_rotate: false,
            show_grid: false,
        }
    }
}
```

## 5. Complete Example

A full-featured viewer with multiple controls:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelFormat, ShaderPreset};

fn app() -> Element {
    let mut model_url = use_signal(|| "".to_string());
    let mut format = use_signal(|| ModelFormat::Cube);
    let mut shader = use_signal(|| ShaderPreset::None);
    let mut auto_rotate = use_signal(|| true);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Sidebar
            div { style: "width: 320px; padding: 20px; background: #1a1a2e; color: white; overflow-y: auto;",
                h1 { style: "color: #DEC647;", "3D Viewer" }
                
                h3 { "Model" }
                select {
                    onchange: move |e| {
                        let (url, fmt) = match e.value().as_str() {
                            "cube" => ("", ModelFormat::Cube),
                            "obj" => ("https://threejs.org/examples/models/obj/male02/male02.obj", ModelFormat::Obj),
                            "gltf" => ("https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf", ModelFormat::Gltf),
                            _ => ("", ModelFormat::Cube),
                        };
                        model_url.set(url.to_string());
                        format.set(fmt);
                    },
                    option { value: "cube", "Cube" }
                    option { value: "obj", "OBJ Character" }
                    option { value: "gltf", "GLTF Helmet" }
                }
                
                h3 { "Shader" }
                select {
                    onchange: move |e| {
                        let s = match e.value().as_str() {
                            "none" => ShaderPreset::None,
                            "gradient" => ShaderPreset::Gradient,
                            "water" => ShaderPreset::Water,
                            "hologram" => ShaderPreset::Hologram,
                            _ => ShaderPreset::None,
                        };
                        shader.set(s);
                    },
                    option { value: "none", "None" }
                    option { value: "gradient", "Gradient" }
                    option { value: "water", "Water" }
                    option { value: "hologram", "Hologram" }
                }
                
                label {
                    input {
                        r#type: "checkbox",
                        checked: "{auto_rotate()}",
                        onchange: move |e| auto_rotate.set(e.checked())
                    }
                    " Auto-rotate"
                }
            }
            
            // 3D View
            div { style: "flex: 1;",
                ThreeView {
                    model_url: if model_url().is_empty() {{ None }} else {{ Some(model_url()) }},
                    format: format(),
                    shader: shader(),
                    auto_rotate: auto_rotate(),
                    auto_center: true,
                    auto_scale: true,
                }
            }
        }
    }
}
```

## Next Steps

- Learn about [supported formats](formats.md)
- Explore [shader effects](shaders.md)
- Read the [API reference](../api/threeview.md)
