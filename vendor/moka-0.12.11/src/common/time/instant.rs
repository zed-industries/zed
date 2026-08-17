use std::time::Duration;

pub(crate) const MAX_NANOS: u64 = u64::MAX - 1;

/// `Instant` represents a point in time since the `Clock` was created. It has
/// nanosecond precision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Instant {
    elapsed_ns: u64,
}

impl Instant {
    pub(crate) fn from_nanos(nanos: u64) -> Instant {
        debug_assert!(nanos <= MAX_NANOS);
        Instant { elapsed_ns: nanos }
    }

    pub(crate) fn from_duration_since_clock_start(duration: Duration) -> Instant {
        Instant::from_nanos(Self::duration_to_saturating_nanoseconds(duration))
    }

    pub(crate) fn as_nanos(&self) -> u64 {
        self.elapsed_ns
    }

    /// Converts a `std::time::Duration` to nanoseconds, saturating to
    /// `MAX_NANOSECONDS` (`u64::MAX - 1`) if the duration is too large.
    /// (`Duration::as_nanos` returns `u128`)
    ///
    /// Note that `u64::MAX - 1` is used here instead of `u64::MAX` because
    /// `u64::MAX` is used by `moka`'s `AtomicTime` to indicate the time is unset.
    pub(crate) fn duration_to_saturating_nanoseconds(duration: Duration) -> u64 {
        u64::try_from(duration.as_nanos())
            .map(|n| n.min(MAX_NANOS))
            .unwrap_or(MAX_NANOS)
    }

    pub(crate) fn saturating_add(&self, duration: Duration) -> Instant {
        let dur_ms = Self::duration_to_saturating_nanoseconds(duration);
        Instant::from_nanos(self.elapsed_ns.saturating_add(dur_ms).min(MAX_NANOS))
    }

    pub(crate) fn saturating_duration_since(&self, earlier: Self) -> Duration
    where
        Self: Sized,
    {
        Duration::from_nanos(self.elapsed_ns.saturating_sub(earlier.elapsed_ns))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saturating_add() {
        let instant = Instant::from_nanos(100_000);
        let duration = Duration::from_nanos(50_000);
        let result = instant.saturating_add(duration);
        assert_eq!(result, Instant::from_nanos(150_000));

        let instant = Instant::from_nanos(u64::MAX - 10_000);
        let duration = Duration::from_nanos(12_000);
        let result = instant.saturating_add(duration);
        assert_eq!(result, Instant::from_nanos(u64::MAX - 1));
    }

    #[test]
    fn test_saturating_duration_since() {
        let instant = Instant::from_nanos(100_000);
        let earlier = Instant::from_nanos(60_000);
        let result = instant.saturating_duration_since(earlier);
        assert_eq!(result, Duration::from_nanos(40_000));

        let instant = Instant::from_nanos(60_000);
        let earlier = Instant::from_nanos(100_000);
        let result = instant.saturating_duration_since(earlier);
        assert_eq!(result, Duration::ZERO);
    }
}
