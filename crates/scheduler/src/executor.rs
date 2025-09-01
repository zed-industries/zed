use crate::{Scheduler, SessionId, Timer};
use async_task::Task;
use futures::FutureExt as _;
use std::{
    future::Future,
    marker::PhantomData,
    pin::pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
    time::Duration,
};

pub struct ForegroundExecutor {
    session_id: SessionId,
    scheduler: Arc<dyn Scheduler>,
    not_send: PhantomData<Rc<()>>,
}

impl ForegroundExecutor {
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let session_id = self.session_id;
        let scheduler = Arc::clone(&self.scheduler);
        let (runnable, task) = async_task::spawn_local(future, move |runnable| {
            scheduler.schedule_foreground(session_id, runnable);
        });
        runnable.schedule();
        task
    }
}

impl ForegroundExecutor {
    pub fn new(session_id: SessionId, scheduler: Arc<dyn Scheduler>) -> Self {
        assert!(
            scheduler.is_main_thread(),
            "ForegroundExecutor must be created on the same thread as the Scheduler"
        );
        Self {
            session_id,
            scheduler,
            not_send: PhantomData,
        }
    }
}

impl BackgroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Self {
        Self { scheduler }
    }
}

pub struct BackgroundExecutor {
    scheduler: Arc<dyn Scheduler>,
}

impl BackgroundExecutor {
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let scheduler = Arc::clone(&self.scheduler);
        let (runnable, task) = async_task::spawn(future, move |runnable| {
            scheduler.schedule_background(runnable);
        });
        runnable.schedule();
        task
    }

    pub fn block<Fut: Future>(&self, future: Fut) -> Fut::Output {
        let (sender, receiver) = std::sync::mpsc::channel();
        let (runnable, mut task) = unsafe {
            async_task::spawn_unchecked(future, move |runnable| {
                sender.send(runnable).unwrap();
            })
        };

        self.scheduler.block(runnable);
        loop {
            while let Ok(runnable) = receiver.try_recv() {
                self.scheduler.block(runnable);
            }

            let unparker = self.scheduler.unparker();
            let waker = Waker::from(Arc::new(WakerFn::new(move || {
                unparker.unpark();
            })));
            let mut cx = Context::from_waker(&waker);
            match task.poll_unpin(&mut cx) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    self.scheduler.park(None);
                }
            }
        }
    }

    pub fn block_with_timeout<Fut: Unpin + Future>(
        &self,
        future: &mut Fut,
        timeout: Duration,
    ) -> Option<Fut::Output> {
        let mut timer = self.timer(timeout).fuse();
        let mut future = pin!(async { future.await }.fuse());
        self.block(async {
            futures::select_biased! {
                _ = timer => None,
                output = future => Some(output),
            }
        })
    }

    pub fn timer(&self, duration: Duration) -> Timer {
        self.scheduler.timer(duration)
    }
}

struct WakerFn<F> {
    f: F,
}

impl<F: Fn()> WakerFn<F> {
    fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F: Fn()> Wake for WakerFn<F> {
    fn wake(self: Arc<Self>) {
        (self.f)();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        (self.f)();
    }
}
