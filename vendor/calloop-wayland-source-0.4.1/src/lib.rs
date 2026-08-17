// SPDX-License-Identifier: MIT

//! Utilities for using an [`EventQueue`] from wayland-client with an event loop
//! that performs polling with [`calloop`](https://crates.io/crates/calloop).
//!
//! # Example
//!
//! ```no_run,rust
//! use calloop::EventLoop;
//! use calloop_wayland_source::WaylandSource;
//! use wayland_client::{Connection, QueueHandle};
//!
//! // Create a Wayland connection and a queue.
//! let connection = Connection::connect_to_env().unwrap();
//! let event_queue = connection.new_event_queue();
//! let queue_handle = event_queue.handle();
//!
//! // Create the calloop event loop to drive everytihng.
//! let mut event_loop: EventLoop<()> = EventLoop::try_new().unwrap();
//! let loop_handle = event_loop.handle();
//!
//! // Insert the wayland source into the calloop's event loop.
//! WaylandSource::new(connection, event_queue).insert(loop_handle).unwrap();
//!
//! // This will start dispatching the event loop and processing pending wayland requests.
//! while let Ok(_) = event_loop.dispatch(None, &mut ()) {
//!     // Your logic here.
//! }
//! ```

#![deny(unsafe_op_in_unsafe_fn)]
use std::io;

use calloop::generic::Generic;
use calloop::{
    EventSource, InsertError, Interest, LoopHandle, Mode, Poll, PostAction, Readiness,
    RegistrationToken, Token, TokenFactory,
};
use rustix::io::Errno;
use wayland_backend::client::{ReadEventsGuard, WaylandError};
use wayland_client::{Connection, DispatchError, EventQueue};

#[cfg(feature = "log")]
use log::error as log_error;
#[cfg(not(feature = "log"))]
use std::eprintln as log_error;

/// An adapter to insert an [`EventQueue`] into a calloop
/// [`EventLoop`](calloop::EventLoop).
///
/// This type implements [`EventSource`] which generates an event whenever
/// events on the event queue need to be dispatched. The event queue available
/// in the callback calloop registers may be used to dispatch pending
/// events using [`EventQueue::dispatch_pending`].
///
/// [`WaylandSource::insert`] can be used to insert this source into an event
/// loop and automatically dispatch pending events on the event queue.
#[derive(Debug)]
pub struct WaylandSource<D> {
    // In theory, we could use the same event queue inside `connection_source`
    // However, calloop's safety requirements mean that we cannot then give
    // mutable access to the queue, which is incompatible with our current interface
    // Additionally, `Connection` is cheaply cloneable, so it's not a huge burden
    queue: EventQueue<D>,
    connection_source: Generic<Connection>,
    read_guard: Option<ReadEventsGuard>,
    /// Calloop's before_will_sleep method allows
    /// skipping the sleeping by returning a `Token`.
    /// We cannot produce this on the fly, so store it here instead
    fake_token: Option<Token>,
    // Some calloop event handlers don't support error handling, so we have to store the error
    // for a short time until we reach a method which allows it
    stored_error: Result<(), io::Error>,
}

impl<D> WaylandSource<D> {
    /// Wrap an [`EventQueue`] as a [`WaylandSource`].
    ///
    /// `queue` must be from the connection `Connection`.
    /// This is not a safety invariant, but not following this may cause
    /// freezes or hangs
    pub fn new(connection: Connection, queue: EventQueue<D>) -> WaylandSource<D> {
        let connection_source = Generic::new(connection, Interest::READ, Mode::Level);

        WaylandSource {
            queue,
            connection_source,
            read_guard: None,
            fake_token: None,
            stored_error: Ok(()),
        }
    }

    /// Access the underlying event queue
    ///
    /// Note that you should not replace this queue with a queue from a
    /// different `Connection`, as that may cause freezes or other hangs.
    pub fn queue(&mut self) -> &mut EventQueue<D> {
        &mut self.queue
    }

    /// Access the connection to the Wayland server
    pub fn connection(&self) -> &Connection {
        self.connection_source.get_ref()
    }

    /// Insert this source into the given event loop.
    ///
    /// This adapter will pass the event loop's shared data as the `D` type for
    /// the event loop.
    pub fn insert(self, handle: LoopHandle<D>) -> Result<RegistrationToken, InsertError<Self>>
    where
        D: 'static,
    {
        handle.insert_source(self, |_, queue, data| queue.dispatch_pending(data))
    }
}

impl<D> EventSource for WaylandSource<D> {
    type Error = calloop::Error;
    type Event = ();
    /// The underlying event queue.
    ///
    /// You should call [`EventQueue::dispatch_pending`] inside your callback
    /// using this queue.
    type Metadata = EventQueue<D>;
    type Ret = Result<usize, DispatchError>;

    const NEEDS_EXTRA_LIFECYCLE_EVENTS: bool = true;

    fn process_events<F>(
        &mut self,
        _: Readiness,
        _: Token,
        mut callback: F,
    ) -> Result<PostAction, Self::Error>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        debug_assert!(self.read_guard.is_none());

        // Take the stored error
        std::mem::replace(&mut self.stored_error, Ok(()))?;

        // We know that the event will either be a fake event
        // produced in `before_will_sleep`, or a "real" event from the underlying
        // source (self.queue_events). Our behaviour in both cases is the same.
        // In theory we might want to call the process_events handler on the underlying
        // event source. However, we know that Generic's `process_events` call is a
        // no-op, so we just handle the event ourselves.

        let queue = &mut self.queue;
        // Dispatch any pending events in the queue
        Self::loop_callback_pending(queue, &mut callback)?;

        // Once dispatching is finished, flush the responses to the compositor
        flush_queue(queue)?;

        Ok(PostAction::Continue)
    }

    fn register(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> calloop::Result<()> {
        self.fake_token = Some(token_factory.token());
        self.connection_source.register(poll, token_factory)
    }

    fn reregister(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> calloop::Result<()> {
        self.connection_source.reregister(poll, token_factory)
    }

    fn unregister(&mut self, poll: &mut Poll) -> calloop::Result<()> {
        self.connection_source.unregister(poll)
    }

    fn before_sleep(&mut self) -> calloop::Result<Option<(Readiness, Token)>> {
        debug_assert!(self.read_guard.is_none());

        flush_queue(&mut self.queue)?;

        self.read_guard = self.queue.prepare_read();
        match self.read_guard {
            Some(_) => Ok(None),
            // If getting the guard failed, that means that there are
            // events in the queue, and so we need to handle the events instantly
            // rather than waiting on an event in polling. We tell calloop this
            // by returning Some here. Note that the readiness value is
            // never used, so we just need some marker
            None => Ok(Some((Readiness::EMPTY, self.fake_token.unwrap()))),
        }
    }

    fn before_handle_events(&mut self, events: calloop::EventIterator<'_>) {
        // It's important that the guard isn't held whilst process_events calls occur
        // This can use arbitrary user-provided code, which may want to use the wayland
        // socket For example, creating a Vulkan surface needs access to the
        // connection
        let guard = self.read_guard.take();
        if events.count() > 0 {
            // Read events from the socket if any are available
            if let Some(Err(WaylandError::Io(err))) = guard.map(ReadEventsGuard::read) {
                // If some other thread read events before us, concurrently, that's an expected
                // case, so this error isn't an issue. Other error kinds do need to be returned,
                // however
                if err.kind() != io::ErrorKind::WouldBlock {
                    // before_handle_events doesn't allow returning errors
                    // For now, cache it in self until process_events is called
                    self.stored_error = Err(err);
                }
            }
        }
    }
}

fn flush_queue<D>(queue: &mut EventQueue<D>) -> Result<(), calloop::Error> {
    if let Err(WaylandError::Io(err)) = queue.flush() {
        // WouldBlock error means the compositor could not process all
        // our messages quickly. Either it is slowed
        // down or we are a spammer. Should not really
        // happen, if it does we do nothing and will flush again later
        if err.kind() != io::ErrorKind::WouldBlock {
            // in case of error, forward it and fast-exit
            log_error!("Error trying to flush the wayland display: {}", err);
            return Err(err.into());
        }
    }
    Ok(())
}

impl<D> WaylandSource<D> {
    /// Loop over the callback until all pending messages have been dispatched.
    fn loop_callback_pending<F>(queue: &mut EventQueue<D>, callback: &mut F) -> io::Result<()>
    where
        F: FnMut((), &mut EventQueue<D>) -> Result<usize, DispatchError>,
    {
        // Loop on the callback until no pending events are left.
        loop {
            match callback((), queue) {
                // No more pending events.
                Ok(0) => break Ok(()),
                Ok(_) => continue,
                Err(DispatchError::Backend(WaylandError::Io(err))) => {
                    return Err(err);
                },
                Err(DispatchError::Backend(WaylandError::Protocol(err))) => {
                    log_error!("Protocol error received on display: {}", err);

                    break Err(Errno::PROTO.into());
                },
                Err(DispatchError::BadMessage { interface, sender_id, opcode }) => {
                    log_error!(
                        "Bad message on interface \"{}\": (sender_id: {}, opcode: {})",
                        interface,
                        sender_id,
                        opcode,
                    );

                    break Err(Errno::PROTO.into());
                },
            }
        }
    }
}
