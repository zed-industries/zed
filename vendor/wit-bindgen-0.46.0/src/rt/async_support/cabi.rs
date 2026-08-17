//! Definition of the "C ABI" of how imported functions interact with exported
//! tasks.
//!
//! Ok this crate is written in Rust, why in the world does this exist? This
//! comment is intended to explain this rationale but the tl;dr; is we want
//! this to work:
//!
//! * Within a single component ...
//! * One rust crate uses `wit-bindgen 0.A.0` to generate an exported function.
//! * One rust crate uses `wit-bindgen 0.B.0` to bind an imported function.
//! * The two crates are connected in the application with
//!   `std::future::Future`.
//!
//! Without this module this situation won't work because 0.A.0 has no
//! knowledge of 0.B.0 meaning that 0.B.0 has no means of inserting a `waitable`
//! into the `waitable-set` managed by 0.A.0's export.
//!
//! To solve this problem the long-term intention is that something will live
//! in `wasi-libc` itself, but in the meantime it's living "somewhere" within
//! `wit-bindgen 0.*.0`. Specifically all `wit-bindgen` versions will
//! reference, via C linkage, a single function which is used to manipulate a
//! single pointer in linear memory. This pointer is a `wasip3_task` structure
//! which has all the various fields to use it.
//!
//! The `wasip3_task_set` symbol is itself defined in C inside of the
//! `src/wit_bindgen_cabi.c` file at this time, specifically because it's
//! annotated with `__weak__` meaning that any definition of it suffices. This
//! isn't possible to define in stable Rust (specifically `__weak__`).
//!
//! Once `wasip3_task_set` is defined everything then operates via indirection,
//! aka based off the returned pointer. The intention is that exported functions
//! will set this (it's sort of like an executor) and then imported functions
//! will all use this as the source of registering waitables. In the end that
//! means that it's possible to share types with `std::future::Future` that
//! are backed at the ABI level with this "channel".
//!
//! In the future it's hoped that this can move into `wasi-libc` itself, or if
//! `wasi-libc` provides something else that would be prioritized over this.
//! For now this is basically an affordance that we're going to be frequently
//! releaseing new major versions of `wit-bindgen` and we don't want to force
//! applications to all be using the exact same version of the bindings
//! generator and async bindings.
//!
//! Additionally for now this file is serving as documentation of this
//! interface.

use core::ffi::c_void;

#[cfg(target_family = "wasm")]
extern "C" {
    /// Sets the global task pointer to `ptr` provided. Returns the previous
    /// value.
    ///
    /// This function acts as a dual getter and a setter. To get the
    /// current task pointer a dummy `ptr` can be provided (e.g. NULL) and then
    /// it's passed back when you're done working with it. When setting the
    /// current task pointer it's recommended to call this and then call it
    /// again with the previous value when the tasks's work is done.
    ///
    /// For executors they need to ensure that the `ptr` passed in lives for
    /// the entire lifetime of the component model task.
    pub fn wasip3_task_set(ptr: *mut wasip3_task) -> *mut wasip3_task;
}

#[cfg(not(target_family = "wasm"))]
pub unsafe extern "C" fn wasip3_task_set(ptr: *mut wasip3_task) -> *mut wasip3_task {
    let _ = ptr;
    unreachable!();
}

/// The first version of `wasip3_task` which implies the existence of the
/// fields `ptr`, `waitable_register`, and `waitable_unregister`.
pub const WASIP3_TASK_V1: u32 = 1;

/// Indirect "vtable" used to connect imported functions and exported tasks.
/// Executors (e.g. exported functions) define and manage this while imports
/// use it.
#[repr(C)]
pub struct wasip3_task {
    /// Currently `WASIP3_TASK_V1`. Indicates what fields are present next
    /// depending on the version here.
    pub version: u32,

    /// Private pointer owned by the `wasip3_task` itself, passed to callbacks
    /// below as the first argument.
    pub ptr: *mut c_void,

    /// Register a new `waitable` for this exported task.
    ///
    /// This exported task will add `waitable` to its `waitable-set`. When it
    /// becomes ready then `callback` will be invoked with the ready code as
    /// well as the `callback_ptr` provided.
    ///
    /// If `waitable` was previously registered with this task then the
    /// previous `callback_ptr` is returned. Otherwise `NULL` is returned.
    ///
    /// It's the caller's responsibility to ensure that `callback_ptr` is valid
    /// until `callback` is invoked, `waitable_unregister` is invoked, or
    /// `waitable_register` is called again to overwrite the value.
    pub waitable_register: unsafe extern "C" fn(
        ptr: *mut c_void,
        waitable: u32,
        callback: unsafe extern "C" fn(callback_ptr: *mut c_void, code: u32),
        callback_ptr: *mut c_void,
    ) -> *mut c_void,

    /// Removes the `waitable` from this task's `waitable-set`.
    ///
    /// Returns the `callback_ptr` passed to `waitable_register` if present, or
    /// `NULL` if it's not present.
    pub waitable_unregister: unsafe extern "C" fn(ptr: *mut c_void, waitable: u32) -> *mut c_void,
}
