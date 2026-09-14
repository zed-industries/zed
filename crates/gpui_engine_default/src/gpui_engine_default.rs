//! GPUI's default engine: Taffy-backed layout evaluation and the text shaping
//! and wrapping caches the facade drives.
//!
//! The scene representation and the engine's interface contracts live in
//! [`gpui_engine`]; this crate holds the concrete implementations that depend
//! on `taffy`.

#![warn(missing_docs)]

mod engine;
mod layout;
mod layout_style;
mod line_layout;
mod line_wrapper;
mod text_system;

pub use layout::*;
pub use line_layout::*;
pub use line_wrapper::*;
pub use text_system::*;

/// Engine internals consumed by the `gpui` facade. Not a stable API.
#[doc(hidden)]
pub mod __private {
    pub use crate::engine::FrameSession;
}
