use anyhow::{anyhow, Result};
use async_task::Runnable;
pub use async_task::Task;
use parking_lot::Mutex;
use rand::prelude::*;
use smol::{channel, prelude::*, Executor};
use std::{
    marker::PhantomData,
    mem,
    pin::Pin,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        mpsc::SyncSender,
        Arc,
    },
    thread,
};

use crate::platform;

pub enum Foreground {
    Platform {
        dispatcher: Arc<dyn platform::Dispatcher>,
        _not_send_or_sync: PhantomData<Rc<()>>,
    },
    Test(smol::LocalExecutor<'static>),
    Deterministic(Arc<Deterministic>),
}

pub enum Background {
    Deterministic(Arc<Deterministic>),
    Production {
        executor: Arc<smol::Executor<'static>>,
        threads: usize,
        _stop: channel::Sender<()>,
    },
}

struct DeterministicState {
    rng: StdRng,
    seed: u64,
    scheduled: Vec<Runnable>,
    spawned_from_foreground: Vec<Runnable>,
    waker: Option<SyncSender<()>>,
}

pub struct Deterministic(Arc<Mutex<DeterministicState>>);

impl Deterministic {
    fn new(seed: u64) -> Self {
        Self(Arc::new(Mutex::new(DeterministicState {
            rng: StdRng::seed_from_u64(seed),
            seed,
            scheduled: Default::default(),
            spawned_from_foreground: Default::default(),
            waker: None,
        })))
    }

    pub fn spawn_from_foreground<F, T>(&self, future: F) -> Task<T>
    where
        T: 'static,
        F: Future<Output = T> + 'static,
    {
        let scheduled_once = AtomicBool::new(false);
        let state = self.0.clone();
        let (runnable, task) = async_task::spawn_local(future, move |runnable| {
            let mut state = state.lock();
            if scheduled_once.fetch_or(true, SeqCst) {
                state.scheduled.push(runnable);
            } else {
                state.spawned_from_foreground.push(runnable);
            }
            if let Some(waker) = state.waker.as_ref() {
                waker.send(()).ok();
            }
        });
        runnable.schedule();
        task
    }

    pub fn spawn<F, T>(&self, future: F) -> Task<T>
    where
        T: 'static + Send,
        F: 'static + Send + Future<Output = T>,
    {
        let state = self.0.clone();
        let (runnable, task) = async_task::spawn(future, move |runnable| {
            let mut state = state.lock();
            state.scheduled.push(runnable);
            if let Some(waker) = state.waker.as_ref() {
                waker.send(()).ok();
            }
        });
        runnable.schedule();
        task
    }

    pub fn run<F, T>(&self, future: F) -> T
    where
        T: 'static,
        F: Future<Output = T> + 'static,
    {
        let (wake_tx, wake_rx) = std::sync::mpsc::sync_channel(32);
        let state = self.0.clone();
        state.lock().waker = Some(wake_tx);

        let (output_tx, output_rx) = std::sync::mpsc::channel();
        self.spawn_from_foreground(async move {
            let output = future.await;
            output_tx.send(output).unwrap();
        })
        .detach();

        loop {
            if let Ok(value) = output_rx.try_recv() {
                state.lock().waker = None;
                return value;
            }

            wake_rx.recv().unwrap();
            let runnable = {
                let state = &mut *state.lock();
                let ix = state
                    .rng
                    .gen_range(0..state.scheduled.len() + state.spawned_from_foreground.len());
                if ix < state.scheduled.len() {
                    state.scheduled.remove(ix)
                } else {
                    state.spawned_from_foreground.remove(0)
                }
            };

            runnable.run();
        }
    }
}

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

    pub fn spawn<T: 'static>(&self, future: impl Future<Output = T> + 'static) -> Task<T> {
        match self {
            Self::Platform { dispatcher, .. } => {
                let dispatcher = dispatcher.clone();
                let schedule = move |runnable: Runnable| dispatcher.run_on_main_thread(runnable);
                let (runnable, task) = async_task::spawn_local(future, schedule);
                runnable.schedule();
                task
            }
            Self::Test(executor) => executor.spawn(future),
            Self::Deterministic(executor) => executor.spawn_from_foreground(future),
        }
    }

    pub fn run<T: 'static>(&self, future: impl 'static + Future<Output = T>) -> T {
        match self {
            Self::Platform { .. } => panic!("you can't call run on a platform foreground executor"),
            Self::Test(executor) => smol::block_on(executor.run(future)),
            Self::Deterministic(executor) => executor.run(future),
        }
    }

    pub fn reset(&self) {
        match self {
            Self::Platform { .. } => panic!("can't call this method on a platform executor"),
            Self::Test(_) => panic!("can't call this method on a test executor"),
            Self::Deterministic(executor) => {
                let state = &mut *executor.0.lock();
                state.rng = StdRng::seed_from_u64(state.seed);
            }
        }
    }
}

impl Background {
    pub fn new() -> Self {
        let executor = Arc::new(Executor::new());
        let stop = channel::unbounded::<()>();
        let threads = num_cpus::get();

        for i in 0..threads {
            let executor = executor.clone();
            let stop = stop.1.clone();
            thread::Builder::new()
                .name(format!("background-executor-{}", i))
                .spawn(move || smol::block_on(executor.run(stop.recv())))
                .unwrap();
        }

        Self::Production {
            executor,
            threads,
            _stop: stop.0,
        }
    }

    pub fn threads(&self) -> usize {
        match self {
            Self::Deterministic(_) => 1,
            Self::Production { threads, .. } => *threads,
        }
    }

    pub fn spawn<T, F>(&self, future: F) -> Task<T>
    where
        T: 'static + Send,
        F: Send + Future<Output = T> + 'static,
    {
        match self {
            Self::Production { executor, .. } => executor.spawn(future),
            Self::Deterministic(executor) => executor.spawn(future),
        }
    }

    pub async fn scoped<'scope, F>(&self, scheduler: F)
    where
        F: FnOnce(&mut Scope<'scope>),
    {
        let mut scope = Scope {
            futures: Default::default(),
            _phantom: PhantomData,
        };
        (scheduler)(&mut scope);
        let spawned = scope
            .futures
            .into_iter()
            .map(|f| self.spawn(f))
            .collect::<Vec<_>>();
        for task in spawned {
            task.await;
        }
    }
}

pub struct Scope<'a> {
    futures: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Scope<'a> {
    pub fn spawn<F>(&mut self, f: F)
    where
        F: Future<Output = ()> + Send + 'a,
    {
        let f = unsafe {
            mem::transmute::<
                Pin<Box<dyn Future<Output = ()> + Send + 'a>>,
                Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
            >(Box::pin(f))
        };
        self.futures.push(f);
    }
}

pub fn deterministic(seed: u64) -> (Rc<Foreground>, Arc<Background>) {
    let executor = Arc::new(Deterministic::new(seed));
    (
        Rc::new(Foreground::Deterministic(executor.clone())),
        Arc::new(Background::Deterministic(executor)),
    )
}
