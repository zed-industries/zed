//! An asynchronously awaitable oneshot channel

use super::{
    ChannelReceiveAccess, ChannelReceiveFuture, ChannelSendError, CloseStatus,
    RecvPollState, RecvWaitQueueEntry,
};
use crate::{
    intrusive_double_linked_list::{LinkedList, ListNode},
    utils::update_waker_ref,
    NoopLock,
};
use core::marker::PhantomData;
use futures_core::task::{Context, Poll};
use lock_api::{Mutex, RawMutex};

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

/// Internal state of the oneshot channel
struct ChannelState<T> {
    /// Whether the channel had been fulfilled before
    is_fulfilled: bool,
    /// The value which is stored inside the channel
    value: Option<T>,
    /// The list of waiters, which are waiting for the channel to get fulfilled
    waiters: LinkedList<RecvWaitQueueEntry>,
}

impl<T> ChannelState<T> {
    fn new() -> ChannelState<T> {
        ChannelState::<T> {
            is_fulfilled: false,
            value: None,
            waiters: LinkedList::new(),
        }
    }

    /// Writes a single value to the channel.
    /// If a value had been written to the channel before, the new value will be rejected.
    fn send(&mut self, value: T) -> Result<(), ChannelSendError<T>> {
        if self.is_fulfilled {
            return Err(ChannelSendError(value));
        }

        self.value = Some(value);
        self.is_fulfilled = true;

        // Wakeup all waiters
        wake_waiters(&mut self.waiters);

        Ok(())
    }

    fn close(&mut self) -> CloseStatus {
        if self.is_fulfilled {
            return CloseStatus::AlreadyClosed;
        }
        self.is_fulfilled = true;

        // Wakeup all waiters
        wake_waiters(&mut self.waiters);

        CloseStatus::NewlyClosed
    }

    /// Tries to read the value from the channel.
    /// If the value isn't available yet, the ChannelReceiveFuture gets added to the
    /// wait queue at the channel, and will be signalled once ready.
    /// This function is only safe as long as the `wait_node`s address is guaranteed
    /// to be stable until it gets removed from the queue.
    unsafe fn try_receive(
        &mut self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<T>> {
        match wait_node.state {
            RecvPollState::Unregistered => {
                let maybe_val = self.value.take();
                match maybe_val {
                    Some(v) => {
                        // A value was available inside the channel and was fetched
                        Poll::Ready(Some(v))
                    }
                    None => {
                        // Check if something was written into the channel before
                        // or the channel was closed.
                        if self.is_fulfilled {
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
            RecvPollState::Notified => {
                unreachable!("Not possible for Oneshot");
            }
        }
    }

    fn remove_waiter(&mut self, wait_node: &mut ListNode<RecvWaitQueueEntry>) {
        // ChannelReceiveFuture only needs to get removed if it had been added to
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

/// A channel which can be used to exchange a single value between two
/// concurrent tasks.
///
/// Tasks can wait for the value to get delivered via `receive`.
/// The returned Future will get fulfilled when a value is sent into the channel.
///
/// The value can only be extracted by a single receiving task. Once the value
/// has been retrieved from the Channel, the Channel is closed and subsequent
/// receive calls will return `None`.
pub struct GenericOneshotChannel<MutexType: RawMutex, T> {
    inner: Mutex<MutexType, ChannelState<T>>,
}

// The channel can be sent to other threads as long as it's not borrowed and the
// value in it can be sent to other threads.
unsafe impl<MutexType: RawMutex + Send, T: Send> Send
    for GenericOneshotChannel<MutexType, T>
{
}
// The channel is thread-safe as long as a thread-safe mutex is used
unsafe impl<MutexType: RawMutex + Sync, T: Send> Sync
    for GenericOneshotChannel<MutexType, T>
{
}

impl<MutexType: RawMutex, T> core::fmt::Debug
    for GenericOneshotChannel<MutexType, T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GenericOneshotChannel").finish()
    }
}

impl<MutexType: RawMutex, T> GenericOneshotChannel<MutexType, T> {
    /// Creates a new OneshotChannel in the given state
    pub fn new() -> GenericOneshotChannel<MutexType, T> {
        GenericOneshotChannel {
            inner: Mutex::new(ChannelState::new()),
        }
    }

    /// Writes a single value to the channel.
    ///
    /// This will notify waiters about the availability of the value.
    /// If a value had been written to the channel before, or if the
    /// channel is closed, the new value will be rejected and
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
    pub fn receive(&self) -> ChannelReceiveFuture<MutexType, T> {
        ChannelReceiveFuture {
            channel: Some(self),
            wait_node: ListNode::new(RecvWaitQueueEntry::new()),
            _phantom: PhantomData,
        }
    }
}

impl<MutexType: RawMutex, T> ChannelReceiveAccess<T>
    for GenericOneshotChannel<MutexType, T>
{
    unsafe fn receive_or_register(
        &self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<T>> {
        self.inner.lock().try_receive(wait_node, cx)
    }

    fn remove_receive_waiter(
        &self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
    ) {
        self.inner.lock().remove_waiter(wait_node)
    }
}

// Export a non thread-safe version using NoopLock

/// A [`GenericOneshotChannel`] which is not thread-safe.
pub type LocalOneshotChannel<T> = GenericOneshotChannel<NoopLock, T>;

#[cfg(feature = "std")]
mod if_std {
    use super::*;
    // Export a thread-safe version using parking_lot::RawMutex

    /// A [`GenericOneshotChannel`] implementation backed by [`parking_lot`].
    pub type OneshotChannel<T> =
        GenericOneshotChannel<parking_lot::RawMutex, T>;
}

#[cfg(feature = "std")]
pub use self::if_std::*;

#[cfg(feature = "alloc")]
mod if_alloc {
    use super::*;

    pub mod shared {
        use super::*;
        use crate::channel::shared::ChannelReceiveFuture;

        struct GenericOneshotChannelSharedState<MutexType, T>
        where
            MutexType: RawMutex,
            T: 'static,
        {
            channel: GenericOneshotChannel<MutexType, T>,
        }

        // Implement ChannelReceiveAccess trait for SharedChannelState, so that it can
        // be used for dynamic dispatch in futures.
        impl<MutexType, T> ChannelReceiveAccess<T>
            for GenericOneshotChannelSharedState<MutexType, T>
        where
            MutexType: RawMutex,
        {
            unsafe fn receive_or_register(
                &self,
                wait_node: &mut ListNode<RecvWaitQueueEntry>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<T>> {
                self.channel.receive_or_register(wait_node, cx)
            }

            fn remove_receive_waiter(
                &self,
                wait_node: &mut ListNode<RecvWaitQueueEntry>,
            ) {
                self.channel.remove_receive_waiter(wait_node)
            }
        }

        /// The sending side of a channel which can be used to exchange values
        /// between concurrent tasks.
        ///
        /// Values can be sent into the channel through `send`.
        pub struct GenericOneshotSender<MutexType, T>
        where
            MutexType: RawMutex,
            T: 'static,
        {
            inner: alloc::sync::Arc<
                GenericOneshotChannelSharedState<MutexType, T>,
            >,
        }

        /// The receiving side of a channel which can be used to exchange values
        /// between concurrent tasks.
        ///
        /// Tasks can receive values from the channel through the `receive` method.
        /// The returned Future will get resolved when a value is sent into the channel.
        pub struct GenericOneshotReceiver<MutexType, T>
        where
            MutexType: RawMutex,
            T: 'static,
        {
            inner: alloc::sync::Arc<
                GenericOneshotChannelSharedState<MutexType, T>,
            >,
        }

        impl<MutexType, T> core::fmt::Debug for GenericOneshotSender<MutexType, T>
        where
            MutexType: RawMutex,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("OneshotSender").finish()
            }
        }

        impl<MutexType, T> core::fmt::Debug for GenericOneshotReceiver<MutexType, T>
        where
            MutexType: RawMutex,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("OneshotReceiver").finish()
            }
        }

        impl<MutexType, T> Drop for GenericOneshotSender<MutexType, T>
        where
            MutexType: RawMutex,
        {
            fn drop(&mut self) {
                // Close the channel, before last sender gets destroyed
                // TODO: We could potentially avoid this, if no receiver is left
                self.inner.channel.close();
            }
        }

        impl<MutexType, T> Drop for GenericOneshotReceiver<MutexType, T>
        where
            MutexType: RawMutex,
        {
            fn drop(&mut self) {
                // Close the channel, before last receiver gets destroyed
                // TODO: We could potentially avoid this, if no sender is left
                self.inner.channel.close();
            }
        }

        /// Creates a new oneshot channel which can be used to exchange values
        /// of type `T` between concurrent tasks.
        /// The ends of the Channel are represented through
        /// the returned Sender and Receiver.
        ///
        /// As soon es either the senders or receivers is closed, the channel
        /// itself will be closed.
        pub fn generic_oneshot_channel<MutexType, T>() -> (
            GenericOneshotSender<MutexType, T>,
            GenericOneshotReceiver<MutexType, T>,
        )
        where
            MutexType: RawMutex,
            T: Send,
        {
            let inner =
                alloc::sync::Arc::new(GenericOneshotChannelSharedState {
                    channel: GenericOneshotChannel::new(),
                });

            let sender = GenericOneshotSender {
                inner: inner.clone(),
            };
            let receiver = GenericOneshotReceiver { inner };

            (sender, receiver)
        }

        impl<MutexType, T> GenericOneshotSender<MutexType, T>
        where
            MutexType: RawMutex + 'static,
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

        impl<MutexType, T> GenericOneshotReceiver<MutexType, T>
        where
            MutexType: RawMutex + 'static,
        {
            /// Returns a future that gets fulfilled when a value is written to the channel.
            /// If the channels gets closed, the future will resolve to `None`.
            pub fn receive(&self) -> ChannelReceiveFuture<MutexType, T> {
                ChannelReceiveFuture {
                    channel: Some(self.inner.clone()),
                    wait_node: ListNode::new(RecvWaitQueueEntry::new()),
                    _phantom: PhantomData,
                }
            }
        }

        // Export parking_lot based shared channels in std mode
        #[cfg(feature = "std")]
        mod if_std {
            use super::*;

            /// A [`GenericOneshotSender`] implementation backed by [`parking_lot`].
            pub type OneshotSender<T> =
                GenericOneshotSender<parking_lot::RawMutex, T>;
            /// A [`GenericOneshotReceiver`] implementation backed by [`parking_lot`].
            pub type OneshotReceiver<T> =
                GenericOneshotReceiver<parking_lot::RawMutex, T>;

            /// Creates a new oneshot channel.
            ///
            /// Refer to [`generic_oneshot_channel`] for details.
            ///
            /// Example for creating a channel to transmit an integer value:
            ///
            /// ```
            /// # use futures_intrusive::channel::shared::oneshot_channel;
            /// let (sender, receiver) = oneshot_channel::<i32>();
            /// ```
            pub fn oneshot_channel<T>() -> (OneshotSender<T>, OneshotReceiver<T>)
            where
                T: Send,
            {
                generic_oneshot_channel::<parking_lot::RawMutex, T>()
            }
        }

        #[cfg(feature = "std")]
        pub use self::if_std::*;
    }
}

#[cfg(feature = "alloc")]
pub use self::if_alloc::*;
