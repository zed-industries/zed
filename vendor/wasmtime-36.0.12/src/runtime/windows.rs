//! windows-specific extension for the `wasmtime` crate.
//!
//! This module is only available on Windows targets.
//! It is not available on Linux or macOS, for example. Note that the import path for
//! this module is `wasmtime::windows::...`, which is intended to emphasize that it
//! is platform-specific.
//!
//! The traits contained in this module are intended to extend various types
//! throughout the `wasmtime` crate with extra functionality that's only
//! available on Windows.

#[cfg(has_native_signals)]
use crate::AsContextMut;
use crate::Store;
#[cfg(has_native_signals)]
use crate::prelude::*;
#[cfg(has_native_signals)]
use windows_sys::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS;

/// Extensions for the [`Store`] type only available on Windows.
pub trait StoreExt {
    /// Configures a custom signal handler to execute.
    ///
    /// TODO: needs more documentation.
    #[cfg(has_native_signals)]
    unsafe fn set_signal_handler<H>(&mut self, handler: H)
    where
        H: 'static + Fn(*mut EXCEPTION_POINTERS) -> bool + Send + Sync;
}

impl<T> StoreExt for Store<T> {
    #[cfg(has_native_signals)]
    unsafe fn set_signal_handler<H>(&mut self, handler: H)
    where
        H: 'static + Fn(*mut EXCEPTION_POINTERS) -> bool + Send + Sync,
    {
        self.as_context_mut()
            .0
            .set_signal_handler(Some(Box::new(handler)));
    }
}
