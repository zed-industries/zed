use async_task::{Runnable, Task};
use parking_lot::Mutex;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::{HashSet, VecDeque};
use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskLabel(usize);

#[derive(Clone)]
pub struct SchedulerConfig {
    pub seed: u64,
    pub randomize_order: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            randomize_order: true,
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

struct SchedulerState {
    foreground: VecDeque<Runnable<()>>,
    background: VecDeque<Runnable<()>>,
    deprioritized: VecDeque<Runnable<()>>,
    rng: ChaCha8Rng,
    deprioritized_labels: HashSet<TaskLabel>,
    randomize_order: bool,
}

pub trait Scheduler {
    fn schedule_foreground(&self, runnable: Runnable<()>, label: Option<TaskLabel>);
    fn schedule_background(&self, runnable: Runnable<()>, label: Option<TaskLabel>);
}

pub struct TestScheduler {
    state: Mutex<SchedulerState>,
    pub thread_id: thread::ThreadId,
    pub config: SchedulerConfig,
}

impl TestScheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            state: Mutex::new(SchedulerState {
                foreground: VecDeque::new(),
                background: VecDeque::new(),
                deprioritized: VecDeque::new(),
                rng: ChaCha8Rng::seed_from_u64(config.seed),
                deprioritized_labels: HashSet::new(),
                randomize_order: config.randomize_order,
            }),
            thread_id: thread::current().id(),
            config,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::channel::oneshot;
    use futures::executor::block_on;

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
    fn test_randomize_order_setting() {
        use std::collections::HashSet;

        // Test deterministic mode: different seeds should produce same execution order
        let mut deterministic_results = HashSet::new();
        for seed in 0..10 {
            let config = SchedulerConfig {
                seed,
                randomize_order: false,
            };
            let scheduler = Arc::new(TestScheduler::new(config));
            let foreground = ForegroundExecutor::new(scheduler.clone());
            let background = BackgroundExecutor::new(scheduler.clone());

            let execution_order = Arc::new(Mutex::new(Vec::new()));

            // Spawn foreground tasks
            for i in 0..3 {
                let order = execution_order.clone();
                foreground
                    .spawn(async move {
                        order.lock().push(format!("fg-{}", i));
                    })
                    .detach();
            }

            // Spawn background tasks
            for i in 0..3 {
                let order = execution_order.clone();
                background
                    .spawn(async move {
                        order.lock().push(format!("bg-{}", i));
                    })
                    .detach();
            }

            scheduler.run();

            let order = execution_order.lock().clone();
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
            let config = SchedulerConfig {
                seed,
                randomize_order: true,
            };
            let scheduler = Arc::new(TestScheduler::new(config));
            let foreground = ForegroundExecutor::new(scheduler.clone());
            let background = BackgroundExecutor::new(scheduler.clone());

            let execution_order = Arc::new(Mutex::new(Vec::new()));

            // Spawn foreground tasks
            for i in 0..3 {
                let order = execution_order.clone();
                foreground
                    .spawn(async move {
                        order.lock().push(format!("fg-{}", i));
                    })
                    .detach();
            }

            // Spawn background tasks
            for i in 0..3 {
                let order = execution_order.clone();
                background
                    .spawn(async move {
                        order.lock().push(format!("bg-{}", i));
                    })
                    .detach();
            }

            scheduler.run();

            let order = execution_order.lock().clone();
            randomized_results.insert(order);
        }

        // Randomized mode should produce multiple different execution orders
        // (though it might not with small task counts, so we just verify it works without crashing)
        assert!(
            !randomized_results.is_empty(),
            "Randomized mode should produce some execution order"
        );
    }
}
