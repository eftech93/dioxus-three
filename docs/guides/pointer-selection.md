# Pointer Events & Selection

Phase 1 features for interacting with 3D objects through mouse/touch input.

## Overview

Dioxus Three now supports:
- **Raycasting**: Detect which 3D objects are under the cursor
- **Selection**: Track selected objects with visual feedback
- **Gizmos**: Visual handles for transforming selected objects

## Basic Raycasting

Detect clicks on 3D objects:

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, PointerEvent};

fn app() -> Element {
    let mut selected = use_signal(|| None::<EntityId>);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            ThreeView {
                models: models(),
                
                on_pointer_down: move |event: PointerEvent| {
                    if let Some(hit) = event.hit {
                        println!("Clicked on entity {:?}", hit.entity_id);
                        println!("At world position: {:?}", hit.point);
                        selected.set(Some(hit.entity_id));
                    } else {
                        // Clicked on empty space
                        selected.set(None);
                    }
                },
            }
        }
    }
}
```

## Selection System

Track multiple selected objects:

```rust
use dioxus_three::{Selection, SelectionMode, SelectionStyle};

fn app() -> Element {
    let mut selection = use_signal(|| Selection::new());
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 300px; padding: 20px;",
                h3 { "Selected: {selection().count()}" }
                button { onclick: move |_| selection.write().clear(), "Clear" }
                
                for id in selection().iter() {
                    div { "Entity: {id}" }
                }
            }
            
            ThreeView {
                models: models(),
                selection: Some(selection()),
                selection_mode: SelectionMode::Multiple,
                selection_style: SelectionStyle {
                    outline: true,
                    outline_color: "#DEC647".to_string(),
                    outline_width: 2.0,
                    highlight: true,
                    highlight_color: "#DEC647".to_string(),
                    highlight_opacity: 0.3,
                    show_gizmo: true,
                },
                
                on_selection_change: move |new_selection: Selection| {
                    selection.set(new_selection);
                },
            }
        }
    }
}
```

## Transform Gizmos

Visual handles for moving/rotating/scaling:

```rust
use dioxus_three::{Gizmo, GizmoMode, GizmoSpace};

fn app() -> Element {
    let mut selected = use_signal(|| None::<EntityId>);
    let mut gizmo_mode = use_signal(|| GizmoMode::Translate);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Mode selector
            div { style: "position: absolute; top: 20px; left: 50%; z-index: 10;",
                button { onclick: move |_| gizmo_mode.set(GizmoMode::Translate), "Move" }
                button { onclick: move |_| gizmo_mode.set(GizmoMode::Rotate), "Rotate" }
                button { onclick: move |_| gizmo_mode.set(GizmoMode::Scale), "Scale" }
            }
            
            ThreeView {
                models: models(),
                
                gizmo: selected().map(|id| Gizmo {
                    target: id,
                    mode: gizmo_mode(),
                    space: GizmoSpace::World,
                    size: 1.0,
                    show_x: true,
                    show_y: true,
                    show_z: true,
                    show_xyz: true,
                    show_planes: true,
                }),
                
                on_gizmo_drag: move |event: GizmoEvent| {
                    println!("Transforming {:?}", event.target);
                    println!("New position: {:?}", event.transform.position);
                    
                    if event.is_finished {
                        println!("Drag completed");
                    }
                },
                
                on_pointer_down: move |e| selected.set(e.hit.map(|h| h.entity_id)),
            }
        }
    }
}
```

## Complete Example

Interactive scene with full control:

```rust
fn app() -> Element {
    let mut selection = use_signal(|| Selection::new());
    let mut mode = use_signal(|| GizmoMode::Translate);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Sidebar
            div { style: "width: 300px; padding: 20px; background: #1a1a2e; color: white;",
                h2 { "Scene Editor" }
                
                h3 { "Gizmo Mode" }
                button { onclick: move |_| mode.set(GizmoMode::Translate), "Translate (T)" }
                button { onclick: move |_| mode.set(GizmoMode::Rotate), "Rotate (R)" }
                button { onclick: move |_| mode.set(GizmoMode::Scale), "Scale (S)" }
                
                h3 { "Selected ({selection().count()})" }
                button { onclick: move |_| selection.write().clear(), "Clear Selection" }
                
                for id in selection().iter() {
                    div { "Entity: {id:?}" }
                }
            }
            
            // 3D View
            ThreeView {
                models: vec![
                    ModelConfig::new("cube1.glb", ModelFormat::Gltf)
                        .with_position(-2.0, 0.0, 0.0),
                    ModelConfig::new("cube2.glb", ModelFormat::Gltf)
                        .with_position(2.0, 0.0, 0.0),
                ],
                
                selection: Some(selection()),
                selection_mode: SelectionMode::Multiple,
                
                gizmo: selection().primary().map(|id| Gizmo::new(id)
                    .with_mode(mode())
                    .with_space(GizmoSpace::World)
                ),
                
                on_selection_change: move |s| selection.set(s),
            }
        }
    }
}
```

## API Reference

### PointerEvent

| Field | Type | Description |
|-------|------|-------------|
| `hit` | `Option<HitInfo>` | Raycast hit information |
| `screen_position` | `Vector2` | Cursor position in pixels |
| `ndc_position` | `Vector2` | Normalized device coords (-1 to 1) |
| `button` | `Option<MouseButton>` | Which button triggered |
| `shift_key` | `bool` | Shift pressed |
| `ctrl_key` | `bool` | Ctrl/Cmd pressed |
| `alt_key` | `bool` | Alt pressed |

### HitInfo

| Field | Type | Description |
|-------|------|-------------|
| `entity_id` | `EntityId` | The hit entity |
| `point` | `Vector3` | World position of hit |
| `normal` | `Vector3` | Surface normal |
| `uv` | `Option<Vector2>` | UV coordinates |
| `distance` | `f32` | Distance from camera |
| `face_index` | `Option<usize>` | Triangle index |
| `instance_id` | `Option<usize>` | For instanced meshes |

### SelectionMode

- `Single` - One selection at a time
- `Multiple` - Ctrl+click to multi-select
- `Toggle` - Click toggles selection

### GizmoMode

- `Translate` - Move objects
- `Rotate` - Rotate objects
- `Scale` - Scale objects

### GizmoSpace

- `World` - Transform in world coordinates
- `Local` - Transform in object coordinates
