//! # UI – Zed UI Primitives & Components
//!
//! This crate provides a set of UI primitives and components that are used to build all of the elements in Zed's UI.
//!

mod clickable;
mod components;
mod disableable;
mod fixed;
pub mod prelude;
mod selectable;
mod styled_ext;
mod styles;
pub mod utils;
mod visible_on_hover;

pub use clickable::*;
pub use components::*;
pub use disableable::*;
pub use fixed::*;
pub use prelude::*;

pub use styled_ext::*;
pub use styles::*;
