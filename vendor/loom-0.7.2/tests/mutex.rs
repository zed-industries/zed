#![deny(warnings, rust_2018_idioms)]

use loom::cell::UnsafeCell;
use loom::sync::atomic::AtomicUsize;
use loom::sync::Mutex;
use loom::thread;

use std::rc::Rc;
use std::sync::atomic::Ordering::SeqCst;

#[test]
fn mutex_enforces_mutal_exclusion() {
    loom::model(|| {
        let data = Rc::new((Mutex::new(0), AtomicUsize::new(0)));

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let data = data.clone();

                thread::spawn(move || {
                    let mut locked = data.0.lock().unwrap();

                    let prev = data.1.fetch_add(1, SeqCst);
                    assert_eq!(prev, *locked);
                    *locked += 1;
                })
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }

        let locked = data.0.lock().unwrap();

        assert_eq!(*locked, data.1.load(SeqCst));
    });
}

#[test]
fn mutex_establishes_seq_cst() {
    loom::model(|| {
        struct Data {
            cell: UnsafeCell<usize>,
            flag: Mutex<bool>,
        }

        let data = Rc::new(Data {
            cell: UnsafeCell::new(0),
            flag: Mutex::new(false),
        });

        {
            let data = data.clone();

            thread::spawn(move || {
                unsafe { data.cell.with_mut(|v| *v = 1) };
                *data.flag.lock().unwrap() = true;
            });
        }

        let flag = *data.flag.lock().unwrap();

        if flag {
            let v = unsafe { data.cell.with(|v| *v) };
            assert_eq!(v, 1);
        }
    });
}

#[test]
fn mutex_into_inner() {
    loom::model(|| {
        let lock = Rc::new(Mutex::new(0));

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let lock = lock.clone();

                thread::spawn(move || {
                    *lock.lock().unwrap() += 1;
                })
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }

        let lock = Rc::try_unwrap(lock).unwrap().into_inner().unwrap();
        assert_eq!(lock, 2);
    })
}
