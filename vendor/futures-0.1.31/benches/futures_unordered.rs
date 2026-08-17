#![feature(test)]

extern crate futures;
extern crate test;

use futures::*;
use futures::stream::FuturesUnordered;
use futures::sync::oneshot;

use test::Bencher;

use std::collections::VecDeque;
use std::thread;

#[bench]
fn oneshots(b: &mut Bencher) {
    const NUM: usize = 10_000;

    b.iter(|| {
        let mut txs = VecDeque::with_capacity(NUM);
        let mut rxs = FuturesUnordered::new();

        for _ in 0..NUM {
            let (tx, rx) = oneshot::channel();
            txs.push_back(tx);
            rxs.push(rx);
        }

        thread::spawn(move || {
            while let Some(tx) = txs.pop_front() {
                let _ = tx.send("hello");
            }
        });

        future::lazy(move || {
            loop {
                if let Ok(Async::Ready(None)) = rxs.poll() {
                    return Ok::<(), ()>(());
                }
            }
        }).wait().unwrap();
    });
}
