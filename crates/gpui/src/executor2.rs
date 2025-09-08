use crate::App;
use futures::channel::mpsc;
use scheduler::Timer;
use smol::prelude::*;
use std::sync::Arc;
use std::{
    fmt::Debug,
    marker::PhantomData,
    mem,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use util::TryFutureExt;

pub use scheduler::{Scheduler, Yield};

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
        fn inner<R: Send + 'static>(
            executor: &scheduler::BackgroundExecutor,
            future: AnyFuture<R>,
        ) -> Task<R> {
            Task(executor.spawn(future))
        }

        inner(&self.0, Box::pin(future))
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

    /// Get the underlying scheduler.
    pub fn scheduler(&self) -> &Arc<dyn Scheduler> {
        &self.0.scheduler()
    }

    /// in tests, run an arbitrary number of tasks (determined by the SEED environment variable)
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_random_delay(&self) -> Yield {
        self.scheduler().as_test().yield_random()
    }

    /// in tests, move time forward. This does not run any tasks, but does make `timer`s ready.
    #[cfg(any(test, feature = "test-support"))]
    pub fn advance_clock(&self, duration: Duration) {
        // self.0.advance_clock(duration)
        todo!()
    }

    /// in tests, run all tasks that are ready to run. If after doing so
    /// the test still has outstanding tasks, this will panic. (See also [`Self::allow_parking`])
    #[cfg(any(test, feature = "test-support"))]
    pub fn run_until_parked(&self) {
        self.scheduler().as_test().run();
    }

    /// in tests, prevents `run_until_parked` from panicking if there are outstanding tasks.
    /// This is useful when you are integrating other (non-GPUI) futures, like disk access, that
    /// do take real async time to run.
    #[cfg(any(test, feature = "test-support"))]
    pub fn allow_parking(&self) {
        self.scheduler().as_test().allow_parking();
    }

    /// undoes the effect of [`Self::allow_parking`].
    #[cfg(any(test, feature = "test-support"))]
    pub fn forbid_parking(&self) {
        self.scheduler().as_test().forbid_parking();
    }

    /// in tests, returns the rng used by the dispatcher and seeded by the `SEED` environment variable
    #[cfg(any(test, feature = "test-support"))]
    pub fn rng(&self) -> StdRng {
        self.scheduler().as_test().rng().lock().clone()
    }

    /// How many CPUs are available for this executor.
    pub fn num_cpus(&self) -> usize {
        #[cfg(any(test, feature = "test-support"))]
        return 4;

        #[cfg(not(any(test, feature = "test-support")))]
        return num_cpus::get();
    }

    #[cfg(any(test, feature = "test-support"))]
    /// in tests, control the number of ticks that `block_with_timeout` will run before timing out.
    pub fn set_block_on_ticks(&self, range: std::ops::RangeInclusive<usize>) {
        self.scheduler().as_test().set_timeout_ticks(range)
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
        #[track_caller]
        fn inner<R: 'static>(
            executor: &scheduler::ForegroundExecutor,
            future: AnyLocalFuture<R>,
        ) -> Task<R> {
            Task(executor.spawn(future))
        }

        inner::<R>(&self.0, Box::pin(future))
    }

    /// Block the current thread until the given future resolves.
    /// Consider using `block_with_timeout` instead.
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.0.block_on(future)
    }

    /// Block the current thread until the given future resolves
    /// or `timeout` has elapsed.
    pub fn block_with_timeout<Fut: Unpin + Future>(
        &self,
        timeout: Duration,
        future: Fut,
    ) -> Result<Fut::Output, Fut> {
        self.0.block_with_timeout(timeout, future)
    }
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
        self.executor.scheduler().block(
            None,
            async {
                self.rx.next().await;
            }
            .boxed(),
            None,
        );
    }
}
