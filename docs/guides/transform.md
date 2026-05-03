# Transform Controls

Control the position, rotation, and scale of your 3D models.

## Static Transforms

Set transforms via props:

### Position

```rust
rsx! {
    ThreeView {
        pos_x: 2.0,   // Move right
        pos_y: 1.0,   // Move up
        pos_z: -3.0,  // Move back
    }
}
```

### Rotation

```rust
rsx! {
    ThreeView {
        rot_x: 45.0,  // Pitch
        rot_y: 90.0,  // Yaw
        rot_z: 0.0,   // Roll
    }
}
```

### Scale

```rust
rsx! {
    ThreeView {
        scale: 2.0,  // Double size
    }
}
```

## Interactive Gizmos (v0.0.3+)

Manipulate objects interactively with visual drag handles:

### Basic Gizmo

```rust
#[component]
fn SceneWithGizmo() -> Element {
    let mut selection = use_signal(|| Selection::empty());
    let mut gizmo = use_signal(|| None::<Gizmo>);

    rsx! {
        ThreeView {
            models: vec![
                ModelConfig {
                    model_url: Some("model.glb".to_string()),
                    format: ModelFormat::Glb,
                    ..Default::default()
                }
            ],
            selection: selection(),
            on_selection_change: move |sel| {
                selection.set(sel.clone());
                gizmo.set(sel.primary().map(|id| Gizmo::new(id)));
            },
            gizmo: gizmo(),
        }
    }
}
```

Click the model to select it, then drag the gizmo handles to transform it.

### Gizmo Modes

Switch between Translate, Rotate, and Scale:

```rust
let mut gizmo = use_signal(|| Some(Gizmo::new(EntityId(0)).with_mode(GizmoMode::Translate)));

// Switch mode
rsx! {
    button { onclick: move |_| gizmo.set(gizmo().map(|g| g.with_mode(GizmoMode::Rotate))), "Rotate" }
    button { onclick: move |_| gizmo.set(gizmo().map(|g| g.with_mode(GizmoMode::Scale))), "Scale" }
    button { onclick: move |_| gizmo.set(gizmo().map(|g| g.with_mode(GizmoMode::Translate))), "Translate" }
}
```

### Gizmo Space

```rust
Gizmo::new(EntityId(0))
    .with_mode(GizmoMode::Rotate)
    .with_space(GizmoSpace::Local)  // Local object space
    // or .with_space(GizmoSpace::World)  // World space
```

### Gizmo Visibility

Control which handles are visible:

```rust
Gizmo {
    target: EntityId(0),
    mode: GizmoMode::Translate,
    show_x: true,
    show_y: true,
    show_z: true,
    show_planes: true,   // Plane handles (translate only)
    show_xyz: true,      // Uniform scale handle (scale only)
    size: 1.5,           // Visual size
    ..Default::default()
}
```

### Handling Gizmo Drag Events

```rust
let mut transform_overrides = use_signal(|| HashMap::<usize, GizmoTransform>::new());

ThreeView {
    gizmo: gizmo(),
    on_gizmo_drag: move |event: GizmoEvent| {
        // Live transform during drag
        println!("{:?}: {:?}", event.mode, event.transform);
        
        // Store for UI readout
        transform_overrides.write().insert(event.target.0, event.transform);
        
        // Persist when drag finishes
        if event.is_finished {
            println!("Final: {:?}", event.transform);
            // Write back to your app state here
        }
    },
}
```

### Transform Readout UI

Display live transform values:

```rust
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
```

## Platform Differences

| Feature | Desktop | Web |
|---------|---------|-----|
| Gizmo implementation | `THREE.TransformControls` | Custom-built handles |
| Translate drag | Plane snapping (official) | Camera-facing plane intersection |
| Rotate drag | Arcball (official) | Arcball rotation |
| Scale drag | Relative scaling (official) | Distance-based scaling |
| Event bridge | `document::eval` + `postMessage` | `wasm_bindgen` closures |

## Performance Notes

**Critical**: Do not bake `transform_overrides` into `props.models`.

❌ Bad — causes full scene reload every frame during drag:
```rust
let model_configs = models.read().iter().enumerate().map(|(i, m)| {
    let mut config = m.config.clone();
    if let Some(ovr) = overrides.get(&i) {
        config.pos_x = ovr.position.x;
    }
    config
}).collect::<Vec<_>>();
```

✅ Good — stable configs, no reload:
```rust
let model_configs = models.read().iter().map(|m| m.config.clone()).collect::<Vec<_>>();
```

The gizmo directly manipulates JS-side objects. Overrides are only for UI readout and persistence.

## Interactive Rotation (Manual)

```rust
fn app() -> Element {
    let mut rot_x = use_signal(|| 0.0f32);
    let mut rot_y = use_signal(|| 0.0f32);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 250px; padding: 20px;",
                input {
                    r#type: "range",
                    min: "0",
                    max: "360",
                    value: "{rot_x()}",
                    oninput: move |e| rot_x.set(e.value().parse().unwrap_or(0.0))
                }
                input {
                    r#type: "range",
                    min: "0",
                    max: "360",
                    value: "{rot_y()}",
                    oninput: move |e| rot_y.set(e.value().parse().unwrap_or(0.0))
                }
            }
            
            ThreeView {
                rot_x: rot_x(),
                rot_y: rot_y(),
                auto_rotate: false,  // Disable auto-rotation for manual control
            }
        }
    }
}
```

## Dynamic Scaling

```rust
fn app() -> Element {
    let mut scale = use_signal(|| 1.0f32);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 200px; padding: 20px;",
                button { onclick: move |_| scale.set(scale() * 1.1), "Zoom In" }
                button { onclick: move |_| scale.set(scale() * 0.9), "Zoom Out" }
                button { onclick: move |_| scale.set(1.0), "Reset" }
            }
            
            ThreeView {
                scale: scale(),
            }
        }
    }
}
```

## Combining Transformations

All transforms work together:

```rust
rsx! {
    ThreeView {
        // Position
        pos_x: 1.0,
        pos_y: 0.5,
        pos_z: 0.0,
        
        // Rotation
        rot_x: 30.0,
        rot_y: 45.0,
        rot_z: 0.0,
        
        // Scale
        scale: 1.5,
    }
}
```

## Auto-Rotate vs Manual Rotation

When `auto_rotate` is true, the Y rotation is overridden by the animation:

```rust
rsx! {
    // Manual control (auto_rotate off)
    ThreeView {
        auto_rotate: false,
        rot_y: 45.0,  // This works
    }
    
    // Auto rotation (overrides rot_y)
    ThreeView {
        auto_rotate: true,
        rot_y: 45.0,  // This is ignored during animation
        rot_speed: 2.0,  // Animation speed
    }
}
```

## Coordinate System

Dioxus Three uses a right-handed coordinate system:

- **X+** → Right
- **Y+** → Up  
- **Z+** → Towards viewer (out of screen)

## Reset Transform

Create a reset button:

```rust
fn app() -> Element {
    let mut pos = use_signal(|| (0.0f32, 0.0f32, 0.0f32));
    let mut rot = use_signal(|| (0.0f32, 0.0f32, 0.0f32));
    let mut scale = use_signal(|| 1.0f32);
    
    let reset = move || {
        pos.set((0.0, 0.0, 0.0));
        rot.set((0.0, 0.0, 0.0));
        scale.set(1.0);
    };
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            button { onclick: move |_| reset(), "Reset Transform" }
            
            ThreeView {
                pos_x: pos().0,
                pos_y: pos().1,
                pos_z: pos().2,
                rot_x: rot().0,
                rot_y: rot().1,
                rot_z: rot().2,
                scale: scale(),
            }
        }
    }
}
```
