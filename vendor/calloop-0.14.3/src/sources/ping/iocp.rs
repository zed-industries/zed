//! IOCP-based implementation of the ping event source.
//!
//! The underlying `Poller` can be woken up at any time, using the `post` method
//! to send an arbitrary packet to the I/O completion port. The complication is
//! emulating a pipe.
//!
//! Since `Poller` is already wrapped in an `Arc`, we can clone it into some
//! synchronized inner state to send a pre-determined packet into it. Thankfully
//! calloop's use of the pipe is constrained enough that we can implement it using
//! a simple bool to keep track of whether or not it is notified.

use crate::sources::EventSource;

use polling::os::iocp::{CompletionPacket, PollerIocpExt};
use polling::Poller;
use tracing::{trace, warn};

use std::fmt;
use std::io;
use std::sync::{Arc, Mutex, TryLockError};

#[inline]
pub fn make_ping() -> io::Result<(Ping, PingSource)> {
    let state = Arc::new(State {
        counter: Mutex::new(Counter {
            notified: false,
            poll_state: None,
        }),
    });

    Ok((
        Ping {
            state: state.clone(),
        },
        PingSource { state },
    ))
}

/// The event to trigger.
#[derive(Clone)]
pub struct Ping {
    state: Arc<State>,
}

impl fmt::Debug for Ping {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_ping(&self.state, "Ping", f)
    }
}

/// The event source.
pub struct PingSource {
    state: Arc<State>,
}

impl fmt::Debug for PingSource {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_ping(&self.state, "PingSource", f)
    }
}

impl Ping {
    /// Send a ping to the `PingSource`.
    pub fn ping(&self) {
        let mut counter = self.state.counter.lock().unwrap_or_else(|e| e.into_inner());

        // Indicate that we are now notified.
        counter.notified = true;

        let poll_state = match &mut counter.poll_state {
            Some(ps) => ps,
            None => {
                warn!("ping was not registered with the event loop");
                return;
            }
        };

        // If we aren't currently inserted in the loop, send our packet.
        if let Err(e) = poll_state.notify() {
            warn!("failed to post packet to IOCP: {e}");
        }
    }
}

impl Drop for Ping {
    fn drop(&mut self) {
        // If this is the last ping, wake up the source so it removes itself.
        if Arc::strong_count(&self.state) <= 2 {
            let mut counter = self.state.counter.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(poll_state) = &mut counter.poll_state {
                if let Err(e) = poll_state.notify() {
                    warn!("failed to post packet to IOCP during drop: {e}");
                }
            }
        }
    }
}

impl EventSource for PingSource {
    type Error = super::PingError;
    type Event = ();
    type Metadata = ();
    type Ret = ();

    fn process_events<F>(
        &mut self,
        _readiness: crate::Readiness,
        token: crate::Token,
        mut callback: F,
    ) -> Result<crate::PostAction, Self::Error>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        let mut counter = self.state.counter.lock().unwrap_or_else(|e| e.into_inner());

        // If we aren't registered, break out.
        let poll_state = match &mut counter.poll_state {
            Some(ps) => ps,
            None => {
                // We were deregistered; indicate to the higher level loop.
                return Ok(crate::PostAction::Disable);
            }
        };

        // We are no longer inserted into the poller.
        poll_state.inserted = false;

        // Make sure this is our token.
        let token: usize = token.inner.into();
        if poll_state.packet.event().key != token {
            warn!(
                "token does not match; expected {:x}, got {:x}",
                poll_state.packet.event().key,
                token
            );
            return Ok(crate::PostAction::Continue);
        }

        // Tell if we are registered.
        if counter.notified {
            counter.notified = false;

            // Call the callback.
            callback((), &mut ());
        }

        // Stop looping if all of the Ping's have been dropped.
        let action = if Arc::strong_count(&self.state) <= 1 {
            crate::PostAction::Remove
        } else {
            crate::PostAction::Continue
        };

        Ok(action)
    }

    fn register(
        &mut self,
        poll: &mut crate::Poll,
        token_factory: &mut crate::TokenFactory,
    ) -> crate::Result<()> {
        let token = token_factory.token();
        let mut counter = self.state.counter.lock().unwrap_or_else(|e| e.into_inner());

        // Make sure we haven't already been registered.
        if counter.poll_state.is_some() {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists).into());
        }

        // Create the event to send.
        let packet = {
            let token = token.inner.into();
            let event = polling::Event::readable(token);
            CompletionPacket::new(event)
        };

        // Create the poll state.
        let poll_state = PollState::new(poll.poller(), packet, counter.notified)?;

        // Substitute it into our poll state.
        counter.poll_state = Some(poll_state);
        Ok(())
    }

    fn reregister(
        &mut self,
        poll: &mut crate::Poll,
        token_factory: &mut crate::TokenFactory,
    ) -> crate::Result<()> {
        let token = token_factory.token();
        let mut counter = self.state.counter.lock().unwrap_or_else(|e| e.into_inner());

        // Make sure that the poller has been registered.
        let poll_state = match &mut counter.poll_state {
            Some(ps) => ps,
            None => return Err(io::Error::from(io::ErrorKind::NotFound).into()),
        };

        // If it's a different poller, throw an error.
        if !Arc::ptr_eq(&poll_state.poller, poll.poller()) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "attempted to reregister() a PingSource with a different poller",
            )
            .into());
        }

        // Change the token if needed.
        let token = token.inner.into();
        let event = polling::Event::readable(token);

        if event.key != poll_state.packet.event().key {
            poll_state.packet = CompletionPacket::new(event);

            if poll_state.inserted {
                poll_state.inserted = false;
                poll_state.notify()?;
            }
        }

        Ok(())
    }

    fn unregister(&mut self, _poll: &mut crate::Poll) -> crate::Result<()> {
        let mut counter = self.state.counter.lock().unwrap_or_else(|e| e.into_inner());

        // Remove our current registration.
        if counter.poll_state.take().is_none() {
            trace!("unregistered a source that wasn't registered");
        }
        Ok(())
    }
}

/// Inner state of the pipe.
struct State {
    /// The counter used to keep track of our state.
    counter: Mutex<Counter>,
}

/// Inner counter of the pipe.
struct Counter {
    /// Are we notified?
    notified: bool,

    /// The `Poller`-related state.
    ///
    /// This is `None` if we aren't inserted into the `Poller` yet.
    poll_state: Option<PollState>,
}

/// The `Poller` combined with some associated state.
struct PollState {
    /// The `Poller` that we are registered in.
    poller: Arc<Poller>,

    /// Are we inserted into the poller?
    inserted: bool,

    /// The completion packet to send.
    packet: CompletionPacket,
}

impl PollState {
    /// Create a new `PollState` based on the `Poller` and the `packet`.
    ///
    /// If `notified` is `true`, a packet is inserted into the poller.
    fn new(poller: &Arc<Poller>, packet: CompletionPacket, notified: bool) -> io::Result<Self> {
        let mut poll_state = Self {
            poller: poller.clone(),
            packet,
            inserted: false,
        };

        if notified {
            poll_state.notify()?;
        }

        Ok(poll_state)
    }

    /// Notify the poller.
    fn notify(&mut self) -> io::Result<()> {
        if !self.inserted {
            self.poller.post(self.packet.clone())?;
            self.inserted = true;
        }

        Ok(())
    }
}

#[inline]
fn debug_ping(state: &State, name: &str, f: &mut fmt::Formatter) -> fmt::Result {
    let counter = match state.counter.try_lock() {
        Ok(counter) => counter,
        Err(TryLockError::WouldBlock) => {
            return f
                .debug_tuple("Ping")
                .field(&format_args!("<locked>"))
                .finish()
        }
        Err(TryLockError::Poisoned(_)) => {
            return f
                .debug_tuple("Ping")
                .field(&format_args!("<poisoned>"))
                .finish()
        }
    };

    let mut s = f.debug_struct(name);
    s.field("notified", &counter.notified);

    // Tell if we are registered.
    match &counter.poll_state {
        Some(poll_state) => {
            s.field("packet", poll_state.packet.event());
            s.field("inserted", &poll_state.inserted);
        }

        None => {
            s.field("packet", &format_args!("<not registered>"));
        }
    }

    s.finish()
}
