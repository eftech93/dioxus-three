//! Gizmos and transform manipulators for Dioxus Three
//!
//! Provides visual handles for translating, rotating, and scaling
//! selected objects in the 3D scene.

use crate::input::{EntityId, Vector3};

/// Type of transformation gizmo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GizmoMode {
    /// Translation handles
    #[default]
    Translate,
    /// Rotation handles
    Rotate,
    /// Scale handles
    Scale,
}

/// Coordinate space for gizmo operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GizmoSpace {
    /// World coordinates
    #[default]
    World,
    /// Local object coordinates
    Local,
}

/// Configuration for a transformation gizmo
#[derive(Debug, Clone, PartialEq)]
pub struct Gizmo {
    /// The entity this gizmo is attached to
    pub target: EntityId,
    /// Current transformation mode
    pub mode: GizmoMode,
    /// Coordinate space
    pub space: GizmoSpace,
    /// Visual size of the gizmo
    pub size: f32,
    /// Show X axis handle
    pub show_x: bool,
    /// Show Y axis handle
    pub show_y: bool,
    /// Show Z axis handle
    pub show_z: bool,
    /// Show uniform scale handle (scale mode only)
    pub show_xyz: bool,
    /// Show plane handles (translate mode only)
    pub show_planes: bool,
}

impl Gizmo {
    pub fn new(target: EntityId) -> Self {
        Self {
            target,
            mode: GizmoMode::Translate,
            space: GizmoSpace::World,
            size: 1.0,
            show_x: true,
            show_y: true,
            show_z: true,
            show_xyz: true,
            show_planes: true,
        }
    }

    pub fn with_mode(mut self, mode: GizmoMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_space(mut self, space: GizmoSpace) -> Self {
        self.space = space;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn hide_x(mut self) -> Self {
        self.show_x = false;
        self
    }

    pub fn hide_y(mut self) -> Self {
        self.show_y = false;
        self
    }

    pub fn hide_z(mut self) -> Self {
        self.show_z = false;
        self
    }
}

/// Event fired during gizmo interaction
#[derive(Debug, Clone)]
pub struct GizmoEvent {
    /// The entity being transformed
    pub target: EntityId,
    /// The gizmo mode
    pub mode: GizmoMode,
    /// The gizmo space
    pub space: GizmoSpace,
    /// Current transform values
    pub transform: GizmoTransform,
    /// Whether this is the final event (drag ended)
    pub is_finished: bool,
}

/// Transform values from gizmo interaction
#[derive(Debug, Clone, Copy, Default)]
pub struct GizmoTransform {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
}

impl GizmoTransform {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

/// Type of gizmo handle being interacted with
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoHandle {
    /// X axis
    X,
    /// Y axis
    Y,
    /// Z axis
    Z,
    /// XY plane
    XY,
    /// YZ plane
    YZ,
    /// XZ plane
    XZ,
    /// All axes (uniform)
    XYZ,
}

/// Builder for creating gizmos
pub struct GizmoBuilder {
    gizmo: Gizmo,
}

impl GizmoBuilder {
    pub fn new(target: EntityId) -> Self {
        Self {
            gizmo: Gizmo::new(target),
        }
    }

    pub fn translate(mut self) -> Self {
        self.gizmo.mode = GizmoMode::Translate;
        self
    }

    pub fn rotate(mut self) -> Self {
        self.gizmo.mode = GizmoMode::Rotate;
        self
    }

    pub fn scale(mut self) -> Self {
        self.gizmo.mode = GizmoMode::Scale;
        self
    }

    pub fn world_space(mut self) -> Self {
        self.gizmo.space = GizmoSpace::World;
        self
    }

    pub fn local_space(mut self) -> Self {
        self.gizmo.space = GizmoSpace::Local;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.gizmo.size = size;
        self
    }

    pub fn build(self) -> Gizmo {
        self.gizmo
    }
}
