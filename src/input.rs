//! Input handling and raycasting support for Dioxus Three
//!
//! Provides pointer event handling, raycasting for 3D object selection,
//! and gesture recognition for touch devices.


/// A unique identifier for entities in the 3D scene
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub usize);

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity({})", self.0)
    }
}

/// Information about a raycast hit
#[derive(Debug, Clone)]
pub struct HitInfo {
    /// The entity that was hit
    pub entity_id: EntityId,
    /// The intersection point in world coordinates
    pub point: Vector3,
    /// The surface normal at the intersection
    pub normal: Vector3,
    /// UV coordinates at the intersection point (if available)
    pub uv: Option<Vector2>,
    /// Distance from the ray origin to the hit point
    pub distance: f32,
    /// The index of the face that was hit (if applicable)
    pub face_index: Option<usize>,
    /// Instance ID for instanced meshes
    pub instance_id: Option<usize>,
}

/// 2D vector for UV coordinates, mouse positions, etc.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// 3D vector for positions, normals, etc.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const UP: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const FORWARD: Self = Self { x: 0.0, y: 0.0, z: 1.0 };
    
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }
    
    pub fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
}

/// Cursor styles for pointer events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorStyle {
    #[default]
    Default,
    Pointer,
    Grab,
    Grabbing,
    Crosshair,
    Move,
    Text,
    Wait,
    Help,
    None,
}

impl CursorStyle {
    pub fn as_css(&self) -> &'static str {
        match self {
            CursorStyle::Default => "default",
            CursorStyle::Pointer => "pointer",
            CursorStyle::Grab => "grab",
            CursorStyle::Grabbing => "grabbing",
            CursorStyle::Crosshair => "crosshair",
            CursorStyle::Move => "move",
            CursorStyle::Text => "text",
            CursorStyle::Wait => "wait",
            CursorStyle::Help => "help",
            CursorStyle::None => "none",
        }
    }
}

/// Configuration for raycasting behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaycastConfig {
    /// Enable raycasting
    pub enabled: bool,
    /// Whether to recursively check children
    pub recursive: bool,
    /// Maximum distance to check
    pub max_distance: f32,
    /// Layer mask for filtering (if implemented)
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

/// Pointer event data
#[derive(Debug, Clone)]
pub struct PointerEvent {
    /// The hit information (if any)
    pub hit: Option<HitInfo>,
    /// Screen position of the pointer
    pub screen_position: Vector2,
    /// Normalized device coordinates (-1 to 1)
    pub ndc_position: Vector2,
    /// Mouse button that triggered the event (if applicable)
    pub button: Option<MouseButton>,
    /// Whether shift key is pressed
    pub shift_key: bool,
    /// Whether ctrl/cmd key is pressed
    pub ctrl_key: bool,
    /// Whether alt key is pressed
    pub alt_key: bool,
}

impl PointerEvent {
    /// Set the cursor style for this interaction
    /// (Note: Actual cursor change happens via JavaScript)
    pub fn set_cursor(&self, _style: CursorStyle) {
        // This is a placeholder - actual implementation will
        // communicate with JavaScript to change cursor
    }
}

/// Pointer drag event data
#[derive(Debug, Clone)]
pub struct PointerDragEvent {
    /// Current hit information (if any)
    pub hit: Option<HitInfo>,
    /// Initial hit when drag started
    pub start_hit: Option<HitInfo>,
    /// Current screen position
    pub screen_position: Vector2,
    /// Screen position where drag started
    pub start_screen_position: Vector2,
    /// Current world position (projected to hit plane)
    pub world_position: Vector3,
    /// World position where drag started
    pub start_world_position: Vector3,
    /// Delta movement since last frame
    pub delta: Vector2,
    /// Total delta since drag started
    pub total_delta: Vector2,
    /// Mouse button being held
    pub button: MouseButton,
}

/// Gesture types for touch devices
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GestureEvent {
    /// Pinch gesture with scale factor and center point
    Pinch { scale: f32, center: Vector2 },
    /// Two-finger rotation
    Rotate { angle: f32, center: Vector2 },
    /// Two-finger pan
    Pan { delta: Vector2 },
}

/// Raycaster for manual raycasting operations
#[derive(Debug, Clone)]
pub struct Raycaster {
    pub origin: Vector3,
    pub direction: Vector3,
    pub near: f32,
    pub far: f32,
}

impl Raycaster {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            near: 0.0,
            far: 1000.0,
        }
    }
    
    /// Cast a ray from camera through screen position
    pub fn from_camera(camera: &Camera, _screen_pos: Vector2) -> Self {
        // This would be implemented in platform-specific code
        // For now, placeholder
        Self::new(camera.position, Vector3::FORWARD)
    }
    
    /// Get a point at a specific distance along the ray
    pub fn at(&self, distance: f32) -> Vector3 {
        self.origin + self.direction * distance
    }
}

/// Camera information for raycasting
#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(position: Vector3, target: Vector3) -> Self {
        Self {
            position,
            target,
            up: Vector3::UP,
            fov: 75.0_f32.to_radians(),
            aspect: 1.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

/// Event handler types for pointer events
pub type PointerEventHandler = Box<dyn Fn(PointerEvent)>;
pub type PointerDragEventHandler = Box<dyn Fn(PointerDragEvent)>;
pub type GestureEventHandler = Box<dyn Fn(GestureEvent)>;

/// Input state tracking
#[derive(Debug, Clone, Default)]
pub struct InputState {
    /// Current pointer position
    pub pointer_position: Vector2,
    /// Whether pointer is currently down
    pub pointer_down: bool,
    /// Current cursor style
    pub cursor_style: CursorStyle,
    /// Currently pressed keys
    pub keys_pressed: Vec<String>,
    /// Mouse delta since last frame
    pub mouse_delta: Vector2,
}

impl InputState {
    pub fn is_key_pressed(&self, key: &str) -> bool {
        self.keys_pressed.iter().any(|k| k.eq_ignore_ascii_case(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_operations() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        
        let sum = a + b;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
        
        let diff = b - a;
        assert_eq!(diff.x, 3.0);
        assert_eq!(diff.y, 3.0);
        assert_eq!(diff.z, 3.0);
        
        let scaled = a * 2.0;
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);
        assert_eq!(scaled.z, 6.0);
    }

    #[test]
    fn test_vector3_distance() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(3.0, 4.0, 0.0);
        
        assert_eq!(a.distance(&b), 5.0);
    }

    #[test]
    fn test_entity_id_display() {
        let id = EntityId(42);
        assert_eq!(format!("{}", id), "Entity(42)");
    }
}
