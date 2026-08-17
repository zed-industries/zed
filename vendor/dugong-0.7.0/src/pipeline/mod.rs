//! Dagre layout pipelines.
//!
//! We keep the crate-level API as `dugong::layout(...)`, so this module is intentionally not
//! named `layout` to avoid a Rust item-name conflict.

mod minimal;

pub use minimal::layout;

#[cfg(feature = "dagreish")]
mod compound;
#[cfg(feature = "dagreish")]
mod dagreish;
#[cfg(feature = "dagreish")]
pub use dagreish::layout_dagreish;
