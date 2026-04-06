# Contributing

We welcome contributions to Dioxus Three!

## Development Setup

1. **Clone the repository**

```bash
git clone https://github.com/eftech93/dioxus-three
cd dioxus-three
```

2. **Build the project**

```bash
cargo build
```

3. **Run the demo**

```bash
cd examples/demo
cargo run
```

## Project Structure

```
dioxus-three/
├── src/
│   └── lib.rs              # Main library code
├── shaders/
│   ├── gradient.frag       # Built-in shaders
│   ├── gradient.vert
│   ├── water.frag
│   ├── water.vert
│   └── ...
├── examples/
│   └── demo/               # Demo application
│       └── src/
│           └── main.rs
├── docs/                   # Documentation
└── README.md
```

## Adding a New Shader

1. Create shader files in `shaders/`:

```glsl
// shaders/myshader.vert
varying vec2 vUv;
void main() {
    vUv = uv;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
```

```glsl
// shaders/myshader.frag
uniform vec3 u_color;
uniform float u_time;
varying vec2 vUv;

void main() {
    gl_FragColor = vec4(u_color * vUv.x, 1.0);
}
```

2. Add to `ShaderPreset` enum:

```rust
pub enum ShaderPreset {
    // ... existing variants
    MyShader,
}
```

3. Implement shader methods:

```rust
impl ShaderPreset {
    fn vertex_shader(&self) -> Option<String> {
        match self {
            // ...
            ShaderPreset::MyShader => {
                Some(include_str!("shaders/myshader.vert").to_string())
            }
            _ => None,
        }
    }
    
    fn fragment_shader(&self) -> Option<String> {
        match self {
            // ...
            ShaderPreset::MyShader => {
                Some(include_str!("shaders/myshader.frag").to_string())
            }
            _ => None,
        }
    }
    
    fn is_animated(&self) -> bool {
        match self {
            // ...
            ShaderPreset::MyShader => true,  // or false
        }
    }
}
```

4. Update documentation

## Adding a New Model Format

1. Add to `ModelFormat` enum:

```rust
pub enum ModelFormat {
    // ... existing formats
    MyFormat,
}
```

2. Implement format methods:

```rust
impl ModelFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            // ...
            ModelFormat::MyFormat => "myformat",
        }
    }
    
    fn loader_js(&self) -> &'static str {
        match self {
            // ...
            ModelFormat::MyFormat => "MyFormatLoader",
        }
    }
    
    fn loader_url(&self) -> &'static str {
        match self {
            // ...
            ModelFormat::MyFormat => "https://.../MyFormatLoader.js",
        }
    }
}
```

3. Update HTML generation if needed for special handling

4. Update documentation

## Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Add doc comments for public APIs
- Keep functions focused and small

## Testing

Test changes with the demo application:

```bash
cd examples/demo
cargo run
```

Test all supported formats and shaders.

## Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Reporting Issues

Please include:
- Rust version (`rustc --version`)
- Dioxus version
- Operating system
- Steps to reproduce
- Expected vs actual behavior

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.
