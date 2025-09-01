use async_task::{Runnable, Task};
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::{HashSet, VecDeque};
use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Poll, Waker};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskLabel(usize);

#[derive(Clone)]
pub struct SchedulerConfig {
    pub seed: u64,
    pub randomize_order: bool,
    pub allow_parking: bool,
}

impl SchedulerConfig {
    pub fn from_seed(seed: u64) -> Self {
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

impl TaskLabel {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT_TASK_LABEL: AtomicUsize = AtomicUsize::new(1);
        Self(NEXT_TASK_LABEL.fetch_add(1, Ordering::SeqCst))
    }
}

struct CustomWaker {
    unparker: Unparker,
}

impl std::task::Wake for CustomWaker {
    fn wake(self: Arc<Self>) {
        self.unparker.unpark();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.unparker.unpark();
    }
}

struct SchedulerState {
    foreground: VecDeque<Runnable<()>>,
    background: VecDeque<Runnable<()>>,
    deprioritized: VecDeque<Runnable<()>>,
    rng: ChaCha8Rng,
    deprioritized_labels: HashSet<TaskLabel>,
    randomize_order: bool,
    allow_parking: bool,
}

pub trait Scheduler {
    fn schedule_foreground(&self, runnable: Runnable<()>, label: Option<TaskLabel>);
    fn schedule_background(&self, runnable: Runnable<()>, label: Option<TaskLabel>);
    fn park(&self, timeout: Option<Duration>) -> bool;
    fn unparker(&self) -> Unparker;
}

pub struct TestScheduler {
    state: Mutex<SchedulerState>,
    pub thread_id: thread::ThreadId,
    pub config: SchedulerConfig,
    parker: Arc<Mutex<Parker>>,
    unparker: Unparker,
}

impl TestScheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        let (parker, unparker) = parking::pair();
        Self {
            state: Mutex::new(SchedulerState {
                foreground: VecDeque::new(),
                background: VecDeque::new(),
                deprioritized: VecDeque::new(),
                rng: ChaCha8Rng::seed_from_u64(config.seed),
                deprioritized_labels: HashSet::new(),
                randomize_order: config.randomize_order,
                allow_parking: config.allow_parking,
            }),
            thread_id: thread::current().id(),
            config,
            parker: Arc::new(Mutex::new(parker)),
            unparker,
        }
    }

    pub fn is_main_thread(&self) -> bool {
        thread::current().id() == self.thread_id
    }

    pub fn deprioritize(&self, label: TaskLabel) {
        self.state.lock().deprioritized_labels.insert(label);
    }

    pub fn run(&self) {
        while self.step() {
            // Continue stepping until no work remains
        }
    }
}

impl Scheduler for TestScheduler {
    fn schedule_foreground(&self, runnable: Runnable<()>, _label: Option<TaskLabel>) {
        self.state.lock().foreground.push_back(runnable);
        self.unparker.unpark();
    }

    fn schedule_background(&self, runnable: Runnable<()>, label: Option<TaskLabel>) {
        let mut state = self.state.lock();
        if let Some(ref lbl) = label {
            if state.deprioritized_labels.contains(lbl) {
                state.deprioritized.push_back(runnable);
                return;
            }
        }
        state.background.push_back(runnable);
        drop(state);
        self.unparker.unpark();
    }

    fn park(&self, timeout: Option<Duration>) -> bool {
        let state = self.state.lock();
        if !state.allow_parking {
            drop(state);
            panic!("Parking forbidden");
        }
        drop(state);

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
}

impl TestScheduler {
    pub fn step(&self) -> bool {
        let mut state = self.state.lock();
        let foreground_count = state.foreground.len();
        let background_count = state.background.len();

        if foreground_count > 0 || background_count > 0 {
            if !state.randomize_order {
                // Deterministic: prefer foreground if available, else background
                if foreground_count > 0 {
                    let runnable = state.foreground.pop_front().unwrap();
                    drop(state);
                    runnable.run();
                    return true;
                } else if background_count > 0 {
                    let runnable = state.background.pop_front().unwrap();
                    drop(state);
                    runnable.run();
                    return true;
                }
            } else {
                // Weighted random selection between foreground and background, like GPUI
                let total_count = foreground_count + background_count;
                let should_pick_foreground = state
                    .rng
                    .gen_ratio(foreground_count as u32, total_count as u32);

                if should_pick_foreground && foreground_count > 0 {
                    let runnable = state.foreground.pop_front().unwrap();
                    drop(state);
                    runnable.run();
                    return true;
                } else if background_count > 0 {
                    let runnable = state.background.pop_front().unwrap();
                    drop(state);
                    runnable.run();
                    return true;
                }
            }
            false
        } else if !state.deprioritized.is_empty() {
            // Only when foreground/background empty, run deprioritized
            let runnable = state.deprioritized.pop_front().unwrap();
            drop(state);
            runnable.run();
            true
        } else {
            // No work available
            false
        }
    }
}

pub struct ForegroundExecutor {
    scheduler: Arc<TestScheduler>,
    not_send: PhantomData<Rc<()>>,
}

impl ForegroundExecutor {
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let scheduler = Arc::clone(&self.scheduler);
        let (runnable, task) = async_task::spawn_local(future, move |runnable| {
            scheduler.schedule_foreground(runnable, None);
        });
        runnable.schedule();
        task
    }
}

impl ForegroundExecutor {
    pub fn new(scheduler: Arc<TestScheduler>) -> Self {
        assert!(
            scheduler.is_main_thread(),
            "ForegroundExecutor must be created on the same thread as the Scheduler"
        );
        Self {
            scheduler,
            not_send: PhantomData,
        }
    }
}

impl BackgroundExecutor {
    pub fn new(scheduler: Arc<TestScheduler>) -> Self {
        Self { scheduler }
    }
}

pub struct BackgroundExecutor {
    scheduler: Arc<TestScheduler>,
}

impl BackgroundExecutor {
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let scheduler = Arc::clone(&self.scheduler);
        let (runnable, task) = async_task::spawn(future, move |runnable| {
            scheduler.schedule_background(runnable, None);
        });
        runnable.schedule();
        task
    }

    pub fn spawn_labeled<F>(&self, future: F, label: TaskLabel) -> Task<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let scheduler = Arc::clone(&self.scheduler);
        let (runnable, task) = async_task::spawn(future, move |runnable| {
            scheduler.schedule_background(runnable, Some(label));
        });
        runnable.schedule();
        task
    }

    pub fn block<Fut: Future>(&self, future: Fut) -> Fut::Output {
        let mut future = Box::pin(future);

        loop {
            let waker = Waker::from(Arc::new(CustomWaker {
                unparker: self.scheduler.unparker(),
            }));
            let mut cx = std::task::Context::from_waker(&waker);

            match future.as_mut().poll(&mut cx) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    if self.scheduler.step() {
                        continue;
                    }
                    self.scheduler.park(None);
                }
            }
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
    use std::collections::HashSet;
    use std::pin::Pin;
    use std::task::Context;

    #[test]
    fn test_foreground_executor_spawn() {
        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::default()));
        let executor = ForegroundExecutor::new(scheduler.clone());
        let task = executor.spawn(async move { 42 });
        scheduler.run();

        // Block on the task to ensure it resolves
        let result = block_on(task);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_background_executor_spawn() {
        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::default()));
        let executor = BackgroundExecutor::new(scheduler.clone());
        let task = executor.spawn(async move { 42 });
        scheduler.run();

        // Block on the task to ensure it resolves
        let result = block_on(task);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_send_from_bg_to_fg() {
        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::default()));
        let foreground = ForegroundExecutor::new(scheduler.clone());
        let background = BackgroundExecutor::new(scheduler.clone());

        let (sender, receiver) = oneshot::channel::<i32>();

        background
            .spawn(async move {
                sender.send(42).unwrap();
            })
            .detach();

        let task = foreground.spawn(async move { receiver.await.unwrap() });

        scheduler.run();

        let result = block_on(task);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_deprioritize_task() {
        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig {
            randomize_order: false,
            ..Default::default()
        }));
        let background = BackgroundExecutor::new(scheduler.clone());

        let label = TaskLabel::new();

        let (sender, receiver) = mpsc::unbounded::<i32>();

        // Spawn first background task
        {
            background
                .spawn({
                    let mut sender = sender.clone();
                    async move {
                        sender.send(1).await.ok();
                    }
                })
                .detach();
        }

        // Deprioritize the middle task's label before spawning it
        scheduler.deprioritize(label);

        // Spawn second (deprioritized) background task
        {
            background
                .spawn_labeled(
                    {
                        let mut sender = sender.clone();
                        async move {
                            sender.send(2).await.ok();
                        }
                    },
                    label,
                )
                .detach();
        }

        // Spawn third background task
        {
            background
                .spawn({
                    let mut sender = sender.clone();
                    async move {
                        sender.send(3).await.ok();
                    }
                })
                .detach();
        }

        drop(sender); // Close sender to signal no more messages
        scheduler.run();

        let order: Vec<i32> = block_on(receiver.collect());
        assert_eq!(order, vec![1, 3, 2]);
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
            let config = SchedulerConfig::from_seed(seed);
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
        let foreground = ForegroundExecutor::new(scheduler.clone());
        let background = BackgroundExecutor::new(scheduler.clone());

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
}
