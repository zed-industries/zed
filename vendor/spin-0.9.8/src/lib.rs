#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

//! This crate provides [spin-based](https://en.wikipedia.org/wiki/Spinlock) versions of the
//! primitives in `std::sync` and `std::lazy`. Because synchronization is done through spinning,
//! the primitives are suitable for use in `no_std` environments.
//!
//! # Features
//!
//! - `Mutex`, `RwLock`, `Once`/`SyncOnceCell`, and `SyncLazy` equivalents
//!
//! - Support for `no_std` environments
//!
//! - [`lock_api`](https://crates.io/crates/lock_api) compatibility
//!
//! - Upgradeable `RwLock` guards
//!
//! - Guards can be sent and shared between threads
//!
//! - Guard leaking
//!
//! - Ticket locks
//!
//! - Different strategies for dealing with contention
//!
//! # Relationship with `std::sync`
//!
//! While `spin` is not a drop-in replacement for `std::sync` (and
//! [should not be considered as such](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html))
//! an effort is made to keep this crate reasonably consistent with `std::sync`.
//!
//! Many of the types defined in this crate have 'additional capabilities' when compared to `std::sync`:
//!
//! - Because spinning does not depend on the thread-driven model of `std::sync`, guards ([`MutexGuard`],
//!   [`RwLockReadGuard`], [`RwLockWriteGuard`], etc.) may be sent and shared between threads.
//!
//! - [`RwLockUpgradableGuard`] supports being upgraded into a [`RwLockWriteGuard`].
//!
//! - Guards support [leaking](https://doc.rust-lang.org/nomicon/leaking.html).
//!
//! - [`Once`] owns the value returned by its `call_once` initializer.
//!
//! - [`RwLock`] supports counting readers and writers.
//!
//! Conversely, the types in this crate do not have some of the features `std::sync` has:
//!
//! - Locks do not track [panic poisoning](https://doc.rust-lang.org/nomicon/poisoning.html).
//!
//! ## Feature flags
//!
//! The crate comes with a few feature flags that you may wish to use.
//!
//! - `lock_api` enables support for [`lock_api`](https://crates.io/crates/lock_api)
//!
//! - `ticket_mutex` uses a ticket lock for the implementation of `Mutex`
//!
//! - `fair_mutex` enables a fairer implementation of `Mutex` that uses eventual fairness to avoid
//!   starvation
//!
//! - `std` enables support for thread yielding instead of spinning

#[cfg(any(test, feature = "std"))]
extern crate core;

#[cfg(feature = "portable_atomic")]
extern crate portable_atomic;

#[cfg(not(feature = "portable_atomic"))]
use core::sync::atomic;
#[cfg(feature = "portable_atomic")]
use portable_atomic as atomic;

#[cfg(feature = "barrier")]
#[cfg_attr(docsrs, doc(cfg(feature = "barrier")))]
pub mod barrier;
#[cfg(feature = "lazy")]
#[cfg_attr(docsrs, doc(cfg(feature = "lazy")))]
pub mod lazy;
#[cfg(feature = "mutex")]
#[cfg_attr(docsrs, doc(cfg(feature = "mutex")))]
pub mod mutex;
#[cfg(feature = "once")]
#[cfg_attr(docsrs, doc(cfg(feature = "once")))]
pub mod once;
pub mod relax;
#[cfg(feature = "rwlock")]
#[cfg_attr(docsrs, doc(cfg(feature = "rwlock")))]
pub mod rwlock;

#[cfg(feature = "mutex")]
#[cfg_attr(docsrs, doc(cfg(feature = "mutex")))]
pub use mutex::MutexGuard;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use relax::Yield;
pub use relax::{RelaxStrategy, Spin};
#[cfg(feature = "rwlock")]
#[cfg_attr(docsrs, doc(cfg(feature = "rwlock")))]
pub use rwlock::RwLockReadGuard;

// Avoid confusing inference errors by aliasing away the relax strategy parameter. Users that need to use a different
// relax strategy can do so by accessing the types through their fully-qualified path. This is a little bit horrible
// but sadly adding a default type parameter is *still* a breaking change in Rust (for understandable reasons).

/// A primitive that synchronizes the execution of multiple threads. See [`barrier::Barrier`] for documentation.
///
/// A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
/// strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.
#[cfg(feature = "barrier")]
#[cfg_attr(docsrs, doc(cfg(feature = "barrier")))]
pub type Barrier = crate::barrier::Barrier;

/// A value which is initialized on the first access. See [`lazy::Lazy`] for documentation.
///
/// A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
/// strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.
#[cfg(feature = "lazy")]
#[cfg_attr(docsrs, doc(cfg(feature = "lazy")))]
pub type Lazy<T, F = fn() -> T> = crate::lazy::Lazy<T, F>;

/// A primitive that synchronizes the execution of multiple threads. See [`mutex::Mutex`] for documentation.
///
/// A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
/// strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.
#[cfg(feature = "mutex")]
#[cfg_attr(docsrs, doc(cfg(feature = "mutex")))]
pub type Mutex<T> = crate::mutex::Mutex<T>;

/// A primitive that provides lazy one-time initialization. See [`once::Once`] for documentation.
///
/// A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
/// strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.
#[cfg(feature = "once")]
#[cfg_attr(docsrs, doc(cfg(feature = "once")))]
pub type Once<T = ()> = crate::once::Once<T>;

/// A lock that provides data access to either one writer or many readers. See [`rwlock::RwLock`] for documentation.
///
/// A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
/// strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.
#[cfg(feature = "rwlock")]
#[cfg_attr(docsrs, doc(cfg(feature = "rwlock")))]
pub type RwLock<T> = crate::rwlock::RwLock<T>;

/// A guard that provides immutable data access but can be upgraded to [`RwLockWriteGuard`]. See
/// [`rwlock::RwLockUpgradableGuard`] for documentation.
///
/// A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
/// strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.
#[cfg(feature = "rwlock")]
#[cfg_attr(docsrs, doc(cfg(feature = "rwlock")))]
pub type RwLockUpgradableGuard<'a, T> = crate::rwlock::RwLockUpgradableGuard<'a, T>;

/// A guard that provides mutable data access. See [`rwlock::RwLockWriteGuard`] for documentation.
///
/// A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
/// strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.
#[cfg(feature = "rwlock")]
#[cfg_attr(docsrs, doc(cfg(feature = "rwlock")))]
pub type RwLockWriteGuard<'a, T> = crate::rwlock::RwLockWriteGuard<'a, T>;

/// Spin synchronisation primitives, but compatible with [`lock_api`](https://crates.io/crates/lock_api).
#[cfg(feature = "lock_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "lock_api")))]
pub mod lock_api {
    /// A lock that provides mutually exclusive data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
    #[cfg(feature = "mutex")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mutex")))]
    pub type Mutex<T> = lock_api_crate::Mutex<crate::Mutex<()>, T>;

    /// A guard that provides mutable data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
    #[cfg(feature = "mutex")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mutex")))]
    pub type MutexGuard<'a, T> = lock_api_crate::MutexGuard<'a, crate::Mutex<()>, T>;

    /// A lock that provides data access to either one writer or many readers (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
    #[cfg(feature = "rwlock")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rwlock")))]
    pub type RwLock<T> = lock_api_crate::RwLock<crate::RwLock<()>, T>;

    /// A guard that provides immutable data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
    #[cfg(feature = "rwlock")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rwlock")))]
    pub type RwLockReadGuard<'a, T> = lock_api_crate::RwLockReadGuard<'a, crate::RwLock<()>, T>;

    /// A guard that provides mutable data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
    #[cfg(feature = "rwlock")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rwlock")))]
    pub type RwLockWriteGuard<'a, T> = lock_api_crate::RwLockWriteGuard<'a, crate::RwLock<()>, T>;

    /// A guard that provides immutable data access but can be upgraded to [`RwLockWriteGuard`] (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
    #[cfg(feature = "rwlock")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rwlock")))]
    pub type RwLockUpgradableReadGuard<'a, T> =
        lock_api_crate::RwLockUpgradableReadGuard<'a, crate::RwLock<()>, T>;
}

/// In the event of an invalid operation, it's best to abort the current process.
#[cfg(feature = "fair_mutex")]
fn abort() -> ! {
    #[cfg(not(feature = "std"))]
    {
        // Panicking while panicking is defined by Rust to result in an abort.
        struct Panic;

        impl Drop for Panic {
            fn drop(&mut self) {
                panic!("aborting due to invalid operation");
            }
        }

        let _panic = Panic;
        panic!("aborting due to invalid operation");
    }

    #[cfg(feature = "std")]
    {
        std::process::abort();
    }
}
