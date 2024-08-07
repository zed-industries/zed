//! # UI – Zed UI Primitives & Components
//!
//! This crate provides a set of UI primitives and components that are used to build all of the elements in Zed's UI.
//!

mod clickable;
mod components;
mod disableable;
mod fixed;
mod key_bindings;
pub mod prelude;
mod selectable;
mod styled_ext;
mod styles;
pub mod utils;
mod visible_on_hover;
mod with_rem_size;

pub use clickable::*;
pub use components::*;
pub use disableable::*;
pub use fixed::*;
pub use key_bindings::*;
pub use prelude::*;
pub use styled_ext::*;
pub use styles::*;
pub use with_rem_size::*;
