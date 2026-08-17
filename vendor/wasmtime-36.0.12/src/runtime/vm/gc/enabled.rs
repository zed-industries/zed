//! Implementation of garbage collection and GC types in Wasmtime.

mod arrayref;
mod data;
mod exnref;
mod externref;
#[cfg(feature = "gc-drc")]
mod free_list;
mod structref;

pub use arrayref::*;
pub use data::*;
pub use exnref::*;
pub use externref::*;
pub use structref::*;

#[cfg(feature = "gc-drc")]
mod drc;
#[cfg(feature = "gc-drc")]
pub use drc::*;

#[cfg(feature = "gc-null")]
mod null;
#[cfg(feature = "gc-null")]
pub use null::*;

// Explicit methods to clearly indicate that truncation is desired when used.
#[expect(
    clippy::cast_possible_truncation,
    reason = "that's the purpose of this method"
)]
fn truncate_i32_to_i16(a: i32) -> i16 {
    a as i16
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "that's the purpose of this method"
)]
fn truncate_i32_to_i8(a: i32) -> i8 {
    a as i8
}
