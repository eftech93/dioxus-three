# ThreeView Component API

The main component for rendering a Three.js 3D scene in Dioxus.

## Basic Usage

```rust
use dioxus::prelude::*;
use dioxus_three::{ThreeView, ModelFormat};

fn App() -> Element {
    rsx! {
        ThreeView {
            model_url: Some("model.glb".to_string()),
            format: ModelFormat::Glb,
        }
    }
}
```

## Props

### Model Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `model_url` | `Option<String>` | `None` | Path or URL to a 3D model file (single-model mode) |
| `format` | `ModelFormat` | `Cube` | File format: `Obj`, `Fbx`, `Gltf`, `Glb`, `Stl`, `Ply`, `Dae`, `Json`, `Cube` |
| `pos_x` | `f32` | `0.0` | Model position X |
| `pos_y` | `f32` | `0.0` | Model position Y |
| `pos_z` | `f32` | `0.0` | Model position Z |
| `rot_x` | `f32` | `0.0` | Model rotation X (degrees) |
| `rot_y` | `f32` | `0.0` | Model rotation Y (degrees) |
| `rot_z` | `f32` | `0.0` | Model rotation Z (degrees) |
| `scale` | `f32` | `1.0` | Model uniform scale |
| `color` | `String` | `"#ff6b6b"` | Material color (hex string) |
| `models` | `Vec<ModelConfig>` | `[]` | Multiple models (overrides `model_url`/`format` when set) |
| `auto_center` | `bool` | `true` | Auto-center the model on load |
| `auto_scale` | `bool` | `false` | Auto-scale to fit viewport |

### Camera Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `cam_x` | `f32` | `5.0` | Camera position X |
| `cam_y` | `f32` | `5.0` | Camera position Y |
| `cam_z` | `f32` | `5.0` | Camera position Z |
| `target_x` | `f32` | `0.0` | Camera look-at target X |
| `target_y` | `f32` | `0.0` | Camera look-at target Y |
| `target_z` | `f32` | `0.0` | Camera look-at target Z |
| `auto_rotate` | `bool` | `true` | Auto-rotate the camera around the scene |
| `rot_speed` | `f32` | `1.0` | Auto-rotation speed |

### Scene Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `show_grid` | `bool` | `true` | Show grid helper |
| `show_axes` | `bool` | `true` | Show axes helper |
| `background` | `String` | `"#1a1a2e"` | Background color (hex) |
| `shadows` | `bool` | `true` | Enable shadows |
| `wireframe` | `bool` | `false` | Render in wireframe mode |
| `shader` | `ShaderPreset` | `None` | Custom shader preset |
| `class` | `String` | `""` | Additional CSS class for container |

### Selection Props (v0.0.3+)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `selection` | `Option<Selection>` | `None` | Current selection state |
| `selection_mode` | `SelectionMode` | `Single` | `Single` or `Multiple` selection mode |
| `selection_style` | `SelectionStyle` | `default()` | Visual style for selection outline |
| `on_selection_change` | `Option<Callback<Selection>>` | `None` | Called when selection changes |

### Gizmo Props (v0.0.3+)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `gizmo` | `Option<Gizmo>` | `None` | Gizmo configuration for the selected object |
| `on_gizmo_drag` | `Option<Callback<GizmoEvent>>` | `None` | Called during gizmo drag (and on finish) |

### Pointer Event Props (v0.0.3+)

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `id` | `Option<String>` | `None` | Unique ID for event routing (recommended when using pointer events) |
| `raycast` | `RaycastConfig` | `default()` | Raycast configuration |
| `on_pointer_down` | `Option<Callback<PointerEvent>>` | `None` | Called on pointer down |
| `on_pointer_move` | `Option<Callback<PointerEvent>>` | `None` | Called on pointer move (hover) |
| `on_pointer_up` | `Option<Callback<PointerEvent>>` | `None` | Called on pointer up |
| `on_pointer_drag` | `Option<Callback<PointerDragEvent>>` | `None` | Called during pointer drag |
| `on_gesture` | `Option<Callback<GestureEvent>>` | `None` | Called on gesture events (pinch, rotate, pan) |

### Debug Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `debug` | `bool` | `false` | Enable debug overlay |

## Types

### Selection

```rust
pub struct Selection;

impl Selection {
    pub fn empty() -> Self;
    pub fn with_mode(mode: SelectionMode) -> Self;
    pub fn is_selected(&self, entity: EntityId) -> bool;
    pub fn select(&mut self, entity: EntityId);
    pub fn toggle(&mut self, entity: EntityId);
    pub fn deselect(&mut self, entity: EntityId);
    pub fn clear(&mut self);
    pub fn count(&self) -> usize;
    pub fn has_selection(&self) -> bool;
    pub fn primary(&self) -> Option<EntityId>;
    pub fn iter(&self) -> impl Iterator<Item = EntityId> + '_;
}
```

### Gizmo

```rust
pub struct Gizmo {
    pub target: EntityId,       // Entity this gizmo is attached to
    pub mode: GizmoMode,        // Translate | Rotate | Scale
    pub space: GizmoSpace,      // World | Local
    pub size: f32,              // Visual size of the gizmo
    pub show_x: bool,           // Show X axis handle
    pub show_y: bool,           // Show Y axis handle
    pub show_z: bool,           // Show Z axis handle
    pub show_xyz: bool,         // Show uniform scale handle (scale mode)
    pub show_planes: bool,      // Show plane handles (translate mode)
}

impl Gizmo {
    pub fn new(target: EntityId) -> Self;
    pub fn with_mode(self, mode: GizmoMode) -> Self;
    pub fn with_space(self, space: GizmoSpace) -> Self;
}
```

### GizmoEvent

```rust
pub struct GizmoEvent {
    pub target: EntityId,
    pub transform: GizmoTransform,  // Current position, rotation, scale
    pub mode: GizmoMode,
    pub is_finished: bool,          // true when drag ends
}
```

### GizmoTransform

```rust
pub struct GizmoTransform {
    pub position: Vector3,
    pub rotation: Vector3,  // Euler angles in radians
    pub scale: Vector3,
}
```

### PointerEvent

```rust
pub struct PointerEvent {
    pub hit: Option<HitInfo>,
    pub screen_position: Vector2,   // Screen coordinates (pixels)
    pub ndc_position: Vector2,      // Normalized device coordinates (-1 to 1)
    pub button: Option<MouseButton>,
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
    pub face_index: Option<usize>,
    pub instance_id: Option<usize>,
}
```

### PointerDragEvent

```rust
pub struct PointerDragEvent {
    pub hit: Option<HitInfo>,
    pub start_hit: Option<HitInfo>,
    pub screen_position: Vector2,
    pub start_screen_position: Vector2,
    pub world_position: Vector3,
    pub start_world_position: Vector3,
    pub delta: Vector2,
    pub total_delta: Vector2,
    pub button: MouseButton,
}
```

### RaycastConfig

```rust
pub struct RaycastConfig {
    pub enabled: bool,
    pub recursive: bool,
    pub max_distance: f32,
    pub layer_mask: Option<u32>,
}

impl Default for RaycastConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            recursive: true,
            max_distance: 1000.0,
            layer_mask: None,
        }
    }
}
```

### SelectionStyle

```rust
pub struct SelectionStyle {
    pub outline: bool,
    pub outline_color: String,      // Hex color
    pub outline_width: f32,
    pub highlight: bool,
    pub highlight_color: String,    // Hex color
    pub highlight_opacity: f32,
    pub show_gizmo: bool,
}
```

## Examples

### Selection + Gizmo

```rust
#[component]
fn SceneWithGizmo() -> Element {
    let mut selection = use_signal(|| Selection::empty());
    let mut gizmo = use_signal(|| None::<Gizmo>);

    rsx! {
        ThreeView {
            models: vec![
                ModelConfig::new("model.glb", ModelFormat::Glb)
            ],
            selection: Some(selection()),
            on_selection_change: move |sel| {
                selection.set(sel.clone());
                gizmo.set(sel.primary().map(|id| Gizmo::new(id)));
            },
            gizmo: gizmo(),
            on_gizmo_drag: move |event: GizmoEvent| {
                println!("{:?} -> {:?}", event.mode, event.transform);
                if event.is_finished {
                    // Persist transform
                }
            },
        }
    }
}
```

### Pointer Events

```rust
#[component]
fn SceneWithPointerEvents() -> Element {
    rsx! {
        ThreeView {
            id: Some("main-view".to_string()),
            model_url: Some("model.glb".to_string()),
            on_pointer_down: move |event| {
                if let Some(hit) = event.hit {
                    println!("Clicked entity: {:?}", hit.entity_id);
                }
            },
            on_pointer_move: move |event| {
                if let Some(hit) = event.hit {
                    println!("Hovering: {:?}", hit.entity_id);
                }
            },
        }
    }
}
```
