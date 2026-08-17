//! "Dummy" implementations of some system primitives for MIRI emulation.
//!
//! Note that at this time this is just enough to run some tests in MIRI but
//! notably WebAssembly tests are not executed at this time (MIRI can't execute
//! Cranelift-generated code).

use std::cell::Cell;

pub mod mmap;
pub mod traphandlers;
pub mod unwind;
pub mod vm;

std::thread_local!(static TLS: Cell<*mut u8> = const { Cell::new(std::ptr::null_mut()) });

#[inline]
pub fn tls_get() -> *mut u8 {
    TLS.with(|p| p.get())
}

#[inline]
pub fn tls_set(ptr: *mut u8) {
    TLS.with(|p| p.set(ptr));
}
