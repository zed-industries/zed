#![allow(missing_docs)]
#![allow(non_camel_case_types)] // For the simd type aliases.

//! Traits for SIMD values.

use crate::scalar::{ComplexField, Field, SubsetOf, SupersetOf};
use crate::simd::{
    PrimitiveSimdValue, SimdBool, SimdComplexField, SimdPartialOrd, SimdRealField, SimdSigned,
    SimdValue,
};
use approx::AbsDiffEq;
use num::{FromPrimitive, Num, One, Zero};
use num_traits::Bounded;
use std::{
    cmp::PartialEq,
    ops::{
        Add, AddAssign, BitAnd, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem,
        RemAssign, Sub, SubAssign,
    },
};
use wide::{CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

#[cfg(feature = "rkyv")]
macro_rules! impl_rkyv {
    ($type:ty, $array:ty) => {
        impl rkyv::Archive for $type {
            type Archived = $array;
            type Resolver = ();

            #[inline]
            unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
                out.write((*self).into_arr());
            }
        }

        impl<S: rkyv::Fallible + ?Sized> rkyv::Serialize<S> for $type {
            #[inline]
            fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }

        impl<D: rkyv::Fallible + ?Sized> rkyv::Deserialize<$type, D> for rkyv::Archived<$type> {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok(<$type>::from_arr(*self))
            }
        }
    };
}

/// A wrapper type of `wide::f32x4` that implements all the relevant traits from `num` and `simba`.
///
/// This is needed to overcome the orphan rules.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WideF32x4(pub wide::f32x4);

#[cfg(feature = "rkyv")]
impl_rkyv!(WideF32x4, [f32; 4]);

/// An SIMD boolean structure associated to `wide::f32x4` that implements all the relevant traits from `simba`.
///
/// This is needed to overcome the orphan rules.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WideBoolF32x4(pub wide::f32x4);

#[cfg(feature = "rkyv")]
impl_rkyv!(WideBoolF32x4, [f32; 4]);

/// A wrapper type of `wide::f32x8` that implements all the relevant traits from `num` and `simba`.
///
/// This is needed to overcome the orphan rules.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WideF32x8(pub wide::f32x8);

#[cfg(feature = "rkyv")]
impl_rkyv!(WideF32x8, [f32; 8]);

/// An SIMD boolean structure associated to `wide::f32x8` that implements all the relevant traits from `simba`.
///
/// This is needed to overcome the orphan rules.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WideBoolF32x8(pub wide::f32x8);

#[cfg(feature = "rkyv")]
impl_rkyv!(WideBoolF32x8, [f32; 8]);

/// A wrapper type of `wide::f64x4` that implements all the relevant traits from `num` and `simba`.
///
/// This is needed to overcome the orphan rules.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WideF64x4(pub wide::f64x4);

#[cfg(feature = "rkyv")]
impl_rkyv!(WideF64x4, [f64; 4]);

/// An SIMD boolean structure associated to `wide::f64x4` that implements all the relevant traits from `simba`.
///
/// This is needed to overcome the orphan rules.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WideBoolF64x4(pub wide::f64x4);

#[cfg(feature = "rkyv")]
impl_rkyv!(WideBoolF64x4, [f64; 4]);

macro_rules! impl_wide_f32 (
    ($f32: ident, $f32xX: ident, $WideF32xX: ident, $WideBoolF32xX: ident, $lanes: expr; $($ii: expr),+) => {
        impl PrimitiveSimdValue for $WideF32xX {}
        impl PrimitiveSimdValue for $WideBoolF32xX {}

        impl $WideF32xX {
            pub const ZERO: Self = $WideF32xX(<wide::$f32xX>::ZERO);
            pub const ONE: Self = $WideF32xX(<wide::$f32xX>::ONE);

            #[inline(always)]
            pub fn into_arr(self) -> [$f32; $lanes] {
                self.0.into()
            }

            #[inline(always)]
            pub fn from_arr(arr: [$f32; $lanes]) -> Self {
                Self(arr.into())
            }

            #[inline(always)]
            pub fn map(self, f: impl Fn($f32) -> $f32) -> Self {
                let arr = self.into_arr();
                Self::from([f(arr[0]), $(f(arr[$ii])),+])
            }

            #[inline(always)]
            pub fn zip_map(self, rhs: Self, f: impl Fn($f32, $f32) -> $f32) -> Self {
                let arr = self.into_arr();
                let rhs = rhs.into_arr();
                Self::from([
                    f(arr[0], rhs[0]),
                    $(f(arr[$ii], rhs[$ii])),+
                ])
            }
        }

        impl $WideBoolF32xX {
            #[inline(always)]
            pub fn from_arr(arr: [$f32; $lanes]) -> Self {
                Self(arr.into())
            }

            #[inline(always)]
            pub fn into_arr(self) -> [$f32; $lanes] {
                self.0.into()
            }
        }

        impl SimdValue for $WideF32xX {
            const LANES: usize = $lanes;
            type Element = $f32;
            type SimdBool = $WideBoolF32xX;

            #[inline(always)]
            fn splat(val: Self::Element) -> Self {
                $WideF32xX(wide::$f32xX::from(val))
            }

            #[inline(always)]
            fn extract(&self, i: usize) -> Self::Element {
                self.into_arr()[i]
            }

            #[inline(always)]
            unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
                *self.into_arr().get_unchecked(i)
            }

            #[inline(always)]
            fn replace(&mut self, i: usize, val: Self::Element) {
                let mut arr = self.into_arr();
                arr[i] = val;
                *self = Self::from(arr);
            }

            #[inline(always)]
            unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
                let mut arr = self.into_arr();
                *arr.get_unchecked_mut(i) = val;
                *self = Self::from(arr);
            }

            #[inline(always)]
            fn select(self, cond: Self::SimdBool, other: Self) -> Self {
                $WideF32xX(cond.0.blend(self.0, other.0))
            }
        }

        impl SimdValue for $WideBoolF32xX {
            const LANES: usize = $lanes;
            type Element = bool;
            type SimdBool = Self;

            #[inline(always)]
            fn splat(val: bool) -> Self {
                let results = [
                    $WideBoolF32xX(wide::$f32xX::ZERO),
                    $WideBoolF32xX(!wide::$f32xX::ZERO),
                ];
                results[val as usize]
            }

            #[inline(always)]
            fn extract(&self, i: usize) -> Self::Element {
                self.into_arr()[i] != 0.0
            }

            #[inline(always)]
            unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
                *self.into_arr().get_unchecked(i) != 0.0
            }

            #[inline(always)]
            fn replace(&mut self, i: usize, val: Self::Element) {
                let vals = [0.0, <$f32>::from_bits(Bounded::max_value())];
                let mut arr = self.into_arr();
                arr[i] = vals[val as usize];
                *self = Self::from_arr(arr);
            }

            #[inline(always)]
            unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
                let vals = [0.0, <$f32>::from_bits(Bounded::max_value())];
                let mut arr = self.into_arr();
                *arr.get_unchecked_mut(i) = vals[val as usize];
                *self = Self::from_arr(arr);
            }

            #[inline(always)]
            fn select(self, cond: Self::SimdBool, other: Self) -> Self {
                $WideBoolF32xX(cond.0.blend(self.0, other.0))
            }
        }

        impl PartialEq for $WideF32xX {
            #[inline]
            fn eq(&self, rhs: &Self) -> bool {
                self.0 == rhs.0
            }
        }

        impl PartialEq for $WideBoolF32xX {
            #[inline]
            fn eq(&self, rhs: &Self) -> bool {
                self.0 == rhs.0
            }
        }

        impl Not for $WideBoolF32xX {
            type Output = Self;

            #[inline]
            fn not(self) -> Self {
                Self(!self.0)
            }
        }

        impl BitXor for $WideBoolF32xX {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl BitOr for $WideBoolF32xX {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl BitAnd for $WideBoolF32xX {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl SimdBool for $WideBoolF32xX {
            #[inline(always)]
            fn bitmask(self) -> u64 {
                let arr = self.into_arr();
                (((arr[0] != 0.0) as u64) << 0)
                    $(| (((arr[$ii] != 0.0) as u64) << $ii))*
            }

            #[inline(always)]
            fn and(self) -> bool {
                let arr = self.into_arr();
                (arr[0].to_bits() $(& arr[$ii].to_bits())*) != 0
            }

            #[inline(always)]
            fn or(self) -> bool {
                let arr = self.into_arr();
                (arr[0].to_bits() $(| arr[$ii].to_bits())*) != 0
            }

            #[inline(always)]
            fn xor(self) -> bool {
                let arr = self.into_arr();
                (arr[0].to_bits() $(^ arr[$ii].to_bits())*) != 0
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
                self.0.none()
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

        impl From<[$f32; $lanes]> for $WideF32xX {
            #[inline(always)]
            fn from(vals: [$f32; $lanes]) -> Self {
                $WideF32xX(wide::$f32xX::from(vals))
            }
        }

        impl From<$WideF32xX> for [$f32; $lanes] {
            #[inline(always)]
            fn from(val: $WideF32xX) -> [$f32; $lanes] {
                val.0.into()
            }
        }

        impl SubsetOf<$WideF32xX> for $WideF32xX {
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

        impl From<[bool; $lanes]> for $WideBoolF32xX {
            #[inline(always)]
            fn from(vals: [bool; $lanes]) -> Self {
                let bits = [0.0, <$f32>::from_bits(Bounded::max_value())];
                $WideBoolF32xX(wide::$f32xX::from([
                    bits[vals[0] as usize],
                    $(bits[vals[$ii] as usize]),*
                ]))
            }
        }

        impl SubsetOf<$WideBoolF32xX> for $WideBoolF32xX {
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

        impl Num for $WideF32xX {
            type FromStrRadixErr = <$f32 as Num>::FromStrRadixErr;

            #[inline(always)]
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                <$f32>::from_str_radix(str, radix).map(Self::splat)
            }
        }

        impl FromPrimitive for $WideF32xX {
            #[inline(always)]
            fn from_i64(n: i64) -> Option<Self> {
                <$f32>::from_i64(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u64(n: u64) -> Option<Self> {
                <$f32>::from_u64(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_isize(n: isize) -> Option<Self> {
                <$f32>::from_isize(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_i8(n: i8) -> Option<Self> {
                <$f32>::from_i8(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_i16(n: i16) -> Option<Self> {
                <$f32>::from_i16(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_i32(n: i32) -> Option<Self> {
                <$f32>::from_i32(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_usize(n: usize) -> Option<Self> {
                <$f32>::from_usize(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u8(n: u8) -> Option<Self> {
                <$f32>::from_u8(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u16(n: u16) -> Option<Self> {
                <$f32>::from_u16(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u32(n: u32) -> Option<Self> {
                <$f32>::from_u32(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_f32(n: f32) -> Option<Self> {
                <$f32>::from_f32(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_f64(n: f64) -> Option<Self> {
                <$f32>::from_f64(n).map(Self::splat)
            }
        }

        impl Zero for $WideF32xX {
            #[inline(always)]
            fn zero() -> Self {
                <$WideF32xX>::splat(<$f32>::zero())
            }

            #[inline(always)]
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }

        impl One for $WideF32xX {
            #[inline(always)]
            fn one() -> Self {
                <$WideF32xX>::splat(<$f32>::one())
            }
        }

        impl Add<$WideF32xX> for $WideF32xX {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl Sub<$WideF32xX> for $WideF32xX {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl Mul<$WideF32xX> for $WideF32xX {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                Self(self.0 * rhs.0)
            }
        }

        impl Div<$WideF32xX> for $WideF32xX {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                Self(self.0 / rhs.0)
            }
        }

        impl Rem<$WideF32xX> for $WideF32xX {
            type Output = Self;

            #[inline(always)]
            fn rem(self, rhs: Self) -> Self {
                self.zip_map(rhs, |a, b| a % b)
            }
        }

        impl AddAssign<$WideF32xX> for $WideF32xX {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl SubAssign<$WideF32xX> for $WideF32xX {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl DivAssign<$WideF32xX> for $WideF32xX {
            #[inline(always)]
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0
            }
        }

        impl MulAssign<$WideF32xX> for $WideF32xX {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0
            }
        }

        impl RemAssign<$WideF32xX> for $WideF32xX {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs;
            }
        }

        impl SimdPartialOrd for $WideF32xX {
            #[inline(always)]
            fn simd_gt(self, other: Self) -> Self::SimdBool {
                $WideBoolF32xX(self.0.cmp_gt(other.0))
            }

            #[inline(always)]
            fn simd_lt(self, other: Self) -> Self::SimdBool {
                $WideBoolF32xX(self.0.cmp_lt(other.0))
            }

            #[inline(always)]
            fn simd_ge(self, other: Self) -> Self::SimdBool {
                $WideBoolF32xX(self.0.cmp_ge(other.0))
            }

            #[inline(always)]
            fn simd_le(self, other: Self) -> Self::SimdBool {
                $WideBoolF32xX(self.0.cmp_le(other.0))
            }

            #[inline(always)]
            fn simd_eq(self, other: Self) -> Self::SimdBool {
                $WideBoolF32xX(self.0.cmp_eq(other.0))
            }

            #[inline(always)]
            fn simd_ne(self, other: Self) -> Self::SimdBool {
                $WideBoolF32xX(self.0.cmp_ne(other.0))
            }

            #[inline(always)]
            fn simd_max(self, other: Self) -> Self {
                $WideF32xX(self.0.max(other.0))
            }
            #[inline(always)]
            fn simd_min(self, other: Self) -> Self {
                $WideF32xX(self.0.min(other.0))
            }

            #[inline(always)]
            fn simd_clamp(self, min: Self, max: Self) -> Self {
                self.simd_min(max).simd_max(min)
            }

            #[inline(always)]
            fn simd_horizontal_min(self) -> Self::Element {
                let arr = self.into_arr();
                arr[0]$(.min(arr[$ii]))*
            }

            #[inline(always)]
            fn simd_horizontal_max(self) -> Self::Element {
                let arr = self.into_arr();
                arr[0]$(.max(arr[$ii]))*
            }
        }

        impl Neg for $WideF32xX {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl SimdSigned for $WideF32xX {
            #[inline(always)]
            fn simd_abs(&self) -> Self {
                $WideF32xX(self.0.abs())
            }

            #[inline(always)]
            fn simd_abs_sub(&self, other: &Self) -> Self {
                $WideF32xX((self.0 - other.0).max(Self::zero().0))
            }

            #[inline(always)]
            fn simd_signum(&self) -> Self {
                // TODO: is there a more efficient way?
                self.map(|x| x.signum())
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

        impl Field for $WideF32xX {}

        impl SimdRealField for $WideF32xX {
            #[inline(always)]
            fn simd_atan2(self, other: Self) -> Self {
                self.zip_map_lanes(other, |a, b| a.atan2(b))
            }

            #[inline(always)]
            fn simd_copysign(self, sign: Self) -> Self {
                let neg_zero = wide::$f32xX::from(-0.0);
                $WideF32xX((neg_zero & sign.0) | ((!neg_zero) & self.0))
            }

            #[inline(always)]
            fn simd_default_epsilon() -> Self {
                Self::splat(<$f32>::default_epsilon())
            }

            #[inline(always)]
            fn simd_pi() -> Self {
                $WideF32xX(wide::$f32xX::PI)
            }

            #[inline(always)]
            fn simd_two_pi() -> Self {
                $WideF32xX(wide::$f32xX::PI + wide::$f32xX::PI)
            }

            #[inline(always)]
            fn simd_frac_pi_2() -> Self {
                $WideF32xX(wide::$f32xX::FRAC_PI_2)
            }

            #[inline(always)]
            fn simd_frac_pi_3() -> Self {
                $WideF32xX(wide::$f32xX::FRAC_PI_3)
            }

            #[inline(always)]
            fn simd_frac_pi_4() -> Self {
                $WideF32xX(wide::$f32xX::FRAC_PI_4)
            }

            #[inline(always)]
            fn simd_frac_pi_6() -> Self {
                $WideF32xX(wide::$f32xX::FRAC_PI_6)
            }

            #[inline(always)]
            fn simd_frac_pi_8() -> Self {
                $WideF32xX(wide::$f32xX::FRAC_PI_8)
            }

            #[inline(always)]
            fn simd_frac_1_pi() -> Self {
                $WideF32xX(wide::$f32xX::FRAC_1_PI)
            }

            #[inline(always)]
            fn simd_frac_2_pi() -> Self {
                $WideF32xX(wide::$f32xX::FRAC_2_PI)
            }

            #[inline(always)]
            fn simd_frac_2_sqrt_pi() -> Self {
                $WideF32xX(wide::$f32xX::FRAC_2_SQRT_PI)
            }

            #[inline(always)]
            fn simd_e() -> Self {
                $WideF32xX(wide::$f32xX::E)
            }

            #[inline(always)]
            fn simd_log2_e() -> Self {
                $WideF32xX(wide::$f32xX::LOG2_E)
            }

            #[inline(always)]
            fn simd_log10_e() -> Self {
                $WideF32xX(wide::$f32xX::LOG10_E)
            }

            #[inline(always)]
            fn simd_ln_2() -> Self {
                $WideF32xX(wide::$f32xX::LN_2)
            }

            #[inline(always)]
            fn simd_ln_10() -> Self {
                $WideF32xX(wide::$f32xX::LN_10)
            }
        }

        impl SimdComplexField for $WideF32xX {
            type SimdRealField = Self;

            #[inline(always)]
            fn simd_horizontal_sum(self) -> Self::Element {
                self.0.reduce_add()
            }

            #[inline(always)]
            fn simd_horizontal_product(self) -> Self::Element {
                self.extract(0) $(* self.extract($ii))*
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
                $WideF32xX(self.0.abs())
            }

            #[inline(always)]
            fn simd_modulus(self) -> Self::SimdRealField {
                $WideF32xX(self.0.abs())
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
                let ge = self.0.cmp_ge(Self::one().0);
                let exp = ge.blend(Self::one().0, -Self::one().0);
                ($WideF32xX(self.0 * exp), $WideF32xX(exp))
            }

            #[inline(always)]
            fn simd_recip(self) -> Self {
                Self::one() / self
            }

            #[inline(always)]
            fn simd_conjugate(self) -> Self {
                self
            }

            #[inline(always)]
            fn simd_scale(self, factor: Self::SimdRealField) -> Self {
                $WideF32xX(self.0 * factor.0)
            }

            #[inline(always)]
            fn simd_unscale(self, factor: Self::SimdRealField) -> Self {
                $WideF32xX(self.0 / factor.0)
            }

            #[inline(always)]
            fn simd_floor(self) -> Self {
                self.map_lanes(|e| e.floor())
            }

            #[inline(always)]
            fn simd_ceil(self) -> Self {
                self.map_lanes(|e| e.ceil())
            }

            #[inline(always)]
            fn simd_round(self) -> Self {
                self.map_lanes(|e| e.round())
            }

            #[inline(always)]
            fn simd_trunc(self) -> Self {
                self.map_lanes(|e| e.trunc())
            }

            #[inline(always)]
            fn simd_fract(self) -> Self {
                self.map_lanes(|e| e.fract())
            }

            #[inline(always)]
            fn simd_abs(self) -> Self {
                $WideF32xX(self.0.abs())
            }

            #[inline(always)]
            fn simd_signum(self) -> Self {
                self.map_lanes(|e| e.signum())
            }

            #[inline(always)]
            fn simd_mul_add(self, a: Self, b: Self) -> Self {
                $WideF32xX(self.0.mul_add(a.0, b.0))
            }

            #[inline(always)]
            fn simd_powi(self, n: i32) -> Self {
                self.map_lanes(|e| e.powi(n))
            }

            #[inline(always)]
            fn simd_powf(self, n: Self) -> Self {
                self.zip_map_lanes(n, |e, n| e.powf(n))
            }

            #[inline(always)]
            fn simd_powc(self, n: Self) -> Self {
                self.zip_map_lanes(n, |e, n| e.powf(n))
            }

            #[inline(always)]
            fn simd_sqrt(self) -> Self {
                $WideF32xX(self.0.sqrt())
            }

            #[inline(always)]
            fn simd_exp(self) -> Self {
                self.map_lanes(|e| e.exp())
            }

            #[inline(always)]
            fn simd_exp2(self) -> Self {
                self.map_lanes(|e| e.exp2())
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
                self.map_lanes(|e| e.ln())
            }

            #[inline(always)]
            fn simd_log(self, base: Self) -> Self {
                self.zip_map_lanes(base, |e, b| e.log(b))
            }

            #[inline(always)]
            fn simd_log2(self) -> Self {
                self.map_lanes(|e| e.log2())
            }

            #[inline(always)]
            fn simd_log10(self) -> Self {
                self.map_lanes(|e| e.log10())
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
                $WideF32xX(self.0.sin())
            }

            #[inline(always)]
            fn simd_cos(self) -> Self {
                $WideF32xX(self.0.cos())
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
                let (sin, cos) = self.0.sin_cos();
                ($WideF32xX(sin), $WideF32xX(cos))
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
        impl SimdComplexField for num_complex::Complex<$WideF32xX> {
            type SimdRealField = $WideF32xX;

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
                let _2 = <$WideF32xX>::one() + <$WideF32xX>::one();
                num_complex::Complex::new(_2, <$WideF32xX>::zero()).simd_powc(self)
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
                let _2 = <$WideF32xX>::one() + <$WideF32xX>::one();
                self.simd_log(_2)
            }

            #[inline]
            fn simd_log10(self) -> Self {
                let _10 = <$WideF32xX>::from_subset(&10.0f64);
                self.simd_log(_10)
            }

            #[inline]
            fn simd_cbrt(self) -> Self {
                let one_third = <$WideF32xX>::from_subset(&(1.0 / 3.0));
                self.simd_powf(one_third)
            }

            #[inline]
            fn simd_powi(self, n: i32) -> Self {
                // TODO: is there a more accurate solution?
                let n = <$WideF32xX>::from_subset(&(n as f64));
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
                let two = <$WideF32xX>::one() + <$WideF32xX>::one();
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
            fn simd_log(self, base: $WideF32xX) -> Self {
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
                Self::new(two_re.simd_sin(), two_im.simd_sinh())
                    .unscale(two_re.simd_cos() + two_im.simd_cosh())
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
                    return Self::new(<$WideF32xX>::zero(), <$WideF32xX>::one() / <$WideF32xX>::zero());
                } else if self == -i {
                    return Self::new(<$WideF32xX>::zero(), -<$WideF32xX>::one() / <$WideF32xX>::zero());
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
                Self::new(two_re.simd_sinh(), two_im.simd_sin())
                    .unscale(two_re.simd_cosh() + two_im.simd_cos())
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
                    return Self::new(<$WideF32xX>::one() / <$WideF32xX>::zero(), <$WideF32xX>::zero());
                } else if self == -one {
                    return Self::new(-<$WideF32xX>::one() / <$WideF32xX>::zero(), <$WideF32xX>::zero());
                }
                ((one + self).simd_ln() - (one - self).simd_ln()) / two
            }
        }
    }
);

macro_rules! impl_scalar_subset_of_simd (
    ($WideF32xX: ty, $f32: ty, $lanes: expr; $($t: ty),*) => {$(
        impl SubsetOf<$WideF32xX> for $t {
            #[inline(always)]
            fn to_superset(&self) -> $WideF32xX {
                <$WideF32xX>::splat(<$f32>::from_subset(self))
            }

            #[inline(always)]
            fn from_superset_unchecked(element: &$WideF32xX) -> $t {
                element.extract(0).to_subset_unchecked()
            }

            #[inline(always)]
            fn is_in_subset(c: &$WideF32xX) -> bool {
                let elt0 = c.extract(0);
                <$t as SubsetOf<$f32>>::is_in_subset(&elt0) &&
                (1..$lanes).all(|i| c.extract(i) == elt0)
            }
        }
    )*}
);

impl_scalar_subset_of_simd!(WideF32x4, f32, 4; u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
impl_scalar_subset_of_simd!(WideF64x4, f64, 4; u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
//#[cfg(feature = "decimal")]
//impl_scalar_subset_of_simd!(WideF32x4, 4; d128);
impl_scalar_subset_of_simd!(WideF32x8, f32, 8; u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
//#[cfg(feature = "decimal")]
//impl_scalar_subset_of_simd!(WideF32x8, 8; d128);

// NOTE: don’t include the 0 for the indices because they are taken care
// for explicitly in the macro (it’s simpler that way).
impl_wide_f32!(f32, f32x4, WideF32x4, WideBoolF32x4, 4; 1, 2, 3);
impl_wide_f32!(f64, f64x4, WideF64x4, WideBoolF64x4, 4; 1, 2, 3);
impl_wide_f32!(f32, f32x8, WideF32x8, WideBoolF32x8, 8; 1, 2, 3, 4, 5, 6, 7);

#[inline]
fn simd_complex_from_polar<N: SimdRealField>(r: N, theta: N) -> num_complex::Complex<N> {
    num_complex::Complex::new(r.clone() * theta.clone().simd_cos(), r * theta.simd_sin())
}
