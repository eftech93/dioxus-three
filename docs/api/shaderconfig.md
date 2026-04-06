# ShaderConfig

Configuration for custom GLSL shaders.

## Definition

```rust
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ShaderConfig {
    pub vertex_shader: Option<String>,
    pub fragment_shader: Option<String>,
    pub uniforms: HashMap<String, f32>,
    pub animated: bool,
}
```

## Fields

### `vertex_shader`
- **Type:** `Option<String>`
- **Default:** `None`
- **Description:** GLSL vertex shader source code

The vertex shader transforms geometry positions. If `None`, a default pass-through shader is used.

```rust
let config = ShaderConfig {
    vertex_shader: Some(r#"
        varying vec2 vUv;
        void main() {
            vUv = uv;
            gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
    "#.to_string()),
    ..Default::default()
};
```

### `fragment_shader`
- **Type:** `Option<String>`
- **Default:** `None`
- **Description:** GLSL fragment shader source code

The fragment shader determines pixel colors. If `None`, a default shader is used.

```rust
let config = ShaderConfig {
    fragment_shader: Some(r#"
        uniform vec3 u_color;
        varying vec2 vUv;
        void main() {
            gl_FragColor = vec4(u_color * vUv.x, 1.0);
        }
    "#.to_string()),
    ..Default::default()
};
```

### `uniforms`
- **Type:** `HashMap<String, f32>`
- **Default:** `HashMap::new()`
- **Description:** Custom uniform values passed to shaders

Uniforms are variables that can be changed from JavaScript/Rust.

```rust
use std::collections::HashMap;

let mut uniforms = HashMap::new();
uniforms.insert("u_intensity".to_string(), 0.8);
uniforms.insert("u_speed".to_string(), 2.0);

let config = ShaderConfig {
    uniforms,
    ..Default::default()
};
```

### `animated`
- **Type:** `bool`
- **Default:** `false`
- **Description:** Whether to update `u_time` uniform each frame

Set to `true` for time-based animations.

```rust
let config = ShaderConfig {
    animated: true,  // Enables u_time uniform
    ..Default::default()
};
```

## Built-in Uniforms

The following uniforms are automatically provided:

| Uniform | Type | Description |
|---------|------|-------------|
| `u_time` | `float` | Elapsed time in seconds (animated only) |
| `u_color` | `vec3` | Color from props (hex converted to RGB) |

## Usage Example

### Complete Custom Shader

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ShaderPreset, ShaderConfig};
use std::collections::HashMap;

fn app() -> Element {
    let mut uniforms = HashMap::new();
    uniforms.insert("u_intensity".to_string(), 0.5);
    
    let shader_config = ShaderConfig {
        vertex_shader: Some(r#"
            uniform float u_time;
            uniform float u_intensity;
            varying vec2 vUv;
            varying float vElevation;
            
            void main() {
                vUv = uv;
                
                // Wave effect
                vec3 pos = position;
                float elevation = sin(pos.x * 4.0 + u_time) * u_intensity;
                pos.z += elevation;
                vElevation = elevation;
                
                gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
            }
        "#.to_string()),
        
        fragment_shader: Some(r#"
            uniform vec3 u_color;
            uniform float u_time;
            varying vec2 vUv;
            varying float vElevation;
            
            void main() {
                // Mix color based on elevation
                vec3 color = u_color * (0.5 + vElevation * 2.0);
                gl_FragColor = vec4(color, 1.0);
            }
        "#.to_string()),
        
        uniforms,
        animated: true,
    };
    
    rsx! {
        ThreeView {
            shader: ShaderPreset::Custom(shader_config),
            color: "#00ffff".to_string(),
            auto_rotate: false,
        }
    }
}
```

## Shader Templates

### Minimal Vertex Shader

```glsl
varying vec2 vUv;

void main() {
    vUv = uv;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
```

### Minimal Fragment Shader

```glsl
uniform vec3 u_color;
varying vec2 vUv;

void main() {
    gl_FragColor = vec4(u_color, 1.0);
}
```

### Animated Fragment Shader

```glsl
uniform vec3 u_color;
uniform float u_time;
varying vec2 vUv;

void main() {
    float pulse = sin(u_time) * 0.5 + 0.5;
    vec3 color = u_color * (0.5 + pulse * 0.5);
    gl_FragColor = vec4(color, 1.0);
}
```

## Common Patterns

### Passing Multiple Uniforms

```rust
let mut uniforms = HashMap::new();
uniforms.insert("u_speed".to_string(), 2.0);
uniforms.insert("u_scale".to_string(), 1.5);
uniforms.insert("u_threshold".to_string(), 0.3);

let config = ShaderConfig {
    uniforms,
    // ...
};
```

### Conditional Animation

```rust
let config = ShaderConfig {
    animated: true,
    // ...
};

// In shader
void main() {
    #ifdef ANIMATED
        float t = u_time;
    #else
        float t = 0.0;
    #endif
    // ...
}
```

## See Also

- [Custom Shaders Guide](../guides/custom-shaders.md) - Detailed tutorial
- [ShaderPreset](shaderpreset.md) - Built-in shaders
- [GLSL Reference](https://www.khronos.org/opengles/sdk/docs/reference_cards/OpenGL-ES-2_0-Reference-card.pdf)
