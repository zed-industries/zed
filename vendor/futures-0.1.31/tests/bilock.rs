#![allow(bare_trait_objects, unknown_lints)]

extern crate futures;

use std::thread;

use futures::prelude::*;
use futures::executor;
use futures::stream;
use futures::future;
use futures::sync::BiLock;

mod support;
use support::*;

#[test]
fn smoke() {
    let future = future::lazy(|| {
        let (a, b) = BiLock::new(1);

        {
            let mut lock = match a.poll_lock() {
                Async::Ready(l) => l,
                Async::NotReady => panic!("poll not ready"),
            };
            assert_eq!(*lock, 1);
            *lock = 2;

            assert!(b.poll_lock().is_not_ready());
            assert!(a.poll_lock().is_not_ready());
        }

        assert!(b.poll_lock().is_ready());
        assert!(a.poll_lock().is_ready());

        {
            let lock = match b.poll_lock() {
                Async::Ready(l) => l,
                Async::NotReady => panic!("poll not ready"),
            };
            assert_eq!(*lock, 2);
        }

        assert_eq!(a.reunite(b).expect("bilock/smoke: reunite error"), 2);

        Ok::<(), ()>(())
    });

    assert!(executor::spawn(future)
                .poll_future_notify(&notify_noop(), 0)
                .expect("failure in poll")
                .is_ready());
}

#[test]
fn concurrent() {
    const N: usize = 10000;
    let (a, b) = BiLock::new(0);

    let a = Increment {
        a: Some(a),
        remaining: N,
    };
    let b = stream::iter_ok::<_, ()>((0..N)).fold(b, |b, _n| {
        b.lock().map(|mut b| {
            *b += 1;
            b.unlock()
        })
    });

    let t1 = thread::spawn(move || a.wait());
    let b = b.wait().expect("b error");
    let a = t1.join().unwrap().expect("a error");

    match a.poll_lock() {
        Async::Ready(l) => assert_eq!(*l, 2 * N),
        Async::NotReady => panic!("poll not ready"),
    }
    match b.poll_lock() {
        Async::Ready(l) => assert_eq!(*l, 2 * N),
        Async::NotReady => panic!("poll not ready"),
    }

    assert_eq!(a.reunite(b).expect("bilock/concurrent: reunite error"), 2 * N);

    struct Increment {
        remaining: usize,
        a: Option<BiLock<usize>>,
    }

    impl Future for Increment {
        type Item = BiLock<usize>;
        type Error = ();

        fn poll(&mut self) -> Poll<BiLock<usize>, ()> {
            loop {
                if self.remaining == 0 {
                    return Ok(self.a.take().unwrap().into())
                }

                let a = self.a.as_ref().unwrap();
                let mut a = match a.poll_lock() {
                    Async::Ready(l) => l,
                    Async::NotReady => return Ok(Async::NotReady),
                };
                self.remaining -= 1;
                *a += 1;
            }
        }
    }
}
