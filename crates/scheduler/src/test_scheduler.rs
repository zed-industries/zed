use crate::{
    BackgroundExecutor, Clock, ForegroundExecutor, Scheduler, SessionId, TestClock, Timer,
};
use async_task::Runnable;
use backtrace::{Backtrace, BacktraceFrame};
use futures::{FutureExt as _, channel::oneshot, future::LocalBoxFuture};
use parking_lot::Mutex;
use rand::prelude::*;
use std::{
    any::type_name_of_val,
    collections::{BTreeMap, VecDeque},
    env,
    fmt::Write,
    future::Future,
    mem,
    ops::RangeInclusive,
    panic::{self, AssertUnwindSafe},
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    thread::{self, Thread},
    time::{Duration, Instant},
};

const PENDING_TRACES_VAR_NAME: &str = "PENDING_TRACES";

pub struct TestScheduler {
    clock: Arc<TestClock>,
    rng: Arc<Mutex<StdRng>>,
    state: Arc<Mutex<SchedulerState>>,
    thread: Thread,
}

impl TestScheduler {
    /// Run a test once with default configuration (seed 0)
    pub fn once<R>(f: impl AsyncFnOnce(Arc<TestScheduler>) -> R) -> R {
        Self::with_seed(0, f)
    }

    /// Run a test multiple times with sequential seeds (0, 1, 2, ...)
    pub fn many<R>(iterations: usize, mut f: impl AsyncFnMut(Arc<TestScheduler>) -> R) -> Vec<R> {
        (0..iterations as u64)
            .map(|seed| {
                let mut unwind_safe_f = AssertUnwindSafe(&mut f);
                match panic::catch_unwind(move || Self::with_seed(seed, &mut *unwind_safe_f)) {
                    Ok(result) => result,
                    Err(error) => {
                        eprintln!("Failing Seed: {seed}");
                        panic::resume_unwind(error);
                    }
                }
            })
            .collect()
    }

    /// Run a test once with a specific seed
    pub fn with_seed<R>(seed: u64, f: impl AsyncFnOnce(Arc<TestScheduler>) -> R) -> R {
        let scheduler = Arc::new(TestScheduler::new(TestSchedulerConfig::with_seed(seed)));
        let future = f(scheduler.clone());
        let result = scheduler.foreground().block_on(future);
        scheduler.run(); // Ensure spawned tasks finish up before returning in tests
        result
    }

    pub fn new(config: TestSchedulerConfig) -> Self {
        Self {
            rng: Arc::new(Mutex::new(StdRng::seed_from_u64(config.seed))),
            state: Arc::new(Mutex::new(SchedulerState {
                runnables: VecDeque::new(),
                timers: Vec::new(),
                blocked_sessions: Vec::new(),
                randomize_order: config.randomize_order,
                allow_parking: config.allow_parking,
                timeout_ticks: config.timeout_ticks,
                next_session_id: SessionId(0),
                capture_pending_traces: config.capture_pending_traces,
                pending_traces: BTreeMap::new(),
                next_trace_id: TraceId(0),
            })),
            clock: Arc::new(TestClock::new()),
            thread: thread::current(),
        }
    }

    pub fn clock(&self) -> Arc<TestClock> {
        self.clock.clone()
    }

    pub fn rng(&self) -> Arc<Mutex<StdRng>> {
        self.rng.clone()
    }

    pub fn set_timeout_ticks(&self, timeout_ticks: RangeInclusive<usize>) {
        self.state.lock().timeout_ticks = timeout_ticks;
    }

    pub fn allow_parking(&self) {
        self.state.lock().allow_parking = true;
    }

    pub fn forbid_parking(&self) {
        self.state.lock().allow_parking = false;
    }

    /// Create a foreground executor for this scheduler
    pub fn foreground(self: &Arc<Self>) -> ForegroundExecutor {
        let session_id = {
            let mut state = self.state.lock();
            state.next_session_id.0 += 1;
            state.next_session_id
        };
        ForegroundExecutor::new(session_id, self.clone())
    }

    /// Create a background executor for this scheduler
    pub fn background(self: &Arc<Self>) -> BackgroundExecutor {
        BackgroundExecutor::new(self.clone())
    }

    pub fn yield_random(&self) -> Yield {
        let rng = &mut *self.rng.lock();
        if rng.random_bool(0.1) {
            Yield(rng.random_range(10..20))
        } else {
            Yield(rng.random_range(0..2))
        }
    }

    pub fn run(&self) {
        while self.step() {
            // Continue until no work remains
        }
    }

    pub fn run_with_clock_advancement(&self) {
        while self.step() || self.advance_clock_to_next_timer() {
            // Continue until no work remains
        }
    }

    fn step(&self) -> bool {
        let elapsed_timers = {
            let mut state = self.state.lock();
            let end_ix = state
                .timers
                .partition_point(|timer| timer.expiration <= self.clock.now());
            state.timers.drain(..end_ix).collect::<Vec<_>>()
        };

        if !elapsed_timers.is_empty() {
            return true;
        }

        let runnable = {
            let state = &mut *self.state.lock();
            let ix = state.runnables.iter().position(|runnable| {
                runnable
                    .session_id
                    .is_none_or(|session_id| !state.blocked_sessions.contains(&session_id))
            });
            ix.and_then(|ix| state.runnables.remove(ix))
        };

        if let Some(runnable) = runnable {
            runnable.run();
            return true;
        }

        false
    }

    fn advance_clock_to_next_timer(&self) -> bool {
        if let Some(timer) = self.state.lock().timers.first() {
            self.clock.advance(timer.expiration - self.clock.now());
            true
        } else {
            false
        }
    }

    pub fn advance_clock(&self, duration: Duration) {
        let next_now = self.clock.now() + duration;
        loop {
            self.run();
            if let Some(timer) = self.state.lock().timers.first()
                && timer.expiration <= next_now
            {
                self.clock.advance(timer.expiration - self.clock.now());
            } else {
                break;
            }
        }
        self.clock.advance(next_now - self.clock.now());
    }

    fn park(&self, deadline: Option<Instant>) -> bool {
        if self.state.lock().allow_parking {
            if let Some(deadline) = deadline {
                let now = Instant::now();
                let timeout = deadline.saturating_duration_since(now);
                thread::park_timeout(timeout);
                now.elapsed() < timeout
            } else {
                thread::park();
                true
            }
        } else if deadline.is_some() {
            false
        } else if self.state.lock().capture_pending_traces {
            let mut pending_traces = String::new();
            for (_, trace) in mem::take(&mut self.state.lock().pending_traces) {
                writeln!(pending_traces, "{:?}", exclude_wakers_from_trace(trace)).unwrap();
            }
            panic!("Parking forbidden. Pending traces:\n{}", pending_traces);
        } else {
            panic!(
                "Parking forbidden. Re-run with {PENDING_TRACES_VAR_NAME}=1 to show pending traces"
            );
        }
    }
}

impl Scheduler for TestScheduler {
    /// Block until the given future completes, with an optional timeout. If the
    /// future is unable to make progress at any moment before the timeout and
    /// no other tasks or timers remain, we panic unless parking is allowed. If
    /// parking is allowed, we block up to the timeout or indefinitely if none
    /// is provided. This is to allow testing a mix of deterministic and
    /// non-deterministic async behavior, such as when interacting with I/O in
    /// an otherwise deterministic test.
    fn block(
        &self,
        session_id: Option<SessionId>,
        mut future: LocalBoxFuture<()>,
        timeout: Option<Duration>,
    ) {
        if let Some(session_id) = session_id {
            self.state.lock().blocked_sessions.push(session_id);
        }

        let deadline = timeout.map(|timeout| Instant::now() + timeout);
        let awoken = Arc::new(AtomicBool::new(false));
        let waker = Box::new(TracingWaker {
            id: None,
            awoken: awoken.clone(),
            thread: self.thread.clone(),
            state: self.state.clone(),
        });
        let waker = unsafe { Waker::new(Box::into_raw(waker) as *const (), &WAKER_VTABLE) };
        let max_ticks = if timeout.is_some() {
            self.rng
                .lock()
                .random_range(self.state.lock().timeout_ticks.clone())
        } else {
            usize::MAX
        };
        let mut cx = Context::from_waker(&waker);

        for _ in 0..max_ticks {
            let Poll::Pending = future.poll_unpin(&mut cx) else {
                break;
            };

            let mut stepped = None;
            while self.rng.lock().random() {
                let stepped = stepped.get_or_insert(false);
                if self.step() {
                    *stepped = true;
                } else {
                    break;
                }
            }

            let stepped = stepped.unwrap_or(true);
            let awoken = awoken.swap(false, SeqCst);
            if !stepped && !awoken && !self.advance_clock_to_next_timer() {
                if !self.park(deadline) {
                    break;
                }
            }
        }

        if session_id.is_some() {
            self.state.lock().blocked_sessions.pop();
        }
    }

    fn schedule_foreground(&self, session_id: SessionId, runnable: Runnable) {
        let mut state = self.state.lock();
        let ix = if state.randomize_order {
            let start_ix = state
                .runnables
                .iter()
                .rposition(|task| task.session_id == Some(session_id))
                .map_or(0, |ix| ix + 1);
            self.rng
                .lock()
                .random_range(start_ix..=state.runnables.len())
        } else {
            state.runnables.len()
        };
        state.runnables.insert(
            ix,
            ScheduledRunnable {
                session_id: Some(session_id),
                runnable,
            },
        );
        drop(state);
        self.thread.unpark();
    }

    fn schedule_background(&self, runnable: Runnable) {
        let mut state = self.state.lock();
        let ix = if state.randomize_order {
            self.rng.lock().random_range(0..=state.runnables.len())
        } else {
            state.runnables.len()
        };
        state.runnables.insert(
            ix,
            ScheduledRunnable {
                session_id: None,
                runnable,
            },
        );
        drop(state);
        self.thread.unpark();
    }

    fn timer(&self, duration: Duration) -> Timer {
        let (tx, rx) = oneshot::channel();
        let state = &mut *self.state.lock();
        state.timers.push(ScheduledTimer {
            expiration: self.clock.now() + duration,
            _notify: tx,
        });
        state.timers.sort_by_key(|timer| timer.expiration);
        Timer(rx)
    }

    fn clock(&self) -> Arc<dyn Clock> {
        self.clock.clone()
    }

    fn as_test(&self) -> &TestScheduler {
        self
    }
}

#[derive(Clone, Debug)]
pub struct TestSchedulerConfig {
    pub seed: u64,
    pub randomize_order: bool,
    pub allow_parking: bool,
    pub capture_pending_traces: bool,
    pub timeout_ticks: RangeInclusive<usize>,
}

impl TestSchedulerConfig {
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }
}

impl Default for TestSchedulerConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            randomize_order: true,
            allow_parking: false,
            capture_pending_traces: env::var(PENDING_TRACES_VAR_NAME)
                .map_or(false, |var| var == "1" || var == "true"),
            timeout_ticks: 0..=1000,
        }
    }
}

struct ScheduledRunnable {
    session_id: Option<SessionId>,
    runnable: Runnable,
}

impl ScheduledRunnable {
    fn run(self) {
        self.runnable.run();
    }
}

struct ScheduledTimer {
    expiration: Instant,
    _notify: oneshot::Sender<()>,
}

struct SchedulerState {
    runnables: VecDeque<ScheduledRunnable>,
    timers: Vec<ScheduledTimer>,
    blocked_sessions: Vec<SessionId>,
    randomize_order: bool,
    allow_parking: bool,
    timeout_ticks: RangeInclusive<usize>,
    next_session_id: SessionId,
    capture_pending_traces: bool,
    next_trace_id: TraceId,
    pending_traces: BTreeMap<TraceId, Backtrace>,
}

const WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    TracingWaker::clone_raw,
    TracingWaker::wake_raw,
    TracingWaker::wake_by_ref_raw,
    TracingWaker::drop_raw,
);

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct TraceId(usize);

struct TracingWaker {
    id: Option<TraceId>,
    awoken: Arc<AtomicBool>,
    thread: Thread,
    state: Arc<Mutex<SchedulerState>>,
}

impl Clone for TracingWaker {
    fn clone(&self) -> Self {
        let mut state = self.state.lock();
        let id = if state.capture_pending_traces {
            let id = state.next_trace_id;
            state.next_trace_id.0 += 1;
            state.pending_traces.insert(id, Backtrace::new_unresolved());
            Some(id)
        } else {
            None
        };
        Self {
            id,
            awoken: self.awoken.clone(),
            thread: self.thread.clone(),
            state: self.state.clone(),
        }
    }
}

impl Drop for TracingWaker {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            self.state.lock().pending_traces.remove(&id);
        }
    }
}

impl TracingWaker {
    fn wake(self) {
        self.wake_by_ref();
    }

    fn wake_by_ref(&self) {
        if let Some(id) = self.id {
            self.state.lock().pending_traces.remove(&id);
        }
        self.awoken.store(true, SeqCst);
        self.thread.unpark();
    }

    fn clone_raw(waker: *const ()) -> RawWaker {
        let waker = waker as *const TracingWaker;
        let waker = unsafe { &*waker };
        RawWaker::new(
            Box::into_raw(Box::new(waker.clone())) as *const (),
            &WAKER_VTABLE,
        )
    }

    fn wake_raw(waker: *const ()) {
        let waker = unsafe { Box::from_raw(waker as *mut TracingWaker) };
        waker.wake();
    }

    fn wake_by_ref_raw(waker: *const ()) {
        let waker = waker as *const TracingWaker;
        let waker = unsafe { &*waker };
        waker.wake_by_ref();
    }

    fn drop_raw(waker: *const ()) {
        let waker = unsafe { Box::from_raw(waker as *mut TracingWaker) };
        drop(waker);
    }
}

pub struct Yield(usize);

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.0 == 0 {
            Poll::Ready(())
        } else {
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn exclude_wakers_from_trace(mut trace: Backtrace) -> Backtrace {
    trace.resolve();
    let mut frames: Vec<BacktraceFrame> = trace.into();
    let waker_clone_frame_ix = frames.iter().position(|frame| {
        frame.symbols().iter().any(|symbol| {
            symbol
                .name()
                .is_some_and(|name| format!("{name:#?}") == type_name_of_val(&Waker::clone))
        })
    });

    if let Some(waker_clone_frame_ix) = waker_clone_frame_ix {
        frames.drain(..waker_clone_frame_ix + 1);
    }

    Backtrace::from(frames)
}
