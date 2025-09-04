use crate::App;
use async_task::Runnable;
use futures::channel::mpsc;
use scheduler::Timer;
use smol::prelude::*;
use std::mem::ManuallyDrop;
use std::panic::Location;
use std::thread::{self, ThreadId};
use std::{
    fmt::Debug,
    marker::PhantomData,
    mem,
    num::NonZeroUsize,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
    task::{Context, Poll},
    time::{Duration, Instant},
};
use util::TryFutureExt;

#[cfg(any(test, feature = "test-support"))]
use rand::rngs::StdRng;

/// A pointer to the executor that is currently running,
/// for spawning background tasks.
#[derive(Clone)]
pub struct BackgroundExecutor(scheduler::BackgroundExecutor);

/// A pointer to the executor that is currently running,
/// for spawning tasks on the main thread.
///
/// This is intentionally `!Send` via the `not_send` marker field. This is because
/// `ForegroundExecutor::spawn` does not require `Send` but checks at runtime that the future is
/// only polled from the same thread it was spawned from. These checks would fail when spawning
/// foreground tasks from from background threads.
#[derive(Clone)]
pub struct ForegroundExecutor(scheduler::ForegroundExecutor);

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
    /// Creates a new task that will resolve with the value
    pub fn ready(val: T) -> Self {
        Task(scheduler::Task::ready(val))
    }

    /// Detaching a task runs it to completion in the background
    pub fn detach(self) {
        self.0.detach()
    }
}

impl<E, T> Task<Result<T, E>>
where
    T: 'static,
    E: 'static + Debug,
{
    /// Run the task to completion in the background and log any
    /// errors that occur.
    #[track_caller]
    pub fn detach_and_log_err(self, cx: &App) {
        let location = core::panic::Location::caller();
        cx.foreground_executor()
            .spawn(self.log_tracked_err(*location))
            .detach();
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        unsafe { self.map_unchecked_mut(|t| &mut t.0) }.poll(cx)
    }
}

/// A task label is an opaque identifier that you can use to
/// refer to a task in tests.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TaskLabel(NonZeroUsize);

impl Default for TaskLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskLabel {
    /// Construct a new task label.
    pub fn new() -> Self {
        static NEXT_TASK_LABEL: AtomicUsize = AtomicUsize::new(1);
        Self(NEXT_TASK_LABEL.fetch_add(1, SeqCst).try_into().unwrap())
    }
}

type AnyLocalFuture<R> = Pin<Box<dyn 'static + Future<Output = R>>>;

type AnyFuture<R> = Pin<Box<dyn 'static + Send + Future<Output = R>>>;

/// BackgroundExecutor lets you run things on background threads.
/// In production this is a thread pool with no ordering guarantees.
/// In tests this is simulated by running tasks one by one in a deterministic
/// (but arbitrary) order controlled by the `SEED` environment variable.
impl BackgroundExecutor {
    /// Create a new BackgroundExecutor
    pub fn new(executor: scheduler::BackgroundExecutor) -> Self {
        Self(executor)
    }

    /// Enqueues the given future to be run to completion on a background thread.
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        // todo!("reduce monomorphization")
        Task(self.0.spawn(future))
    }

    /// Enqueues the given future to be run to completion on a background thread.
    /// The given label can be used to control the priority of the task in tests.
    pub fn spawn_labeled<R>(
        &self,
        _label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
    {
        // todo!("Solve deprioritization in project_tests.rs")
        Task(self.0.spawn(future))
    }

    /// Used by the test harness to run an async test in a synchronous fashion.
    #[cfg(any(test, feature = "test-support"))]
    #[track_caller]
    pub fn block_test<R>(&self, future: impl Future<Output = R>) -> R {
        todo!("move this to foreground executor (do we even need it ...test??)")
    }

    /// Block the current thread until the given future resolves.
    /// Consider using `block_with_timeout` instead.
    pub fn block<R>(&self, _future: impl Future<Output = R>) -> R {
        todo!("move this to foreground executor")
    }

    /// Block the current thread until the given future resolves
    /// or `duration` has elapsed.
    pub fn block_with_timeout<Fut: Future>(
        &self,
        _duration: Duration,
        _future: Fut,
    ) -> Result<Fut::Output, Fut> {
        todo!("move this to foreground executor")
    }

    /// Scoped lets you start a number of tasks and waits
    /// for all of them to complete before returning.
    pub async fn scoped<'scope, F>(&self, f: F)
    where
        F: FnOnce(&mut Scope<'scope>),
    {
        let mut scope = Scope::new(self.clone());
        (f)(&mut scope);
        let spawned = mem::take(&mut scope.futures)
            .into_iter()
            .map(|f| self.spawn(f))
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
        todo!("convert to a chrono::DateTime?")
    }

    /// Returns a task that will complete after the given duration.
    /// Depending on other concurrent tasks the elapsed duration may be longer
    /// than requested.
    pub fn timer(&self, duration: Duration) -> Timer {
        self.0.timer(duration)
    }

    /// in tests, start_waiting lets you indicate which task is waiting (for debugging only)
    #[cfg(any(test, feature = "test-support"))]
    pub fn start_waiting(&self) {
        // self.0.start_waiting()
        todo!()
    }

    /// in tests, removes the debugging data added by start_waiting
    #[cfg(any(test, feature = "test-support"))]
    pub fn finish_waiting(&self) {
        // self.0.finish_waiting()
        todo!()
    }

    /// in tests, run an arbitrary number of tasks (determined by the SEED environment variable)
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_random_delay(&self) -> impl Future<Output = ()> + use<> {
        // todo!()
        std::future::pending()
    }

    /// in tests, indicate that a given task from `spawn_labeled` should run after everything else
    #[cfg(any(test, feature = "test-support"))]
    pub fn deprioritize(&self, task_label: TaskLabel) {
        // self.0.deprioritize(task_label)
        todo!()
    }

    /// in tests, move time forward. This does not run any tasks, but does make `timer`s ready.
    #[cfg(any(test, feature = "test-support"))]
    pub fn advance_clock(&self, duration: Duration) {
        // self.0.advance_clock(duration)
        todo!()
    }

    /// in tests, run one task.
    #[cfg(any(test, feature = "test-support"))]
    pub fn tick(&self) -> bool {
        // self.0.tick()
        todo!()
    }

    /// in tests, run all tasks that are ready to run. If after doing so
    /// the test still has outstanding tasks, this will panic. (See also [`Self::allow_parking`])
    #[cfg(any(test, feature = "test-support"))]
    pub fn run_until_parked(&self) {
        // self.0.run_until_parked()
        todo!()
    }

    /// in tests, prevents `run_until_parked` from panicking if there are outstanding tasks.
    /// This is useful when you are integrating other (non-GPUI) futures, like disk access, that
    /// do take real async time to run.
    #[cfg(any(test, feature = "test-support"))]
    pub fn allow_parking(&self) {
        // self.0.allow_parking()
        todo!()
    }

    /// undoes the effect of [`Self::allow_parking`].
    #[cfg(any(test, feature = "test-support"))]
    pub fn forbid_parking(&self) {
        // self.0.forbid_parking()
        todo!()
    }

    /// adds detail to the "parked with nothing let to run" message.
    #[cfg(any(test, feature = "test-support"))]
    pub fn set_waiting_hint(&self, msg: Option<String>) {
        // self.0.set_waiting_hint(msg)
        todo!()
    }

    /// in tests, returns the rng used by the dispatcher and seeded by the `SEED` environment variable
    #[cfg(any(test, feature = "test-support"))]
    pub fn rng(&self) -> StdRng {
        // self.0.rng()
        todo!()
    }

    /// How many CPUs are available to the dispatcher.
    pub fn num_cpus(&self) -> usize {
        // self.0.num_cpus()
        todo!()
    }

    #[cfg(any(test, feature = "test-support"))]
    /// in tests, control the number of ticks that `block_with_timeout` will run before timing out.
    pub fn set_block_on_ticks(&self, range: std::ops::RangeInclusive<usize>) {
        // self.0.set_block_on_ticks(range)
        todo!()
    }
}

/// ForegroundExecutor runs things on the main thread.
impl ForegroundExecutor {
    /// Create a new ForegroundExecutor
    pub fn new(executor: scheduler::ForegroundExecutor) -> Self {
        Self(executor)
    }

    /// Enqueues the given Task to run on the main thread at some point in the future.
    #[track_caller]
    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where
        R: 'static,
    {
        // let dispatcher = self.dispatcher.clone();

        // #[track_caller]
        // fn inner<R: 'static>(
        //     dispatcher: Arc<dyn PlatformDispatcher>,
        //     future: AnyLocalFuture<R>,
        // ) -> Task<R> {
        //     let (runnable, task) = spawn_local_with_source_location(future, move |runnable| {
        //         dispatcher.dispatch_on_main_thread(runnable)
        //     });
        //     runnable.schedule();
        //     Task(TaskState::Spawned(task))
        // }
        // inner::<R>(dispatcher, Box::pin(future))
        // todo!("reduce monomorphization and spawn_local_with_source_location")
        Task(self.0.spawn(future))
    }
}

/// Variant of `async_task::spawn_local` that includes the source location of the spawn in panics.
///
/// Copy-modified from:
/// <https://github.com/smol-rs/async-task/blob/ca9dbe1db9c422fd765847fa91306e30a6bb58a9/src/runnable.rs#L405>
#[track_caller]
fn spawn_local_with_source_location<Fut, S>(
    future: Fut,
    schedule: S,
) -> (Runnable<()>, async_task::Task<Fut::Output, ()>)
where
    Fut: Future + 'static,
    Fut::Output: 'static,
    S: async_task::Schedule<()> + Send + Sync + 'static,
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

    // Wrap the future into one that checks which thread it's on.
    let future = Checked {
        id: thread_id(),
        inner: ManuallyDrop::new(future),
        location: Location::caller(),
    };

    unsafe { async_task::spawn_unchecked(future, schedule) }
}

/// Scope manages a set of tasks that are enqueued and waited on together. See [`BackgroundExecutor::scoped`].
pub struct Scope<'a> {
    executor: BackgroundExecutor,
    futures: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    tx: Option<mpsc::Sender<()>>,
    rx: mpsc::Receiver<()>,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Scope<'a> {
    fn new(executor: BackgroundExecutor) -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self {
            executor,
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
