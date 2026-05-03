# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.4] - 2026-05-02

### Added

- **`Selection::empty()`** — Convenience constructor alias for `Selection::new()`.
- **Selection outline scaling** — Outline boxes now scale with the selected object on both Desktop and Web by tracking `originalSize` in `userData` and updating `outlineGroup.scale` each frame.
- **Gizmo depth fix** — All gizmo handles now render on top by setting `depthTest=false`, `depthWrite=false`, and `renderOrder=999` on both Desktop (`THREE.TransformControls`) and Web (custom `gizmoGroup`).
- **CI auto-publish** — GitHub Actions now automatically publishes to crates.io and creates a Git tag + GitHub Release when a version bump is merged to `main`.
- **Documentation accuracy pass** — Fixed all `dioxus_three::prelude::*` references to explicit imports, corrected `ModelConfig` field examples (`url` not `model_url`), and aligned `PointerEvent` / `Selection` API docs with actual struct definitions.
- **Docs site structure** — Added `docs/api/README.md` as a landing page for the API reference.

### Changed

- **Desktop outline style** — Removed corner cube markers so the Desktop selection outline now matches the Web implementation (wireframe box + inner glow only).
- **Desktop state updates** — The Desktop iframe now uses `postMessage` for camera, selection, and gizmo updates instead of regenerating the full `srcdoc` HTML on every prop change. HTML is only regenerated when the model count changes.
- **Desktop eval bridge** — Added `e.source` validation and iframe-side `[POINTER]` / `[RAYCAST]` debug logging for safer event propagation.

### Fixed

- **Desktop click selection** — Fixed selection propagation through the eval bridge with proper source validation.
- **Gizmo scale disappearing** — Fixed gizmo handles being occluded by large scaled objects via the depth rendering fix.
- **CI failures** — Fixed `cargo fmt` violations, resolved all `cargo clippy` warnings (`derivable_impls`, `let_unit_value`, `redundant_closure`), and installed missing Ubuntu system dependencies (`libglib2.0-dev`, `libxdo-dev`, `libwebkit2gtk-4.1-dev`, etc.) for the `dioxus-desktop` dev-dependency chain.

## [0.0.3] - 2026-04-06

### Added

- Phase 1 features: Raycasting, Selection, and Transform Gizmos.
- Desktop implementation using iframe with `srcdoc` and `document::eval` bridge.
- Web/WASM implementation rendering to `<canvas>` with custom-built gizmos.
- `ModelConfig` builder API for loading cubes, spheres, and external models.
- `ShaderPreset` system for built-in visual effects.
- Multi-platform support: Desktop, Web (WASM), and Mobile (WebView).
