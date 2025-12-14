use crate::{profiler, App, PlatformDispatcher, PlatformScheduler, RunnableMeta, TaskTiming};
use futures::channel::mpsc;
use scheduler::Scheduler;
use smol::prelude::*;
use std::{
    fmt::Debug,
    future::Future,
    marker::PhantomData,
    mem,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};
use util::TryFutureExt;

pub use scheduler::{Priority, RealtimePriority};

/// A pointer to the executor that is currently running,
/// for spawning background tasks.
#[derive(Clone)]
pub struct BackgroundExecutor {
    scheduler: Arc<dyn Scheduler>,
    dispatcher: Arc<dyn PlatformDispatcher>,
}

/// A pointer to the executor that is currently running,
/// for spawning tasks on the main thread.
#[derive(Clone)]
pub struct ForegroundExecutor {
    inner: scheduler::ForegroundExecutor,
    dispatcher: Arc<dyn PlatformDispatcher>,
    not_send: PhantomData<Rc<()>>,
}

/// Task is a primitive that allows work to happen in the background.
///
/// It implements [`Future`] so you can `.await` on it.
///
/// If you drop a task it will be cancelled immediately. Calling [`Task::detach`] allows
/// the task to continue running, but with no way to return a value.
#[must_use]
#[derive(Debug)]
pub struct Task<T>(scheduler::Task<T>);

impl<T> Task<T> {
    /// Creates a new task that will resolve with the value.
    pub fn ready(val: T) -> Self {
        Task(scheduler::Task::ready(val))
    }

    /// Returns true if the task has completed or was created with `Task::ready`.
    pub fn is_ready(&self) -> bool {
        self.0.is_ready()
    }

    /// Detaching a task runs it to completion in the background.
    pub fn detach(self) {
        self.0.detach()
    }

    /// Wraps a scheduler::Task.
    pub fn from_scheduler(task: scheduler::Task<T>) -> Self {
        Task(task)
    }
}

impl<T, E> Task<Result<T, E>>
where
    T: 'static,
    E: 'static + Debug,
{
    /// Run the task to completion in the background and log any errors that occur.
    #[track_caller]
    pub fn detach_and_log_err(self, cx: &App) {
        let location = core::panic::Location::caller();
        cx.foreground_executor()
            .spawn(self.log_tracked_err(*location))
            .detach();
    }
}

impl<T> std::future::Future for Task<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // SAFETY: Task is a repr(transparent) wrapper around scheduler::Task,
        // and we're just projecting the pin through to the inner task.
        let inner = unsafe { self.map_unchecked_mut(|t| &mut t.0) };
        inner.poll(cx)
    }
}

impl BackgroundExecutor {
    /// Creates a new BackgroundExecutor from the given PlatformDispatcher.
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        let scheduler: Arc<dyn Scheduler> = if let Some(test_dispatcher) = dispatcher.as_test() {
            test_dispatcher.scheduler().clone()
        } else {
            Arc::new(PlatformScheduler::new(dispatcher.clone()))
        };

        #[cfg(not(any(test, feature = "test-support")))]
        let scheduler: Arc<dyn Scheduler> = Arc::new(PlatformScheduler::new(dispatcher.clone()));

        Self {
            scheduler,
            dispatcher,
        }
    }

    /// Enqueues the given future to be run to completion on a background thread.
    #[track_caller]
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.spawn_with_priority(Priority::default(), future)
    }

    /// Enqueues the given future to be run to completion on a background thread with the given priority.
    ///
    /// For `Priority::Realtime`, the task runs on a dedicated OS thread with elevated priority
    /// (suitable for audio workloads). This spawns a new thread that persists for the lifetime
    /// of the task, using a channel to send runnables to the thread.
    ///
    /// **Note**: `Priority::Realtime` will panic in tests because real OS threads break test
    /// determinism. Use `Priority::High` in tests instead.
    #[track_caller]
    pub fn spawn_with_priority<R>(
        &self,
        priority: Priority,
        future: impl Future<Output = R> + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
    {
        if let Priority::Realtime(realtime) = priority {
            // Realtime tasks run on dedicated OS threads with elevated priority.
            // We create a channel to send runnables to the dedicated thread.
            let dispatcher = self.dispatcher.clone();
            let location = std::panic::Location::caller();
            let (tx, rx) = flume::bounded::<async_task::Runnable<RunnableMeta>>(1);

            dispatcher.spawn_realtime(
                realtime,
                Box::new(move || {
                    while let Ok(runnable) = rx.recv() {
                        let start = Instant::now();
                        let location = runnable.metadata().location;
                        let mut timing = TaskTiming {
                            location,
                            start,
                            end: None,
                        };
                        profiler::add_task_timing(timing);

                        runnable.run();

                        let end = Instant::now();
                        timing.end = Some(end);
                        profiler::add_task_timing(timing);
                    }
                }),
            );

            let (runnable, task) = async_task::Builder::new()
                .metadata(RunnableMeta { location })
                .spawn(
                    move |_| future,
                    move |runnable| {
                        let _ = tx.send(runnable);
                    },
                );
            runnable.schedule();
            Task::from_scheduler(scheduler::Task::from_async_task(task))
        } else {
            let inner = scheduler::BackgroundExecutor::new(self.scheduler.clone());
            Task::from_scheduler(inner.spawn_with_priority(priority, future))
        }
    }

    /// Enqueues the given future to be run to completion on a background thread and blocking the current task on it.
    ///
    /// This allows to spawn background work that borrows from its scope. Note that the supplied future will run to
    /// completion before the current task is resumed, even if the current task is slated for cancellation.
    pub async fn await_on_background<R>(&self, future: impl Future<Output = R> + Send) -> R
    where
        R: Send,
    {
        use crate::RunnableMeta;
        use parking_lot::{Condvar, Mutex};

        struct NotifyOnDrop<'a>(&'a (Condvar, Mutex<bool>));

        impl Drop for NotifyOnDrop<'_> {
            fn drop(&mut self) {
                *self.0 .1.lock() = true;
                self.0 .0.notify_all();
            }
        }

        struct WaitOnDrop<'a>(&'a (Condvar, Mutex<bool>));

        impl Drop for WaitOnDrop<'_> {
            fn drop(&mut self) {
                let mut done = self.0 .1.lock();
                if !*done {
                    self.0 .0.wait(&mut done);
                }
            }
        }

        let dispatcher = self.dispatcher.clone();
        let location = core::panic::Location::caller();

        let pair = &(Condvar::new(), Mutex::new(false));
        let _wait_guard = WaitOnDrop(pair);

        let (runnable, task) = unsafe {
            async_task::Builder::new()
                .metadata(RunnableMeta { location })
                .spawn_unchecked(
                    move |_| async {
                        let _notify_guard = NotifyOnDrop(pair);
                        future.await
                    },
                    move |runnable| {
                        dispatcher.dispatch(runnable, Priority::default());
                    },
                )
        };
        runnable.schedule();
        task.await
    }

    /// Used by the test harness to run an async test in a synchronous fashion.
    #[cfg(any(test, feature = "test-support"))]
    #[track_caller]
    pub fn block_test<R>(&self, future: impl Future<Output = R>) -> R {
        use std::cell::Cell;

        let test_dispatcher = self
            .dispatcher
            .as_test()
            .expect("block_test requires a test dispatcher");
        let scheduler = test_dispatcher.scheduler();

        let output = Cell::new(None);
        let future = async {
            output.set(Some(future.await));
        };
        let mut future = std::pin::pin!(future);

        scheduler.block(Some(test_dispatcher.session_id()), future.as_mut(), None);

        output.take().expect("block_test future did not complete")
    }

    /// Block the current thread until the given future resolves.
    /// Consider using `block_with_timeout` instead.
    pub fn block<R>(&self, future: impl Future<Output = R>) -> R {
        use std::cell::Cell;

        let output = Cell::new(None);
        let future = async {
            output.set(Some(future.await));
        };
        let mut future = std::pin::pin!(future);

        #[cfg(any(test, feature = "test-support"))]
        let session_id = self.dispatcher.as_test().map(|t| t.session_id());
        #[cfg(not(any(test, feature = "test-support")))]
        let session_id = None;

        self.scheduler.block(session_id, future.as_mut(), None);

        output.take().expect("block future did not complete")
    }

    /// Block the current thread until the given future resolves or the timeout elapses.
    pub fn block_with_timeout<R, Fut: Future<Output = R>>(
        &self,
        duration: Duration,
        future: Fut,
    ) -> Result<R, impl Future<Output = R> + use<R, Fut>> {
        use std::cell::Cell;

        let output = Cell::new(None);
        let mut future = Box::pin(future);

        {
            let future_ref = &mut future;
            let wrapper = async {
                output.set(Some(future_ref.await));
            };
            let mut wrapper = std::pin::pin!(wrapper);

            #[cfg(any(test, feature = "test-support"))]
            let session_id = self.dispatcher.as_test().map(|t| t.session_id());
            #[cfg(not(any(test, feature = "test-support")))]
            let session_id = None;

            self.scheduler
                .block(session_id, wrapper.as_mut(), Some(duration));
        }

        match output.take() {
            Some(value) => Ok(value),
            None => Err(future),
        }
    }

    /// Scoped lets you start a number of tasks and waits
    /// for all of them to complete before returning.
    pub async fn scoped<'scope, F>(&self, scheduler: F)
    where
        F: FnOnce(&mut Scope<'scope>),
    {
        let mut scope = Scope::new(self.clone(), Priority::default());
        (scheduler)(&mut scope);
        let spawned = mem::take(&mut scope.futures)
            .into_iter()
            .map(|f| self.spawn_with_priority(scope.priority, f))
            .collect::<Vec<_>>();
        for task in spawned {
            task.await;
        }
    }

    /// Scoped lets you start a number of tasks and waits
    /// for all of them to complete before returning.
    pub async fn scoped_priority<'scope, F>(&self, priority: Priority, scheduler: F)
    where
        F: FnOnce(&mut Scope<'scope>),
    {
        let mut scope = Scope::new(self.clone(), priority);
        (scheduler)(&mut scope);
        let spawned = mem::take(&mut scope.futures)
            .into_iter()
            .map(|f| self.spawn_with_priority(scope.priority, f))
            .collect::<Vec<_>>();
        for task in spawned {
            task.await;
        }
    }

    /// Get the current time.
    ///
    /// Calling this instead of `std::time::Instant::now` allows the use
    /// of fake timers in tests.
    pub fn now(&self) -> Instant {
        self.scheduler.clock().now()
    }

    /// Returns a task that will complete after the given duration.
    /// Depending on other concurrent tasks the elapsed duration may be longer
    /// than requested.
    pub fn timer(&self, duration: Duration) -> Task<()> {
        if duration.is_zero() {
            return Task::ready(());
        }
        self.spawn(self.scheduler.timer(duration))
    }

    /// In tests, run an arbitrary number of tasks (determined by the SEED environment variable)
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_random_delay(&self) -> impl Future<Output = ()> + use<> {
        self.dispatcher.as_test().unwrap().simulate_random_delay()
    }

    /// In tests, move time forward. This does not run any tasks, but does make `timer`s ready.
    #[cfg(any(test, feature = "test-support"))]
    pub fn advance_clock(&self, duration: Duration) {
        self.dispatcher.as_test().unwrap().advance_clock(duration)
    }

    /// In tests, run one task.
    #[cfg(any(test, feature = "test-support"))]
    pub fn tick(&self) -> bool {
        self.dispatcher.as_test().unwrap().scheduler().tick()
    }

    /// In tests, run all tasks that are ready to run.
    #[cfg(any(test, feature = "test-support"))]
    pub fn run_until_parked(&self) {
        let scheduler = self.dispatcher.as_test().unwrap().scheduler();
        while scheduler.tick() {}
    }

    /// In tests, prevents `run_until_parked` from panicking if there are outstanding tasks.
    #[cfg(any(test, feature = "test-support"))]
    pub fn allow_parking(&self) {
        self.dispatcher
            .as_test()
            .unwrap()
            .scheduler()
            .allow_parking();
    }

    /// Sets the range of ticks to run before timing out in block_on.
    #[cfg(any(test, feature = "test-support"))]
    pub fn set_block_on_ticks(&self, range: std::ops::RangeInclusive<usize>) {
        self.dispatcher
            .as_test()
            .unwrap()
            .scheduler()
            .set_timeout_ticks(range);
    }

    /// Undoes the effect of [`Self::allow_parking`].
    #[cfg(any(test, feature = "test-support"))]
    pub fn forbid_parking(&self) {
        self.dispatcher
            .as_test()
            .unwrap()
            .scheduler()
            .forbid_parking();
    }

    /// In tests, returns the rng used by the dispatcher.
    #[cfg(any(test, feature = "test-support"))]
    pub fn rng(&self) -> scheduler::SharedRng {
        self.dispatcher.as_test().unwrap().scheduler().rng()
    }

    /// How many CPUs are available to the dispatcher.
    pub fn num_cpus(&self) -> usize {
        #[cfg(any(test, feature = "test-support"))]
        if self.dispatcher.as_test().is_some() {
            return 4;
        }
        num_cpus::get()
    }

    /// Whether we're on the main thread.
    pub fn is_main_thread(&self) -> bool {
        self.dispatcher.is_main_thread()
    }

    #[doc(hidden)]
    pub fn dispatcher(&self) -> &Arc<dyn PlatformDispatcher> {
        &self.dispatcher
    }
}

impl ForegroundExecutor {
    /// Creates a new ForegroundExecutor from the given PlatformDispatcher.
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        let (scheduler, session_id): (Arc<dyn Scheduler>, _) =
            if let Some(test_dispatcher) = dispatcher.as_test() {
                (
                    test_dispatcher.scheduler().clone(),
                    test_dispatcher.session_id(),
                )
            } else {
                let platform_scheduler = Arc::new(PlatformScheduler::new(dispatcher.clone()));
                let session_id = platform_scheduler.allocate_session_id();
                (platform_scheduler, session_id)
            };

        #[cfg(not(any(test, feature = "test-support")))]
        let (scheduler, session_id): (Arc<dyn Scheduler>, _) = {
            let platform_scheduler = Arc::new(PlatformScheduler::new(dispatcher.clone()));
            let session_id = platform_scheduler.allocate_session_id();
            (platform_scheduler, session_id)
        };

        let inner = scheduler::ForegroundExecutor::new(session_id, scheduler);

        Self {
            inner,
            dispatcher,
            not_send: PhantomData,
        }
    }

    /// Enqueues the given Task to run on the main thread.
    #[track_caller]
    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where
        R: 'static,
    {
        Task::from_scheduler(self.inner.spawn(future))
    }

    /// Enqueues the given Task to run on the main thread with the given priority.
    #[track_caller]
    pub fn spawn_with_priority<R>(
        &self,
        _priority: Priority,
        future: impl Future<Output = R> + 'static,
    ) -> Task<R>
    where
        R: 'static,
    {
        // Priority is ignored for foreground tasks - they run in order on the main thread
        Task::from_scheduler(self.inner.spawn(future))
    }

    #[doc(hidden)]
    pub fn dispatcher(&self) -> &Arc<dyn PlatformDispatcher> {
        &self.dispatcher
    }
}

/// Scope manages a set of tasks that are enqueued and waited on together. See [`BackgroundExecutor::scoped`].
pub struct Scope<'a> {
    executor: BackgroundExecutor,
    priority: Priority,
    futures: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    tx: Option<mpsc::Sender<()>>,
    rx: mpsc::Receiver<()>,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Scope<'a> {
    fn new(executor: BackgroundExecutor, priority: Priority) -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self {
            executor,
            priority,
            tx: Some(tx),
            rx,
            futures: Default::default(),
            lifetime: PhantomData,
        }
    }

    /// How many CPUs are available to the dispatcher.
    pub fn num_cpus(&self) -> usize {
        self.executor.num_cpus()
    }

    /// Spawn a future into this scope.
    #[track_caller]
    pub fn spawn<F>(&mut self, f: F)
    where
        F: Future<Output = ()> + Send + 'a,
    {
        let tx = self.tx.clone().unwrap();

        // SAFETY: The 'a lifetime is guaranteed to outlive any of these futures because
        // dropping this `Scope` blocks until all of the futures have resolved.
        let f = unsafe {
            mem::transmute::<
                Pin<Box<dyn Future<Output = ()> + Send + 'a>>,
                Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
            >(Box::pin(async move {
                f.await;
                drop(tx);
            }))
        };
        self.futures.push(f);
    }
}

impl Drop for Scope<'_> {
    fn drop(&mut self) {
        self.tx.take().unwrap();

        // Wait until the channel is closed, which means that all of the spawned
        // futures have resolved.
        self.executor.block(self.rx.next());
    }
}
