# Changelog

All notable changes to Dioxus Three will be documented in this file.

**Maintainer:** Esteban Puello - [eftech93@gmail.com](mailto:eftech93@gmail.com)  
**Repository:** [github.com/eftech93/dioxus-three](https://github.com/eftech93/dioxus-three)

## [0.0.3]

### Added

- **Phase 1: Input & Selection System** — Full raycasting, selection, and gizmo support
  - `RaycastConfig` with `enabled`, `recursive`, `max_distance`, `layer_mask`
  - Pointer events: `on_pointer_down`, `on_pointer_move`, `on_pointer_up`, `on_pointer_drag`
  - Gesture events: `on_gesture` (pinch, rotate, pan)
  - `Selection` state with `SelectionMode::Single` and `SelectionMode::Multiple`
  - `SelectionStyle` for customizable outline (color, width, glow)
  - `Gizmo` struct with `Translate`, `Rotate`, `Scale` modes and `World`/`Local` space
  - `on_gizmo_drag` callback with live `GizmoTransform` during drag and `is_finished` flag
  - Selection outline: wireframe box + inner glow (no corner cubes)
  - `EntityId` type for identifying scene objects

- **Desktop Gizmo Support** — Official `THREE.TransformControls` in iframe
  - Translate, Rotate, Scale handles with axis/plane constraints
  - Gizmo events bridged via `document::eval` + `postMessage`
  - `isClickOnGizmo` correctly filters for mesh objects only (`isMesh` check)

- **Web Gizmo Support** — Custom-built gizmos with manual raycasting
  - Translate handles: arrow cones with camera-facing plane intersection drag math
  - Rotate handles: tori (rings) with arcball rotation
  - Scale handles: boxes with distance-based scaling
  - Center uniform-scale handle (white box)
  - Live `entityMap` reads from `canvas.dioxusThreeState` to avoid stale references

- **Desktop iframe state update optimization**
  - HTML generated once via `use_signal`; only regenerated when model count changes
  - All other prop updates sent via `postMessage("update-state")` without iframe reload
  - Camera, selection, gizmo, and selection-style updates handled in-message

- **Transform readout UI** — Both desktop and web demos show live position/rotation/scale

### Fixed

- **Web Platform Camera Controls** - Fixed camera position not updating in web demo
  - Camera object now properly stored in `dioxusThreeState`
  - Immediate camera position updates when state changes

- **Desktop gizmo click detection** — `isClickOnGizmo` now checks `intersects[i].object.isMesh`
  - Previously counted gizmo lines as hits, causing selection handler to bail out

- **Web `entityMap` stale reference** — `updateGizmo` reads from live `canvas.dioxusThreeState.entityMap`
  - Previously used captured closure variable pointing to removed meshes

- **Translate drag math** — Replaced `closestPointOnLineToRay` with camera-facing plane intersection
  - Old approach found closest approach between infinite lines, often giving wrong results
  - New approach: create plane containing drag axis, intersect mouse ray, project delta onto axis

- **Scale sensitivity** — Increased multiplier from `*2` to `*4`

- **Choppy drag performance** — `transform_overrides` no longer baked into `model_configs`
  - Baking overrides changed `props.models` every frame, triggering full model reload
  - Fixed by passing raw `m.config.clone()`; gizmo manipulates JS objects directly
  - Added defensive model comparison in web implementation

- **Selection outline cleanup** — Removed 8 corner-marker cubes from outline
  - Now just wireframe box + inner glow for cleaner look

### Technical

- `use_reactive` hook for prop change detection
- `use_effect` with signal subscriptions for real-time updates
- `wasm_bindgen_futures` for async loader loading
- JavaScript state object stored on canvas element (`dioxusThreeState`)
- Desktop: `document::eval` bridge for iframe→Rust events
- Web: `wasm_bindgen` closures (`dioxusThreeRustBridge`) for canvas→Rust events
- `updateModels()` in desktop iframe: updates transforms/creates cubes/removes objects without full reload

## [0.0.2] - 2024-04-07

### Added

- **Web (WASM) Platform Support** - Full support for Dioxus Web with reactive updates
  - Uses HTML5 Canvas with Three.js
  - Dynamic loader injection for different model formats
  - Real-time state synchronization between Rust and JavaScript
- **Multi-Model Support** - Load and display multiple models simultaneously
  - `models` prop accepts `Vec<ModelConfig>` for multiple models
  - Each model has independent position, rotation, scale, and color
  - Dynamic model loading/unloading
- **Mobile Platform Fixes** - Fixed signal subscription for mobile demo
- **Model Format Loaders** - Automatic loading of format-specific loaders:
  - OBJLoader for OBJ files
  - GLTFLoader for glTF/GLB files
  - FBXLoader for FBX files (with fflate dependency)
  - STLLoader for STL files
  - PLYLoader for PLY files

### Changed

- **Web Implementation** - Complete rewrite of web platform
  - Direct canvas rendering instead of iframe
  - JavaScript state object for real-time updates
  - Async loader loading for external models
- **Demo Updates** - Added `ThreeViewWrapper` component for proper signal handling

### Fixed

- Web platform control panel not updating the scene
- Mobile platform signal subscription issues
- Model loaders not being available for external models

### Technical

- `use_reactive` hook for prop change detection
- `use_effect` with signal subscriptions for real-time updates
- `wasm_bindgen_futures` for async loader loading
- JavaScript state object stored on canvas element

## [0.0.1] - 2024-04-05

### Added

- Initial release of Dioxus Three
- `ThreeView` component for embedding 3D scenes
- Support for 8 model formats: OBJ, FBX, GLTF, GLB, STL, PLY, DAE, and built-in Cube
- 6 built-in shader presets: Gradient, Water, Hologram, Toon, Heatmap, and None
- Custom GLSL shader support with `ShaderConfig`
- Transform controls: position, rotation, scale
- Camera control: position and target
- Auto-rotation with adjustable speed
- Auto-center and auto-scale options
- Wireframe mode
- Grid and axes helpers
- Shadow support
- Demo application with interactive controls

### Technical

- Three.js r128 via CDN
- WebView-based rendering (desktop/mobile)
- GLSL shader embedding
- Format-specific loader injection
- Animated shader uniforms

## Future Releases

### Planned Features

- [ ] Texture loading from URLs
- [ ] Animation playback for glTF/FBX
- [ ] Post-processing effects (bloom, DOF, SSAO)
- [ ] Offline mode (bundle Three.js)
- [ ] Multiple viewports
- [ ] Screenshot/export functionality
