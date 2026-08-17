//! Archived versions of `time` types.

use crate::Archived;

/// An archived [`Duration`](core::time::Duration).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedDuration {
    secs: Archived<u64>,
    nanos: Archived<u32>,
}

const NANOS_PER_SEC: u32 = 1_000_000_000;
const NANOS_PER_MILLI: u32 = 1_000_000;
const NANOS_PER_MICRO: u32 = 1_000;
const MILLIS_PER_SEC: u64 = 1_000;
const MICROS_PER_SEC: u64 = 1_000_000;

impl ArchivedDuration {
    /// Returns the number of _whole_ seconds contained by this
    /// `ArchivedDuration`.
    ///
    /// The returned value does not include the fractional (nanosecond) part of the duration, which
    /// can be obtained using [`subsec_nanos`].
    ///
    /// [`subsec_nanos`]: ArchivedDuration::subsec_nanos
    #[inline]
    pub const fn as_secs(&self) -> u64 {
        from_archived!(self.secs)
    }

    /// Returns the fractional part of this `ArchivedDuration`, in whole milliseconds.
    ///
    /// This method does **not** return the length of the duration when represented by milliseconds.
    /// The returned number always represents a fractional portion of a second (i.e., it is less
    /// than one thousand).
    #[inline]
    pub const fn subsec_millis(&self) -> u32 {
        from_archived!(self.nanos) / NANOS_PER_MILLI
    }

    /// Returns the fractional part of this `ArchivedDuration`, in whole microseconds.
    ///
    /// This method does **not** return the length of the duration when represented by microseconds.
    /// The returned number always represents a fractional portion of a second (i.e., it is less
    /// than one million).
    #[inline]
    pub const fn subsec_micros(&self) -> u32 {
        from_archived!(self.nanos) / NANOS_PER_MICRO
    }

    /// Returns the fractional part of this `Duration`, in nanoseconds.
    ///
    /// This method does **not** return the length of the duration when represented by nanoseconds.
    /// The returned number always represents a fractional portion of a second (i.e., it is less
    /// than one billion).
    #[inline]
    pub const fn subsec_nanos(&self) -> u32 {
        from_archived!(self.nanos)
    }

    /// Returns the total number of whole milliseconds contained by this `ArchivedDuration`.
    #[inline]
    pub const fn as_millis(&self) -> u128 {
        self.as_secs() as u128 * MILLIS_PER_SEC as u128
            + (self.subsec_nanos() / NANOS_PER_MILLI) as u128
    }

    /// Returns the total number of whole microseconds contained by this `ArchivedDuration`.
    #[inline]
    pub const fn as_micros(&self) -> u128 {
        self.as_secs() as u128 * MICROS_PER_SEC as u128
            + (self.subsec_nanos() / NANOS_PER_MICRO) as u128
    }

    /// Returns the total number of nanoseconds contained by this `ArchivedDuration`.
    #[inline]
    pub const fn as_nanos(&self) -> u128 {
        self.as_secs() as u128 * NANOS_PER_SEC as u128 + self.subsec_nanos() as u128
    }

    /// Returns the number of seconds contained by this `ArchivedDuration` as `f64`.
    ///
    /// The returned value does include the fractional (nanosecond) part of the duration.
    #[inline]
    pub fn as_secs_f64(&self) -> f64 {
        (self.as_secs() as f64) + (self.subsec_nanos() as f64) / (NANOS_PER_SEC as f64)
    }

    /// Returns the number of seconds contained by this `ArchivedDuration` as `f32`.
    ///
    /// The returned value does include the fractional (nanosecond) part of the duration.
    #[inline]
    pub fn as_secs_f32(&self) -> f32 {
        (self.as_secs() as f32) + (self.subsec_nanos() as f32) / (NANOS_PER_SEC as f32)
    }

    /// Constructs an archived duration at the given position.
    ///
    /// # Safety
    ///
    /// `out` must point to memory suitable for holding an `ArchivedDuration`.
    #[inline]
    pub unsafe fn emplace(secs: u64, nanos: u32, out: *mut ArchivedDuration) {
        use core::ptr::addr_of_mut;

        addr_of_mut!((*out).secs).write(to_archived!(secs));
        addr_of_mut!((*out).nanos).write(to_archived!(nanos));
    }
}

#[cfg(feature = "validation")]
const _: () = {
    use crate::Fallible;
    use bytecheck::CheckBytes;
    use core::fmt;

    /// An error resulting from an invalid duration.
    ///
    /// Durations must have a `secs` and `nanos` that when combined do not overflow a `u64`.
    #[derive(Debug)]
    pub struct DurationError;

    impl fmt::Display for DurationError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Duration error: nanos field is greater than 1 billion and overflows the seconds counter")
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for DurationError {}

    impl<C: Fallible + ?Sized> CheckBytes<C> for ArchivedDuration {
        type Error = DurationError;

        #[inline]
        unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
            // The fields of `ArchivedDuration` are always valid
            let duration = &*value;
            let secs = from_archived!(duration.secs);

            if secs
                .checked_add((duration.nanos / 1_000_000_000) as u64)
                .is_none()
            {
                Err(DurationError)
            } else {
                Ok(duration)
            }
        }
    }
};
