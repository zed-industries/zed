#![allow(unsafe_code)]

#[cfg(target_pointer_width = "32")]
compile_error!("Various MacOS FFI bindings assume we are on a 64-bit architechture");

/// Re-export of the mach2 library for users who want to call mach specific functions
pub use mach2;

pub mod errors;
pub mod mach;
pub mod minidump_writer;
mod streams;
pub mod task_dumper;
