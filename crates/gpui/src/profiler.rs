use itertools::Itertools;
use scheduler::{Instant, SpawnTime};
use std::{
    cell::LazyCell,
    collections::{HashMap, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::ThreadId,
    time::Duration,
};

mod actions;
pub use actions::{ActionStatistics, ActionTiming, take_action_stats};
pub(crate) use actions::{save_action_timing, update_running_action};

use serde::{Deserialize, Serialize};

use crate::{SharedString, TasksIncluded};

#[cfg(feature = "profiler")]
#[doc(hidden)]
pub fn get_all_timings(included: gpui::TasksIncluded) -> Vec<gpui::ThreadTaskTimings> {
    let global_thread_timings = GLOBAL_THREAD_TIMINGS.lock();
    ThreadTaskTimings::collect(&global_thread_timings, included)
}

#[cfg(feature = "profiler")]
#[doc(hidden)]
pub fn get_current_thread_timings(included: TasksIncluded) -> gpui::ThreadTaskTimings {
    gpui::profiler::get_current_thread_task_timings(included)
}

#[cfg(feature = "profiler")]
#[doc(hidden)]
pub fn take_all_stats(included: TasksIncluded) -> Vec<gpui::ThreadTaskStatistics> {
    let global_timings = GLOBAL_THREAD_TIMINGS.lock();
    ThreadTaskStatistics::collect_and_reset(&global_timings, included)
}

#[cfg(not(feature = "profiler"))]
#[doc(hidden)]
pub fn get_all_timings(_included: gpui::TasksIncluded) -> Vec<gpui::ThreadTaskTimings> {
    Vec::new()
}
#[cfg(not(feature = "profiler"))]
#[doc(hidden)]
pub fn get_current_thread_timings(_included: TasksIncluded) -> gpui::ThreadTaskTimings {
    gpui::ThreadTaskTimings {
        thread_name: None,
        thread_id: std::thread::current().id(),
        timings: Vec::new(),
        stats: TaskStatistics::default(),
        total_pushed: 0,
    }
}
#[cfg(not(feature = "profiler"))]
#[doc(hidden)]
pub fn take_all_stats(_included: TasksIncluded) -> Vec<gpui::ThreadTaskStatistics> {
    Vec::new()
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
pub struct YieldTime(pub Instant);

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct TaskTiming {
    pub location: &'static core::panic::Location<'static>,
    pub spawned: SpawnTime,
    pub start: Instant,
    pub end: YieldTime,
}

impl std::fmt::Debug for TaskTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskTiming")
            .field("location", &self.location)
            .field("since_spawned", &self.spawned.0.elapsed())
            .field("last_poll_duration", &self.poll_duration())
            .field("total_runtime", &self.since_spawn())
            .finish()
    }
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
pub struct ActiveTiming {
    pub location: &'static core::panic::Location<'static>,
    pub spawned: SpawnTime,
    pub start: Instant,
}

impl TaskTiming {
    /// A task timing with a duration of zero. Any task will replace this in history.
    pub fn placeholder() -> Self {
        let now = Instant::now();
        Self {
            location: std::panic::Location::caller(),
            spawned: SpawnTime(now),
            start: now,
            end: YieldTime(now),
        }
    }

    #[inline(always)]
    pub fn poll_duration(&self) -> Duration {
        self.end.0 - self.start
    }

    #[inline(always)]
    fn since_spawn(&self) -> Duration {
        self.end.0 - self.spawned.0
    }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ThreadTaskTimings {
    pub thread_name: Option<String>,
    pub thread_id: ThreadId,
    pub timings: Vec<TaskTiming>,
    pub stats: TaskStatistics,
    pub total_pushed: u64,
}

impl ThreadTaskTimings {
    /// Convert global thread timings into their structured format.
    pub fn collect(timings: &[GlobalThreadTimings], included: TasksIncluded) -> Vec<Self> {
        timings
            .iter()
            .filter_map(|t| match t.timings.upgrade() {
                Some(timings) => Some((t.thread_id, timings)),
                _ => None,
            })
            .map(|(thread_id, timings)| {
                let timings = timings.lock();
                let thread_name = timings.thread_name.clone();
                let total_pushed = timings.total_pushed;
                let completed = &timings.timings;

                let mut vec = Vec::with_capacity(completed.len() + 1); // +1 for running task
                let (s1, s2) = completed.as_slices();
                vec.extend_from_slice(s1);
                vec.extend_from_slice(s2);
                if let TasksIncluded::CompletedAndRunning = included
                    && let Some(running) = timings.running
                {
                    vec.push(TaskTiming {
                        location: running.location,
                        spawned: running.spawned,
                        start: running.start,
                        end: YieldTime(Instant::now()),
                    })
                }

                ThreadTaskTimings {
                    thread_name,
                    thread_id,
                    timings: vec,
                    stats: timings.stats.clone(),
                    total_pushed,
                }
            })
            .collect()
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ThreadTaskStatistics {
    pub thread_name: Option<String>,
    pub thread_id: ThreadId,
    pub stats: TaskStatistics,
}

impl ThreadTaskStatistics {
    pub fn collect_and_reset(
        timings: &[GlobalThreadTimings],
        include_running: TasksIncluded,
    ) -> Vec<Self> {
        timings
            .iter()
            .filter_map(|t| match t.timings.upgrade() {
                Some(timings) => Some((t.thread_id, timings)),
                _ => None,
            })
            .map(|(thread_id, timings)| {
                let mut timings = timings.lock();
                let thread_name = timings.thread_name.clone();

                let mut stats = std::mem::take(&mut timings.stats);
                if let TasksIncluded::CompletedAndRunning = include_running
                    && let Some(ActiveTiming {
                        location,
                        spawned,
                        start,
                    }) = timings.running
                {
                    let end = YieldTime(Instant::now());
                    let timing = TaskTiming {
                        location,
                        spawned,
                        start,
                        end,
                    };
                    stats.add_runtime(timing);
                    stats.add_yield_timing(timing);
                }

                Self {
                    thread_name,
                    thread_id,
                    stats,
                }
            })
            .collect()
    }
}

/// Serializable variant of [`core::panic::Location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedLocation {
    /// Name of the source file
    pub file: SharedString,
    /// Line in the source file
    pub line: u32,
    /// Column in the source file
    pub column: u32,
}

impl From<&core::panic::Location<'static>> for SerializedLocation {
    fn from(value: &core::panic::Location<'static>) -> Self {
        SerializedLocation {
            file: value.file().into(),
            line: value.line(),
            column: value.column(),
        }
    }
}

/// Serializable variant of [`TaskTiming`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedTaskTiming {
    /// Location of the timing
    pub location: SerializedLocation,
    /// Time at which the measurement was reported in nanoseconds
    pub start: u128,
    /// Duration of the measurement in nanoseconds
    pub duration: u128,
}

impl SerializedTaskTiming {
    /// Convert an array of [`TaskTiming`] into their serializable format
    ///
    /// # Params
    ///
    /// `anchor` - [`Instant`] that should be earlier than all timings to use as base anchor
    pub fn convert(anchor: Instant, timings: &[TaskTiming]) -> Vec<SerializedTaskTiming> {
        let serialized = timings
            .iter()
            .map(|timing| {
                let start = timing.start.duration_since(anchor).as_nanos();
                let duration = timing.end.0.duration_since(timing.start).as_nanos();
                SerializedTaskTiming {
                    location: timing.location.into(),
                    start,
                    duration,
                }
            })
            .collect::<Vec<_>>();

        serialized
    }

    /// `anchor` - [`Instant`] that should be earlier than all timings to use as base anchor
    pub fn from(anchor: Instant, timing: TaskTiming) -> SerializedTaskTiming {
        let start = timing.start.duration_since(anchor).as_nanos();
        let duration = timing.end.0.duration_since(timing.start).as_nanos();
        SerializedTaskTiming {
            location: timing.location.into(),
            start,
            duration,
        }
    }
}

/// Serializable variant of [`ThreadTaskTimings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedThreadTaskTimings {
    /// Thread name
    pub thread_name: Option<String>,
    /// Hash of the thread id
    pub thread_id: u64,
    /// Timing records for this thread
    pub timings: Vec<SerializedTaskTiming>,
}

impl SerializedThreadTaskTimings {
    /// Convert [`ThreadTaskTimings`] into their serializable format
    ///
    /// # Params
    ///
    /// `anchor` - [`Instant`] that should be earlier than all timings to use as base anchor
    pub fn convert(anchor: Instant, timings: ThreadTaskTimings) -> SerializedThreadTaskTimings {
        let serialized_timings = SerializedTaskTiming::convert(anchor, &timings.timings);

        let mut hasher = DefaultHasher::new();
        timings.thread_id.hash(&mut hasher);
        let thread_id = hasher.finish();

        SerializedThreadTaskTimings {
            thread_name: timings.thread_name,
            thread_id,
            timings: serialized_timings,
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ThreadTimingsDelta {
    /// Hashed thread id
    pub thread_id: u64,
    /// Thread name, if known
    pub thread_name: Option<String>,
    /// New timings since the last call. If the circular buffer wrapped around
    /// since the previous poll, some entries may have been lost.
    pub new_timings: Vec<SerializedTaskTiming>,
}

/// Tracks which timing events have already been seen so that callers can request only unseen events.
#[doc(hidden)]
pub struct ProfilingCollector {
    startup_time: Instant,
    cursors: HashMap<ThreadId, u64>,
}

impl ProfilingCollector {
    pub fn new(startup_time: Instant) -> Self {
        Self {
            startup_time,
            cursors: HashMap::default(),
        }
    }

    pub fn startup_time(&self) -> Instant {
        self.startup_time
    }

    pub fn collect_unseen(
        &mut self,
        all_timings: Vec<ThreadTaskTimings>,
    ) -> Vec<ThreadTimingsDelta> {
        let mut deltas = Vec::with_capacity(all_timings.len());

        for thread in all_timings {
            let mut hasher = DefaultHasher::new();
            thread.thread_id.hash(&mut hasher);
            let hashed_id = hasher.finish();

            let prev_cursor = self.cursors.get(&thread.thread_id).copied().unwrap_or(0);
            let buffer_len = thread.timings.len() as u64;
            let buffer_start = thread.total_pushed.saturating_sub(buffer_len);

            let mut slice = if prev_cursor < buffer_start {
                // Cursor fell behind the buffer — some entries were evicted.
                // Return everything still in the buffer.
                thread.timings.as_slice()
            } else {
                let skip = (prev_cursor - buffer_start) as usize;
                &thread.timings[skip.min(thread.timings.len())..]
            };

            let cursor_advance = thread.total_pushed;
            self.cursors.insert(thread.thread_id, cursor_advance);

            if slice.is_empty() {
                continue;
            }

            let new_timings = SerializedTaskTiming::convert(self.startup_time, slice);

            deltas.push(ThreadTimingsDelta {
                thread_id: hashed_id,
                thread_name: thread.thread_name,
                new_timings,
            });
        }

        deltas
    }

    pub fn reset(&mut self) {
        self.cursors.clear();
    }
}

// Allow 16MiB of task timing entries.
// VecDeque grows by doubling its capacity when full, so keep this a power of 2 to avoid wasting
// memory.
#[cfg(feature = "profiler")]
const MAX_TASK_TIMINGS: usize = (16 * 1024 * 1024) / core::mem::size_of::<TaskTiming>();

#[doc(hidden)]
pub(crate) type TaskTimings = VecDeque<TaskTiming>;

#[doc(hidden)]
pub type GuardedTaskTimings = spin::Mutex<ThreadTimings>;

#[doc(hidden)]
pub struct GlobalThreadTimings {
    pub thread_id: ThreadId,
    pub timings: std::sync::Weak<GuardedTaskTimings>,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct TaskStatistics {
    pub poll_time_to_beat: Duration,
    pub runtime_to_beat: Duration,
    pub longest_poll_times: [TaskTiming; 5],
    pub longest_runtimes: [TaskTiming; 5],
}

impl std::fmt::Display for TaskStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Tasks that blocked the longest before yielding\n")?;
        for timing in self.longest_poll_times {
            f.write_fmt(format_args!(
                "{:<20} - {}:{}\n",
                format!("{:?}", timing.poll_duration()),
                timing.location.file(),
                timing.location.column()
            ))?;
        }
        f.write_str("Tasks that ran the longest\n")?;
        for timing in self.longest_runtimes {
            f.write_fmt(format_args!(
                "{:<20} - {}:{}\n",
                format!("{:?}", timing.since_spawn()),
                timing.location.file(),
                timing.location.column()
            ))?;
        }
        Ok(())
    }
}

impl Default for TaskStatistics {
    fn default() -> Self {
        Self {
            // Do not track polls that are not problematic
            // this keeps more calls on the fast path
            poll_time_to_beat: Duration::from_micros(100),
            runtime_to_beat: Duration::from_micros(100),
            longest_poll_times: [TaskTiming::placeholder(); 5],
            longest_runtimes: [TaskTiming::placeholder(); 5],
        }
    }
}

impl TaskStatistics {
    #[inline(always)]
    fn add_yield_timing(&mut self, task: TaskTiming) {
        let yielded_after = task.poll_duration();
        if yielded_after >= self.poll_time_to_beat {
            std::hint::cold_path(); // most tasks are not the worst, optimize for that
            let to_replace = self
                .longest_poll_times
                .iter()
                .position_min_by_key(|task| task.since_spawn())
                .expect("guarded by the comparison with nth_longest_yield_time");
            self.longest_poll_times[to_replace] = task;

            self.poll_time_to_beat = self
                .longest_poll_times
                .iter()
                .map(|task| task.since_spawn())
                .min()
                .expect("never empty");
        }
    }

    #[inline(always)]
    fn add_runtime(&mut self, task: TaskTiming) {
        let runtime = task.since_spawn();
        if runtime >= self.runtime_to_beat {
            std::hint::cold_path(); // most tasks are not the worst, optimize for that
            let to_replace = self
                .longest_runtimes
                .iter()
                .position_min_by_key(|task| task.since_spawn())
                .expect("guarded by the comparison with nth_longest_yield_time");
            self.longest_runtimes[to_replace] = task;

            self.runtime_to_beat = self
                .longest_runtimes
                .iter()
                .map(|task| task.since_spawn())
                .min()
                .expect("never empty");
        }
    }
}

#[doc(hidden)]
pub static GLOBAL_THREAD_TIMINGS: spin::Mutex<Vec<GlobalThreadTimings>> =
    spin::Mutex::new(Vec::new());

thread_local! {
    #[doc(hidden)]
    pub static THREAD_TIMINGS: LazyCell<Arc<GuardedTaskTimings>> = LazyCell::new(|| {
        let current_thread = std::thread::current();
        let thread_name = current_thread.name();
        let thread_id = current_thread.id();
        let timings = ThreadTimings::new(thread_name.map(|e| e.to_string()), thread_id);
        let timings = Arc::new(spin::Mutex::new(timings));

        {
            let timings = Arc::downgrade(&timings);
            let global_timings = GlobalThreadTimings {
                thread_id: std::thread::current().id(),
                timings,
            };
            GLOBAL_THREAD_TIMINGS.lock().push(global_timings);
        }

        timings
    });
}

#[doc(hidden)]
pub struct ThreadTimings {
    pub thread_name: Option<String>,
    pub thread_id: ThreadId,
    pub timings: TaskTimings,
    pub running: Option<ActiveTiming>,
    pub stats: TaskStatistics,
    pub total_pushed: u64,
}

impl ThreadTimings {
    pub fn new(thread_name: Option<String>, thread_id: ThreadId) -> Self {
        ThreadTimings {
            thread_name,
            thread_id,
            timings: TaskTimings::new(),
            stats: TaskStatistics::default(),
            total_pushed: 0,
            running: None,
        }
    }

    #[cfg(feature = "profiler")]
    pub fn update_running_task(
        &mut self,
        spawned: SpawnTime,
        location: &'static std::panic::Location<'_>,
    ) {
        let start = Instant::now();
        self.running = Some(ActiveTiming {
            spawned,
            location,
            start,
        });
    }
    #[cfg(not(feature = "profiler"))]
    pub fn update_running_task(&mut self, _: SpawnTime, _: &'static std::panic::Location<'_>) {}

    #[cfg(feature = "profiler")]
    pub fn save_task_timing(&mut self, ended: YieldTime) {
        let ActiveTiming {
            location,
            start,
            spawned,
        } = self
            .running
            .take()
            .expect("this function is only ever called after register_task_start");

        let timing = TaskTiming {
            location,
            spawned,
            start,
            end: ended,
        };
        self.stats.add_yield_timing(timing);
        self.stats.add_runtime(timing);

        if trace_enabled() {
            std::hint::cold_path(); // optimize for when the profiling is off
            if self.timings.len() >= MAX_TASK_TIMINGS {
                self.timings.pop_front();
            }
            self.timings.push_back(timing);
            self.total_pushed += 1;
        }
    }
    #[cfg(not(feature = "profiler"))]
    pub fn save_task_timing(&mut self, _: YieldTime) {}

    // Running tasks are included in the reliability trace, which is written
    // whenever the foreground executor makes no progress for > n seconds
    pub fn get_thread_task_timings(&self, includes: TasksIncluded) -> ThreadTaskTimings {
        ThreadTaskTimings {
            thread_name: self.thread_name.clone(),
            thread_id: self.thread_id,
            timings: self
                .timings
                .iter()
                .cloned()
                .chain(
                    self.running
                        .filter(|_| matches!(includes, TasksIncluded::CompletedAndRunning))
                        .map(|running| TaskTiming {
                            spawned: running.spawned,
                            location: running.location,
                            start: running.start,
                            end: YieldTime(Instant::now()),
                        }),
                )
                .collect(),
            stats: self.stats.clone(),
            total_pushed: self.total_pushed,
        }
    }
}

impl Drop for ThreadTimings {
    fn drop(&mut self) {
        let mut thread_timings = GLOBAL_THREAD_TIMINGS.lock();

        let Some((index, _)) = thread_timings
            .iter()
            .enumerate()
            .find(|(_, t)| t.thread_id == self.thread_id)
        else {
            return;
        };
        thread_timings.swap_remove(index);
    }
}

#[doc(hidden)]
pub fn update_running_task(spawned: SpawnTime, location: &'static std::panic::Location<'_>) {
    THREAD_TIMINGS.with(|timings| {
        timings.lock().update_running_task(spawned, location);
    });
}

#[doc(hidden)]
pub fn save_task_timing() {
    let yielded_at = YieldTime(Instant::now());
    THREAD_TIMINGS.with(|timings| {
        timings.lock().save_task_timing(yielded_at);
    });
}

#[doc(hidden)]
pub fn get_current_thread_task_timings(include_running: TasksIncluded) -> ThreadTaskTimings {
    THREAD_TIMINGS.with(|timings| timings.lock().get_thread_task_timings(include_running))
}

static PROFILER_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enables or disables task timing trace collection at runtime.
///
/// When transitioning from enabled to disabled, `add_task_timing` becomes a
/// cheaper since only cheap statistics are gathered. The existing per-thread
/// buffers for traces are cleared so stale data isn't reported after a later
/// re-enable. Calls with the current value are a no-op.
pub fn set_trace_enabled(enabled: bool) -> bool {
    if PROFILER_ENABLED.swap(enabled, Ordering::AcqRel) == enabled {
        return false;
    }

    if !enabled {
        for global in GLOBAL_THREAD_TIMINGS.lock().iter() {
            if let Some(timings) = global.timings.upgrade() {
                let mut timings = timings.lock();
                timings.timings.clear();
                timings.timings.shrink_to_fit();
                timings.total_pushed = 0;
            }
        }
    }
    true
}

/// Returns whether task timing tracing is enabled.
pub fn trace_enabled() -> bool {
    PROFILER_ENABLED.load(Ordering::Relaxed)
}
