#![allow(bare_trait_objects, unknown_lints)]

extern crate futures;

use std::any::Any;

use futures::sync::oneshot;
use std::iter::FromIterator;
use futures::stream::{futures_unordered, FuturesUnordered};
use futures::prelude::*;

mod support;

#[test]
fn works_1() {
    let (a_tx, a_rx) = oneshot::channel::<u32>();
    let (b_tx, b_rx) = oneshot::channel::<u32>();
    let (c_tx, c_rx) = oneshot::channel::<u32>();

    let stream = futures_unordered(vec![a_rx, b_rx, c_rx]);

    let mut spawn = futures::executor::spawn(stream);
    b_tx.send(99).unwrap();
    assert_eq!(Some(Ok(99)), spawn.wait_stream());

    a_tx.send(33).unwrap();
    c_tx.send(33).unwrap();
    assert_eq!(Some(Ok(33)), spawn.wait_stream());
    assert_eq!(Some(Ok(33)), spawn.wait_stream());
    assert_eq!(None, spawn.wait_stream());
}

#[test]
fn works_2() {
    let (a_tx, a_rx) = oneshot::channel::<u32>();
    let (b_tx, b_rx) = oneshot::channel::<u32>();
    let (c_tx, c_rx) = oneshot::channel::<u32>();

    let stream = futures_unordered(vec![
        Box::new(a_rx) as Box<Future<Item = _, Error = _>>,
        Box::new(b_rx.join(c_rx).map(|(a, b)| a + b)),
    ]);

    let mut spawn = futures::executor::spawn(stream);
    a_tx.send(33).unwrap();
    b_tx.send(33).unwrap();
    assert!(spawn.poll_stream_notify(&support::notify_noop(), 0).unwrap().is_ready());
    c_tx.send(33).unwrap();
    assert!(spawn.poll_stream_notify(&support::notify_noop(), 0).unwrap().is_ready());
}

#[test]
fn from_iterator() {
    use futures::future::ok;
    use futures::stream::FuturesUnordered;

    let stream = vec![
        ok::<u32, ()>(1),
        ok::<u32, ()>(2),
        ok::<u32, ()>(3)
    ].into_iter().collect::<FuturesUnordered<_>>();
    assert_eq!(stream.len(), 3);
    assert_eq!(stream.collect().wait(), Ok(vec![1,2,3]));
}

#[test]
fn finished_future_ok() {
    let (_a_tx, a_rx) = oneshot::channel::<Box<Any+Send>>();
    let (b_tx, b_rx) = oneshot::channel::<Box<Any+Send>>();
    let (c_tx, c_rx) = oneshot::channel::<Box<Any+Send>>();

    let stream = futures_unordered(vec![
        Box::new(a_rx) as Box<Future<Item = _, Error = _>>,
        Box::new(b_rx.select(c_rx).then(|res| Ok(Box::new(res) as Box<Any+Send>))),
    ]);

    let mut spawn = futures::executor::spawn(stream);
    for _ in 0..10 {
        assert!(spawn.poll_stream_notify(&support::notify_noop(), 0).unwrap().is_not_ready());
    }

    b_tx.send(Box::new(())).unwrap();
    let next = spawn.poll_stream_notify(&support::notify_noop(), 0).unwrap();
    assert!(next.is_ready());
    c_tx.send(Box::new(())).unwrap();
    assert!(spawn.poll_stream_notify(&support::notify_noop(), 0).unwrap().is_not_ready());
    assert!(spawn.poll_stream_notify(&support::notify_noop(), 0).unwrap().is_not_ready());
}

#[test]
fn iter_mut_cancel() {
    let (a_tx, a_rx) = oneshot::channel::<u32>();
    let (b_tx, b_rx) = oneshot::channel::<u32>();
    let (c_tx, c_rx) = oneshot::channel::<u32>();

    let mut stream = futures_unordered(vec![a_rx, b_rx, c_rx]);

    for rx in stream.iter_mut() {
        rx.close();
    }

    assert!(a_tx.is_canceled());
    assert!(b_tx.is_canceled());
    assert!(c_tx.is_canceled());

    let mut spawn = futures::executor::spawn(stream);
    assert_eq!(Some(Err(futures::sync::oneshot::Canceled)), spawn.wait_stream());
    assert_eq!(Some(Err(futures::sync::oneshot::Canceled)), spawn.wait_stream());
    assert_eq!(Some(Err(futures::sync::oneshot::Canceled)), spawn.wait_stream());
    assert_eq!(None, spawn.wait_stream());
}

#[test]
fn iter_mut_len() {
    let mut stream = futures_unordered(vec![
        futures::future::empty::<(),()>(),
        futures::future::empty::<(),()>(),
        futures::future::empty::<(),()>()
    ]);

    let mut iter_mut = stream.iter_mut();
    assert_eq!(iter_mut.len(), 3);
    assert!(iter_mut.next().is_some());
    assert_eq!(iter_mut.len(), 2);
    assert!(iter_mut.next().is_some());
    assert_eq!(iter_mut.len(), 1);
    assert!(iter_mut.next().is_some());
    assert_eq!(iter_mut.len(), 0);
    assert!(iter_mut.next().is_none());
}

#[test]
fn polled_only_once_at_most_per_iteration() {
    #[derive(Debug, Clone, Copy, Default)]
    struct F {
        polled: bool,
    }

    impl Future for F {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
            if self.polled {
                panic!("polled twice")
            } else {
                self.polled = true;
                Ok(Async::NotReady)
            }
        }
    }


    let tasks = FuturesUnordered::from_iter(vec![F::default(); 10]);
    let mut tasks = futures::executor::spawn(tasks);
    assert!(tasks.poll_stream_notify(&support::notify_noop(), 0).unwrap().is_not_ready());
    assert_eq!(10, tasks.get_mut().iter_mut().filter(|f| f.polled).count());

    let tasks = FuturesUnordered::from_iter(vec![F::default(); 33]);
    let mut tasks = futures::executor::spawn(tasks);
    assert!(tasks.poll_stream_notify(&support::notify_noop(), 0).unwrap().is_not_ready());
    assert_eq!(33, tasks.get_mut().iter_mut().filter(|f| f.polled).count());

    let tasks = FuturesUnordered::<F>::new();
    let mut tasks = futures::executor::spawn(tasks);
    assert!(tasks.poll_stream_notify(&support::notify_noop(), 0).unwrap().is_ready());
}
