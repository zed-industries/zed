#![allow(missing_docs)]
#![allow(non_camel_case_types)] // For the simd type aliases.

//! Traits for SIMD values.

use crate::scalar::{ComplexField, Field, SubsetOf, SupersetOf};
use crate::simd::{
    PrimitiveSimdValue, SimdBool, SimdComplexField, SimdPartialOrd, SimdRealField, SimdSigned,
    SimdValue,
};
use approx::AbsDiffEq;
#[cfg(feature = "decimal")]
use decimal::d128;
use num::{FromPrimitive, Num, One, Zero};
use std::{
    fmt,
    ops::{
        Add, AddAssign, BitAnd, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem,
        RemAssign, Sub, SubAssign,
    },
    simd::{
        self as portable_simd, cmp::SimdOrd, cmp::SimdPartialEq,
        cmp::SimdPartialOrd as PortableSimdPartialOrd, num::SimdFloat, num::SimdInt, num::SimdUint,
        StdFloat,
    },
};

// This is a hack to allow use to reuse `_0` as integers or as identifier,
// depending on whether or not `ident_to_value` has been called in scope.
// This helps writing macros that define both `::new` and `From([T; lanes()])`.
macro_rules! ident_to_value (
    () => {
        const _0: usize = 0; const _1: usize = 1; const _2: usize = 2; const _3: usize = 3; const _4: usize = 4; const _5: usize = 5; const _6: usize = 6; const _7: usize = 7;
        const _8: usize = 8; const _9: usize = 9; const _10: usize = 10; const _11: usize = 11; const _12: usize = 12; const _13: usize = 13; const _14: usize = 14; const _15: usize = 15;
        const _16: usize = 16; const _17: usize = 17; const _18: usize = 18; const _19: usize = 19; const _20: usize = 20; const _21: usize = 21; const _22: usize = 22; const _23: usize = 23;
        const _24: usize = 24; const _25: usize = 25; const _26: usize = 26; const _27: usize = 27; const _28: usize = 28; const _29: usize = 29; const _30: usize = 30; const _31: usize = 31;
        const _32: usize = 32; const _33: usize = 33; const _34: usize = 34; const _35: usize = 35; const _36: usize = 36; const _37: usize = 37; const _38: usize = 38; const _39: usize = 39;
        const _40: usize = 40; const _41: usize = 41; const _42: usize = 42; const _43: usize = 43; const _44: usize = 44; const _45: usize = 45; const _46: usize = 46; const _47: usize = 47;
        const _48: usize = 48; const _49: usize = 49; const _50: usize = 50; const _51: usize = 51; const _52: usize = 52; const _53: usize = 53; const _54: usize = 54; const _55: usize = 55;
        const _56: usize = 56; const _57: usize = 57; const _58: usize = 58; const _59: usize = 59; const _60: usize = 60; const _61: usize = 61; const _62: usize = 62; const _63: usize = 63;
    }
);

/// An Simd structure that implements all the relevant traits from `num` an `simba`.
///
/// This is needed to overcome the orphan rules.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Simd<N>(pub N);

macro_rules! impl_bool_simd (
    ($($t: ty, $lanes: literal, $($i: ident),*;)*) => {$(
        impl fmt::Display for Simd<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if Self::LANES == 1 {
                    return self.extract(0).fmt(f);
                }

                write!(f, "({}", self.extract(0))?;

                for i in 1..Self::LANES {
                    write!(f, ", {}", self.extract(i))?;
                }

                write!(f, ")")
            }
        }

        impl Simd<$t> {
            #[inline]
            pub fn new($($i: bool),*) -> Self {
                Simd(<$t>::from_array([$($i),*]))
            }
        }

        impl PrimitiveSimdValue for Simd<$t> {}

        impl SimdValue for Simd<$t> {
            const LANES: usize = $lanes;
            type Element = bool;
            type SimdBool = Simd<$t>;

            #[inline(always)]
            fn splat(val: Self::Element) -> Self {
                Simd(<$t>::splat(val))
            }

            #[inline(always)]
            fn extract(&self, i: usize) -> Self::Element {
                self.0.test(i)
            }

            #[inline(always)]
            unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
                self.0.test_unchecked(i)
            }

            #[inline(always)]
            fn replace(&mut self, i: usize, val: Self::Element) {
                self.0.set(i, val)
            }

            #[inline(always)]
            unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
                self.0.set_unchecked(i, val)
            }

            #[inline(always)]
            fn select(self, cond: Self::SimdBool, other: Self) -> Self {
                let x = cond.0.select(self.0.to_int(), other.0.to_int());
                Self(<$t>::from_int(x))
            }
        }

        impl From<[bool; $lanes]> for Simd<$t> {
            #[inline(always)]
            fn from(vals: [bool; $lanes]) -> Self {
                ident_to_value!();
                Simd(<$t>::from_array(vals))
            }
        }

        impl Not for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
                Self(!self.0)
            }
        }

        impl BitAnd<Simd<$t>> for Simd<$t> {
            type Output = Self;
            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                Simd(self.0.bitand(rhs.0))
            }
        }

        impl BitOr<Simd<$t>> for Simd<$t> {
            type Output = Self;
            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                Simd(self.0.bitor(rhs.0))
            }
        }

        impl BitXor<Simd<$t>> for Simd<$t> {
            type Output = Self;
            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self {
                Simd(self.0.bitxor(rhs.0))
            }
        }

        impl SimdBool for Simd<$t> {
            #[inline(always)]
            fn bitmask(self) -> u64 {
                self.0.to_bitmask() as u64
            }

            #[inline(always)]
            fn and(self) -> bool {
                self.0.to_int().reduce_and() != 0
            }

            #[inline(always)]
            fn or(self) -> bool {
                self.0.to_int().reduce_or() != 0
            }

            #[inline(always)]
            fn xor(self) -> bool {
                self.0.to_int().reduce_xor() != 0
            }

            #[inline(always)]
            fn all(self) -> bool {
                self.0.all()
            }

            #[inline(always)]
            fn any(self) -> bool {
                self.0.any()
            }

            #[inline(always)]
            fn none(self) -> bool {
                !self.0.any()
            }

            #[inline(always)]
            fn if_else<Res: SimdValue<SimdBool = Self>>(
                self,
                if_value: impl FnOnce() -> Res,
                else_value: impl FnOnce() -> Res,
            ) -> Res {
                let a = if_value();
                let b = else_value();
                a.select(self, b)
            }

            #[inline(always)]
            fn if_else2<Res: SimdValue<SimdBool = Self>>(
                self,
                if_value: impl FnOnce() -> Res,
                else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res),
                else_value: impl FnOnce() -> Res,
            ) -> Res {
                let a = if_value();
                let b = else_if.1();
                let c = else_value();

                let cond_a = self;
                let cond_b = else_if.0();

                a.select(cond_a, b.select(cond_b, c))
            }

            #[inline(always)]
            fn if_else3<Res: SimdValue<SimdBool = Self>>(
                self,
                if_value: impl FnOnce() -> Res,
                else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res),
                else_else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res),
                else_value: impl FnOnce() -> Res,
            ) -> Res {
                let a = if_value();
                let b = else_if.1();
                let c = else_else_if.1();
                let d = else_value();

                let cond_a = self;
                let cond_b = else_if.0();
                let cond_c = else_else_if.0();

                a.select(cond_a, b.select(cond_b, c.select(cond_c, d)))
            }
        }
    )*}
);

macro_rules! impl_scalar_subset_of_simd (
    ($($t: ty),*) => {$(
        impl<N2> SubsetOf<Simd<N2>> for $t
            where Simd<N2>: SimdValue + Copy,
                  <Simd<N2> as SimdValue>::Element: SupersetOf<$t> + PartialEq, {
            #[inline(always)]
            fn to_superset(&self) -> Simd<N2> {
                Simd::<N2>::splat(<Simd<N2> as SimdValue>::Element::from_subset(self))
            }

            #[inline(always)]
            fn from_superset_unchecked(element: &Simd<N2>) -> $t {
                element.extract(0).to_subset_unchecked()
            }

            #[inline(always)]
            fn is_in_subset(c: &Simd<N2>) -> bool {
                let elt0 = c.extract(0);
                elt0.is_in_subset() &&
                (1..Simd::<N2>::LANES).all(|i| c.extract(i) == elt0)
            }
        }
    )*}
);

impl_scalar_subset_of_simd!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
#[cfg(feature = "decimal")]
impl_scalar_subset_of_simd!(d128);

macro_rules! impl_simd_value (
    ($($t: ty, $elt: ty, $bool: ty, $($i: ident),*;)*) => ($(
        impl fmt::Display for Simd<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if Self::LANES == 1 {
                    return self.extract(0).fmt(f);
                }

                write!(f, "({}", self.extract(0))?;

                for i in 1..Self::LANES {
                    write!(f, ", {}", self.extract(i))?;
                }

                write!(f, ")")
            }
        }

        impl Simd<$t> {
            pub const ZERO: Self = Simd(<$t>::from_array([0 as $elt; <$t>::LEN]));
            pub const ONE: Self = Simd(<$t>::from_array([1 as $elt; <$t>::LEN]));

            #[inline]
            pub fn new($($i: $elt),*) -> Self {
                Simd(<$t>::from_array([$($i),*]))
            }
        }

        impl PrimitiveSimdValue for Simd<$t> {}

        impl SimdValue for Simd<$t> {
            const LANES: usize = <$t>::LEN;
            type Element = $elt;
            type SimdBool = $bool;

            #[inline(always)]
            fn splat(val: Self::Element) -> Self {
                Simd(<$t>::splat(val))
            }

            #[inline(always)]
            fn extract(&self, i: usize) -> Self::Element {
                self.0[i]
            }

            #[inline(always)]
            unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
                self.0[i]
            }

            #[inline(always)]
            fn replace(&mut self, i: usize, val: Self::Element) {
                self.0[i] = val;
            }

            #[inline(always)]
            unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
                self.0[i] = val;
            }

            #[inline(always)]
            fn select(self, cond: Self::SimdBool, other: Self) -> Self {
                Self(cond.0.select(self.0, other.0))
            }
        }
    )*)
);

macro_rules! impl_uint_simd (
    ($($t: ty, $elt: ty, $bool: ty, $($i: ident),*;)*) => ($(
        impl_simd_value!($t, $elt, $bool $(, $i)*;);

        impl Simd<$t> {
            /// Instantiates a new vector with the values of the `slice`.
            ///
            /// # Panics
            ///
            /// If `slice.len() < Self::LANES`.
            #[inline]
            pub fn from_slice_unaligned(slice: &[$elt]) -> Self {
                Simd(<$t>::from_slice(slice))
            }
        }

        impl From<[$elt; <$t>::LEN]> for Simd<$t> {
            #[inline(always)]
            fn from(vals: [$elt; <$t>::LEN]) -> Self {
                Simd(<$t>::from(vals))
            }
        }

        impl From<Simd<$t>> for [$elt; <$t>::LEN] {
            #[inline(always)]
            fn from(val: Simd<$t>) -> [$elt; <$t>::LEN] {
                val.0.to_array()
            }
        }

        impl SubsetOf<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn to_superset(&self) -> Self {
                *self
            }

            #[inline(always)]
            fn from_superset(element: &Self) -> Option<Self> {
                Some(*element)
            }

            #[inline(always)]
            fn from_superset_unchecked(element: &Self) -> Self {
                *element
            }

            #[inline(always)]
            fn is_in_subset(_: &Self) -> bool {
                true
            }
        }

        impl Num for Simd<$t> {
            type FromStrRadixErr = <$elt as Num>::FromStrRadixErr;

            #[inline(always)]
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                <$elt>::from_str_radix(str, radix).map(Self::splat)
            }
        }

        impl FromPrimitive for Simd<$t> {
            #[inline(always)]
            fn from_i64(n: i64) -> Option<Self> {
                <$elt>::from_i64(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u64(n: u64) -> Option<Self> {
                <$elt>::from_u64(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_isize(n: isize) -> Option<Self>  {
                <$elt>::from_isize(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_i8(n: i8) -> Option<Self>  {
                <$elt>::from_i8(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_i16(n: i16) -> Option<Self>  {
                <$elt>::from_i16(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_i32(n: i32) -> Option<Self>  {
                <$elt>::from_i32(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_usize(n: usize) -> Option<Self>  {
                <$elt>::from_usize(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u8(n: u8) -> Option<Self>  {
                <$elt>::from_u8(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u16(n: u16) -> Option<Self>  {
                <$elt>::from_u16(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u32(n: u32) -> Option<Self>  {
                <$elt>::from_u32(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_f32(n: f32) -> Option<Self>  {
                <$elt>::from_f32(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_f64(n: f64) -> Option<Self>  {
                <$elt>::from_f64(n).map(Self::splat)
            }
        }


        impl Zero for Simd<$t> {
            #[inline(always)]
            fn zero() -> Self {
                Simd(<$t>::splat(<$elt>::zero()))
            }

            #[inline(always)]
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }

        impl One for Simd<$t> {
            #[inline(always)]
            fn one() -> Self {
                Simd(<$t>::splat(<$elt>::one()))
            }
        }

        impl Add<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl Sub<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl Mul<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                Self(self.0 * rhs.0)
            }
        }

        impl Div<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                Self(self.0 / rhs.0)
            }
        }

        impl Rem<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn rem(self, rhs: Self) -> Self {
                Self(self.0 % rhs.0)
            }
        }

        impl AddAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl SubAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl DivAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0
            }
        }

        impl MulAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0
            }
        }

        impl RemAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0
            }
        }

        impl SimdPartialOrd for Simd<$t> {
            #[inline(always)]
            fn simd_gt(self, other: Self) -> Self::SimdBool {
                Simd(self.0.simd_gt(other.0))
            }

            #[inline(always)]
            fn simd_lt(self, other: Self) -> Self::SimdBool {
                Simd(self.0.simd_lt(other.0))
            }

            #[inline(always)]
            fn simd_ge(self, other: Self) -> Self::SimdBool {
                Simd(self.0.simd_ge(other.0))
            }

            #[inline(always)]
            fn simd_le(self, other: Self) -> Self::SimdBool {
                Simd(self.0.simd_le(other.0))
            }

            #[inline(always)]
            fn simd_eq(self, other: Self) -> Self::SimdBool {
                Simd(self.0.simd_eq(other.0))
            }

            #[inline(always)]
            fn simd_ne(self, other: Self) -> Self::SimdBool {
                Simd(self.0.simd_ne(other.0))
            }

            #[inline(always)]
            fn simd_max(self, other: Self) -> Self {
                Simd(self.0.simd_max(other.0))
            }
            #[inline(always)]
            fn simd_min(self, other: Self) -> Self {
                Simd(self.0.simd_min(other.0))
            }

            #[inline(always)]
            fn simd_clamp(self, min: Self, max: Self) -> Self {
                self.simd_max(min).simd_min(max)
            }

            #[inline(always)]
            fn simd_horizontal_min(self) -> Self::Element {
                self.0.reduce_min()
            }

            #[inline(always)]
            fn simd_horizontal_max(self) -> Self::Element {
                self.0.reduce_max()
            }
        }

//        impl MeetSemilattice for Simd<$t> {
//            #[inline(always)]
//            fn meet(&self, other: &Self) -> Self {
//                Simd(self.0.min(other.0))
//            }
//        }
//
//        impl JoinSemilattice for Simd<$t> {
//            #[inline(always)]
//            fn join(&self, other: &Self) -> Self {
//                Simd(self.0.max(other.0))
//            }
//        }
    )*)
);

macro_rules! impl_int_simd (
    ($($t: ty, $elt: ty, $bool: ty, $($i: ident),*;)*) => ($(
        impl_uint_simd!($t, $elt, $bool $(, $i)*;);

        impl Neg for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }
    )*)
);

macro_rules! impl_float_simd (
    ($($t: ty, $elt: ident, $int: ty, $bool: ty, $($i: ident),*;)*) => ($(
        impl_int_simd!($t, $elt, $bool $(, $i)*;);

        // TODO: this should be part of impl_int_simd
        // but those methods do not seem to be implemented
        // by portable_simd for integers.
        impl SimdSigned for Simd<$t> {
            #[inline(always)]
            fn simd_abs(&self) -> Self {
                Simd(self.0.abs())
            }

            #[inline(always)]
            fn simd_abs_sub(&self, other: &Self) -> Self {
                Simd((self.0 - other.0).simd_max(Self::zero().0))
            }

            #[inline(always)]
            fn simd_signum(&self) -> Self {
                // NOTE: is there a more efficient way of doing this?
                let zero = Self::zero().0;
                let one = Self::one().0;
                let gt = self.0.simd_gt(zero);
                let lt = self.0.simd_lt(zero);
                Simd(lt.select(-one, gt.select(one, zero)))
            }

            #[inline(always)]
            fn is_simd_positive(&self) -> Self::SimdBool {
                self.simd_gt(Self::zero())
            }

            #[inline(always)]
            fn is_simd_negative(&self) -> Self::SimdBool {
                self.simd_lt(Self::zero())
            }
        }

        impl Field for Simd<$t> {}

        impl SimdRealField for Simd<$t> {
            #[inline(always)]
            fn simd_atan2(self, other: Self) -> Self {
                self.zip_map_lanes(other, |a, b| a.atan2(b))
            }

            #[inline(always)]
            fn simd_copysign(self, sign: Self) -> Self {
                Simd(self.0.copysign(sign.0))
            }

            #[inline(always)]
            fn simd_default_epsilon() -> Self {
                Self::splat(<$elt>::default_epsilon())
            }

            #[inline(always)]
            fn simd_pi() -> Self {
                use std::$elt::consts::*;
                Self::splat(PI)
            }

            #[inline(always)]
            fn simd_two_pi() -> Self {
                use std::$elt::consts::*;
                Self::splat(PI + PI)
            }

            #[inline(always)]
            fn simd_frac_pi_2() -> Self {
                use std::$elt::consts::*;
                Self::splat(FRAC_PI_2)
            }

            #[inline(always)]
            fn simd_frac_pi_3() -> Self {
                use std::$elt::consts::*;
                Self::splat(FRAC_PI_3)
            }

            #[inline(always)]
            fn simd_frac_pi_4() -> Self {
                use std::$elt::consts::*;
                Self::splat(FRAC_PI_4)
            }

            #[inline(always)]
            fn simd_frac_pi_6() -> Self {
                use std::$elt::consts::*;
                Self::splat(FRAC_PI_6)
            }

            #[inline(always)]
            fn simd_frac_pi_8() -> Self {
                use std::$elt::consts::*;
                Self::splat(FRAC_PI_8)
            }

            #[inline(always)]
            fn simd_frac_1_pi() -> Self {
                use std::$elt::consts::*;
                Self::splat(FRAC_1_PI)
            }

            #[inline(always)]
            fn simd_frac_2_pi() -> Self {
                use std::$elt::consts::*;
                Self::splat(FRAC_2_PI)
            }

            #[inline(always)]
            fn simd_frac_2_sqrt_pi() -> Self {
                use std::$elt::consts::*;
                Self::splat(FRAC_2_SQRT_PI)
            }

            #[inline(always)]
            fn simd_e() -> Self {
                use std::$elt::consts::*;
                Self::splat(E)
            }

            #[inline(always)]
            fn simd_log2_e() -> Self {
                use std::$elt::consts::*;
                Self::splat(LOG2_E)
            }

            #[inline(always)]
            fn simd_log10_e() -> Self {
                use std::$elt::consts::*;
                Self::splat(LOG10_E)
            }

            #[inline(always)]
            fn simd_ln_2() -> Self {
                use std::$elt::consts::*;
                Self::splat(LN_2)
            }

            #[inline(always)]
            fn simd_ln_10() -> Self {
                use std::$elt::consts::*;
                Self::splat(LN_10)
            }
        }

        impl SimdComplexField for Simd<$t> {
            type SimdRealField = Self;

            #[inline(always)]
            fn simd_horizontal_sum(self) -> Self::Element {
                self.0.reduce_sum()
            }

            #[inline(always)]
            fn simd_horizontal_product(self) -> Self::Element {
                self.0.reduce_product()
            }

            #[inline(always)]
            fn from_simd_real(re: Self::SimdRealField) -> Self {
                re
            }

            #[inline(always)]
            fn simd_real(self) -> Self::SimdRealField {
                self
            }

            #[inline(always)]
            fn simd_imaginary(self) -> Self::SimdRealField {
                Self::zero()
            }

            #[inline(always)]
            fn simd_norm1(self) -> Self::SimdRealField {
                Simd(self.0.abs())
            }

            #[inline(always)]
            fn simd_modulus(self) -> Self::SimdRealField {
                Simd(self.0.abs())
            }

            #[inline(always)]
            fn simd_modulus_squared(self) -> Self::SimdRealField {
                self * self
            }

            #[inline(always)]
            fn simd_argument(self) -> Self::SimdRealField {
                self.map_lanes(|e| e.argument())
            }

            #[inline(always)]
            fn simd_to_exp(self) -> (Self::SimdRealField, Self) {
                let ge = self.0.simd_ge(Self::one().0);
                let exp = ge.select(Self::one().0, -Self::one().0);
                (Simd(self.0 * exp), Simd(exp))
            }

            #[inline(always)]
            fn simd_recip(self) -> Self {
                Self(self.0.recip())
            }

            #[inline(always)]
            fn simd_conjugate(self) -> Self {
                self
            }

            #[inline(always)]
            fn simd_scale(self, factor: Self::SimdRealField) -> Self {
                Simd(self.0 * factor.0)
            }

            #[inline(always)]
            fn simd_unscale(self, factor: Self::SimdRealField) -> Self {
                Simd(self.0 / factor.0)
            }

            #[inline(always)]
            fn simd_floor(self) -> Self {
                Self(self.0.floor())
            }

            #[inline(always)]
            fn simd_ceil(self) -> Self {
                Self(self.0.ceil())
            }

            #[inline(always)]
            fn simd_round(self) -> Self {
                Self(self.0.round())
            }

            #[inline(always)]
            fn simd_trunc(self) -> Self {
                Self(self.0.trunc())
            }

            #[inline(always)]
            fn simd_fract(self) -> Self {
                Self(self.0.fract())
            }

            #[inline(always)]
            fn simd_abs(self) -> Self {
                Self(self.0.abs())
            }

            #[inline(always)]
            fn simd_signum(self) -> Self {
                Self(self.0.signum())
            }

            #[inline(always)]
            fn simd_mul_add(self, a: Self, b: Self) -> Self {
                Simd(self.0.mul_add(a.0, b.0))
            }

            #[inline(always)]
            fn simd_powi(self, n: i32) -> Self {
               self.map_lanes(|e| e.powi(n))
            }

            #[inline(always)]
            fn simd_powf(self, n: Self) -> Self {
               self.zip_map_lanes(n, |e, p| e.powf(p))
            }

            #[inline(always)]
            fn simd_powc(self, n: Self) -> Self {
                self.simd_powf(n)
            }

            #[inline(always)]
            fn simd_sqrt(self) -> Self {
                Simd(self.0.sqrt())
            }

            #[inline(always)]
            fn simd_exp(self) -> Self {
                Self(self.0.exp())
            }

            #[inline(always)]
            fn simd_exp2(self) -> Self {
                Self(self.0.exp2())
            }

            #[inline(always)]
            fn simd_exp_m1(self) -> Self {
                self.map_lanes(|e| e.exp_m1())
            }

            #[inline(always)]
            fn simd_ln_1p(self) -> Self {
                self.map_lanes(|e| e.ln_1p())
            }

            #[inline(always)]
            fn simd_ln(self) -> Self {
                Self(self.0.ln())
            }

            #[inline(always)]
            fn simd_log(self, base: Self) -> Self {
                self.zip_map_lanes(base, |e, b| e.log(b))
            }

            #[inline(always)]
            fn simd_log2(self) -> Self {
                Self(self.0.log2())
            }

            #[inline(always)]
            fn simd_log10(self) -> Self {
                Self(self.0.log10())
            }

            #[inline(always)]
            fn simd_cbrt(self) -> Self {
                self.map_lanes(|e| e.cbrt())
            }

            #[inline(always)]
            fn simd_hypot(self, other: Self) -> Self::SimdRealField {
                self.zip_map_lanes(other, |e, o| e.hypot(o))
            }

            #[inline(always)]
            fn simd_sin(self) -> Self {
                Self(self.0.sin())
            }

            #[inline(always)]
            fn simd_cos(self) -> Self {
                Self(self.0.cos())
            }

            #[inline(always)]
            fn simd_tan(self) -> Self {
                self.map_lanes(|e| e.tan())
            }

            #[inline(always)]
            fn simd_asin(self) -> Self {
                self.map_lanes(|e| e.asin())
            }

            #[inline(always)]
            fn simd_acos(self) -> Self {
                self.map_lanes(|e| e.acos())
            }

            #[inline(always)]
            fn simd_atan(self) -> Self {
                self.map_lanes(|e| e.atan())
            }

            #[inline(always)]
            fn simd_sin_cos(self) -> (Self, Self) {
                (self.simd_sin(), self.simd_cos())
            }

//            #[inline(always]
//            fn simd_exp_m1(self) -> Self {
//                $libm::exp_m1(self)
//            }
//
//            #[inline(always]
//            fn simd_ln_1p(self) -> Self {
//                $libm::ln_1p(self)
//            }
//
            #[inline(always)]
            fn simd_sinh(self) -> Self {
                self.map_lanes(|e| e.sinh())
            }

            #[inline(always)]
            fn simd_cosh(self) -> Self {
                self.map_lanes(|e| e.cosh())
            }

            #[inline(always)]
            fn simd_tanh(self) -> Self {
                self.map_lanes(|e| e.tanh())
            }

            #[inline(always)]
            fn simd_asinh(self) -> Self {
                self.map_lanes(|e| e.asinh())
            }

            #[inline(always)]
            fn simd_acosh(self) -> Self {
                self.map_lanes(|e| e.acosh())
            }

            #[inline(always)]
            fn simd_atanh(self) -> Self {
                self.map_lanes(|e| e.atanh())
            }
        }

        // NOTE: most of the impls in there are copy-paste from the implementation of
        // ComplexField for num_complex::Complex. Unfortunately, we can't reuse the implementations
        // so easily.
        impl SimdComplexField for num_complex::Complex<Simd<$t>> {
            type SimdRealField = Simd<$t>;

            #[inline(always)]
            fn simd_horizontal_sum(self) -> Self::Element {
                num_complex::Complex::new(self.re.simd_horizontal_sum(), self.im.simd_horizontal_sum())
            }

            #[inline(always)]
            fn simd_horizontal_product(self) -> Self::Element {
                let mut prod = self.extract(0);
                for ii in 1..Self::LANES {
                    prod *= self.extract(ii)
                }
                prod
            }

            #[inline]
            fn from_simd_real(re: Self::SimdRealField) -> Self {
                Self::new(re, Self::SimdRealField::zero())
            }

            #[inline]
            fn simd_real(self) -> Self::SimdRealField {
                self.re
            }

            #[inline]
            fn simd_imaginary(self) -> Self::SimdRealField {
                self.im
            }

            #[inline]
            fn simd_argument(self) -> Self::SimdRealField {
                self.im.simd_atan2(self.re)
            }

            #[inline]
            fn simd_modulus(self) -> Self::SimdRealField {
                self.re.simd_hypot(self.im)
            }

            #[inline]
            fn simd_modulus_squared(self) -> Self::SimdRealField {
                self.re * self.re + self.im * self.im
            }

            #[inline]
            fn simd_norm1(self) -> Self::SimdRealField {
                self.re.simd_abs() + self.im.simd_abs()
            }

            #[inline]
            fn simd_recip(self) -> Self {
                Self::one() / self
            }

            #[inline]
            fn simd_conjugate(self) -> Self {
                self.conj()
            }

            #[inline]
            fn simd_scale(self, factor: Self::SimdRealField) -> Self {
                self * factor
            }

            #[inline]
            fn simd_unscale(self, factor: Self::SimdRealField) -> Self {
                self / factor
            }

            #[inline]
            fn simd_floor(self) -> Self {
                Self::new(self.re.simd_floor(), self.im.simd_floor())
            }

            #[inline]
            fn simd_ceil(self) -> Self {
                Self::new(self.re.simd_ceil(), self.im.simd_ceil())
            }

            #[inline]
            fn simd_round(self) -> Self {
                Self::new(self.re.simd_round(), self.im.simd_round())
            }

            #[inline]
            fn simd_trunc(self) -> Self {
                Self::new(self.re.simd_trunc(), self.im.simd_trunc())
            }

            #[inline]
            fn simd_fract(self) -> Self {
                Self::new(self.re.simd_fract(), self.im.simd_fract())
            }

            #[inline]
            fn simd_mul_add(self, a: Self, b: Self) -> Self {
                self * a + b
            }

            #[inline]
            fn simd_abs(self) -> Self::SimdRealField {
                self.simd_modulus()
            }

            #[inline]
            fn simd_exp2(self) -> Self {
                let _2 = Simd::<$t>::one() + Simd::<$t>::one();
                num_complex::Complex::new(_2, Simd::<$t>::zero()).simd_powc(self)
            }

            #[inline]
            fn simd_exp_m1(self) -> Self {
                self.simd_exp() - Self::one()
            }

            #[inline]
            fn simd_ln_1p(self) -> Self {
                (Self::one() + self).simd_ln()
            }

            #[inline]
            fn simd_log2(self) -> Self {
                let _2 = Simd::<$t>::one() + Simd::<$t>::one();
                self.simd_log(_2)
            }

            #[inline]
            fn simd_log10(self) -> Self {
                let _10 = Simd::<$t>::from_subset(&10.0f64);
                self.simd_log(_10)
            }

            #[inline]
            fn simd_cbrt(self) -> Self {
                let one_third = Simd::<$t>::from_subset(&(1.0 / 3.0));
                self.simd_powf(one_third)
            }

            #[inline]
            fn simd_powi(self, n: i32) -> Self {
                // TODO: is there a more accurate solution?
                let n = Simd::<$t>::from_subset(&(n as f64));
                self.simd_powf(n)
            }

            /*
             *
             *
             * Unfortunately we are forced to copy-paste all
             * those impls from https://github.com/rust-num/num-complex/blob/master/src/lib.rs
             * to avoid requiring `std`.
             *
             *
             */
            /// Computes `e^(self)`, where `e` is the base of the natural logarithm.
            #[inline]
            fn simd_exp(self) -> Self {
                // formula: e^(a + bi) = e^a (cos(b) + i*sin(b))
                // = from_polar(e^a, b)
                simd_complex_from_polar(self.re.simd_exp(), self.im)
            }

            /// Computes the principal value of natural logarithm of `self`.
            ///
            /// This function has one branch cut:
            ///
            /// * `(-∞, 0]`, continuous from above.
            ///
            /// The branch satisfies `-π ≤ arg(ln(z)) ≤ π`.
            #[inline]
            fn simd_ln(self) -> Self {
                // formula: ln(z) = ln|z| + i*arg(z)
                let (r, theta) = self.simd_to_polar();
                Self::new(r.simd_ln(), theta)
            }

            /// Computes the principal value of the square root of `self`.
            ///
            /// This function has one branch cut:
            ///
            /// * `(-∞, 0)`, continuous from above.
            ///
            /// The branch satisfies `-π/2 ≤ arg(sqrt(z)) ≤ π/2`.
            #[inline]
            fn simd_sqrt(self) -> Self {
                // formula: sqrt(r e^(it)) = sqrt(r) e^(it/2)
                let two = Simd::<$t>::one() + Simd::<$t>::one();
                let (r, theta) = self.simd_to_polar();
                simd_complex_from_polar(r.simd_sqrt(), theta / two)
            }

            #[inline]
            fn simd_hypot(self, b: Self) -> Self::SimdRealField {
                (self.simd_modulus_squared() + b.simd_modulus_squared()).simd_sqrt()
            }

            /// Raises `self` to a floating point power.
            #[inline]
            fn simd_powf(self, exp: Self::SimdRealField) -> Self {
                // formula: x^y = (ρ e^(i θ))^y = ρ^y e^(i θ y)
                // = from_polar(ρ^y, θ y)
                let (r, theta) = self.simd_to_polar();
                simd_complex_from_polar(r.simd_powf(exp), theta * exp)
            }

            /// Returns the logarithm of `self` with respect to an arbitrary base.
            #[inline]
            fn simd_log(self, base: Simd<$t>) -> Self {
                // formula: log_y(x) = log_y(ρ e^(i θ))
                // = log_y(ρ) + log_y(e^(i θ)) = log_y(ρ) + ln(e^(i θ)) / ln(y)
                // = log_y(ρ) + i θ / ln(y)
                let (r, theta) = self.simd_to_polar();
                Self::new(r.simd_log(base), theta / base.simd_ln())
            }

            /// Raises `self` to a complex power.
            #[inline]
            fn simd_powc(self, exp: Self) -> Self {
                // formula: x^y = (a + i b)^(c + i d)
                // = (ρ e^(i θ))^c (ρ e^(i θ))^(i d)
                //    where ρ=|x| and θ=arg(x)
                // = ρ^c e^(−d θ) e^(i c θ) ρ^(i d)
                // = p^c e^(−d θ) (cos(c θ)
                //   + i sin(c θ)) (cos(d ln(ρ)) + i sin(d ln(ρ)))
                // = p^c e^(−d θ) (
                //   cos(c θ) cos(d ln(ρ)) − sin(c θ) sin(d ln(ρ))
                //   + i(cos(c θ) sin(d ln(ρ)) + sin(c θ) cos(d ln(ρ))))
                // = p^c e^(−d θ) (cos(c θ + d ln(ρ)) + i sin(c θ + d ln(ρ)))
                // = from_polar(p^c e^(−d θ), c θ + d ln(ρ))
                let (r, theta) = self.simd_to_polar();
                simd_complex_from_polar(
                    r.simd_powf(exp.re) * (-exp.im * theta).simd_exp(),
                    exp.re * theta + exp.im * r.simd_ln(),
                )
            }

            /*
            /// Raises a floating point number to the complex power `self`.
            #[inline]
            fn simd_expf(&self, base: T) -> Self {
                // formula: x^(a+bi) = x^a x^bi = x^a e^(b ln(x) i)
                // = from_polar(x^a, b ln(x))
                Self::from_polar(&base.powf(self.re), &(self.im * base.ln()))
            }
            */

            /// Computes the sine of `self`.
            #[inline]
            fn simd_sin(self) -> Self {
                // formula: sin(a + bi) = sin(a)cosh(b) + i*cos(a)sinh(b)
                Self::new(
                    self.re.simd_sin() * self.im.simd_cosh(),
                    self.re.simd_cos() * self.im.simd_sinh(),
                )
            }

            /// Computes the cosine of `self`.
            #[inline]
            fn simd_cos(self) -> Self {
                // formula: cos(a + bi) = cos(a)cosh(b) - i*sin(a)sinh(b)
                Self::new(
                    self.re.simd_cos() * self.im.simd_cosh(),
                    -self.re.simd_sin() * self.im.simd_sinh(),
                )
            }

            #[inline]
            fn simd_sin_cos(self) -> (Self, Self) {
                let (rsin, rcos) = self.re.simd_sin_cos();
                let (isinh, icosh) = self.im.simd_sinh_cosh();
                let sin = Self::new(rsin * icosh, rcos * isinh);
                let cos = Self::new(rcos * icosh, -rsin * isinh);

                (sin, cos)
            }

            /// Computes the tangent of `self`.
            #[inline]
            fn simd_tan(self) -> Self {
                // formula: tan(a + bi) = (sin(2a) + i*sinh(2b))/(cos(2a) + cosh(2b))
                let (two_re, two_im) = (self.re + self.re, self.im + self.im);
                Self::new(two_re.simd_sin(), two_im.simd_sinh()).unscale(two_re.simd_cos() + two_im.simd_cosh())
            }

            /// Computes the principal value of the inverse sine of `self`.
            ///
            /// This function has two branch cuts:
            ///
            /// * `(-∞, -1)`, continuous from above.
            /// * `(1, ∞)`, continuous from below.
            ///
            /// The branch satisfies `-π/2 ≤ Re(asin(z)) ≤ π/2`.
            #[inline]
            fn simd_asin(self) -> Self {
                // formula: arcsin(z) = -i ln(sqrt(1-z^2) + iz)
                let i = Self::i();
                -i * ((Self::one() - self * self).simd_sqrt() + i * self).simd_ln()
            }

            /// Computes the principal value of the inverse cosine of `self`.
            ///
            /// This function has two branch cuts:
            ///
            /// * `(-∞, -1)`, continuous from above.
            /// * `(1, ∞)`, continuous from below.
            ///
            /// The branch satisfies `0 ≤ Re(acos(z)) ≤ π`.
            #[inline]
            fn simd_acos(self) -> Self {
                // formula: arccos(z) = -i ln(i sqrt(1-z^2) + z)
                let i = Self::i();
                -i * (i * (Self::one() - self * self).simd_sqrt() + self).simd_ln()
            }

            /// Computes the principal value of the inverse tangent of `self`.
            ///
            /// This function has two branch cuts:
            ///
            /// * `(-∞i, -i]`, continuous from the left.
            /// * `[i, ∞i)`, continuous from the right.
            ///
            /// The branch satisfies `-π/2 ≤ Re(atan(z)) ≤ π/2`.
            #[inline]
            fn simd_atan(self) -> Self {
                // formula: arctan(z) = (ln(1+iz) - ln(1-iz))/(2i)
                let i = Self::i();
                let one = Self::one();
                let two = one + one;

                if self == i {
                    return Self::new(Simd::<$t>::zero(), Simd::<$t>::one() / Simd::<$t>::zero());
                } else if self == -i {
                    return Self::new(Simd::<$t>::zero(), -Simd::<$t>::one() / Simd::<$t>::zero());
                }

                ((one + i * self).simd_ln() - (one - i * self).simd_ln()) / (two * i)
            }

            /// Computes the hyperbolic sine of `self`.
            #[inline]
            fn simd_sinh(self) -> Self {
                // formula: sinh(a + bi) = sinh(a)cos(b) + i*cosh(a)sin(b)
                Self::new(
                    self.re.simd_sinh() * self.im.simd_cos(),
                    self.re.simd_cosh() * self.im.simd_sin(),
                )
            }

            /// Computes the hyperbolic cosine of `self`.
            #[inline]
            fn simd_cosh(self) -> Self {
                // formula: cosh(a + bi) = cosh(a)cos(b) + i*sinh(a)sin(b)
                Self::new(
                    self.re.simd_cosh() * self.im.simd_cos(),
                    self.re.simd_sinh() * self.im.simd_sin(),
                )
            }

            #[inline]
            fn simd_sinh_cosh(self) -> (Self, Self) {
                let (rsinh, rcosh) = self.re.simd_sinh_cosh();
                let (isin, icos) = self.im.simd_sin_cos();
                let sin = Self::new(rsinh * icos, rcosh * isin);
                let cos = Self::new(rcosh * icos, rsinh * isin);

                (sin, cos)
            }

            /// Computes the hyperbolic tangent of `self`.
            #[inline]
            fn simd_tanh(self) -> Self {
                // formula: tanh(a + bi) = (sinh(2a) + i*sin(2b))/(cosh(2a) + cos(2b))
                let (two_re, two_im) = (self.re + self.re, self.im + self.im);
                Self::new(two_re.simd_sinh(), two_im.simd_sin()).unscale(two_re.simd_cosh() + two_im.simd_cos())
            }

            /// Computes the principal value of inverse hyperbolic sine of `self`.
            ///
            /// This function has two branch cuts:
            ///
            /// * `(-∞i, -i)`, continuous from the left.
            /// * `(i, ∞i)`, continuous from the right.
            ///
            /// The branch satisfies `-π/2 ≤ Im(asinh(z)) ≤ π/2`.
            #[inline]
            fn simd_asinh(self) -> Self {
                // formula: arcsinh(z) = ln(z + sqrt(1+z^2))
                let one = Self::one();
                (self + (one + self * self).simd_sqrt()).simd_ln()
            }

            /// Computes the principal value of inverse hyperbolic cosine of `self`.
            ///
            /// This function has one branch cut:
            ///
            /// * `(-∞, 1)`, continuous from above.
            ///
            /// The branch satisfies `-π ≤ Im(acosh(z)) ≤ π` and `0 ≤ Re(acosh(z)) < ∞`.
            #[inline]
            fn simd_acosh(self) -> Self {
                // formula: arccosh(z) = 2 ln(sqrt((z+1)/2) + sqrt((z-1)/2))
                let one = Self::one();
                let two = one + one;
                two * (((self + one) / two).simd_sqrt() + ((self - one) / two).simd_sqrt()).simd_ln()
            }

            /// Computes the principal value of inverse hyperbolic tangent of `self`.
            ///
            /// This function has two branch cuts:
            ///
            /// * `(-∞, -1]`, continuous from above.
            /// * `[1, ∞)`, continuous from below.
            ///
            /// The branch satisfies `-π/2 ≤ Im(atanh(z)) ≤ π/2`.
            #[inline]
            fn simd_atanh(self) -> Self {
                // formula: arctanh(z) = (ln(1+z) - ln(1-z))/2
                let one = Self::one();
                let two = one + one;
                if self == one {
                    return Self::new(Simd::<$t>::one() / Simd::<$t>::zero(), Simd::<$t>::zero());
                } else if self == -one {
                    return Self::new(-Simd::<$t>::one() / Simd::<$t>::zero(), Simd::<$t>::zero());
                }
                ((one + self).simd_ln() - (one - self).simd_ln()) / two
            }
        }
    )*)
);

#[inline]
fn simd_complex_from_polar<N: SimdRealField>(r: N, theta: N) -> num_complex::Complex<N> {
    num_complex::Complex::new(r.clone() * theta.clone().simd_cos(), r * theta.simd_sin())
}

impl_float_simd!(
    portable_simd::f32x2, f32, portable_simd::i32x2, mask32x2, _0, _1;
    portable_simd::f32x4, f32, portable_simd::i32x4, mask32x4, _0, _1, _2, _3;
    portable_simd::f32x8, f32, portable_simd::i32x8, mask32x8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::f32x16, f32, portable_simd::i32x16, mask32x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::f64x2, f64, portable_simd::i64x2, mask64x2, _0, _1;
    portable_simd::f64x4, f64, portable_simd::i64x4, mask64x4, _0, _1, _2, _3;
    portable_simd::f64x8, f64, portable_simd::i64x8, mask64x8, _0, _1, _2, _3, _4, _5, _6, _7;
);

impl_int_simd!(
    portable_simd::i16x2, i16, mask16x2, _0, _1;
    portable_simd::i16x4, i16, mask16x4, _0, _1, _2, _3;
    portable_simd::i16x8, i16, mask16x8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::i16x16, i16, mask16x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::i16x32, i16, mask16x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    portable_simd::i32x2, i32, mask32x2, _0, _1;
    portable_simd::i32x4, i32, mask32x4, _0, _1, _2, _3;
    portable_simd::i32x8, i32, mask32x8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::i32x16, i32, mask32x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::i64x2, i64, mask64x2, _0, _1;
    portable_simd::i64x4, i64, mask64x4, _0, _1, _2, _3;
    portable_simd::i64x8, i64, mask64x8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::i8x2, i8, mask8x2, _0, _1;
    portable_simd::i8x4, i8, mask8x4, _0, _1, _2, _3;
    portable_simd::i8x8, i8, mask8x8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::i8x16, i8, mask8x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::i8x32, i8, mask8x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    portable_simd::i8x64, i8, mask8x64, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    portable_simd::isizex2, isize, masksizex2, _0, _1;
    portable_simd::isizex4, isize, masksizex4, _0, _1, _2, _3;
    portable_simd::isizex8, isize, masksizex8, _0, _1, _2, _3, _4, _5, _6, _7;
);

impl_uint_simd!(
    portable_simd::u16x2, u16, mask16x2, _0, _1;
    portable_simd::u16x4, u16, mask16x4, _0, _1, _2, _3;
    portable_simd::u16x8, u16, mask16x8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::u16x16, u16, mask16x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::u16x32, u16, mask16x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    portable_simd::u32x2, u32, mask32x2, _0, _1;
    portable_simd::u32x4, u32, mask32x4, _0, _1, _2, _3;
    portable_simd::u32x8, u32, mask32x8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::u32x16, u32, mask32x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::u64x2, u64, mask64x2, _0, _1;
    portable_simd::u64x4, u64, mask64x4, _0, _1, _2, _3;
    portable_simd::u64x8, u64, mask64x8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::u8x2, u8, mask8x2, _0, _1;
    portable_simd::u8x4, u8, mask8x4, _0, _1, _2, _3;
    portable_simd::u8x8, u8, mask8x8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::u8x16, u8, mask8x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::u8x32, u8, mask8x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    portable_simd::u8x64, u8, mask8x64, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    portable_simd::usizex2, usize, masksizex2, _0, _1;
    portable_simd::usizex4, usize, masksizex4, _0, _1, _2, _3;
    portable_simd::usizex8, usize, masksizex8, _0, _1, _2, _3, _4, _5, _6, _7;
);

impl_bool_simd!(
    portable_simd::mask16x2, 2, _0, _1;
    portable_simd::mask16x4, 4, _0, _1, _2, _3;
    portable_simd::mask16x8, 8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::mask16x16, 16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::mask16x32, 32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    portable_simd::mask32x2, 2, _0, _1;
    portable_simd::mask32x4, 4, _0, _1, _2, _3;
    portable_simd::mask32x8, 8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::mask32x16, 16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::mask64x2, 2, _0, _1;
    portable_simd::mask64x4, 4, _0, _1, _2, _3;
    portable_simd::mask64x8, 8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::mask8x2, 2, _0, _1;
    portable_simd::mask8x4, 4, _0, _1, _2, _3;
    portable_simd::mask8x8, 8, _0, _1, _2, _3, _4, _5, _6, _7;
    portable_simd::mask8x16, 16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    portable_simd::mask8x32, 32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    portable_simd::mask8x64, 64, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    portable_simd::masksizex2, 2, _0, _1;
    portable_simd::masksizex4, 4, _0, _1, _2, _3;
    portable_simd::masksizex8, 8, _0, _1, _2, _3, _4, _5, _6, _7;
);

//
// NOTE: the following does not work because of the orphan rules.
//
//macro_rules! impl_simd_complex_from(
//    ($($t: ty, $elt: ty $(, $i: expr)*;)*) => ($(
//        impl From<[num_complex::Complex<$elt>; <$t>::LANES]> for num_complex::Complex<Simd<$t>> {
//            #[inline(always)]
//            fn from(vals: [num_complex::Complex<$elt>; <$t>::LANES]) -> Self {
//                num_complex::Complex {
//                    re: <$t>::from([$(vals[$i].re),*]),
//                    im: <$t>::from([$(vals[$i].im),*]),
//                }
//            }
//        }
//    )*)
//);
//
//impl_simd_complex_from!(
//    portable_simd::f32x2, f32, 0, 1;
//    portable_simd::f32x4, f32, 0, 1, 2, 3;
//    portable_simd::f32x8, f32, 0, 1, 2, 3, 4, 5, 6, 7;
//    portable_simd::f32x16, f32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15;
//);

//////////////////////////////////////////
//               Aliases                //
//////////////////////////////////////////

pub type f32x2 = Simd<portable_simd::f32x2>;
pub type f32x4 = Simd<portable_simd::f32x4>;
pub type f32x8 = Simd<portable_simd::f32x8>;
pub type f32x16 = Simd<portable_simd::f32x16>;
pub type f64x2 = Simd<portable_simd::f64x2>;
pub type f64x4 = Simd<portable_simd::f64x4>;
pub type f64x8 = Simd<portable_simd::f64x8>;
pub type i16x2 = Simd<portable_simd::i16x2>;
pub type i16x4 = Simd<portable_simd::i16x4>;
pub type i16x8 = Simd<portable_simd::i16x8>;
pub type i16x16 = Simd<portable_simd::i16x16>;
pub type i16x32 = Simd<portable_simd::i16x32>;
pub type i32x2 = Simd<portable_simd::i32x2>;
pub type i32x4 = Simd<portable_simd::i32x4>;
pub type i32x8 = Simd<portable_simd::i32x8>;
pub type i32x16 = Simd<portable_simd::i32x16>;
pub type i64x2 = Simd<portable_simd::i64x2>;
pub type i64x4 = Simd<portable_simd::i64x4>;
pub type i64x8 = Simd<portable_simd::i64x8>;
pub type i8x2 = Simd<portable_simd::i8x2>;
pub type i8x4 = Simd<portable_simd::i8x4>;
pub type i8x8 = Simd<portable_simd::i8x8>;
pub type i8x16 = Simd<portable_simd::i8x16>;
pub type i8x32 = Simd<portable_simd::i8x32>;
pub type i8x64 = Simd<portable_simd::i8x64>;
pub type isizex2 = Simd<portable_simd::isizex2>;
pub type isizex4 = Simd<portable_simd::isizex4>;
pub type isizex8 = Simd<portable_simd::isizex8>;
pub type u16x2 = Simd<portable_simd::u16x2>;
pub type u16x4 = Simd<portable_simd::u16x4>;
pub type u16x8 = Simd<portable_simd::u16x8>;
pub type u16x16 = Simd<portable_simd::u16x16>;
pub type u16x32 = Simd<portable_simd::u16x32>;
pub type u32x2 = Simd<portable_simd::u32x2>;
pub type u32x4 = Simd<portable_simd::u32x4>;
pub type u32x8 = Simd<portable_simd::u32x8>;
pub type u32x16 = Simd<portable_simd::u32x16>;
pub type u64x2 = Simd<portable_simd::u64x2>;
pub type u64x4 = Simd<portable_simd::u64x4>;
pub type u64x8 = Simd<portable_simd::u64x8>;
pub type u8x2 = Simd<portable_simd::u8x2>;
pub type u8x4 = Simd<portable_simd::u8x4>;
pub type u8x8 = Simd<portable_simd::u8x8>;
pub type u8x16 = Simd<portable_simd::u8x16>;
pub type u8x32 = Simd<portable_simd::u8x32>;
pub type u8x64 = Simd<portable_simd::u8x64>;
pub type usizex2 = Simd<portable_simd::usizex2>;
pub type usizex4 = Simd<portable_simd::usizex4>;
pub type usizex8 = Simd<portable_simd::usizex8>;

pub type mask8x1 = Simd<portable_simd::mask8x1>;
pub type mask8x2 = Simd<portable_simd::mask8x2>;
pub type mask8x4 = Simd<portable_simd::mask8x4>;
pub type mask8x8 = Simd<portable_simd::mask8x8>;
pub type mask8x16 = Simd<portable_simd::mask8x16>;
pub type mask8x32 = Simd<portable_simd::mask8x32>;
pub type mask8x64 = Simd<portable_simd::mask8x64>;
pub type mask16x1 = Simd<portable_simd::mask16x1>;
pub type mask16x2 = Simd<portable_simd::mask16x2>;
pub type mask16x4 = Simd<portable_simd::mask16x4>;
pub type mask16x8 = Simd<portable_simd::mask16x8>;
pub type mask16x16 = Simd<portable_simd::mask16x16>;
pub type mask16x32 = Simd<portable_simd::mask16x32>;
pub type mask32x1 = Simd<portable_simd::mask32x1>;
pub type mask32x2 = Simd<portable_simd::mask32x2>;
pub type mask32x4 = Simd<portable_simd::mask32x4>;
pub type mask32x8 = Simd<portable_simd::mask32x8>;
pub type mask32x16 = Simd<portable_simd::mask32x16>;
pub type mask64x1 = Simd<portable_simd::mask64x1>;
pub type mask64x2 = Simd<portable_simd::mask64x2>;
pub type mask64x4 = Simd<portable_simd::mask64x4>;
pub type mask64x8 = Simd<portable_simd::mask64x8>;
pub type masksizex1 = Simd<portable_simd::masksizex1>;
pub type masksizex2 = Simd<portable_simd::masksizex2>;
pub type masksizex4 = Simd<portable_simd::masksizex4>;
pub type masksizex8 = Simd<portable_simd::masksizex8>;
