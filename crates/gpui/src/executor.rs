use crate::{App, PlatformDispatcher, PlatformScheduler};
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

pub use scheduler::{FallibleTask, Priority};

/// A pointer to the executor that is currently running,
/// for spawning background tasks.
#[derive(Clone)]
pub struct BackgroundExecutor {
    inner: scheduler::BackgroundExecutor,
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

    /// Converts this task into a fallible task that returns `Option<T>`.
    ///
    /// Unlike the standard `Task<T>`, a [`FallibleTask`] will return `None`
    /// if the task was cancelled.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Background task that gracefully handles cancellation:
    /// cx.background_spawn(async move {
    ///     let result = foreground_task.fallible().await;
    ///     if let Some(value) = result {
    ///         // Process the value
    ///     }
    ///     // If None, task was cancelled - just exit gracefully
    /// }).detach();
    /// ```
    pub fn fallible(self) -> FallibleTask<T> {
        self.0.fallible()
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
            inner: scheduler::BackgroundExecutor::new(scheduler),
            dispatcher,
        }
    }

    /// Close this executor. Tasks will not run after this is called.
    pub fn close(&self) {
        self.inner.close();
    }

    /// Enqueues the given future to be run to completion on a background thread.
    #[track_caller]
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.spawn_with_priority(Priority::default(), future.boxed())
    }

    /// Enqueues the given future to be run to completion on a background thread with the given priority.
    ///
    /// When `Priority::RealtimeAudio` is used, the task runs on a dedicated thread with
    /// realtime scheduling priority, suitable for audio processing.
    #[track_caller]
    pub fn spawn_with_priority<R>(
        &self,
        priority: Priority,
        future: impl Future<Output = R> + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
    {
        if priority == Priority::RealtimeAudio {
            Task::from_scheduler(self.inner.spawn_realtime(future))
        } else {
            Task::from_scheduler(self.inner.spawn_with_priority(priority, future))
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
        use std::sync::{Arc, atomic::AtomicBool};

        struct NotifyOnDrop<'a>(&'a (Condvar, Mutex<bool>));

        impl Drop for NotifyOnDrop<'_> {
            fn drop(&mut self) {
                *self.0.1.lock() = true;
                self.0.0.notify_all();
            }
        }

        struct WaitOnDrop<'a>(&'a (Condvar, Mutex<bool>));

        impl Drop for WaitOnDrop<'_> {
            fn drop(&mut self) {
                let mut done = self.0.1.lock();
                if !*done {
                    self.0.0.wait(&mut done);
                }
            }
        }

        let dispatcher = self.dispatcher.clone();
        let location = core::panic::Location::caller();
        let closed = Arc::new(AtomicBool::new(false));

        let pair = &(Condvar::new(), Mutex::new(false));
        let _wait_guard = WaitOnDrop(pair);

        let (runnable, task) = unsafe {
            async_task::Builder::new()
                .metadata(RunnableMeta { location, closed })
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
        self.inner.scheduler().clock().now()
    }

    /// Returns a task that will complete after the given duration.
    /// Depending on other concurrent tasks the elapsed duration may be longer
    /// than requested.
    pub fn timer(&self, duration: Duration) -> Task<()> {
        if duration.is_zero() {
            return Task::ready(());
        }
        self.spawn(self.inner.scheduler().timer(duration))
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

    /// In tests, run tasks until the scheduler would park.
    ///
    /// Under the scheduler-backed test dispatcher, `tick()` will not advance the clock, so a pending
    /// timer can keep `has_pending_tasks()` true even after all currently-runnable tasks have been
    /// drained. To preserve the historical semantics that tests relied on (drain all work that can
    /// make progress), we advance the clock to the next timer when no runnable tasks remain.
    #[cfg(any(test, feature = "test-support"))]
    pub fn run_until_parked(&self) {
        let scheduler = self.dispatcher.as_test().unwrap().scheduler();
        scheduler.run();
    }

    /// In tests, prevents `run_until_parked` from panicking if there are outstanding tasks.
    #[cfg(any(test, feature = "test-support"))]
    pub fn allow_parking(&self) {
        self.dispatcher
            .as_test()
            .unwrap()
            .scheduler()
            .allow_parking();

        if std::env::var("GPUI_RUN_UNTIL_PARKED_LOG").ok().as_deref() == Some("1") {
            log::warn!("[gpui::executor] allow_parking: enabled");
        }
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

    /// Close this executor. Tasks will not run after this is called.
    pub fn close(&self) {
        self.inner.close();
    }

    /// Enqueues the given Task to run on the main thread.
    #[track_caller]
    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where
        R: 'static,
    {
        Task::from_scheduler(self.inner.spawn(future.boxed_local()))
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

    /// Used by the test harness to run an async test in a synchronous fashion.
    #[cfg(any(test, feature = "test-support"))]
    #[track_caller]
    pub fn block_test<R>(&self, future: impl Future<Output = R>) -> R {
        use std::cell::Cell;

        let scheduler = self.inner.scheduler();

        let output = Cell::new(None);
        let future = async {
            output.set(Some(future.await));
        };
        let mut future = std::pin::pin!(future);

        // In async GPUI tests, we must allow foreground tasks scheduled by the test itself
        // (which are associated with the test session) to make progress while we block.
        // Otherwise, awaiting futures that depend on same-session foreground work can deadlock.
        scheduler.block(None, future.as_mut(), None);

        output.take().expect("block_test future did not complete")
    }

    /// Block the current thread until the given future resolves.
    /// Consider using `block_with_timeout` instead.
    pub fn block_on<R>(&self, future: impl Future<Output = R>) -> R {
        self.inner.block_on(future)
    }

    /// Block the current thread until the given future resolves or the timeout elapses.
    pub fn block_with_timeout<R, Fut: Future<Output = R>>(
        &self,
        duration: Duration,
        future: Fut,
    ) -> Result<R, impl Future<Output = R> + use<R, Fut>> {
        self.inner.block_with_timeout(duration, future)
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
        let future = async {
            self.rx.next().await;
        };
        let mut future = std::pin::pin!(future);
        self.executor
            .inner
            .scheduler()
            .block(None, future.as_mut(), None);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{App, TestDispatcher, TestPlatform};
    use std::cell::RefCell;

    /// Helper to create test infrastructure.
    /// Returns (dispatcher, background_executor, app).
    fn create_test_app() -> (TestDispatcher, BackgroundExecutor, Rc<crate::AppCell>) {
        let dispatcher = TestDispatcher::new(0);
        let arc_dispatcher = Arc::new(dispatcher.clone());
        let background_executor = BackgroundExecutor::new(arc_dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(arc_dispatcher);

        let platform = TestPlatform::new(background_executor.clone(), foreground_executor);
        let asset_source = Arc::new(());
        let http_client = http_client::FakeHttpClient::with_404_response();

        let app = App::new_app(platform, asset_source, http_client);
        (dispatcher, background_executor, app)
    }

    #[test]
    fn sanity_test_tasks_run() {
        let (dispatcher, _background_executor, app) = create_test_app();
        let foreground_executor = app.borrow().foreground_executor.clone();

        let task_ran = Rc::new(RefCell::new(false));

        foreground_executor
            .spawn({
                let task_ran = Rc::clone(&task_ran);
                async move {
                    *task_ran.borrow_mut() = true;
                }
            })
            .detach();

        // Run dispatcher while app is still alive
        dispatcher.run_until_parked();

        // Task should have run
        assert!(
            *task_ran.borrow(),
            "Task should run normally when app is alive"
        );
    }

    #[test]
    fn test_task_cancelled_when_app_dropped() {
        let (dispatcher, _background_executor, app) = create_test_app();
        let foreground_executor = app.borrow().foreground_executor.clone();
        let app_weak = Rc::downgrade(&app);

        let task_ran = Rc::new(RefCell::new(false));
        let task_ran_clone = Rc::clone(&task_ran);

        foreground_executor
            .spawn(async move {
                *task_ran_clone.borrow_mut() = true;
            })
            .detach();

        drop(app);

        assert!(app_weak.upgrade().is_none(), "App should have been dropped");

        dispatcher.run_until_parked();

        // The task should have been cancelled, not run
        assert!(
            !*task_ran.borrow(),
            "Task should have been cancelled when app was dropped, but it ran!"
        );
    }

    #[test]
    fn test_nested_tasks_both_cancel() {
        let (dispatcher, _background_executor, app) = create_test_app();
        let foreground_executor = app.borrow().foreground_executor.clone();
        let app_weak = Rc::downgrade(&app);

        let outer_completed = Rc::new(RefCell::new(false));
        let inner_completed = Rc::new(RefCell::new(false));
        let reached_await = Rc::new(RefCell::new(false));

        let outer_flag = Rc::clone(&outer_completed);
        let inner_flag = Rc::clone(&inner_completed);
        let await_flag = Rc::clone(&reached_await);

        // Channel to block the inner task until we're ready
        let (tx, rx) = futures::channel::oneshot::channel::<()>();

        let inner_executor = foreground_executor.clone();

        foreground_executor
            .spawn(async move {
                let inner_task = inner_executor.spawn({
                    let inner_flag = Rc::clone(&inner_flag);
                    async move {
                        rx.await.ok();
                        *inner_flag.borrow_mut() = true;
                    }
                });

                *await_flag.borrow_mut() = true;

                inner_task.await;

                *outer_flag.borrow_mut() = true;
            })
            .detach();

        // Run dispatcher until outer task reaches the await point
        // The inner task will be blocked on the channel
        dispatcher.run_until_parked();

        // Verify we actually reached the await point before dropping the app
        assert!(
            *reached_await.borrow(),
            "Outer task should have reached the await point"
        );

        // Neither task should have completed yet
        assert!(
            !*outer_completed.borrow(),
            "Outer task should not have completed yet"
        );
        assert!(
            !*inner_completed.borrow(),
            "Inner task should not have completed yet"
        );

        // Drop the channel sender and app while outer is awaiting inner
        drop(tx);
        drop(app);
        assert!(app_weak.upgrade().is_none(), "App should have been dropped");

        // Run dispatcher - both tasks should be cancelled
        dispatcher.run_until_parked();

        // Neither task should have completed (both were cancelled)
        assert!(
            !*outer_completed.borrow(),
            "Outer task should have been cancelled, not completed"
        );
        assert!(
            !*inner_completed.borrow(),
            "Inner task should have been cancelled, not completed"
        );
    }

    #[test]
    #[should_panic]
    fn test_polling_cancelled_task_panics() {
        let (dispatcher, _background_executor, app) = create_test_app();
        let foreground_executor = app.borrow().foreground_executor.clone();
        let app_weak = Rc::downgrade(&app);

        let task = foreground_executor.spawn(async move { 42 });

        drop(app);

        assert!(app_weak.upgrade().is_none(), "App should have been dropped");

        dispatcher.run_until_parked();

        foreground_executor.block_on(task);
    }

    #[test]
    fn test_polling_cancelled_task_returns_none_with_fallible() {
        let (dispatcher, _background_executor, app) = create_test_app();
        let foreground_executor = app.borrow().foreground_executor.clone();
        let app_weak = Rc::downgrade(&app);

        let task = foreground_executor.spawn(async move { 42 }).fallible();

        drop(app);

        assert!(app_weak.upgrade().is_none(), "App should have been dropped");

        dispatcher.run_until_parked();

        let result = foreground_executor.block_on(task);
        assert_eq!(result, None, "Cancelled task should return None");
    }
}
