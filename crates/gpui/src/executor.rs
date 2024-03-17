use crate::{AppContext, PlatformDispatcher};
use futures::{channel::mpsc, pin_mut, FutureExt};
use smol::prelude::*;
use std::{
    fmt::Debug,
    marker::PhantomData,
    mem,
    num::NonZeroUsize,
    pin::Pin,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    task::{Context, Poll},
    time::Duration,
};
use util::TryFutureExt;
use waker_fn::waker_fn;

#[cfg(any(test, feature = "test-support"))]
use rand::rngs::StdRng;

/// A pointer to the executor that is currently running,
/// for spawning background tasks.
#[derive(Clone)]
pub struct BackgroundExecutor {
    dispatcher: Arc<dyn PlatformDispatcher>,
}

/// A pointer to the executor that is currently running,
/// for spawning tasks on the main thread.
#[derive(Clone)]
pub struct ForegroundExecutor {
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
pub enum Task<T> {
    /// A task that is ready to return a value
    Ready(Option<T>),

    /// A task that is currently running.
    Spawned(async_task::Task<T>),
}

impl<T> Task<T> {
    /// Creates a new task that will resolve with the value
    pub fn ready(val: T) -> Self {
        Task::Ready(Some(val))
    }

    /// Detaching a task runs it to completion in the background
    pub fn detach(self) {
        match self {
            Task::Ready(_) => {}
            Task::Spawned(task) => task.detach(),
        }
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
    pub fn detach_and_log_err(self, cx: &AppContext) {
        let location = core::panic::Location::caller();
        cx.foreground_executor()
            .spawn(self.log_tracked_err(*location))
            .detach();
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match unsafe { self.get_unchecked_mut() } {
            Task::Ready(val) => Poll::Ready(val.take().unwrap()),
            Task::Spawned(task) => task.poll(cx),
        }
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
    #[doc(hidden)]
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self { dispatcher }
    }

    /// Enqueues the given future to be run to completion on a background thread.
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.spawn_internal::<R>(Box::pin(future), None)
    }

    /// Enqueues the given future to be run to completion on a background thread.
    /// The given label can be used to control the priority of the task in tests.
    pub fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
    {
        self.spawn_internal::<R>(Box::pin(future), Some(label))
    }

    fn spawn_internal<R: Send + 'static>(
        &self,
        future: AnyFuture<R>,
        label: Option<TaskLabel>,
    ) -> Task<R> {
        let dispatcher = self.dispatcher.clone();
        let (runnable, task) =
            async_task::spawn(future, move |runnable| dispatcher.dispatch(runnable, label));
        runnable.schedule();
        Task::Spawned(task)
    }

    /// Used by the test harness to run an async test in a synchronous fashion.
    #[cfg(any(test, feature = "test-support"))]
    #[track_caller]
    pub fn block_test<R>(&self, future: impl Future<Output = R>) -> R {
        if let Ok(value) = self.block_internal(false, future, usize::MAX) {
            value
        } else {
            unreachable!()
        }
    }

    /// Block the current thread until the given future resolves.
    /// Consider using `block_with_timeout` instead.
    pub fn block<R>(&self, future: impl Future<Output = R>) -> R {
        if let Ok(value) = self.block_internal(true, future, usize::MAX) {
            value
        } else {
            unreachable!()
        }
    }

    #[track_caller]
    pub(crate) fn block_internal<R>(
        &self,
        background_only: bool,
        future: impl Future<Output = R>,
        mut max_ticks: usize,
    ) -> Result<R, ()> {
        pin_mut!(future);
        let unparker = self.dispatcher.unparker();
        let awoken = Arc::new(AtomicBool::new(false));

        let waker = waker_fn({
            let awoken = awoken.clone();
            move || {
                awoken.store(true, SeqCst);
                unparker.unpark();
            }
        });
        let mut cx = std::task::Context::from_waker(&waker);

        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(result) => return Ok(result),
                Poll::Pending => {
                    if max_ticks == 0 {
                        return Err(());
                    }
                    max_ticks -= 1;

                    if !self.dispatcher.tick(background_only) {
                        if awoken.swap(false, SeqCst) {
                            continue;
                        }

                        #[cfg(any(test, feature = "test-support"))]
                        if let Some(test) = self.dispatcher.as_test() {
                            if !test.parking_allowed() {
                                let mut backtrace_message = String::new();
                                if let Some(backtrace) = test.waiting_backtrace() {
                                    backtrace_message =
                                        format!("\nbacktrace of waiting future:\n{:?}", backtrace);
                                }
                                panic!("parked with nothing left to run\n{:?}", backtrace_message)
                            }
                        }

                        self.dispatcher.park();
                    }
                }
            }
        }
    }

    /// Block the current thread until the given future resolves
    /// or `duration` has elapsed.
    pub fn block_with_timeout<R>(
        &self,
        duration: Duration,
        future: impl Future<Output = R>,
    ) -> Result<R, impl Future<Output = R>> {
        let mut future = Box::pin(future.fuse());
        if duration.is_zero() {
            return Err(future);
        }

        #[cfg(any(test, feature = "test-support"))]
        let max_ticks = self
            .dispatcher
            .as_test()
            .map_or(usize::MAX, |dispatcher| dispatcher.gen_block_on_ticks());
        #[cfg(not(any(test, feature = "test-support")))]
        let max_ticks = usize::MAX;

        let mut timer = self.timer(duration).fuse();

        let timeout = async {
            futures::select_biased! {
                value = future => Ok(value),
                _ = timer => Err(()),
            }
        };
        match self.block_internal(true, timeout, max_ticks) {
            Ok(Ok(value)) => Ok(value),
            _ => Err(future),
        }
    }

    /// Scoped lets you start a number of tasks and waits
    /// for all of them to complete before returning.
    pub async fn scoped<'scope, F>(&self, scheduler: F)
    where
        F: FnOnce(&mut Scope<'scope>),
    {
        let mut scope = Scope::new(self.clone());
        (scheduler)(&mut scope);
        let spawned = mem::take(&mut scope.futures)
            .into_iter()
            .map(|f| self.spawn(f))
            .collect::<Vec<_>>();
        for task in spawned {
            task.await;
        }
    }

    /// Returns a task that will complete after the given duration.
    /// Depending on other concurrent tasks the elapsed duration may be longer
    /// than requested.
    pub fn timer(&self, duration: Duration) -> Task<()> {
        let (runnable, task) = async_task::spawn(async move {}, {
            let dispatcher = self.dispatcher.clone();
            move |runnable| dispatcher.dispatch_after(duration, runnable)
        });
        runnable.schedule();
        Task::Spawned(task)
    }

    /// in tests, start_waiting lets you indicate which task is waiting (for debugging only)
    #[cfg(any(test, feature = "test-support"))]
    pub fn start_waiting(&self) {
        self.dispatcher.as_test().unwrap().start_waiting();
    }

    /// in tests, removes the debugging data added by start_waiting
    #[cfg(any(test, feature = "test-support"))]
    pub fn finish_waiting(&self) {
        self.dispatcher.as_test().unwrap().finish_waiting();
    }

    /// in tests, run an arbitrary number of tasks (determined by the SEED environment variable)
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_random_delay(&self) -> impl Future<Output = ()> {
        self.dispatcher.as_test().unwrap().simulate_random_delay()
    }

    /// in tests, indicate that a given task from `spawn_labeled` should run after everything else
    #[cfg(any(test, feature = "test-support"))]
    pub fn deprioritize(&self, task_label: TaskLabel) {
        self.dispatcher.as_test().unwrap().deprioritize(task_label)
    }

    /// in tests, move time forward. This does not run any tasks, but does make `timer`s ready.
    #[cfg(any(test, feature = "test-support"))]
    pub fn advance_clock(&self, duration: Duration) {
        self.dispatcher.as_test().unwrap().advance_clock(duration)
    }

    /// in tests, run one task.
    #[cfg(any(test, feature = "test-support"))]
    pub fn tick(&self) -> bool {
        self.dispatcher.as_test().unwrap().tick(false)
    }

    /// in tests, run all tasks that are ready to run. If after doing so
    /// the test still has outstanding tasks, this will panic. (See also `allow_parking`)
    #[cfg(any(test, feature = "test-support"))]
    pub fn run_until_parked(&self) {
        self.dispatcher.as_test().unwrap().run_until_parked()
    }

    /// in tests, prevents `run_until_parked` from panicking if there are outstanding tasks.
    /// This is useful when you are integrating other (non-GPUI) futures, like disk access, that
    /// do take real async time to run.
    #[cfg(any(test, feature = "test-support"))]
    pub fn allow_parking(&self) {
        self.dispatcher.as_test().unwrap().allow_parking();
    }

    /// undoes the effect of [`allow_parking`].
    #[cfg(any(test, feature = "test-support"))]
    pub fn forbid_parking(&self) {
        self.dispatcher.as_test().unwrap().forbid_parking();
    }

    /// in tests, returns the rng used by the dispatcher and seeded by the `SEED` environment variable
    #[cfg(any(test, feature = "test-support"))]
    pub fn rng(&self) -> StdRng {
        self.dispatcher.as_test().unwrap().rng()
    }

    /// How many CPUs are available to the dispatcher
    pub fn num_cpus(&self) -> usize {
        num_cpus::get()
    }

    /// Whether we're on the main thread.
    pub fn is_main_thread(&self) -> bool {
        self.dispatcher.is_main_thread()
    }

    #[cfg(any(test, feature = "test-support"))]
    /// in tests, control the number of ticks that `block_with_timeout` will run before timing out.
    pub fn set_block_on_ticks(&self, range: std::ops::RangeInclusive<usize>) {
        self.dispatcher.as_test().unwrap().set_block_on_ticks(range);
    }
}

/// ForegroundExecutor runs things on the main thread.
impl ForegroundExecutor {
    /// Creates a new ForegroundExecutor from the given PlatformDispatcher.
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self {
            dispatcher,
            not_send: PhantomData,
        }
    }

    /// Enqueues the given Task to run on the main thread at some point in the future.
    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where
        R: 'static,
    {
        let dispatcher = self.dispatcher.clone();
        fn inner<R: 'static>(
            dispatcher: Arc<dyn PlatformDispatcher>,
            future: AnyLocalFuture<R>,
        ) -> Task<R> {
            let (runnable, task) = async_task::spawn_local(future, move |runnable| {
                dispatcher.dispatch_on_main_thread(runnable)
            });
            runnable.schedule();
            Task::Spawned(task)
        }
        inner::<R>(dispatcher, Box::pin(future))
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

impl<'a> Drop for Scope<'a> {
    fn drop(&mut self) {
        self.tx.take().unwrap();

        // Wait until the channel is closed, which means that all of the spawned
        // futures have resolved.
        self.executor.block(self.rx.next());
    }
}
