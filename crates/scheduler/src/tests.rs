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
    task::{Context, Poll},
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
    let executor = BackgroundExecutor::new(scheduler);
    let (tx, rx) = oneshot::channel();

    // Spawn background task to send value
    let _ = executor
        .spawn(async move {
            tx.send(42).unwrap();
        })
        .detach();

    // Block on receiving the value
    let result = executor.block_on(async { rx.await.unwrap() });
    assert_eq!(result, 42);
}

#[test]
#[should_panic(expected = "Parking forbidden")]
fn test_parking_panics() {
    let scheduler = Arc::new(TestScheduler::new(SchedulerConfig::default()));
    let executor = BackgroundExecutor::new(scheduler);
    executor.block_on(future::pending::<()>());
}

#[test]
fn test_block_with_parking() {
    let config = SchedulerConfig {
        allow_parking: true,
        ..Default::default()
    };
    let scheduler = Arc::new(TestScheduler::new(config));
    let executor = BackgroundExecutor::new(scheduler);
    let (tx, rx) = oneshot::channel();

    // Spawn background task to send value
    let _ = executor
        .spawn(async move {
            tx.send(42).unwrap();
        })
        .detach();

    // Block on receiving the value (will park if needed)
    let result = executor.block_on(async { rx.await.unwrap() });
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

#[test]
fn test_block_with_timeout() {
    // Test case: future completes within timeout
    TestScheduler::once(async |scheduler| {
        let background = scheduler.background();
        let mut future = future::ready(42);
        let output = background.block_with_timeout(&mut future, Duration::from_millis(100));
        assert_eq!(output, Some(42));
    });

    // Test case: future times out
    TestScheduler::once(async |scheduler| {
        let background = scheduler.background();
        let mut future = future::pending::<()>();
        let output = background.block_with_timeout(&mut future, Duration::from_millis(50));
        assert_eq!(output, None);
    });

    // Test case: future makes progress via timer but still times out
    let mut results = BTreeSet::new();
    TestScheduler::many(100, async |scheduler| {
        let background = scheduler.background();
        let mut task = background.spawn(async move {
            Yield { polls: 10 }.await;
            42
        });
        let output = background.block_with_timeout(&mut task, Duration::from_millis(50));
        results.insert(output);
    });
    assert_eq!(
        results.into_iter().collect::<Vec<_>>(),
        vec![None, Some(42)]
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
