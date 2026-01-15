use crate::{Priority, RunnableMeta, Scheduler, SessionId, Timer};
use std::{
    future::Future,
    marker::PhantomData,
    mem::ManuallyDrop,
    panic::Location,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll},
    thread::{self, ThreadId},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct ForegroundExecutor {
    session_id: SessionId,
    scheduler: Arc<dyn Scheduler>,
    closed: Arc<AtomicBool>,
    not_send: PhantomData<Rc<()>>,
}

impl ForegroundExecutor {
    pub fn new(session_id: SessionId, scheduler: Arc<dyn Scheduler>) -> Self {
        Self {
            session_id,
            scheduler,
            closed: Arc::new(AtomicBool::new(false)),
            not_send: PhantomData,
        }
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn scheduler(&self) -> &Arc<dyn Scheduler> {
        &self.scheduler
    }

    /// Returns the closed flag for this executor.
    pub fn closed(&self) -> &Arc<AtomicBool> {
        &self.closed
    }

    /// Close this executor. Tasks will not run after this is called.
    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
    }

    #[track_caller]
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let session_id = self.session_id;
        let scheduler = Arc::clone(&self.scheduler);
        let location = Location::caller();
        let closed = self.closed.clone();
        let (runnable, task) = spawn_local_with_source_location(
            future,
            move |runnable| {
                scheduler.schedule_foreground(session_id, runnable);
            },
            RunnableMeta { location, closed },
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

    pub fn timer(&self, duration: Duration) -> Timer {
        self.scheduler.timer(duration)
    }

    pub fn now(&self) -> Instant {
        self.scheduler.clock().now()
    }
}

#[derive(Clone)]
pub struct BackgroundExecutor {
    scheduler: Arc<dyn Scheduler>,
    closed: Arc<AtomicBool>,
}

impl BackgroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Self {
        Self {
            scheduler,
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Returns the closed flag for this executor.
    pub fn closed(&self) -> &Arc<AtomicBool> {
        &self.closed
    }

    /// Close this executor. Tasks will not run after this is called.
    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
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
        let scheduler = Arc::clone(&self.scheduler);
        let location = Location::caller();
        let closed = self.closed.clone();
        let (runnable, task) = async_task::Builder::new()
            .metadata(RunnableMeta { location, closed })
            .spawn(
                move |_| future,
                move |runnable| {
                    scheduler.schedule_background_with_priority(runnable, priority);
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
        let closed = self.closed.clone();
        let (tx, rx) = flume::bounded::<async_task::Runnable<RunnableMeta>>(1);

        self.scheduler.spawn_realtime(Box::new(move || {
            while let Ok(runnable) = rx.recv() {
                if runnable.metadata().is_closed() {
                    continue;
                }
                runnable.run();
            }
        }));

        let (runnable, task) = async_task::Builder::new()
            .metadata(RunnableMeta { location, closed })
            .spawn(
                move |_| future,
                move |runnable| {
                    let _ = tx.send(runnable);
                },
            );
        runnable.schedule();
        Task(TaskState::Spawned(task))
    }

    pub fn timer(&self, duration: Duration) -> Timer {
        self.scheduler.timer(duration)
    }

    pub fn now(&self) -> Instant {
        self.scheduler.clock().now()
    }

    pub fn scheduler(&self) -> &Arc<dyn Scheduler> {
        &self.scheduler
    }
}

/// Task is a primitive that allows work to happen in the background.
///
/// It implements [`Future`] so you can `.await` on it.
///
/// If you drop a task it will be cancelled immediately. Calling [`Task::detach`] allows
/// the task to continue running, but with no way to return a value.
#[must_use]
#[derive(Debug)]
pub struct Task<T>(TaskState<T>);

#[derive(Debug)]
enum TaskState<T> {
    /// A task that is ready to return a value
    Ready(Option<T>),

    /// A task that is currently running.
    Spawned(async_task::Task<T, RunnableMeta>),
}

impl<T> Task<T> {
    /// Creates a new task that will resolve with the value
    pub fn ready(val: T) -> Self {
        Task(TaskState::Ready(Some(val)))
    }

    /// Creates a Task from an async_task::Task
    pub fn from_async_task(task: async_task::Task<T, RunnableMeta>) -> Self {
        Task(TaskState::Spawned(task))
    }

    pub fn is_ready(&self) -> bool {
        match &self.0 {
            TaskState::Ready(_) => true,
            TaskState::Spawned(task) => task.is_finished(),
        }
    }

    /// Detaching a task runs it to completion in the background
    pub fn detach(self) {
        match self {
            Task(TaskState::Ready(_)) => {}
            Task(TaskState::Spawned(task)) => task.detach(),
        }
    }

    /// Converts this task into a fallible task that returns `Option<T>`.
    pub fn fallible(self) -> FallibleTask<T> {
        FallibleTask(match self.0 {
            TaskState::Ready(val) => FallibleTaskState::Ready(val),
            TaskState::Spawned(task) => FallibleTaskState::Spawned(task.fallible()),
        })
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
        }
    }
}

impl<T> Future for FallibleTask<T> {
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match unsafe { self.get_unchecked_mut() } {
            FallibleTask(FallibleTaskState::Ready(val)) => Poll::Ready(val.take()),
            FallibleTask(FallibleTaskState::Spawned(task)) => Pin::new(task).poll(cx),
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
        }
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match unsafe { self.get_unchecked_mut() } {
            Task(TaskState::Ready(val)) => Poll::Ready(val.take().unwrap()),
            Task(TaskState::Spawned(task)) => Pin::new(task).poll(cx),
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
            assert!(
                self.id == thread_id(),
                "local task dropped by a thread that didn't spawn it. Task spawned at {}",
                self.location
            );
            unsafe {
                ManuallyDrop::drop(&mut self.inner);
            }
        }
    }

    impl<F: Future> Future for Checked<F> {
        type Output = F::Output;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            assert!(
                self.id == thread_id(),
                "local task polled by a thread that didn't spawn it. Task spawned at {}",
                self.location
            );
            unsafe { self.map_unchecked_mut(|c| &mut *c.inner).poll(cx) }
        }
    }

    let location = metadata.location;

    unsafe {
        async_task::Builder::new()
            .metadata(metadata)
            .spawn_unchecked(
                move |_| Checked {
                    id: thread_id(),
                    inner: ManuallyDrop::new(future),
                    location,
                },
                schedule,
            )
    }
}
