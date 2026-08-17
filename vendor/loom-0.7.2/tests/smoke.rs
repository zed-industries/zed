#![deny(warnings, rust_2018_idioms)]

use loom::sync::atomic::AtomicUsize;
use loom::thread;

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::Arc;

#[test]
#[should_panic]
fn checks_fail() {
    struct BuggyInc {
        num: AtomicUsize,
    }

    impl BuggyInc {
        fn new() -> BuggyInc {
            BuggyInc {
                num: AtomicUsize::new(0),
            }
        }

        fn inc(&self) {
            let curr = self.num.load(Acquire);
            self.num.store(curr + 1, Release);
        }
    }

    loom::model(|| {
        let buggy_inc = Arc::new(BuggyInc::new());

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let buggy_inc = buggy_inc.clone();
                thread::spawn(move || buggy_inc.inc())
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }

        assert_eq!(2, buggy_inc.num.load(Relaxed));
    });
}
