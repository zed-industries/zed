use std::time::{Duration, Instant as StdInstant};

#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
use parking_lot::RwLock;

// This is `moka`'s `Instant` struct.
use super::Instant;

#[derive(Default, Clone)]
pub(crate) struct Clock {
    ty: ClockType,
}

#[derive(Clone)]
enum ClockType {
    /// A clock that uses `std::time::Instant` as the source of time.
    Standard { origin: StdInstant },
    #[cfg(feature = "quanta")]
    /// A clock that uses both `std::time::Instant` and `quanta::Instant` as the
    /// sources of time.
    Hybrid {
        std_origin: StdInstant,
        quanta_origin: quanta::Instant,
    },
    #[cfg(test)]
    /// A clock that uses a mocked source of time.
    Mocked { mock: Arc<Mock> },
}

impl Default for ClockType {
    /// Create a new `ClockType` with the current time as the origin.
    ///
    /// If the `quanta` feature is enabled, `Hybrid` will be used. Otherwise,
    /// `Standard` will be used.
    fn default() -> Self {
        #[cfg(feature = "quanta")]
        {
            return ClockType::Hybrid {
                std_origin: StdInstant::now(),
                quanta_origin: quanta::Instant::now(),
            };
        }

        #[allow(unreachable_code)]
        ClockType::Standard {
            origin: StdInstant::now(),
        }
    }
}

impl Clock {
    #[cfg(test)]
    /// Creates a new `Clock` with a mocked source of time.
    pub(crate) fn mock() -> (Clock, Arc<Mock>) {
        let mock = Arc::new(Mock::default());
        let clock = Clock {
            ty: ClockType::Mocked {
                mock: Arc::clone(&mock),
            },
        };
        (clock, mock)
    }

    /// Returns the current time using a reliable source of time.
    ///
    /// When the type is `Standard` or `Hybrid`, the time is based on
    /// `std::time::Instant`. When the type is `Mocked`, the time is based on the
    /// mocked source of time.
    pub(crate) fn now(&self) -> Instant {
        match &self.ty {
            ClockType::Standard { origin } => {
                Instant::from_duration_since_clock_start(origin.elapsed())
            }
            #[cfg(feature = "quanta")]
            ClockType::Hybrid { std_origin, .. } => {
                Instant::from_duration_since_clock_start(std_origin.elapsed())
            }
            #[cfg(test)]
            ClockType::Mocked { mock } => Instant::from_duration_since_clock_start(mock.elapsed()),
        }
    }

    /// Returns the current time _maybe_ using a fast but less reliable source of
    /// time. The time may drift from the time returned by `now`, or not be
    /// monotonically increasing.
    ///
    /// This is useful for performance critical code that does not require the same
    /// level of precision as `now`. (e.g. measuring the time between two events for
    /// metrics)
    ///
    /// When the type is `Standard` or `Mocked`, `now` is internally called. So there
    /// is no performance benefit.
    ///
    /// When the type is `Hybrid`, the time is based on `quanta::Instant`, which can
    /// be faster than `std::time::Instant`, depending on the CPU architecture.
    pub(crate) fn fast_now(&self) -> Instant {
        match &self.ty {
            #[cfg(feature = "quanta")]
            ClockType::Hybrid { quanta_origin, .. } => {
                Instant::from_duration_since_clock_start(quanta_origin.elapsed())
            }
            ClockType::Standard { .. } => self.now(),
            #[cfg(test)]
            ClockType::Mocked { .. } => self.now(),
        }
    }

    /// Converts an `Instant` to a `std::time::Instant`.
    ///
    /// **IMPORTANT**: The caller must ensure that the `Instant` was created by this
    /// `Clock`, otherwise the resulting `std::time::Instant` will be incorrect.
    pub(crate) fn to_std_instant(&self, instant: Instant) -> StdInstant {
        match &self.ty {
            ClockType::Standard { origin } => {
                let duration = Duration::from_nanos(instant.as_nanos());
                *origin + duration
            }
            #[cfg(feature = "quanta")]
            ClockType::Hybrid { std_origin, .. } => {
                let duration = Duration::from_nanos(instant.as_nanos());
                *std_origin + duration
            }
            #[cfg(test)]
            ClockType::Mocked { mock } => {
                let duration = Duration::from_nanos(instant.as_nanos());
                // https://github.com/moka-rs/moka/issues/487
                //
                // This `dbg!` will workaround an incorrect compilation by Rust
                // 1.84.0 for the armv7-unknown-linux-musleabihf target in the
                // release build of the tests.
                dbg!(mock.origin + duration)
            }
        }
    }
}

#[cfg(test)]
pub(crate) struct Mock {
    origin: StdInstant,
    now: RwLock<StdInstant>,
}

#[cfg(test)]
impl Default for Mock {
    fn default() -> Self {
        let origin = StdInstant::now();
        Self {
            origin,
            now: RwLock::new(origin),
        }
    }
}

#[cfg(test)]
impl Mock {
    pub(crate) fn increment(&self, amount: Duration) {
        *self.now.write() += amount;
    }

    pub(crate) fn elapsed(&self) -> Duration {
        self.now.read().duration_since(self.origin)
    }
}
