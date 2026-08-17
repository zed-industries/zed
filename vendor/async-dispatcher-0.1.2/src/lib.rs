pub use async_task::Runnable;
use futures_lite::FutureExt;
use std::{
    error::Error,
    fmt,
    future::Future,
    sync::{mpsc::RecvTimeoutError, OnceLock},
    task::Poll,
    time::{Duration, Instant},
};

pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    futures_lite::future::block_on(future)
}

static DISPATCHER: OnceLock<Box<dyn Dispatcher>> = OnceLock::new();

pub trait Dispatcher: 'static + Send + Sync {
    fn dispatch(&self, runnable: Runnable);
    fn dispatch_after(&self, duration: Duration, runnable: Runnable);
}

pub fn set_dispatcher(dispatcher: impl Dispatcher) {
    DISPATCHER.set(Box::new(dispatcher)).ok();
}

fn get_dispatcher() -> &'static dyn Dispatcher {
    DISPATCHER
        .get()
        .expect("The dispatcher requires a call to set_dispatcher()")
        .as_ref()
}

#[derive(Debug)]
pub struct JoinHandle<T> {
    task: Option<async_task::Task<T>>,
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static + Send,
    F::Output: 'static + Send,
{
    let dispatcher = get_dispatcher();
    let (runnable, task) = async_task::spawn(future, |runnable| dispatcher.dispatch(runnable));
    runnable.schedule();
    JoinHandle { task: Some(task) }
}

impl<T> Future for JoinHandle<T> {
    type Output = T;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        std::pin::Pin::new(
            self.task
                .as_mut()
                .expect("poll should not be called after drop"),
        )
        .poll(cx)
    }
}

impl<T> Drop for JoinHandle<T> {
    fn drop(&mut self) {
        self.task
            .take()
            .expect("This is the only place the option is mutated")
            .detach();
    }
}

pub struct Sleep {
    task: async_task::Task<()>,
}

pub fn sleep(time: Duration) -> Sleep {
    let dispatcher = get_dispatcher();
    let (runnable, task) = async_task::spawn(async {}, move |runnable| {
        dispatcher.dispatch_after(time, runnable)
    });
    runnable.schedule();

    Sleep { task }
}

impl Sleep {
    pub fn reset(&mut self, deadline: Instant) {
        let duration = deadline.saturating_duration_since(Instant::now());
        self.task = sleep(duration).task
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        std::pin::Pin::new(&mut self.task).poll(cx)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeoutError;

impl Error for TimeoutError {}

pub fn timeout<T>(
    duration: Duration,
    future: T,
) -> impl Future<Output = Result<T::Output, TimeoutError>>
where
    T: Future,
{
    let future = async move { Ok(future.await) };
    let timeout = async move {
        sleep(duration).await;
        Err(TimeoutError)
    };
    future.or(timeout)
}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "future has timed out".fmt(f)
    }
}

pub fn thread_dispatcher() -> impl Dispatcher {
    struct SimpleDispatcher {
        tx: std::sync::mpsc::Sender<(Runnable, Option<Instant>)>,
        _thread: std::thread::JoinHandle<()>,
    }

    impl Dispatcher for SimpleDispatcher {
        fn dispatch(&self, runnable: Runnable) {
            self.tx.send((runnable, None)).ok();
        }

        fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
            self.tx
                .send((runnable, Some(Instant::now() + duration)))
                .ok();
        }
    }

    let (tx, rx) = std::sync::mpsc::channel::<(Runnable, Option<Instant>)>();
    let _thread = std::thread::spawn(move || {
        let mut timers = Vec::<(Runnable, Instant)>::new();
        let mut recv_timeout = Duration::MAX;
        loop {
            match rx.recv_timeout(recv_timeout) {
                Ok((runnable, time)) => {
                    if let Some(time) = time {
                        let now = Instant::now();
                        if time > now {
                            let ix = match timers.binary_search_by_key(&time, |t| t.1) {
                                Ok(i) | Err(i) => i,
                            };
                            timers.insert(ix, (runnable, time));
                            recv_timeout = timers.first().unwrap().1 - now;
                            continue;
                        }
                    }
                    runnable.run();
                }
                Err(RecvTimeoutError::Timeout) => {
                    let now = Instant::now();
                    while let Some((_, time)) = timers.first() {
                        if *time > now {
                            recv_timeout = *time - now;
                            break;
                        }
                        timers.remove(0).0.run();
                    }
                }
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
    });

    SimpleDispatcher { tx, _thread }
}

#[cfg(feature = "macros")]
pub use async_dispatcher_macros::test;

#[cfg(feature = "macros")]
pub use async_dispatcher_macros::main;
