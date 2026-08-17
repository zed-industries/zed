//! A signal notifier that uses an asynchronous channel.
//!
//! This is only valid for Windows, where there are less strict signal handling
//! requirements.

use crate::Signal;

use atomic_waker::AtomicWaker;

use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

/// The notifier that uses an asynchronous channel.
#[derive(Debug)]
pub struct Notifier {
    /// The pipe used to send signals.
    pipe: Arc<Pipe>,
}

impl Notifier {
    /// Create a new signal notifier.
    pub(super) fn new() -> io::Result<Self> {
        Ok(Self {
            pipe: Arc::new(Pipe {
                count: AtomicUsize::new(0),
                waker: AtomicWaker::new(),
            }),
        })
    }

    /// Add a signal to the notifier.
    ///
    /// Returns a closure to be passed to signal-hook.
    pub(super) fn add_signal(
        &mut self,
        _signal: Signal,
    ) -> io::Result<impl Fn() + Send + Sync + 'static> {
        let pipe = self.pipe.clone();
        Ok(move || {
            pipe.push();
        })
    }

    /// Remove a signal from the notifier.
    pub(super) fn remove_signal(&mut self, _signal: Signal) -> io::Result<()> {
        Ok(())
    }

    /// Get the next signal.
    pub(super) fn poll_next(&self, cx: &mut Context<'_>) -> Poll<io::Result<Signal>> {
        let mut count = self.pipe.count.load(Ordering::SeqCst);
        let mut registered = false;

        loop {
            // If the count is greater than zero, then we have a signal.
            if count > 0 {
                // Decrement the count.
                let new_count = count - 1;
                if let Err(new_count) = self.pipe.count.compare_exchange(
                    count,
                    new_count,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    count = new_count;
                    continue;
                }

                // Return the signal.
                return Poll::Ready(Ok(Signal::Int));
            }

            // If the count is zero, then we need to wait for a signal.
            if registered {
                // We are already registered, so we need to wait for a signal.
                return Poll::Pending;
            } else {
                registered = true;
                self.pipe.waker.register(cx.waker());

                // Try again.
                count = self.pipe.count.load(Ordering::SeqCst);
            }
        }
    }
}

#[derive(Debug)]
struct Pipe {
    /// The number of SIGINT signals received.
    count: AtomicUsize,

    /// The waker to wake up.
    waker: AtomicWaker,
}

impl Pipe {
    /// Add a signal to the notifier.
    fn push(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
        self.waker.wake();
    }
}
