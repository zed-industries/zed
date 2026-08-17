//! This crate is the implementation of Wasmtime's C API.
//!
//! This crate is normally not intended to be used from Rust itself. For that,
//! see the `wasmtime` crate. It is possible to use this crate via Cargo, for
//! Rust crates that wrap C libraries that use wasmtime. Most often, this crate
//! is compiled as a cdylib or staticlib, via the `wasmtime-c-api` crate.
//!
//! Documentation for this crate largely lives in the header
//! files of the `include` directory for this crate.
//!
//! At a high level this crate implements the `wasm.h` API with some gymnastics,
//! but otherwise an accompanying `wasmtime.h` API is provided which is more
//! specific to Wasmtime and has fewer gymnastics to implement.

#![expect(non_camel_case_types, reason = "matching C style, not Rust")]
#![expect(unsafe_op_in_unsafe_fn, reason = "crate isn't migrated yet")]

pub use wasmtime;

mod config;
mod engine;
mod error;
mod r#extern;
mod func;
mod global;
mod instance;
mod linker;
mod memory;
mod module;
#[cfg(feature = "profiling")]
mod profiling;
mod r#ref;
mod sharedmemory;
mod store;
mod table;
mod trap;
mod types;
mod val;
mod vec;

pub use crate::config::*;
pub use crate::engine::*;
pub use crate::error::*;
pub use crate::r#extern::*;
pub use crate::func::*;
pub use crate::global::*;
pub use crate::instance::*;
pub use crate::linker::*;
pub use crate::memory::*;
pub use crate::module::*;
pub use crate::r#ref::*;
pub use crate::store::*;
pub use crate::table::*;
pub use crate::trap::*;
pub use crate::types::*;
pub use crate::val::*;
pub use crate::vec::*;

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use crate::r#async::*;

#[cfg(feature = "wasi")]
mod wasi;
#[cfg(feature = "wasi")]
pub use crate::wasi::*;

#[cfg(all(feature = "component-model", feature = "wasi"))]
mod wasip2;
#[cfg(all(feature = "component-model", feature = "wasi"))]
pub use crate::wasip2::*;

#[cfg(feature = "wat")]
mod wat2wasm;
#[cfg(feature = "wat")]
pub use crate::wat2wasm::*;

#[cfg(feature = "component-model")]
mod component;
#[cfg(feature = "component-model")]
pub use crate::component::*;

/// Initialize a `MaybeUninit<T>`
///
/// TODO: Replace calls to this function with
/// https://doc.rust-lang.org/nightly/std/mem/union.MaybeUninit.html#method.write
/// once it is stable.
pub(crate) fn initialize<T>(dst: &mut std::mem::MaybeUninit<T>, val: T) {
    unsafe {
        std::ptr::write(dst.as_mut_ptr(), val);
    }
}

/// Helper for running a C-defined finalizer over some data when the Rust
/// structure is dropped.
pub struct ForeignData {
    data: *mut std::ffi::c_void,
    finalizer: Option<extern "C" fn(*mut std::ffi::c_void)>,
}

unsafe impl Send for ForeignData {}
unsafe impl Sync for ForeignData {}

impl Drop for ForeignData {
    fn drop(&mut self) {
        if let Some(f) = self.finalizer {
            f(self.data);
        }
    }
}

/// Helper for creating Rust slices from C inputs.
///
/// This specifically disregards the `ptr` argument if the length is zero. The
/// `ptr` in that case maybe `NULL` or invalid, and it's not valid to have a
/// zero-length Rust slice with a `NULL` pointer.
unsafe fn slice_from_raw_parts<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    if len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(ptr, len)
    }
}

/// Same as above, but for `*_mut`
unsafe fn slice_from_raw_parts_mut<'a, T>(ptr: *mut T, len: usize) -> &'a mut [T] {
    if len == 0 {
        &mut []
    } else {
        std::slice::from_raw_parts_mut(ptr, len)
    }
}

pub(crate) fn abort(name: &str) -> ! {
    eprintln!("`{name}` is not implemented");
    std::process::abort();
}
