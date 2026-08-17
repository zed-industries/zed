use std::time::Duration;

pub(crate) mod builder_utils;
pub(crate) mod concurrent;
pub(crate) mod deque;
pub(crate) mod entry;
pub(crate) mod error;
pub(crate) mod frequency_sketch;
pub(crate) mod iter;
pub(crate) mod time;
pub(crate) mod timer_wheel;

#[cfg(test)]
pub(crate) mod test_utils;

use self::concurrent::constants::{
    DEFAULT_EVICTION_BATCH_SIZE, DEFAULT_MAINTENANCE_TASK_TIMEOUT_MILLIS,
    DEFAULT_MAX_LOG_SYNC_REPEATS,
};

// Note: `CacheRegion` cannot have more than four enum variants. This is because
// `crate::{sync,unsync}::DeqNodes` uses a `tagptr::TagNonNull<DeqNode<T>, 2>`
// pointer, where the 2-bit tag is `CacheRegion`.
#[derive(Clone, Copy, Debug, Eq)]
pub(crate) enum CacheRegion {
    Window = 0,
    MainProbation = 1,
    MainProtected = 2,
    Other = 3,
}

impl From<usize> for CacheRegion {
    fn from(n: usize) -> Self {
        match n {
            0 => Self::Window,
            1 => Self::MainProbation,
            2 => Self::MainProtected,
            3 => Self::Other,
            _ => panic!("No such CacheRegion variant for {n}"),
        }
    }
}

impl CacheRegion {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Window => "window",
            Self::MainProbation => "main probation",
            Self::MainProtected => "main protected",
            Self::Other => "other",
        }
    }
}

impl PartialEq<Self> for CacheRegion {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialEq<usize> for CacheRegion {
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HousekeeperConfig {
    /// The timeout duration for the `run_pending_tasks` method. This is a safe-guard
    /// to prevent cache read/write operations (that may call `run_pending_tasks`
    /// internally) from being blocked for a long time when the user wrote a slow
    /// eviction listener closure.
    ///
    /// Used only when the eviction listener closure is set for the cache instance.
    ///
    /// Default: `DEFAULT_MAINTENANCE_TASK_TIMEOUT_MILLIS`
    pub(crate) maintenance_task_timeout: Duration,
    /// The maximum repeat count for receiving operation logs from the read and write
    /// log channels. Default: `MAX_LOG_SYNC_REPEATS`.
    pub(crate) max_log_sync_repeats: u32,
    /// The batch size of entries to be processed by each internal eviction method.
    /// Default: `EVICTION_BATCH_SIZE`.
    pub(crate) eviction_batch_size: u32,
}

impl Default for HousekeeperConfig {
    fn default() -> Self {
        Self {
            maintenance_task_timeout: Duration::from_millis(
                DEFAULT_MAINTENANCE_TASK_TIMEOUT_MILLIS,
            ),
            max_log_sync_repeats: DEFAULT_MAX_LOG_SYNC_REPEATS as u32,
            eviction_batch_size: DEFAULT_EVICTION_BATCH_SIZE,
        }
    }
}

impl HousekeeperConfig {
    #[cfg(test)]
    pub(crate) fn new(
        maintenance_task_timeout: Option<Duration>,
        max_log_sync_repeats: Option<u32>,
        eviction_batch_size: Option<u32>,
    ) -> Self {
        Self {
            maintenance_task_timeout: maintenance_task_timeout.unwrap_or(Duration::from_millis(
                DEFAULT_MAINTENANCE_TASK_TIMEOUT_MILLIS,
            )),
            max_log_sync_repeats: max_log_sync_repeats
                .unwrap_or(DEFAULT_MAX_LOG_SYNC_REPEATS as u32),
            eviction_batch_size: eviction_batch_size.unwrap_or(DEFAULT_EVICTION_BATCH_SIZE),
        }
    }
}

// Ensures the value fits in a range of `128u32..=u32::MAX`.
pub(crate) fn sketch_capacity(max_capacity: u64) -> u32 {
    max_capacity.try_into().unwrap_or(u32::MAX).max(128)
}

#[cfg(test)]
pub(crate) fn available_parallelism() -> usize {
    use std::{num::NonZeroUsize, thread::available_parallelism};
    available_parallelism().map(NonZeroUsize::get).unwrap_or(1)
}
