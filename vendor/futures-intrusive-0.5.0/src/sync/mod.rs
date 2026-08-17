//! Asynchronous synchronization primitives based on intrusive collections.
//!
//! This module provides various primitives for synchronizing concurrently
//! executing futures.

mod manual_reset_event;

pub use self::manual_reset_event::{
    GenericManualResetEvent, GenericWaitForEventFuture, LocalManualResetEvent,
    LocalWaitForEventFuture,
};

#[cfg(feature = "std")]
pub use self::manual_reset_event::{ManualResetEvent, WaitForEventFuture};

mod mutex;

pub use self::mutex::{
    GenericMutex, GenericMutexGuard, GenericMutexLockFuture, LocalMutex,
    LocalMutexGuard, LocalMutexLockFuture,
};

#[cfg(feature = "std")]
pub use self::mutex::{Mutex, MutexGuard, MutexLockFuture};

mod semaphore;

pub use self::semaphore::{
    GenericSemaphore, GenericSemaphoreAcquireFuture, GenericSemaphoreReleaser,
    LocalSemaphore, LocalSemaphoreAcquireFuture, LocalSemaphoreReleaser,
};

#[cfg(feature = "alloc")]
pub use self::semaphore::{
    GenericSharedSemaphore, GenericSharedSemaphoreAcquireFuture,
    GenericSharedSemaphoreReleaser,
};

#[cfg(feature = "std")]
pub use self::semaphore::{
    Semaphore, SemaphoreAcquireFuture, SemaphoreReleaser, SharedSemaphore,
    SharedSemaphoreAcquireFuture, SharedSemaphoreReleaser,
};
