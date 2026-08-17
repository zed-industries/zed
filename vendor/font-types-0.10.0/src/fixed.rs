//! fixed-point numerical types

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// shared between Fixed, F26Dot6, F2Dot14, F4Dot12, F6Dot10
macro_rules! fixed_impl {
    ($name:ident, $bits:literal, $fract_bits:literal, $ty:ty) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern, bytemuck::NoUninit))]
        #[repr(transparent)]
        #[doc = concat!(stringify!($bits), "-bit signed fixed point number with ", stringify!($fract_bits), " bits of fraction." )]
        pub struct $name($ty);
        impl $name {
            /// Minimum value.
            pub const MIN: Self = Self(<$ty>::MIN);

            /// Maximum value.
            pub const MAX: Self = Self(<$ty>::MAX);

            /// This type's smallest representable value
            pub const EPSILON: Self = Self(1);

            /// Representation of 0.0.
            pub const ZERO: Self = Self(0);

            /// Representation of 1.0.
            pub const ONE: Self = Self(1 << $fract_bits);

            const INT_MASK: $ty = !0 << $fract_bits;
            const ROUND: $ty = 1 << ($fract_bits - 1);
            const FRACT_BITS: usize = $fract_bits;

            /// Creates a new fixed point value from the underlying bit representation.
            #[inline(always)]
            pub const fn from_bits(bits: $ty) -> Self {
                Self(bits)
            }

            /// Returns the underlying bit representation of the value.
            #[inline(always)]
            pub const fn to_bits(self) -> $ty {
                self.0
            }

            //TODO: is this actually useful?
            /// Returns the nearest integer value.
            #[inline(always)]
            pub const fn round(self) -> Self {
                Self(self.0.wrapping_add(Self::ROUND) & Self::INT_MASK)
            }

            /// Returns the absolute value of the number.
            #[inline(always)]
            pub const fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns the largest integer less than or equal to the number.
            #[inline(always)]
            pub const fn floor(self) -> Self {
                Self(self.0 & Self::INT_MASK)
            }

            /// Returns the fractional part of the number.
            #[inline(always)]
            pub const fn fract(self) -> Self {
                Self(self.0 - self.floor().0)
            }

            /// Wrapping addition.
            #[inline(always)]
            pub fn wrapping_add(self, other: Self) -> Self {
                Self(self.0.wrapping_add(other.0))
            }

            /// Saturating addition.
            #[inline(always)]
            pub const fn saturating_add(self, other: Self) -> Self {
                Self(self.0.saturating_add(other.0))
            }

            /// Checked addition.
            #[inline(always)]
            pub fn checked_add(self, other: Self) -> Option<Self> {
                self.0.checked_add(other.0).map(|inner| Self(inner))
            }

            /// Wrapping substitution.
            #[inline(always)]
            pub const fn wrapping_sub(self, other: Self) -> Self {
                Self(self.0.wrapping_sub(other.0))
            }

            /// Saturating substitution.
            #[inline(always)]
            pub const fn saturating_sub(self, other: Self) -> Self {
                Self(self.0.saturating_sub(other.0))
            }

            /// The representation of this number as a big-endian byte array.
            #[inline(always)]
            pub const fn to_be_bytes(self) -> [u8; $bits / 8] {
                self.0.to_be_bytes()
            }
        }

        impl Add for $name {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: Self) -> Self {
                Self(self.0.wrapping_add(other.0))
            }
        }

        impl AddAssign for $name {
            #[inline(always)]
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        impl Sub for $name {
            type Output = Self;
            #[inline(always)]
            fn sub(self, other: Self) -> Self {
                Self(self.0.wrapping_sub(other.0))
            }
        }

        impl SubAssign for $name {
            #[inline(always)]
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }
    };
}

/// Implements multiplication and division operators for fixed types.
macro_rules! fixed_mul_div {
    ($ty:ty) => {
        impl $ty {
            /// Multiplies `self` by `a` and divides the product by `b`.
            // This one is specifically not always inlined due to size and
            // frequency of use. We leave it to compiler discretion.
            #[inline]
            pub const fn mul_div(&self, a: Self, b: Self) -> Self {
                let mut sign = 1;
                let mut su = self.0 as u64;
                let mut au = a.0 as u64;
                let mut bu = b.0 as u64;
                if self.0 < 0 {
                    su = 0u64.wrapping_sub(su);
                    sign = -1;
                }
                if a.0 < 0 {
                    au = 0u64.wrapping_sub(au);
                    sign = -sign;
                }
                if b.0 < 0 {
                    bu = 0u64.wrapping_sub(bu);
                    sign = -sign;
                }
                let result = if bu > 0 {
                    su.wrapping_mul(au).wrapping_add(bu >> 1) / bu
                } else {
                    0x7FFFFFFF
                };
                Self(if sign < 0 {
                    -(result as i32)
                } else {
                    result as i32
                })
            }
        }

        impl Mul for $ty {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: Self) -> Self::Output {
                let ab = self.0 as i64 * other.0 as i64;
                Self(((ab + 0x8000 - i64::from(ab < 0)) >> 16) as i32)
            }
        }

        impl MulAssign for $ty {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl Div for $ty {
            type Output = Self;
            #[inline(always)]
            fn div(self, other: Self) -> Self::Output {
                let mut sign = 1;
                let mut a = self.0;
                let mut b = other.0;
                if a < 0 {
                    a = -a;
                    sign = -1;
                }
                if b < 0 {
                    b = -b;
                    sign = -sign;
                }
                let q = if b == 0 {
                    0x7FFFFFFF
                } else {
                    ((((a as u64) << 16) + ((b as u64) >> 1)) / (b as u64)) as u32
                };
                Self(if sign < 0 { -(q as i32) } else { q as i32 })
            }
        }

        impl DivAssign for $ty {
            #[inline(always)]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }

        impl Neg for $ty {
            type Output = Self;
            #[inline(always)]
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }
    };
}

/// impl float conversion methods.
///
/// We convert to different float types in order to ensure we can roundtrip
/// without floating point error.
macro_rules! float_conv {
    ($name:ident, $to:ident, $from:ident, $ty:ty) => {
        impl $name {
            #[doc = concat!("Creates a fixed point value from a", stringify!($ty), ".")]
            ///
            /// This operation is lossy; the float will be rounded to the nearest
            /// representable value.
            #[inline(always)]
            pub fn $from(x: $ty) -> Self {
                // When x is positive: 1.0 - 0.5 =  0.5
                // When x is negative: 0.0 - 0.5 = -0.5
                let frac = (x.is_sign_positive() as u8 as $ty) - 0.5;
                Self((x * Self::ONE.0 as $ty + frac) as _)
            }

            #[doc = concat!("Returns the value as an ", stringify!($ty), ".")]
            ///
            /// This operation is lossless: all representable values can be
            /// round-tripped.
            #[inline(always)]
            pub fn $to(self) -> $ty {
                let int = ((self.0 & Self::INT_MASK) >> Self::FRACT_BITS) as $ty;
                let fract = (self.0 & !Self::INT_MASK) as $ty / Self::ONE.0 as $ty;
                int + fract
            }
        }

        //hack: we can losslessly go to float, so use those fmt impls
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.$to().fmt(f)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.$to().fmt(f)
            }
        }
    };
}

fixed_impl!(F2Dot14, 16, 14, i16);
fixed_impl!(F4Dot12, 16, 12, i16);
fixed_impl!(F6Dot10, 16, 10, i16);
fixed_impl!(Fixed, 32, 16, i32);
fixed_impl!(F26Dot6, 32, 6, i32);
fixed_mul_div!(Fixed);
fixed_mul_div!(F26Dot6);
float_conv!(F2Dot14, to_f32, from_f32, f32);
float_conv!(F4Dot12, to_f32, from_f32, f32);
float_conv!(F6Dot10, to_f32, from_f32, f32);
float_conv!(Fixed, to_f64, from_f64, f64);
float_conv!(F26Dot6, to_f64, from_f64, f64);
crate::newtype_scalar!(F2Dot14, [u8; 2]);
crate::newtype_scalar!(F4Dot12, [u8; 2]);
crate::newtype_scalar!(F6Dot10, [u8; 2]);
crate::newtype_scalar!(Fixed, [u8; 4]);

impl Fixed {
    /// Creates a 16.16 fixed point value from a 32 bit integer.
    #[inline(always)]
    pub const fn from_i32(i: i32) -> Self {
        Self(i << 16)
    }

    /// Converts a 16.16 fixed point value to a 32 bit integer, rounding off
    /// the fractional bits.
    #[inline(always)]
    pub const fn to_i32(self) -> i32 {
        self.0.wrapping_add(0x8000) >> 16
    }

    /// Converts a 16.16 to 26.6 fixed point value.
    #[inline(always)]
    pub const fn to_f26dot6(self) -> F26Dot6 {
        F26Dot6(self.0.wrapping_add(0x200) >> 10)
    }

    /// Converts a 16.16 to 2.14 fixed point value.
    ///
    /// This specific conversion is defined by the spec:
    /// <https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview#coordinate-scales-and-normalization>
    ///
    /// "5. Convert the final, normalized 16.16 coordinate value to 2.14 by this method: add 0x00000002,
    /// and sign-extend shift to the right by 2."
    #[inline(always)]
    pub const fn to_f2dot14(self) -> F2Dot14 {
        F2Dot14((self.0.wrapping_add(2) >> 2) as _)
    }

    /// Converts a 16.16 fixed point value to a single precision floating
    /// point value.
    ///
    /// This operation is lossy. Use `to_f64()` for a lossless conversion.
    #[inline(always)]
    pub fn to_f32(self) -> f32 {
        const SCALE_FACTOR: f32 = 1.0 / 65536.0;
        self.0 as f32 * SCALE_FACTOR
    }
}

impl From<i32> for Fixed {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl F26Dot6 {
    /// Creates a 26.6 fixed point value from a 32 bit integer.
    #[inline(always)]
    pub const fn from_i32(i: i32) -> Self {
        Self(i << 6)
    }

    /// Converts a 26.6 fixed point value to a 32 bit integer, rounding off
    /// the fractional bits.
    #[inline(always)]
    pub const fn to_i32(self) -> i32 {
        self.0.wrapping_add(32) >> 6
    }

    /// Converts a 26.6 fixed point value to a single precision floating
    /// point value.
    ///
    /// This operation is lossy. Use `to_f64()` for a lossless conversion.
    #[inline(always)]
    pub fn to_f32(self) -> f32 {
        const SCALE_FACTOR: f32 = 1.0 / 64.0;
        self.0 as f32 * SCALE_FACTOR
    }
}

impl F2Dot14 {
    /// Converts a 2.14 to 16.16 fixed point value.
    #[inline(always)]
    pub const fn to_fixed(self) -> Fixed {
        Fixed(self.0 as i32 * 4)
    }
}

#[cfg(test)]
mod tests {
    #![allow(overflowing_literals)] // we want to specify byte values directly
    use super::*;

    #[test]
    fn f2dot14_floats() {
        // Examples from https://docs.microsoft.com/en-us/typography/opentype/spec/otff#data-types
        assert_eq!(F2Dot14(0x7fff), F2Dot14::from_f32(1.999939));
        assert_eq!(F2Dot14(0x7000), F2Dot14::from_f32(1.75));
        assert_eq!(F2Dot14(0x0001), F2Dot14::from_f32(0.0000610356));
        assert_eq!(F2Dot14(0x0000), F2Dot14::from_f32(0.0));
        assert_eq!(F2Dot14(0xffff), F2Dot14::from_f32(-0.000061));
        assert_eq!(F2Dot14(0x8000), F2Dot14::from_f32(-2.0));
    }

    #[test]
    fn roundtrip_f2dot14() {
        for i in i16::MIN..=i16::MAX {
            let val = F2Dot14(i);
            assert_eq!(val, F2Dot14::from_f32(val.to_f32()));
        }
    }

    #[test]
    fn round_f2dot14() {
        assert_eq!(F2Dot14(0x7000).round(), F2Dot14::from_f32(-2.0));
        assert_eq!(F2Dot14(0x1F00).round(), F2Dot14::from_f32(0.0));
        assert_eq!(F2Dot14(0x2000).round(), F2Dot14::from_f32(1.0));
    }

    #[test]
    fn round_fixed() {
        //TODO: make good test cases
        assert_eq!(Fixed(0x0001_7FFE).round(), Fixed(0x0001_0000));
        assert_eq!(Fixed(0x0001_7FFF).round(), Fixed(0x0001_0000));
        assert_eq!(Fixed(0x0001_8000).round(), Fixed(0x0002_0000));
    }

    // disabled because it's slow; these were just for my edification anyway
    //#[test]
    //fn roundtrip_fixed() {
    //for i in i32::MIN..=i32::MAX {
    //let val = Fixed(i);
    //assert_eq!(val, Fixed::from_f64(val.to_f64()));
    //}
    //}

    #[test]
    fn fixed_floats() {
        assert_eq!(Fixed(0x7fff_0000), Fixed::from_f64(32767.));
        assert_eq!(Fixed(0x7000_0001), Fixed::from_f64(28672.00001525879));
        assert_eq!(Fixed(0x0001_0000), Fixed::from_f64(1.0));
        assert_eq!(Fixed(0x0000_0000), Fixed::from_f64(0.0));
        assert_eq!(
            Fixed(i32::from_be_bytes([0xff; 4])),
            Fixed::from_f64(-0.000015259)
        );
        assert_eq!(Fixed(0x7fff_ffff), Fixed::from_f64(32768.0));
    }

    // We lost the f64::round() intrinsic when dropping std and the
    // alternative implementation was very slightly incorrect, throwing
    // off some tests. This makes sure we match.
    #[test]
    fn fixed_floats_rounding() {
        fn with_round_intrinsic(x: f64) -> Fixed {
            Fixed((x * 65536.0).round() as i32)
        }
        // These particular values were tripping up tests
        let inputs = [0.05, 0.6, 0.2, 0.4, 0.67755];
        for input in inputs {
            assert_eq!(Fixed::from_f64(input), with_round_intrinsic(input));
            // Test negated values as well for good measure
            assert_eq!(Fixed::from_f64(-input), with_round_intrinsic(-input));
        }
    }

    #[test]
    fn fixed_to_int() {
        assert_eq!(Fixed::from_f64(1.0).to_i32(), 1);
        assert_eq!(Fixed::from_f64(1.5).to_i32(), 2);
        assert_eq!(F26Dot6::from_f64(1.0).to_i32(), 1);
        assert_eq!(F26Dot6::from_f64(1.5).to_i32(), 2);
    }

    #[test]
    fn fixed_from_int() {
        assert_eq!(Fixed::from_i32(1000).to_bits(), 1000 << 16);
        assert_eq!(F26Dot6::from_i32(1000).to_bits(), 1000 << 6);
    }

    #[test]
    fn fixed_to_f26dot6() {
        assert_eq!(Fixed::from_f64(42.5).to_f26dot6(), F26Dot6::from_f64(42.5));
    }

    #[test]
    fn fixed_muldiv() {
        assert_eq!(
            Fixed::from_f64(0.5) * Fixed::from_f64(2.0),
            Fixed::from_f64(1.0)
        );
        assert_eq!(
            Fixed::from_f64(0.5) / Fixed::from_f64(2.0),
            Fixed::from_f64(0.25)
        );
    }
}
