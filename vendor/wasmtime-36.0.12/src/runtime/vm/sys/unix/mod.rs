//! Implementation of Wasmtime's system primitives for Unix-like operating
//! systems.
//!
//! This module handles Linux and macOS for example.

use core::cell::Cell;

#[cfg(has_virtual_memory)]
pub mod mmap;
pub mod traphandlers;
#[cfg(has_host_compiler_backend)]
pub mod unwind;
#[cfg(has_virtual_memory)]
pub mod vm;

#[cfg(all(has_native_signals, target_vendor = "apple"))]
pub mod machports;
#[cfg(has_native_signals)]
pub mod signals;

std::thread_local!(static TLS: Cell<*mut u8> = const { Cell::new(std::ptr::null_mut()) });

#[inline]
pub fn tls_get() -> *mut u8 {
    TLS.with(|p| p.get())
}

#[inline]
pub fn tls_set(ptr: *mut u8) {
    TLS.with(|p| p.set(ptr));
}
