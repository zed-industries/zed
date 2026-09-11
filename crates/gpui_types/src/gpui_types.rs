//! Core API primitive types shared between GPUI and its platform backends.
//!
//! This crate sits at the bottom of GPUI's dependency graph. It holds the pure
//! value types — geometry (`Pixels`, `Point`, `Size`, `Bounds`, …) and colors
//! (`Rgba`, `Hsla`, `Background`, …) — that both the `gpui` crate and the
//! platform implementations depend on.
//!
//! Keeping these types here means a platform backend can depend on the API
//! primitives without depending on all of `gpui`. The `gpui` crate re-exports
//! everything in this crate, so `gpui::Pixels` and `gpui_types::Pixels` name
//! the same type and consumers do not need to know this crate exists.

#![warn(missing_docs)]

mod color;
mod geometry;
mod layout;

#[cfg(feature = "lyon")]
mod lyon_bridge;

pub use color::*;
pub use geometry::*;
pub use layout::*;
