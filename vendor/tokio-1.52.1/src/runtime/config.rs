#![cfg_attr(
    any(not(all(tokio_unstable, feature = "full")), target_family = "wasm"),
    allow(dead_code)
)]
use crate::runtime::{Callback, TaskCallback};
use crate::util::RngSeedGenerator;

pub(crate) struct Config {
    /// How many ticks before pulling a task from the global/remote queue?
    pub(crate) global_queue_interval: Option<u32>,

    /// How many ticks before yielding to the driver for timer and I/O events?
    pub(crate) event_interval: u32,

    /// Callback for a worker parking itself
    pub(crate) before_park: Option<Callback>,

    /// Callback for a worker unparking itself
    pub(crate) after_unpark: Option<Callback>,

    /// To run before each task is spawned.
    pub(crate) before_spawn: Option<TaskCallback>,

    /// To run after each task is terminated.
    pub(crate) after_termination: Option<TaskCallback>,

    /// To run before each poll
    #[cfg(tokio_unstable)]
    pub(crate) before_poll: Option<TaskCallback>,

    /// To run after each poll
    #[cfg(tokio_unstable)]
    pub(crate) after_poll: Option<TaskCallback>,

    /// The multi-threaded scheduler includes a per-worker LIFO slot used to
    /// store the last scheduled task. This can improve certain usage patterns,
    /// especially message passing between tasks.
    ///
    /// In Tokio versions before 1.51, tasks in the LIFO slot could not be
    /// stolen, which could cause issues in applications with long poll times.
    /// As a stop-gap, this unstable option lets users disable the LIFO task.
    /// Now that the LIFO slot is stealable, we may remove this option in a
    /// future version.
    pub(crate) disable_lifo_slot: bool,

    /// Random number generator seed to configure runtimes to act in a
    /// deterministic way.
    pub(crate) seed_generator: RngSeedGenerator,

    /// How to build poll time histograms
    pub(crate) metrics_poll_count_histogram: Option<crate::runtime::HistogramBuilder>,

    #[cfg(tokio_unstable)]
    /// How to respond to unhandled task panics.
    pub(crate) unhandled_panic: crate::runtime::UnhandledPanic,

    /// If `true`, an idle worker is woken whenever a worker thread transitions
    /// from polling the I/O driver to polling its own tasks (requires
    /// `tokio_unstable`).
    pub(crate) enable_eager_driver_handoff: bool,
}
