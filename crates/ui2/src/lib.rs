//! # UI – Zed UI Primitives & Components
//!
//! This crate provides a set of UI primitives and components that are used to build all of the elements in Zed's UI.
//!
//! ## Work in Progress
//!
//! This crate is still a work in progress. The initial primitives and components are built for getting all the UI on the screen,
//! much of the state and functionality is mocked or hard codeded, and performance has not been a focus.
//!
//! Expect some inconsistencies from component to component as we work out the best way to build these components.
//!
//! ## Getting Started
//!
//! This is a quick primer to get you started using the UI components.
//!
//! You shouldn't need to construct an element from scratch very often. If you find
//! yourself manually styling things like hover, text colors, etc, you should
//! probably check that there isn't already a base component for whatever you are building.
//!
//! Here is an into to some of the most common elements:
//!
//! ### Text
//!
//! For generic UI text most frequently you will use a [`Label`] component.
//!
//! ```rust
//! use ui2::prelude::*;
//! use ui2::{Label, LabelColor};
//!
//! pub fn render_some_ui_text<V: 'static>() -> impl Component<V> {
//!     div().p_2().child(
//!         Label::new("Hello World")
//!             .color(LabelColor::Muted)
//!     )
//! }
//! ```
//!
//! ### Interactive Elements
//!
//! - Icon: To make an icon interactive, use [`IconButton`].
//! - Button: To make a button interactive, use [`Button`].
//!
//! ## Design Philosophy
//!
//! Work in Progress!
//!

// TODO: Fix warnings instead of supressing.
#![allow(dead_code, unused_variables)]

mod components;
mod elements;
mod elevation;
pub mod prelude;
pub mod settings;
mod static_data;
pub mod utils;

pub use components::*;
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

#[cfg(feature = "stories")]
mod story;
#[cfg(feature = "stories")]
pub use story::*;
