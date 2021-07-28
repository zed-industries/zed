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
    ops::RangeInclusive,
    pin::Pin,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    task::{Context, Poll},
    thread,
    time::Duration,
};
use waker_fn::waker_fn;

use crate::{platform, util};

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
    scheduled_from_foreground: Vec<(Runnable, Backtrace)>,
    scheduled_from_background: Vec<(Runnable, Backtrace)>,
    spawned_from_foreground: Vec<(Runnable, Backtrace)>,
    forbid_parking: bool,
    block_on_ticks: RangeInclusive<usize>,
}

pub struct Deterministic {
    state: Arc<Mutex<DeterministicState>>,
    parker: Mutex<parking::Parker>,
}

impl Deterministic {
    fn new(seed: u64) -> Self {
        Self {
            state: Arc::new(Mutex::new(DeterministicState {
                rng: StdRng::seed_from_u64(seed),
                seed,
                scheduled_from_foreground: Default::default(),
                scheduled_from_background: Default::default(),
                spawned_from_foreground: Default::default(),
                forbid_parking: false,
                block_on_ticks: 0..=1000,
            })),
            parker: Default::default(),
        }
    }

    pub fn spawn_from_foreground<F, T>(&self, future: F) -> Task<T>
    where
        T: 'static,
        F: Future<Output = T> + 'static,
    {
        let backtrace = Backtrace::new_unresolved();
        let scheduled_once = AtomicBool::new(false);
        let state = self.state.clone();
        let unparker = self.parker.lock().unparker();
        let (runnable, task) = async_task::spawn_local(future, move |runnable| {
            let mut state = state.lock();
            let backtrace = backtrace.clone();
            if scheduled_once.fetch_or(true, SeqCst) {
                state.scheduled_from_foreground.push((runnable, backtrace));
            } else {
                state.spawned_from_foreground.push((runnable, backtrace));
            }
            unparker.unpark();
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
        let state = self.state.clone();
        let unparker = self.parker.lock().unparker();
        let (runnable, task) = async_task::spawn(future, move |runnable| {
            let mut state = state.lock();
            state
                .scheduled_from_background
                .push((runnable, backtrace.clone()));
            unparker.unpark();
        });
        runnable.schedule();
        task
    }

    pub fn run<F, T>(&self, future: F) -> T
    where
        T: 'static,
        F: Future<Output = T> + 'static,
    {
        smol::pin!(future);

        let unparker = self.parker.lock().unparker();
        let waker = waker_fn(move || {
            unparker.unpark();
        });

        let mut cx = Context::from_waker(&waker);
        let mut trace = Trace::default();
        loop {
            let mut state = self.state.lock();
            let runnable_count = state.scheduled_from_foreground.len()
                + state.scheduled_from_background.len()
                + state.spawned_from_foreground.len();

            let ix = state.rng.gen_range(0..=runnable_count);
            if ix < state.scheduled_from_foreground.len() {
                let (_, backtrace) = &state.scheduled_from_foreground[ix];
                trace.record(&state, backtrace.clone());
                let runnable = state.scheduled_from_foreground.remove(ix).0;
                drop(state);
                runnable.run();
            } else if ix - state.scheduled_from_foreground.len()
                < state.scheduled_from_background.len()
            {
                let ix = ix - state.scheduled_from_foreground.len();
                let (_, backtrace) = &state.scheduled_from_background[ix];
                trace.record(&state, backtrace.clone());
                let runnable = state.scheduled_from_background.remove(ix).0;
                drop(state);
                runnable.run();
            } else if ix < runnable_count {
                let (_, backtrace) = &state.spawned_from_foreground[0];
                trace.record(&state, backtrace.clone());
                let runnable = state.spawned_from_foreground.remove(0).0;
                drop(state);
                runnable.run();
            } else {
                drop(state);
                if let Poll::Ready(result) = future.as_mut().poll(&mut cx) {
                    return result;
                }
                let state = self.state.lock();
                if state.scheduled_from_foreground.is_empty()
                    && state.scheduled_from_background.is_empty()
                    && state.spawned_from_foreground.is_empty()
                {
                    if state.forbid_parking {
                        panic!("deterministic executor parked after a call to forbid_parking");
                    }
                    drop(state);
                    self.parker.lock().park();
                }

                continue;
            }
        }
    }

    pub fn block_on<F, T>(&self, future: F) -> Option<T>
    where
        T: 'static,
        F: Future<Output = T>,
    {
        smol::pin!(future);

        let unparker = self.parker.lock().unparker();
        let waker = waker_fn(move || {
            unparker.unpark();
        });
        let max_ticks = {
            let mut state = self.state.lock();
            let range = state.block_on_ticks.clone();
            state.rng.gen_range(range)
        };

        let mut cx = Context::from_waker(&waker);
        let mut trace = Trace::default();
        for _ in 0..max_ticks {
            let mut state = self.state.lock();
            let runnable_count = state.scheduled_from_background.len();
            let ix = state.rng.gen_range(0..=runnable_count);
            if ix < state.scheduled_from_background.len() {
                let (_, backtrace) = &state.scheduled_from_background[ix];
                trace.record(&state, backtrace.clone());
                let runnable = state.scheduled_from_background.remove(ix).0;
                drop(state);
                runnable.run();
            } else {
                drop(state);
                if let Poll::Ready(result) = future.as_mut().poll(&mut cx) {
                    return Some(result);
                }
                let state = self.state.lock();
                if state.scheduled_from_background.is_empty() {
                    if state.forbid_parking {
                        panic!("deterministic executor parked after a call to forbid_parking");
                    }
                    drop(state);
                    self.parker.lock().park();
                }

                continue;
            }
        }

        None
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
                .scheduled_from_foreground
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

    pub fn forbid_parking(&self) {
        match self {
            Self::Deterministic(executor) => {
                let mut state = executor.state.lock();
                state.forbid_parking = true;
                state.rng = StdRng::seed_from_u64(state.seed);
            }
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    pub fn set_block_on_ticks(&self, range: RangeInclusive<usize>) {
        match self {
            Self::Deterministic(executor) => executor.state.lock().block_on_ticks = range,
            _ => panic!("this method can only be called on a deterministic executor"),
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

    pub fn block_with_timeout<F, T>(&self, timeout: Duration, mut future: F) -> Result<T, F>
    where
        T: 'static,
        F: 'static + Unpin + Future<Output = T>,
    {
        let output = match self {
            Self::Production { .. } => {
                smol::block_on(util::timeout(timeout, Pin::new(&mut future))).ok()
            }
            Self::Deterministic(executor) => executor.block_on(Pin::new(&mut future)),
        };

        if let Some(output) = output {
            Ok(output)
        } else {
            Err(future)
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
