use crate::{Instant, Priority, RunnableMeta, Scheduler, SessionId, Timer};
use async_task::Runnable;
use std::{
    any::Any,
    future::Future,
    marker::PhantomData,
    mem::ManuallyDrop,
    panic::Location,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Waker},
    thread::{self, ThreadId},
    time::Duration,
};

/// A `!Send` executor pinned to a single session. Tasks spawned on it run in
/// order on whichever thread drains the dispatch destination supplied at
/// construction time — typically the main thread for the default session, or
/// a dedicated OS thread for sessions created by `spawn_dedicated_thread`.
#[derive(Clone)]
pub struct LocalExecutor {
    session_id: SessionId,
    scheduler: Arc<dyn Scheduler>,
    // Spawned tasks' schedule callbacks each hold an `Arc` clone of this
    // closure, so the destination it captures stays alive as long as work
    // could still land on it.
    dispatch: Arc<dyn Fn(Runnable<RunnableMeta>) + Send + Sync>,
    not_send: PhantomData<Rc<()>>,
}

impl LocalExecutor {
    /// Constructs a local executor that runs spawned tasks by sending their
    /// runnables through `dispatch`. The `scheduler` is retained for access to
    /// clocks, timers, and other scheduler-level services.
    ///
    /// For the common case of routing runnables through
    /// `Scheduler::schedule_local`, callers pass a closure that does exactly
    /// that. `spawn_dedicated_thread` instead passes a closure that sends to
    /// the dedicated thread's channel.
    pub fn new(
        session_id: SessionId,
        scheduler: Arc<dyn Scheduler>,
        dispatch: impl Fn(Runnable<RunnableMeta>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            session_id,
            scheduler,
            dispatch: Arc::new(dispatch),
            not_send: PhantomData,
        }
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn scheduler(&self) -> &Arc<dyn Scheduler> {
        &self.scheduler
    }

    #[track_caller]
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let dispatch = self.dispatch.clone();
        let location = Location::caller();
        let (runnable, task) = spawn_local_with_source_location(
            future,
            move |runnable| dispatch(runnable),
            RunnableMeta {
                location,
                spawned: crate::SpawnTime(Instant::now()),
            },
        );
        runnable.schedule();
        Task(TaskState::Spawned(task))
    }

    /// Like [`Self::spawn`], but routes the task's runnables through the
    /// given dispatch destination instead of this executor's own. The
    /// destination must deliver runnables to this executor's thread: the
    /// future is polled and dropped on the spawning thread.
    #[track_caller]
    pub fn spawn_with_dispatch<F>(
        &self,
        future: F,
        dispatch: impl Fn(Runnable<RunnableMeta>) + Send + Sync + 'static,
    ) -> Task<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let location = Location::caller();
        let (runnable, task) = spawn_local_with_source_location(
            future,
            dispatch,
            RunnableMeta {
                location,
                spawned: crate::SpawnTime(Instant::now()),
            },
        );
        runnable.schedule();
        Task(TaskState::Spawned(task))
    }

    pub fn block_on<Fut: Future>(&self, future: Fut) -> Fut::Output {
        use std::cell::Cell;

        let output = Cell::new(None);
        let future = async {
            output.set(Some(future.await));
        };
        let mut future = std::pin::pin!(future);

        self.scheduler
            .block(Some(self.session_id), future.as_mut(), None);

        output.take().expect("block_on future did not complete")
    }

    /// Block until the future completes or timeout occurs.
    /// Returns Ok(output) if completed, Err(future) if timed out.
    pub fn block_with_timeout<Fut: Future>(
        &self,
        timeout: Duration,
        future: Fut,
    ) -> Result<Fut::Output, impl Future<Output = Fut::Output> + use<Fut>> {
        use std::cell::Cell;

        let output = Cell::new(None);
        let mut future = Box::pin(future);

        {
            let future_ref = &mut future;
            let wrapper = async {
                output.set(Some(future_ref.await));
            };
            let mut wrapper = std::pin::pin!(wrapper);

            self.scheduler
                .block(Some(self.session_id), wrapper.as_mut(), Some(timeout));
        }

        match output.take() {
            Some(value) => Ok(value),
            None => Err(future),
        }
    }

    #[track_caller]
    pub fn timer(&self, duration: Duration) -> Timer {
        self.scheduler.timer(duration)
    }

    pub fn now(&self) -> Instant {
        self.scheduler.clock().now()
    }

    /// Spawn a closure on a fresh session pinned to its own [`LocalExecutor`].
    /// The closure runs on a new OS thread under `PlatformScheduler`, or on
    /// the test scheduler's loop under `TestScheduler`.
    ///
    /// The returned `Task` represents the dedicated work: dropping it cancels
    /// the dedicated closure, `.await`ing it yields the closure's return
    /// value, `.detach()`ing it lets the dedicated work run independently of
    /// the caller.
    #[track_caller]
    pub fn spawn_dedicated<F, Fut>(&self, f: F) -> Task<Fut::Output>
    where
        F: FnOnce(LocalExecutor) -> Fut + Send + 'static,
        Fut: Future + 'static,
        Fut::Output: Send + Sync + 'static,
    {
        self.scheduler
            .clone()
            .spawn_dedicated(box_dedicated(f))
            .downcast::<Fut::Output>()
    }
}

/// Boxes the user-supplied dedicated closure into the type-erased shape
/// expected by [`Scheduler::spawn_dedicated`]. The user's `Fut::Output` is
/// boxed as `Box<dyn Any + Send + Sync>` on the dedicated side and downcast
/// back to `Fut::Output` by [`Task::downcast`] in the wrapper.
fn box_dedicated<F, Fut>(
    f: F,
) -> Box<
    dyn FnOnce(LocalExecutor) -> Pin<Box<dyn Future<Output = Box<dyn Any + Send + Sync>> + 'static>>
        + Send
        + 'static,
>
where
    F: FnOnce(LocalExecutor) -> Fut + Send + 'static,
    Fut: Future + 'static,
    Fut::Output: Send + Sync + 'static,
{
    Box::new(move |executor| {
        Box::pin(async move { Box::new(f(executor).await) as Box<dyn Any + Send + Sync> })
    })
}

#[derive(Clone)]
pub struct BackgroundExecutor {
    scheduler: Arc<dyn Scheduler>,
}

impl BackgroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Self {
        Self { scheduler }
    }

    #[track_caller]
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.spawn_with_priority(Priority::default(), future)
    }

    #[track_caller]
    pub fn spawn_with_priority<F>(&self, priority: Priority, future: F) -> Task<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let scheduler = Arc::downgrade(&self.scheduler);
        let location = Location::caller();
        let (runnable, task) = async_task::Builder::new()
            .metadata(RunnableMeta {
                location,
                spawned: crate::SpawnTime(Instant::now()),
            })
            .spawn(
                move |_| future,
                move |runnable| {
                    if let Some(scheduler) = scheduler.upgrade() {
                        scheduler.schedule_background_with_priority(runnable, priority);
                    }
                },
            );
        runnable.schedule();
        Task(TaskState::Spawned(task))
    }

    /// Spawns a future on a dedicated realtime thread for audio processing.
    #[track_caller]
    pub fn spawn_realtime<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let location = Location::caller();
        let (tx, rx) = flume::bounded::<async_task::Runnable<RunnableMeta>>(1);

        self.scheduler.spawn_realtime(Box::new(move || {
            while let Ok(runnable) = rx.recv() {
                runnable.run();
            }
        }));

        let (runnable, task) = async_task::Builder::new()
            .metadata(RunnableMeta {
                location,
                spawned: crate::SpawnTime(Instant::now()),
            })
            .spawn(
                move |_| future,
                move |runnable| {
                    let _ = tx.send(runnable);
                },
            );
        runnable.schedule();
        Task(TaskState::Spawned(task))
    }

    #[track_caller]
    pub fn timer(&self, duration: Duration) -> Timer {
        self.scheduler.timer(duration)
    }

    pub fn now(&self) -> Instant {
        self.scheduler.clock().now()
    }

    pub fn scheduler(&self) -> &Arc<dyn Scheduler> {
        &self.scheduler
    }

    /// Spawn a closure on a fresh session pinned to its own [`LocalExecutor`].
    /// The closure runs on a new OS thread under `PlatformScheduler`, or on
    /// the test scheduler's loop under `TestScheduler`.
    ///
    /// The returned `Task` represents the dedicated work: dropping it cancels
    /// the dedicated closure, `.await`ing it yields the closure's return
    /// value, `.detach()`ing it lets the dedicated work run independently of
    /// the caller.
    #[track_caller]
    pub fn spawn_dedicated<F, Fut>(&self, f: F) -> Task<Fut::Output>
    where
        F: FnOnce(LocalExecutor) -> Fut + Send + 'static,
        Fut: Future + 'static,
        Fut::Output: Send + Sync + 'static,
    {
        self.scheduler
            .clone()
            .spawn_dedicated(box_dedicated(f))
            .downcast::<Fut::Output>()
    }
}

/// A long-lived handle to one dedicated session: every future spawned on it
/// runs on that session's single thread, never on the background pool.
///
/// Use this when tasks rely on thread-local state being coherent across
/// spawns, or when the per-thread setup cost of the work is high enough that
/// it should be paid once. Creating the session costs a thread (on the web, a
/// worker — expensive); each spawn afterwards is just a channel send.
///
/// Dropping the handle cancels the session: queued and future spawns resolve
/// to cancelled tasks.
pub struct DedicatedExecutor {
    sender: flume::Sender<Runnable<RunnableMeta>>,
    _session: Task<()>,
}

impl DedicatedExecutor {
    /// Starts a dedicated session on `executor` and returns a handle that
    /// spawns futures onto it. Under `TestScheduler` the session runs on the
    /// deterministic test loop; no real thread is created.
    #[track_caller]
    pub fn new(executor: &BackgroundExecutor) -> Self {
        let (sender, receiver) = flume::unbounded::<Runnable<RunnableMeta>>();
        let session = executor.spawn_dedicated(move |_executor| async move {
            while let Ok(runnable) = receiver.recv_async().await {
                runnable.run();
            }
        });
        Self {
            sender,
            _session: session,
        }
    }

    /// Spawns a future onto the dedicated session's thread.
    ///
    /// The returned task has the usual semantics: dropping it cancels the
    /// future, `.await`ing it yields the output, `.detach()`ing it lets it
    /// run to completion on its own.
    #[track_caller]
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let sender = self.sender.clone();
        let (runnable, task) = async_task::Builder::new()
            .metadata(RunnableMeta::new_with_callers_location())
            .spawn(
                move |_| future,
                move |runnable| {
                    let _ = sender.send(runnable);
                },
            );
        runnable.schedule();
        Task(TaskState::Spawned(task))
    }
}

/// Task is a primitive that allows work to happen in the background.
///
/// It implements [`Future`] so you can `.await` on it.
///
/// If you drop a task it will be cancelled immediately. Calling [`Task::detach`] allows
/// the task to continue running, but with no way to return a value.
#[must_use]
pub struct Task<T>(TaskState<T>);

enum TaskState<T> {
    /// A task that is ready to return a value
    Ready(Option<T>),

    /// A task that is currently running.
    Spawned(async_task::Task<T, RunnableMeta>),

    /// A typed view of a [`Task<Box<dyn Any + Send + Sync>>`] obtained via
    /// [`Task::downcast`]. The inner task drives the actual work; the
    /// downcast layer just unwraps the `Box<dyn Any + Send + Sync>` on poll.
    Downcast {
        inner: Box<Task<Box<dyn Any + Send + Sync>>>,
        marker: PhantomData<fn() -> T>,
    },

    /// A task whose real handle is delivered later by another thread (see
    /// [`Task::rendezvous`]). Once delivered, polling replaces this state
    /// with the delivered task's state.
    Rendezvous(RendezvousReceiver<T>),
}

/// State shared between the two halves of a [`Task::rendezvous`] pair.
enum RendezvousState<T> {
    /// No task delivered yet; holds the consumer's waker if it polled.
    Pending(Option<Waker>),
    /// The producer delivered before the consumer consumed the task.
    Delivered(Task<T>),
    /// The consumer was dropped before delivery; delivery cancels the task.
    Cancelled,
    /// The consumer was detached before delivery; delivery detaches the task.
    Detached,
    /// The consumer took the delivered task.
    Taken,
}

pub(crate) struct RendezvousReceiver<T> {
    shared: Arc<parking_lot::Mutex<RendezvousState<T>>>,
}

impl<T> RendezvousReceiver<T> {
    fn poll_take(&self, cx: &mut Context) -> Poll<Task<T>> {
        let mut state = self.shared.lock();
        match std::mem::replace(&mut *state, RendezvousState::Taken) {
            RendezvousState::Delivered(task) => Poll::Ready(task),
            RendezvousState::Pending(_) => {
                *state = RendezvousState::Pending(Some(cx.waker().clone()));
                Poll::Pending
            }
            RendezvousState::Cancelled | RendezvousState::Detached | RendezvousState::Taken => {
                unreachable!("a rendezvous task was polled after its receiver was consumed")
            }
        }
    }

    fn is_ready(&self) -> bool {
        match &*self.shared.lock() {
            RendezvousState::Delivered(task) => task.is_ready(),
            _ => false,
        }
    }

    fn detach(self) {
        let mut state = self.shared.lock();
        match std::mem::replace(&mut *state, RendezvousState::Detached) {
            RendezvousState::Delivered(task) => {
                *state = RendezvousState::Taken;
                drop(state);
                task.detach();
            }
            // The producer applies the detached disposition on delivery.
            RendezvousState::Pending(_) => {}
            RendezvousState::Cancelled | RendezvousState::Detached | RendezvousState::Taken => {
                unreachable!("a rendezvous task was detached after its receiver was consumed")
            }
        }
    }
}

impl<T> Drop for RendezvousReceiver<T> {
    fn drop(&mut self) {
        let mut state = self.shared.lock();
        match &*state {
            RendezvousState::Pending(_) => *state = RendezvousState::Cancelled,
            RendezvousState::Delivered(_) => {
                let delivered = std::mem::replace(&mut *state, RendezvousState::Cancelled);
                drop(state);
                drop(delivered);
            }
            // `Taken`, `Detached`, and `Cancelled` record dispositions that
            // this drop must not overwrite: the receiver is also dropped as a
            // normal side effect of taking or detaching.
            _ => {}
        }
    }
}

pub(crate) struct RendezvousSender<T> {
    shared: Arc<parking_lot::Mutex<RendezvousState<T>>>,
}

impl<T> RendezvousSender<T> {
    /// Hands the real task to the rendezvous, applying the consumer's
    /// disposition if it was dropped or detached before delivery.
    pub(crate) fn deliver(self, task: Task<T>) {
        let mut state = self.shared.lock();
        match std::mem::replace(&mut *state, RendezvousState::Delivered(task)) {
            RendezvousState::Pending(waker) => {
                drop(state);
                if let Some(waker) = waker {
                    waker.wake();
                }
            }
            RendezvousState::Cancelled => {
                let delivered = std::mem::replace(&mut *state, RendezvousState::Cancelled);
                drop(state);
                drop(delivered);
            }
            RendezvousState::Detached => {
                let RendezvousState::Delivered(task) =
                    std::mem::replace(&mut *state, RendezvousState::Detached)
                else {
                    unreachable!("the delivered task was just stored");
                };
                drop(state);
                task.detach();
            }
            RendezvousState::Delivered(_) | RendezvousState::Taken => {
                unreachable!("a rendezvous task was delivered twice")
            }
        }
    }
}

impl<T> Task<T> {
    /// Creates a new task that will resolve with the value
    pub fn ready(val: T) -> Self {
        Task(TaskState::Ready(Some(val)))
    }

    /// Creates a task whose real handle arrives later through the returned
    /// sender, typically from another thread. Until delivery the task is
    /// pending; afterwards it behaves exactly like the delivered task.
    /// Dropping or detaching the task before delivery is honored on delivery.
    pub(crate) fn rendezvous() -> (Self, RendezvousSender<T>) {
        let shared = Arc::new(parking_lot::Mutex::new(RendezvousState::Pending(None)));
        (
            Task(TaskState::Rendezvous(RendezvousReceiver {
                shared: shared.clone(),
            })),
            RendezvousSender { shared },
        )
    }

    /// Creates a Task from an async_task::Task
    pub fn from_async_task(task: async_task::Task<T, RunnableMeta>) -> Self {
        Task(TaskState::Spawned(task))
    }

    pub fn is_ready(&self) -> bool {
        match &self.0 {
            TaskState::Ready(_) => true,
            TaskState::Spawned(task) => task.is_finished(),
            TaskState::Downcast { inner, .. } => inner.is_ready(),
            TaskState::Rendezvous(receiver) => receiver.is_ready(),
        }
    }

    /// Detaching a task runs it to completion in the background
    pub fn detach(self) {
        match self {
            Task(TaskState::Ready(_)) => {}
            Task(TaskState::Spawned(task)) => task.detach(),
            Task(TaskState::Downcast { inner, .. }) => inner.detach(),
            Task(TaskState::Rendezvous(receiver)) => receiver.detach(),
        }
    }

    /// Converts this task into a fallible task that returns `Option<T>`.
    pub fn fallible(self) -> FallibleTask<T> {
        FallibleTask(match self.0 {
            TaskState::Ready(val) => FallibleTaskState::Ready(val),
            TaskState::Spawned(task) => FallibleTaskState::Spawned(task.fallible()),
            TaskState::Downcast { inner, .. } => FallibleTaskState::Downcast {
                inner: Box::new(inner.fallible()),
                marker: PhantomData,
            },
            TaskState::Rendezvous(receiver) => FallibleTaskState::Rendezvous(receiver),
        })
    }
}

impl Task<Box<dyn Any + Send + Sync>> {
    /// Reinterprets the boxed output as a concrete `T` via downcast on
    /// completion. Used by [`LocalExecutor::spawn_dedicated`] and
    /// [`BackgroundExecutor::spawn_dedicated`] to recover the user closure's
    /// `Fut::Output` from the dyn-safe [`Scheduler::spawn_dedicated`].
    ///
    /// Panics on poll if the inner output is not in fact a `T` -- a logic
    /// error in whatever produced the inner task, since the downcast type is
    /// chosen by the caller of `downcast`.
    pub fn downcast<T: Send + Sync + 'static>(self) -> Task<T> {
        Task(TaskState::Downcast {
            inner: Box::new(self),
            marker: PhantomData,
        })
    }
}

impl<T> std::fmt::Debug for Task<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            TaskState::Ready(_) => f.debug_tuple("Task::Ready").finish(),
            TaskState::Spawned(task) => f.debug_tuple("Task::Spawned").field(task).finish(),
            TaskState::Downcast { inner, .. } => {
                f.debug_tuple("Task::Downcast").field(inner).finish()
            }
            TaskState::Rendezvous(_) => f.debug_tuple("Task::Rendezvous").finish(),
        }
    }
}

/// A task that returns `Option<T>` instead of panicking when cancelled.
#[must_use]
pub struct FallibleTask<T>(FallibleTaskState<T>);

enum FallibleTaskState<T> {
    /// A task that is ready to return a value
    Ready(Option<T>),

    /// A task that is currently running (wraps async_task::FallibleTask).
    Spawned(async_task::FallibleTask<T, RunnableMeta>),

    /// Mirror of [`TaskState::Downcast`] for fallible tasks.
    Downcast {
        inner: Box<FallibleTask<Box<dyn Any + Send + Sync>>>,
        marker: PhantomData<fn() -> T>,
    },

    /// Mirror of [`TaskState::Rendezvous`] for fallible tasks.
    Rendezvous(RendezvousReceiver<T>),
}

impl<T> FallibleTask<T> {
    /// Creates a new fallible task that will resolve with the value.
    pub fn ready(val: T) -> Self {
        FallibleTask(FallibleTaskState::Ready(Some(val)))
    }

    /// Detaching a task runs it to completion in the background.
    pub fn detach(self) {
        match self.0 {
            FallibleTaskState::Ready(_) => {}
            FallibleTaskState::Spawned(task) => task.detach(),
            FallibleTaskState::Downcast { inner, .. } => inner.detach(),
            FallibleTaskState::Rendezvous(receiver) => receiver.detach(),
        }
    }
}

impl<T: 'static> Future for FallibleTask<T> {
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        loop {
            match &mut this.0 {
                FallibleTaskState::Ready(val) => return Poll::Ready(val.take()),
                FallibleTaskState::Spawned(task) => return Pin::new(task).poll(cx),
                FallibleTaskState::Downcast { inner, .. } => {
                    return match Pin::new(inner.as_mut()).poll(cx) {
                        Poll::Ready(Some(boxed_any)) => Poll::Ready(Some(
                            *boxed_any
                                .downcast::<T>()
                                .expect("FallibleTask::poll: downcast type mismatch"),
                        )),
                        Poll::Ready(None) => Poll::Ready(None),
                        Poll::Pending => Poll::Pending,
                    };
                }
                FallibleTaskState::Rendezvous(receiver) => match receiver.poll_take(cx) {
                    Poll::Ready(task) => {
                        this.0 = task.fallible().0;
                        continue;
                    }
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }
}

impl<T> std::fmt::Debug for FallibleTask<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            FallibleTaskState::Ready(_) => f.debug_tuple("FallibleTask::Ready").finish(),
            FallibleTaskState::Spawned(task) => {
                f.debug_tuple("FallibleTask::Spawned").field(task).finish()
            }
            FallibleTaskState::Downcast { inner, .. } => f
                .debug_tuple("FallibleTask::Downcast")
                .field(inner)
                .finish(),
            FallibleTaskState::Rendezvous(_) => f.debug_tuple("FallibleTask::Rendezvous").finish(),
        }
    }
}

impl<T: 'static> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        loop {
            match &mut this.0 {
                TaskState::Ready(val) => return Poll::Ready(val.take().unwrap()),
                TaskState::Spawned(task) => return Pin::new(task).poll(cx),
                TaskState::Downcast { inner, .. } => {
                    return match Pin::new(inner.as_mut()).poll(cx) {
                        Poll::Ready(boxed_any) => Poll::Ready(
                            *boxed_any
                                .downcast::<T>()
                                .expect("Task::poll: downcast type mismatch"),
                        ),
                        Poll::Pending => Poll::Pending,
                    };
                }
                TaskState::Rendezvous(receiver) => match receiver.poll_take(cx) {
                    Poll::Ready(task) => {
                        this.0 = task.0;
                        continue;
                    }
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }
}

/// Variant of `async_task::spawn_local` that includes the source location of the spawn in panics.
#[track_caller]
fn spawn_local_with_source_location<Fut, S>(
    future: Fut,
    schedule: S,
    metadata: RunnableMeta,
) -> (
    async_task::Runnable<RunnableMeta>,
    async_task::Task<Fut::Output, RunnableMeta>,
)
where
    Fut: Future + 'static,
    Fut::Output: 'static,
    S: async_task::Schedule<RunnableMeta> + Send + Sync + 'static,
{
    #[inline]
    fn thread_id() -> ThreadId {
        std::thread_local! {
            static ID: ThreadId = thread::current().id();
        }
        ID.try_with(|id| *id)
            .unwrap_or_else(|_| thread::current().id())
    }

    struct Checked<F> {
        id: ThreadId,
        inner: ManuallyDrop<F>,
        location: &'static Location<'static>,
    }

    impl<F> Drop for Checked<F> {
        fn drop(&mut self) {
            assert_eq!(
                self.id,
                thread_id(),
                "local task dropped by a thread that didn't spawn it. Task spawned at {}",
                self.location
            );
            // SAFETY: `inner` is wrapped in `ManuallyDrop`, so this is the only
            // place it is dropped. The thread check above ensures local futures
            // are dropped on the thread that created them.
            unsafe { ManuallyDrop::drop(&mut self.inner) };
        }
    }

    impl<F: Future> Future for Checked<F> {
        type Output = F::Output;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // SAFETY: We don't move any fields out of `self`; this mutable
            // reference is only used to check metadata and to project the pin to
            // `inner` below.
            let this = unsafe { self.get_unchecked_mut() };
            assert!(
                this.id == thread_id(),
                "local task polled by a thread that didn't spawn it. Task spawned at {}",
                this.location
            );
            // SAFETY: `inner` is structurally pinned by `Checked`; after
            // `Checked` is pinned, `inner` is never moved. The thread check
            // above ensures the local future is only polled by its spawning
            // thread.
            unsafe { Pin::new_unchecked(&mut *this.inner).poll(cx) }
        }
    }

    let location = metadata.location;

    let future = move |_| Checked {
        id: thread_id(),
        inner: ManuallyDrop::new(future),
        location,
    };

    let builder = async_task::Builder::new().metadata(metadata);
    // SAFETY: `Checked` enforces the invariants required by `spawn_unchecked`:
    // the non-`Send` future is only polled and dropped on the thread that
    // spawned it.
    unsafe { builder.spawn_unchecked(future, schedule) }
}
