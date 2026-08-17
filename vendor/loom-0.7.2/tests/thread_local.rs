#![deny(warnings, rust_2018_idioms)]
use loom::thread;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn thread_local() {
    loom::thread_local! {
        static THREAD_LOCAL: RefCell<usize> = RefCell::new(1);
    }

    fn do_test(n: usize) {
        THREAD_LOCAL.with(|local| {
            assert_eq!(*local.borrow(), 1);
        });
        THREAD_LOCAL.with(|local| {
            assert_eq!(*local.borrow(), 1);
            *local.borrow_mut() = n;
            assert_eq!(*local.borrow(), n);
        });
        THREAD_LOCAL.with(|local| {
            assert_eq!(*local.borrow(), n);
        });
    }

    loom::model(|| {
        let t1 = thread::spawn(|| do_test(2));

        let t2 = thread::spawn(|| do_test(3));

        do_test(4);

        t1.join().unwrap();
        t2.join().unwrap();
    });
}

#[test]
fn nested_with() {
    loom::thread_local! {
        static LOCAL1: RefCell<usize> = RefCell::new(1);
        static LOCAL2: RefCell<usize> = RefCell::new(2);
    }

    loom::model(|| {
        LOCAL1.with(|local1| *local1.borrow_mut() = LOCAL2.with(|local2| *local2.borrow()));
    });
}

#[test]
fn drop() {
    static DROPS: AtomicUsize = AtomicUsize::new(0);

    struct CountDrops {
        drops: &'static AtomicUsize,
        dummy: bool,
    }

    impl Drop for CountDrops {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Release);
        }
    }

    impl CountDrops {
        fn new(drops: &'static AtomicUsize) -> Self {
            Self { drops, dummy: true }
        }
    }

    loom::thread_local! {
        static DROPPED_LOCAL: CountDrops = CountDrops::new(&DROPS);
    }

    loom::model(|| {
        assert_eq!(DROPS.load(Ordering::Acquire), 0);

        thread::spawn(|| {
            // force access to the thread local so that it's initialized.
            DROPPED_LOCAL.with(|local| assert!(local.dummy));
            assert_eq!(DROPS.load(Ordering::Acquire), 0);
        })
        .join()
        .unwrap();

        // When the first spawned thread completed, its copy of the thread local
        // should have been dropped.
        assert_eq!(DROPS.load(Ordering::Acquire), 1);

        thread::spawn(|| {
            // force access to the thread local so that it's initialized.
            DROPPED_LOCAL.with(|local| assert!(local.dummy));
            assert_eq!(DROPS.load(Ordering::Acquire), 1);
        })
        .join()
        .unwrap();

        // When the second spawned thread completed, its copy of the thread local
        // should have been dropped as well.
        assert_eq!(DROPS.load(Ordering::Acquire), 2);

        // force access to the thread local so that it's initialized.
        DROPPED_LOCAL.with(|local| assert!(local.dummy));
    });

    // Finally, when the model's "main thread" completes, its copy of the local
    // should also be dropped.
    assert_eq!(DROPS.load(Ordering::Acquire), 3);
}
