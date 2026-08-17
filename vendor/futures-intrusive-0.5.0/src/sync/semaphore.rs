//! An asynchronously awaitable semaphore for synchronization between concurrently
//! executing futures.

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
use lock_api::{Mutex as LockApiMutex, RawMutex};

/// Tracks how the future had interacted with the semaphore
#[derive(PartialEq)]
enum PollState {
    /// The task has never interacted with the semaphore.
    New,
    /// The task was added to the wait queue at the semaphore.
    Waiting,
    /// The task had previously waited on the semaphore, but was notified
    /// that the semaphore was released in the meantime and that the task
    /// thereby could retry.
    Notified,
    /// The task had been polled to completion.
    Done,
}

/// Tracks the SemaphoreAcquireFuture waiting state.
struct WaitQueueEntry {
    /// The task handle of the waiting task
    task: Option<Waker>,
    /// Current polling state
    state: PollState,
    /// The amount of permits that should be obtained
    required_permits: usize,
}

impl WaitQueueEntry {
    /// Creates a new WaitQueueEntry
    fn new(required_permits: usize) -> WaitQueueEntry {
        WaitQueueEntry {
            task: None,
            state: PollState::New,
            required_permits,
        }
    }
}

/// Internal state of the `Semaphore`
struct SemaphoreState {
    is_fair: bool,
    permits: usize,
    waiters: LinkedList<WaitQueueEntry>,
}

impl SemaphoreState {
    fn new(is_fair: bool, permits: usize) -> Self {
        SemaphoreState {
            is_fair,
            permits,
            waiters: LinkedList::new(),
        }
    }

    /// Wakes up the last waiter and removes it from the wait queue
    fn wakeup_waiters(&mut self) {
        // Wake as many tasks as the permits allow
        let mut available = self.permits;

        loop {
            match self.waiters.peek_last_mut() {
                None => return,
                Some(last_waiter) => {
                    // Check if enough permits are available for this waiter.
                    // If not then a wakeup attempt won't be successful.
                    if available < last_waiter.required_permits {
                        return;
                    }
                    available -= last_waiter.required_permits;

                    // Notify the waiter that it can try to acquire the semaphore again.
                    // The notification gets tracked inside the waiter.
                    // If the waiter aborts it's wait (drops the future), another task
                    // must be woken.
                    if last_waiter.state != PollState::Notified {
                        last_waiter.state = PollState::Notified;

                        let task = &last_waiter.task;
                        if let Some(ref handle) = task {
                            handle.wake_by_ref();
                        }
                    }

                    // In the case of a non-fair semaphore, the waiters are directly
                    // removed from the semaphores wait queue when woken.
                    // That avoids having to remove the wait element later.
                    if !self.is_fair {
                        self.waiters.remove_last();
                    } else {
                        // For a fair Semaphore we never wake more than 1 task.
                        // That one needs to acquire the Semaphore.
                        // TODO: We actually should be able to wake more, since
                        // it's guaranteed that both tasks could make progress.
                        // However the we currently can't peek iterate in reverse order.
                        return;
                    }
                }
            }
        }
    }

    fn permits(&self) -> usize {
        self.permits
    }

    /// Releases a certain amount of permits back to the semaphore
    fn release(&mut self, permits: usize) {
        if permits == 0 {
            return;
        }
        // TODO: Overflow check
        self.permits += permits;

        // Wakeup the last waiter
        self.wakeup_waiters();
    }

    /// Tries to acquire the given amount of permits synchronously.
    ///
    /// Returns true if the permits were obtained and false otherwise.
    fn try_acquire_sync(&mut self, required_permits: usize) -> bool {
        // Permits can only be obtained synchronously if there are
        // - enough permits available
        // - the Semaphore is either not fair, or there are no waiters
        // - required_permits == 0
        if (self.permits >= required_permits)
            && (!self.is_fair
                || self.waiters.is_empty()
                || required_permits == 0)
        {
            self.permits -= required_permits;
            true
        } else {
            false
        }
    }

    /// Tries to acquire the Semaphore from a WaitQueueEntry.
    /// If it isn't available, the WaitQueueEntry gets added to the wait
    /// queue at the Semaphore, and will be signalled once ready.
    /// This function is only safe as long as the `wait_node`s address is guaranteed
    /// to be stable until it gets removed from the queue.
    unsafe fn try_acquire(
        &mut self,
        wait_node: &mut ListNode<WaitQueueEntry>,
        cx: &mut Context<'_>,
    ) -> Poll<()> {
        match wait_node.state {
            PollState::New => {
                // The fast path - enough permits are available
                if self.try_acquire_sync(wait_node.required_permits) {
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
                // The SemaphoreAcquireFuture is already in the queue.
                if self.is_fair {
                    // The task needs to wait until it gets notified in order to
                    // maintain the ordering.
                    // However the caller might have passed a different `Waker`.
                    // In this case we need to update it.
                    update_waker_ref(&mut wait_node.task, cx);
                    Poll::Pending
                } else {
                    // For throughput improvement purposes, check immediately
                    // if enough permits are available
                    if self.permits >= wait_node.required_permits {
                        self.permits -= wait_node.required_permits;
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
                // We had been woken by the semaphore, since the semaphore is available again.
                // The semaphore thereby removed us from the waiters list.
                // Just try to lock again. If the semaphore isn't available,
                // we need to add it to the wait queue again.
                if self.permits >= wait_node.required_permits {
                    if self.is_fair {
                        // In a fair Semaphore, the WaitQueueEntry is kept in the
                        // linked list and must be removed here
                        // Safety: Due to the state, we know that the node must be part
                        // of the waiter list
                        self.force_remove_waiter(wait_node);
                    }
                    self.permits -= wait_node.required_permits;
                    if self.is_fair {
                        // There might be another task which is ready to run,
                        // but couldn't, since it was blocked behind the fair waiter.
                        self.wakeup_waiters();
                    }
                    wait_node.state = PollState::Done;
                    Poll::Ready(())
                } else {
                    // A fair semaphore should never end up in that branch, since
                    // it's only notified when it's permits are guaranteed to
                    // be available. assert! in order to find logic bugs
                    assert!(
                        !self.is_fair,
                        "Fair semaphores should always be ready when notified"
                    );
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
    /// This function is only safe as long as the reference that is passed here
    /// equals the reference/address under which the waiter was added.
    /// The waiter must not have been moved in between.
    fn remove_waiter(&mut self, wait_node: &mut ListNode<WaitQueueEntry>) {
        // SemaphoreAcquireFuture only needs to get removed if it had been added to
        // the wait queue of the Semaphore. This has happened in the PollState::Waiting case.
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
                // Wakeup more waiters
                self.wakeup_waiters();
            }
            PollState::Waiting => {
                // Remove the WaitQueueEntry from the linked list
                // Safety: Due to the state, we know that the node must be part
                // of the waiter list
                unsafe { self.force_remove_waiter(wait_node) };
                wait_node.state = PollState::Done;
            }
            PollState::New | PollState::Done => {}
        }
    }
}

/// An RAII guard returned by the `acquire` and `try_acquire` methods.
///
/// When this structure is dropped (falls out of scope),
/// the amount of permits that was used in the `acquire()` call will be released
/// back to the Semaphore.
pub struct GenericSemaphoreReleaser<'a, MutexType: RawMutex> {
    /// The Semaphore which is associated with this Releaser
    semaphore: &'a GenericSemaphore<MutexType>,
    /// The amount of permits to release
    permits: usize,
}

impl<MutexType: RawMutex> core::fmt::Debug
    for GenericSemaphoreReleaser<'_, MutexType>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GenericSemaphoreReleaser").finish()
    }
}

impl<MutexType: RawMutex> GenericSemaphoreReleaser<'_, MutexType> {
    /// Prevents the SemaphoreReleaser from automatically releasing the permits
    /// when it gets dropped.
    /// This is helpful if the permits must be acquired for a longer lifetime
    /// than the one of the SemaphoreReleaser.
    /// If this method is used it is important to release the acquired permits
    /// manually back to the Semaphore.
    pub fn disarm(&mut self) -> usize {
        let permits = self.permits;
        self.permits = 0;
        permits
    }
}

impl<MutexType: RawMutex> Drop for GenericSemaphoreReleaser<'_, MutexType> {
    fn drop(&mut self) {
        // Release the requested amount of permits to the semaphore
        if self.permits != 0 {
            self.semaphore.state.lock().release(self.permits);
        }
    }
}

/// A future which resolves when the target semaphore has been successfully acquired.
#[must_use = "futures do nothing unless polled"]
pub struct GenericSemaphoreAcquireFuture<'a, MutexType: RawMutex> {
    /// The Semaphore which should get acquired trough this Future
    semaphore: Option<&'a GenericSemaphore<MutexType>>,
    /// Node for waiting at the semaphore
    wait_node: ListNode<WaitQueueEntry>,
    /// Whether the obtained permits should automatically be released back
    /// to the semaphore.
    auto_release: bool,
}

// Safety: Futures can be sent between threads as long as the underlying
// semaphore is thread-safe (Sync), which allows to poll/register/unregister from
// a different thread.
unsafe impl<'a, MutexType: RawMutex + Sync> Send
    for GenericSemaphoreAcquireFuture<'a, MutexType>
{
}

impl<'a, MutexType: RawMutex> core::fmt::Debug
    for GenericSemaphoreAcquireFuture<'a, MutexType>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GenericSemaphoreAcquireFuture").finish()
    }
}

impl<'a, MutexType: RawMutex> Future
    for GenericSemaphoreAcquireFuture<'a, MutexType>
{
    type Output = GenericSemaphoreReleaser<'a, MutexType>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: The next operations are safe, because Pin promises us that
        // the address of the wait queue entry inside GenericSemaphoreAcquireFuture is stable,
        // and we don't move any fields inside the future until it gets dropped.
        let mut_self: &mut GenericSemaphoreAcquireFuture<MutexType> =
            unsafe { Pin::get_unchecked_mut(self) };

        let semaphore = mut_self
            .semaphore
            .expect("polled GenericSemaphoreAcquireFuture after completion");
        let mut semaphore_state = semaphore.state.lock();

        let poll_res =
            unsafe { semaphore_state.try_acquire(&mut mut_self.wait_node, cx) };

        match poll_res {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => {
                // The semaphore was acquired.
                mut_self.semaphore = None;
                let to_release = match mut_self.auto_release {
                    true => mut_self.wait_node.required_permits,
                    false => 0,
                };
                Poll::Ready(GenericSemaphoreReleaser::<'a, MutexType> {
                    semaphore,
                    permits: to_release,
                })
            }
        }
    }
}

impl<'a, MutexType: RawMutex> FusedFuture
    for GenericSemaphoreAcquireFuture<'a, MutexType>
{
    fn is_terminated(&self) -> bool {
        self.semaphore.is_none()
    }
}

impl<'a, MutexType: RawMutex> Drop
    for GenericSemaphoreAcquireFuture<'a, MutexType>
{
    fn drop(&mut self) {
        // If this GenericSemaphoreAcquireFuture has been polled and it was added to the
        // wait queue at the semaphore, it must be removed before dropping.
        // Otherwise the semaphore would access invalid memory.
        if let Some(semaphore) = self.semaphore {
            let mut semaphore_state = semaphore.state.lock();
            // Analysis: Does the number of permits play a role here?
            // The future was notified because there was a certain amount of permits
            // available.
            // Removing the waiter will wake up as many tasks as there are permits
            // available inside the Semaphore now. If this is bigger than the
            // amount of permits required for this task, then additional new
            // tasks might get woken. However that isn't bad, since
            // those tasks should get into the wait state anyway.
            semaphore_state.remove_waiter(&mut self.wait_node);
        }
    }
}

/// A futures-aware semaphore.
pub struct GenericSemaphore<MutexType: RawMutex> {
    state: LockApiMutex<MutexType, SemaphoreState>,
}

// It is safe to send semaphores between threads, as long as they are not used and
// thereby borrowed
unsafe impl<MutexType: RawMutex + Send> Send for GenericSemaphore<MutexType> {}
// The Semaphore is thread-safe as long as the utilized Mutex is thread-safe
unsafe impl<MutexType: RawMutex + Sync> Sync for GenericSemaphore<MutexType> {}

impl<MutexType: RawMutex> core::fmt::Debug for GenericSemaphore<MutexType> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Semaphore")
            .field("permits", &self.permits())
            .finish()
    }
}

impl<MutexType: RawMutex> GenericSemaphore<MutexType> {
    /// Creates a new futures-aware semaphore.
    ///
    /// `is_fair` defines whether the `Semaphore` should behave be fair regarding the
    /// order of waiters. A fair `Semaphore` will only allow the oldest waiter on
    /// a `Semaphore` to retry acquiring it once it's available again.
    /// Other waiters must wait until either this acquire attempt completes, and
    /// the `Semaphore` has enough permits after that, or until the
    /// [`SemaphoreAcquireFuture`] which tried to acquire the `Semaphore` is dropped.
    ///
    /// If the `Semaphore` isn't fair, waiters that wait for a high amount of
    /// permits might never succeed since the permits might be stolen in between
    /// by other waiters. Therefore use-cases which make use of very different
    /// amount of permits per acquire should use fair semaphores.
    /// For use-cases where each `acquire()` tries to acquire the same amount of
    /// permits an unfair `Semaphore` might provide throughput advantages.
    ///
    /// `permits` is the amount of permits that a semaphore should hold when
    /// created.
    pub fn new(is_fair: bool, permits: usize) -> GenericSemaphore<MutexType> {
        GenericSemaphore::<MutexType> {
            state: LockApiMutex::new(SemaphoreState::new(is_fair, permits)),
        }
    }

    /// Acquire a certain amount of permits on a semaphore asynchronously.
    ///
    /// This method returns a future that will resolve once the given amount of
    /// permits have been acquired.
    /// The Future will resolve to a [`GenericSemaphoreReleaser`], which will
    /// release all acquired permits automatically when dropped.
    pub fn acquire(
        &self,
        nr_permits: usize,
    ) -> GenericSemaphoreAcquireFuture<'_, MutexType> {
        GenericSemaphoreAcquireFuture::<MutexType> {
            semaphore: Some(&self),
            wait_node: ListNode::new(WaitQueueEntry::new(nr_permits)),
            auto_release: true,
        }
    }

    /// Tries to acquire a certain amount of permits on a semaphore.
    ///
    /// If acquiring the permits is successful, a [`GenericSemaphoreReleaser`]
    /// will be returned, which will release all acquired permits automatically
    /// when dropped.
    ///
    /// Otherwise `None` will be returned.
    pub fn try_acquire(
        &self,
        nr_permits: usize,
    ) -> Option<GenericSemaphoreReleaser<'_, MutexType>> {
        if self.state.lock().try_acquire_sync(nr_permits) {
            Some(GenericSemaphoreReleaser {
                semaphore: self,
                permits: nr_permits,
            })
        } else {
            None
        }
    }

    /// Releases the given amount of permits back to the semaphore.
    ///
    /// This method should in most cases not be used, since the
    /// [`GenericSemaphoreReleaser`] which is obtained when acquiring a Semaphore
    /// will automatically release the obtained permits again.
    ///
    /// Therefore this method should only be used if the automatic release was
    /// disabled by calling [`GenericSemaphoreReleaser::disarm`],
    /// or when the amount of permits in the Semaphore
    /// should increase from the initial amount.
    pub fn release(&self, nr_permits: usize) {
        self.state.lock().release(nr_permits)
    }

    /// Returns the amount of permits that are available on the semaphore
    pub fn permits(&self) -> usize {
        self.state.lock().permits()
    }
}

// Export a non thread-safe version using NoopLock

/// A [`GenericSemaphore`] which is not thread-safe.
pub type LocalSemaphore = GenericSemaphore<NoopLock>;
/// A [`GenericSemaphoreReleaser`] for [`LocalSemaphore`].
pub type LocalSemaphoreReleaser<'a> = GenericSemaphoreReleaser<'a, NoopLock>;
/// A [`GenericSemaphoreAcquireFuture`] for [`LocalSemaphore`].
pub type LocalSemaphoreAcquireFuture<'a> =
    GenericSemaphoreAcquireFuture<'a, NoopLock>;

#[cfg(feature = "std")]
mod if_std {
    use super::*;

    // Export a thread-safe version using parking_lot::RawMutex

    /// A [`GenericSemaphore`] backed by [`parking_lot`].
    pub type Semaphore = GenericSemaphore<parking_lot::RawMutex>;
    /// A [`GenericSemaphoreReleaser`] for [`Semaphore`].
    pub type SemaphoreReleaser<'a> =
        GenericSemaphoreReleaser<'a, parking_lot::RawMutex>;
    /// A [`GenericSemaphoreAcquireFuture`] for [`Semaphore`].
    pub type SemaphoreAcquireFuture<'a> =
        GenericSemaphoreAcquireFuture<'a, parking_lot::RawMutex>;
}

#[cfg(feature = "std")]
pub use self::if_std::*;

#[cfg(feature = "alloc")]
mod if_alloc {
    use super::*;

    use alloc::sync::Arc;

    /// An RAII guard returned by the `acquire` and `try_acquire` methods.
    ///
    /// When this structure is dropped (falls out of scope),
    /// the amount of permits that was used in the `acquire()` call will be released
    /// back to the Semaphore.
    pub struct GenericSharedSemaphoreReleaser<MutexType: RawMutex> {
        /// The Semaphore which is associated with this Releaser
        semaphore: GenericSharedSemaphore<MutexType>,
        /// The amount of permits to release
        permits: usize,
    }

    impl<MutexType: RawMutex> core::fmt::Debug
        for GenericSharedSemaphoreReleaser<MutexType>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GenericSharedSemaphoreReleaser").finish()
        }
    }

    impl<MutexType: RawMutex> GenericSharedSemaphoreReleaser<MutexType> {
        /// Prevents the SharedSemaphoreReleaser from automatically releasing the permits
        /// when it gets dropped.
        ///
        /// This is helpful if the permits must be acquired for a longer lifetime
        /// than the one of the SemaphoreReleaser.
        ///
        /// If this method is used it is important to release the acquired permits
        /// manually back to the Semaphore.
        pub fn disarm(&mut self) -> usize {
            let permits = self.permits;
            self.permits = 0;
            permits
        }
    }

    impl<MutexType: RawMutex> Drop for GenericSharedSemaphoreReleaser<MutexType> {
        fn drop(&mut self) {
            // Release the requested amount of permits to the semaphore
            if self.permits != 0 {
                self.semaphore.state.lock().release(self.permits);
            }
        }
    }

    /// A future which resolves when the target semaphore has been successfully acquired.
    #[must_use = "futures do nothing unless polled"]
    pub struct GenericSharedSemaphoreAcquireFuture<MutexType: RawMutex> {
        /// The Semaphore which should get acquired trough this Future
        semaphore: Option<GenericSharedSemaphore<MutexType>>,
        /// Node for waiting at the semaphore
        wait_node: ListNode<WaitQueueEntry>,
        /// Whether the obtained permits should automatically be released back
        /// to the semaphore.
        auto_release: bool,
    }

    // Safety: Futures can be sent between threads as long as the underlying
    // semaphore is thread-safe (Sync), which allows to poll/register/unregister from
    // a different thread.
    unsafe impl<MutexType: RawMutex + Sync> Send
        for GenericSharedSemaphoreAcquireFuture<MutexType>
    {
    }

    impl<MutexType: RawMutex> core::fmt::Debug
        for GenericSharedSemaphoreAcquireFuture<MutexType>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GenericSharedSemaphoreAcquireFuture")
                .finish()
        }
    }

    impl<MutexType: RawMutex> Future
        for GenericSharedSemaphoreAcquireFuture<MutexType>
    {
        type Output = GenericSharedSemaphoreReleaser<MutexType>;

        fn poll(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Self::Output> {
            // Safety: The next operations are safe, because Pin promises us that
            // the address of the wait queue entry inside
            // GenericSharedSemaphoreAcquireFuture is stable,
            // and we don't move any fields inside the future until it gets dropped.
            let mut_self: &mut GenericSharedSemaphoreAcquireFuture<MutexType> =
                unsafe { Pin::get_unchecked_mut(self) };

            let semaphore = mut_self.semaphore.take().expect(
                "polled GenericSharedSemaphoreAcquireFuture after completion",
            );

            let poll_res = unsafe {
                let mut semaphore_state = semaphore.state.lock();
                semaphore_state.try_acquire(&mut mut_self.wait_node, cx)
            };

            match poll_res {
                Poll::Pending => {
                    mut_self.semaphore.replace(semaphore);
                    Poll::Pending
                }
                Poll::Ready(()) => {
                    let to_release = match mut_self.auto_release {
                        true => mut_self.wait_node.required_permits,
                        false => 0,
                    };
                    Poll::Ready(GenericSharedSemaphoreReleaser::<MutexType> {
                        semaphore,
                        permits: to_release,
                    })
                }
            }
        }
    }

    impl<MutexType: RawMutex> FusedFuture
        for GenericSharedSemaphoreAcquireFuture<MutexType>
    {
        fn is_terminated(&self) -> bool {
            self.semaphore.is_none()
        }
    }

    impl<MutexType: RawMutex> Drop
        for GenericSharedSemaphoreAcquireFuture<MutexType>
    {
        fn drop(&mut self) {
            // If this GenericSharedSemaphoreAcquireFuture has been polled and it was added to the
            // wait queue at the semaphore, it must be removed before dropping.
            // Otherwise the semaphore would access invalid memory.
            if let Some(semaphore) = self.semaphore.take() {
                let mut semaphore_state = semaphore.state.lock();
                // Analysis: Does the number of permits play a role here?
                // The future was notified because there was a certain amount of permits
                // available.
                // Removing the waiter will wake up as many tasks as there are permits
                // available inside the Semaphore now. If this is bigger than the
                // amount of permits required for this task, then additional new
                // tasks might get woken. However that isn't bad, since
                // those tasks should get into the wait state anyway.
                semaphore_state.remove_waiter(&mut self.wait_node);
            }
        }
    }

    /// A futures-aware shared semaphore.
    pub struct GenericSharedSemaphore<MutexType: RawMutex> {
        state: Arc<LockApiMutex<MutexType, SemaphoreState>>,
    }

    impl<MutexType: RawMutex> Clone for GenericSharedSemaphore<MutexType> {
        fn clone(&self) -> Self {
            Self {
                state: self.state.clone(),
            }
        }
    }

    // It is safe to send semaphores between threads, as long as they are not used and
    // thereby borrowed
    unsafe impl<MutexType: RawMutex + Send + Sync> Send
        for GenericSharedSemaphore<MutexType>
    {
    }
    // The Semaphore is thread-safe as long as the utilized Mutex is thread-safe
    unsafe impl<MutexType: RawMutex + Sync> Sync
        for GenericSharedSemaphore<MutexType>
    {
    }

    impl<MutexType: RawMutex> core::fmt::Debug
        for GenericSharedSemaphore<MutexType>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Semaphore")
                .field("permits", &self.permits())
                .finish()
        }
    }

    impl<MutexType: RawMutex> GenericSharedSemaphore<MutexType> {
        /// Creates a new futures-aware shared semaphore.
        ///
        /// See `GenericSharedSemaphore` for more information.
        pub fn new(
            is_fair: bool,
            permits: usize,
        ) -> GenericSharedSemaphore<MutexType> {
            GenericSharedSemaphore::<MutexType> {
                state: Arc::new(LockApiMutex::new(SemaphoreState::new(
                    is_fair, permits,
                ))),
            }
        }

        /// Acquire a certain amount of permits on a semaphore asynchronously.
        ///
        /// This method returns a future that will resolve once the given amount of
        /// permits have been acquired.
        /// The Future will resolve to a [`GenericSharedSemaphoreReleaser`], which will
        /// release all acquired permits automatically when dropped.
        pub fn acquire(
            &self,
            nr_permits: usize,
        ) -> GenericSharedSemaphoreAcquireFuture<MutexType> {
            GenericSharedSemaphoreAcquireFuture::<MutexType> {
                semaphore: Some(self.clone()),
                wait_node: ListNode::new(WaitQueueEntry::new(nr_permits)),
                auto_release: true,
            }
        }

        /// Tries to acquire a certain amount of permits on a semaphore.
        ///
        /// If acquiring the permits is successful, a [`GenericSharedSemaphoreReleaser`]
        /// will be returned, which will release all acquired permits automatically
        /// when dropped.
        ///
        /// Otherwise `None` will be returned.
        pub fn try_acquire(
            &self,
            nr_permits: usize,
        ) -> Option<GenericSharedSemaphoreReleaser<MutexType>> {
            if self.state.lock().try_acquire_sync(nr_permits) {
                Some(GenericSharedSemaphoreReleaser {
                    semaphore: self.clone(),
                    permits: nr_permits,
                })
            } else {
                None
            }
        }

        /// Releases the given amount of permits back to the semaphore.
        ///
        /// This method should in most cases not be used, since the
        /// [`GenericSharedSemaphoreReleaser`] which is obtained when acquiring a Semaphore
        /// will automatically release the obtained permits again.
        ///
        /// Therefore this method should only be used if the automatic release was
        /// disabled by calling [`GenericSharedSemaphoreReleaser::disarm`],
        /// or when the amount of permits in the Semaphore
        /// should increase from the initial amount.
        pub fn release(&self, nr_permits: usize) {
            self.state.lock().release(nr_permits)
        }

        /// Returns the amount of permits that are available on the semaphore
        pub fn permits(&self) -> usize {
            self.state.lock().permits()
        }
    }

    // Export parking_lot based shared semaphores in std mode
    #[cfg(feature = "std")]
    mod if_std {
        use super::*;

        /// A [`GenericSharedSemaphore`] backed by [`parking_lot`].
        pub type SharedSemaphore =
            GenericSharedSemaphore<parking_lot::RawMutex>;
        /// A [`GenericSharedSemaphoreReleaser`] for [`SharedSemaphore`].
        pub type SharedSemaphoreReleaser =
            GenericSharedSemaphoreReleaser<parking_lot::RawMutex>;
        /// A [`GenericSharedSemaphoreAcquireFuture`] for [`SharedSemaphore`].
        pub type SharedSemaphoreAcquireFuture =
            GenericSharedSemaphoreAcquireFuture<parking_lot::RawMutex>;
    }

    #[cfg(feature = "std")]
    pub use self::if_std::*;
}

#[cfg(feature = "alloc")]
pub use self::if_alloc::*;
