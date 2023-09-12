use anyhow::{anyhow, Result};
use async_task::Runnable;
use futures::channel::mpsc;
use smol::{channel, prelude::*, Executor};
use std::{
    any::Any,
    fmt::{self, Display},
    marker::PhantomData,
    mem,
    panic::Location,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll},
    thread,
    time::Duration,
};

use crate::{
    platform::{self, Dispatcher},
    util, AppContext,
};

pub enum Foreground {
    Platform {
        dispatcher: Arc<dyn platform::Dispatcher>,
        _not_send_or_sync: PhantomData<Rc<()>>,
    },
    #[cfg(any(test, feature = "test-support"))]
    Deterministic {
        cx_id: usize,
        executor: Arc<Deterministic>,
    },
}

pub enum Background {
    #[cfg(any(test, feature = "test-support"))]
    Deterministic { executor: Arc<Deterministic> },
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

#[cfg(any(test, feature = "test-support"))]
struct DeterministicState {
    rng: rand::prelude::StdRng,
    seed: u64,
    scheduled_from_foreground: collections::HashMap<usize, Vec<ForegroundRunnable>>,
    scheduled_from_background: Vec<BackgroundRunnable>,
    forbid_parking: bool,
    block_on_ticks: std::ops::RangeInclusive<usize>,
    now: std::time::Instant,
    next_timer_id: usize,
    pending_timers: Vec<(usize, std::time::Instant, postage::barrier::Sender)>,
    waiting_backtrace: Option<backtrace::Backtrace>,
    next_runnable_id: usize,
    poll_history: Vec<ExecutorEvent>,
    previous_poll_history: Option<Vec<ExecutorEvent>>,
    enable_runnable_backtraces: bool,
    runnable_backtraces: collections::HashMap<usize, backtrace::Backtrace>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExecutorEvent {
    PollRunnable { id: usize },
    EnqueuRunnable { id: usize },
}

#[cfg(any(test, feature = "test-support"))]
struct ForegroundRunnable {
    id: usize,
    runnable: Runnable,
    main: bool,
}

#[cfg(any(test, feature = "test-support"))]
struct BackgroundRunnable {
    id: usize,
    runnable: Runnable,
}

#[cfg(any(test, feature = "test-support"))]
pub struct Deterministic {
    state: Arc<parking_lot::Mutex<DeterministicState>>,
    parker: parking_lot::Mutex<parking::Parker>,
}

#[must_use]
pub enum Timer {
    Production(smol::Timer),
    #[cfg(any(test, feature = "test-support"))]
    Deterministic(DeterministicTimer),
}

#[cfg(any(test, feature = "test-support"))]
pub struct DeterministicTimer {
    rx: postage::barrier::Receiver,
    id: usize,
    state: Arc<parking_lot::Mutex<DeterministicState>>,
}

#[cfg(any(test, feature = "test-support"))]
impl Deterministic {
    pub fn new(seed: u64) -> Arc<Self> {
        use rand::prelude::*;

        Arc::new(Self {
            state: Arc::new(parking_lot::Mutex::new(DeterministicState {
                rng: StdRng::seed_from_u64(seed),
                seed,
                scheduled_from_foreground: Default::default(),
                scheduled_from_background: Default::default(),
                forbid_parking: false,
                block_on_ticks: 0..=1000,
                now: std::time::Instant::now(),
                next_timer_id: Default::default(),
                pending_timers: Default::default(),
                waiting_backtrace: None,
                next_runnable_id: 0,
                poll_history: Default::default(),
                previous_poll_history: Default::default(),
                enable_runnable_backtraces: false,
                runnable_backtraces: Default::default(),
            })),
            parker: Default::default(),
        })
    }

    pub fn execution_history(&self) -> Vec<ExecutorEvent> {
        self.state.lock().poll_history.clone()
    }

    pub fn set_previous_execution_history(&self, history: Option<Vec<ExecutorEvent>>) {
        self.state.lock().previous_poll_history = history;
    }

    pub fn enable_runnable_backtrace(&self) {
        self.state.lock().enable_runnable_backtraces = true;
    }

    pub fn runnable_backtrace(&self, runnable_id: usize) -> backtrace::Backtrace {
        let mut backtrace = self.state.lock().runnable_backtraces[&runnable_id].clone();
        backtrace.resolve();
        backtrace
    }

    pub fn build_background(self: &Arc<Self>) -> Arc<Background> {
        Arc::new(Background::Deterministic {
            executor: self.clone(),
        })
    }

    pub fn build_foreground(self: &Arc<Self>, id: usize) -> Rc<Foreground> {
        Rc::new(Foreground::Deterministic {
            cx_id: id,
            executor: self.clone(),
        })
    }

    fn spawn_from_foreground(
        &self,
        cx_id: usize,
        future: AnyLocalFuture,
        main: bool,
    ) -> AnyLocalTask {
        let state = self.state.clone();
        let id;
        {
            let mut state = state.lock();
            id = util::post_inc(&mut state.next_runnable_id);
            if state.enable_runnable_backtraces {
                state
                    .runnable_backtraces
                    .insert(id, backtrace::Backtrace::new_unresolved());
            }
        }

        let unparker = self.parker.lock().unparker();
        let (runnable, task) = async_task::spawn_local(future, move |runnable| {
            let mut state = state.lock();
            state.push_to_history(ExecutorEvent::EnqueuRunnable { id });
            state
                .scheduled_from_foreground
                .entry(cx_id)
                .or_default()
                .push(ForegroundRunnable { id, runnable, main });
            unparker.unpark();
        });
        runnable.schedule();
        task
    }

    fn spawn(&self, future: AnyFuture) -> AnyTask {
        let state = self.state.clone();
        let id;
        {
            let mut state = state.lock();
            id = util::post_inc(&mut state.next_runnable_id);
            if state.enable_runnable_backtraces {
                state
                    .runnable_backtraces
                    .insert(id, backtrace::Backtrace::new_unresolved());
            }
        }

        let unparker = self.parker.lock().unparker();
        let (runnable, task) = async_task::spawn(future, move |runnable| {
            let mut state = state.lock();
            state
                .poll_history
                .push(ExecutorEvent::EnqueuRunnable { id });
            state
                .scheduled_from_background
                .push(BackgroundRunnable { id, runnable });
            unparker.unpark();
        });
        runnable.schedule();
        task
    }

    fn run<'a>(
        &self,
        cx_id: usize,
        main_future: Pin<Box<dyn 'a + Future<Output = Box<dyn Any>>>>,
    ) -> Box<dyn Any> {
        use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

        let woken = Arc::new(AtomicBool::new(false));

        let state = self.state.clone();
        let id;
        {
            let mut state = state.lock();
            id = util::post_inc(&mut state.next_runnable_id);
            if state.enable_runnable_backtraces {
                state
                    .runnable_backtraces
                    .insert(id, backtrace::Backtrace::new_unresolved());
            }
        }

        let unparker = self.parker.lock().unparker();
        let (runnable, mut main_task) = unsafe {
            async_task::spawn_unchecked(main_future, move |runnable| {
                let state = &mut *state.lock();
                state
                    .scheduled_from_foreground
                    .entry(cx_id)
                    .or_default()
                    .push(ForegroundRunnable {
                        id: util::post_inc(&mut state.next_runnable_id),
                        runnable,
                        main: true,
                    });
                unparker.unpark();
            })
        };
        runnable.schedule();

        loop {
            if let Some(result) = self.run_internal(woken.clone(), Some(&mut main_task)) {
                return result;
            }

            if !woken.load(SeqCst) {
                self.state.lock().will_park();
            }

            woken.store(false, SeqCst);
            self.parker.lock().park();
        }
    }

    pub fn run_until_parked(&self) {
        use std::sync::atomic::AtomicBool;
        let woken = Arc::new(AtomicBool::new(false));
        self.run_internal(woken, None);
    }

    fn run_internal(
        &self,
        woken: Arc<std::sync::atomic::AtomicBool>,
        mut main_task: Option<&mut AnyLocalTask>,
    ) -> Option<Box<dyn Any>> {
        use rand::prelude::*;
        use std::sync::atomic::Ordering::SeqCst;

        let unparker = self.parker.lock().unparker();
        let waker = waker_fn::waker_fn(move || {
            woken.store(true, SeqCst);
            unparker.unpark();
        });

        let mut cx = Context::from_waker(&waker);
        loop {
            let mut state = self.state.lock();

            if state.scheduled_from_foreground.is_empty()
                && state.scheduled_from_background.is_empty()
            {
                if let Some(main_task) = main_task {
                    if let Poll::Ready(result) = main_task.poll(&mut cx) {
                        return Some(result);
                    }
                }

                return None;
            }

            if !state.scheduled_from_background.is_empty() && state.rng.gen() {
                let background_len = state.scheduled_from_background.len();
                let ix = state.rng.gen_range(0..background_len);
                let background_runnable = state.scheduled_from_background.remove(ix);
                state.push_to_history(ExecutorEvent::PollRunnable {
                    id: background_runnable.id,
                });
                drop(state);
                background_runnable.runnable.run();
            } else if !state.scheduled_from_foreground.is_empty() {
                let available_cx_ids = state
                    .scheduled_from_foreground
                    .keys()
                    .copied()
                    .collect::<Vec<_>>();
                let cx_id_to_run = *available_cx_ids.iter().choose(&mut state.rng).unwrap();
                let scheduled_from_cx = state
                    .scheduled_from_foreground
                    .get_mut(&cx_id_to_run)
                    .unwrap();
                let foreground_runnable = scheduled_from_cx.remove(0);
                if scheduled_from_cx.is_empty() {
                    state.scheduled_from_foreground.remove(&cx_id_to_run);
                }
                state.push_to_history(ExecutorEvent::PollRunnable {
                    id: foreground_runnable.id,
                });

                drop(state);

                foreground_runnable.runnable.run();
                if let Some(main_task) = main_task.as_mut() {
                    if foreground_runnable.main {
                        if let Poll::Ready(result) = main_task.poll(&mut cx) {
                            return Some(result);
                        }
                    }
                }
            }
        }
    }

    fn block<F, T>(&self, future: &mut F, max_ticks: usize) -> Option<T>
    where
        F: Unpin + Future<Output = T>,
    {
        use rand::prelude::*;

        let unparker = self.parker.lock().unparker();
        let waker = waker_fn::waker_fn(move || {
            unparker.unpark();
        });

        let mut cx = Context::from_waker(&waker);
        for _ in 0..max_ticks {
            let mut state = self.state.lock();
            let runnable_count = state.scheduled_from_background.len();
            let ix = state.rng.gen_range(0..=runnable_count);
            if ix < state.scheduled_from_background.len() {
                let background_runnable = state.scheduled_from_background.remove(ix);
                state.push_to_history(ExecutorEvent::PollRunnable {
                    id: background_runnable.id,
                });
                drop(state);
                background_runnable.runnable.run();
            } else {
                drop(state);
                if let Poll::Ready(result) = future.poll(&mut cx) {
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

    pub fn timer(&self, duration: Duration) -> Timer {
        let (tx, rx) = postage::barrier::channel();
        let mut state = self.state.lock();
        let wakeup_at = state.now + duration;
        let id = util::post_inc(&mut state.next_timer_id);
        match state
            .pending_timers
            .binary_search_by_key(&wakeup_at, |e| e.1)
        {
            Ok(ix) | Err(ix) => state.pending_timers.insert(ix, (id, wakeup_at, tx)),
        }
        let state = self.state.clone();
        Timer::Deterministic(DeterministicTimer { rx, id, state })
    }

    pub fn now(&self) -> std::time::Instant {
        let state = self.state.lock();
        state.now
    }

    pub fn advance_clock(&self, duration: Duration) {
        let new_now = self.state.lock().now + duration;
        loop {
            self.run_until_parked();
            let mut state = self.state.lock();

            if let Some((_, wakeup_time, _)) = state.pending_timers.first() {
                let wakeup_time = *wakeup_time;
                if wakeup_time <= new_now {
                    let timer_count = state
                        .pending_timers
                        .iter()
                        .take_while(|(_, t, _)| *t == wakeup_time)
                        .count();
                    state.now = wakeup_time;
                    let timers_to_wake = state
                        .pending_timers
                        .drain(0..timer_count)
                        .collect::<Vec<_>>();
                    drop(state);
                    drop(timers_to_wake);
                    continue;
                }
            }

            break;
        }

        self.state.lock().now = new_now;
    }

    pub fn start_waiting(&self) {
        self.state.lock().waiting_backtrace = Some(backtrace::Backtrace::new_unresolved());
    }

    pub fn finish_waiting(&self) {
        self.state.lock().waiting_backtrace.take();
    }

    pub fn forbid_parking(&self) {
        use rand::prelude::*;

        let mut state = self.state.lock();
        state.forbid_parking = true;
        state.rng = StdRng::seed_from_u64(state.seed);
    }

    pub fn allow_parking(&self) {
        use rand::prelude::*;

        let mut state = self.state.lock();
        state.forbid_parking = false;
        state.rng = StdRng::seed_from_u64(state.seed);
    }

    pub async fn simulate_random_delay(&self) {
        use rand::prelude::*;
        use smol::future::yield_now;
        if self.state.lock().rng.gen_bool(0.2) {
            let yields = self.state.lock().rng.gen_range(1..=10);
            for _ in 0..yields {
                yield_now().await;
            }
        }
    }

    pub fn record_backtrace(&self) {
        let mut state = self.state.lock();
        if state.enable_runnable_backtraces {
            let current_id = state
                .poll_history
                .iter()
                .rev()
                .find_map(|event| match event {
                    ExecutorEvent::PollRunnable { id } => Some(*id),
                    _ => None,
                });
            if let Some(id) = current_id {
                state
                    .runnable_backtraces
                    .insert(id, backtrace::Backtrace::new_unresolved());
            }
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        #[cfg(any(test, feature = "test-support"))]
        if let Timer::Deterministic(DeterministicTimer { state, id, .. }) = self {
            state
                .lock()
                .pending_timers
                .retain(|(timer_id, _, _)| timer_id != id)
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &mut *self {
            #[cfg(any(test, feature = "test-support"))]
            Self::Deterministic(DeterministicTimer { rx, .. }) => {
                use postage::stream::{PollRecv, Stream as _};
                smol::pin!(rx);
                match rx.poll_recv(&mut postage::Context::from_waker(cx.waker())) {
                    PollRecv::Ready(()) | PollRecv::Closed => Poll::Ready(()),
                    PollRecv::Pending => Poll::Pending,
                }
            }
            Self::Production(timer) => {
                smol::pin!(timer);
                match timer.poll(cx) {
                    Poll::Ready(_) => Poll::Ready(()),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl DeterministicState {
    fn push_to_history(&mut self, event: ExecutorEvent) {
        use std::fmt::Write as _;

        self.poll_history.push(event);
        if let Some(prev_history) = &self.previous_poll_history {
            let ix = self.poll_history.len() - 1;
            let prev_event = prev_history[ix];
            if event != prev_event {
                let mut message = String::new();
                writeln!(
                    &mut message,
                    "current runnable backtrace:\n{:?}",
                    self.runnable_backtraces.get_mut(&event.id()).map(|trace| {
                        trace.resolve();
                        util::CwdBacktrace(trace)
                    })
                )
                .unwrap();
                writeln!(
                    &mut message,
                    "previous runnable backtrace:\n{:?}",
                    self.runnable_backtraces
                        .get_mut(&prev_event.id())
                        .map(|trace| {
                            trace.resolve();
                            util::CwdBacktrace(trace)
                        })
                )
                .unwrap();
                panic!("detected non-determinism after {ix}. {message}");
            }
        }
    }

    fn will_park(&mut self) {
        if self.forbid_parking {
            let mut backtrace_message = String::new();
            #[cfg(any(test, feature = "test-support"))]
            if let Some(backtrace) = self.waiting_backtrace.as_mut() {
                backtrace.resolve();
                backtrace_message = format!(
                    "\nbacktrace of waiting future:\n{:?}",
                    util::CwdBacktrace(backtrace)
                );
            }

            panic!(
                "deterministic executor parked after a call to forbid_parking{}",
                backtrace_message
            );
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl ExecutorEvent {
    pub fn id(&self) -> usize {
        match self {
            ExecutorEvent::PollRunnable { id } => *id,
            ExecutorEvent::EnqueuRunnable { id } => *id,
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
            #[cfg(any(test, feature = "test-support"))]
            Self::Deterministic { cx_id, executor } => {
                executor.spawn_from_foreground(*cx_id, future, false)
            }
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn run<T: 'static>(&self, future: impl Future<Output = T>) -> T {
        let future = async move { Box::new(future.await) as Box<dyn Any> }.boxed_local();
        let result = match self {
            Self::Deterministic { cx_id, executor } => executor.run(*cx_id, future),
            Self::Platform { .. } => panic!("you can't call run on a platform foreground executor"),
        };
        *result.downcast().unwrap()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn run_until_parked(&self) {
        match self {
            Self::Deterministic { executor, .. } => executor.run_until_parked(),
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn parking_forbidden(&self) -> bool {
        match self {
            Self::Deterministic { executor, .. } => executor.state.lock().forbid_parking,
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn start_waiting(&self) {
        match self {
            Self::Deterministic { executor, .. } => executor.start_waiting(),
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn finish_waiting(&self) {
        match self {
            Self::Deterministic { executor, .. } => executor.finish_waiting(),
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn forbid_parking(&self) {
        match self {
            Self::Deterministic { executor, .. } => executor.forbid_parking(),
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn allow_parking(&self) {
        match self {
            Self::Deterministic { executor, .. } => executor.allow_parking(),
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn advance_clock(&self, duration: Duration) {
        match self {
            Self::Deterministic { executor, .. } => executor.advance_clock(duration),
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_block_on_ticks(&self, range: std::ops::RangeInclusive<usize>) {
        match self {
            Self::Deterministic { executor, .. } => executor.state.lock().block_on_ticks = range,
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
            #[cfg(any(test, feature = "test-support"))]
            Self::Deterministic { executor } => executor.spawn(future),
        };
        Task::send(any_task)
    }

    pub fn block<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T>,
    {
        smol::pin!(future);
        match self {
            Self::Production { .. } => smol::block_on(&mut future),
            #[cfg(any(test, feature = "test-support"))]
            Self::Deterministic { executor, .. } => {
                executor.block(&mut future, usize::MAX).unwrap()
            }
        }
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
                #[cfg(any(test, feature = "test-support"))]
                Self::Deterministic { executor, .. } => {
                    use rand::prelude::*;
                    let max_ticks = {
                        let mut state = executor.state.lock();
                        let range = state.block_on_ticks.clone();
                        state.rng.gen_range(range)
                    };
                    executor.block(&mut future, max_ticks)
                }
            };
            if let Some(output) = output {
                return Ok(*output.downcast().unwrap());
            }
        }
        Err(async { *future.await.downcast().unwrap() })
    }

    pub async fn scoped<'scope, F>(self: &Arc<Self>, scheduler: F)
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

    pub fn timer(&self, duration: Duration) -> Timer {
        match self {
            Background::Production { .. } => Timer::Production(smol::Timer::after(duration)),
            #[cfg(any(test, feature = "test-support"))]
            Background::Deterministic { executor } => executor.timer(duration),
        }
    }

    pub fn now(&self) -> std::time::Instant {
        match self {
            Background::Production { .. } => std::time::Instant::now(),
            #[cfg(any(test, feature = "test-support"))]
            Background::Deterministic { executor } => executor.now(),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn rng<'a>(&'a self) -> impl 'a + std::ops::DerefMut<Target = rand::prelude::StdRng> {
        match self {
            Self::Deterministic { executor, .. } => {
                parking_lot::lock_api::MutexGuard::map(executor.state.lock(), |s| &mut s.rng)
            }
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn simulate_random_delay(&self) {
        match self {
            Self::Deterministic { executor, .. } => {
                executor.simulate_random_delay().await;
            }
            _ => {
                panic!("this method can only be called on a deterministic executor")
            }
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn record_backtrace(&self) {
        match self {
            Self::Deterministic { executor, .. } => executor.record_backtrace(),
            _ => {
                panic!("this method can only be called on a deterministic executor")
            }
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn start_waiting(&self) {
        match self {
            Self::Deterministic { executor, .. } => executor.start_waiting(),
            _ => panic!("this method can only be called on a deterministic executor"),
        }
    }
}

impl Default for Background {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Scope<'a> {
    executor: Arc<Background>,
    futures: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    tx: Option<mpsc::Sender<()>>,
    rx: mpsc::Receiver<()>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Scope<'a> {
    fn new(executor: Arc<Background>) -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self {
            executor,
            tx: Some(tx),
            rx,
            futures: Default::default(),
            _phantom: PhantomData,
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
    #[track_caller]
    pub fn detach_and_log_err(self, cx: &mut AppContext) {
        let caller = Location::caller();
        cx.spawn(|_| async move {
            if let Err(err) = self.await {
                log::error!("{}:{}: {:#}", caller.file(), caller.line(), err);
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
