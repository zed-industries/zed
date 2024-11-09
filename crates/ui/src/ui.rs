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

mod component_registry;
mod components;
pub mod prelude;
mod styles;
mod tests;
mod traits;
pub mod utils;

pub use component_registry::{get_all_component_previews, init_component_registry};
pub use components::*;
pub use prelude::*;
pub use styles::*;

pub(crate) mod internal {
    /// A crate-internal extension of the prelude, used to expose the crate-specific
    /// needs like the component registry or component-preview types
    pub mod prelude {
        pub use crate::prelude::*;
        pub use crate::register_components;
        pub use crate::traits::component_preview::*;
    }
}
