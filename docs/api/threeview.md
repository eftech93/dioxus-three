# ThreeView Component

The main component for rendering 3D scenes.

## Overview

```rust
use dioxus::prelude::*;
use dioxus_three::ThreeView;

#[component]
pub fn ThreeView(props: ThreeViewProps) -> Element
```

## Props

### Model Loading

#### `model_url`
- **Type:** `Option<String>`
- **Default:** `None`
- **Description:** URL or local path to the 3D model file

```rust
ThreeView {
    model_url: Some("https://example.com/model.obj".to_string()),
}
```

#### `format`
- **Type:** `ModelFormat`
- **Default:** `ModelFormat::Cube`
- **Description:** Format of the model file

```rust
use dioxus_three::ModelFormat;

ThreeView {
    format: ModelFormat::Obj,
}
```

#### `models`
- **Type:** `Vec<ModelConfig>`
- **Default:** `[]`
- **Description:** Multiple models to display simultaneously

```rust
use dioxus_three::{ModelConfig, ModelFormat};

ThreeView {
    models: vec![
        ModelConfig::new("", ModelFormat::Cube)
            .with_position(0.0, 0.0, 0.0)
            .with_color("#ff6b6b"),
        ModelConfig::new("https://example.com/helmet.gltf", ModelFormat::Gltf)
            .with_position(2.0, 0.0, 0.0)
            .with_scale(0.5),
    ],
}
```

#### `auto_center`
- **Type:** `bool`
- **Default:** `true`
- **Description:** Automatically center the model at the origin

```rust
ThreeView {
    auto_center: true,
}
```

#### `auto_scale`
- **Type:** `bool`
- **Default:** `false`
- **Description:** Automatically scale the model to fit the viewport

```rust
ThreeView {
    auto_scale: true,
}
```

### Transform

#### `pos_x`, `pos_y`, `pos_z`
- **Type:** `f32`
- **Default:** `0.0`
- **Description:** Model position in 3D space

```rust
ThreeView {
    pos_x: 1.0,
    pos_y: 0.5,
    pos_z: -2.0,
}
```

#### `rot_x`, `rot_y`, `rot_z`
- **Type:** `f32`
- **Default:** `0.0`
- **Description:** Model rotation in degrees

```rust
ThreeView {
    rot_x: 45.0,  // Pitch
    rot_y: 90.0,  // Yaw
    rot_z: 0.0,   // Roll
}
```

#### `scale`
- **Type:** `f32`
- **Default:** `1.0`
- **Description:** Uniform scale factor

```rust
ThreeView {
    scale: 2.0,  // Double size
}
```

### Appearance

#### `color`
- **Type:** `String`
- **Default:** `"#ff6b6b"`
- **Description:** Material color as hex string

```rust
ThreeView {
    color: "#00ff00".to_string(),
}
```

#### `wireframe`
- **Type:** `bool`
- **Default:** `false`
- **Description:** Render in wireframe mode

```rust
ThreeView {
    wireframe: true,
}
```

#### `shader`
- **Type:** `ShaderPreset`
- **Default:** `ShaderPreset::None`
- **Description:** Shader effect to apply

```rust
use dioxus_three::ShaderPreset;

ThreeView {
    shader: ShaderPreset::Gradient,
}
```

#### `background`
- **Type:** `String`
- **Default:** `"#1a1a2e"`
- **Description:** Scene background color as hex string

```rust
ThreeView {
    background: "#000000".to_string(),
}
```

#### `show_grid`
- **Type:** `bool`
- **Default:** `true`
- **Description:** Show grid helper

```rust
ThreeView {
    show_grid: true,
}
```

#### `show_axes`
- **Type:** `bool`
- **Default:** `true`
- **Description:** Show axes helper

```rust
ThreeView {
    show_axes: true,
}
```

#### `shadows`
- **Type:** `bool`
- **Default:** `true`
- **Description:** Enable shadow rendering

```rust
ThreeView {
    shadows: true,
}
```

### Camera

#### `cam_x`, `cam_y`, `cam_z`
- **Type:** `f32`
- **Default:** `5.0`
- **Description:** Camera position

```rust
ThreeView {
    cam_x: 10.0,
    cam_y: 5.0,
    cam_z: 10.0,
}
```

#### `target_x`, `target_y`, `target_z`
- **Type:** `f32`
- **Default:** `0.0`
- **Description:** Point the camera looks at

```rust
ThreeView {
    target_x: 0.0,
    target_y: 2.0,
    target_z: 0.0,
}
```

### Animation

#### `auto_rotate`
- **Type:** `bool`
- **Default:** `true`
- **Description:** Automatically rotate the model

```rust
ThreeView {
    auto_rotate: true,
}
```

#### `rot_speed`
- **Type:** `f32`
- **Default:** `1.0`
- **Description:** Auto-rotation speed multiplier

```rust
ThreeView {
    rot_speed: 2.0,  // Double speed
}
```

### Styling

#### `class`
- **Type:** `String`
- **Default:** `""`
- **Description:** Additional CSS classes for the container

```rust
ThreeView {
    class: "my-custom-class".to_string(),
}
```

## Complete Example

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelConfig, ModelFormat, ShaderPreset};

fn app() -> Element {
    rsx! {
        ThreeView {
            // Multiple models
            models: vec![
                ModelConfig::new("", ModelFormat::Cube)
                    .with_position(0.0, 0.0, 0.0)
                    .with_color("#ff6b6b"),
                ModelConfig::new("https://example.com/helmet.gltf", ModelFormat::Gltf)
                    .with_position(2.0, 0.0, 0.0)
                    .with_scale(0.5),
            ],
            
            // Transform (applies to all models unless overridden in ModelConfig)
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            rot_x: 0.0,
            rot_y: 45.0,
            rot_z: 0.0,
            scale: 1.0,
            
            // Appearance
            color: "#ff6b6b".to_string(),
            wireframe: false,
            shader: ShaderPreset::None,
            background: "#1a1a2e".to_string(),
            show_grid: true,
            show_axes: true,
            shadows: true,
            
            // Camera
            cam_x: 5.0,
            cam_y: 5.0,
            cam_z: 5.0,
            
            // Animation
            auto_rotate: true,
            rot_speed: 1.0,
        }
    }
}
```

## Platform Notes

### Desktop
- Uses WebView with iframe
- Full HTML regeneration on prop changes

### Web (WASM)
- Uses HTML5 Canvas
- Real-time state synchronization
- Requires signal-based wrapper for updates

### Mobile
- Uses WebView (same as desktop)
- Touch-friendly controls

## See Also

- [ModelFormat](modelformat.md) - Supported model formats
- [ShaderPreset](shaderpreset.md) - Built-in shader effects
- [ShaderConfig](shaderconfig.md) - Custom shader configuration
- [ModelConfig](../guides/models.md) - Multi-model configuration
