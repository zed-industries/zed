//! Monotonic clocks

use core::sync::atomic::{AtomicUsize, Ordering};

/// A monotonic source of time.
///
/// Clocks must always returning increasing timestamps.
pub trait Clock: Sync {
    /// Returns a timestamp in milliseconds which represents the current time
    /// according to the clock.
    ///
    /// Clocks must only return timestamps that are bigger or equal than what
    /// they returned on the last call to `now()`.
    fn now(&self) -> u64;
}

/// A [`Clock`] which can be set to arbitrary timestamps for testing purposes.
///
/// It can be used in a test case as demonstrated in the following example:
/// ```
/// use futures_intrusive::timer::MockClock;
/// # #[cfg(feature = "std")]
/// # use futures_intrusive::timer::TimerService;
///
/// static TEST_CLOCK: MockClock = MockClock::new();
/// TEST_CLOCK.set_time(2300); // Set the current time
/// # #[cfg(feature = "std")]
/// let timer = TimerService::new(&TEST_CLOCK);
/// ```
pub struct MockClock {
    now: core::sync::atomic::AtomicUsize,
}

impl core::fmt::Debug for MockClock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let now = self.now();
        f.debug_struct("MockClock").field("now", &now).finish()
    }
}

impl MockClock {
    /// Creates a new instance of the [`MockClock`], which is initialized to
    /// timestamp 0.
    pub const fn new() -> MockClock {
        MockClock {
            now: AtomicUsize::new(0),
        }
    }

    /// Sets the current timestamp inside to [`MockClock`] to the given value
    pub fn set_time(&self, timestamp: u64) {
        if timestamp > (core::usize::MAX as u64) {
            panic!("timestamps bigger than usize::MAX are not supported")
        }
        let to_set = timestamp as usize;
        self.now.store(to_set, Ordering::Release);
    }
}

impl Clock for MockClock {
    fn now(&self) -> u64 {
        self.now.load(Ordering::Relaxed) as u64
    }
}

#[cfg(feature = "std")]
mod if_std {
    use super::*;
    use std::time::Instant;

    /// A Clock that makes use of the Standard libraries [`std::time::Instant`]
    /// functionality in order to generate monotonically increasing timestamps.
    pub struct StdClock {
        start: Instant,
    }

    impl core::fmt::Debug for StdClock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StdClock").finish()
        }
    }

    impl StdClock {
        /// Creates a new [`StdClock`]
        pub fn new() -> StdClock {
            StdClock {
                start: Instant::now(),
            }
        }
    }

    impl Clock for StdClock {
        fn now(&self) -> u64 {
            let elapsed = Instant::now() - self.start;
            elapsed.as_millis() as u64
        }
    }
}

#[cfg(feature = "std")]
pub use self::if_std::*;
