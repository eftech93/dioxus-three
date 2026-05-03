# Dioxus Three - Mobile Demo

A mobile-optimized demo of the Dioxus Three 3D viewer component for iOS and Android.

## Prerequisites

### General

```bash
# Install Dioxus CLI
cargo install dioxus-cli

# Install mobile targets
rustup target add aarch64-apple-ios
rustup target add aarch64-linux-android
```

### Android Setup

1. Install Android SDK and NDK
2. Set environment variables:
```bash
export ANDROID_SDK_ROOT=$HOME/Library/Android/sdk
export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/25.2.9519653
export PATH="$ANDROID_SDK_ROOT/platform-tools:$PATH"
```

3. Install cargo-ndk:
```bash
cargo install cargo-ndk
```

### iOS Setup (macOS only)

1. Install Xcode from App Store
2. Install Xcode command line tools:
```bash
xcode-select --install
```

## Running the Demo

### Android

```bash
cd examples/mobile-demo

# Build and run on connected device/emulator
cargo ndk -t arm64-v8a -o app/src/main/jniLibs/ build --release

# Or use Dioxus CLI
dx build --platform android
```

### iOS (macOS only)

```bash
cd examples/mobile-demo

# Build for iOS
dx build --platform ios

# Or use cargo directly
cargo build --target aarch64-apple-ios --release
```

## Features

This mobile demo includes:

- 🎮 **Touch-Optimized UI** - Bottom sheet controls, swipe-friendly interface
- 📱 **Mobile-First Design** - Large touch targets, responsive layout
- 🎨 **Quick Actions** - Floating buttons for common operations
- 📦 **Model Management** - Horizontal scrolling model list
- 🔄 **Auto-rotation** - Toggle with one tap
- 🎯 **Transform Controls** - Position, rotation, scale editing
- 🖱️ **Object Selection** - Tap to select objects
- 🔧 **Gizmo Manipulation** - Translate, Rotate, Scale handles (experimental)
- 📷 **Camera Presets** - Top, Side, Isometric views

## Architecture

This demo uses:
- **Dioxus Mobile** - WebView-based mobile app framework
- **Three.js** - 3D graphics via CDN (loaded in WebView)
- **Tailwind CSS** - Mobile-responsive styling

The mobile version uses the same `ThreeView` component as desktop (via WebView), ensuring consistent rendering across platforms.

## Project Structure

```
mobile-demo/
├── Cargo.toml          # Mobile dependencies
├── Dioxus.toml         # Mobile build config
├── README.md           # This file
└── src/
    └── main.rs         # Mobile app entry point
```

## Platform Notes

### Android
- Minimum SDK: 21 (Android 5.0)
- Target SDK: 34
- Uses WebView for rendering

### iOS
- Minimum iOS: 13.0
- Uses WKWebView for rendering
- Requires macOS for building

## Troubleshooting

### Android build fails
- Ensure NDK is properly installed and `ANDROID_NDK_ROOT` is set
- Check that `cargo-ndk` is installed

### iOS build fails
- Ensure Xcode license is accepted: `sudo xcodebuild -license accept`
- Check that iOS targets are installed: `rustup target list | grep ios`

### App crashes on launch
- Check device logs: `adb logcat` (Android) or Console app (iOS)
- Ensure internet permission is granted (Android: `AndroidManifest.xml`)
