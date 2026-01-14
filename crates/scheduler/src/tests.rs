use super::*;
use futures::{
    FutureExt,
    channel::{mpsc, oneshot},
    executor::block_on,
    future,
    sink::SinkExt,
    stream::{FuturesUnordered, StreamExt},
};
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashSet},
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Waker},
};

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

    TestScheduler::many(100, async |scheduler| {
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
fn test_timer_ordering() {
    TestScheduler::many(1, async |scheduler| {
        let background = scheduler.background();
        let futures = FuturesUnordered::new();
        futures.push(
            async {
                background.timer(Duration::from_millis(100)).await;
                2
            }
            .boxed(),
        );
        futures.push(
            async {
                background.timer(Duration::from_millis(50)).await;
                1
            }
            .boxed(),
        );
        futures.push(
            async {
                background.timer(Duration::from_millis(150)).await;
                3
            }
            .boxed(),
        );
        assert_eq!(futures.collect::<Vec<_>>().await, vec![1, 2, 3]);
    });
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
        let config = TestSchedulerConfig {
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
        let config = TestSchedulerConfig::with_seed(seed);
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

async fn capture_execution_order(config: TestSchedulerConfig) -> Vec<String> {
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
    let scheduler = Arc::new(TestScheduler::new(TestSchedulerConfig::default()));
    let (tx, rx) = oneshot::channel();

    // Spawn background task to send value
    let _ = scheduler
        .background()
        .spawn(async move {
            tx.send(42).unwrap();
        })
        .detach();

    // Block on receiving the value
    let result = scheduler.foreground().block_on(async { rx.await.unwrap() });
    assert_eq!(result, 42);
}

#[test]
#[should_panic(expected = "Parking forbidden. Pending traces:")]
fn test_parking_panics() {
    let config = TestSchedulerConfig {
        capture_pending_traces: true,
        ..Default::default()
    };
    let scheduler = Arc::new(TestScheduler::new(config));
    scheduler.foreground().block_on(async {
        let (_tx, rx) = oneshot::channel::<()>();
        rx.await.unwrap(); // This will never complete
    });
}

#[test]
fn test_block_with_parking() {
    let config = TestSchedulerConfig {
        allow_parking: true,
        ..Default::default()
    };
    let scheduler = Arc::new(TestScheduler::new(config));
    let (tx, rx) = oneshot::channel();

    // Spawn background task to send value
    let _ = scheduler
        .background()
        .spawn(async move {
            tx.send(42).unwrap();
        })
        .detach();

    // Block on receiving the value (will park if needed)
    let result = scheduler.foreground().block_on(async { rx.await.unwrap() });
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
}

#[test]
fn test_block_with_timeout() {
    // Test case: future completes within timeout
    TestScheduler::once(async |scheduler| {
        let foreground = scheduler.foreground();
        let future = future::ready(42);
        let output = foreground.block_with_timeout(Duration::from_millis(100), future);
        assert_eq!(output.ok(), Some(42));
    });

    // Test case: future times out
    TestScheduler::once(async |scheduler| {
        // Make timeout behavior deterministic by forcing the timeout tick budget to be exactly 0.
        // This prevents `block_with_timeout` from making progress via extra scheduler stepping and
        // accidentally completing work that we expect to time out.
        scheduler.set_timeout_ticks(0..=0);

        let foreground = scheduler.foreground();
        let future = future::pending::<()>();
        let output = foreground.block_with_timeout(Duration::from_millis(50), future);
        assert!(output.is_err(), "future should not have finished");
    });

    // Test case: future makes progress via timer but still times out
    let mut results = BTreeSet::new();
    TestScheduler::many(100, async |scheduler| {
        // Keep the existing probabilistic behavior here (do not force 0 ticks), since this subtest
        // is explicitly checking that some seeds/timeouts can complete while others can time out.
        let task = scheduler.background().spawn(async move {
            Yield { polls: 10 }.await;
            42
        });
        let output = scheduler
            .foreground()
            .block_with_timeout(Duration::from_millis(50), task);
        results.insert(output.ok());
    });
    assert_eq!(
        results.into_iter().collect::<Vec<_>>(),
        vec![None, Some(42)]
    );

    // Regression test:
    // A timed-out future must not be cancelled. The returned future should still be
    // pollable to completion later. We also want to ensure time only advances when we
    // explicitly advance it (not by yielding).
    TestScheduler::once(async |scheduler| {
        // Force immediate timeout: the timeout tick budget is 0 so we will not step or
        // advance timers inside `block_with_timeout`.
        scheduler.set_timeout_ticks(0..=0);

        let background = scheduler.background();

        // This task should only complete once time is explicitly advanced.
        let task = background.spawn({
            let scheduler = scheduler.clone();
            async move {
                scheduler.timer(Duration::from_millis(100)).await;
                123
            }
        });

        // This should time out before we advance time enough for the timer to fire.
        let timed_out = scheduler
            .foreground()
            .block_with_timeout(Duration::from_millis(50), task);
        assert!(
            timed_out.is_err(),
            "expected timeout before advancing the clock enough for the timer"
        );

        // Now explicitly advance time and ensure the returned future can complete.
        let mut task = timed_out.err().unwrap();
        scheduler.advance_clock(Duration::from_millis(100));
        scheduler.run();

        let output = scheduler.foreground().block_on(&mut task);
        assert_eq!(output, 123);
    });
}

// When calling block, we shouldn't make progress on foreground-spawned futures with the same session id.
#[test]
fn test_block_does_not_progress_same_session_foreground() {
    let mut task2_made_progress_once = false;
    TestScheduler::many(1000, async |scheduler| {
        let foreground1 = scheduler.foreground();
        let foreground2 = scheduler.foreground();

        let task1 = foreground1.spawn(async move {});
        let task2 = foreground2.spawn(async move {});

        foreground1.block_on(async {
            scheduler.yield_random().await;
            assert!(!task1.is_ready());
            task2_made_progress_once |= task2.is_ready();
        });

        task1.await;
        task2.await;
    });

    assert!(
        task2_made_progress_once,
        "Expected task from different foreground executor to make progress (at least once)"
    );
}

struct Yield {
    polls: usize,
}

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls -= 1;
        if self.polls == 0 {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[test]
fn test_nondeterministic_wake_detection() {
    let config = TestSchedulerConfig {
        allow_parking: false,
        ..Default::default()
    };
    let scheduler = Arc::new(TestScheduler::new(config));

    // A future that captures its waker and sends it to an external thread
    struct SendWakerToThread {
        waker_tx: Option<std::sync::mpsc::Sender<Waker>>,
    }

    impl Future for SendWakerToThread {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Some(tx) = self.waker_tx.take() {
                tx.send(cx.waker().clone()).ok();
            }
            Poll::Ready(())
        }
    }

    let (waker_tx, waker_rx) = std::sync::mpsc::channel::<Waker>();

    // Get a waker by running a future that sends it
    scheduler.foreground().block_on(SendWakerToThread {
        waker_tx: Some(waker_tx),
    });

    // Spawn a real OS thread that will call wake() on the waker
    let handle = std::thread::spawn(move || {
        if let Ok(waker) = waker_rx.recv() {
            // This should trigger the non-determinism detection
            waker.wake();
        }
    });

    // Wait for the spawned thread to complete
    handle.join().ok();

    // The non-determinism error should be detected when end_test is called
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        scheduler.end_test();
    }));
    assert!(result.is_err(), "Expected end_test to panic");
    let panic_payload = result.unwrap_err();
    let panic_message = panic_payload
        .downcast_ref::<String>()
        .map(|s| s.as_str())
        .or_else(|| panic_payload.downcast_ref::<&str>().copied())
        .unwrap_or("<unknown panic>");
    assert!(
        panic_message.contains("Your test is not deterministic"),
        "Expected panic message to contain non-determinism error, got: {}",
        panic_message
    );
}

#[test]
fn test_nondeterministic_wake_allowed_with_parking() {
    let config = TestSchedulerConfig {
        allow_parking: true,
        ..Default::default()
    };
    let scheduler = Arc::new(TestScheduler::new(config));

    // A future that captures its waker and sends it to an external thread
    struct WakeFromExternalThread {
        waker_sent: bool,
        waker_tx: Option<std::sync::mpsc::Sender<Waker>>,
    }

    impl Future for WakeFromExternalThread {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if !self.waker_sent {
                self.waker_sent = true;
                if let Some(tx) = self.waker_tx.take() {
                    tx.send(cx.waker().clone()).ok();
                }
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        }
    }

    let (waker_tx, waker_rx) = std::sync::mpsc::channel::<Waker>();

    // Spawn a real OS thread that will call wake() on the waker
    std::thread::spawn(move || {
        if let Ok(waker) = waker_rx.recv() {
            // With allow_parking, this should NOT panic
            waker.wake();
        }
    });

    // This should complete without panicking
    scheduler.foreground().block_on(WakeFromExternalThread {
        waker_sent: false,
        waker_tx: Some(waker_tx),
    });
}

#[test]
fn test_nondeterministic_waker_drop_detection() {
    let config = TestSchedulerConfig {
        allow_parking: false,
        ..Default::default()
    };
    let scheduler = Arc::new(TestScheduler::new(config));

    // A future that captures its waker and sends it to an external thread
    struct SendWakerToThread {
        waker_tx: Option<std::sync::mpsc::Sender<Waker>>,
    }

    impl Future for SendWakerToThread {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Some(tx) = self.waker_tx.take() {
                tx.send(cx.waker().clone()).ok();
            }
            Poll::Ready(())
        }
    }

    let (waker_tx, waker_rx) = std::sync::mpsc::channel::<Waker>();

    // Get a waker by running a future that sends it
    scheduler.foreground().block_on(SendWakerToThread {
        waker_tx: Some(waker_tx),
    });

    // Spawn a real OS thread that will drop the waker without calling wake
    let handle = std::thread::spawn(move || {
        if let Ok(waker) = waker_rx.recv() {
            // This should trigger the non-determinism detection on drop
            drop(waker);
        }
    });

    // Wait for the spawned thread to complete
    handle.join().ok();

    // The non-determinism error should be detected when end_test is called
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        scheduler.end_test();
    }));
    assert!(result.is_err(), "Expected end_test to panic");
    let panic_payload = result.unwrap_err();
    let panic_message = panic_payload
        .downcast_ref::<String>()
        .map(|s| s.as_str())
        .or_else(|| panic_payload.downcast_ref::<&str>().copied())
        .unwrap_or("<unknown panic>");
    assert!(
        panic_message.contains("Your test is not deterministic"),
        "Expected panic message to contain non-determinism error, got: {}",
        panic_message
    );
}

#[test]
fn test_background_priority_scheduling() {
    use parking_lot::Mutex;

    // Run many iterations to get statistical significance
    let mut high_before_low_count = 0;
    let iterations = 100;

    for seed in 0..iterations {
        let config = TestSchedulerConfig::with_seed(seed);
        let scheduler = Arc::new(TestScheduler::new(config));
        let background = scheduler.background();

        let execution_order = Arc::new(Mutex::new(Vec::new()));

        // Spawn low priority tasks first
        for i in 0..3 {
            let order = execution_order.clone();
            background
                .spawn_with_priority(Priority::Low, async move {
                    order.lock().push(format!("low-{}", i));
                })
                .detach();
        }

        // Spawn high priority tasks second
        for i in 0..3 {
            let order = execution_order.clone();
            background
                .spawn_with_priority(Priority::High, async move {
                    order.lock().push(format!("high-{}", i));
                })
                .detach();
        }

        scheduler.run();

        // Count how many high priority tasks ran in the first half
        let order = execution_order.lock();
        let high_in_first_half = order
            .iter()
            .take(3)
            .filter(|s| s.starts_with("high"))
            .count();

        if high_in_first_half >= 2 {
            high_before_low_count += 1;
        }
    }

    // High priority tasks should tend to run before low priority tasks
    // With weights of 60 vs 10, high priority should dominate early execution
    assert!(
        high_before_low_count > iterations / 2,
        "Expected high priority tasks to run before low priority tasks more often. \
         Got {} out of {} iterations",
        high_before_low_count,
        iterations
    );
}
