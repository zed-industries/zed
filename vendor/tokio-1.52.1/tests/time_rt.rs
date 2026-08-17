#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::runtime::Runtime;
use tokio::time::*;

use std::sync::mpsc;

fn rt_combinations() -> Vec<Runtime> {
    let mut rts = vec![];

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rts.push(rt);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    rts.push(rt);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    rts.push(rt);

    #[cfg(tokio_unstable)]
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_alt_timer()
            .enable_all()
            .build()
            .unwrap();
        rts.push(rt);

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_alt_timer()
            .enable_all()
            .build()
            .unwrap();
        rts.push(rt);
    }

    rts
}

#[cfg(all(feature = "rt-multi-thread", not(target_os = "wasi")))] // Wasi doesn't support threads
#[test]
fn timer_with_threaded_runtime() {
    use tokio::runtime::Runtime;

    {
        let rt = Runtime::new().unwrap();
        let (tx, rx) = mpsc::channel();

        rt.spawn(async move {
            let when = Instant::now() + Duration::from_millis(10);

            sleep_until(when).await;
            assert!(Instant::now() >= when);

            tx.send(()).unwrap();
        });

        rx.recv().unwrap();
    }

    #[cfg(tokio_unstable)]
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_alt_timer()
            .build()
            .unwrap();
        let (tx, rx) = mpsc::channel();

        rt.block_on(async move {
            let when = Instant::now() + Duration::from_millis(10);

            sleep_until(when).await;
            assert!(Instant::now() >= when);

            tx.send(()).unwrap();
        });

        rx.recv().unwrap();
    }
}

#[test]
fn timer_with_current_thread_scheduler() {
    use tokio::runtime::Builder;

    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let (tx, rx) = mpsc::channel();

    rt.block_on(async move {
        let when = Instant::now() + Duration::from_millis(10);

        sleep_until(when).await;
        assert!(Instant::now() >= when);

        tx.send(()).unwrap();
    });

    rx.recv().unwrap();
}

#[test]
fn starving() {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    struct Starve<T: Future<Output = ()> + Unpin>(T, u64);

    impl<T: Future<Output = ()> + Unpin> Future for Starve<T> {
        type Output = u64;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u64> {
            if Pin::new(&mut self.0).poll(cx).is_ready() {
                return Poll::Ready(self.1);
            }

            self.1 += 1;

            cx.waker().wake_by_ref();

            Poll::Pending
        }
    }

    for rt in rt_combinations() {
        rt.block_on(async {
            let when = Instant::now() + Duration::from_millis(10);
            let starve = Starve(Box::pin(sleep_until(when)), 0);

            starve.await;
            assert!(Instant::now() >= when);
        });
    }
}

#[test]
fn timeout_value() {
    use tokio::sync::oneshot;

    for rt in rt_combinations() {
        rt.block_on(async {
            let (_tx, rx) = oneshot::channel::<()>();

            let now = Instant::now();
            let dur = Duration::from_millis(10);

            let res = timeout(dur, rx).await;
            assert!(res.is_err());
            assert!(Instant::now() >= now + dur);
        });
    }
}
