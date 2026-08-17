#![feature(test)]

extern crate futures;
extern crate test;

use futures::{Future, Poll, Async};
use futures::task::{self, Task};

use test::Bencher;

#[bench]
fn thread_yield_single_thread_one_wait(b: &mut Bencher) {
    const NUM: usize = 10_000;

    struct Yield {
        rem: usize,
    }

    impl Future for Yield {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Poll<(), ()> {
            if self.rem == 0 {
                Ok(Async::Ready(()))
            } else {
                self.rem -= 1;
                task::current().notify();
                Ok(Async::NotReady)
            }
        }
    }

    b.iter(|| {
        let y = Yield { rem: NUM };
        y.wait().unwrap();
    });
}

#[bench]
fn thread_yield_single_thread_many_wait(b: &mut Bencher) {
    const NUM: usize = 10_000;

    struct Yield {
        rem: usize,
    }

    impl Future for Yield {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Poll<(), ()> {
            if self.rem == 0 {
                Ok(Async::Ready(()))
            } else {
                self.rem -= 1;
                task::current().notify();
                Ok(Async::NotReady)
            }
        }
    }

    b.iter(|| {
        for _ in 0..NUM {
            let y = Yield { rem: 1 };
            y.wait().unwrap();
        }
    });
}

#[bench]
fn thread_yield_multi_thread(b: &mut Bencher) {
    use std::sync::mpsc;
    use std::thread;

    const NUM: usize = 1_000;

    let (tx, rx) = mpsc::sync_channel::<Task>(10_000);

    struct Yield {
        rem: usize,
        tx: mpsc::SyncSender<Task>,
    }

    impl Future for Yield {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Poll<(), ()> {
            if self.rem == 0 {
                Ok(Async::Ready(()))
            } else {
                self.rem -= 1;
                self.tx.send(task::current()).unwrap();
                Ok(Async::NotReady)
            }
        }
    }

    thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            task.notify();
        }
    });

    b.iter(move || {
        let y = Yield {
            rem: NUM,
            tx: tx.clone(),
        };

        y.wait().unwrap();
    });
}
