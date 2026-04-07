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

## 3. Multiple Models

Display multiple models simultaneously:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelConfig, ModelFormat};

fn app() -> Element {
    let models = vec![
        ModelConfig::new("", ModelFormat::Cube)
            .with_position(-2.0, 0.0, 0.0)
            .with_color("#ff6b6b"),
        ModelConfig::new("https://example.com/helmet.gltf", ModelFormat::Gltf)
            .with_position(0.0, 0.0, 0.0)
            .with_scale(0.5),
        ModelConfig::new("https://example.com/duck.gltf", ModelFormat::Gltf)
            .with_position(2.0, 0.0, 0.0)
            .with_scale(0.3),
    ];
    
    rsx! {
        ThreeView {
            models: models,
            auto_rotate: true,
        }
    }
}
```

## 4. Interactive Controls (Desktop)

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

## 5. Interactive Controls (Web/Mobile)

For web and mobile platforms, use a wrapper component for proper signal handling:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelConfig, ShaderPreset};

/// Wrapper that ensures proper signal subscriptions
#[component]
fn ThreeViewWrapper(
    models: Signal<Vec<ModelConfig>>,
    cam_x: Signal<f32>,
    cam_y: Signal<f32>,
    cam_z: Signal<f32>,
    auto_rotate: Signal<bool>,
    shader: Signal<ShaderPreset>,
) -> Element {
    // Read signals to subscribe to changes
    let model_configs = models.read().clone();
    let cx = cam_x();
    let cy = cam_y();
    let cz = cam_z();
    let ar = auto_rotate();
    let sh = shader();
    
    rsx! {
        ThreeView {
            models: model_configs,
            cam_x: cx,
            cam_y: cy,
            cam_z: cz,
            auto_rotate: ar,
            shader: sh,
        }
    }
}

fn app() -> Element {
    let models = use_signal(|| vec![ModelConfig::new("", ModelFormat::Cube)]);
    let cam_x = use_signal(|| 8.0f32);
    let cam_y = use_signal(|| 8.0f32);
    let cam_z = use_signal(|| 8.0f32);
    let auto_rotate = use_signal(|| true);
    let shader = use_signal(|| ShaderPreset::None);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Controls
            div { style: "width: 300px; padding: 20px;",
                button {
                    onclick: move |_| {
                        let mut m = models.write();
                        m.push(ModelConfig::new("", ModelFormat::Cube));
                    },
                    "Add Cube"
                }
            }
            
            // 3D View with wrapper
            ThreeViewWrapper {
                models: models,
                cam_x: cam_x,
                cam_y: cam_y,
                cam_z: cam_z,
                auto_rotate: auto_rotate,
                shader: shader,
            }
        }
    }
}
```

## 6. Adding Shader Effects

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

## 7. Complete Example

A full-featured viewer with multiple controls:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelConfig, ModelFormat, ShaderPreset};

fn app() -> Element {
    let mut models = use_signal(|| vec![
        ModelConfig::new("", ModelFormat::Cube).with_color("#ff6b6b")
    ]);
    let mut shader = use_signal(|| ShaderPreset::None);
    let mut auto_rotate = use_signal(|| true);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Sidebar
            div { style: "width: 320px; padding: 20px; background: #1a1a2e; color: white; overflow-y: auto;",
                h1 { style: "color: #DEC647;", "3D Viewer" }
                
                h3 { "Add Models" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 10px;",
                    button {
                        onclick: move |_| {
                            models.write().push(ModelConfig::new("", ModelFormat::Cube));
                        },
                        "Cube"
                    }
                    button {
                        onclick: move |_| {
                            models.write().push(
                                ModelConfig::new(
                                    "https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf",
                                    ModelFormat::Gltf
                                )
                            );
                        },
                        "Helmet"
                    }
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
                    models: models.read().clone(),
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
- Understand the [architecture](architecture.md)
