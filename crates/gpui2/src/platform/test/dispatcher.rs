use crate::PlatformDispatcher;
use async_task::Runnable;
use collections::{BTreeMap, VecDeque};
use parking_lot::Mutex;
use rand::prelude::*;
use std::time::{Duration, Instant};

pub struct TestDispatcher(Mutex<TestDispatcherState>);

struct TestDispatcherState {
    random: StdRng,
    foreground: VecDeque<Runnable>,
    background: Vec<Runnable>,
    delayed: BTreeMap<Instant, Runnable>,
    time: Instant,
    is_main_thread: bool,
}

impl TestDispatcher {
    pub fn new(random: StdRng) -> Self {
        let state = TestDispatcherState {
            random,
            foreground: VecDeque::new(),
            background: Vec::new(),
            delayed: BTreeMap::new(),
            time: Instant::now(),
            is_main_thread: true,
        };

        TestDispatcher(Mutex::new(state))
    }
}

impl PlatformDispatcher for TestDispatcher {
    fn is_main_thread(&self) -> bool {
        self.0.lock().is_main_thread
    }

    fn dispatch(&self, runnable: Runnable) {
        self.0.lock().background.push(runnable);
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.0.lock().foreground.push_back(runnable);
    }

    fn dispatch_after(&self, duration: std::time::Duration, runnable: Runnable) {
        let mut state = self.0.lock();
        let next_time = state.time + duration;
        state.delayed.insert(next_time, runnable);
    }

    fn poll(&self) -> bool {
        let mut state = self.0.lock();

        while let Some((deadline, _)) = state.delayed.first_key_value() {
            if *deadline > state.time {
                break;
            }
            let (_, runnable) = state.delayed.pop_first().unwrap();
            state.background.push(runnable);
        }

        if state.foreground.is_empty() && state.background.is_empty() {
            return false;
        }

        let foreground_len = state.foreground.len();
        let background_len = state.background.len();
        let main_thread = background_len == 0
            || state
                .random
                .gen_ratio(foreground_len as u32, background_len as u32);
        let was_main_thread = state.is_main_thread;
        state.is_main_thread = main_thread;

        let runnable = if main_thread {
            state.foreground.pop_front().unwrap()
        } else {
            let ix = state.random.gen_range(0..background_len);
            state.background.remove(ix)
        };

        drop(state);
        runnable.run();

        self.0.lock().is_main_thread = was_main_thread;

        true
    }

    fn advance_clock(&self, by: Duration) {
        self.0.lock().time += by;
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
