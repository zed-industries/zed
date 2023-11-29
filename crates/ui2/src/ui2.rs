//! # UI – Zed UI Primitives & Components
//!
//! This crate provides a set of UI primitives and components that are used to build all of the elements in Zed's UI.
//!
//! ## Work in Progress
//!
//! This crate is still a work in progress. The initial primitives and components are built for getting all the UI on the screen,
//! much of the state and functionality is mocked or hard codeded, and performance has not been a focus.
//!

#![doc = include_str!("../docs/hello-world.md")]
#![doc = include_str!("../docs/building-ui.md")]
#![doc = include_str!("../docs/todo.md")]

mod clickable;
mod components;
mod disableable;
mod fixed;
pub mod prelude;
mod selectable;
mod slot;
mod styled_ext;
mod styles;
mod toggleable;
pub mod utils;

pub use clickable::*;
pub use components::*;
pub use disableable::*;
pub use fixed::*;
pub use prelude::*;
pub use selectable::*;
pub use slot::*;
pub use styled_ext::*;
pub use styles::*;
pub use toggleable::*;
