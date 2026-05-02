//! Dioxus Three - A Three.js component for Dioxus
//!
//! Provides a simple component for embedding interactive 3D scenes
//! using Three.js within Dioxus applications.
//!
//! ## Platform Support
//!
//! - **Desktop** (Windows, macOS, Linux): Uses WebView with iframe
//! - **Web** (WASM): Renders directly to canvas element
//! - **Mobile** (iOS, Android): Uses WebView (similar to desktop)
//!
//! Supports multiple 3D formats: OBJ, FBX, GLTF, GLB, STL, PLY, and more.
//! Also supports custom GLSL shaders for advanced visual effects.

use dioxus::prelude::*;
use std::collections::HashMap;

// Input handling and raycasting
pub mod input;
pub mod selection;
pub mod gizmos;

// Re-export input types
pub use input::{
    EntityId, Vector2, Vector3, HitInfo, 
    PointerEvent, PointerDragEvent, GestureEvent,
    RaycastConfig, Raycaster, Camera, MouseButton, CursorStyle,
};
pub use selection::{Selection, SelectionMode, SelectionStyle};
pub use gizmos::{Gizmo, GizmoMode, GizmoSpace, GizmoEvent, GizmoTransform};

// Platform-specific modules
#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(target_arch = "wasm32")]
mod web;

// Re-export platform-specific ThreeView
#[cfg(not(target_arch = "wasm32"))]
pub use desktop::ThreeView;
#[cfg(target_arch = "wasm32")]
pub use web::ThreeView;

/// Custom shader configuration
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ShaderConfig {
    /// Vertex shader GLSL code (optional - uses default if not provided)
    pub vertex_shader: Option<String>,
    /// Fragment shader GLSL code (optional - uses default if not provided)
    pub fragment_shader: Option<String>,
    /// Uniform values to pass to shaders (float values)
    pub uniforms: HashMap<String, f32>,
    /// Time-based animation (automatically sets `u_time` uniform)
    pub animated: bool,
}

/// Built-in shader presets
#[derive(Clone, PartialEq, Debug)]
pub enum ShaderPreset {
    /// No custom shader (default StandardMaterial)
    None,
    /// Animated gradient
    Gradient,
    /// Water/wave effect
    Water,
    /// Hologram effect
    Hologram,
    /// Toon/cel shading
    Toon,
    /// Heat map visualization
    Heatmap,
    /// Custom shader with provided config
    Custom(ShaderConfig),
}

/// 3D model format types
#[derive(Clone, PartialEq, Debug)]
pub enum ModelFormat {
    /// Wavefront OBJ format
    Obj,
    /// Autodesk FBX format
    Fbx,
    /// glTF 2.0 format (JSON)
    Gltf,
    /// glTF 2.0 binary format
    Glb,
    /// STL format (StereoLithography)
    Stl,
    /// Stanford PLY format
    Ply,
    /// Collada DAE format
    Dae,
    /// Three.js JSON format
    Json,
    /// Default cube (no file)
    Cube,
}

impl ModelFormat {
    /// Get the format identifier string
    pub fn as_str(&self) -> &'static str {
        match self {
            ModelFormat::Obj => "obj",
            ModelFormat::Fbx => "fbx",
            ModelFormat::Gltf => "gltf",
            ModelFormat::Glb => "glb",
            ModelFormat::Stl => "stl",
            ModelFormat::Ply => "ply",
            ModelFormat::Dae => "dae",
            ModelFormat::Json => "json",
            ModelFormat::Cube => "cube",
        }
    }

    fn loader_js(&self) -> &'static str {
        match self {
            ModelFormat::Obj => "OBJLoader",
            ModelFormat::Fbx => "FBXLoader",
            ModelFormat::Gltf | ModelFormat::Glb => "GLTFLoader",
            ModelFormat::Stl => "STLLoader",
            ModelFormat::Ply => "PLYLoader",
            ModelFormat::Dae => "ColladaLoader",
            ModelFormat::Json => "ObjectLoader",
            ModelFormat::Cube => "",
        }
    }

    fn loader_url(&self) -> &'static str {
        match self {
            ModelFormat::Obj => {
                "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/OBJLoader.js"
            }
            ModelFormat::Fbx => {
                "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/FBXLoader.js"
            }
            ModelFormat::Gltf | ModelFormat::Glb => {
                "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/GLTFLoader.js"
            }
            ModelFormat::Stl => {
                "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/STLLoader.js"
            }
            ModelFormat::Ply => {
                "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/PLYLoader.js"
            }
            ModelFormat::Dae => {
                "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/ColladaLoader.js"
            }
            ModelFormat::Json => "",
            ModelFormat::Cube => "",
        }
    }

    /// Get additional dependency URLs required by this loader
    fn extra_scripts(&self) -> Vec<&'static str> {
        match self {
            // FBXLoader requires fflate for decompression
            ModelFormat::Fbx => vec!["https://cdn.jsdelivr.net/npm/fflate@0.8.0/umd/index.js"],
            _ => vec![],
        }
    }
}

/// Configuration for a single model in a multi-model scene
#[derive(Clone, PartialEq, Debug)]
pub struct ModelConfig {
    /// URL or path to the model file
    pub url: String,
    /// Format of the model
    pub format: ModelFormat,
    /// Position X
    pub pos_x: f32,
    /// Position Y
    pub pos_y: f32,
    /// Position Z
    pub pos_z: f32,
    /// Rotation X (degrees)
    pub rot_x: f32,
    /// Rotation Y (degrees)
    pub rot_y: f32,
    /// Rotation Z (degrees)
    pub rot_z: f32,
    /// Scale
    pub scale: f32,
    /// Color (hex string)
    pub color: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            format: ModelFormat::Cube,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
            scale: 1.0,
            color: "#ff6b6b".to_string(),
        }
    }
}

impl ModelConfig {
    /// Create a new model config with just URL and format
    pub fn new(url: impl Into<String>, format: ModelFormat) -> Self {
        Self {
            url: url.into(),
            format,
            ..Default::default()
        }
    }

    /// Set position
    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.pos_x = x;
        self.pos_y = y;
        self.pos_z = z;
        self
    }

    /// Set rotation
    pub fn with_rotation(mut self, x: f32, y: f32, z: f32) -> Self {
        self.rot_x = x;
        self.rot_y = y;
        self.rot_z = z;
        self
    }

    /// Set scale
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Set color
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }
}

/// Properties for the ThreeView component
#[derive(Props, Clone, PartialEq)]
pub struct ThreeViewProps {
    /// Model file path or URL (optional - uses cube if not provided)
    #[props(default = None)]
    pub model_url: Option<String>,
    /// Model format
    #[props(default = ModelFormat::Cube)]
    pub format: ModelFormat,
    /// Model position X
    #[props(default = 0.0)]
    pub pos_x: f32,
    /// Model position Y
    #[props(default = 0.0)]
    pub pos_y: f32,
    /// Model position Z
    #[props(default = 0.0)]
    pub pos_z: f32,
    /// Model rotation X (degrees)
    #[props(default = 0.0)]
    pub rot_x: f32,
    /// Model rotation Y (degrees)
    #[props(default = 0.0)]
    pub rot_y: f32,
    /// Model rotation Z (degrees)
    #[props(default = 0.0)]
    pub rot_z: f32,
    /// Model scale
    #[props(default = 1.0)]
    pub scale: f32,
    /// Model color/material (hex string like "#ff6b6b")
    #[props(default = "#ff6b6b".to_string())]
    pub color: String,
    /// Multiple models to load (optional - if set, model_url/format are ignored)
    #[props(default = Vec::new())]
    pub models: Vec<ModelConfig>,
    /// Auto-center the model
    #[props(default = true)]
    pub auto_center: bool,
    /// Auto-scale to fit viewport
    #[props(default = false)]
    pub auto_scale: bool,
    /// Camera position X
    #[props(default = 5.0)]
    pub cam_x: f32,
    /// Camera position Y
    #[props(default = 5.0)]
    pub cam_y: f32,
    /// Camera position Z
    #[props(default = 5.0)]
    pub cam_z: f32,
    /// Camera target X
    #[props(default = 0.0)]
    pub target_x: f32,
    /// Camera target Y
    #[props(default = 0.0)]
    pub target_y: f32,
    /// Camera target Z
    #[props(default = 0.0)]
    pub target_z: f32,
    /// Auto-rotate the model
    #[props(default = true)]
    pub auto_rotate: bool,
    /// Auto-rotation speed
    #[props(default = 1.0)]
    pub rot_speed: f32,
    /// Show grid helper
    #[props(default = true)]
    pub show_grid: bool,
    /// Show axes helper
    #[props(default = true)]
    pub show_axes: bool,
    /// Background color
    #[props(default = "#1a1a2e".to_string())]
    pub background: String,
    /// Additional CSS class for the container
    #[props(default = String::new())]
    pub class: String,
    /// Enable shadows
    #[props(default = true)]
    pub shadows: bool,
    /// Wireframe mode
    #[props(default = false)]
    pub wireframe: bool,
    /// Shader preset or custom shader
    #[props(default = ShaderPreset::None)]
    pub shader: ShaderPreset,
    
    // === Phase 1: Input & Selection Features ===
    
    /// Unique ID for this view (needed for pointer event routing)
    #[props(default = None)]
    pub id: Option<String>,
    
    /// Raycast configuration
    #[props(default = RaycastConfig::default())]
    pub raycast: RaycastConfig,
    
    /// Callback for pointer down events
    #[props(default = None)]
    pub on_pointer_down: Option<Callback<PointerEvent>>,
    
    /// Callback for pointer up events
    #[props(default = None)]
    pub on_pointer_up: Option<Callback<PointerEvent>>,
    
    /// Callback for pointer move events (hover)
    #[props(default = None)]
    pub on_pointer_move: Option<Callback<PointerEvent>>,
    
    /// Callback for pointer drag events
    #[props(default = None)]
    pub on_pointer_drag: Option<Callback<PointerDragEvent>>,
    
    /// Callback for gesture events (pinch, rotate, pan)
    #[props(default = None)]
    pub on_gesture: Option<Callback<GestureEvent>>,
    
    /// Current selection state
    #[props(default = None)]
    pub selection: Option<Selection>,
    
    /// Selection mode
    #[props(default = SelectionMode::Single)]
    pub selection_mode: SelectionMode,
    
    /// Visual style for selection
    #[props(default = SelectionStyle::default())]
    pub selection_style: SelectionStyle,
    
    /// Callback when selection changes
    #[props(default = None)]
    pub on_selection_change: Option<Callback<Selection>>,
    
    /// Gizmo configuration for transform manipulation
    #[props(default = None)]
    pub gizmo: Option<Gizmo>,
    
    /// Callback during gizmo drag
    #[props(default = None)]
    pub on_gizmo_drag: Option<Callback<GizmoEvent>>,
    
    /// Enable debug overlay
    #[props(default = false)]
    pub debug: bool,
}

impl ShaderPreset {
    /// Get the vertex shader code for this preset
    fn vertex_shader(&self) -> Option<String> {
        match self {
            ShaderPreset::None => None,
            ShaderPreset::Gradient => Some(include_str!("shaders/gradient.vert").to_string()),
            ShaderPreset::Water => Some(include_str!("shaders/water.vert").to_string()),
            ShaderPreset::Hologram => Some(include_str!("shaders/hologram.vert").to_string()),
            ShaderPreset::Toon => Some(include_str!("shaders/toon.vert").to_string()),
            ShaderPreset::Heatmap => Some(include_str!("shaders/heatmap.vert").to_string()),
            ShaderPreset::Custom(config) => config.vertex_shader.clone(),
        }
    }

    /// Get the fragment shader code for this preset
    fn fragment_shader(&self) -> Option<String> {
        match self {
            ShaderPreset::None => None,
            ShaderPreset::Gradient => Some(include_str!("shaders/gradient.frag").to_string()),
            ShaderPreset::Water => Some(include_str!("shaders/water.frag").to_string()),
            ShaderPreset::Hologram => Some(include_str!("shaders/hologram.frag").to_string()),
            ShaderPreset::Toon => Some(include_str!("shaders/toon.frag").to_string()),
            ShaderPreset::Heatmap => Some(include_str!("shaders/heatmap.frag").to_string()),
            ShaderPreset::Custom(config) => config.fragment_shader.clone(),
        }
    }

    /// Check if this shader uses time animation
    fn is_animated(&self) -> bool {
        match self {
            ShaderPreset::None => false,
            ShaderPreset::Gradient | ShaderPreset::Water | ShaderPreset::Hologram => true,
            ShaderPreset::Custom(config) => config.animated,
            _ => false,
        }
    }
}

/// Build loader scripts for multiple models
pub fn build_loader_scripts_for_models(models: &[ModelConfig]) -> String {
    let mut scripts: Vec<String> = vec![];
    let mut seen_formats: Vec<ModelFormat> = vec![];

    for model in models {
        if seen_formats.contains(&model.format) {
            continue;
        }
        seen_formats.push(model.format.clone());

        let loader_url = model.format.loader_url();
        if loader_url.is_empty() {
            continue;
        }

        for extra in model.format.extra_scripts() {
            let script = format!(r#"<script src="{}"></script>"#, extra);
            if !scripts.contains(&script) {
                scripts.push(script);
            }
        }

        scripts.push(format!(r#"<script src="{}"></script>"#, loader_url));
    }

    scripts.join("\n    ")
}

/// Build loader scripts for single model
pub fn build_loader_scripts_single(format: &ModelFormat, model_url: &Option<String>) -> String {
    let url = model_url.clone().unwrap_or_default();
    let has_model = !url.is_empty() && *format != ModelFormat::Cube;
    let loader_url = format.loader_url();

    if !has_model || loader_url.is_empty() {
        return String::new();
    }

    let mut scripts: Vec<String> = format
        .extra_scripts()
        .iter()
        .map(|url| format!(r#"<script src="{}"></script>"#, url))
        .collect();

    scripts.push(format!(r#"<script src="{}"></script>"#, loader_url));
    scripts.join("\n    ")
}

/// Build JavaScript code for loading multiple models (interactive version with entity IDs)
pub fn build_multi_model_loading_interactive(models: &[ModelConfig], shadows: bool) -> String {
    let shadows_str = shadows.to_string().to_lowercase();

    let load_calls: Vec<String> = models.iter().enumerate().map(|(idx, model)| {
        let loader_class = model.format.loader_js();
        let is_geometry_loader = matches!(model.format, ModelFormat::Stl | ModelFormat::Ply);
        let url = &model.url;
        let pos_x = model.pos_x;
        let pos_y = model.pos_y;
        let pos_z = model.pos_z;
        let rot_x = model.rot_x.to_radians();
        let rot_y = model.rot_y.to_radians();
        let rot_z = model.rot_z.to_radians();
        let scale = model.scale;
        let color = &model.color;
        let default_color = "#ff6b6b";

        if model.format == ModelFormat::Cube {
            format!(
                r#"(function() {{ 
                    const entityId = {idx};
                    const geometry = new THREE.BoxGeometry(1, 1, 1); 
                    const material = new THREE.MeshStandardMaterial({{ color: "{color}", roughness: 0.5, metalness: 0.3 }}); 
                    const mesh = new THREE.Mesh(geometry, material); 
                    mesh.position.set({pos_x}, {pos_y}, {pos_z}); 
                    mesh.rotation.set({rot_x}, {rot_y}, {rot_z}); 
                    mesh.scale.setScalar({scale}); 
                    mesh.castShadow = {shadows_str}; 
                    mesh.receiveShadow = {shadows_str};
                    mesh.userData = {{ entityId: entityId }};
                    modelContainer.add(mesh); 
                    entityMap.set(entityId, mesh);
                    nextEntityId = Math.max(nextEntityId, entityId + 1);
                }})();"#
            )
        } else if is_geometry_loader {
            format!(
                r#"(function() {{ 
                    const entityId = {idx};
                    const loader = new THREE.{loader_class}(); 
                    loader.load("{url}", function(geometry) {{ 
                        const material = new THREE.MeshStandardMaterial({{ color: "{color}", roughness: 0.5, metalness: 0.1, side: THREE.DoubleSide }}); 
                        const mesh = new THREE.Mesh(geometry, material); 
                        mesh.position.set({pos_x}, {pos_y}, {pos_z}); 
                        mesh.rotation.set({rot_x}, {rot_y}, {rot_z}); 
                        mesh.scale.setScalar({scale}); 
                        mesh.castShadow = {shadows_str}; 
                        mesh.receiveShadow = {shadows_str};
                        mesh.userData = {{ entityId: entityId }};
                        modelContainer.add(mesh); 
                        entityMap.set(entityId, mesh);
                        nextEntityId = Math.max(nextEntityId, entityId + 1);
                    }}, undefined, function(err) {{ console.error('Failed to load model {idx}:', err); }}); 
                }})();"#
            )
        } else {
            let color_js = if color != default_color {
                format!(
                    r#"if (child.material) {{ if (Array.isArray(child.material)) {{ child.material.forEach(m => m.color.set("{color}")); }} else {{ child.material.color.set("{color}"); }} }}"#,
                    color = color
                )
            } else {
                String::new()
            };
            format!(
                r#"(function() {{ 
                    const entityId = {idx};
                    const loader = new THREE.{loader_class}(); 
                    loader.load("{url}", function(object) {{ 
                        let model = object.scene || object.dae || object; 
                        model.position.set({pos_x}, {pos_y}, {pos_z}); 
                        model.rotation.set({rot_x}, {rot_y}, {rot_z}); 
                        model.scale.setScalar({scale}); 
                        model.traverse(function(child) {{ 
                            if (child.isMesh) {{ 
                                child.castShadow = {shadows_str}; 
                                child.receiveShadow = {shadows_str}; 
                                child.userData = {{ entityId: entityId }};
                                {color_js} 
                            }} 
                        }}); 
                        model.userData = {{ entityId: entityId }};
                        modelContainer.add(model); 
                        entityMap.set(entityId, model);
                        nextEntityId = Math.max(nextEntityId, entityId + 1);
                    }}, undefined, function(err) {{ console.error('Failed to load model {idx}:', err); }}); 
                }})();"#,
                loader_class = loader_class,
                url = url,
                pos_x = pos_x,
                pos_y = pos_y,
                pos_z = pos_z,
                rot_x = rot_x,
                rot_y = rot_y,
                rot_z = rot_z,
                scale = scale,
                shadows_str = shadows_str,
                color_js = color_js,
                idx = idx
            )
        }
    }).collect();

    format!("loadingEl.style.display = 'none'; {}", load_calls.join(" "))
}

/// Build JavaScript code for loading a single model (interactive version with entity ID)
pub fn build_single_model_loading_interactive(
    format: &ModelFormat,
    model_url: &Option<String>,
    auto_center: bool,
    auto_scale: bool,
    shadows: bool,
) -> String {
    let url = model_url.clone().unwrap_or_default();
    let has_model = !url.is_empty() && *format != ModelFormat::Cube;
    let loader_class = format.loader_js();
    let is_geometry_loader = matches!(format, ModelFormat::Stl | ModelFormat::Ply);
    let auto_center_str = auto_center.to_string().to_lowercase();
    let auto_scale_str = auto_scale.to_string().to_lowercase();
    let shadows_str = shadows.to_string().to_lowercase();

    if !has_model {
        return r#"const entityId = 0; const geometry = new THREE.BoxGeometry(1, 1, 1); let material = new THREE.MeshStandardMaterial({ color: state.color, roughness: 0.5, metalness: 0.3, wireframe: state.wireframe }); model = new THREE.Mesh(geometry, material); model.castShadow = true; model.receiveShadow = true; model.userData = { entityId: entityId }; modelContainer.add(model); entityMap.set(entityId, model); nextEntityId = 1; loadingEl.style.display = 'none';"#.to_string();
    }

    if is_geometry_loader {
        format!(
            r#"const entityId = 0; const loader = new THREE.{loader_class}(); loader.load("{url}", function(geometry) {{ loadingEl.style.display = 'none'; const material = new THREE.MeshStandardMaterial({{ color: state.color, roughness: 0.5, metalness: 0.1, wireframe: state.wireframe, side: THREE.DoubleSide }}); model = new THREE.Mesh(geometry, material); model.castShadow = {shadows_str}; model.receiveShadow = {shadows_str}; model.userData = {{ entityId: entityId }}; if ({auto_center_str}) {{ const box = new THREE.Box3().setFromObject(model); const center = box.getCenter(new THREE.Vector3()); model.position.sub(center); }} if ({auto_scale_str}) {{ const box = new THREE.Box3().setFromObject(model); const size = box.getSize(new THREE.Vector3()); const maxDim = Math.max(size.x, size.y, size.z); if (maxDim > 0) {{ const s = 2 / maxDim; model.scale.setScalar(s); }} }} modelContainer.add(model); entityMap.set(entityId, model); nextEntityId = 1; updateTransform(); }}, function(xhr) {{ const percent = xhr.loaded / xhr.total * 100; loadingEl.textContent = 'Loading: ' + Math.round(percent) + '%'; }}, function(error) {{ console.error('Error loading model:', error); loadingEl.style.display = 'none'; errorEl.style.display = 'block'; errorEl.textContent = 'Failed to load model: ' + (error.message || 'Unknown error'); const geometry = new THREE.BoxGeometry(1, 1, 1); const material = new THREE.MeshStandardMaterial({{ color: 0xff6b6b }}); model = new THREE.Mesh(geometry, material); model.userData = {{ entityId: entityId }}; modelContainer.add(model); entityMap.set(entityId, model); nextEntityId = 1; }});"#
        )
    } else {
        format!(
            r#"const entityId = 0; const loader = new THREE.{loader_class}(); loader.load("{url}", function(object) {{ loadingEl.style.display = 'none'; if (object.scene) {{ model = object.scene; }} else if (object.dae) {{ model = object.scene; }} else {{ model = object; }} model.traverse(function(child) {{ if (child.isMesh) {{ child.castShadow = {shadows_str}; child.receiveShadow = {shadows_str}; child.userData = {{ entityId: entityId }}; if (!child.material) {{ child.material = new THREE.MeshStandardMaterial({{ color: state.color, roughness: 0.5, metalness: 0.3 }}); }} const materials = Array.isArray(child.material) ? child.material : [child.material]; materials.forEach(m => {{ if (m.opacity !== undefined && m.opacity < 0.1) m.opacity = 1.0; if (m.transparent === true && m.opacity < 0.1) m.transparent = false; if (state.color !== '#ff6b6b' && m.color) {{ m.color.set(state.color); }} m.wireframe = state.wireframe; }}); }} }}); model.userData = {{ entityId: entityId }}; if ({auto_center_str}) {{ const box = new THREE.Box3().setFromObject(model); const center = box.getCenter(new THREE.Vector3()); model.position.sub(center); }} if ({auto_scale_str}) {{ const box = new THREE.Box3().setFromObject(model); const size = box.getSize(new THREE.Vector3()); const maxDim = Math.max(size.x, size.y, size.z); if (maxDim > 0) {{ const s = 2 / maxDim; model.scale.setScalar(s); }} }} modelContainer.add(model); entityMap.set(entityId, model); nextEntityId = 1; updateTransform(); }}, function(xhr) {{ const percent = xhr.loaded / xhr.total * 100; loadingEl.textContent = 'Loading: ' + Math.round(percent) + '%'; }}, function(error) {{ console.error('Error loading model:', error); loadingEl.style.display = 'none'; errorEl.style.display = 'block'; errorEl.textContent = 'Failed to load model: ' + (error.message || 'Unknown error'); const geometry = new THREE.BoxGeometry(1, 1, 1); const material = new THREE.MeshStandardMaterial({{ color: 0xff6b6b }}); model = new THREE.Mesh(geometry, material); model.userData = {{ entityId: entityId }}; modelContainer.add(model); entityMap.set(entityId, model); nextEntityId = 1; }});"#
        )
    }
}

/// Generate the HTML with embedded Three.js
/// Includes full raycasting, selection, and gizmo interaction support
pub fn generate_three_js_html(props: &ThreeViewProps) -> String {
    let rot_x_rad = props.rot_x.to_radians();
    let rot_y_rad = props.rot_y.to_radians();
    let rot_z_rad = props.rot_z.to_radians();

    // Legacy single-model variables (for backward compatibility with template)
    let _loader_url = props.format.loader_url();
    let _loader_class = props.format.loader_js();
    let format_str = props.format.as_str();
    let model_url = props.model_url.clone().unwrap_or_default();
    let _has_model = !model_url.is_empty() && props.format != ModelFormat::Cube;

    // Check if using multiple models
    let use_multiple_models = !props.models.is_empty();

    // Build loader script tags
    let loader_script = if use_multiple_models {
        build_loader_scripts_for_models(&props.models)
    } else {
        build_loader_scripts_single(&props.format, &props.model_url)
    };

    // Build model loading JavaScript code
    let model_loading_code = if use_multiple_models {
        build_multi_model_loading_interactive(&props.models, props.shadows)
    } else {
        build_single_model_loading_interactive(
            &props.format,
            &props.model_url,
            props.auto_center,
            props.auto_scale,
            props.shadows,
        )
    };

    // Build shader code if needed
    let (_shader_material_code, shader_uniforms, _shader_animated) =
        build_shader_code(&props.shader);

    // Serialize selection and gizmo state for JavaScript
    let selection_ids_json = props.selection.as_ref()
        .map(|s| {
            let ids: Vec<String> = s.iter().map(|e| e.0.to_string()).collect();
            format!("[{}]", ids.join(", "))
        })
        .unwrap_or_else(|| "[]".to_string());
    
    let gizmo_config_json = props.gizmo.as_ref()
        .map(|g| {
            format!(
                r#"{{"target": {}, "mode": "{:?}", "space": "{:?}"}}"#,
                g.target.0,
                g.mode,
                g.space
            )
        })
        .unwrap_or_else(|| "null".to_string());
    
    let selection_style_json = format!(
        r#"{{"outline": {}, "outline_color": "{}", "outline_width": {}, "highlight": {}, "highlight_color": "{}", "highlight_opacity": {}, "show_gizmo": {}}}"#,
        props.selection_style.outline.to_string().to_lowercase(),
        props.selection_style.outline_color,
        props.selection_style.outline_width,
        props.selection_style.highlight.to_string().to_lowercase(),
        props.selection_style.highlight_color,
        props.selection_style.highlight_opacity,
        props.selection_style.show_gizmo.to_string().to_lowercase(),
    );
    
    let raycast_enabled = props.raycast.enabled;
    let selection_enabled = props.selection.is_some();

    // Build the HTML with full interaction support
    let html = format!(
        r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{ width: 100%; height: 100%; overflow: hidden; background: {bg}; }}
        #canvas-container {{ width: 100%; height: 100%; }}
        canvas {{ display: block; }}
        #loading {{ 
            position: absolute; 
            top: 50%; 
            left: 50%; 
            transform: translate(-50%, -50%); 
            color: white; 
            font-family: sans-serif;
            font-size: 14px;
        }}
        #error {{
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            color: #ff6b6b;
            font-family: sans-serif;
            font-size: 14px;
            text-align: center;
            display: none;
        }}
        canvas {{
            cursor: default;
        }}
        canvas.hovering {{
            cursor: pointer;
        }}
        canvas.dragging {{
            cursor: grabbing;
        }}
    </style>
</head>
<body>
    <div id="canvas-container"></div>
    <div id="loading">Loading 3D model...</div>
    <div id="error"></div>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/TransformControls.js"></script>
    {loader_script}
    <script>
        console.log("Dioxus Three: Initializing ({fmt})...");
        
        const container = document.getElementById('canvas-container');
        const loadingEl = document.getElementById('loading');
        const errorEl = document.getElementById('error');
        const width = container.clientWidth || window.innerWidth;
        const height = container.clientHeight || window.innerHeight;
        
        const scene = new THREE.Scene();
        scene.background = new THREE.Color('{bg}');
        
        const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
        camera.position.set({cam_x}, {cam_y}, {cam_z});
        camera.lookAt({target_x}, {target_y}, {target_z});
        
        const renderer = new THREE.WebGLRenderer({{ antialias: true }});
        renderer.setSize(width, height);
        renderer.setPixelRatio(window.devicePixelRatio);
        renderer.shadowMap.enabled = {shadows};
        renderer.shadowMap.type = THREE.PCFSoftShadowMap;
        container.appendChild(renderer.domElement);
        
        // Brighter ambient light for better material visibility
        const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
        scene.add(ambientLight);
        
        // Main directional light (sun)
        const dirLight = new THREE.DirectionalLight(0xffffff, 1.2);
        dirLight.position.set(10, 20, 10);
        dirLight.castShadow = {shadows};
        dirLight.shadow.mapSize.width = 2048;
        dirLight.shadow.mapSize.height = 2048;
        scene.add(dirLight);
        
        // Fill light from opposite side
        const fillLight = new THREE.DirectionalLight(0xffffff, 0.4);
        fillLight.position.set(-10, 10, -10);
        scene.add(fillLight);
        
        // Back light for rim lighting
        const backLight = new THREE.DirectionalLight(0xffffff, 0.3);
        backLight.position.set(0, 5, -10);
        scene.add(backLight);
        
        if ({show_grid}) {{
            const gridHelper = new THREE.GridHelper(20, 20, 0x444444, 0x222222);
            scene.add(gridHelper);
        }}
        if ({show_axes}) {{
            const axesHelper = new THREE.AxesHelper(2);
            scene.add(axesHelper);
        }}
        
        let model = null;
        let modelContainer = new THREE.Group();
        scene.add(modelContainer);
        
        // ============ INTERACTIVE SYSTEMS ============
        
        // Entity management
        const entityMap = new Map(); // entityId -> object
        let nextEntityId = 0;
        
        // Selection system
        let selectedEntities = new Set({selection_ids_json});
        let selectionOutlines = new Map(); // entityId -> outline mesh
        let outlineToMeshMap = new Map(); // outline group -> source mesh
        
        // Gizmo system
        let transformControl = null;
        let gizmoTarget = null;
        let currentGizmoMode = 'translate';
        let currentGizmoSpace = 'world';
        
        // Raycasting system
        const raycaster = new THREE.Raycaster();
        const mouse = new THREE.Vector2();
        let isDragging = false;
        let isGizmoDragging = false;
        
        const raycastEnabled = {raycast_enabled};
        const selectionEnabled = {selection_enabled};
        
        // State
        let selectionStyle = {selection_style_json};
        
        let state = {{
            rotX: {rot_x},
            rotY: {rot_y},
            rotZ: {rot_z},
            scale: {scale},
            color: "{color}",
            autoRotate: {auto_rotate},
            rotSpeed: {rot_speed},
            wireframe: {wireframe}
        }};
        let autoRotY = 0;
        
        // Initialize TransformControls
        function initTransformControls() {{
            if (typeof THREE.TransformControls === 'undefined') {{
                console.warn('TransformControls not available');
                return null;
            }}
            const control = new THREE.TransformControls(camera, renderer.domElement);
            control.addEventListener('dragging-changed', function(event) {{
                isGizmoDragging = event.value;
                console.log('[GIZMO] dragging-changed:', event.value, 'attached to:', gizmoTarget);
                if (event.value) {{
                    renderer.domElement.classList.add('dragging');
                }} else {{
                    renderer.domElement.classList.remove('dragging');
                }}
            }});
            control.addEventListener('change', function() {{
                if (gizmoTarget && control.object) {{
                    console.log('[GIZMO] change event - target:', gizmoTarget, 'scale:', control.object.scale.x.toFixed(3), control.object.scale.y.toFixed(3), control.object.scale.z.toFixed(3));
                    // Notify parent about transform change
                    notifyGizmoDrag(gizmoTarget, control.object, false);
                }}
            }});
            control.addEventListener('mouseUp', function() {{
                if (gizmoTarget && control.object) {{
                    notifyGizmoDrag(gizmoTarget, control.object, true);
                }}
            }});
            scene.add(control);
            return control;
        }}
        
        // Create selection outline for an object
        function createSelectionOutline(object, entityId) {{
            // Remove existing outline
            removeSelectionOutline(entityId);
            
            const outlineColorHex = selectionStyle.outline_color || '#FFD700';
            const outlineColor = new THREE.Color(outlineColorHex).getHex();
            
            const box = new THREE.Box3().setFromObject(object);
            const size = box.getSize(new THREE.Vector3());
            const center = box.getCenter(new THREE.Vector3());
            const maxDim = Math.max(size.x, size.y, size.z);
            
            // Create a group for the outline
            const outlineGroup = new THREE.Group();
            
            // Main wireframe box - thicker and more visible
            const outlineGeometry = new THREE.BoxGeometry(size.x * 1.08, size.y * 1.08, size.z * 1.08);
            const outlineMaterial = new THREE.MeshBasicMaterial({{
                color: outlineColor,
                wireframe: true,
                transparent: true,
                opacity: 1.0
            }});
            const outline = new THREE.Mesh(outlineGeometry, outlineMaterial);
            outlineGroup.add(outline);
            
            // Inner glow effect - semi-transparent fill
            const glowGeometry = new THREE.BoxGeometry(size.x * 1.04, size.y * 1.04, size.z * 1.04);
            const glowMaterial = new THREE.MeshBasicMaterial({{
                color: outlineColor,
                transparent: true,
                opacity: 0.15,
                side: THREE.BackSide
            }});
            const glow = new THREE.Mesh(glowGeometry, glowMaterial);
            outlineGroup.add(glow);
            
            // Corner markers for better visibility
            const cornerSize = maxDim * 0.08;
            const cornerGeometry = new THREE.BoxGeometry(cornerSize, cornerSize, cornerSize);
            const cornerMaterial = new THREE.MeshBasicMaterial({{ color: outlineColor }});
            
            const corners = [
                {{ x: 1, y: 1, z: 1 }}, {{ x: -1, y: 1, z: 1 }},
                {{ x: 1, y: -1, z: 1 }}, {{ x: -1, y: -1, z: 1 }},
                {{ x: 1, y: 1, z: -1 }}, {{ x: -1, y: 1, z: -1 }},
                {{ x: 1, y: -1, z: -1 }}, {{ x: -1, y: -1, z: -1 }}
            ];
            
            corners.forEach(corner => {{
                const cornerMesh = new THREE.Mesh(cornerGeometry, cornerMaterial);
                cornerMesh.position.set(
                    corner.x * size.x * 0.54,
                    corner.y * size.y * 0.54,
                    corner.z * size.z * 0.54
                );
                outlineGroup.add(cornerMesh);
            }});
            
            // Position the outline group
            outlineGroup.position.copy(center);
            
            // If object is a mesh, match its world transform
            if (object.parent) {{
                object.parent.add(outlineGroup);
            }} else {{
                scene.add(outlineGroup);
            }}
            
            selectionOutlines.set(entityId, outlineGroup);
            outlineToMeshMap.set(outlineGroup, object);
        }}
        
        // Remove selection outline
        function removeSelectionOutline(entityId) {{
            const outline = selectionOutlines.get(entityId);
            if (outline) {{
                if (outline.parent) outline.parent.remove(outline);
                outline.traverse(function(child) {{
                    if (child.geometry) child.geometry.dispose();
                    if (child.material) {{
                        if (Array.isArray(child.material)) {{
                            child.material.forEach(m => m.dispose());
                        }} else {{
                            child.material.dispose();
                        }}
                    }}
                }});
                selectionOutlines.delete(entityId);
            }}
        }}
        
        // Update all selection visualizations
        function updateSelectionVisuals() {{
            // Clear all outlines
            for (const [entityId, outline] of selectionOutlines) {{
                if (outline.parent) outline.parent.remove(outline);
                outline.traverse(function(child) {{
                    if (child.geometry) child.geometry.dispose();
                    if (child.material) {{
                        if (Array.isArray(child.material)) {{
                            child.material.forEach(m => m.dispose());
                        }} else {{
                            child.material.dispose();
                        }}
                    }}
                }});
            }}
            selectionOutlines.clear();
            outlineToMeshMap.clear();
            
            // Create outlines for selected entities
            for (const entityId of selectedEntities) {{
                const obj = entityMap.get(entityId);
                if (obj) {{
                    createSelectionOutline(obj, entityId);
                }}
            }}
        }}
        
        // Update gizmo position and mode
        function updateGizmo(gizmoConfig) {{
            if (!gizmoConfig) gizmoConfig = {gizmo_config_json};
            if (!gizmoConfig || !transformControl) {{
                if (transformControl) transformControl.detach();
                gizmoTarget = null;
                return;
            }}
            
            const targetObj = entityMap.get(gizmoConfig.target);
            if (!targetObj) return;
            
            // Set mode
            const mode = gizmoConfig.mode.toLowerCase();
            if (mode !== currentGizmoMode) {{
                currentGizmoMode = mode;
                transformControl.setMode(mode === 'translate' ? 'translate' : mode === 'rotate' ? 'rotate' : 'scale');
            }}
            
            // Set space
            const space = gizmoConfig.space.toLowerCase();
            if (space !== currentGizmoSpace) {{
                currentGizmoSpace = space;
                transformControl.setSpace(space === 'local' ? 'local' : 'world');
            }}
            
            // Attach to target
            if (gizmoTarget !== gizmoConfig.target) {{
                gizmoTarget = gizmoConfig.target;
                transformControl.attach(targetObj);
            }}
        }}
        
        // Notify parent window about gizmo drag
        function notifyGizmoDrag(entityId, object, isFinished) {{
            const eventData = {{
                target: entityId,
                mode: currentGizmoMode,
                space: currentGizmoSpace,
                transform: {{
                    position: {{
                        x: object.position.x,
                        y: object.position.y,
                        z: object.position.z
                    }},
                    rotation: {{
                        x: object.rotation.x,
                        y: object.rotation.y,
                        z: object.rotation.z
                    }},
                    scale: {{
                        x: object.scale.x,
                        y: object.scale.y,
                        z: object.scale.z
                    }}
                }},
                isFinished: !!isFinished
            }};
            
            console.log('[GIZMO] notifyGizmoDrag - entity:', entityId, 'finished:', isFinished, 'scale:', object.scale.x.toFixed(3), object.scale.y.toFixed(3), object.scale.z.toFixed(3));
            
            // Send message to parent window
            if (window.parent !== window) {{
                window.parent.postMessage({{
                    type: 'gizmo-drag',
                    data: eventData
                }}, '*');
            }}
        }}
        
        // Notify parent about pointer event
        function notifyPointerEvent(type, event, hit) {{
            const eventData = {{
                hit: hit,
                screenPosition: {{ x: event.clientX, y: event.clientY }},
                ndcPosition: {{ x: mouse.x, y: mouse.y }},
                button: event.button === 0 ? 'Left' : event.button === 2 ? 'Right' : 'Middle',
                shiftKey: event.shiftKey,
                ctrlKey: event.ctrlKey,
                altKey: event.altKey
            }};
            
            if (window.parent !== window) {{
                window.parent.postMessage({{
                    type: type,
                    data: eventData
                }}, '*');
            }}
        }}
        
        // Raycast to find intersected entity
        function raycastEntity(clientX, clientY) {{
            const rect = renderer.domElement.getBoundingClientRect();
            mouse.x = ((clientX - rect.left) / rect.width) * 2 - 1;
            mouse.y = -((clientY - rect.top) / rect.height) * 2 + 1;
            
            raycaster.setFromCamera(mouse, camera);
            
            // Collect all selectable objects
            const selectableObjects = [];
            for (const [entityId, obj] of entityMap) {{
                if (obj) selectableObjects.push(obj);
            }}
            
            const intersects = raycaster.intersectObjects(selectableObjects, true);
            if (intersects.length > 0) {{
                // Find the entity ID for this intersection
                let targetObj = intersects[0].object;
                let entityId = null;
                
                // Traverse up to find entity with userData
                while (targetObj) {{
                    for (const [id, obj] of entityMap) {{
                        if (obj === targetObj || isDescendant(obj, targetObj)) {{
                            entityId = id;
                            break;
                        }}
                    }}
                    if (entityId !== null) break;
                    targetObj = targetObj.parent;
                }}
                
                if (entityId !== null) {{
                    return {{
                        entityId: entityId,
                        point: intersects[0].point,
                        normal: intersects[0].face ? intersects[0].face.normal : new THREE.Vector3(0, 1, 0),
                        distance: intersects[0].distance
                    }};
                }}
            }}
            return null;
        }}
        
        // Check if target is descendant of parent
        function isDescendant(parent, target) {{
            if (!parent || !target) return false;
            let found = false;
            parent.traverse(function(child) {{
                if (child === target) found = true;
            }});
            return found;
        }}
        
        // Check if clicking on TransformControls gizmo
        function isClickOnGizmo(event) {{
            if (!transformControl || !transformControl.object) return false;
            
            const rect = renderer.domElement.getBoundingClientRect();
            mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
            mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
            
            raycaster.setFromCamera(mouse, camera);
            const intersects = raycaster.intersectObjects(transformControl.children, true);
            
            for (let i = 0; i < intersects.length; i++) {{
                // Only count clicks on actual interactive handles (Mesh), not lines
                if (intersects[i].object.visible && intersects[i].object.isMesh) {{
                    console.log('[GIZMO] Clicked on handle:', intersects[i].object.type, intersects[i].object.name || '');
                    return true;
                }}
            }}
            if (intersects.length > 0) {{
                console.log('[GIZMO] Clicked on non-handle:', intersects.map(i => ({{ type: i.object.type, visible: i.object.visible, isMesh: i.object.isMesh }})));
            }}
            return false;
        }}
        
        // Pointer event handlers
        function onPointerDown(event) {{
            if (!raycastEnabled) return;
            
            // Don't process if clicking on gizmo
            if (transformControl && transformControl.dragging) return;
            
            // Check if clicking on gizmo handle - let TransformControls handle it
            if (isClickOnGizmo(event)) return;
            
            isDragging = false;
            const hit = raycastEntity(event.clientX, event.clientY);
            
            // Handle selection within the iframe
            if (selectionEnabled && hit) {{
                if (event.shiftKey) {{
                    if (selectedEntities.has(hit.entityId)) {{
                        selectedEntities.delete(hit.entityId);
                    }} else {{
                        selectedEntities.add(hit.entityId);
                    }}
                }} else {{
                    selectedEntities.clear();
                    selectedEntities.add(hit.entityId);
                }}
                updateSelectionVisuals();
                updateGizmo();
                
                // Notify parent about selection change
                if (window.parent !== window) {{
                    window.parent.postMessage({{
                        type: 'selection-change',
                        selection: Array.from(selectedEntities)
                    }}, '*');
                }}
            }} else if (selectionEnabled && !event.shiftKey) {{
                selectedEntities.clear();
                updateSelectionVisuals();
                updateGizmo();
                
                if (window.parent !== window) {{
                    window.parent.postMessage({{
                        type: 'selection-change',
                        selection: Array.from(selectedEntities)
                    }}, '*');
                }}
            }}
            
            notifyPointerEvent('pointer-down', event, hit);
        }}
        
        function onPointerMove(event) {{
            if (!raycastEnabled) return;
            
            const rect = renderer.domElement.getBoundingClientRect();
            mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
            mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
            
            raycaster.setFromCamera(mouse, camera);
            
            // Check for hover
            const selectableObjects = [];
            for (const [entityId, obj] of entityMap) {{
                if (obj) selectableObjects.push(obj);
            }}
            
            const intersects = raycaster.intersectObjects(selectableObjects, true);
            if (intersects.length > 0 && !isGizmoDragging) {{
                renderer.domElement.classList.add('hovering');
            }} else {{
                renderer.domElement.classList.remove('hovering');
            }}
            
            if (event.buttons > 0) {{
                isDragging = true;
            }}
        }}
        
        function onPointerUp(event) {{
            if (!raycastEnabled) return;
            notifyPointerEvent('pointer-up', event, null);
        }}
        
        // Add event listeners
        renderer.domElement.addEventListener('pointerdown', onPointerDown);
        renderer.domElement.addEventListener('pointermove', onPointerMove);
        renderer.domElement.addEventListener('pointerup', onPointerUp);
        renderer.domElement.addEventListener('contextmenu', e => e.preventDefault());
        
        // Listen for messages from parent
        window.addEventListener('message', function(event) {{
            if (event.data && event.data.type === 'update-selection') {{
                selectedEntities = new Set(event.data.selection);
                updateSelectionVisuals();
                updateGizmo();
            }} else if (event.data && event.data.type === 'update-gizmo') {{
                updateGizmo();
            }} else if (event.data && event.data.type === 'update-selection-style') {{
                selectionStyle = {{ ...selectionStyle, ...event.data.style }};
                updateSelectionVisuals();
            }} else if (event.data && event.data.type === 'update-state') {{
                const data = event.data;
                
                // Camera
                if (data.camX !== undefined) {{
                    state.camX = data.camX;
                    state.camY = data.camY;
                    state.camZ = data.camZ;
                    camera.position.set(state.camX, state.camY, state.camZ);
                }}
                if (data.targetX !== undefined) {{
                    state.targetX = data.targetX;
                    state.targetY = data.targetY;
                    state.targetZ = data.targetZ;
                    camera.lookAt(state.targetX, state.targetY, state.targetZ);
                }}
                
                // Scene settings
                if (data.autoRotate !== undefined) state.autoRotate = data.autoRotate;
                if (data.rotSpeed !== undefined) state.rotSpeed = data.rotSpeed;
                if (data.scale !== undefined) state.scale = data.scale;
                if (data.color !== undefined) state.color = data.color;
                if (data.background !== undefined) {{
                    state.background = data.background;
                    scene.background = new THREE.Color(state.background);
                }}
                if (data.showGrid !== undefined) state.showGrid = data.showGrid;
                if (data.showAxes !== undefined) state.showAxes = data.showAxes;
                if (data.wireframe !== undefined) state.wireframe = data.wireframe;
                
                updateTransform();
                
                if (data.selection !== undefined) {{
                    selectedEntities = new Set(data.selection);
                    updateSelectionVisuals();
                }}
                if (data.gizmo !== undefined) {{
                    updateGizmo(data.gizmo);
                }}
                // NOTE: We intentionally do NOT update models via postMessage
                // because TransformControls handles transforms directly in the iframe.
                // Sending model transforms would create a race condition where
                // updateModels resets the object while the user is dragging.
                // Model changes (add/remove) are handled by iframe reload instead.
            }}
        }});
        
        async function loadModel() {{
            try {{
                {model_loading_code}
                
                // Initialize after models are loaded
                transformControl = initTransformControls();
                updateSelectionVisuals();
                updateGizmo();
            }} catch (e) {{
                console.error('Error:', e);
                loadingEl.style.display = 'none';
                errorEl.style.display = 'block';
                errorEl.textContent = 'Error: ' + e.message;
            }}
        }}
        
        function updateTransform() {{
            if (!modelContainer) return;
            modelContainer.position.set({pos_x}, {pos_y}, {pos_z});
            modelContainer.scale.setScalar(state.scale);
            if (!state.autoRotate) {{
                modelContainer.rotation.set(state.rotX, state.rotY, state.rotZ);
            }} else {{
                modelContainer.rotation.x = state.rotX;
                modelContainer.rotation.z = state.rotZ;
            }}
            modelContainer.traverse(function (child) {{
                if (child.isMesh && child.material) {{
                    child.material.wireframe = state.wireframe;
                }}
            }});
        }}
        
        // Update model transforms without full reload
        function updateModels(modelsData) {{
            if (!modelsData || !Array.isArray(modelsData)) return;
            
            const seenIds = new Set();
            
            for (const model of modelsData) {{
                const entityId = model.entityId;
                seenIds.add(entityId);
                
                let obj = entityMap.get(entityId);
                if (!obj) {{
                    // Create new cube for missing objects
                    if (model.format === 'cube') {{
                        const geometry = new THREE.BoxGeometry(1, 1, 1);
                        const material = new THREE.MeshStandardMaterial({{ 
                            color: model.color || '#ffffff',
                            roughness: 0.5, 
                            metalness: 0.3 
                        }});
                        obj = new THREE.Mesh(geometry, material);
                        obj.castShadow = true;
                        obj.receiveShadow = true;
                    }} else {{
                        continue;
                    }}
                    obj.userData = {{ entityId: entityId }};
                    modelContainer.add(obj);
                    entityMap.set(entityId, obj);
                }}
                
                if (obj) {{
                    obj.position.set(model.posX || 0, model.posY || 0, model.posZ || 0);
                    obj.rotation.set(
                        (model.rotX || 0) * Math.PI / 180,
                        (model.rotY || 0) * Math.PI / 180,
                        (model.rotZ || 0) * Math.PI / 180
                    );
                    obj.scale.setScalar(model.scale !== undefined ? model.scale : 1);
                    
                    if (obj.material && obj.material.color && model.color) {{
                        obj.material.color.set(model.color);
                    }}
                }}
            }}
            
            // Remove deleted models
            for (const [id, obj] of entityMap) {{
                if (!seenIds.has(id)) {{
                    if (obj.parent) obj.parent.remove(obj);
                    if (obj.geometry) obj.geometry.dispose();
                    if (obj.material) {{
                        if (Array.isArray(obj.material)) {{
                            obj.material.forEach(m => m.dispose());
                        }} else {{
                            obj.material.dispose();
                        }}
                    }}
                    entityMap.delete(id);
                }}
            }}
            
            updateSelectionVisuals();
            updateGizmo();
        }}
        
        window.updateThreeView = function(params) {{
            state = {{ ...state, ...params }};
            if (params.camX !== undefined) camera.position.set(state.camX, state.camY, state.camZ);
            if (params.targetX !== undefined) camera.lookAt(state.targetX, state.targetY, state.targetZ);
            if (params.background !== undefined) scene.background = new THREE.Color(state.background);
            updateTransform();
        }};
        
        // Expose functions for external control
        window.setSelection = function(selection) {{
            selectedEntities = new Set(selection);
            updateSelectionVisuals();
            updateGizmo();
        }};
        
        window.setGizmoConfig = function(config) {{
            updateGizmo();
        }};
        
        function animate() {{
            requestAnimationFrame(animate);
            if (state.autoRotate && modelContainer) {{
                autoRotY += state.rotSpeed * 0.01;
                modelContainer.rotation.y = state.rotY + autoRotY;
            }}
            
            // Update selection outlines to track their source meshes
            for (const [outlineGroup, sourceMesh] of outlineToMeshMap) {{
                if (sourceMesh.parent) {{
                    const box = new THREE.Box3().setFromObject(sourceMesh);
                    const center = box.getCenter(new THREE.Vector3());
                    outlineGroup.position.copy(center);
                    outlineGroup.rotation.copy(sourceMesh.rotation);
                }}
            }}
            
            {shader_uniforms}
            renderer.render(scene, camera);
        }}
        
        window.addEventListener('resize', () => {{
            const w = container.clientWidth || window.innerWidth;
            const h = container.clientHeight || window.innerHeight;
            camera.aspect = w / h;
            camera.updateProjectionMatrix();
            renderer.setSize(w, h);
        }});
        
        loadModel();
        updateTransform();
        animate();
        console.log("Dioxus Three: Running with interaction support");
    </script>
</body>
</html>"##,
        bg = props.background,
        loader_script = loader_script,
        fmt = format_str,
        cam_x = props.cam_x,
        cam_y = props.cam_y,
        cam_z = props.cam_z,
        target_x = props.target_x,
        target_y = props.target_y,
        target_z = props.target_z,
        shadows = props.shadows.to_string().to_lowercase(),
        show_grid = props.show_grid.to_string().to_lowercase(),
        show_axes = props.show_axes.to_string().to_lowercase(),
        rot_x = rot_x_rad,
        rot_y = rot_y_rad,
        rot_z = rot_z_rad,
        scale = props.scale,
        color = props.color,
        auto_rotate = props.auto_rotate.to_string().to_lowercase(),
        rot_speed = props.rot_speed,
        wireframe = props.wireframe.to_string().to_lowercase(),
        pos_x = props.pos_x,
        pos_y = props.pos_y,
        pos_z = props.pos_z,
        shader_uniforms = shader_uniforms,
        model_loading_code = model_loading_code,
        selection_ids_json = selection_ids_json,
        gizmo_config_json = gizmo_config_json,
        selection_style_json = selection_style_json,
        raycast_enabled = raycast_enabled.to_string().to_lowercase(),
        selection_enabled = selection_enabled.to_string().to_lowercase(),
    );

    html
}

/// Build shader code for the Three.js scene
pub fn build_shader_code(shader: &ShaderPreset) -> (String, String, bool) {
    match shader {
        ShaderPreset::None => (String::new(), String::new(), false),
        _ => {
            let vert = shader.vertex_shader().unwrap_or_default();
            let frag = shader.fragment_shader().unwrap_or_default();
            let animated = shader.is_animated();

            let material_code = format!(
                r#"
            // Shader material
            const shaderMaterial = new THREE.ShaderMaterial({{
                uniforms: {{
                    u_time: {{ value: 0 }},
                    u_color: {{ value: new THREE.Color(state.color) }}
                }},
                vertexShader: `{}`,
                fragmentShader: `{}`,
                transparent: true,
                side: THREE.DoubleSide
            }});
            material = shaderMaterial;
            "#,
                vert.replace("`", "\\`"),
                frag.replace("`", "\\`")
            );

            let uniforms_code = r#"
            // Update shader uniforms
            if (material && material.uniforms) {
                material.uniforms.u_time.value = performance.now() * 0.001;
                material.uniforms.u_color.value.set(state.color);
            }
            "#
            .to_string();

            (material_code, uniforms_code, animated)
        }
    }
}

/// Serialize model configs to JSON for postMessage updates (desktop iframe)
pub fn models_to_json(models: &[ModelConfig]) -> String {
    let mut parts = Vec::new();
    for (idx, model) in models.iter().enumerate() {
        parts.push(format!(
            r#"{{"entityId":{},"url":"{}","format":"{}","posX":{},"posY":{},"posZ":{},"rotX":{},"rotY":{},"rotZ":{},"scale":{},"color":"{}"}}"#,
            idx,
            model.url.replace('\\', "\\\\").replace('"', "\\\""),
            model.format.as_str(),
            model.pos_x,
            model.pos_y,
            model.pos_z,
            model.rot_x,
            model.rot_y,
            model.rot_z,
            model.scale,
            model.color.replace('\\', "\\\\").replace('"', "\\\"")
        ));
    }
    format!("[{}]", parts.join(","))
}
