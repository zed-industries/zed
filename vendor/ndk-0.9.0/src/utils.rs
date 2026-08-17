//! Internal utilities
use log::{error, log_enabled, Level};
use std::ffi::{c_int, CStr, CString};
use std::io::{Error, Result};

/// Turns standard `<errno.h>` status codes - typically rewrapped by Android's [`Errors.h`] - into
/// Rust's [`std::io::Error`].
///
/// [`Errors.h`]: https://cs.android.com/android/platform/superproject/+/master:system/core/libutils/include/utils/Errors.h
pub(crate) fn status_to_io_result(status: i32) -> Result<()> {
    match status {
        0 => Ok(()),
        r if r < 0 => Err(Error::from_raw_os_error(-r)),
        r => unreachable!("Status is positive integer {}", r),
    }
}

pub(crate) fn android_log(level: Level, tag: &CStr, msg: &CStr) {
    let prio = match level {
        Level::Error => ffi::android_LogPriority::ANDROID_LOG_ERROR,
        Level::Warn => ffi::android_LogPriority::ANDROID_LOG_WARN,
        Level::Info => ffi::android_LogPriority::ANDROID_LOG_INFO,
        Level::Debug => ffi::android_LogPriority::ANDROID_LOG_DEBUG,
        Level::Trace => ffi::android_LogPriority::ANDROID_LOG_VERBOSE,
    };
    unsafe {
        ffi::__android_log_write(prio.0 as c_int, tag.as_ptr(), msg.as_ptr());
    }
}

pub(crate) fn log_panic(panic: Box<dyn std::any::Any + Send>) {
    fn log_panic(panic_str: &str) {
        const RUST_PANIC_TAG: &CStr =
            unsafe { CStr::from_bytes_with_nul_unchecked(b"RustPanic\0") };

        let panic_str = CString::new(panic_str).unwrap_or_default();

        // Use the Rust logger if installed and enabled, otherwise fall back to the Android system
        // logger so there is at least some record of the panic
        if log_enabled!(Level::Error) {
            error!("RustPanic: {}", panic_str.to_string_lossy());
            log::logger().flush();
        } else {
            android_log(Level::Error, RUST_PANIC_TAG, &panic_str);
        }
    }

    match panic.downcast::<String>() {
        Ok(panic_string) => log_panic(&panic_string),
        Err(panic) => match panic.downcast::<&str>() {
            Ok(panic_str) => log_panic(&panic_str),
            Err(_) => log_panic("Unknown panic message type"),
        },
    }
}

/// Run a closure and abort the program if it panics.
///
/// This is generally used to ensure Rust callbacks won't unwind past the FFI boundary, which leads
/// to undefined behaviour.
pub(crate) fn abort_on_panic<R>(f: impl FnOnce() -> R) -> R {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_or_else(|panic| {
        // Try logging the panic before aborting
        //
        // Just in case our attempt to log a panic could itself cause a panic we use a
        // second catch_unwind here.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| log_panic(panic)));
        std::process::abort();
    })
}
