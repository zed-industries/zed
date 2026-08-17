extern crate futures;

use std::sync::mpsc::channel;

use futures::prelude::*;
use futures::sync::oneshot;
use futures::future::{err, ok};

mod support;
use support::*;

#[test]
fn map() {
    // Whatever runs after a `map` should have dropped the closure by that
    // point.
    let (tx, rx) = channel::<()>();
    let (tx2, rx2) = channel();
    err::<i32, i32>(1).map(move |a| { drop(tx); a }).map_err(move |_| {
        assert!(rx.recv().is_err());
        tx2.send(()).unwrap()
    }).forget();
    rx2.recv().unwrap();
}

#[test]
fn map_err() {
    // Whatever runs after a `map_err` should have dropped the closure by that
    // point.
    let (tx, rx) = channel::<()>();
    let (tx2, rx2) = channel();
    ok::<i32, i32>(1).map_err(move |a| { drop(tx); a }).map(move |_| {
        assert!(rx.recv().is_err());
        tx2.send(()).unwrap()
    }).forget();
    rx2.recv().unwrap();
}

struct FutureData<F, T> {
    _data: T,
    future: F,
}

impl<F: Future, T: Send + 'static> Future for FutureData<F, T> {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

#[test]
fn and_then_drops_eagerly() {
    let (c, p) = oneshot::channel::<()>();
    let (tx, rx) = channel::<()>();
    let (tx2, rx2) = channel();
    FutureData { _data: tx, future: p }.and_then(move |_| {
        assert!(rx.recv().is_err());
        tx2.send(()).unwrap();
        ok(1)
    }).forget();
    assert!(rx2.try_recv().is_err());
    c.send(()).unwrap();
    rx2.recv().unwrap();
}

// #[test]
// fn or_else_drops_eagerly() {
//     let (p1, c1) = oneshot::<(), ()>();
//     let (p2, c2) = oneshot::<(), ()>();
//     let (tx, rx) = channel::<()>();
//     let (tx2, rx2) = channel();
//     p1.map(move |a| { drop(tx); a }).or_else(move |_| {
//         assert!(rx.recv().is_err());
//         p2
//     }).map(move |_| tx2.send(()).unwrap()).forget();
//     assert!(rx2.try_recv().is_err());
//     c1.fail(());
//     assert!(rx2.try_recv().is_err());
//     c2.finish(());
//     rx2.recv().unwrap();
// }
