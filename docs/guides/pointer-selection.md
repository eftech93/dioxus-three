# Pointer Events & Selection

Dioxus Three provides pointer event handling and object selection across all platforms.

## Overview

The interaction system consists of:
- **Raycasting**: Hit detection for pointer events
- **Selection**: Click to select, multi-select with Shift
- **Pointer Events**: `on_pointer_down`, `on_pointer_move`, `on_pointer_up`

## Enabling Raycasting

Raycasting is enabled by default. To configure it:

```rust
ThreeView {
    raycast: RaycastConfig {
        enabled: true,
        recursive: true,
        max_distance: 100.0,
        layer_mask: None,
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
        selection: Some(selection()),
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
    // Shift+click to add/remove from selection
}
```

### Selection Styling

```rust
ThreeView {
    selection_style: SelectionStyle {
        outline: true,
        outline_color: "#DEC647".to_string(),
        outline_width: 2.0,
        highlight: true,
        highlight_color: "#DEC647".to_string(),
        highlight_opacity: 0.3,
        show_gizmo: true,
    },
}
```

The default selection visual is a wireframe box + inner glow around the selected object. The outline scales with the object.

### Selection API

```rust
let sel = Selection::empty();              // No selection
let sel = Selection::with_mode(SelectionMode::Single);
sel.select(EntityId(0));                   // Select entity 0
let primary = sel.primary();               // Option<EntityId>
let is_selected = sel.is_selected(EntityId(0)); // bool
```

## Pointer Events

### Basic Pointer Events

```rust
ThreeView {
    id: Some("main-view".to_string()),
    on_pointer_down: move |event: PointerEvent| {
        println!("Down at {:?}", event.screen_position);
        if let Some(hit) = event.hit {
            println!("Hit entity: {:?}", hit.entity_id);
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
    pub hit: Option<HitInfo>,           // Hitted entity info (if any)
    pub screen_position: Vector2,       // Screen coordinates in pixels
    pub ndc_position: Vector2,          // NDC (-1 to 1)
    pub button: Option<MouseButton>,    // Left | Right | Middle
    pub shift_key: bool,
    pub ctrl_key: bool,
    pub alt_key: bool,
}

pub struct HitInfo {
    pub entity_id: EntityId,
    pub point: Vector3,
    pub normal: Vector3,
    pub uv: Option<Vector2>,
    pub distance: f32,
}
```

### Pointer Drag Events

```rust
ThreeView {
    on_pointer_drag: move |event: PointerDragEvent| {
        println!("Drag delta: {:?}", event.delta);
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
        ModelConfig::new("model.glb", ModelFormat::Glb)
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
                selection: Some(selection()),
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
                    .unwrap_or_else(|| {
                        let m = &models.read()[primary.0].config;
                        GizmoTransform {
                            position: Vector3::new(m.pos_x, m.pos_y, m.pos_z),
                            rotation: Vector3::new(m.rot_x.to_radians(), m.rot_y.to_radians(), m.rot_z.to_radians()),
                            scale: Vector3::new(m.scale, m.scale, m.scale),
                        }
                    });

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
- Gizmo handles render on top of objects (no occlusion)
- Events bridged via `document::eval` + `postMessage`

### Web
- Selection via manual raycasting in WASM
- Gizmo interaction via custom-built handles
- Gizmo handles render on top of objects (no occlusion)
- Events bridged via `wasm_bindgen` closures

### Mobile
- Same as Desktop (WebView-based)
- Touch events not yet fully tested
