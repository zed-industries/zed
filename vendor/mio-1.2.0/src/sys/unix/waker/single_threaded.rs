use std::io;
use std::os::fd::RawFd;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

/// Waker backed by a boolean value.
///
/// This implementation is meant for systems with only a single thread of
/// control, in which case it is not possible to wake a thread which is already
/// blocked on a select call. Thus, this may only be used to ensure the next
/// select call by the current thread polls without blocking.
///
/// Note that this is currently meant only for use with the `poll(2)`
/// implementation, which has special support for using a zero timeout when
/// `Waker::woken` returns `true`.
#[derive(Debug)]
pub(crate) struct Waker {
    woken: AtomicBool,
}

impl Waker {
    pub(crate) fn new_unregistered() -> io::Result<Waker> {
        Ok(Waker {
            woken: AtomicBool::new(false),
        })
    }

    pub(crate) fn wake(&self) -> io::Result<()> {
        self.woken.store(true, Relaxed);
        Ok(())
    }

    /// Only non-`None` for the `pipe(2)`- and `eventfd(2)`-based `Waker`s:
    pub(crate) fn fd(&self) -> Option<RawFd> {
        None
    }

    pub(crate) fn woken(&self) -> bool {
        self.woken.load(Relaxed)
    }

    pub(crate) fn ack_and_reset(&self) {
        self.woken.store(false, Relaxed);
    }
}
