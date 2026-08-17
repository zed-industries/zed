// Copyright (c) 2017 CtrlC developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use crate::error::Error as CtrlcError;

#[cfg(not(target_vendor = "apple"))]
#[allow(static_mut_refs)] // rust-version = "1.69.0"
mod implementation {
    static mut SEMAPHORE: nix::libc::sem_t = unsafe { std::mem::zeroed() };
    const SEM_THREAD_SHARED: nix::libc::c_int = 0;

    pub unsafe fn sem_init() {
        nix::libc::sem_init(&mut SEMAPHORE as *mut _, SEM_THREAD_SHARED, 0);
    }

    pub unsafe fn sem_post() {
        // No errors apply. EOVERFLOW is hypothetically possible but it's equivalent to a success for our oneshot use-case.
        let _ = nix::libc::sem_post(&mut SEMAPHORE as *mut _);
    }

    pub unsafe fn sem_wait_forever() {
        // The only realistic error is EINTR, which is restartable.
        while nix::libc::sem_wait(&mut SEMAPHORE as *mut _) == -1 {}
    }
}

#[cfg(target_vendor = "apple")]
mod implementation {
    static mut SEMAPHORE: dispatch::ffi::dispatch_semaphore_t = std::ptr::null_mut();

    pub unsafe fn sem_init() {
        SEMAPHORE = dispatch::ffi::dispatch_semaphore_create(0);
    }

    pub unsafe fn sem_post() {
        dispatch::ffi::dispatch_semaphore_signal(SEMAPHORE);
    }

    pub unsafe fn sem_wait_forever() {
        dispatch::ffi::dispatch_semaphore_wait(SEMAPHORE, dispatch::ffi::DISPATCH_TIME_FOREVER);
    }
}

/// Platform specific error type
pub type Error = nix::Error;

/// Platform specific signal type
pub type Signal = nix::sys::signal::Signal;

extern "C" fn os_handler(_: nix::libc::c_int) {
    unsafe {
        implementation::sem_post();
    }
}

/// Register os signal handler.
///
/// Must be called before calling [`block_ctrl_c()`](fn.block_ctrl_c.html)
/// and should only be called once.
///
/// # Errors
/// Will return an error if a system error occurred.
///
#[inline]
pub unsafe fn init_os_handler(overwrite: bool) -> Result<(), Error> {
    use nix::sys::signal;

    implementation::sem_init();

    let handler = signal::SigHandler::Handler(os_handler);
    #[cfg(not(target_os = "nto"))]
    let new_action = signal::SigAction::new(
        handler,
        signal::SaFlags::SA_RESTART,
        signal::SigSet::empty(),
    );
    // SA_RESTART is not supported on QNX Neutrino 7.1 and before
    #[cfg(target_os = "nto")]
    let new_action =
        signal::SigAction::new(handler, signal::SaFlags::empty(), signal::SigSet::empty());

    let sigint_old = signal::sigaction(signal::Signal::SIGINT, &new_action)?;
    if !overwrite && sigint_old.handler() != signal::SigHandler::SigDfl {
        signal::sigaction(signal::Signal::SIGINT, &sigint_old).unwrap();
        return Err(nix::Error::EEXIST);
    }

    #[cfg(feature = "termination")]
    {
        let sigterm_old = match signal::sigaction(signal::Signal::SIGTERM, &new_action) {
            Ok(old) => old,
            Err(e) => {
                signal::sigaction(signal::Signal::SIGINT, &sigint_old).unwrap();
                return Err(e);
            }
        };
        if !overwrite && sigterm_old.handler() != signal::SigHandler::SigDfl {
            signal::sigaction(signal::Signal::SIGINT, &sigint_old).unwrap();
            signal::sigaction(signal::Signal::SIGTERM, &sigterm_old).unwrap();
            return Err(nix::Error::EEXIST);
        }
        let sighup_old = match signal::sigaction(signal::Signal::SIGHUP, &new_action) {
            Ok(old) => old,
            Err(e) => {
                signal::sigaction(signal::Signal::SIGINT, &sigint_old).unwrap();
                signal::sigaction(signal::Signal::SIGTERM, &sigterm_old).unwrap();
                return Err(e);
            }
        };
        if !overwrite && sighup_old.handler() != signal::SigHandler::SigDfl {
            signal::sigaction(signal::Signal::SIGINT, &sigint_old).unwrap();
            signal::sigaction(signal::Signal::SIGTERM, &sigterm_old).unwrap();
            signal::sigaction(signal::Signal::SIGHUP, &sighup_old).unwrap();
            return Err(nix::Error::EEXIST);
        }
    }

    Ok(())
}

/// Blocks until a Ctrl-C signal is received.
///
/// Must be called after calling [`init_os_handler()`](fn.init_os_handler.html).
///
/// # Errors
/// None.
///
#[inline]
pub unsafe fn block_ctrl_c() -> Result<(), CtrlcError> {
    implementation::sem_wait_forever();
    Ok(())
}
