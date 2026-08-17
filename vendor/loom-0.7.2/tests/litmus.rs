#![deny(warnings, rust_2018_idioms)]

use loom::sync::atomic::AtomicUsize;
use loom::thread;

use std::collections::HashSet;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};

// Loom currently does not support load buffering.
#[test]
#[ignore]
fn load_buffering() {
    let values = Arc::new(Mutex::new(HashSet::new()));
    let values_ = values.clone();
    loom::model(move || {
        let x = Arc::new(AtomicUsize::new(0));
        let y = Arc::new(AtomicUsize::new(0));

        let th = {
            let (x, y) = (x.clone(), y.clone());
            thread::spawn(move || {
                x.store(y.load(Relaxed), Relaxed);
            })
        };

        let a = x.load(Relaxed);
        y.store(1, Relaxed);

        th.join().unwrap();
        values.lock().unwrap().insert(a);
    });
    assert!(values_.lock().unwrap().contains(&1));
}

#[test]
fn store_buffering() {
    let values = Arc::new(Mutex::new(HashSet::new()));
    let values_ = values.clone();
    loom::model(move || {
        let x = Arc::new(AtomicUsize::new(0));
        let y = Arc::new(AtomicUsize::new(0));

        let a = {
            let (x, y) = (x.clone(), y.clone());
            thread::spawn(move || {
                x.store(1, Relaxed);
                y.load(Relaxed)
            })
        };

        y.store(1, Relaxed);
        let b = x.load(Relaxed);

        let a = a.join().unwrap();
        values.lock().unwrap().insert((a, b));
    });
    assert!(values_.lock().unwrap().contains(&(0, 0)));
}
