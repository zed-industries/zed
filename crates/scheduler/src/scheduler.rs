mod executor;

pub use executor::*;

use async_task::Runnable;
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SessionId(u16);

#[derive(Clone)]
pub struct SchedulerConfig {
    pub seed: u64,
    pub randomize_order: bool,
    pub allow_parking: bool,
}

impl SchedulerConfig {
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            randomize_order: true,
            allow_parking: false,
        }
    }
}

struct ScheduledTask {
    session_id: Option<SessionId>,
    runnable: Runnable,
}

struct SchedulerState {
    tasks: VecDeque<ScheduledTask>,
    rng: ChaCha8Rng,
    randomize_order: bool,
    allow_parking: bool,
    next_session_id: SessionId,
}

pub trait Scheduler: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn schedule_foreground(&self, session_id: SessionId, runnable: Runnable);
    fn schedule_background(&self, runnable: Runnable);
    fn park(&self, timeout: Option<Duration>) -> bool;
    fn unparker(&self) -> Unparker;
    fn step(&self) -> bool;
}

pub struct TestScheduler {
    state: Mutex<SchedulerState>,
    pub thread_id: thread::ThreadId,
    pub config: SchedulerConfig,
    parker: Arc<Mutex<Parker>>,
    unparker: Unparker,
}

impl TestScheduler {
    /// Run a test once with default configuration (seed 0)
    pub fn once<R>(f: impl AsyncFnOnce(Arc<TestScheduler>) -> R) -> R {
        Self::with_seed(0, f)
    }

    /// Run a test multiple times with sequential seeds (0, 1, 2, ...)
    pub fn many<R>(iterations: usize, mut f: impl AsyncFnMut(Arc<TestScheduler>) -> R) -> Vec<R> {
        (0..iterations as u64)
            .map(|i| Self::with_seed(i, &mut f))
            .collect()
    }

    /// Run a test once with a specific seed
    pub fn with_seed<R>(seed: u64, f: impl AsyncFnOnce(Arc<TestScheduler>) -> R) -> R {
        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::with_seed(seed)));
        let background = BackgroundExecutor::new(scheduler.clone());
        let future = f(scheduler.clone());
        background.block(future)
    }

    pub fn new(config: SchedulerConfig) -> Self {
        let (parker, unparker) = parking::pair();
        Self {
            state: Mutex::new(SchedulerState {
                tasks: VecDeque::new(),
                rng: ChaCha8Rng::seed_from_u64(config.seed),
                randomize_order: config.randomize_order,
                allow_parking: config.allow_parking,
                next_session_id: SessionId(0),
            }),
            thread_id: thread::current().id(),
            config,
            parker: Arc::new(Mutex::new(parker)),
            unparker,
        }
    }

    pub fn run(&self) {
        while self.step() {
            // Continue stepping until no work remains
        }
    }

    pub fn random_delay(&self) -> Yield {
        Yield {
            count: self.state.lock().rng.gen_range(0..=10),
        }
    }

    /// Create a foreground executor for this scheduler
    pub fn foreground(self: &Arc<Self>) -> ForegroundExecutor {
        let session_id = {
            let mut state = self.state.lock();
            state.next_session_id.0 += 1;
            state.next_session_id
        };
        ForegroundExecutor::new(session_id, self.clone())
    }

    /// Create a background executor for this scheduler
    pub fn background(self: &Arc<Self>) -> BackgroundExecutor {
        BackgroundExecutor::new(self.clone())
    }
}

impl Scheduler for TestScheduler {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.thread_id
    }

    fn schedule_foreground(&self, session_id: SessionId, runnable: Runnable) {
        {
            let state = &mut *self.state.lock();
            let ix = if state.randomize_order {
                let start_ix = state
                    .tasks
                    .iter()
                    .rposition(|task| task.session_id == Some(session_id))
                    .map_or(0, |ix| ix + 1);
                state.rng.gen_range(start_ix..=state.tasks.len())
            } else {
                state.tasks.len()
            };
            state.tasks.insert(
                ix,
                ScheduledTask {
                    session_id: Some(session_id),
                    runnable,
                },
            );
        }
        self.unparker.unpark();
    }

    fn schedule_background(&self, runnable: Runnable) {
        {
            let state = &mut *self.state.lock();
            let ix = if state.randomize_order {
                state.rng.gen_range(0..=state.tasks.len())
            } else {
                state.tasks.len()
            };
            state.tasks.insert(
                ix,
                ScheduledTask {
                    session_id: None,
                    runnable,
                },
            );
        }
        self.unparker.unpark();
    }

    fn park(&self, timeout: Option<Duration>) -> bool {
        {
            let state = self.state.lock();
            if !state.allow_parking {
                drop(state);
                panic!("Parking forbidden");
            }
        }

        if let Some(duration) = timeout {
            self.parker.lock().park_timeout(duration);
        } else {
            self.parker.lock().park();
        }
        true
    }

    fn unparker(&self) -> Unparker {
        self.unparker.clone()
    }

    fn step(&self) -> bool {
        let mut state = self.state.lock();
        if let Some(task) = state.tasks.pop_front() {
            drop(state);
            task.runnable.run();
            true
        } else {
            false
        }
    }
}

pub struct Yield {
    count: usize,
}

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.count == 0 {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            self.count -= 1;
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::channel::{mpsc, oneshot};
    use futures::executor::block_on;
    use futures::sink::SinkExt;
    use futures::stream::StreamExt;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::pin::Pin;
    use std::rc::Rc;
    use std::task::{Context, Poll};

    #[test]
    fn test_foreground_executor_spawn() {
        let result = TestScheduler::once(async |scheduler| {
            let task = scheduler.foreground().spawn(async move { 42 });
            task.await
        });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_background_executor_spawn() {
        TestScheduler::once(async |scheduler| {
            let task = scheduler.background().spawn(async move { 42 });
            let result = task.await;
            assert_eq!(result, 42);
        });
    }

    #[test]
    fn test_foreground_ordering() {
        let mut traces = HashSet::new();

        TestScheduler::many(1000, async |scheduler| {
            #[derive(Hash, PartialEq, Eq)]
            struct TraceEntry {
                session: usize,
                task: usize,
            }

            let trace = Rc::new(RefCell::new(Vec::new()));

            let foreground_1 = scheduler.foreground();
            for task in 0..10 {
                foreground_1
                    .spawn({
                        let trace = trace.clone();
                        async move {
                            trace.borrow_mut().push(TraceEntry { session: 0, task });
                        }
                    })
                    .detach();
            }

            let foreground_2 = scheduler.foreground();
            for task in 0..10 {
                foreground_2
                    .spawn({
                        let trace = trace.clone();
                        async move {
                            trace.borrow_mut().push(TraceEntry { session: 1, task });
                        }
                    })
                    .detach();
            }

            scheduler.run();

            assert_eq!(
                trace
                    .borrow()
                    .iter()
                    .filter(|entry| entry.session == 0)
                    .map(|entry| entry.task)
                    .collect::<Vec<_>>(),
                (0..10).collect::<Vec<_>>()
            );
            assert_eq!(
                trace
                    .borrow()
                    .iter()
                    .filter(|entry| entry.session == 1)
                    .map(|entry| entry.task)
                    .collect::<Vec<_>>(),
                (0..10).collect::<Vec<_>>()
            );

            traces.insert(trace.take());
        });

        assert!(traces.len() > 1, "Expected at least two traces");
    }

    #[test]
    fn test_send_from_bg_to_fg() {
        TestScheduler::once(async |scheduler| {
            let foreground = scheduler.foreground();
            let background = scheduler.background();

            let (sender, receiver) = oneshot::channel::<i32>();

            background
                .spawn(async move {
                    sender.send(42).unwrap();
                })
                .detach();

            let task = foreground.spawn(async move { receiver.await.unwrap() });
            let result = task.await;
            assert_eq!(result, 42);
        });
    }

    #[test]
    fn test_randomize_order() {
        // Test deterministic mode: different seeds should produce same execution order
        let mut deterministic_results = HashSet::new();
        for seed in 0..10 {
            let config = SchedulerConfig {
                seed,
                randomize_order: false,
                ..Default::default()
            };
            let order = block_on(capture_execution_order(config));
            assert_eq!(order.len(), 6);
            deterministic_results.insert(order);
        }

        // All deterministic runs should produce the same result
        assert_eq!(
            deterministic_results.len(),
            1,
            "Deterministic mode should always produce same execution order"
        );

        // Test randomized mode: different seeds can produce different execution orders
        let mut randomized_results = HashSet::new();
        for seed in 0..20 {
            let config = SchedulerConfig::with_seed(seed);
            let order = block_on(capture_execution_order(config));
            assert_eq!(order.len(), 6);
            randomized_results.insert(order);
        }

        // Randomized mode should produce multiple different execution orders
        assert!(
            randomized_results.len() > 1,
            "Randomized mode should produce multiple different orders"
        );
    }

    async fn capture_execution_order(config: SchedulerConfig) -> Vec<String> {
        let scheduler = Arc::new(TestScheduler::new(config));
        let foreground = scheduler.foreground();
        let background = scheduler.background();

        let (sender, receiver) = mpsc::unbounded::<String>();

        // Spawn foreground tasks
        for i in 0..3 {
            let mut sender = sender.clone();
            foreground
                .spawn(async move {
                    sender.send(format!("fg-{}", i)).await.ok();
                })
                .detach();
        }

        // Spawn background tasks
        for i in 0..3 {
            let mut sender = sender.clone();
            background
                .spawn(async move {
                    sender.send(format!("bg-{}", i)).await.ok();
                })
                .detach();
        }

        drop(sender); // Close sender to signal no more messages
        scheduler.run();

        receiver.collect().await
    }

    #[test]
    fn test_block() {
        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::default()));
        let executor = BackgroundExecutor::new(scheduler.clone());
        let (tx, rx) = oneshot::channel();

        // Spawn background task to send value
        let _ = executor
            .spawn(async move {
                tx.send(42).unwrap();
            })
            .detach();

        // Block on receiving the value
        let result = executor.block(async { rx.await.unwrap() });
        assert_eq!(result, 42);
    }

    #[test]
    #[should_panic(expected = "Parking forbidden")]
    fn test_parking_panics() {
        // Custom future that yields indefinitely without completing
        struct NeverFuture;

        impl Future for NeverFuture {
            type Output = ();

            fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
                Poll::Pending
            }
        }

        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::default()));
        let executor = BackgroundExecutor::new(scheduler);
        executor.block(NeverFuture);
    }

    #[test]
    fn test_block_with_parking() {
        let config = SchedulerConfig {
            allow_parking: true,
            ..Default::default()
        };
        let scheduler = Arc::new(TestScheduler::new(config));
        let executor = BackgroundExecutor::new(scheduler.clone());
        let (tx, rx) = oneshot::channel();

        // Spawn background task to send value
        let _ = executor
            .spawn(async move {
                tx.send(42).unwrap();
            })
            .detach();

        // Block on receiving the value (will park if needed)
        let result = executor.block(async { rx.await.unwrap() });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_helper_methods() {
        // Test the once method
        let result = TestScheduler::once(async |scheduler: Arc<TestScheduler>| {
            let background = scheduler.background();
            background.spawn(async { 42 }).await
        });
        assert_eq!(result, 42);

        // Test the many method
        let results = TestScheduler::many(3, async |scheduler: Arc<TestScheduler>| {
            let background = scheduler.background();
            background.spawn(async { 10 }).await
        });
        assert_eq!(results, vec![10, 10, 10]);

        // Test the with_seed method
        let result = TestScheduler::with_seed(123, async |scheduler: Arc<TestScheduler>| {
            let background = scheduler.background();

            // Spawn a background task and wait for its result
            let task = background.spawn(async { 99 });
            task.await
        });
        assert_eq!(result, 99);
    }
}
