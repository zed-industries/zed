//! An unsafe (non-thread-safe) lock, equivalent to UnsafeCell

use core::marker::PhantomData;
use lock_api::{GuardSend, RawMutex};

/// An unsafe (non-thread-safe) lock, equivalent to UnsafeCell
#[derive(Debug)]
pub struct NoopLock {
    /// Assigned in order to make the type !Sync
    _phantom: PhantomData<*mut ()>,
}

unsafe impl RawMutex for NoopLock {
    const INIT: NoopLock = NoopLock {
        _phantom: PhantomData,
    };

    type GuardMarker = GuardSend;

    fn lock(&self) {}

    fn try_lock(&self) -> bool {
        true
    }

    unsafe fn unlock(&self) {}
}
