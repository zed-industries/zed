// For MIRI, there's no way to implement longjmp/setjmp. The only possible way
// to implement this is with panic/catch_panic, but the entrypoint into Rust
// from wasm is defined as `extern "C"` which isn't allowed to panic. That
// means that panicking here triggers UB which gets routed to `libc::abort()`.
//
// This maens that on MIRI all tests which trap are configured to be skipped at
// this time.
//
// Note that no actual JIT code runs in MIRI so this is purely here for
// host-to-host calls.

use crate::prelude::*;
use crate::runtime::vm::VMContext;
use core::ptr::NonNull;

pub unsafe fn wasmtime_setjmp(
    _jmp_buf: *mut *const u8,
    callback: extern "C" fn(*mut u8, NonNull<VMContext>) -> bool,
    payload: *mut u8,
    callee: NonNull<VMContext>,
) -> bool {
    callback(payload, callee)
}

pub unsafe fn wasmtime_longjmp(_jmp_buf: *const u8) -> ! {
    unsafe {
        libc::abort();
    }
}

#[allow(missing_docs)]
pub type SignalHandler = Box<dyn Fn() + Send + Sync>;

pub fn lazy_per_thread_init() {}
