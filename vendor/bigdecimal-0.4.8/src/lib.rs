// Copyright 2016 Adam Sunderland
//           2016-2023 Andrew Kubera
//           2017 Ruben De Smet
// See the COPYRIGHT file at the top-level directory of this
// distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A Big Decimal
//!
//! `BigDecimal` allows storing any real number to arbitrary precision; which
//! avoids common floating point errors (such as 0.1 + 0.2 ≠ 0.3) at the
//! cost of complexity.
//!
//! Internally, `BigDecimal` uses a `BigInt` object, paired with a 64-bit
//! integer which determines the position of the decimal point. Therefore,
//! the precision *is not* actually arbitrary, but limited to 2<sup>63</sup>
//! decimal places.
//!
//! Common numerical operations are overloaded, so we can treat them
//! the same way we treat other numbers.
//!
//! It is not recommended to convert a floating point number to a decimal
//! directly, as the floating point representation may be unexpected.
//!
//! # Example
//!
//! ```
//! use bigdecimal::BigDecimal;
//! use std::str::FromStr;
//!
//! let input = "0.8";
//! let dec = BigDecimal::from_str(&input).unwrap();
//! let float = f32::from_str(&input).unwrap();
//!
//! println!("Input ({}) with 10 decimals: {} vs {})", input, dec, float);
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::style)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_return)]
#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::approx_constant)]
#![allow(clippy::wrong_self_convention)]
#![cfg_attr(test, allow(clippy::useless_vec))]
#![allow(unused_imports)]


pub extern crate num_bigint;
pub extern crate num_traits;
extern crate num_integer;

#[cfg(test)]
extern crate paste;

#[cfg(feature = "serde")]
extern crate serde as serde_crate;

#[cfg(all(test, any(feature = "serde", feature = "serde_json")))]
extern crate serde_test;

#[cfg(all(test, feature = "serde_json"))]
extern crate serde_json;

#[cfg(feature = "std")]
include!("./with_std.rs");

#[cfg(not(feature = "std"))]
include!("./without_std.rs");

// make available some standard items
use self::stdlib::cmp::{self, Ordering};
use self::stdlib::convert::TryFrom;
use self::stdlib::default::Default;
use self::stdlib::hash::{Hash, Hasher};
use self::stdlib::num::{ParseFloatError, ParseIntError};
use self::stdlib::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Rem, RemAssign,
};
use self::stdlib::iter::Sum;
use self::stdlib::str::FromStr;
use self::stdlib::string::{String, ToString};
use self::stdlib::fmt;
use self::stdlib::Vec;
use self::stdlib::borrow::Cow;

use num_bigint::{BigInt, BigUint, ParseBigIntError, Sign};
use num_integer::Integer as IntegerTrait;
pub use num_traits::{FromPrimitive, Num, One, Pow, Signed, ToPrimitive, Zero};

use stdlib::f64::consts::LOG2_10;


// const DEFAULT_PRECISION: u64 = ${RUST_BIGDECIMAL_DEFAULT_PRECISION} or 100;
include!(concat!(env!("OUT_DIR"), "/default_precision.rs"));

#[macro_use]
mod macros;

// "low level" functions
mod arithmetic;

// From<T>, To<T>, TryFrom<T> impls
mod impl_convert;
mod impl_trait_from_str;

// Add<T>, Sub<T>, etc...
mod impl_ops;
mod impl_ops_add;
mod impl_ops_sub;
mod impl_ops_mul;
mod impl_ops_div;
mod impl_ops_rem;

// PartialEq
mod impl_cmp;

// Implementations of num_traits
mod impl_num;

// Implementations of std::fmt traits and stringificaiton routines
mod impl_fmt;

// Implementations for deserializations and serializations
#[cfg(any(feature = "serde", feature="serde_json"))]
pub mod impl_serde;

/// re-export serde-json derive modules
#[cfg(feature = "serde_json")]
pub mod serde {
    /// Parse JSON number directly to BigDecimal
    pub use impl_serde::arbitrary_precision as json_num;
    /// Parse JSON (number | null) directly to Option<BigDecimal>
    pub use impl_serde::arbitrary_precision_option as json_num_option;
}


// construct BigDecimals from strings and floats
mod parsing;

// Routines for rounding
pub mod rounding;
pub use rounding::RoundingMode;

// Mathematical context
mod context;
pub use context::Context;

use arithmetic::{
    ten_to_the,
    ten_to_the_uint,
    ten_to_the_u64,
    diff,
    diff_usize,
    count_decimal_digits,
    count_decimal_digits_uint,
};


/// Internal function used for rounding
///
/// returns 1 if most significant digit is >= 5, otherwise 0
///
/// This is used after dividing a number by a power of ten and
/// rounding the last digit.
///
#[inline(always)]
fn get_rounding_term(num: &BigInt) -> u8 {
    if num.is_zero() {
        return 0;
    }

    let digits = (num.bits() as f64 / LOG2_10) as u64;
    let mut n = ten_to_the(digits);

    // loop-method
    loop {
        if *num < n {
            return 1;
        }
        n *= 5;
        if *num < n {
            return 0;
        }
        n *= 2;
    }

    // string-method
    // let s = format!("{}", num);
    // let high_digit = u8::from_str(&s[0..1]).unwrap();
    // if high_digit < 5 { 0 } else { 1 }
}

/// A big decimal type.
///
#[derive(Clone, Eq)]
pub struct BigDecimal {
    int_val: BigInt,
    // A positive scale means a negative power of 10
    scale: i64,
}

#[cfg(not(feature = "std"))]
// f64::exp2 is only available in std, we have to use an external crate like libm
fn exp2(x: f64) -> f64 {
    libm::exp2(x)
}

#[cfg(feature = "std")]
fn exp2(x: f64) -> f64 {
    x.exp2()
}

impl BigDecimal {
    /// Creates and initializes a `BigDecimal`.
    ///
    /// The more explicit method `from_bigint` should be preferred, as new
    /// may change in the future.
    ///
    #[inline]
    pub fn new(digits: BigInt, scale: i64) -> BigDecimal {
        BigDecimal::from_bigint(digits, scale)
    }

    /// Construct BigDecimal from BigInt and a scale
    pub fn from_bigint(digits: BigInt, scale: i64) -> BigDecimal {
        BigDecimal {
            int_val: digits,
            scale: scale,
        }
    }

    /// Construct positive BigDecimal from BigUint and a scale
    pub fn from_biguint(digits: BigUint, scale: i64) -> BigDecimal {
        let n = BigInt::from_biguint(Sign::Plus, digits);
        BigDecimal::from_bigint(n, scale)
    }

    /// Make a BigDecimalRef of this value
    pub fn to_ref(&self) -> BigDecimalRef<'_> {
        // search for "From<&'a BigDecimal> for BigDecimalRef<'a>"
        self.into()
    }

    /// Returns the scale of the BigDecimal, the total number of
    /// digits to the right of the decimal point (including insignificant
    /// leading zeros)
    ///
    /// # Examples
    ///
    /// ```
    /// use bigdecimal::BigDecimal;
    /// use std::str::FromStr;
    ///
    /// let a = BigDecimal::from(12345);  // No fractional part
    /// let b = BigDecimal::from_str("123.45").unwrap();  // Fractional part
    /// let c = BigDecimal::from_str("0.0000012345").unwrap();  // Completely fractional part
    /// let d = BigDecimal::from_str("5e9").unwrap();  // Negative-fractional part
    ///
    /// assert_eq!(a.fractional_digit_count(), 0);
    /// assert_eq!(b.fractional_digit_count(), 2);
    /// assert_eq!(c.fractional_digit_count(), 10);
    /// assert_eq!(d.fractional_digit_count(), -9);
    /// ```
    #[inline]
    pub fn fractional_digit_count(&self) -> i64 {
        self.scale
    }

    /// Creates and initializes a `BigDecimal`.
    ///
    /// Decodes using `str::from_utf8` and forwards to `BigDecimal::from_str_radix`.
    /// Only base-10 is supported.
    ///
    /// # Examples
    ///
    /// ```
    /// use bigdecimal::{BigDecimal, Zero};
    ///
    /// assert_eq!(BigDecimal::parse_bytes(b"0", 10).unwrap(), BigDecimal::zero());
    /// assert_eq!(BigDecimal::parse_bytes(b"13", 10).unwrap(), BigDecimal::from(13));
    /// ```
    #[inline]
    pub fn parse_bytes(buf: &[u8], radix: u32) -> Option<BigDecimal> {
        stdlib::str::from_utf8(buf)
                    .ok()
                    .and_then(|s| BigDecimal::from_str_radix(s, radix).ok())
    }

    /// Return a new BigDecimal object equivalent to self, with internal
    /// scaling set to the number specified.
    /// If the new_scale is lower than the current value (indicating a larger
    /// power of 10), digits will be dropped (as precision is lower)
    ///
    #[inline]
    pub fn with_scale(&self, new_scale: i64) -> BigDecimal {
        if self.int_val.is_zero() {
            return BigDecimal::new(BigInt::zero(), new_scale);
        }

        match new_scale.cmp(&self.scale) {
            Ordering::Greater => {
                let scale_diff = new_scale - self.scale;
                let int_val = &self.int_val * ten_to_the(scale_diff as u64);
                BigDecimal::new(int_val, new_scale)
            }
            Ordering::Less => {
                let scale_diff = self.scale - new_scale;
                let int_val = &self.int_val / ten_to_the(scale_diff as u64);
                BigDecimal::new(int_val, new_scale)
            }
            Ordering::Equal => self.clone(),
        }
    }

    /// Return a new BigDecimal after shortening the digits and rounding
    ///
    /// ```
    /// # use bigdecimal::*;
    ///
    /// let n: BigDecimal = "129.41675".parse().unwrap();
    ///
    /// assert_eq!(n.with_scale_round(2, RoundingMode::Up),  "129.42".parse().unwrap());
    /// assert_eq!(n.with_scale_round(-1, RoundingMode::Down),  "120".parse().unwrap());
    /// assert_eq!(n.with_scale_round(4, RoundingMode::HalfEven),  "129.4168".parse().unwrap());
    /// ```
    pub fn with_scale_round(&self, new_scale: i64, mode: RoundingMode) -> BigDecimal {
        use stdlib::cmp::Ordering::*;

        if self.int_val.is_zero() {
            return BigDecimal::new(BigInt::zero(), new_scale);
        }

        match new_scale.cmp(&self.scale) {
            Ordering::Equal => {
                self.clone()
            }
            Ordering::Greater => {
                // increase number of zeros
                let scale_diff = new_scale - self.scale;
                let int_val = &self.int_val * ten_to_the(scale_diff as u64);
                BigDecimal::new(int_val, new_scale)
            }
            Ordering::Less => {
                let (sign, mut digits) = self.int_val.to_radix_le(10);

                let digit_count = digits.len();
                let int_digit_count = digit_count as i64 - self.scale;
                let rounded_int = match int_digit_count.cmp(&-new_scale) {
                    Equal => {
                        let (&last_digit, remaining) = digits.split_last().unwrap();
                        let trailing_zeros = remaining.iter().all(Zero::is_zero);
                        let rounded_digit = mode.round_pair(sign, (0, last_digit), trailing_zeros);
                        BigInt::new(sign, vec![rounded_digit as u32])
                    }
                    Less => {
                        debug_assert!(!digits.iter().all(Zero::is_zero));
                        let rounded_digit = mode.round_pair(sign, (0, 0), false);
                        BigInt::new(sign, vec![rounded_digit as u32])
                    }
                    Greater => {
                        // location of new rounding point
                        let scale_diff = (self.scale - new_scale) as usize;

                        let low_digit = digits[scale_diff - 1];
                        let high_digit = digits[scale_diff];
                        let trailing_zeros = digits[0..scale_diff-1].iter().all(Zero::is_zero);
                        let rounded_digit = mode.round_pair(sign, (high_digit, low_digit), trailing_zeros);

                        debug_assert!(rounded_digit <= 10);

                        if rounded_digit < 10 {
                            digits[scale_diff] = rounded_digit;
                        } else {
                            digits[scale_diff] = 0;
                            let mut i = scale_diff + 1;
                            loop {
                                if i == digit_count {
                                    digits.push(1);
                                    break;
                                }

                                if digits[i] < 9 {
                                    digits[i] += 1;
                                    break;
                                }

                                digits[i] = 0;
                                i += 1;
                            }
                        }

                        BigInt::from_radix_le(sign, &digits[scale_diff..], 10).unwrap()
                    }
                };

                BigDecimal::new(rounded_int, new_scale)
            }
        }
    }

    /// Return a new BigDecimal object with same value and given scale,
    /// padding with zeros or truncating digits as needed
    ///
    /// Useful for aligning decimals before adding/subtracting.
    ///
    fn take_and_scale(mut self, new_scale: i64) -> BigDecimal {
        self.set_scale(new_scale);
        self
    }

    /// Change to requested scale by multiplying or truncating
    fn set_scale(&mut self, new_scale: i64) {
        if self.int_val.is_zero() {
            self.scale = new_scale;
            return;
        }

        match diff(new_scale, self.scale) {
            (Ordering::Greater, scale_diff) => {
                self.scale = new_scale;
                if scale_diff < 20 {
                    self.int_val *= ten_to_the_u64(scale_diff as u8);
                } else {
                    self.int_val *= ten_to_the(scale_diff);
                }
            }
            (Ordering::Less, scale_diff) => {
                self.scale = new_scale;
                if scale_diff < 20 {
                    self.int_val /= ten_to_the_u64(scale_diff as u8);
                } else {
                    self.int_val /= ten_to_the(scale_diff);
                }
            }
            (Ordering::Equal, _) => {},
        }
    }

    /// set scale only if new_scale is greater than current
    pub(crate) fn extend_scale_to(&mut self, new_scale: i64) {
        if new_scale > self.scale {
            self.set_scale(new_scale)
        }
    }

    /// Take and return bigdecimal with the given sign
    ///
    /// The Sign value `NoSign` is ignored: only use Plus & Minus
    ///
    pub(crate) fn take_with_sign(self, sign: Sign) -> BigDecimal {
        let BigDecimal { scale, mut int_val } = self;
        if int_val.sign() != sign && sign != Sign::NoSign {
            int_val = int_val.neg();
        }
        BigDecimal {
            int_val: int_val,
            scale: scale,
        }
    }

    /// Return a new BigDecimal object with precision set to new value
    ///
    /// ```
    /// # use bigdecimal::*;
    ///
    /// let n: BigDecimal = "129.41675".parse().unwrap();
    ///
    /// assert_eq!(n.with_prec(2),  "130".parse().unwrap());
    ///
    /// let n_p12 = n.with_prec(12);
    /// let (i, scale) = n_p12.as_bigint_and_exponent();
    /// assert_eq!(n_p12, "129.416750000".parse().unwrap());
    /// assert_eq!(i, 129416750000_u64.into());
    /// assert_eq!(scale, 9);
    /// ```
    pub fn with_prec(&self, prec: u64) -> BigDecimal {
        let digits = self.digits();

        match digits.cmp(&prec) {
            Ordering::Greater => {
                let diff = digits - prec;
                let p = ten_to_the(diff);
                let (mut q, r) = self.int_val.div_rem(&p);

                // check for "leading zero" in remainder term; otherwise round
                if p < 10 * &r {
                    q += get_rounding_term(&r);
                }

                BigDecimal {
                    int_val: q,
                    scale: self.scale - diff as i64,
                }
            }
            Ordering::Less => {
                let diff = prec - digits;
                BigDecimal {
                    int_val: &self.int_val * ten_to_the(diff),
                    scale: self.scale + diff as i64,
                }
            }
            Ordering::Equal => self.clone(),
        }
    }

    /// Return this BigDecimal with the given precision, rounding if needed
    #[cfg(rustc_1_46)]  // Option::zip
    #[allow(clippy::incompatible_msrv)]
    pub fn with_precision_round(&self, prec: stdlib::num::NonZeroU64, round: RoundingMode) -> BigDecimal {
        let digit_count = self.digits();
        let new_prec = prec.get().to_i64();
        let new_scale = new_prec
                        .zip(digit_count.to_i64())
                        .and_then(|(new_prec, old_prec)| new_prec.checked_sub(old_prec))
                        .and_then(|prec_diff| self.scale.checked_add(prec_diff))
                        .expect("precision overflow");

        self.with_scale_round(new_scale, round)
    }

    #[cfg(not(rustc_1_46))]
    pub fn with_precision_round(&self, prec: stdlib::num::NonZeroU64, round: RoundingMode) -> BigDecimal {
        let new_scale = self.digits().to_i64().and_then(
                            |old_prec| {
                                prec.get().to_i64().and_then(
                                    |new_prec| { new_prec.checked_sub(old_prec) })})
                            .and_then(|prec_diff| self.scale.checked_add(prec_diff))
                            .expect("precision overflow");

        self.with_scale_round(new_scale, round)
    }

    /// Return the sign of the `BigDecimal` as `num::bigint::Sign`.
    ///
    /// ```
    /// # use bigdecimal::{BigDecimal, num_bigint::Sign};
    ///
    /// fn sign_of(src: &str) -> Sign {
    ///    let n: BigDecimal = src.parse().unwrap();
    ///    n.sign()
    /// }
    ///
    /// assert_eq!(sign_of("-1"), Sign::Minus);
    /// assert_eq!(sign_of("0"),  Sign::NoSign);
    /// assert_eq!(sign_of("1"),  Sign::Plus);
    /// ```
    #[inline]
    pub fn sign(&self) -> num_bigint::Sign {
        self.int_val.sign()
    }

    /// Return the internal big integer value and an exponent. Note that a positive
    /// exponent indicates a negative power of 10.
    ///
    /// # Examples
    ///
    /// ```
    /// use bigdecimal::{BigDecimal, num_bigint::BigInt};
    ///
    /// let n: BigDecimal = "1.23456".parse().unwrap();
    /// let expected = ("123456".parse::<BigInt>().unwrap(), 5);
    /// assert_eq!(n.as_bigint_and_exponent(), expected);
    /// ```
    #[inline]
    pub fn as_bigint_and_exponent(&self) -> (BigInt, i64) {
        (self.int_val.clone(), self.scale)
    }

    /// Take BigDecimal and split into `num::BigInt` of digits, and the scale
    ///
    /// Scale is number of digits after the decimal point, can be negative.
    ///
    pub fn into_bigint_and_scale(self) -> (BigInt, i64) {
        (self.int_val, self.scale)
    }


    /// Return digits as borrowed Cow of integer digits, and its scale
    ///
    /// Scale is number of digits after the decimal point, can be negative.
    ///
    pub fn as_bigint_and_scale(&self) -> (Cow<'_, BigInt>, i64) {
        let cow_int = Cow::Borrowed(&self.int_val);
        (cow_int, self.scale)
    }

    /// Convert into the internal big integer value and an exponent. Note that a positive
    /// exponent indicates a negative power of 10.
    ///
    /// # Examples
    ///
    /// ```
    /// use bigdecimal::{BigDecimal, num_bigint::BigInt};
    ///
    /// let n: BigDecimal = "1.23456".parse().unwrap();
    /// let expected = ("123456".parse::<num_bigint::BigInt>().unwrap(), 5);
    /// assert_eq!(n.into_bigint_and_exponent(), expected);
    /// ```
    #[inline]
    pub fn into_bigint_and_exponent(self) -> (BigInt, i64) {
        (self.int_val, self.scale)
    }

    /// Number of digits in the non-scaled integer representation
    ///
    #[inline]
    pub fn digits(&self) -> u64 {
        count_decimal_digits(&self.int_val)
    }

    /// Compute the absolute value of number
    ///
    /// ```
    /// # use bigdecimal::BigDecimal;
    /// let n: BigDecimal = "123.45".parse().unwrap();
    /// assert_eq!(n.abs(), "123.45".parse().unwrap());
    ///
    /// let n: BigDecimal = "-123.45".parse().unwrap();
    /// assert_eq!(n.abs(), "123.45".parse().unwrap());
    /// ```
    #[inline]
    pub fn abs(&self) -> BigDecimal {
        BigDecimal {
            int_val: self.int_val.abs(),
            scale: self.scale,
        }
    }

    /// Multiply decimal by 2 (efficiently)
    ///
    /// ```
    /// # use bigdecimal::BigDecimal;
    /// let n: BigDecimal = "123.45".parse().unwrap();
    /// assert_eq!(n.double(), "246.90".parse().unwrap());
    /// ```
    pub fn double(&self) -> BigDecimal {
        if self.is_zero() {
            self.clone()
        } else {
            BigDecimal {
                int_val: self.int_val.clone() * 2,
                scale: self.scale,
            }
        }
    }

    /// Divide decimal by 2 (efficiently)
    ///
    /// *Note*: If the last digit in the decimal is odd, the precision
    ///         will increase by 1
    ///
    /// ```
    /// # use bigdecimal::BigDecimal;
    /// let n: BigDecimal = "123.45".parse().unwrap();
    /// assert_eq!(n.half(), "61.725".parse().unwrap());
    /// ```
    #[inline]
    pub fn half(&self) -> BigDecimal {
        if self.is_zero() {
            self.clone()
        } else if self.int_val.is_even() {
            BigDecimal {
                int_val: self.int_val.clone().div(2u8),
                scale: self.scale,
            }
        } else {
            BigDecimal {
                int_val: self.int_val.clone().mul(5u8),
                scale: self.scale + 1,
            }
        }
    }

    /// Square a decimal: *x²*
    ///
    /// No rounding or truncating of digits; this is the full result
    /// of the squaring operation.
    ///
    /// *Note*: doubles the scale of bigdecimal, which might lead to
    ///         accidental exponential-complexity if used in a loop.
    ///
    /// ```
    /// # use bigdecimal::BigDecimal;
    /// let n: BigDecimal = "1.1156024145937225657484".parse().unwrap();
    /// assert_eq!(n.square(), "1.24456874744734405154288399835406316085210256".parse().unwrap());
    ///
    /// let n: BigDecimal = "-9.238597585E+84".parse().unwrap();
    /// assert_eq!(n.square(), "8.5351685337567832225E+169".parse().unwrap());
    /// ```
    pub fn square(&self) -> BigDecimal {
        if self.is_zero() || self.is_one() {
            self.clone()
        } else {
            BigDecimal {
                int_val: self.int_val.clone() * &self.int_val,
                scale: self.scale * 2,
            }
        }
    }

    /// Cube a decimal: *x³*
    ///
    /// No rounding or truncating of digits; this is the full result
    /// of the cubing operation.
    ///
    /// *Note*: triples the scale of bigdecimal, which might lead to
    ///         accidental exponential-complexity if used in a loop.
    ///
    /// ```
    /// # use bigdecimal::BigDecimal;
    /// let n: BigDecimal = "1.1156024145937225657484".parse().unwrap();
    /// assert_eq!(n.cube(), "1.388443899780141911774491376394890472130000455312878627147979955904".parse().unwrap());
    ///
    /// let n: BigDecimal = "-9.238597585E+84".parse().unwrap();
    /// assert_eq!(n.cube(), "-7.88529874035334084567570176625E+254".parse().unwrap());
    /// ```
    pub fn cube(&self) -> BigDecimal {
        if self.is_zero() || self.is_one() {
            self.clone()
        } else {
            BigDecimal {
                int_val: self.int_val.clone() * &self.int_val * &self.int_val,
                scale: self.scale * 3,
            }
        }
    }

    /// Take the square root of the number
    ///
    /// Uses default-precision, set from build time environment variable
    //// `RUST_BIGDECIMAL_DEFAULT_PRECISION` (defaults to 100)
    ///
    /// If the value is < 0, None is returned
    ///
    /// ```
    /// # use bigdecimal::BigDecimal;
    /// let n: BigDecimal = "1.1156024145937225657484".parse().unwrap();
    /// assert_eq!(n.sqrt().unwrap(), "1.056220817156016181190291268045893004363809142172289919023269377496528394924695970851558013658193913".parse().unwrap());
    ///
    /// let n: BigDecimal = "-9.238597585E+84".parse().unwrap();
    /// assert_eq!(n.sqrt(), None);
    /// ```
    #[inline]
    pub fn sqrt(&self) -> Option<BigDecimal> {
        self.sqrt_with_context(&Context::default())
    }

    /// Take the square root of the number, using context for precision and rounding
    ///
    pub fn sqrt_with_context(&self, ctx: &Context) -> Option<BigDecimal> {
        if self.is_zero() || self.is_one() {
            return Some(self.clone());
        }
        if self.is_negative() {
            return None;
        }

        let uint = self.int_val.magnitude();
        let result = arithmetic::sqrt::impl_sqrt(uint, self.scale, ctx);

        Some(result)
    }

    /// Take the cube root of the number, using default context
    ///
    #[inline]
    pub fn cbrt(&self) -> BigDecimal {
        self.cbrt_with_context(&Context::default())
    }

    /// Take cube root of self, using properties of context
    pub fn cbrt_with_context(&self, ctx: &Context) -> BigDecimal {
        if self.is_zero() || self.is_one() {
            return self.clone();
        }

        arithmetic::cbrt::impl_cbrt_int_scale(&self.int_val, self.scale, ctx)
    }

    /// Compute the reciprical of the number: x<sup>-1</sup>
    #[inline]
    pub fn inverse(&self) -> BigDecimal {
        self.inverse_with_context(&Context::default())
    }

    /// Return inverse of self, rounding with ctx
    pub fn inverse_with_context(&self, ctx: &Context) -> BigDecimal {
        if self.is_zero() || self.is_one() {
            return self.clone();
        }

        let uint = self.int_val.magnitude();
        let result = arithmetic::inverse::impl_inverse_uint_scale(uint, self.scale, ctx);

        // always copy sign
        result.take_with_sign(self.sign())
    }

    /// Return given number rounded to 'round_digits' precision after the
    /// decimal point, using default rounding mode
    ///
    /// Default rounding mode is `HalfEven`, but can be configured at compile-time
    /// by the environment variable: `RUST_BIGDECIMAL_DEFAULT_ROUNDING_MODE`
    /// (or by patching _build.rs_ )
    ///
    pub fn round(&self, round_digits: i64) -> BigDecimal {
        self.with_scale_round(round_digits, Context::default().rounding_mode())
    }

    /// Return true if this number has zero fractional part (is equal
    /// to an integer)
    ///
    #[inline]
    pub fn is_integer(&self) -> bool {
        if self.scale <= 0 {
            true
        } else {
            (self.int_val.clone() % ten_to_the(self.scale as u64)).is_zero()
        }
    }

    /// Evaluate the natural-exponential function e<sup>x</sup>
    ///
    #[inline]
    pub fn exp(&self) -> BigDecimal {
        if self.is_zero() {
            return BigDecimal::one();
        }

        let target_precision = DEFAULT_PRECISION;

        let precision = self.digits();

        let mut term = self.clone();
        let mut result = self.clone() + BigDecimal::one();
        let mut prev_result = result.clone();
        let mut factorial = BigInt::one();

        for n in 2.. {
            term *= self;
            factorial *= n;
            // ∑ term=x^n/n!
            result += impl_division(term.int_val.clone(), &factorial, term.scale, 117 + precision);

            let trimmed_result = result.with_prec(target_precision + 5);
            if prev_result == trimmed_result {
                return trimmed_result.with_prec(target_precision);
            }
            prev_result = trimmed_result;
        }
        unreachable!("Loop did not converge")
    }

    #[must_use]
    pub fn normalized(&self) -> BigDecimal {
        if self == &BigDecimal::zero() {
            return BigDecimal::zero();
        }
        let (sign, mut digits) = self.int_val.to_radix_be(10);
        let trailing_count = digits.iter().rev().take_while(|i| **i == 0).count();
        let trunc_to = digits.len() - trailing_count;
        digits.truncate(trunc_to);
        let int_val = BigInt::from_radix_be(sign, &digits, 10).unwrap();
        let scale = self.scale - trailing_count as i64;
        BigDecimal::new(int_val, scale)
    }

    //////////////////////////
    // Formatting methods

    /// Create string of decimal in standard decimal notation.
    ///
    /// Unlike standard formatter, this never prints the number in
    /// scientific notation.
    ///
    /// # Panics
    /// If the magnitude of the exponent is _very_ large, this may
    /// cause out-of-memory errors, or overflowing panics.
    ///
    /// # Examples
    /// ```
    /// # use bigdecimal::BigDecimal;
    /// let n: BigDecimal = "123.45678".parse().unwrap();
    /// assert_eq!(&n.to_plain_string(), "123.45678");
    ///
    /// let n: BigDecimal = "1e-10".parse().unwrap();
    /// assert_eq!(&n.to_plain_string(), "0.0000000001");
    /// ```
    pub fn to_plain_string(&self) -> String {
        let mut output = String::new();
        self.write_plain_string(&mut output).expect("Could not write to string");
        output
    }

    /// Write decimal value in decimal notation to the writer object.
    ///
    /// # Panics
    /// If the exponent is very large or very small, the number of
    /// this will print that many trailing or leading zeros.
    /// If exabytes, this will likely panic.
    ///
    pub fn write_plain_string<W: fmt::Write>(&self, wtr: &mut W) -> fmt::Result {
        write!(wtr, "{}", impl_fmt::FullScaleFormatter(self.to_ref()))
    }

    /// Create string of this bigdecimal in scientific notation
    ///
    /// ```
    /// # use bigdecimal::BigDecimal;
    /// let n = BigDecimal::from(12345678);
    /// assert_eq!(&n.to_scientific_notation(), "1.2345678e7");
    /// ```
    pub fn to_scientific_notation(&self) -> String {
        let mut output = String::new();
        self.write_scientific_notation(&mut output).expect("Could not write to string");
        output
    }

    /// Write bigdecimal in scientific notation to writer `w`
    pub fn write_scientific_notation<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        impl_fmt::write_scientific_notation(self, w)
    }

    /// Create string of this bigdecimal in engineering notation
    ///
    /// Engineering notation is scientific notation with the exponent
    /// coerced to a multiple of three
    ///
    /// ```
    /// # use bigdecimal::BigDecimal;
    /// let n = BigDecimal::from(12345678);
    /// assert_eq!(&n.to_engineering_notation(), "12.345678e6");
    /// ```
    ///
    pub fn to_engineering_notation(&self) -> String {
        let mut output = String::new();
        self.write_engineering_notation(&mut output).expect("Could not write to string");
        output
    }

    /// Write bigdecimal in engineering notation to writer `w`
    pub fn write_engineering_notation<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        impl_fmt::write_engineering_notation(self, w)
    }

}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseBigDecimalError {
    ParseDecimal(ParseFloatError),
    ParseInt(ParseIntError),
    ParseBigInt(ParseBigIntError),
    Empty,
    Other(String),
}

impl fmt::Display for ParseBigDecimalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseBigDecimalError::*;

        match *self {
            ParseDecimal(ref e) => e.fmt(f),
            ParseInt(ref e) => e.fmt(f),
            ParseBigInt(ref e) => e.fmt(f),
            Empty => "Failed to parse empty string".fmt(f),
            Other(ref reason) => reason[..].fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseBigDecimalError {
    fn description(&self) -> &str {
        "failed to parse bigint/biguint"
    }
}

impl From<ParseFloatError> for ParseBigDecimalError {
    fn from(err: ParseFloatError) -> ParseBigDecimalError {
        ParseBigDecimalError::ParseDecimal(err)
    }
}

impl From<ParseIntError> for ParseBigDecimalError {
    fn from(err: ParseIntError) -> ParseBigDecimalError {
        ParseBigDecimalError::ParseInt(err)
    }
}

impl From<ParseBigIntError> for ParseBigDecimalError {
    fn from(err: ParseBigIntError) -> ParseBigDecimalError {
        ParseBigDecimalError::ParseBigInt(err)
    }
}

#[allow(deprecated)] // trim_right_match -> trim_end_match
impl Hash for BigDecimal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut dec_str = self.int_val.to_str_radix(10);
        let scale = self.scale;
        let zero = self.int_val.is_zero();
        if scale > 0 && !zero {
            let mut cnt = 0;
            dec_str = dec_str
                .trim_right_matches(|x| {
                    cnt += 1;
                    x == '0' && cnt <= scale
                })
                .to_string();
        } else if scale < 0 && !zero {
            dec_str.push_str(&"0".repeat(self.scale.abs() as usize));
        }
        dec_str.hash(state);
    }
}


impl Default for BigDecimal {
    #[inline]
    fn default() -> BigDecimal {
        Zero::zero()
    }
}

impl Zero for BigDecimal {
    #[inline]
    fn zero() -> BigDecimal {
        BigDecimal::new(BigInt::zero(), 0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.int_val.is_zero()
    }
}

impl One for BigDecimal {
    #[inline]
    fn one() -> BigDecimal {
        BigDecimal::new(BigInt::one(), 0)
    }
}


fn impl_division(mut num: BigInt, den: &BigInt, mut scale: i64, max_precision: u64) -> BigDecimal {
    // quick zero check
    if num.is_zero() {
        return BigDecimal::new(num, 0);
    }

    match (num.is_negative(), den.is_negative()) {
        (true, true) => return impl_division(num.neg(), &den.neg(), scale, max_precision),
        (true, false) => return -impl_division(num.neg(), den, scale, max_precision),
        (false, true) => return -impl_division(num, &den.neg(), scale, max_precision),
        (false, false) => (),
    }

    // shift digits until numerator is larger than denominator (set scale appropriately)
    while num < *den {
        scale += 1;
        num *= 10;
    }

    // first division
    let (mut quotient, mut remainder) = num.div_rem(den);

    // division complete
    if remainder.is_zero() {
        return BigDecimal {
            int_val: quotient,
            scale: scale,
        };
    }

    let mut precision = count_decimal_digits(&quotient);

    // shift remainder by 1 decimal;
    // quotient will be 1 digit upon next division
    remainder *= 10;

    while !remainder.is_zero() && precision < max_precision {
        let (q, r) = remainder.div_rem(den);
        quotient = quotient * 10 + q;
        remainder = r * 10;

        precision += 1;
        scale += 1;
    }

    if !remainder.is_zero() {
        // round final number with remainder
        quotient += get_rounding_term(&remainder.div(den));
    }

    let result = BigDecimal::new(quotient, scale);
    // println!(" {} / {}\n = {}\n", self, other, result);
    return result;
}



impl Signed for BigDecimal {
    #[inline]
    fn abs(&self) -> BigDecimal {
        match self.sign() {
            Sign::Plus | Sign::NoSign => self.clone(),
            Sign::Minus => -self,
        }
    }

    #[inline]
    fn abs_sub(&self, other: &BigDecimal) -> BigDecimal {
        if *self <= *other {
            Zero::zero()
        } else {
            self - other
        }
    }

    #[inline]
    fn signum(&self) -> BigDecimal {
        match self.sign() {
            Sign::Plus => One::one(),
            Sign::NoSign => Zero::zero(),
            Sign::Minus => -Self::one(),
        }
    }

    #[inline]
    fn is_positive(&self) -> bool {
        self.sign() == Sign::Plus
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.sign() == Sign::Minus
    }
}

impl Sum for BigDecimal {
    #[inline]
    fn sum<I: Iterator<Item = BigDecimal>>(iter: I) -> BigDecimal {
        iter.fold(Zero::zero(), |a, b| a + b)
    }
}

impl<'a> Sum<&'a BigDecimal> for BigDecimal {
    #[inline]
    fn sum<I: Iterator<Item = &'a BigDecimal>>(iter: I) -> BigDecimal {
        iter.fold(Zero::zero(), |a, b| a + b)
    }
}


/// Immutable big-decimal, referencing a borrowed buffer of digits
///
/// The non-digit information like `scale` and `sign` may be changed
/// on these objects, which otherwise would require cloning the full
/// digit buffer in the BigDecimal.
///
/// Built from full `BigDecimal` object using the `to_ref()` method.
/// `BigDecimal` not implement `AsRef`, so we will reserve the method
/// `as_ref()` for a later time.
///
/// May be transformed into full BigDecimal object using the `to_owned()`
/// method.
/// This clones the bigdecimal digits.
///
/// BigDecimalRef (or `Into<BigDecimalRef>`) should be preferred over
/// using `&BigDecimal` for library functions that need an immutable
/// reference to a bigdecimal, as it may be much more efficient.
///
/// NOTE: Using `&BigDecimalRef` is redundant, and not recommended.
///
/// ## Examples
///
/// ```
/// # use bigdecimal::*; use std::ops::Neg;
/// fn add_one<'a, N: Into<BigDecimalRef<'a>>>(n: N) -> BigDecimal {
///     n.into() + 1
/// }
///
/// let n: BigDecimal = "123.456".parse().unwrap();
///
/// // call via "standard" reference (implements Into)
/// let m = add_one(&n);
/// assert_eq!(m, "124.456".parse().unwrap());
///
/// // call by negating the reference (fast: no-digit cloning involved)
/// let m = add_one(n.to_ref().neg());
/// assert_eq!(m, "-122.456".parse().unwrap());
/// ```
///
#[derive(Clone, Copy, Debug, Eq)]
pub struct BigDecimalRef<'a> {
    sign: Sign,
    digits: &'a BigUint,
    scale: i64,
}

impl BigDecimalRef<'_> {
    /// Clone digits to make this reference a full BigDecimal object
    pub fn to_owned(&self) -> BigDecimal {
        BigDecimal {
            scale: self.scale,
            int_val: BigInt::from_biguint(self.sign, self.digits.clone()),
        }
    }

    /// Clone digits, returning BigDecimal with given scale
    ///
    /// ```
    /// # use bigdecimal::*;
    ///
    /// let n: BigDecimal = "123.45678".parse().unwrap();
    /// let r = n.to_ref();
    /// assert_eq!(r.to_owned_with_scale(5), n.clone());
    /// assert_eq!(r.to_owned_with_scale(0), "123".parse().unwrap());
    /// assert_eq!(r.to_owned_with_scale(-1), "12e1".parse().unwrap());
    ///
    /// let x = r.to_owned_with_scale(8);
    /// assert_eq!(&x, &n);
    /// assert_eq!(x.fractional_digit_count(), 8);
    /// ```
    pub fn to_owned_with_scale(&self, scale: i64) -> BigDecimal {
        use stdlib::cmp::Ordering::*;

        let digits = match arithmetic::diff(self.scale, scale) {
            (Equal, _) => self.digits.clone(),
            (Less, scale_diff) => {
                if scale_diff < 20 {
                    self.digits * ten_to_the_u64(scale_diff as u8)
                } else {
                    self.digits * ten_to_the_uint(scale_diff)
                }
            }
            (Greater, scale_diff) => {
                if scale_diff < 20 {
                    self.digits / ten_to_the_u64(scale_diff as u8)
                } else {
                    self.digits / ten_to_the_uint(scale_diff)
                }
            }
        };

        BigDecimal {
            scale: scale,
            int_val: BigInt::from_biguint(self.sign, digits),
        }
    }

    /// Borrow digits as Cow
    pub(crate) fn to_cow_biguint_and_scale(&self) -> (Cow<'_, BigUint>, i64) {
        let cow_int = Cow::Borrowed(self.digits);
        (cow_int, self.scale)
    }

    /// Sign of decimal
    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Return number of digits 'right' of the decimal point
    /// (including leading zeros)
    pub fn fractional_digit_count(&self) -> i64 {
        self.scale
    }

    /// Count total number of decimal digits
    pub fn count_digits(&self) -> u64 {
        count_decimal_digits_uint(self.digits)
    }

    /// Return the number of trailing zeros in the referenced integer
    #[allow(dead_code)]
    fn count_trailing_zeroes(&self) -> usize {
        if self.digits.is_zero() || self.digits.is_odd() {
            return 0;
        }

        let digit_pairs = self.digits.to_radix_le(100);
        let loc =  digit_pairs.iter().position(|&d| d != 0).unwrap_or(0);

        2 * loc + usize::from(digit_pairs[loc] % 10 == 0)
    }

    /// Split into components
    pub(crate) fn as_parts(&self) -> (Sign, i64, &BigUint) {
        (self.sign, self.scale, self.digits)
    }

    /// Take absolute value of the decimal (non-negative sign)
    pub fn abs(&self) -> Self {
        Self {
            sign: self.sign * self.sign,
            digits: self.digits,
            scale: self.scale,
        }
    }

    /// Create BigDecimal from this reference, rounding to precision and
    /// with rounding-mode of the given context
    ///
    ///
    pub fn round_with_context(&self, ctx: &Context) -> BigDecimal {
        ctx.round_decimal_ref(*self)
    }

    /// Take square root of this number
    pub fn sqrt_with_context(&self, ctx: &Context) -> Option<BigDecimal> {
        use Sign::*;

        let (sign, scale, uint) = self.as_parts();

        match sign {
            Minus => None,
            NoSign => Some(Zero::zero()),
            Plus => Some(arithmetic::sqrt::impl_sqrt(uint, scale, ctx)),
        }
    }

    /// Take square root of absolute-value of the number
    pub fn sqrt_abs_with_context(&self, ctx: &Context) -> BigDecimal {
        use Sign::*;

        let (_, scale, uint) = self.as_parts();
        arithmetic::sqrt::impl_sqrt(uint, scale, ctx)
    }

    /// Take square root, copying sign of the initial decimal
    pub fn sqrt_copysign_with_context(&self, ctx: &Context) -> BigDecimal {
        use Sign::*;

        let (sign, scale, uint) = self.as_parts();
        let mut result = arithmetic::sqrt::impl_sqrt(uint, scale, ctx);
        if sign == Minus {
            result.int_val = result.int_val.neg();
        }
        result
    }

    /// Return if the referenced decimal is zero
    pub fn is_zero(&self) -> bool {
        self.digits.is_zero()
    }

    /// Clone this value into dest
    pub fn clone_into(&self, dest: &mut BigDecimal) {
        dest.int_val = num_bigint::BigInt::from_biguint(self.sign, self.digits.clone());
        dest.scale = self.scale;
    }
}

impl<'a> From<&'a BigDecimal> for BigDecimalRef<'a> {
    fn from(n: &'a BigDecimal) -> Self {
        let sign = n.int_val.sign();
        let mag = n.int_val.magnitude();
        Self {
            sign: sign,
            digits: mag,
            scale: n.scale,
        }
    }
}

impl<'a> From<&'a BigInt> for BigDecimalRef<'a> {
    fn from(n: &'a BigInt) -> Self {
        Self {
            sign: n.sign(),
            digits: n.magnitude(),
            scale: 0,
        }
    }
}


/// pair i64 'scale' with some other value
#[derive(Clone, Copy)]
struct WithScale<T> {
    pub value: T,
    pub scale: i64,
}

impl<T: fmt::Debug> fmt::Debug for WithScale<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(scale={} {:?})", self.scale, self.value)
    }
}

impl<T> From<(T, i64)> for WithScale<T> {
    fn from(pair: (T, i64)) -> Self {
        Self { value: pair.0, scale: pair.1 }
    }
}

impl<'a> From<WithScale<&'a BigInt>> for BigDecimalRef<'a> {
    fn from(obj: WithScale<&'a BigInt>) -> Self {
        Self {
            scale: obj.scale,
            sign: obj.value.sign(),
            digits: obj.value.magnitude(),
        }
    }
}

impl<'a> From<WithScale<&'a BigUint>> for BigDecimalRef<'a> {
    fn from(obj: WithScale<&'a BigUint>) -> Self {
        Self {
            scale: obj.scale,
            sign: Sign::Plus,
            digits: obj.value,
        }
    }
}

impl<T: Zero> WithScale<&T> {
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}


#[rustfmt::skip]
#[cfg(test)]
#[allow(non_snake_case)]
mod bigdecimal_tests {
    use crate::{stdlib, BigDecimal, ToString, FromStr, TryFrom};
    use num_traits::{ToPrimitive, FromPrimitive, Signed, Zero, One};
    use num_bigint;
    use paste::paste;


    mod from_biguint {
        use super::*;
        use num_bigint::BigUint;
        use num_bigint::Sign;

        macro_rules! impl_case {
            ($name:ident; $i:literal; $scale:literal) => {
                impl_case!($name; $i.into(); $scale; Plus);
            };
            ($name:ident; $i:expr; $scale:literal; $sign:ident) => {
                #[test]
                fn $name() {
                    let i: BigUint = $i;
                    let d = BigDecimal::from_biguint(i.clone(), $scale);
                    assert_eq!(d.int_val.magnitude(), &i);
                    assert_eq!(d.scale, $scale);
                    assert_eq!(d.sign(), Sign::$sign);
                }
            };
        }

        impl_case!(case_0en3; BigUint::zero(); 3; NoSign);
        impl_case!(case_30e2; 30u8; -2);
        impl_case!(case_7446124798en5; 7446124798u128; 5);
    }

    #[test]
    fn test_fractional_digit_count() {
        // Zero value
        let vals = BigDecimal::from(0);
        assert_eq!(vals.fractional_digit_count(), 0);
        assert_eq!(vals.to_ref().fractional_digit_count(), 0);

        // Fractional part with trailing zeros
        let vals = BigDecimal::from_str("1.0").unwrap();
        assert_eq!(vals.fractional_digit_count(), 1);
        assert_eq!(vals.to_ref().fractional_digit_count(), 1);

        // Fractional part
        let vals = BigDecimal::from_str("1.23").unwrap();
        assert_eq!(vals.fractional_digit_count(), 2);
        assert_eq!(vals.to_ref().fractional_digit_count(), 2);

        // shifted to 'left' has negative scale
        let vals = BigDecimal::from_str("123e5").unwrap();
        assert_eq!(vals.fractional_digit_count(), -5);
        assert_eq!(vals.to_ref().fractional_digit_count(), -5);
    }

    #[test]
    fn test_sum() {
        let vals = vec![
            BigDecimal::from_f32(2.5).unwrap(),
            BigDecimal::from_f32(0.3).unwrap(),
            BigDecimal::from_f32(0.001).unwrap(),
        ];

        let expected_sum = BigDecimal::from_str("2.801000011968426406383514404296875").unwrap();
        let sum = vals.iter().sum::<BigDecimal>();

        assert_eq!(expected_sum, sum);
    }

    #[test]
    fn test_sum1() {
        let vals = vec![
            BigDecimal::from_f32(0.1).unwrap(),
            BigDecimal::from_f32(0.2).unwrap(),
        ];

        let expected_sum = BigDecimal::from_str("0.300000004470348358154296875").unwrap();
        let sum = vals.iter().sum::<BigDecimal>();

        assert_eq!(expected_sum, sum);
    }

    #[test]
    fn test_to_i64() {
        let vals = vec![
            ("12.34", 12),
            ("3.14", 3),
            ("50", 50),
            ("50000", 50000),
            ("0.001", 0),
            // TODO: Is the desired behaviour to round?
            //("0.56", 1),
        ];
        for (s, ans) in vals {
            let calculated = BigDecimal::from_str(s).unwrap().to_i64().unwrap();

            assert_eq!(ans, calculated);
        }
    }

    #[test]
    fn test_to_i128() {
        let vals = vec![
            ("170141183460469231731687303715884105727", 170141183460469231731687303715884105727),
            ("-170141183460469231731687303715884105728", -170141183460469231731687303715884105728),
            ("12.34", 12),
            ("3.14", 3),
            ("-123.90", -123),
            ("50", 50),
            ("0.001", 0),
        ];
        for (s, ans) in vals {
            let calculated = BigDecimal::from_str(s).unwrap().to_i128();

            assert_eq!(Some(ans), calculated);
        }
    }

    #[test]
    fn test_to_u128() {
        let vals = vec![
            ("340282366920938463463374607431768211455", 340282366920938463463374607431768211455),
            ("12.34", 12),
            ("3.14", 3),
            ("50", 50),
            ("0.001", 0),
        ];
        for (s, ans) in vals {
            let calculated = BigDecimal::from_str(s).unwrap().to_u128().unwrap();

            assert_eq!(ans, calculated);
        }
    }

    #[test]
    fn test_from_i8() {
        let vals = vec![
            ("0", 0),
            ("1", 1),
            ("12", 12),
            ("-13", -13),
            ("111", 111),
            ("-128", i8::MIN),
            ("127", i8::MAX),
        ];
        for (s, n) in vals {
            let expected = BigDecimal::from_str(s).unwrap();
            let value = BigDecimal::from_i8(n).unwrap();
            assert_eq!(expected, value);
        }
    }

    #[test]
    fn test_from_f32() {
        let vals = vec![
            ("0.0", 0.0),
            ("1.0", 1.0),
            ("0.5", 0.5),
            ("0.25", 0.25),
            ("50.", 50.0),
            ("50000", 50000.),
            ("0.001000000047497451305389404296875", 0.001),
            ("12.340000152587890625", 12.34),
            ("0.15625", 0.15625),
            ("3.1415927410125732421875", stdlib::f32::consts::PI),
            ("31415.927734375", stdlib::f32::consts::PI * 10000.0),
            ("94247.78125", stdlib::f32::consts::PI * 30000.0),
            ("1048576", 1048576.),
        ];
        for (s, n) in vals {
            let expected = BigDecimal::from_str(s).unwrap();
            let value = BigDecimal::from_f32(n).unwrap();
            assert_eq!(expected, value);
        }
    }

    #[test]
    fn test_from_f64() {
        let vals = vec![
            ("1.0", 1.0f64),
            ("0.5", 0.5),
            ("50", 50.),
            ("50000", 50000.),
            ("0.001000000000000000020816681711721685132943093776702880859375", 0.001),
            ("0.25", 0.25),
            ("12.339999999999999857891452847979962825775146484375", 12.34),
            ("0.15625", 5.0 * 0.03125),
            ("0.333333333333333314829616256247390992939472198486328125", 1.0 / 3.0),
            ("3.141592653589793115997963468544185161590576171875", stdlib::f64::consts::PI),
            ("31415.926535897931898944079875946044921875", stdlib::f64::consts::PI * 10000.0f64),
            ("94247.779607693795696832239627838134765625", stdlib::f64::consts::PI * 30000.0f64),
        ];
        for (s, n) in vals {
            let expected = BigDecimal::from_str(s).unwrap();
            let value = BigDecimal::from_f64(n).unwrap();
            assert_eq!(expected, value);
            // assert_eq!(expected, n);
        }
    }

    #[test]
    fn test_nan_float() {
        assert!(BigDecimal::try_from(f32::NAN).is_err());
        assert!(BigDecimal::try_from(f64::NAN).is_err());
    }

    mod equals {
        use super::*;

        macro_rules! impl_case {
            ($name:ident: $input_a:literal == $input_b:literal) => {
                #[test]
                fn $name() {
                    let a: BigDecimal = $input_a.parse().unwrap();
                    let b: BigDecimal = $input_b.parse().unwrap();
                    assert_eq!(&a, &b);
                    assert_eq!(a.clone(), b.clone());
                }
            };
            ($name:ident: $input_a:literal != $input_b:literal) => {
                #[test]
                fn $name() {
                    let a: BigDecimal = $input_a.parse().unwrap();
                    let b: BigDecimal = $input_b.parse().unwrap();
                    assert_ne!(&a, &b);
                    assert_ne!(a.clone(), b.clone());
                }
            };
        }

        impl_case!(case_2: "2" == ".2e1");
        impl_case!(case_0e1: "0e1" == "0.0");
        impl_case!(case_n0: "-0" == "0.0");
        impl_case!(case_n901d3: "-901.3" == "-0.901300e+3");
        impl_case!(case_n0901300en3: "-901.3" == "-0901300e-3");
        impl_case!(case_2123121e1231: "2123121e1231" == "212.3121e1235");

        impl_case!(case_ne_2: "2" != ".2e2");
        impl_case!(case_ne_1e45: "1e45" != "1e-900");
        impl_case!(case_ne_1e900: "1e+900" != "1e-900");
    }

    #[test]
    fn test_hash_equal() {
        use stdlib::DefaultHasher;
        use stdlib::hash::{Hash, Hasher};

        fn hash<T>(obj: &T) -> u64
            where T: Hash
        {
            let mut hasher = DefaultHasher::new();
            obj.hash(&mut hasher);
            hasher.finish()
        }

        let vals = vec![
            ("1.1234", "1.1234000"),
            ("1.12340000", "1.1234"),
            ("001.1234", "1.1234000"),
            ("001.1234", "0001.1234"),
            ("1.1234000000", "1.1234000"),
            ("1.12340", "1.1234000000"),
            ("-0901300e-3", "-901.3"),
            ("-0.901300e+3", "-901.3"),
            ("100", "100.00"),
            ("100.00", "100"),
            ("0.00", "0"),
            ("0.00", "0.000"),
            ("-0.00", "0.000"),
            ("0.00", "-0.000"),
        ];
        for &(x,y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap();
            let b = BigDecimal::from_str(y).unwrap();
            assert_eq!(a, b);
            assert_eq!(hash(&a), hash(&b), "hash({}) != hash({})", a, b);
        }
    }

    #[test]
    fn test_hash_not_equal() {
        use stdlib::DefaultHasher;
        use stdlib::hash::{Hash, Hasher};

        fn hash<T>(obj: &T) -> u64
            where T: Hash
        {
            let mut hasher = DefaultHasher::new();
            obj.hash(&mut hasher);
            hasher.finish()
        }

        let vals = vec![
            ("1.1234", "1.1234001"),
            ("10000", "10"),
            ("10", "10000"),
            ("10.0", "100"),
        ];
        for &(x,y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap();
            let b = BigDecimal::from_str(y).unwrap();
            assert!(a != b, "{} == {}", a, b);
            assert!(hash(&a) != hash(&b), "hash({}) == hash({})", a, b);
        }
    }

    #[test]
    fn test_hash_equal_scale() {
        use stdlib::DefaultHasher;
        use stdlib::hash::{Hash, Hasher};

        fn hash<T>(obj: &T) -> u64
            where T: Hash
        {
            let mut hasher = DefaultHasher::new();
            obj.hash(&mut hasher);
            hasher.finish()
        }

        let vals = vec![
            ("1234.5678", -2, "1200", 0),
            ("1234.5678", -2, "1200", -2),
            ("1234.5678", 0, "1234.1234", 0),
            ("1234.5678", -3, "1200", -3),
            ("-1234", -2, "-1200", 0),
        ];
        for &(x,xs,y,ys) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap().with_scale(xs);
            let b = BigDecimal::from_str(y).unwrap().with_scale(ys);
            assert_eq!(a, b);
            assert_eq!(hash(&a), hash(&b), "hash({}) != hash({})", a, b);
        }
    }

    #[test]
    fn test_with_prec() {
        let vals = vec![
            ("7", 1, "7"),
            ("7", 2, "7.0"),
            ("895", 2, "900"),
            ("8934", 2, "8900"),
            ("8934", 1, "9000"),
            ("1.0001", 5, "1.0001"),
            ("1.0001", 4, "1"),
            ("1.00009", 6, "1.00009"),
            ("1.00009", 5, "1.0001"),
            ("1.00009", 4, "1.000"),
        ];
        for &(x, p, y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap().with_prec(p);
            assert_eq!(a, BigDecimal::from_str(y).unwrap());
        }
    }


    #[test]
    fn test_digits() {
        let vals = vec![
            ("0", 1),
            ("7", 1),
            ("10", 2),
            ("8934", 4),
        ];
        for &(x, y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap();
            assert_eq!(a.digits(), y);
        }
    }

    #[test]
    fn test_get_rounding_term() {
        use num_bigint::BigInt;
        use super::get_rounding_term;
        let vals = vec![
            ("0", 0),
            ("4", 0),
            ("5", 1),
            ("10", 0),
            ("15", 0),
            ("49", 0),
            ("50", 1),
            ("51", 1),
            ("8934", 1),
            ("9999", 1),
            ("10000", 0),
            ("50000", 1),
            ("99999", 1),
            ("100000", 0),
            ("100001", 0),
            ("10000000000", 0),
            ("9999999999999999999999999999999999999999", 1),
            ("10000000000000000000000000000000000000000", 0),
        ];
        for &(x, y) in vals.iter() {
            let a = BigInt::from_str(x).unwrap();
            assert_eq!(get_rounding_term(&a), y, "{}", x);
        }
    }

    #[test]
    fn test_abs() {
        let vals = vec![
            ("10", "10"),
            ("-10", "10"),
        ];
        for &(x, y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap().abs();
            let b = BigDecimal::from_str(y).unwrap();
            assert!(a == b, "{} == {}", a, b);
        }
    }

    #[test]
    fn test_count_decimal_digits() {
        use num_bigint::BigInt;
        use super::count_decimal_digits;
        let vals = vec![
            ("10", 2),
            ("1", 1),
            ("9", 1),
            ("999", 3),
            ("1000", 4),
            ("9900", 4),
            ("9999", 4),
            ("10000", 5),
            ("99999", 5),
            ("100000", 6),
            ("999999", 6),
            ("1000000", 7),
            ("9999999", 7),
            ("999999999999", 12),
            ("999999999999999999999999", 24),
            ("999999999999999999999999999999999999999999999999", 48),
            ("999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999", 96),
            ("199999911199999999999999999999999999999999999999999999999999999999999999999999999999999999999000", 96),
            ("999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999991", 192),
            ("199999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999", 192),
            ("-1", 1),
            ("-6", 1),
            ("-10", 2),
            ("-999999999999999999999999", 24),
        ];
        for &(x, y) in vals.iter() {
            let a = BigInt::from_str(x).unwrap();
            let b = count_decimal_digits(&a);
            assert_eq!(b, y);
        }
    }

    #[test]
    fn test_half() {
        let vals = vec![
            ("100", "50."),
            ("2", "1"),
            (".2", ".1"),
            ("42", "21"),
            ("3", "1.5"),
            ("99", "49.5"),
            ("3.141592653", "1.5707963265"),
            ("3.1415926536", "1.5707963268"),
        ];
        for &(x, y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap().half();
            let b = BigDecimal::from_str(y).unwrap();
            assert_eq!(a, b);
            assert_eq!(a.scale, b.scale);
        }
    }

    #[test]
    fn test_round() {
        let test_cases = vec![
            ("1.45", 1, "1.4"),
            ("1.444445", 1, "1.4"),
            ("1.44", 1, "1.4"),
            ("0.444", 2, "0.44"),
            ("4.5", 0, "4"),
            ("4.05", 1, "4.0"),
            ("4.050", 1, "4.0"),
            ("4.15", 1, "4.2"),
            ("0.0045", 2, "0.00"),
            ("5.5", -1, "10"),
            ("-1.555", 2, "-1.56"),
            ("-1.555", 99, "-1.555"),
            ("5.5", 0, "6"),
            ("-1", -1, "0"),
            ("5", -1, "0"),
            ("44", -1, "40"),
            ("44", -99, "0"),
            ("44", 99, "44"),
            ("1.4499999999", -1, "0"),
            ("1.4499999999", 0, "1"),
            ("1.4499999999", 1, "1.4"),
            ("1.4499999999", 2, "1.45"),
            ("1.4499999999", 3, "1.450"),
            ("1.4499999999", 4, "1.4500"),
            ("1.4499999999", 10, "1.4499999999"),
            ("1.4499999999", 15, "1.449999999900000"),
            ("-1.4499999999", 1, "-1.4"),
            ("1.449999999", 1, "1.4"),
            ("-9999.444455556666", 10, "-9999.4444555567"),
            ("-12345678987654321.123456789", 8, "-12345678987654321.12345679"),
            ("0.33333333333333333333333333333333333333333333333333333333333333333333333333333333333333", 0, "0"),
            ("0.1165085714285714285714285714285714285714", 0, "0"),
            ("0.1165085714285714285714285714285714285714", 2, "0.12"),
            ("0.1165085714285714285714285714285714285714", 5, "0.11651"),
            ("0.1165085714285714285714285714285714285714", 8, "0.11650857"),
            ("-1.5", 0, "-2"),
            ("-1.2", 0, "-1"),
            ("-0.68", 0, "-1"),
            ("-0.5", 0, "0"),
            ("-0.49", 0, "0"),
        ];
        for &(x, digits, y) in test_cases.iter() {
            let a = BigDecimal::from_str(x).unwrap();
            let b = BigDecimal::from_str(y).unwrap();
            let rounded = a.round(digits);
            assert_eq!(rounded, b);
        }
    }

    #[test]
    fn round_large_number() {
        use super::BigDecimal;

        let z = BigDecimal::from_str("3.4613133327063255443352353815722045816611958409944513040035462804475524").unwrap();
        let expected = BigDecimal::from_str("11.9806899871705702711783103817684242408972124568942276285200973527647213").unwrap();
        let zsq = &z*&z;
        let zsq = zsq.round(70);
        debug_assert_eq!(zsq, expected);
    }

    #[test]
    fn test_is_integer() {
        let true_vals = vec![
            "100",
            "100.00",
            "1724e4",
            "31.47e8",
            "-31.47e8",
            "-0.0",
        ];

        let false_vals = vec![
            "100.1",
            "0.001",
            "3147e-3",
            "3147e-8",
            "-0.01",
            "-1e-3",
        ];

        for s in true_vals {
            let d = BigDecimal::from_str(s).unwrap();
            assert!(d.is_integer());
        }

        for s in false_vals {
            let d = BigDecimal::from_str(s).unwrap();
            assert!(!d.is_integer());
        }
    }

    #[test]
    fn test_inverse() {
        let vals = vec![
            ("100", "0.01"),
            ("2", "0.5"),
            (".2", "5"),
            ("3.141592653", "0.3183098862435492205742690218851870990799646487459493049686604293188738877535183744268834079171116523"),
        ];
        for &(x, y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap();
            let i = a.inverse();
            let b = BigDecimal::from_str(y).unwrap();
            assert_eq!(i, b);
            assert_eq!(BigDecimal::from(1)/&a, b);
            assert_eq!(i.inverse(), a);
            // assert_eq!(a.scale, b.scale, "scale mismatch ({} != {}", a, b);
        }
    }

    mod double {
        use super::*;

        include!("lib.tests.double.rs");
    }

    #[test]
    fn test_square() {
        let vals = vec![
            ("1.00", "1.00"),
            ("1.5", "2.25"),
            ("1.50", "2.2500"),
            ("5", "25"),
            ("5.0", "25.00"),
            ("-5.0", "25.00"),
            ("5.5", "30.25"),
            ("0.80", "0.6400"),
            ("0.01234", "0.0001522756"),
            ("3.1415926", "9.86960406437476"),
        ];
        for &(x, y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap().square();
            let b = BigDecimal::from_str(y).unwrap();
            assert_eq!(a, b);
            assert_eq!(a.scale, b.scale);
        }
    }

    #[test]
    fn test_cube() {
        let vals = vec![
            ("1.00", "1.00"),
            ("1.50", "3.375000"),
            ("5", "125"),
            ("5.0", "125.000"),
            ("5.00", "125.000000"),
            ("-5", "-125"),
            ("-5.0", "-125.000"),
            ("2.01", "8.120601"),
            ("5.5", "166.375"),
            ("0.01234", "0.000001879080904"),
            ("3.1415926", "31.006275093569669642776"),
        ];
        for &(x, y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap().cube();
            let b = BigDecimal::from_str(y).unwrap();
            assert_eq!(a, b);
            assert_eq!(a.scale, b.scale);
        }
    }

    #[test]
    fn test_exp() {
        let vals = vec![
            ("0", "1"),
            ("1", "2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427"),
            ("1.01", "2.745601015016916493989776316660387624073750819595962291667398087987297168243899027802501018008905180"),
            ("0.5", "1.648721270700128146848650787814163571653776100710148011575079311640661021194215608632776520056366643"),
            ("-1", "0.3678794411714423215955237701614608674458111310317678345078368016974614957448998033571472743459196437"),
            ("-0.01", "0.9900498337491680535739059771800365577720790812538374668838787452931477271687452950182155307793838110"),
            ("-10.04", "0.00004361977305405268676261569570537884674661515701779752139657120453194647205771372804663141467275928595"),
            //("-1000.04", "4.876927702336787390535723208392195312680380995235400234563172353460484039061383367037381490416091595E-435"),
            ("-20.07", "1.921806899438469499721914055500607234723811054459447828795824348465763824284589956630853464778332349E-9"),
            ("10", "22026.46579480671651695790064528424436635351261855678107423542635522520281857079257519912096816452590"),
            ("20", "485165195.4097902779691068305415405586846389889448472543536108003159779961427097401659798506527473494"),
            //("777.7", "5.634022488451236612534495413455282583175841288248965283178668787259870456538271615076138061788051442E+337"),
        ];
        for &(x, y) in vals.iter() {
            let a = BigDecimal::from_str(x).unwrap().exp();
            let b = BigDecimal::from_str(y).unwrap();
            assert_eq!(a, b);
        }
    }

    mod to_plain_string {
        use super::*;

        macro_rules! impl_test {
            ($name:ident: $input:literal => $expected:literal) => {
                #[test]
                fn $name() {
                    let n: BigDecimal = $input.parse().unwrap();
                    let s = n.to_plain_string();
                    assert_eq!(&s, $expected);
                }
            };
        }

        impl_test!(case_zero: "0" => "0");
        impl_test!(case_1en18: "1e-18" => "0.000000000000000001");
        impl_test!(case_n72e4: "-72e4" => "-720000");
        impl_test!(case_95517338e30: "95517338e30" => "95517338000000000000000000000000000000");
        impl_test!(case_29478en30: "29478e-30" => "0.000000000000000000000000029478");
        impl_test!(case_30740d4897: "30740.4897" => "30740.4897");
    }

    #[test]
    fn test_signed() {
        assert!(!BigDecimal::zero().is_positive());
        assert!(!BigDecimal::one().is_negative());

        assert!(BigDecimal::one().is_positive());
        assert!((-BigDecimal::one()).is_negative());
        assert!((-BigDecimal::one()).abs().is_positive());
    }

    mod normalize {
        use super::*;

        macro_rules! impl_case {
            ( $name:ident: ($i:literal, $s:literal) =>  ($e_int_val:literal, $e_scale:literal) ) => {
                #[test]
                fn $name() {
                    let d = BigDecimal::new($i.into(), $s);
                    let n = d.normalized();
                    assert_eq!(n.int_val, $e_int_val.into());
                    assert_eq!(n.scale, $e_scale);
                }
            }
        }

        impl_case!(case_0e3: (0, -3) => (0, 0));
        impl_case!(case_0en50: (0, 50) => (0, 0));
        impl_case!(case_10en2: (10, 2) => (1, 1));
        impl_case!(case_11en2: (11, 2) => (11, 2));
        impl_case!(case_132400en4: (132400, 4) => (1324, 2));
        impl_case!(case_1_900_000en3: (1_900_000, 3) => (19, -2));
        impl_case!(case_834700e4: (834700, -4) => (8347, -6));
        impl_case!(case_n834700e4: (-9900, 2) => (-99, 0));
    }

    #[test]
    fn test_from_i128() {
        let value = BigDecimal::from_i128(-368934881474191032320).unwrap();
        let expected = BigDecimal::from_str("-368934881474191032320").unwrap();
        assert_eq!(value, expected);
    }

    #[test]
    fn test_from_u128() {
        let value = BigDecimal::from_u128(668934881474191032320).unwrap();
        let expected = BigDecimal::from_str("668934881474191032320").unwrap();
        assert_eq!(value, expected);
    }

    #[test]
    fn test_parse_roundtrip() {
        let vals = vec![
            "1.0",
            "0.5",
            "50",
            "50000",
            "0.001000000000000000020816681711721685132943093776702880859375",
            "0.25",
            "12.339999999999999857891452847979962825775146484375",
            "0.15625",
            "0.333333333333333314829616256247390992939472198486328125",
            "3.141592653589793115997963468544185161590576171875",
            "31415.926535897931898944079875946044921875",
            "94247.779607693795696832239627838134765625",
            "1331.107",
            "1.0",
            "2e1",
            "0.00123",
            "-123",
            "-1230",
            "12.3",
            "123e-1",
            "1.23e+1",
            "1.23E+3",
            "1.23E-8",
            "-1.23E-10",
            "123_",
            "31_862_140.830_686_979",
            "-1_1.2_2",
            "999.521_939",
            "679.35_84_03E-2",
            "271576662.__E4",
            // Large decimals with small text representations
            "1E10000",
            "1E-10000",
            "1.129387461293874682630000000487984723987459E10000",
            "11293874612938746826340000000087984723987459E10000",
        ];
        for s in vals {
            let expected = BigDecimal::from_str(s).unwrap();
            let display = format!("{}", expected);
            let parsed = BigDecimal::from_str(&display).unwrap();
            assert_eq!(expected, parsed, "[{}] didn't round trip through [{}]", s, display);
        }
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test_with_scale_round {
    use super::*;
    use paste::paste;

    include!("lib.tests.with_scale_round.rs");
}


#[cfg(all(test, property_tests))]
extern crate proptest;

#[cfg(all(test, property_tests))]
mod proptests {
    use super::*;
    use paste::paste;
    use proptest::*;

    include!("lib.tests.property-tests.rs");
}
