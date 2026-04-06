# Camera Control

Control the camera position and target to frame your 3D scene.

## Camera Position

Set the camera location in 3D space:

```rust
rsx! {
    ThreeView {
        cam_x: 10.0,   // Right
        cam_y: 5.0,    // Up
        cam_z: 10.0,   // Back
    }
}
```

## Camera Target

Control where the camera looks:

```rust
rsx! {
    ThreeView {
        cam_x: 5.0,
        cam_y: 5.0,
        cam_z: 5.0,
        target_x: 0.0,  // Look at center
        target_y: 2.0,  // Slightly above origin
        target_z: 0.0,
    }
}
```

## Preset Views

Create preset camera angles:

```rust
fn app() -> Element {
    let mut cam = use_signal(|| (5.0f32, 5.0f32, 5.0f32));
    
    let set_view = move |view: &str| {
        match view {
            "front" => cam.set((0.0, 0.0, 10.0)),
            "side" => cam.set((10.0, 0.0, 0.0)),
            "top" => cam.set((0.0, 10.0, 0.01)),  // Slight offset to avoid gimbal lock
            "iso" => cam.set((5.0, 5.0, 5.0)),
            _ => {}
        }
    };
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 200px; padding: 20px;",
                button { onclick: move |_| set_view("front"), "Front" }
                button { onclick: move |_| set_view("side"), "Side" }
                button { onclick: move |_| set_view("top"), "Top" }
                button { onclick: move |_| set_view("iso"), "Isometric" }
            }
            
            ThreeView {
                cam_x: cam().0,
                cam_y: cam().1,
                cam_z: cam().2,
            }
        }
    }
}
```

## Camera Distance

Control viewing distance with scale:

```rust
fn app() -> Element {
    let mut distance = use_signal(|| 5.0f32);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 200px; padding: 20px;",
                input {
                    r#type: "range",
                    min: "1",
                    max: "20",
                    value: "{distance()}",
                    oninput: move |e| distance.set(e.value().parse().unwrap_or(5.0))
                }
                "Distance: {distance():.1}"
            }
            
            ThreeView {
                cam_x: distance(),
                cam_y: distance(),
                cam_z: distance(),
            }
        }
    }
}
```

## Field of View

The default FOV is 75 degrees. This provides a natural perspective for most models.

## Camera Animations

Animate the camera smoothly:

```rust
fn app() -> Element {
    let mut angle = use_signal(|| 0.0f32);
    let mut radius = use_signal(|| 10.0f32);
    
    // Orbit camera
    let x = radius() * angle().cos();
    let z = radius() * angle().sin();
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            input {
                r#type: "range",
                min: "0",
                max: "6.28",
                step: "0.01",
                value: "{angle()}",
                oninput: move |e| angle.set(e.value().parse().unwrap_or(0.0))
            }
            
            ThreeView {
                cam_x: x,
                cam_y: 5.0,
                cam_z: z,
                target_x: 0.0,
                target_y: 0.0,
                target_z: 0.0,
            }
        }
    }
}
```

## Best Practices

1. **Use auto_center** - Centers models for consistent framing
2. **Default view** - Isometric view (5, 5, 5) works for most models
3. **Avoid gimbal lock** - Don't set cam_y exactly to 0 for top view
4. **Consider model size** - Adjust camera distance based on model bounds
