# Changelog

All notable changes to Dioxus Three will be documented in this file.

**Maintainer:** Esteban Puello - [eftech93@gmail.com](mailto:eftech93@gmail.com)  
**Repository:** [github.com/eftech93/dioxus-three](https://github.com/eftech93/dioxus-three)

## [0.1.0] - 2024-04-05

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
- WebView-based rendering
- GLSL shader embedding
- Format-specific loader injection
- Animated shader uniforms

## Future Releases

### Planned Features

- [ ] Texture loading from URLs
- [ ] Animation playback for glTF/FBX
- [ ] Post-processing effects (bloom, DOF, SSAO)
- [ ] Raycasting for click/hover events
- [ ] Rust↔JavaScript bridge for real-time updates
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
| 0.1.0 | 2024-04-05 | Initial release |

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
