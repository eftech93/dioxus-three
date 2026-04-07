//! Web/WASM implementation of ThreeView using canvas element

use crate::ThreeViewProps;
use dioxus::prelude::*;

/// A Three.js 3D viewer component for Dioxus Web
///
/// Uses a native canvas element with Three.js injected via script tags.
#[component]
pub fn ThreeView(props: ThreeViewProps) -> Element {
    let canvas_id = use_signal(|| format!("three-canvas-{}", js_sys::Math::random()));
    let class = props.class.clone();

    // Store props in individual signals so we can track changes
    let mut cam_x = use_signal(|| props.cam_x);
    let mut cam_y = use_signal(|| props.cam_y);
    let mut cam_z = use_signal(|| props.cam_z);
    let mut target_x = use_signal(|| props.target_x);
    let mut target_y = use_signal(|| props.target_y);
    let mut target_z = use_signal(|| props.target_z);
    let mut auto_rotate = use_signal(|| props.auto_rotate);
    let mut rot_speed = use_signal(|| props.rot_speed);
    let mut rot_x = use_signal(|| props.rot_x);
    let mut rot_y = use_signal(|| props.rot_y);
    let mut rot_z = use_signal(|| props.rot_z);
    let mut scale = use_signal(|| props.scale);
    let mut color = use_signal(|| props.color.clone());
    let mut background = use_signal(|| props.background.clone());
    let mut show_grid = use_signal(|| props.show_grid);
    let mut show_axes = use_signal(|| props.show_axes);
    let mut wireframe = use_signal(|| props.wireframe);
    let mut models = use_signal(|| props.models.clone());

    // Update signals when props change
    use_effect(use_reactive((&props,), move |(new_props,)| {
        cam_x.set(new_props.cam_x);
        cam_y.set(new_props.cam_y);
        cam_z.set(new_props.cam_z);
        target_x.set(new_props.target_x);
        target_y.set(new_props.target_y);
        target_z.set(new_props.target_z);
        auto_rotate.set(new_props.auto_rotate);
        rot_speed.set(new_props.rot_speed);
        rot_x.set(new_props.rot_x);
        rot_y.set(new_props.rot_y);
        rot_z.set(new_props.rot_z);
        scale.set(new_props.scale);
        color.set(new_props.color.clone());
        background.set(new_props.background.clone());
        show_grid.set(new_props.show_grid);
        show_axes.set(new_props.show_axes);
        wireframe.set(new_props.wireframe);
        models.set(new_props.models.clone());
    }));

    // Effect that runs when models change - reload the scene models
    use_effect(move || {
        let id = canvas_id();
        let mds = models();

        web_sys::console::log_1(&format!("ThreeView models changed: {} models", mds.len()).into());

        wasm_bindgen_futures::spawn_local(async move {
            // Load required loaders first
            load_required_loaders(&mds).await;

            // Wait a bit for loaders to be available
            wait_ms(500).await;

            // Generate model loading code
            let model_js = build_models_js(&mds);

            // Reload models in the scene
            let js_code = format!(
                r#"
                (function() {{
                    const canvas = document.getElementById('{}');
                    if (!canvas || !canvas.dioxusThreeState) {{
                        console.log('Canvas or state not found for model update');
                        return;
                    }}
                    
                    const {{ scene, modelContainer, state }} = canvas.dioxusThreeState;
                    const THREE = window.THREE;
                    
                    // Clear existing models
                    while(modelContainer.children.length > 0){{
                        modelContainer.remove(modelContainer.children[0]);
                    }}
                    
                    // Load new models
                    {}
                    
                    console.log('Models reloaded: {} models');
                }})();
                "#,
                id,
                model_js,
                mds.len()
            );

            if let Err(e) = js_sys::eval(&js_code) {
                web_sys::console::error_1(&format!("Failed to update models: {:?}", e).into());
            }
        });
    });

    // Effect that runs when any of the transform/camera values change
    use_effect(move || {
        let id = canvas_id();
        let cx = cam_x();
        let cy = cam_y();
        let cz = cam_z();
        let tx = target_x();
        let ty = target_y();
        let tz = target_z();
        let ar = auto_rotate();
        let rs = rot_speed();
        let rx = rot_x();
        let ry = rot_y();
        let rz = rot_z();
        let sc = scale();
        let col = color();
        let bg = background();
        let sg = show_grid();
        let sa = show_axes();
        let wf = wireframe();

        // Build update JS
        let rot_x_rad = rx.to_radians();
        let rot_y_rad = ry.to_radians();
        let rot_z_rad = rz.to_radians();

        web_sys::console::log_1(
            &format!(
                "ThreeView update: cam=({:.1},{:.1},{:.1}), auto_rotate={}, scale={:.1}",
                cx, cy, cz, ar, sc
            )
            .into(),
        );

        let js_code = format!(
            r#"
            (function() {{
                const canvas = document.getElementById('{}');
                if (!canvas || !canvas.dioxusThreeState) {{
                    console.log('Canvas or state not found');
                    return;
                }}
                
                const {{ state, camera }} = canvas.dioxusThreeState;
                
                state.camX = {};
                state.camY = {};
                state.camZ = {};
                state.targetX = {};
                state.targetY = {};
                state.targetZ = {};
                state.autoRotate = {};
                state.rotSpeed = {};
                state.rotX = {};
                state.rotY = {};
                state.rotZ = {};
                state.scale = {};
                state.color = '{}';
                state.background = '{}';
                state.showGrid = {};
                state.showAxes = {};
                state.wireframe = {};
                
                // Update camera position immediately
                if (camera) {{
                    camera.position.set(state.camX, state.camY, state.camZ);
                    camera.lookAt(state.targetX, state.targetY, state.targetZ);
                    console.log('Camera position updated:', state.camX, state.camY, state.camZ);
                }}
                
                console.log('JS state updated:', {{ camX: state.camX, camY: state.camY, camZ: state.camZ, scale: state.scale, autoRotate: state.autoRotate }});
            }})();
            "#,
            id,
            cx,
            cy,
            cz,
            tx,
            ty,
            tz,
            ar.to_string().to_lowercase(),
            rs,
            rot_x_rad,
            rot_y_rad,
            rot_z_rad,
            sc,
            col,
            bg,
            sg.to_string().to_lowercase(),
            sa.to_string().to_lowercase(),
            wf.to_string().to_lowercase()
        );

        if let Err(e) = js_sys::eval(&js_code) {
            web_sys::console::error_1(&format!("Failed to update scene: {:?}", e).into());
        }
    });

    rsx! {
        canvas {
            id: canvas_id(),
            class: class.as_str(),
            style: "width: 100%; height: 100%; display: block;",
            onmounted: move |_element| {
                let id = canvas_id();
                wasm_bindgen_futures::spawn_local(async move {
                    init_three_js(&id).await;
                });
            },
        }
    }
}

/// Wait for a specified number of milliseconds
async fn wait_ms(ms: u32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(window) = web_sys::window() {
            let _ =
                window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms as i32);
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// Initialize Three.js on the canvas
async fn init_three_js(canvas_id: &str) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            web_sys::console::error_1(&"No global window".into());
            return;
        }
    };

    let document = match window.document() {
        Some(d) => d,
        None => {
            web_sys::console::error_1(&"No document".into());
            return;
        }
    };

    let canvas = match document.get_element_by_id(canvas_id) {
        Some(el) => el,
        None => {
            web_sys::console::error_1(&format!("Canvas element {} not found", canvas_id).into());
            return;
        }
    };

    // Inject Three.js if not already loaded
    if !is_three_js_loaded(&document) {
        web_sys::console::log_1(&"Loading Three.js...".into());
        load_script(
            &document,
            "https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js",
        )
        .await;
    }

    // Wait for Three.js to be available
    let mut attempts = 0;
    while !is_three_js_loaded(&document) && attempts < 50 {
        wait_ms(200).await;
        attempts += 1;
    }

    if !is_three_js_loaded(&document) {
        web_sys::console::error_1(&"Failed to load Three.js after waiting".into());
        return;
    }

    web_sys::console::log_1(&"Three.js loaded successfully!".into());
    create_scene(&canvas);
}

/// Check if Three.js is already loaded
fn is_three_js_loaded(_document: &web_sys::Document) -> bool {
    if let Some(window) = web_sys::window() {
        let result = js_sys::Reflect::get(&window, &"THREE".into());
        return result.map(|v| !v.is_undefined()).unwrap_or(false);
    }
    false
}

/// Check if a specific script is loaded
fn is_script_loaded(document: &web_sys::Document, url: &str) -> bool {
    let scripts = document.get_elements_by_tag_name("script");
    for i in 0..scripts.length() {
        if let Some(script) = scripts.item(i) {
            if let Some(src) = script.get_attribute("src") {
                if src.contains(url) {
                    return true;
                }
            }
        }
    }
    false
}

/// Load a script dynamically and wait for it to load
async fn load_script(document: &web_sys::Document, url: &str) {
    if is_script_loaded(document, url) {
        return;
    }

    web_sys::console::log_1(&format!("Loading script: {}", url).into());

    let script = match document.create_element("script") {
        Ok(s) => s,
        Err(_) => return,
    };

    let _ = script.set_attribute("src", url);
    let _ = script.set_attribute("async", "false");

    let head = match document.head() {
        Some(h) => h,
        None => return,
    };

    let _ = head.append_child(&script);

    let mut attempts = 0;
    while !is_script_loaded(document, url) && attempts < 50 {
        wait_ms(100).await;
        attempts += 1;
    }

    wait_ms(500).await;

    web_sys::console::log_1(&format!("Script loaded: {}", url).into());
}

/// Create the Three.js scene
fn create_scene(canvas: &web_sys::Element) {
    let js_code = format!(
        r#"
        (function() {{
            const canvas = document.getElementById('{}');
            if (!canvas) {{
                console.error('Canvas not found');
                return;
            }}
            
            function initScene() {{
                if (typeof window.THREE === 'undefined') {{
                    setTimeout(initScene, 100);
                    return;
                }}
                createScene();
            }}
            
            function createScene() {{
                const THREE = window.THREE;
                
                const scene = new THREE.Scene();
                scene.background = new THREE.Color('#1a1a2e');
                
                if (canvas.clientWidth === 0 || canvas.clientHeight === 0) {{
                    canvas.style.width = '100%';
                    canvas.style.height = '100%';
                    canvas.width = canvas.parentElement ? canvas.parentElement.clientWidth : 800;
                    canvas.height = canvas.parentElement ? canvas.parentElement.clientHeight : 600;
                }}
                
                const width = canvas.clientWidth || canvas.width || 800;
                const height = canvas.clientHeight || canvas.height || 600;
                const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
                camera.position.set(8, 8, 8);
                camera.lookAt(0, 0, 0);
                
                const renderer = new THREE.WebGLRenderer({{ canvas: canvas, antialias: true }});
                renderer.setSize(width, height);
                renderer.setPixelRatio(window.devicePixelRatio);
                renderer.shadowMap.enabled = true;
                
                const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
                scene.add(ambientLight);
                
                const dirLight = new THREE.DirectionalLight(0xffffff, 1.2);
                dirLight.position.set(10, 20, 10);
                dirLight.castShadow = true;
                scene.add(dirLight);
                
                const fillLight = new THREE.DirectionalLight(0xffffff, 0.4);
                fillLight.position.set(-10, 10, -10);
                scene.add(fillLight);
                
                let gridHelper = new THREE.GridHelper(20, 20, 0x444444, 0x222222);
                scene.add(gridHelper);
                
                let axesHelper = new THREE.AxesHelper(2);
                scene.add(axesHelper);
                
                let modelContainer = new THREE.Group();
                scene.add(modelContainer);
                
                // Default cube
                const geometry = new THREE.BoxGeometry(1, 1, 1);
                const material = new THREE.MeshStandardMaterial({{ 
                    color: '#ff6b6b', 
                    roughness: 0.5, 
                    metalness: 0.3
                }});
                const model = new THREE.Mesh(geometry, material);
                model.castShadow = true;
                model.receiveShadow = true;
                modelContainer.add(model);
                
                const state = {{
                    rotX: 0,
                    rotY: 0,
                    rotZ: 0,
                    autoRotate: true,
                    rotSpeed: 1.0,
                    scale: 1.0,
                    color: '#ff6b6b',
                    camX: 8,
                    camY: 8,
                    camZ: 8,
                    targetX: 0,
                    targetY: 0,
                    targetZ: 0,
                    showGrid: true,
                    showAxes: true,
                    wireframe: false,
                    background: '#1a1a2e'
                }};
                
                let autoRotY = 0;
                
                function animate() {{
                    requestAnimationFrame(animate);
                    
                    camera.position.set(state.camX, state.camY, state.camZ);
                    camera.lookAt(state.targetX, state.targetY, state.targetZ);
                    
                    scene.background = new THREE.Color(state.background);
                    
                    if (state.showGrid && !gridHelper.parent) {{
                        scene.add(gridHelper);
                    }} else if (!state.showGrid && gridHelper.parent) {{
                        scene.remove(gridHelper);
                    }}
                    
                    if (state.showAxes && !axesHelper.parent) {{
                        scene.add(axesHelper);
                    }} else if (!state.showAxes && axesHelper.parent) {{
                        scene.remove(axesHelper);
                    }}
                    
                    modelContainer.traverse(function(child) {{
                        if (child.isMesh && child.material) {{
                            if (Array.isArray(child.material)) {{
                                child.material.forEach(m => m.wireframe = state.wireframe);
                            }} else {{
                                child.material.wireframe = state.wireframe;
                            }}
                        }}
                    }});
                    
                    modelContainer.scale.setScalar(state.scale);
                    
                    if (state.autoRotate) {{
                        autoRotY += state.rotSpeed * 0.01;
                        modelContainer.rotation.y = state.rotY + autoRotY;
                        modelContainer.rotation.x = state.rotX;
                        modelContainer.rotation.z = state.rotZ;
                    }} else {{
                        modelContainer.rotation.set(state.rotX, state.rotY, state.rotZ);
                    }}
                    
                    renderer.render(scene, camera);
                }}
                animate();
                
                window.addEventListener('resize', () => {{
                    const w = canvas.clientWidth || canvas.width || 800;
                    const h = canvas.clientHeight || canvas.height || 600;
                    renderer.setSize(w, h);
                    camera.aspect = w / h;
                    camera.updateProjectionMatrix();
                }});
                
                canvas.dioxusThreeState = {{ scene, camera, modelContainer, state }};
                
                console.log('Dioxus Three: Scene initialized');
            }}
            
            initScene();
        }})();
        "#,
        canvas.id(),
    );

    if let Err(e) = js_sys::eval(&js_code) {
        web_sys::console::error_1(&format!("Failed to create scene: {:?}", e).into());
    }
}

/// Build JavaScript code to load models
fn build_models_js(models: &[crate::ModelConfig]) -> String {
    if models.is_empty() {
        // Default cube
        return r#"
            const geometry = new THREE.BoxGeometry(1, 1, 1);
            const material = new THREE.MeshStandardMaterial({ 
                color: '#ff6b6b', 
                roughness: 0.5, 
                metalness: 0.3
            });
            const model = new THREE.Mesh(geometry, material);
            model.castShadow = true;
            model.receiveShadow = true;
            modelContainer.add(model);
        "#
        .to_string();
    }

    let mut model_code = String::new();

    for (idx, model) in models.iter().enumerate() {
        let pos_x = model.pos_x;
        let pos_y = model.pos_y;
        let pos_z = model.pos_z;
        let rot_x = model.rot_x.to_radians();
        let rot_y = model.rot_y.to_radians();
        let rot_z = model.rot_z.to_radians();
        let scl = model.scale;
        let color = &model.color;
        let url = &model.url;

        let code = if model.format == crate::ModelFormat::Cube || model.url.is_empty() {
            format!(
                r#"
                // Model {}: Cube
                (function() {{
                    const geometry = new THREE.BoxGeometry(1, 1, 1);
                    const material = new THREE.MeshStandardMaterial({{ 
                        color: '{}', 
                        roughness: 0.5, 
                        metalness: 0.3 
                    }});
                    const mesh = new THREE.Mesh(geometry, material);
                    mesh.position.set({}, {}, {});
                    mesh.rotation.set({}, {}, {});
                    mesh.scale.setScalar({});
                    mesh.castShadow = true;
                    mesh.receiveShadow = true;
                    modelContainer.add(mesh);
                }})();
                "#,
                idx, color, pos_x, pos_y, pos_z, rot_x, rot_y, rot_z, scl
            )
        } else {
            // Load external model using appropriate loader
            let loader_class = model.format.loader_js();
            let is_geometry_loader = matches!(
                model.format,
                crate::ModelFormat::Stl | crate::ModelFormat::Ply
            );

            if is_geometry_loader {
                // Geometry loaders (STL, PLY) - load geometry and create mesh
                format!(
                    r#"
                    // Model {}: {} 
                    (function() {{
                        if (typeof THREE.{loader} === 'undefined') {{
                            console.warn('Loader {loader} not available');
                            // Fallback cube
                            const geometry = new THREE.BoxGeometry(1, 1, 1);
                            const material = new THREE.MeshStandardMaterial({{ color: '{color}' }});
                            const mesh = new THREE.Mesh(geometry, material);
                            mesh.position.set({pos_x}, {pos_y}, {pos_z});
                            mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                            mesh.scale.setScalar({scl});
                            modelContainer.add(mesh);
                            return;
                        }}
                        
                        const loader = new THREE.{loader}();
                        loader.load(
                            '{url}',
                            function(geometry) {{
                                const material = new THREE.MeshStandardMaterial({{ 
                                    color: '{color}', 
                                    roughness: 0.5, 
                                    metalness: 0.1 
                                }});
                                const mesh = new THREE.Mesh(geometry, material);
                                mesh.position.set({pos_x}, {pos_y}, {pos_z});
                                mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                                mesh.scale.setScalar({scl});
                                mesh.castShadow = true;
                                mesh.receiveShadow = true;
                                modelContainer.add(mesh);
                                console.log('Loaded model {idx}: {url}');
                            }},
                            undefined,
                            function(error) {{
                                console.error('Failed to load model {idx}:', error);
                                // Fallback cube
                                const geometry = new THREE.BoxGeometry(1, 1, 1);
                                const material = new THREE.MeshStandardMaterial({{ color: '{color}' }});
                                const mesh = new THREE.Mesh(geometry, material);
                                mesh.position.set({pos_x}, {pos_y}, {pos_z});
                                mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                                mesh.scale.setScalar({scl});
                                modelContainer.add(mesh);
                            }}
                        );
                    }})();
                    "#,
                    idx,
                    model.format.as_str(),
                    loader = loader_class,
                    url = url.replace("'", "\\'"),
                    color = color,
                    pos_x = pos_x,
                    pos_y = pos_y,
                    pos_z = pos_z,
                    rot_x = rot_x,
                    rot_y = rot_y,
                    rot_z = rot_z,
                    scl = scl,
                    idx = idx
                )
            } else {
                // Scene loaders (GLTF, OBJ, FBX) - load object/scene
                format!(
                    r#"
                    // Model {}: {}
                    (function() {{
                        if (typeof THREE.{loader} === 'undefined') {{
                            console.warn('Loader {loader} not available');
                            // Fallback cube
                            const geometry = new THREE.BoxGeometry(1, 1, 1);
                            const material = new THREE.MeshStandardMaterial({{ color: '{color}' }});
                            const mesh = new THREE.Mesh(geometry, material);
                            mesh.position.set({pos_x}, {pos_y}, {pos_z});
                            mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                            mesh.scale.setScalar({scl});
                            modelContainer.add(mesh);
                            return;
                        }}
                        
                        const loader = new THREE.{loader}();
                        loader.load(
                            '{url}',
                            function(object) {{
                                let model = object.scene || object;
                                model.position.set({pos_x}, {pos_y}, {pos_z});
                                model.rotation.set({rot_x}, {rot_y}, {rot_z});
                                model.scale.setScalar({scl});
                                model.traverse(function(child) {{
                                    if (child.isMesh) {{
                                        child.castShadow = true;
                                        child.receiveShadow = true;
                                    }}
                                }});
                                modelContainer.add(model);
                                console.log('Loaded model {idx}: {url}');
                            }},
                            function(xhr) {{
                                console.log('Model {idx} loading: ' + (xhr.loaded / xhr.total * 100) + '%');
                            }},
                            function(error) {{
                                console.error('Failed to load model {idx}:', error);
                                // Fallback cube
                                const geometry = new THREE.BoxGeometry(1, 1, 1);
                                const material = new THREE.MeshStandardMaterial({{ color: '{color}' }});
                                const mesh = new THREE.Mesh(geometry, material);
                                mesh.position.set({pos_x}, {pos_y}, {pos_z});
                                mesh.rotation.set({rot_x}, {rot_y}, {rot_z});
                                mesh.scale.setScalar({scl});
                                modelContainer.add(mesh);
                            }}
                        );
                    }})();
                    "#,
                    idx,
                    model.format.as_str(),
                    loader = loader_class,
                    url = url.replace("'", "\\'"),
                    color = color,
                    pos_x = pos_x,
                    pos_y = pos_y,
                    pos_z = pos_z,
                    rot_x = rot_x,
                    rot_y = rot_y,
                    rot_z = rot_z,
                    scl = scl,
                    idx = idx
                )
            }
        };

        model_code.push_str(&code);
    }

    model_code
}

/// Load all required loaders for the given models
async fn load_required_loaders(models: &[crate::ModelConfig]) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    // Collect unique formats that need loaders
    let mut unique_formats: Vec<crate::ModelFormat> = vec![];
    for model in models {
        if model.format != crate::ModelFormat::Cube
            && !model.url.is_empty()
            && !unique_formats.contains(&model.format)
        {
            unique_formats.push(model.format.clone());
        }
    }

    // Load each required loader
    for format in &unique_formats {
        let loader_url = get_loader_url(format);
        if !loader_url.is_empty() {
            load_script(&document, loader_url).await;

            // Load extra dependencies (like fflate for FBX)
            for extra in format.extra_scripts() {
                load_script(&document, extra).await;
            }
        }
    }
}

/// Get the loader URL for a format
fn get_loader_url(format: &crate::ModelFormat) -> &'static str {
    match format {
        crate::ModelFormat::Obj => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/OBJLoader.js"
        }
        crate::ModelFormat::Fbx => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/FBXLoader.js"
        }
        crate::ModelFormat::Gltf | crate::ModelFormat::Glb => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/GLTFLoader.js"
        }
        crate::ModelFormat::Stl => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/STLLoader.js"
        }
        crate::ModelFormat::Ply => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/PLYLoader.js"
        }
        crate::ModelFormat::Dae => {
            "https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/loaders/ColladaLoader.js"
        }
        crate::ModelFormat::Json => "",
        crate::ModelFormat::Cube => "",
    }
}
