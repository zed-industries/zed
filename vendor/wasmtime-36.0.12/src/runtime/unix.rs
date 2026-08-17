//! Unix-specific extension for the `wasmtime` crate.
//!
//! This module is only available on Unix targets, for example Linux. Note that
//! this module is notably not available on macOS or Windows.  Note that the
//! import path for this module is `wasmtime::unix::...`, which is intended to
//! emphasize that it is platform-specific.
//!
//! The traits contained in this module are intended to extend various types
//! throughout the `wasmtime` crate with extra functionality that's only
//! available on Unix.

#[cfg(has_native_signals)]
use crate::AsContextMut;
use crate::Store;
#[cfg(has_native_signals)]
use crate::prelude::*;

/// Extensions for the [`Store`] type only available on Unix.
pub trait StoreExt {
    // TODO: needs more docs?
    /// The signal handler must be
    /// [async-signal-safe](http://man7.org/linux/man-pages/man7/signal-safety.7.html).
    #[cfg(has_native_signals)]
    unsafe fn set_signal_handler<H>(&mut self, handler: H)
    where
        H: 'static
            + Fn(libc::c_int, *const libc::siginfo_t, *const libc::c_void) -> bool
            + Send
            + Sync;
}

impl<T> StoreExt for Store<T> {
    #[cfg(has_native_signals)]
    unsafe fn set_signal_handler<H>(&mut self, handler: H)
    where
        H: 'static
            + Fn(libc::c_int, *const libc::siginfo_t, *const libc::c_void) -> bool
            + Send
            + Sync,
    {
        self.as_context_mut()
            .0
            .set_signal_handler(Some(Box::new(handler)));
    }
}
