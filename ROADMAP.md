# Dioxus Three Roadmap

A comprehensive list of features and improvements planned for Dioxus Three.

## Core Architecture Improvements

### 1. **Unified Scene Graph API**
Replace flat model list with hierarchical scene graph:

```rust
use dioxus_three::scene::{Node, Transform, Scene};

fn app() -> Element {
    let scene = Scene::new()
        .with_node(
            Node::new("root")
                .with_transform(Transform::new().translate(0.0, 0.0, 0.0))
                .with_child(
                    Node::new("player")
                        .with_model("player.glb")
                        .with_transform(Transform::new().translate(0.0, 0.0, 0.0))
                        .with_script(PlayerMovementScript)
                )
                .with_child(
                    Node::new("enemies")
                        .with_transform(Transform::new().translate(10.0, 0.0, 10.0))
                        .with_child(
                            Node::new("enemy1").with_model("enemy.glb")
                        )
                        .with_child(
                            Node::new("enemy2").with_model("enemy.glb")
                        )
                );
    
    rsx! {
        ThreeView {
            scene: scene,
        }
    }
}
```

### 2. **Component-Based Entity System**
ECS-like architecture for flexible object composition:

```rust
use dioxus_three::ecs::{Entity, Component, Query};

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Velocity(Vector3);

#[derive(Component)]
struct Rotator { speed: f32 }

// System that runs every frame
fn rotation_system(mut query: Query<(&mut Transform, &Rotator)>, delta_time: f32) {
    for (mut transform, rotator) in query.iter_mut() {
        transform.rotate_y(rotator.speed * delta_time);
    }
}

fn app() -> Element {
    let world = World::new()
        .with_system(rotation_system)
        .with_entity(
            Entity::new()
                .with_model("cube.glb")
                .with(Transform::default())
                .with(Rotator { speed: 45.0 })
        );
    
    rsx! { ThreeView { world: world } }
}
```

---

## Pointer & Selection System

### 3. **Raycasting API**
Full raycasting support for mouse/touch interaction:

```rust
use dioxus_three::input::{Raycast, PointerEvent, HitInfo};

fn app() -> Element {
    let mut selected = use_signal(|| None::<EntityId>);
    let mut hovered = use_signal(|| None::<EntityId>);
    
    // Raycast configuration
    let raycast_config = RaycastConfig {
        enabled: true,
        recursive: true, // Hit children
        layer_mask: LayerMask::all(), // Or specific layers
        max_distance: 1000.0,
    };
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Info panel for selected object
            div { style: "width: 300px; padding: 20px;",
                if let Some(id) = selected() {
                    h3 { "Selected: {id:?}" }
                    TransformControls { entity: id }
                }
            }
            
            ThreeView {
                models: models(),
                raycast: raycast_config,
                
                // Pointer down (click/touch start)
                on_pointer_down: move |event: PointerEvent| {
                    if let Some(hit) = event.hit {
                        println!("Clicked on entity {:?}", hit.entity_id);
                        println!("Intersection point: {:?}", hit.point);
                        println!("Normal: {:?}", hit.normal);
                        println!("UV: {:?}", hit.uv);
                        println!("Distance: {}", hit.distance);
                        
                        selected.set(Some(hit.entity_id));
                    }
                },
                
                // Pointer move (hover)
                on_pointer_move: move |event: PointerEvent| {
                    if let Some(hit) = event.hit {
                        hovered.set(Some(hit.entity_id));
                        
                        // Change cursor style
                        event.set_cursor(CursorStyle::Pointer);
                    } else {
                        hovered.set(None);
                        event.set_cursor(CursorStyle::Default);
                    }
                },
                
                // Pointer up (release)
                on_pointer_up: move |event: PointerEvent| {
                    // Handle drag end, etc.
                },
                
                // Pointer drag
                on_pointer_drag: move |event: PointerDragEvent| {
                    if let Some(hit) = &event.hit {
                        // Move object with mouse
                        let new_pos = event.world_position;
                        world.move_entity(hit.entity_id, new_pos);
                    }
                },
                
                // Multi-touch support
                on_gesture: move |gesture: GestureEvent| {
                    match gesture {
                        GestureEvent::Pinch { scale, center } => {
                            // Zoom camera or scale object
                            camera.zoom(scale);
                        }
                        GestureEvent::Rotate { angle, center } => {
                            // Rotate camera or object
                        }
                        GestureEvent::Pan { delta } => {
                            // Pan camera
                        }
                    }
                },
            }
        }
    }
}
```

### 4. **Selection API**
Manage selection state with visual feedback:

```rust
use dioxus_three::selection::{Selection, SelectionMode};

fn app() -> Element {
    let mut selection = use_signal(|| Selection::new());
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 300px; padding: 20px;",
                button {
                    onclick: move |_| selection.clear(),
                    "Clear Selection"
                }
                
                // List selected objects
                for id in selection.iter() {
                    div { "Entity: {id:?}" }
                }
            }
            
            ThreeView {
                models: models(),
                selection: selection(),
                selection_mode: SelectionMode::Single, // or Multiple, Toggle
                
                // Visual feedback for selection
                selection_style: SelectionStyle {
                    outline: true,
                    outline_color: "#DEC647",
                    outline_width: 2.0,
                    highlight: true,
                    highlight_color: "#DEC647",
                    highlight_opacity: 0.3,
                },
                
                on_selection_change: move |new_selection: Selection| {
                    selection.set(new_selection);
                },
            }
        }
    }
}
```

### 5. **Gizmos & Manipulators**
Visual handles for transforming selected objects:

```rust
use dioxus_three::gizmos::{Gizmo, GizmoMode, GizmoSpace};

fn app() -> Element {
    let mut selected = use_signal(|| None::<EntityId>);
    let mut gizmo_mode = use_signal(|| GizmoMode::Translate);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Toolbar
            div { style: "position: absolute; top: 20px; left: 50%; transform: translateX(-50%); z-index: 10;",
                button { onclick: move |_| gizmo_mode.set(GizmoMode::Translate), "Move" }
                button { onclick: move |_| gizmo_mode.set(GizmoMode::Rotate), "Rotate" }
                button { onclick: move |_| gizmo_mode.set(GizmoMode::Scale), "Scale" }
            }
            
            ThreeView {
                models: models(),
                
                // Transform gizmo
                gizmo: selected().map(|id| Gizmo {
                    target: id,
                    mode: gizmo_mode(),
                    space: GizmoSpace::World, // or Local
                    size: 1.0,
                    show_x: true,
                    show_y: true,
                    show_z: true,
                }),
                
                // While dragging gizmo
                on_gizmo_drag: move |event: GizmoEvent| {
                    match event.mode {
                        GizmoMode::Translate => {
                            world.set_position(event.target, event.transform.position);
                        }
                        GizmoMode::Rotate => {
                            world.set_rotation(event.target, event.transform.rotation);
                        }
                        GizmoMode::Scale => {
                            world.set_scale(event.target, event.transform.scale);
                        }
                    }
                },
                
                on_pointer_down: move |e| selected.set(e.hit.map(|h| h.entity_id)),
            }
        }
    }
}
```

---

## Transformation System

### 6. **Programmatic Transformations**
Apply transformations via Rust API:

```rust
use dioxus_three::transform::{Transform, Tween, Easing};

fn app() -> Element {
    let mut world = use_signal(|| World::new());
    let entity = world.spawn(ModelConfig::new("cube.glb", ModelFormat::Gltf));
    
    // Immediate transform
    let move_to_right = move || {
        world.transform(entity)
            .set_position(5.0, 0.0, 0.0)
            .set_rotation(0.0, 45.0, 0.0)
            .set_scale(2.0);
    };
    
    // Smooth animation with tweening
    let animate_move = move || {
        world.transform(entity).animate(
            Transform::new()
                .translate(10.0, 5.0, 0.0)
                .rotate(0.0, 180.0, 0.0)
                .scale(1.5),
            Tween {
                duration: Duration::from_secs(2),
                easing: Easing::EaseInOutCubic,
                delay: Duration::from_millis(0),
                loop_: false,
                yoyo: false,
                on_complete: Some(Box::new(|| println!("Animation complete!"))),
            },
        );
    };
    
    // Chain animations
    let sequence = move || {
        world.transform(entity)
            .animate_to(Transform::new().translate(5.0, 0.0, 0.0), Duration::from_secs(1))
            .then(Transform::new().translate(5.0, 5.0, 0.0), Duration::from_secs(1))
            .then(Transform::new().translate(0.0, 5.0, 0.0), Duration::from_secs(1))
            .then(Transform::new().translate(0.0, 0.0, 0.0), Duration::from_secs(1));
    };
    
    // Physics-based animation (spring)
    let spring_animation = move || {
        world.transform(entity).animate_spring(
            Transform::new().translate(0.0, 10.0, 0.0),
            Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
            },
        );
    };
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 300px; padding: 20px;",
                button { onclick: move |_| move_to_right(), "Teleport Right" }
                button { onclick: move |_| animate_move(), "Animate Move" }
                button { onclick: move |_| sequence(), "Play Sequence" }
                button { onclick: move |_| spring_animation(), "Spring Jump" }
                
                // Stop all animations
                button { onclick: move |_| world.transform(entity).stop_animations(), "Stop" }
                button { onclick: move |_| world.transform(entity).pause_animations(), "Pause" }
                button { onclick: move |_| world.transform(entity).resume_animations(), "Resume" }
            }
            
            ThreeView { world: world() }
        }
    }
}
```

### 7. **Transform Constraints**
Limit transformations for gameplay or UI:

```rust
use dioxus_three::constraints::{Constraint, Axis};

fn app() -> Element {
    let entity = world.spawn(ModelConfig::new("slider.glb", ModelFormat::Gltf));
    
    // Constrain to X axis only
    world.add_constraint(entity, Constraint::TranslateAxis(Axis::X));
    
    // Constrain to a plane
    world.add_constraint(entity, Constraint::TranslatePlane {
        normal: Vector3::UP,
        height: 0.0,
    });
    
    // Constrain to a volume
    world.add_constraint(entity, Constraint::BoundingBox {
        min: (-10.0, 0.0, -10.0),
        max: (10.0, 5.0, 10.0),
    });
    
    // Snap to grid
    world.add_constraint(entity, Constraint::Snap {
        translate: 1.0,  // Snap to 1-unit grid
        rotate: 45.0,    // Snap to 45-degree increments
        scale: 0.5,
    });
    
    // Constrain rotation
    world.add_constraint(entity, Constraint::RotateAxis(Axis::Y));
    
    // Limit scale
    world.add_constraint(entity, Constraint::ScaleRange {
        min: 0.5,
        max: 3.0,
    });
    
    rsx! { ThreeView { world: world } }
}
```

---

## Scripting System

### 8. **Rust-Based Scripts**
Attach behavior scripts to entities:

```rust
use dioxus_three::script::{Script, ScriptContext, ScriptState};
use async_trait::async_trait;

// Define a custom script
#[derive(Clone)]
struct OrbitalMotion {
    center: Vector3,
    radius: f32,
    speed: f32,
    axis: Vector3,
}

#[async_trait]
impl Script for OrbitalMotion {
    fn on_start(&mut self, ctx: &mut ScriptContext) {
        println!("Orbital motion started for {:?}", ctx.entity);
    }
    
    fn update(&mut self, ctx: &mut ScriptContext, delta_time: f32) {
        let time = ctx.time.elapsed_secs();
        let angle = time * self.speed;
        
        let new_pos = self.center + Vector3::new(
            angle.cos() * self.radius,
            0.0,
            angle.sin() * self.radius,
        );
        
        ctx.transform.set_position(new_pos);
        ctx.transform.look_at(self.center, Vector3::UP);
    }
    
    fn on_stop(&mut self, ctx: &mut ScriptContext) {
        println!("Orbital motion stopped");
    }
}

// Physics-based script
#[derive(Clone)]
struct BouncingBall {
    velocity: Vector3,
    gravity: f32,
    bounce_factor: f32,
    floor_y: f32,
}

#[async_trait]
impl Script for BouncingBall {
    fn update(&mut self, ctx: &mut ScriptContext, delta_time: f32) {
        // Apply gravity
        self.velocity.y -= self.gravity * delta_time;
        
        // Update position
        let pos = ctx.transform.position();
        let new_pos = pos + self.velocity * delta_time;
        
        // Floor collision
        if new_pos.y < self.floor_y {
            new_pos.y = self.floor_y;
            self.velocity.y = -self.velocity.y * self.bounce_factor;
        }
        
        ctx.transform.set_position(new_pos);
    }
}

// State machine script
#[derive(Clone)]
enum EnemyState {
    Idle,
    Patrol(Vector3, Vector3),
    Chase(EntityId),
    Attack,
}

#[derive(Clone)]
struct EnemyAI {
    state: EnemyState,
    detection_range: f32,
    attack_range: f32,
    move_speed: f32,
}

#[async_trait]
impl Script for EnemyAI {
    fn update(&mut self, ctx: &mut ScriptContext, delta_time: f32) {
        match &self.state {
            EnemyState::Idle => {
                // Look for player
                if let Some(player) = ctx.world.find_entity("player") {
                    let distance = ctx.transform.distance_to(player);
                    if distance < self.detection_range {
                        self.state = EnemyState::Chase(player);
                    }
                }
            }
            EnemyState::Patrol(start, end) => {
                // Move between patrol points
                let pos = ctx.transform.position();
                let target = if (pos - *start).length() < 0.1 { *end } else { *start };
                ctx.transform.move_towards(target, self.move_speed * delta_time);
            }
            EnemyState::Chase(target_id) => {
                if let Some(target) = ctx.world.get_entity(*target_id) {
                    let distance = ctx.transform.distance_to(target);
                    
                    if distance > self.detection_range * 1.5 {
                        self.state = EnemyState::Idle;
                    } else if distance < self.attack_range {
                        self.state = EnemyState::Attack;
                    } else {
                        // Move towards target
                        let target_pos = target.transform.position();
                        ctx.transform.move_towards(target_pos, self.move_speed * delta_time);
                        ctx.transform.look_at(target_pos, Vector3::UP);
                    }
                }
            }
            EnemyState::Attack => {
                // Attack logic
                println!("Attacking player!");
                
                // Return to chase after attack
                if let Some(player) = ctx.world.find_entity("player") {
                    self.state = EnemyState::Chase(player);
                }
            }
        }
    }
}

fn app() -> Element {
    let mut world = use_signal(|| World::new());
    
    // Planet orbiting sun
    let planet = world.spawn(ModelConfig::new("planet.glb", ModelFormat::Gltf));
    world.add_script(planet, OrbitalMotion {
        center: Vector3::ZERO,
        radius: 10.0,
        speed: 0.5,
        axis: Vector3::UP,
    });
    
    // Bouncing ball
    let ball = world.spawn(ModelConfig::new("ball.glb", ModelFormat::Gltf));
    world.add_script(ball, BouncingBall {
        velocity: Vector3::new(0.0, 10.0, 0.0),
        gravity: 9.81,
        bounce_factor: 0.8,
        floor_y: 0.0,
    });
    
    // Enemy with AI
    let enemy = world.spawn(ModelConfig::new("enemy.glb", ModelFormat::Gltf)
        .with_position(5.0, 0.0, 5.0));
    world.add_script(enemy, EnemyAI {
        state: EnemyState::Patrol(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(10.0, 0.0, 10.0)
        ),
        detection_range: 5.0,
        attack_range: 1.5,
        move_speed: 2.0,
    });
    
    rsx! { ThreeView { world: world() } }
}
```

### 9. **Script Lifecycle & Events**
Handle events within scripts:

```rust
use dioxus_three::script::{Script, CollisionEvent, InputEvent};

#[derive(Clone)]
struct PlayerController {
    speed: f32,
    jump_force: f32,
    is_grounded: bool,
}

#[async_trait]
impl Script for PlayerController {
    fn on_start(&mut self, ctx: &mut ScriptContext) {
        ctx.input.set_cursor_lock(true); // Lock cursor for FPS
    }
    
    fn update(&mut self, ctx: &mut ScriptContext, delta_time: f32) {
        // Keyboard input
        let mut move_dir = Vector3::ZERO;
        
        if ctx.input.key_pressed(KeyCode::W) {
            move_dir += ctx.transform.forward();
        }
        if ctx.input.key_pressed(KeyCode::S) {
            move_dir -= ctx.transform.forward();
        }
        if ctx.input.key_pressed(KeyCode::A) {
            move_dir -= ctx.transform.right();
        }
        if ctx.input.key_pressed(KeyCode::D) {
            move_dir += ctx.transform.right();
        }
        
        // Apply movement
        if move_dir != Vector3::ZERO {
            move_dir = move_dir.normalize();
            ctx.transform.translate(move_dir * self.speed * delta_time);
        }
        
        // Mouse look
        let mouse_delta = ctx.input.mouse_delta();
        ctx.transform.rotate_y(mouse_delta.x * 0.1);
        ctx.camera.pitch(mouse_delta.y * 0.1);
        
        // Jump
        if self.is_grounded && ctx.input.key_just_pressed(KeyCode::Space) {
            ctx.physics.add_impulse(Vector3::UP * self.jump_force);
            self.is_grounded = false;
        }
    }
    
    fn on_collision(&mut self, ctx: &mut ScriptContext, event: CollisionEvent) {
        if event.normal.y > 0.5 {
            self.is_grounded = true;
        }
    }
    
    fn on_pointer_enter(&mut self, ctx: &mut ScriptContext) {
        ctx.renderer.set_highlight(true);
    }
    
    fn on_pointer_exit(&mut self, ctx: &mut ScriptContext) {
        ctx.renderer.set_highlight(false);
    }
    
    fn on_click(&mut self, ctx: &mut ScriptContext, button: MouseButton) {
        match button {
            MouseButton::Left => {
                // Shoot
                ctx.world.spawn_bullet(
                    ctx.transform.position(),
                    ctx.transform.forward()
                );
            }
            MouseButton::Right => {
                // Aim
            }
        }
    }
}
```

### 10. **Coroutine Scripts**
Async/await for complex sequences:

```rust
use dioxus_three::script::coroutine;

#[derive(Clone)]
struct CutsceneScript;

#[async_trait]
impl Script for CutsceneScript {
    async fn run(&mut self, ctx: &mut ScriptContext) {
        // Move camera to start position
        ctx.camera.animate_to(
            CameraTransform::new()
                .position(0.0, 5.0, -10.0)
                .look_at(0.0, 0.0, 0.0),
            Duration::from_secs(2)
        ).await;
        
        // Spawn character
        let hero = ctx.world.spawn(ModelConfig::new("hero.glb", ModelFormat::Gltf));
        
        // Hero walks in
        ctx.transform(hero).animate_to(
            Transform::new().translate(0.0, 0.0, 5.0),
            Duration::from_secs(3)
        ).await;
        
        // Wait a bit
        coroutine::sleep(Duration::from_secs(1)).await;
        
        // Camera zoom in
        ctx.camera.animate_to(
            CameraTransform::new()
                .position(0.0, 2.0, 2.0)
                .look_at(0.0, 1.5, 0.0),
            Duration::from_secs(1)
        ).await;
        
        // Show dialogue
        ctx.ui.show_dialogue("hero", "Hello, world!");
        
        // Wait for user to continue
        ctx.input.wait_for_key(KeyCode::Space).await;
        
        // Continue cutscene...
        ctx.ui.hide_dialogue();
    }
}
```

---

## Lighting System

### 11. **Full Lighting Control**

```rust
use dioxus_three::lighting::{Light, LightType, ShadowConfig};

fn app() -> Element {
    let lights = vec![
        // Ambient
        Light::ambient("ambient", "#404040", 0.5),
        
        // Directional (sun)
        Light::directional("sun", "#ffffff", 1.0)
            .with_position(10.0, 20.0, 10.0)
            .with_target(0.0, 0.0, 0.0)
            .with_shadows(ShadowConfig {
                enabled: true,
                map_size: 2048,
                bias: -0.0001,
                normal_bias: 0.02,
                radius: 2.0,
            }),
        
        // Point (torch)
        Light::point("torch", "#ffaa00", 1.5)
            .with_position(2.0, 3.0, 2.0)
            .with_distance(20.0)
            .with_decay(2.0),
        
        // Spot (flashlight)
        Light::spot("flashlight", "#ffffff", 2.0)
            .with_position(0.0, 1.7, 0.0)
            .with_target(0.0, 0.0, -5.0)
            .with_angle(45.0)
            .with_penumbra(0.3)
            .with_distance(30.0),
        
        // Hemisphere (sky/ground)
        Light::hemisphere("sky", "#87CEEB", "#362d1d", 0.6),
        
        // RectArea (studio light)
        Light::rect_area("studio", "#ffffff", 2.0)
            .with_position(5.0, 5.0, 5.0)
            .with_width(2.0)
            .with_height(2.0)
            .with_look_at(0.0, 0.0, 0.0),
    ];
    
    // Dynamic light control
    let mut torch_on = use_signal(|| true);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            button {
                onclick: move |_| {
                    torch_on.set(!torch_on());
                    lights[2].set_intensity(if torch_on() { 1.5 } else { 0.0 });
                },
                if torch_on() { "Turn Torch Off" } else { "Turn Torch On" }
            }
            
            ThreeView {
                models: models(),
                lights: lights,
            }
        }
    }
}
```

---

## Camera System

### 12. **Advanced Camera Controllers**

```rust
use dioxus_three::camera::{CameraController, OrbitControls, FPSControls};

fn app() -> Element {
    let mut mode = use_signal(|| CameraMode::Orbit);
    
    let camera = match mode() {
        CameraMode::Orbit => CameraController::Orbit(OrbitControls {
            target: Vector3::ZERO,
            distance: 10.0,
            min_distance: 1.0,
            max_distance: 100.0,
            azimuth: 45.0, // Horizontal angle
            polar: 60.0,   // Vertical angle
            min_polar: 5.0,
            max_polar: 175.0,
            enable_damping: true,
            damping_factor: 0.05,
            enable_zoom: true,
            zoom_speed: 1.0,
            enable_rotate: true,
            rotate_speed: 1.0,
            enable_pan: true,
            pan_speed: 1.0,
            screen_space_panning: true,
            key_pan_speed: 7.0,
            auto_rotate: false,
            auto_rotate_speed: 2.0,
            enable_keys: true,
            keys: KeyMappings {
                left: KeyCode::ArrowLeft,
                right: KeyCode::ArrowRight,
                up: KeyCode::ArrowUp,
                down: KeyCode::ArrowDown,
            },
        }),
        
        CameraMode::FPS => CameraController::FPS(FPSControls {
            position: Vector3::new(0.0, 1.7, 5.0),
            yaw: 0.0,
            pitch: 0.0,
            movement_speed: 5.0,
            sprint_multiplier: 2.0,
            look_speed: 0.1,
            enable_joystick: true,
            joystick_deadzone: 0.1,
            constrain_pitch: true,
            min_pitch: -89.0,
            max_pitch: 89.0,
        }),
        
        CameraMode::Follow => CameraController::Follow(FollowControls {
            target: player_entity,
            offset: Vector3::new(0.0, 3.0, -5.0),
            smoothness: 0.1,
            look_at_offset: Vector3::new(0.0, 1.0, 0.0),
        }),
        
        CameraMode::Cinematic => CameraController::Cinematic(CinematicControls {
            waypoints: vec![
                CameraWaypoint {
                    position: Vector3::new(0.0, 5.0, 10.0),
                    look_at: Vector3::new(0.0, 0.0, 0.0),
                    duration: Duration::from_secs(5),
                    easing: Easing::EaseInOutQuad,
                },
                CameraWaypoint {
                    position: Vector3::new(10.0, 2.0, 0.0),
                    look_at: Vector3::new(0.0, 0.0, 0.0),
                    duration: Duration::from_secs(3),
                    easing: Easing::EaseOutCubic,
                },
            ],
            loop_: true,
        }),
    };
    
    rsx! {
        ThreeView {
            models: models(),
            camera_controller: camera,
        }
    }
}
```

---

## Texture & Material System

### 13. **PBR Materials**

```rust
use dioxus_three::material::{Material, PbrMaterial, Texture};

fn app() -> Element {
    let rusted_metal = PbrMaterial {
        albedo: Texture::load("rusted_metal/color.jpg"),
        normal: Some(Texture::load("rusted_metal/normal.jpg")),
        roughness: Some(Texture::load("rusted_metal/roughness.jpg")),
        metalness: Some(Texture::load("rusted_metal/metalness.jpg")),
        ao: Some(Texture::load("rusted_metal/ao.jpg")),
        emissive: Some(Texture::load("rusted_metal/emissive.jpg")),
        emissive_intensity: 1.0,
    };
    
    let glass = PbrMaterial {
        albedo: Texture::color("#ffffff"),
        roughness: Some(Texture::value(0.0)),
        metalness: Some(Texture::value(0.0)),
        transmission: Some(1.0), // Glass-like
        ior: 1.5,
        thickness: 0.1,
        attenuation_color: "#ffffff",
        attenuation_distance: 1.0,
    };
    
    let animated_water = PbrMaterial {
        albedo: Texture::load("water/albedo.jpg"),
        normal: Some(Texture::animated("water/normal_{frame}.jpg", 30)), // 30 FPS
        roughness: Some(Texture::value(0.1)),
        ..Default::default()
    };
    
    rsx! {
        ThreeView {
            models: vec![
                ModelConfig::new("barrel.glb", ModelFormat::Gltf)
                    .with_material(rusted_metal),
                ModelConfig::new("window.glb", ModelFormat::Gltf)
                    .with_material(glass),
                ModelConfig::new("water_plane.glb", ModelFormat::Gltf)
                    .with_material(animated_water),
            ],
        }
    }
}
```

---

## Post-Processing

### 14. **Post-Processing Stack**

```rust
use dioxus_three::post_processing::{Effect, PostProcessingStack};

fn app() -> Element {
    let post_process = PostProcessingStack {
        effects: vec![
            Effect::Bloom {
                intensity: 0.5,
                threshold: 0.8,
                radius: 0.5,
            },
            Effect::SSAO {
                radius: 0.5,
                intensity: 1.0,
                bias: 0.025,
            },
            Effect::DepthOfField {
                focus_distance: 10.0,
                aperture: 0.5,
                max_blur: 1.0,
            },
            Effect::ChromaticAberration {
                intensity: 0.05,
            },
            Effect::Vignette {
                intensity: 0.5,
                smoothness: 0.5,
                color: "#000000",
            },
            Effect::ColorGrading {
                saturation: 1.1,
                contrast: 1.05,
                brightness: 0.05,
                lut: Some("color_grading_lut.cube"),
            },
            Effect::ToneMapping {
                mode: ToneMappingMode::ACES,
                exposure: 1.0,
            },
        ],
    };
    
    rsx! {
        ThreeView {
            models: models(),
            post_processing: post_process,
        }
    }
}
```

---

## Physics Integration

### 15. **Physics World (Optional Feature)**

```rust
use dioxus_three::physics::{PhysicsWorld, RigidBody, Collider, PhysicsMaterial};

#[cfg(feature = "physics")]
fn app() -> Element {
    let physics = PhysicsWorld::new()
        .with_gravity(Vector3::new(0.0, -9.81, 0.0));
    
    let mut world = World::new()
        .with_physics(physics);
    
    // Static ground
    let ground = world.spawn(
        ModelConfig::new("ground.glb", ModelFormat::Gltf)
            .with_rigid_body(RigidBody::Static)
            .with_collider(Collider::Plane { normal: Vector3::UP })
    );
    
    // Dynamic boxes
    for i in 0..10 {
        let box_entity = world.spawn(
            ModelConfig::new("crate.glb", ModelFormat::Gltf)
                .with_position(i as f32 * 2.0, 10.0, 0.0)
                .with_rigid_body(RigidBody::Dynamic { 
                    mass: 10.0,
                    linear_damping: 0.1,
                    angular_damping: 0.1,
                })
                .with_collider(Collider::Box {
                    width: 1.0,
                    height: 1.0,
                    depth: 1.0,
                }.with_material(PhysicsMaterial {
                    friction: 0.5,
                    restitution: 0.3,
                }))
        );
    }
    
    // Character controller
    let player = world.spawn(
        ModelConfig::new("player.glb", ModelFormat::Gltf)
            .with_rigid_body(RigidBody::Kinematic)
            .with_collider(Collider::Capsule {
                radius: 0.5,
                height: 1.7,
            })
    );
    
    // Add force via script
    world.add_script(player, PlayerPhysicsController);
    
    rsx! {
        ThreeView {
            world: world,
            physics_debug: true, // Show collision shapes
        }
    }
}
```

---

## Environment & Atmosphere

### 16. **Environment System**

```rust
use dioxus_three::environment::{Environment, Fog, Skybox};

fn app() -> Element {
    let env = Environment {
        skybox: Some(Skybox::HDRI {
            path: "sky_sunset.hdr",
            rotation: 0.0,
        }),
        // Or procedural sky
        // skybox: Some(Skybox::Procedural(ProceduralSky {
        //     turbidity: 10.0,
        //     rayleigh: 3.0,
        //     mie_coefficient: 0.005,
        //     mie_directional_g: 0.7,
        //     elevation: 2.0,
        //     azimuth: 180.0,
        // })),
        
        fog: Some(Fog::ExponentialHeight {
            color: "#1a1a2e",
            density: 0.02,
            height_falloff: 0.05,
            start_height: 0.0,
        }),
        
        ambient_light: AmbientLight {
            color: "#ffffff",
            intensity: 0.3,
        },
        
        reflection_probe: Some(ReflectionProbe::Realtime {
            resolution: 256,
            update_frequency: UpdateFrequency::EveryFrame,
        }),
    };
    
    rsx! {
        ThreeView {
            models: models(),
            environment: env,
        }
    }
}
```

---

## Audio

### 17. **Spatial Audio**

```rust
use dioxus_three::audio::{AudioSource, AudioListener, AudioConfig};

fn app() -> Element {
    let audio_sources = vec![
        AudioSource {
            name: "ambient",
            url: "forest_ambient.mp3",
            position: None, // Global
            volume: 0.5,
            loop_: true,
            spatial: false,
            autoplay: true,
        },
        AudioSource {
            name: "waterfall",
            url: "waterfall.mp3",
            position: Some((10.0, 0.0, -5.0)),
            volume: 1.0,
            loop_: true,
            spatial: true,
            ref_distance: 5.0,
            max_distance: 50.0,
            rolloff_factor: 1.0,
        },
        AudioSource {
            name: "footsteps",
            url: "footstep_{n}.mp3", // Random from set
            position: Some((0.0, 0.0, 0.0)),
            volume: 0.8,
            loop_: false,
            spatial: true,
            autoplay: false,
        },
    ];
    
    // Trigger sound from script
    let play_footstep = move || {
        world.trigger_audio("footsteps");
    };
    
    rsx! {
        ThreeView {
            models: models(),
            audio_sources: audio_sources,
            audio_listener: AudioListener {
                parent: Some(player_entity), // Attach to player
                offset: Vector3::new(0.0, 1.7, 0.0), // Head height
            },
            audio_config: AudioConfig {
                global_volume: 1.0,
                max_channels: 32,
            },
        }
    }
}
```

---

## Performance & Optimization

### 18. **Instancing & LOD**

```rust
use dioxus_three::optimization::{Instancing, LodGroup, OcclusionCulling};

fn app() -> Element {
    // Instancing - render 10000 trees efficiently
    let tree_instances = (0..10000).map(|i| {
        Instance {
            position: random_position(),
            rotation: random_rotation(),
            scale: random_scale(0.8, 1.2),
            color: None, // Or per-instance color
        }
    }).collect();
    
    // LOD - different models at different distances
    let lod_tree = LodGroup {
        levels: vec![
            LodLevel { model: "tree_high.glb", distance: 0.0 },
            LodLevel { model: "tree_medium.glb", distance: 50.0 },
            LodLevel { model: "tree_low.glb", distance: 150.0 },
            LodLevel { model: "", distance: 300.0 }, // Culled
        ],
    };
    
    rsx! {
        ThreeView {
            models: vec![
                ModelConfig::new("tree_high.glb", ModelFormat::Gltf)
                    .with_instances(tree_instances)
                    .with_lod(lod_tree),
            ],
            occlusion_culling: OcclusionCulling::Enabled {
                resolution: 512,
            },
            frustum_culling: true,
        }
    }
}
```

---

## XR/VR Support

### 19. **WebXR Integration**

```rust
use dioxus_three::xr::{XRSession, XRMode, XRConfig};

fn app() -> Element {
    let mut xr_mode = use_signal(|| None::<XRMode>);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "position: absolute; top: 20px; left: 20px; z-index: 10;",
                button {
                    onclick: move |_| xr_mode.set(Some(XRMode::VR)),
                    "Enter VR"
                }
                button {
                    onclick: move |_| xr_mode.set(Some(XRMode::AR)),
                    "Enter AR"
                }
            }
            
            ThreeView {
                models: models(),
                xr: xr_mode().map(|mode| XRConfig {
                    mode: mode,
                    reference_space: ReferenceSpace::LocalFloor,
                    controllers: true,
                    hand_tracking: true,
                    hit_test: mode == XRMode::AR,
                    dom_overlay: Some("xr-ui".to_string()),
                    on_session_start: move |session: XRSession| {
                        println!("XR Session started");
                    },
                    on_controller_connected: move |controller: XRController| {
                        println!("Controller connected: {:?}", controller.handedness);
                    },
                    on_select: move |event: XRSelectEvent| {
                        // Handle controller select button
                        if let Some(hit) = event.hit_test {
                            spawn_object_at(hit.position);
                        }
                    },
                }),
            }
        }
    }
}
```

---

## UI Integration

### 20. **3D UI Elements**

```rust
use dioxus_three::ui::{WorldUI, Billboard, CanvasUI};

fn app() -> Element {
    rsx! {
        ThreeView {
            models: models(),
            world_ui: vec![
                // 3D label that always faces camera
                WorldUI::Billboard(Billboard {
                    position: (0.0, 2.0, 0.0),
                    content: rsx! {
                        div { style: "background: rgba(0,0,0,0.7); color: white; padding: 10px;",
                            h3 { "Treasure Chest" }
                            p { "Click to open" }
                        }
                    },
                    scale: 0.01, // meters per pixel
                }),
                
                // UI fixed to entity
                WorldUI::Attached {
                    entity: player_entity,
                    offset: (0.0, 2.5, 0.0),
                    content: rsx! {
                        div { style: "background: red; color: white;",
                            "Health: {player_health}%"
                        }
                    },
                },
                
                // Render Dioxus UI on 3D canvas
                WorldUI::Canvas(CanvasUI {
                    position: (5.0, 2.0, 0.0),
                    rotation: (0.0, -90.0, 0.0),
                    width: 1024,
                    height: 768,
                    content: rsx! {
                        div { style: "width: 100%; height: 100%; background: white;",
                            h1 { "Interactive Screen" }
                            button { onclick: move |_| {}, "Click me!" }
                            input { placeholder: "Type here..." }
                        }
                    },
                }),
            ],
        }
    }
}
```

---

## Utility Features

### 21. **Screenshot & Recording**

```rust
use dioxus_three::capture::{Screenshot, Recorder, VideoFormat};

fn app() -> Element {
    let view_id = "main-view";
    
    let take_screenshot = move || {
        let png_data = dioxus_three::screenshot(view_id, ScreenshotOptions {
            width: 1920,
            height: 1080,
            format: ImageFormat::PNG,
        });
        
        // Save or upload
        save_to_disk(png_data, "screenshot.png");
    };
    
    let start_recording = move || {
        let handle = dioxus_three::start_recording(view_id, RecordingOptions {
            fps: 60,
            width: 1920,
            height: 1080,
            format: VideoFormat::WebM,
            bitrate: 5000000,
        });
        
        // Stop after 10 seconds
        spawn(async move {
            sleep(Duration::from_secs(10)).await;
            let video_data = dioxus_three::stop_recording(handle);
            save_to_disk(video_data, "recording.webm");
        });
    };
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "position: absolute; top: 20px; right: 20px; z-index: 10;",
                button { onclick: move |_| take_screenshot(), "📷 Screenshot" }
                button { onclick: move |_| start_recording(), "🎥 Record 10s" }
            }
            
            ThreeView {
                id: view_id,
                models: models(),
            }
        }
    }
}
```

### 22. **Performance Profiler**

```rust
use dioxus_three::debug::{DebugOverlay, PerformanceStats};

fn app() -> Element {
    let mut show_debug = use_signal(|| false);
    
    rsx! {
        ThreeView {
            models: models(),
            debug: DebugOverlay {
                enabled: show_debug(),
                show_fps: true,
                show_draw_calls: true,
                show_triangles: true,
                show_memory: true,
                show_bounding_boxes: true,
                show_normals: false,
            },
            on_stats: move |stats: PerformanceStats| {
                if stats.fps < 30.0 {
                    println!("Warning: Low FPS! {} fps", stats.fps);
                }
            },
        }
    }
}
```

---

## Implementation Priority

### Phase 1: Core Stability
1. ✅ Scene Properties Management (DONE)
2. ✅ Web Platform Camera Fix (DONE)
3. 🔲 Pointer/Raycasting System
4. 🔲 Selection API
5. 🔲 Gizmos & Transform Manipulators

### Phase 2: Essential Features
6. 🔲 Lighting System
7. 🔲 Advanced Camera Controllers
8. 🔲 PBR Materials & Textures
9. 🔲 Animation System
10. 🔲 Scripting System

### Phase 3: Advanced Features
11. 🔲 Physics Integration
12. 🔲 Post-Processing
13. 🔲 Environment & Fog
14. 🔲 Particle Systems
15. 🔲 Audio Spatialization

### Phase 4: Platform Extensions
16. 🔲 XR/VR Support
17. 🔲 Instancing & LOD
18. 🔲 World UI
19. 🔲 Screenshot/Recording
20. 🔲 Performance Tools

---

## Contributing

Contributions are welcome! Priority areas:

- **Web Platform**: Optimizing state synchronization
- **Documentation**: Examples and tutorials
- **Testing**: Cross-platform testing infrastructure
- **Performance**: Benchmarking and optimization

See [CONTRIBUTING.md](docs/guides/contributing.md) for guidelines.
