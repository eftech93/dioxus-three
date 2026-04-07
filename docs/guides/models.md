# Loading Models

Learn how to load and display 3D models in your Dioxus application.

## Basic Model Loading

Load a model from a URL:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelFormat};

fn app() -> Element {
    rsx! {
        ThreeView {
            model_url: Some("https://example.com/model.obj".to_string()),
            format: ModelFormat::Obj,
        }
    }
}
```

## Auto-Center and Auto-Scale

Models come in various sizes. Use these options to fit them in view:

```rust
rsx! {
    ThreeView {
        model_url: Some("https://example.com/model.obj".to_string()),
        format: ModelFormat::Obj,
        auto_center: true,  // Center model at origin
        auto_scale: true,   // Scale to fit viewport
    }
}
```

## Multiple Models

Display multiple models simultaneously with independent transforms:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelConfig, ModelFormat};

fn app() -> Element {
    let models = vec![
        // Red cube at origin
        ModelConfig::new("", ModelFormat::Cube)
            .with_position(0.0, 0.0, 0.0)
            .with_color("#ff6b6b"),
        
        // Helmet to the right
        ModelConfig::new(
            "https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf",
            ModelFormat::Gltf
        )
            .with_position(2.0, 0.0, 0.0)
            .with_scale(0.5),
        
        // Duck to the left
        ModelConfig::new(
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Duck/glTF/Duck.gltf",
            ModelFormat::Gltf
        )
            .with_position(-2.0, 0.0, 0.0)
            .with_scale(0.3)
            .with_rotation(0.0, 90.0, 0.0),
    ];
    
    rsx! {
        ThreeView {
            models: models,
            auto_rotate: true,
        }
    }
}
```

### ModelConfig Options

| Method | Description |
|--------|-------------|
| `new(url, format)` | Create a new model config |
| `with_position(x, y, z)` | Set position in 3D space |
| `with_rotation(x, y, z)` | Set rotation in degrees |
| `with_scale(scale)` | Set uniform scale |
| `with_color(color)` | Set material color (hex) |

## Dynamic Model Loading

Add or remove models at runtime:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelConfig, ModelFormat};

fn app() -> Element {
    let mut models = use_signal(|| vec![
        ModelConfig::new("", ModelFormat::Cube).with_color("#ff6b6b")
    ]);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Control panel
            div { style: "width: 250px; padding: 20px; background: #333; color: white;",
                h3 { "Add Models" }
                
                button {
                    onclick: move |_| {
                        models.write().push(
                            ModelConfig::new("", ModelFormat::Cube)
                                .with_position(
                                    rand::random::<f32>() * 4.0 - 2.0,
                                    0.0,
                                    rand::random::<f32>() * 4.0 - 2.0
                                )
                                .with_color("#00ff00")
                        );
                    },
                    "Add Random Cube"
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
                    "Add Helmet"
                }
                
                button {
                    onclick: move |_| {
                        let mut m = models.write();
                        if m.len() > 1 {
                            m.pop();
                        }
                    },
                    "Remove Last"
                }
            }
            
            // 3D View
            ThreeView {
                models: models.read().clone(),
                auto_center: true,
                auto_scale: true,
            }
        }
    }
}
```

## Loading States

The component handles loading automatically, showing a spinner until the model is ready.

## Error Handling

If a model fails to load, the component automatically falls back to a colored cube and logs the error to the console.

## Best Practices

1. **Use HTTPS URLs** - Avoid mixed content issues
2. **Enable CORS** - Servers must allow cross-origin requests
3. **Optimize models** - Large models take longer to load
4. **Use GLB for best results** - Single file, efficient format
5. **Set appropriate scale** - Use `auto_scale` for unknown models
6. **Position models** - When using multiple models, position them so they don't overlap

## Model Preloading

For better UX, preload models before displaying:

```rust
fn app() -> Element {
    let mut is_loading = use_signal(|| true);
    
    use_effect(move || {
        // Simulate model preloading
        spawn(async move {
            // Preload logic here
            is_loading.set(false);
        });
    });
    
    rsx! {
        if is_loading() {
            "Loading..."
        } else {
            ThreeView {
                model_url: Some("https://example.com/model.glb".to_string()),
                format: ModelFormat::Glb,
            }
        }
    }
}
```

## Switching Between Models

Replace models dynamically:

```rust
fn app() -> Element {
    let mut current_model = use_signal(|| 0usize);
    
    let model_list = vec![
        ("Cube", ModelConfig::new("", ModelFormat::Cube)),
        ("Helmet", ModelConfig::new(
            "https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf",
            ModelFormat::Gltf
        )),
        ("Duck", ModelConfig::new(
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Duck/glTF/Duck.gltf",
            ModelFormat::Gltf
        )),
    ];
    
    let (_, config) = &model_list[current_model()];
    
    rsx! {
        div { style: "height: 100vh;",
            // Model selector
            div { style: "position: absolute; top: 20px; left: 20px; z-index: 10;",
                for (i, (name, _)) in model_list.iter().enumerate() {
                    button {
                        style: if i == current_model() { 
                            "background: #DEC647; margin-right: 10px;" 
                        } else { 
                            "margin-right: 10px;" 
                        },
                        onclick: move |_| current_model.set(i),
                        "{name}"
                    }
                }
            }
            
            ThreeView {
                models: vec![config.clone()],
                auto_center: true,
                auto_scale: true,
            }
        }
    }
}
```
