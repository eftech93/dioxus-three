# Dioxus Three - Examples

This directory contains example applications demonstrating how to use Dioxus Three across different platforms.

## Examples Overview

| Example | Platform | Description |
|---------|----------|-------------|
| [demo](./demo) | Desktop | Full-featured desktop demo with multi-model support |
| [web-demo](./web-demo) | Web/WASM | Browser-based demo with responsive UI |
| [mobile-demo](./mobile-demo) | iOS/Android | Mobile-optimized demo with touch-friendly controls |

## Quick Start

### Desktop Demo

```bash
cd examples/demo
cargo run
```

### Web Demo

```bash
# Install Dioxus CLI if not already installed
cargo install dioxus-cli

# Run web demo
cd examples/web-demo
dx serve --platform web
```

Open http://localhost:8080 in your browser.

### Mobile Demo

#### Android
```bash
cd examples/mobile-demo

# Build for Android
cargo ndk -t arm64-v8a -o app/src/main/jniLibs/ build --release

# Or use Dioxus CLI
dx build --platform android
```

#### iOS (macOS only)
```bash
cd examples/mobile-demo

# Build for iOS
dx build --platform ios
```

## Platform-Specific Notes

### Desktop
- Uses WebView to render Three.js
- Full window management via dioxus-desktop
- Best for development and testing

### Web
- Compiles to WASM for browser execution
- Uses web-sys for DOM manipulation
- Responsive design for mobile browsers
- Requires `wasm32-unknown-unknown` target

### Mobile
- Uses WebView (WKWebView on iOS, WebView on Android)
- Same rendering engine as desktop
- Touch-optimized UI components
- Requires platform-specific SDKs (Android SDK/NDK, Xcode)

## Shared Features

All examples support:

- **Multiple 3D formats**: OBJ, FBX, GLTF, GLB, STL, PLY, DAE
- **Shader effects**: Gradient, Water, Hologram, Toon, Heatmap
- **Camera controls**: Position and preset views
- **Transform editing**: Position, rotation, scale
- **Auto-rotation**: Configurable speed
- **Model management**: Add, remove, select models

## Architecture

Each example uses the same `ThreeView` component from the main library:

```rust
use dioxus_three::{ThreeView, ModelConfig, ModelFormat};

rsx! {
    ThreeView {
        models: vec![
            ModelConfig::new("model.obj", ModelFormat::Obj)
                .with_color("#ff6b6b")
                .with_position(0.0, 0.0, 0.0),
        ],
        auto_rotate: true,
        shader: ShaderPreset::Gradient,
    }
}
```

The library automatically selects the appropriate implementation:
- **Desktop/Mobile**: WebView-based rendering via `desktop.rs`
- **Web**: Canvas-based rendering via `web.rs` using web-sys

## Customization

Each example can be customized by modifying:
- `src/main.rs` - Application logic and UI
- `Cargo.toml` - Dependencies
- `Dioxus.toml` - Platform configuration

## Building from Root

You can also build examples from the project root:

```bash
# Desktop example
cargo run --example demo

# Web example (requires wasm32 target)
cargo build --example web-demo --target wasm32-unknown-unknown

# Note: Mobile examples must be built from their directories
```
