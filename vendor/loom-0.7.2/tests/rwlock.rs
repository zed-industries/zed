use loom::sync::{Arc, RwLock, TryLockResult};
use loom::thread;

use std::rc::Rc;
use std::sync::TryLockError;

#[test]
fn rwlock_read_one() {
    loom::model(|| {
        let lock = Arc::new(RwLock::new(1));
        let c_lock = lock.clone();

        let n = lock.read().unwrap();
        assert_eq!(*n, 1);

        thread::spawn(move || {
            let r = c_lock.read();
            assert!(r.is_ok());
        })
        .join()
        .unwrap();
    });
}

#[test]
fn rwlock_read_two_write_one() {
    loom::model(|| {
        let lock = Arc::new(RwLock::new(1));

        for _ in 0..2 {
            let lock = lock.clone();

            thread::spawn(move || {
                let _l = lock.read().unwrap();

                thread::yield_now();
            });
        }

        let _l = lock.write().unwrap();
        thread::yield_now();
    });
}

#[test]
fn rwlock_write_three() {
    loom::model(|| {
        let lock = Arc::new(RwLock::new(1));

        for _ in 0..2 {
            let lock = lock.clone();
            thread::spawn(move || {
                let _l = lock.write().unwrap();

                thread::yield_now();
            });
        }

        let _l = lock.write().unwrap();
        thread::yield_now();
    });
}

#[test]
fn rwlock_write_then_try_write() {
    loom::model(|| {
        let lock = Arc::new(RwLock::new(1));

        let _l1 = lock.write().unwrap();

        assert!(matches!(
            lock.try_write(),
            TryLockResult::Err(TryLockError::WouldBlock)
        ));
    });
}

#[test]
fn rwlock_write_then_try_read() {
    loom::model(|| {
        let lock = Arc::new(RwLock::new(1));

        let _l1 = lock.write().unwrap();

        assert!(matches!(
            lock.try_read(),
            TryLockResult::Err(TryLockError::WouldBlock)
        ));
    });
}

#[test]
fn rwlock_read_then_try_write() {
    loom::model(|| {
        let lock = Arc::new(RwLock::new(1));

        let _l1 = lock.write().unwrap();

        assert!(matches!(
            lock.try_write(),
            TryLockResult::Err(TryLockError::WouldBlock)
        ));
    });
}

#[test]
fn rwlock_try_read() {
    loom::model(|| {
        let lock = RwLock::new(1);

        match lock.try_read() {
            Ok(n) => assert_eq!(*n, 1),
            Err(_) => unreachable!(),
        };
    });
}

#[test]
fn rwlock_write() {
    loom::model(|| {
        let lock = RwLock::new(1);

        let mut n = lock.write().unwrap();
        *n = 2;

        assert!(lock.try_read().is_err());
    });
}

#[test]
fn rwlock_try_write() {
    loom::model(|| {
        let lock = RwLock::new(1);

        let n = lock.read().unwrap();
        assert_eq!(*n, 1);

        assert!(lock.try_write().is_err());
    });
}

#[test]
fn rwlock_into_inner() {
    loom::model(|| {
        let lock = Rc::new(RwLock::new(0));

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let lock = lock.clone();

                thread::spawn(move || {
                    *lock.write().unwrap() += 1;
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
