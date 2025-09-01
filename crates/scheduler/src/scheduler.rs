use async_task::Runnable;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::VecDeque;
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

pub trait Scheduler {
    fn schedule_foreground(&mut self, runnable: Runnable<()>, label: Option<TaskLabel>);
    fn schedule_background(&mut self, runnable: Runnable<()>, label: Option<TaskLabel>);
}

pub struct TestScheduler {
    pub foreground: VecDeque<Runnable<()>>,
    pub background: VecDeque<Runnable<()>>,
    pub deprioritized: VecDeque<Runnable<()>>,
    pub thread_id: thread::ThreadId,
    rng: ChaCha8Rng,
}

impl TestScheduler {
    pub fn new() -> Self {
        Self {
            foreground: VecDeque::new(),
            background: VecDeque::new(),
            deprioritized: VecDeque::new(),
            thread_id: thread::current().id(),
            rng: ChaCha8Rng::seed_from_u64(0),
        }
    }

    pub fn is_main_thread(&self) -> bool {
        thread::current().id() == self.thread_id
    }
}

impl Scheduler for TestScheduler {
    fn schedule_foreground(&mut self, runnable: Runnable<()>, _label: Option<TaskLabel>) {
        self.foreground.push_back(runnable);
    }

    fn schedule_background(&mut self, runnable: Runnable<()>, _label: Option<TaskLabel>) {
        self.background.push_back(runnable);
    }
}

impl TestScheduler {
    fn step(&mut self) {
        let foreground_count = self.foreground.len();
        let background_count = self.background.len();

        if foreground_count > 0 || background_count > 0 {
            // Weighted random selection between foreground and background, like GPUI
            let total_count = foreground_count + background_count;
            let should_pick_foreground = self
                .rng
                .gen_ratio(foreground_count as u32, total_count as u32);

            if should_pick_foreground && foreground_count > 0 {
                let runnable = self.foreground.pop_front().unwrap();
                runnable.run();
            } else if background_count > 0 {
                let runnable = self.background.pop_front().unwrap();
                runnable.run();
            }
        } else if !self.deprioritized.is_empty() {
            // Only when foreground/background empty, run deprioritized
            let runnable = self.deprioritized.pop_front().unwrap();
            runnable.run();
        }
    }
}

pub struct ForegroundExecutor {
    scheduler: Arc<TestScheduler>,
    not_send: PhantomData<Rc<()>>,
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

pub struct BackgroundExecutor {
    scheduler: Arc<TestScheduler>,
}
