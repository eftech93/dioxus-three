# ShaderPreset

Built-in shader effects for 3D models.

## Definition

```rust
#[derive(Clone, PartialEq, Debug)]
pub enum ShaderPreset {
    None,
    Gradient,
    Water,
    Hologram,
    Toon,
    Heatmap,
    Custom(ShaderConfig),
}
```

## Variants

### `ShaderPreset::None`

Standard PBR material (default).

```rust
use dioxus_three::ShaderPreset;

ThreeView {
    shader: ShaderPreset::None,
}
```

Uses Three.js `MeshStandardMaterial` with:
- Physically based rendering
- Roughness: 0.5
- Metalness: 0.3

### `ShaderPreset::Gradient`

Animated RGB gradient effect.

```rust
ThreeView {
    shader: ShaderPreset::Gradient,
    auto_rotate: false,
}
```

**Animation:** Colors cycle across the model surface over time.

**Best for:** Showcases, presentations

### `ShaderPreset::Water`

Animated water wave effect.

```rust
ThreeView {
    shader: ShaderPreset::Water,
    auto_rotate: false,
}
```

**Animation:** Vertex displacement simulates flowing water.

**Best for:** Fluid simulations, organic shapes

### `ShaderPreset::Hologram`

Sci-fi holographic projection.

```rust
ThreeView {
    shader: ShaderPreset::Hologram,
    color: "#00ffff".to_string(),
}
```

**Features:**
- Scanline effect
- Transparency
- Glow effect
- Edge highlighting

**Best for:** Sci-fi interfaces, futuristic UIs

### `ShaderPreset::Toon`

Cel/toon shading for cartoon look.

```rust
ThreeView {
    shader: ShaderPreset::Toon,
}
```

**Features:**
- Discrete shading bands
- Outline effect
- Cartoon aesthetic

**Best for:** Games, stylized art

### `ShaderPreset::Heatmap`

Temperature visualization.

```rust
ThreeView {
    shader: ShaderPreset::Heatmap,
}
```

**Features:**
- Color based on height/position
- Hot/cold visualization
- Data visualization

**Best for:** Scientific visualization, thermal imaging

### `ShaderPreset::Custom`

Use your own GLSL shaders.

```rust
use dioxus_three::{ShaderPreset, ShaderConfig};
use std::collections::HashMap;

let config = ShaderConfig {
    vertex_shader: Some("...".to_string()),
    fragment_shader: Some("...".to_string()),
    uniforms: HashMap::new(),
    animated: true,
};

ThreeView {
    shader: ShaderPreset::Custom(config),
}
```

See [Custom Shaders](../guides/custom-shaders.md) for details.

## Animation Support

| Preset | Animated | Notes |
|--------|----------|-------|
| None | ❌ | Static material |
| Gradient | ✅ | Cycles colors |
| Water | ✅ | Moves vertices |
| Hologram | ✅ | Scanlines pulse |
| Toon | ❌ | Static shading |
| Heatmap | ❌ | Static colors |
| Custom | Configurable | Set `animated` field |

## Usage Patterns

### Disable Auto-Rotate for Animated Shaders

```rust
// Good - shader provides its own animation
ThreeView {
    shader: ShaderPreset::Gradient,
    auto_rotate: false,
}

// Good - auto-rotate adds motion
ThreeView {
    shader: ShaderPreset::Toon,
    auto_rotate: true,
}
```

### Switching Shaders

```rust
fn app() -> Element {
    let mut shader = use_signal(|| ShaderPreset::None);
    
    rsx! {
        select {
            onchange: move |e| {
                shader.set(match e.value().as_str() {
                    "gradient" => ShaderPreset::Gradient,
                    "water" => ShaderPreset::Water,
                    "hologram" => ShaderPreset::Hologram,
                    _ => ShaderPreset::None,
                });
            },
            option { value: "none", "None" }
            option { value: "gradient", "Gradient" }
            option { value: "water", "Water" }
            option { value: "hologram", "Hologram" }
        }
        
        ThreeView {
            shader: shader(),
        }
    }
}
```

## See Also

- [Shader Effects](../guides/shaders.md) - Detailed guide
- [Custom Shaders](../guides/custom-shaders.md) - Write your own
- [ShaderConfig](shaderconfig.md) - Configuration struct
