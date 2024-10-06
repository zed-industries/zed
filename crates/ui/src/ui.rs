//! # UI – Zed UI Primitives & Components
//!
//! This crate provides a set of UI primitives and components that are used to build all of the elements in Zed's UI.
//!

mod components;
mod path_str_tests;
pub mod prelude;
mod styled_ext;
mod styles;
mod traits;
pub mod utils;
mod with_rem_size;

pub use components::*;
pub use prelude::*;
pub use styled_ext::*;
pub use styles::*;
pub use with_rem_size::*;
