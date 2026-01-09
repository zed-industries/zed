use crate::{
    BackgroundExecutor, Clock, ForegroundExecutor, Priority, RunnableMeta, Scheduler, SessionId,
    TestClock, Timer,
};
use async_task::Runnable;
use backtrace::{Backtrace, BacktraceFrame};
use futures::channel::oneshot;
use parking_lot::{Mutex, MutexGuard};
use rand::{
    distr::{StandardUniform, uniform::SampleRange, uniform::SampleUniform},
    prelude::*,
};
use std::{
    any::type_name_of_val,
    collections::{BTreeMap, HashSet, VecDeque},
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
    pub fn many<R>(
        default_iterations: usize,
        mut f: impl AsyncFnMut(Arc<TestScheduler>) -> R,
    ) -> Vec<R> {
        let num_iterations = std::env::var("ITERATIONS")
            .map(|iterations| iterations.parse().unwrap())
            .unwrap_or(default_iterations);

        let seed = std::env::var("SEED")
            .map(|seed| seed.parse().unwrap())
            .unwrap_or(0);

        (seed..num_iterations as u64)
            .map(|seed| {
                let mut unwind_safe_f = AssertUnwindSafe(&mut f);
                eprintln!("Running seed: {seed}");
                match panic::catch_unwind(move || Self::with_seed(seed, &mut *unwind_safe_f)) {
                    Ok(result) => result,
                    Err(error) => {
                        eprintln!("\x1b[31mFailing Seed: {seed}\x1b[0m");
                        panic::resume_unwind(error);
                    }
                }
            })
            .collect()
    }

    fn with_seed<R>(seed: u64, f: impl AsyncFnOnce(Arc<TestScheduler>) -> R) -> R {
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
                is_main_thread: true,
            })),
            clock: Arc::new(TestClock::new()),
            thread: thread::current(),
        }
    }

    pub fn clock(&self) -> Arc<TestClock> {
        self.clock.clone()
    }

    pub fn rng(&self) -> SharedRng {
        SharedRng(self.rng.clone())
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

    pub fn parking_allowed(&self) -> bool {
        self.state.lock().allow_parking
    }

    pub fn is_main_thread(&self) -> bool {
        self.state.lock().is_main_thread
    }

    /// Allocate a new session ID for foreground task scheduling.
    /// This is used by GPUI's TestDispatcher to map dispatcher instances to sessions.
    pub fn allocate_session_id(&self) -> SessionId {
        let mut state = self.state.lock();
        state.next_session_id.0 += 1;
        state.next_session_id
    }

    /// Create a foreground executor for this scheduler
    pub fn foreground(self: &Arc<Self>) -> ForegroundExecutor {
        let session_id = self.allocate_session_id();
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

    /// Execute one tick of the scheduler, processing expired timers and running
    /// at most one task. Returns true if any work was done.
    ///
    /// This is the public interface for GPUI's TestDispatcher to drive task execution.
    pub fn tick(&self) -> bool {
        self.step_filtered(false)
    }

    /// Execute one tick, but only run background tasks (no foreground/session tasks).
    /// Returns true if any work was done.
    pub fn tick_background_only(&self) -> bool {
        self.step_filtered(true)
    }

    /// Check if there are any pending tasks or timers that could run.
    pub fn has_pending_tasks(&self) -> bool {
        let state = self.state.lock();
        !state.runnables.is_empty() || !state.timers.is_empty()
    }

    /// Returns counts of (foreground_tasks, background_tasks) currently queued.
    /// Foreground tasks are those with a session_id, background tasks have none.
    pub fn pending_task_counts(&self) -> (usize, usize) {
        let state = self.state.lock();
        let foreground = state
            .runnables
            .iter()
            .filter(|r| r.session_id.is_some())
            .count();
        let background = state
            .runnables
            .iter()
            .filter(|r| r.session_id.is_none())
            .count();
        (foreground, background)
    }

    fn step(&self) -> bool {
        self.step_filtered(false)
    }

    fn step_filtered(&self, background_only: bool) -> bool {
        let (elapsed_count, runnables_before) = {
            let mut state = self.state.lock();
            let end_ix = state
                .timers
                .partition_point(|timer| timer.expiration <= self.clock.now());
            let elapsed: Vec<_> = state.timers.drain(..end_ix).collect();
            let count = elapsed.len();
            let runnables = state.runnables.len();
            drop(state);
            // Dropping elapsed timers here wakes the waiting futures
            drop(elapsed);
            (count, runnables)
        };

        if elapsed_count > 0 {
            let runnables_after = self.state.lock().runnables.len();
            if std::env::var("DEBUG_SCHEDULER").is_ok() {
                eprintln!(
                    "[scheduler] Expired {} timers at {:?}, runnables: {} -> {}",
                    elapsed_count,
                    self.clock.now(),
                    runnables_before,
                    runnables_after
                );
            }
            return true;
        }

        let runnable = {
            let state = &mut *self.state.lock();

            // Find candidate tasks:
            // - For foreground tasks (with session_id), only the first task from each session
            //   is a candidate (to preserve intra-session ordering)
            // - For background tasks (no session_id), all are candidates
            // - Tasks from blocked sessions are excluded
            // - If background_only is true, skip foreground tasks entirely
            let mut seen_sessions = HashSet::new();
            let candidate_indices: Vec<usize> = state
                .runnables
                .iter()
                .enumerate()
                .filter(|(_, runnable)| {
                    if let Some(session_id) = runnable.session_id {
                        // Skip foreground tasks if background_only mode
                        if background_only {
                            return false;
                        }
                        // Exclude tasks from blocked sessions
                        if state.blocked_sessions.contains(&session_id) {
                            return false;
                        }
                        // Only include first task from each session (insert returns true if new)
                        seen_sessions.insert(session_id)
                    } else {
                        // Background tasks are always candidates
                        true
                    }
                })
                .map(|(ix, _)| ix)
                .collect();

            if candidate_indices.is_empty() {
                None
            } else if state.randomize_order {
                // Use priority-weighted random selection
                let weights: Vec<u32> = candidate_indices
                    .iter()
                    .map(|&ix| state.runnables[ix].priority.weight())
                    .collect();
                let total_weight: u32 = weights.iter().sum();

                if total_weight == 0 {
                    // Fallback to uniform random if all weights are zero
                    let choice = self.rng.lock().random_range(0..candidate_indices.len());
                    state.runnables.remove(candidate_indices[choice])
                } else {
                    let mut target = self.rng.lock().random_range(0..total_weight);
                    let mut selected_idx = 0;
                    for (i, &weight) in weights.iter().enumerate() {
                        if target < weight {
                            selected_idx = i;
                            break;
                        }
                        target -= weight;
                    }
                    state.runnables.remove(candidate_indices[selected_idx])
                }
            } else {
                // Non-randomized: just take the first candidate task
                state.runnables.remove(candidate_indices[0])
            }
        };

        if let Some(runnable) = runnable {
            // Check if the executor that spawned this task was closed
            if runnable.runnable.metadata().is_closed() {
                return true;
            }
            let is_foreground = runnable.session_id.is_some();
            let was_main_thread = self.state.lock().is_main_thread;
            self.state.lock().is_main_thread = is_foreground;
            runnable.run();
            self.state.lock().is_main_thread = was_main_thread;
            return true;
        }

        false
    }

    pub fn advance_clock_to_next_timer(&self) -> bool {
        if let Some(timer) = self.state.lock().timers.first() {
            self.clock.advance(timer.expiration - self.clock.now());
            true
        } else {
            false
        }
    }

    pub fn advance_clock(&self, duration: Duration) {
        let debug = std::env::var("DEBUG_SCHEDULER").is_ok();
        let start = self.clock.now();
        let next_now = start + duration;
        if debug {
            let timer_count = self.state.lock().timers.len();
            eprintln!(
                "[scheduler] advance_clock({:?}) from {:?}, {} pending timers",
                duration, start, timer_count
            );
        }
        loop {
            self.run();
            if let Some(timer) = self.state.lock().timers.first()
                && timer.expiration <= next_now
            {
                let advance_to = timer.expiration;
                if debug {
                    eprintln!(
                        "[scheduler] Advancing clock {:?} -> {:?} for timer",
                        self.clock.now(),
                        advance_to
                    );
                }
                self.clock.advance(advance_to - self.clock.now());
            } else {
                break;
            }
        }
        self.clock.advance(next_now - self.clock.now());
        if debug {
            eprintln!(
                "[scheduler] advance_clock done, now at {:?}",
                self.clock.now()
            );
        }
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
        mut future: Pin<&mut dyn Future<Output = ()>>,
        timeout: Option<Duration>,
    ) -> bool {
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

        let mut completed = false;
        for _ in 0..max_ticks {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    completed = true;
                    break;
                }
                Poll::Pending => {}
            }

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

        completed
    }

    fn schedule_foreground(&self, session_id: SessionId, runnable: Runnable<RunnableMeta>) {
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
                priority: Priority::default(),
                runnable,
            },
        );
        drop(state);
        self.thread.unpark();
    }

    fn schedule_background_with_priority(
        &self,
        runnable: Runnable<RunnableMeta>,
        priority: Priority,
    ) {
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
                priority,
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

    fn as_test(&self) -> Option<&TestScheduler> {
        Some(self)
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
    priority: Priority,
    runnable: Runnable<RunnableMeta>,
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
    is_main_thread: bool,
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

/// A wrapper around `Arc<Mutex<StdRng>>` that provides convenient methods
/// for random number generation without requiring explicit locking.
#[derive(Clone)]
pub struct SharedRng(Arc<Mutex<StdRng>>);

impl SharedRng {
    /// Lock the inner RNG for direct access. Use this when you need multiple
    /// random operations without re-locking between each one.
    pub fn lock(&self) -> MutexGuard<'_, StdRng> {
        self.0.lock()
    }

    /// Generate a random value in the given range.
    pub fn random_range<T, R>(&self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.0.lock().random_range(range)
    }

    /// Generate a random boolean with the given probability of being true.
    pub fn random_bool(&self, p: f64) -> bool {
        self.0.lock().random_bool(p)
    }

    /// Generate a random value of the given type.
    pub fn random<T>(&self) -> T
    where
        StandardUniform: Distribution<T>,
    {
        self.0.lock().random()
    }

    /// Generate a random ratio - true with probability `numerator/denominator`.
    pub fn random_ratio(&self, numerator: u32, denominator: u32) -> bool {
        self.0.lock().random_ratio(numerator, denominator)
    }
}

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
