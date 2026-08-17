use super::AbortOnDropHandle;
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    runtime::Handle,
    task::{AbortHandle, Id, JoinError, JoinHandle},
};

/// A FIFO queue for of tasks spawned on a Tokio runtime.
///
/// A [`JoinQueue`] can be used to await the completion of the tasks in FIFO
/// order. That is, if tasks are spawned in the order A, B, C, then
/// awaiting the next completed task will always return A first, then B,
/// then C, regardless of the order in which the tasks actually complete.
///
/// All of the tasks must have the same return type `T`.
///
/// When the [`JoinQueue`] is dropped, all tasks in the [`JoinQueue`] are
/// immediately aborted.
pub struct JoinQueue<T>(VecDeque<AbortOnDropHandle<T>>);

impl<T> JoinQueue<T> {
    /// Create a new empty [`JoinQueue`].
    pub const fn new() -> Self {
        Self(VecDeque::new())
    }

    /// Creates an empty [`JoinQueue`] with space for at least `capacity` tasks.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }

    /// Returns the number of tasks currently in the [`JoinQueue`].
    ///
    /// This includes both tasks that are currently running and tasks that have
    /// completed but not yet been removed from the queue because outputting of
    /// them waits for FIFO order.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the [`JoinQueue`] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Spawn the provided task on the [`JoinQueue`], returning an [`AbortHandle`]
    /// that can be used to remotely cancel the task.
    ///
    /// The provided future will start running in the background immediately
    /// when this method is called, even if you don't await anything on this
    /// [`JoinQueue`].
    ///
    /// # Panics
    ///
    /// This method panics if called outside of a Tokio runtime.
    ///
    /// [`AbortHandle`]: tokio::task::AbortHandle
    #[track_caller]
    pub fn spawn<F>(&mut self, task: F) -> AbortHandle
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.push_back(tokio::spawn(task))
    }

    /// Spawn the provided task on the provided runtime and store it in this
    /// [`JoinQueue`] returning an [`AbortHandle`] that can be used to remotely
    /// cancel the task.
    ///
    /// The provided future will start running in the background immediately
    /// when this method is called, even if you don't await anything on this
    /// [`JoinQueue`].
    ///
    /// [`AbortHandle`]: tokio::task::AbortHandle
    #[track_caller]
    pub fn spawn_on<F>(&mut self, task: F, handle: &Handle) -> AbortHandle
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.push_back(handle.spawn(task))
    }

    /// Spawn the provided task on the current [`LocalSet`] or [`LocalRuntime`]
    /// and store it in this [`JoinQueue`], returning an [`AbortHandle`] that
    /// can be used to remotely cancel the task.
    ///
    /// The provided future will start running in the background immediately
    /// when this method is called, even if you don't await anything on this
    /// [`JoinQueue`].
    ///
    /// # Panics
    ///
    /// This method panics if it is called outside of a `LocalSet` or `LocalRuntime`.
    ///
    /// [`LocalSet`]: tokio::task::LocalSet
    /// [`LocalRuntime`]: tokio::runtime::LocalRuntime
    /// [`AbortHandle`]: tokio::task::AbortHandle
    #[track_caller]
    pub fn spawn_local<F>(&mut self, task: F) -> AbortHandle
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        self.push_back(tokio::task::spawn_local(task))
    }

    /// Spawn the blocking code on the blocking threadpool and store
    /// it in this [`JoinQueue`], returning an [`AbortHandle`] that can be
    /// used to remotely cancel the task.
    ///
    /// # Panics
    ///
    /// This method panics if called outside of a Tokio runtime.
    ///
    /// [`AbortHandle`]: tokio::task::AbortHandle
    #[track_caller]
    pub fn spawn_blocking<F>(&mut self, f: F) -> AbortHandle
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.push_back(tokio::task::spawn_blocking(f))
    }

    /// Spawn the blocking code on the blocking threadpool of the
    /// provided runtime and store it in this [`JoinQueue`], returning an
    /// [`AbortHandle`] that can be used to remotely cancel the task.
    ///
    /// [`AbortHandle`]: tokio::task::AbortHandle
    #[track_caller]
    pub fn spawn_blocking_on<F>(&mut self, f: F, handle: &Handle) -> AbortHandle
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.push_back(handle.spawn_blocking(f))
    }

    fn push_back(&mut self, jh: JoinHandle<T>) -> AbortHandle {
        let jh = AbortOnDropHandle::new(jh);
        let abort_handle = jh.abort_handle();
        self.0.push_back(jh);
        abort_handle
    }

    /// Waits until the next task in FIFO order completes and returns its output.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Cancel Safety
    ///
    /// This method is cancel safe. If `join_next` is used as the event in a `tokio::select!`
    /// statement and some other branch completes first, it is guaranteed that no tasks were
    /// removed from this [`JoinQueue`].
    pub async fn join_next(&mut self) -> Option<Result<T, JoinError>> {
        std::future::poll_fn(|cx| self.poll_join_next(cx)).await
    }

    /// Waits until the next task in FIFO order completes and returns its output,
    /// along with the [task ID] of the completed task.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// When this method returns an error, then the id of the task that failed can be accessed
    /// using the [`JoinError::id`] method.
    ///
    /// # Cancel Safety
    ///
    /// This method is cancel safe. If `join_next_with_id` is used as the event in a `tokio::select!`
    /// statement and some other branch completes first, it is guaranteed that no tasks were
    /// removed from this [`JoinQueue`].
    ///
    /// [task ID]: tokio::task::Id
    /// [`JoinError::id`]: fn@tokio::task::JoinError::id
    pub async fn join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>> {
        std::future::poll_fn(|cx| self.poll_join_next_with_id(cx)).await
    }

    /// Tries to poll an `AbortOnDropHandle` without blocking or yielding.
    ///
    /// Note that on success the handle will panic on subsequent polls
    /// since it becomes consumed.
    fn try_poll_handle(jh: &mut AbortOnDropHandle<T>) -> Option<Result<T, JoinError>> {
        let waker = futures_util::task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Since this function is not async and cannot be forced to yield, we should
        // disable budgeting when we want to check for the `JoinHandle` readiness.
        let jh = std::pin::pin!(tokio::task::coop::unconstrained(jh));
        if let Poll::Ready(res) = jh.poll(&mut cx) {
            Some(res)
        } else {
            None
        }
    }

    /// Tries to join the next task in FIFO order if it has completed.
    ///
    /// Returns `None` if the queue is empty or if the next task is not yet ready.
    pub fn try_join_next(&mut self) -> Option<Result<T, JoinError>> {
        let jh = self.0.front_mut()?;
        let res = Self::try_poll_handle(jh)?;
        // Use `detach` to avoid calling `abort` on a task that has already completed.
        // Dropping `AbortOnDropHandle` would abort the task, but since it is finished,
        // we only need to drop the `JoinHandle` for cleanup.
        drop(self.0.pop_front().unwrap().detach());
        Some(res)
    }

    /// Tries to join the next task in FIFO order if it has completed and return its output,
    /// along with its [task ID].
    ///
    /// Returns `None` if the queue is empty or if the next task is not yet ready.
    ///
    /// When this method returns an error, then the id of the task that failed can be accessed
    /// using the [`JoinError::id`] method.
    ///
    /// [task ID]: tokio::task::Id
    /// [`JoinError::id`]: fn@tokio::task::JoinError::id
    pub fn try_join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>> {
        let jh = self.0.front_mut()?;
        let res = Self::try_poll_handle(jh)?;
        // Use `detach` to avoid calling `abort` on a task that has already completed.
        // Dropping `AbortOnDropHandle` would abort the task, but since it is finished,
        // we only need to drop the `JoinHandle` for cleanup.
        let jh = self.0.pop_front().unwrap().detach();
        let id = jh.id();
        drop(jh);
        Some(res.map(|output| (id, output)))
    }

    /// Aborts all tasks and waits for them to finish shutting down.
    ///
    /// Calling this method is equivalent to calling [`abort_all`] and then calling [`join_next`] in
    /// a loop until it returns `None`.
    ///
    /// This method ignores any panics in the tasks shutting down. When this call returns, the
    /// [`JoinQueue`] will be empty.
    ///
    /// [`abort_all`]: fn@Self::abort_all
    /// [`join_next`]: fn@Self::join_next
    pub async fn shutdown(&mut self) {
        self.abort_all();
        while self.join_next().await.is_some() {}
    }

    /// Awaits the completion of all tasks in this [`JoinQueue`], returning a vector of their results.
    ///
    /// The results will be stored in the order they were spawned, not the order they completed.
    /// This is a convenience method that is equivalent to calling [`join_next`] in
    /// a loop. If any tasks on the [`JoinQueue`] fail with an [`JoinError`], then this call
    /// to `join_all` will panic and all remaining tasks on the [`JoinQueue`] are
    /// cancelled. To handle errors in any other way, manually call [`join_next`]
    /// in a loop.
    ///
    /// # Cancel Safety
    ///
    /// This method is not cancel safe as it calls `join_next` in a loop. If you need
    /// cancel safety, manually call `join_next` in a loop with `Vec` accumulator.
    ///
    /// [`join_next`]: fn@Self::join_next
    /// [`JoinError::id`]: fn@tokio::task::JoinError::id
    pub async fn join_all(mut self) -> Vec<T> {
        let mut output = Vec::with_capacity(self.len());

        while let Some(res) = self.join_next().await {
            match res {
                Ok(t) => output.push(t),
                Err(err) if err.is_panic() => std::panic::resume_unwind(err.into_panic()),
                Err(err) => panic!("{err}"),
            }
        }
        output
    }

    /// Aborts all tasks on this [`JoinQueue`].
    ///
    /// This does not remove the tasks from the [`JoinQueue`]. To wait for the tasks to complete
    /// cancellation, you should call `join_next` in a loop until the [`JoinQueue`] is empty.
    pub fn abort_all(&mut self) {
        self.0.iter().for_each(|jh| jh.abort());
    }

    /// Removes all tasks from this [`JoinQueue`] without aborting them.
    ///
    /// The tasks removed by this call will continue to run in the background even if the [`JoinQueue`]
    /// is dropped.
    pub fn detach_all(&mut self) {
        self.0.drain(..).for_each(|jh| drop(jh.detach()));
    }

    /// Polls for the next task in [`JoinQueue`] to complete.
    ///
    /// If this returns `Poll::Ready(Some(_))`, then the task that completed is removed from the queue.
    ///
    /// When the method returns `Poll::Pending`, the `Waker` in the provided `Context` is scheduled
    /// to receive a wakeup when a task in the [`JoinQueue`] completes. Note that on multiple calls to
    /// `poll_join_next`, only the `Waker` from the `Context` passed to the most recent call is
    /// scheduled to receive a wakeup.
    ///
    /// # Returns
    ///
    /// This function returns:
    ///
    ///  * `Poll::Pending` if the [`JoinQueue`] is not empty but there is no task whose output is
    ///    available right now.
    ///  * `Poll::Ready(Some(Ok(value)))` if the next task in this [`JoinQueue`] has completed.
    ///    The `value` is the return value that task.
    ///  * `Poll::Ready(Some(Err(err)))` if the next task in this [`JoinQueue`] has panicked or been
    ///    aborted. The `err` is the `JoinError` from the panicked/aborted task.
    ///  * `Poll::Ready(None)` if the [`JoinQueue`] is empty.
    pub fn poll_join_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<T, JoinError>>> {
        let jh = match self.0.front_mut() {
            None => return Poll::Ready(None),
            Some(jh) => jh,
        };
        if let Poll::Ready(res) = Pin::new(jh).poll(cx) {
            // Use `detach` to avoid calling `abort` on a task that has already completed.
            // Dropping `AbortOnDropHandle` would abort the task, but since it is finished,
            // we only need to drop the `JoinHandle` for cleanup.
            drop(self.0.pop_front().unwrap().detach());
            Poll::Ready(Some(res))
        } else {
            Poll::Pending
        }
    }

    /// Polls for the next task in [`JoinQueue`] to complete.
    ///
    /// If this returns `Poll::Ready(Some(_))`, then the task that completed is removed from the queue.
    ///
    /// When the method returns `Poll::Pending`, the `Waker` in the provided `Context` is scheduled
    /// to receive a wakeup when a task in the [`JoinQueue`] completes. Note that on multiple calls to
    /// `poll_join_next`, only the `Waker` from the `Context` passed to the most recent call is
    /// scheduled to receive a wakeup.
    ///
    /// # Returns
    ///
    /// This function returns:
    ///
    ///  * `Poll::Pending` if the [`JoinQueue`] is not empty but there is no task whose output is
    ///    available right now.
    ///  * `Poll::Ready(Some(Ok((id, value))))` if the next task in this [`JoinQueue`] has completed.
    ///    The `value` is the return value that task, and `id` is its [task ID].
    ///  * `Poll::Ready(Some(Err(err)))` if the next task in this [`JoinQueue`] has panicked or been
    ///    aborted. The `err` is the `JoinError` from the panicked/aborted task.
    ///  * `Poll::Ready(None)` if the [`JoinQueue`] is empty.
    ///
    /// [task ID]: tokio::task::Id
    pub fn poll_join_next_with_id(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<(Id, T), JoinError>>> {
        let jh = match self.0.front_mut() {
            None => return Poll::Ready(None),
            Some(jh) => jh,
        };
        if let Poll::Ready(res) = Pin::new(jh).poll(cx) {
            // Use `detach` to avoid calling `abort` on a task that has already completed.
            // Dropping `AbortOnDropHandle` would abort the task, but since it is finished,
            // we only need to drop the `JoinHandle` for cleanup.
            let jh = self.0.pop_front().unwrap().detach();
            let id = jh.id();
            drop(jh);
            // If the task succeeded, add the task ID to the output. Otherwise, the
            // `JoinError` will already have the task's ID.
            Poll::Ready(Some(res.map(|output| (id, output))))
        } else {
            Poll::Pending
        }
    }
}

impl<T> std::fmt::Debug for JoinQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(|jh| JoinHandle::id(jh.as_ref())))
            .finish()
    }
}

impl<T> Default for JoinQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Collect an iterator of futures into a [`JoinQueue`].
///
/// This is equivalent to calling [`JoinQueue::spawn`] on each element of the iterator.
impl<T, F> std::iter::FromIterator<F> for JoinQueue<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    fn from_iter<I: IntoIterator<Item = F>>(iter: I) -> Self {
        let mut set = Self::new();
        iter.into_iter().for_each(|task| {
            set.spawn(task);
        });
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple type that does not implement [`std::fmt::Debug`].
    struct NotDebug;

    fn is_debug<T: std::fmt::Debug>() {}

    #[test]
    fn assert_debug() {
        is_debug::<JoinQueue<NotDebug>>();
    }
}
