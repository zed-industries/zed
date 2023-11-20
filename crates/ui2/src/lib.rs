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

mod components;
mod elevation;
pub mod prelude;
pub mod settings;
mod static_data;
mod styled_ext;
mod to_extract;
pub mod utils;

pub use components::*;
use gpui::actions;
pub use prelude::*;
pub use static_data::*;
pub use styled_ext::*;
pub use to_extract::*;

// This needs to be fully qualified with `crate::` otherwise we get a panic
// at:
//   thread '<unnamed>' panicked at crates/gpui2/src/platform/mac/platform.rs:66:81:
//   called `Option::unwrap()` on a `None` value
//
// AFAICT this is something to do with conflicting names between crates and modules that
// interfaces with declaring the `ClassDecl`.
pub use crate::settings::*;

#[cfg(feature = "stories")]
mod story;
#[cfg(feature = "stories")]
pub use story::*;
actions!(NoAction);

pub fn binding(key: &str) -> gpui::KeyBinding {
    gpui::KeyBinding::new(key, NoAction {}, None)
}
