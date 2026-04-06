# Supported Formats

Dioxus Three supports multiple 3D model formats through Three.js loaders.

## Format Overview

| Format | Extension | Best For | Loader |
|--------|-----------|----------|--------|
| **OBJ** | `.obj` | General 3D models, widely supported | OBJLoader |
| **FBX** | `.fbx` | Autodesk ecosystem, animations | FBXLoader |
| **glTF** | `.gltf` | Web-optimized, PBR materials | GLTFLoader |
| **GLB** | `.glb` | Binary glTF, single file | GLTFLoader |
| **STL** | `.stl` | 3D printing, CAD | STLLoader |
| **PLY** | `.ply` | Point clouds, scanned data | PLYLoader |
| **DAE** | `.dae` | Collada, asset exchange | ColladaLoader |

## Usage by Format

### OBJ (Wavefront)

```rust
use dioxus_three::{ThreeView, ModelFormat};

rsx! {
    ThreeView {
        model_url: Some("https://example.com/model.obj".to_string()),
        format: ModelFormat::Obj,
        auto_center: true,
    }
}
```

**Pros:**
- Widely supported
- Simple text format
- Good for static meshes

**Cons:**
- No animations
- Separate material file (.mtl)
- No PBR materials

### FBX (Autodesk)

```rust
use dioxus_three::{ThreeView, ModelFormat};

rsx! {
    ThreeView {
        model_url: Some("https://example.com/model.fbx".to_string()),
        format: ModelFormat::Fbx,
        auto_scale: true,
    }
}
```

**Pros:**
- Industry standard
- Supports animations
- Embedded textures

**Cons:**
- Complex format
- Requires fflate library
- Can be slow to load

### glTF / GLB

```rust
use dioxus_three::{ThreeView, ModelFormat};

// glTF (JSON + binary)
rsx! {
    ThreeView {
        model_url: Some("https://example.com/model.gltf".to_string()),
        format: ModelFormat::Gltf,
    }
}

// GLB (single binary file)
rsx! {
    ThreeView {
        model_url: Some("https://example.com/model.glb".to_string()),
        format: ModelFormat::Glb,
    }
}
```

**Pros:**
- Designed for web
- PBR materials
- Efficient transmission
- Animations support

**Cons:**
- Newer format (less legacy support)

### STL (StereoLithography)

```rust
use dioxus_three::{ThreeView, ModelFormat};

rsx! {
    ThreeView {
        model_url: Some("https://example.com/model.stl".to_string()),
        format: ModelFormat::Stl,
        color: "#ff6b6b".to_string(),  // STL has no color data
    }
}
```

**Pros:**
- 3D printing standard
- Simple geometry only

**Cons:**
- No materials/colors
- No textures
- Just raw triangles

### PLY (Stanford Polygon Library)

```rust
use dioxus_three::{ThreeView, ModelFormat};

rsx! {
    ThreeView {
        model_url: Some("https://example.com/model.ply".to_string()),
        format: ModelFormat::Ply,
        auto_center: true,
    }
}
```

**Pros:**
- Point cloud support
- Vertex colors
- Simple format

**Cons:**
- No materials
- Limited animation support

### DAE (Collada)

```rust
use dioxus_three::{ThreeView, ModelFormat};

rsx! {
    ThreeView {
        model_url: Some("https://example.com/model.dae".to_string()),
        format: ModelFormat::Dae,
    }
}
```

**Pros:**
- XML-based
- Asset exchange format
- Supports physics

**Cons:**
- Large file sizes
- Slower parsing
- Being replaced by glTF

## Finding Models

### Free Model Repositories

- **Three.js Examples:** `https://threejs.org/examples/models/`
- **Khronos glTF Samples:** `https://github.com/KhronosGroup/glTF-Sample-Models`
- **Sketchfab** (CC licensed)
- **Google Poly** (archived, mirrors available)

### Example URLs

```rust
let models = vec![
    // OBJ
    ("https://threejs.org/examples/models/obj/male02/male02.obj", ModelFormat::Obj),
    
    // glTF
    ("https://threejs.org/examples/models/gltf/DamagedHelmet/glTF/DamagedHelmet.gltf", ModelFormat::Gltf),
    
    // FBX
    ("https://threejs.org/examples/models/fbx/Samba%20Dancing.fbx", ModelFormat::Fbx),
    
    // STL
    ("https://threejs.org/examples/models/stl/ascii/slotted_disk.stl", ModelFormat::Stl),
    
    // PLY
    ("https://threejs.org/examples/models/ply/ascii/dolphins.ply", ModelFormat::Ply),
    
    // DAE
    ("https://threejs.org/examples/models/collada/abb_irb52_7_120.dae", ModelFormat::Dae),
];
```

## Local Files

For local development, serve files via HTTP:

```bash
# Python 3
python3 -m http.server 8080

# Node.js (if installed)
npx serve .

# PHP
php -S localhost:8080
```

Then reference with `http://localhost:8080/model.obj`.

## Format Selection Guide

| Use Case | Recommended Format |
|----------|-------------------|
| General 3D viewing | glTF/GLB |
| Game assets | glTF/GLB or FBX |
| 3D printing | STL |
| CAD/Engineering | OBJ or STL |
| Scan data | PLY |
| Legacy assets | FBX or DAE |
| Web optimization | GLB (binary glTF) |
