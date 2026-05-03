# Dioxus Three - Web Demo

A web-optimized demo of the Dioxus Three 3D viewer component using Dioxus Web (WASM).

## Prerequisites

```bash
# Install Dioxus CLI
cargo install dioxus-cli

# Install WASM target
rustup target add wasm32-unknown-unknown
```

## Running the Demo

### Development Server

```bash
cd examples/web-demo

# Serve with hot reload
dx serve --platform web

# Or specify port
dx serve --platform web --port 8080
```

Then open http://localhost:8080 in your browser.

### Building for Production

```bash
# Build optimized release
dx build --platform web --release

# Output will be in dist/ directory
```

## Features

This web demo includes:

- 🎮 **Interactive 3D Viewer** - Multi-model scene support
- 📱 **Responsive Design** - Collapsible sidebar for mobile devices
- 🎨 **Shader Effects** - Gradient, Water, Hologram, Toon, Heatmap presets
- 📷 **Camera Controls** - Position and preset views (Top, Side, Isometric)
- 🔄 **Auto-rotation** - Adjustable rotation speed
- 📦 **Preset Models** - Quick-load popular Three.js example models
- 🎯 **Transform Controls** - Position, rotation, scale, and color per model
- 🖱️ **Object Selection** - Click to select objects with outline highlight
- 🔧 **Gizmo Manipulation** - Translate, Rotate, Scale handles for selected objects

## Browser Compatibility

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Mobile browsers (iOS Safari, Chrome Android)

## Architecture

This demo uses:
- **Dioxus Web** - WASM-based rendering
- **Three.js** - 3D graphics via CDN
- **Tailwind CSS** - Styling

The `ThreeView` component automatically uses the web implementation when compiled for `wasm32` target, which renders directly to a `<canvas>` element using `web-sys` bindings.
