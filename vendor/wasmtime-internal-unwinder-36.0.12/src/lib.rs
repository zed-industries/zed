//! Wasmtime unwinder.
//!
//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.

#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "cranelift")]
extern crate alloc;

mod stackwalk;
pub use stackwalk::*;
mod arch;
#[allow(
    unused_imports,
    reason = "`arch` is intentionally an empty module on some platforms"
)]
pub use arch::*;
mod exception_table;
pub use exception_table::*;
mod throw;
pub use throw::*;
