use crate::time::{Duration, Instant};
use ambient_authority::AmbientAuthority;
use std::time;

/// A reference to a monotonically nondecreasing clock.
///
/// This does not directly correspond to anything in `std`, however its methods
/// correspond to [methods in `std::time::Instant`].
///
/// [methods in `std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html#impl
pub struct MonotonicClock(());

impl MonotonicClock {
    /// Constructs a new instance of `Self`.
    ///
    /// # Ambient Authority
    ///
    /// This uses ambient authority to accesses clocks.
    #[inline]
    pub const fn new(ambient_authority: AmbientAuthority) -> Self {
        let _ = ambient_authority;
        Self(())
    }

    /// Returns an instant corresponding to "now".
    ///
    /// This corresponds to [`std::time::Instant::now`].
    #[inline]
    pub fn now(&self) -> Instant {
        Instant::from_std(time::Instant::now())
    }

    /// Returns the amount of time elapsed since this instant was created.
    ///
    /// This corresponds to [`std::time::Instant::elapsed`].
    #[inline]
    pub fn elapsed(&self, instant: Instant) -> Duration {
        instant.std.elapsed()
    }
}
