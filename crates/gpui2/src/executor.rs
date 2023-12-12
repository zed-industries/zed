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

#[derive(Clone)]
pub struct BackgroundExecutor {
    dispatcher: Arc<dyn PlatformDispatcher>,
}

#[derive(Clone)]
pub struct ForegroundExecutor {
    dispatcher: Arc<dyn PlatformDispatcher>,
    not_send: PhantomData<Rc<()>>,
}

#[must_use]
#[derive(Debug)]
pub enum Task<T> {
    Ready(Option<T>),
    Spawned(async_task::Task<T>),
}

impl<T> Task<T> {
    pub fn ready(val: T) -> Self {
        Task::Ready(Some(val))
    }

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
    #[track_caller]
    pub fn detach_and_log_err(self, cx: &mut AppContext) {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TaskLabel(NonZeroUsize);

impl TaskLabel {
    pub fn new() -> Self {
        static NEXT_TASK_LABEL: AtomicUsize = AtomicUsize::new(1);
        Self(NEXT_TASK_LABEL.fetch_add(1, SeqCst).try_into().unwrap())
    }
}

type AnyLocalFuture<R> = Pin<Box<dyn 'static + Future<Output = R>>>;

type AnyFuture<R> = Pin<Box<dyn 'static + Send + Future<Output = R>>>;

impl BackgroundExecutor {
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

    #[cfg(any(test, feature = "test-support"))]
    #[track_caller]
    pub fn block_test<R>(&self, future: impl Future<Output = R>) -> R {
        if let Ok(value) = self.block_internal(false, future, usize::MAX) {
            value
        } else {
            unreachable!()
        }
    }

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

    pub fn timer(&self, duration: Duration) -> Task<()> {
        let (runnable, task) = async_task::spawn(async move {}, {
            let dispatcher = self.dispatcher.clone();
            move |runnable| dispatcher.dispatch_after(duration, runnable)
        });
        runnable.schedule();
        Task::Spawned(task)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn start_waiting(&self) {
        self.dispatcher.as_test().unwrap().start_waiting();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn finish_waiting(&self) {
        self.dispatcher.as_test().unwrap().finish_waiting();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_random_delay(&self) -> impl Future<Output = ()> {
        self.dispatcher.as_test().unwrap().simulate_random_delay()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn deprioritize(&self, task_label: TaskLabel) {
        self.dispatcher.as_test().unwrap().deprioritize(task_label)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn advance_clock(&self, duration: Duration) {
        self.dispatcher.as_test().unwrap().advance_clock(duration)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn tick(&self) -> bool {
        self.dispatcher.as_test().unwrap().tick(false)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn run_until_parked(&self) {
        self.dispatcher.as_test().unwrap().run_until_parked()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn allow_parking(&self) {
        self.dispatcher.as_test().unwrap().allow_parking();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn rng(&self) -> StdRng {
        self.dispatcher.as_test().unwrap().rng()
    }

    pub fn num_cpus(&self) -> usize {
        num_cpus::get()
    }

    pub fn is_main_thread(&self) -> bool {
        self.dispatcher.is_main_thread()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_block_on_ticks(&self, range: std::ops::RangeInclusive<usize>) {
        self.dispatcher.as_test().unwrap().set_block_on_ticks(range);
    }
}

impl ForegroundExecutor {
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self {
            dispatcher,
            not_send: PhantomData,
        }
    }

    /// Enqueues the given closure to be run on any thread. The closure returns
    /// a future which will be run to completion on any available thread.
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

    pub fn spawn<F>(&mut self, f: F)
    where
        F: Future<Output = ()> + Send + 'a,
    {
        let tx = self.tx.clone().unwrap();

        // Safety: The 'a lifetime is guaranteed to outlive any of these futures because
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
