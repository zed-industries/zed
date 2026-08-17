//! The dummy implementation of garbage-collection, for when the `gc` Cargo
//! feature is disabled.
//!
//! We provide dummy/uninhabited types so that downstream users (and the rest of
//! Wasmtime) need to do fewer `#[cfg(...)]`s for when GC is enabled versus
//! disabled at compile time. While we implement dummy methods for these types'
//! public methods, we do not, however, create dummy constructors constructors.

#![expect(missing_docs, unreachable_code, reason = "dummy module")]

mod anyref;
mod arrayref;
mod eqref;
mod exnref;
mod externref;
mod i31;
mod rooting;
mod structref;

pub use anyref::*;
pub use arrayref::*;
pub use eqref::*;
pub use exnref::*;
pub use externref::*;
pub use i31::*;
pub use rooting::*;
pub use structref::*;
