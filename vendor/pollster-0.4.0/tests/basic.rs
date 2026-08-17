use std::time::{Duration, Instant};

#[test]
fn basic() {
    let make_fut = || std::future::ready(42);

    // Immediately ready
    assert_eq!(pollster::block_on(make_fut()), 42);

    // Ready after a timeout
    let then = Instant::now();
    pollster::block_on(futures_timer::Delay::new(Duration::from_millis(250)));
    assert!(Instant::now().duration_since(then) > Duration::from_millis(250));
}

#[test]
fn mpsc() {
    use std::{
        sync::atomic::{AtomicUsize, Ordering::SeqCst},
        thread,
    };
    use tokio::sync::mpsc;

    const BOUNDED: usize = 16;
    const MESSAGES: usize = 100_000;

    let (a_tx, mut a_rx) = mpsc::channel(BOUNDED);
    let (b_tx, mut b_rx) = mpsc::channel(BOUNDED);

    let thread_a = thread::spawn(move || {
        pollster::block_on(async {
            while let Some(msg) = a_rx.recv().await {
                b_tx.send(msg).await.expect("send on b");
            }
        });
    });

    let thread_b = thread::spawn(move || {
        pollster::block_on(async move {
            for _ in 0..MESSAGES {
                a_tx.send(()).await.expect("Send on a");
            }
        });
    });

    pollster::block_on(async move {
        let sum = AtomicUsize::new(0);

        while sum.fetch_add(1, SeqCst) < MESSAGES {
            b_rx.recv().await;
        }

        assert_eq!(sum.load(SeqCst), MESSAGES + 1);
    });

    thread_a.join().expect("join thread_a");
    thread_b.join().expect("join thread_b");
}
