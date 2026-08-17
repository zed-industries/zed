//! Immediate operands for Cranelift instructions
//!
//! This module defines the types of immediate operands that can appear on Cranelift instructions.
//! Each type here should have a corresponding definition in the
//! `cranelift-codegen/meta/src/shared/immediates` crate in the meta language.

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt::{self, Display, Formatter};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Sub};
use core::str::FromStr;
use core::{i32, u32};
use cranelift_entity::{Signed, Unsigned};
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// Convert a type into a vector of bytes; all implementors in this file must use little-endian
/// orderings of bytes to match WebAssembly's little-endianness.
pub trait IntoBytes {
    /// Return the little-endian byte representation of the implementing type.
    fn into_bytes(self) -> Vec<u8>;
}

impl IntoBytes for u8 {
    fn into_bytes(self) -> Vec<u8> {
        vec![self]
    }
}

impl IntoBytes for i8 {
    fn into_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }
}

impl IntoBytes for i16 {
    fn into_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl IntoBytes for i32 {
    fn into_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl IntoBytes for Vec<u8> {
    fn into_bytes(self) -> Vec<u8> {
        self
    }
}

/// 64-bit immediate signed integer operand.
///
/// An `Imm64` operand can also be used to represent immediate values of smaller integer types by
/// sign-extending to `i64`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Imm64(i64);

impl Imm64 {
    /// Create a new `Imm64` representing the signed number `x`.
    pub fn new(x: i64) -> Self {
        Self(x)
    }

    /// Return self negated.
    pub fn wrapping_neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }

    /// Returns the value of this immediate.
    pub fn bits(&self) -> i64 {
        self.0
    }

    /// Mask this immediate to the given power-of-two bit width.
    #[must_use]
    pub(crate) fn mask_to_width(&self, bit_width: u32) -> Self {
        debug_assert!(bit_width.is_power_of_two());

        if bit_width >= 64 {
            return *self;
        }

        let bit_width = i64::from(bit_width);
        let mask = (1 << bit_width) - 1;
        let masked = self.0 & mask;
        Imm64(masked)
    }

    /// Sign extend this immediate as if it were a signed integer of the given
    /// power-of-two width.
    #[must_use]
    pub fn sign_extend_from_width(&self, bit_width: u32) -> Self {
        debug_assert!(
            bit_width.is_power_of_two(),
            "{bit_width} is not a power of two"
        );

        if bit_width >= 64 {
            return *self;
        }

        let bit_width = i64::from(bit_width);
        let delta = 64 - bit_width;
        let sign_extended = (self.0 << delta) >> delta;
        Imm64(sign_extended)
    }

    /// Zero extend this immediate as if it were an unsigned integer of the
    /// given power-of-two width.
    #[must_use]
    pub fn zero_extend_from_width(&self, bit_width: u32) -> Self {
        debug_assert!(
            bit_width.is_power_of_two(),
            "{bit_width} is not a power of two"
        );

        if bit_width >= 64 {
            return *self;
        }

        let bit_width = u64::from(bit_width);
        let delta = 64 - bit_width;
        let zero_extended = (self.0.unsigned() << delta) >> delta;
        Imm64(zero_extended.signed())
    }
}

impl From<Imm64> for i64 {
    fn from(val: Imm64) -> i64 {
        val.0
    }
}

impl IntoBytes for Imm64 {
    fn into_bytes(self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }
}

impl From<i64> for Imm64 {
    fn from(x: i64) -> Self {
        Self(x)
    }
}

impl Display for Imm64 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let x = self.0;
        if x < 10_000 {
            // Use decimal for small and negative numbers.
            write!(f, "{x}")
        } else {
            write_hex(x as u64, f)
        }
    }
}

/// Parse a 64-bit signed number.
fn parse_i64(s: &str) -> Result<i64, &'static str> {
    let negative = s.starts_with('-');
    let s2 = if negative || s.starts_with('+') {
        &s[1..]
    } else {
        s
    };

    let mut value = parse_u64(s2)?;

    // We support the range-and-a-half from -2^63 .. 2^64-1.
    if negative {
        value = value.wrapping_neg();
        // Don't allow large negative values to wrap around and become positive.
        if value as i64 > 0 {
            return Err("Negative number too small");
        }
    }
    Ok(value as i64)
}

impl FromStr for Imm64 {
    type Err = &'static str;

    // Parse a decimal or hexadecimal `Imm64`, formatted as above.
    fn from_str(s: &str) -> Result<Self, &'static str> {
        parse_i64(s).map(Self::new)
    }
}

/// 64-bit immediate unsigned integer operand.
///
/// A `Uimm64` operand can also be used to represent immediate values of smaller integer types by
/// zero-extending to `i64`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Uimm64(u64);

impl Uimm64 {
    /// Create a new `Uimm64` representing the unsigned number `x`.
    pub fn new(x: u64) -> Self {
        Self(x)
    }

    /// Return self negated.
    pub fn wrapping_neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }
}

impl From<Uimm64> for u64 {
    fn from(val: Uimm64) -> u64 {
        val.0
    }
}

impl From<u64> for Uimm64 {
    fn from(x: u64) -> Self {
        Self(x)
    }
}

/// Hexadecimal with a multiple of 4 digits and group separators:
///
///   0xfff0
///   0x0001_ffff
///   0xffff_ffff_fff8_4400
///
fn write_hex(x: u64, f: &mut Formatter) -> fmt::Result {
    let mut pos = (64 - x.leading_zeros() - 1) & 0xf0;
    write!(f, "0x{:04x}", (x >> pos) & 0xffff)?;
    while pos > 0 {
        pos -= 16;
        write!(f, "_{:04x}", (x >> pos) & 0xffff)?;
    }
    Ok(())
}

impl Display for Uimm64 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let x = self.0;
        if x < 10_000 {
            // Use decimal for small numbers.
            write!(f, "{x}")
        } else {
            write_hex(x, f)
        }
    }
}

/// Parse a 64-bit unsigned number.
fn parse_u64(s: &str) -> Result<u64, &'static str> {
    let mut value: u64 = 0;
    let mut digits = 0;

    if s.starts_with("-0x") {
        return Err("Invalid character in hexadecimal number");
    } else if let Some(num) = s.strip_prefix("0x") {
        // Hexadecimal.
        for ch in num.chars() {
            match ch.to_digit(16) {
                Some(digit) => {
                    digits += 1;
                    if digits > 16 {
                        return Err("Too many hexadecimal digits");
                    }
                    // This can't overflow given the digit limit.
                    value = (value << 4) | u64::from(digit);
                }
                None => {
                    // Allow embedded underscores, but fail on anything else.
                    if ch != '_' {
                        return Err("Invalid character in hexadecimal number");
                    }
                }
            }
        }
    } else {
        // Decimal number, possibly negative.
        for ch in s.chars() {
            match ch.to_digit(10) {
                Some(digit) => {
                    digits += 1;
                    match value.checked_mul(10) {
                        None => return Err("Too large decimal number"),
                        Some(v) => value = v,
                    }
                    match value.checked_add(u64::from(digit)) {
                        None => return Err("Too large decimal number"),
                        Some(v) => value = v,
                    }
                }
                None => {
                    // Allow embedded underscores, but fail on anything else.
                    if ch != '_' {
                        return Err("Invalid character in decimal number");
                    }
                }
            }
        }
    }

    if digits == 0 {
        return Err("No digits in number");
    }

    Ok(value)
}

impl FromStr for Uimm64 {
    type Err = &'static str;

    // Parse a decimal or hexadecimal `Uimm64`, formatted as above.
    fn from_str(s: &str) -> Result<Self, &'static str> {
        parse_u64(s).map(Self::new)
    }
}

/// 8-bit unsigned integer immediate operand.
///
/// This is used to indicate lane indexes typically.
pub type Uimm8 = u8;

/// A 32-bit unsigned integer immediate operand.
///
/// This is used to represent sizes of memory objects.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Uimm32(u32);

impl From<Uimm32> for u32 {
    fn from(val: Uimm32) -> u32 {
        val.0
    }
}

impl From<Uimm32> for u64 {
    fn from(val: Uimm32) -> u64 {
        val.0.into()
    }
}

impl From<Uimm32> for i64 {
    fn from(val: Uimm32) -> i64 {
        i64::from(val.0)
    }
}

impl From<u32> for Uimm32 {
    fn from(x: u32) -> Self {
        Self(x)
    }
}

impl Display for Uimm32 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.0 < 10_000 {
            write!(f, "{}", self.0)
        } else {
            write_hex(u64::from(self.0), f)
        }
    }
}

impl FromStr for Uimm32 {
    type Err = &'static str;

    // Parse a decimal or hexadecimal `Uimm32`, formatted as above.
    fn from_str(s: &str) -> Result<Self, &'static str> {
        parse_i64(s).and_then(|x| {
            if 0 <= x && x <= i64::from(u32::MAX) {
                Ok(Self(x as u32))
            } else {
                Err("Uimm32 out of range")
            }
        })
    }
}

/// A 128-bit immediate operand.
///
/// This is used as an immediate value in SIMD instructions.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct V128Imm(pub [u8; 16]);

impl V128Imm {
    /// Iterate over the bytes in the constant.
    pub fn bytes(&self) -> impl Iterator<Item = &u8> {
        self.0.iter()
    }

    /// Convert the immediate into a vector.
    pub fn to_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Convert the immediate into a slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
}

impl From<&[u8]> for V128Imm {
    fn from(slice: &[u8]) -> Self {
        assert_eq!(slice.len(), 16);
        let mut buffer = [0; 16];
        buffer.copy_from_slice(slice);
        Self(buffer)
    }
}

impl From<u128> for V128Imm {
    fn from(val: u128) -> Self {
        V128Imm(val.to_le_bytes())
    }
}

/// 32-bit signed immediate offset.
///
/// This is used to encode an immediate offset for load/store instructions. All supported ISAs have
/// a maximum load/store offset that fits in an `i32`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Offset32(i32);

impl Offset32 {
    /// Create a new `Offset32` representing the signed number `x`.
    pub fn new(x: i32) -> Self {
        Self(x)
    }

    /// Create a new `Offset32` representing the signed number `x` if possible.
    pub fn try_from_i64(x: i64) -> Option<Self> {
        let x = i32::try_from(x).ok()?;
        Some(Self::new(x))
    }

    /// Add in the signed number `x` if possible.
    pub fn try_add_i64(self, x: i64) -> Option<Self> {
        let x = i32::try_from(x).ok()?;
        let ret = self.0.checked_add(x)?;
        Some(Self::new(ret))
    }
}

impl From<Offset32> for i32 {
    fn from(val: Offset32) -> i32 {
        val.0
    }
}

impl From<Offset32> for i64 {
    fn from(val: Offset32) -> i64 {
        i64::from(val.0)
    }
}

impl From<i32> for Offset32 {
    fn from(x: i32) -> Self {
        Self(x)
    }
}

impl From<u8> for Offset32 {
    fn from(val: u8) -> Offset32 {
        Self(val.into())
    }
}

impl Display for Offset32 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // 0 displays as an empty offset.
        if self.0 == 0 {
            return Ok(());
        }

        // Always include a sign.
        write!(f, "{}", if self.0 < 0 { '-' } else { '+' })?;

        let val = i64::from(self.0).abs();
        if val < 10_000 {
            write!(f, "{val}")
        } else {
            write_hex(val as u64, f)
        }
    }
}

impl FromStr for Offset32 {
    type Err = &'static str;

    // Parse a decimal or hexadecimal `Offset32`, formatted as above.
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !(s.starts_with('-') || s.starts_with('+')) {
            return Err("Offset must begin with sign");
        }
        parse_i64(s).and_then(|x| {
            if i64::from(i32::MIN) <= x && x <= i64::from(i32::MAX) {
                Ok(Self::new(x as i32))
            } else {
                Err("Offset out of range")
            }
        })
    }
}

// FIXME(rust-lang/rust#83527): Replace with `${ignore()}` once it is stabilised.
macro_rules! ignore {
    ($($t:tt)*) => {};
}

macro_rules! ieee_float {
    (
        name = $name:ident,
        bits = $bits:literal,
        significand_bits = $significand_bits:literal,
        bits_ty = $bits_ty:ident,
        float_ty = $float_ty:ident,
        $(as_float = $as_float:ident,)?
        $(rust_type_not_stable = $rust_type_not_stable:ident,)?
    ) => {
        /// An IEEE
        #[doc = concat!("binary", stringify!($bits))]
        /// immediate floating point value, represented as a
        #[doc = stringify!($bits_ty)]
        /// containing the bit pattern.
        ///
        /// We specifically avoid using a
        #[doc = stringify!($float_ty)]
        /// here since some architectures may silently alter floats.
        /// See: <https://github.com/bytecodealliance/wasmtime/pull/2251#discussion_r498508646>
        ///
        /// The [PartialEq] and [Hash] implementations are over the underlying bit pattern, but
        /// [PartialOrd] respects IEEE754 semantics.
        ///
        /// All bit patterns are allowed.
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        #[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
        #[repr(C)]
        pub struct $name {
            bits: $bits_ty
        }

        impl $name {
            const BITS: u8 = $bits;
            const SIGNIFICAND_BITS: u8 = $significand_bits;
            const EXPONENT_BITS: u8 = Self::BITS - Self::SIGNIFICAND_BITS - 1;
            const SIGN_MASK: $bits_ty = 1 << (Self::EXPONENT_BITS + Self::SIGNIFICAND_BITS);
            const SIGNIFICAND_MASK: $bits_ty = $bits_ty::MAX >> (Self::EXPONENT_BITS + 1);
            const EXPONENT_MASK: $bits_ty = !Self::SIGN_MASK & !Self::SIGNIFICAND_MASK;
            /// The positive WebAssembly canonical NaN.
            pub const NAN: Self = Self::with_bits(Self::EXPONENT_MASK | (1 << (Self::SIGNIFICAND_BITS - 1)));

            /// Create a new
            #[doc = concat!("`", stringify!($name), "`")]
            /// containing the bits of `bits`.
            pub const fn with_bits(bits: $bits_ty) -> Self {
                Self { bits }
            }

            /// Get the bitwise representation.
            pub fn bits(self) -> $bits_ty {
                self.bits
            }

            $(
                /// Create a new
                #[doc = concat!("`", stringify!($name), "`")]
                /// representing the number `x`.
                pub fn with_float(x: $float_ty) -> Self {
                    Self::with_bits(x.to_bits())
                }

                /// Converts `self` to a Rust
                #[doc = concat!("`", stringify!($float_ty), "`.")]
                pub fn $as_float(self) -> $float_ty {
                    $float_ty::from_bits(self.bits())
                }
            )?

            /// Computes the absolute value of `self`.
            pub fn abs(self) -> Self {
                Self::with_bits(self.bits() & !Self::SIGN_MASK)
            }

            /// Returns a number composed of the magnitude of `self` and the sign of `sign`.
            pub fn copysign(self, sign: Self) -> Self {
                Self::with_bits((self.bits() & !Self::SIGN_MASK) | (sign.bits() & Self::SIGN_MASK))
            }

            /// Returns the minimum of `self` and `other`, following the WebAssembly/IEEE 754-2019 definition.
            pub fn minimum(self, other: Self) -> Self {
                // FIXME: Replace with Rust float method once it is stabilised.
                if self.is_nan() || other.is_nan() {
                    Self::NAN
                } else if self.is_zero() && other.is_zero() {
                    if self.is_negative() {
                        self
                    } else {
                        other
                    }
                } else if self <= other {
                    self
                } else {
                    other
                }
            }

            /// Returns the maximum of `self` and `other`, following the WebAssembly/IEEE 754-2019 definition.
            pub fn maximum(self, other: Self) -> Self {
                // FIXME: Replace with Rust float method once it is stabilised.
                if self.is_nan() || other.is_nan() {
                    Self::NAN
                } else if self.is_zero() && other.is_zero() {
                    if self.is_positive() {
                        self
                    } else {
                        other
                    }
                } else if self >= other {
                    self
                } else {
                    other
                }
            }

            /// Create an
            #[doc = concat!("`", stringify!($name), "`")]
            /// number representing `2.0^n`.
            pub fn pow2<I: Into<i32>>(n: I) -> Self {
                let n = n.into();
                let w = Self::EXPONENT_BITS;
                let t = Self::SIGNIFICAND_BITS;
                let bias = (1 << (w - 1)) - 1;
                let exponent = n + bias;
                assert!(exponent > 0, "Underflow n={}", n);
                assert!(exponent < (1 << w) + 1, "Overflow n={}", n);
                Self::with_bits((exponent as $bits_ty) << t)
            }

            /// Create an
            #[doc = concat!("`", stringify!($name), "`")]
            /// number representing the greatest negative value not convertible from
            #[doc = concat!("`", stringify!($float_ty), "`")]
            /// to a signed integer with width n.
            pub fn fcvt_to_sint_negative_overflow<I: Into<i32>>(n: I) -> Self {
                let n = n.into();
                debug_assert!(n < i32::from(Self::BITS));
                debug_assert!(i32::from(Self::SIGNIFICAND_BITS) + 1 - n < i32::from(Self::BITS));
                Self::with_bits((1 << (Self::BITS - 1)) | Self::pow2(n - 1).bits() | (1 << (i32::from(Self::SIGNIFICAND_BITS) + 1 - n)))
            }

            /// Check if the value is a NaN. For
            #[doc = concat!("`", stringify!($name), "`,")]
            /// this means checking that all the exponent bits are set and the significand is non-zero.
            pub fn is_nan(self) -> bool {
                self.abs().bits() > Self::EXPONENT_MASK
            }

            /// Returns true if `self` has a negative sign, including 0.0, NaNs with positive sign bit and positive infinity.
            pub fn is_positive(self) -> bool {
                !self.is_negative()
            }

            /// Returns true if `self` has a negative sign, including -0.0, NaNs with negative sign bit and negative infinity.
            pub fn is_negative(self) -> bool {
                self.bits() & Self::SIGN_MASK == Self::SIGN_MASK
            }

            /// Returns `true` if `self` is positive or negative zero.
            pub fn is_zero(self) -> bool {
                self.abs().bits() == 0
            }

            /// Returns `None` if `self` is a NaN and `Some(self)` otherwise.
            pub fn non_nan(self) -> Option<Self> {
                Some(self).filter(|f| !f.is_nan())
            }

            $(
                /// Returns the square root of `self`.
                pub fn sqrt(self) -> Self {
                    Self::with_float(self.$as_float().sqrt())
                }

                /// Returns the smallest integer greater than or equal to `self`.
                pub fn ceil(self) -> Self {
                    Self::with_float(self.$as_float().ceil())
                }

                /// Returns the largest integer less than or equal to `self`.
                pub fn floor(self) -> Self {
                    Self::with_float(self.$as_float().floor())
                }

                /// Returns the integer part of `self`. This means that non-integer numbers are always truncated towards zero.
                pub fn trunc(self) -> Self {
                    Self::with_float(self.$as_float().trunc())
                }

                /// Returns the nearest integer to `self`. Rounds half-way cases to the number
                /// with an even least significant digit.
                pub fn round_ties_even(self) -> Self {
                    Self::with_float(self.$as_float().round_ties_even())
                }
            )?
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
                $(self.$as_float().partial_cmp(&rhs.$as_float()))?
                $(
                    ignore!($rust_type_not_stable);
                    // FIXME(#8312): Use builtin Rust comparisons once `f16` and `f128` support is stabalised.
                    if self.is_nan() || rhs.is_nan() {
                        // One of the floats is a NaN.
                        return None;
                    }
                    if self.is_zero() || rhs.is_zero() {
                        // Zeros are always equal regardless of sign.
                        return Some(Ordering::Equal);
                    }
                    let lhs_positive = self.is_positive();
                    let rhs_positive = rhs.is_positive();
                    if lhs_positive != rhs_positive {
                        // Different signs: negative < positive
                        return lhs_positive.partial_cmp(&rhs_positive);
                    }
                    // Finite or infinity will order correctly with an integer comparison of the bits.
                    if lhs_positive {
                        self.bits().partial_cmp(&rhs.bits())
                    } else {
                        // Reverse the comparison when both floats are negative.
                        rhs.bits().partial_cmp(&self.bits())
                    }
                )?
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                format_float(u128::from(self.bits()), Self::EXPONENT_BITS, Self::SIGNIFICAND_BITS, f)
            }
        }

        impl FromStr for $name {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, &'static str> {
                match parse_float(s, Self::EXPONENT_BITS, Self::SIGNIFICAND_BITS) {
                    Ok(b) => Ok(Self::with_bits(b.try_into().unwrap())),
                    Err(s) => Err(s),
                }
            }
        }

        impl IntoBytes for $name {
            fn into_bytes(self) -> Vec<u8> {
                self.bits().to_le_bytes().to_vec()
            }
        }

        impl Neg for $name {
            type Output = Self;

            fn neg(self) -> Self {
                Self::with_bits(self.bits() ^ Self::SIGN_MASK)
            }
        }



        $(
            impl From<$float_ty> for $name {
                fn from(x: $float_ty) -> Self {
                    Self::with_float(x)
                }
            }

            impl Add for $name {
                type Output = Self;

                fn add(self, rhs: Self) -> Self {
                    Self::with_float(self.$as_float() + rhs.$as_float())
                }
            }

            impl Sub for $name {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self {
                    Self::with_float(self.$as_float() - rhs.$as_float())
                }
            }

            impl Mul for $name {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self {
                    Self::with_float(self.$as_float() * rhs.$as_float())
                }
            }

            impl Div for $name {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    Self::with_float(self.$as_float() / rhs.$as_float())
                }
            }
        )?

        impl BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                Self::with_bits(self.bits() & rhs.bits())
            }
        }

        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                Self::with_bits(self.bits() | rhs.bits())
            }
        }

        impl BitXor for $name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self {
                Self::with_bits(self.bits() ^ rhs.bits())
            }
        }

        impl Not for $name {
            type Output = Self;

            fn not(self) -> Self {
                Self::with_bits(!self.bits())
            }
        }
    };
}

ieee_float! {
    name = Ieee16,
    bits = 16,
    significand_bits = 10,
    bits_ty = u16,
    float_ty = f16,
    rust_type_not_stable = rust_type_not_stable,
}

ieee_float! {
    name = Ieee32,
    bits = 32,
    significand_bits = 23,
    bits_ty = u32,
    float_ty = f32,
    as_float = as_f32,
}

ieee_float! {
    name = Ieee64,
    bits = 64,
    significand_bits = 52,
    bits_ty = u64,
    float_ty = f64,
    as_float = as_f64,
}

ieee_float! {
    name = Ieee128,
    bits = 128,
    significand_bits = 112,
    bits_ty = u128,
    float_ty = f128,
    rust_type_not_stable = rust_type_not_stable,
}

/// Format a floating point number in a way that is reasonably human-readable, and that can be
/// converted back to binary without any rounding issues. The hexadecimal formatting of normal and
/// subnormal numbers is compatible with C99 and the `printf "%a"` format specifier. The NaN and Inf
/// formats are not supported by C99.
///
/// The encoding parameters are:
///
/// w - exponent field width in bits
/// t - trailing significand field width in bits
///
fn format_float(bits: u128, w: u8, t: u8, f: &mut Formatter) -> fmt::Result {
    debug_assert!(w > 0 && w <= 16, "Invalid exponent range");
    debug_assert!(1 + w + t <= 128, "Too large IEEE format for u128");
    debug_assert!((t + w + 1).is_power_of_two(), "Unexpected IEEE format size");

    let max_e_bits = (1u128 << w) - 1;
    let t_bits = bits & ((1u128 << t) - 1); // Trailing significand.
    let e_bits = (bits >> t) & max_e_bits; // Biased exponent.
    let sign_bit = (bits >> (w + t)) & 1;

    let bias: i32 = (1 << (w - 1)) - 1;
    let e = e_bits as i32 - bias; // Unbiased exponent.
    let emin = 1 - bias; // Minimum exponent.

    // How many hexadecimal digits are needed for the trailing significand?
    let digits = (t + 3) / 4;
    // Trailing significand left-aligned in `digits` hexadecimal digits.
    let left_t_bits = t_bits << (4 * digits - t);

    // All formats share the leading sign.
    if sign_bit != 0 {
        write!(f, "-")?;
    }

    if e_bits == 0 {
        if t_bits == 0 {
            // Zero.
            write!(f, "0.0")
        } else {
            // Subnormal.
            write!(
                f,
                "0x0.{0:01$x}p{2}",
                left_t_bits,
                usize::from(digits),
                emin
            )
        }
    } else if e_bits == max_e_bits {
        // Always print a `+` or `-` sign for these special values.
        // This makes them easier to parse as they can't be confused as identifiers.
        if sign_bit == 0 {
            write!(f, "+")?;
        }
        if t_bits == 0 {
            // Infinity.
            write!(f, "Inf")
        } else {
            // NaN.
            let payload = t_bits & ((1 << (t - 1)) - 1);
            if t_bits & (1 << (t - 1)) != 0 {
                // Quiet NaN.
                if payload != 0 {
                    write!(f, "NaN:0x{payload:x}")
                } else {
                    write!(f, "NaN")
                }
            } else {
                // Signaling NaN.
                write!(f, "sNaN:0x{payload:x}")
            }
        }
    } else {
        // Normal number.
        write!(f, "0x1.{0:01$x}p{2}", left_t_bits, usize::from(digits), e)
    }
}

/// Parse a float using the same format as `format_float` above.
///
/// The encoding parameters are:
///
/// w - exponent field width in bits
/// t - trailing significand field width in bits
///
fn parse_float(s: &str, w: u8, t: u8) -> Result<u128, &'static str> {
    debug_assert!(w > 0 && w <= 16, "Invalid exponent range");
    debug_assert!(1 + w + t <= 128, "Too large IEEE format for u128");
    debug_assert!((t + w + 1).is_power_of_two(), "Unexpected IEEE format size");

    let (sign_bit, s2) = if let Some(num) = s.strip_prefix('-') {
        (1u128 << (t + w), num)
    } else if let Some(num) = s.strip_prefix('+') {
        (0, num)
    } else {
        (0, s)
    };

    if !s2.starts_with("0x") {
        let max_e_bits = ((1u128 << w) - 1) << t;
        let quiet_bit = 1u128 << (t - 1);

        // The only decimal encoding allowed is 0.
        if s2 == "0.0" {
            return Ok(sign_bit);
        }

        if s2 == "Inf" {
            // +/- infinity: e = max, t = 0.
            return Ok(sign_bit | max_e_bits);
        }
        if s2 == "NaN" {
            // Canonical quiet NaN: e = max, t = quiet.
            return Ok(sign_bit | max_e_bits | quiet_bit);
        }
        if let Some(nan) = s2.strip_prefix("NaN:0x") {
            // Quiet NaN with payload.
            return match u128::from_str_radix(nan, 16) {
                Ok(payload) if payload < quiet_bit => {
                    Ok(sign_bit | max_e_bits | quiet_bit | payload)
                }
                _ => Err("Invalid NaN payload"),
            };
        }
        if let Some(nan) = s2.strip_prefix("sNaN:0x") {
            // Signaling NaN with payload.
            return match u128::from_str_radix(nan, 16) {
                Ok(payload) if 0 < payload && payload < quiet_bit => {
                    Ok(sign_bit | max_e_bits | payload)
                }
                _ => Err("Invalid sNaN payload"),
            };
        }

        return Err("Float must be hexadecimal");
    }
    let s3 = &s2[2..];

    let mut digits = 0u8;
    let mut digits_before_period: Option<u8> = None;
    let mut significand = 0u128;
    let mut exponent = 0i32;

    for (idx, ch) in s3.char_indices() {
        match ch {
            '.' => {
                // This is the radix point. There can only be one.
                if digits_before_period != None {
                    return Err("Multiple radix points");
                } else {
                    digits_before_period = Some(digits);
                }
            }
            'p' => {
                // The following exponent is a decimal number.
                let exp_str = &s3[1 + idx..];
                match exp_str.parse::<i16>() {
                    Ok(e) => {
                        exponent = i32::from(e);
                        break;
                    }
                    Err(_) => return Err("Bad exponent"),
                }
            }
            _ => match ch.to_digit(16) {
                Some(digit) => {
                    digits += 1;
                    if digits > 32 {
                        return Err("Too many digits");
                    }
                    significand = (significand << 4) | u128::from(digit);
                }
                None => return Err("Invalid character"),
            },
        }
    }

    if digits == 0 {
        return Err("No digits");
    }

    if significand == 0 {
        // This is +/- 0.0.
        return Ok(sign_bit);
    }

    // Number of bits appearing after the radix point.
    match digits_before_period {
        None => {} // No radix point present.
        Some(d) => exponent -= 4 * i32::from(digits - d),
    };

    // Normalize the significand and exponent.
    let significant_bits = (128 - significand.leading_zeros()) as u8;
    if significant_bits > t + 1 {
        let adjust = significant_bits - (t + 1);
        if significand & ((1u128 << adjust) - 1) != 0 {
            return Err("Too many significant bits");
        }
        // Adjust significand down.
        significand >>= adjust;
        exponent += i32::from(adjust);
    } else {
        let adjust = t + 1 - significant_bits;
        significand <<= adjust;
        exponent -= i32::from(adjust);
    }
    debug_assert_eq!(significand >> t, 1);

    // Trailing significand excludes the high bit.
    let t_bits = significand & ((1 << t) - 1);

    let max_exp = (1i32 << w) - 2;
    let bias: i32 = (1 << (w - 1)) - 1;
    exponent += bias + i32::from(t);

    if exponent > max_exp {
        Err("Magnitude too large")
    } else if exponent > 0 {
        // This is a normal number.
        let e_bits = (exponent as u128) << t;
        Ok(sign_bit | e_bits | t_bits)
    } else if 1 - exponent <= i32::from(t) {
        // This is a subnormal number: e = 0, t = significand bits.
        // Renormalize significand for exponent = 1.
        let adjust = 1 - exponent;
        if significand & ((1u128 << adjust) - 1) != 0 {
            Err("Subnormal underflow")
        } else {
            significand >>= adjust;
            Ok(sign_bit | significand)
        }
    } else {
        Err("Magnitude too small")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use core::{f32, f64};

    #[test]
    fn format_imm64() {
        assert_eq!(Imm64(0).to_string(), "0");
        assert_eq!(Imm64(9999).to_string(), "9999");
        assert_eq!(Imm64(10000).to_string(), "0x2710");
        assert_eq!(Imm64(-9999).to_string(), "-9999");
        assert_eq!(Imm64(-10000).to_string(), "-10000");
        assert_eq!(Imm64(0xffff).to_string(), "0xffff");
        assert_eq!(Imm64(0x10000).to_string(), "0x0001_0000");
    }

    #[test]
    fn format_uimm64() {
        assert_eq!(Uimm64(0).to_string(), "0");
        assert_eq!(Uimm64(9999).to_string(), "9999");
        assert_eq!(Uimm64(10000).to_string(), "0x2710");
        assert_eq!(Uimm64(-9999i64 as u64).to_string(), "0xffff_ffff_ffff_d8f1");
        assert_eq!(
            Uimm64(-10000i64 as u64).to_string(),
            "0xffff_ffff_ffff_d8f0"
        );
        assert_eq!(Uimm64(0xffff).to_string(), "0xffff");
        assert_eq!(Uimm64(0x10000).to_string(), "0x0001_0000");
    }

    // Verify that `text` can be parsed as a `T` into a value that displays as `want`.
    #[track_caller]
    fn parse_ok<T: FromStr + Display>(text: &str, want: &str)
    where
        <T as FromStr>::Err: Display,
    {
        match text.parse::<T>() {
            Err(s) => panic!("\"{text}\".parse() error: {s}"),
            Ok(x) => assert_eq!(x.to_string(), want),
        }
    }

    // Verify that `text` fails to parse as `T` with the error `msg`.
    fn parse_err<T: FromStr + Display>(text: &str, msg: &str)
    where
        <T as FromStr>::Err: Display,
    {
        match text.parse::<T>() {
            Err(s) => assert_eq!(s.to_string(), msg),
            Ok(x) => panic!("Wanted Err({msg}), but got {x}"),
        }
    }

    #[test]
    fn parse_imm64() {
        parse_ok::<Imm64>("0", "0");
        parse_ok::<Imm64>("1", "1");
        parse_ok::<Imm64>("-0", "0");
        parse_ok::<Imm64>("-1", "-1");
        parse_ok::<Imm64>("0x0", "0");
        parse_ok::<Imm64>("0xf", "15");
        parse_ok::<Imm64>("-0x9", "-9");

        // Probe limits.
        parse_ok::<Imm64>("0xffffffff_ffffffff", "-1");
        parse_ok::<Imm64>("0x80000000_00000000", "-9223372036854775808");
        parse_ok::<Imm64>("-0x80000000_00000000", "-9223372036854775808");
        parse_err::<Imm64>("-0x80000000_00000001", "Negative number too small");
        parse_ok::<Imm64>("18446744073709551615", "-1");
        parse_ok::<Imm64>("-9223372036854775808", "-9223372036854775808");
        // Overflow both the `checked_add` and `checked_mul`.
        parse_err::<Imm64>("18446744073709551616", "Too large decimal number");
        parse_err::<Imm64>("184467440737095516100", "Too large decimal number");
        parse_err::<Imm64>("-9223372036854775809", "Negative number too small");

        // Underscores are allowed where digits go.
        parse_ok::<Imm64>("0_0", "0");
        parse_ok::<Imm64>("-_10_0", "-100");
        parse_ok::<Imm64>("_10_", "10");
        parse_ok::<Imm64>("0x97_88_bb", "0x0097_88bb");
        parse_ok::<Imm64>("0x_97_", "151");

        parse_err::<Imm64>("", "No digits in number");
        parse_err::<Imm64>("-", "No digits in number");
        parse_err::<Imm64>("_", "No digits in number");
        parse_err::<Imm64>("0x", "No digits in number");
        parse_err::<Imm64>("0x_", "No digits in number");
        parse_err::<Imm64>("-0x", "No digits in number");
        parse_err::<Imm64>(" ", "Invalid character in decimal number");
        parse_err::<Imm64>("0 ", "Invalid character in decimal number");
        parse_err::<Imm64>(" 0", "Invalid character in decimal number");
        parse_err::<Imm64>("--", "Invalid character in decimal number");
        parse_err::<Imm64>("-0x-", "Invalid character in hexadecimal number");
        parse_err::<Imm64>("abc", "Invalid character in decimal number");
        parse_err::<Imm64>("-abc", "Invalid character in decimal number");

        // Hex count overflow.
        parse_err::<Imm64>("0x0_0000_0000_0000_0000", "Too many hexadecimal digits");
    }

    #[test]
    fn parse_uimm64() {
        parse_ok::<Uimm64>("0", "0");
        parse_ok::<Uimm64>("1", "1");
        parse_ok::<Uimm64>("0x0", "0");
        parse_ok::<Uimm64>("0xf", "15");
        parse_ok::<Uimm64>("0xffffffff_fffffff7", "0xffff_ffff_ffff_fff7");

        // Probe limits.
        parse_ok::<Uimm64>("0xffffffff_ffffffff", "0xffff_ffff_ffff_ffff");
        parse_ok::<Uimm64>("0x80000000_00000000", "0x8000_0000_0000_0000");
        parse_ok::<Uimm64>("18446744073709551615", "0xffff_ffff_ffff_ffff");
        // Overflow both the `checked_add` and `checked_mul`.
        parse_err::<Uimm64>("18446744073709551616", "Too large decimal number");
        parse_err::<Uimm64>("184467440737095516100", "Too large decimal number");

        // Underscores are allowed where digits go.
        parse_ok::<Uimm64>("0_0", "0");
        parse_ok::<Uimm64>("_10_", "10");
        parse_ok::<Uimm64>("0x97_88_bb", "0x0097_88bb");
        parse_ok::<Uimm64>("0x_97_", "151");

        parse_err::<Uimm64>("", "No digits in number");
        parse_err::<Uimm64>("_", "No digits in number");
        parse_err::<Uimm64>("0x", "No digits in number");
        parse_err::<Uimm64>("0x_", "No digits in number");
        parse_err::<Uimm64>("-", "Invalid character in decimal number");
        parse_err::<Uimm64>("-0x", "Invalid character in hexadecimal number");
        parse_err::<Uimm64>(" ", "Invalid character in decimal number");
        parse_err::<Uimm64>("0 ", "Invalid character in decimal number");
        parse_err::<Uimm64>(" 0", "Invalid character in decimal number");
        parse_err::<Uimm64>("--", "Invalid character in decimal number");
        parse_err::<Uimm64>("-0x-", "Invalid character in hexadecimal number");
        parse_err::<Uimm64>("-0", "Invalid character in decimal number");
        parse_err::<Uimm64>("-1", "Invalid character in decimal number");
        parse_err::<Uimm64>("abc", "Invalid character in decimal number");
        parse_err::<Uimm64>("-abc", "Invalid character in decimal number");

        // Hex count overflow.
        parse_err::<Uimm64>("0x0_0000_0000_0000_0000", "Too many hexadecimal digits");
    }

    #[test]
    fn format_offset32() {
        assert_eq!(Offset32(0).to_string(), "");
        assert_eq!(Offset32(1).to_string(), "+1");
        assert_eq!(Offset32(-1).to_string(), "-1");
        assert_eq!(Offset32(9999).to_string(), "+9999");
        assert_eq!(Offset32(10000).to_string(), "+0x2710");
        assert_eq!(Offset32(-9999).to_string(), "-9999");
        assert_eq!(Offset32(-10000).to_string(), "-0x2710");
        assert_eq!(Offset32(0xffff).to_string(), "+0xffff");
        assert_eq!(Offset32(0x10000).to_string(), "+0x0001_0000");
    }

    #[test]
    fn parse_offset32() {
        parse_ok::<Offset32>("+0", "");
        parse_ok::<Offset32>("+1", "+1");
        parse_ok::<Offset32>("-0", "");
        parse_ok::<Offset32>("-1", "-1");
        parse_ok::<Offset32>("+0x0", "");
        parse_ok::<Offset32>("+0xf", "+15");
        parse_ok::<Offset32>("-0x9", "-9");
        parse_ok::<Offset32>("-0x8000_0000", "-0x8000_0000");

        parse_err::<Offset32>("+0x8000_0000", "Offset out of range");
    }

    #[test]
    fn format_ieee16() {
        assert_eq!(Ieee16::with_bits(0).to_string(), "0.0"); // 0.0
        assert_eq!(Ieee16::with_bits(0x8000).to_string(), "-0.0"); // -0.0
        assert_eq!(Ieee16::with_bits(0x3c00).to_string(), "0x1.000p0"); // 1.0
        assert_eq!(Ieee16::with_bits(0x3e00).to_string(), "0x1.800p0"); // 1.5
        assert_eq!(Ieee16::with_bits(0x3800).to_string(), "0x1.000p-1"); // 0.5
        assert_eq!(
            Ieee16::with_bits(0x1400).to_string(), // `f16::EPSILON`
            "0x1.000p-10"
        );
        assert_eq!(
            Ieee16::with_bits(0xfbff).to_string(), // `f16::MIN`
            "-0x1.ffcp15"
        );
        assert_eq!(
            Ieee16::with_bits(0x7bff).to_string(), // `f16::MAX`
            "0x1.ffcp15"
        );
        // Smallest positive normal number.
        assert_eq!(
            Ieee16::with_bits(0x0400).to_string(), // `f16::MIN_POSITIVE`
            "0x1.000p-14"
        );
        // Subnormals.
        assert_eq!(
            Ieee16::with_bits(0x0200).to_string(), // `f16::MIN_POSITIVE / 2.0`
            "0x0.800p-14"
        );
        assert_eq!(
            Ieee16::with_bits(0x0001).to_string(), // `f16::MIN_POSITIVE * f16::EPSILON`
            "0x0.004p-14"
        );
        assert_eq!(
            Ieee16::with_bits(0x7c00).to_string(), // `f16::INFINITY`
            "+Inf"
        );
        assert_eq!(
            Ieee16::with_bits(0xfc00).to_string(), // `f16::NEG_INFINITY`
            "-Inf"
        );
        assert_eq!(
            Ieee16::with_bits(0x7e00).to_string(), // `f16::NAN`
            "+NaN"
        );
        assert_eq!(
            Ieee16::with_bits(0xfe00).to_string(), // `-f16::NAN`
            "-NaN"
        );
        // Construct some qNaNs with payloads.
        assert_eq!(Ieee16::with_bits(0x7e01).to_string(), "+NaN:0x1");
        assert_eq!(Ieee16::with_bits(0x7f01).to_string(), "+NaN:0x101");
        // Signaling NaNs.
        assert_eq!(Ieee16::with_bits(0x7c01).to_string(), "+sNaN:0x1");
        assert_eq!(Ieee16::with_bits(0x7d01).to_string(), "+sNaN:0x101");
    }

    #[test]
    fn parse_ieee16() {
        parse_ok::<Ieee16>("0.0", "0.0");
        parse_ok::<Ieee16>("+0.0", "0.0");
        parse_ok::<Ieee16>("-0.0", "-0.0");
        parse_ok::<Ieee16>("0x0", "0.0");
        parse_ok::<Ieee16>("0x0.0", "0.0");
        parse_ok::<Ieee16>("0x.0", "0.0");
        parse_ok::<Ieee16>("0x0.", "0.0");
        parse_ok::<Ieee16>("0x1", "0x1.000p0");
        parse_ok::<Ieee16>("+0x1", "0x1.000p0");
        parse_ok::<Ieee16>("-0x1", "-0x1.000p0");
        parse_ok::<Ieee16>("0x10", "0x1.000p4");
        parse_ok::<Ieee16>("0x10.0", "0x1.000p4");
        parse_err::<Ieee16>("0.", "Float must be hexadecimal");
        parse_err::<Ieee16>(".0", "Float must be hexadecimal");
        parse_err::<Ieee16>("0", "Float must be hexadecimal");
        parse_err::<Ieee16>("-0", "Float must be hexadecimal");
        parse_err::<Ieee16>(".", "Float must be hexadecimal");
        parse_err::<Ieee16>("", "Float must be hexadecimal");
        parse_err::<Ieee16>("-", "Float must be hexadecimal");
        parse_err::<Ieee16>("0x", "No digits");
        parse_err::<Ieee16>("0x..", "Multiple radix points");

        // Check significant bits.
        parse_ok::<Ieee16>("0x0.ffe", "0x1.ffcp-1");
        parse_ok::<Ieee16>("0x1.ffc", "0x1.ffcp0");
        parse_ok::<Ieee16>("0x3.ff8", "0x1.ffcp1");
        parse_ok::<Ieee16>("0x7.ff", "0x1.ffcp2");
        parse_ok::<Ieee16>("0xf.fe", "0x1.ffcp3");
        parse_err::<Ieee16>("0x1.ffe", "Too many significant bits");
        parse_err::<Ieee16>("0x1.ffc00000000000000000000000000000", "Too many digits");

        // Exponents.
        parse_ok::<Ieee16>("0x1p3", "0x1.000p3");
        parse_ok::<Ieee16>("0x1p-3", "0x1.000p-3");
        parse_ok::<Ieee16>("0x1.0p3", "0x1.000p3");
        parse_ok::<Ieee16>("0x2.0p3", "0x1.000p4");
        parse_ok::<Ieee16>("0x1.0p15", "0x1.000p15");
        parse_ok::<Ieee16>("0x1.0p-14", "0x1.000p-14");
        parse_ok::<Ieee16>("0x0.1p-10", "0x1.000p-14");
        parse_err::<Ieee16>("0x2.0p15", "Magnitude too large");

        // Subnormals.
        parse_ok::<Ieee16>("0x1.0p-15", "0x0.800p-14");
        parse_ok::<Ieee16>("0x1.0p-24", "0x0.004p-14");
        parse_ok::<Ieee16>("0x0.004p-14", "0x0.004p-14");
        parse_err::<Ieee16>("0x0.102p-14", "Subnormal underflow");
        parse_err::<Ieee16>("0x1.8p-24", "Subnormal underflow");
        parse_err::<Ieee16>("0x1.0p-25", "Magnitude too small");

        // NaNs and Infs.
        parse_ok::<Ieee16>("Inf", "+Inf");
        parse_ok::<Ieee16>("+Inf", "+Inf");
        parse_ok::<Ieee16>("-Inf", "-Inf");
        parse_ok::<Ieee16>("NaN", "+NaN");
        parse_ok::<Ieee16>("+NaN", "+NaN");
        parse_ok::<Ieee16>("-NaN", "-NaN");
        parse_ok::<Ieee16>("NaN:0x0", "+NaN");
        parse_err::<Ieee16>("NaN:", "Float must be hexadecimal");
        parse_err::<Ieee16>("NaN:0", "Float must be hexadecimal");
        parse_err::<Ieee16>("NaN:0x", "Invalid NaN payload");
        parse_ok::<Ieee16>("NaN:0x001", "+NaN:0x1");
        parse_ok::<Ieee16>("NaN:0x101", "+NaN:0x101");
        parse_err::<Ieee16>("NaN:0x301", "Invalid NaN payload");
        parse_ok::<Ieee16>("sNaN:0x1", "+sNaN:0x1");
        parse_err::<Ieee16>("sNaN:0x0", "Invalid sNaN payload");
        parse_ok::<Ieee16>("sNaN:0x101", "+sNaN:0x101");
        parse_err::<Ieee16>("sNaN:0x301", "Invalid sNaN payload");
    }

    #[test]
    fn pow2_ieee16() {
        assert_eq!(Ieee16::pow2(0).to_string(), "0x1.000p0");
        assert_eq!(Ieee16::pow2(1).to_string(), "0x1.000p1");
        assert_eq!(Ieee16::pow2(-1).to_string(), "0x1.000p-1");
        assert_eq!(Ieee16::pow2(15).to_string(), "0x1.000p15");
        assert_eq!(Ieee16::pow2(-14).to_string(), "0x1.000p-14");

        assert_eq!((-Ieee16::pow2(1)).to_string(), "-0x1.000p1");
    }

    #[test]
    fn fcvt_to_sint_negative_overflow_ieee16() {
        // FIXME(#8312): Replace with commented out version once Rust f16 support is stabilised.
        // let n = 8;
        // assert_eq!(
        //     -((1u16 << (n - 1)) as f16) - 1.0,
        //     Ieee16::fcvt_to_sint_negative_overflow(n).as_f16()
        // );
        let n = 8;
        assert_eq!(
            "-0x1.020p7",
            Ieee16::fcvt_to_sint_negative_overflow(n).to_string()
        );
    }

    #[test]
    fn format_ieee32() {
        assert_eq!(Ieee32::with_float(0.0).to_string(), "0.0");
        assert_eq!(Ieee32::with_float(-0.0).to_string(), "-0.0");
        assert_eq!(Ieee32::with_float(1.0).to_string(), "0x1.000000p0");
        assert_eq!(Ieee32::with_float(1.5).to_string(), "0x1.800000p0");
        assert_eq!(Ieee32::with_float(0.5).to_string(), "0x1.000000p-1");
        assert_eq!(
            Ieee32::with_float(f32::EPSILON).to_string(),
            "0x1.000000p-23"
        );
        assert_eq!(Ieee32::with_float(f32::MIN).to_string(), "-0x1.fffffep127");
        assert_eq!(Ieee32::with_float(f32::MAX).to_string(), "0x1.fffffep127");
        // Smallest positive normal number.
        assert_eq!(
            Ieee32::with_float(f32::MIN_POSITIVE).to_string(),
            "0x1.000000p-126"
        );
        // Subnormals.
        assert_eq!(
            Ieee32::with_float(f32::MIN_POSITIVE / 2.0).to_string(),
            "0x0.800000p-126"
        );
        assert_eq!(
            Ieee32::with_float(f32::MIN_POSITIVE * f32::EPSILON).to_string(),
            "0x0.000002p-126"
        );
        assert_eq!(Ieee32::with_float(f32::INFINITY).to_string(), "+Inf");
        assert_eq!(Ieee32::with_float(f32::NEG_INFINITY).to_string(), "-Inf");
        assert_eq!(Ieee32::with_float(f32::NAN).to_string(), "+NaN");
        assert_eq!(Ieee32::with_float(-f32::NAN).to_string(), "-NaN");
        // Construct some qNaNs with payloads.
        assert_eq!(Ieee32::with_bits(0x7fc00001).to_string(), "+NaN:0x1");
        assert_eq!(Ieee32::with_bits(0x7ff00001).to_string(), "+NaN:0x300001");
        // Signaling NaNs.
        assert_eq!(Ieee32::with_bits(0x7f800001).to_string(), "+sNaN:0x1");
        assert_eq!(Ieee32::with_bits(0x7fa00001).to_string(), "+sNaN:0x200001");
    }

    #[test]
    fn parse_ieee32() {
        parse_ok::<Ieee32>("0.0", "0.0");
        parse_ok::<Ieee32>("+0.0", "0.0");
        parse_ok::<Ieee32>("-0.0", "-0.0");
        parse_ok::<Ieee32>("0x0", "0.0");
        parse_ok::<Ieee32>("0x0.0", "0.0");
        parse_ok::<Ieee32>("0x.0", "0.0");
        parse_ok::<Ieee32>("0x0.", "0.0");
        parse_ok::<Ieee32>("0x1", "0x1.000000p0");
        parse_ok::<Ieee32>("+0x1", "0x1.000000p0");
        parse_ok::<Ieee32>("-0x1", "-0x1.000000p0");
        parse_ok::<Ieee32>("0x10", "0x1.000000p4");
        parse_ok::<Ieee32>("0x10.0", "0x1.000000p4");
        parse_err::<Ieee32>("0.", "Float must be hexadecimal");
        parse_err::<Ieee32>(".0", "Float must be hexadecimal");
        parse_err::<Ieee32>("0", "Float must be hexadecimal");
        parse_err::<Ieee32>("-0", "Float must be hexadecimal");
        parse_err::<Ieee32>(".", "Float must be hexadecimal");
        parse_err::<Ieee32>("", "Float must be hexadecimal");
        parse_err::<Ieee32>("-", "Float must be hexadecimal");
        parse_err::<Ieee32>("0x", "No digits");
        parse_err::<Ieee32>("0x..", "Multiple radix points");

        // Check significant bits.
        parse_ok::<Ieee32>("0x0.ffffff", "0x1.fffffep-1");
        parse_ok::<Ieee32>("0x1.fffffe", "0x1.fffffep0");
        parse_ok::<Ieee32>("0x3.fffffc", "0x1.fffffep1");
        parse_ok::<Ieee32>("0x7.fffff8", "0x1.fffffep2");
        parse_ok::<Ieee32>("0xf.fffff0", "0x1.fffffep3");
        parse_err::<Ieee32>("0x1.ffffff", "Too many significant bits");
        parse_err::<Ieee32>("0x1.fffffe00000000000000000000000000", "Too many digits");

        // Exponents.
        parse_ok::<Ieee32>("0x1p3", "0x1.000000p3");
        parse_ok::<Ieee32>("0x1p-3", "0x1.000000p-3");
        parse_ok::<Ieee32>("0x1.0p3", "0x1.000000p3");
        parse_ok::<Ieee32>("0x2.0p3", "0x1.000000p4");
        parse_ok::<Ieee32>("0x1.0p127", "0x1.000000p127");
        parse_ok::<Ieee32>("0x1.0p-126", "0x1.000000p-126");
        parse_ok::<Ieee32>("0x0.1p-122", "0x1.000000p-126");
        parse_err::<Ieee32>("0x2.0p127", "Magnitude too large");

        // Subnormals.
        parse_ok::<Ieee32>("0x1.0p-127", "0x0.800000p-126");
        parse_ok::<Ieee32>("0x1.0p-149", "0x0.000002p-126");
        parse_ok::<Ieee32>("0x0.000002p-126", "0x0.000002p-126");
        parse_err::<Ieee32>("0x0.100001p-126", "Subnormal underflow");
        parse_err::<Ieee32>("0x1.8p-149", "Subnormal underflow");
        parse_err::<Ieee32>("0x1.0p-150", "Magnitude too small");

        // NaNs and Infs.
        parse_ok::<Ieee32>("Inf", "+Inf");
        parse_ok::<Ieee32>("+Inf", "+Inf");
        parse_ok::<Ieee32>("-Inf", "-Inf");
        parse_ok::<Ieee32>("NaN", "+NaN");
        parse_ok::<Ieee32>("+NaN", "+NaN");
        parse_ok::<Ieee32>("-NaN", "-NaN");
        parse_ok::<Ieee32>("NaN:0x0", "+NaN");
        parse_err::<Ieee32>("NaN:", "Float must be hexadecimal");
        parse_err::<Ieee32>("NaN:0", "Float must be hexadecimal");
        parse_err::<Ieee32>("NaN:0x", "Invalid NaN payload");
        parse_ok::<Ieee32>("NaN:0x000001", "+NaN:0x1");
        parse_ok::<Ieee32>("NaN:0x300001", "+NaN:0x300001");
        parse_err::<Ieee32>("NaN:0x400001", "Invalid NaN payload");
        parse_ok::<Ieee32>("sNaN:0x1", "+sNaN:0x1");
        parse_err::<Ieee32>("sNaN:0x0", "Invalid sNaN payload");
        parse_ok::<Ieee32>("sNaN:0x200001", "+sNaN:0x200001");
        parse_err::<Ieee32>("sNaN:0x400001", "Invalid sNaN payload");
    }

    #[test]
    fn pow2_ieee32() {
        assert_eq!(Ieee32::pow2(0).to_string(), "0x1.000000p0");
        assert_eq!(Ieee32::pow2(1).to_string(), "0x1.000000p1");
        assert_eq!(Ieee32::pow2(-1).to_string(), "0x1.000000p-1");
        assert_eq!(Ieee32::pow2(127).to_string(), "0x1.000000p127");
        assert_eq!(Ieee32::pow2(-126).to_string(), "0x1.000000p-126");

        assert_eq!((-Ieee32::pow2(1)).to_string(), "-0x1.000000p1");
    }

    #[test]
    fn fcvt_to_sint_negative_overflow_ieee32() {
        for n in [8, 16] {
            assert_eq!(
                -((1u32 << (n - 1)) as f32) - 1.0,
                Ieee32::fcvt_to_sint_negative_overflow(n).as_f32(),
                "n = {n}"
            );
        }
    }

    #[test]
    fn format_ieee64() {
        assert_eq!(Ieee64::with_float(0.0).to_string(), "0.0");
        assert_eq!(Ieee64::with_float(-0.0).to_string(), "-0.0");
        assert_eq!(Ieee64::with_float(1.0).to_string(), "0x1.0000000000000p0");
        assert_eq!(Ieee64::with_float(1.5).to_string(), "0x1.8000000000000p0");
        assert_eq!(Ieee64::with_float(0.5).to_string(), "0x1.0000000000000p-1");
        assert_eq!(
            Ieee64::with_float(f64::EPSILON).to_string(),
            "0x1.0000000000000p-52"
        );
        assert_eq!(
            Ieee64::with_float(f64::MIN).to_string(),
            "-0x1.fffffffffffffp1023"
        );
        assert_eq!(
            Ieee64::with_float(f64::MAX).to_string(),
            "0x1.fffffffffffffp1023"
        );
        // Smallest positive normal number.
        assert_eq!(
            Ieee64::with_float(f64::MIN_POSITIVE).to_string(),
            "0x1.0000000000000p-1022"
        );
        // Subnormals.
        assert_eq!(
            Ieee64::with_float(f64::MIN_POSITIVE / 2.0).to_string(),
            "0x0.8000000000000p-1022"
        );
        assert_eq!(
            Ieee64::with_float(f64::MIN_POSITIVE * f64::EPSILON).to_string(),
            "0x0.0000000000001p-1022"
        );
        assert_eq!(Ieee64::with_float(f64::INFINITY).to_string(), "+Inf");
        assert_eq!(Ieee64::with_float(f64::NEG_INFINITY).to_string(), "-Inf");
        assert_eq!(Ieee64::with_float(f64::NAN).to_string(), "+NaN");
        assert_eq!(Ieee64::with_float(-f64::NAN).to_string(), "-NaN");
        // Construct some qNaNs with payloads.
        assert_eq!(
            Ieee64::with_bits(0x7ff8000000000001).to_string(),
            "+NaN:0x1"
        );
        assert_eq!(
            Ieee64::with_bits(0x7ffc000000000001).to_string(),
            "+NaN:0x4000000000001"
        );
        // Signaling NaNs.
        assert_eq!(
            Ieee64::with_bits(0x7ff0000000000001).to_string(),
            "+sNaN:0x1"
        );
        assert_eq!(
            Ieee64::with_bits(0x7ff4000000000001).to_string(),
            "+sNaN:0x4000000000001"
        );
    }

    #[test]
    fn parse_ieee64() {
        parse_ok::<Ieee64>("0.0", "0.0");
        parse_ok::<Ieee64>("-0.0", "-0.0");
        parse_ok::<Ieee64>("0x0", "0.0");
        parse_ok::<Ieee64>("0x0.0", "0.0");
        parse_ok::<Ieee64>("0x.0", "0.0");
        parse_ok::<Ieee64>("0x0.", "0.0");
        parse_ok::<Ieee64>("0x1", "0x1.0000000000000p0");
        parse_ok::<Ieee64>("-0x1", "-0x1.0000000000000p0");
        parse_ok::<Ieee64>("0x10", "0x1.0000000000000p4");
        parse_ok::<Ieee64>("0x10.0", "0x1.0000000000000p4");
        parse_err::<Ieee64>("0.", "Float must be hexadecimal");
        parse_err::<Ieee64>(".0", "Float must be hexadecimal");
        parse_err::<Ieee64>("0", "Float must be hexadecimal");
        parse_err::<Ieee64>("-0", "Float must be hexadecimal");
        parse_err::<Ieee64>(".", "Float must be hexadecimal");
        parse_err::<Ieee64>("", "Float must be hexadecimal");
        parse_err::<Ieee64>("-", "Float must be hexadecimal");
        parse_err::<Ieee64>("0x", "No digits");
        parse_err::<Ieee64>("0x..", "Multiple radix points");

        // Check significant bits.
        parse_ok::<Ieee64>("0x0.fffffffffffff8", "0x1.fffffffffffffp-1");
        parse_ok::<Ieee64>("0x1.fffffffffffff", "0x1.fffffffffffffp0");
        parse_ok::<Ieee64>("0x3.ffffffffffffe", "0x1.fffffffffffffp1");
        parse_ok::<Ieee64>("0x7.ffffffffffffc", "0x1.fffffffffffffp2");
        parse_ok::<Ieee64>("0xf.ffffffffffff8", "0x1.fffffffffffffp3");
        parse_err::<Ieee64>("0x3.fffffffffffff", "Too many significant bits");
        parse_err::<Ieee64>("0x001.fffffe000000000000000000000000", "Too many digits");

        // Exponents.
        parse_ok::<Ieee64>("0x1p3", "0x1.0000000000000p3");
        parse_ok::<Ieee64>("0x1p-3", "0x1.0000000000000p-3");
        parse_ok::<Ieee64>("0x1.0p3", "0x1.0000000000000p3");
        parse_ok::<Ieee64>("0x2.0p3", "0x1.0000000000000p4");
        parse_ok::<Ieee64>("0x1.0p1023", "0x1.0000000000000p1023");
        parse_ok::<Ieee64>("0x1.0p-1022", "0x1.0000000000000p-1022");
        parse_ok::<Ieee64>("0x0.1p-1018", "0x1.0000000000000p-1022");
        parse_err::<Ieee64>("0x2.0p1023", "Magnitude too large");

        // Subnormals.
        parse_ok::<Ieee64>("0x1.0p-1023", "0x0.8000000000000p-1022");
        parse_ok::<Ieee64>("0x1.0p-1074", "0x0.0000000000001p-1022");
        parse_ok::<Ieee64>("0x0.0000000000001p-1022", "0x0.0000000000001p-1022");
        parse_err::<Ieee64>("0x0.10000000000008p-1022", "Subnormal underflow");
        parse_err::<Ieee64>("0x1.8p-1074", "Subnormal underflow");
        parse_err::<Ieee64>("0x1.0p-1075", "Magnitude too small");

        // NaNs and Infs.
        parse_ok::<Ieee64>("Inf", "+Inf");
        parse_ok::<Ieee64>("-Inf", "-Inf");
        parse_ok::<Ieee64>("NaN", "+NaN");
        parse_ok::<Ieee64>("-NaN", "-NaN");
        parse_ok::<Ieee64>("NaN:0x0", "+NaN");
        parse_err::<Ieee64>("NaN:", "Float must be hexadecimal");
        parse_err::<Ieee64>("NaN:0", "Float must be hexadecimal");
        parse_err::<Ieee64>("NaN:0x", "Invalid NaN payload");
        parse_ok::<Ieee64>("NaN:0x000001", "+NaN:0x1");
        parse_ok::<Ieee64>("NaN:0x4000000000001", "+NaN:0x4000000000001");
        parse_err::<Ieee64>("NaN:0x8000000000001", "Invalid NaN payload");
        parse_ok::<Ieee64>("sNaN:0x1", "+sNaN:0x1");
        parse_err::<Ieee64>("sNaN:0x0", "Invalid sNaN payload");
        parse_ok::<Ieee64>("sNaN:0x4000000000001", "+sNaN:0x4000000000001");
        parse_err::<Ieee64>("sNaN:0x8000000000001", "Invalid sNaN payload");
    }

    #[test]
    fn pow2_ieee64() {
        assert_eq!(Ieee64::pow2(0).to_string(), "0x1.0000000000000p0");
        assert_eq!(Ieee64::pow2(1).to_string(), "0x1.0000000000000p1");
        assert_eq!(Ieee64::pow2(-1).to_string(), "0x1.0000000000000p-1");
        assert_eq!(Ieee64::pow2(1023).to_string(), "0x1.0000000000000p1023");
        assert_eq!(Ieee64::pow2(-1022).to_string(), "0x1.0000000000000p-1022");

        assert_eq!((-Ieee64::pow2(1)).to_string(), "-0x1.0000000000000p1");
    }

    #[test]
    fn fcvt_to_sint_negative_overflow_ieee64() {
        for n in [8, 16, 32] {
            assert_eq!(
                -((1u64 << (n - 1)) as f64) - 1.0,
                Ieee64::fcvt_to_sint_negative_overflow(n).as_f64(),
                "n = {n}"
            );
        }
    }

    #[test]
    fn format_ieee128() {
        assert_eq!(
            Ieee128::with_bits(0x00000000000000000000000000000000).to_string(), // 0.0
            "0.0"
        );
        assert_eq!(
            Ieee128::with_bits(0x80000000000000000000000000000000).to_string(), // -0.0
            "-0.0"
        );
        assert_eq!(
            Ieee128::with_bits(0x3fff0000000000000000000000000000).to_string(), // 1.0
            "0x1.0000000000000000000000000000p0"
        );
        assert_eq!(
            Ieee128::with_bits(0x3fff8000000000000000000000000000).to_string(), // 1.5
            "0x1.8000000000000000000000000000p0"
        );
        assert_eq!(
            Ieee128::with_bits(0x3ffe0000000000000000000000000000).to_string(), // 0.5
            "0x1.0000000000000000000000000000p-1"
        );
        assert_eq!(
            Ieee128::with_bits(0x3f8f0000000000000000000000000000).to_string(), // `f128::EPSILON`
            "0x1.0000000000000000000000000000p-112"
        );
        assert_eq!(
            Ieee128::with_bits(0xfffeffffffffffffffffffffffffffff).to_string(), // `f128::MIN`
            "-0x1.ffffffffffffffffffffffffffffp16383"
        );
        assert_eq!(
            Ieee128::with_bits(0x7ffeffffffffffffffffffffffffffff).to_string(), // `f128::MAX`
            "0x1.ffffffffffffffffffffffffffffp16383"
        );
        // Smallest positive normal number.
        assert_eq!(
            Ieee128::with_bits(0x00010000000000000000000000000000).to_string(), // `f128::MIN_POSITIVE`
            "0x1.0000000000000000000000000000p-16382"
        );
        // Subnormals.
        assert_eq!(
            Ieee128::with_bits(0x00008000000000000000000000000000).to_string(), // `f128::MIN_POSITIVE / 2.0`
            "0x0.8000000000000000000000000000p-16382"
        );
        assert_eq!(
            Ieee128::with_bits(0x00000000000000000000000000000001).to_string(), // `f128::MIN_POSITIVE * f128::EPSILON`
            "0x0.0000000000000000000000000001p-16382"
        );
        assert_eq!(
            Ieee128::with_bits(0x7fff0000000000000000000000000000).to_string(), // `f128::INFINITY`
            "+Inf"
        );
        assert_eq!(
            Ieee128::with_bits(0xffff0000000000000000000000000000).to_string(), // `f128::NEG_INFINITY`
            "-Inf"
        );
        assert_eq!(
            Ieee128::with_bits(0x7fff8000000000000000000000000000).to_string(), // `f128::NAN`
            "+NaN"
        );
        assert_eq!(
            Ieee128::with_bits(0xffff8000000000000000000000000000).to_string(), // `-f128::NAN`
            "-NaN"
        );
        // Construct some qNaNs with payloads.
        assert_eq!(
            Ieee128::with_bits(0x7fff8000000000000000000000000001).to_string(),
            "+NaN:0x1"
        );
        assert_eq!(
            Ieee128::with_bits(0x7fffc000000000000000000000000001).to_string(),
            "+NaN:0x4000000000000000000000000001"
        );
        // Signaling NaNs.
        assert_eq!(
            Ieee128::with_bits(0x7fff0000000000000000000000000001).to_string(),
            "+sNaN:0x1"
        );
        assert_eq!(
            Ieee128::with_bits(0x7fff4000000000000000000000000001).to_string(),
            "+sNaN:0x4000000000000000000000000001"
        );
    }

    #[test]
    fn parse_ieee128() {
        parse_ok::<Ieee128>("0.0", "0.0");
        parse_ok::<Ieee128>("-0.0", "-0.0");
        parse_ok::<Ieee128>("0x0", "0.0");
        parse_ok::<Ieee128>("0x0.0", "0.0");
        parse_ok::<Ieee128>("0x.0", "0.0");
        parse_ok::<Ieee128>("0x0.", "0.0");
        parse_ok::<Ieee128>("0x1", "0x1.0000000000000000000000000000p0");
        parse_ok::<Ieee128>("-0x1", "-0x1.0000000000000000000000000000p0");
        parse_ok::<Ieee128>("0x10", "0x1.0000000000000000000000000000p4");
        parse_ok::<Ieee128>("0x10.0", "0x1.0000000000000000000000000000p4");
        parse_err::<Ieee128>("0.", "Float must be hexadecimal");
        parse_err::<Ieee128>(".0", "Float must be hexadecimal");
        parse_err::<Ieee128>("0", "Float must be hexadecimal");
        parse_err::<Ieee128>("-0", "Float must be hexadecimal");
        parse_err::<Ieee128>(".", "Float must be hexadecimal");
        parse_err::<Ieee128>("", "Float must be hexadecimal");
        parse_err::<Ieee128>("-", "Float must be hexadecimal");
        parse_err::<Ieee128>("0x", "No digits");
        parse_err::<Ieee128>("0x..", "Multiple radix points");

        // Check significant bits.
        parse_ok::<Ieee128>(
            "0x0.ffffffffffffffffffffffffffff8",
            "0x1.ffffffffffffffffffffffffffffp-1",
        );
        parse_ok::<Ieee128>(
            "0x1.ffffffffffffffffffffffffffff",
            "0x1.ffffffffffffffffffffffffffffp0",
        );
        parse_ok::<Ieee128>(
            "0x3.fffffffffffffffffffffffffffe",
            "0x1.ffffffffffffffffffffffffffffp1",
        );
        parse_ok::<Ieee128>(
            "0x7.fffffffffffffffffffffffffffc",
            "0x1.ffffffffffffffffffffffffffffp2",
        );
        parse_ok::<Ieee128>(
            "0xf.fffffffffffffffffffffffffff8",
            "0x1.ffffffffffffffffffffffffffffp3",
        );
        parse_err::<Ieee128>(
            "0x3.ffffffffffffffffffffffffffff",
            "Too many significant bits",
        );
        parse_err::<Ieee128>("0x001.fffffe000000000000000000000000", "Too many digits");

        // Exponents.
        parse_ok::<Ieee128>("0x1p3", "0x1.0000000000000000000000000000p3");
        parse_ok::<Ieee128>("0x1p-3", "0x1.0000000000000000000000000000p-3");
        parse_ok::<Ieee128>("0x1.0p3", "0x1.0000000000000000000000000000p3");
        parse_ok::<Ieee128>("0x2.0p3", "0x1.0000000000000000000000000000p4");
        parse_ok::<Ieee128>("0x1.0p16383", "0x1.0000000000000000000000000000p16383");
        parse_ok::<Ieee128>("0x1.0p-16382", "0x1.0000000000000000000000000000p-16382");
        parse_ok::<Ieee128>("0x0.1p-16378", "0x1.0000000000000000000000000000p-16382");
        parse_err::<Ieee128>("0x2.0p16383", "Magnitude too large");

        // Subnormals.
        parse_ok::<Ieee128>("0x1.0p-16383", "0x0.8000000000000000000000000000p-16382");
        parse_ok::<Ieee128>("0x1.0p-16494", "0x0.0000000000000000000000000001p-16382");
        parse_ok::<Ieee128>(
            "0x0.0000000000000000000000000001p-16382",
            "0x0.0000000000000000000000000001p-16382",
        );
        parse_err::<Ieee128>(
            "0x0.10000000000000000000000000008p-16382",
            "Subnormal underflow",
        );
        parse_err::<Ieee128>("0x1.8p-16494", "Subnormal underflow");
        parse_err::<Ieee128>("0x1.0p-16495", "Magnitude too small");

        // NaNs and Infs.
        parse_ok::<Ieee128>("Inf", "+Inf");
        parse_ok::<Ieee128>("-Inf", "-Inf");
        parse_ok::<Ieee128>("NaN", "+NaN");
        parse_ok::<Ieee128>("-NaN", "-NaN");
        parse_ok::<Ieee128>("NaN:0x0", "+NaN");
        parse_err::<Ieee128>("NaN:", "Float must be hexadecimal");
        parse_err::<Ieee128>("NaN:0", "Float must be hexadecimal");
        parse_err::<Ieee128>("NaN:0x", "Invalid NaN payload");
        parse_ok::<Ieee128>("NaN:0x000001", "+NaN:0x1");
        parse_ok::<Ieee128>(
            "NaN:0x4000000000000000000000000001",
            "+NaN:0x4000000000000000000000000001",
        );
        parse_err::<Ieee128>("NaN:0x8000000000000000000000000001", "Invalid NaN payload");
        parse_ok::<Ieee128>("sNaN:0x1", "+sNaN:0x1");
        parse_err::<Ieee128>("sNaN:0x0", "Invalid sNaN payload");
        parse_ok::<Ieee128>(
            "sNaN:0x4000000000000000000000000001",
            "+sNaN:0x4000000000000000000000000001",
        );
        parse_err::<Ieee128>(
            "sNaN:0x8000000000000000000000000001",
            "Invalid sNaN payload",
        );
    }

    #[test]
    fn pow2_ieee128() {
        assert_eq!(
            Ieee128::pow2(0).to_string(),
            "0x1.0000000000000000000000000000p0"
        );
        assert_eq!(
            Ieee128::pow2(1).to_string(),
            "0x1.0000000000000000000000000000p1"
        );
        assert_eq!(
            Ieee128::pow2(-1).to_string(),
            "0x1.0000000000000000000000000000p-1"
        );
        assert_eq!(
            Ieee128::pow2(16383).to_string(),
            "0x1.0000000000000000000000000000p16383"
        );
        assert_eq!(
            Ieee128::pow2(-16382).to_string(),
            "0x1.0000000000000000000000000000p-16382"
        );

        assert_eq!(
            (-Ieee128::pow2(1)).to_string(),
            "-0x1.0000000000000000000000000000p1"
        );
    }

    #[test]
    fn fcvt_to_sint_negative_overflow_ieee128() {
        // FIXME(#8312): Replace with commented out version once Rust f128 support is stabilised.
        // for n in [8, 16, 32, 64] {
        //     assert_eq!(
        //         -((1u128 << (n - 1)) as f128) - 1.0,
        //         Ieee128::fcvt_to_sint_negative_overflow(n).as_f128(),
        //         "n = {n}"
        //     );
        // }
        for (n, expected) in [
            (8, "-0x1.0200000000000000000000000000p7"),
            (16, "-0x1.0002000000000000000000000000p15"),
            (32, "-0x1.0000000200000000000000000000p31"),
            (64, "-0x1.0000000000000002000000000000p63"),
        ] {
            assert_eq!(
                expected,
                Ieee128::fcvt_to_sint_negative_overflow(n).to_string(),
                "n = {n}"
            );
        }
    }
}
