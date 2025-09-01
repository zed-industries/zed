use anyhow::Result;
use async_task::Runnable;
use parking_lot::Mutex;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::any::Any;
use std::collections::VecDeque;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;

use futures::channel::oneshot;
use futures::executor;
use std::thread::{self, ThreadId};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TaskLabel(usize);

pub trait Scheduler: Send + Sync + Any {
    fn schedule(&self, runnable: Runnable, label: Option<TaskLabel>);
    fn schedule_foreground(&self, runnable: Runnable, label: Option<TaskLabel>);
    fn is_main_thread(&self) -> bool;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(usize);

pub struct Task<R>(async_task::Task<R>);

impl<R> Task<R> {
    pub fn id(&self) -> TaskId {
        TaskId(0) // Placeholder
    }
}

impl Default for TaskLabel {
    fn default() -> Self {
        TaskLabel(0)
    }
}

pub struct SchedulerConfig {
    pub randomize_order: bool,
    pub seed: u64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            randomize_order: true,
            seed: 0,
        }
    }
}

pub struct TestScheduler {
    inner: Mutex<TestSchedulerInner>,
}

struct TestSchedulerInner {
    rng: ChaCha8Rng,
    foreground_queue: VecDeque<Runnable>,
    creation_thread_id: ThreadId,
}

impl TestScheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            inner: Mutex::new(TestSchedulerInner {
                rng: ChaCha8Rng::seed_from_u64(config.seed),
                foreground_queue: VecDeque::new(),
                creation_thread_id: thread::current().id(),
            }),
        }
    }

    pub fn tick(&self, background_only: bool) -> bool {
        let mut inner = self.inner.lock();
        if !background_only {
            if let Some(runnable) = inner.foreground_queue.pop_front() {
                drop(inner); // Unlock while running
                runnable.run();
                return true;
            }
        }
        false
    }

    pub fn run(&self) {
        while self.tick(false) {}
    }
}

impl Scheduler for TestScheduler {
    fn schedule(&self, runnable: Runnable, _label: Option<TaskLabel>) {
        runnable.run();
    }

    fn schedule_foreground(&self, runnable: Runnable, _label: Option<TaskLabel>) {
        self.inner.lock().foreground_queue.push_back(runnable);
    }

    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.inner.lock().creation_thread_id
    }
}

pub struct ForegroundExecutor {
    scheduler: Arc<dyn Scheduler>,
    _phantom: PhantomData<()>,
}

impl ForegroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Result<Self> {
        Ok(Self {
            scheduler,
            _phantom: PhantomData,
        })
    }

    pub fn spawn<R: 'static>(&self, future: impl Future<Output = R> + 'static) -> Task<R> {
        let scheduler = self.scheduler.clone();
        let (runnable, task) = async_task::spawn_local(future, move |runnable| {
            scheduler.schedule_foreground(runnable, None);
        });
        runnable.schedule();
        Task(task)
    }

    pub fn spawn_labeled<R: 'static>(
        &self,
        future: impl Future<Output = R> + 'static,
        label: TaskLabel,
    ) -> Task<R> {
        let scheduler = self.scheduler.clone();
        let (runnable, task) = async_task::spawn_local(future, move |runnable| {
            scheduler.schedule_foreground(runnable, Some(label));
        });
        runnable.schedule();
        Task(task)
    }
}

pub struct BackgroundExecutor {
    scheduler: Arc<dyn Scheduler>,
}

impl BackgroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Result<Self> {
        Ok(Self { scheduler })
    }

    pub fn spawn<R: 'static + Send>(
        &self,
        future: impl Future<Output = R> + Send + 'static,
    ) -> Task<R> {
        let scheduler = self.scheduler.clone();
        let (runnable, task) = async_task::spawn(future, move |runnable| {
            scheduler.schedule_foreground(runnable, None);
        });
        runnable.schedule();
        Task(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_basic_spawn_and_run() {
        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::default()));
        let executor = ForegroundExecutor::new(scheduler.clone()).unwrap();

        let flag = Arc::new(AtomicBool::new(false));
        assert!(!flag.load(Ordering::SeqCst));
        let _task = executor.spawn({
            let flag = flag.clone();
            async move {
                flag.store(true, Ordering::SeqCst);
            }
        });

        assert!(!flag.load(Ordering::SeqCst));

        scheduler.run();

        assert!(flag.load(Ordering::SeqCst));
    }

    #[test]
    fn test_background_task_with_foreground_wait() {
        let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::default()));

        // Create a oneshot channel to send data from background to foreground
        let (tx, rx) = oneshot::channel();

        // Spawn background task that sends 42
        let bg_executor = BackgroundExecutor::new(scheduler.clone()).unwrap();
        let _background_task = bg_executor.spawn(async move {
            tx.send(42).unwrap();
        });

        // Run all tasks
        scheduler.run();

        // Block on receiving the value from the background task
        let received = executor::block_on(rx).unwrap();

        // Assert on the result
        assert_eq!(received, 42);
    }
}
