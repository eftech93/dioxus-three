# Shader Effects

Dioxus Three comes with built-in shader effects for stunning visual presentations.

## Built-in Presets

| Preset | Description | Animated |
|--------|-------------|----------|
| `Gradient` | Animated RGB gradient | ✅ |
| `Water` | Animated water waves | ✅ |
| `Hologram` | Sci-fi hologram with scanlines | ✅ |
| `Toon` | Cel/toon shading | ❌ |
| `Heatmap` | Temperature visualization | ❌ |

## Using Presets

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ShaderPreset};

fn app() -> Element {
    rsx! {
        ThreeView {
            shader: ShaderPreset::Water,
            auto_rotate: false,  // Animated shaders look better without rotation
        }
    }
}
```

## Gradient Shader

Animated color cycling across the model surface:

```rust
rsx! {
    ThreeView {
        shader: ShaderPreset::Gradient,
        auto_rotate: false,
    }
}
```

**Best for:** Showcases, presentations, eye-catching displays

## Water Shader

Simulates flowing water with wave animation:

```rust
rsx! {
    ThreeView {
        shader: ShaderPreset::Water,
        auto_rotate: false,
        scale: 2.0,
    }
}
```

**Best for:** Fluid simulations, organic shapes, water-themed UIs

## Hologram Shader

Sci-fi holographic projection effect with scanlines:

```rust
rsx! {
    ThreeView {
        shader: ShaderPreset::Hologram,
        auto_rotate: true,
        color: "#00ffff".to_string(),  // Cyan hologram
    }
}
```

**Best for:** Sci-fi interfaces, futuristic presentations, HUD elements

## Toon Shader

Cel-shaded cartoon appearance:

```rust
rsx! {
    ThreeView {
        shader: ShaderPreset::Toon,
        auto_rotate: true,
    }
}
```

**Best for:** Games, stylized presentations, cartoon aesthetics

## Heatmap Shader

Temperature visualization with color gradients:

```rust
rsx! {
    ThreeView {
        shader: ShaderPreset::Heatmap,
        auto_rotate: false,
    }
}
```

**Best for:** Data visualization, thermal imaging, scientific displays

## Shader Selector Example

Create a shader selector UI:

```rust
fn app() -> Element {
    let mut shader = use_signal(|| ShaderPreset::None);
    
    let presets = vec![
        ("None", ShaderPreset::None),
        ("Gradient", ShaderPreset::Gradient),
        ("Water", ShaderPreset::Water),
        ("Hologram", ShaderPreset::Hologram),
        ("Toon", ShaderPreset::Toon),
        ("Heatmap", ShaderPreset::Heatmap),
    ];
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 250px; padding: 20px; background: #1a1a2e; color: white;",
                h3 { "Shader Effect" }
                for (name, preset) in presets {
                    button {
                        style: if shader() == preset { "background: #DEC647;" } else { "" },
                        onclick: move |_| shader.set(preset.clone()),
                        "{name}"
                    }
                }
            }
            
            ThreeView {
                shader: shader(),
                auto_rotate: shader() == ShaderPreset::None || 
                             shader() == ShaderPreset::Toon ||
                             shader() == ShaderPreset::Heatmap,
            }
        }
    }
}
```

## Combining with Models

Shaders work with any model format:

```rust
rsx! {
    ThreeView {
        model_url: Some("https://example.com/helmet.gltf".to_string()),
        format: ModelFormat::Gltf,
        shader: ShaderPreset::Hologram,
        auto_center: true,
    }
}
```

## Performance Considerations

- Animated shaders update every frame
- Use `auto_rotate: false` for animated shaders
- Complex shaders may impact performance on low-end devices
- Fragment shaders affect every pixel - keep them efficient

## Next Steps

Learn to create your own shaders with [Custom Shaders](custom-shaders.md).
