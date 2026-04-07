//! Desktop implementation of ThreeView using WebView iframe

use crate::{generate_three_js_html, ThreeViewProps};
use dioxus::prelude::*;

/// A Three.js 3D viewer component for Dioxus Desktop
///
/// Uses a WebView iframe to render the Three.js scene.
/// Supports all features including multiple models, shaders, and animations.
#[component]
pub fn ThreeView(props: ThreeViewProps) -> Element {
    let html = generate_three_js_html(&props);

    rsx! {
        iframe {
            class: "{props.class}",
            style: "width: 100%; height: 100%; border: none;",
            srcdoc: "{html}",
        }
    }
}
