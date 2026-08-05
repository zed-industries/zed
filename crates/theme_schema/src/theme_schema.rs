//! Serialized theme schema and runtime refinement.
//!
//! This crate owns Zed's JSON theme contract and converts serialized theme
//! families into the runtime types provided by the `theme` crate.

mod loader;
mod schema;

pub use loader::*;
pub use schema::*;
