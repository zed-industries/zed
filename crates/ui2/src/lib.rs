#![allow(dead_code, unused_variables)]

mod components;
mod element_ext;
mod elements;
pub mod prelude;
pub mod settings;
mod static_data;
mod theme;

pub use components::*;
pub use element_ext::*;
pub use elements::*;
pub use prelude::*;
pub use static_data::*;

// This needs to be fully qualified with `crate::` otherwise we get a panic
// at:
//   thread '<unnamed>' panicked at crates/gpui2/src/platform/mac/platform.rs:66:81:
//   called `Option::unwrap()` on a `None` value
//
// AFAICT this is something to do with conflicting names between crates and modules that
// interfaces with declaring the `ClassDecl`.
pub use crate::settings::*;
pub use crate::theme::*;

#[cfg(feature = "stories")]
mod story;
#[cfg(feature = "stories")]
pub use story::*;
