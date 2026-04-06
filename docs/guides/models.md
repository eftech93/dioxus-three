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

## Dynamic Model Loading

Switch models based on user selection:

```rust
fn app() -> Element {
    let mut current_model = use_signal(|| 0usize);
    
    let models = vec![
        ("Cube", None, ModelFormat::Cube),
        ("Suzanne", Some("https://example.com/suzanne.obj"), ModelFormat::Obj),
        ("Helmet", Some("https://example.com/helmet.gltf"), ModelFormat::Gltf),
    ];
    
    let (name, url, format) = &models[current_model()];
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 250px; padding: 20px; background: #333; color: white;",
                h3 { "Select Model" }
                for (i, (n, _, _)) in models.iter().enumerate() {
                    button {
                        style: if i == current_model() { "background: #DEC647;" } else { "" },
                        onclick: move |_| current_model.set(i),
                        "{n}"
                    }
                }
            }
            
            ThreeView {
                model_url: url.clone(),
                format: format.clone(),
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

If a model fails to load, the component automatically falls back to a red cube and logs the error to the console.

## Best Practices

1. **Use HTTPS URLs** - Avoid mixed content issues
2. **Enable CORS** - Servers must allow cross-origin requests
3. **Optimize models** - Large models take longer to load
4. **Use GLB for best results** - Single file, efficient format
5. **Set appropriate scale** - Use `auto_scale` for unknown models

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
