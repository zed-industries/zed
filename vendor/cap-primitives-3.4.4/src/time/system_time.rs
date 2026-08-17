use crate::time::{Duration, SystemTimeError};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::{fmt, time};

/// A measurement of the system clock, useful for talking to external entities
/// like the file system or other processes.
///
/// This corresponds to [`std::time::SystemTime`].
///
/// This `SystemTime` has no `now`, `elapsed` methods. To obtain the current
/// time or measure the duration to the current time, first obtain a
/// [`SystemClock`], and then call [`SystemClock::now`] or
/// [`SystemClock::elapsed`] instead. The `UNIX_EPOCH` constant is at
/// [`SystemClock::UNIX_EPOCH`].
///
/// Similar to the [`filetime` crate], when
/// `RUSTFLAGS=--cfg emulate_second_only_system` is set, `SystemTime` will
/// round times from the operating system down to the second. This emulates
/// the behavior of some file systems, mostly
/// [HFS], allowing debugging on other hardware.
///
/// [`SystemClock`]: crate::time::SystemClock
/// [`SystemClock::now`]: crate::time::SystemClock::now
/// [`SystemClock::elapsed`]: crate::time::SystemClock::elapsed
/// [`SystemClock::UNIX_EPOCH`]: crate::time::SystemClock::UNIX_EPOCH
/// [`filetime` crate]: https://crates.io/crates/filetime
/// [HFS]: https://en.wikipedia.org/wiki/HFS_Plus
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SystemTime {
    pub(crate) std: time::SystemTime,
}

impl SystemTime {
    /// Constructs a new instance of `Self` from the given
    /// [`std::time::SystemTime`].
    // TODO: Make this a `const fn` once `time::Duration::checked_add` is a `const fn`.
    #[inline]
    pub fn from_std(std: time::SystemTime) -> Self {
        if cfg!(emulate_second_only_system) {
            match std.duration_since(time::SystemTime::UNIX_EPOCH) {
                Ok(duration) => {
                    let secs = time::Duration::from_secs(duration.as_secs());
                    Self {
                        std: time::SystemTime::UNIX_EPOCH.checked_add(secs).unwrap(),
                    }
                }
                Err(_) => {
                    let duration = time::SystemTime::UNIX_EPOCH.duration_since(std).unwrap();
                    let secs = time::Duration::from_secs(duration.as_secs());
                    Self {
                        std: time::SystemTime::UNIX_EPOCH.checked_sub(secs).unwrap(),
                    }
                }
            }
        } else {
            Self { std }
        }
    }

    /// Constructs a new instance of [`std::time::SystemTime`] from the given
    /// `Self`.
    #[inline]
    pub const fn into_std(self) -> time::SystemTime {
        self.std
    }

    /// Returns the amount of time elapsed from another instant to this one.
    ///
    /// This corresponds to [`std::time::SystemTime::duration_since`].
    #[inline]
    pub fn duration_since(&self, earlier: Self) -> Result<Duration, SystemTimeError> {
        self.std.duration_since(earlier.std)
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be
    /// represented as `SystemTime` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// This corresponds to [`std::time::SystemTime::checked_add`].
    #[inline]
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.std.checked_add(duration).map(Self::from_std)
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be
    /// represented as `SystemTime` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// This corresponds to [`std::time::SystemTime::checked_sub`].
    #[inline]
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.std.checked_sub(duration).map(Self::from_std)
    }
}

impl Add<Duration> for SystemTime {
    type Output = Self;

    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be
    /// represented by the underlying data structure. See
    /// [`SystemTime::checked_add`] for a version without panic.
    #[inline]
    fn add(self, dur: Duration) -> Self {
        self.checked_add(dur)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for SystemTime {
    #[inline]
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for SystemTime {
    type Output = Self;

    #[inline]
    fn sub(self, dur: Duration) -> Self {
        self.checked_sub(dur)
            .expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for SystemTime {
    #[inline]
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

impl fmt::Debug for SystemTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
