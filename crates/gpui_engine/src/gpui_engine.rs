//! GPUI's engine contract: the scene representation that `gpui` builds and the
//! platform backends present.
//!
//! This crate holds the intermediate representation (the [`Scene`] and its draw
//! primitives) together with the interfaces its collaborators implement. It
//! deliberately has no dependency on the windowing layer, on `taffy`, or on any
//! GPU driver, so alternative renderers and engines can reuse or replace the
//! scene representation without depending on those implementations.

#![warn(missing_docs)]

mod atlas;
mod bounds_tree;
mod font_fallbacks;
mod font_features;
mod frame_session;
mod layout;
mod line_wrapper;
mod render;
mod renderer;
mod scene;
mod style;
mod text;
mod text_system;

pub use atlas::*;
pub use font_fallbacks::*;
pub use font_features::*;
pub use frame_session::*;
pub use layout::*;
pub use line_wrapper::*;
pub use render::*;
pub use renderer::*;
pub use scene::*;
pub use style::*;
pub use text::*;
pub use text_system::*;
