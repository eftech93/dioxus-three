# Custom Shaders

Create your own GLSL shaders for unique visual effects.

## Shader Basics

Shaders are small programs that run on the GPU:

- **Vertex Shader** - Transforms geometry positions
- **Fragment Shader** - Determines pixel colors

## ShaderConfig Structure

```rust
use dioxus_three::ShaderConfig;
use std::collections::HashMap;

let config = ShaderConfig {
    vertex_shader: Some(String::from("...")),
    fragment_shader: Some(String::from("...")),
    uniforms: HashMap::new(),
    animated: true,
};
```

## Simple Custom Shader

A basic shader that colors based on UV coordinates:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ShaderPreset, ShaderConfig};

fn app() -> Element {
    let shader_config = ShaderConfig {
        vertex_shader: Some(r#"
            varying vec2 vUv;
            void main() {
                vUv = uv;
                gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
            }
        "#.to_string()),
        fragment_shader: Some(r#"
            uniform vec3 u_color;
            uniform float u_time;
            varying vec2 vUv;
            
            void main() {
                // Gradient based on UV + time
                vec3 color = u_color * (0.5 + 0.5 * sin(vUv.x * 10.0 + u_time));
                gl_FragColor = vec4(color, 1.0);
            }
        "#.to_string()),
        uniforms: HashMap::new(),
        animated: true,
    };
    
    rsx! {
        ThreeView {
            shader: ShaderPreset::Custom(shader_config),
            color: "#ff6b6b".to_string(),
        }
    }
}
```

## Built-in Uniforms

The following uniforms are automatically provided:

| Uniform | Type | Description |
|---------|------|-------------|
| `u_time` | float | Elapsed time in seconds (animated only) |
| `u_color` | vec3 | Color from props (converted from hex) |

## Using Custom Uniforms

Pass additional values to your shader:

```rust
use std::collections::HashMap;

let mut uniforms = HashMap::new();
uniforms.insert("u_intensity".to_string(), 0.8);
uniforms.insert("u_speed".to_string(), 2.0);

let shader_config = ShaderConfig {
    vertex_shader: Some(r#"
        uniform float u_speed;
        uniform float u_time;
        varying vec2 vUv;
        
        void main() {
            vUv = uv;
            vec3 pos = position;
            // Wave animation
            pos.z += sin(pos.x * u_speed + u_time) * 0.1;
            gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
        }
    "#.to_string()),
    fragment_shader: Some(r#"
        uniform vec3 u_color;
        uniform float u_intensity;
        varying vec2 vUv;
        
        void main() {
            float brightness = 0.5 + sin(vUv.y * 20.0) * u_intensity;
            gl_FragColor = vec4(u_color * brightness, 1.0);
        }
    "#.to_string()),
    uniforms,
    animated: true,
};
```

## Shader Examples

### Pulse Effect

```glsl
// Vertex
varying vec3 vPosition;
void main() {
    vPosition = position;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}

// Fragment
uniform vec3 u_color;
uniform float u_time;
varying vec3 vPosition;

void main() {
    float pulse = sin(u_time * 3.0) * 0.5 + 0.5;
    vec3 color = u_color * (0.5 + pulse * 0.5);
    gl_FragColor = vec4(color, 1.0);
}
```

### Rim Lighting

```glsl
// Vertex
varying vec3 vNormal;
varying vec3 vViewPosition;

void main() {
    vNormal = normalize(normalMatrix * normal);
    vec4 mvPosition = modelViewMatrix * vec4(position, 1.0);
    vViewPosition = -mvPosition.xyz;
    gl_Position = projectionMatrix * mvPosition;
}

// Fragment
uniform vec3 u_color;
varying vec3 vNormal;
varying vec3 vViewPosition;

void main() {
    vec3 normal = normalize(vNormal);
    vec3 viewDir = normalize(vViewPosition);
    float rim = 1.0 - max(dot(normal, viewDir), 0.0);
    rim = pow(rim, 3.0);
    vec3 color = u_color + vec3(0.3, 0.5, 1.0) * rim;
    gl_FragColor = vec4(color, 1.0);
}
```

### Checkerboard Pattern

```glsl
// Fragment
uniform vec3 u_color;
uniform float u_time;
varying vec2 vUv;

void main() {
    float checkers = step(0.5, fract(vUv.x * 8.0)) + step(0.5, fract(vUv.y * 8.0));
    checkers = mod(checkers, 2.0);
    vec3 color = mix(u_color, u_color * 0.5, checkers);
    gl_FragColor = vec4(color, 1.0);
}
```

## Vertex Shader Template

```glsl
// Standard vertex shader template
varying vec2 vUv;
varying vec3 vNormal;
varying vec3 vPosition;

void main() {
    vUv = uv;
    vNormal = normalize(normalMatrix * normal);
    vPosition = (modelMatrix * vec4(position, 1.0)).xyz;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
```

## Fragment Shader Template

```glsl
// Standard fragment shader template
uniform vec3 u_color;
uniform float u_time;
varying vec2 vUv;
varying vec3 vNormal;
varying vec3 vPosition;

void main() {
    // Your shader code here
    gl_FragColor = vec4(u_color, 1.0);
}
```

## GLSL Quick Reference

| Type | Description | Example |
|------|-------------|---------|
| `float` | Single number | `1.0` |
| `vec2` | 2D vector | `vec2(1.0, 2.0)` |
| `vec3` | 3D vector/RGB | `vec3(1.0, 0.0, 0.0)` |
| `vec4` | 4D vector/RGBA | `vec4(1.0, 0.0, 0.0, 1.0)` |

### Common Functions

```glsl
// Math
sin(x), cos(x), tan(x)      // Trigonometry
pow(x, y)                   // Power
sqrt(x)                     // Square root
mix(a, b, t)               // Linear interpolation
clamp(x, min, max)         // Clamp value
step(edge, x)              // Step function
smoothstep(edge0, edge1, x) // Smooth step

// Vector
length(v)                  // Vector length
dot(a, b)                  // Dot product
cross(a, b)                // Cross product
normalize(v)               // Normalize vector
reflect(I, N)              // Reflection
refract(I, N, eta)         // Refraction

// Color
vec3 rgb = vec3(1.0, 0.0, 0.0);  // Red
float brightness = length(rgb);   // Brightness
```

## Debugging Shaders

If your shader doesn't work:

1. **Check browser console** for GLSL compile errors
2. **Start simple** - Test with basic color output first
3. **Verify uniforms** - Ensure all uniforms are declared
4. **Check varyings** - Match varying names between shaders

## Resources

- [The Book of Shaders](https://thebookofshaders.com/)
- [GLSL Reference](https://www.khronos.org/opengles/sdk/docs/reference_cards/OpenGL-ES-2_0-Reference-card.pdf)
- [Shadertoy](https://www.shadertoy.com/) - Shader examples
