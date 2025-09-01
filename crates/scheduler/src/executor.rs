use crate::{Scheduler, SessionId};
use async_task::Task;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

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
        let mut future = pin!(future);
        let waker = Waker::from(Arc::new(WakerFn::new({
            let unparker = self.scheduler.unparker();
            move || {
                unparker.unpark();
            }
        })));
        let mut cx = Context::from_waker(&waker);
        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    if self.scheduler.step() {
                        continue;
                    }
                    self.scheduler.park(None);
                }
            }
        }
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
