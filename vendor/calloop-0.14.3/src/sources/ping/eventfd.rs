//! Eventfd based implementation of the ping event source.
//!
//! # Implementation notes
//!
//! The eventfd is a much lighter signalling mechanism provided by the Linux
//! kernel. Rather than write an arbitrary sequence of bytes, it only has a
//! 64-bit counter.
//!
//! To avoid closing the eventfd early, we wrap it in a RAII-style closer
//! `CloseOnDrop` in `make_ping()`. When all the senders are dropped, another
//! wrapper `FlagOnDrop` handles signalling this to the event source, which is
//! the sole owner of the eventfd itself. The senders have weak references to
//! the eventfd, and if the source is dropped before the senders, they will
//! simply not do anything (except log a message).
//!
//! To differentiate between regular ping events and close ping events, we add 2
//! to the counter for regular events and 1 for close events. In the source we
//! can then check the LSB and if it's set, we know it was a close event. This
//! only works if a close event never fires more than once.

use std::os::unix::io::{AsFd, BorrowedFd, OwnedFd};
use std::sync::Arc;

use rustix::event::{eventfd, EventfdFlags};
use rustix::io::{read, write, Errno};
use tracing::warn;

use super::PingError;
use crate::{
    generic::Generic, EventSource, Interest, Mode, Poll, PostAction, Readiness, Token, TokenFactory,
};

// These are not bitfields! They are increments to add to the eventfd counter.
// Since the fd can only be closed once, we can effectively use the
// INCREMENT_CLOSE value as a bitmask when checking.
const INCREMENT_PING: u64 = 0x2;
const INCREMENT_CLOSE: u64 = 0x1;

#[inline]
pub fn make_ping() -> std::io::Result<(Ping, PingSource)> {
    let read = eventfd(0, EventfdFlags::CLOEXEC | EventfdFlags::NONBLOCK)?;

    // We only have one fd for the eventfd. If the sending end closes it when
    // all copies are dropped, the receiving end will be closed as well. We need
    // to make sure the fd is not closed until all holders of it have dropped
    // it.

    let fd = Arc::new(read);

    let ping = Ping {
        event: Arc::new(FlagOnDrop(Arc::clone(&fd))),
    };

    let source = PingSource {
        event: Generic::new(ArcAsFd(fd), Interest::READ, Mode::Level),
    };

    Ok((ping, source))
}

// Helper functions for the event source IO.

#[inline]
fn send_ping(fd: BorrowedFd<'_>, count: u64) -> std::io::Result<()> {
    assert!(count > 0);
    match write(fd, &count.to_ne_bytes()) {
        // The write succeeded, the ping will wake up the loop.
        Ok(_) => Ok(()),

        // The counter hit its cap, which means previous calls to write() will
        // wake up the loop.
        Err(Errno::AGAIN) => Ok(()),

        // Anything else is a real error.
        Err(e) => Err(e.into()),
    }
}

#[inline]
fn drain_ping(fd: BorrowedFd<'_>) -> std::io::Result<u64> {
    // The eventfd counter is effectively a u64.
    const NBYTES: usize = 8;
    let mut buf = [0u8; NBYTES];

    match read(fd, &mut buf) {
        // Reading from an eventfd should only ever produce 8 bytes. No looping
        // is required.
        Ok(NBYTES) => Ok(u64::from_ne_bytes(buf)),

        Ok(_) => unreachable!(),

        // Any other error can be propagated.
        Err(e) => Err(e.into()),
    }
}

// Rust 1.64.0 adds an `AsFd` implementation for `Arc`, so this won't be needed
#[derive(Debug)]
struct ArcAsFd(Arc<OwnedFd>);

impl AsFd for ArcAsFd {
    fn as_fd(&self) -> BorrowedFd {
        self.0.as_fd()
    }
}

// The event source is simply a generic source with one of the eventfds.
#[derive(Debug)]
pub struct PingSource {
    event: Generic<ArcAsFd>,
}

impl EventSource for PingSource {
    type Event = ();
    type Metadata = ();
    type Ret = ();
    type Error = PingError;

    fn process_events<C>(
        &mut self,
        readiness: Readiness,
        token: Token,
        mut callback: C,
    ) -> Result<PostAction, Self::Error>
    where
        C: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        self.event
            .process_events(readiness, token, |_, fd| {
                let counter = drain_ping(fd.as_fd())?;

                // If the LSB is set, it means we were closed. If anything else
                // is also set, it means we were pinged. The two are not
                // mutually exclusive.
                let close = (counter & INCREMENT_CLOSE) != 0;
                let ping = (counter & (u64::MAX - 1)) != 0;

                if ping {
                    callback((), &mut ());
                }

                if close {
                    Ok(PostAction::Remove)
                } else {
                    Ok(PostAction::Continue)
                }
            })
            .map_err(|e| PingError(e.into()))
    }

    fn register(&mut self, poll: &mut Poll, token_factory: &mut TokenFactory) -> crate::Result<()> {
        self.event.register(poll, token_factory)
    }

    fn reregister(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> crate::Result<()> {
        self.event.reregister(poll, token_factory)
    }

    fn unregister(&mut self, poll: &mut Poll) -> crate::Result<()> {
        self.event.unregister(poll)
    }
}

#[derive(Clone, Debug)]
pub struct Ping {
    // This is an Arc because it's potentially shared with clones. The last one
    // dropped needs to signal to the event source via the eventfd.
    event: Arc<FlagOnDrop>,
}

impl Ping {
    /// Send a ping to the `PingSource`.
    pub fn ping(&self) {
        if let Err(e) = send_ping(self.event.0.as_fd(), INCREMENT_PING) {
            warn!("Failed to write a ping: {e:?}");
        }
    }
}

/// This manages signalling to the PingSource when it's dropped. There should
/// only ever be one of these per PingSource.
#[derive(Debug)]
struct FlagOnDrop(Arc<OwnedFd>);

impl Drop for FlagOnDrop {
    fn drop(&mut self) {
        if let Err(e) = send_ping(self.0.as_fd(), INCREMENT_CLOSE) {
            warn!("Failed to send close ping: {e:?}");
        }
    }
}
