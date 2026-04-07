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

/// Build JavaScript code for loading multiple models
pub fn build_multi_model_loading(models: &[ModelConfig], shadows: bool) -> String {
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
                r#"(function() {{ const geometry = new THREE.BoxGeometry(1, 1, 1); const material = new THREE.MeshStandardMaterial({{ color: "{color}", roughness: 0.5, metalness: 0.3 }}); const mesh = new THREE.Mesh(geometry, material); mesh.position.set({pos_x}, {pos_y}, {pos_z}); mesh.rotation.set({rot_x}, {rot_y}, {rot_z}); mesh.scale.setScalar({scale}); mesh.castShadow = {shadows_str}; mesh.receiveShadow = {shadows_str}; modelContainer.add(mesh); }})();"#
            )
        } else if is_geometry_loader {
            format!(
                r#"(function() {{ const loader = new THREE.{loader_class}(); loader.load("{url}", function(geometry) {{ const material = new THREE.MeshStandardMaterial({{ color: "{color}", roughness: 0.5, metalness: 0.1, side: THREE.DoubleSide }}); const mesh = new THREE.Mesh(geometry, material); mesh.position.set({pos_x}, {pos_y}, {pos_z}); mesh.rotation.set({rot_x}, {rot_y}, {rot_z}); mesh.scale.setScalar({scale}); mesh.castShadow = {shadows_str}; mesh.receiveShadow = {shadows_str}; modelContainer.add(mesh); }}, undefined, function(err) {{ console.error('Failed to load model {idx}:', err); }}); }})();"#
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
                r#"(function() {{ const loader = new THREE.{loader_class}(); loader.load("{url}", function(object) {{ let model = object.scene || object.dae || object; model.position.set({pos_x}, {pos_y}, {pos_z}); model.rotation.set({rot_x}, {rot_y}, {rot_z}); model.scale.setScalar({scale}); model.traverse(function(child) {{ if (child.isMesh) {{ child.castShadow = {shadows_str}; child.receiveShadow = {shadows_str}; {color_js} }} }}); modelContainer.add(model); }}, undefined, function(err) {{ console.error('Failed to load model {idx}:', err); }}); }})();"#,
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

/// Build JavaScript code for loading a single model
pub fn build_single_model_loading(
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
        return "const geometry = new THREE.BoxGeometry(1, 1, 1); let material = new THREE.MeshStandardMaterial({ color: state.color, roughness: 0.5, metalness: 0.3, wireframe: state.wireframe }); model = new THREE.Mesh(geometry, material); model.castShadow = true; model.receiveShadow = true; modelContainer.add(model); loadingEl.style.display = 'none';".to_string();
    }

    if is_geometry_loader {
        format!(
            r#"const loader = new THREE.{loader_class}(); loader.load("{url}", function(geometry) {{ loadingEl.style.display = 'none'; const material = new THREE.MeshStandardMaterial({{ color: state.color, roughness: 0.5, metalness: 0.1, wireframe: state.wireframe, side: THREE.DoubleSide }}); model = new THREE.Mesh(geometry, material); model.castShadow = {shadows_str}; model.receiveShadow = {shadows_str}; if ({auto_center_str}) {{ const box = new THREE.Box3().setFromObject(model); const center = box.getCenter(new THREE.Vector3()); model.position.sub(center); }} if ({auto_scale_str}) {{ const box = new THREE.Box3().setFromObject(model); const size = box.getSize(new THREE.Vector3()); const maxDim = Math.max(size.x, size.y, size.z); if (maxDim > 0) {{ const s = 2 / maxDim; model.scale.setScalar(s); }} }} modelContainer.add(model); updateTransform(); }}, function(xhr) {{ const percent = xhr.loaded / xhr.total * 100; loadingEl.textContent = 'Loading: ' + Math.round(percent) + '%'; }}, function(error) {{ console.error('Error loading model:', error); loadingEl.style.display = 'none'; errorEl.style.display = 'block'; errorEl.textContent = 'Failed to load model: ' + (error.message || 'Unknown error'); const geometry = new THREE.BoxGeometry(1, 1, 1); const material = new THREE.MeshStandardMaterial({{ color: 0xff6b6b }}); model = new THREE.Mesh(geometry, material); modelContainer.add(model); }});"#
        )
    } else {
        format!(
            r#"const loader = new THREE.{loader_class}(); loader.load("{url}", function(object) {{ loadingEl.style.display = 'none'; if (object.scene) {{ model = object.scene; }} else if (object.dae) {{ model = object.scene; }} else {{ model = object; }} model.traverse(function(child) {{ if (child.isMesh) {{ child.castShadow = {shadows_str}; child.receiveShadow = {shadows_str}; if (!child.material) {{ child.material = new THREE.MeshStandardMaterial({{ color: state.color, roughness: 0.5, metalness: 0.3 }}); }} const materials = Array.isArray(child.material) ? child.material : [child.material]; materials.forEach(m => {{ if (m.opacity !== undefined && m.opacity < 0.1) m.opacity = 1.0; if (m.transparent === true && m.opacity < 0.1) m.transparent = false; if (state.color !== '#ff6b6b' && m.color) {{ m.color.set(state.color); }} m.wireframe = state.wireframe; }}); }} }}); if ({auto_center_str}) {{ const box = new THREE.Box3().setFromObject(model); const center = box.getCenter(new THREE.Vector3()); model.position.sub(center); }} if ({auto_scale_str}) {{ const box = new THREE.Box3().setFromObject(model); const size = box.getSize(new THREE.Vector3()); const maxDim = Math.max(size.x, size.y, size.z); if (maxDim > 0) {{ const s = 2 / maxDim; model.scale.setScalar(s); }} }} modelContainer.add(model); updateTransform(); }}, function(xhr) {{ const percent = xhr.loaded / xhr.total * 100; loadingEl.textContent = 'Loading: ' + Math.round(percent) + '%'; }}, function(error) {{ console.error('Error loading model:', error); loadingEl.style.display = 'none'; errorEl.style.display = 'block'; errorEl.textContent = 'Failed to load model: ' + (error.message || 'Unknown error'); const geometry = new THREE.BoxGeometry(1, 1, 1); const material = new THREE.MeshStandardMaterial({{ color: 0xff6b6b }}); model = new THREE.Mesh(geometry, material); modelContainer.add(model); }});"#
        )
    }
}

/// Generate the HTML with embedded Three.js
pub fn generate_three_js_html(props: &ThreeViewProps) -> String {
    let rot_x_rad = props.rot_x.to_radians();
    let rot_y_rad = props.rot_y.to_radians();
    let rot_z_rad = props.rot_z.to_radians();

    // Legacy single-model variables (for backward compatibility with template)
    let loader_url = props.format.loader_url();
    let loader_class = props.format.loader_js();
    let format_str = props.format.as_str();
    let model_url = props.model_url.clone().unwrap_or_default();
    let has_model = !model_url.is_empty() && props.format != ModelFormat::Cube;

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
        build_multi_model_loading(&props.models, props.shadows)
    } else {
        build_single_model_loading(
            &props.format,
            &props.model_url,
            props.auto_center,
            props.auto_scale,
            props.shadows,
        )
    };

    // Build shader code if needed
    let (shader_material_code, shader_uniforms, _shader_animated) =
        build_shader_code(&props.shader);

    // Build the HTML
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
    </style>
</head>
<body>
    <div id="canvas-container"></div>
    <div id="loading">Loading 3D model...</div>
    <div id="error"></div>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
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
        
        async function loadModel() {{
            try {{
                {model_loading_code}
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
        
        window.updateThreeView = function(params) {{
            state = {{ ...state, ...params }};
            if (params.camX !== undefined) camera.position.set(state.camX, state.camY, state.camZ);
            if (params.targetX !== undefined) camera.lookAt(state.targetX, state.targetY, state.targetZ);
            updateTransform();
        }};
        
        function animate() {{
            requestAnimationFrame(animate);
            if (state.autoRotate && modelContainer) {{
                autoRotY += state.rotSpeed * 0.01;
                modelContainer.rotation.y = state.rotY + autoRotY;
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
        console.log("Dioxus Three: Running");
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
