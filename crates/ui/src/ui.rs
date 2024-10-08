#![deny(missing_docs)]

//! # UI – Zed UI Primitives & Components
//!
//! This crate provides a set of UI primitives and components that are used to build all of the elements in Zed's UI.
//!
//! ## Related Crates:
//!
//! - [`ui_macros`] - proc_macros support for this crate
//! - [`ui_input`] - the single line input component
//!

mod components;
pub mod prelude;
mod styles;
mod tests;
mod traits;
pub mod utils;

pub use components::*;
pub use prelude::*;
pub use styles::*;
