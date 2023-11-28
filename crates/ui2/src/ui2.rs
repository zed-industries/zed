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
// TODO: Fix warnings instead of supressing.
#![allow(dead_code, unused_variables)]

mod clickable;
mod components;
mod fixed;
pub mod prelude;
mod selectable;
mod styled_ext;
mod styles;
pub mod utils;

pub use clickable::*;
pub use components::*;
pub use fixed::*;
pub use prelude::*;
pub use selectable::*;
pub use styled_ext::*;
pub use styles::*;
