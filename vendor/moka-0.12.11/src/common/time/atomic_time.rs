use crate::common::time::Instant;

use portable_atomic::AtomicU64;
use std::sync::atomic::Ordering;

/// `AtomicInstant` is a wrapper around `AtomicU64` that provides thread-safe access
/// to an `Instant`.
///
/// `u64::MAX` is used to represent an unset `Instant`.
#[derive(Debug)]
pub(crate) struct AtomicInstant {
    instant: AtomicU64,
}

impl Default for AtomicInstant {
    /// Creates a new `AtomicInstant` with an unset `Instant`.
    fn default() -> Self {
        Self {
            instant: AtomicU64::new(u64::MAX),
        }
    }
}

impl AtomicInstant {
    /// Creates a new `AtomicInstant` with the given `Instant`.
    pub(crate) fn new(instant: Instant) -> Self {
        // Ensure the `Instant` is not `u64::MAX`, which means unset.
        debug_assert!(instant.as_nanos() != u64::MAX);

        Self {
            instant: AtomicU64::new(instant.as_nanos()),
        }
    }

    /// Clears the `Instant`.
    pub(crate) fn clear(&self) {
        self.instant.store(u64::MAX, Ordering::Release);
    }

    /// Returns `true` if the `Instant` is set.
    pub(crate) fn is_set(&self) -> bool {
        self.instant.load(Ordering::Acquire) != u64::MAX
    }

    /// Returns the `Instant` if it is set, otherwise `None`.
    pub(crate) fn instant(&self) -> Option<Instant> {
        let ts = self.instant.load(Ordering::Acquire);
        if ts == u64::MAX {
            None
        } else {
            Some(Instant::from_nanos(ts))
        }
    }

    /// Sets the `Instant`.
    pub(crate) fn set_instant(&self, instant: Instant) {
        // Ensure the `Instant` is not `u64::MAX`, which means unset.
        debug_assert!(instant.as_nanos() != u64::MAX);

        self.instant.store(instant.as_nanos(), Ordering::Release);
    }
}
