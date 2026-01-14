use crate::{PlatformDispatcher, Priority, RunnableVariant};
use scheduler::{Clock, Scheduler, SessionId, TestScheduler, TestSchedulerConfig, Yield};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

/// TestDispatcher provides deterministic async execution for tests.
///
/// This implementation delegates task scheduling to the scheduler crate's `TestScheduler`.
/// Access the scheduler directly via `scheduler()` for clock, rng, and parking control.
#[doc(hidden)]
pub struct TestDispatcher {
    session_id: SessionId,
    scheduler: Arc<TestScheduler>,
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

        TestDispatcher {
            session_id,
            scheduler,
        }
    }

    pub fn scheduler(&self) -> &Arc<TestScheduler> {
        &self.scheduler
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn advance_clock(&self, by: Duration) {
        self.scheduler.advance_clock(by);
    }

    pub fn advance_clock_to_next_timer(&self) -> bool {
        self.scheduler.advance_clock_to_next_timer()
    }

    pub fn simulate_random_delay(&self) -> Yield {
        self.scheduler.yield_random()
    }

    pub fn tick(&self, background_only: bool) -> bool {
        if background_only {
            self.scheduler.tick_background_only()
        } else {
            self.scheduler.tick()
        }
    }

    pub fn run_until_parked(&self) {
        while self.tick(false) {}
    }
}

impl Clone for TestDispatcher {
    fn clone(&self) -> Self {
        let session_id = self.scheduler.allocate_session_id();
        Self {
            session_id,
            scheduler: self.scheduler.clone(),
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
        self.scheduler.is_main_thread()
    }

    fn now(&self) -> Instant {
        self.scheduler.clock().now()
    }

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        self.scheduler
            .schedule_background_with_priority(runnable, priority);
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        self.scheduler
            .schedule_foreground(self.session_id, runnable);
    }

    fn dispatch_after(&self, _duration: Duration, _runnable: RunnableVariant) {
        panic!(
            "dispatch_after should not be called in tests. \
            Use BackgroundExecutor::timer() which uses the scheduler's native timer."
        );
    }

    fn as_test(&self) -> Option<&TestDispatcher> {
        Some(self)
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        std::thread::spawn(move || {
            f();
        });
    }
}
