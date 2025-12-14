use crate::{PlatformDispatcher, Priority, RunnableVariant, TaskLabel};
use parking::Unparker;
use parking_lot::Mutex;
use rand::prelude::*;
use scheduler::{Clock, Scheduler, SessionId, TestScheduler, TestSchedulerConfig};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
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
    state: Arc<Mutex<TestDispatcherState>>,
}

struct TestDispatcherState {
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
            unparkers: Vec::new(),
        };

        TestDispatcher {
            session_id,
            scheduler,
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub fn scheduler(&self) -> &Arc<TestScheduler> {
        &self.scheduler
    }

    pub fn advance_clock(&self, by: Duration) {
        self.scheduler.advance_clock(by);
    }

    pub fn advance_clock_to_next_timer(&self) -> bool {
        self.scheduler.advance_clock_to_next_timer()
    }

    pub fn simulate_random_delay(&self) -> impl 'static + Send + Future<Output = ()> + use<> {
        struct YieldNow {
            count: usize,
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
        if background_only {
            self.scheduler.tick_background_only()
        } else {
            self.scheduler.tick()
        }
    }

    pub fn run_until_parked(&self) {
        while self.tick(false) {}
    }

    pub fn unpark_all(&self) {
        let unparkers: Vec<_> = self.state.lock().unparkers.drain(..).collect();
        for unparker in unparkers {
            unparker.unpark();
        }
    }

    pub fn push_unparker(&self, unparker: Unparker) {
        self.state.lock().unparkers.push(unparker);
    }

    pub fn unparker_count(&self) -> usize {
        self.state.lock().unparkers.len()
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
        self.scheduler.is_main_thread()
    }

    fn now(&self) -> Instant {
        self.scheduler.clock().now()
    }

    fn dispatch(&self, runnable: RunnableVariant, _label: Option<TaskLabel>, priority: Priority) {
        self.scheduler
            .schedule_background_with_priority(runnable, priority);
        self.unpark_all();
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        self.scheduler.schedule_foreground(self.session_id, runnable);
        self.unpark_all();
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

    fn spawn_realtime(&self, _priority: crate::RealtimePriority, _f: Box<dyn FnOnce() + Send>) {
        panic!(
            "spawn_realtime is not supported in TestDispatcher. \
            Real OS threads break test determinism - tests would become \
            flaky and unreproducible even with the same SEED. \
            Use a different Priority (High, Medium, Low) instead."
        );
    }
}
