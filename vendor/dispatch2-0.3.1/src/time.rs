use core::time::Duration;

#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

/// An abstract representation of time.
#[doc(alias = "dispatch_time_t")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct DispatchTime(pub u64);

#[cfg(feature = "objc2")]
// SAFETY: `DispatchTime` is `#[repr(transparent)]`.
unsafe impl Encode for DispatchTime {
    const ENCODING: Encoding = u64::ENCODING;
}

#[cfg(feature = "objc2")]
// SAFETY: Same as above.
unsafe impl RefEncode for DispatchTime {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl DispatchTime {
    /// The current time.
    #[doc(alias = "DISPATCH_TIME_NOW")]
    pub const NOW: Self = Self(0);

    /// A time in the distant future.
    #[doc(alias = "DISPATCH_TIME_FOREVER")]
    // TODO: Swift calls this `distantFuture`
    pub const FOREVER: Self = Self(u64::MAX);

    /// TODO.
    #[doc(alias = "DISPATCH_WALLTIME_NOW")]
    pub const WALLTIME_NOW: Self = Self(!1);
}

impl TryFrom<Duration> for DispatchTime {
    type Error = ();

    #[inline]
    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        let secs = value.as_secs() as i64;

        let delta = secs
            .checked_mul(1_000_000_000)
            .and_then(|x| x.checked_add(i64::from(value.subsec_nanos())))
            .ok_or(())?;
        // delta cannot overflow
        Ok(Self::NOW.time(delta))
    }
}

// TODO: Expand this with inspiration from Time.swift in:
// https://github.com/swiftlang/swift-corelibs-libdispatch/blob/swift-6.1-RELEASE/src/swift/Time.swift
