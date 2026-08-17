#![deny(warnings, rust_2018_idioms)]

use loom::sync::atomic::AtomicUsize;
use loom::thread;

use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

#[test]
fn yield_completes() {
    loom::model(|| {
        let inc = Arc::new(AtomicUsize::new(0));

        {
            let inc = inc.clone();
            thread::spawn(move || {
                inc.store(1, Relaxed);
            });
        }

        loop {
            if 1 == inc.load(Relaxed) {
                return;
            }

            loom::thread::yield_now();
        }
    });
}
