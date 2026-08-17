//! Module with traits for parsing a formatted string into a number.
//!
//! # Examples
//! ```
//! use num_format::Locale;
//! use num_format::parsing::ParseFormatted;
//!
//! fn main() {
//!     let s = "1,000,000";
//!     let n = s.parse_formatted::<_, u32>(&Locale::en).unwrap();
//!     assert_eq!(n, 1_000_000);
//! }
//! ```

use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
use core::str;

use crate::constants::*;
use crate::error::Error;
use crate::format::Format;
use crate::sealed::Sealed;

/// Trait that provides string-like types with a [`parse_formatted`]
/// method, allowing conversion from a formatted string into a number.
///
/// # Examples
/// ```
/// use num_format::Locale;
/// use num_format::parsing::ParseFormatted;
///
/// fn main() {
///     let s = "1,000,000";
///     let n = s.parse_formatted::<_, u32>(&Locale::en).unwrap();
///     assert_eq!(n, 1_000_000);
/// }
/// ```
///
/// [`parse_formatted`]: trait.ParseFormatted.html#method.parse_formatted
pub trait ParseFormatted {
    /// Converts `self` (typically a formatted string) into a number (see [Examples] above).
    ///
    /// [Examples]: trait.ParseFormatted.html#examples
    fn parse_formatted<F, N>(&self, format: &F) -> Result<N, Error>
    where
        F: Format,
        N: FromFormattedStr;
}

impl<S> ParseFormatted for S
where
    S: AsRef<str>,
{
    fn parse_formatted<F, N>(&self, format: &F) -> Result<N, Error>
    where
        F: Format,
        N: FromFormattedStr,
    {
        FromFormattedStr::from_formatted_str(self.as_ref(), format)
    }
}

/// Marker trait for number types (e.g. `u32`) that string-like types can be parsed
/// into via the [`ParseFormatted`] trait.
///
/// This trait is sealed; so you may not implement it on your own types.
///
/// [`ParseFormatted`]: trait.ParseFormatted.html
pub trait FromFormattedStr: Sealed + Sized {
    #[allow(missing_docs)]
    fn from_formatted_str<F>(s: &str, format: &F) -> Result<Self, Error>
    where
        F: Format;
}

macro_rules! impl_from_formatted_str {
    ($type:ty, $max_len:expr) => {
        impl FromFormattedStr for $type {
            fn from_formatted_str<F>(s: &str, format: &F) -> Result<Self, Error>
            where
                F: Format,
            {
                const BUF_LEN: usize = $max_len;
                let mut buf: [u8; BUF_LEN] = [0; BUF_LEN];

                let minus_sign = format.minus_sign().into_str();
                let is_negative = s.starts_with(minus_sign);

                let mut index = 0;
                if is_negative {
                    buf[index] = '-' as u8;
                    index += 1;
                }
                for c in s.chars() {
                    if c.is_numeric() {
                        if index > BUF_LEN {
                            return Err(Error::parse_number(&s));
                        }
                        buf[index] = c as u8;
                        index += 1;
                    }
                }

                if index == 0 {
                    return Err(Error::parse_number(&s));
                }

                let s2 = unsafe { str::from_utf8_unchecked(&buf[..index]) };
                let n = s2.parse::<$type>().map_err(|_| Error::parse_locale(&s))?;

                Ok(n)
            }
        }
    };
}

impl_from_formatted_str!(u8, U8_MAX_LEN);
impl_from_formatted_str!(u16, U16_MAX_LEN);
impl_from_formatted_str!(u32, U32_MAX_LEN);
impl_from_formatted_str!(usize, USIZE_MAX_LEN);
impl_from_formatted_str!(u64, U64_MAX_LEN);
impl_from_formatted_str!(u128, U128_MAX_LEN);

impl_from_formatted_str!(i8, I8_MAX_LEN);
impl_from_formatted_str!(i16, I16_MAX_LEN);
impl_from_formatted_str!(i32, I32_MAX_LEN);
impl_from_formatted_str!(isize, ISIZE_MAX_LEN);
impl_from_formatted_str!(i64, I64_MAX_LEN);
impl_from_formatted_str!(i128, I128_MAX_LEN);

macro_rules! impl_from_formatted_str_non_zero {
    ($type:ty, $related_type:ty, $max_len:expr) => {
        impl FromFormattedStr for $type {
            fn from_formatted_str<F>(s: &str, format: &F) -> Result<Self, Error>
            where
                F: Format,
            {
                let n = s.parse_formatted::<_, $related_type>(format)?;
                let n = Self::new(n).ok_or_else(|| Error::parse_number(s))?;
                Ok(n)
            }
        }
    };
}

impl_from_formatted_str_non_zero!(NonZeroU8, u8, U8_MAX_LEN);
impl_from_formatted_str_non_zero!(NonZeroU16, u16, U16_MAX_LEN);
impl_from_formatted_str_non_zero!(NonZeroU32, u32, U32_MAX_LEN);
impl_from_formatted_str_non_zero!(NonZeroUsize, usize, USIZE_MAX_LEN);
impl_from_formatted_str_non_zero!(NonZeroU64, u64, U64_MAX_LEN);
impl_from_formatted_str_non_zero!(NonZeroU128, u128, U128_MAX_LEN);

#[cfg(feature = "with-num-bigint")]
mod num {
    use num_bigint::{BigInt, BigUint};

    use super::*;

    macro_rules! impl_from_formatted_str_num_bigint {
        ($type:ty) => {
            impl FromFormattedStr for $type {
                fn from_formatted_str<F>(s: &str, format: &F) -> Result<Self, Error>
                where
                    F: Format,
                {
                    let mut buf = Vec::new();

                    let minus_sign = format.minus_sign().into_str();
                    let is_negative = s.starts_with(minus_sign);

                    if is_negative {
                        buf.push('-' as u8);
                    }
                    for c in s.chars() {
                        if c.is_numeric() {
                            buf.push(c as u8);
                        }
                    }

                    if buf.is_empty() {
                        return Err(Error::parse_number(&s));
                    }

                    let s2 = unsafe { str::from_utf8_unchecked(&buf[..]) };
                    let n = s2.parse::<$type>().map_err(|_| Error::parse_locale(&s))?;

                    Ok(n)
                }
            }
        };
    }

    impl_from_formatted_str_num_bigint!(BigInt);
    impl_from_formatted_str_num_bigint!(BigUint);

    #[cfg(test)]
    mod tests {
        use num_bigint::{ToBigInt, ToBigUint};

        use super::*;
        use crate::locale::Locale;

        #[test]
        fn test_parsing_num_bigint() {
            assert_eq!(
                "1,000,000"
                    .parse_formatted::<_, BigUint>(&Locale::en)
                    .unwrap(),
                1_000_000.to_biguint().unwrap()
            );
            assert_eq!(
                "-1,000,000"
                    .parse_formatted::<_, BigInt>(&Locale::en)
                    .unwrap(),
                (-1_000_000).to_bigint().unwrap()
            );
        }
    }
}
