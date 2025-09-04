use crate::{
    BackgroundExecutor, Clock as _, ForegroundExecutor, Scheduler, SessionId, TestClock, Timer,
};
use async_task::Runnable;
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use futures::{FutureExt as _, channel::oneshot, future::LocalBoxFuture};
use parking_lot::Mutex;
use rand::prelude::*;
use std::{
    collections::VecDeque,
    future::Future,
    panic::{self, AssertUnwindSafe},
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
    task::{Context, Poll, Wake, Waker},
    thread,
    time::{Duration, Instant},
};

pub struct TestScheduler {
    clock: Arc<TestClock>,
    rng: Arc<Mutex<StdRng>>,
    state: Mutex<SchedulerState>,
    pub thread_id: thread::ThreadId,
    pub config: SchedulerConfig,
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
        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::with_seed(seed)));
        let future = f(scheduler.clone());
        let result = scheduler.block_on(future);
        scheduler.run();
        result
    }

    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            rng: Arc::new(Mutex::new(StdRng::seed_from_u64(config.seed))),
            state: Mutex::new(SchedulerState {
                runnables: VecDeque::new(),
                timers: Vec::new(),
                randomize_order: config.randomize_order,
                allow_parking: config.allow_parking,
                next_session_id: SessionId(0),
            }),
            thread_id: thread::current().id(),
            clock: Arc::new(TestClock::new()),
            config,
        }
    }

    pub fn clock(&self) -> Arc<TestClock> {
        self.clock.clone()
    }

    pub fn rng(&self) -> Arc<Mutex<StdRng>> {
        self.rng.clone()
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

    pub fn block_on<Fut: Future>(&self, future: Fut) -> Fut::Output {
        (self as &dyn Scheduler).block_on(future)
    }

    pub fn yield_random(&self) -> Yield {
        Yield(self.rng.lock().random_range(0..20))
    }

    pub fn run(&self) {
        while self.step() || self.advance_clock() {
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

        let runnable = self.state.lock().runnables.pop_front();
        if let Some(runnable) = runnable {
            runnable.run();
            return true;
        }

        false
    }

    fn advance_clock(&self) -> bool {
        if let Some(timer) = self.state.lock().timers.first() {
            self.clock.set_now(timer.expiration);
            true
        } else {
            false
        }
    }
}

impl Scheduler for TestScheduler {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.thread_id
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
    }

    fn timer(&self, duration: Duration) -> Timer {
        let (tx, rx) = oneshot::channel();
        let expiration = self.clock.now() + ChronoDuration::from_std(duration).unwrap();
        let state = &mut *self.state.lock();
        state.timers.push(ScheduledTimer {
            expiration,
            _notify: tx,
        });
        state.timers.sort_by_key(|timer| timer.expiration);
        Timer(rx)
    }

    /// Block until the given future completes, with an optional timeout. If the
    /// future is unable to make progress at any moment before the timeout and
    /// no other tasks or timers remain, we panic unless parking is allowed. If
    /// parking is allowed, we block up to the timeout or indefinitely if none
    /// is provided. This is to allow testing a mix of deterministic and
    /// non-deterministic async behavior, such as when interacting with I/O in
    /// an otherwise deterministic test.
    fn block(&self, mut future: LocalBoxFuture<()>, timeout: Option<Duration>) {
        let (parker, unparker) = parking::pair();
        let deadline = timeout.map(|timeout| Instant::now() + timeout);
        let awoken = Arc::new(AtomicBool::new(false));
        let waker = Waker::from(Arc::new(WakerFn::new({
            let awoken = awoken.clone();
            move || {
                awoken.store(true, SeqCst);
                unparker.unpark();
            }
        })));
        let max_ticks = if timeout.is_some() {
            self.rng
                .lock()
                .random_range(0..=self.config.max_timeout_ticks)
        } else {
            usize::MAX
        };
        let mut cx = Context::from_waker(&waker);

        for _ in 0..max_ticks {
            let Poll::Pending = future.poll_unpin(&mut cx) else {
                break;
            };

            let mut stepped = None;
            while self.rng.lock().random() && stepped.unwrap_or(true) {
                *stepped.get_or_insert(false) |= self.step();
            }

            let stepped = stepped.unwrap_or(true);
            let awoken = awoken.swap(false, SeqCst);
            if !stepped && !awoken && !self.advance_clock() {
                if self.state.lock().allow_parking {
                    if !park(&parker, deadline) {
                        break;
                    }
                } else if deadline.is_some() {
                    break;
                } else {
                    panic!("Parking forbidden");
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SchedulerConfig {
    pub seed: u64,
    pub randomize_order: bool,
    pub allow_parking: bool,
    pub max_timeout_ticks: usize,
}

impl SchedulerConfig {
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            randomize_order: true,
            allow_parking: false,
            max_timeout_ticks: 1000,
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
    expiration: DateTime<Utc>,
    _notify: oneshot::Sender<()>,
}

struct SchedulerState {
    runnables: VecDeque<ScheduledRunnable>,
    timers: Vec<ScheduledTimer>,
    randomize_order: bool,
    allow_parking: bool,
    next_session_id: SessionId,
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

fn park(parker: &parking::Parker, deadline: Option<Instant>) -> bool {
    if let Some(deadline) = deadline {
        parker.park_deadline(deadline)
    } else {
        parker.park();
        true
    }
}
