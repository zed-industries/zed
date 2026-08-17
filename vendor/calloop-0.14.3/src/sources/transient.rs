//! Wrapper for a transient Calloop event source.
//!
//! If you have high level event source that you expect to remain in the event
//! loop indefinitely, and another event source nested inside that one that you
//! expect to require removal or disabling from time to time, this module can
//! handle it for you.

/// A [`TransientSource`] wraps a Calloop event source and manages its
/// registration. A user of this type only needs to perform the usual Calloop
/// calls (`process_events()` and `*register()`) and the return value of
/// [`process_events()`](crate::EventSource::process_events).
///
/// Rather than needing to check for the full set of
/// [`PostAction`](crate::PostAction) values returned from `process_events()`,
/// you can just check for `Continue` or `Reregister` and pass that back out
/// through your own `process_events()` implementation. In your registration
/// functions, you then only need to call the same function on this type ie.
/// `register()` inside `register()` etc.
///
/// For example, say you have a source that contains a channel along with some
/// other logic. If the channel's sending end has been dropped, it needs to be
/// removed from the loop. So to manage this, you use this in your struct:
///
/// ```none,actually-rust-but-see-https://github.com/rust-lang/rust/issues/63193
/// struct CompositeSource {
///    // Event source for channel.
///    mpsc_receiver: TransientSource<calloop::channel::Channel<T>>,
///
///    // Any other fields go here...
/// }
/// ```
///
/// To create the transient source, you can simply use the `Into`
/// implementation:
///
/// ```none,actually-rust-but-see-https://github.com/rust-lang/rust/issues/63193
/// let (sender, source) = channel();
/// let mpsc_receiver: TransientSource<Channel> = source.into();
/// ```
///
/// (If you want to start off with an empty `TransientSource`, you can just use
/// `Default::default()` instead.)
///
/// `TransientSource` implements [`EventSource`](crate::EventSource) and passes
/// through `process_events()` calls, so in the parent's `process_events()`
/// implementation you can just do this:
///
/// ```none,actually-rust-but-see-https://github.com/rust-lang/rust/issues/63193
/// fn process_events<F>(
///     &mut self,
///     readiness: calloop::Readiness,
///     token: calloop::Token,
///     callback: F,
/// ) -> Result<calloop::PostAction, Self::Error>
/// where
///     F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
/// {
///     let channel_return = self.mpsc_receiver.process_events(readiness, token, callback)?;
///
///     // Perform other logic here...
///
///     Ok(channel_return)
/// }
/// ```
///
/// Note that:
///
///   - You can call `process_events()` on the `TransientSource<Channel>` even
///     if the channel has been unregistered and dropped. All that will happen
///     is that you won't get any events from it.
///
///   - The [`PostAction`](crate::PostAction) returned from `process_events()`
///     will only ever be `PostAction::Continue` or `PostAction::Reregister`.
///     You will still need to combine this with the result of any other sources
///     (transient or not).
///
/// Once you return `channel_return` from your `process_events()` method (and
/// assuming it propagates all the way up to the event loop itself through any
/// other event sources), the event loop might call `reregister()` on your
/// source. All your source has to do is:
///
/// ```none,actually-rust-but-see-https://github.com/rust-lang/rust/issues/63193
/// fn reregister(
///     &mut self,
///     poll: &mut calloop::Poll,
///     token_factory: &mut calloop::TokenFactory,
/// ) -> crate::Result<()> {
///     self.mpsc_receiver.reregister(poll, token_factory)?;
///
///     // Other registration actions...
///
///     Ok(())
/// }
/// ```
///
/// The `TransientSource` will take care of updating the registration of the
/// inner source, even if it actually needs to be unregistered or initially
/// registered.
///
/// ## Replacing or removing `TransientSource`s
///
/// Not properly removing or replacing `TransientSource`s can cause spurious
/// wakeups of the event loop, and in some cases can leak file descriptors or
/// fail to free entries in Calloop's internal data structures. No unsoundness
/// or undefined behaviour will result, but leaking file descriptors can result
/// in errors or panics.
///
/// If you want to remove a source before it returns `PostAction::Remove`, use
/// the [`TransientSource::remove()`] method. If you want to replace a source
/// with another one, use the [`TransientSource::replace()`] method. Either of
/// these may be called at any time during processing or from outside the event
/// loop. Both require either returning `PostAction::Reregister` from the
/// `process_event()` call that does this, or reregistering the event source
/// some other way eg. via the top-level loop handle.
///
/// If, instead, you directly assign a new source to the variable holding the
/// `TransientSource`, the inner source will be dropped before it can be
/// unregistered. For example:
///
/// ```none,actually-rust-but-see-https://github.com/rust-lang/rust/issues/63193
/// self.mpsc_receiver = Default::default();
/// self.mpsc_receiver = new_channel.into();
/// ```
#[derive(Debug, Default)]
pub struct TransientSource<T> {
    state: TransientSourceState<T>,
}

/// This is the internal state of the [`TransientSource`], as a separate type so
/// it's not exposed.
#[derive(Debug)]
enum TransientSourceState<T> {
    /// The source should be kept in the loop.
    Keep(T),
    /// The source needs to be registered with the loop.
    Register(T),
    /// The source needs to be disabled but kept.
    Disable(T),
    /// The source needs to be removed from the loop.
    Remove(T),
    /// The source is being replaced by another. For most API purposes (eg.
    /// `map()`), this will be treated as the `Register` state enclosing the new
    /// source.
    Replace {
        /// The new source, which will be registered and used from now on.
        new: T,
        /// The old source, which will be unregistered and dropped.
        old: T,
    },
    /// The source has been removed from the loop and dropped (this might also
    /// be observed if there is a panic while changing states).
    None,
}

impl<T> Default for TransientSourceState<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T> TransientSourceState<T> {
    /// If a caller needs to flag the contained source for removal or
    /// registration, we need to replace the enum variant safely. This requires
    /// having a `None` value in there temporarily while we do the swap.
    ///
    /// If the variant is `None` the value will not change and `replacer` will
    /// not be called. If the variant is `Replace` then `replacer` will be
    /// called **on the new source**, which may cause the old source to leak
    /// registration in the event loop if it has not yet been unregistered.
    ///
    /// The `replacer` function here is expected to be one of the enum variant
    /// constructors eg. `replace(TransientSource::Remove)`.
    fn replace_state<F>(&mut self, replacer: F)
    where
        F: FnOnce(T) -> Self,
    {
        *self = match std::mem::take(self) {
            Self::Keep(source)
            | Self::Register(source)
            | Self::Remove(source)
            | Self::Disable(source)
            | Self::Replace { new: source, .. } => replacer(source),
            Self::None => return,
        };
    }
}

impl<T> TransientSource<T> {
    /// Apply a function to the enclosed source, if it exists and is not about
    /// to be removed.
    pub fn map<F, U>(&mut self, f: F) -> Option<U>
    where
        F: FnOnce(&mut T) -> U,
    {
        match &mut self.state {
            TransientSourceState::Keep(source)
            | TransientSourceState::Register(source)
            | TransientSourceState::Disable(source)
            | TransientSourceState::Replace { new: source, .. } => Some(f(source)),
            TransientSourceState::Remove(_) | TransientSourceState::None => None,
        }
    }

    /// Returns `true` if there is no wrapped event source.
    pub fn is_none(&self) -> bool {
        matches!(self.state, TransientSourceState::None)
    }

    /// Removes the wrapped event source from the event loop and this wrapper.
    ///
    /// If this is called from outside of the event loop, you will need to wake
    /// up the event loop for any changes to take place. If it is called from
    /// within the event loop, you must return `PostAction::Reregister` from
    /// your own event source's `process_events()`, and the source will be
    /// unregistered as needed after it exits.
    pub fn remove(&mut self) {
        self.state.replace_state(TransientSourceState::Remove);
    }

    /// Replace the currently wrapped source with the given one.  No more events
    /// will be generated from the old source after this point. The old source
    /// will not be dropped immediately, it will be kept so that it can be
    /// deregistered.
    ///
    /// If this is called from outside of the event loop, you will need to wake
    /// up the event loop for any changes to take place. If it is called from
    /// within the event loop, you must return `PostAction::Reregister` from
    /// your own event source's `process_events()`, and the sources will be
    /// registered and unregistered as needed after it exits.
    pub fn replace(&mut self, new: T) {
        self.state
            .replace_state(|old| TransientSourceState::Replace { new, old });
    }
}

impl<T: crate::EventSource> From<T> for TransientSource<T> {
    fn from(source: T) -> Self {
        Self {
            state: TransientSourceState::Register(source),
        }
    }
}

impl<T: crate::EventSource> crate::EventSource for TransientSource<T> {
    type Event = T::Event;
    type Metadata = T::Metadata;
    type Ret = T::Ret;
    type Error = T::Error;

    fn process_events<F>(
        &mut self,
        readiness: crate::Readiness,
        token: crate::Token,
        callback: F,
    ) -> Result<crate::PostAction, Self::Error>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        let reregister = if let TransientSourceState::Keep(source) = &mut self.state {
            let child_post_action = source.process_events(readiness, token, callback)?;

            match child_post_action {
                // Nothing needs to change.
                crate::PostAction::Continue => false,

                // Our child source needs re-registration, therefore this
                // wrapper needs re-registration.
                crate::PostAction::Reregister => true,

                // If our nested source needs to be removed or disabled, we need
                // to swap it out for the "Remove" or "Disable" variant.
                crate::PostAction::Disable => {
                    self.state.replace_state(TransientSourceState::Disable);
                    true
                }

                crate::PostAction::Remove => {
                    self.state.replace_state(TransientSourceState::Remove);
                    true
                }
            }
        } else {
            false
        };

        let post_action = if reregister {
            crate::PostAction::Reregister
        } else {
            crate::PostAction::Continue
        };

        Ok(post_action)
    }

    fn register(
        &mut self,
        poll: &mut crate::Poll,
        token_factory: &mut crate::TokenFactory,
    ) -> crate::Result<()> {
        match &mut self.state {
            TransientSourceState::Keep(source) => {
                source.register(poll, token_factory)?;
            }
            TransientSourceState::Register(source)
            | TransientSourceState::Disable(source)
            | TransientSourceState::Replace { new: source, .. } => {
                source.register(poll, token_factory)?;
                self.state.replace_state(TransientSourceState::Keep);
                // Drops the disposed source in the Replace case.
            }
            TransientSourceState::Remove(_source) => {
                self.state.replace_state(|_| TransientSourceState::None);
            }
            TransientSourceState::None => (),
        }
        Ok(())
    }

    fn reregister(
        &mut self,
        poll: &mut crate::Poll,
        token_factory: &mut crate::TokenFactory,
    ) -> crate::Result<()> {
        match &mut self.state {
            TransientSourceState::Keep(source) => source.reregister(poll, token_factory)?,
            TransientSourceState::Register(source) => {
                source.register(poll, token_factory)?;
                self.state.replace_state(TransientSourceState::Keep);
            }
            TransientSourceState::Disable(source) => {
                source.unregister(poll)?;
            }
            TransientSourceState::Remove(source) => {
                source.unregister(poll)?;
                self.state.replace_state(|_| TransientSourceState::None);
            }
            TransientSourceState::Replace { new, old } => {
                old.unregister(poll)?;
                new.register(poll, token_factory)?;
                self.state.replace_state(TransientSourceState::Keep);
                // Drops 'dispose'.
            }
            TransientSourceState::None => (),
        }
        Ok(())
    }

    fn unregister(&mut self, poll: &mut crate::Poll) -> crate::Result<()> {
        match &mut self.state {
            TransientSourceState::Keep(source)
            | TransientSourceState::Register(source)
            | TransientSourceState::Disable(source) => source.unregister(poll)?,
            TransientSourceState::Remove(source) => {
                source.unregister(poll)?;
                self.state.replace_state(|_| TransientSourceState::None);
            }
            TransientSourceState::Replace { new, old } => {
                old.unregister(poll)?;
                new.unregister(poll)?;
                self.state.replace_state(TransientSourceState::Register);
            }
            TransientSourceState::None => (),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        channel::{channel, Channel, Event},
        ping::{make_ping, PingSource},
        Dispatcher, EventSource, PostAction,
    };
    use std::{
        rc::Rc,
        sync::atomic::{AtomicBool, Ordering},
        time::Duration,
    };

    #[test]
    fn test_transient_drop() {
        // A test source that sets a flag when it's dropped.
        struct TestSource<'a> {
            dropped: &'a AtomicBool,
            ping: PingSource,
        }

        impl Drop for TestSource<'_> {
            fn drop(&mut self) {
                self.dropped.store(true, Ordering::Relaxed)
            }
        }

        impl crate::EventSource for TestSource<'_> {
            type Event = ();
            type Metadata = ();
            type Ret = ();
            type Error = Box<dyn std::error::Error + Sync + Send>;

            fn process_events<F>(
                &mut self,
                readiness: crate::Readiness,
                token: crate::Token,
                callback: F,
            ) -> Result<crate::PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                self.ping.process_events(readiness, token, callback)?;
                Ok(PostAction::Remove)
            }

            fn register(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.ping.register(poll, token_factory)
            }

            fn reregister(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.ping.reregister(poll, token_factory)
            }

            fn unregister(&mut self, poll: &mut crate::Poll) -> crate::Result<()> {
                self.ping.unregister(poll)
            }
        }

        // Test that the inner source is actually dropped when it asks to be
        // removed from the loop, while the TransientSource remains. We use two
        // flags for this:
        // - fired: should be set only when the inner event source has an event
        // - dropped: set by the drop handler for the inner source (it's an
        //   AtomicBool becaues it requires a longer lifetime than the fired
        //   flag)
        let mut fired = false;
        let dropped = false.into();

        // The inner source that should be dropped after the first loop run.
        let (pinger, ping) = make_ping().unwrap();
        let inner = TestSource {
            dropped: &dropped,
            ping,
        };

        // The TransientSource wrapper.
        let outer: TransientSource<_> = inner.into();

        let mut event_loop = crate::EventLoop::try_new().unwrap();
        let handle = event_loop.handle();

        let _token = handle
            .insert_source(outer, |_, _, fired| {
                *fired = true;
            })
            .unwrap();

        // First loop run: the ping generates an event for the inner source.
        pinger.ping();

        event_loop.dispatch(Duration::ZERO, &mut fired).unwrap();

        assert!(fired);
        assert!(dropped.load(Ordering::Relaxed));

        // Second loop run: the ping does nothing because the receiver has been
        // dropped.
        fired = false;

        pinger.ping();

        event_loop.dispatch(Duration::ZERO, &mut fired).unwrap();
        assert!(!fired);
    }

    #[test]
    fn test_transient_passthrough() {
        // Test that event processing works when a source is nested inside a
        // TransientSource. In particular, we want to ensure that the final
        // event is received even if it corresponds to that same event source
        // returning `PostAction::Remove`.
        let (sender, receiver) = channel();
        let outer: TransientSource<_> = receiver.into();

        let mut event_loop = crate::EventLoop::try_new().unwrap();
        let handle = event_loop.handle();

        // Our callback puts the receied events in here for us to check later.
        let mut msg_queue = vec![];

        let _token = handle
            .insert_source(outer, |msg, _, queue: &mut Vec<_>| {
                queue.push(msg);
            })
            .unwrap();

        // Send some data and drop the sender. We specifically want to test that
        // we get the "closed" message.
        sender.send(0u32).unwrap();
        sender.send(1u32).unwrap();
        sender.send(2u32).unwrap();
        sender.send(3u32).unwrap();
        drop(sender);

        // Run loop once to process events.
        event_loop.dispatch(Duration::ZERO, &mut msg_queue).unwrap();

        assert!(matches!(
            msg_queue.as_slice(),
            &[
                Event::Msg(0u32),
                Event::Msg(1u32),
                Event::Msg(2u32),
                Event::Msg(3u32),
                Event::Closed
            ]
        ));
    }

    #[test]
    fn test_transient_map() {
        struct IdSource {
            id: u32,
            ping: PingSource,
        }

        impl EventSource for IdSource {
            type Event = u32;
            type Metadata = ();
            type Ret = ();
            type Error = Box<dyn std::error::Error + Sync + Send>;

            fn process_events<F>(
                &mut self,
                readiness: crate::Readiness,
                token: crate::Token,
                mut callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                let id = self.id;
                self.ping
                    .process_events(readiness, token, |_, md| callback(id, md))?;

                let action = if self.id > 2 {
                    PostAction::Remove
                } else {
                    PostAction::Continue
                };

                Ok(action)
            }

            fn register(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.ping.register(poll, token_factory)
            }

            fn reregister(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.ping.reregister(poll, token_factory)
            }

            fn unregister(&mut self, poll: &mut crate::Poll) -> crate::Result<()> {
                self.ping.unregister(poll)
            }
        }

        struct WrapperSource(TransientSource<IdSource>);

        impl EventSource for WrapperSource {
            type Event = <IdSource as EventSource>::Event;
            type Metadata = <IdSource as EventSource>::Metadata;
            type Ret = <IdSource as EventSource>::Ret;
            type Error = <IdSource as EventSource>::Error;

            fn process_events<F>(
                &mut self,
                readiness: crate::Readiness,
                token: crate::Token,
                callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                let action = self.0.process_events(readiness, token, callback);
                self.0.map(|inner| inner.id += 1);
                action
            }

            fn register(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.0.map(|inner| inner.id += 1);
                self.0.register(poll, token_factory)
            }

            fn reregister(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.0.map(|inner| inner.id += 1);
                self.0.reregister(poll, token_factory)
            }

            fn unregister(&mut self, poll: &mut crate::Poll) -> crate::Result<()> {
                self.0.map(|inner| inner.id += 1);
                self.0.unregister(poll)
            }
        }

        // To test the id later.
        let mut id = 0;

        // Create our source.
        let (pinger, ping) = make_ping().unwrap();
        let inner = IdSource { id, ping };

        // The TransientSource wrapper.
        let outer: TransientSource<_> = inner.into();

        // The top level source.
        let top = WrapperSource(outer);

        // Create a dispatcher so we can check the source afterwards.
        let dispatcher = Dispatcher::new(top, |got_id, _, test_id| {
            *test_id = got_id;
        });

        let mut event_loop = crate::EventLoop::try_new().unwrap();
        let handle = event_loop.handle();

        let token = handle.register_dispatcher(dispatcher.clone()).unwrap();

        // First loop run: the ping generates an event for the inner source.
        // The ID should be 1 after the increment in register().
        pinger.ping();
        event_loop.dispatch(Duration::ZERO, &mut id).unwrap();
        assert_eq!(id, 1);

        // Second loop run: the ID should be 2 after the previous
        // process_events().
        pinger.ping();
        event_loop.dispatch(Duration::ZERO, &mut id).unwrap();
        assert_eq!(id, 2);

        // Third loop run: the ID should be 3 after another process_events().
        pinger.ping();
        event_loop.dispatch(Duration::ZERO, &mut id).unwrap();
        assert_eq!(id, 3);

        // Fourth loop run: the callback is no longer called by the inner
        // source, so our local ID is not incremented.
        pinger.ping();
        event_loop.dispatch(Duration::ZERO, &mut id).unwrap();
        assert_eq!(id, 3);

        // Remove the dispatcher so we can inspect the sources.
        handle.remove(token);

        let mut top_after = dispatcher.into_source_inner();

        // I expect the inner source to be dropped, so the TransientSource
        // variant is None (its version of None, not Option::None), so its map()
        // won't call the passed-in function (hence the unreachable!()) and its
        // return value should be Option::None.
        assert!(top_after.0.map(|_| unreachable!()).is_none());
    }

    #[test]
    fn test_transient_disable() {
        // Test that disabling and enabling is handled properly.
        struct DisablingSource(PingSource);

        impl EventSource for DisablingSource {
            type Event = ();
            type Metadata = ();
            type Ret = ();
            type Error = Box<dyn std::error::Error + Sync + Send>;

            fn process_events<F>(
                &mut self,
                readiness: crate::Readiness,
                token: crate::Token,
                callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                self.0.process_events(readiness, token, callback)?;
                Ok(PostAction::Disable)
            }

            fn register(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.0.register(poll, token_factory)
            }

            fn reregister(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.0.reregister(poll, token_factory)
            }

            fn unregister(&mut self, poll: &mut crate::Poll) -> crate::Result<()> {
                self.0.unregister(poll)
            }
        }

        // Flag for checking when the source fires.
        let mut fired = false;

        // Create our source.
        let (pinger, ping) = make_ping().unwrap();

        let inner = DisablingSource(ping);

        // The TransientSource wrapper.
        let outer: TransientSource<_> = inner.into();

        let mut event_loop = crate::EventLoop::try_new().unwrap();
        let handle = event_loop.handle();
        let token = handle
            .insert_source(outer, |_, _, fired| {
                *fired = true;
            })
            .unwrap();

        // Ping here and not later, to check that disabling after an event is
        // triggered but not processed does not discard the event.
        pinger.ping();
        event_loop.dispatch(Duration::ZERO, &mut fired).unwrap();
        assert!(fired);

        // Source should now be disabled.
        pinger.ping();
        fired = false;
        event_loop.dispatch(Duration::ZERO, &mut fired).unwrap();
        assert!(!fired);

        // Re-enable the source.
        handle.enable(&token).unwrap();

        // Trigger another event.
        pinger.ping();
        fired = false;
        event_loop.dispatch(Duration::ZERO, &mut fired).unwrap();
        assert!(fired);
    }

    #[test]
    fn test_transient_replace_unregister() {
        // This is a bit of a complex test, but it essentially boils down to:
        // how can a "parent" event source containing a TransientSource replace
        // the "child" source without leaking the source's registration?

        // First, a source that finishes immediately. This is so we cover the
        // edge case of replacing a source as soon as it wants to be removed.
        struct FinishImmediatelySource {
            source: PingSource,
            data: Option<i32>,
            registered: bool,
            dropped: Rc<AtomicBool>,
        }

        impl FinishImmediatelySource {
            // The constructor passes out the drop flag so we can check that
            // this source was or wasn't dropped.
            fn new(source: PingSource, data: i32) -> (Self, Rc<AtomicBool>) {
                let dropped = Rc::new(false.into());

                (
                    Self {
                        source,
                        data: Some(data),
                        registered: false,
                        dropped: Rc::clone(&dropped),
                    },
                    dropped,
                )
            }
        }

        impl EventSource for FinishImmediatelySource {
            type Event = i32;
            type Metadata = ();
            type Ret = ();
            type Error = Box<dyn std::error::Error + Sync + Send>;

            fn process_events<F>(
                &mut self,
                readiness: crate::Readiness,
                token: crate::Token,
                mut callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                let mut data = self.data.take();

                self.source.process_events(readiness, token, |_, _| {
                    if let Some(data) = data.take() {
                        callback(data, &mut ())
                    }
                })?;

                self.data = data;

                Ok(if self.data.is_none() {
                    PostAction::Remove
                } else {
                    PostAction::Continue
                })
            }

            fn register(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.registered = true;
                self.source.register(poll, token_factory)
            }

            fn reregister(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.source.reregister(poll, token_factory)
            }

            fn unregister(&mut self, poll: &mut crate::Poll) -> crate::Result<()> {
                self.registered = false;
                self.source.unregister(poll)
            }
        }

        // The drop handler sets a flag we can check for debugging (we want to
        // know that the source itself was dropped), and also checks that the
        // source was unregistered. Ultimately neither the source nor its
        // registration should be leaked.

        impl Drop for FinishImmediatelySource {
            fn drop(&mut self) {
                assert!(!self.registered, "source dropped while still registered");
                self.dropped.store(true, Ordering::Relaxed);
            }
        }

        // Our wrapper source handles detecting when the child source finishes,
        // and replacing that child source with another one that will generate
        // more events. This is one intended use case of the TransientSource.

        struct WrapperSource {
            current: TransientSource<FinishImmediatelySource>,
            replacement: Option<FinishImmediatelySource>,
            dropped: Rc<AtomicBool>,
        }

        impl WrapperSource {
            // The constructor passes out the drop flag so we can check that
            // this source was or wasn't dropped.
            fn new(
                first: FinishImmediatelySource,
                second: FinishImmediatelySource,
            ) -> (Self, Rc<AtomicBool>) {
                let dropped = Rc::new(false.into());

                (
                    Self {
                        current: first.into(),
                        replacement: second.into(),
                        dropped: Rc::clone(&dropped),
                    },
                    dropped,
                )
            }
        }

        impl EventSource for WrapperSource {
            type Event = i32;
            type Metadata = ();
            type Ret = ();
            type Error = Box<dyn std::error::Error + Sync + Send>;

            fn process_events<F>(
                &mut self,
                readiness: crate::Readiness,
                token: crate::Token,
                mut callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                // Did our event source generate an event?
                let mut fired = false;

                let post_action = self.current.process_events(readiness, token, |data, _| {
                    callback(data, &mut ());
                    fired = true;
                })?;

                if fired {
                    // The event source will be unregistered after the current
                    // process_events() iteration is finished. The replace()
                    // method will handle doing that even while we've added a
                    // new source.
                    if let Some(replacement) = self.replacement.take() {
                        self.current.replace(replacement);
                    }

                    // Parent source is responsible for flagging this, but it's
                    // already set.
                    assert_eq!(post_action, PostAction::Reregister);
                }

                Ok(post_action)
            }

            fn register(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.current.register(poll, token_factory)
            }

            fn reregister(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.current.reregister(poll, token_factory)
            }

            fn unregister(&mut self, poll: &mut crate::Poll) -> crate::Result<()> {
                self.current.unregister(poll)
            }
        }

        impl Drop for WrapperSource {
            fn drop(&mut self) {
                self.dropped.store(true, Ordering::Relaxed);
            }
        }

        // Construct the various nested sources - FinishImmediatelySource inside
        // TransientSource inside WrapperSource. The numbers let us verify which
        // event source fires first.
        let (ping0_tx, ping0_rx) = crate::ping::make_ping().unwrap();
        let (ping1_tx, ping1_rx) = crate::ping::make_ping().unwrap();
        let (inner0, inner0_dropped) = FinishImmediatelySource::new(ping0_rx, 0);
        let (inner1, inner1_dropped) = FinishImmediatelySource::new(ping1_rx, 1);
        let (outer, outer_dropped) = WrapperSource::new(inner0, inner1);

        // Now the actual test starts.

        let mut event_loop: crate::EventLoop<(Option<i32>, crate::LoopSignal)> =
            crate::EventLoop::try_new().unwrap();
        let handle = event_loop.handle();
        let signal = event_loop.get_signal();

        // This is how we communicate with the event sources.
        let mut context = (None, signal);

        let _token = handle
            .insert_source(outer, |data, _, (evt, sig)| {
                *evt = Some(data);
                sig.stop();
            })
            .unwrap();

        // Ensure our sources fire.
        ping0_tx.ping();
        ping1_tx.ping();

        // Use run() rather than dispatch() because it's not strictly part of
        // any API contract as to how many runs of the event loop it takes to
        // replace the nested source.
        event_loop.run(None, &mut context, |_| {}).unwrap();

        // First, make sure the inner source actually did fire.
        assert_eq!(context.0.take(), Some(0), "first inner source did not fire");

        // Make sure that the outer source is still alive.
        assert!(
            !outer_dropped.load(Ordering::Relaxed),
            "outer source already dropped"
        );

        // Make sure that the inner child source IS dropped now.
        assert!(
            inner0_dropped.load(Ordering::Relaxed),
            "first inner source not dropped"
        );

        // Make sure that, in between the first event and second event, the
        // replacement child source still exists.
        assert!(
            !inner1_dropped.load(Ordering::Relaxed),
            "replacement inner source dropped"
        );

        // Run the event loop until we get a second event.
        event_loop.run(None, &mut context, |_| {}).unwrap();

        // Ensure the replacement source fired (which checks that it was
        // registered and is being processed by the TransientSource).
        assert_eq!(context.0.take(), Some(1), "replacement source did not fire");
    }

    #[test]
    fn test_transient_remove() {
        // This tests that calling remove(), even before an event source has
        // requested its own removal, results in the event source being removed.

        const STOP_AT: i32 = 2;

        // A wrapper source to automate the removal of the inner source.
        struct WrapperSource {
            inner: TransientSource<Channel<i32>>,
        }

        impl EventSource for WrapperSource {
            type Event = i32;
            type Metadata = ();
            type Ret = ();
            type Error = Box<dyn std::error::Error + Sync + Send>;

            fn process_events<F>(
                &mut self,
                readiness: crate::Readiness,
                token: crate::Token,
                mut callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                let mut remove = false;

                let mut post_action = self.inner.process_events(readiness, token, |evt, _| {
                    if let Event::Msg(num) = evt {
                        callback(num, &mut ());
                        remove = num >= STOP_AT;
                    }
                })?;

                if remove {
                    self.inner.remove();
                    post_action |= PostAction::Reregister;
                }

                Ok(post_action)
            }

            fn register(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.inner.register(poll, token_factory)
            }

            fn reregister(
                &mut self,
                poll: &mut crate::Poll,
                token_factory: &mut crate::TokenFactory,
            ) -> crate::Result<()> {
                self.inner.reregister(poll, token_factory)
            }

            fn unregister(&mut self, poll: &mut crate::Poll) -> crate::Result<()> {
                self.inner.unregister(poll)
            }
        }

        // Create our sources and loop.

        let (sender, receiver) = channel();
        let wrapper = WrapperSource {
            inner: receiver.into(),
        };

        let mut event_loop = crate::EventLoop::try_new().unwrap();
        let handle = event_loop.handle();

        handle
            .insert_source(wrapper, |num, _, out: &mut Option<_>| {
                *out = Some(num);
            })
            .unwrap();

        // Storage for callback data.
        let mut out = None;

        // Send some data we expect to get callbacks for.
        for num in 0..=STOP_AT {
            sender.send(num).unwrap();
            event_loop.dispatch(Duration::ZERO, &mut out).unwrap();
            assert_eq!(out.take(), Some(num));
        }

        // Now we expect the receiver to be gone.
        assert!(matches!(
            sender.send(STOP_AT + 1),
            Err(std::sync::mpsc::SendError { .. })
        ));
    }
}
