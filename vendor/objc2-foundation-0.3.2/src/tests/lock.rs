#![cfg(feature = "NSLock")]
use crate::{NSLock, NSLocking};

#[test]
fn lock_unlock() {
    let lock = NSLock::new();
    // SAFETY: Unlocked from the same thread that locked.
    unsafe {
        lock.lock();
        assert!(!lock.tryLock());
        lock.unlock();
        assert!(lock.tryLock());
        lock.unlock();
    }
}
