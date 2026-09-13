//! GPUI's rendering engine: scene graph, sprite atlas identifiers, and the
//! renderer contract.
//!
//! This crate holds the frame representation that `gpui` builds and the
//! platform backends present. It deliberately has no dependency on the
//! windowing layer so alternative engines can reuse or replace the scene
//! representation.

#![warn(missing_docs)]

mod atlas;
mod bounds_tree;
mod font_fallbacks;
mod font_features;
mod layout;
mod line_layout;
mod line_wrapper;
mod render;
mod renderer;
mod scene;
mod text;
mod text_system;

pub use atlas::*;
pub use font_fallbacks::*;
pub use font_features::*;
pub use layout::*;
pub use line_layout::*;
pub use line_wrapper::*;
pub use render::*;
pub use renderer::*;
pub use scene::*;
pub use text::*;
pub use text_system::*;
