use anyhow::{anyhow, Result};
use async_task::Runnable;
pub use async_task::Task;
use backtrace::{Backtrace, BacktraceFmt, BytesOrWideString};
use parking_lot::Mutex;
use rand::prelude::*;
use smol::{channel, prelude::*, Executor};
use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem,
    pin::Pin,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        mpsc::Sender,
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
        _stop: channel::Sender<()>,
    },
}

struct DeterministicState {
    rng: StdRng,
    seed: u64,
    scheduled: Vec<(Runnable, Backtrace)>,
    spawned_from_foreground: Vec<(Runnable, Backtrace)>,
    waker: Option<Sender<()>>,
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
        let backtrace = Backtrace::new_unresolved();
        let scheduled_once = AtomicBool::new(false);
        let state = self.0.clone();
        let (runnable, task) = async_task::spawn_local(future, move |runnable| {
            let mut state = state.lock();
            let backtrace = backtrace.clone();
            if scheduled_once.fetch_or(true, SeqCst) {
                state.scheduled.push((runnable, backtrace));
            } else {
                state.spawned_from_foreground.push((runnable, backtrace));
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
        let backtrace = Backtrace::new_unresolved();
        let state = self.0.clone();
        let (runnable, task) = async_task::spawn(future, move |runnable| {
            let mut state = state.lock();
            state.scheduled.push((runnable, backtrace.clone()));
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
        let (wake_tx, wake_rx) = std::sync::mpsc::channel();
        let state = self.0.clone();
        state.lock().waker = Some(wake_tx);

        let (output_tx, output_rx) = std::sync::mpsc::channel();
        self.spawn_from_foreground(async move {
            let output = future.await;
            output_tx.send(output).unwrap();
        })
        .detach();

        let mut trace = Trace::default();
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
                    let (_, backtrace) = &state.scheduled[ix];
                    trace.record(&state, backtrace.clone());
                    state.scheduled.remove(ix).0
                } else {
                    let (_, backtrace) = &state.spawned_from_foreground[0];
                    trace.record(&state, backtrace.clone());
                    state.spawned_from_foreground.remove(0).0
                }
            };

            runnable.run();
        }
    }
}

#[derive(Default)]
struct Trace {
    executed: Vec<Backtrace>,
    scheduled: Vec<Vec<Backtrace>>,
    spawned_from_foreground: Vec<Vec<Backtrace>>,
}

impl Trace {
    fn record(&mut self, state: &DeterministicState, executed: Backtrace) {
        self.scheduled.push(
            state
                .scheduled
                .iter()
                .map(|(_, backtrace)| backtrace.clone())
                .collect(),
        );
        self.spawned_from_foreground.push(
            state
                .spawned_from_foreground
                .iter()
                .map(|(_, backtrace)| backtrace.clone())
                .collect(),
        );
        self.executed.push(executed);
    }

    fn resolve(&mut self) {
        for backtrace in &mut self.executed {
            backtrace.resolve();
        }

        for backtraces in &mut self.scheduled {
            for backtrace in backtraces {
                backtrace.resolve();
            }
        }

        for backtraces in &mut self.spawned_from_foreground {
            for backtrace in backtraces {
                backtrace.resolve();
            }
        }
    }
}

impl Debug for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct FirstCwdFrameInBacktrace<'a>(&'a Backtrace);

        impl<'a> Debug for FirstCwdFrameInBacktrace<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
                let cwd = std::env::current_dir().unwrap();
                let mut print_path = |fmt: &mut fmt::Formatter<'_>, path: BytesOrWideString<'_>| {
                    fmt::Display::fmt(&path, fmt)
                };
                let mut fmt = BacktraceFmt::new(f, backtrace::PrintFmt::Full, &mut print_path);
                for frame in self.0.frames() {
                    let mut formatted_frame = fmt.frame();
                    if frame
                        .symbols()
                        .iter()
                        .any(|s| s.filename().map_or(false, |f| f.starts_with(&cwd)))
                    {
                        formatted_frame.backtrace_frame(frame)?;
                        break;
                    }
                }
                fmt.finish()
            }
        }

        for ((backtrace, scheduled), spawned_from_foreground) in self
            .executed
            .iter()
            .zip(&self.scheduled)
            .zip(&self.spawned_from_foreground)
        {
            writeln!(f, "Scheduled")?;
            for backtrace in scheduled {
                writeln!(f, "- {:?}", FirstCwdFrameInBacktrace(backtrace))?;
            }
            if scheduled.is_empty() {
                writeln!(f, "None")?;
            }
            writeln!(f, "==========")?;

            writeln!(f, "Spawned from foreground")?;
            for backtrace in spawned_from_foreground {
                writeln!(f, "- {:?}", FirstCwdFrameInBacktrace(backtrace))?;
            }
            if spawned_from_foreground.is_empty() {
                writeln!(f, "None")?;
            }
            writeln!(f, "==========")?;

            writeln!(f, "Run: {:?}", FirstCwdFrameInBacktrace(backtrace))?;
            writeln!(f, "+++++++++++++++++++")?;
        }

        Ok(())
    }
}

impl Drop for Trace {
    fn drop(&mut self) {
        let trace_on_panic = if let Ok(trace_on_panic) = std::env::var("EXECUTOR_TRACE_ON_PANIC") {
            trace_on_panic == "1" || trace_on_panic == "true"
        } else {
            false
        };
        let trace_always = if let Ok(trace_always) = std::env::var("EXECUTOR_TRACE_ALWAYS") {
            trace_always == "1" || trace_always == "true"
        } else {
            false
        };

        if trace_always || (trace_on_panic && thread::panicking()) {
            self.resolve();
            dbg!(self);
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

        for i in 0..2 * num_cpus::get() {
            let executor = executor.clone();
            let stop = stop.1.clone();
            thread::Builder::new()
                .name(format!("background-executor-{}", i))
                .spawn(move || smol::block_on(executor.run(stop.recv())))
                .unwrap();
        }

        Self::Production {
            executor,
            _stop: stop.0,
        }
    }

    pub fn num_cpus(&self) -> usize {
        num_cpus::get()
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
