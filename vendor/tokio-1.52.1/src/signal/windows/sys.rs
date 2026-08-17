use std::io;
use std::sync::OnceLock;

use crate::signal::RxFuture;
use crate::sync::watch;

use windows_sys::core::BOOL;
use windows_sys::Win32::System::Console as console;

type EventInfo = watch::Sender<()>;

pub(super) fn ctrl_break() -> io::Result<RxFuture> {
    new(&registry().ctrl_break)
}

pub(super) fn ctrl_close() -> io::Result<RxFuture> {
    new(&registry().ctrl_close)
}

pub(super) fn ctrl_c() -> io::Result<RxFuture> {
    new(&registry().ctrl_c)
}

pub(super) fn ctrl_logoff() -> io::Result<RxFuture> {
    new(&registry().ctrl_logoff)
}

pub(super) fn ctrl_shutdown() -> io::Result<RxFuture> {
    new(&registry().ctrl_shutdown)
}

fn new(event_info: &EventInfo) -> io::Result<RxFuture> {
    global_init()?;
    let rx = event_info.subscribe();
    Ok(RxFuture::new(rx))
}

fn event_requires_infinite_sleep_in_handler(signum: u32) -> bool {
    // Returning from the handler function of those events immediately terminates the process.
    // So for async systems, the easiest solution is to simply never return from
    // the handler function.
    //
    // For more information, see:
    // https://learn.microsoft.com/en-us/windows/console/handlerroutine#remarks
    matches!(
        signum,
        console::CTRL_CLOSE_EVENT | console::CTRL_LOGOFF_EVENT | console::CTRL_SHUTDOWN_EVENT
    )
}

#[derive(Debug, Default)]
struct Registry {
    ctrl_break: EventInfo,
    ctrl_close: EventInfo,
    ctrl_c: EventInfo,
    ctrl_logoff: EventInfo,
    ctrl_shutdown: EventInfo,
}

impl Registry {
    fn event_info(&self, signum: u32) -> Option<&EventInfo> {
        match signum {
            console::CTRL_BREAK_EVENT => Some(&self.ctrl_break),
            console::CTRL_CLOSE_EVENT => Some(&self.ctrl_close),
            console::CTRL_C_EVENT => Some(&self.ctrl_c),
            console::CTRL_LOGOFF_EVENT => Some(&self.ctrl_logoff),
            console::CTRL_SHUTDOWN_EVENT => Some(&self.ctrl_shutdown),
            _ => None,
        }
    }
}

fn registry() -> &'static Registry {
    static REGISTRY: OnceLock<Registry> = OnceLock::new();

    REGISTRY.get_or_init(Default::default)
}

fn global_init() -> io::Result<()> {
    static INIT: OnceLock<Result<(), Option<i32>>> = OnceLock::new();

    INIT.get_or_init(|| {
        let rc = unsafe { console::SetConsoleCtrlHandler(Some(handler), 1) };
        if rc == 0 {
            Err(io::Error::last_os_error().raw_os_error())
        } else {
            Ok(())
        }
    })
    .map_err(|e| {
        e.map_or_else(
            || io::Error::new(io::ErrorKind::Other, "registering signal handler failed"),
            io::Error::from_raw_os_error,
        )
    })
}

unsafe extern "system" fn handler(ty: u32) -> BOOL {
    // Ignore unknown control signal types.
    let Some(event_info) = registry().event_info(ty) else {
        return 0;
    };

    // According to https://learn.microsoft.com/en-us/windows/console/handlerroutine
    // the handler routine is always invoked in a new thread, thus we don't
    // have the same restrictions as in Unix signal handlers, meaning we can
    // go ahead and perform the broadcast here.
    match event_info.send(()) {
        Ok(_) if event_requires_infinite_sleep_in_handler(ty) => loop {
            std::thread::park();
        },
        Ok(_) => 1,
        // No one is listening for this notification any more
        // let the OS fire the next (possibly the default) handler.
        Err(_) => 0,
    }
}

#[cfg(all(test, not(loom)))]
mod tests {
    use super::*;
    use crate::runtime::Runtime;

    use tokio_test::{assert_ok, assert_pending, assert_ready_ok, task};

    unsafe fn raise_event(signum: u32) {
        if event_requires_infinite_sleep_in_handler(signum) {
            // Those events will enter an infinite loop in `handler`, so
            // we need to run them on a separate thread
            std::thread::spawn(move || unsafe { super::handler(signum) });
        } else {
            unsafe { super::handler(signum) };
        }
    }

    #[test]
    fn ctrl_c() {
        let rt = rt();
        let _enter = rt.enter();

        let mut ctrl_c = task::spawn(crate::signal::ctrl_c());

        assert_pending!(ctrl_c.poll());

        // Windows doesn't have a good programmatic way of sending events
        // like sending signals on Unix, so we'll stub out the actual OS
        // integration and test that our handling works.
        unsafe {
            raise_event(console::CTRL_C_EVENT);
        }

        assert_ready_ok!(ctrl_c.poll());
    }

    #[test]
    fn ctrl_break() {
        let rt = rt();

        rt.block_on(async {
            let mut ctrl_break = assert_ok!(crate::signal::windows::ctrl_break());

            // Windows doesn't have a good programmatic way of sending events
            // like sending signals on Unix, so we'll stub out the actual OS
            // integration and test that our handling works.
            unsafe {
                raise_event(console::CTRL_BREAK_EVENT);
            }

            ctrl_break.recv().await.unwrap();
        });
    }

    #[test]
    fn ctrl_close() {
        let rt = rt();

        rt.block_on(async {
            let mut ctrl_close = assert_ok!(crate::signal::windows::ctrl_close());

            // Windows doesn't have a good programmatic way of sending events
            // like sending signals on Unix, so we'll stub out the actual OS
            // integration and test that our handling works.
            unsafe {
                raise_event(console::CTRL_CLOSE_EVENT);
            }

            ctrl_close.recv().await.unwrap();
        });
    }

    #[test]
    fn ctrl_shutdown() {
        let rt = rt();

        rt.block_on(async {
            let mut ctrl_shutdown = assert_ok!(crate::signal::windows::ctrl_shutdown());

            // Windows doesn't have a good programmatic way of sending events
            // like sending signals on Unix, so we'll stub out the actual OS
            // integration and test that our handling works.
            unsafe {
                raise_event(console::CTRL_SHUTDOWN_EVENT);
            }

            ctrl_shutdown.recv().await.unwrap();
        });
    }

    #[test]
    fn ctrl_logoff() {
        let rt = rt();

        rt.block_on(async {
            let mut ctrl_logoff = assert_ok!(crate::signal::windows::ctrl_logoff());

            // Windows doesn't have a good programmatic way of sending events
            // like sending signals on Unix, so we'll stub out the actual OS
            // integration and test that our handling works.
            unsafe {
                raise_event(console::CTRL_LOGOFF_EVENT);
            }

            ctrl_logoff.recv().await.unwrap();
        });
    }

    fn rt() -> Runtime {
        crate::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
    }
}
