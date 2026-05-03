# Managing Scene Properties

Learn how to dynamically control scene properties using Dioxus signals for reactive, real-time updates.

## Overview

Dioxus Three integrates seamlessly with Dioxus signals to provide reactive 3D scenes. When you update a signal, the scene updates automatically.

```rust
use dioxus::prelude::*;
use dioxus_three::ThreeView;

fn app() -> Element {
    // Create signals for properties you want to control
    let mut cam_x = use_signal(|| 5.0f32);
    let mut auto_rotate = use_signal(|| true);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Control panel
            div { style: "width: 250px; padding: 20px;",
                input {
                    r#type: "range",
                    min: "-30",
                    max: "30",
                    value: "{cam_x()}",
                    oninput: move |e| cam_x.set(e.value().parse().unwrap_or(5.0))
                }
                button {
                    onclick: move |_| auto_rotate.set(!auto_rotate()),
                    if auto_rotate() { "Stop Rotation" } else { "Start Rotation" }
                }
            }
            
            // 3D View - automatically updates when signals change
            ThreeView {
                cam_x: cam_x(),
                auto_rotate: auto_rotate(),
            }
        }
    }
}
```

## Signal-Based Property Management

### Why Use Signals?

| Approach | Pros | Cons |
|----------|------|------|
| **Static Props** | Simple, good for static scenes | No runtime changes |
| **Signals** | Reactive, real-time updates | Slightly more code |
| **State Structs** | Organized, grouped properties | More complex |

### Creating Property Signals

Create signals for each property you want to control:

```rust
fn app() -> Element {
    // Camera signals
    let mut cam_x = use_signal(|| 8.0f32);
    let mut cam_y = use_signal(|| 8.0f32);
    let mut cam_z = use_signal(|| 8.0f32);
    
    // Transform signals
    let mut pos_x = use_signal(|| 0.0f32);
    let mut rot_y = use_signal(|| 0.0f32);
    let mut scale = use_signal(|| 1.0f32);
    
    // Scene signals
    let mut show_grid = use_signal(|| true);
    let mut show_axes = use_signal(|| true);
    let mut wireframe = use_signal(|| false);
    
    // Animation signals
    let mut auto_rotate = use_signal(|| true);
    let mut rot_speed = use_signal(|| 1.0f32);
    
    rsx! {
        ThreeView {
            cam_x: cam_x(),
            cam_y: cam_y(),
            cam_z: cam_z(),
            pos_x: pos_x(),
            rot_y: rot_y(),
            scale: scale(),
            show_grid: show_grid(),
            show_axes: show_axes(),
            wireframe: wireframe(),
            auto_rotate: auto_rotate(),
            rot_speed: rot_speed(),
        }
    }
}
```

## Camera Property Management

### Camera Position with Sliders

```rust
fn app() -> Element {
    let mut cam_x = use_signal(|| 8.0f32);
    let mut cam_y = use_signal(|| 8.0f32);
    let mut cam_z = use_signal(|| 8.0f32);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 300px; padding: 20px; background: #1a1a2e; color: white;",
                h3 { "Camera Control" }
                
                // X Slider
                div { style: "margin: 10px 0;",
                    label { "X: {cam_x():.1}" }
                    input {
                        r#type: "range",
                        min: "-30",
                        max: "30",
                        step: "0.1",
                        value: "{cam_x()}",
                        oninput: move |e| cam_x.set(e.value().parse().unwrap_or(8.0))
                    }
                }
                
                // Y Slider
                div { style: "margin: 10px 0;",
                    label { "Y: {cam_y():.1}" }
                    input {
                        r#type: "range",
                        min: "-30",
                        max: "30",
                        step: "0.1",
                        value: "{cam_y()}",
                        oninput: move |e| cam_y.set(e.value().parse().unwrap_or(8.0))
                    }
                }
                
                // Z Slider
                div { style: "margin: 10px 0;",
                    label { "Z: {cam_z():.1}" }
                    input {
                        r#type: "range",
                        min: "-30",
                        max: "30",
                        step: "0.1",
                        value: "{cam_z()}",
                        oninput: move |e| cam_z.set(e.value().parse().unwrap_or(8.0))
                    }
                }
                
                // Preset buttons
                div { style: "margin-top: 20px;",
                    button {
                        onclick: move |_| { cam_x.set(0.0); cam_y.set(15.0); cam_z.set(0.01); },
                        "Top View"
                    }
                    button {
                        onclick: move |_| { cam_x.set(15.0); cam_y.set(0.0); cam_z.set(0.0); },
                        "Side View"
                    }
                    button {
                        onclick: move |_| { cam_x.set(10.0); cam_y.set(10.0); cam_z.set(10.0); },
                        "Isometric"
                    }
                }
            }
            
            ThreeView {
                cam_x: cam_x(),
                cam_y: cam_y(),
                cam_z: cam_z(),
            }
        }
    }
}
```

### Orbiting Camera

```rust
fn app() -> Element {
    let mut angle = use_signal(|| 0.0f32);
    let mut radius = use_signal(|| 10.0f32);
    let mut height = use_signal(|| 5.0f32);
    
    // Calculate camera position
    let cam_x = radius() * angle().cos();
    let cam_z = radius() * angle().sin();
    let cam_y = height();
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 300px; padding: 20px;",
                input {
                    r#type: "range",
                    min: "0",
                    max: "6.28",
                    step: "0.01",
                    value: "{angle()}",
                    oninput: move |e| angle.set(e.value().parse().unwrap_or(0.0))
                }
                "Angle: {angle():.2}"
                
                input {
                    r#type: "range",
                    min: "5",
                    max: "30",
                    value: "{radius()}",
                    oninput: move |e| radius.set(e.value().parse().unwrap_or(10.0))
                }
                "Radius: {radius():.1}"
            }
            
            ThreeView {
                cam_x: cam_x,
                cam_y: cam_y,
                cam_z: cam_z,
                target_x: 0.0,
                target_y: 0.0,
                target_z: 0.0,
            }
        }
    }
}
```

## Model Transform Management

### Interactive Transform Controls

```rust
fn app() -> Element {
    let mut pos = use_signal(|| (0.0f32, 0.0f32, 0.0f32));
    let mut rot = use_signal(|| (0.0f32, 0.0f32, 0.0f32));
    let mut scale = use_signal(|| 1.0f32);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 300px; padding: 20px; background: #1a1a2e; color: white; overflow-y: auto;",
                h3 { "Transform" }
                
                // Position
                h4 { "Position" }
                Slider { label: "X", value: pos().0, min: -10.0, max: 10.0, 
                    onchange: move |v| pos.set((v, pos().1, pos().2)) }
                Slider { label: "Y", value: pos().1, min: -10.0, max: 10.0,
                    onchange: move |v| pos.set((pos().0, v, pos().2)) }
                Slider { label: "Z", value: pos().2, min: -10.0, max: 10.0,
                    onchange: move |v| pos.set((pos().0, pos().1, v)) }
                
                // Rotation
                h4 { "Rotation" }
                Slider { label: "X", value: rot().0, min: 0.0, max: 360.0,
                    onchange: move |v| rot.set((v, rot().1, rot().2)) }
                Slider { label: "Y", value: rot().1, min: 0.0, max: 360.0,
                    onchange: move |v| rot.set((rot().0, v, rot().2)) }
                Slider { label: "Z", value: rot().2, min: 0.0, max: 360.0,
                    onchange: move |v| rot.set((rot().0, rot().1, v)) }
                
                // Scale
                h4 { "Scale" }
                Slider { label: "S", value: scale(), min: 0.1, max: 5.0,
                    onchange: move |v| scale.set(v) }
            }
            
            ThreeView {
                pos_x: pos().0,
                pos_y: pos().1,
                pos_z: pos().2,
                rot_x: rot().0,
                rot_y: rot().1,
                rot_z: rot().2,
                scale: scale(),
                auto_rotate: false,
            }
        }
    }
}

#[component]
fn Slider(label: String, value: f32, min: f32, max: f32, onchange: EventHandler<f32>) -> Element {
    rsx! {
        div { style: "margin: 5px 0;",
            label { style: "display: inline-block; width: 30px;", "{label}" }
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "0.1",
                value: "{value}",
                oninput: move |e| onchange.call(e.value().parse().unwrap_or(value))
            }
            span { style: "float: right;", "{value:.1}" }
        }
    }
}
```

## Scene Settings Management

### Toggles for Visual Helpers

```rust
fn app() -> Element {
    let mut show_grid = use_signal(|| true);
    let mut show_axes = use_signal(|| true);
    let mut shadows = use_signal(|| true);
    let mut wireframe = use_signal(|| false);
    
    let mut bg_color = use_signal(|| "#1a1a2e".to_string());
    let mut model_color = use_signal(|| "#ff6b6b".to_string());
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 250px; padding: 20px; background: #1a1a2e; color: white;",
                h3 { "Scene Settings" }
                
                // Toggles
                Toggle { label: "Show Grid", checked: show_grid(),
                    onchange: move |v| show_grid.set(v) }
                Toggle { label: "Show Axes", checked: show_axes(),
                    onchange: move |v| show_axes.set(v) }
                Toggle { label: "Shadows", checked: shadows(),
                    onchange: move |v| shadows.set(v) }
                Toggle { label: "Wireframe", checked: wireframe(),
                    onchange: move |v| wireframe.set(v) }
                
                // Colors
                h4 { "Colors" }
                div { style: "margin: 10px 0;",
                    label { "Background" }
                    input {
                        r#type: "color",
                        value: "{bg_color()}",
                        oninput: move |e| bg_color.set(e.value())
                    }
                }
                div { style: "margin: 10px 0;",
                    label { "Model" }
                    input {
                        r#type: "color",
                        value: "{model_color()}",
                        oninput: move |e| model_color.set(e.value())
                    }
                }
            }
            
            ThreeView {
                show_grid: show_grid(),
                show_axes: show_axes(),
                shadows: shadows(),
                wireframe: wireframe(),
                background: bg_color(),
                color: model_color(),
            }
        }
    }
}

#[component]
fn Toggle(label: String, checked: bool, onchange: EventHandler<bool>) -> Element {
    rsx! {
        label { style: "display: flex; align-items: center; margin: 10px 0; cursor: pointer;",
            input {
                r#type: "checkbox",
                checked: "{checked}",
                onchange: move |e| onchange.call(e.checked())
            }
            span { style: "margin-left: 8px;", "{label}" }
        }
    }
}
```

## Animation Control

### Auto-Rotation with Speed

```rust
fn app() -> Element {
    let mut auto_rotate = use_signal(|| true);
    let mut rot_speed = use_signal(|| 1.0f32);
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 250px; padding: 20px;",
                label { style: "display: flex; align-items: center;",
                    input {
                        r#type: "checkbox",
                        checked: "{auto_rotate()}",
                        onchange: move |e| auto_rotate.set(e.checked())
                    }
                    span { style: "margin-left: 8px;", "Auto Rotate" }
                }
                
                if auto_rotate() {
                    div { style: "margin-top: 10px;",
                        label { "Speed: {rot_speed():.1}x" }
                        input {
                            r#type: "range",
                            min: "0.1",
                            max: "5.0",
                            step: "0.1",
                            value: "{rot_speed()}",
                            oninput: move |e| rot_speed.set(e.value().parse().unwrap_or(1.0))
                        }
                    }
                }
            }
            
            ThreeView {
                auto_rotate: auto_rotate(),
                rot_speed: rot_speed(),
            }
        }
    }
}
```

## Shader Management

### Dynamic Shader Switching

```rust
use dioxus_three::ShaderPreset;

fn app() -> Element {
    let mut current_shader = use_signal(|| ShaderPreset::None);
    
    let shaders = vec![
        ("None", ShaderPreset::None),
        ("Gradient", ShaderPreset::Gradient),
        ("Water", ShaderPreset::Water),
        ("Hologram", ShaderPreset::Hologram),
        ("Toon", ShaderPreset::Toon),
        ("Heatmap", ShaderPreset::Heatmap),
    ];
    
    rsx! {
        div { style: "display: flex; height: 100vh;",
            div { style: "width: 250px; padding: 20px;",
                h3 { "Shader Effects" }
                
                for (name, shader) in shaders {
                    button {
                        style: if matches_shader(&current_shader(), &shader) {
                            "background: #DEC647; margin: 5px;"
                        } else {
                            "margin: 5px;"
                        },
                        onclick: move |_| current_shader.set(shader.clone()),
                        "{name}"
                    }
                }
            }
            
            ThreeView {
                shader: current_shader(),
                auto_rotate: matches!(current_shader(), ShaderPreset::None),
            }
        }
    }
}

fn matches_shader(a: &ShaderPreset, b: &ShaderPreset) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
```

## Complete Control Panel Example

```rust
fn app() -> Element {
    // Camera
    let mut cam_x = use_signal(|| 8.0f32);
    let mut cam_y = use_signal(|| 8.0f32);
    let mut cam_z = use_signal(|| 8.0f32);
    
    // Transform
    let mut scale = use_signal(|| 1.0f32);
    let mut rot_y = use_signal(|| 0.0f32);
    
    // Scene
    let mut show_grid = use_signal(|| true);
    let mut auto_rotate = use_signal(|| true);
    let mut wireframe = use_signal(|| false);
    
    // Shader
    let mut shader = use_signal(|| ShaderPreset::None);
    
    rsx! {
        div { style: "display: flex; height: 100vh; font-family: sans-serif;",
            // Control Panel
            div { style: "width: 320px; padding: 20px; background: #1a1a2e; color: white; overflow-y: auto;",
                h2 { style: "color: #DEC647; margin-top: 0;", "3D Scene Control" }
                
                // Camera Section
                ControlSection { title: "Camera",
                    CamSlider { label: "X", value: cam_x, min: -30.0, max: 30.0 }
                    CamSlider { label: "Y", value: cam_y, min: -30.0, max: 30.0 }
                    CamSlider { label: "Z", value: cam_z, min: -30.0, max: 30.0 }
                    
                    div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 5px; margin-top: 10px;",
                        CamButton { label: "Top", x: 0.0, y: 15.0, z: 0.01, cam_x, cam_y, cam_z }
                        CamButton { label: "Side", x: 15.0, y: 0.0, z: 0.0, cam_x, cam_y, cam_z }
                        CamButton { label: "Iso", x: 10.0, y: 10.0, z: 10.0, cam_x, cam_y, cam_z }
                    }
                }
                
                // Transform Section
                ControlSection { title: "Transform",
                    Slider { label: "Scale", value: scale, min: 0.1, max: 5.0, step: 0.1 }
                    if !auto_rotate() {
                        Slider { label: "Rotate Y", value: rot_y, min: 0.0, max: 360.0, step: 1.0 }
                    }
                }
                
                // Scene Section
                ControlSection { title: "Scene",
                    Toggle { label: "Grid", value: show_grid }
                    Toggle { label: "Auto Rotate", value: auto_rotate }
                    Toggle { label: "Wireframe", value: wireframe }
                }
                
                // Shader Section
                ControlSection { title: "Shader",
                    ShaderSelect { current: shader }
                }
                
                // Reset
                button {
                    style: "width: 100%; padding: 10px; background: #ff6b6b; color: white; border: none; cursor: pointer;",
                    onclick: move || {
                        cam_x.set(8.0); cam_y.set(8.0); cam_z.set(8.0);
                        scale.set(1.0); rot_y.set(0.0);
                        show_grid.set(true); auto_rotate.set(true); wireframe.set(false);
                        shader.set(ShaderPreset::None);
                    },
                    "Reset All"
                }
            }
            
            // 3D View
            ThreeView {
                cam_x: cam_x(),
                cam_y: cam_y(),
                cam_z: cam_z(),
                scale: scale(),
                rot_y: rot_y(),
                show_grid: show_grid(),
                auto_rotate: auto_rotate(),
                wireframe: wireframe(),
                shader: shader(),
            }
        }
    }
}

#[component]
fn ControlSection(title: String, children: Element) -> Element {
    rsx! {
        div { style: "margin-bottom: 20px; padding-bottom: 20px; border-bottom: 1px solid #333;",
            h3 { style: "color: #888; font-size: 14px; text-transform: uppercase; margin: 0 0 10px 0;",
                "{title}"
            }
            {children}
        }
    }
}

#[component]
fn Slider(label: String, value: Signal<f32>, min: f32, max: f32, step: f32) -> Element {
    rsx! {
        div { style: "margin: 8px 0;",
            div { style: "display: flex; justify-content: space-between;",
                label { style: "font-size: 12px;", "{label}" }
                span { style: "font-size: 12px; color: #DEC647;", "{value():.1}" }
            }
            input {
                style: "width: 100%;",
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value()}",
                oninput: move |e| value.set(e.value().parse().unwrap_or(value()))
            }
        }
    }
}

#[component]
fn CamSlider(label: String, value: Signal<f32>, min: f32, max: f32) -> Element {
    Slider { label, value, min, max, step: 0.1 }
}

#[component]
fn CamButton(
    label: String,
    x: f32, y: f32, z: f32,
    cam_x: Signal<f32>, cam_y: Signal<f32>, cam_z: Signal<f32>
) -> Element {
    rsx! {
        button {
            style: "padding: 5px; background: #333; color: white; border: none; cursor: pointer;",
            onclick: move || { cam_x.set(x); cam_y.set(y); cam_z.set(z); },
            "{label}"
        }
    }
}

#[component]
fn Toggle(label: String, value: Signal<bool>) -> Element {
    rsx! {
        label { style: "display: flex; align-items: center; margin: 8px 0; cursor: pointer;",
            input {
                r#type: "checkbox",
                checked: "{value()}",
                onchange: move |e| value.set(e.checked())
            }
            span { style: "margin-left: 8px; font-size: 14px;", "{label}" }
        }
    }
}

#[component]
fn ShaderSelect(current: Signal<ShaderPreset>) -> Element {
    let options = vec![
        ("None", ShaderPreset::None),
        ("Gradient", ShaderPreset::Gradient),
        ("Water", ShaderPreset::Water),
        ("Hologram", ShaderPreset::Hologram),
        ("Toon", ShaderPreset::Toon),
        ("Heatmap", ShaderPreset::Heatmap),
    ];
    
    rsx! {
        select {
            style: "width: 100%; padding: 8px; background: #333; color: white; border: none;",
            onchange: move |e| {
                current.set(match e.value().as_str() {
                    "gradient" => ShaderPreset::Gradient,
                    "water" => ShaderPreset::Water,
                    "hologram" => ShaderPreset::Hologram,
                    "toon" => ShaderPreset::Toon,
                    "heatmap" => ShaderPreset::Heatmap,
                    _ => ShaderPreset::None,
                });
            },
            option { value: "none", selected: matches!(current(), ShaderPreset::None), "None" }
            option { value: "gradient", selected: matches!(current(), ShaderPreset::Gradient), "Gradient" }
            option { value: "water", selected: matches!(current(), ShaderPreset::Water), "Water" }
            option { value: "hologram", selected: matches!(current(), ShaderPreset::Hologram), "Hologram" }
            option { value: "toon", selected: matches!(current(), ShaderPreset::Toon), "Toon" }
            option { value: "heatmap", selected: matches!(current(), ShaderPreset::Heatmap), "Heatmap" }
        }
    }
}
```

## Platform-Specific Considerations

### Desktop Platform

On desktop, the WebView iframe HTML is generated once and only regenerated when the model count changes. All other prop updates (camera, selection, gizmo, etc.) are sent via `postMessage` without iframe reload:

```rust
// Desktop - postMessage state update (no reload)
ThreeView {
    cam_x: cam_x(),  // Sends postMessage to iframe
}

// Only this triggers HTML regeneration:
models.write().push(new_model);  // Model count changed
```

### Web Platform

On web (WASM), state updates are synchronized via JavaScript without full re-renders:

```rust
// Web - Efficient state synchronization
ThreeView {
    cam_x: cam_x(),  // Updates JavaScript state object
}
```

### Signal Optimization

For better performance with frequent updates:

```rust
fn app() -> Element {
    // Use Memo for derived values
    let angle = use_signal(|| 0.0f32);
    let radius = use_signal(|| 10.0f32);
    
    // Memoized camera position
    let cam_pos = use_memo(move || {
        (
            radius() * angle().cos(),
            5.0f32,
            radius() * angle().sin()
        )
    });
    
    rsx! {
        ThreeView {
            cam_x: cam_pos().0,
            cam_y: cam_pos().1,
            cam_z: cam_pos().2,
        }
    }
}
```

## Best Practices

1. **Use signals for interactive properties** - Camera, transforms, toggles
2. **Group related properties** - Use tuples or structs for position/rotation
3. **Memoize derived values** - For calculated properties like orbital positions
4. **Use controlled inputs** - Always bind input values to signals
5. **Provide reset functionality** - Allow users to return to default state
6. **Organize controls** - Group by category (Camera, Transform, Scene)
7. **Show current values** - Display numeric values next to sliders
8. **Use presets** - Common views (Top, Side, Isometric) for camera

## Property Reference

| Category | Properties | Signal Type |
|----------|------------|-------------|
| **Camera** | `cam_x`, `cam_y`, `cam_z`, `target_x`, `target_y`, `target_z` | `f32` |
| **Position** | `pos_x`, `pos_y`, `pos_z` | `f32` |
| **Rotation** | `rot_x`, `rot_y`, `rot_z` | `f32` |
| **Scale** | `scale` | `f32` |
| **Appearance** | `color`, `background`, `wireframe` | `String`/`bool` |
| **Scene** | `show_grid`, `show_axes`, `shadows` | `bool` |
| **Animation** | `auto_rotate`, `rot_speed` | `bool`/`f32` |
| **Shader** | `shader` | `ShaderPreset` |
