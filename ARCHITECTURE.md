# Dioxus Three - Architecture

## Overview

Dioxus Three is a Dioxus Desktop component that renders interactive 3D content using Three.js via a WebView, rather than native GPU rendering. This approach avoids platform-specific GPU issues while providing full access to Three.js's mature 3D capabilities.

## Architecture Diagram

```
┌─────────────────────────────────────────────┐
│              Dioxus Desktop App             │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │         WebView Component           │   │
│  │                                     │   │
│  │   ┌─────────────────────────────┐  │   │
│  │   │     Three.js Scene          │  │   │
│  │   │                             │  │   │
│  │   │   ┌─────┐ ┌─────┐ ┌─────┐  │  │   │
│  │   │   │Mesh │ │Mesh │ │Mesh │  │  │   │
│  │   │   └─────┘ └─────┘ └─────┘  │  │   │
│  │   │                             │  │   │
│  │   │   ┌─────┐ ┌─────┐ ┌─────┐  │  │   │
│  │   │   │Light│ │Light│ │Camera│  │  │   │
│  │   │   └─────┘ └─────┘ └─────┘  │  │   │
│  │   └─────────────────────────────┘  │   │
│  │                                     │   │
│  │   ┌─────────────────────────────┐  │   │
│  │   │   ShaderMaterial (GLSL)     │  │   │
│  │   │                             │  │   │
│  │   │   ┌─────┐ ┌─────┐ ┌─────┐  │  │   │
│  │   │   │ vs  │ │ fs  │ │uni  │  │  │   │
│  │   │   └─────┘ └─────┘ └─────┘  │  │   │
│  │   └─────────────────────────────┘  │   │
│  └─────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
         │
         │ Props (one-way data flow)
         ▼
┌─────────────────────────────────────────────┐
│              ThreeView Component            │
│              (Dioxus Component)             │
│                                             │
│  ┌──────────────┐  ┌──────────────┐        │
│  │ Model Loader │  │ Shader System│        │
│  │              │  │              │        │
│  │ • ObjLoader  │  │ • Presets    │        │
│  │ • FbxLoader  │  │ • Custom     │        │
│  │ • GltfLoader │  │ • Uniforms   │        │
│  └──────────────┘  └──────────────┘        │
│                                             │
│  ┌──────────────┐  ┌──────────────┐        │
│  │ Transform    │  │ Camera       │        │
│  └──────────────┘  └──────────────┘        │
└─────────────────────────────────────────────┘
```

## Core Components

### 1. `ThreeView` Component

The main Dioxus component that renders a Three.js scene in a WebView.

```rust
#[component]
pub fn ThreeView(props: ThreeViewProps) -> Element
```

**Key Props:**
- `model_url` - URL/path to 3D model
- `format` - Model format (Obj, Fbx, Gltf, etc.)
- `shader` - Shader effect preset
- Transform (pos_x/y/z, rot_x/y/z, scale)
- Camera (cam_x/y/z, target_x/y/z)
- Appearance (color, wireframe, background)

### 2. HTML Generation (`generate_three_js_html`)

Generates the HTML document that runs inside the WebView:

1. **Template Setup** - Creates HTML skeleton with Three.js CDN
2. **Material Generation** - Creates MeshStandardMaterial or ShaderMaterial
3. **Model Loading** - Injects appropriate Three.js loader based on format
4. **Scene Building** - Camera, lights, renderer, helpers
5. **Animation Loop** - Handles auto-rotation and shader time updates

### 3. Model Loader System

Supports multiple 3D formats via Three.js loaders loaded on-demand:

```rust
pub enum ModelFormat {
    Obj, Fbx, Gltf, Glb, Stl, Ply, Dae, Json,
    Cube,  // Built-in default
}
```

**Loader Injection:**
Each format has a corresponding CDN script that gets conditionally injected.

**Loading Flow:**
1. User provides URL + format
2. HTML generator adds appropriate loader script
3. Three.js loads the model asynchronously
4. On success: Apply material, center/scale if enabled
5. On error: Fallback to cube with error logged

### 4. Shader System

**ShaderPreset Enum:**
```rust
pub enum ShaderPreset {
    None,                                    // Standard PBR
    Water,                                   // Animated water waves
    Fire,                                    // Animated fire effect
    Gradient { color1, color2 },             // Animated color gradient
    Pulse { color, speed },                  // Pulsing color animation
    Custom(ShaderConfig),                    // User-defined shaders
}
```

**ShaderConfig:**
```rust
pub struct ShaderConfig {
    pub vertex_shader: Option<String>,
    pub fragment_shader: Option<String>,
    pub uniforms: HashMap<String, ShaderUniform>,
    pub animated: bool,
}
```

**Shader Generation Flow:**
1. User sets `shader` prop on `ThreeView`
2. Component calls `generate_three_js_html()` with shader settings
3. HTML generator requests shader code from `ShaderPreset`
4. If `ShaderPreset::Custom`, user-provided GLSL is used
5. If built-in preset, built-in GLSL strings are returned
6. HTML includes Three.js `ShaderMaterial` with vertex/fragment shaders
7. Uniforms are passed as JavaScript object
8. If animated, `u_time` uniform is updated in render loop

**Built-in Shaders:**
- Located in `shaders/` directory
- GLSL files: `water.frag`, `fire.frag`, `gradient.frag`, `pulse.frag`
- Embedded in Rust code as string literals

**Custom Shader Example:**
```rust
let shader = ShaderConfig {
    vertex_shader: Some(r#"
        varying vec2 vUv;
        void main() {
            vUv = uv;
            gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
    "#.to_string()),
    fragment_shader: Some(r#"
        uniform vec3 u_color;
        varying vec2 vUv;
        void main() {
            gl_FragColor = vec4(u_color * vUv.x, 1.0);
        }
    "#.to_string()),
    uniforms: [("u_color".to_string(), ShaderUniform::Color("#ff0000".to_string()))]
        .into_iter().collect(),
    animated: false,
};
```

### 5. Uniform System

Shader uniforms pass data from JavaScript to GLSL:

```rust
pub enum ShaderUniform {
    Float(f32),
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Color(String),  // Hex color converted to vec3
}
```

**Auto-uniforms:**
- `u_time` - Automatically set for animated shaders
- `u_resolution` - Viewport dimensions
- `u_color` - Mesh color from props

## Data Flow

### Props → HTML Generation

```
ThreeViewProps
    │
    ├──> generate_three_js_html()
    │         │
    │         ├──> Build HTML template
    │         ├──> Inject Three.js CDN
    │         ├──> Generate material code
    │         │       └── shader.vertex_shader()
    │         │       └── shader.fragment_shader()
    │         │       └── shader.uniforms()
    │         ├──> Inject model loader
    │         ├──> Build scene graph
    │         └──> Setup animation loop
    │               └── shader.is_animated()
    │
    └──> Render WebView with srcdoc
```

### Component Lifecycle

1. **Mount** - `ThreeView` renders WebView with generated HTML
2. **Load** - Three.js initializes, model loads asynchronously
3. **Animate** - RequestAnimationFrame loop runs
4. **Update** - Props changes regenerate HTML (full re-render)

## Technical Decisions

### Why Three.js via WebView?

**Rejected Approach:** Native wgpu
- ❌ Requires event loop on main thread
- ❌ Dual window setup problematic on macOS
- ❌ Complex platform-specific window management

**Chosen Approach:** Three.js in WebView
- ✅ Single window, simpler lifecycle
- ✅ Mature 3D library with extensive loaders
- ✅ GLSL shader support built-in
- ✅ Cross-platform consistency
- ✅ Easy asset loading via HTTP

### Why No Rust ↔ JS Bridge?

Data flows one-way: Rust props → HTML string → WebView
- Simpler implementation
- No async complexity
- Props change triggers full re-render (acceptable for this use case)

### ShaderPreset vs Custom Shaders

**Built-in presets** provide:
- Zero configuration effects
- Consistent naming
- Optimized GLSL

**Custom shaders** provide:
- Full creative control
- Domain-specific effects
- Integration with existing shader libraries

## File Structure

```
dioxus-three/
├── src/
│   └── lib.rs              # Main component and shader system
├── shaders/
│   ├── water.frag          # Water wave effect
│   ├── fire.frag           # Fire effect
│   ├── gradient.frag       # Color gradient
│   └── pulse.frag          # Pulsing animation
├── examples/
│   └── demo/
│       └── src/
│           └── main.rs     # Demo application
└── README.md
```

## Future Enhancements

Potential improvements:
1. **Texture Support** - Load custom textures via URLs
2. **Lighting Controls** - Adjustable lights (directional, point, ambient)
3. **Post-processing** - Bloom, DOF, SSAO effects
4. **Animation Clips** - Play skeletal animations from glTF/FBX
5. **Interaction** - Raycasting for click/hover events
6. **Performance** - Virtual scrolling for multiple views
7. **Offline Mode** - Bundle Three.js instead of CDN
8. **Shader Hot-reload** - Edit shaders and see changes live

## Performance Considerations

- **Model size** - Large models may take time to download/parse
- **Shader complexity** - Complex fragment shaders impact FPS
- **Multiple views** - Each WebView is a separate process
- **Memory** - Three.js scene holds GPU resources

## Security Notes

- Models loaded from external URLs (CORS dependent)
- JavaScript runs in isolated WebView
- No eval() or dynamic code execution from user input
- Shader code is sanitized (basic HTML escaping)
