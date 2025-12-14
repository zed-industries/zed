use crate::{PlatformDispatcher, Priority, RunnableVariant, TaskLabel};
use backtrace::Backtrace;
use collections::{HashMap, HashSet, VecDeque};
use parking::Unparker;
use parking_lot::Mutex;
use rand::prelude::*;
use std::{
    future::Future,
    hash::{Hash, Hasher},
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
        let enabled = std::env::var("DEBUG_SCHEDULER").map(|v| v == "1").unwrap_or(false);
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

#[doc(hidden)]
pub struct TestDispatcher {
    id: TestDispatcherId,
    state: Arc<Mutex<TestDispatcherState>>,
}

struct TestDispatcherState {
    random: StdRng,
    foreground: HashMap<TestDispatcherId, VecDeque<RunnableVariant>>,
    background: Vec<RunnableVariant>,
    deprioritized_background: Vec<RunnableVariant>,
    delayed: Vec<(Duration, RunnableVariant)>,
    start_time: Instant,
    time: Duration,
    is_main_thread: bool,
    next_id: TestDispatcherId,
    allow_parking: bool,
    waiting_hint: Option<String>,
    waiting_backtrace: Option<Backtrace>,
    deprioritized_task_labels: HashSet<TaskLabel>,
    block_on_ticks: RangeInclusive<usize>,
    unparkers: Vec<Unparker>,
    /// Tracks execution order for determinism verification.
    /// This hash is updated each time a task is executed, incorporating
    /// the task's source location. If two runs with the same seed produce
    /// different hashes, there is non-determinism in the test.
    execution_hash: u64,
    /// Count of tasks executed, for debugging.
    execution_count: u64,
}

impl TestDispatcher {
    pub fn new(random: StdRng) -> Self {
        let state = TestDispatcherState {
            random,
            foreground: HashMap::default(),
            background: Vec::new(),
            deprioritized_background: Vec::new(),
            delayed: Vec::new(),
            time: Duration::ZERO,
            start_time: Instant::now(),
            is_main_thread: true,
            next_id: TestDispatcherId(1),
            allow_parking: false,
            waiting_hint: None,
            waiting_backtrace: None,
            deprioritized_task_labels: Default::default(),
            block_on_ticks: 0..=1000,
            unparkers: Default::default(),
            execution_hash: 0,
            execution_count: 0,
        };

        TestDispatcher {
            id: TestDispatcherId(0),
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub fn advance_clock(&self, by: Duration) {
        let new_now = self.state.lock().time + by;
        loop {
            self.run_until_parked();
            let state = self.state.lock();
            let next_due_time = state.delayed.first().map(|(time, _)| *time);
            drop(state);
            if let Some(due_time) = next_due_time
                && due_time <= new_now
            {
                self.state.lock().time = due_time;
                continue;
            }
            break;
        }
        self.state.lock().time = new_now;
    }

    pub fn advance_clock_to_next_delayed(&self) -> bool {
        let next_due_time = self.state.lock().delayed.first().map(|(time, _)| *time);
        if let Some(next_due_time) = next_due_time {
            self.state.lock().time = next_due_time;
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
            count: self.state.lock().random.random_range(0..10),
        }
    }

    pub fn tick(&self, background_only: bool) -> bool {
        let mut state = self.state.lock();

        while let Some((deadline, _)) = state.delayed.first() {
            if *deadline > state.time {
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
                    foreground_len, background_len, deprioritized_len, delayed_len, unparkers_count
                );
                return false;
            }
            let ix = state.random.random_range(0..deprioritized_background_len);
            main_thread = false;
            task_source = "deprioritized";
            dispatcher_log!(
                "tick() selecting deprioritized[{}] of {} | fg={} bg={} delayed={}",
                ix, deprioritized_background_len, foreground_len, background_len, delayed_len
            );
            runnable = state.deprioritized_background.swap_remove(ix);
        } else {
            main_thread = state.random.random_ratio(
                foreground_len as u32,
                (foreground_len + background_len) as u32,
            );
            if main_thread {
                task_source = "foreground";
                let state = &mut *state;
                runnable = state
                    .foreground
                    .values_mut()
                    .filter(|runnables| !runnables.is_empty())
                    .choose(&mut state.random)
                    .unwrap()
                    .pop_front()
                    .unwrap();
                dispatcher_log!(
                    "tick() selecting foreground (ratio {}/{}) | fg={} bg={} delayed={}",
                    foreground_len, foreground_len + background_len,
                    foreground_len - 1, background_len, delayed_len
                );
            } else {
                task_source = "background";
                let ix = state.random.random_range(0..background_len);
                dispatcher_log!(
                    "tick() selecting background[{}] (ratio {}/{}) | fg={} bg={} delayed={}",
                    ix, foreground_len, foreground_len + background_len,
                    foreground_len, background_len - 1, delayed_len
                );
                runnable = state.background.swap_remove(ix);
            };
        };

        let was_main_thread = state.is_main_thread;
        state.is_main_thread = main_thread;
        drop(state);

        // Log task location before running (helps identify which task is executing)
        // Also update execution hash for determinism tracking
        let location_str = match &runnable {
            RunnableVariant::Meta(r) => {
                let loc = r.metadata().location;
                format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
            }
            RunnableVariant::Compat(_) => "compat-task".to_string(),
        };

        // Update execution hash with task location for determinism verification
        {
            let mut state = self.state.lock();
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            state.execution_hash.hash(&mut hasher);
            location_str.hash(&mut hasher);
            state.execution_count.hash(&mut hasher);
            state.execution_hash = hasher.finish();
            state.execution_count += 1;
        }

        dispatcher_log!(
            "tick() RUNNING {} task from {} | main_thread={} | exec_count={} exec_hash={:016x}",
            task_source, location_str, main_thread,
            self.state.lock().execution_count,
            self.state.lock().execution_hash
        );

        match runnable {
            RunnableVariant::Meta(runnable) => runnable.run(),
            RunnableVariant::Compat(runnable) => runnable.run(),
        };

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
        self.state.lock().allow_parking
    }

    pub fn allow_parking(&self) {
        self.state.lock().allow_parking = true
    }

    pub fn forbid_parking(&self) {
        self.state.lock().allow_parking = false
    }

    pub fn set_waiting_hint(&self, msg: Option<String>) {
        self.state.lock().waiting_hint = msg
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
        self.state.lock().random.clone()
    }

    pub fn set_block_on_ticks(&self, range: std::ops::RangeInclusive<usize>) {
        self.state.lock().block_on_ticks = range;
    }

    pub fn gen_block_on_ticks(&self) -> usize {
        let mut lock = self.state.lock();
        let block_on_ticks = lock.block_on_ticks.clone();
        lock.random.random_range(block_on_ticks)
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
    ///
    /// # Example
    /// ```ignore
    /// let hash_before = dispatcher.execution_hash();
    /// // ... do some work ...
    /// let hash_after = dispatcher.execution_hash();
    /// eprintln!("Execution hash: {} -> {} (count: {})",
    ///     hash_before, hash_after, dispatcher.execution_count());
    /// ```
    pub fn execution_hash(&self) -> u64 {
        self.state.lock().execution_hash
    }

    /// Returns the number of tasks executed so far.
    pub fn execution_count(&self) -> u64 {
        self.state.lock().execution_count
    }

    /// Resets the execution hash and count. Useful for isolating
    /// determinism checks to specific sections of a test.
    pub fn reset_execution_tracking(&self) {
        let mut state = self.state.lock();
        state.execution_hash = 0;
        state.execution_count = 0;
    }
}

impl Clone for TestDispatcher {
    fn clone(&self) -> Self {
        let id = post_inc(&mut self.state.lock().next_id.0);
        Self {
            id: TestDispatcherId(id),
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
        let state = self.state.lock();
        state.start_time + state.time
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
            bg_len, unparkers_before
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
            fg_len, unparkers_before
        );
        self.unpark_all();
    }

    fn dispatch_after(&self, duration: std::time::Duration, runnable: RunnableVariant) {
        let mut state = self.state.lock();
        let next_time = state.time + duration;
        let ix = match state.delayed.binary_search_by_key(&next_time, |e| e.0) {
            Ok(ix) | Err(ix) => ix,
        };
        state.delayed.insert(ix, (next_time, runnable));
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
