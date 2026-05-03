# API Reference

Reference documentation for Dioxus Three types, components, and configuration.

## Components

- [ThreeView](threeview.md) - Main 3D viewer component with all props

## Types

- [ModelFormat](modelformat.md) - Supported 3D model formats
- [ShaderPreset](shaderpreset.md) - Built-in shader effects
- [ShaderConfig](shaderconfig.md) - Custom shader configuration

## Rust Types (from `dioxus_three` crate)

### Selection

```rust
pub struct Selection {
    // Internal HashSet of selected entities
}

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
    pub target: EntityId,
    pub mode: GizmoMode,    // Translate | Rotate | Scale
    pub space: GizmoSpace,  // World | Local
    pub size: f32,
    pub show_x: bool,
    pub show_y: bool,
    pub show_z: bool,
    pub show_xyz: bool,     // Uniform scale handle (scale mode only)
    pub show_planes: bool,  // Plane handles (translate mode only)
}

impl Gizmo {
    pub fn new(target: EntityId) -> Self;
    pub fn with_mode(self, mode: GizmoMode) -> Self;
    pub fn with_space(self, space: GizmoSpace) -> Self;
}
```

### PointerEvent

```rust
pub struct PointerEvent {
    pub hit: Option<HitInfo>,
    pub screen_position: Vector2,  // Screen coordinates in pixels
    pub ndc_position: Vector2,     // Normalized device coordinates (-1 to 1)
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

### RaycastConfig

```rust
pub struct RaycastConfig {
    pub enabled: bool,
    pub recursive: bool,
    pub max_distance: f32,
    pub layer_mask: Option<u32>,
}
```

Default: `enabled: true`, `recursive: true`, `max_distance: 1000.0`, `layer_mask: None`.

### ModelConfig

```rust
pub struct ModelConfig {
    pub url: String,           // Path or URL to model file
    pub format: ModelFormat,   // Model format
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,            // Degrees
    pub rot_y: f32,            // Degrees
    pub rot_z: f32,            // Degrees
    pub scale: f32,
    pub color: String,         // Hex color
}

impl ModelConfig {
    pub fn new(url: impl Into<String>, format: ModelFormat) -> Self;
    pub fn with_position(self, x: f32, y: f32, z: f32) -> Self;
    pub fn with_rotation(self, x: f32, y: f32, z: f32) -> Self;
    pub fn with_scale(self, scale: f32) -> Self;
    pub fn with_color(self, color: impl Into<String>) -> Self;
}
```
