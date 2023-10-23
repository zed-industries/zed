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
//! - [ThemeColor](crate::color::ThemeColor) is your one stop shop for all colors in the UI.
//!
//! ## Design Philosophy
//!
//! Work in Progress!
//!

#![allow(dead_code, unused_variables)]

mod color;
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
