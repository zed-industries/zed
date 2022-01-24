use anyhow::{anyhow, Result};
use async_task::Runnable;
use backtrace::{Backtrace, BacktraceFmt, BytesOrWideString};
use parking_lot::Mutex;
use postage::{barrier, prelude::Stream as _};
use rand::prelude::*;
use smol::{channel, prelude::*, Executor, Timer};
use std::{
    any::Any,
    fmt::{self, Debug, Display},
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
    time::{Duration, Instant},
};
use waker_fn::waker_fn;

use crate::{
    platform::{self, Dispatcher},
    util, MutableAppContext,
};

pub enum Foreground {
    Platform {
        dispatcher: Arc<dyn platform::Dispatcher>,
        _not_send_or_sync: PhantomData<Rc<()>>,
    },
    Deterministic(Arc<Deterministic>),
}

pub enum Background {
    Deterministic {
        executor: Arc<Deterministic>,
    },
    Production {
        executor: Arc<smol::Executor<'static>>,
        _stop: channel::Sender<()>,
    },
}

type AnyLocalFuture = Pin<Box<dyn 'static + Future<Output = Box<dyn Any + 'static>>>>;
type AnyFuture = Pin<Box<dyn 'static + Send + Future<Output = Box<dyn Any + Send + 'static>>>>;
type AnyTask = async_task::Task<Box<dyn Any + Send + 'static>>;
type AnyLocalTask = async_task::Task<Box<dyn Any + 'static>>;

#[must_use]
pub enum Task<T> {
    Ready(Option<T>),
    Local {
        any_task: AnyLocalTask,
        result_type: PhantomData<T>,
    },
    Send {
        any_task: AnyTask,
        result_type: PhantomData<T>,
    },
}

unsafe impl<T: Send> Send for Task<T> {}

struct DeterministicState {
    rng: StdRng,
    seed: u64,
    scheduled_from_foreground: Vec<(Runnable, Backtrace)>,
    scheduled_from_background: Vec<(Runnable, Backtrace)>,
    spawned_from_foreground: Vec<(Runnable, Backtrace)>,
    forbid_parking: bool,
    block_on_ticks: RangeInclusive<usize>,
    now: Instant,
    pending_timers: Vec<(Instant, barrier::Sender)>,
    waiting_backtrace: Option<Backtrace>,
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
                now: Instant::now(),
                pending_timers: Default::default(),
                waiting_backtrace: None,
            })),
            parker: Default::default(),
        }
    }

    fn spawn_from_foreground(&self, future: AnyLocalFuture) -> AnyLocalTask {
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

    fn spawn(&self, future: AnyFuture) -> AnyTask {
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

    fn run(&self, mut future: AnyLocalFuture) -> Box<dyn Any> {
        let woken = Arc::new(AtomicBool::new(false));
        loop {
            if let Some(result) = self.run_internal(woken.clone(), &mut future) {
                return result;
            }

            if !woken.load(SeqCst) {
                self.state.lock().will_park();
            }

            woken.store(false, SeqCst);
            self.parker.lock().park();
        }
    }

    fn run_until_parked(&self) {
        let woken = Arc::new(AtomicBool::new(false));
        let mut future = any_local_future(std::future::pending::<()>());
        self.run_internal(woken, &mut future);
    }

    fn run_internal(
        &self,
        woken: Arc<AtomicBool>,
        future: &mut AnyLocalFuture,
    ) -> Option<Box<dyn Any>> {
        let unparker = self.parker.lock().unparker();
        let waker = waker_fn(move || {
            woken.store(true, SeqCst);
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
                if let Poll::Ready(result) = future.poll(&mut cx) {
                    return Some(result);
                }

                let state = self.state.lock();

                if state.scheduled_from_foreground.is_empty()
                    && state.scheduled_from_background.is_empty()
                    && state.spawned_from_foreground.is_empty()
                {
                    return None;
                }
            }
        }
    }

    fn block_on(&self, future: &mut AnyLocalFuture) -> Option<Box<dyn Any>> {
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
                let mut state = self.state.lock();
                if state.scheduled_from_background.is_empty() {
                    state.will_park();
                    drop(state);
                    self.parker.lock().park();
                }

                continue;
            }
        }

        None
    }
}

impl DeterministicState {
    fn will_park(&mut self) {
        if self.forbid_parking {
            let mut backtrace_message = String::new();
            if let Some(backtrace) = self.waiting_backtrace.as_mut() {
                backtrace.resolve();
                backtrace_message = format!(
                    "\nbacktrace of waiting future:\n{:?}",
                    CwdBacktrace::new(backtrace)
                );
            }

            panic!(
                "deterministic executor parked after a call to forbid_parking{}",
                backtrace_message
            );
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

struct CwdBacktrace<'a> {
    backtrace: &'a Backtrace,
    first_frame_only: bool,
}

impl<'a> CwdBacktrace<'a> {
    fn new(backtrace: &'a Backtrace) -> Self {
        Self {
            backtrace,
            first_frame_only: false,
        }
    }

    fn first_frame(backtrace: &'a Backtrace) -> Self {
        Self {
            backtrace,
            first_frame_only: true,
        }
    }
}

impl<'a> Debug for CwdBacktrace<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let cwd = std::env::current_dir().unwrap();
        let mut print_path = |fmt: &mut fmt::Formatter<'_>, path: BytesOrWideString<'_>| {
            fmt::Display::fmt(&path, fmt)
        };
        let mut fmt = BacktraceFmt::new(f, backtrace::PrintFmt::Full, &mut print_path);
        for frame in self.backtrace.frames() {
            let mut formatted_frame = fmt.frame();
            if frame
                .symbols()
                .iter()
                .any(|s| s.filename().map_or(false, |f| f.starts_with(&cwd)))
            {
                formatted_frame.backtrace_frame(frame)?;
                if self.first_frame_only {
                    break;
                }
            }
        }
        fmt.finish()
    }
}

impl Debug for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ((backtrace, scheduled), spawned_from_foreground) in self
            .executed
            .iter()
            .zip(&self.scheduled)
            .zip(&self.spawned_from_foreground)
        {
            writeln!(f, "Scheduled")?;
            for backtrace in scheduled {
                writeln!(f, "- {:?}", CwdBacktrace::first_frame(backtrace))?;
            }
            if scheduled.is_empty() {
                writeln!(f, "None")?;
            }
            writeln!(f, "==========")?;

            writeln!(f, "Spawned from foreground")?;
            for backtrace in spawned_from_foreground {
                writeln!(f, "- {:?}", CwdBacktrace::first_frame(backtrace))?;
            }
            if spawned_from_foreground.is_empty() {
                writeln!(f, "None")?;
            }
            writeln!(f, "==========")?;

            writeln!(f, "Run: {:?}", CwdBacktrace::first_frame(backtrace))?;
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

    pub fn spawn<T: 'static>(&self, future: impl Future<Output = T> + 'static) -> Task<T> {
        let future = any_local_future(future);
        let any_task = match self {
            Self::Deterministic(executor) => executor.spawn_from_foreground(future),
            Self::Platform { dispatcher, .. } => {
                fn spawn_inner(
                    future: AnyLocalFuture,
                    dispatcher: &Arc<dyn Dispatcher>,
                ) -> AnyLocalTask {
                    let dispatcher = dispatcher.clone();
                    let schedule =
                        move |runnable: Runnable| dispatcher.run_on_main_thread(runnable);
                    let (runnable, task) = async_task::spawn_local(future, schedule);
                    runnable.schedule();
                    task
                }
                spawn_inner(future, dispatcher)
            }
        };
        Task::local(any_task)
    }

    pub fn run<T: 'static>(&self, future: impl 'static + Future<Output = T>) -> T {
        let future = any_local_future(future);
        let any_value = match self {
            Self::Deterministic(executor) => executor.run(future),
            Self::Platform { .. } => panic!("you can't call run on a platform foreground executor"),
        };
        *any_value.downcast().unwrap()
    }

    pub fn parking_forbidden(&self) -> bool {
        match self {
            Self::Deterministic(executor) => executor.state.lock().forbid_parking,
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    pub fn start_waiting(&self) {
        match self {
            Self::Deterministic(executor) => {
                executor.state.lock().waiting_backtrace = Some(Backtrace::new_unresolved());
            }
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    pub fn finish_waiting(&self) {
        match self {
            Self::Deterministic(executor) => {
                executor.state.lock().waiting_backtrace.take();
            }
            _ => panic!("this method can only be called on a deterministic executor"),
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

    pub async fn timer(&self, duration: Duration) {
        match self {
            Self::Deterministic(executor) => {
                let (tx, mut rx) = barrier::channel();
                {
                    let mut state = executor.state.lock();
                    let wakeup_at = state.now + duration;
                    state.pending_timers.push((wakeup_at, tx));
                }
                rx.recv().await;
            }
            _ => {
                Timer::after(duration).await;
            }
        }
    }

    pub fn advance_clock(&self, duration: Duration) {
        match self {
            Self::Deterministic(executor) => {
                executor.run_until_parked();

                let mut state = executor.state.lock();
                state.now += duration;
                let now = state.now;
                let mut pending_timers = mem::take(&mut state.pending_timers);
                drop(state);

                pending_timers.retain(|(wakeup, _)| *wakeup > now);
                executor.state.lock().pending_timers.extend(pending_timers);
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
        let future = any_future(future);
        let any_task = match self {
            Self::Production { executor, .. } => executor.spawn(future),
            Self::Deterministic { executor, .. } => executor.spawn(future),
        };
        Task::send(any_task)
    }

    pub fn block_with_timeout<F, T>(
        &self,
        timeout: Duration,
        future: F,
    ) -> Result<T, impl Future<Output = T>>
    where
        T: 'static,
        F: 'static + Unpin + Future<Output = T>,
    {
        let mut future = any_local_future(future);
        if !timeout.is_zero() {
            let output = match self {
                Self::Production { .. } => smol::block_on(util::timeout(timeout, &mut future)).ok(),
                Self::Deterministic { executor, .. } => executor.block_on(&mut future),
            };
            if let Some(output) = output {
                return Ok(*output.downcast().unwrap());
            }
        }
        Err(async { *future.await.downcast().unwrap() })
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
        Arc::new(Background::Deterministic { executor }),
    )
}

impl<T> Task<T> {
    pub fn ready(value: T) -> Self {
        Self::Ready(Some(value))
    }

    fn local(any_task: AnyLocalTask) -> Self {
        Self::Local {
            any_task,
            result_type: PhantomData,
        }
    }

    pub fn detach(self) {
        match self {
            Task::Ready(_) => {}
            Task::Local { any_task, .. } => any_task.detach(),
            Task::Send { any_task, .. } => any_task.detach(),
        }
    }
}

impl<T: 'static, E: 'static + Display> Task<Result<T, E>> {
    pub fn detach_and_log_err(self, cx: &mut MutableAppContext) {
        cx.spawn(|_| async move {
            if let Err(err) = self.await {
                log::error!("{}", err);
            }
        })
        .detach();
    }
}

impl<T: Send> Task<T> {
    fn send(any_task: AnyTask) -> Self {
        Self::Send {
            any_task,
            result_type: PhantomData,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Task<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Task::Ready(value) => value.fmt(f),
            Task::Local { any_task, .. } => any_task.fmt(f),
            Task::Send { any_task, .. } => any_task.fmt(f),
        }
    }
}

impl<T: 'static> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match unsafe { self.get_unchecked_mut() } {
            Task::Ready(value) => Poll::Ready(value.take().unwrap()),
            Task::Local { any_task, .. } => {
                any_task.poll(cx).map(|value| *value.downcast().unwrap())
            }
            Task::Send { any_task, .. } => {
                any_task.poll(cx).map(|value| *value.downcast().unwrap())
            }
        }
    }
}

fn any_future<T, F>(future: F) -> AnyFuture
where
    T: 'static + Send,
    F: Future<Output = T> + Send + 'static,
{
    async { Box::new(future.await) as Box<dyn Any + Send> }.boxed()
}

fn any_local_future<T, F>(future: F) -> AnyLocalFuture
where
    T: 'static,
    F: Future<Output = T> + 'static,
{
    async { Box::new(future.await) as Box<dyn Any> }.boxed_local()
}
