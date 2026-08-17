use std::pin::pin;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

use loom::sync::atomic::AtomicBool;
use loom::thread::{spawn, yield_now};

use crate::gate;
use crate::{Barrier, Gate, Lock, Pager, Semaphore};

#[test]
fn lock_shared() {
    loom::model(|| {
        let lock = Arc::new(Lock::default());
        let check = Arc::new(AtomicBool::new(false));

        lock.lock_sync();

        let lock_clone = lock.clone();
        let check_clone = check.clone();
        let thread_1 = spawn(move || {
            assert!(lock_clone.share_sync());
            assert!(check_clone.load(Relaxed));
        });

        let lock_clone = lock.clone();
        let check_clone = check.clone();
        let thread_2 = spawn(move || {
            assert!(lock_clone.share_sync());
            assert!(check_clone.load(Relaxed));
        });

        check.store(true, Relaxed);
        assert!(lock.release_lock());
        assert!(thread_1.join().is_ok());
        assert!(thread_2.join().is_ok());
        assert!(lock.release_share());
        assert!(lock.release_share());
    });
}

#[test]
fn lock_exclusive() {
    loom::model(|| {
        let lock = Arc::new(Lock::default());
        let check = Arc::new(AtomicBool::new(false));

        lock.lock_sync();

        let lock_clone = lock.clone();
        let check_clone = check.clone();
        let thread_1 = spawn(move || {
            assert!(lock_clone.lock_sync());
            assert!(check_clone.load(Relaxed));
            assert!(lock_clone.release_lock());
        });

        let lock_clone = lock.clone();
        let check_clone = check.clone();
        let thread_2 = spawn(move || {
            assert!(lock_clone.lock_sync());
            assert!(check_clone.load(Relaxed));
            assert!(lock_clone.release_lock());
        });

        check.store(true, Relaxed);
        assert!(lock.release_lock());
        assert!(thread_1.join().is_ok());
        assert!(thread_2.join().is_ok());
    });
}

#[test]
fn share_poison() {
    loom::model(|| {
        let lock = Arc::new(Lock::default());

        lock.lock_sync();

        let lock_clone = lock.clone();
        let thread_1 = spawn(move || {
            if !lock_clone.share_sync() {
                assert!(lock_clone.is_poisoned(Relaxed));
            }
        });

        let lock_clone = lock.clone();
        let thread_2 = spawn(move || {
            if !lock_clone.share_sync() {
                assert!(lock_clone.is_poisoned(Relaxed));
            }
        });

        assert!(lock.poison_lock());
        assert!(thread_1.join().is_ok());
        assert!(thread_2.join().is_ok());
    });
}

#[test]
fn lock_poison() {
    loom::model(|| {
        let lock = Arc::new(Lock::default());

        lock.lock_sync();

        let lock_clone = lock.clone();
        let thread_1 = spawn(move || {
            assert!(lock_clone.lock_sync());
            assert!(lock_clone.poison_lock());
        });

        let lock_clone = lock.clone();
        let thread_2 = spawn(move || {
            if lock_clone.lock_sync() {
                assert!(lock_clone.release_lock());
            } else {
                assert!(lock_clone.is_poisoned(Relaxed));
            }
        });

        assert!(lock.release_lock());

        assert!(thread_1.join().is_ok());
        assert!(thread_2.join().is_ok());
        assert!(lock.is_poisoned(Relaxed));
    });
}

#[test]
fn barrier() {
    loom::model(|| {
        let barrier = Arc::new(Barrier::with_count(2));
        let check = Arc::new(AtomicBool::new(false));

        let barrier_clone = barrier.clone();
        let check_clone = check.clone();
        let thread = spawn(move || {
            if barrier_clone.wait_sync() {
                assert!(!check_clone.swap(true, Relaxed));
            }
            barrier_clone.wait_sync();
        });

        if barrier.wait_sync() {
            assert!(!check.swap(true, Relaxed));
        }
        barrier.wait_sync();
        assert!(thread.join().is_ok());
        assert!(check.load(Relaxed));
    });
}

#[test]
fn semaphore_release_acquire() {
    loom::model(|| {
        let semaphore = Arc::new(Semaphore::default());
        let check = Arc::new(AtomicBool::new(false));

        semaphore.acquire_many_sync(Semaphore::MAX_PERMITS);

        let semaphore_clone = semaphore.clone();
        let check_clone = check.clone();
        let thread = spawn(move || {
            semaphore_clone.acquire_many_sync(9);
            assert!(check_clone.load(Relaxed));
        });

        check.store(true, Relaxed);
        assert!(semaphore.release_many(8));
        assert!(semaphore.release());
        assert!(thread.join().is_ok());
    });
}

#[test]
fn gate_enter() {
    loom::model(|| {
        let gate = Arc::new(Gate::default());
        let gate_clone = gate.clone();
        let thread = spawn(move || {
            assert_eq!(gate_clone.enter_sync(), Ok(gate::State::Controlled));
        });

        loop {
            if gate.permit() == Ok(1) {
                break;
            }
            yield_now();
        }
        assert!(thread.join().is_ok());
    });
}

#[test]
fn gate_seal() {
    loom::model(|| {
        let gate = Arc::new(Gate::default());
        let gate_clone = gate.clone();
        let thread = spawn(move || {
            assert_eq!(gate_clone.enter_sync(), Err(gate::Error::Sealed));
        });

        gate.seal();
        assert!(thread.join().is_ok());
    });
}

#[test]
fn drop_future() {
    loom::model(|| {
        let semaphore = Arc::new(Semaphore::default());
        semaphore.acquire_many_sync(Semaphore::MAX_PERMITS);
        let semaphore_clone = semaphore.clone();
        let thread = spawn(move || {
            semaphore_clone.acquire_many_sync(9);
        });
        {
            let mut pinned_pager = pin!(Pager::default());
            assert!(semaphore.register_pager(&mut pinned_pager, 11, false));
        }

        assert!(semaphore.release_many(Semaphore::MAX_PERMITS));
        assert!(thread.join().is_ok());
    });
}
