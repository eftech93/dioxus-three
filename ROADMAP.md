# Dioxus Three Roadmap

A practical roadmap for `dioxus-three`: a Dioxus component that wraps Three.js for cross-platform 3D visualization.

## Philosophy

This is a **viewer, visualization toolkit, physics sandbox, and scene orchestration layer**. We prioritize:

1. **Stability** — Reliable model loading and rendering across Desktop/Web/Mobile
2. **Ergonomics** — Clean Rust API over Three.js capabilities
3. **Performance** — No full-scene reloads, efficient state sync
4. **Physics** — Full rigid-body, soft-body, and destruction simulation via Bullet/ammo.js
5. **Engineering & Advanced Visualization** — Measurement, sectioning, photorealism, CAD/BIM workflows

The architecture is: **Rust orchestrates, JavaScript executes.** The Rust API is a thin, type-safe layer that configures and controls Three.js, **ammo.js (Bullet Physics)**, web-ifc-three, and other JS-side libraries.

**On building a physics engine from scratch:** Don't. Bullet Physics (ammo.js) is one of the most battle-tested engines in existence — it powers Blender, was used in countless AAA games, and supports rigid bodies, soft bodies, cloth, vehicles, ragdolls, and destructibles. A from-scratch physics engine would be a multi-year project and would not surpass it.

---

## Phase 0: Foundation (Completed)

### v0.0.1 — Initial Release
- `ThreeView` component with model loading (OBJ, FBX, GLTF, GLB, STL, PLY, DAE, Cube)
- Camera control, auto-rotation, wireframe, grid/axes helpers
- 6 built-in shader presets + custom GLSL support
- Desktop (WebView iframe) and Web (WASM canvas) platforms

### v0.0.2 — Web Platform
- Multi-model support (`Vec<ModelConfig>`)
- Web-specific canvas rendering with `wasm_bindgen`
- Dynamic loader injection for format-specific loaders

### v0.0.3 — Interaction (Phase 1)
- Raycasting with `RaycastConfig`
- Pointer events: `on_pointer_down`, `on_pointer_move`, `on_pointer_up`, `on_pointer_drag`
- `Selection` API with `SelectionMode::Single` / `Multiple`
- Selection outline (wireframe box + inner glow)
- Transform gizmos (Translate, Rotate, Scale) with `World`/`Local` space
- Desktop: `THREE.TransformControls` via iframe + `postMessage`
- Web: Custom-built gizmos with manual raycasting and drag math

### v0.0.4 — Polish & CI
- Selection outline scales with object bounds
- Gizmo depth fix (always render on top)
- Desktop iframe state updates via `postMessage` (no full reload)
- Documentation overhaul with accurate API references
- CI auto-publish to crates.io on version bump

---

## Phase 1: Scene Structure & Stability

Before advanced features, the scene graph and loading pipeline must be rock solid.

### 1.1 Scene Graph / Model Hierarchy
Replace the flat `Vec<ModelConfig>` with hierarchical scene support:

```rust
use dioxus_three::{SceneNode, SceneGraph};

let scene = SceneGraph::new()
    .with_root(
        SceneNode::group("assembly")
            .with_child(SceneNode::model("chassis.glb"))
            .with_child(
                SceneNode::group("wheel-assembly")
                    .with_child(SceneNode::model("wheel.glb").at(1.5, 0.0, 1.0))
                    .with_child(SceneNode::model("wheel.glb").at(-1.5, 0.0, 1.0))
            )
    );
```

**Why this matters:**
- CAD models (glTF/FBX) have nested assemblies
- Enables exploded views and assembly tree browser
- Required for proper transform inheritance

### 1.2 Loading States & Error Handling

```rust
rsx! {
    ThreeView {
        models: models(),
        on_load_progress: move |progress: LoadProgress| {
            // progress.loaded_bytes, progress.total_bytes, progress.model_name
        },
        on_load_error: move |error: LoadError| {
            // error.model_url, error.message (CORS, 404, parse failure)
        },
    }
}
```

**Why this matters:**
- Large CAD models take seconds to load
- Users need progress feedback
- Failed loads currently break the scene silently

### 1.3 Memory Management
- `ModelConfig::unload()` or `SceneNode::remove()` to free GPU resources
- `ThreeView::dispose()` to clean up the entire Three.js scene
- Prevent memory leaks on Desktop when views are destroyed/recreated

### 1.4 Offline Mode
Bundle Three.js and loaders instead of CDN:

```rust
ThreeView {
    models: models(),
    offline: true, // Use bundled Three.js r128 + loaders
}
```

**Implementation:** Vendor Three.js into the crate and inject via `include_str!` for Desktop, or via `manganis` asset bundling for Web.

---

## Phase 2: Visualization Essentials

Core rendering features that every user expects.

### 2.1 Lighting System

```rust
use dioxus_three::lighting::{Light, LightType, ShadowConfig};

let lights = vec![
    Light::directional("sun", "#ffffff", 1.0)
        .at(10.0, 20.0, 10.0)
        .looking_at(0.0, 0.0, 0.0)
        .with_shadows(ShadowConfig { map_size: 2048, bias: -0.0001 }),
    Light::ambient("fill", "#404040", 0.3),
    Light::point("torch", "#ffaa00", 1.5).at(2.0, 3.0, 2.0),
    Light::hemisphere("sky", "#87CEEB", "#362d1d", 0.6),
];
```

### 2.2 Advanced Camera Controllers

```rust
use dioxus_three::camera::{CameraController, OrbitConfig, FpsConfig};

let camera = CameraController::orbit(OrbitConfig {
    target: Vector3::ZERO,
    distance: 10.0,
    min_distance: 0.1,
    max_distance: 1000.0,
    enable_damping: true,
    damping_factor: 0.05,
});
```

Controllers: `Orbit`, `FPS`, `Follow(target_entity)`, `Ortho`.

### 2.3 PBR Materials & Textures

```rust
use dioxus_three::material::{Material, Texture};

let material = Material::pbr()
    .with_albedo(Texture::load("metal/color.jpg"))
    .with_normal(Texture::load("metal/normal.jpg"))
    .with_roughness(Texture::load("metal/roughness.jpg"))
    .with_metalness(Texture::load("metal/metalness.jpg"));

ModelConfig::new("part.glb", ModelFormat::Gltf)
    .with_material(material)
```

### 2.4 Animation Playback
Play skeletal animations from glTF/FBX:

```rust
ThreeView {
    models: vec![
        ModelConfig::new("robot.glb", ModelFormat::Gltf)
            .with_animation("walk", AnimationConfig {
                loop_: true,
                speed: 1.0,
            })
    ],
    on_animation_event: move |event: AnimationEvent| {
        // Animation finished, loop point reached
    },
}
```

---

## Phase 3: Engineering / CAD Toolkit

The **differentiating feature set** for engineering, inspection, and manufacturing workflows.

### 3.1 Measurement Tools

```rust
use dioxus_three::cad::{MeasurementTool, MeasurementResult};

rsx! {
    ThreeView {
        models: models(),
        measurement_tool: Some(MeasurementTool::Distance),
        on_measurement: move |result: MeasurementResult| {
            match result {
                MeasurementResult::Distance { point_a, point_b, length } => {
                    println!("Distance: {:.2} mm", length);
                }
                MeasurementResult::Angle { point_a, vertex, point_b, degrees } => {
                    println!("Angle: {:.1}°", degrees);
                }
                MeasurementResult::Area { points, area } => {
                    println!("Area: {:.2} mm²", area);
                }
            }
        },
    }
}
```

**Implementation:** Raycaster picks two points on mesh surfaces → world-space distance calculation → DOM overlay shows the dimension line and value.

### 3.2 Sectioning / Clipping Planes

```rust
use dioxus_three::cad::ClippingPlane;

let planes = vec![
    ClippingPlane::x(0.0).with_visualizer(true),  // Cut at X=0, show plane outline
    ClippingPlane::y(1.0).with_flip(true),
];

ThreeView {
    models: models(),
    clipping_planes: planes,
}
```

- Draggable section plane handles
- Cross-section hatch pattern fill
- Multiple simultaneous clipping planes

### 3.3 Exploded Views

```rust
use dioxus_three::cad::ExplodedView;

let exploded = ExplodedView::new()
    .with_separation(2.0)        // Distance between parts
    .with_direction(Vector3::Y); // Explode along Y axis

ThreeView {
    scene: scene(),
    exploded_view: Some(exploded),
}
```

Requires scene graph (#1.1) to know which meshes belong to which assembly parts.

### 3.4 Assembly Tree Browser

```rust
// In your Dioxus UI, outside ThreeView
let tree = scene().hierarchy_tree();

rsx! {
    div { class: "sidebar",
        for node in tree.iter() {
            div {
                class: if node.is_selected { "selected" } else { "" },
                onclick: move |_| selection.write().toggle(node.entity_id),
                style: "padding-left: {node.depth * 20}px",
                "{node.name}"
            }
        }
    }
    ThreeView { scene: scene() }
}
```

Parses glTF/FBX scene graph into a Rust tree structure with `EntityId` mapping.

### 3.5 PMI (Product Manufacturing Information) Display

```rust
use dioxus_three::cad::PmiAnnotation;

let annotations = vec![
    PmiAnnotation::dimension("Ø10.00 ±0.05")
        .at(Vector3::new(5.0, 2.0, 0.0))
        .facing_camera(true),
    PmiAnnotation::tolerance("H7")
        .attached_to(entity_id),
];

ThreeView {
    models: models(),
    pmi_annotations: annotations,
}
```

**Implementation:** DOM overlays positioned at 3D world coordinates using CSS 3D transforms (project world → screen space each frame).

### 3.6 Mesh Analysis Visualization

```rust
ThreeView {
    models: models(),
    analysis_mode: Some(AnalysisMode::Wireframe),
    // or AnalysisMode::Normals,
    // or AnalysisMode::Curvature { min: -1.0, max: 1.0 },
    // or AnalysisMode::BoundingBox,
}
```

### 3.7 CSG — Boolean Operations

Constructive Solid Geometry for engineering part operations:

```rust
use dioxus_three::csg::{Operation, CsgMesh};

let result = CsgMesh::from("part_a.glb")
    .boolean(Operation::Subtract, &CsgMesh::from("hole_tool.glb"))
    .boolean(Operation::Union, &CsgMesh::from("attachment.glb"));
```

**Implementation:** `three-bvh-csg` or `three-csg-ts` on the JS side.

### 3.8 Decals

Project textures onto mesh surfaces for annotation marks, damage reporting, or branding:

```rust
ThreeView {
    decals: vec![
        Decal {
            texture: "inspection_mark.png",
            position: hit.point,
            normal: hit.normal,
            size: Vector2::new(0.5, 0.5),
            projection: DecalProjection::ProjectAlongNormal,
        },
    ],
}
```

### 3.9 Scene Comparison / Visual Diff

Compare two versions of a model side-by-side or as an overlay:

```rust
ThreeView {
    diff_mode: Some(DiffMode {
        baseline: "model_v1.glb",
        current: "model_v2.glb",
        added_color: "#00ff00",
        removed_color: "#ff0000",
        changed_color: "#ffff00",
    }),
}
```

**Why it matters:** Essential for CAD review, design approval workflows, and version control for 3D.

### 3.10 2D Technical Drawing / SVG Overlay

Generate dimensioned drawings from 3D with automatic projection:

```rust
ThreeView {
    drawing_mode: Some(DrawingMode::Orthographic(OrthographicView::Front)),
    dimensions: vec![
        Dimension::linear(point_a, point_b).with_tolerance("±0.05"),
        Dimension::diameter(circle_center, radius),
        Dimension::angle(vertex, arm_a, arm_b),
    ],
}
```

### 3.11 Point Clouds & LiDAR

Massive point cloud rendering with LOD for surveying and digital twins:

```rust
ModelConfig::point_cloud("survey.las")
    .with_point_size(2.0)
    .with_color_mode(PointColor::HeightMap)
    .with_lod(vec![
        LodLevel { max_points: 10_000_000, distance: 0.0 },
        LodLevel { max_points: 1_000_000, distance: 100.0 },
        LodLevel { max_points: 100_000, distance: 500.0 },
    ])
```

---

## Phase 4: Advanced Rendering

Photorealism, novel rendering techniques, and visual effects.

### 4.1 Post-Processing Stack

```rust
use dioxus_three::post_process::{Effect, PostProcessStack};

ThreeView {
    models: models(),
    post_process: PostProcessStack::new()
        .with(Effect::bloom(0.5, 0.8))
        .with(Effect::ssao(0.5, 1.0))
        .with(Effect::tone_mapping_aces(1.0)),
}
```

### 4.2 Environment & Atmosphere

```rust
use dioxus_three::environment::{Environment, Skybox, Fog};

ThreeView {
    models: models(),
    environment: Environment {
        skybox: Some(Skybox::hdri("sky_sunset.hdr")),
        fog: Some(Fog::exponential_height("#1a1a2e", 0.02)),
        ambient_occlusion: true,
    },
}
```

### 4.3 Path Tracing (Photorealistic)

`three-gpu-pathtracer` gives ray-traced quality in real-time for product configurators and jewelry:

```rust
ThreeView {
    renderer: RendererMode::PathTraced(PathTraceConfig {
        bounces: 4,
        denoise: true,
        tone_mapping: ToneMapping::AgX,
    }),
}
```

### 4.4 Gaussian Splatting

Photorealistic scenes from photos/scans — the hottest tech in 3D right now:

```rust
ThreeView {
    models: vec![
        ModelConfig::splat("museum.ply"), // .ply or .splat format
    ],
    splat_quality: SplatQuality::High,
}
```

### 4.5 Screenshot & Export

```rust
let png_bytes = dioxus_three::screenshot(ScreenshotOptions {
    width: 1920,
    height: 1080,
    transparent_background: false,
});
```

### 4.6 Instancing & LOD

```rust
ModelConfig::new("tree.glb", ModelFormat::Gltf)
    .with_instances(vec![Instance { position, rotation, scale }; 10000])
    .with_lod(vec![
        LodLevel { model: "tree_high.glb", distance: 0.0 },
        LodLevel { model: "tree_low.glb", distance: 100.0 },
    ])
```

---

## Phase 5: Physics & Simulation

### 5.1 Physics Engine Integration

The physics engine runs **JavaScript-side** (cannon-es or ammo.js) to avoid per-frame transform sync overhead over the Rust↔JS bridge.

**Option A: `cannon-es` — Fast, Modern, Pure JS**
The successor to Cannon.js. Best for rigid bodies, constraints, vehicles, and ragdolls.

```rust
ThreeView {
    models: models(),
    physics: Some(PhysicsConfig {
        engine: PhysicsEngine::CannonEs,
        gravity: Vector3::new(0.0, -9.81, 0.0),
        solver_iterations: 10,
    }),
    models: vec![
        ModelConfig::new("ground.glb", ModelFormat::Gltf)
            .with_physics(PhysicsBody::Static),
        ModelConfig::new("crate.glb", ModelFormat::Gltf)
            .at(0.0, 10.0, 0.0)
            .with_physics(PhysicsBody::Dynamic { mass: 5.0, restitution: 0.3 }),
    ],
}
```

**Option B: `ammo.js` — Bullet Physics, Most Powerful**
The full Bullet physics engine compiled to WASM. Supports soft bodies (cloth, deformable meshes, inflatable objects), concave mesh collision, complex constraints (hinge, slider, 6-DOF), and destruction simulation. This is what Blender uses.

```rust
ModelConfig::new("tire.glb", ModelFormat::Gltf)
    .with_physics(PhysicsBody::SoftBody {
        pressure: 100.0,
        volume_conservation: true,
    })
```

**Option C: Bridge `Rapier` from Rust**
Run Rapier in Rust, sync transforms to Three.js. Clean Rust API but adds sync complexity. Best if physics calculations must drive Rust business logic.

**Recommendation:** Use `ammo.js` as the default. It is the full Bullet Physics engine — the same one used by Blender and many AAA games. It supports everything: rigid bodies, soft bodies, cloth, vehicles, ragdolls, destruction, and concave mesh collision. `cannon-es` can be a lighter fallback for simple rigid-body scenes.

### 5.2 Physics Constraints & Joints

```rust
ModelConfig::new("door.glb", ModelFormat::Gltf)
    .with_physics(PhysicsBody::Dynamic { mass: 10.0 })
    .with_constraint(Constraint::Hinge {
        pivot: Vector3::new(-0.9, 0.0, 0.0),
        axis: Vector3::Y,
        limits: Some((-120.0_f32.to_radians(), 0.0)),
    })
```

### 5.3 Physics-Raycast Integration

Gizmo dragging and object selection should respect physics colliders:
- Click on a physics body → select it
- Drag with gizmo → physics body moves (kinematic control)
- Release → body becomes dynamic again

### 5.4 Vehicles

Full vehicle simulation with suspension, steering, and drivetrain:

```rust
ModelConfig::new("car_chassis.glb", ModelFormat::Gltf)
    .with_physics(PhysicsBody::Dynamic { mass: 800.0 })
    .with_vehicle(VehicleConfig {
        wheel_models: vec!["wheel.glb"; 4],
        wheel_positions: vec![
            Vector3::new(0.8, 0.0, 1.2),   // Front left
            Vector3::new(-0.8, 0.0, 1.2),  // Front right
            Vector3::new(0.8, 0.0, -1.2),  // Rear left
            Vector3::new(-0.8, 0.0, -1.2), // Rear right
        ],
        suspension_stiffness: 20.0,
        suspension_damping: 2.3,
        max_engine_force: 2000.0,
        steering_max: 0.5,
        drive_type: DriveType::AllWheel,
    })
```

### 5.5 Ragdolls

Character ragdoll simulation for impact/death animations:

```rust
ModelConfig::new("character.glb", ModelFormat::Gltf)
    .with_ragdoll(RagdollConfig {
        joints: vec![
            RagdollJoint::spine("spine", "hips", "chest"),
            RagdollJoint::cone("neck", "chest", "head", 45.0, 45.0),
            RagdollJoint::hinge("elbow_l", "upperarm_l", "forearm_l", Vector3::Z, 0.0, 150.0),
            RagdollJoint::hinge("knee_l", "thigh_l", "shin_l", Vector3::Z, 0.0, 150.0),
        ],
        limb_masses: vec![("head", 5.0), ("chest", 20.0), ("upperarm_l", 3.0)],
    })
```

### 5.6 Cloth & Ropes

Soft-body simulation for fabric, cables, and nets:

```rust
// Hanging cloth
ModelConfig::new("flag.glb", ModelFormat::Gltf)
    .with_physics(PhysicsBody::Cloth {
        anchors: vec![0, 1], // Pin top vertices
        wind_force: Vector3::new(5.0, 0.0, 2.0),
        structural_stiffness: 0.8,
        bending_stiffness: 0.5,
    })

// Rope/cable
ModelConfig::new("cable.glb", ModelFormat::Gltf)
    .with_physics(PhysicsBody::Rope {
        start_anchor: Some(crane_hook_entity),
        end_anchor: Some(load_entity),
        segments: 20,
        stiffness: 0.9,
    })
```

### 5.7 Destruction / Fracture

Breakable objects that shatter on impact:

```rust
ModelConfig::new("window.glb", ModelFormat::Gltf)
    .with_physics(PhysicsBody::Breakable {
        fracture_pattern: FracturePattern::Voronoi { cell_count: 50 },
        shard_mass_scale: 0.1,
        activation_impulse: 50.0, // Min impact to break
    })
```

### 5.8 Fluid Simulation

 ammo.js supports SPH (Smoothed Particle Hydrodynamics) for fluids:

```rust
ThreeView {
    fluids: vec![
        FluidConfig {
            particle_count: 5000,
            particle_size: 0.05,
            viscosity: 0.5,
            density: 1000.0, // kg/m³
            emitter: FluidEmitter::box(Vector3::ZERO, Vector3::new(1.0, 2.0, 1.0)),
            color: "#4dabf7",
        },
    ],
}
```

### 5.9 Physics Events & Callbacks

React to collisions and physics state from Rust:

```rust
ThreeView {
    models: models(),
    physics: Some(physics()),
    on_physics_collision: move |event: CollisionEvent| {
        println!("Collision: {:?} hit {:?} with impulse {}",
            event.body_a, event.body_b, event.impulse);

        if event.impulse > 100.0 {
            // Trigger damage, sound, particle effect
            spawn_sparks_at(event.contact_point);
        }
    },
    on_physics_sleep: move |event: SleepEvent| {
        // Body came to rest — optimize by reducing updates
    },
}
```

### 5.10 Animation Timeline & Sequencer

Not just playback — **authoring** keyframe sequences for product demos and assembly instructions:

```rust
let timeline = AnimationTimeline::new()
    .with_track("door", vec![
        Keyframe::at(0.0).rotation(0.0, 0.0, 0.0),
        Keyframe::at(2.0).rotation(0.0, 90.0, 0.0).ease(Easing::EaseOutCubic),
    ])
    .with_track("light", vec![
        Keyframe::at(0.0).intensity(0.0),
        Keyframe::at(1.0).intensity(1.5),
    ]);

ThreeView {
    animation_timeline: Some(timeline),
    on_timeline_event: move |event| { /* scrub, pause, loop */ },
}
```

---

## Phase 6: Platform Extensions & Distribution

### 6.1 BIM / IFC Support

Building Information Modeling support via `web-ifc-three`:

```rust
ModelConfig::ifc("building.ifc")
    .with_ifc_options(IfcOptions {
        show_spaces: true,
        show_structural: true,
        hide_mep: false,
    })
```

Parse IFC metadata into Rust structs for UI panels (room schedules, material takeoffs). Opens the entire AEC industry as a user base.

### 6.2 Collaborative / Multiplayer Cursors

Multiple users viewing the same scene with synced cameras, selections, and cursors:

```rust
ThreeView {
    collaboration: Some(CollabConfig {
        room_id: "project-123",
        user_name: "Esteban",
        user_color: "#4dabf7",
        sync_selection: true,
        sync_camera: true,
    }),
}
```

**Implementation:** `yjs` for CRDT sync, or a simple WebSocket relay. Cursor positions projected from world → screen each frame.

### 6.3 Server-Side Rendering / Thumbnails

Headless rendering for generating thumbnails, previews, and documentation images:

```rust
let thumbnail = dioxus_three::render_headless(RenderRequest {
    model: "product.glb",
    width: 512,
    height: 512,
    camera_position: Vector3::new(5.0, 3.0, 5.0),
    background: "#ffffff",
    output: OutputFormat::Png,
}).await;
```

**Implementation:** Puppeteer + Three.js in headless Chrome, or `three.js` under Node with `node-canvas`/`gl`.

### 6.4 Mobile Parity
- Full gizmo support on Mobile WebView
- Touch-optimized camera controls (pinch-to-zoom, two-finger rotate)
- Test and stabilize the mobile implementation

### 6.5 WebXR (VR/AR)

```rust
ThreeView {
    models: models(),
    xr: Some(XrConfig {
        mode: XrMode::VR, // or AR
        reference_space: ReferenceSpace::LocalFloor,
        controllers: true,
    }),
}
```

**Note:** WebXR is Web-only. Desktop VR would require a completely different architecture.

---

## Out of Scope (Intentionally)

These are architectural patterns that don't fit this library:

| Feature | Why Not |
|---------|---------|
| Full ECS | Overkill; Three.js already has a scene graph. Use Dioxus signals. |
| Scripting system | Use Dioxus signals and Rust code directly |
| AI / pathfinding | Use your own Rust logic outside the component |
| Spatial audio | Out of scope for a 3D viewer |
| Networking / multiplayer game logic | Use Dioxus + your own networking |
| Terrain generation | Use a dedicated engine |
| Custom physics engine | ammo.js (Bullet) already exists and is world-class. Building from scratch would take years and not exceed it. |

---

## Implementation Priority

### Now (v0.0.5)
1. Scene graph / model hierarchy
2. Loading states and error handling
3. Memory management (`dispose()`)

### Next (v0.0.6)
4. Lighting system
5. Advanced camera controllers (orbit damping, FPS, ortho)
6. PBR materials and textures

### Soon (v0.0.7–0.0.8)
7. Animation playback
8. Measurement tools
9. Sectioning / clipping planes
10. CSG (boolean operations)
11. Decals
12. Offline mode

### Physics & Simulation (v0.0.9–0.1.0)
13. **Physics engine integration** — ammo.js (Bullet Physics) with rigid bodies, soft bodies, cloth
14. **Physics constraints & joints** — Hinge, slider, 6-DOF, springs
15. **Vehicles** — Suspension, steering, drivetrain simulation
16. **Ragdolls** — Character joint chains for impact/death physics
17. **Cloth & ropes** — Fabric, cables, nets with anchors and wind
18. **Destruction / fracture** — Voronoi shattering, breakable objects
19. **Fluid simulation** — SPH particles for water, smoke, sand
20. **Physics-raycast integration** — Gizmo dragging respects colliders
21. **Physics events** — Collision callbacks, sleep/wake notifications

### Mid-term (v0.0.9–0.1.0)
22. Exploded views
23. Assembly tree browser
24. PMI display
25. Scene comparison / visual diff
26. Point cloud rendering
27. Technical drawing / SVG overlay

### Later (v0.2.0+)
28. Animation timeline / sequencer
29. Path tracing
30. Gaussian splatting
31. Post-processing stack
32. BIM / IFC support
33. Collaborative cursors
34. Server-side rendering
35. WebXR

---

## Contributing

Priority areas:
- **Physics**: ammo.js (Bullet) bridge — rigid bodies, soft bodies, vehicles, ragdolls, cloth, destruction
- **Web Platform**: State synchronization optimization
- **Engineering Tools**: Measurement, sectioning, exploded views
- **Advanced Rendering**: Path tracing, Gaussian splatting integration
- **Documentation**: Examples for physics, CAD/inspection, and product configurator workflows
- **Testing**: Cross-platform CI for Desktop/Web/Mobile

See [CONTRIBUTING.md](docs/guides/contributing.md) for guidelines.
