//! Future-aware synchronization
//!
//! This module, which is modeled after `std::sync`, contains user-space
//! synchronization tools that work with futures, streams and sinks. In
//! particular, these synchronizers do *not* block physical OS threads, but
//! instead work at the task level.
//!
//! More information and examples of how to use these synchronization primitives
//! can be found [online at tokio.rs].
//!
//! [online at tokio.rs]: https://tokio.rs/docs/going-deeper-futures/synchronization/

pub mod oneshot;
pub mod mpsc;
mod bilock;

pub use self::bilock::{BiLock, BiLockGuard, BiLockAcquire, BiLockAcquired};
