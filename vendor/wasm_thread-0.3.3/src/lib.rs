#![cfg_attr(target_arch = "wasm32", feature(stdarch_wasm_atomic_wait))]

// Import reusable APIs from std
pub use std::thread::{current, sleep, Result, Thread, ThreadId};

#[cfg(target_arch = "wasm32")]
mod wasm32;

#[cfg(not(target_arch = "wasm32"))]
pub use std::thread::*;

#[cfg(target_arch = "wasm32")]
pub use wasm32::*;
