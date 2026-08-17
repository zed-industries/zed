#![deny(warnings, rust_2018_idioms)]

use loom::sync::atomic::AtomicUsize;
use loom::sync::{Condvar, Mutex};
use loom::thread;

use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;

#[test]
fn notify_one() {
    loom::model(|| {
        let inc = Arc::new(Inc::new());

        for _ in 0..1 {
            let inc = inc.clone();
            thread::spawn(move || inc.inc());
        }

        inc.wait();
    });
}

#[test]
fn notify_all() {
    loom::model(|| {
        let inc = Arc::new(Inc::new());

        let mut waiters = Vec::new();
        for _ in 0..2 {
            let inc = inc.clone();
            waiters.push(thread::spawn(move || inc.wait()));
        }

        thread::spawn(move || inc.inc_all()).join().expect("inc");

        for th in waiters {
            th.join().expect("waiter");
        }
    });
}

struct Inc {
    num: AtomicUsize,
    mutex: Mutex<()>,
    condvar: Condvar,
}

impl Inc {
    fn new() -> Inc {
        Inc {
            num: AtomicUsize::new(0),
            mutex: Mutex::new(()),
            condvar: Condvar::new(),
        }
    }

    fn wait(&self) {
        let mut guard = self.mutex.lock().unwrap();

        loop {
            let val = self.num.load(SeqCst);
            if 1 == val {
                break;
            }

            guard = self.condvar.wait(guard).unwrap();
        }
    }

    fn inc(&self) {
        self.num.store(1, SeqCst);
        drop(self.mutex.lock().unwrap());
        self.condvar.notify_one();
    }

    fn inc_all(&self) {
        self.num.store(1, SeqCst);
        drop(self.mutex.lock().unwrap());
        self.condvar.notify_all();
    }
}
