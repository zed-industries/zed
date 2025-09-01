use async_task::{Runnable, Task};
use parking_lot::Mutex;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskLabel(usize);

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
}

pub trait Scheduler {
    fn schedule_foreground(&self, runnable: Runnable<()>, label: Option<TaskLabel>);
    fn schedule_background(&self, runnable: Runnable<()>, label: Option<TaskLabel>);
}

pub struct TestScheduler {
    state: Mutex<SchedulerState>,
    pub thread_id: thread::ThreadId,
}

impl TestScheduler {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(SchedulerState {
                foreground: VecDeque::new(),
                background: VecDeque::new(),
                deprioritized: VecDeque::new(),
                rng: ChaCha8Rng::seed_from_u64(0),
                deprioritized_labels: HashSet::new(),
            }),
            thread_id: thread::current().id(),
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
            // Weighted random selection between foreground and background, like GPUI
            let total_count = foreground_count + background_count;
            let should_pick_foreground = state
                .rng
                .gen_ratio(foreground_count as u32, total_count as u32);

            if should_pick_foreground && foreground_count > 0 {
                let runnable = state.foreground.pop_front().unwrap();
                drop(state);
                runnable.run();
                true
            } else if background_count > 0 {
                let runnable = state.background.pop_front().unwrap();
                drop(state);
                runnable.run();
                true
            } else {
                false
            }
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

    #[test]
    fn test_foreground_executor_spawn() {
        let scheduler = Arc::new(TestScheduler::new());
        let executor = ForegroundExecutor::new(scheduler.clone());
        let task_ran = Rc::new(RefCell::new(false));
        let _task = executor.spawn({
            let task_ran = task_ran.clone();
            async move {
                *task_ran.borrow_mut() = true;
            }
        });
        scheduler.run();
        assert!(*task_ran.borrow());
    }

    #[test]
    fn test_background_executor_spawn() {
        let scheduler = Arc::new(TestScheduler::new());
        let executor = BackgroundExecutor::new(scheduler.clone());
        let executed = Arc::new(parking_lot::Mutex::new(false));
        let executed_clone = executed.clone();
        let _task = executor.spawn(async move {
            *executed_clone.lock() = true;
        });
        scheduler.run();
        assert!(*executed.lock());
    }
}
