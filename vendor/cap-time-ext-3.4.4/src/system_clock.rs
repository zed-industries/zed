#[cfg(not(windows))]
use rustix::time::{clock_getres, ClockId};
use std::time::{self, Duration};

/// Extension trait for `cap_std::time::SystemClock`.
pub trait SystemClockExt {
    /// A system clock datapoint.
    type SystemTime;

    /// Similar to `SystemClock::now`, but takes an additional `precision`
    /// parameter allowing callers to inform the implementation when they
    /// don't need full precision. The implementation need not make any
    /// effort to provide a time with greater precision.
    fn now_with(&self, precision: Duration) -> Self::SystemTime;

    /// Return the resolution of the clock.
    fn resolution(&self) -> Duration;
}

#[cfg(not(windows))]
impl SystemClockExt for cap_primitives::time::SystemClock {
    type SystemTime = cap_primitives::time::SystemTime;

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn now_with(&self, _precision: Duration) -> Self::SystemTime {
        // On systems with no optimized form of `clock_gettime`, ignore the
        // precision argument.
        Self::SystemTime::from_std(time::SystemTime::now())
    }

    fn resolution(&self) -> Duration {
        let spec = clock_getres(ClockId::Realtime);
        Duration::new(
            spec.tv_sec.try_into().unwrap(),
            spec.tv_nsec.try_into().unwrap(),
        )
    }
}

#[cfg(windows)]
impl SystemClockExt for cap_primitives::time::SystemClock {
    type SystemTime = cap_primitives::time::SystemTime;

    #[inline]
    fn now_with(&self, _precision: Duration) -> Self::SystemTime {
        // On systems with no optimized form of `clock_gettime`, ignore the
        // precision argument.
        Self::SystemTime::from_std(time::SystemTime::now())
    }

    fn resolution(&self) -> Duration {
        // According to [this blog post], the system timer resolution is 55ms
        // or 10ms. Use the more conservative of the two.
        //
        // [this blog post]: https://devblogs.microsoft.com/oldnewthing/20170921-00/?p=97057
        Duration::new(0, 55_000_000)
    }
}
