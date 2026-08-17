//! Internal dependency of the `wasmtime` crate.
//!
//! This crate is responsible for defining types and basic runtime structures
//! used by the `wasmtime` crate. This additionally defines primitives of
//! compilation and what compilers are expected to emit.
//!
//! If you don't already know what this crate is you probably want to use
//! `wasmtime`, not this crate.

#![deny(missing_docs)]
#![warn(clippy::cast_sign_loss)]
#![no_std]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;
extern crate alloc;

pub mod prelude;

mod address_map;
#[macro_use]
mod builtin;
mod demangling;
mod error;
mod ext;
mod gc;
mod hostcall;
mod module;
mod module_artifacts;
mod module_types;
pub mod obj;
mod ref_bits;
mod scopevec;
mod stack_map;
mod stack_switching;
mod trap_encoding;
mod tunables;
mod types;
mod vmoffsets;

pub use self::ext::*;
pub use crate::address_map::*;
pub use crate::builtin::*;
pub use crate::demangling::*;
pub use crate::error::*;
pub use crate::gc::*;
pub use crate::hostcall::*;
pub use crate::module::*;
pub use crate::module_artifacts::*;
pub use crate::module_types::*;
pub use crate::ref_bits::*;
pub use crate::scopevec::ScopeVec;
pub use crate::stack_map::*;
pub use crate::stack_switching::*;
pub use crate::trap_encoding::*;
pub use crate::tunables::*;
pub use crate::types::*;
pub use crate::vmoffsets::*;
pub use object;

pub use wasmparser;

#[cfg(feature = "compile")]
mod compile;
#[cfg(feature = "compile")]
pub use crate::compile::*;

#[cfg(feature = "component-model")]
pub mod component;
#[cfg(all(feature = "component-model", feature = "compile"))]
pub mod fact;

// Reexport all of these type-level since they're quite commonly used and it's
// much easier to refer to everything through one crate rather than importing
// one of three and making sure you're using the right one.
pub use cranelift_entity::*;

/// Version number of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
