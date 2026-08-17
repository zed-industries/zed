//! Examples of output of the [`generate!`] macro.
//!
//! This module is only included in docs.rs documentation and is not present in
//! the actual crate when compiling from crates.io. The purpose of this module
//! is to showcase what the output of the [`generate!`] macro looks like.
//!
//! [`generate!`]: crate::generate

/// An example of generated bindings for top-level imported functions and
/// interfaces into a world.
///
/// The code used to generate this module is:
///
/// ```rust
#[doc = include_str!("./examples/_0_world_imports.rs")]
/// ```
pub mod _0_world_imports;

/// An example of importing interfaces into a world.
///
/// The code used to generate this module is:
///
/// ```rust
#[doc = include_str!("./examples/_1_interface_imports.rs")]
/// ```
pub mod _1_interface_imports;

/// An example of importing resources into a world.
///
/// The code used to generate this module is:
///
/// ```rust
#[doc = include_str!("./examples/_2_imported_resources.rs")]
/// ```
pub mod _2_imported_resources;

/// An example of exporting items from a world and the traits that they
/// generate.
///
/// The code used to generate this module is:
///
/// ```rust
#[doc = include_str!("./examples/_3_world_exports.rs")]
/// ```
pub mod _3_world_exports;

/// An example of exporting resources from a world and the traits that they
/// generate.
///
/// The code used to generate this module is:
///
/// ```rust
#[doc = include_str!("./examples/_4_exported_resources.rs")]
/// ```
pub mod _4_exported_resources;
