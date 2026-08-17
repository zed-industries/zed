use crate::time::Duration;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::{fmt, time};

/// A measurement of a monotonically nondecreasing clock.
///
/// This corresponds to [`std::time::Instant`].
///
/// This `Instant` has no `now` or `elapsed` methods. To obtain the current
/// time or measure the duration to the current time, first obtain a
/// [`MonotonicClock`], and then call [`MonotonicClock::now`] or
/// [`MonotonicClock::elapsed`] instead.
///
/// [`MonotonicClock`]: crate::time::MonotonicClock
/// [`MonotonicClock::now`]: crate::time::MonotonicClock::now
/// [`MonotonicClock::elapsed`]: crate::time::MonotonicClock::elapsed
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Instant {
    pub(crate) std: time::Instant,
}

impl Instant {
    /// Constructs a new instance of `Self` from the given
    /// [`std::time::Instant`].
    #[inline]
    pub const fn from_std(std: time::Instant) -> Self {
        Self { std }
    }

    /// Returns the amount of time elapsed from another instant to this one.
    ///
    /// This corresponds to [`std::time::Instant::duration_since`].
    #[inline]
    pub fn duration_since(&self, earlier: Self) -> Duration {
        self.std.duration_since(earlier.std)
    }

    /// Returns the amount of time elapsed from another instant to this one, or
    /// None if that instant is later than this one.
    ///
    /// This corresponds to [`std::time::Instant::checked_duration_since`].
    #[inline]
    pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
        self.std.checked_duration_since(earlier.std)
    }

    /// Returns the amount of time elapsed from another instant to this one, or
    /// zero duration if that instant is later than this one.
    ///
    /// This corresponds to [`std::time::Instant::saturating_duration_since`].
    #[inline]
    pub fn saturating_duration_since(&self, earlier: Self) -> Duration {
        self.std.saturating_duration_since(earlier.std)
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// This corresponds to [`std::time::Instant::checked_add`].
    #[inline]
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.std.checked_add(duration).map(Self::from_std)
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// This corresponds to [`std::time::Instant::checked_sub`].
    #[inline]
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.std.checked_sub(duration).map(Self::from_std)
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be
    /// represented by the underlying data structure. See
    /// [`Instant::checked_add`] for a version without panic.
    #[inline]
    fn add(self, other: Duration) -> Self {
        self.checked_add(other)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    #[inline]
    fn sub(self, other: Duration) -> Self {
        self.checked_sub(other)
            .expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, other: Self) -> Duration {
        self.duration_since(other)
    }
}

impl fmt::Debug for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
