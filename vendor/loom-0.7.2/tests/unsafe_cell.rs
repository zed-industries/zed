#![deny(warnings, rust_2018_idioms)]

use loom::cell::UnsafeCell;
use loom::sync::atomic::AtomicUsize;
use loom::thread;

use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::Arc;

#[test]
fn atomic_causality_success() {
    struct Chan {
        data: UnsafeCell<usize>,
        guard: AtomicUsize,
    }

    impl Chan {
        fn set(&self) {
            unsafe {
                self.data.with_mut(|v| {
                    *v += 123;
                });
            }

            self.guard.store(1, Release);
        }

        fn get(&self) {
            if 0 == self.guard.load(Acquire) {
                return;
            }

            unsafe {
                self.data.with(|v| {
                    assert_eq!(*v, 123);
                });
            }
        }
    }

    loom::model(|| {
        let chan = Arc::new(Chan {
            data: UnsafeCell::new(0),
            guard: AtomicUsize::new(0),
        });

        let th = {
            let chan = chan.clone();
            thread::spawn(move || {
                chan.set();
            })
        };

        // Try getting before joining
        chan.get();

        th.join().unwrap();

        chan.get();
    });
}

#[test]
#[should_panic]
fn atomic_causality_fail() {
    struct Chan {
        data: UnsafeCell<usize>,
        guard: AtomicUsize,
    }

    impl Chan {
        fn set(&self) {
            unsafe {
                self.data.with_mut(|v| {
                    *v += 123;
                });
            }

            self.guard.store(1, Release);
        }

        fn get(&self) {
            unsafe {
                self.data.with(|v| {
                    assert_eq!(*v, 123);
                });
            }
        }
    }

    loom::model(|| {
        let chan = Arc::new(Chan {
            data: UnsafeCell::new(0),
            guard: AtomicUsize::new(0),
        });

        let th = {
            let chan = chan.clone();
            thread::spawn(move || chan.set())
        };

        // Try getting before joining
        chan.get();

        th.join().unwrap();

        chan.get();
    });
}

#[derive(Clone)]
struct Data(Arc<UnsafeCell<usize>>);

impl Data {
    fn new(v: usize) -> Self {
        Data(Arc::new(UnsafeCell::new(v)))
    }

    fn get(&self) -> usize {
        self.0.with(|v| unsafe { *v })
    }

    fn inc(&self) -> usize {
        self.0.with_mut(|v| unsafe {
            *v += 1;
            *v
        })
    }
}

#[test]
#[should_panic]
fn unsafe_cell_race_mut_mut_1() {
    loom::model(|| {
        let x = Data::new(1);
        let y = x.clone();

        let th1 = thread::spawn(move || x.inc());
        y.inc();

        th1.join().unwrap();

        assert_eq!(4, y.inc());
    });
}

#[test]
#[should_panic]
fn unsafe_cell_race_mut_mut_2() {
    loom::model(|| {
        let x = Data::new(1);
        let y = x.clone();
        let z = x.clone();

        let th1 = thread::spawn(move || x.inc());
        let th2 = thread::spawn(move || y.inc());

        th1.join().unwrap();
        th2.join().unwrap();

        assert_eq!(4, z.inc());
    });
}

#[test]
#[should_panic]
fn unsafe_cell_race_mut_immut_1() {
    loom::model(|| {
        let x = Data::new(1);
        let y = x.clone();

        let th1 = thread::spawn(move || assert_eq!(2, x.inc()));
        y.get();

        th1.join().unwrap();

        assert_eq!(3, y.inc());
    });
}

#[test]
#[should_panic]
fn unsafe_cell_race_mut_immut_2() {
    loom::model(|| {
        let x = Data::new(1);
        let y = x.clone();

        let th1 = thread::spawn(move || x.get());
        assert_eq!(2, y.inc());

        th1.join().unwrap();

        assert_eq!(3, y.inc());
    });
}

#[test]
#[should_panic]
fn unsafe_cell_race_mut_immut_3() {
    loom::model(|| {
        let x = Data::new(1);
        let y = x.clone();
        let z = x.clone();

        let th1 = thread::spawn(move || assert_eq!(2, x.inc()));
        let th2 = thread::spawn(move || y.get());

        th1.join().unwrap();
        th2.join().unwrap();

        assert_eq!(3, z.inc());
    });
}

#[test]
#[should_panic]
fn unsafe_cell_race_mut_immut_4() {
    loom::model(|| {
        let x = Data::new(1);
        let y = x.clone();
        let z = x.clone();

        let th1 = thread::spawn(move || x.get());
        let th2 = thread::spawn(move || assert_eq!(2, y.inc()));

        th1.join().unwrap();
        th2.join().unwrap();

        assert_eq!(3, z.inc());
    });
}

#[test]
#[should_panic]
fn unsafe_cell_race_mut_immut_5() {
    loom::model(|| {
        let x = Data::new(1);
        let y = x.clone();
        let z = x.clone();

        let th1 = thread::spawn(move || x.get());
        let th2 = thread::spawn(move || {
            assert_eq!(1, y.get());
            assert_eq!(2, y.inc());
        });

        th1.join().unwrap();
        th2.join().unwrap();

        assert_eq!(3, z.inc());
    });
}

#[test]
fn unsafe_cell_ok_1() {
    loom::model(|| {
        let x = Data::new(1);

        assert_eq!(2, x.inc());

        let th1 = thread::spawn(move || {
            assert_eq!(3, x.inc());
            x
        });

        let x = th1.join().unwrap();

        assert_eq!(4, x.inc());
    });
}

#[test]
fn unsafe_cell_ok_2() {
    loom::model(|| {
        let x = Data::new(1);

        assert_eq!(1, x.get());
        assert_eq!(2, x.inc());

        let th1 = thread::spawn(move || {
            assert_eq!(2, x.get());
            assert_eq!(3, x.inc());
            x
        });

        let x = th1.join().unwrap();

        assert_eq!(3, x.get());
        assert_eq!(4, x.inc());
    });
}

#[test]
fn unsafe_cell_ok_3() {
    loom::model(|| {
        let x = Data::new(1);
        let y = x.clone();

        let th1 = thread::spawn(move || {
            assert_eq!(1, x.get());

            let z = x.clone();
            let th2 = thread::spawn(move || {
                assert_eq!(1, z.get());
            });

            assert_eq!(1, x.get());
            th2.join().unwrap();
        });

        assert_eq!(1, y.get());

        th1.join().unwrap();

        assert_eq!(2, y.inc());
    });
}

#[test]
#[should_panic]
fn unsafe_cell_access_after_sync() {
    loom::model(|| {
        let s1 = Arc::new((AtomicUsize::new(0), UnsafeCell::new(0)));
        let s2 = s1.clone();

        thread::spawn(move || {
            s1.0.store(1, Release);
            s1.1.with_mut(|ptr| unsafe { *ptr = 1 });
        });

        if 1 == s2.0.load(Acquire) {
            s2.1.with_mut(|ptr| unsafe { *ptr = 2 });
        }
    });
}
