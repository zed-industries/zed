#![deny(warnings, rust_2018_idioms)]

use loom::cell::UnsafeCell;
use loom::sync::atomic::{fence, AtomicBool};
use loom::thread;

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};
use std::sync::Arc;

#[test]
fn fence_sw_base() {
    loom::model(|| {
        let data = Arc::new(UnsafeCell::new(0));
        let flag = Arc::new(AtomicBool::new(false));

        let th = {
            let (data, flag) = (data.clone(), flag.clone());
            thread::spawn(move || {
                data.with_mut(|ptr| unsafe { *ptr = 42 });
                fence(Release);
                flag.store(true, Relaxed);
            })
        };

        if flag.load(Relaxed) {
            fence(Acquire);
            assert_eq!(42, data.with_mut(|ptr| unsafe { *ptr }));
        }
        th.join().unwrap();
    });
}

#[test]
fn fence_sw_collapsed_store() {
    loom::model(|| {
        let data = Arc::new(UnsafeCell::new(0));
        let flag = Arc::new(AtomicBool::new(false));

        let th = {
            let (data, flag) = (data.clone(), flag.clone());
            thread::spawn(move || {
                data.with_mut(|ptr| unsafe { *ptr = 42 });
                flag.store(true, Release);
            })
        };

        if flag.load(Relaxed) {
            fence(Acquire);
            assert_eq!(42, data.with_mut(|ptr| unsafe { *ptr }));
        }
        th.join().unwrap();
    });
}

#[test]
fn fence_sw_collapsed_load() {
    loom::model(|| {
        let data = Arc::new(UnsafeCell::new(0));
        let flag = Arc::new(AtomicBool::new(false));

        let th = {
            let (data, flag) = (data.clone(), flag.clone());
            thread::spawn(move || {
                data.with_mut(|ptr| unsafe { *ptr = 42 });
                fence(Release);
                flag.store(true, Relaxed);
            })
        };

        if flag.load(Acquire) {
            assert_eq!(42, data.with_mut(|ptr| unsafe { *ptr }));
        }
        th.join().unwrap();
    });
}

// SB+fences from the Promising Semantics paper (https://sf.snu.ac.kr/promise-concurrency/)
#[test]
fn sb_fences() {
    loom::model(|| {
        let x = Arc::new(AtomicBool::new(false));
        let y = Arc::new(AtomicBool::new(false));

        let a = {
            let (x, y) = (x.clone(), y.clone());
            thread::spawn(move || {
                x.store(true, Relaxed);
                fence(SeqCst);
                y.load(Relaxed)
            })
        };

        y.store(true, Relaxed);
        fence(SeqCst);
        let b = x.load(Relaxed);

        if !a.join().unwrap() {
            assert!(b);
        }
    });
}

#[test]
fn fence_hazard_pointer() {
    loom::model(|| {
        let reachable = Arc::new(AtomicBool::new(true));
        let protected = Arc::new(AtomicBool::new(false));
        let allocated = Arc::new(AtomicBool::new(true));

        let th = {
            let (reachable, protected, allocated) =
                (reachable.clone(), protected.clone(), allocated.clone());
            thread::spawn(move || {
                // put in protected list
                protected.store(true, Relaxed);
                fence(SeqCst);
                // validate, then access
                if reachable.load(Relaxed) {
                    assert!(allocated.load(Relaxed));
                }
            })
        };

        // unlink/retire
        reachable.store(false, Relaxed);
        fence(SeqCst);
        // reclaim unprotected
        if !protected.load(Relaxed) {
            allocated.store(false, Relaxed);
        }

        th.join().unwrap();
    });
}

// RWC+syncs from the SCFix paper (https://plv.mpi-sws.org/scfix/)
// The specified behavior was allowed in C/C++11, which later turned out to be too weak.
// C/C++20 and all the implementations of C/C++11 disallow this behavior.
#[test]
fn rwc_syncs() {
    // ... what else would you call them?
    #![allow(clippy::many_single_char_names)]
    loom::model(|| {
        let x = Arc::new(AtomicBool::new(false));
        let y = Arc::new(AtomicBool::new(false));

        let t2 = {
            let (x, y) = (x.clone(), y.clone());
            thread::spawn(move || {
                let a = x.load(Relaxed);
                fence(SeqCst);
                let b = y.load(Relaxed);
                (a, b)
            })
        };

        let t3 = {
            let x = x.clone();
            thread::spawn(move || {
                y.store(true, Relaxed);
                fence(SeqCst);
                x.load(Relaxed)
            })
        };

        x.store(true, Relaxed);

        let (a, b) = t2.join().unwrap();
        let c = t3.join().unwrap();

        if a && !b && !c {
            panic!();
        }
    });
}

// W+RWC from the SCFix paper (https://plv.mpi-sws.org/scfix/)
// The specified behavior was allowed in C/C++11, which later turned out to be too weak.
// C/C++20 and most of the implementations of C/C++11 disallow this behavior.
#[test]
fn w_rwc() {
    #![allow(clippy::many_single_char_names)]
    loom::model(|| {
        let x = Arc::new(AtomicBool::new(false));
        let y = Arc::new(AtomicBool::new(false));
        let z = Arc::new(AtomicBool::new(false));

        let t2 = {
            let (y, z) = (y.clone(), z.clone());
            thread::spawn(move || {
                let a = z.load(Acquire);
                fence(SeqCst);
                let b = y.load(Relaxed);
                (a, b)
            })
        };

        let t3 = {
            let x = x.clone();
            thread::spawn(move || {
                y.store(true, Relaxed);
                fence(SeqCst);
                x.load(Relaxed)
            })
        };

        x.store(true, Relaxed);
        z.store(true, Release);

        let (a, b) = t2.join().unwrap();
        let c = t3.join().unwrap();

        if a && !b && !c {
            panic!();
        }
    });
}
