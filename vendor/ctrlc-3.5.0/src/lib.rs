// Copyright (c) 2017 CtrlC developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#![warn(missing_docs)]

//! Cross platform handling of Ctrl-C signals.
//!
//! [HandlerRoutine]:https://msdn.microsoft.com/en-us/library/windows/desktop/ms683242.aspx
//!
//! [set_handler()](fn.set_handler.html) allows setting a handler closure which is executed on
//! `Ctrl+C`. On Unix, this corresponds to a `SIGINT` signal. On windows, `Ctrl+C` corresponds to
//! [`CTRL_C_EVENT`][HandlerRoutine] or [`CTRL_BREAK_EVENT`][HandlerRoutine].
//!
//! Setting a handler will start a new dedicated signal handling thread where we
//! execute the handler each time we receive a `Ctrl+C` signal. There can only be
//! one handler, you would typically set one at the start of your program.
//!
//! # Example
//! ```no_run
//! # #[allow(clippy::needless_doctest_main)]
//! use std::sync::atomic::{AtomicBool, Ordering};
//! use std::sync::Arc;
//!
//! fn main() {
//!     let running = Arc::new(AtomicBool::new(true));
//!     let r = running.clone();
//!
//!     ctrlc::set_handler(move || {
//!         r.store(false, Ordering::SeqCst);
//!     }).expect("Error setting Ctrl-C handler");
//!
//!     println!("Waiting for Ctrl-C...");
//!     while running.load(Ordering::SeqCst) {}
//!     println!("Got it! Exiting...");
//! }
//! ```
//!
//! # Handling SIGTERM and SIGHUP
//! Handling of `SIGTERM` and `SIGHUP` can be enabled with `termination` feature. If this is enabled,
//! the handler specified by `set_handler()` will be executed for `SIGINT`, `SIGTERM` and `SIGHUP`.
//!

#[macro_use]

mod error;
mod platform;
pub use platform::Signal;
mod signal;
pub use signal::*;

pub use error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;

static INIT: AtomicBool = AtomicBool::new(false);
static INIT_LOCK: Mutex<()> = Mutex::new(());

/// Register signal handler for Ctrl-C.
///
/// Starts a new dedicated signal handling thread. Should only be called once,
/// typically at the start of your program.
///
/// # Example
/// ```no_run
/// ctrlc::set_handler(|| println!("Hello world!")).expect("Error setting Ctrl-C handler");
/// ```
///
/// # Warning
/// On Unix, the handler registration for `SIGINT`, (`SIGTERM` and `SIGHUP` if termination feature
/// is enabled) or `SA_SIGINFO` posix signal handlers will be overwritten. On Windows, multiple
/// handler routines are allowed, but they are called on a last-registered, first-called basis
/// until the signal is handled.
///
/// ctrlc::try_set_handler will error (on Unix) if another signal handler exists for the same
/// signal(s) that ctrlc is trying to attach the handler to.
///
/// On Unix, signal dispositions and signal handlers are inherited by child processes created via
/// `fork(2)` on, but not by child processes created via `execve(2)`.
/// Signal handlers are not inherited on Windows.
///
/// # Errors
/// Will return an error if a system error occurred while setting the handler.
///
/// # Panics
/// Any panic in the handler will not be caught and will cause the signal handler thread to stop.
pub fn set_handler<F>(user_handler: F) -> Result<(), Error>
where
    F: FnMut() + 'static + Send,
{
    init_and_set_handler(user_handler, true)
}

/// The same as ctrlc::set_handler but errors if a handler already exists for the signal(s).
///
/// # Errors
/// Will return an error if another handler exists or if a system error occurred while setting the
/// handler.
pub fn try_set_handler<F>(user_handler: F) -> Result<(), Error>
where
    F: FnMut() + 'static + Send,
{
    init_and_set_handler(user_handler, false)
}

fn init_and_set_handler<F>(user_handler: F, overwrite: bool) -> Result<(), Error>
where
    F: FnMut() + 'static + Send,
{
    if !INIT.load(Ordering::Acquire) {
        let _guard = INIT_LOCK.lock().unwrap();

        if !INIT.load(Ordering::Relaxed) {
            set_handler_inner(user_handler, overwrite)?;
            INIT.store(true, Ordering::Release);
            return Ok(());
        }
    }

    Err(Error::MultipleHandlers)
}

fn set_handler_inner<F>(mut user_handler: F, overwrite: bool) -> Result<(), Error>
where
    F: FnMut() + 'static + Send,
{
    unsafe {
        platform::init_os_handler(overwrite)?;
    }

    thread::Builder::new()
        .name("ctrl-c".into())
        .spawn(move || loop {
            unsafe {
                platform::block_ctrl_c().expect("Critical system error while waiting for Ctrl-C");
            }
            user_handler();
        })
        .map_err(Error::System)?;

    Ok(())
}
