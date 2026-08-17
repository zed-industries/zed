//! An asynchronously awaitable mutex for synchronization between concurrently
//! executing futures.

use crate::{
    intrusive_double_linked_list::{LinkedList, ListNode},
    utils::update_waker_ref,
    NoopLock,
};
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    pin::Pin,
};
use futures_core::{
    future::{FusedFuture, Future},
    task::{Context, Poll, Waker},
};
use lock_api::{Mutex as LockApiMutex, RawMutex};

/// Tracks how the future had interacted with the mutex
#[derive(PartialEq)]
enum PollState {
    /// The task has never interacted with the mutex.
    New,
    /// The task was added to the wait queue at the mutex.
    Waiting,
    /// The task had previously waited on the mutex, but was notified
    /// that the mutex was released in the meantime.
    Notified,
    /// The task had been polled to completion.
    Done,
}

/// Tracks the MutexLockFuture waiting state.
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

/// Internal state of the `Mutex`
struct MutexState {
    is_fair: bool,
    is_locked: bool,
    waiters: LinkedList<WaitQueueEntry>,
}

impl MutexState {
    fn new(is_fair: bool) -> Self {
        MutexState {
            is_fair,
            is_locked: false,
            waiters: LinkedList::new(),
        }
    }

    /// Returns the `Waker` associated with the up the last waiter
    ///
    /// If the Mutex is not fair, removes the associated wait node also from
    /// the wait queue
    fn return_last_waiter(&mut self) -> Option<Waker> {
        let last_waiter = if self.is_fair {
            self.waiters.peek_last_mut()
        } else {
            self.waiters.remove_last()
        };

        if let Some(last_waiter) = last_waiter {
            // Notify the waiter that it can try to lock the mutex again.
            // The notification gets tracked inside the waiter.
            // If the waiter aborts it's wait (drops the future), another task
            // must be woken.
            last_waiter.state = PollState::Notified;

            let task = &mut last_waiter.task;
            return task.take();
        }

        None
    }

    fn is_locked(&self) -> bool {
        self.is_locked
    }

    /// Unlocks the mutex
    ///
    /// This is expected to be only called from the current holder of the mutex.
    /// The method returns the `Waker` which is associated with the task that
    /// needs to get woken due to the unlock.
    fn unlock(&mut self) -> Option<Waker> {
        if self.is_locked {
            self.is_locked = false;
            // TODO: Does this require a memory barrier for the actual data,
            // or is this covered by unlocking the mutex which protects the data?
            // Wakeup the last waiter
            self.return_last_waiter()
        } else {
            None
        }
    }

    /// Tries to lock the mutex synchronously.
    ///
    /// Returns true if the lock obtained and false otherwise.
    fn try_lock_sync(&mut self) -> bool {
        // The lock can only be obtained synchronously if
        // - it is not locked
        // - the Semaphore is either not fair, or there are no waiters
        // - required_permits == 0
        if !self.is_locked && (!self.is_fair || self.waiters.is_empty()) {
            self.is_locked = true;
            true
        } else {
            false
        }
    }

    /// Tries to acquire the Mutex from a WaitQueueEntry.
    ///
    /// If it isn't available, the WaitQueueEntry gets added to the wait
    /// queue at the Mutex, and will be signalled once ready.
    /// This function is only safe as long as the `wait_node`s address is guaranteed
    /// to be stable until it gets removed from the queue.
    unsafe fn try_lock(
        &mut self,
        wait_node: &mut ListNode<WaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<()> {
        match wait_node.state {
            PollState::New => {
                // The fast path - the Mutex isn't locked by anyone else.
                // If the mutex is fair, noone must be in the wait list before us.
                if self.try_lock_sync() {
                    wait_node.state = PollState::Done;
                    Poll::Ready(())
                } else {
                    // Add the task to the wait queue
                    wait_node.task = Some(cx.waker().clone());
                    wait_node.state = PollState::Waiting;
                    self.waiters.add_front(wait_node);
                    Poll::Pending
                }
            }
            PollState::Waiting => {
                // The MutexLockFuture is already in the queue.
                if self.is_fair {
                    // The task needs to wait until it gets notified in order to
                    // maintain the ordering. However the caller might have
                    // passed a different `Waker`. In this case we need to update it.
                    update_waker_ref(&mut wait_node.task, cx);
                    Poll::Pending
                } else {
                    // For throughput improvement purposes, grab the lock immediately
                    // if it's available.
                    if !self.is_locked {
                        self.is_locked = true;
                        wait_node.state = PollState::Done;
                        // Since this waiter has been registered before, it must
                        // get removed from the waiter list.
                        // Safety: Due to the state, we know that the node must be part
                        // of the waiter list
                        self.force_remove_waiter(wait_node);
                        Poll::Ready(())
                    } else {
                        // The caller might have passed a different `Waker`.
                        // In this case we need to update it.
                        update_waker_ref(&mut wait_node.task, cx);
                        Poll::Pending
                    }
                }
            }
            PollState::Notified => {
                // We had been woken by the mutex, since the mutex is available again.
                // The mutex thereby removed us from the waiters list.
                // Just try to lock again. If the mutex isn't available,
                // we need to add it to the wait queue again.
                if !self.is_locked {
                    if self.is_fair {
                        // In a fair Mutex, the WaitQueueEntry is kept in the
                        // linked list and must be removed here
                        // Safety: Due to the state, we know that the node must be part
                        // of the waiter list
                        self.force_remove_waiter(wait_node);
                    }
                    self.is_locked = true;
                    wait_node.state = PollState::Done;
                    Poll::Ready(())
                } else {
                    // Fair mutexes should always be able to acquire the lock
                    // after they had been notified
                    debug_assert!(!self.is_fair);
                    // Add to queue
                    wait_node.task = Some(cx.waker().clone());
                    wait_node.state = PollState::Waiting;
                    self.waiters.add_front(wait_node);
                    Poll::Pending
                }
            }
            PollState::Done => {
                // The future had been polled to completion before
                panic!("polled Mutex after completion");
            }
        }
    }

    /// Tries to remove a waiter from the wait queue, and panics if the
    /// waiter is no longer valid.
    unsafe fn force_remove_waiter(
        &mut self,
        wait_node: &mut ListNode<WaitQueueEntry>,
    ) {
        if !self.waiters.remove(wait_node) {
            // Panic if the address isn't found. This can only happen if the contract was
            // violated, e.g. the WaitQueueEntry got moved after the initial poll.
            panic!("Future could not be removed from wait queue");
        }
    }

    /// Removes the waiter from the list.
    ///
    /// This function is only safe as long as the reference that is passed here
    /// equals the reference/address under which the waiter was added.
    /// The waiter must not have been moved in between.
    ///
    /// Returns the `Waker` of another task which might get ready to run due to
    /// this.
    fn remove_waiter(
        &mut self,
        wait_node: &mut ListNode<WaitQueueEntry>,
    ) -> Option<Waker> {
        // MutexLockFuture only needs to get removed if it had been added to
        // the wait queue of the Mutex. This has happened in the PollState::Waiting case.
        // If the current waiter was notified, another waiter must get notified now.
        match wait_node.state {
            PollState::Notified => {
                if self.is_fair {
                    // In a fair Mutex, the WaitQueueEntry is kept in the
                    // linked list and must be removed here
                    // Safety: Due to the state, we know that the node must be part
                    // of the waiter list
                    unsafe { self.force_remove_waiter(wait_node) };
                }
                wait_node.state = PollState::Done;
                // Since the task was notified but did not lock the Mutex,
                // another task gets the chance to run.
                self.return_last_waiter()
            }
            PollState::Waiting => {
                // Remove the WaitQueueEntry from the linked list
                // Safety: Due to the state, we know that the node must be part
                // of the waiter list
                unsafe { self.force_remove_waiter(wait_node) };
                wait_node.state = PollState::Done;
                None
            }
            PollState::New | PollState::Done => None,
        }
    }
}

/// An RAII guard returned by the `lock` and `try_lock` methods.
/// When this structure is dropped (falls out of scope), the lock will be
/// unlocked.
pub struct GenericMutexGuard<'a, MutexType: RawMutex, T: 'a> {
    /// The Mutex which is associated with this Guard
    mutex: &'a GenericMutex<MutexType, T>,
}

impl<MutexType: RawMutex, T: core::fmt::Debug> core::fmt::Debug
    for GenericMutexGuard<'_, MutexType, T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GenericMutexGuard").finish()
    }
}

impl<MutexType: RawMutex, T> Drop for GenericMutexGuard<'_, MutexType, T> {
    fn drop(&mut self) {
        // Release the mutex
        let waker = { self.mutex.state.lock().unlock() };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

impl<MutexType: RawMutex, T> Deref for GenericMutexGuard<'_, MutexType, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<MutexType: RawMutex, T> DerefMut for GenericMutexGuard<'_, MutexType, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

// Safety: GenericMutexGuard may only be used across threads if the underlying
// type is Sync.
unsafe impl<MutexType: RawMutex, T: Sync> Sync
    for GenericMutexGuard<'_, MutexType, T>
{
}

/// A future which resolves when the target mutex has been successfully acquired.
#[must_use = "futures do nothing unless polled"]
pub struct GenericMutexLockFuture<'a, MutexType: RawMutex, T: 'a> {
    /// The Mutex which should get locked trough this Future
    mutex: Option<&'a GenericMutex<MutexType, T>>,
    /// Node for waiting at the mutex
    wait_node: ListNode<WaitQueueEntry>,
}

// Safety: Futures can be sent between threads as long as the underlying
// mutex is thread-safe (Sync), which allows to poll/register/unregister from
// a different thread.
unsafe impl<'a, MutexType: RawMutex + Sync, T: 'a> Send
    for GenericMutexLockFuture<'a, MutexType, T>
{
}

impl<'a, MutexType: RawMutex, T: core::fmt::Debug> core::fmt::Debug
    for GenericMutexLockFuture<'a, MutexType, T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GenericMutexLockFuture").finish()
    }
}

impl<'a, MutexType: RawMutex, T> Future
    for GenericMutexLockFuture<'a, MutexType, T>
{
    type Output = GenericMutexGuard<'a, MutexType, T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: The next operations are safe, because Pin promises us that
        // the address of the wait queue entry inside GenericMutexLockFuture is stable,
        // and we don't move any fields inside the future until it gets dropped.
        let mut_self: &mut GenericMutexLockFuture<MutexType, T> =
            unsafe { Pin::get_unchecked_mut(self) };

        let mutex = mut_self
            .mutex
            .expect("polled GenericMutexLockFuture after completion");
        let mut mutex_state = mutex.state.lock();

        let poll_res =
            unsafe { mutex_state.try_lock(&mut mut_self.wait_node, cx) };

        match poll_res {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => {
                // The mutex was acquired
                mut_self.mutex = None;
                Poll::Ready(GenericMutexGuard::<'a, MutexType, T> { mutex })
            }
        }
    }
}

impl<'a, MutexType: RawMutex, T> FusedFuture
    for GenericMutexLockFuture<'a, MutexType, T>
{
    fn is_terminated(&self) -> bool {
        self.mutex.is_none()
    }
}

impl<'a, MutexType: RawMutex, T> Drop
    for GenericMutexLockFuture<'a, MutexType, T>
{
    fn drop(&mut self) {
        // If this GenericMutexLockFuture has been polled and it was added to the
        // wait queue at the mutex, it must be removed before dropping.
        // Otherwise the mutex would access invalid memory.
        let waker = if let Some(mutex) = self.mutex {
            let mut mutex_state = mutex.state.lock();
            mutex_state.remove_waiter(&mut self.wait_node)
        } else {
            None
        };

        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

/// A futures-aware mutex.
pub struct GenericMutex<MutexType: RawMutex, T> {
    value: UnsafeCell<T>,
    state: LockApiMutex<MutexType, MutexState>,
}

// It is safe to send mutexes between threads, as long as they are not used and
// thereby borrowed
unsafe impl<T: Send, MutexType: RawMutex + Send> Send
    for GenericMutex<MutexType, T>
{
}
// The mutex is thread-safe as long as the utilized mutex is thread-safe
unsafe impl<T: Send, MutexType: RawMutex + Sync> Sync
    for GenericMutex<MutexType, T>
{
}

impl<MutexType: RawMutex, T: core::fmt::Debug> core::fmt::Debug
    for GenericMutex<MutexType, T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mutex")
            .field("is_locked", &self.is_locked())
            .finish()
    }
}

impl<MutexType: RawMutex, T> GenericMutex<MutexType, T> {
    /// Creates a new futures-aware mutex.
    ///
    /// `is_fair` defines whether the `Mutex` should behave be fair regarding the
    /// order of waiters. A fair `Mutex` will only allow the first waiter which
    /// tried to lock but failed to lock the `Mutex` once it's available again.
    /// Other waiters must wait until either this locking attempt completes, and
    /// the `Mutex` gets unlocked again, or until the `MutexLockFuture` which
    /// tried to gain the lock is dropped.
    pub fn new(value: T, is_fair: bool) -> GenericMutex<MutexType, T> {
        GenericMutex::<MutexType, T> {
            value: UnsafeCell::new(value),
            state: LockApiMutex::new(MutexState::new(is_fair)),
        }
    }

    /// Acquire the mutex asynchronously.
    ///
    /// This method returns a future that will resolve once the mutex has been
    /// successfully acquired.
    pub fn lock(&self) -> GenericMutexLockFuture<'_, MutexType, T> {
        GenericMutexLockFuture::<MutexType, T> {
            mutex: Some(&self),
            wait_node: ListNode::new(WaitQueueEntry::new()),
        }
    }

    /// Tries to acquire the mutex
    ///
    /// If acquiring the mutex is successful, a [`GenericMutexGuard`]
    /// will be returned, which allows to access the contained data.
    ///
    /// Otherwise `None` will be returned.
    pub fn try_lock(&self) -> Option<GenericMutexGuard<'_, MutexType, T>> {
        if self.state.lock().try_lock_sync() {
            Some(GenericMutexGuard { mutex: self })
        } else {
            None
        }
    }

    /// Returns whether the mutex is locked.
    pub fn is_locked(&self) -> bool {
        self.state.lock().is_locked()
    }
}

// Export a non thread-safe version using NoopLock

/// A [`GenericMutex`] which is not thread-safe.
pub type LocalMutex<T> = GenericMutex<NoopLock, T>;
/// A [`GenericMutexGuard`] for [`LocalMutex`].
pub type LocalMutexGuard<'a, T> = GenericMutexGuard<'a, NoopLock, T>;
/// A [`GenericMutexLockFuture`] for [`LocalMutex`].
pub type LocalMutexLockFuture<'a, T> = GenericMutexLockFuture<'a, NoopLock, T>;

#[cfg(feature = "std")]
mod if_std {
    use super::*;

    // Export a thread-safe version using parking_lot::RawMutex

    /// A [`GenericMutex`] backed by [`parking_lot`].
    pub type Mutex<T> = GenericMutex<parking_lot::RawMutex, T>;
    /// A [`GenericMutexGuard`] for [`Mutex`].
    pub type MutexGuard<'a, T> =
        GenericMutexGuard<'a, parking_lot::RawMutex, T>;
    /// A [`GenericMutexLockFuture`] for [`Mutex`].
    pub type MutexLockFuture<'a, T> =
        GenericMutexLockFuture<'a, parking_lot::RawMutex, T>;
}

#[cfg(feature = "std")]
pub use self::if_std::*;
