extern crate futures;

use std::panic::{self, AssertUnwindSafe};

use futures::prelude::*;
use futures::Async::*;
use futures::future;
use futures::stream::FuturesUnordered;
use futures::sync::oneshot;

trait AssertSendSync: Send + Sync {}
impl AssertSendSync for FuturesUnordered<()> {}

#[test]
fn basic_usage() {
    future::lazy(move || {
        let mut queue = FuturesUnordered::new();
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        let (tx3, rx3) = oneshot::channel();

        queue.push(rx1);
        queue.push(rx2);
        queue.push(rx3);

        assert!(!queue.poll().unwrap().is_ready());

        tx2.send("hello").unwrap();

        assert_eq!(Ready(Some("hello")), queue.poll().unwrap());
        assert!(!queue.poll().unwrap().is_ready());

        tx1.send("world").unwrap();
        tx3.send("world2").unwrap();

        assert_eq!(Ready(Some("world")), queue.poll().unwrap());
        assert_eq!(Ready(Some("world2")), queue.poll().unwrap());
        assert_eq!(Ready(None), queue.poll().unwrap());

        Ok::<_, ()>(())
    }).wait().unwrap();
}

#[test]
fn resolving_errors() {
    future::lazy(move || {
        let mut queue = FuturesUnordered::new();
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        let (tx3, rx3) = oneshot::channel();

        queue.push(rx1);
        queue.push(rx2);
        queue.push(rx3);

        assert!(!queue.poll().unwrap().is_ready());

        drop(tx2);

        assert!(queue.poll().is_err());
        assert!(!queue.poll().unwrap().is_ready());

        drop(tx1);
        tx3.send("world2").unwrap();

        assert!(queue.poll().is_err());
        assert_eq!(Ready(Some("world2")), queue.poll().unwrap());
        assert_eq!(Ready(None), queue.poll().unwrap());

        Ok::<_, ()>(())
    }).wait().unwrap();
}

#[test]
fn dropping_ready_queue() {
    future::lazy(move || {
        let mut queue = FuturesUnordered::new();
        let (mut tx1, rx1) = oneshot::channel::<()>();
        let (mut tx2, rx2) = oneshot::channel::<()>();
        let (mut tx3, rx3) = oneshot::channel::<()>();

        queue.push(rx1);
        queue.push(rx2);
        queue.push(rx3);

        assert!(!tx1.poll_cancel().unwrap().is_ready());
        assert!(!tx2.poll_cancel().unwrap().is_ready());
        assert!(!tx3.poll_cancel().unwrap().is_ready());

        drop(queue);

        assert!(tx1.poll_cancel().unwrap().is_ready());
        assert!(tx2.poll_cancel().unwrap().is_ready());
        assert!(tx3.poll_cancel().unwrap().is_ready());

        Ok::<_, ()>(())
    }).wait().unwrap();
}

#[test]
fn stress() {
    const ITER: usize = 300;

    use std::sync::{Arc, Barrier};
    use std::thread;

    for i in 0..ITER {
        let n = (i % 10) + 1;

        let mut queue = FuturesUnordered::new();

        for _ in 0..5 {
            let barrier = Arc::new(Barrier::new(n + 1));

            for num in 0..n {
                let barrier = barrier.clone();
                let (tx, rx) = oneshot::channel();

                queue.push(rx);

                thread::spawn(move || {
                    barrier.wait();
                    tx.send(num).unwrap();
                });
            }

            barrier.wait();

            let mut sync = queue.wait();

            let mut rx: Vec<_> = (&mut sync)
                .take(n)
                .map(|res| res.unwrap())
                .collect();

            assert_eq!(rx.len(), n);

            rx.sort();

            for num in 0..n {
                assert_eq!(rx[num], num);
            }

            queue = sync.into_inner();
        }
    }
}

#[test]
fn panicking_future_dropped() {
    future::lazy(move || {
        let mut queue = FuturesUnordered::new();
        queue.push(future::poll_fn(|| -> Poll<i32, i32> {
            panic!()
        }));

        let r = panic::catch_unwind(AssertUnwindSafe(|| queue.poll()));
        assert!(r.is_err());
        assert!(queue.is_empty());
        assert_eq!(Ready(None), queue.poll().unwrap());

        Ok::<_, ()>(())
    }).wait().unwrap();
}
