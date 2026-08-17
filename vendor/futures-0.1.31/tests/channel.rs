#![allow(bare_trait_objects, unknown_lints)]

extern crate futures;

use std::sync::atomic::*;

use futures::prelude::*;
use futures::future::result;
use futures::sync::mpsc;

mod support;
use support::*;

#[test]
fn sequence() {
    let (tx, mut rx) = mpsc::channel(1);

    sassert_empty(&mut rx);
    sassert_empty(&mut rx);

    let amt = 20;
    send(amt, tx).forget();
    let mut rx = rx.wait();
    for i in (1..amt + 1).rev() {
        assert_eq!(rx.next(), Some(Ok(i)));
    }
    assert_eq!(rx.next(), None);

    fn send(n: u32, sender: mpsc::Sender<u32>)
            -> Box<Future<Item=(), Error=()> + Send> {
        if n == 0 {
            return Box::new(result(Ok(())))
        }
        Box::new(sender.send(n).map_err(|_| ()).and_then(move |sender| {
            send(n - 1, sender)
        }))
    }
}

#[test]
fn drop_sender() {
    let (tx, mut rx) = mpsc::channel::<u32>(1);
    drop(tx);
    sassert_done(&mut rx);
}

#[test]
fn drop_rx() {
    let (tx, rx) = mpsc::channel::<u32>(1);
    let tx = tx.send(1).wait().ok().unwrap();
    drop(rx);
    assert!(tx.send(1).wait().is_err());
}

#[test]
fn drop_order() {
    #[allow(deprecated)]
    static DROPS: AtomicUsize = ATOMIC_USIZE_INIT;
    let (tx, rx) = mpsc::channel(1);

    struct A;

    impl Drop for A {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }

    let tx = tx.send(A).wait().unwrap();
    assert_eq!(DROPS.load(Ordering::SeqCst), 0);
    drop(rx);
    assert_eq!(DROPS.load(Ordering::SeqCst), 1);
    assert!(tx.send(A).wait().is_err());
    assert_eq!(DROPS.load(Ordering::SeqCst), 2);
}
