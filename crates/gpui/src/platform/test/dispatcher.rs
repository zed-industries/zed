use crate::{PlatformDispatcher, Priority, RunnableVariant, TaskLabel};
use backtrace::Backtrace;
use parking::Unparker;
use parking_lot::Mutex;
use rand::prelude::*;
use scheduler::{Clock, Scheduler, SessionId, TestScheduler, TestSchedulerConfig};
use std::{
    future::Future,
    ops::RangeInclusive,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll},
    time::{Duration, Instant},
};

static DISPATCHER_LOG_ENABLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static DISPATCHER_LOG_INITIALIZED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static DISPATCHER_EVENT_SEQ: AtomicU64 = AtomicU64::new(0);

/// Enable detailed dispatcher logging for debugging race conditions
#[allow(dead_code)]
pub fn enable_dispatcher_logging() {
    DISPATCHER_LOG_ENABLED.store(true, Ordering::SeqCst);
}

/// Disable detailed dispatcher logging
#[allow(dead_code)]
pub fn disable_dispatcher_logging() {
    DISPATCHER_LOG_ENABLED.store(false, Ordering::SeqCst);
}

fn check_dispatcher_log_init() {
    if !DISPATCHER_LOG_INITIALIZED.load(Ordering::Relaxed) {
        let enabled = std::env::var("DEBUG_SCHEDULER")
            .map(|v| v == "1")
            .unwrap_or(false);
        DISPATCHER_LOG_ENABLED.store(enabled, Ordering::SeqCst);
        DISPATCHER_LOG_INITIALIZED.store(true, Ordering::SeqCst);
        if enabled {
            eprintln!("[DISP] Dispatcher debugging enabled via DEBUG_SCHEDULER=1");
        }
    }
}

macro_rules! dispatcher_log {
    ($($arg:tt)*) => {
        {
            check_dispatcher_log_init();
            if DISPATCHER_LOG_ENABLED.load(Ordering::Relaxed) {
                let seq = DISPATCHER_EVENT_SEQ.fetch_add(1, Ordering::SeqCst);
                eprintln!("[DISP {:>6}] [thread {:?}] {}", seq, std::thread::current().id(), format!($($arg)*));
            }
        }
    };
}

/// TestDispatcher provides deterministic async execution for tests.
///
/// This implementation delegates task scheduling to the scheduler crate's `TestScheduler`,
/// which provides timing, clock, randomization, and task queue management. The dispatcher
/// maintains only the delayed task queue (for dispatch_after) and state needed for
/// GPUI-specific behavior like is_main_thread tracking.
#[doc(hidden)]
pub struct TestDispatcher {
    session_id: SessionId,
    scheduler: Arc<TestScheduler>,
    state: Arc<Mutex<TestDispatcherState>>,
}

struct TestDispatcherState {
    delayed: Vec<(Instant, RunnableVariant)>,
    is_main_thread: bool,
    waiting_hint: Option<String>,
    waiting_backtrace: Option<Backtrace>,
    block_on_ticks: RangeInclusive<usize>,
    unparkers: Vec<Unparker>,
}

impl TestDispatcher {
    pub fn new(seed: u64) -> Self {
        let scheduler = Arc::new(TestScheduler::new(TestSchedulerConfig {
            seed,
            randomize_order: true,
            allow_parking: false,
            capture_pending_traces: std::env::var("PENDING_TRACES")
                .map_or(false, |var| var == "1" || var == "true"),
            timeout_ticks: 0..=1000,
        }));

        let session_id = scheduler.allocate_session_id();

        let state = TestDispatcherState {
            delayed: Vec::new(),
            is_main_thread: true,
            waiting_hint: None,
            waiting_backtrace: None,
            block_on_ticks: 0..=1000,
            unparkers: Default::default(),
        };

        TestDispatcher {
            session_id,
            scheduler,
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub fn advance_clock(&self, by: Duration) {
        let target_time = self.scheduler.clock().now() + by;
        loop {
            self.run_until_parked();
            let next_delayed = self.state.lock().delayed.first().map(|(time, _)| *time);
            if let Some(delayed_time) = next_delayed {
                if delayed_time <= target_time {
                    let advance_by = delayed_time - self.scheduler.clock().now();
                    self.scheduler.advance_clock(advance_by);
                    continue;
                }
            }
            break;
        }
        let remaining = target_time - self.scheduler.clock().now();
        if remaining > Duration::ZERO {
            self.scheduler.advance_clock(remaining);
        }
    }

    pub fn advance_clock_to_next_delayed(&self) -> bool {
        let next_delayed = self.state.lock().delayed.first().map(|(time, _)| *time);
        if let Some(delayed_time) = next_delayed {
            let now = self.scheduler.clock().now();
            if delayed_time > now {
                self.scheduler.advance_clock(delayed_time - now);
            }
            return true;
        }
        false
    }

    pub fn simulate_random_delay(&self) -> impl 'static + Send + Future<Output = ()> + use<> {
        struct YieldNow {
            pub(crate) count: usize,
        }

        impl Future for YieldNow {
            type Output = ();

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
                if self.count > 0 {
                    self.count -= 1;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                } else {
                    Poll::Ready(())
                }
            }
        }

        YieldNow {
            count: self.scheduler.rng().lock().random_range(0..10),
        }
    }

    pub fn tick(&self, background_only: bool) -> bool {
        let now = self.scheduler.clock().now();

        // Move due delayed tasks to the scheduler's background queue
        {
            let mut state = self.state.lock();
            while let Some((deadline, _)) = state.delayed.first() {
                if *deadline > now {
                    break;
                }
                let (_, runnable) = state.delayed.remove(0);
                self.scheduler.schedule_background(runnable);
            }
        }

        let (foreground_count, background_count) = self.scheduler.pending_task_counts();
        let delayed_count = self.state.lock().delayed.len();

        if foreground_count == 0 && background_count == 0 {
            dispatcher_log!(
                "tick() -> false (no tasks) | fg={} bg={} delayed={}",
                foreground_count,
                background_count,
                delayed_count
            );
            return false;
        }

        // Determine if we should run a foreground or background task
        let run_foreground = if background_only || foreground_count == 0 {
            false
        } else if background_count == 0 {
            true
        } else {
            self.scheduler.rng().lock().random_ratio(
                foreground_count as u32,
                (foreground_count + background_count) as u32,
            )
        };

        // Track is_main_thread based on what type of task we're running
        let was_main_thread = self.state.lock().is_main_thread;
        self.state.lock().is_main_thread = run_foreground;

        dispatcher_log!(
            "tick() running {} task | fg={} bg={} delayed={} main_thread={}",
            if run_foreground { "foreground" } else { "background" },
            foreground_count,
            background_count,
            delayed_count,
            run_foreground,
        );

        // Run a task through the scheduler
        let did_work = if background_only {
            self.scheduler.tick_background_only()
        } else {
            self.scheduler.tick()
        };

        self.state.lock().is_main_thread = was_main_thread;

        did_work
    }

    pub fn run_until_parked(&self) {
        while self.tick(false) {}
    }

    pub fn parking_allowed(&self) -> bool {
        self.scheduler.parking_allowed()
    }

    pub fn allow_parking(&self) {
        self.scheduler.allow_parking();
    }

    pub fn forbid_parking(&self) {
        self.scheduler.forbid_parking();
    }

    pub fn set_waiting_hint(&self, msg: Option<String>) {
        self.state.lock().waiting_hint = msg;
    }

    pub fn waiting_hint(&self) -> Option<String> {
        self.state.lock().waiting_hint.clone()
    }

    pub fn start_waiting(&self) {
        self.state.lock().waiting_backtrace = Some(Backtrace::new_unresolved());
    }

    pub fn finish_waiting(&self) {
        self.state.lock().waiting_backtrace.take();
    }

    pub fn waiting_backtrace(&self) -> Option<Backtrace> {
        self.state.lock().waiting_backtrace.as_ref().map(|trace| {
            let mut trace = trace.clone();
            trace.resolve();
            trace
        })
    }

    pub fn rng(&self) -> Arc<parking_lot::Mutex<StdRng>> {
        self.scheduler.rng()
    }

    pub fn set_block_on_ticks(&self, range: RangeInclusive<usize>) {
        self.state.lock().block_on_ticks = range;
    }

    pub fn gen_block_on_ticks(&self) -> usize {
        let range = self.state.lock().block_on_ticks.clone();
        self.scheduler.rng().lock().random_range(range)
    }

    pub fn unpark_all(&self) {
        let unparkers: Vec<_> = self.state.lock().unparkers.drain(..).collect();
        let count = unparkers.len();
        if count > 0 {
            dispatcher_log!("unpark_all() | unparking {} threads", count);
        }
        for unparker in unparkers {
            unparker.unpark();
        }
    }

    pub fn push_unparker(&self, unparker: Unparker) {
        let mut state = self.state.lock();
        let count_before = state.unparkers.len();
        state.unparkers.push(unparker);
        let (fg, bg) = self.scheduler.pending_task_counts();
        dispatcher_log!(
            "push_unparker() | unparkers: {} -> {} | fg={} bg={}",
            count_before,
            state.unparkers.len(),
            fg,
            bg
        );
    }

    pub fn unparker_count(&self) -> usize {
        self.state.lock().unparkers.len()
    }

    /// Returns a reference to the underlying TestScheduler.
    /// This can be used for advanced testing scenarios that need direct scheduler access.
    pub fn scheduler(&self) -> &Arc<TestScheduler> {
        &self.scheduler
    }
}

impl Clone for TestDispatcher {
    fn clone(&self) -> Self {
        let session_id = self.scheduler.allocate_session_id();
        Self {
            session_id,
            scheduler: self.scheduler.clone(),
            state: self.state.clone(),
        }
    }
}

impl PlatformDispatcher for TestDispatcher {
    fn get_all_timings(&self) -> Vec<crate::ThreadTaskTimings> {
        Vec::new()
    }

    fn get_current_thread_timings(&self) -> Vec<crate::TaskTiming> {
        Vec::new()
    }

    fn is_main_thread(&self) -> bool {
        self.state.lock().is_main_thread
    }

    fn now(&self) -> Instant {
        self.scheduler.clock().now()
    }

    fn dispatch(&self, runnable: RunnableVariant, _label: Option<TaskLabel>, priority: Priority) {
        let (fg, bg) = self.scheduler.pending_task_counts();
        let unparkers_before = self.state.lock().unparkers.len();

        self.scheduler
            .schedule_background_with_priority(runnable, priority);

        dispatcher_log!(
            "dispatch() | fg={} bg={} unparkers_at_dispatch={} (about to unpark_all)",
            fg,
            bg + 1,
            unparkers_before
        );
        self.unpark_all();
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        let (fg, bg) = self.scheduler.pending_task_counts();
        let unparkers_before = self.state.lock().unparkers.len();

        self.scheduler.schedule_foreground(self.session_id, runnable);

        dispatcher_log!(
            "dispatch_on_main_thread() | fg={} bg={} unparkers_at_dispatch={} (about to unpark_all)",
            fg + 1,
            bg,
            unparkers_before
        );
        self.unpark_all();
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let mut state = self.state.lock();
        let deadline = self.scheduler.clock().now() + duration;
        let ix = match state.delayed.binary_search_by_key(&deadline, |e| e.0) {
            Ok(ix) | Err(ix) => ix,
        };
        state.delayed.insert(ix, (deadline, runnable));
    }

    fn as_test(&self) -> Option<&TestDispatcher> {
        Some(self)
    }

    fn spawn_realtime(&self, _priority: crate::RealtimePriority, _f: Box<dyn FnOnce() + Send>) {
        panic!(
            "spawn_realtime is not supported in TestDispatcher. \
            Real OS threads break test determinism - tests would become \
            flaky and unreproducible even with the same SEED. \
            Use a different Priority (High, Medium, Low) instead."
        );
    }
}
