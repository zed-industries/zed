#![feature(test)]

#[macro_use]
extern crate futures;
extern crate test;

use futures::{Async, Poll, AsyncSink};
use futures::executor;
use futures::executor::{Notify, NotifyHandle};

use futures::sink::Sink;
use futures::stream::Stream;

use futures::sync::mpsc::unbounded;
use futures::sync::mpsc::channel;
use futures::sync::mpsc::Sender;
use futures::sync::mpsc::UnboundedSender;


use test::Bencher;

fn notify_noop() -> NotifyHandle {
    struct Noop;

    impl Notify for Noop {
        fn notify(&self, _id: usize) {}
    }

    const NOOP : &'static Noop = &Noop;

    NotifyHandle::from(NOOP)
}

/// Single producer, single consumer
#[bench]
fn unbounded_1_tx(b: &mut Bencher) {
    b.iter(|| {
        let (tx, rx) = unbounded();

        let mut rx = executor::spawn(rx);

        // 1000 iterations to avoid measuring overhead of initialization
        // Result should be divided by 1000
        for i in 0..1000 {

            // Poll, not ready, park
            assert_eq!(Ok(Async::NotReady), rx.poll_stream_notify(&notify_noop(), 1));

            UnboundedSender::unbounded_send(&tx, i).unwrap();

            // Now poll ready
            assert_eq!(Ok(Async::Ready(Some(i))), rx.poll_stream_notify(&notify_noop(), 1));
        }
    })
}

/// 100 producers, single consumer
#[bench]
fn unbounded_100_tx(b: &mut Bencher) {
    b.iter(|| {
        let (tx, rx) = unbounded();

        let mut rx = executor::spawn(rx);

        let tx: Vec<_> = (0..100).map(|_| tx.clone()).collect();

        // 1000 send/recv operations total, result should be divided by 1000
        for _ in 0..10 {
            for i in 0..tx.len() {
                assert_eq!(Ok(Async::NotReady), rx.poll_stream_notify(&notify_noop(), 1));

                UnboundedSender::unbounded_send(&tx[i], i).unwrap();

                assert_eq!(Ok(Async::Ready(Some(i))), rx.poll_stream_notify(&notify_noop(), 1));
            }
        }
    })
}

#[bench]
fn unbounded_uncontended(b: &mut Bencher) {
    b.iter(|| {
        let (tx, mut rx) = unbounded();

        for i in 0..1000 {
            UnboundedSender::unbounded_send(&tx, i).expect("send");
            // No need to create a task, because poll is not going to park.
            assert_eq!(Ok(Async::Ready(Some(i))), rx.poll());
        }
    })
}


/// A Stream that continuously sends incrementing number of the queue
struct TestSender {
    tx: Sender<u32>,
    last: u32, // Last number sent
}

// Could be a Future, it doesn't matter
impl Stream for TestSender {
    type Item = u32;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.tx.start_send(self.last + 1) {
            Err(_) => panic!(),
            Ok(AsyncSink::Ready) => {
                self.last += 1;
                Ok(Async::Ready(Some(self.last)))
            }
            Ok(AsyncSink::NotReady(_)) => {
                Ok(Async::NotReady)
            }
        }
    }
}


/// Single producers, single consumer
#[bench]
fn bounded_1_tx(b: &mut Bencher) {
    b.iter(|| {
        let (tx, rx) = channel(0);

        let mut tx = executor::spawn(TestSender {
            tx: tx,
            last: 0,
        });

        let mut rx = executor::spawn(rx);

        for i in 0..1000 {
            assert_eq!(Ok(Async::Ready(Some(i + 1))), tx.poll_stream_notify(&notify_noop(), 1));
            assert_eq!(Ok(Async::NotReady), tx.poll_stream_notify(&notify_noop(), 1));
            assert_eq!(Ok(Async::Ready(Some(i + 1))), rx.poll_stream_notify(&notify_noop(), 1));
        }
    })
}

/// 100 producers, single consumer
#[bench]
fn bounded_100_tx(b: &mut Bencher) {
    b.iter(|| {
        // Each sender can send one item after specified capacity
        let (tx, rx) = channel(0);

        let mut tx: Vec<_> = (0..100).map(|_| {
            executor::spawn(TestSender {
                tx: tx.clone(),
                last: 0
            })
        }).collect();

        let mut rx = executor::spawn(rx);

        for i in 0..10 {
            for j in 0..tx.len() {
                // Send an item
                assert_eq!(Ok(Async::Ready(Some(i + 1))), tx[j].poll_stream_notify(&notify_noop(), 1));
                // Then block
                assert_eq!(Ok(Async::NotReady), tx[j].poll_stream_notify(&notify_noop(), 1));
                // Recv the item
                assert_eq!(Ok(Async::Ready(Some(i + 1))), rx.poll_stream_notify(&notify_noop(), 1));
            }
        }
    })
}
