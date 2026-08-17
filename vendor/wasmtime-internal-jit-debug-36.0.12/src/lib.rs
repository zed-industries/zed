//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "gdb_jit_int")]
pub mod gdb_jit_int;

#[cfg(all(feature = "perf_jitdump", target_os = "linux"))]
pub mod perf_jitdump;
