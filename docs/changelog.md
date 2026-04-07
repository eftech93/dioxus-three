# Changelog

All notable changes to Dioxus Three will be documented in this file.

**Maintainer:** Esteban Puello - [eftech93@gmail.com](mailto:eftech93@gmail.com)  
**Repository:** [github.com/eftech93/dioxus-three](https://github.com/eftech93/dioxus-three)

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
- [ ] Raycasting for click/hover events
- [ ] Offline mode (bundle Three.js)
- [ ] Multiple viewports
- [ ] Screenshot/export functionality

### Potential Improvements

- [ ] Message passing instead of HTML regeneration
- [ ] Model caching
- [ ] Virtual scrolling for multiple views
- [ ] Shader hot-reload in development
- [ ] VR/AR support via WebXR

---

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 0.0.2 | 2024-04-07 | Web platform support, multi-model support |
| 0.0.1 | 2024-04-05 | Initial release |

---

## Contributing to Changelog

When submitting PRs, please add an entry under the appropriate section:

- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security improvements

Format: `- Description ([#PR](link))`
