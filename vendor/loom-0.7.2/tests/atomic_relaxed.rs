#![deny(warnings, rust_2018_idioms)]

use loom::sync::atomic::AtomicUsize;
use loom::thread;

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::Arc;

#[test]
fn compare_and_swap() {
    loom::model(|| {
        let num = Arc::new(AtomicUsize::new(0));

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let num = num.clone();

                thread::spawn(move || {
                    let mut curr = num.load(Relaxed);

                    loop {
                        let actual = num.compare_and_swap(curr, curr + 1, Relaxed);

                        if actual == curr {
                            return;
                        }

                        curr = actual;
                    }
                })
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }

        assert_eq!(2, num.load(Relaxed));
    });
}

#[test]
fn check_ordering_valid() {
    loom::model(|| {
        let n1 = Arc::new((AtomicUsize::new(0), AtomicUsize::new(0)));
        let n2 = n1.clone();

        thread::spawn(move || {
            n1.0.store(1, Relaxed);
            n1.1.store(1, Release);
        });

        if 1 == n2.1.load(Acquire) {
            assert_eq!(1, n2.0.load(Relaxed));
        }
    });
}

#[test]
#[should_panic]
fn check_ordering_invalid_1() {
    loom::model(|| {
        let n1 = Arc::new((AtomicUsize::new(0), AtomicUsize::new(0)));
        let n2 = n1.clone();

        thread::spawn(move || {
            n1.0.store(1, Relaxed);
            n1.1.store(1, Release);
        });

        if 1 == n2.1.load(Relaxed) {
            assert_eq!(1, n2.0.load(Relaxed));
        }
    });
}

#[test]
#[should_panic]
fn check_ordering_invalid_2() {
    loom::model(|| {
        let n1 = Arc::new((AtomicUsize::new(0), AtomicUsize::new(0)));
        let n2 = n1.clone();

        thread::spawn(move || {
            n1.0.store(1, Relaxed);
            n1.1.store(1, Relaxed);
        });

        if 1 == n2.1.load(Relaxed) {
            assert_eq!(1, n2.0.load(Relaxed));
        }
    });
}
