use crate::{PlatformDispatcher, Priority, RunnableVariant, TaskLabel};
use backtrace::Backtrace;
use collections::{HashMap, HashSet, VecDeque};
use parking::Unparker;
use parking_lot::Mutex;
use rand::prelude::*;
use scheduler::{Clock, TestScheduler, TestSchedulerConfig};
use std::{
    future::Future,
    hash::Hash,
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
use util::post_inc;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct TestDispatcherId(usize);

/// TestDispatcher provides deterministic async execution for tests.
///
/// This is a hybrid implementation that uses the scheduler crate's `TestScheduler`
/// for timing, clock, and randomization, while keeping GPUI's own task queues
/// for handling `RunnableVariant` (which has multiple variants that TestScheduler
/// cannot process directly).
#[doc(hidden)]
pub struct TestDispatcher {
    id: TestDispatcherId,
    scheduler: Arc<TestScheduler>,
    state: Arc<Mutex<TestDispatcherState>>,
}

struct TestDispatcherState {
    foreground: HashMap<TestDispatcherId, VecDeque<RunnableVariant>>,
    background: Vec<RunnableVariant>,
    deprioritized_background: Vec<RunnableVariant>,
    delayed: Vec<(Instant, RunnableVariant)>,
    is_main_thread: bool,
    next_id: TestDispatcherId,
    waiting_hint: Option<String>,
    waiting_backtrace: Option<Backtrace>,
    deprioritized_task_labels: HashSet<TaskLabel>,
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

        let state = TestDispatcherState {
            foreground: HashMap::default(),
            background: Vec::new(),
            deprioritized_background: Vec::new(),
            delayed: Vec::new(),
            is_main_thread: true,
            next_id: TestDispatcherId(1),
            waiting_hint: None,
            waiting_backtrace: None,
            deprioritized_task_labels: Default::default(),
            block_on_ticks: 0..=1000,
            unparkers: Default::default(),
        };

        TestDispatcher {
            id: TestDispatcherId(0),
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
        // Note: We use a local YieldNow instead of `self.scheduler.yield_random()` because
        // the scheduler's version uses a different distribution (0..2 with 10% chance of 10..20)
        // and consumes 2 RNG values per call. GPUI tests rely on the original 0..10 range
        // with a single RNG consumption for deterministic behavior.
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
        let mut state = self.state.lock();
        let now = self.scheduler.clock().now();

        while let Some((deadline, _)) = state.delayed.first() {
            if *deadline > now {
                break;
            }
            let (_, runnable) = state.delayed.remove(0);
            state.background.push(runnable);
        }

        let foreground_len: usize = if background_only {
            0
        } else {
            state
                .foreground
                .values()
                .map(|runnables| runnables.len())
                .sum()
        };
        let background_len = state.background.len();
        let deprioritized_len = state.deprioritized_background.len();
        let delayed_len = state.delayed.len();
        let unparkers_count = state.unparkers.len();

        let runnable;
        let main_thread;
        let task_source: &str;
        if foreground_len == 0 && background_len == 0 {
            let deprioritized_background_len = state.deprioritized_background.len();
            if deprioritized_background_len == 0 {
                dispatcher_log!(
                    "tick() -> false (no tasks) | fg={} bg={} depri={} delayed={} unparkers={}",
                    foreground_len,
                    background_len,
                    deprioritized_len,
                    delayed_len,
                    unparkers_count
                );
                return false;
            }
            let ix = self
                .scheduler
                .rng()
                .lock()
                .random_range(0..deprioritized_background_len);
            main_thread = false;
            task_source = "deprioritized";
            dispatcher_log!(
                "tick() selecting deprioritized[{}] of {} | fg={} bg={} delayed={}",
                ix,
                deprioritized_background_len,
                foreground_len,
                background_len,
                delayed_len
            );
            runnable = state.deprioritized_background.swap_remove(ix);
        } else {
            main_thread = self.scheduler.rng().lock().random_ratio(
                foreground_len as u32,
                (foreground_len + background_len) as u32,
            );
            if main_thread {
                task_source = "foreground";
                let rng_arc = self.scheduler.rng();
                let mut rng = rng_arc.lock();
                runnable = state
                    .foreground
                    .values_mut()
                    .filter(|runnables| !runnables.is_empty())
                    .choose(&mut *rng)
                    .unwrap()
                    .pop_front()
                    .unwrap();
                dispatcher_log!(
                    "tick() selecting foreground (ratio {}/{}) | fg={} bg={} delayed={}",
                    foreground_len,
                    foreground_len + background_len,
                    foreground_len - 1,
                    background_len,
                    delayed_len
                );
            } else {
                task_source = "background";
                let ix = self.scheduler.rng().lock().random_range(0..background_len);
                dispatcher_log!(
                    "tick() selecting background[{}] (ratio {}/{}) | fg={} bg={} delayed={}",
                    ix,
                    foreground_len,
                    foreground_len + background_len,
                    foreground_len,
                    background_len - 1,
                    delayed_len
                );
                runnable = state.background.swap_remove(ix);
            };
        };

        let was_main_thread = state.is_main_thread;
        state.is_main_thread = main_thread;
        drop(state);

        // Log task location before running (helps identify which task is executing)
        let RunnableVariant::Meta(ref r) = runnable;
        let loc = r.metadata().location;
        let location_str = format!("{}:{}:{}", loc.file(), loc.line(), loc.column());

        dispatcher_log!(
            "tick() RUNNING {} task from {} | main_thread={}",
            task_source,
            location_str,
            main_thread,
        );

        let RunnableVariant::Meta(runnable) = runnable;
        runnable.run();

        self.state.lock().is_main_thread = was_main_thread;

        true
    }

    pub fn deprioritize(&self, task_label: TaskLabel) {
        self.state
            .lock()
            .deprioritized_task_labels
            .insert(task_label);
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
        self.state.lock().waiting_backtrace.take().map(|mut b| {
            b.resolve();
            b
        })
    }

    pub fn rng(&self) -> StdRng {
        self.scheduler.rng().lock().clone()
    }

    pub fn set_block_on_ticks(&self, range: RangeInclusive<usize>) {
        self.state.lock().block_on_ticks = range.clone();
        self.scheduler.set_timeout_ticks(range);
    }

    pub fn gen_block_on_ticks(&self) -> usize {
        let block_on_ticks = self.state.lock().block_on_ticks.clone();
        self.scheduler.rng().lock().random_range(block_on_ticks)
    }

    pub fn unpark_all(&self) {
        let mut state = self.state.lock();
        let count = state.unparkers.len();
        state.unparkers.retain(|parker| parker.unpark());
        dispatcher_log!("unpark_all() | unparked {} threads", count);
    }

    pub fn push_unparker(&self, unparker: Unparker) {
        let mut state = self.state.lock();
        let count_before = state.unparkers.len();
        state.unparkers.push(unparker);
        dispatcher_log!(
            "push_unparker() | unparkers: {} -> {} | fg={} bg={} depri={}",
            count_before,
            state.unparkers.len(),
            state.foreground.values().map(|v| v.len()).sum::<usize>(),
            state.background.len(),
            state.deprioritized_background.len()
        );
    }

    pub fn unparker_count(&self) -> usize {
        self.state.lock().unparkers.len()
    }

    /// Returns a hash of the execution order so far.
    ///
    /// This can be used to verify test determinism: if two runs with the same
    /// seed produce different execution hashes at the same point, there is
    /// non-determinism in the execution (likely from real OS threads, smol::spawn,
    /// or other sources outside TestDispatcher's control).
    /// Returns a reference to the underlying TestScheduler.
    /// This can be used for advanced testing scenarios that need direct scheduler access.
    pub fn scheduler(&self) -> &Arc<TestScheduler> {
        &self.scheduler
    }
}

impl Clone for TestDispatcher {
    fn clone(&self) -> Self {
        let id = post_inc(&mut self.state.lock().next_id.0);
        Self {
            id: TestDispatcherId(id),
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

    fn dispatch(&self, runnable: RunnableVariant, label: Option<TaskLabel>, _priority: Priority) {
        let (bg_len, unparkers_before) = {
            let mut state = self.state.lock();
            let unparkers_before = state.unparkers.len();
            if label.is_some_and(|label| state.deprioritized_task_labels.contains(&label)) {
                state.deprioritized_background.push(runnable);
            } else {
                state.background.push(runnable);
            }
            (state.background.len(), unparkers_before)
        };
        dispatcher_log!(
            "dispatch() | bg_len={} unparkers_at_dispatch={} (about to unpark_all)",
            bg_len,
            unparkers_before
        );
        self.unpark_all();
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        let (fg_len, unparkers_before) = {
            let mut state = self.state.lock();
            let unparkers_before = state.unparkers.len();
            state
                .foreground
                .entry(self.id)
                .or_default()
                .push_back(runnable);
            let fg_len: usize = state.foreground.values().map(|v| v.len()).sum();
            (fg_len, unparkers_before)
        };
        dispatcher_log!(
            "dispatch_on_main_thread() | fg_len={} unparkers_at_dispatch={} (about to unpark_all)",
            fg_len,
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
