# Architecture

Dioxus Three has three platform implementations that share a common Rust API but use different rendering strategies.

## High-Level Architecture

```
┌─────────────────────────────────────────┐
│           Your Dioxus App               │
│                                         │
│  ┌─────────────┐    ┌──────────────┐  │
│  │ ThreeView   │    │  App State   │  │
│  │  Component  │◄──►│ (Selection,  │  │
│  │             │    │  Transforms) │  │
│  └──────┬──────┘    └──────────────┘  │
│         │                               │
│  ┌──────┴──────┐    ┌──────────────┐  │
│  │   Bridge    │◄──►│  GizmoEvents │  │
│  │ (Platform-  │    │  PointerEvents│  │
│  │  specific)  │    │  Selection    │  │
│  └──────┬──────┘    └──────────────┘  │
└─────────┼───────────────────────────────┘
          │
    ┌─────┴─────┐
    │  Three.js │
    │  Renderer │
    └───────────┘
```

## Platform Implementations

### Desktop (`src/desktop.rs`)

Uses a WebView with an iframe containing a complete Three.js scene.

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

1. **HTML generated once**: The complete HTML document (including Three.js from CDN) is generated via `use_signal` only when the model count changes. This avoids expensive iframe reloads during interaction.

2. **State updates via `postMessage`**: Camera, selection, gizmo, and style updates are sent via `postMessage` to the iframe without regeneration.

3. **Event bridge via `document::eval`**: Pointer events, gizmo drag events, and selection changes are received from the iframe via `document::eval` polling.

4. **Official `THREE.TransformControls`**: Gizmos are the official Three.js controls, providing translate, rotate, and scale handles.

### Web (`src/web.rs`)

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

1. **Custom-built gizmos**: Translate, rotate, and scale handles are built from Three.js primitives (arrow cones, tori, boxes) with manual raycasting.

2. **Manual drag math**: 
   - **Translate**: Camera-facing plane intersection. A plane is created containing the drag axis, the mouse ray is intersected with this plane, and the delta is projected onto the axis.
   - **Rotate**: Arcball rotation around the axis.
   - **Scale**: Distance-based scaling along the axis.

3. **Bridge via `wasm_bindgen`**: Events are sent to Rust via `dioxusThreeRustBridge` JS function, which calls a WASM closure.

4. **Live state references**: The `updateGizmo` function reads `entityMap` from the canvas's live state object (`canvas.dioxusThreeState`) rather than captured closure variables, preventing stale references after model updates.

### Mobile (`src/mobile.rs`)

Uses the same WebView approach as Desktop. Implementation exists but gizmo features have not been fully tested.

## Shared Components

### `src/lib.rs`

Contains platform-independent code:
- `ThreeViewProps` struct with all component properties
- `ModelConfig`, `ShaderPreset` types
- `generate_three_js_html()` for desktop iframe HTML generation
- Model loading JS builders
- Selection, gizmo, and event types

### `src/input.rs`

Input system types:
- `EntityId`, `Vector3`
- `PointerEvent`, `PointerDragEvent`
- `GestureEvent`, `Modifiers`, `PointerButton`
- `RaycastConfig`

### `src/selection.rs`

Selection system:
- `Selection` struct (list of selected entities)
- `SelectionMode` (Single, Multiple)
- `SelectionStyle` (outline color, width, glow)

### `src/gizmos.rs`

Gizmo system:
- `Gizmo` struct (target, mode, space, size, visibility flags)
- `GizmoMode` (Translate, Rotate, Scale)
- `GizmoSpace` (World, Local)
- `GizmoEvent`, `GizmoTransform`

## Event Flow

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
         Update signals (selection, gizmo, transform_overrides)
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
JS pointerdown handler: raycaster.intersectObjects(gizmoGroup)
    │
    ▼
Check if hit is on gizmo handle
    │
    ├── Yes → Start gizmo drag mode
    │           On move: plane-intersection math
    │           On up: end drag, call dioxusThreeRustBridge("gizmoDrag", ...)
    │
    └── No  → raycaster.intersectObjects(modelContainer)
                  │
                  ▼
         Hit model → call dioxusThreeRustBridge("pointerDown", ...)
                  │
                  ▼
         Rust closure: Update selection signal
                  │
                  ▼
         Re-render
                  │
                  ▼
         ThreeView: updateGizmo() reads live entityMap
                  │
                  ▼
         Gizmo positioned at new target
```

## Model Loading

### Multi-Model Loading

Both platforms support loading multiple models into a single scene:

```rust
ThreeView {
    models: vec![
        ModelConfig { model_url: Some("a.glb".to_string()), pos_x: -2.0, ..Default::default() },
        ModelConfig { model_url: Some("b.glb".to_string()), pos_x: 2.0, ..Default::default() },
    ],
}
```

Each model gets an `entityId` stored in `userData` for raycast identification.

### Desktop Model Updates

On `update-state` postMessage, the desktop iframe:
1. Updates existing object transforms (position, rotation, scale)
2. Creates new cubes for added models
3. Removes objects for deleted models
4. Does NOT reload the entire scene

## Performance Considerations

### Preventing Reloads During Gizmo Drag

The critical performance optimization: **do not bake `transform_overrides` into `props.models`**.

❌ Bad (causes full reload every frame):
```rust
let model_configs = models.read().iter().enumerate().map(|(i, m)| {
    let mut config = m.config.clone();
    if let Some(ovr) = overrides.get(&i) {
        config.pos_x = ovr.position.x; // ... etc
    }
    config
}).collect::<Vec<_>>();

ThreeView { models: model_configs } // Changes every frame = reload
```

✅ Good (no reload during drag):
```rust
let model_configs = models.read().iter().map(|m| m.config.clone()).collect::<Vec<_>>();

ThreeView { models: model_configs } // Stable during drag
```

The gizmo directly manipulates JS-side Three.js objects. Overrides are only for UI readout and persistence on drag finish.

### Desktop: HTML Regeneration Triggers

HTML is regenerated ONLY when `props.models.len()` changes. All other prop changes use `postMessage`.

### Web: Defensive Model Comparison

The web implementation compares incoming models against cached state and only applies actual changes.
