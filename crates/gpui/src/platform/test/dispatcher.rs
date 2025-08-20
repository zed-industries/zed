use crate::{PlatformDispatcher, TaskLabel};
use async_task::Runnable;
use backtrace::Backtrace;
use collections::{HashMap, HashSet, VecDeque};
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use rand::prelude::*;
use std::{
    future::Future,
    ops::RangeInclusive,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use util::post_inc;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct TestDispatcherId(usize);

#[doc(hidden)]
pub struct TestDispatcher {
    id: TestDispatcherId,
    state: Arc<Mutex<TestDispatcherState>>,
    parker: Arc<Mutex<Parker>>,
    unparker: Unparker,
}

struct TestDispatcherState {
    random: StdRng,
    foreground: HashMap<TestDispatcherId, VecDeque<Runnable>>,
    background: Vec<Runnable>,
    deprioritized_background: Vec<Runnable>,
    delayed: Vec<(Duration, Runnable)>,
    start_time: Instant,
    time: Duration,
    is_main_thread: bool,
    next_id: TestDispatcherId,
    allow_parking: bool,
    waiting_hint: Option<String>,
    waiting_backtrace: Option<Backtrace>,
    deprioritized_task_labels: HashSet<TaskLabel>,
    block_on_ticks: RangeInclusive<usize>,
}

impl TestDispatcher {
    pub fn new(random: StdRng) -> Self {
        let (parker, unparker) = parking::pair();
        let state = TestDispatcherState {
            random,
            foreground: HashMap::default(),
            background: Vec::new(),
            deprioritized_background: Vec::new(),
            delayed: Vec::new(),
            time: Duration::ZERO,
            start_time: Instant::now(),
            is_main_thread: true,
            next_id: TestDispatcherId(1),
            allow_parking: false,
            waiting_hint: None,
            waiting_backtrace: None,
            deprioritized_task_labels: Default::default(),
            block_on_ticks: 0..=1000,
        };

        TestDispatcher {
            id: TestDispatcherId(0),
            state: Arc::new(Mutex::new(state)),
            parker: Arc::new(Mutex::new(parker)),
            unparker,
        }
    }

    pub fn advance_clock(&self, by: Duration) {
        let new_now = self.state.lock().time + by;
        loop {
            self.run_until_parked();
            let state = self.state.lock();
            let next_due_time = state.delayed.first().map(|(time, _)| *time);
            drop(state);
            if let Some(due_time) = next_due_time
                && due_time <= new_now
            {
                self.state.lock().time = due_time;
                continue;
            }
            break;
        }
        self.state.lock().time = new_now;
    }

    pub fn advance_clock_to_next_delayed(&self) -> bool {
        let next_due_time = self.state.lock().delayed.first().map(|(time, _)| *time);
        if let Some(next_due_time) = next_due_time {
            self.state.lock().time = next_due_time;
            return true;
        }
        false
    }

    pub fn simulate_random_delay(&self) -> impl 'static + Send + Future<Output = ()> + use<> {
        struct YieldNow {
            pub(crate) count: usize,
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
            count: self.state.lock().random.gen_range(0..10),
        }
    }

    pub fn tick(&self, background_only: bool) -> bool {
        let mut state = self.state.lock();

        while let Some((deadline, _)) = state.delayed.first() {
            if *deadline > state.time {
                break;
            }
            let (_, runnable) = state.delayed.remove(0);
            state.background.push(runnable);
        }

        let foreground_len: usize = if background_only {
            0
        } else {
            state
                .foreground
                .values()
                .map(|runnables| runnables.len())
                .sum()
        };
        let background_len = state.background.len();

        let runnable;
        let main_thread;
        if foreground_len == 0 && background_len == 0 {
            let deprioritized_background_len = state.deprioritized_background.len();
            if deprioritized_background_len == 0 {
                return false;
            }
            let ix = state.random.gen_range(0..deprioritized_background_len);
            main_thread = false;
            runnable = state.deprioritized_background.swap_remove(ix);
        } else {
            main_thread = state.random.gen_ratio(
                foreground_len as u32,
                (foreground_len + background_len) as u32,
            );
            if main_thread {
                let state = &mut *state;
                runnable = state
                    .foreground
                    .values_mut()
                    .filter(|runnables| !runnables.is_empty())
                    .choose(&mut state.random)
                    .unwrap()
                    .pop_front()
                    .unwrap();
            } else {
                let ix = state.random.gen_range(0..background_len);
                runnable = state.background.swap_remove(ix);
            };
        };

        let was_main_thread = state.is_main_thread;
        state.is_main_thread = main_thread;
        drop(state);
        runnable.run();
        self.state.lock().is_main_thread = was_main_thread;

        true
    }

    pub fn deprioritize(&self, task_label: TaskLabel) {
        self.state
            .lock()
            .deprioritized_task_labels
            .insert(task_label);
    }

    pub fn run_until_parked(&self) {
        while self.tick(false) {}
    }

    pub fn parking_allowed(&self) -> bool {
        self.state.lock().allow_parking
    }

    pub fn allow_parking(&self) {
        self.state.lock().allow_parking = true
    }

    pub fn forbid_parking(&self) {
        self.state.lock().allow_parking = false
    }

    pub fn set_waiting_hint(&self, msg: Option<String>) {
        self.state.lock().waiting_hint = msg
    }

    pub fn waiting_hint(&self) -> Option<String> {
        self.state.lock().waiting_hint.clone()
    }

    pub fn start_waiting(&self) {
        self.state.lock().waiting_backtrace = Some(Backtrace::new_unresolved());
    }

    pub fn finish_waiting(&self) {
        self.state.lock().waiting_backtrace.take();
    }

    pub fn waiting_backtrace(&self) -> Option<Backtrace> {
        self.state.lock().waiting_backtrace.take().map(|mut b| {
            b.resolve();
            b
        })
    }

    pub fn rng(&self) -> StdRng {
        self.state.lock().random.clone()
    }

    pub fn set_block_on_ticks(&self, range: std::ops::RangeInclusive<usize>) {
        self.state.lock().block_on_ticks = range;
    }

    pub fn gen_block_on_ticks(&self) -> usize {
        let mut lock = self.state.lock();
        let block_on_ticks = lock.block_on_ticks.clone();
        lock.random.gen_range(block_on_ticks)
    }
}

impl Clone for TestDispatcher {
    fn clone(&self) -> Self {
        let id = post_inc(&mut self.state.lock().next_id.0);
        Self {
            id: TestDispatcherId(id),
            state: self.state.clone(),
            parker: self.parker.clone(),
            unparker: self.unparker.clone(),
        }
    }
}

impl PlatformDispatcher for TestDispatcher {
    fn is_main_thread(&self) -> bool {
        self.state.lock().is_main_thread
    }

    fn now(&self) -> Instant {
        let state = self.state.lock();
        state.start_time + state.time
    }

    fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>) {
        {
            let mut state = self.state.lock();
            if label.is_some_and(|label| state.deprioritized_task_labels.contains(&label)) {
                state.deprioritized_background.push(runnable);
            } else {
                state.background.push(runnable);
            }
        }
        self.unparker.unpark();
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.state
            .lock()
            .foreground
            .entry(self.id)
            .or_default()
            .push_back(runnable);
        self.unparker.unpark();
    }

    fn dispatch_after(&self, duration: std::time::Duration, runnable: Runnable) {
        let mut state = self.state.lock();
        let next_time = state.time + duration;
        let ix = match state.delayed.binary_search_by_key(&next_time, |e| e.0) {
            Ok(ix) | Err(ix) => ix,
        };
        state.delayed.insert(ix, (next_time, runnable));
    }
    fn park(&self, _: Option<std::time::Duration>) -> bool {
        self.parker.lock().park();
        true
    }

    fn unparker(&self) -> Unparker {
        self.unparker.clone()
    }

    fn as_test(&self) -> Option<&TestDispatcher> {
        Some(self)
    }
}
