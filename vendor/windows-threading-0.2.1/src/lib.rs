#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![no_std]
#![expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]

mod bindings;
use bindings::*;

mod pool;
pub use pool::*;

extern crate alloc;
use alloc::boxed::Box;
use core::ffi::c_void;

/// Submit the closure to the default thread pool.
///
/// * The closure must have `'static` lifetime as the thread may outlive the lifetime in which `submit` is called.
/// * The closure must be `Send` as it will be sent to another thread for execution.
pub fn submit<F: FnOnce() + Send + 'static>(f: F) {
    // This is safe because the closure has `'static` lifetime.
    unsafe {
        try_submit(core::ptr::null(), f);
    }
}

/// Calls the closure on each element of the iterator in parallel, waiting for all closures to finish.
///
/// * The closure does not require `'static` lifetime since the `for_each` function bounds the lifetime of all submitted closures.
/// * The closure must be `Sync` as multiple threads will refer to it.
/// * The iterator items must be `Send` as they will be sent from one thread to another.
pub fn for_each<I, F, T>(i: I, f: F)
where
    I: Iterator<Item = T>,
    F: Fn(T) + Sync,
    T: Send,
{
    Pool::with_scope(|pool| {
        for item in i {
            pool.submit(|| f(item));
        }
    });
}

/// The thread identifier of the calling thread.
pub fn thread_id() -> u32 {
    unsafe { GetCurrentThreadId() }
}

/// Suspends the execution of the current thread until the time-out interval elapses.
pub fn sleep(milliseconds: u32) {
    unsafe {
        Sleep(milliseconds);
    }
}

// When used correctly, the Windows thread pool APIs only fail when memory is exhausted. This function will cause such failures to `panic`.
fn check<D: Default + PartialEq>(result: D) -> D {
    if result == D::default() {
        panic!("allocation failed");
    }

    result
}

// This function is `unsafe` as it cannot ensure that the lifetime of the closure is sufficient or
// whether the `environment` pointer is valid.
unsafe fn try_submit<F: FnOnce() + Send>(environment: *const TP_CALLBACK_ENVIRON_V3, f: F) {
    unsafe extern "system" fn callback<F: FnOnce() + Send>(
        _: PTP_CALLBACK_INSTANCE,
        callback: *mut c_void,
    ) {
        unsafe {
            Box::from_raw(callback as *mut F)();
        }
    }

    unsafe {
        check(TrySubmitThreadpoolCallback(
            Some(callback::<F>),
            Box::into_raw(Box::new(f)) as _,
            environment,
        ));
    }
}
