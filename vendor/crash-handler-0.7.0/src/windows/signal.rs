//! Windows doesn't have an exception for process aborts, so we hook `SIGABRT`

/// Installs our `SIGABRT` handler, returning any previously registered handler,
/// which should be restored later
///
/// # Safety
///
/// Performs syscalls
pub(crate) unsafe fn install_abort_handler() -> Result<libc::sighandler_t, std::io::Error> {
    // It would be nice to use sigaction here since it's better, but it isn't
    // supported on Windows :p
    unsafe {
        let old_handler = libc::signal(libc::SIGABRT, signal_handler as usize);
        if old_handler != usize::MAX {
            Ok(old_handler)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

/// Restores the action for `SIGABRT` to the specified handler
///
/// # Safety
///
/// Performs syscalls
#[inline]
pub(crate) unsafe fn restore_abort_handler(handler: libc::sighandler_t) {
    unsafe { libc::signal(libc::SIGABRT, handler) };
}

unsafe extern "C" fn signal_handler(signal: i32, _subcode: i32) {
    // Sanity check
    assert_eq!(signal, libc::SIGABRT);

    // https://github.com/chromium/crashpad/blob/fca8871ca3fb721d3afab370ca790122f9333bfd/client/crashpad_client_win.cc#L197
    unsafe { super::state::simulate_exception(Some(super::ExceptionCode::Abort as _)) };
}
