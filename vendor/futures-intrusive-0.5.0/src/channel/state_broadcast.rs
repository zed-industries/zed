//! An asynchronously awaitable state broadcasting channel

use super::{ChannelSendError, CloseStatus};
use crate::{
    intrusive_double_linked_list::{LinkedList, ListNode},
    utils::update_waker_ref,
    NoopLock,
};
use core::marker::PhantomData;
use core::pin::Pin;
use futures_core::{
    future::{FusedFuture, Future},
    task::{Context, Poll, Waker},
};
use lock_api::{Mutex, RawMutex};

/// An ID, which allows to differentiate states received from a Channel.
/// Elements with a bigger state ID (`id > otherId`) have been published more
/// recently into the Channel.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub struct StateId(u64);

impl StateId {
    /// Returns the initial StateId, which is guaranteed to return the
    /// oldest buffered value available.
    pub fn new() -> Self {
        StateId(0)
    }
}

/// Tracks how the future had interacted with the channel
#[derive(PartialEq, Debug)]
pub enum RecvPollState {
    /// The task is not registered at the wait queue at the channel
    Unregistered,
    /// The task was added to the wait queue at the channel.
    Registered,
}

/// Tracks the channel futures waiting state.
/// Access to this struct is synchronized through the channel.
#[derive(Debug)]
pub struct RecvWaitQueueEntry {
    /// The task handle of the waiting task
    task: Option<Waker>,
    /// Current polling state
    state: RecvPollState,
    /// The minimum state ID we are waiting for
    state_id: StateId,
}

impl RecvWaitQueueEntry {
    /// Creates a new RecvWaitQueueEntry
    pub fn new(state_id: StateId) -> RecvWaitQueueEntry {
        RecvWaitQueueEntry {
            task: None,
            state_id,
            state: RecvPollState::Unregistered,
        }
    }
}

/// Adapter trait that allows Futures to generically interact with Channel
/// implementations via dynamic dispatch.
pub trait ChannelReceiveAccess<T> {
    unsafe fn receive_or_register(
        &self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<(StateId, T)>>;

    fn remove_receive_waiter(
        &self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
    );
}

/// A Future that is returned by the `receive` function on a state broadcast channel.
/// The future gets resolved with `Some((state_id, state))` when a value could be
/// received from the channel.
///
/// `state` represents the new state which had been retrieved from the channel.
///
/// `state_id` is the [`StateId`] which can be passed as a parameter to
/// `receive()` in order to fetch the next state from the channel.
///
/// If the channels gets closed and no items are still enqueued inside the
/// channel, the future will resolve to `None`.
#[must_use = "futures do nothing unless polled"]
pub struct StateReceiveFuture<'a, MutexType, T>
where
    T: Clone,
{
    /// The channel that is associated with this StateReceiveFuture
    channel: Option<&'a dyn ChannelReceiveAccess<T>>,
    /// Node for waiting on the channel
    wait_node: ListNode<RecvWaitQueueEntry>,
    /// Marker for mutex type
    _phantom: PhantomData<MutexType>,
}

// Safety: Channel futures can be sent between threads as long as the underlying
// channel is thread-safe (Sync), which allows to poll/register/unregister from
// a different thread.
unsafe impl<'a, MutexType: Sync, T: Clone + Send> Send
    for StateReceiveFuture<'a, MutexType, T>
{
}

impl<'a, MutexType, T: Clone> core::fmt::Debug
    for StateReceiveFuture<'a, MutexType, T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StateReceiveFuture").finish()
    }
}

impl<'a, MutexType, T: Clone> Future for StateReceiveFuture<'a, MutexType, T> {
    type Output = Option<(StateId, T)>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<(StateId, T)>> {
        // It might be possible to use Pin::map_unchecked here instead of the two unsafe APIs.
        // However this didn't seem to work for some borrow checker reasons

        // Safety: The next operations are safe, because Pin promises us that
        // the address of the wait queue entry inside StateReceiveFuture is stable,
        // and we don't move any fields inside the future until it gets dropped.
        let mut_self: &mut StateReceiveFuture<MutexType, T> =
            unsafe { Pin::get_unchecked_mut(self) };

        let channel = mut_self
            .channel
            .expect("polled StateReceiveFuture after completion");

        let poll_res =
            unsafe { channel.receive_or_register(&mut mut_self.wait_node, cx) };

        if poll_res.is_ready() {
            // A value was available
            mut_self.channel = None;
        }

        poll_res
    }
}

impl<'a, MutexType, T: Clone> FusedFuture
    for StateReceiveFuture<'a, MutexType, T>
{
    fn is_terminated(&self) -> bool {
        self.channel.is_none()
    }
}

impl<'a, MutexType, T: Clone> Drop for StateReceiveFuture<'a, MutexType, T> {
    fn drop(&mut self) {
        // If this StateReceiveFuture has been polled and it was added to the
        // wait queue at the channel, it must be removed before dropping.
        // Otherwise the channel would access invalid memory.
        if let Some(channel) = self.channel {
            channel.remove_receive_waiter(&mut self.wait_node);
        }
    }
}

fn wake_waiters(waiters: &mut LinkedList<RecvWaitQueueEntry>) {
    // Remove all waiters from the waiting list in reverse order and wake them.
    // We reverse the waiter list, so that the oldest waker (which is
    // at the end of the list), gets woken first and has the best
    // chance to grab the channel value.
    waiters.reverse_drain(|waiter| {
        if let Some(handle) = waiter.task.take() {
            handle.wake();
        }
        waiter.state = RecvPollState::Unregistered;
    });
}

/// Internal state of the state broadcast channel
struct ChannelState<T> {
    /// Whether the channel was actively closed
    is_closed: bool,
    /// The ID of the next state.
    state_id: StateId,
    /// The value which is stored inside the channel
    value: Option<T>,
    /// The list of waiters, which are waiting for the channel to get fulfilled
    waiters: LinkedList<RecvWaitQueueEntry>,
}

impl<T> ChannelState<T>
where
    T: Clone,
{
    fn new() -> ChannelState<T> {
        ChannelState::<T> {
            is_closed: false,
            state_id: StateId(0),
            value: None,
            waiters: LinkedList::new(),
        }
    }

    /// Writes a single value to the channel.
    /// If the maximum amount of values had been written, the new value will be rejected.
    fn send(&mut self, value: T) -> Result<(), ChannelSendError<T>> {
        if self.is_closed || self.state_id.0 == core::u64::MAX {
            return Err(ChannelSendError(value));
        }

        self.value = Some(value);
        self.state_id.0 += 1;

        // Wakeup all waiters
        wake_waiters(&mut self.waiters);

        Ok(())
    }

    fn close(&mut self) -> CloseStatus {
        if self.is_closed {
            return CloseStatus::AlreadyClosed;
        }
        self.is_closed = true;

        // Wakeup all waiters
        wake_waiters(&mut self.waiters);

        CloseStatus::NewlyClosed
    }

    fn try_receive(&mut self, state_id: StateId) -> Option<(StateId, T)> {
        let val = self.value.as_ref()?;
        if state_id < self.state_id {
            Some((self.state_id, val.clone()))
        } else {
            None
        }
    }

    /// Tries to read the value from the channel.
    /// If the value isn't available yet, the StateReceiveFuture gets added to the
    /// wait queue at the channel, and will be signalled once ready.
    /// This function is only safe as long as the `wait_node`s address is guaranteed
    /// to be stable until it gets removed from the queue.
    unsafe fn receive_or_register(
        &mut self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<(StateId, T)>> {
        match wait_node.state {
            RecvPollState::Unregistered => {
                // The caller must wait for a value if either there is no value
                // available yet, or if the value isn't newer than what the
                // caller requested.
                let val_to_deliver = match &self.value {
                    Some(ref v) if wait_node.state_id < self.state_id => {
                        Some(v.clone())
                    }
                    Some(_) | None => None,
                };

                match val_to_deliver {
                    Some(v) => {
                        // A value that satisfies the caller is available.
                        Poll::Ready(Some((self.state_id, v)))
                    }
                    None => {
                        // Check if something was written into the channel before
                        // or the channel was closed.
                        if self.is_closed {
                            Poll::Ready(None)
                        } else {
                            // Added the task to the wait queue
                            wait_node.task = Some(cx.waker().clone());
                            wait_node.state = RecvPollState::Registered;
                            self.waiters.add_front(wait_node);
                            Poll::Pending
                        }
                    }
                }
            }
            RecvPollState::Registered => {
                // Since the channel wakes up all waiters and moves their states
                // to unregistered there can't be any value in the channel in this state.
                // However the caller might have passed a different `Waker`.
                // In this case we need to update it.
                update_waker_ref(&mut wait_node.task, cx);
                Poll::Pending
            }
        }
    }

    fn remove_waiter(&mut self, wait_node: &mut ListNode<RecvWaitQueueEntry>) {
        // StateReceiveFuture only needs to get removed if it had been added to
        // the wait queue of the channel. This has happened in the RecvPollState::Waiting case.
        if let RecvPollState::Registered = wait_node.state {
            // Safety: Due to the state, we know that the node must be part
            // of the waiter list
            if !unsafe { self.waiters.remove(wait_node) } {
                // Panic if the address isn't found. This can only happen if the contract was
                // violated, e.g. the RecvWaitQueueEntry got moved after the initial poll.
                panic!("Future could not be removed from wait queue");
            }
            wait_node.state = RecvPollState::Unregistered;
        }
    }
}

/// A channel which can be used to synchronize the state between a sender an
/// arbitrary number of receivers.
///
/// The sender can publish its state.
///
/// The receivers can wait for state updates by announcing the most recent state
/// that is already known to them.
pub struct GenericStateBroadcastChannel<MutexType: RawMutex, T> {
    inner: Mutex<MutexType, ChannelState<T>>,
}

// The channel can be sent to other threads as long as it's not borrowed and the
// value in it can be sent to other threads.
unsafe impl<MutexType: RawMutex + Send, T: Send> Send
    for GenericStateBroadcastChannel<MutexType, T>
{
}
// The channel is thread-safe as long as a thread-safe mutex is used
unsafe impl<MutexType: RawMutex + Sync, T: Send> Sync
    for GenericStateBroadcastChannel<MutexType, T>
{
}

impl<MutexType: RawMutex, T> core::fmt::Debug
    for GenericStateBroadcastChannel<MutexType, T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GenericStateBroadcastChannel").finish()
    }
}

impl<MutexType: RawMutex, T> GenericStateBroadcastChannel<MutexType, T>
where
    T: Clone,
{
    /// Creates a new State Broadcast Channel in the given state
    pub fn new() -> GenericStateBroadcastChannel<MutexType, T>
    where
        T: Clone,
    {
        GenericStateBroadcastChannel {
            inner: Mutex::new(ChannelState::new()),
        }
    }

    /// Writes a single value to the channel.
    ///
    /// This will notify waiters about the availability of the value.
    /// If the maximum amount of values had been written to the channel,
    /// or if the channel is closed, the new value will be rejected and
    /// returned inside the error variant.
    pub fn send(&self, value: T) -> Result<(), ChannelSendError<T>> {
        self.inner.lock().send(value)
    }

    /// Closes the channel.
    ///
    /// This will notify waiters about closure, by fulfilling pending `Future`s
    /// with `None`.
    /// `send(value)` attempts which follow this call will fail with a
    /// [`ChannelSendError`].
    pub fn close(&self) -> CloseStatus {
        self.inner.lock().close()
    }

    /// Returns a future that gets fulfilled when a value is written to the channel
    /// or the channel is closed.
    /// `state_id` specifies the minimum state ID that should be retrieved
    /// by the `receive` operation.
    ///
    /// The returned [`StateReceiveFuture`] will get fulfilled with the
    /// retrieved value as well as the [`StateId`] which is required to retrieve
    /// the following state.
    pub fn receive(
        &self,
        state_id: StateId,
    ) -> StateReceiveFuture<MutexType, T> {
        StateReceiveFuture {
            channel: Some(self),
            wait_node: ListNode::new(RecvWaitQueueEntry::new(state_id)),
            _phantom: PhantomData,
        }
    }

    /// Attempt to retrieve a value whose `StateId` is greater than the one provided.
    ///
    /// Returns `None` if no value is found in the channel, or if the current `StateId`
    /// of the value is less or equal to the one provided.
    pub fn try_receive(&self, state_id: StateId) -> Option<(StateId, T)> {
        self.inner.lock().try_receive(state_id)
    }
}

impl<MutexType: RawMutex, T: Clone> ChannelReceiveAccess<T>
    for GenericStateBroadcastChannel<MutexType, T>
{
    unsafe fn receive_or_register(
        &self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<(StateId, T)>> {
        self.inner.lock().receive_or_register(wait_node, cx)
    }

    fn remove_receive_waiter(
        &self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
    ) {
        self.inner.lock().remove_waiter(wait_node)
    }
}

// Export a non thread-safe version using NoopLock

/// A [`GenericStateBroadcastChannel`] which is not thread-safe.
pub type LocalStateBroadcastChannel<T> =
    GenericStateBroadcastChannel<NoopLock, T>;

#[cfg(feature = "std")]
mod if_std {
    use super::*;

    // Export a thread-safe version using parking_lot::RawMutex

    /// A [`GenericStateBroadcastChannel`] implementation backed by [`parking_lot`].
    pub type StateBroadcastChannel<T> =
        GenericStateBroadcastChannel<parking_lot::RawMutex, T>;
}

#[cfg(feature = "std")]
pub use self::if_std::*;

#[cfg(feature = "alloc")]
mod if_alloc {
    use super::*;

    pub mod shared {
        use super::*;
        use core::sync::atomic::{AtomicUsize, Ordering};

        struct GenericStateBroadcastChannelSharedState<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone + 'static,
        {
            /// The amount of [`GenericSender`] instances which reference this state.
            senders: AtomicUsize,
            /// The amount of [`GenericReceiver`] instances which reference this state.
            receivers: AtomicUsize,
            /// The channel on which is acted.
            channel: GenericStateBroadcastChannel<MutexType, T>,
        }

        // Implement ChannelReceiveAccess trait for SharedChannelState, so that it can
        // be used for dynamic dispatch in futures.
        impl<MutexType, T> ChannelReceiveAccess<T>
            for GenericStateBroadcastChannelSharedState<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone + 'static,
        {
            unsafe fn receive_or_register(
                &self,
                wait_node: &mut ListNode<RecvWaitQueueEntry>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<(StateId, T)>> {
                self.channel.receive_or_register(wait_node, cx)
            }

            fn remove_receive_waiter(
                &self,
                wait_node: &mut ListNode<RecvWaitQueueEntry>,
            ) {
                self.channel.remove_receive_waiter(wait_node)
            }
        }

        /// A Future that is returned by the `receive` function on a state broadcast channel.
        /// The future gets resolved with `Some((state_id, state))` when a value could be
        /// received from the channel.
        ///
        /// `state` represents the new state which had been retrieved from the channel.
        ///
        /// `state_id` is the [`StateId`] which can be passed as a parameter to
        /// `receive()` in order to fetch the next state from the channel.
        ///
        /// If the channels gets closed and no items are still enqueued inside the
        /// channel, the future will resolve to `None`.
        #[must_use = "futures do nothing unless polled"]
        pub struct StateReceiveFuture<MutexType, T> {
            /// The Channel that is associated with this StateReceiveFuture
            channel: Option<alloc::sync::Arc<dyn ChannelReceiveAccess<T>>>,
            /// Node for waiting on the channel
            wait_node: ListNode<RecvWaitQueueEntry>,
            /// Marker for mutex type
            _phantom: PhantomData<MutexType>,
        }

        // Safety: Channel futures can be sent between threads as long as the underlying
        // channel is thread-safe (Sync), which allows to poll/register/unregister from
        // a different thread.
        unsafe impl<MutexType: Sync, T: Clone + Send> Send
            for StateReceiveFuture<MutexType, T>
        {
        }

        impl<MutexType, T> core::fmt::Debug for StateReceiveFuture<MutexType, T> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("StateReceiveFuture").finish()
            }
        }

        impl<MutexType, T> Future for StateReceiveFuture<MutexType, T> {
            type Output = Option<(StateId, T)>;

            fn poll(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<(StateId, T)>> {
                // It might be possible to use Pin::map_unchecked here instead of the two unsafe APIs.
                // However this didn't seem to work for some borrow checker reasons

                // Safety: The next operations are safe, because Pin promises us that
                // the address of the wait queue entry inside StateReceiveFuture is stable,
                // and we don't move any fields inside the future until it gets dropped.
                let mut_self: &mut StateReceiveFuture<MutexType, T> =
                    unsafe { Pin::get_unchecked_mut(self) };

                let channel = mut_self
                    .channel
                    .take()
                    .expect("polled StateReceiveFuture after completion");

                let poll_res = unsafe {
                    channel.receive_or_register(&mut mut_self.wait_node, cx)
                };

                if poll_res.is_ready() {
                    // A value was available
                    mut_self.channel = None;
                } else {
                    mut_self.channel = Some(channel)
                }

                poll_res
            }
        }

        impl<MutexType, T> FusedFuture for StateReceiveFuture<MutexType, T> {
            fn is_terminated(&self) -> bool {
                self.channel.is_none()
            }
        }

        impl<MutexType, T> Drop for StateReceiveFuture<MutexType, T> {
            fn drop(&mut self) {
                // If this StateReceiveFuture has been polled and it was added to the
                // wait queue at the channel, it must be removed before dropping.
                // Otherwise the channel would access invalid memory.
                if let Some(channel) = &self.channel {
                    channel.remove_receive_waiter(&mut self.wait_node);
                }
            }
        }

        /// The sending side of a channel which can be used to exchange values
        /// between concurrent tasks.
        ///
        /// Values can be sent into the channel through `send`.
        pub struct GenericStateSender<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone + 'static,
        {
            inner: alloc::sync::Arc<
                GenericStateBroadcastChannelSharedState<MutexType, T>,
            >,
        }

        /// The receiving side of a channel which can be used to exchange values
        /// between concurrent tasks.
        ///
        /// Tasks can receive values from the channel through the `receive` method.
        /// The returned Future will get resolved when a value is sent into the channel.
        pub struct GenericStateReceiver<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone + 'static,
        {
            inner: alloc::sync::Arc<
                GenericStateBroadcastChannelSharedState<MutexType, T>,
            >,
        }

        impl<MutexType, T> core::fmt::Debug for GenericStateSender<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("StateSender").finish()
            }
        }

        impl<MutexType, T> core::fmt::Debug for GenericStateReceiver<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("StateReceiver").finish()
            }
        }

        impl<MutexType, T> Clone for GenericStateSender<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone,
        {
            fn clone(&self) -> Self {
                let old_size =
                    self.inner.senders.fetch_add(1, Ordering::Relaxed);
                if old_size > (core::isize::MAX) as usize {
                    panic!("Reached maximum refcount");
                }
                GenericStateSender {
                    inner: self.inner.clone(),
                }
            }
        }

        impl<MutexType, T> Drop for GenericStateSender<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone,
        {
            fn drop(&mut self) {
                if self.inner.senders.fetch_sub(1, Ordering::Release) != 1 {
                    return;
                }
                core::sync::atomic::fence(Ordering::Acquire);
                // Close the channel, before last sender gets destroyed
                // TODO: We could potentially avoid this, if no receiver is left
                self.inner.channel.close();
            }
        }

        impl<MutexType, T> Clone for GenericStateReceiver<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone,
        {
            fn clone(&self) -> Self {
                let old_size =
                    self.inner.receivers.fetch_add(1, Ordering::Relaxed);
                if old_size > (core::isize::MAX) as usize {
                    panic!("Reached maximum refcount");
                }
                GenericStateReceiver {
                    inner: self.inner.clone(),
                }
            }
        }

        impl<MutexType, T> Drop for GenericStateReceiver<MutexType, T>
        where
            MutexType: RawMutex,
            T: Clone,
        {
            fn drop(&mut self) {
                if self.inner.receivers.fetch_sub(1, Ordering::Release) != 1 {
                    return;
                }
                core::sync::atomic::fence(Ordering::Acquire);
                // Close the channel, before last receiver gets destroyed
                // TODO: We could potentially avoid this, if no sender is left
                self.inner.channel.close();
            }
        }

        /// Creates a new state broadcast channel which can be used to exchange values
        /// of type `T` between concurrent tasks.
        /// The ends of the Channel are represented through
        /// the returned Sender and Receiver.
        ///
        /// As soon es either the senders or receivers is closed, the channel
        /// itself will be closed.
        pub fn generic_state_broadcast_channel<MutexType, T>() -> (
            GenericStateSender<MutexType, T>,
            GenericStateReceiver<MutexType, T>,
        )
        where
            MutexType: RawMutex,
            T: Clone + Send,
        {
            let inner = alloc::sync::Arc::new(
                GenericStateBroadcastChannelSharedState {
                    channel: GenericStateBroadcastChannel::new(),
                    senders: AtomicUsize::new(1),
                    receivers: AtomicUsize::new(1),
                },
            );

            let sender = GenericStateSender {
                inner: inner.clone(),
            };
            let receiver = GenericStateReceiver { inner };

            (sender, receiver)
        }

        impl<MutexType, T> GenericStateSender<MutexType, T>
        where
            MutexType: RawMutex + 'static,
            T: Clone,
        {
            /// Writes a single value to the channel.
            ///
            /// This will notify waiters about the availability of the value.
            /// If a value had been written to the channel before, or if the
            /// channel is closed, the new value will be rejected and
            /// returned inside the error variant.
            pub fn send(&self, value: T) -> Result<(), ChannelSendError<T>> {
                self.inner.channel.send(value)
            }
        }

        impl<MutexType, T> GenericStateReceiver<MutexType, T>
        where
            MutexType: RawMutex + 'static,
            T: Clone,
        {
            /// Returns a future that gets fulfilled when a value is written to the channel
            /// or the channel is closed.
            /// `state_id` specifies the minimum state ID that should be retrieved
            /// by the `receive` operation.
            ///
            /// The returned [`StateReceiveFuture`] will get fulfilled with the
            /// retrieved value as well as the [`StateId`] which is required to retrieve
            /// the following state
            pub fn receive(
                &self,
                state_id: StateId,
            ) -> StateReceiveFuture<MutexType, T> {
                StateReceiveFuture {
                    channel: Some(self.inner.clone()),
                    wait_node: ListNode::new(RecvWaitQueueEntry::new(state_id)),
                    _phantom: PhantomData,
                }
            }

            /// Attempt to retrieve a value whose `StateId` is greater than the one provided.
            ///
            /// Returns `None` if no value is found in the channel, or if the current `StateId`
            /// of the value is less or equal to the one provided.
            pub fn try_receive(
                &self,
                state_id: StateId,
            ) -> Option<(StateId, T)> {
                self.inner.channel.try_receive(state_id)
            }
        }

        // Export parking_lot based shared channels in std mode
        #[cfg(feature = "std")]
        mod if_std {
            use super::*;

            /// A [`GenericStateSender`] implementation backed by [`parking_lot`].
            pub type StateSender<T> =
                GenericStateSender<parking_lot::RawMutex, T>;
            /// A [`GenericStateReceiver`] implementation backed by [`parking_lot`].
            pub type StateReceiver<T> =
                GenericStateReceiver<parking_lot::RawMutex, T>;

            /// Creates a new state broadcast channel.
            ///
            /// Refer to [`generic_state_broadcast_channel`] for details.
            ///
            /// Example for creating a channel to transmit an integer value:
            ///
            /// ```
            /// # use futures_intrusive::channel::shared::state_broadcast_channel;
            /// let (sender, receiver) = state_broadcast_channel::<i32>();
            /// ```
            pub fn state_broadcast_channel<T>(
            ) -> (StateSender<T>, StateReceiver<T>)
            where
                T: Clone + Send,
            {
                generic_state_broadcast_channel::<parking_lot::RawMutex, T>()
            }
        }

        #[cfg(feature = "std")]
        pub use self::if_std::*;
    }
}

#[cfg(feature = "alloc")]
pub use self::if_alloc::*;
