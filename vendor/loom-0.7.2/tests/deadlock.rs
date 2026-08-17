#![deny(warnings, rust_2018_idioms)]

use loom::sync::Mutex;
use loom::thread;

use std::rc::Rc;

#[test]
#[should_panic]
fn two_mutexes_deadlock() {
    loom::model(|| {
        let a = Rc::new(Mutex::new(1));
        let b = Rc::new(Mutex::new(2));

        let th1 = {
            let a = a.clone();
            let b = b.clone();

            thread::spawn(move || {
                let a_lock = a.lock().unwrap();
                let b_lock = b.lock().unwrap();
                assert_eq!(*a_lock + *b_lock, 3);
            })
        };
        let th2 = {
            thread::spawn(move || {
                let b_lock = b.lock().unwrap();
                let a_lock = a.lock().unwrap();
                assert_eq!(*a_lock + *b_lock, 3);
            })
        };
        th1.join().unwrap();
        th2.join().unwrap();
    });
}
