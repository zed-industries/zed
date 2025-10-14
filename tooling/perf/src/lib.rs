//! Some constants and datatypes used in the Zed perf profiler. Should only be
//! consumed by the crate providing the matching macros.
//!
//! For usage documentation, see the docs on this crate's binary.

/// The implementation of the this crate is kept in a separate module
/// so that it is easy to publish this crate as part of GPUI's dependencies
mod implementation;
pub use implementation::*;
