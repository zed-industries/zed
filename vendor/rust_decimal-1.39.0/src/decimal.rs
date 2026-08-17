use crate::constants::{
    MAX_I128_REPR, MAX_SCALE_U32, POWERS_10, SCALE_MASK, SCALE_SHIFT, SIGN_MASK, SIGN_SHIFT, U32_MASK, U8_MASK,
    UNSIGN_MASK,
};
use crate::ops;
use crate::Error;
use core::{
    cmp::{Ordering::Equal, *},
    fmt,
    hash::{Hash, Hasher},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

// Diesel configuration
#[cfg(feature = "diesel")]
use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::Numeric};

#[allow(unused_imports)] // It's not actually dead code below, but the compiler thinks it is.
#[cfg(not(feature = "std"))]
use num_traits::float::FloatCore;
use num_traits::{FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};

/// The smallest value that can be represented by this decimal type.
const MIN: Decimal = Decimal {
    flags: 2_147_483_648,
    lo: 4_294_967_295,
    mid: 4_294_967_295,
    hi: 4_294_967_295,
};

/// The largest value that can be represented by this decimal type.
const MAX: Decimal = Decimal {
    flags: 0,
    lo: 4_294_967_295,
    mid: 4_294_967_295,
    hi: 4_294_967_295,
};

const ZERO: Decimal = Decimal {
    flags: 0,
    lo: 0,
    mid: 0,
    hi: 0,
};
const ONE: Decimal = Decimal {
    flags: 0,
    lo: 1,
    mid: 0,
    hi: 0,
};
const TWO: Decimal = Decimal {
    flags: 0,
    lo: 2,
    mid: 0,
    hi: 0,
};
const TEN: Decimal = Decimal {
    flags: 0,
    lo: 10,
    mid: 0,
    hi: 0,
};
const ONE_HUNDRED: Decimal = Decimal {
    flags: 0,
    lo: 100,
    mid: 0,
    hi: 0,
};
const ONE_THOUSAND: Decimal = Decimal {
    flags: 0,
    lo: 1000,
    mid: 0,
    hi: 0,
};
const NEGATIVE_ONE: Decimal = Decimal {
    flags: 2147483648,
    lo: 1,
    mid: 0,
    hi: 0,
};

/// `UnpackedDecimal` contains unpacked representation of `Decimal` where each component
/// of decimal-format stored in it's own field
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnpackedDecimal {
    pub negative: bool,
    pub scale: u32,
    pub hi: u32,
    pub mid: u32,
    pub lo: u32,
}

/// `Decimal` represents a 128 bit representation of a fixed-precision decimal number.
/// The finite set of values of type `Decimal` are of the form m / 10<sup>e</sup>,
/// where m is an integer such that -2<sup>96</sup> < m < 2<sup>96</sup>, and e is an integer
/// between 0 and 28 inclusive.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, AsExpression), diesel(sql_type = Numeric))]
#[cfg_attr(feature = "c-repr", repr(C))]
#[cfg_attr(feature = "align16", repr(align(16)))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize, borsh::BorshSchema)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, Deserialize, Serialize),
    archive(compare(PartialEq)),
    archive_attr(derive(Clone, Copy, Debug))
)]
#[cfg_attr(feature = "rkyv-safe", archive(check_bytes))]
pub struct Decimal {
    // Bits 0-15: unused
    // Bits 16-23: Contains "e", a value between 0-28 that indicates the scale
    // Bits 24-30: unused
    // Bit 31: the sign of the Decimal value, 0 meaning positive and 1 meaning negative.
    flags: u32,
    // The lo, mid, hi, and flags fields contain the representation of the
    // Decimal value as a 96-bit integer.
    hi: u32,
    lo: u32,
    mid: u32,
}

#[cfg(feature = "ndarray")]
impl ndarray::ScalarOperand for Decimal {}

/// `RoundingStrategy` represents the different rounding strategies that can be used by
/// `round_dp_with_strategy`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoundingStrategy {
    /// When a number is halfway between two others, it is rounded toward the nearest even number.
    /// Also known as "Bankers Rounding".
    /// e.g.
    /// 6.5 -> 6, 7.5 -> 8
    MidpointNearestEven,
    /// When a number is halfway between two others, it is rounded toward the nearest number that
    /// is away from zero. e.g. 6.4 -> 6, 6.5 -> 7, -6.5 -> -7
    MidpointAwayFromZero,
    /// When a number is halfway between two others, it is rounded toward the nearest number that
    /// is toward zero. e.g. 6.4 -> 6, 6.5 -> 6, -6.5 -> -6
    MidpointTowardZero,
    /// The number is always rounded toward zero. e.g. -6.8 -> -6, 6.8 -> 6
    ToZero,
    /// The number is always rounded away from zero. e.g. -6.8 -> -7, 6.8 -> 7
    AwayFromZero,
    /// The number is always rounded towards negative infinity. e.g. 6.8 -> 6, -6.8 -> -7
    ToNegativeInfinity,
    /// The number is always rounded towards positive infinity. e.g. 6.8 -> 7, -6.8 -> -6
    ToPositiveInfinity,

    /// When a number is halfway between two others, it is rounded toward the nearest even number.
    /// e.g.
    /// 6.5 -> 6, 7.5 -> 8
    #[deprecated(since = "1.11.0", note = "Please use RoundingStrategy::MidpointNearestEven instead")]
    BankersRounding,
    /// Rounds up if the value >= 5, otherwise rounds down, e.g. 6.5 -> 7
    #[deprecated(since = "1.11.0", note = "Please use RoundingStrategy::MidpointAwayFromZero instead")]
    RoundHalfUp,
    /// Rounds down if the value =< 5, otherwise rounds up, e.g. 6.5 -> 6, 6.51 -> 7 1.4999999 -> 1
    #[deprecated(since = "1.11.0", note = "Please use RoundingStrategy::MidpointTowardZero instead")]
    RoundHalfDown,
    /// Always round down.
    #[deprecated(since = "1.11.0", note = "Please use RoundingStrategy::ToZero instead")]
    RoundDown,
    /// Always round up.
    #[deprecated(since = "1.11.0", note = "Please use RoundingStrategy::AwayFromZero instead")]
    RoundUp,
}

#[allow(dead_code)]
impl Decimal {
    /// The smallest value that can be represented by this decimal type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::MIN, dec!(-79_228_162_514_264_337_593_543_950_335));
    /// ```
    pub const MIN: Decimal = MIN;
    /// The largest value that can be represented by this decimal type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::MAX, dec!(79_228_162_514_264_337_593_543_950_335));
    /// ```
    pub const MAX: Decimal = MAX;
    /// A constant representing 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::ZERO, dec!(0));
    /// ```
    pub const ZERO: Decimal = ZERO;
    /// A constant representing 1.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::ONE, dec!(1));
    /// ```
    pub const ONE: Decimal = ONE;
    /// A constant representing -1.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::NEGATIVE_ONE, dec!(-1));
    /// ```
    pub const NEGATIVE_ONE: Decimal = NEGATIVE_ONE;
    /// A constant representing 2.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::TWO, dec!(2));
    /// ```
    pub const TWO: Decimal = TWO;
    /// A constant representing 10.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::TEN, dec!(10));
    /// ```
    pub const TEN: Decimal = TEN;
    /// A constant representing 100.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::ONE_HUNDRED, dec!(100));
    /// ```
    pub const ONE_HUNDRED: Decimal = ONE_HUNDRED;
    /// A constant representing 1000.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::ONE_THOUSAND, dec!(1000));
    /// ```
    pub const ONE_THOUSAND: Decimal = ONE_THOUSAND;
    /// The maximum supported scale value.
    ///
    /// Some operations, such as [`Self::rescale`] may accept larger scale values, but  these
    /// operations will result in a final value with a scale no larger than this.
    ///
    /// Note that the maximum scale is _not_ the same as the maximum possible numeric precision in
    /// base-10.
    pub const MAX_SCALE: u32 = MAX_SCALE_U32;

    /// A constant representing π as 3.1415926535897932384626433833
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::PI, dec!(3.1415926535897932384626433833));
    /// ```
    #[cfg(feature = "maths")]
    pub const PI: Decimal = Decimal {
        flags: 1835008,
        lo: 1102470953,
        mid: 185874565,
        hi: 1703060790,
    };
    /// A constant representing π/2 as 1.5707963267948966192313216916
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::HALF_PI, dec!(1.5707963267948966192313216916));
    /// ```
    #[cfg(feature = "maths")]
    pub const HALF_PI: Decimal = Decimal {
        flags: 1835008,
        lo: 2698719124,
        mid: 92937282,
        hi: 851530395,
    };
    /// A constant representing π/4 as 0.7853981633974483096156608458
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::QUARTER_PI, dec!(0.7853981633974483096156608458));
    /// ```
    #[cfg(feature = "maths")]
    pub const QUARTER_PI: Decimal = Decimal {
        flags: 1835008,
        lo: 1349359562,
        mid: 2193952289,
        hi: 425765197,
    };
    /// A constant representing 2π as 6.2831853071795864769252867666
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::TWO_PI, dec!(6.2831853071795864769252867666));
    /// ```
    #[cfg(feature = "maths")]
    pub const TWO_PI: Decimal = Decimal {
        flags: 1835008,
        lo: 2204941906,
        mid: 371749130,
        hi: 3406121580,
    };
    /// A constant representing Euler's number (e) as 2.7182818284590452353602874714
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::E, dec!(2.7182818284590452353602874714));
    /// ```
    #[cfg(feature = "maths")]
    pub const E: Decimal = Decimal {
        flags: 1835008,
        lo: 2239425882,
        mid: 3958169141,
        hi: 1473583531,
    };
    /// A constant representing the inverse of Euler's number (1/e) as 0.3678794411714423215955237702
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// assert_eq!(Decimal::E_INVERSE, dec!(0.3678794411714423215955237702));
    /// ```
    #[cfg(feature = "maths")]
    pub const E_INVERSE: Decimal = Decimal {
        flags: 1835008,
        lo: 2384059206,
        mid: 2857938002,
        hi: 199427844,
    };

    /// Returns a `Decimal` with a 64 bit `m` representation and corresponding `e` scale.
    ///
    /// # Arguments
    ///
    /// * `num` - An i64 that represents the `m` portion of the decimal number
    /// * `scale` - A u32 representing the `e` portion of the decimal number.
    ///
    /// # Panics
    ///
    /// This function panics if `scale` is > [`Self::MAX_SCALE`].
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let pi = Decimal::new(3141, 3);
    /// assert_eq!(pi.to_string(), "3.141");
    /// ```
    #[must_use]
    pub fn new(num: i64, scale: u32) -> Decimal {
        match Self::try_new(num, scale) {
            Err(e) => panic!("{}", e),
            Ok(d) => d,
        }
    }

    /// Checked version of [`Self::new`]. Will return an error instead of panicking at run-time.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rust_decimal::Decimal;
    /// #
    /// let max = Decimal::try_new(i64::MAX, u32::MAX);
    /// assert!(max.is_err());
    /// ```
    pub const fn try_new(num: i64, scale: u32) -> crate::Result<Decimal> {
        if scale > Self::MAX_SCALE {
            return Err(Error::ScaleExceedsMaximumPrecision(scale));
        }
        let flags: u32 = scale << SCALE_SHIFT;
        if num < 0 {
            let pos_num = num.wrapping_neg() as u64;
            return Ok(Decimal {
                flags: flags | SIGN_MASK,
                hi: 0,
                lo: (pos_num & U32_MASK) as u32,
                mid: ((pos_num >> 32) & U32_MASK) as u32,
            });
        }
        Ok(Decimal {
            flags,
            hi: 0,
            lo: (num as u64 & U32_MASK) as u32,
            mid: ((num as u64 >> 32) & U32_MASK) as u32,
        })
    }

    /// Creates a `Decimal` using a 128 bit signed `m` representation and corresponding `e` scale.
    ///
    /// # Arguments
    ///
    /// * `num` - An i128 that represents the `m` portion of the decimal number
    /// * `scale` - A u32 representing the `e` portion of the decimal number.
    ///
    /// # Panics
    ///
    /// This function panics if `scale` is > [`Self::MAX_SCALE`] or if `num` exceeds the maximum
    /// supported 96 bits.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rust_decimal::Decimal;
    /// #
    /// let pi = Decimal::from_i128_with_scale(3141i128, 3);
    /// assert_eq!(pi.to_string(), "3.141");
    /// ```
    #[must_use]
    pub fn from_i128_with_scale(num: i128, scale: u32) -> Decimal {
        match Self::try_from_i128_with_scale(num, scale) {
            Ok(d) => d,
            Err(e) => panic!("{}", e),
        }
    }

    /// Checked version of `Decimal::from_i128_with_scale`. Will return `Err` instead
    /// of panicking at run-time.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rust_decimal::Decimal;
    /// #
    /// let max = Decimal::try_from_i128_with_scale(i128::MAX, u32::MAX);
    /// assert!(max.is_err());
    /// ```
    pub const fn try_from_i128_with_scale(num: i128, scale: u32) -> crate::Result<Decimal> {
        if scale > Self::MAX_SCALE {
            Err(Error::ScaleExceedsMaximumPrecision(scale))
        } else if num > MAX_I128_REPR {
            Err(Error::ExceedsMaximumPossibleValue)
        } else if num < -MAX_I128_REPR {
            Err(Error::LessThanMinimumPossibleValue)
        } else {
            Ok(Self::from_i128_with_scale_unchecked(num, scale))
        }
    }

    #[inline]
    pub(crate) const fn from_i128_with_scale_unchecked(num: i128, scale: u32) -> Decimal {
        let flags = flags(num < 0, scale);
        let num = num.unsigned_abs();
        Decimal {
            flags,
            lo: (num as u64 & U32_MASK) as u32,
            mid: ((num as u64 >> 32) & U32_MASK) as u32,
            hi: ((num >> 64) as u64 & U32_MASK) as u32,
        }
    }

    /// Returns a `Decimal` using the instances constituent parts.
    ///
    /// # Arguments
    ///
    /// * `lo` - The low 32 bits of a 96-bit integer.
    /// * `mid` - The middle 32 bits of a 96-bit integer.
    /// * `hi` - The high 32 bits of a 96-bit integer.
    /// * `negative` - `true` to indicate a negative number.
    /// * `scale` - A power of 10 ranging from 0 to [`Self::MAX_SCALE`].
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let pi = Decimal::from_parts(1102470952, 185874565, 1703060790, false, 28);
    /// assert_eq!(pi.to_string(), "3.1415926535897932384626433832");
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_parts(lo: u32, mid: u32, hi: u32, negative: bool, scale: u32) -> Decimal {
        assert!(scale <= Self::MAX_SCALE, "Scale exceeds maximum supported scale");
        Decimal {
            lo,
            mid,
            hi,
            flags: flags(
                if lo == 0 && mid == 0 && hi == 0 {
                    false
                } else {
                    negative
                },
                scale,
            ),
        }
    }

    #[must_use]
    pub(crate) const fn from_parts_raw(lo: u32, mid: u32, hi: u32, flags: u32) -> Decimal {
        if lo == 0 && mid == 0 && hi == 0 {
            Decimal {
                lo,
                mid,
                hi,
                flags: flags & SCALE_MASK,
            }
        } else {
            Decimal { flags, hi, lo, mid }
        }
    }

    /// Returns a `Result` which if successful contains the `Decimal` constitution of
    /// the scientific notation provided by `value`.
    ///
    /// # Arguments
    ///
    /// * `value` - The scientific notation of the `Decimal`.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// # fn main() -> Result<(), rust_decimal::Error> {
    /// let value = Decimal::from_scientific("9.7e-7")?;
    /// assert_eq!(value.to_string(), "0.00000097");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn from_scientific(value: &str) -> Result<Decimal, Error> {
        const ERROR_MESSAGE: &str = "Failed to parse";

        let mut split = value.splitn(2, ['e', 'E']);

        let base = split.next().ok_or_else(|| Error::from(ERROR_MESSAGE))?;
        let exp = split.next().ok_or_else(|| Error::from(ERROR_MESSAGE))?;

        let mut ret = Decimal::from_str(base)?;
        let current_scale = ret.scale();

        if let Some(stripped) = exp.strip_prefix('-') {
            let exp: u32 = stripped.parse().map_err(|_| Error::from(ERROR_MESSAGE))?;
            if exp > Self::MAX_SCALE {
                return Err(Error::ScaleExceedsMaximumPrecision(exp));
            }
            ret.set_scale(current_scale + exp)?;
        } else {
            let exp: u32 = exp.parse().map_err(|_| Error::from(ERROR_MESSAGE))?;
            if exp <= current_scale {
                ret.set_scale(current_scale - exp)?;
            } else if exp > 0 {
                use crate::constants::BIG_POWERS_10;

                // This is a case whereby the mantissa needs to be larger to be correctly
                // represented within the decimal type. A good example is 1.2E10. At this point,
                // we've parsed 1.2 as the base and 10 as the exponent. To represent this within a
                // Decimal type we effectively store the mantissa as 12,000,000,000 and scale as
                // zero.
                if exp > Self::MAX_SCALE {
                    return Err(Error::ScaleExceedsMaximumPrecision(exp));
                }
                let mut exp = exp as usize;
                // Max two iterations. If exp is 1 then it needs to index position 0 of the array.
                while exp > 0 {
                    let pow;
                    if exp >= BIG_POWERS_10.len() {
                        pow = BIG_POWERS_10[BIG_POWERS_10.len() - 1];
                        exp -= BIG_POWERS_10.len();
                    } else {
                        pow = BIG_POWERS_10[exp - 1];
                        exp = 0;
                    }

                    let pow = Decimal {
                        flags: 0,
                        lo: pow as u32,
                        mid: (pow >> 32) as u32,
                        hi: 0,
                    };
                    match ret.checked_mul(pow) {
                        Some(r) => ret = r,
                        None => return Err(Error::ExceedsMaximumPossibleValue),
                    };
                }
                ret.normalize_assign();
            }
        }
        Ok(ret)
    }

    /// Returns a `Result` which if successful contains the `Decimal` constitution of
    /// the scientific notation provided by `value`. If the exponent is negative and
    /// the given base and exponent would exceed [Decimal::MAX_SCALE] then this
    /// functions attempts to round the base to fit.
    ///
    /// # Arguments
    ///
    /// * `value` - The scientific notation of the `Decimal`.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal::Error;
    /// #
    /// # fn main() -> Result<(), rust_decimal::Error> {
    /// let value = Decimal::from_scientific_lossy("2.710505431213761e-20")?;
    /// assert_eq!(value.to_string(), "0.0000000000000000000271050543");
    ///
    /// let value = Decimal::from_scientific_lossy("2.5e-28")?;
    /// assert_eq!(value.to_string(), "0.0000000000000000000000000003");
    ///
    /// let value = Decimal::from_scientific_lossy("-2.5e-28")?;
    /// assert_eq!(value.to_string(), "-0.0000000000000000000000000003");
    ///
    /// let err = Decimal::from_scientific_lossy("2e-29").unwrap_err();
    /// assert_eq!(err, Error::ScaleExceedsMaximumPrecision(29));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn from_scientific_lossy(value: &str) -> Result<Decimal, Error> {
        const ERROR_MESSAGE: &str = "Failed to parse";

        let mut split = value.splitn(2, ['e', 'E']);

        let base = split.next().ok_or_else(|| Error::from(ERROR_MESSAGE))?;
        let exp = split.next().ok_or_else(|| Error::from(ERROR_MESSAGE))?;

        let mut ret = Decimal::from_str(base)?;
        let current_scale = ret.scale();

        if let Some(stripped) = exp.strip_prefix('-') {
            let exp: u32 = stripped.parse().map_err(|_| Error::from(ERROR_MESSAGE))?;
            if exp > Self::MAX_SCALE {
                return Err(Error::ScaleExceedsMaximumPrecision(exp));
            }
            if current_scale + exp > Self::MAX_SCALE {
                ret.rescale(Self::MAX_SCALE - exp);
                ret.set_scale(Self::MAX_SCALE)?;
            } else {
                ret.set_scale(current_scale + exp)?;
            }
        } else {
            let exp: u32 = exp.parse().map_err(|_| Error::from(ERROR_MESSAGE))?;
            if exp <= current_scale {
                ret.set_scale(current_scale - exp)?;
            } else if exp > 0 {
                use crate::constants::BIG_POWERS_10;

                // This is a case whereby the mantissa needs to be larger to be correctly
                // represented within the decimal type. A good example is 1.2E10. At this point,
                // we've parsed 1.2 as the base and 10 as the exponent. To represent this within a
                // Decimal type we effectively store the mantissa as 12,000,000,000 and scale as
                // zero.
                if exp > Self::MAX_SCALE {
                    return Err(Error::ScaleExceedsMaximumPrecision(exp));
                }
                let mut exp = exp as usize;
                // Max two iterations. If exp is 1 then it needs to index position 0 of the array.
                while exp > 0 {
                    let pow;
                    if exp >= BIG_POWERS_10.len() {
                        pow = BIG_POWERS_10[BIG_POWERS_10.len() - 1];
                        exp -= BIG_POWERS_10.len();
                    } else {
                        pow = BIG_POWERS_10[exp - 1];
                        exp = 0;
                    }

                    let pow = Decimal {
                        flags: 0,
                        lo: pow as u32,
                        mid: (pow >> 32) as u32,
                        hi: 0,
                    };
                    match ret.checked_mul(pow) {
                        Some(r) => ret = r,
                        None => return Err(Error::ExceedsMaximumPossibleValue),
                    };
                }
                ret.normalize_assign();
            }
        }
        Ok(ret)
    }

    /// Converts a string slice in a given base to a decimal.
    ///
    /// The string is expected to be an optional + sign followed by digits.
    /// Digits are a subset of these characters, depending on radix, and will return an error if outside
    /// the expected range:
    ///
    /// * 0-9
    /// * a-z
    /// * A-Z
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// #
    /// # fn main() -> Result<(), rust_decimal::Error> {
    /// assert_eq!(Decimal::from_str_radix("A", 16)?.to_string(), "10");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn from_str_radix(str: &str, radix: u32) -> Result<Self, crate::Error> {
        if radix == 10 {
            crate::str::parse_str_radix_10(str)
        } else {
            crate::str::parse_str_radix_n(str, radix)
        }
    }

    /// Parses a string slice into a decimal. If the value underflows and cannot be represented with the
    /// given scale then this will return an error.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// # use rust_decimal::Error;
    /// #
    /// # fn main() -> Result<(), rust_decimal::Error> {
    /// assert_eq!(Decimal::from_str_exact("0.001")?.to_string(), "0.001");
    /// assert_eq!(Decimal::from_str_exact("0.00000_00000_00000_00000_00000_001")?.to_string(), "0.0000000000000000000000000001");
    /// assert_eq!(Decimal::from_str_exact("0.00000_00000_00000_00000_00000_0001"), Err(Error::Underflow));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn from_str_exact(str: &str) -> Result<Self, crate::Error> {
        crate::str::parse_str_radix_10_exact(str)
    }

    /// Returns the scale of the decimal number, otherwise known as `e`.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let num = Decimal::new(1234, 3);
    /// assert_eq!(num.scale(), 3u32);
    /// ```
    #[inline]
    #[must_use]
    pub const fn scale(&self) -> u32 {
        (self.flags & SCALE_MASK) >> SCALE_SHIFT
    }

    /// Returns the mantissa of the decimal number.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// # use rust_decimal_macros::dec;
    ///
    /// let num = dec!(-1.2345678);
    /// assert_eq!(num.mantissa(), -12345678i128);
    /// assert_eq!(num.scale(), 7);
    /// ```
    #[must_use]
    pub const fn mantissa(&self) -> i128 {
        let raw = (self.lo as i128) | ((self.mid as i128) << 32) | ((self.hi as i128) << 64);
        if self.is_sign_negative() {
            -raw
        } else {
            raw
        }
    }

    /// Returns true if this Decimal number is equivalent to zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// #
    /// let num = Decimal::ZERO;
    /// assert!(num.is_zero());
    /// ```
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.lo | self.mid | self.hi == 0
    }

    /// Returns true if this Decimal number has zero fractional part (is equal to an integer)
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// # use rust_decimal_macros::dec;
    /// #
    /// assert_eq!(dec!(5).is_integer(), true);
    /// // Trailing zeros are also ignored
    /// assert_eq!(dec!(5.0000).is_integer(), true);
    /// // If there is a fractional part then it is not an integer
    /// assert_eq!(dec!(5.1).is_integer(), false);
    /// ```
    #[must_use]
    pub fn is_integer(&self) -> bool {
        let scale = self.scale();
        if scale == 0 || self.is_zero() {
            return true;
        }

        // Check if it can be divided by 10^scale without remainder
        let mut bits = self.mantissa_array3();
        let mut scale = scale;
        while scale > 0 {
            let remainder = if scale > 9 {
                scale -= 9;
                ops::array::div_by_u32(&mut bits, POWERS_10[9])
            } else {
                let power = POWERS_10[scale as usize];
                scale = 0;
                ops::array::div_by_u32(&mut bits, power)
            };
            if remainder > 0 {
                return false;
            }
        }
        true
    }

    /// An optimized method for changing the sign of a decimal number.
    ///
    /// # Arguments
    ///
    /// * `positive`: true if the resulting decimal should be positive.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let mut one = Decimal::ONE;
    /// one.set_sign(false);
    /// assert_eq!(one.to_string(), "-1");
    /// ```
    #[deprecated(since = "1.4.0", note = "please use `set_sign_positive` instead")]
    pub fn set_sign(&mut self, positive: bool) {
        self.set_sign_positive(positive);
    }

    /// An optimized method for changing the sign of a decimal number.
    ///
    /// # Arguments
    ///
    /// * `positive`: true if the resulting decimal should be positive.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let mut one = Decimal::ONE;
    /// one.set_sign_positive(false);
    /// assert_eq!(one.to_string(), "-1");
    /// ```
    #[inline(always)]
    pub fn set_sign_positive(&mut self, positive: bool) {
        if positive {
            self.flags &= UNSIGN_MASK;
        } else {
            self.flags |= SIGN_MASK;
        }
    }

    /// An optimized method for changing the sign of a decimal number.
    ///
    /// # Arguments
    ///
    /// * `negative`: true if the resulting decimal should be negative.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let mut one = Decimal::ONE;
    /// one.set_sign_negative(true);
    /// assert_eq!(one.to_string(), "-1");
    /// ```
    #[inline(always)]
    pub fn set_sign_negative(&mut self, negative: bool) {
        self.set_sign_positive(!negative);
    }

    /// An optimized method for changing the scale of a decimal number.
    ///
    /// # Arguments
    ///
    /// * `scale`: the new scale of the number
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// # fn main() -> Result<(), rust_decimal::Error> {
    /// let mut one = Decimal::ONE;
    /// one.set_scale(5)?;
    /// assert_eq!(one.to_string(), "0.00001");
    /// #    Ok(())
    /// # }
    /// ```
    pub fn set_scale(&mut self, scale: u32) -> Result<(), Error> {
        if scale > Self::MAX_SCALE {
            return Err(Error::ScaleExceedsMaximumPrecision(scale));
        }
        self.flags = (scale << SCALE_SHIFT) | (self.flags & SIGN_MASK);
        Ok(())
    }

    /// Modifies the `Decimal` towards the desired scale, attempting to do so without changing the
    /// underlying number itself.
    ///
    /// Setting the scale to something less then the current `Decimal`s scale will
    /// cause the newly created `Decimal` to perform rounding using the `MidpointAwayFromZero` strategy.
    ///
    /// Scales greater than the maximum precision that can be represented by `Decimal` will be
    /// automatically rounded to either [`Self::MAX_SCALE`] or the maximum precision that can
    /// be represented with the given mantissa.
    ///
    /// # Arguments
    /// * `scale`: The desired scale to use for the new `Decimal` number.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// # use rust_decimal_macros::dec;
    ///
    /// // Rescaling to a higher scale preserves the value
    /// let mut number = dec!(1.123);
    /// assert_eq!(number.scale(), 3);
    /// number.rescale(6);
    /// assert_eq!(number.to_string(), "1.123000");
    /// assert_eq!(number.scale(), 6);
    ///
    /// // Rescaling to a lower scale forces the number to be rounded
    /// let mut number = dec!(1.45);
    /// assert_eq!(number.scale(), 2);
    /// number.rescale(1);
    /// assert_eq!(number.to_string(), "1.5");
    /// assert_eq!(number.scale(), 1);
    ///
    /// // This function never fails. Consequently, if a scale is provided that is unable to be
    /// // represented using the given mantissa, then the maximum possible scale is used.
    /// let mut number = dec!(11.76470588235294);
    /// assert_eq!(number.scale(), 14);
    /// number.rescale(28);
    /// // A scale of 28 cannot be represented given this mantissa, however it was able to represent
    /// // a number with a scale of 27
    /// assert_eq!(number.to_string(), "11.764705882352940000000000000");
    /// assert_eq!(number.scale(), 27);
    /// ```
    pub fn rescale(&mut self, scale: u32) {
        let mut array = [self.lo, self.mid, self.hi];
        let mut value_scale = self.scale();
        ops::array::rescale_internal(&mut array, &mut value_scale, scale);
        self.lo = array[0];
        self.mid = array[1];
        self.hi = array[2];
        self.flags = flags(self.is_sign_negative(), value_scale);
    }

    /// Returns a serialized version of the decimal number.
    /// The resulting byte array will have the following representation:
    ///
    /// * Bytes 1-4: flags
    /// * Bytes 5-8: lo portion of `m`
    /// * Bytes 9-12: mid portion of `m`
    /// * Bytes 13-16: high portion of `m`
    #[must_use]
    pub const fn serialize(&self) -> [u8; 16] {
        [
            (self.flags & U8_MASK) as u8,
            ((self.flags >> 8) & U8_MASK) as u8,
            ((self.flags >> 16) & U8_MASK) as u8,
            ((self.flags >> 24) & U8_MASK) as u8,
            (self.lo & U8_MASK) as u8,
            ((self.lo >> 8) & U8_MASK) as u8,
            ((self.lo >> 16) & U8_MASK) as u8,
            ((self.lo >> 24) & U8_MASK) as u8,
            (self.mid & U8_MASK) as u8,
            ((self.mid >> 8) & U8_MASK) as u8,
            ((self.mid >> 16) & U8_MASK) as u8,
            ((self.mid >> 24) & U8_MASK) as u8,
            (self.hi & U8_MASK) as u8,
            ((self.hi >> 8) & U8_MASK) as u8,
            ((self.hi >> 16) & U8_MASK) as u8,
            ((self.hi >> 24) & U8_MASK) as u8,
        ]
    }

    /// Deserializes the given bytes into a decimal number.
    /// The deserialized byte representation must be 16 bytes and adhere to the following convention:
    ///
    /// * Bytes 1-4: flags
    /// * Bytes 5-8: lo portion of `m`
    /// * Bytes 9-12: mid portion of `m`
    /// * Bytes 13-16: high portion of `m`
    #[must_use]
    pub fn deserialize(bytes: [u8; 16]) -> Decimal {
        // We can bound flags by a bitwise mask to correspond to:
        //   Bits 0-15: unused
        //   Bits 16-23: Contains "e", a value between 0-28 that indicates the scale
        //   Bits 24-30: unused
        //   Bit 31: the sign of the Decimal value, 0 meaning positive and 1 meaning negative.
        let mut raw = Decimal {
            flags: ((bytes[0] as u32)
                | ((bytes[1] as u32) << 8)
                | ((bytes[2] as u32) << 16)
                | ((bytes[3] as u32) << 24))
                & 0x801F_0000,
            lo: (bytes[4] as u32) | ((bytes[5] as u32) << 8) | ((bytes[6] as u32) << 16) | ((bytes[7] as u32) << 24),
            mid: (bytes[8] as u32) | ((bytes[9] as u32) << 8) | ((bytes[10] as u32) << 16) | ((bytes[11] as u32) << 24),
            hi: (bytes[12] as u32)
                | ((bytes[13] as u32) << 8)
                | ((bytes[14] as u32) << 16)
                | ((bytes[15] as u32) << 24),
        };
        // Scale must be bound to maximum precision. Only two values can be greater than this
        if raw.scale() > Self::MAX_SCALE {
            let mut bits = raw.mantissa_array3();
            let remainder = match raw.scale() {
                29 => ops::array::div_by_power::<1>(&mut bits),
                30 => ops::array::div_by_power::<2>(&mut bits),
                31 => ops::array::div_by_power::<3>(&mut bits),
                _ => 0,
            };
            if remainder >= 5 {
                ops::array::add_one_internal(&mut bits);
            }
            raw.lo = bits[0];
            raw.mid = bits[1];
            raw.hi = bits[2];
            raw.flags = flags(raw.is_sign_negative(), Self::MAX_SCALE);
        }
        raw
    }

    /// Returns `true` if the decimal is negative.
    #[deprecated(since = "0.6.3", note = "please use `is_sign_negative` instead")]
    #[must_use]
    pub fn is_negative(&self) -> bool {
        self.is_sign_negative()
    }

    /// Returns `true` if the decimal is positive.
    #[deprecated(since = "0.6.3", note = "please use `is_sign_positive` instead")]
    #[must_use]
    pub fn is_positive(&self) -> bool {
        self.is_sign_positive()
    }

    /// Returns `true` if the sign bit of the decimal is negative.
    ///
    /// # Example
    /// ```
    /// # use rust_decimal::prelude::*;
    /// #
    /// assert_eq!(true, Decimal::new(-1, 0).is_sign_negative());
    /// assert_eq!(false, Decimal::new(1, 0).is_sign_negative());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_sign_negative(&self) -> bool {
        self.flags & SIGN_MASK > 0
    }

    /// Returns `true` if the sign bit of the decimal is positive.
    ///
    /// # Example
    /// ```
    /// # use rust_decimal::prelude::*;
    /// #
    /// assert_eq!(false, Decimal::new(-1, 0).is_sign_positive());
    /// assert_eq!(true, Decimal::new(1, 0).is_sign_positive());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_sign_positive(&self) -> bool {
        self.flags & SIGN_MASK == 0
    }

    /// Returns the minimum possible number that `Decimal` can represent.
    #[deprecated(since = "1.12.0", note = "Use the associated constant Decimal::MIN")]
    #[must_use]
    pub const fn min_value() -> Decimal {
        MIN
    }

    /// Returns the maximum possible number that `Decimal` can represent.
    #[deprecated(since = "1.12.0", note = "Use the associated constant Decimal::MAX")]
    #[must_use]
    pub const fn max_value() -> Decimal {
        MAX
    }

    /// Returns a new `Decimal` integral with no fractional portion.
    /// This is a true truncation whereby no rounding is performed.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// #
    /// let pi = dec!(3.141);
    /// assert_eq!(pi.trunc(), dec!(3));
    ///
    /// // Negative numbers are similarly truncated without rounding
    /// let neg = dec!(-1.98765);
    /// assert_eq!(neg.trunc(), Decimal::NEGATIVE_ONE);
    /// ```
    #[must_use]
    pub fn trunc(&self) -> Decimal {
        let mut working = [self.lo, self.mid, self.hi];
        let mut working_scale = self.scale();
        ops::array::truncate_internal(&mut working, &mut working_scale, 0);
        Decimal {
            lo: working[0],
            mid: working[1],
            hi: working[2],
            flags: flags(self.is_sign_negative(), working_scale),
        }
    }

    /// Returns a new `Decimal` with the fractional portion delimited by `scale`.
    /// This is a true truncation whereby no rounding is performed.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// #
    /// let pi = dec!(3.141592);
    /// assert_eq!(pi.trunc_with_scale(2), dec!(3.14));
    ///
    /// // Negative numbers are similarly truncated without rounding
    /// let neg = dec!(-1.98765);
    /// assert_eq!(neg.trunc_with_scale(1), dec!(-1.9));
    /// ```
    #[must_use]
    pub fn trunc_with_scale(&self, scale: u32) -> Decimal {
        let mut working = [self.lo, self.mid, self.hi];
        let mut working_scale = self.scale();
        ops::array::truncate_internal(&mut working, &mut working_scale, scale);
        Decimal {
            lo: working[0],
            mid: working[1],
            hi: working[2],
            flags: flags(self.is_sign_negative(), working_scale),
        }
    }

    /// Returns a new `Decimal` representing the fractional portion of the number.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let pi = Decimal::new(3141, 3);
    /// let fract = Decimal::new(141, 3);
    /// // note that it returns a decimal
    /// assert_eq!(pi.fract(), fract);
    /// ```
    #[must_use]
    pub fn fract(&self) -> Decimal {
        // This is essentially the original number minus the integral.
        // Could possibly be optimized in the future
        *self - self.trunc()
    }

    /// Computes the absolute value of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let num = Decimal::new(-3141, 3);
    /// assert_eq!(num.abs().to_string(), "3.141");
    /// ```
    #[must_use]
    pub fn abs(&self) -> Decimal {
        let mut me = *self;
        me.set_sign_positive(true);
        me
    }

    /// Returns the largest integer less than or equal to a number.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let num = Decimal::new(3641, 3);
    /// assert_eq!(num.floor().to_string(), "3");
    /// ```
    #[must_use]
    pub fn floor(&self) -> Decimal {
        let scale = self.scale();
        if scale == 0 {
            // Nothing to do
            return *self;
        }

        // Opportunity for optimization here
        let floored = self.trunc();
        if self.is_sign_negative() && !self.fract().is_zero() {
            floored - ONE
        } else {
            floored
        }
    }

    /// Returns the smallest integer greater than or equal to a number.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let num = Decimal::new(3141, 3);
    /// assert_eq!(num.ceil().to_string(), "4");
    /// let num = Decimal::new(3, 0);
    /// assert_eq!(num.ceil().to_string(), "3");
    /// ```
    #[must_use]
    pub fn ceil(&self) -> Decimal {
        let scale = self.scale();
        if scale == 0 {
            // Nothing to do
            return *self;
        }

        // Opportunity for optimization here
        if self.is_sign_positive() && !self.fract().is_zero() {
            self.trunc() + ONE
        } else {
            self.trunc()
        }
    }

    /// Returns the maximum of the two numbers.
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let x = Decimal::new(1, 0);
    /// let y = Decimal::new(2, 0);
    /// assert_eq!(y, x.max(y));
    /// ```
    #[must_use]
    pub fn max(self, other: Decimal) -> Decimal {
        if self < other {
            other
        } else {
            self
        }
    }

    /// Returns the minimum of the two numbers.
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// let x = Decimal::new(1, 0);
    /// let y = Decimal::new(2, 0);
    /// assert_eq!(x, x.min(y));
    /// ```
    #[must_use]
    pub fn min(self, other: Decimal) -> Decimal {
        if self > other {
            other
        } else {
            self
        }
    }

    /// Strips any trailing zero's from a `Decimal` and converts -0 to 0.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// # fn main() -> Result<(), rust_decimal::Error> {
    /// let number = Decimal::from_str("3.100")?;
    /// assert_eq!(number.normalize().to_string(), "3.1");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn normalize(&self) -> Decimal {
        let mut result = *self;
        result.normalize_assign();
        result
    }

    /// An in place version of `normalize`. Strips any trailing zero's from a `Decimal` and converts -0 to 0.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// # fn main() -> Result<(), rust_decimal::Error> {
    /// let mut number = Decimal::from_str("3.100")?;
    /// assert_eq!(number.to_string(), "3.100");
    /// number.normalize_assign();
    /// assert_eq!(number.to_string(), "3.1");
    /// # Ok(())
    /// # }
    /// ```
    pub fn normalize_assign(&mut self) {
        if self.is_zero() {
            self.flags = 0;
            return;
        }

        let mut scale = self.scale();
        if scale == 0 {
            return;
        }

        let mut result = self.mantissa_array3();
        let mut working = self.mantissa_array3();
        while scale > 0 {
            if ops::array::div_by_u32(&mut working, 10) > 0 {
                break;
            }
            scale -= 1;
            result.copy_from_slice(&working);
        }
        self.lo = result[0];
        self.mid = result[1];
        self.hi = result[2];
        self.flags = flags(self.is_sign_negative(), scale);
    }

    /// Returns a new `Decimal` number with no fractional portion (i.e. an integer).
    /// Rounding currently follows "Bankers Rounding" rules. e.g. 6.5 -> 6, 7.5 -> 8
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// #
    /// // Demonstrating bankers rounding...
    /// let number_down = Decimal::new(65, 1);
    /// let number_up   = Decimal::new(75, 1);
    /// assert_eq!(number_down.round().to_string(), "6");
    /// assert_eq!(number_up.round().to_string(), "8");
    /// ```
    #[must_use]
    pub fn round(&self) -> Decimal {
        self.round_dp(0)
    }

    /// Returns a new `Decimal` number with the specified number of decimal points for fractional
    /// portion.
    /// Rounding is performed using the provided [`RoundingStrategy`]
    ///
    /// # Arguments
    /// * `dp`: the number of decimal points to round to.
    /// * `strategy`: the [`RoundingStrategy`] to use.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::{Decimal, RoundingStrategy};
    /// # use rust_decimal_macros::dec;
    /// #
    /// let tax = dec!(3.4395);
    /// assert_eq!(tax.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero).to_string(), "3.44");
    /// ```
    #[must_use]
    pub fn round_dp_with_strategy(&self, dp: u32, strategy: RoundingStrategy) -> Decimal {
        let old_scale = self.scale();

        // return early if decimal has a smaller number of fractional places than dp
        // e.g. 2.51 rounded to 3 decimal places is 2.51
        if old_scale <= dp {
            return *self;
        }

        // Short circuit for zero
        if self.is_zero() {
            return Decimal {
                lo: 0,
                mid: 0,
                hi: 0,
                flags: flags(self.is_sign_negative(), dp),
            };
        }

        let mut value = [self.lo, self.mid, self.hi];
        let mut value_scale = self.scale();
        let negative = self.is_sign_negative();

        value_scale -= dp;

        // Rescale to zero so it's easier to work with
        while value_scale > 0 {
            if value_scale < 10 {
                ops::array::div_by_u32(&mut value, POWERS_10[value_scale as usize]);
                value_scale = 0;
            } else {
                ops::array::div_by_u32(&mut value, POWERS_10[9]);
                value_scale -= 9;
            }
        }

        // Do some midpoint rounding checks
        // We're actually doing two things here.
        //  1. Figuring out midpoint rounding when we're right on the boundary. e.g. 2.50000
        //  2. Figuring out whether to add one or not e.g. 2.51
        // For this, we need to figure out the fractional portion that is additional to
        // the rounded number. e.g. for 0.12345 rounding to 2dp we'd want 345.
        // We're doing the equivalent of losing precision (e.g. to get 0.12)
        // then increasing the precision back up to 0.12000
        let mut offset = [self.lo, self.mid, self.hi];
        let mut diff = old_scale - dp;

        while diff > 0 {
            if diff < 10 {
                ops::array::div_by_u32(&mut offset, POWERS_10[diff as usize]);
                break;
            } else {
                ops::array::div_by_u32(&mut offset, POWERS_10[9]);
                // Only 9 as this array starts with 1
                diff -= 9;
            }
        }

        let mut diff = old_scale - dp;

        while diff > 0 {
            if diff < 10 {
                ops::array::mul_by_u32(&mut offset, POWERS_10[diff as usize]);
                break;
            } else {
                ops::array::mul_by_u32(&mut offset, POWERS_10[9]);
                // Only 9 as this array starts with 1
                diff -= 9;
            }
        }

        let mut decimal_portion = [self.lo, self.mid, self.hi];
        ops::array::sub_by_internal(&mut decimal_portion, &offset);

        // If the decimal_portion is zero then we round based on the other data
        let mut cap = [5, 0, 0];
        for _ in 0..(old_scale - dp - 1) {
            ops::array::mul_by_u32(&mut cap, 10);
        }
        let order = ops::array::cmp_internal(&decimal_portion, &cap);

        #[allow(deprecated)]
        match strategy {
            RoundingStrategy::BankersRounding | RoundingStrategy::MidpointNearestEven => {
                match order {
                    Ordering::Equal => {
                        if (value[0] & 1) == 1 {
                            ops::array::add_one_internal(&mut value);
                        }
                    }
                    Ordering::Greater => {
                        // Doesn't matter about the decimal portion
                        ops::array::add_one_internal(&mut value);
                    }
                    _ => {}
                }
            }
            RoundingStrategy::RoundHalfDown | RoundingStrategy::MidpointTowardZero => {
                if let Ordering::Greater = order {
                    ops::array::add_one_internal(&mut value);
                }
            }
            RoundingStrategy::RoundHalfUp | RoundingStrategy::MidpointAwayFromZero => {
                // when Ordering::Equal, decimal_portion is 0.5 exactly
                // when Ordering::Greater, decimal_portion is > 0.5
                match order {
                    Ordering::Equal => {
                        ops::array::add_one_internal(&mut value);
                    }
                    Ordering::Greater => {
                        // Doesn't matter about the decimal portion
                        ops::array::add_one_internal(&mut value);
                    }
                    _ => {}
                }
            }
            RoundingStrategy::RoundUp | RoundingStrategy::AwayFromZero => {
                if !ops::array::is_all_zero(&decimal_portion) {
                    ops::array::add_one_internal(&mut value);
                }
            }
            RoundingStrategy::ToPositiveInfinity => {
                if !negative && !ops::array::is_all_zero(&decimal_portion) {
                    ops::array::add_one_internal(&mut value);
                }
            }
            RoundingStrategy::ToNegativeInfinity => {
                if negative && !ops::array::is_all_zero(&decimal_portion) {
                    ops::array::add_one_internal(&mut value);
                }
            }
            RoundingStrategy::RoundDown | RoundingStrategy::ToZero => (),
        }

        Decimal::from_parts(value[0], value[1], value[2], negative, dp)
    }

    /// Returns a new `Decimal` number with the specified number of decimal points for fractional portion.
    /// Rounding currently follows "Bankers Rounding" rules. e.g. 6.5 -> 6, 7.5 -> 8
    ///
    /// # Arguments
    /// * `dp`: the number of decimal points to round to.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// #
    /// let pi = dec!(3.1415926535897932384626433832);
    /// assert_eq!(pi.round_dp(2).to_string(), "3.14");
    /// ```
    #[must_use]
    pub fn round_dp(&self, dp: u32) -> Decimal {
        self.round_dp_with_strategy(dp, RoundingStrategy::MidpointNearestEven)
    }

    /// Returns `Some(Decimal)` number rounded to the specified number of significant digits. If
    /// the resulting number is unable to be represented by the `Decimal` number then `None` will
    /// be returned.
    /// When the number of significant figures of the `Decimal` being rounded is greater than the requested
    /// number of significant digits then rounding will be performed using `MidpointNearestEven` strategy.
    ///
    /// # Arguments
    /// * `digits`: the number of significant digits to round to.
    ///
    /// # Remarks
    /// A significant figure is determined using the following rules:
    /// 1. Non-zero digits are always significant.
    /// 2. Zeros between non-zero digits are always significant.
    /// 3. Leading zeros are never significant.
    /// 4. Trailing zeros are only significant if the number contains a decimal point.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    ///
    /// let value = dec!(305.459);
    /// assert_eq!(value.round_sf(0), Some(dec!(0)));
    /// assert_eq!(value.round_sf(1), Some(dec!(300)));
    /// assert_eq!(value.round_sf(2), Some(dec!(310)));
    /// assert_eq!(value.round_sf(3), Some(dec!(305)));
    /// assert_eq!(value.round_sf(4), Some(dec!(305.5)));
    /// assert_eq!(value.round_sf(5), Some(dec!(305.46)));
    /// assert_eq!(value.round_sf(6), Some(dec!(305.459)));
    /// assert_eq!(value.round_sf(7), Some(dec!(305.4590)));
    /// assert_eq!(Decimal::MAX.round_sf(1), None);
    ///
    /// let value = dec!(0.012301);
    /// assert_eq!(value.round_sf(3), Some(dec!(0.0123)));
    /// ```
    #[must_use]
    pub fn round_sf(&self, digits: u32) -> Option<Decimal> {
        self.round_sf_with_strategy(digits, RoundingStrategy::MidpointNearestEven)
    }

    /// Returns `Some(Decimal)` number rounded to the specified number of significant digits. If
    /// the resulting number is unable to be represented by the `Decimal` number then `None` will
    /// be returned.
    /// When the number of significant figures of the `Decimal` being rounded is greater than the requested
    /// number of significant digits then rounding will be performed using the provided [RoundingStrategy].
    ///
    /// # Arguments
    /// * `digits`: the number of significant digits to round to.
    /// * `strategy`: if required, the rounding strategy to use.
    ///
    /// # Remarks
    /// A significant figure is determined using the following rules:
    /// 1. Non-zero digits are always significant.
    /// 2. Zeros between non-zero digits are always significant.
    /// 3. Leading zeros are never significant.
    /// 4. Trailing zeros are only significant if the number contains a decimal point.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::{Decimal, RoundingStrategy};
    /// # use rust_decimal_macros::dec;
    ///
    /// let value = dec!(305.459);
    /// assert_eq!(value.round_sf_with_strategy(0, RoundingStrategy::ToZero), Some(dec!(0)));
    /// assert_eq!(value.round_sf_with_strategy(1, RoundingStrategy::ToZero), Some(dec!(300)));
    /// assert_eq!(value.round_sf_with_strategy(2, RoundingStrategy::ToZero), Some(dec!(300)));
    /// assert_eq!(value.round_sf_with_strategy(3, RoundingStrategy::ToZero), Some(dec!(305)));
    /// assert_eq!(value.round_sf_with_strategy(4, RoundingStrategy::ToZero), Some(dec!(305.4)));
    /// assert_eq!(value.round_sf_with_strategy(5, RoundingStrategy::ToZero), Some(dec!(305.45)));
    /// assert_eq!(value.round_sf_with_strategy(6, RoundingStrategy::ToZero), Some(dec!(305.459)));
    /// assert_eq!(value.round_sf_with_strategy(7, RoundingStrategy::ToZero), Some(dec!(305.4590)));
    /// assert_eq!(Decimal::MAX.round_sf_with_strategy(1, RoundingStrategy::ToZero), Some(dec!(70000000000000000000000000000)));
    ///
    /// let value = dec!(0.012301);
    /// assert_eq!(value.round_sf_with_strategy(3, RoundingStrategy::AwayFromZero), Some(dec!(0.0124)));
    /// ```
    #[must_use]
    pub fn round_sf_with_strategy(&self, digits: u32, strategy: RoundingStrategy) -> Option<Decimal> {
        if self.is_zero() || digits == 0 {
            return Some(Decimal::ZERO);
        }

        // We start by grabbing the mantissa and figuring out how many significant figures it is
        // made up of. We do this by just dividing by 10 and checking remainders - effectively
        // we're performing a naive log10.
        let mut working = self.mantissa_array3();
        let mut mantissa_sf = 0;
        while !ops::array::is_all_zero(&working) {
            let _remainder = ops::array::div_by_u32(&mut working, 10u32);
            mantissa_sf += 1;
            if working[2] == 0 && working[1] == 0 && working[0] == 1 {
                mantissa_sf += 1;
                break;
            }
        }
        let scale = self.scale();

        match digits.cmp(&mantissa_sf) {
            Ordering::Greater => {
                // If we're requesting a higher number of significant figures, we rescale
                let mut array = [self.lo, self.mid, self.hi];
                let mut value_scale = scale;
                ops::array::rescale_internal(&mut array, &mut value_scale, scale + digits - mantissa_sf);
                Some(Decimal {
                    lo: array[0],
                    mid: array[1],
                    hi: array[2],
                    flags: flags(self.is_sign_negative(), value_scale),
                })
            }
            Ordering::Less => {
                // We're requesting a lower number of significant digits.
                let diff = mantissa_sf - digits;
                // If the diff is greater than the scale we're focused on the integral. Otherwise, we can
                // just round.
                if diff > scale {
                    use crate::constants::BIG_POWERS_10;
                    // We need to adjust the integral portion. This also should be rounded, consequently
                    // we reduce the number down, round it, and then scale back up.
                    // E.g. If we have 305.459 scaling to a sf of 2 - we first reduce the number
                    // down to 30.5459, round it to 31 and then scale it back up to 310.
                    // Likewise, if we have 12301 scaling to a sf of 3 - we first reduce the number
                    // down to 123.01, round it to 123 and then scale it back up to 12300.
                    let mut num = *self;
                    let mut exp = (diff - scale) as usize;
                    while exp > 0 {
                        let pow;
                        if exp >= BIG_POWERS_10.len() {
                            pow = Decimal::from(BIG_POWERS_10[BIG_POWERS_10.len() - 1]);
                            exp -= BIG_POWERS_10.len();
                        } else {
                            pow = Decimal::from(BIG_POWERS_10[exp - 1]);
                            exp = 0;
                        }
                        num = num.checked_div(pow)?;
                    }
                    let mut num = num.round_dp_with_strategy(0, strategy).trunc();
                    let mut exp = (mantissa_sf - digits - scale) as usize;
                    while exp > 0 {
                        let pow;
                        if exp >= BIG_POWERS_10.len() {
                            pow = Decimal::from(BIG_POWERS_10[BIG_POWERS_10.len() - 1]);
                            exp -= BIG_POWERS_10.len();
                        } else {
                            pow = Decimal::from(BIG_POWERS_10[exp - 1]);
                            exp = 0;
                        }
                        num = num.checked_mul(pow)?;
                    }
                    Some(num)
                } else {
                    Some(self.round_dp_with_strategy(scale - diff, strategy))
                }
            }
            Ordering::Equal => {
                // Case where significant figures = requested significant digits.
                Some(*self)
            }
        }
    }

    /// Convert `Decimal` to an internal representation of the underlying struct. This is useful
    /// for debugging the internal state of the object.
    ///
    /// # Important Disclaimer
    /// This is primarily intended for library maintainers. The internal representation of a
    /// `Decimal` is considered "unstable" for public use.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    ///
    /// let pi = dec!(3.1415926535897932384626433832);
    /// assert_eq!(format!("{:?}", pi), "3.1415926535897932384626433832");
    /// assert_eq!(format!("{:?}", pi.unpack()), "UnpackedDecimal { \
    ///     negative: false, scale: 28, hi: 1703060790, mid: 185874565, lo: 1102470952 \
    /// }");
    /// ```
    #[must_use]
    pub const fn unpack(&self) -> UnpackedDecimal {
        UnpackedDecimal {
            negative: self.is_sign_negative(),
            scale: self.scale(),
            hi: self.hi,
            lo: self.lo,
            mid: self.mid,
        }
    }

    #[inline(always)]
    pub(crate) const fn lo(&self) -> u32 {
        self.lo
    }

    #[inline(always)]
    pub(crate) const fn mid(&self) -> u32 {
        self.mid
    }

    #[inline(always)]
    pub(crate) const fn hi(&self) -> u32 {
        self.hi
    }

    #[inline(always)]
    pub(crate) const fn flags(&self) -> u32 {
        self.flags
    }

    #[inline(always)]
    pub(crate) const fn mantissa_array3(&self) -> [u32; 3] {
        [self.lo, self.mid, self.hi]
    }

    #[inline(always)]
    pub(crate) const fn mantissa_array4(&self) -> [u32; 4] {
        [self.lo, self.mid, self.hi, 0]
    }

    /// Parses a 32-bit float into a Decimal number whilst retaining any non-guaranteed precision.
    ///
    /// Typically when a float is parsed in Rust Decimal, any excess bits (after ~7.22 decimal points for
    /// f32 as per IEEE-754) are removed due to any digits following this are considered an approximation
    /// at best. This function bypasses this additional step and retains these excess bits.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// #
    /// // Usually floats are parsed leveraging float guarantees. i.e. 0.1_f32 => 0.1
    /// assert_eq!("0.1", Decimal::from_f32(0.1_f32).unwrap().to_string());
    ///
    /// // Sometimes, we may want to represent the approximation exactly.
    /// assert_eq!("0.100000001490116119384765625", Decimal::from_f32_retain(0.1_f32).unwrap().to_string());
    /// ```
    pub fn from_f32_retain(n: f32) -> Option<Self> {
        from_f32(n, false)
    }

    /// Parses a 64-bit float into a Decimal number whilst retaining any non-guaranteed precision.
    ///
    /// Typically when a float is parsed in Rust Decimal, any excess bits (after ~15.95 decimal points for
    /// f64 as per IEEE-754) are removed due to any digits following this are considered an approximation
    /// at best. This function bypasses this additional step and retains these excess bits.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_decimal::prelude::*;
    /// #
    /// // Usually floats are parsed leveraging float guarantees. i.e. 0.1_f64 => 0.1
    /// assert_eq!("0.1", Decimal::from_f64(0.1_f64).unwrap().to_string());
    ///
    /// // Sometimes, we may want to represent the approximation exactly.
    /// assert_eq!("0.1000000000000000055511151231", Decimal::from_f64_retain(0.1_f64).unwrap().to_string());
    /// ```
    pub fn from_f64_retain(n: f64) -> Option<Self> {
        from_f64(n, false)
    }
}

impl Default for Decimal {
    /// Returns the default value for a `Decimal` (equivalent to `Decimal::ZERO`). [Read more]
    ///
    /// [Read more]: core::default::Default#tymethod.default
    #[inline]
    fn default() -> Self {
        ZERO
    }
}

pub(crate) enum CalculationResult {
    Ok(Decimal),
    Overflow,
    DivByZero,
}

#[inline]
const fn flags(neg: bool, scale: u32) -> u32 {
    (scale << SCALE_SHIFT) | ((neg as u32) << SIGN_SHIFT)
}

macro_rules! integer_docs {
    ( true ) => {
        " by truncating and returning the integer component"
    };
    ( false ) => {
        ""
    };
}

// #[doc] attributes are formatted poorly with rustfmt so skip for now.
// See https://github.com/rust-lang/rustfmt/issues/5062 for more information.
#[rustfmt::skip]
macro_rules! impl_try_from_decimal {
    ($TInto:ty, $conversion_fn:path, $additional_docs:expr) => {
        #[doc = concat!(
            "Try to convert a `Decimal` to `",
            stringify!($TInto),
            "`",
            $additional_docs,
            ".\n\nCan fail if the `Decimal` is out of range for `",
            stringify!($TInto),
            "`.",
        )]
        impl TryFrom<Decimal> for $TInto {
            type Error = crate::Error;

            #[inline]
            fn try_from(t: Decimal) -> Result<Self, Error> {
                $conversion_fn(&t).ok_or_else(|| Error::ConversionTo(stringify!($TInto).into()))
            }
        }
    };
}

impl_try_from_decimal!(f32, Decimal::to_f32, integer_docs!(false));
impl_try_from_decimal!(f64, Decimal::to_f64, integer_docs!(false));
impl_try_from_decimal!(isize, Decimal::to_isize, integer_docs!(true));
impl_try_from_decimal!(i8, Decimal::to_i8, integer_docs!(true));
impl_try_from_decimal!(i16, Decimal::to_i16, integer_docs!(true));
impl_try_from_decimal!(i32, Decimal::to_i32, integer_docs!(true));
impl_try_from_decimal!(i64, Decimal::to_i64, integer_docs!(true));
impl_try_from_decimal!(i128, Decimal::to_i128, integer_docs!(true));
impl_try_from_decimal!(usize, Decimal::to_usize, integer_docs!(true));
impl_try_from_decimal!(u8, Decimal::to_u8, integer_docs!(true));
impl_try_from_decimal!(u16, Decimal::to_u16, integer_docs!(true));
impl_try_from_decimal!(u32, Decimal::to_u32, integer_docs!(true));
impl_try_from_decimal!(u64, Decimal::to_u64, integer_docs!(true));
impl_try_from_decimal!(u128, Decimal::to_u128, integer_docs!(true));

// #[doc] attributes are formatted poorly with rustfmt so skip for now.
// See https://github.com/rust-lang/rustfmt/issues/5062 for more information.
#[rustfmt::skip]
macro_rules! impl_try_from_primitive {
    ($TFrom:ty, $conversion_fn:path $(, $err:expr)?) => {
        #[doc = concat!(
            "Try to convert a `",
            stringify!($TFrom),
            "` into a `Decimal`.\n\nCan fail if the value is out of range for `Decimal`."
        )]
        impl TryFrom<$TFrom> for Decimal {
            type Error = crate::Error;

            #[inline]
            fn try_from(t: $TFrom) -> Result<Self, Error> {
                $conversion_fn(t) $( .ok_or_else(|| $err) )?
            }
        }
    };
}

impl_try_from_primitive!(f32, Self::from_f32, Error::ConversionTo("Decimal".into()));
impl_try_from_primitive!(f64, Self::from_f64, Error::ConversionTo("Decimal".into()));
impl_try_from_primitive!(&str, core::str::FromStr::from_str);

macro_rules! impl_from {
    ($T:ty, $from_ty:path) => {
        ///
        /// Conversion to `Decimal`.
        ///
        impl core::convert::From<$T> for Decimal {
            #[inline]
            fn from(t: $T) -> Self {
                $from_ty(t).unwrap()
            }
        }
    };
}

impl_from!(isize, FromPrimitive::from_isize);
impl_from!(i8, FromPrimitive::from_i8);
impl_from!(i16, FromPrimitive::from_i16);
impl_from!(i32, FromPrimitive::from_i32);
impl_from!(i64, FromPrimitive::from_i64);
impl_from!(usize, FromPrimitive::from_usize);
impl_from!(u8, FromPrimitive::from_u8);
impl_from!(u16, FromPrimitive::from_u16);
impl_from!(u32, FromPrimitive::from_u32);
impl_from!(u64, FromPrimitive::from_u64);

impl_from!(i128, FromPrimitive::from_i128);
impl_from!(u128, FromPrimitive::from_u128);

impl Zero for Decimal {
    fn zero() -> Decimal {
        ZERO
    }

    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl One for Decimal {
    fn one() -> Decimal {
        ONE
    }
}

impl Signed for Decimal {
    fn abs(&self) -> Self {
        self.abs()
    }

    fn abs_sub(&self, other: &Self) -> Self {
        if self <= other {
            ZERO
        } else {
            self - other
        }
    }

    fn signum(&self) -> Self {
        if self.is_zero() {
            ZERO
        } else {
            let mut value = ONE;
            if self.is_sign_negative() {
                value.set_sign_negative(true);
            }
            value
        }
    }

    fn is_positive(&self) -> bool {
        self.is_sign_positive()
    }

    fn is_negative(&self) -> bool {
        self.is_sign_negative()
    }
}

impl Num for Decimal {
    type FromStrRadixErr = Error;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Decimal::from_str_radix(str, radix)
    }
}

impl FromStr for Decimal {
    type Err = Error;

    fn from_str(value: &str) -> Result<Decimal, Self::Err> {
        crate::str::parse_str_radix_10(value)
    }
}

impl FromPrimitive for Decimal {
    fn from_i32(n: i32) -> Option<Decimal> {
        let flags: u32;
        let value_copy: i64;
        if n >= 0 {
            flags = 0;
            value_copy = n as i64;
        } else {
            flags = SIGN_MASK;
            value_copy = -(n as i64);
        }
        Some(Decimal {
            flags,
            lo: value_copy as u32,
            mid: 0,
            hi: 0,
        })
    }

    fn from_i64(n: i64) -> Option<Decimal> {
        let flags: u32;
        let value_copy: i128;
        if n >= 0 {
            flags = 0;
            value_copy = n as i128;
        } else {
            flags = SIGN_MASK;
            value_copy = -(n as i128);
        }
        Some(Decimal {
            flags,
            lo: value_copy as u32,
            mid: (value_copy >> 32) as u32,
            hi: 0,
        })
    }

    fn from_i128(n: i128) -> Option<Decimal> {
        let flags;
        let unsigned;
        if n >= 0 {
            unsigned = n as u128;
            flags = 0;
        } else {
            unsigned = n.unsigned_abs();
            flags = SIGN_MASK;
        };
        // Check if we overflow
        if unsigned >> 96 != 0 {
            return None;
        }
        Some(Decimal {
            flags,
            lo: unsigned as u32,
            mid: (unsigned >> 32) as u32,
            hi: (unsigned >> 64) as u32,
        })
    }

    fn from_u32(n: u32) -> Option<Decimal> {
        Some(Decimal {
            flags: 0,
            lo: n,
            mid: 0,
            hi: 0,
        })
    }

    fn from_u64(n: u64) -> Option<Decimal> {
        Some(Decimal {
            flags: 0,
            lo: n as u32,
            mid: (n >> 32) as u32,
            hi: 0,
        })
    }

    fn from_u128(n: u128) -> Option<Decimal> {
        // Check if we overflow
        if n >> 96 != 0 {
            return None;
        }
        Some(Decimal {
            flags: 0,
            lo: n as u32,
            mid: (n >> 32) as u32,
            hi: (n >> 64) as u32,
        })
    }

    fn from_f32(n: f32) -> Option<Decimal> {
        // By default, we remove excess bits. This allows 0.1_f64 == dec!(0.1).
        from_f32(n, true)
    }

    fn from_f64(n: f64) -> Option<Decimal> {
        // By default, we remove excess bits. This allows 0.1_f64 == dec!(0.1).
        from_f64(n, true)
    }
}

#[inline]
fn from_f64(n: f64, remove_excess_bits: bool) -> Option<Decimal> {
    // Handle the case if it is NaN, Infinity or -Infinity
    if !n.is_finite() {
        return None;
    }

    // It's a shame we can't use a union for this due to it being broken up by bits
    // i.e. 1/11/52 (sign, exponent, mantissa)
    // See https://en.wikipedia.org/wiki/IEEE_754-1985
    // n = (sign*-1) * 2^exp * mantissa
    // Decimal of course stores this differently... 10^-exp * significand
    let raw = n.to_bits();
    let positive = (raw >> 63) == 0;
    let biased_exponent = ((raw >> 52) & 0x7FF) as i32;
    let mantissa = raw & 0x000F_FFFF_FFFF_FFFF;

    // Handle the special zero case
    if biased_exponent == 0 && mantissa == 0 {
        let mut zero = ZERO;
        if !positive {
            zero.set_sign_negative(true);
        }
        return Some(zero);
    }

    // Get the bits and exponent2
    let mut exponent2 = biased_exponent - 1023;
    let mut bits = [
        (mantissa & 0xFFFF_FFFF) as u32,
        ((mantissa >> 32) & 0xFFFF_FFFF) as u32,
        0u32,
    ];
    if biased_exponent == 0 {
        // Denormalized number - correct the exponent
        exponent2 += 1;
    } else {
        // Add extra hidden bit to mantissa
        bits[1] |= 0x0010_0000;
    }

    // The act of copying a mantissa as integer bits is equivalent to shifting
    // left the mantissa 52 bits. The exponent is reduced to compensate.
    exponent2 -= 52;

    // Convert to decimal
    base2_to_decimal(&mut bits, exponent2, positive, true, remove_excess_bits)
}

#[inline]
fn from_f32(n: f32, remove_excess_bits: bool) -> Option<Decimal> {
    // Handle the case if it is NaN, Infinity or -Infinity
    if !n.is_finite() {
        return None;
    }

    // It's a shame we can't use a union for this due to it being broken up by bits
    // i.e. 1/8/23 (sign, exponent, mantissa)
    // See https://en.wikipedia.org/wiki/IEEE_754-1985
    // n = (sign*-1) * 2^exp * mantissa
    // Decimal of course stores this differently... 10^-exp * significand
    let raw = n.to_bits();
    let positive = (raw >> 31) == 0;
    let biased_exponent = ((raw >> 23) & 0xFF) as i32;
    let mantissa = raw & 0x007F_FFFF;

    // Handle the special zero case
    if biased_exponent == 0 && mantissa == 0 {
        let mut zero = ZERO;
        if !positive {
            zero.set_sign_negative(true);
        }
        return Some(zero);
    }

    // Get the bits and exponent2
    let mut exponent2 = biased_exponent - 127;
    let mut bits = [mantissa, 0u32, 0u32];
    if biased_exponent == 0 {
        // Denormalized number - correct the exponent
        exponent2 += 1;
    } else {
        // Add extra hidden bit to mantissa
        bits[0] |= 0x0080_0000;
    }

    // The act of copying a mantissa as integer bits is equivalent to shifting
    // left the mantissa 23 bits. The exponent is reduced to compensate.
    exponent2 -= 23;

    // Convert to decimal
    base2_to_decimal(&mut bits, exponent2, positive, false, remove_excess_bits)
}

fn base2_to_decimal(
    bits: &mut [u32; 3],
    exponent2: i32,
    positive: bool,
    is64: bool,
    remove_excess_bits: bool,
) -> Option<Decimal> {
    // 2^exponent2 = (10^exponent2)/(5^exponent2)
    //             = (5^-exponent2)*(10^exponent2)
    let mut exponent5 = -exponent2;
    let mut exponent10 = exponent2; // Ultimately, we want this for the scale

    while exponent5 > 0 {
        // Check to see if the mantissa is divisible by 2
        if bits[0] & 0x1 == 0 {
            exponent10 += 1;
            exponent5 -= 1;

            // We can divide by 2 without losing precision
            let hi_carry = bits[2] & 0x1 == 1;
            bits[2] >>= 1;
            let mid_carry = bits[1] & 0x1 == 1;
            bits[1] = (bits[1] >> 1) | if hi_carry { SIGN_MASK } else { 0 };
            bits[0] = (bits[0] >> 1) | if mid_carry { SIGN_MASK } else { 0 };
        } else {
            // The mantissa is NOT divisible by 2. Therefore the mantissa should
            // be multiplied by 5, unless the multiplication overflows.
            exponent5 -= 1;

            let mut temp = [bits[0], bits[1], bits[2]];
            if ops::array::mul_by_u32(&mut temp, 5) == 0 {
                // Multiplication succeeded without overflow, so copy result back
                bits[0] = temp[0];
                bits[1] = temp[1];
                bits[2] = temp[2];
            } else {
                // Multiplication by 5 overflows. The mantissa should be divided
                // by 2, and therefore will lose significant digits.
                exponent10 += 1;

                // Shift right
                let hi_carry = bits[2] & 0x1 == 1;
                bits[2] >>= 1;
                let mid_carry = bits[1] & 0x1 == 1;
                bits[1] = (bits[1] >> 1) | if hi_carry { SIGN_MASK } else { 0 };
                bits[0] = (bits[0] >> 1) | if mid_carry { SIGN_MASK } else { 0 };
            }
        }
    }

    // In order to divide the value by 5, it is best to multiply by 2/10.
    // Therefore, exponent10 is decremented, and the mantissa should be multiplied by 2
    while exponent5 < 0 {
        if bits[2] & SIGN_MASK == 0 {
            // No far left bit, the mantissa can withstand a shift-left without overflowing
            exponent10 -= 1;
            exponent5 += 1;
            ops::array::shl1_internal(bits, 0);
        } else if exponent10 * 2 > -exponent5 {
            // Multiplying by >=2 which, due to the previous condition, means an overflow.
            return None;
        } else {
            // The mantissa would overflow if shifted. Therefore it should be
            // directly divided by 5. This will lose significant digits, unless
            // by chance the mantissa happens to be divisible by 5.
            exponent5 += 1;
            ops::array::div_by_u32(bits, 5);
        }
    }

    // At this point, the mantissa has assimilated the exponent5, but
    // exponent10 might not be suitable for assignment. exponent10 must be
    // in the range [-MAX_SCALE..0], so the mantissa must be scaled up or
    // down appropriately.
    while exponent10 > 0 {
        // In order to bring exponent10 down to 0, the mantissa should be
        // multiplied by 10 to compensate. If the exponent10 is too big, this
        // will cause the mantissa to overflow.
        if ops::array::mul_by_u32(bits, 10) == 0 {
            exponent10 -= 1;
        } else {
            // Overflowed - return?
            return None;
        }
    }

    // In order to bring exponent up to -MAX_SCALE, the mantissa should
    // be divided by 10 to compensate. If the exponent10 is too small, this
    // will cause the mantissa to underflow and become 0.
    while exponent10 < -(Decimal::MAX_SCALE as i32) {
        let rem10 = ops::array::div_by_u32(bits, 10);
        exponent10 += 1;
        if ops::array::is_all_zero(bits) {
            // Underflow, unable to keep dividing
            exponent10 = 0;
        } else if rem10 >= 5 {
            ops::array::add_one_internal(bits);
        }
    }

    if remove_excess_bits {
        // This step is required in order to remove excess bits of precision from the
        // end of the bit representation, down to the precision guaranteed by the
        // floating point number (see IEEE-754).
        if is64 {
            // Guaranteed to approx 15/16 dp
            while exponent10 < 0 && (bits[2] != 0 || (bits[1] & 0xFFF0_0000) != 0) {
                let rem10 = ops::array::div_by_u32(bits, 10);
                exponent10 += 1;
                if rem10 >= 5 {
                    ops::array::add_one_internal(bits);
                }
            }
        } else {
            // Guaranteed to about 7/8 dp
            while exponent10 < 0 && ((bits[0] & 0xFF00_0000) != 0 || bits[1] != 0 || bits[2] != 0) {
                let rem10 = ops::array::div_by_u32(bits, 10);
                exponent10 += 1;
                if rem10 >= 5 {
                    ops::array::add_one_internal(bits);
                }
            }
        }

        // Remove multiples of 10 from the representation
        while exponent10 < 0 {
            let mut temp = [bits[0], bits[1], bits[2]];
            let remainder = ops::array::div_by_u32(&mut temp, 10);
            if remainder == 0 {
                exponent10 += 1;
                bits[0] = temp[0];
                bits[1] = temp[1];
                bits[2] = temp[2];
            } else {
                break;
            }
        }
    }

    Some(Decimal {
        lo: bits[0],
        mid: bits[1],
        hi: bits[2],
        flags: flags(!positive, -exponent10 as u32),
    })
}

impl ToPrimitive for Decimal {
    fn to_i64(&self) -> Option<i64> {
        let d = self.trunc();
        // If it is in the hi bit then it is a clear overflow.
        if d.hi != 0 {
            // Overflow
            return None;
        }
        let negative = self.is_sign_negative();

        // A bit more convoluted in terms of checking when it comes to the hi bit due to twos-complement
        if d.mid & 0x8000_0000 > 0 {
            if negative && d.mid == 0x8000_0000 && d.lo == 0 {
                // We do this because below we try to convert the i64 to a positive first - of which
                // doesn't fit into an i64.
                return Some(i64::MIN);
            }
            return None;
        }

        let raw: i64 = (i64::from(d.mid) << 32) | i64::from(d.lo);
        if negative {
            Some(raw.neg())
        } else {
            Some(raw)
        }
    }

    fn to_i128(&self) -> Option<i128> {
        let d = self.trunc();
        let raw: i128 = ((i128::from(d.hi) << 64) | (i128::from(d.mid) << 32)) | i128::from(d.lo);
        if self.is_sign_negative() {
            Some(-raw)
        } else {
            Some(raw)
        }
    }

    fn to_u64(&self) -> Option<u64> {
        if self.is_sign_negative() {
            return None;
        }

        let d = self.trunc();
        if d.hi != 0 {
            // Overflow
            return None;
        }

        Some((u64::from(d.mid) << 32) | u64::from(d.lo))
    }

    fn to_u128(&self) -> Option<u128> {
        if self.is_sign_negative() {
            return None;
        }

        let d = self.trunc();
        Some((u128::from(d.hi) << 64) | (u128::from(d.mid) << 32) | u128::from(d.lo))
    }

    fn to_f64(&self) -> Option<f64> {
        if self.scale() == 0 {
            // If scale is zero, we are storing a 96-bit integer value, that would
            // always fit into i128, which in turn is always representable as f64,
            // albeit with loss of precision for values outside of -2^53..2^53 range.
            let integer = self.to_i128();
            integer.map(|i| i as f64)
        } else {
            let neg = self.is_sign_negative();
            let mut mantissa: u128 = self.lo.into();
            mantissa |= (self.mid as u128) << 32;
            mantissa |= (self.hi as u128) << 64;
            // scale is at most 28, so this fits comfortably into a u128.
            let scale = self.scale();
            let precision: u128 = 10_u128.pow(scale);
            let integral_part = mantissa / precision;
            let frac_part = mantissa % precision;
            let frac_f64 = (frac_part as f64) / (precision as f64);
            let integral = integral_part as f64;
            // If there is a fractional component then we will need to add that and remove any
            // inaccuracies that creep in during addition. Otherwise, if the fractional component
            // is zero we can exit early.
            if frac_f64.is_zero() {
                if neg {
                    return Some(-integral);
                }
                return Some(integral);
            }
            let value = integral + frac_f64;
            let round_to = 10f64.powi(self.scale() as i32);
            let rounded = (value * round_to).round() / round_to;
            if neg {
                Some(-rounded)
            } else {
                Some(rounded)
            }
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let (rep, additional) = crate::str::to_str_internal(self, false, f.precision());
        if let Some(additional) = additional {
            let value = [rep.as_str(), "0".repeat(additional).as_str()].concat();
            f.pad_integral(self.is_sign_positive(), "", value.as_str())
        } else {
            f.pad_integral(self.is_sign_positive(), "", rep.as_str())
        }
    }
}

impl fmt::Debug for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::LowerExp for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::str::fmt_scientific_notation(self, "e", f)
    }
}

impl fmt::UpperExp for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::str::fmt_scientific_notation(self, "E", f)
    }
}

impl Neg for Decimal {
    type Output = Decimal;

    fn neg(self) -> Decimal {
        let mut copy = self;
        copy.set_sign_negative(self.is_sign_positive());
        copy
    }
}

impl Neg for &Decimal {
    type Output = Decimal;

    fn neg(self) -> Decimal {
        Decimal {
            flags: flags(!self.is_sign_negative(), self.scale()),
            hi: self.hi,
            lo: self.lo,
            mid: self.mid,
        }
    }
}

impl AddAssign for Decimal {
    fn add_assign(&mut self, other: Decimal) {
        let result = self.add(other);
        self.lo = result.lo;
        self.mid = result.mid;
        self.hi = result.hi;
        self.flags = result.flags;
    }
}

impl<'a> AddAssign<&'a Decimal> for Decimal {
    fn add_assign(&mut self, other: &'a Decimal) {
        Decimal::add_assign(self, *other)
    }
}

impl AddAssign<Decimal> for &mut Decimal {
    fn add_assign(&mut self, other: Decimal) {
        Decimal::add_assign(*self, other)
    }
}

impl<'a> AddAssign<&'a Decimal> for &'a mut Decimal {
    fn add_assign(&mut self, other: &'a Decimal) {
        Decimal::add_assign(*self, *other)
    }
}

impl SubAssign for Decimal {
    fn sub_assign(&mut self, other: Decimal) {
        let result = self.sub(other);
        self.lo = result.lo;
        self.mid = result.mid;
        self.hi = result.hi;
        self.flags = result.flags;
    }
}

impl<'a> SubAssign<&'a Decimal> for Decimal {
    fn sub_assign(&mut self, other: &'a Decimal) {
        Decimal::sub_assign(self, *other)
    }
}

impl SubAssign<Decimal> for &mut Decimal {
    fn sub_assign(&mut self, other: Decimal) {
        Decimal::sub_assign(*self, other)
    }
}

impl<'a> SubAssign<&'a Decimal> for &'a mut Decimal {
    fn sub_assign(&mut self, other: &'a Decimal) {
        Decimal::sub_assign(*self, *other)
    }
}

impl MulAssign for Decimal {
    fn mul_assign(&mut self, other: Decimal) {
        let result = self.mul(other);
        self.lo = result.lo;
        self.mid = result.mid;
        self.hi = result.hi;
        self.flags = result.flags;
    }
}

impl<'a> MulAssign<&'a Decimal> for Decimal {
    fn mul_assign(&mut self, other: &'a Decimal) {
        Decimal::mul_assign(self, *other)
    }
}

impl MulAssign<Decimal> for &mut Decimal {
    fn mul_assign(&mut self, other: Decimal) {
        Decimal::mul_assign(*self, other)
    }
}

impl<'a> MulAssign<&'a Decimal> for &'a mut Decimal {
    fn mul_assign(&mut self, other: &'a Decimal) {
        Decimal::mul_assign(*self, *other)
    }
}

impl DivAssign for Decimal {
    fn div_assign(&mut self, other: Decimal) {
        let result = self.div(other);
        self.lo = result.lo;
        self.mid = result.mid;
        self.hi = result.hi;
        self.flags = result.flags;
    }
}

impl<'a> DivAssign<&'a Decimal> for Decimal {
    fn div_assign(&mut self, other: &'a Decimal) {
        Decimal::div_assign(self, *other)
    }
}

impl DivAssign<Decimal> for &mut Decimal {
    fn div_assign(&mut self, other: Decimal) {
        Decimal::div_assign(*self, other)
    }
}

impl<'a> DivAssign<&'a Decimal> for &'a mut Decimal {
    fn div_assign(&mut self, other: &'a Decimal) {
        Decimal::div_assign(*self, *other)
    }
}

impl RemAssign for Decimal {
    fn rem_assign(&mut self, other: Decimal) {
        let result = self.rem(other);
        self.lo = result.lo;
        self.mid = result.mid;
        self.hi = result.hi;
        self.flags = result.flags;
    }
}

impl<'a> RemAssign<&'a Decimal> for Decimal {
    fn rem_assign(&mut self, other: &'a Decimal) {
        Decimal::rem_assign(self, *other)
    }
}

impl RemAssign<Decimal> for &mut Decimal {
    fn rem_assign(&mut self, other: Decimal) {
        Decimal::rem_assign(*self, other)
    }
}

impl<'a> RemAssign<&'a Decimal> for &'a mut Decimal {
    fn rem_assign(&mut self, other: &'a Decimal) {
        Decimal::rem_assign(*self, *other)
    }
}

impl PartialEq for Decimal {
    #[inline]
    fn eq(&self, other: &Decimal) -> bool {
        self.cmp(other) == Equal
    }
}

impl Eq for Decimal {}

impl Hash for Decimal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let n = self.normalize();
        n.lo.hash(state);
        n.mid.hash(state);
        n.hi.hash(state);
        n.flags.hash(state);
    }
}

impl PartialOrd for Decimal {
    #[inline]
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Decimal) -> Ordering {
        ops::cmp_impl(self, other)
    }
}

impl Product for Decimal {
    /// Panics if out-of-bounds
    fn product<I: Iterator<Item = Decimal>>(iter: I) -> Self {
        let mut product = ONE;
        for i in iter {
            product *= i;
        }
        product
    }
}

impl<'a> Product<&'a Decimal> for Decimal {
    /// Panics if out-of-bounds
    fn product<I: Iterator<Item = &'a Decimal>>(iter: I) -> Self {
        let mut product = ONE;
        for i in iter {
            product *= i;
        }
        product
    }
}

impl Sum for Decimal {
    fn sum<I: Iterator<Item = Decimal>>(iter: I) -> Self {
        let mut sum = ZERO;
        for i in iter {
            sum += i;
        }
        sum
    }
}

impl<'a> Sum<&'a Decimal> for Decimal {
    fn sum<I: Iterator<Item = &'a Decimal>>(iter: I) -> Self {
        let mut sum = ZERO;
        for i in iter {
            sum += i;
        }
        sum
    }
}
