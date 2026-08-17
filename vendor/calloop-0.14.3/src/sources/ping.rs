//! Ping to the event loop
//!
//! This is an event source that just produces `()` events whevener the associated
//! [`Ping::ping`](Ping#method.ping) method is called. If the event source is pinged multiple times
//! between a single dispatching, it'll only generate one event.
//!
//! This event source is a simple way of waking up the event loop from an other part of your program
//! (and is what backs the [`LoopSignal`](crate::LoopSignal)). It can also be used as a building
//! block to construct event sources whose source of event is not file descriptor, but rather an
//! userspace source (like an other thread).

// The ping source has platform-dependent implementations provided by modules
// under this one. These modules should expose:
// - a make_ping() function
// - a Ping type
// - a PingSource type
//
// See eg. the pipe implementation for these items' specific requirements.

#[cfg(target_os = "linux")]
mod eventfd;
#[cfg(target_os = "linux")]
use eventfd as platform;

#[cfg(windows)]
mod iocp;
#[cfg(windows)]
use iocp as platform;

#[cfg(not(any(target_os = "linux", windows)))]
mod pipe;
#[cfg(not(any(target_os = "linux", windows)))]
use pipe as platform;

use std::fmt;

/// Create a new ping event source
///
/// you are given a [`Ping`] instance, which can be cloned and used to ping the
/// event loop, and a [`PingSource`], which you can insert in your event loop to
/// receive the pings.
pub fn make_ping() -> std::io::Result<(Ping, PingSource)> {
    platform::make_ping()
}

/// The ping event source
///
/// You can insert it in your event loop to receive pings.
///
/// If you use it directly, it will automatically remove itself from the event loop
/// once all [`Ping`] instances are dropped.
pub type Ping = platform::Ping;

/// The Ping handle
///
/// This handle can be cloned and sent accross threads. It can be used to
/// send pings to the `PingSource`.
pub type PingSource = platform::PingSource;

/// An error arising from processing events for a ping.
#[derive(Debug)]
pub struct PingError(Box<dyn std::error::Error + Sync + Send>);

impl fmt::Display for PingError {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for PingError {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::transient::TransientSource;
    use std::time::Duration;

    use super::*;

    #[test]
    fn ping() {
        let mut event_loop = crate::EventLoop::<bool>::try_new().unwrap();

        let (ping, source) = make_ping().unwrap();

        event_loop
            .handle()
            .insert_source(source, |(), &mut (), dispatched| *dispatched = true)
            .unwrap();

        ping.ping();

        let mut dispatched = false;
        event_loop
            .dispatch(std::time::Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(dispatched);

        // Ping has been drained an no longer generates events
        let mut dispatched = false;
        event_loop
            .dispatch(std::time::Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(!dispatched);
    }

    #[test]
    fn ping_closed() {
        let mut event_loop = crate::EventLoop::<bool>::try_new().unwrap();

        let (_, source) = make_ping().unwrap();
        event_loop
            .handle()
            .insert_source(source, |(), &mut (), dispatched| *dispatched = true)
            .unwrap();

        let mut dispatched = false;

        // If the sender is closed from the start, the ping should first trigger
        // once, disabling itself but not invoking the callback
        event_loop
            .dispatch(std::time::Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(!dispatched);

        // Then it should not trigger any more, so this dispatch should wait the whole 100ms
        let now = std::time::Instant::now();
        event_loop
            .dispatch(std::time::Duration::from_millis(100), &mut dispatched)
            .unwrap();
        assert!(now.elapsed() >= std::time::Duration::from_millis(100));
    }

    #[test]
    fn ping_removed() {
        // This keeps track of whether the event fired.
        let mut dispatched = false;

        let mut event_loop = crate::EventLoop::<bool>::try_new().unwrap();

        let (sender, source) = make_ping().unwrap();
        let wrapper = TransientSource::from(source);

        // Check that the source starts off in the wrapper.
        assert!(!wrapper.is_none());

        // Put the source in the loop.

        let dispatcher =
            crate::Dispatcher::new(wrapper, |(), &mut (), dispatched| *dispatched = true);

        let token = event_loop
            .handle()
            .register_dispatcher(dispatcher.clone())
            .unwrap();

        // Drop the sender and check that it's actually removed.
        drop(sender);

        // There should be no event, but the loop still needs to wake up to
        // process the close event (just like in the ping_closed() test).
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(!dispatched);

        // Pull the source wrapper out.

        event_loop.handle().remove(token);
        let wrapper = dispatcher.into_source_inner();

        // Check that the inner source is now gone.
        assert!(wrapper.is_none());
    }

    #[test]
    fn ping_fired_and_removed() {
        // This is like ping_removed() with the single difference that we fire a
        // ping and drop it between two successive dispatches of the loop.

        // This keeps track of whether the event fired.
        let mut dispatched = false;

        let mut event_loop = crate::EventLoop::<bool>::try_new().unwrap();

        let (sender, source) = make_ping().unwrap();
        let wrapper = TransientSource::from(source);

        // Check that the source starts off in the wrapper.
        assert!(!wrapper.is_none());

        // Put the source in the loop.

        let dispatcher =
            crate::Dispatcher::new(wrapper, |(), &mut (), dispatched| *dispatched = true);

        let token = event_loop
            .handle()
            .register_dispatcher(dispatcher.clone())
            .unwrap();

        // Send a ping AND drop the sender and check that it's actually removed.
        sender.ping();
        drop(sender);

        // There should be an event, but the source should be removed from the
        // loop immediately after.
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(dispatched);

        // Pull the source wrapper out.

        event_loop.handle().remove(token);
        let wrapper = dispatcher.into_source_inner();

        // Check that the inner source is now gone.
        assert!(wrapper.is_none());
    }

    #[test]
    fn ping_multiple_senders() {
        // This is like ping_removed() but for testing the behaviour of multiple
        // senders.

        // This keeps track of whether the event fired.
        let mut dispatched = false;

        let mut event_loop = crate::EventLoop::<bool>::try_new().unwrap();

        let (sender0, source) = make_ping().unwrap();
        let wrapper = TransientSource::from(source);
        let sender1 = sender0.clone();
        let sender2 = sender1.clone();

        // Check that the source starts off in the wrapper.
        assert!(!wrapper.is_none());

        // Put the source in the loop.

        let dispatcher =
            crate::Dispatcher::new(wrapper, |(), &mut (), dispatched| *dispatched = true);

        let token = event_loop
            .handle()
            .register_dispatcher(dispatcher.clone())
            .unwrap();

        // Send a ping AND drop the sender and check that it's actually removed.
        sender0.ping();
        drop(sender0);

        // There should be an event, and the source should remain in the loop.
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(dispatched);

        // Now test that the clones still work. Drop after the dispatch loop
        // instead of before, this time.
        dispatched = false;

        sender1.ping();

        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(dispatched);

        // Finally, drop all of them without sending anything.

        dispatched = false;

        drop(sender1);
        drop(sender2);

        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(!dispatched);

        // Pull the source wrapper out.

        event_loop.handle().remove(token);
        let wrapper = dispatcher.into_source_inner();

        // Check that the inner source is now gone.
        assert!(wrapper.is_none());
    }
}
