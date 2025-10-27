use std::{
    cell::LazyCell,
    hash::Hasher,
    hash::{DefaultHasher, Hash},
    sync::Arc,
    thread::ThreadId,
    time::Instant,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
pub struct TaskTiming {
    pub location: &'static core::panic::Location<'static>,
    pub start: Instant,
    pub end: Instant,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ThreadTaskTimings {
    pub thread_id: ThreadId,
    pub timings: Vec<TaskTiming>,
}

impl ThreadTaskTimings {
    pub(crate) fn convert(timings: &[GlobalThreadTimings]) -> Vec<Self> {
        timings
            .iter()
            .filter_map(|t| match t.timings.upgrade() {
                Some(timings) => Some((t.thread_id, timings)),
                _ => None,
            })
            .map(|(thread_id, timings)| {
                let timings = &timings.lock().timings;

                let mut vec = Vec::with_capacity(timings.len());

                let (s1, s2) = timings.as_slices();
                vec.extend_from_slice(s1);
                vec.extend_from_slice(s2);

                ThreadTaskTimings {
                    thread_id,
                    timings: vec,
                }
            })
            .collect()
    }
}

/// Serializable variant of [`core::panic::Location`]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SerializedLocation<'a> {
    /// Name of the source file
    pub file: &'a str,
    /// Line in the source file
    pub line: u32,
    /// Column in the source file
    pub column: u32,
}

impl<'a> From<&'a core::panic::Location<'a>> for SerializedLocation<'a> {
    fn from(value: &'a core::panic::Location<'a>) -> Self {
        SerializedLocation {
            file: value.file(),
            line: value.line(),
            column: value.column(),
        }
    }
}

/// Serializable variant of [`TaskTiming`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedTaskTiming<'a> {
    /// Location of the timing
    #[serde(borrow)]
    pub location: SerializedLocation<'a>,
    /// Time at which the measurement was reported
    pub start: DateTime<Utc>,
    /// Duration of the measurement in nanoseconds
    pub duration: u128,
}

impl<'a> SerializedTaskTiming<'a> {
    /// Convert an array of [`TaskTiming`] into their serializable format
    pub fn convert(timings: &[TaskTiming]) -> Vec<SerializedTaskTiming<'static>> {
        let now = Utc::now();
        let instant_now = Instant::now();

        timings
            .iter()
            .map(|timing| {
                let start = now - instant_now.duration_since(timing.start);
                let duration = instant_now.duration_since(timing.end).as_nanos();
                SerializedTaskTiming {
                    location: timing.location.into(),
                    start,
                    duration,
                }
            })
            .collect()
    }
}

/// Serializable variant of [`ThreadTaskTimings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedThreadTaskTimings<'a> {
    /// Hash of the thread id
    pub thread_id: u64,
    /// Timing records for this thread
    #[serde(borrow)]
    pub timings: Vec<SerializedTaskTiming<'a>>,
}

impl<'a> SerializedThreadTaskTimings<'a> {
    /// Convert [`ThreadTaskTimings`] into their serializable format
    pub fn convert(timings: ThreadTaskTimings) -> SerializedThreadTaskTimings<'static> {
        let serialized_timings = SerializedTaskTiming::convert(&timings.timings);

        let mut hasher = DefaultHasher::new();
        timings.thread_id.hash(&mut hasher);
        let thread_id = hasher.finish();

        SerializedThreadTaskTimings {
            thread_id,
            timings: serialized_timings,
        }
    }
}

// Allow 20mb of task timing entries
const MAX_TASK_TIMINGS: usize = (20 * 1024 * 1024) / core::mem::size_of::<TaskTiming>();

pub(crate) type TaskTimings = circular_buffer::CircularBuffer<MAX_TASK_TIMINGS, TaskTiming>;
pub(crate) type GuardedTaskTimings = spin::Mutex<ThreadTimings>;

pub(crate) struct GlobalThreadTimings {
    pub thread_id: ThreadId,
    pub timings: std::sync::Weak<GuardedTaskTimings>,
}

pub(crate) static GLOBAL_THREAD_TIMINGS: spin::Mutex<Vec<GlobalThreadTimings>> =
    spin::Mutex::new(Vec::new());

thread_local! {
    pub(crate) static THREAD_TIMINGS: LazyCell<Arc<GuardedTaskTimings>> = LazyCell::new(|| {
        let thread_id = std::thread::current().id();
        let timings = ThreadTimings::new(thread_id);
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

pub(crate) struct ThreadTimings {
    pub thread_id: ThreadId,
    pub timings: Box<TaskTimings>,
}

impl ThreadTimings {
    pub(crate) fn new(thread_id: ThreadId) -> Self {
        ThreadTimings {
            thread_id,
            timings: TaskTimings::boxed(),
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
