# Dioxus Three

A Dioxus wrapper around Three.js for building cross-platform 3D applications in Rust.

**⚠️ Pre-alpha**: Active development. APIs will change.

## Overview

Dioxus Three provides a Dioxus component that embeds a Three.js 3D viewer, allowing you to:

- Load and display 3D models (GLB, GLTF, OBJ, FBX, STL, PLY, Collada, 3MF)
- Render multiple models in a single scene
- Apply custom vertex/fragment shaders
- Control camera, lighting, and scene settings
- Interact with the scene via pointer events and selection
- Manipulate objects with transform gizmos (translate, rotate, scale)

## Features

| Feature | Status | Description |
|---------|--------|-------------|
| Model Loading | ✅ Ready | GLB, GLTF, OBJ, FBX, STL, PLY, Collada, 3MF |
| Multiple Models | ✅ Ready | Render multiple models in one scene |
| Camera Control | ✅ Ready | Orbit controls, custom positions |
| Shader Support | ✅ Ready | Custom vertex/fragment shaders |
| Desktop (WebView) | ✅ Ready | Tauri/WebView on macOS, Linux, Windows |
| Web (WASM) | ✅ Ready | WASM + Three.js canvas |
| Mobile (WebView) | 🔄 Partial | WebView-based, iOS/Android |
| **Selection** | ✅ **v0.0.3** | Click to select objects, multi-select with Shift |
| **Gizmos** | ✅ **v0.0.3** | Translate, Rotate, Scale handles |
| **Pointer Events** | ✅ **v0.0.3** | `on_pointer_down`, `on_pointer_move`, `on_pointer_up` |
| **Raycasting** | ✅ **v0.0.3** | Hit detection for clicks and hovers |
| Animation | 🔄 Planned | Keyframe and procedural animation |
| Physics | 🔄 Planned | Rapier.js integration |
| Post-Processing | 🔄 Planned | Bloom, SSAO, etc. |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-three = "0.0.4"
dioxus = "0.6"
```

Or use the git version for the latest:

```toml
[dependencies]
dioxus-three = { git = "https://github.com/eftech93/dioxus-three" }
```

## Quick Start

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelFormat};

fn App() -> Element {
    rsx! {
        ThreeView {
            model_url: Some("model.glb".to_string()),
            format: ModelFormat::Glb,
            cam_x: 2.0,
            cam_y: 2.0,
            cam_z: 5.0,
        }
    }
}
```

## Examples

### Selection & Gizmos

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelFormat, ModelConfig, Selection, Gizmo, GizmoMode, GizmoEvent, SelectionMode};

#[component]
fn App() -> Element {
    let mut selection = use_signal(|| Selection::empty());
    let mut gizmo = use_signal(|| None::<Gizmo>);

    rsx! {
        ThreeView {
            models: vec![
                ModelConfig::new("model.glb", ModelFormat::Glb)
            ],
            selection: Some(selection()),
            selection_mode: SelectionMode::Single,
            on_selection_change: move |sel| {
                selection.set(sel.clone());
                if let Some(id) = sel.primary() {
                    gizmo.set(Some(Gizmo::new(id).with_mode(GizmoMode::Translate)));
                } else {
                    gizmo.set(None);
                }
            },
            gizmo: gizmo(),
            on_gizmo_drag: move |event: GizmoEvent| {
                println!("Gizmo drag: {:?}", event.transform);
                if event.is_finished {
                    // Persist the transform to your app state
                }
            },
        }
    }
}
```

### Custom Shader

```rust
ThreeView {
    model_url: Some("model.glb".to_string()),
    shader: ShaderPreset::Custom {
        vertex: include_str!("shaders/vertex.glsl"),
        fragment: include_str!("shaders/fragment.glsl"),
    },
}
```

### Multiple Models

```rust
ThreeView {
    models: vec![
        ModelConfig::new("car.glb", ModelFormat::Glb)
            .with_position(-2.0, 0.0, 0.0)
            .with_color("#ff6b6b"),
        ModelConfig::new("wheel.glb", ModelFormat::Glb)
            .with_position(2.0, 0.0, 0.0)
            .with_color("#4ecdc4"),
    ],
}
```

## Platform Notes

### Desktop (macOS, Linux, Windows)

Uses a WebView iframe with Three.js loaded from CDN. Includes:
- Full orbit camera controls
- **Official `THREE.TransformControls`** for gizmos (translate, rotate, scale)
- Pointer events via `document::eval` bridge
- **State updates via `postMessage`** (no iframe reload on camera/selection/gizmo changes)
- Selection outline with wireframe box + inner glow
- Gizmo handles always render on top via `depthTest: false`

### Web (WASM)

Renders Three.js directly to a `<canvas>` element:
- Native canvas rendering
- **Custom-built gizmos** using Three.js primitives (arrows, tori, boxes)
- Manual raycasting and drag math
- Bridge via `wasm_bindgen` closures (`dioxusThreeRustBridge`)
- Same selection and gizmo features as Desktop
- Gizmo handles always render on top via `depthTest: false`

### Mobile

Uses WebView approach similar to Desktop. Gizmo features available but not yet fully tested.

## Documentation

- [API Reference](docs/api/README.md) - Component props and types
- [Guides](docs/guides/README.md) - How-to guides
- [Architecture](ARCHITECTURE.md) - Internal design
- [Changelog](docs/changelog.md) - Version history

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT License - see [LICENSE](LICENSE) file.
