use crate::{AppContext, PlatformDispatcher};
use futures::{channel::mpsc, pin_mut};
use smol::prelude::*;
use std::{
    fmt::Debug,
    marker::PhantomData,
    mem,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use util::TryFutureExt;
use waker_fn::waker_fn;

#[derive(Clone)]
pub struct Executor {
    dispatcher: Arc<dyn PlatformDispatcher>,
}

#[must_use]
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
    T: 'static + Send,
    E: 'static + Send + Debug,
{
    pub fn detach_and_log_err(self, cx: &mut AppContext) {
        cx.executor().spawn(self.log_err()).detach();
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

impl Executor {
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self { dispatcher }
    }

    /// Enqueues the given closure to be run on any thread. The closure returns
    /// a future which will be run to completion on any available thread.
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        let dispatcher = self.dispatcher.clone();
        let (runnable, task) =
            async_task::spawn(future, move |runnable| dispatcher.dispatch(runnable));
        runnable.schedule();
        Task::Spawned(task)
    }

    /// Enqueues the given closure to run on the application's event loop.
    /// Returns the result asynchronously.
    pub fn run_on_main<F, R>(&self, func: F) -> Task<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        if self.dispatcher.is_main_thread() {
            Task::ready(func())
        } else {
            self.spawn_on_main(move || async move { func() })
        }
    }

    /// Enqueues the given closure to be run on the application's event loop. The
    /// closure returns a future which will be run to completion on the main thread.
    pub fn spawn_on_main<F, R>(&self, func: impl FnOnce() -> F + Send + 'static) -> Task<R>
    where
        F: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let (runnable, task) = async_task::spawn(
            {
                let this = self.clone();
                async move {
                    let task = this.spawn_on_main_local(func());
                    task.await
                }
            },
            {
                let dispatcher = self.dispatcher.clone();
                move |runnable| dispatcher.dispatch_on_main_thread(runnable)
            },
        );
        runnable.schedule();
        Task::Spawned(task)
    }

    /// Enqueues the given closure to be run on the application's event loop. Must
    /// be called on the main thread.
    pub fn spawn_on_main_local<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where
        R: 'static,
    {
        assert!(
            self.dispatcher.is_main_thread(),
            "must be called on main thread"
        );

        let dispatcher = self.dispatcher.clone();
        let (runnable, task) = async_task::spawn_local(future, move |runnable| {
            dispatcher.dispatch_on_main_thread(runnable)
        });
        runnable.schedule();
        Task::Spawned(task)
    }

    pub fn block<R>(&self, future: impl Future<Output = R>) -> R {
        pin_mut!(future);
        let (parker, unparker) = parking::pair();
        let waker = waker_fn(move || {
            unparker.unpark();
        });
        let mut cx = std::task::Context::from_waker(&waker);

        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    if !self.dispatcher.poll() {
                        #[cfg(any(test, feature = "test-support"))]
                        if let Some(_) = self.dispatcher.as_test() {
                            panic!("blocked with nothing left to run")
                        }
                        parker.park();
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
        let mut future = Box::pin(future);
        if duration.is_zero() {
            return Err(future);
        }

        let timeout = {
            let future = &mut future;
            async {
                let timer = async {
                    self.timer(duration).await;
                    Err(())
                };
                let future = async move { Ok(future.await) };
                timer.race(future).await
            }
        };
        match self.block(timeout) {
            Ok(value) => Ok(value),
            Err(_) => Err(future),
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
        todo!("start_waiting")
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn finish_waiting(&self) {
        todo!("finish_waiting")
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_random_delay(&self) -> impl Future<Output = ()> {
        self.dispatcher.as_test().unwrap().simulate_random_delay()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn advance_clock(&self, duration: Duration) {
        self.dispatcher.as_test().unwrap().advance_clock(duration)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn run_until_parked(&self) {
        self.dispatcher.as_test().unwrap().run_until_parked()
    }

    pub fn num_cpus(&self) -> usize {
        num_cpus::get()
    }

    pub fn is_main_thread(&self) -> bool {
        self.dispatcher.is_main_thread()
    }
}

pub struct Scope<'a> {
    executor: Executor,
    futures: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    tx: Option<mpsc::Sender<()>>,
    rx: mpsc::Receiver<()>,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Scope<'a> {
    fn new(executor: Executor) -> Self {
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
