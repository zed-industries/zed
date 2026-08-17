#![allow(missing_docs)]
#![allow(non_camel_case_types)] // For the simd type aliases.

//! SIMD values based on auto-vectorization.

use crate::scalar::{Field, SubsetOf, SupersetOf};
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

/// A SIMD structure that implements all the relevant traits from `num` an `simba`.
///
/// This is needed to overcome the orphan rules.
#[repr(align(16))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self", bound(archive = "N: rkyv::Archive<Archived = N>"))
)]
pub struct AutoSimd<N>(pub N);

/// A SIMD boolean structure that implements all the relevant traits from `num` an `simba`.
///
/// This is needed to overcome the orphan rules.
#[repr(align(16))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self", bound(archive = "N: rkyv::Archive<Archived = N>"))
)]
pub struct AutoBoolSimd<N>(pub N);

macro_rules! impl_bool_simd (
    ($($t: ty, $lanes: expr, $($i: ident),*;)*) => {$(
        impl_simd_value!($t, bool, $lanes, AutoSimd<$t> $(, $i)*;);

        impl AutoSimd<$t> {
            pub const ZERO: Self = AutoSimd([false; $lanes]);
            pub const ONE: Self = AutoSimd([true; $lanes]);

            #[inline(always)]
            pub fn new($($i: bool),*) -> Self {
                AutoSimd([$($i),*])
            }
        }

        impl From<[bool; $lanes]> for AutoSimd<$t> {
            #[inline(always)]
            fn from(vals: [bool; $lanes]) -> Self {
                Self(vals)
            }
        }

        impl Not for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
                self.map(|x| !x)
            }
        }

        impl BitAnd<AutoSimd<$t>> for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                self.zip_map(rhs, |x, y| x & y)
            }
        }

        impl BitOr<AutoSimd<$t>> for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                self.zip_map(rhs, |x, y| x | y)
            }
        }

        impl BitXor<AutoSimd<$t>> for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self {
                self.zip_map(rhs, |x, y| x ^ y)
            }
        }

        impl SimdBool for AutoSimd<$t> {
            #[inline(always)]
            fn bitmask(self) -> u64 {
                ident_to_value!();
                0u64 $(
                    | ((self.0[$i] as u64) << $i)
                 )*
            }

            #[inline(always)]
            fn and(self) -> bool {
                ident_to_value!();
                true $(
                    && self.0[$i]
                 )*
            }

            #[inline(always)]
            fn or(self) -> bool {
                ident_to_value!();
                false $(
                    || self.0[$i]
                 )*
            }

            #[inline(always)]
            fn xor(self) -> bool {
                ident_to_value!();
                false $(
                    ^ self.0[$i]
                 )*
            }

            #[inline(always)]
            fn all(self) -> bool {
                self.and()
            }

            #[inline(always)]
            fn any(self) -> bool {
                self.or()
            }

            #[inline(always)]
            fn none(self) -> bool {
                !self.any()
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
        impl<N2> SubsetOf<AutoSimd<N2>> for $t
            where AutoSimd<N2>: SimdValue + Copy,
                  <AutoSimd<N2> as SimdValue>::Element: SupersetOf<$t> + PartialEq, {
            #[inline(always)]
            fn to_superset(&self) -> AutoSimd<N2> {
                AutoSimd::<N2>::splat(<AutoSimd<N2> as SimdValue>::Element::from_subset(self))
            }

            #[inline(always)]
            fn from_superset_unchecked(element: &AutoSimd<N2>) -> $t {
                element.extract(0).to_subset_unchecked()
            }

            #[inline(always)]
            fn is_in_subset(c: &AutoSimd<N2>) -> bool {
                let elt0 = c.extract(0);
                elt0.is_in_subset() &&
                (1..AutoSimd::<N2>::LANES).all(|i| c.extract(i) == elt0)
            }
        }
    )*}
);

impl_scalar_subset_of_simd!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
#[cfg(feature = "decimal")]
impl_scalar_subset_of_simd!(d128);

macro_rules! impl_simd_value (
    ($($t: ty, $elt: ty, $lanes: expr, $bool: ty, $($i: ident),*;)*) => ($(
        impl ArrTransform for AutoSimd<$t> {
            #[inline(always)]
            fn map(self, f: impl Fn(Self::Element) -> Self::Element) -> Self {
                ident_to_value!();
                Self([$(f(self.0[$i])),*])
            }

            #[inline(always)]
            fn zip_map(self, other: Self, f: impl Fn(Self::Element, Self::Element) -> Self::Element) -> Self {
                ident_to_value!();
                Self([$(f(self.0[$i], other.0[$i])),*])
            }

            #[inline(always)]
            fn zip_zip_map(self, b: Self, c: Self, f: impl Fn(Self::Element, Self::Element, Self::Element) -> Self::Element) -> Self {
                ident_to_value!();
                Self([$(f(self.0[$i], b.0[$i], c.0[$i])),*])
            }

            #[inline(always)]
            fn map_bool(self, f: impl Fn(Self::Element) -> bool) -> Self::SimdBool {
                ident_to_value!();
                AutoSimd([$(f(self.0[$i])),*])
            }

            #[inline(always)]
            fn zip_map_bool(self, other: Self, f: impl Fn(Self::Element, Self::Element) -> bool) -> Self::SimdBool {
                ident_to_value!();
                AutoSimd([$(f(self.0[$i], other.0[$i])),*])
            }
        }

        impl fmt::Display for AutoSimd<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if Self::LANES == 1 {
                    return self.extract(0).fmt(f);
                }

                write!(f, "({}", self.extract(0))?;

                #[allow(clippy::reversed_empty_ranges)] // Needed for LANES == 1 that’s in a different code path.
                for i in 1..Self::LANES {
                    write!(f, ", {}", self.extract(i))?;
                }

                write!(f, ")")
            }
        }

        impl PrimitiveSimdValue for AutoSimd<$t> {}

        impl SimdValue for AutoSimd<$t> {
            const LANES: usize = $lanes;
            type Element = $elt;
            type SimdBool = $bool;


            #[inline(always)]
            fn splat(val: Self::Element) -> Self {
                AutoSimd([val; $lanes])
            }

            #[inline(always)]
            fn extract(&self, i: usize) -> Self::Element {
                self.0[i]
            }

            #[inline(always)]
            unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
                *self.0.get_unchecked(i)
            }

            #[inline(always)]
            fn replace(&mut self, i: usize, val: Self::Element) {
                self.0[i] = val
            }

            #[inline(always)]
            unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
                *self.0.get_unchecked_mut(i) = val
            }

            #[inline(always)]
            fn select(self, cond: Self::SimdBool, other: Self) -> Self {
                ident_to_value!();
                Self([
                    $(if cond.0[$i] { self.0[$i] } else { other.0[$i] }),*
                ])
            }
        }
    )*)
);

macro_rules! impl_uint_simd (
    ($($t: ty, $elt: ty, $lanes: expr, $bool: ty, $($i: ident),*;)*) => ($(
        impl_simd_value!($t, $elt, $lanes, $bool $(, $i)*;);

        impl AutoSimd<$t> {
            pub const ZERO: Self = AutoSimd([0 as $elt; $lanes]);
            pub const ONE: Self = AutoSimd([1 as $elt; $lanes]);

            pub fn new($($i: $elt),*) -> Self {
                AutoSimd([$($i),*])
            }
        }

        impl From<[$elt; $lanes]> for AutoSimd<$t> {
            #[inline(always)]
            fn from(vals: [$elt; $lanes]) -> Self {
                AutoSimd(vals)
            }
        }

        impl From<AutoSimd<$t>> for [$elt; $lanes] {
            #[inline(always)]
            fn from(val: AutoSimd<$t>) -> [$elt; $lanes] {
                val.0
            }
        }

        impl SubsetOf<AutoSimd<$t>> for AutoSimd<$t> {
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

        impl Num for AutoSimd<$t> {
            type FromStrRadixErr = <$elt as Num>::FromStrRadixErr;

            #[inline(always)]
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                <$elt>::from_str_radix(str, radix).map(Self::splat)
            }
        }

        impl FromPrimitive for AutoSimd<$t> {
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


        impl Zero for AutoSimd<$t> {
            #[inline(always)]
            fn zero() -> Self {
                AutoSimd([<$elt>::zero(); $lanes])
            }

            #[inline(always)]
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }

        impl One for AutoSimd<$t> {
            #[inline(always)]
            fn one() -> Self {
                AutoSimd([<$elt>::one(); $lanes])
            }
        }

        impl Add<AutoSimd<$t>> for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                self.zip_map(rhs, |x, y| x + y)
            }
        }

        impl Sub<AutoSimd<$t>> for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                self.zip_map(rhs, |x, y| x - y)
            }
        }

        impl Mul<AutoSimd<$t>> for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                self.zip_map(rhs, |x, y| x * y)
            }
        }

        impl Div<AutoSimd<$t>> for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                self.zip_map(rhs, |x, y| x / y)
            }
        }

        impl Rem<AutoSimd<$t>> for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn rem(self, rhs: Self) -> Self {
                self.zip_map(rhs, |x, y| x % y)
            }
        }

        impl AddAssign<AutoSimd<$t>> for AutoSimd<$t> {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl SubAssign<AutoSimd<$t>> for AutoSimd<$t> {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl DivAssign<AutoSimd<$t>> for AutoSimd<$t> {
            #[inline(always)]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }

        impl MulAssign<AutoSimd<$t>> for AutoSimd<$t> {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl RemAssign<AutoSimd<$t>> for AutoSimd<$t> {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs;
            }
        }

        impl SimdPartialOrd for AutoSimd<$t> {
            #[inline(always)]
            fn simd_gt(self, other: Self) -> Self::SimdBool {
                self.zip_map_bool(other, |x, y| x.simd_gt(y))
            }

            #[inline(always)]
            fn simd_lt(self, other: Self) -> Self::SimdBool {
                self.zip_map_bool(other, |x, y| x.simd_lt(y))
            }

            #[inline(always)]
            fn simd_ge(self, other: Self) -> Self::SimdBool {
                self.zip_map_bool(other, |x, y| x.simd_ge(y))
            }

            #[inline(always)]
            fn simd_le(self, other: Self) -> Self::SimdBool {
                self.zip_map_bool(other, |x, y| x.simd_le(y))
            }

            #[inline(always)]
            fn simd_eq(self, other: Self) -> Self::SimdBool {
                self.zip_map_bool(other, |x, y| x.simd_eq(y))
            }

            #[inline(always)]
            fn simd_ne(self, other: Self) -> Self::SimdBool {
                self.zip_map_bool(other, |x, y| x.simd_ne(y))
            }

            #[inline(always)]
            fn simd_max(self, other: Self) -> Self {
                self.zip_map(other, |x, y| x.simd_max(y))
            }
            #[inline(always)]
            fn simd_min(self, other: Self) -> Self {
                self.zip_map(other, |x, y| x.simd_min(y))
            }

            #[inline(always)]
            fn simd_clamp(self, min: Self, max: Self) -> Self {
                self.simd_max(min).simd_min(max)
            }

            #[inline(always)]
            fn simd_horizontal_min(self) -> Self::Element {
                ident_to_value!();
                self.0[0] $(.simd_min(self.0[$i]))*
            }

            #[inline(always)]
            fn simd_horizontal_max(self) -> Self::Element {
                ident_to_value!();
                self.0[0] $(.simd_max(self.0[$i]))*
            }
        }

//        impl MeetSemilattice for AutoSimd<$t> {
//            #[inline(always)]
//            fn meet(&self, other: &Self) -> Self {
//                AutoSimd(self.0.min(other.0))
//            }
//        }
//
//        impl JoinSemilattice for AutoSimd<$t> {
//            #[inline(always)]
//            fn join(&self, other: &Self) -> Self {
//                AutoSimd(self.0.max(other.0))
//            }
//        }
    )*)
);

macro_rules! impl_int_simd (
    ($($t: ty, $elt: ty, $lanes: expr, $bool: ty, $($i: ident),*;)*) => ($(
        impl_uint_simd!($t, $elt, $lanes, $bool $(, $i)*;);

        impl Neg for AutoSimd<$t> {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                self.map(|x| -x)
            }
        }
    )*)
);

macro_rules! impl_float_simd (
    ($($t: ty, $elt: ty, $lanes: expr, $int: ty, $bool: ty, $($i: ident),*;)*) => ($(
        impl_int_simd!($t, $elt, $lanes, $bool $(, $i)*;);

        // TODO: this should be part of impl_int_simd
        impl SimdSigned for AutoSimd<$t> {
            #[inline(always)]
            fn simd_abs(&self) -> Self {
                self.map(|x| x.simd_abs())
            }

            #[inline(always)]
            fn simd_abs_sub(&self, other: &Self) -> Self {
                self.zip_map(*other, |x, y| x.simd_abs_sub(&y))
            }

            #[inline(always)]
            fn simd_signum(&self) -> Self {
                self.map(|x| x.simd_signum())
            }

            #[inline(always)]
            fn is_simd_positive(&self) -> Self::SimdBool {
                self.map_bool(|x| x.is_simd_positive())
            }

            #[inline(always)]
            fn is_simd_negative(&self) -> Self::SimdBool {
                self.map_bool(|x| x.is_simd_negative())
            }
        }

        impl Field for AutoSimd<$t> {}

        #[cfg(any(feature = "std", feature = "libm", feature = "libm_force"))]
        impl SimdRealField for AutoSimd<$t> {
            #[inline(always)]
            fn simd_atan2(self, other: Self) -> Self {
                self.zip_map(other, |x, y| x.simd_atan2(y))
            }

            #[inline(always)]
            fn simd_copysign(self, sign: Self) -> Self {
                self.zip_map(sign, |me, sgn| me.simd_copysign(sgn))
            }

            #[inline(always)]
            fn simd_default_epsilon() -> Self {
                Self::splat(<$elt>::default_epsilon())
            }

            #[inline(always)]
            fn simd_pi() -> Self {
                Self::splat(<$elt>::simd_pi())
            }

            #[inline(always)]
            fn simd_two_pi() -> Self {
                Self::splat(<$elt>::simd_two_pi())
            }

            #[inline(always)]
            fn simd_frac_pi_2() -> Self {
                Self::splat(<$elt>::simd_frac_pi_2())
            }

            #[inline(always)]
            fn simd_frac_pi_3() -> Self {
                Self::splat(<$elt>::simd_frac_pi_3())
            }

            #[inline(always)]
            fn simd_frac_pi_4() -> Self {
                Self::splat(<$elt>::simd_frac_pi_4())
            }

            #[inline(always)]
            fn simd_frac_pi_6() -> Self {
                Self::splat(<$elt>::simd_frac_pi_6())
            }

            #[inline(always)]
            fn simd_frac_pi_8() -> Self {
                Self::splat(<$elt>::simd_frac_pi_8())
            }

            #[inline(always)]
            fn simd_frac_1_pi() -> Self {
                Self::splat(<$elt>::simd_frac_1_pi())
            }

            #[inline(always)]
            fn simd_frac_2_pi() -> Self {
                Self::splat(<$elt>::simd_frac_2_pi())
            }

            #[inline(always)]
            fn simd_frac_2_sqrt_pi() -> Self {
                Self::splat(<$elt>::simd_frac_2_sqrt_pi())
            }


            #[inline(always)]
            fn simd_e() -> Self {
                Self::splat(<$elt>::simd_e())
            }

            #[inline(always)]
            fn simd_log2_e() -> Self {
                Self::splat(<$elt>::simd_log2_e())
            }

            #[inline(always)]
            fn simd_log10_e() -> Self {
                Self::splat(<$elt>::simd_log10_e() )
            }

            #[inline(always)]
            fn simd_ln_2() -> Self {
                Self::splat(<$elt>::simd_ln_2())
            }

            #[inline(always)]
            fn simd_ln_10() -> Self {
                Self::splat(<$elt>::simd_ln_10())
            }
        }

        #[cfg(any(feature = "std", feature = "libm", feature = "libm_force"))]
        impl SimdComplexField for AutoSimd<$t> {
            type SimdRealField = Self;

            #[inline(always)]
            fn simd_horizontal_sum(self) -> Self::Element {
                self.0.iter().sum()
            }

            #[inline(always)]
            fn simd_horizontal_product(self) -> Self::Element {
                self.0.iter().product()
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
                self.map(|x| x.simd_norm1())
            }

            #[inline(always)]
            fn simd_modulus(self) -> Self::SimdRealField {
                self.map(|x| x.simd_modulus())
            }

            #[inline(always)]
            fn simd_modulus_squared(self) -> Self::SimdRealField {
                self.map(|x| x.simd_modulus_squared())
            }

            #[inline(always)]
            fn simd_argument(self) -> Self::SimdRealField {
                self.map(|x| x.simd_argument())
            }

            #[inline(always)]
            fn simd_to_exp(self) -> (Self::SimdRealField, Self) {
                let ge = self.simd_ge(Self::one());
                let exp = Self::one().select(ge, -Self::one());
                (self * exp, exp)
            }

            #[inline(always)]
            fn simd_recip(self) -> Self {
                self.map(|x| x.simd_recip())
            }

            #[inline(always)]
            fn simd_conjugate(self) -> Self {
                self.map(|x| x.simd_conjugate())
            }

            #[inline(always)]
            fn simd_scale(self, factor: Self::SimdRealField) -> Self {
                self.zip_map(factor, |x, y| x.simd_scale(y))
            }

            #[inline(always)]
            fn simd_unscale(self, factor: Self::SimdRealField) -> Self {
                self.zip_map(factor, |x, y| x.simd_unscale(y))
            }

            #[inline(always)]
            fn simd_floor(self) -> Self {
                self.map(|e| e.simd_floor())
            }

            #[inline(always)]
            fn simd_ceil(self) -> Self {
                self.map(|e| e.simd_ceil())
            }

            #[inline(always)]
            fn simd_round(self) -> Self {
                self.map(|e| e.simd_round())
            }

            #[inline(always)]
            fn simd_trunc(self) -> Self {
                self.map(|e| e.simd_trunc())
            }

            #[inline(always)]
            fn simd_fract(self) -> Self {
                self.map(|e| e.simd_fract())
            }

            #[inline(always)]
            fn simd_abs(self) -> Self {
                self.map(|e| e.simd_abs())
            }

            #[inline(always)]
            fn simd_signum(self) -> Self {
                self.map(|e| e.simd_signum())
            }

            #[inline(always)]
            fn simd_mul_add(self, a: Self, b: Self) -> Self {
                self.zip_zip_map(a, b, |x, y, z| x.simd_mul_add(y, z))
            }

            #[inline(always)]
            fn simd_powi(self, n: i32) -> Self {
                self.map(|e| e.simd_powi(n))
            }

            #[inline(always)]
            fn simd_powf(self, n: Self) -> Self {
                self.zip_map(n, |x, y| x.simd_powf(y))
            }

            #[inline(always)]
            fn simd_powc(self, n: Self) -> Self {
                self.zip_map(n, |x, y| x.simd_powc(y))
            }

            #[inline(always)]
            fn simd_sqrt(self) -> Self {
                self.map(|x| x.simd_sqrt())
            }

            #[inline(always)]
            fn simd_exp(self) -> Self {
                self.map(|x| x.simd_exp())
            }

            #[inline(always)]
            fn simd_exp2(self) -> Self {
                self.map(|x| x.simd_exp2())
            }


            #[inline(always)]
            fn simd_exp_m1(self) -> Self {
                self.map(|x| x.simd_exp_m1())
            }

            #[inline(always)]
            fn simd_ln_1p(self) -> Self {
                self.map(|x| x.simd_ln_1p())
            }

            #[inline(always)]
            fn simd_ln(self) -> Self {
                self.map(|x| x.simd_ln())
            }

            #[inline(always)]
            fn simd_log(self, base: Self) -> Self {
                self.zip_map(base, |x, y| x.simd_log(y))
            }

            #[inline(always)]
            fn simd_log2(self) -> Self {
                self.map(|x| x.simd_log2())
            }

            #[inline(always)]
            fn simd_log10(self) -> Self {
                self.map(|x| x.simd_log10())
            }

            #[inline(always)]
            fn simd_cbrt(self) -> Self {
                self.map(|x| x.simd_cbrt())
            }

            #[inline(always)]
            fn simd_hypot(self, other: Self) -> Self::SimdRealField {
                self.zip_map(other, |x, y| x.simd_hypot(y))
            }

            #[inline(always)]
            fn simd_sin(self) -> Self {
                self.map(|x| x.simd_sin())
            }

            #[inline(always)]
            fn simd_cos(self) -> Self {
                self.map(|x| x.simd_cos())
            }

            #[inline(always)]
            fn simd_tan(self) -> Self {
                self.map(|x| x.simd_tan())
            }

            #[inline(always)]
            fn simd_asin(self) -> Self {
                self.map(|x| x.simd_asin())
            }

            #[inline(always)]
            fn simd_acos(self) -> Self {
                self.map(|x| x.simd_acos())
            }

            #[inline(always)]
            fn simd_atan(self) -> Self {
                self.map(|x| x.simd_atan())
            }

            #[inline(always)]
            fn simd_sin_cos(self) -> (Self, Self) {
                (self.simd_sin(), self.simd_cos())
            }

//            #[inline(always)]
//            fn simd_exp_m1(self) -> Self {
//                $libm::exp_m1(self)
//            }
//
//            #[inline(always)]
//            fn simd_ln_1p(self) -> Self {
//                $libm::ln_1p(self)
//            }
//
            #[inline(always)]
            fn simd_sinh(self) -> Self {
                self.map(|x| x.simd_sinh())
            }

            #[inline(always)]
            fn simd_cosh(self) -> Self {
                self.map(|x| x.simd_cosh())
            }

            #[inline(always)]
            fn simd_tanh(self) -> Self {
                self.map(|x| x.simd_tanh())
            }

            #[inline(always)]
            fn simd_asinh(self) -> Self {
                self.map(|x| x.simd_asinh())
            }

            #[inline(always)]
            fn simd_acosh(self) -> Self {
                self.map(|x| x.simd_acosh())
            }

            #[inline(always)]
            fn simd_atanh(self) -> Self {
                self.map(|x| x.simd_atanh())
            }
        }

        // NOTE: most of the impls in there are copy-paste from the implementation of
        // ComplexField for num_complex::Complex. Unfortunately, we can't reuse the implementations
        // so easily.
        #[cfg(any(feature = "std", feature = "libm", feature = "libm_force"))]
        impl SimdComplexField for num_complex::Complex<AutoSimd<$t>> {
            type SimdRealField = AutoSimd<$t>;

            #[inline(always)]
            fn simd_horizontal_sum(self) -> Self::Element {
                num_complex::Complex::new(self.re.simd_horizontal_sum(), self.im.simd_horizontal_sum())
            }

            #[inline(always)]
            fn simd_horizontal_product(self) -> Self::Element {
                let mut prod = self.extract(0);
                for ii in 1..$lanes {
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
                let _2 = AutoSimd::<$t>::one() + AutoSimd::<$t>::one();
                num_complex::Complex::new(_2, AutoSimd::<$t>::zero()).simd_powc(self)
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
                let _2 = AutoSimd::<$t>::one() + AutoSimd::<$t>::one();
                self.simd_log(_2)
            }

            #[inline]
            fn simd_log10(self) -> Self {
                let _10 = AutoSimd::<$t>::from_subset(&10.0f64);
                self.simd_log(_10)
            }

            #[inline]
            fn simd_cbrt(self) -> Self {
                let one_third = AutoSimd::<$t>::from_subset(&(1.0 / 3.0));
                self.simd_powf(one_third)
            }

            #[inline]
            fn simd_powi(self, n: i32) -> Self {
                // TODO: is there a more accurate solution?
                let n = AutoSimd::<$t>::from_subset(&(n as f64));
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
                let two = AutoSimd::<$t>::one() + AutoSimd::<$t>::one();
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
            fn simd_log(self, base: AutoSimd<$t>) -> Self {
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
                    return Self::new(AutoSimd::<$t>::zero(), AutoSimd::<$t>::one() / AutoSimd::<$t>::zero());
                } else if self == -i {
                    return Self::new(AutoSimd::<$t>::zero(), -AutoSimd::<$t>::one() / AutoSimd::<$t>::zero());
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
                    return Self::new(AutoSimd::<$t>::one() / AutoSimd::<$t>::zero(), AutoSimd::<$t>::zero());
                } else if self == -one {
                    return Self::new(-AutoSimd::<$t>::one() / AutoSimd::<$t>::zero(), AutoSimd::<$t>::zero());
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
    [f32; 2], f32, 2, [i32; 2], AutoBoolx2, _0, _1;
    [f32; 4], f32, 4, [i32; 4], AutoBoolx4, _0, _1, _2, _3;
    [f32; 8], f32, 8, [i32; 8], AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
    [f32; 16], f32, 16, [i32; 16], AutoBoolx16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [f64; 2], f64, 2, [i64; 2], AutoBoolx2, _0, _1;
    [f64; 4], f64, 4, [i64; 4], AutoBoolx4, _0, _1, _2, _3;
    [f64; 8], f64, 8, [i64; 8], AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
);

impl_int_simd!(
    [i128; 1], i128, 1, AutoBoolx1, _0;
    [i128; 2], i128, 2, AutoBoolx2, _0, _1;
    [i128; 4], i128, 4, AutoBoolx4, _0, _1, _2, _3;
    [i16; 2], i16, 2, AutoBoolx2, _0, _1;
    [i16; 4], i16, 4, AutoBoolx4, _0, _1, _2, _3;
    [i16; 8], i16, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
    [i16; 16], i16, 16, AutoBoolx16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [i16; 32], i16, 32, AutoBoolx32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    [i32; 2], i32, 2, AutoBoolx2, _0, _1;
    [i32; 4], i32, 4, AutoBoolx4, _0, _1, _2, _3;
    [i32; 8], i32, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
    [i32; 16], i32, 16, AutoBoolx16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [i64; 2], i64, 2, AutoBoolx2, _0, _1;
    [i64; 4], i64, 4, AutoBoolx4, _0, _1, _2, _3;
    [i64; 8], i64, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
    [i8; 2], i8, 2, AutoBoolx2, _0, _1;
    [i8; 4], i8, 4, AutoBoolx4, _0, _1, _2, _3;
    [i8; 8], i8, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
    [i8; 16], i8, 16, AutoBoolx16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [i8; 32], i8, 32, AutoBoolx32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    // [i8; 64], i8, 64, AutoBoolx64, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    [isize; 2], isize, 2, AutoBoolx2, _0, _1;
    [isize; 4], isize, 4, AutoBoolx4, _0, _1, _2, _3;
    [isize; 8], isize, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
);

impl_uint_simd!(
    [u128; 1], u128, 1, AutoBoolx1, _0;
    [u128; 2], u128, 2, AutoBoolx2, _0, _1;
    [u128; 4], u128, 4, AutoBoolx4, _0, _1, _2, _3;
    [u16; 2], u16, 2, AutoBoolx2, _0, _1;
    [u16; 4], u16, 4, AutoBoolx4, _0, _1, _2, _3;
    [u16; 8], u16, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
    [u16; 16], u16, 16, AutoBoolx16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [u16; 32], u16, 32, AutoBoolx32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    [u32; 2], u32, 2, AutoBoolx2, _0, _1;
    [u32; 4], u32, 4, AutoBoolx4, _0, _1, _2, _3;
    [u32; 8], u32, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
    [u32; 16], u32, 16, AutoBoolx16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [u64; 2], u64, 2, AutoBoolx2, _0, _1;
    [u64; 4], u64, 4, AutoBoolx4, _0, _1, _2, _3;
    [u64; 8], u64, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
    [u8; 2], u8, 2, AutoBoolx2, _0, _1;
    [u8; 4], u8, 4, AutoBoolx4, _0, _1, _2, _3;
    [u8; 8], u8, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
    [u8; 16], u8, 16, AutoBoolx16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [u8; 32], u8, 32, AutoBoolx32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    // [u8; 64], u8, 64, AutoBoolx64, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    [usize; 2], usize, 2, AutoBoolx2, _0, _1;
    [usize; 4], usize, 4, AutoBoolx4, _0, _1, _2, _3;
    [usize; 8], usize, 8, AutoBoolx8, _0, _1, _2, _3, _4, _5, _6, _7;
);

impl_bool_simd!(
    [bool; 1], 1, _0;
    [bool; 2], 2, _0, _1;
    [bool; 4], 4, _0, _1, _2, _3;
    [bool; 8], 8, _0, _1, _2, _3, _4, _5, _6, _7;
    [bool; 16], 16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [bool; 32], 32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    // [bool; 64], 64, 0, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
);

//
// NOTE: the following does not work because of the orphan rules.
//
//macro_rules! impl_simd_complex_from(
//    ($($t: ty, $elt: ty $(, $i: expr)*;)*) => ($(
//        impl From<[num_complex::Complex<$elt>; $lanes]> for num_complex::Complex<AutoSimd<$t>> {
//            #[inline(always)]
//            fn from(vals: [num_complex::Complex<$elt>; $lanes]) -> Self {
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
//    [f32; 2], f32, 0, 1;
//    [f32; 4], f32, 0, 1, 2, 3;
//    [f32; 8], f32, 0, 1, 2, 3, 4, 5, 6, 7;
//    [f32; 16], f32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15;
//);

//////////////////////////////////////////
//               Aliases                //
//////////////////////////////////////////

pub type AutoF32x2 = AutoSimd<[f32; 2]>;
pub type AutoF32x4 = AutoSimd<[f32; 4]>;
pub type AutoF32x8 = AutoSimd<[f32; 8]>;
pub type AutoF32x16 = AutoSimd<[f32; 16]>;
pub type AutoF64x2 = AutoSimd<[f64; 2]>;
pub type AutoF64x4 = AutoSimd<[f64; 4]>;
pub type AutoF64x8 = AutoSimd<[f64; 8]>;
pub type AutoI128x1 = AutoSimd<[i128; 1]>;
pub type AutoI128x2 = AutoSimd<[i128; 2]>;
pub type AutoI128x4 = AutoSimd<[i128; 4]>;
pub type AutoI16x2 = AutoSimd<[i16; 2]>;
pub type AutoI16x4 = AutoSimd<[i16; 4]>;
pub type AutoI16x8 = AutoSimd<[i16; 8]>;
pub type AutoI16x16 = AutoSimd<[i16; 16]>;
pub type AutoI16x32 = AutoSimd<[i16; 32]>;
pub type AutoI32x2 = AutoSimd<[i32; 2]>;
pub type AutoI32x4 = AutoSimd<[i32; 4]>;
pub type AutoI32x8 = AutoSimd<[i32; 8]>;
pub type AutoI32x16 = AutoSimd<[i32; 16]>;
pub type AutoI64x2 = AutoSimd<[i64; 2]>;
pub type AutoI64x4 = AutoSimd<[i64; 4]>;
pub type AutoI64x8 = AutoSimd<[i64; 8]>;
pub type AutoI8x2 = AutoSimd<[i8; 2]>;
pub type AutoI8x4 = AutoSimd<[i8; 4]>;
pub type AutoI8x8 = AutoSimd<[i8; 8]>;
pub type AutoI8x16 = AutoSimd<[i8; 16]>;
pub type AutoI8x32 = AutoSimd<[i8; 32]>;
// pub type AutoI8x64 = AutoSimd<[i8; 64]>;
pub type AutoIsizex2 = AutoSimd<[isize; 2]>;
pub type AutoIsizex4 = AutoSimd<[isize; 4]>;
pub type AutoIsizex8 = AutoSimd<[isize; 8]>;
pub type AutoU128x1 = AutoSimd<[u128; 1]>;
pub type AutoU128x2 = AutoSimd<[u128; 2]>;
pub type AutoU128x4 = AutoSimd<[u128; 4]>;
pub type AutoU16x2 = AutoSimd<[u16; 2]>;
pub type AutoU16x4 = AutoSimd<[u16; 4]>;
pub type AutoU16x8 = AutoSimd<[u16; 8]>;
pub type AutoU16x16 = AutoSimd<[u16; 16]>;
pub type AutoU16x32 = AutoSimd<[u16; 32]>;
pub type AutoU32x2 = AutoSimd<[u32; 2]>;
pub type AutoU32x4 = AutoSimd<[u32; 4]>;
pub type AutoU32x8 = AutoSimd<[u32; 8]>;
pub type AutoU32x16 = AutoSimd<[u32; 16]>;
pub type AutoU64x2 = AutoSimd<[u64; 2]>;
pub type AutoU64x4 = AutoSimd<[u64; 4]>;
pub type AutoU64x8 = AutoSimd<[u64; 8]>;
pub type AutoU8x2 = AutoSimd<[u8; 2]>;
pub type AutoU8x4 = AutoSimd<[u8; 4]>;
pub type AutoU8x8 = AutoSimd<[u8; 8]>;
pub type AutoU8x16 = AutoSimd<[u8; 16]>;
pub type AutoU8x32 = AutoSimd<[u8; 32]>;
// pub type AutoU8x64 = AutoSimd<[u8; 64]>;
pub type AutoUsizex2 = AutoSimd<[usize; 2]>;
pub type AutoUsizex4 = AutoSimd<[usize; 4]>;
pub type AutoUsizex8 = AutoSimd<[usize; 8]>;

pub type AutoBoolx1 = AutoSimd<[bool; 1]>;
pub type AutoBoolx16 = AutoSimd<[bool; 16]>;
pub type AutoBoolx2 = AutoSimd<[bool; 2]>;
pub type AutoBoolx32 = AutoSimd<[bool; 32]>;
pub type AutoBoolx4 = AutoSimd<[bool; 4]>;
// pub type AutoBoolx64 = AutoSimd<[bool; 64]>;
pub type AutoBoolx8 = AutoSimd<[bool; 8]>;

/*
 * Helper trait to transform an array.
 */
trait ArrTransform: SimdValue {
    fn map(self, f: impl Fn(Self::Element) -> Self::Element) -> Self;
    fn zip_map(
        self,
        other: Self,
        f: impl Fn(Self::Element, Self::Element) -> Self::Element,
    ) -> Self;
    fn zip_zip_map(
        self,
        b: Self,
        c: Self,
        f: impl Fn(Self::Element, Self::Element, Self::Element) -> Self::Element,
    ) -> Self;
    fn map_bool(self, f: impl Fn(Self::Element) -> bool) -> Self::SimdBool;
    fn zip_map_bool(
        self,
        other: Self,
        f: impl Fn(Self::Element, Self::Element) -> bool,
    ) -> Self::SimdBool;
}
