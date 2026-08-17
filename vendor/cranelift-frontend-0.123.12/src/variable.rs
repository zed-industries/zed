//! A basic `Variable` implementation.
//!
//! Frontends can use any indexing scheme they see fit and
//! generate the appropriate `Variable` instances.
//!
//! Note: The `Variable` is used by Cranelift to index into densely allocated
//! arrays containing information about your mutable variables
//! Thus, make sure that Variable's indexes are allocated contiguously and
//! starting at `0`.

use core::u32;
use cranelift_codegen::entity::entity_impl;

/// An opaque reference to a variable.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Variable(u32);

entity_impl!(Variable, "var");
