// #[cfg(not(test))]
use anyhow::{anyhow, Result};
use async_task::Runnable;
use smol::prelude::*;
use smol::{channel, Executor};
use std::rc::Rc;
use std::sync::Arc;
use std::{marker::PhantomData, thread};

use crate::platform;

pub enum Foreground {
    Platform {
        dispatcher: Arc<dyn platform::Dispatcher>,
        _not_send_or_sync: PhantomData<Rc<()>>,
    },
    Test(smol::LocalExecutor<'static>),
}

pub enum ForegroundTask<T> {
    Platform(async_task::Task<T>),
    Test(smol::Task<T>),
}

pub struct Background {
    executor: Arc<smol::Executor<'static>>,
    _stop: channel::Sender<()>,
}

pub type BackgroundTask<T> = smol::Task<T>;

impl Foreground {
    pub fn platform(dispatcher: Arc<dyn platform::Dispatcher>) -> Result<Self> {
        if dispatcher.is_main_thread() {
            Ok(Self::Platform {
                dispatcher,
                _not_send_or_sync: PhantomData,
            })
        } else {
            Err(anyhow!("must be constructed on main thread"))
        }
    }

    pub fn test() -> Self {
        Self::Test(smol::LocalExecutor::new())
    }

    pub fn spawn<T: 'static>(
        &self,
        future: impl Future<Output = T> + 'static,
    ) -> ForegroundTask<T> {
        match self {
            Self::Platform { dispatcher, .. } => {
                let dispatcher = dispatcher.clone();
                let schedule = move |runnable: Runnable| dispatcher.run_on_main_thread(runnable);
                let (runnable, task) = async_task::spawn_local(future, schedule);
                runnable.schedule();
                ForegroundTask::Platform(task)
            }
            Self::Test(executor) => ForegroundTask::Test(executor.spawn(future)),
        }
    }

    pub async fn run<T>(&self, future: impl Future<Output = T>) -> T {
        match self {
            Self::Platform { .. } => panic!("you can't call run on a platform foreground executor"),
            Self::Test(executor) => executor.run(future).await,
        }
    }
}

impl<T> ForegroundTask<T> {
    pub fn detach(self) {
        match self {
            Self::Platform(task) => task.detach(),
            Self::Test(task) => task.detach(),
        }
    }

    pub async fn cancel(self) -> Option<T> {
        match self {
            Self::Platform(task) => task.cancel().await,
            Self::Test(task) => task.cancel().await,
        }
    }
}

impl Background {
    pub fn new() -> Self {
        let executor = Arc::new(Executor::new());
        let stop = channel::unbounded::<()>();

        for i in 0..num_cpus::get() {
            let executor = executor.clone();
            let stop = stop.1.clone();
            thread::Builder::new()
                .name(format!("background-executor-{}", i))
                .spawn(move || smol::block_on(executor.run(stop.recv())))
                .unwrap();
        }

        Self {
            executor,
            _stop: stop.0,
        }
    }

    pub fn spawn<T>(&self, future: impl Send + Future<Output = T> + 'static) -> BackgroundTask<T>
    where
        T: 'static + Send,
    {
        self.executor.spawn(future)
    }
}
