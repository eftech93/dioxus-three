# Pointer Events & Selection

Dioxus Three provides pointer event handling and object selection across all platforms.

## Overview

The interaction system consists of:
- **Raycasting**: Hit detection for pointer events
- **Selection**: Click to select, multi-select with Ctrl/Cmd
- **Pointer Events**: `on_pointer_down`, `on_pointer_move`, `on_pointer_up`

## Enabling Raycasting

Raycasting is enabled by default. To configure it:

```rust
ThreeView {
    raycast: RaycastConfig {
        enabled: true,
        recursive: true,
        max_distance: 100.0,
        layer_mask: 0xFFFFFFFF,
    },
}
```

## Selection

### Basic Selection

```rust
let mut selection = use_signal(|| Selection::empty());

rsx! {
    ThreeView {
        models: models(),
        selection: selection(),
        selection_mode: SelectionMode::Single,
        on_selection_change: move |sel| {
            selection.set(sel);
        },
    }
}
```

### Multi-Selection

```rust
ThreeView {
    selection_mode: SelectionMode::Multiple,
    // Ctrl/Cmd+click to add/remove from selection
}
```

### Selection Styling

```rust
ThreeView {
    selection_style: SelectionStyle {
        outline_color: "#00ff88".to_string(),
        outline_width: 2.0,
        glow_color: "#00ff8844".to_string(),
        glow_size: 4.0,
    },
}
```

The default selection visual is a wireframe box + inner glow around the selected object.

### Selection API

```rust
let sel = Selection::empty();           // No selection
let sel = Selection::single(EntityId(0)); // Single selection
let primary = sel.primary();            // Option<EntityId>
let contains = sel.contains(EntityId(0)); // bool
```

## Pointer Events

### Basic Pointer Events

```rust
ThreeView {
    id: "main-view",  // Required for event routing
    on_pointer_down: move |event: PointerEvent| {
        println!("Down at {:?}", event.screen_position);
        if let Some(id) = event.entity_id {
            println!("Hit entity: {:?}", id);
        }
    },
    on_pointer_move: move |event: PointerEvent| {
        // Called on hover/drag
    },
    on_pointer_up: move |event: PointerEvent| {
        println!("Up at {:?}", event.screen_position);
    },
}
```

### Pointer Event Structure

```rust
pub struct PointerEvent {
    pub entity_id: Option<EntityId>,     // Hitted entity (if any)
    pub position: (f32, f32),            // NDC (-1 to 1)
    pub screen_position: (f32, f32),     // Pixels
    pub button: PointerButton,           // Left | Right | Middle
    pub modifiers: Modifiers,            // Shift | Ctrl | Alt | Meta
}
```

### Pointer Drag Events

```rust
ThreeView {
    on_pointer_drag: move |event: PointerDragEvent| {
        println!("Drag from {:?} to {:?}", event.start, event.current);
    },
}
```

## Complete Example: Selection + Transform Readout

```rust
#[component]
fn InteractiveScene() -> Element {
    let mut selection = use_signal(|| Selection::empty());
    let mut gizmo = use_signal(|| None::<Gizmo>);
    let mut transform_overrides = use_signal(|| HashMap::<usize, GizmoTransform>::new());

    let models = use_signal(|| vec![
        ModelWithTransform {
            config: ModelConfig {
                model_url: Some("model.glb".to_string()),
                format: ModelFormat::Glb,
                ..Default::default()
            },
        }
    ]);

    // Build model configs WITHOUT baking overrides to prevent reloads during drag
    let model_configs: Vec<ModelConfig> = models.read()
        .iter()
        .map(|m| m.config.clone())
        .collect();

    rsx! {
        div {
            ThreeView {
                models: model_configs,
                selection: selection(),
                selection_mode: SelectionMode::Single,
                on_selection_change: move |sel| {
                    selection.set(sel.clone());
                    gizmo.set(sel.primary().map(|id| Gizmo::new(id)));
                },
                gizmo: gizmo(),
                on_gizmo_drag: move |event: GizmoEvent| {
                    transform_overrides.write().insert(event.target.0, event.transform);
                    if event.is_finished {
                        // Persist: write back to your app state
                        println!("Final transform: {:?}", event.transform);
                    }
                },
            }

            // Transform readout UI
            if let Some(primary) = selection().primary() {
                let tf = transform_overrides.read()
                    .get(&primary.0)
                    .cloned()
                    .unwrap_or_else(|| GizmoTransform::from_model(&models.read()[primary.0].config));

                div { class: "transform-panel",
                    p { "Position: ({:.2}, {:.2}, {:.2})", tf.position.x, tf.position.y, tf.position.z }
                    p { "Rotation: ({:.2}, {:.2}, {:.2})", tf.rotation.x, tf.rotation.y, tf.rotation.z }
                    p { "Scale: ({:.2}, {:.2}, {:.2})", tf.scale.x, tf.scale.y, tf.scale.z }
                }
            }
        }
    }
}
```

### Important: Transform Persistence Pattern

When using gizmos, the gizmo directly manipulates JS-side Three.js objects. To keep the UI responsive and avoid full scene reloads:

1. **Pass raw configs** to `ThreeView` (without baking `transform_overrides` into `models`)
2. **Store overrides separately** in a `HashMap<EntityId, GizmoTransform>`
3. **Read from overrides** for UI display
4. **Persist on `is_finished`** by writing back to your canonical app state

Baking overrides into `props.models` causes the model config to change every frame during drag, triggering a full scene reload and choppy performance.

## Platform Behavior

### Desktop
- Selection via raycast against scene objects
- Gizmo interaction via `THREE.TransformControls`
- Events bridged via `document::eval` + `postMessage`

### Web
- Selection via manual raycasting in WASM
- Gizmo interaction via custom-built handles
- Events bridged via `wasm_bindgen` closures

### Mobile
- Same as Desktop (WebView-based)
- Touch events not yet fully tested
