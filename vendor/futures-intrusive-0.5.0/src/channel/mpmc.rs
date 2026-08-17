//! An asynchronously awaitable multi producer multi consumer channel

use crate::intrusive_double_linked_list::{LinkedList, ListNode};
use crate::{
    buffer::{ArrayBuf, RingBuf},
    utils::update_waker_ref,
    NoopLock,
};
use core::{marker::PhantomData, pin::Pin};
use futures_core::{
    future::Future,
    stream::{FusedStream, Stream},
    task::{Context, Poll, Waker},
};
use lock_api::{Mutex, RawMutex};

use super::{
    ChannelReceiveAccess, ChannelReceiveFuture, ChannelSendAccess,
    ChannelSendFuture, CloseStatus, RecvPollState, RecvWaitQueueEntry,
    SendPollState, SendWaitQueueEntry, TryReceiveError, TrySendError,
};

fn wake_recv_waiters(waiters: &mut LinkedList<RecvWaitQueueEntry>) {
    // Remove all waiters from the waiting list in reverse order and wake them.
    // We reverse the waiter list, so that the oldest waker (which is
    // at the end of the list), gets woken first and has the best
    // chance to grab the channel value.
    waiters.reverse_drain(|waiter| {
        if let Some(handle) = waiter.task.take() {
            handle.wake();
        }
        // The only kind of waiter that could have been stored here are
        // registered waiters (with a value), since others are removed
        // whenever their value had been copied into the channel.
        waiter.state = RecvPollState::Unregistered;
    });
}

fn wake_send_waiters<T>(waiters: &mut LinkedList<SendWaitQueueEntry<T>>) {
    // Remove all waiters from the waiting list in reverse order and wake them.
    // We reverse the waiter list, so that the oldest waker (which is
    // at the end of the list), gets woken first and has the best
    // chance to send.
    waiters.reverse_drain(|waiter| {
        if let Some(handle) = waiter.task.take() {
            handle.wake();
        }
        waiter.state = SendPollState::Unregistered;
    });
}

/// Wakes up the last waiter and removes it from the wait queue
#[must_use]
fn return_oldest_receive_waiter(
    waiters: &mut LinkedList<RecvWaitQueueEntry>,
) -> Option<Waker> {
    let last_waiter = waiters.remove_last();

    if let Some(last_waiter) = last_waiter {
        last_waiter.state = RecvPollState::Notified;
        last_waiter.task.take()
    } else {
        None
    }
}

/// Internal state of the channel
struct ChannelState<T, A>
where
    A: RingBuf<Item = T>,
{
    /// Whether the channel had been closed
    is_closed: bool,
    /// The value which is stored inside the channel
    buffer: A,
    /// Futures which are waiting on receive
    receive_waiters: LinkedList<RecvWaitQueueEntry>,
    /// Futures which are waiting on send
    send_waiters: LinkedList<SendWaitQueueEntry<T>>,
}

impl<T, A> ChannelState<T, A>
where
    A: RingBuf<Item = T>,
{
    fn new(buffer: A) -> ChannelState<T, A> {
        ChannelState::<T, A> {
            is_closed: false,
            buffer,
            receive_waiters: LinkedList::new(),
            send_waiters: LinkedList::new(),
        }
    }

    fn clear(&mut self) {
        while !self.buffer.is_empty() {
            self.buffer.pop();
        }
    }

    fn close(&mut self) -> CloseStatus {
        if self.is_closed {
            return CloseStatus::AlreadyClosed;
        }
        self.is_closed = true;

        // Wakeup all send and receive waiters, since they are now guaranteed
        // to make progress.
        wake_recv_waiters(&mut self.receive_waiters);
        wake_send_waiters(&mut self.send_waiters);

        CloseStatus::NewlyClosed
    }

    /// Attempt to send a value without waiting.
    /// Returns a `Waker` if sending the value lead enabled a task to run.
    fn try_send(&mut self, value: T) -> Result<Option<Waker>, TrySendError<T>> {
        debug_assert!(
            self.buffer.capacity() > 0,
            "try_send is not supported for unbuffered channels"
        );

        if self.is_closed {
            Err(TrySendError::Closed(value))
        } else if self.buffer.can_push() {
            self.buffer.push(value);

            // Return the oldest receive waiter
            Ok(return_oldest_receive_waiter(&mut self.receive_waiters))
        } else {
            Err(TrySendError::Full(value))
        }
    }

    /// Tries to send a value to the channel.
    /// If the value isn't available yet, the ChannelSendFuture gets added to the
    /// wait queue at the channel, and will be signalled once ready.
    /// If the channels is already closed, the value to send is returned.
    /// This function is only safe as long as the `wait_node`s address is guaranteed
    /// to be stable until it gets removed from the queue.
    /// If sending the value succeeded, the `Waker` for a task which can receive
    /// the value is returned.
    unsafe fn send_or_register(
        &mut self,
        wait_node: &mut ListNode<SendWaitQueueEntry<T>>,
        cx: &mut Context<'_>,
    ) -> (Poll<()>, Option<T>, Option<Waker>) {
        match wait_node.state {
            SendPollState::Unregistered => {
                if self.is_closed {
                    let value = wait_node.value.take();
                    return (Poll::Ready(()), value, None);
                }

                if !self.buffer.can_push() {
                    // If the capacity is exhausted, register a waiter
                    wait_node.task = Some(cx.waker().clone());
                    wait_node.state = SendPollState::Registered;
                    self.send_waiters.add_front(wait_node);

                    // Return the oldest receive waiter
                    let waker =
                        return_oldest_receive_waiter(&mut self.receive_waiters);
                    return (Poll::Pending, None, waker);
                } else {
                    // Otherwise copy the value directly into the channel
                    let value = wait_node
                        .value
                        .take()
                        .expect("wait_node must contain value");
                    self.buffer.push(value);

                    // Return the oldest receive waiter
                    let waker =
                        return_oldest_receive_waiter(&mut self.receive_waiters);

                    (Poll::Ready(()), None, waker)
                }
            }
            SendPollState::Registered => {
                // Since the channel wakes up all waiters and moves their states
                // to unregistered there can't be space available in the channel.
                // However the caller might have passed a different `Waker`.
                // In this case we need to update it.
                update_waker_ref(&mut wait_node.task, cx);
                (Poll::Pending, None, None)
            }
            SendPollState::SendComplete => {
                // The transfer is complete, and the sender has already been removed from the
                // list of pending senders
                (Poll::Ready(()), None, None)
            }
        }
    }

    /// If there is a send waiter, copy it's value into the channel buffer and complete it.
    /// The method may only be called if there is space in the receive buffer.
    #[must_use]
    fn try_copy_value_from_oldest_waiter(&mut self) -> Option<Waker> {
        let last_waiter = self.send_waiters.remove_last();

        if let Some(last_waiter) = last_waiter {
            let value = last_waiter
                .value
                .take()
                .expect("wait_node must contain value");
            self.buffer.push(value);

            last_waiter.state = SendPollState::SendComplete;

            last_waiter.task.take()
        } else {
            None
        }
    }

    /// Tries to extract a value from the sending waiter which has been waiting
    /// longest on the send operation to complete.
    fn try_take_value_from_sender(&mut self) -> Option<(T, Option<Waker>)> {
        // Safety: The method is only called inside the lock on a consistent
        // list.
        match self.send_waiters.remove_last() {
            Some(last_sender) => {
                // This path should be only used for 0 capacity queues.
                // Since the list is not empty, a value is available.
                // Extract it from the sender in order to return it
                debug_assert_eq!(0, self.buffer.capacity());

                // Safety: The sender can't be invalid, since we only add valid
                // senders to the queue
                let val =
                    last_sender.value.take().expect("Value must be available");
                last_sender.state = SendPollState::SendComplete;

                // Return the waiter
                Some((val, last_sender.task.take()))
            }
            None => None,
        }
    }

    /// Tries to receive a value from the channel without waiting.
    fn try_receive(&mut self) -> Result<(T, Option<Waker>), TryReceiveError> {
        if !self.buffer.is_empty() {
            let val = self.buffer.pop();

            // Since this means a space in the buffer had been freed,
            // try to copy a value from a potential waiter into the channel.
            let waker = self.try_copy_value_from_oldest_waiter();

            Ok((val, waker))
        } else if let Some((val, waker)) = self.try_take_value_from_sender() {
            Ok((val, waker))
        } else if self.is_closed {
            Err(TryReceiveError::Closed)
        } else {
            Err(TryReceiveError::Empty)
        }
    }

    /// Tries to read the value from the channel.
    /// If the value isn't available yet, the ChannelReceiveFuture gets added to the
    /// wait queue at the channel, and will be signalled once ready.
    /// This function is only safe as long as the `wait_node`s address is guaranteed
    /// to be stable until it gets removed from the queue.
    unsafe fn receive_or_register(
        &mut self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<(T, Option<Waker>)>> {
        match wait_node.state {
            RecvPollState::Unregistered | RecvPollState::Notified => {
                wait_node.state = RecvPollState::Unregistered;

                match self.try_receive() {
                    Ok(val) => Poll::Ready(Some(val)),
                    Err(TryReceiveError::Closed) => Poll::Ready(None),
                    Err(TryReceiveError::Empty) => {
                        // Added the task to the wait queue
                        wait_node.task = Some(cx.waker().clone());
                        wait_node.state = RecvPollState::Registered;
                        self.receive_waiters.add_front(wait_node);
                        Poll::Pending
                    }
                }
            }
            RecvPollState::Registered => {
                // Since the channel wakes up all waiters and moves their states
                // to unregistered there can't be any value in the channel in
                // this state. However the caller might have passed a different `Waker`.
                // In this case we need to update it.
                update_waker_ref(&mut wait_node.task, cx);
                Poll::Pending
            }
        }
    }

    fn remove_send_waiter(
        &mut self,
        wait_node: &mut ListNode<SendWaitQueueEntry<T>>,
    ) {
        // ChannelSendFuture only needs to get removed if it had been added to
        // the wait queue of the channel.
        // This has happened in the SendPollState::Registered case.
        match wait_node.state {
            SendPollState::Registered => {
                // Safety: Due to the state, we know that the node must be part
                // of the waiter list
                if !unsafe { self.send_waiters.remove(wait_node) } {
                    // Panic if the address isn't found. This can only happen if the contract was
                    // violated, e.g. the WaitQueueEntry got moved after the initial poll.
                    panic!("Future could not be removed from wait queue");
                }
                wait_node.state = SendPollState::Unregistered;
            }
            SendPollState::Unregistered => {}
            SendPollState::SendComplete => {
                // Send was complete. In that case the queue item is not in the list
            }
        }
    }

    #[must_use]
    fn remove_receive_waiter(
        &mut self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
    ) -> Option<Waker> {
        // ChannelReceiveFuture only needs to get removed if it had been added to
        // the wait queue of the channel. This has happened in the RecvPollState::Registered case.
        match wait_node.state {
            RecvPollState::Registered => {
                // Safety: Due to the state, we know that the node must be part
                // of the waiter list
                if !unsafe { self.receive_waiters.remove(wait_node) } {
                    // Panic if the address isn't found. This can only happen if the contract was
                    // violated, e.g. the WaitQueueEntry got moved after the initial poll.
                    panic!("Future could not be removed from wait queue");
                }
                wait_node.state = RecvPollState::Unregistered;
                None
            }
            RecvPollState::Notified => {
                // wakeup another receive waiter instead
                wait_node.state = RecvPollState::Unregistered;
                return_oldest_receive_waiter(&mut self.receive_waiters)
            }
            RecvPollState::Unregistered => None,
        }
    }
}

/// A channel which can be used to exchange values of type `T` between
/// concurrent tasks.
///
/// `A` represents the backing buffer for a Channel. E.g. a channel which
/// can buffer up to 4 `i32` values can be created via:
///
/// ```
/// # use futures_intrusive::channel::LocalChannel;
/// let channel: LocalChannel<i32, [i32; 4]> = LocalChannel::new();
/// ```
///
/// Tasks can receive values from the channel through the `receive` method.
/// The returned Future will get resolved when a value is sent into the channel.
/// Values can be sent into the channel through `send`.
/// The returned Future will get resolved when the value has been stored
/// inside the channel.
pub struct GenericChannel<MutexType: RawMutex, T, A>
where
    A: RingBuf<Item = T>,
{
    inner: Mutex<MutexType, ChannelState<T, A>>,
}

// The channel can be sent to other threads as long as it's not borrowed and the
// value in it can be sent to other threads.
unsafe impl<MutexType: RawMutex + Send, T: Send, A> Send
    for GenericChannel<MutexType, T, A>
where
    A: RingBuf<Item = T> + Send,
{
}
// The channel is thread-safe as long as a thread-safe mutex is used
unsafe impl<MutexType: RawMutex + Sync, T: Send, A> Sync
    for GenericChannel<MutexType, T, A>
where
    A: RingBuf<Item = T>,
{
}

impl<MutexType: RawMutex, T, A> core::fmt::Debug
    for GenericChannel<MutexType, T, A>
where
    A: RingBuf<Item = T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Channel").finish()
    }
}

impl<MutexType: RawMutex, T, A> GenericChannel<MutexType, T, A>
where
    A: RingBuf<Item = T>,
{
    /// Creates a new Channel, utilizing the default capacity that
    /// the RingBuffer in `A` provides.
    pub fn new() -> Self {
        GenericChannel {
            inner: Mutex::new(ChannelState::new(A::new())),
        }
    }

    /// Creates a new Channel, which has storage for a `capacity` items.
    /// Depending on the utilized `RingBuf` type, the capacity argument might
    /// be ignored and the default capacity might be utilized.
    pub fn with_capacity(capacity: usize) -> Self {
        GenericChannel {
            inner: Mutex::new(ChannelState::new(A::with_capacity(capacity))),
        }
    }

    /// Returns a future that gets fulfilled when the value has been written to
    /// the channel.
    /// If the channel gets closed while the send is in progress, sending the
    /// value will fail, and the future will deliver the value back.
    pub fn send(&self, value: T) -> ChannelSendFuture<MutexType, T> {
        ChannelSendFuture {
            channel: Some(self),
            wait_node: ListNode::new(SendWaitQueueEntry::new(value)),
            _phantom: PhantomData,
        }
    }

    /// Attempt to send the value without waiting.
    ///
    /// This operation is not supported for unbuffered channels and will
    /// panic if the capacity of the `RingBuf` is zero. The reason for this is
    /// that the actual value transfer on unbuffered channels always happens
    /// when a receiving task copies the value out of the sending task while it
    /// is waiting. If the sending task does not wait, the value can not be
    /// transferred. Since this method can therefore never yield a reasonable
    /// result with unbuffered channels, it panics in order to highlight the
    /// use of an inappropriate API.
    pub fn try_send(&self, value: T) -> Result<(), TrySendError<T>> {
        let result = { self.inner.lock().try_send(value) };

        match result {
            Ok(Some(waker)) => {
                waker.wake();
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Returns a future that gets fulfilled when a value is written to the channel.
    /// If the channels gets closed, the future will resolve to `None`.
    pub fn receive(&self) -> ChannelReceiveFuture<MutexType, T> {
        ChannelReceiveFuture {
            channel: Some(self),
            wait_node: ListNode::new(RecvWaitQueueEntry::new()),
            _phantom: PhantomData,
        }
    }

    /// Attempt to receive a value of the channel without waiting.
    pub fn try_receive(&self) -> Result<T, TryReceiveError> {
        let result = { self.inner.lock().try_receive() };

        match result {
            Ok((val, waker)) => {
                if let Some(waker) = waker {
                    waker.wake();
                }
                Ok(val)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns a stream that will receive values from this channel.
    ///
    /// This stream does not yield `None` when the channel is empty,
    /// instead it yields `None` when it is terminated.
    pub fn stream(&self) -> ChannelStream<MutexType, T, A> {
        ChannelStream {
            channel: Some(self),
            future: None,
        }
    }

    /// Closes the channel.
    /// All pending and future send attempts will fail.
    /// Receive attempts will continue to succeed as long as there are items
    /// stored inside the channel. Further attempts will fail.
    pub fn close(&self) -> CloseStatus {
        self.inner.lock().close()
    }
}

impl<MutexType: RawMutex, T, A> ChannelSendAccess<T>
    for GenericChannel<MutexType, T, A>
where
    A: RingBuf<Item = T>,
{
    unsafe fn send_or_register(
        &self,
        wait_node: &mut ListNode<SendWaitQueueEntry<T>>,
        cx: &mut Context<'_>,
    ) -> (Poll<()>, Option<T>) {
        let (poll_result, value, waker) =
            { self.inner.lock().send_or_register(wait_node, cx) };

        if let Some(waker) = waker {
            waker.wake();
        }

        (poll_result, value)
    }

    fn remove_send_waiter(
        &self,
        wait_node: &mut ListNode<SendWaitQueueEntry<T>>,
    ) {
        self.inner.lock().remove_send_waiter(wait_node)
    }
}

impl<MutexType: RawMutex, T, A> ChannelReceiveAccess<T>
    for GenericChannel<MutexType, T, A>
where
    A: RingBuf<Item = T>,
{
    unsafe fn receive_or_register(
        &self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<T>> {
        let result = { self.inner.lock().receive_or_register(wait_node, cx) };

        match result {
            Poll::Ready(Some((val, waker))) => {
                if let Some(waker) = waker {
                    waker.wake();
                }
                Poll::Ready(Some(val))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }

    fn remove_receive_waiter(
        &self,
        wait_node: &mut ListNode<RecvWaitQueueEntry>,
    ) {
        let waker = { self.inner.lock().remove_receive_waiter(wait_node) };

        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

/// A stream that receives from a `GenericChannel`.
///
/// Not driving the `ChannelStream` to completion after it has been polled
/// might lead to lost wakeup notifications.
#[derive(Debug)]
pub struct ChannelStream<'a, MutexType: RawMutex, T, A>
where
    A: RingBuf<Item = T>,
{
    channel: Option<&'a GenericChannel<MutexType, T, A>>,
    future: Option<ChannelReceiveFuture<'a, MutexType, T>>,
}

impl<'a, MutexType, T, A> Stream for ChannelStream<'a, MutexType, T, A>
where
    A: RingBuf<Item = T>,
    MutexType: RawMutex,
{
    type Item = T;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        // It might be possible to use Pin::map_unchecked here instead of the two unsafe APIs.
        // However this didn't seem to work for some borrow checker reasons

        // Safety: The next operations are safe, because Pin promises us that
        // the address of the wait queue entry inside ChannelReceiveFuture is stable,
        // and we don't move any fields inside the future until it gets dropped.
        let mut_self: &mut Self = unsafe { Pin::get_unchecked_mut(self) };
        match mut_self.channel.take() {
            Some(channel) => {
                // Poll the next element.
                if mut_self.future.is_none() {
                    mut_self.future.replace(channel.receive());
                }
                let fut = mut_self.future.as_mut().unwrap();

                // Safety: We guarantee that the pinned future will not move until
                // it resolves by storing it as part of the pinned `Stream`
                let poll = unsafe {
                    let pin_fut = Pin::new_unchecked(fut);
                    pin_fut.poll(cx)
                };

                // Future was resolved, drop it.
                if poll.is_ready() {
                    mut_self.future.take();

                    // If the channel was terminated, we let it drop.
                    if let Poll::Ready(None) = &poll {
                        return poll;
                    }
                }

                // The channel was not terminated, so we reuse it.
                mut_self.channel.replace(channel);

                poll
            }
            // Channel was terminated.
            None => Poll::Ready(None),
        }
    }
}

impl<'a, MutexType, T, A> FusedStream for ChannelStream<'a, MutexType, T, A>
where
    A: RingBuf<Item = T>,
    MutexType: RawMutex,
{
    fn is_terminated(&self) -> bool {
        self.channel.is_none()
    }
}

// Export a non thread-safe version using NoopLock

/// A [`GenericChannel`] implementation which is not thread-safe.
pub type LocalChannel<T, A> = GenericChannel<NoopLock, T, ArrayBuf<T, A>>;

/// An unbuffered [`GenericChannel`] implementation which is not thread-safe.
pub type LocalUnbufferedChannel<T> = LocalChannel<T, [T; 0]>;

#[cfg(feature = "std")]
mod if_std {
    use super::*;

    // Export a thread-safe version using parking_lot::RawMutex

    // TODO: We might also want to bind Channel to GenericChannel<..., FixedHeapBuf>,
    // which performs less type-churn.
    // However since we can't bind LocalChannel to that too due to no-std compatibility,
    // this would to introduce some inconsistency between those types.
    // It's also bit unfortunate that there are now `new()` and `with_capacity`
    // methods on both types, but for the array backed implementation only
    // `new()` is meaningful, while for the heap backed implementation only
    // `with_capacity()` is meaningful.

    /// A [`GenericChannel`] implementation backed by [`parking_lot`].
    pub type Channel<T, A> =
        GenericChannel<parking_lot::RawMutex, T, ArrayBuf<T, A>>;

    /// An unbuffered [`GenericChannel`] implementation backed by [`parking_lot`].
    pub type UnbufferedChannel<T> = Channel<T, [T; 0]>;
}

#[cfg(feature = "std")]
pub use self::if_std::*;

#[cfg(feature = "alloc")]
mod if_alloc {
    use super::*;

    /// Channel implementations where Sender and Receiver sides are cloneable
    /// and owned.
    /// The Futures produced by channels in this module don't require a lifetime
    /// parameter.
    pub mod shared {
        use super::*;
        use crate::channel::shared::{ChannelReceiveFuture, ChannelSendFuture};
        use core::sync::atomic::{AtomicUsize, Ordering};

        /// Shared Channel State, which is referenced by Senders and Receivers
        struct GenericChannelSharedState<MutexType, T, A>
        where
            MutexType: RawMutex,
            T: 'static,
            A: RingBuf<Item = T>,
        {
            /// The amount of [`GenericSender`] instances which reference this state.
            senders: AtomicUsize,
            /// The amount of [`GenericReceiver`] instances which reference this state.
            receivers: AtomicUsize,
            /// The channel on which is acted.
            channel: GenericChannel<MutexType, T, A>,
        }

        // Implement ChannelAccess trait for SharedChannelState, so that it can
        // be used for dynamic dispatch in futures.
        impl<MutexType, T, A> ChannelReceiveAccess<T>
            for GenericChannelSharedState<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
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

        // Implement ChannelAccess trait for SharedChannelState, so that it can
        // be used for dynamic dispatch in futures.
        impl<MutexType, T, A> ChannelSendAccess<T>
            for GenericChannelSharedState<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
        {
            unsafe fn send_or_register(
                &self,
                wait_node: &mut ListNode<SendWaitQueueEntry<T>>,
                cx: &mut Context<'_>,
            ) -> (Poll<()>, Option<T>) {
                self.channel.send_or_register(wait_node, cx)
            }

            fn remove_send_waiter(
                &self,
                wait_node: &mut ListNode<SendWaitQueueEntry<T>>,
            ) {
                self.channel.remove_send_waiter(wait_node)
            }
        }

        /// The sending side of a channel which can be used to exchange values
        /// between concurrent tasks.
        ///
        /// Values can be sent into the channel through `send`.
        /// The returned Future will get resolved when the value has been stored inside the channel.
        pub struct GenericSender<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
            T: 'static,
        {
            inner: alloc::sync::Arc<GenericChannelSharedState<MutexType, T, A>>,
        }

        /// The receiving side of a channel which can be used to exchange values
        /// between concurrent tasks.
        ///
        /// Tasks can receive values from the channel through the `receive` method.
        /// The returned Future will get resolved when a value is sent into the channel.
        pub struct GenericReceiver<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
            T: 'static,
        {
            inner: alloc::sync::Arc<GenericChannelSharedState<MutexType, T, A>>,
        }

        impl<MutexType, T, A> core::fmt::Debug for GenericSender<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sender").finish()
            }
        }

        impl<MutexType, T, A> core::fmt::Debug for GenericReceiver<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Receiver").finish()
            }
        }

        impl<MutexType, T, A> Clone for GenericSender<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
        {
            fn clone(&self) -> Self {
                let old_size =
                    self.inner.senders.fetch_add(1, Ordering::Relaxed);
                if old_size > (core::isize::MAX) as usize {
                    panic!("Reached maximum refcount");
                }
                GenericSender {
                    inner: self.inner.clone(),
                }
            }
        }

        impl<MutexType, T, A> Drop for GenericSender<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
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

        impl<MutexType, T, A> Clone for GenericReceiver<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
        {
            fn clone(&self) -> Self {
                let old_size =
                    self.inner.receivers.fetch_add(1, Ordering::Relaxed);
                if old_size > (core::isize::MAX) as usize {
                    panic!("Reached maximum refcount");
                }
                GenericReceiver {
                    inner: self.inner.clone(),
                }
            }
        }

        impl<MutexType, T, A> Drop for GenericReceiver<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
        {
            fn drop(&mut self) {
                if self.inner.receivers.fetch_sub(1, Ordering::Release) != 1 {
                    return;
                }
                core::sync::atomic::fence(Ordering::Acquire);
                // Close the channel, before last receiver gets destroyed
                // TODO: We could potentially avoid this, if no sender is left
                self.inner.channel.close();

                // Now drop the content of the channel. This ensures that
                // the content of the channel is dropped even if a sender is held.
                self.inner.channel.inner.lock().clear();
            }
        }

        /// Creates a new Channel which can be used to exchange values of type `T` between
        /// concurrent tasks. The ends of the Channel are represented through
        /// the returned Sender and Receiver.
        /// Both the Sender and Receiver can be cloned in order to let more tasks
        /// interact with the Channel.
        ///
        /// As soon es either all Senders or all Receivers are closed, the Channel
        /// itself will be closed.
        ///
        /// The channel can buffer up to `capacity` items internally.
        pub fn generic_channel<MutexType, T, A>(
            capacity: usize,
        ) -> (
            GenericSender<MutexType, T, A>,
            GenericReceiver<MutexType, T, A>,
        )
        where
            MutexType: RawMutex,
            A: RingBuf<Item = T>,
            T: Send,
        {
            let inner = alloc::sync::Arc::new(GenericChannelSharedState {
                channel: GenericChannel::with_capacity(capacity),
                senders: AtomicUsize::new(1),
                receivers: AtomicUsize::new(1),
            });

            let sender = GenericSender {
                inner: inner.clone(),
            };
            let receiver = GenericReceiver { inner };

            (sender, receiver)
        }

        impl<MutexType, T, A> GenericSender<MutexType, T, A>
        where
            MutexType: 'static + RawMutex,
            A: 'static + RingBuf<Item = T>,
        {
            /// Returns a future that gets fulfilled when the value has been written to
            /// the channel.
            /// If the channel gets closed while the send is in progress, sending the
            /// value will fail, and the future will deliver the value back.
            pub fn send(&self, value: T) -> ChannelSendFuture<MutexType, T> {
                ChannelSendFuture {
                    channel: Some(self.inner.clone()),
                    wait_node: ListNode::new(SendWaitQueueEntry::new(value)),
                    _phantom: PhantomData,
                }
            }

            /// Attempt to send the value without waiting.
            ///
            /// This operation is not supported for unbuffered channels and will
            /// panic if the capacity of the `RingBuf` is zero. The reason for this is
            /// that the actual value transfer on unbuffered channels always happens
            /// when a receiving task copies the value out of the sending task while it
            /// is waiting. If the sending task does not wait, the value can not be
            /// transferred. Since this method can therefore never yield a reasonable
            /// result with unbuffered channels, it panics in order to highlight the
            /// use of an inappropriate API.
            pub fn try_send(&self, value: T) -> Result<(), TrySendError<T>> {
                self.inner.channel.try_send(value)
            }

            /// Closes the channel.
            /// All pending future send attempts will fail.
            /// Receive attempts will continue to succeed as long as there are items
            /// stored inside the channel. Further attempts will return `None`.
            pub fn close(&self) -> CloseStatus {
                self.inner.channel.close()
            }
        }

        impl<MutexType, T, A> GenericReceiver<MutexType, T, A>
        where
            MutexType: 'static + RawMutex,
            A: 'static + RingBuf<Item = T>,
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

            /// Attempt to receive form the channel without waiting.
            pub fn try_receive(&self) -> Result<T, TryReceiveError> {
                self.inner.channel.try_receive()
            }

            /// Closes the channel.
            /// All pending future send attempts will fail.
            /// Receive attempts will continue to succeed as long as there are items
            /// stored inside the channel. Further attempts will return `None`.
            pub fn close(&self) -> CloseStatus {
                self.inner.channel.close()
            }

            /// Returns a stream that will receive values from this channel.
            ///
            /// This stream does not yield `None` when the channel is empty,
            /// instead it yields `None` when it is terminated.
            pub fn into_stream(self) -> SharedStream<MutexType, T, A> {
                SharedStream {
                    receiver: self,
                    future: None,
                    is_terminated: false,
                }
            }
        }

        /// A stream that receives from channel using a `GenericReceiver`.
        ///
        /// Not driving the `SharedStream` to completion after it has been polled
        /// might lead to lost wakeup notifications.
        #[derive(Debug)]
        pub struct SharedStream<MutexType, T, A>
        where
            MutexType: 'static + RawMutex,
            T: 'static,
            A: 'static + RingBuf<Item = T>,
        {
            receiver: GenericReceiver<MutexType, T, A>,
            future: Option<ChannelReceiveFuture<MutexType, T>>,
            is_terminated: bool,
        }

        impl<MutexType, T, A> SharedStream<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: 'static + RingBuf<Item = T>,
        {
            /// Closes the channel.
            /// All pending and future send attempts will fail.
            /// Receive attempts will continue to succeed as long as there are items
            /// stored inside the channel. Further attempts will fail.
            pub fn close(&self) -> CloseStatus {
                self.receiver.close()
            }
        }

        impl<MutexType, T, A> Stream for SharedStream<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: 'static + RingBuf<Item = T>,
        {
            type Item = T;

            fn poll_next(
                mut self: Pin<&mut Self>,
                cx: &mut Context,
            ) -> Poll<Option<Self::Item>> {
                if self.is_terminated {
                    return Poll::Ready(None);
                }

                // Safety: This is safe since this is a pinned projection
                // that lives as long as the scope.
                let mut pin_fut = unsafe {
                    self.as_mut().map_unchecked_mut(|v| {
                        // Poll the next element.
                        if v.future.is_none() {
                            v.future.replace(v.receiver.receive());
                        }
                        &mut v.future
                    })
                };

                let poll = pin_fut.as_mut().as_pin_mut().unwrap().poll(cx);

                // Future was resolved, drop it.
                if poll.is_ready() {
                    pin_fut.set(None);

                    if let Poll::Ready(None) = &poll {
                        // Safety: This is safe because `is_terminated` is never
                        // considered pinned (i.e. not structuraly pinned).
                        unsafe {
                            self.get_unchecked_mut().is_terminated = true
                        };
                    }
                }

                poll
            }
        }

        impl<MutexType, T, A> FusedStream for SharedStream<MutexType, T, A>
        where
            MutexType: RawMutex,
            A: 'static + RingBuf<Item = T>,
        {
            fn is_terminated(&self) -> bool {
                self.is_terminated
            }
        }

        // Export parking_lot based shared channels in std mode
        #[cfg(feature = "std")]
        mod if_std {
            use super::*;

            use crate::buffer::GrowingHeapBuf;

            /// A [`GenericSender`] implementation backed by [`parking_lot`].
            ///
            /// Uses a `GrowingHeapBuf` whose capacity grows dynamically up to
            /// the given limit. Refer to [`GrowingHeapBuf`] for more information.
            ///
            /// [`GrowingHeapBuf`]: ../../buffer/struct.GrowingHeapBuf.html
            pub type Sender<T> =
                GenericSender<parking_lot::RawMutex, T, GrowingHeapBuf<T>>;
            /// A [`GenericReceiver`] implementation backed by [`parking_lot`].
            ///
            /// Uses a `GrowingHeapBuf` whose capacity grows dynamically up to
            /// the given limit. Refer to [`GrowingHeapBuf`] for more information.
            ///
            /// [`GrowingHeapBuf`]: ../../buffer/struct.GrowingHeapBuf.html
            pub type Receiver<T> =
                GenericReceiver<parking_lot::RawMutex, T, GrowingHeapBuf<T>>;

            /// Creates a new channel with the given buffering capacity
            ///
            /// Uses a `GrowingHeapBuf` whose capacity grows dynamically up to
            /// the given limit. Refer to [`generic_channel`] and [`GrowingHeapBuf`] for more information.
            ///
            /// [`GrowingHeapBuf`]: ../../buffer/struct.GrowingHeapBuf.html
            ///
            /// ```
            /// # use futures_intrusive::channel::shared::channel;
            /// let (sender, receiver) = channel::<i32>(4);
            /// ```
            pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>)
            where
                T: Send,
            {
                generic_channel::<parking_lot::RawMutex, T, GrowingHeapBuf<T>>(
                    capacity,
                )
            }
            /// A [`GenericSender`] implementation backed by [`parking_lot`].
            pub type UnbufferedSender<T> =
                GenericSender<parking_lot::RawMutex, T, GrowingHeapBuf<T>>;
            /// A [`GenericReceiver`] implementation backed by [`parking_lot`].
            pub type UnbufferedReceiver<T> =
                GenericReceiver<parking_lot::RawMutex, T, GrowingHeapBuf<T>>;

            /// Creates a new unbuffered channel.
            ///
            /// Refer to [`generic_channel`] for details.
            pub fn unbuffered_channel<T>() -> (Sender<T>, Receiver<T>)
            where
                T: Send,
            {
                generic_channel::<parking_lot::RawMutex, T, GrowingHeapBuf<T>>(
                    0,
                )
            }
        }

        #[cfg(feature = "std")]
        pub use self::if_std::*;
    }
}

#[cfg(feature = "alloc")]
pub use self::if_alloc::*;
