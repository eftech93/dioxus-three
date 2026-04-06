# ModelFormat

Enumeration of supported 3D model formats.

## Definition

```rust
#[derive(Clone, PartialEq, Debug)]
pub enum ModelFormat {
    Obj,
    Fbx,
    Gltf,
    Glb,
    Stl,
    Ply,
    Dae,
    Json,
    Cube,
}
```

## Variants

### `ModelFormat::Obj`

Wavefront OBJ format - widely supported text-based format.

```rust
use dioxus_three::ModelFormat;

ThreeView {
    model_url: Some("model.obj".to_string()),
    format: ModelFormat::Obj,
}
```

**Pros:**
- Simple text format
- Widely supported
- Good for static meshes

**Cons:**
- No animations
- Requires separate .mtl file for materials
- No PBR materials

### `ModelFormat::Fbx`

Autodesk FBX format - industry standard.

```rust
ThreeView {
    model_url: Some("model.fbx".to_string()),
    format: ModelFormat::Fbx,
}
```

**Pros:**
- Industry standard
- Supports animations
- Embedded textures

**Cons:**
- Complex format
- Larger file sizes

### `ModelFormat::Gltf`

GL Transmission Format (JSON version).

```rust
ThreeView {
    model_url: Some("model.gltf".to_string()),
    format: ModelFormat::Gltf,
}
```

**Pros:**
- Designed for web
- PBR materials
- Animations

**Cons:**
- May have separate binary/textures

### `ModelFormat::Glb`

GL Transmission Format (binary version).

```rust
ThreeView {
    model_url: Some("model.glb".to_string()),
    format: ModelFormat::Glb,
}
```

**Pros:**
- Single file
- All data embedded
- Most efficient for web

**Cons:**
- Binary format (not human-readable)

### `ModelFormat::Stl`

StereoLithography format - 3D printing standard.

```rust
ThreeView {
    model_url: Some("model.stl".to_string()),
    format: ModelFormat::Stl,
    color: "#ff6b6b".to_string(),  // STL has no color data
}
```

**Pros:**
- 3D printing standard
- Simple geometry

**Cons:**
- No materials/colors
- Just raw triangles

### `ModelFormat::Ply`

Stanford Polygon Library format.

```rust
ThreeView {
    model_url: Some("model.ply".to_string()),
    format: ModelFormat::Ply,
}
```

**Pros:**
- Point cloud support
- Vertex colors
- Simple format

**Cons:**
- No materials
- Limited animation support

### `ModelFormat::Dae`

Collada format - asset exchange format.

```rust
ThreeView {
    model_url: Some("model.dae".to_string()),
    format: ModelFormat::Dae,
}
```

**Pros:**
- XML-based
- Good for asset exchange
- Physics support

**Cons:**
- Large file sizes
- Being replaced by glTF

### `ModelFormat::Json`

Three.js JSON format.

```rust
ThreeView {
    model_url: Some("model.json".to_string()),
    format: ModelFormat::Json,
}
```

### `ModelFormat::Cube`

Default built-in cube - no file needed.

```rust
ThreeView {
    format: ModelFormat::Cube,  // or just omit model_url
}
```

## Methods

### `as_str()`

Returns the format identifier string.

```rust
let format = ModelFormat::Obj;
assert_eq!(format.as_str(), "obj");
```

## Comparison

| Format | Animation | PBR | Textures | Size | Web |
|--------|-----------|-----|----------|------|-----|
| OBJ | ❌ | ❌ | Separate | Small | ✅ |
| FBX | ✅ | ⚠️ | Embedded | Large | ✅ |
| glTF | ✅ | ✅ | Separate | Small | ✅ |
| GLB | ✅ | ✅ | Embedded | Small | ✅ |
| STL | ❌ | ❌ | None | Small | ✅ |
| PLY | ❌ | ❌ | Vertex | Small | ✅ |
| DAE | ✅ | ⚠️ | Embedded | Large | ✅ |

## Recommendation

For best results, use **GLB** (binary glTF) format:
- Single file
- Efficient
- Full feature support
- Web-optimized
