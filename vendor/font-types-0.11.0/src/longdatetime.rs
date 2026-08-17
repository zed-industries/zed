//! a datetime type

/// A simple datetime type.
///
/// This represented as a number of seconds since 12:00 midnight, January 1, 1904, UTC.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct LongDateTime(i64);

impl LongDateTime {
    /// Create with a number of seconds relative to 1904-01-01 00:00.
    pub const fn new(secs: i64) -> Self {
        Self(secs)
    }

    /// The number of seconds since 00:00 1904-01-01, UTC.
    ///
    /// This can be a negative number, which presumably represents a date prior
    /// to the reference date.
    pub const fn as_secs(&self) -> i64 {
        self.0
    }

    /// The representation of this datetime as a big-endian byte array.
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }
}

crate::newtype_scalar!(LongDateTime, [u8; 8]);
//TODO: maybe a 'chrono' feature for constructing these sanely?
