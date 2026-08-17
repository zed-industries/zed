use async_signal::{Signal, Signals};
use futures_lite::{future, prelude::*};

/// Send SIGINT to the current process.
#[cfg(unix)]
fn sigint() {
    unsafe {
        libc::raise(libc::SIGINT);
    }
}

/// Send SIGINT to the current process.
#[cfg(windows)]
fn sigint() {
    unsafe {
        windows_sys::Win32::System::Console::GenerateConsoleCtrlEvent(
            windows_sys::Win32::System::Console::CTRL_C_EVENT,
            0,
        );
    }
}

#[test]
fn test_sigint() {
    future::block_on(async {
        let mut signals = Signals::new(Some(Signal::Int)).unwrap();
        let mut next = signals.next();
        assert!(future::poll_once(&mut next).await.is_none());
        sigint();
        assert_eq!(signals.next().await.unwrap().unwrap(), Signal::Int);
    });
}
