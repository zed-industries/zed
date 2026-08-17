//! Implementation of Wasmtime's system primitives for Windows.

use std::cell::Cell;

#[cfg(has_virtual_memory)]
pub mod mmap;
pub mod traphandlers;
#[cfg(has_native_signals)]
mod vectored_exceptions;
pub mod vm;

#[cfg(all(target_pointer_width = "64", has_host_compiler_backend))]
pub mod unwind64;
#[cfg(all(target_pointer_width = "64", has_host_compiler_backend))]
pub use unwind64 as unwind;

#[cfg(all(not(target_pointer_width = "64"), has_host_compiler_backend))]
compile_error!("don't know how to unwind non-64 bit platforms");

std::thread_local!(static TLS: Cell<*mut u8> = const { Cell::new(std::ptr::null_mut()) });

#[inline]
pub fn tls_get() -> *mut u8 {
    TLS.with(|p| p.get())
}

#[inline]
pub fn tls_set(ptr: *mut u8) {
    TLS.with(|p| p.set(ptr));
}
