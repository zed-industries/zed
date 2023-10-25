use crate::PlatformDispatcher;
use async_task::Runnable;
use collections::{BTreeMap, HashMap, VecDeque};
use parking_lot::Mutex;
use rand::prelude::*;
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use util::post_inc;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct TestDispatcherId(usize);

pub struct TestDispatcher {
    id: TestDispatcherId,
    state: Arc<Mutex<TestDispatcherState>>,
}

struct TestDispatcherState {
    random: StdRng,
    foreground: HashMap<TestDispatcherId, VecDeque<Runnable>>,
    background: Vec<Runnable>,
    delayed: BTreeMap<Instant, Runnable>,
    time: Instant,
    is_main_thread: bool,
    next_id: TestDispatcherId,
}

impl TestDispatcher {
    pub fn new(random: StdRng) -> Self {
        let state = TestDispatcherState {
            random,
            foreground: HashMap::default(),
            background: Vec::new(),
            delayed: BTreeMap::new(),
            time: Instant::now(),
            is_main_thread: true,
            next_id: TestDispatcherId(1),
        };

        TestDispatcher {
            id: TestDispatcherId(0),
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub fn advance_clock(&self, by: Duration) {
        self.state.lock().time += by;
    }

    pub fn simulate_random_delay(&self) -> impl Future<Output = ()> {
        pub struct YieldNow {
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
            count: self.state.lock().random.gen_range(0..10),
        }
    }
}

impl Clone for TestDispatcher {
    fn clone(&self) -> Self {
        let id = post_inc(&mut self.state.lock().next_id.0);
        Self {
            id: TestDispatcherId(id),
            state: self.state.clone(),
        }
    }
}

impl PlatformDispatcher for TestDispatcher {
    fn is_main_thread(&self) -> bool {
        self.state.lock().is_main_thread
    }

    fn dispatch(&self, runnable: Runnable) {
        self.state.lock().background.push(runnable);
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.state
            .lock()
            .foreground
            .entry(self.id)
            .or_default()
            .push_back(runnable);
    }

    fn dispatch_after(&self, duration: std::time::Duration, runnable: Runnable) {
        let mut state = self.state.lock();
        let next_time = state.time + duration;
        state.delayed.insert(next_time, runnable);
    }

    fn poll(&self) -> bool {
        let mut state = self.state.lock();

        while let Some((deadline, _)) = state.delayed.first_key_value() {
            if *deadline > state.time {
                break;
            }
            let (_, runnable) = state.delayed.pop_first().unwrap();
            state.background.push(runnable);
        }

        let foreground_len: usize = state
            .foreground
            .values()
            .map(|runnables| runnables.len())
            .sum();
        let background_len = state.background.len();

        if foreground_len == 0 && background_len == 0 {
            return false;
        }

        let main_thread = state.random.gen_ratio(
            foreground_len as u32,
            (foreground_len + background_len) as u32,
        );
        let was_main_thread = state.is_main_thread;
        state.is_main_thread = main_thread;

        let runnable = if main_thread {
            let state = &mut *state;
            let runnables = state
                .foreground
                .values_mut()
                .filter(|runnables| !runnables.is_empty())
                .choose(&mut state.random)
                .unwrap();
            runnables.pop_front().unwrap()
        } else {
            let ix = state.random.gen_range(0..background_len);
            state.background.swap_remove(ix)
        };

        drop(state);
        runnable.run();

        self.state.lock().is_main_thread = was_main_thread;

        true
    }

    fn as_test(&self) -> Option<&TestDispatcher> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Executor;
    use std::sync::Arc;

    #[test]
    fn test_dispatch() {
        let dispatcher = TestDispatcher::new(StdRng::seed_from_u64(0));
        let executor = Executor::new(Arc::new(dispatcher));

        let result = executor.block(async { executor.run_on_main(|| 1).await });
        assert_eq!(result, 1);

        let result = executor.block({
            let executor = executor.clone();
            async move {
                executor
                    .spawn_on_main({
                        let executor = executor.clone();
                        assert!(executor.is_main_thread());
                        || async move {
                            assert!(executor.is_main_thread());
                            let result = executor
                                .spawn({
                                    let executor = executor.clone();
                                    async move {
                                        assert!(!executor.is_main_thread());

                                        let result = executor
                                            .spawn_on_main({
                                                let executor = executor.clone();
                                                || async move {
                                                    assert!(executor.is_main_thread());
                                                    2
                                                }
                                            })
                                            .await;

                                        assert!(!executor.is_main_thread());
                                        result
                                    }
                                })
                                .await;
                            assert!(executor.is_main_thread());
                            result
                        }
                    })
                    .await
            }
        });
        assert_eq!(result, 2);
    }
}
