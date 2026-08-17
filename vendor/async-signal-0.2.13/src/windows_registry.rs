//! Signal registration for Windows.
//!
//! Windows on its own does not have a concept of signals; signal handling is instead done by the
//! Windows CRT, which Rust does not typically interact with. `signal-hook-registry` works by
//! interfacing with the CRT, which I'd argue is somewhat of an anti-pattern in Rust. Instead,
//! for this crate, let's take a look at the signals that Windows does have and how we can
//! leverage them without the CRT.
//!
//! The six signals that Windows defines are SIGINT, SIGTERM, SIGABRT, SIGILL, SIGSEGV, and
//! SIGFPE. The CRT will return an error if you try to use any other signals. Of these six, only
//! SIGINT is usable for real life purposes.
//!
//! - SIGILL, SIGSEGV and SIGFPE are serious errors that are intended to crash the program. Users who
//!   listen for this signals will need to often run raw C functions (`longjmp`, direct address
//!   manipulation, etc etc) to accurately handle these signals. In other words, these aren't something
//!   that can really be handled using our `async` strategy!
//! - SIGABRT is internal to the CRT and is only raised when `libc::abort()` is called. This makes it
//!   somewhat of a futile exercise to listen for it in Rust, since `std::process::abort()` will
//!   not raise `SIGABRT` and will just abort the process directly.
//! - SIGTERM is never actually raised by Windows outside of the `libc::raise()` function.
//! - SIGINT corresponds to the `CTRL_C_EVENT` event handled by the `SetConsoleCtrlHandler` function.
//!   This is probably also the only signal that is actually useful to listen for.
//!
//! Therefore, all we need to do to properly handle signals on Windows is to just listen for the
//! `CTRL_C_EVENT` event. This is done by calling `SetConsoleCtrlHandler` with a callback function
//! that iterates through a linked list of registered callbacks and calls them.

use async_lock::OnceCell;
use slab::Slab;
use windows_sys::core::BOOL;
use windows_sys::Win32::System::Console::{SetConsoleCtrlHandler, CTRL_C_EVENT};

use std::io::Result;
use std::mem;
use std::os::raw::c_int;
use std::sync::Mutex;

use super::signum::SIGINT;

/// The ID of a signal handler.
pub(crate) type SigId = usize;

/// Register a handler into the global registry.
///
/// # Safety
///
/// This function is safe; it is only unsafe for consistency.
pub(crate) unsafe fn register(
    signal: c_int,
    handler: impl Fn() + Send + Sync + 'static,
) -> Result<SigId> {
    // If this signal isn't SIGINT, then we can't register it.
    if signal != SIGINT {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "unsupported signal",
        ));
    }

    // Register the handler into the global registry.
    Ok(Registry::get()?.register(handler))
}

/// Deregister a handler from the global registry.
pub fn unregister(id: SigId) {
    if let Ok(registry) = Registry::get() {
        registry.unregister(id)
    }
}

/// The global registry of signal handlers.
struct Registry {
    /// The list of signal handlers.
    handlers: Mutex<Slab<Handler>>,
}

/// A closure that handles a signal.
type Handler = Box<dyn Fn() + Send + Sync + 'static>;

impl Registry {
    /// Get the global instance of the registry.
    fn get() -> Result<&'static Self> {
        static REGISTRY: OnceCell<Registry> = OnceCell::new();

        REGISTRY.get_or_try_init_blocking(|| {
            // Register ourselves into the global registry.
            let res = unsafe { SetConsoleCtrlHandler(Some(Self::handle_event), true as _) };

            if res == 0 {
                return Err(std::io::Error::last_os_error());
            }

            Ok(Registry {
                handlers: Mutex::new(Slab::new()),
            })
        })
    }

    /// Handle a console control event.
    unsafe extern "system" fn handle_event(event: u32) -> BOOL {
        // Make sure panics aren't transmitted across the FFI boundary.
        struct AbortOnDrop;

        impl Drop for AbortOnDrop {
            fn drop(&mut self) {
                std::process::abort();
            }
        }

        let _abort_on_drop = AbortOnDrop;

        // If the event is CTRL_C_EVENT, then we should handle it.
        if event == CTRL_C_EVENT {
            // Get the global registry.
            let registry = match Self::get() {
                Ok(registry) => registry,
                Err(_) => return false as BOOL,
            };

            // Note that Windows runs these handlers in another thread, so there's no need to
            // worry about async signal safety.
            let handlers = registry.handlers.lock().unwrap_or_else(|e| e.into_inner());

            for handler in handlers.iter() {
                (handler.1)();
            }

            mem::forget(_abort_on_drop);
            return true as BOOL;
        }

        mem::forget(_abort_on_drop);
        false as BOOL
    }

    /// Register a handler for a signal.
    fn register(&self, handler: impl Fn() + Send + Sync + 'static) -> usize {
        self.handlers
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(Box::new(handler))
    }

    /// Unregister a handler for a signal.
    fn unregister(&self, id: usize) {
        self.handlers
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .try_remove(id);
    }
}
