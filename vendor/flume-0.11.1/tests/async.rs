#[cfg(all(feature = "async", not(target_os = "unknown")))]
use {
    flume::*,
    futures::{stream::FuturesUnordered, StreamExt, TryFutureExt, Future},
    futures::task::{Context, Waker, Poll},
    async_std::prelude::FutureExt,
    std::{time::Duration, sync::{atomic::{AtomicUsize, Ordering}, Arc}},
};

#[cfg(all(feature = "async", not(target_os = "unknown")))]
#[test]
fn r#async_recv() {
    let (tx, rx) = unbounded();

    let t = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(250));
        tx.send(42u32).unwrap();
    });

    async_std::task::block_on(async {
        assert_eq!(rx.recv_async().await.unwrap(), 42);
    });

    t.join().unwrap();
}

#[cfg(all(feature = "async", not(target_os = "unknown")))]
#[test]
fn r#async_send() {
    let (tx, rx) = bounded(1);

    let t = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(250));
        assert_eq!(rx.recv(), Ok(42));
    });

    async_std::task::block_on(async {
        tx.send_async(42u32).await.unwrap();
    });

    t.join().unwrap();
}

#[cfg(all(feature = "async", not(target_os = "unknown")))]
#[test]
fn r#async_recv_disconnect() {
    let (tx, rx) = bounded::<i32>(0);

    let t = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(250));
        drop(tx)
    });

    async_std::task::block_on(async {
        assert_eq!(rx.recv_async().await, Err(RecvError::Disconnected));
    });

    t.join().unwrap();
}

#[cfg(all(feature = "async", not(target_os = "unknown")))]
#[test]
fn r#async_send_disconnect() {
    let (tx, rx) = bounded(0);

    let t = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(250));
        drop(rx)
    });

    async_std::task::block_on(async {
        assert_eq!(tx.send_async(42u32).await, Err(SendError(42)));
    });

    t.join().unwrap();
}

#[cfg(all(feature = "async", not(target_os = "unknown")))]
#[test]
fn r#async_recv_drop_recv() {
    let (tx, rx) = bounded::<i32>(10);

    let recv_fut = rx.recv_async();

    async_std::task::block_on(async {
        let res = async_std::future::timeout(std::time::Duration::from_millis(500), rx.recv_async()).await;
        assert!(res.is_err());
    });

    let rx2 = rx.clone();
    let t = std::thread::spawn(move || {
        async_std::task::block_on(async {
            rx2.recv_async().await
        })
    });

    std::thread::sleep(std::time::Duration::from_millis(500));

    tx.send(42).unwrap();

    drop(recv_fut);

    assert_eq!(t.join().unwrap(), Ok(42))
}

#[cfg(all(feature = "async", not(target_os = "unknown")))]
#[async_std::test]
async fn r#async_send_1_million_no_drop_or_reorder() {
    #[derive(Debug)]
    enum Message {
        Increment {
            old: u64,
        },
        ReturnCount,
    }

    let (tx, rx) = unbounded();

    let t = async_std::task::spawn(async move {
        let mut count = 0u64;

        while let Ok(Message::Increment { old }) = rx.recv_async().await {
            assert_eq!(old, count);
            count += 1;
        }

        count
    });

    for next in 0..1_000_000 {
        tx.send(Message::Increment { old: next }).unwrap();
    }

    tx.send(Message::ReturnCount).unwrap();

    let count = t.await;
    assert_eq!(count, 1_000_000)
}

#[cfg(all(feature = "async", not(target_os = "unknown")))]
#[async_std::test]
async fn parallel_async_receivers() {
    let (tx, rx) = flume::unbounded();
    let send_fut = async move {
        let n_sends: usize = 100000;
        for _ in 0..n_sends {
            tx.send_async(()).await.unwrap();
        }
    };

    async_std::task::spawn(
        send_fut
            .timeout(Duration::from_secs(5))
            .map_err(|_| panic!("Send timed out!"))
    );

    let mut futures_unordered = (0..250)
        .map(|_| async {
            while let Ok(()) = rx.recv_async().await
            /* rx.recv() is OK */
            {}
        })
        .collect::<FuturesUnordered<_>>();

    let recv_fut = async {
        while futures_unordered.next().await.is_some() {}
    };

    recv_fut
        .timeout(Duration::from_secs(5))
        .map_err(|_| panic!("Receive timed out!"))
        .await
        .unwrap();

    println!("recv end");
}

#[cfg(all(feature = "async", not(target_os = "unknown")))]
#[test]
fn change_waker() {
    let (tx, rx) = flume::bounded(1);
    tx.send(()).unwrap();

    struct DebugWaker(Arc<AtomicUsize>, Waker);

    impl DebugWaker {
        fn new() -> Self {
            let woken = Arc::new(AtomicUsize::new(0));
            let woken_cloned = woken.clone();
            let waker = waker_fn::waker_fn(move || {
                woken.fetch_add(1, Ordering::SeqCst);
            });
            DebugWaker(woken_cloned, waker)
        }

        fn woken(&self) -> usize {
            self.0.load(Ordering::SeqCst)
        }

        fn ctx(&self) -> Context {
            Context::from_waker(&self.1)
        }
    }

    // Check that the waker is correctly updated when sending tasks change their wakers
    {
        let send_fut = tx.send_async(());
        futures::pin_mut!(send_fut);

        let (waker1, waker2) = (DebugWaker::new(), DebugWaker::new());

        // Set the waker to waker1
        assert_eq!(send_fut.as_mut().poll(&mut waker1.ctx()), Poll::Pending);

        // Change the waker to waker2
        assert_eq!(send_fut.poll(&mut waker2.ctx()), Poll::Pending);

        // Wake the future
        rx.recv().unwrap();

        // Check that waker2 was woken and waker1 was not
        assert_eq!(waker1.woken(), 0);
        assert_eq!(waker2.woken(), 1);
    }

    // Check that the waker is correctly updated when receiving tasks change their wakers
    {
        rx.recv().unwrap();
        let recv_fut = rx.recv_async();
        futures::pin_mut!(recv_fut);

        let (waker1, waker2) = (DebugWaker::new(), DebugWaker::new());

        // Set the waker to waker1
        assert_eq!(recv_fut.as_mut().poll(&mut waker1.ctx()), Poll::Pending);

        // Change the waker to waker2
        assert_eq!(recv_fut.poll(&mut waker2.ctx()), Poll::Pending);

        // Wake the future
        tx.send(()).unwrap();

        // Check that waker2 was woken and waker1 was not
        assert_eq!(waker1.woken(), 0);
        assert_eq!(waker2.woken(), 1);
    }
}

#[cfg(all(feature = "async", not(target_os = "unknown")))]
#[test]
fn spsc_single_threaded_value_ordering() {
    async fn test() {
        let (tx, rx) = flume::bounded(4);
        tokio::select! {
        _ = producer(tx) => {},
        _ = consumer(rx) => {},
    }
    }

    async fn producer(tx: flume::Sender<usize>) {
        for i in 0..100 {
            tx.send_async(i).await.unwrap();
        }
    }

    async fn consumer(rx: flume::Receiver<usize>) {
        let mut expected = 0;
        while let Ok(value) = rx.recv_async().await {
            assert_eq!(value, expected);
            expected += 1;
        }
    }

    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(test());
}
