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
mod render;
mod renderer;
mod scene;

pub use atlas::*;
pub use render::*;
pub use renderer::*;
pub use scene::*;
