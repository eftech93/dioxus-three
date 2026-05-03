# Dioxus Three Documentation

Welcome to the Dioxus Three documentation.

## What is Dioxus Three?

A Dioxus component that wraps Three.js, providing a cross-platform 3D viewer for Rust applications. Works on Desktop (WebView), Web (WASM), and Mobile (WebView).

## Installation

```toml
[dependencies]
dioxus-three = "0.0.3"
dioxus = "0.6"
```

## Feature Overview

| Feature | Status | Notes |
|---------|--------|-------|
| Model loading (GLB/GLTF/OBJ/FBX/STL/PLY/Collada/3MF) | ✅ Ready | Multiple formats supported |
| Multiple models per scene | ✅ Ready | Compose scenes from multiple models |
| Camera control (orbit, position, target) | ✅ Ready | Full orbit controls |
| Custom shaders | ✅ Ready | Vertex + fragment shader support |
| Desktop (macOS, Linux, Windows) | ✅ Ready | WebView iframe with Three.js |
| Web (WASM) | ✅ Ready | Native canvas rendering |
| Mobile (iOS, Android) | 🔄 Partial | WebView-based |
| **Object selection** | ✅ **v0.0.3** | Click to select, multi-select with Shift |
| **Transform gizmos** | ✅ **v0.0.3** | Translate, Rotate, Scale handles |
| **Pointer events** | ✅ **v0.0.3** | `on_pointer_down`/`move`/`up` callbacks |
| **Raycasting** | ✅ **v0.0.3** | Hit detection for clicks and hovers |
| Animation | 🔄 Planned | Keyframe and procedural |
| Physics | 🔄 Planned | Rapier.js integration |
| Post-processing | 🔄 Planned | Bloom, SSAO, etc. |

## Guides

### Getting Started
- [Quick Start](guides/quickstart.md) - Step-by-step first scene
- [Loading Models](guides/models.md) - Load single and multiple 3D models
- [Camera Control](guides/camera.md) - Camera position, target, presets
- [Managing Scene Properties](guides/scene-properties.md) - Reactive scene control with signals

### Input & Interaction
- [Pointer Events & Selection](guides/pointer-selection.md) - Click, select, gizmos
- [Transform Gizmos](guides/transform.md) - Translate, rotate, scale objects

### Advanced
- [Custom Shaders](guides/shaders.md) - Writing vertex/fragment shaders
- [Architecture](guides/architecture.md) - How Dioxus Three works internally

## API Reference

- [ThreeView Component](api/threeview.md) - All props and configuration options
- [ModelFormat](api/modelformat.md) - Supported model formats
- [ShaderPreset](api/shaderpreset.md) - Built-in shader effects
- [ShaderConfig](api/shaderconfig.md) - Custom shader configuration

## Platform Notes

### Desktop

- Uses WebView iframe with Three.js from CDN
- Gizmos via official `THREE.TransformControls`
- Events bridged via `document::eval` + `postMessage`
- State updates sent without iframe reload
- Gizmo handles render on top via `depthTest: false`

### Web

- Renders to native `<canvas>` via WASM
- Custom-built gizmos with manual raycasting
- Bridge via `wasm_bindgen` closures
- Same selection/gizmo features as Desktop
- Gizmo handles render on top via `depthTest: false`

### Mobile

- WebView approach similar to Desktop
- Gizmos available but not yet fully tested

## Migration

### From v0.0.2 to v0.0.3

v0.0.3 adds Phase 1 interaction features:

- **New**: `selection`, `selection_mode`, `selection_style`, `on_selection_change`
- **New**: `gizmo`, `on_gizmo_drag`
- **New**: `raycast`, `on_pointer_down`, `on_pointer_move`, `on_pointer_up`, `on_pointer_drag`, `on_gesture`
- **New**: `id` prop for event routing (recommended when using pointer events)

Existing props remain unchanged. Upgrade is additive.

## Changelog

See [changelog.md](changelog.md) for version history.

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md).
