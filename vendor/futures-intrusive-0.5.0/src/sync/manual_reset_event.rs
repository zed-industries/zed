//! An asynchronously awaitable event for signalization between tasks

use crate::{
    intrusive_double_linked_list::{LinkedList, ListNode},
    utils::update_waker_ref,
    NoopLock,
};
use core::pin::Pin;
use futures_core::{
    future::{FusedFuture, Future},
    task::{Context, Poll, Waker},
};
use lock_api::{Mutex, RawMutex};

/// Tracks how the future had interacted with the event
#[derive(PartialEq)]
enum PollState {
    /// The task has never interacted with the event.
    New,
    /// The task was added to the wait queue at the event.
    Waiting,
    /// The task has been polled to completion.
    Done,
}

/// Tracks the WaitForEventFuture waiting state.
/// Access to this struct is synchronized through the mutex in the Event.
struct WaitQueueEntry {
    /// The task handle of the waiting task
    task: Option<Waker>,
    /// Current polling state
    state: PollState,
}

impl WaitQueueEntry {
    /// Creates a new WaitQueueEntry
    fn new() -> WaitQueueEntry {
        WaitQueueEntry {
            task: None,
            state: PollState::New,
        }
    }
}

/// Internal state of the `ManualResetEvent` pair above
struct EventState {
    is_set: bool,
    waiters: LinkedList<WaitQueueEntry>,
}

impl EventState {
    fn new(is_set: bool) -> EventState {
        EventState {
            is_set,
            waiters: LinkedList::new(),
        }
    }

    fn reset(&mut self) {
        self.is_set = false;
    }

    fn set(&mut self) {
        if self.is_set != true {
            self.is_set = true;

            // Wakeup all waiters
            // This happens inside the lock to make cancellation reliable
            // If we would access waiters outside of the lock, the pointers
            // may no longer be valid.
            // Typically this shouldn't be an issue, since waking a task should
            // only move it from the blocked into the ready state and not have
            // further side effects.

            // Use a reverse iterator, so that the oldest waiter gets
            // scheduled first
            self.waiters.reverse_drain(|waiter| {
                if let Some(handle) = waiter.task.take() {
                    handle.wake();
                }
                waiter.state = PollState::Done;
            });
        }
    }

    fn is_set(&self) -> bool {
        self.is_set
    }

    /// Checks if the event is set. If it is this returns immediately.
    /// If the event isn't set, the WaitForEventFuture gets added to the wait
    /// queue at the event, and will be signalled once ready.
    /// This function is only safe as long as the `wait_node`s address is guaranteed
    /// to be stable until it gets removed from the queue.
    unsafe fn try_wait(
        &mut self,
        wait_node: &mut ListNode<WaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<()> {
        match wait_node.state {
            PollState::New => {
                if self.is_set {
                    // The event is already signaled
                    wait_node.state = PollState::Done;
                    Poll::Ready(())
                } else {
                    // Added the task to the wait queue
                    wait_node.task = Some(cx.waker().clone());
                    wait_node.state = PollState::Waiting;
                    self.waiters.add_front(wait_node);
                    Poll::Pending
                }
            }
            PollState::Waiting => {
                // The WaitForEventFuture is already in the queue.
                // The event can't have been set, since this would change the
                // waitstate inside the mutex. However the caller might have
                // passed a different `Waker`. In this case we need to update it.
                update_waker_ref(&mut wait_node.task, cx);
                Poll::Pending
            }
            PollState::Done => {
                // We have been woken up by the event.
                // This does not guarantee that the event is still set. It could
                // have been reset it in the meantime.
                Poll::Ready(())
            }
        }
    }

    fn remove_waiter(&mut self, wait_node: &mut ListNode<WaitQueueEntry>) {
        // WaitForEventFuture only needs to get removed if it has been added to
        // the wait queue of the Event. This has happened in the PollState::Waiting case.
        if let PollState::Waiting = wait_node.state {
            // Safety: Due to the state, we know that the node must be part
            // of the waiter list
            if !unsafe { self.waiters.remove(wait_node) } {
                // Panic if the address isn't found. This can only happen if the contract was
                // violated, e.g. the WaitQueueEntry got moved after the initial poll.
                panic!("Future could not be removed from wait queue");
            }
            wait_node.state = PollState::Done;
        }
    }
}

/// A synchronization primitive which can be either in the set or reset state.
///
/// Tasks can wait for the event to get set by obtaining a Future via `wait`.
/// This Future will get fulfilled when the event has been set.
pub struct GenericManualResetEvent<MutexType: RawMutex> {
    inner: Mutex<MutexType, EventState>,
}

// The Event is can be sent to other threads as long as it's not borrowed
unsafe impl<MutexType: RawMutex + Send> Send
    for GenericManualResetEvent<MutexType>
{
}
// The Event is thread-safe as long as the utilized Mutex is thread-safe
unsafe impl<MutexType: RawMutex + Sync> Sync
    for GenericManualResetEvent<MutexType>
{
}

impl<MutexType: RawMutex> core::fmt::Debug
    for GenericManualResetEvent<MutexType>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ManualResetEvent").finish()
    }
}

impl<MutexType: RawMutex> GenericManualResetEvent<MutexType> {
    /// Creates a new ManualResetEvent in the given state
    pub fn new(is_set: bool) -> GenericManualResetEvent<MutexType> {
        GenericManualResetEvent {
            inner: Mutex::<MutexType, EventState>::new(EventState::new(is_set)),
        }
    }

    /// Sets the event.
    ///
    /// Setting the event will notify all pending waiters.
    pub fn set(&self) {
        self.inner.lock().set()
    }

    /// Resets the event.
    pub fn reset(&self) {
        self.inner.lock().reset()
    }

    /// Returns whether the event is set
    pub fn is_set(&self) -> bool {
        self.inner.lock().is_set()
    }

    /// Returns a future that gets fulfilled when the event is set.
    pub fn wait(&self) -> GenericWaitForEventFuture<MutexType> {
        GenericWaitForEventFuture {
            event: Some(self),
            wait_node: ListNode::new(WaitQueueEntry::new()),
        }
    }

    unsafe fn try_wait(
        &self,
        wait_node: &mut ListNode<WaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<()> {
        self.inner.lock().try_wait(wait_node, cx)
    }

    fn remove_waiter(&self, wait_node: &mut ListNode<WaitQueueEntry>) {
        self.inner.lock().remove_waiter(wait_node)
    }
}

/// A Future that is resolved once the corresponding ManualResetEvent has been set
#[must_use = "futures do nothing unless polled"]
pub struct GenericWaitForEventFuture<'a, MutexType: RawMutex> {
    /// The ManualResetEvent that is associated with this WaitForEventFuture
    event: Option<&'a GenericManualResetEvent<MutexType>>,
    /// Node for waiting at the event
    wait_node: ListNode<WaitQueueEntry>,
}

// Safety: Futures can be sent between threads as long as the underlying
// event is thread-safe (Sync), which allows to poll/register/unregister from
// a different thread.
unsafe impl<'a, MutexType: RawMutex + Sync> Send
    for GenericWaitForEventFuture<'a, MutexType>
{
}

impl<'a, MutexType: RawMutex> core::fmt::Debug
    for GenericWaitForEventFuture<'a, MutexType>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GenericWaitForEventFuture").finish()
    }
}

impl<'a, MutexType: RawMutex> Future
    for GenericWaitForEventFuture<'a, MutexType>
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // It might be possible to use Pin::map_unchecked here instead of the two unsafe APIs.
        // However this didn't seem to work for some borrow checker reasons

        // Safety: The next operations are safe, because Pin promises us that
        // the address of the wait queue entry inside MutexLocalFuture is stable,
        // and we don't move any fields inside the future until it gets dropped.
        let mut_self: &mut GenericWaitForEventFuture<MutexType> =
            unsafe { Pin::get_unchecked_mut(self) };

        let event = mut_self
            .event
            .expect("polled WaitForEventFuture after completion");

        let poll_res = unsafe { event.try_wait(&mut mut_self.wait_node, cx) };

        if let Poll::Ready(()) = poll_res {
            // The event was set
            mut_self.event = None;
        }

        poll_res
    }
}

impl<'a, MutexType: RawMutex> FusedFuture
    for GenericWaitForEventFuture<'a, MutexType>
{
    fn is_terminated(&self) -> bool {
        self.event.is_none()
    }
}

impl<'a, MutexType: RawMutex> Drop
    for GenericWaitForEventFuture<'a, MutexType>
{
    fn drop(&mut self) {
        // If this WaitForEventFuture has been polled and it was added to the
        // wait queue at the event, it must be removed before dropping.
        // Otherwise the event would access invalid memory.
        if let Some(ev) = self.event {
            ev.remove_waiter(&mut self.wait_node);
        }
    }
}

// Export a non thread-safe version using NoopLock

/// A [`GenericManualResetEvent`] which is not thread-safe.
pub type LocalManualResetEvent = GenericManualResetEvent<NoopLock>;
/// A [`GenericWaitForEventFuture`] for [`LocalManualResetEvent`].
pub type LocalWaitForEventFuture<'a> = GenericWaitForEventFuture<'a, NoopLock>;

#[cfg(feature = "std")]
mod if_std {
    use super::*;

    // Export a thread-safe version using parking_lot::RawMutex

    /// A [`GenericManualResetEvent`] implementation backed by [`parking_lot`].
    pub type ManualResetEvent = GenericManualResetEvent<parking_lot::RawMutex>;
    /// A [`GenericWaitForEventFuture`] for [`ManualResetEvent`].
    pub type WaitForEventFuture<'a> =
        GenericWaitForEventFuture<'a, parking_lot::RawMutex>;
}

#[cfg(feature = "std")]
pub use self::if_std::*;
