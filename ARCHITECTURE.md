# Dioxus Three - Architecture

## Overview

Dioxus Three is a cross-platform Dioxus component that renders interactive 3D content using Three.js. It supports Desktop (WebView), Web (WASM), and Mobile (WebView) platforms through three distinct implementations that share a common Rust API.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                  Your Dioxus Application                    │
│                                                             │
│  ┌─────────────────┐    ┌──────────────────────────────┐  │
│  │   ThreeView     │◄──►│      App State               │  │
│  │   Component     │    │ (Selection, Transforms, etc.)│  │
│  └────────┬────────┘    └──────────────────────────────┘  │
│           │                                                 │
│     ┌─────┴─────┐                                           │
│     │  Bridge   │◄──► Events: Pointer, Gizmo, Selection    │
│     │ (Platform │    State: Camera, Models, Gizmo config   │
│     │  specific)│                                           │
│     └─────┬─────┘                                           │
└───────────┼─────────────────────────────────────────────────┘
            │
    ┌───────┴───────┐
    │   Three.js    │
    │   Renderer    │
    └───────────────┘
```

## Platform Implementations

### Desktop (`src/desktop.rs`) — WebView + iframe

Uses a WebView with an iframe containing a complete Three.js scene loaded from CDN.

```
┌─────────────────┐     ┌─────────────────────┐
│   Dioxus App    │     │    WebView Iframe   │
│                 │     │                     │
│ ┌─────────────┐ │     │ ┌─────────────────┐ │
│ │ ThreeView   │ │     │ │ Three.js Scene  │ │
│ │ Component   │ │     │ │ (from CDN)      │ │
│ └──────┬──────┘ │     │ │                 │ │
│        │        │     │ │ • TransformCtrl │ │
│        ▼        │     │ │ • OrbitControls │ │
│ ┌─────────────┐ │     │ │ • Raycaster     │ │
│ │ use_signal  │ │     │ │ • Model Loader  │ │
│ │ (HTML once) │─┼────►│ │ • Outline FX    │ │
│ └─────────────┘ │     │ └─────────────────┘ │
│                 │     │          ▲          │
│ ┌─────────────┐ │     │          │          │
│ │document::eval│ │     │   postMessage       │
│ │ (events in) │◄┼─────┤   (events out)      │
│ └─────────────┘ │     └─────────────────────┘
│                 │
│ ┌─────────────┐ │
│ │ postMessage │─┼────► update-state, camera,│
│ │ (state out) │ │     gizmo, selection      │
│ └─────────────┘ │     (no iframe reload)    │
└─────────────────┘
```

**Key design decisions:**

1. **HTML generated once**: The complete HTML document is generated via `use_signal` only when the model count changes.
2. **State updates via `postMessage`**: Camera, selection, gizmo, and style updates are sent without iframe regeneration.
3. **Event bridge via `document::eval`**: Pointer events, gizmo drag events, and selection changes are received via `document::eval` polling.
4. **Official `THREE.TransformControls`**: Gizmos are the official Three.js controls.

### Web (`src/web.rs`) — Native Canvas + WASM

Renders directly to a `<canvas>` element using Three.js via WASM.

```
┌──────────────────────────────────────────┐
│            Dioxus App (WASM)             │
│                                          │
│  ┌─────────────┐    ┌─────────────────┐  │
│  │ ThreeView   │    │ wasm_bindgen    │  │
│  │ Component   │◄──►│ Closures        │  │
│  └──────┬──────┘    │                 │  │
│         │           │ • pointer down  │  │
│         ▼           │ • pointer move  │  │
│  ┌─────────────┐    │ • gizmo drag    │  │
│  │  <canvas>   │    │ • selection     │  │
│  │  Element    │    └─────────────────┘  │
│  └──────┬──────┘                         │
│         │                                 │
│         ▼                                 │
│  ┌─────────────────────────────────────┐  │
│  │         Three.js (JS)               │  │
│  │                                     │  │
│  │  • Custom gizmos (arrows, tori,    │  │
│  │    boxes)                           │  │
│  │  • Manual raycasting                │  │
│  │  • Plane-intersection drag math     │  │
│  │  • OrbitControls                    │  │
│  │  • Model loader                     │  │
│  │  • Outline FX                       │  │
│  └─────────────────────────────────────┘  │
└──────────────────────────────────────────┘
```

**Key design decisions:**

1. **Custom-built gizmos**: Handles built from Three.js primitives with manual raycasting.
2. **Manual drag math**: Camera-facing plane intersection for translate, arcball for rotate, distance-based for scale.
3. **Bridge via `wasm_bindgen`**: Events sent via `dioxusThreeRustBridge` JS function.
4. **Live state references**: `updateGizmo` reads from `canvas.dioxusThreeState` to avoid stale references.

### Mobile (`src/mobile.rs`)

Uses the same WebView approach as Desktop. Gizmo features exist but have not been fully tested.

## Shared Components

### `src/lib.rs` — Platform-Independent Core

- `ThreeViewProps` — All component properties
- `ModelConfig`, `ShaderPreset` — Model and shader types
- `generate_three_js_html()` — Desktop iframe HTML generation
- Model loading JS builders
- Selection, gizmo, and event types

### `src/input.rs` — Input System

- `EntityId`, `Vector3`
- `PointerEvent`, `PointerDragEvent`, `GestureEvent`
- `RaycastConfig`

### `src/selection.rs` — Selection System

- `Selection` — List of selected entities
- `SelectionMode` — Single, Multiple
- `SelectionStyle` — Outline color, width, glow

### `src/gizmos.rs` — Gizmo System

- `Gizmo` — Target, mode, space, size, visibility flags
- `GizmoMode` — Translate, Rotate, Scale
- `GizmoSpace` — World, Local
- `GizmoEvent`, `GizmoTransform`

## Data Flow

### Desktop Event Flow

```
User clicks in iframe
    │
    ▼
iframe JS: raycaster.intersectObjects(scene)
    │
    ▼
iframe JS: Check if click is on gizmo handle (isMesh check)
    │
    ├── Yes → TransformControls handles it → postMessage("gizmo-drag")
    │
    └── No  → Check model hit → postMessage("selection-change")
                  │
                  ▼
         Rust (document::eval): Receive postMessage
                  │
                  ▼
         Update signals (selection, gizmo, transforms)
                  │
                  ▼
         Re-render with new props
                  │
                  ▼
         ThreeView detects prop changes
                  │
                  ▼
         Send postMessage("update-state") to iframe
                  │
                  ▼
         iframe updates camera, gizmo, selection, outline
```

### Web Event Flow

```
User clicks on canvas
    │
    ▼
JS pointerdown: raycaster.intersectObjects(gizmoGroup)
    │
    ▼
Check if hit is on gizmo handle
    │
    ├── Yes → Start gizmo drag
    │           On move: plane-intersection math
    │           On up: end drag, call dioxusThreeRustBridge("gizmoDrag", ...)
    │
    └── No  → raycaster.intersectObjects(modelContainer)
                  │
                  ▼
         Hit model → dioxusThreeRustBridge("pointerDown", ...)
                  │
                  ▼
         Rust closure: Update selection signal
                  │
                  ▼
         Re-render → updateGizmo() reads live entityMap
                  │
                  ▼
         Gizmo positioned at new target
```

## Shader System

### ShaderPreset Enum

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

### ShaderConfig

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

### Uniform System

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

## Technical Decisions

### Why Three.js?

**Rejected Approach:** Native wgpu
- ❌ Requires event loop on main thread
- ❌ Dual window setup problematic on macOS
- ❌ Complex platform-specific window management

**Chosen Approach:** Three.js
- ✅ Mature 3D library with extensive loaders
- ✅ GLSL shader support built-in
- ✅ Cross-platform consistency
- ✅ Easy asset loading via HTTP
- ✅ Active ecosystem

### Why Different Implementations per Platform?

**Desktop (WebView iframe):**
- Can load Three.js from CDN easily
- Official `TransformControls` available
- Simpler event bridging via `postMessage`

**Web (Canvas + WASM):**
- Cannot use iframe in WASM context
- Direct canvas rendering for better performance
- Custom gizmos needed since CDN scripts can't be injected the same way

**Mobile (WebView):**
- Same constraints as Desktop
- Shares implementation approach

### Rust ↔ JS Bridge

The original design had no bridge (one-way props → HTML). v0.0.3 added bidirectional communication:

- **Desktop**: `document::eval` + `postMessage` for events and state updates
- **Web**: `wasm_bindgen` closures (`dioxusThreeRustBridge`) for events

State updates no longer trigger full iframe reloads on Desktop. Only model count changes regenerate HTML.

## File Structure

```
dioxus-three/
├── src/
│   ├── lib.rs              # Platform-independent core, HTML generation
│   ├── desktop.rs          # Desktop: WebView iframe implementation
│   ├── web.rs              # Web: Canvas + WASM implementation
│   ├── mobile.rs           # Mobile: WebView implementation
│   ├── input.rs            # Input types (PointerEvent, RaycastConfig, etc.)
│   ├── selection.rs        # Selection types and logic
│   └── gizmos.rs           # Gizmo types and configuration
├── shaders/
│   ├── water.frag          # Water wave effect
│   ├── fire.frag           # Fire effect
│   ├── gradient.frag       # Color gradient
│   └── pulse.frag          # Pulsing animation
├── examples/
│   ├── demo/               # Desktop demo
│   ├── web-demo/           # Web/WASM demo
│   └── mobile-demo/        # Mobile demo
└── docs/                   # Documentation
```

## Performance Considerations

### Preventing Reloads During Gizmo Drag

**Critical**: Do not bake `transform_overrides` into `props.models`.

❌ Bad (causes full reload every frame):
```rust
let model_configs = models.read().iter().enumerate().map(|(i, m)| {
    let mut config = m.config.clone();
    if let Some(ovr) = overrides.get(&i) {
        config.pos_x = ovr.position.x;
    }
    config
}).collect::<Vec<_>>();
```

✅ Good (stable configs):
```rust
let model_configs = models.read().iter().map(|m| m.config.clone()).collect::<Vec<_>>();
```

The gizmo directly manipulates JS-side objects. Overrides are only for UI readout and persistence.

### Other Considerations

- **Model size** - Large models may take time to download/parse
- **Shader complexity** - Complex fragment shaders impact FPS
- **Multiple views** - Each WebView is a separate process (Desktop)
- **Memory** - Three.js scene holds GPU resources

## Security Notes

- Models loaded from external URLs (CORS dependent)
- JavaScript runs in isolated WebView
- No eval() or dynamic code execution from user input
- Shader code is sanitized (basic HTML escaping)

## Future Enhancements

Potential improvements:
1. **Texture Support** - Load custom textures via URLs
2. **Lighting Controls** - Adjustable lights (directional, point, ambient)
3. **Post-processing** - Bloom, DOF, SSAO effects
4. **Animation Clips** - Play skeletal animations from glTF/FBX
5. **Performance** - Virtual scrolling for multiple views
6. **Offline Mode** - Bundle Three.js instead of CDN
7. **Shader Hot-reload** - Edit shaders and see changes live
