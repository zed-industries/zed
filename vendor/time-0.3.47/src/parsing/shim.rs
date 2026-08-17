//! Extension traits for things either not implemented or not yet stable in the MSRV.

/// Marker trait for integer types.
pub(crate) trait Integer: Sized {
    /// The maximum number of digits that this type can have.
    const MAX_NUM_DIGITS: u8;
    /// The zero value for this type.
    const ZERO: Self;

    /// Push a digit onto the end of this integer, assuming no overflow.
    ///
    /// This is equivalent to `self * 10 + digit`.
    fn push_digit(self, digit: u8) -> Self;

    /// Push a digit onto the end of this integer, returning `None` on overflow.
    ///
    /// This is equivalent to `self.checked_mul(10)?.checked_add(digit)`.
    fn checked_push_digit(self, digit: u8) -> Option<Self>;
}

/// Parse the given types from bytes.
macro_rules! impl_parse_bytes {
    ($($t:ty)*) => ($(
        impl Integer for $t {
            const MAX_NUM_DIGITS: u8 = match Self::MAX.checked_ilog10() {
                Some(digits) => digits as u8 + 1,
                None => 1,
            };

            const ZERO: Self = 0;

            #[allow(trivial_numeric_casts, reason = "macro-generated code")]
            #[inline]
            fn push_digit(self, digit: u8) -> Self {
                self * 10 + digit as Self
            }

            #[allow(trivial_numeric_casts, reason = "macro-generated code")]
            #[inline]
            fn checked_push_digit(self, digit: u8) -> Option<Self> {
                self.checked_mul(10)?.checked_add(digit as Self)
            }
        }
    )*)
}
impl_parse_bytes! { u8 u16 u32 u128 }
