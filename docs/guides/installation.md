# Installation

**Maintainer:** Esteban Puello - [eftech93@gmail.com](mailto:eftech93@gmail.com)  
**Repository:** [github.com/eftech93/dioxus-three](https://github.com/eftech93/dioxus-three)

## Requirements

- **Rust** 1.76+ (install from [rustup.rs](https://rustup.rs))
- **Dioxus** 0.6+
- **Internet connection** (for Three.js CDN and external models)

## Add to Your Project

Add `dioxus-three` to your `Cargo.toml`:

```toml
[dependencies]
dioxus-three = "0.0.3"
dioxus = { version = "0.6", features = ["desktop"] }
```

### Platform-Specific Dependencies

**For Desktop:**
```toml
[dependencies]
dioxus-three = "0.0.3"
dioxus = { version = "0.6", features = ["desktop"] }
dioxus-desktop = "0.6"
```

**For Web (WASM):**
```toml
[dependencies]
dioxus-three = "0.0.3"
dioxus = { version = "0.6", features = ["web"] }
```

**For Mobile:**
```toml
[dependencies]
dioxus-three = "0.0.3"
dioxus = { version = "0.6", features = ["mobile"] }
dioxus-mobile = "0.6"
```

## Running the Demo

Clone the repository and run the demo:

```bash
git clone https://github.com/eftech93/dioxus-three
cd dioxus-three/examples/demo
cargo run
```

## Platform-Specific Examples

### Desktop

```bash
cd examples/demo
cargo run
```

### Web (WASM)

```bash
cd examples/web-demo

# Install Dioxus CLI if needed: cargo install dioxus-cli
dx serve --platform web
```

### Mobile

```bash
cd examples/mobile-demo

# Android
dx build --platform android

# iOS (macOS only)
dx build --platform ios
```

## Verify Installation

Create a simple test application:

```rust
use dioxus::prelude::*;
use dioxus_three::ThreeView;

fn main() {
    dioxus_desktop::launch(app);
}

fn app() -> Element {
    rsx! {
        div { style: "width: 100vw; height: 100vh;",
            ThreeView {
                auto_rotate: true,
                color: "#00ff00".to_string(),
            }
        }
    }
}
```

Run with:

```bash
cargo run
```

You should see a rotating green cube!

## Troubleshooting

### Missing dependencies

If you get compilation errors, ensure you have the correct Dioxus version:

```toml
[dependencies]
dioxus = { version = "0.6", features = ["desktop"] }
dioxus-three = "0.0.3"
```

### Models not loading

Ensure you have an internet connection - Three.js is loaded from CDN and external models need to be fetched.

For local files, use a local HTTP server:

```bash
python3 -m http.server 8080
```

Then access via `http://localhost:8080/model.obj`.

### Web build issues

Make sure you have the Dioxus CLI installed:

```bash
cargo install dioxus-cli
```

And the correct target:

```bash
rustup target add wasm32-unknown-unknown
```

### Mobile build issues

Ensure you have the Android SDK (for Android) or Xcode (for iOS) installed.

See the [Dioxus Mobile documentation](https://dioxuslabs.com/learn/0.6/reference/mobile) for setup instructions.
