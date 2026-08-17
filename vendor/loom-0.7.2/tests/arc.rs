#![deny(warnings, rust_2018_idioms)]

use loom::cell::UnsafeCell;
use loom::sync::atomic::AtomicBool;
use loom::sync::atomic::Ordering::{Acquire, Release};
use loom::sync::Arc;
use loom::sync::Notify;
use loom::thread;

struct State {
    data: UnsafeCell<usize>,
    guard: AtomicBool,
}

impl Drop for State {
    fn drop(&mut self) {
        self.data.with(|ptr| unsafe {
            assert_eq!(1, *ptr);
        });
    }
}

#[test]
fn basic_usage() {
    loom::model(|| {
        let num = Arc::new(State {
            data: UnsafeCell::new(0),
            guard: AtomicBool::new(false),
        });

        let num2 = num.clone();
        thread::spawn(move || {
            num2.data.with_mut(|ptr| unsafe { *ptr = 1 });
            num2.guard.store(true, Release);
        });

        loop {
            if num.guard.load(Acquire) {
                num.data.with(|ptr| unsafe {
                    assert_eq!(1, *ptr);
                });
                break;
            }

            thread::yield_now();
        }
    });
}

#[test]
fn sync_in_drop() {
    loom::model(|| {
        let num = Arc::new(State {
            data: UnsafeCell::new(0),
            guard: AtomicBool::new(false),
        });

        let num2 = num.clone();
        thread::spawn(move || {
            num2.data.with_mut(|ptr| unsafe { *ptr = 1 });
            num2.guard.store(true, Release);
            drop(num2);
        });

        drop(num);
    });
}

#[test]
#[should_panic]
fn detect_mem_leak() {
    loom::model(|| {
        let num = Arc::new(State {
            data: UnsafeCell::new(0),
            guard: AtomicBool::new(false),
        });

        std::mem::forget(num);
    });
}

#[test]
fn try_unwrap_succeeds() {
    loom::model(|| {
        let num = Arc::new(0usize);
        let num2 = Arc::clone(&num);
        drop(num2);
        let _ = Arc::try_unwrap(num).unwrap();
    });
}

#[test]
fn try_unwrap_fails() {
    loom::model(|| {
        let num = Arc::new(0usize);
        let num2 = Arc::clone(&num);
        let num = Arc::try_unwrap(num).unwrap_err();

        drop(num2);

        let _ = Arc::try_unwrap(num).unwrap();
    });
}

#[test]
fn try_unwrap_multithreaded() {
    loom::model(|| {
        let num = Arc::new(0usize);
        let num2 = Arc::clone(&num);
        let can_drop = Arc::new(Notify::new());
        let thread = {
            let can_drop = can_drop.clone();
            thread::spawn(move || {
                can_drop.wait();
                drop(num2);
            })
        };

        // The other thread is holding the other arc clone, so we can't unwrap the arc.
        let num = Arc::try_unwrap(num).unwrap_err();

        // Allow the thread to proceed.
        can_drop.notify();

        // After the thread drops the other clone, the arc should be
        // unwrappable.
        thread.join().unwrap();
        let _ = Arc::try_unwrap(num).unwrap();
    });
}
