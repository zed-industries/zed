#[cfg(has_host_compiler_backend)]
use crate::runtime::vm::VMContext;
#[cfg(has_host_compiler_backend)]
use std::ptr::NonNull;

#[cfg(has_host_compiler_backend)]
#[link(name = "wasmtime-helpers")]
unsafe extern "C" {
    #[wasmtime_versioned_export_macros::versioned_link]
    pub fn wasmtime_setjmp(
        jmp_buf: *mut *const u8,
        callback: extern "C" fn(*mut u8, NonNull<VMContext>) -> bool,
        payload: *mut u8,
        callee: NonNull<VMContext>,
    ) -> bool;

    #[wasmtime_versioned_export_macros::versioned_link]
    pub fn wasmtime_longjmp(jmp_buf: *const u8) -> !;
}

pub fn lazy_per_thread_init() {
    // unused on Windows
}

cfg_if::cfg_if! {
    if #[cfg(has_native_signals)] {
        pub use super::vectored_exceptions::{TrapHandler, SignalHandler };
    } else {
        pub enum SignalHandler {}
    }
}
