pub(crate) const DEFAULT_MAX_LOG_SYNC_REPEATS: usize = 4;
pub(crate) const LOG_SYNC_INTERVAL_MILLIS: u64 = 300;

pub(crate) const READ_LOG_FLUSH_POINT: usize = 64;
pub(crate) const WRITE_LOG_FLUSH_POINT: usize = 64;

// 384 elements
pub(crate) const READ_LOG_CH_SIZE: usize =
    READ_LOG_FLUSH_POINT * (DEFAULT_MAX_LOG_SYNC_REPEATS + 2);

// 384 elements
pub(crate) const WRITE_LOG_CH_SIZE: usize =
    WRITE_LOG_FLUSH_POINT * (DEFAULT_MAX_LOG_SYNC_REPEATS + 2);

// TODO: Calculate the batch size based on the number of entries in the cache (or an
// estimated number of entries to evict)
pub(crate) const DEFAULT_EVICTION_BATCH_SIZE: u32 = WRITE_LOG_CH_SIZE as u32;

/// The default timeout duration for the `run_pending_tasks` method.
pub(crate) const DEFAULT_MAINTENANCE_TASK_TIMEOUT_MILLIS: u64 = 100;

#[cfg(feature = "sync")]
pub(crate) const WRITE_RETRY_INTERVAL_MICROS: u64 = 50;
