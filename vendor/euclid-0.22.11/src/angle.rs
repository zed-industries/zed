// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::approxeq::ApproxEq;
use crate::trig::Trig;

use core::cmp::{Eq, PartialEq};
use core::hash::Hash;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
use num_traits::real::Real;
use num_traits::{Float, FloatConst, NumCast, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An angle in radians
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Angle<T> {
    pub radians: T,
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable> Zeroable for Angle<T> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod> Pod for Angle<T> {}

#[cfg(feature = "arbitrary")]
impl<'a, T> arbitrary::Arbitrary<'a> for Angle<T>
where
    T: arbitrary::Arbitrary<'a>,
{
    // This implementation could be derived, but the derive would require an `extern crate std`.
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Angle {
            radians: arbitrary::Arbitrary::arbitrary(u)?,
        })
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        <T as arbitrary::Arbitrary>::size_hint(depth)
    }
}

impl<T> Angle<T> {
    #[inline]
    pub fn radians(radians: T) -> Self {
        Angle { radians }
    }

    #[inline]
    pub fn get(self) -> T {
        self.radians
    }
}

impl<T> Angle<T>
where
    T: Trig,
{
    #[inline]
    pub fn degrees(deg: T) -> Self {
        Angle {
            radians: T::degrees_to_radians(deg),
        }
    }

    #[inline]
    pub fn to_degrees(self) -> T {
        T::radians_to_degrees(self.radians)
    }
}

impl<T> Angle<T>
where
    T: Rem<Output = T> + Sub<Output = T> + Add<Output = T> + Zero + FloatConst + PartialOrd + Copy,
{
    /// Returns this angle in the [0..2*PI[ range.
    pub fn positive(&self) -> Self {
        let two_pi = T::PI() + T::PI();
        let mut a = self.radians % two_pi;
        if a < T::zero() {
            a = a + two_pi;
        }
        Angle::radians(a)
    }

    /// Returns this angle in the ]-PI..PI] range.
    pub fn signed(&self) -> Self {
        Angle::pi() - (Angle::pi() - *self).positive()
    }
}

impl<T> Angle<T>
where
    T: Rem<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + One
        + FloatConst
        + Copy,
{
    /// Returns the shortest signed angle between two angles.
    ///
    /// Takes wrapping and signs into account.
    pub fn angle_to(&self, to: Self) -> Self {
        let two = T::one() + T::one();
        let max = T::PI() * two;
        let d = (to.radians - self.radians) % max;

        Angle::radians(two * d % max - d)
    }

    /// Linear interpolation between two angles, using the shortest path.
    pub fn lerp(&self, other: Self, t: T) -> Self {
        *self + self.angle_to(other) * t
    }
}

impl<T> Angle<T>
where
    T: Float,
{
    /// Returns `true` if the angle is a finite number.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.radians.is_finite()
    }
}

impl<T> Angle<T>
where
    T: Real,
{
    /// Returns `(sin(self), cos(self))`.
    pub fn sin_cos(self) -> (T, T) {
        self.radians.sin_cos()
    }
}

impl<T> Angle<T>
where
    T: Zero,
{
    pub fn zero() -> Self {
        Angle::radians(T::zero())
    }
}

impl<T> Angle<T>
where
    T: FloatConst + Add<Output = T>,
{
    pub fn pi() -> Self {
        Angle::radians(T::PI())
    }

    pub fn two_pi() -> Self {
        Angle::radians(T::PI() + T::PI())
    }

    pub fn frac_pi_2() -> Self {
        Angle::radians(T::FRAC_PI_2())
    }

    pub fn frac_pi_3() -> Self {
        Angle::radians(T::FRAC_PI_3())
    }

    pub fn frac_pi_4() -> Self {
        Angle::radians(T::FRAC_PI_4())
    }
}

impl<T> Angle<T>
where
    T: NumCast + Copy,
{
    /// Cast from one numeric representation to another.
    #[inline]
    pub fn cast<NewT: NumCast>(&self) -> Angle<NewT> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another.
    pub fn try_cast<NewT: NumCast>(&self) -> Option<Angle<NewT>> {
        NumCast::from(self.radians).map(|radians| Angle { radians })
    }

    // Convenience functions for common casts.

    /// Cast angle to `f32`.
    #[inline]
    pub fn to_f32(&self) -> Angle<f32> {
        self.cast()
    }

    /// Cast angle `f64`.
    #[inline]
    pub fn to_f64(&self) -> Angle<f64> {
        self.cast()
    }
}

impl<T: Add<T, Output = T>> Add for Angle<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::radians(self.radians + other.radians)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<&Self> for Angle<T> {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        Self::radians(self.radians + other.radians)
    }
}

impl<T: Add + Zero> Sum for Angle<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T: 'a + Add + Copy + Zero> Sum<&'a Self> for Angle<T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<T: AddAssign<T>> AddAssign for Angle<T> {
    fn add_assign(&mut self, other: Angle<T>) {
        self.radians += other.radians;
    }
}

impl<T: Sub<T, Output = T>> Sub<Angle<T>> for Angle<T> {
    type Output = Angle<T>;
    fn sub(self, other: Angle<T>) -> <Self as Sub>::Output {
        Angle::radians(self.radians - other.radians)
    }
}

impl<T: SubAssign<T>> SubAssign for Angle<T> {
    fn sub_assign(&mut self, other: Angle<T>) {
        self.radians -= other.radians;
    }
}

impl<T: Div<T, Output = T>> Div<Angle<T>> for Angle<T> {
    type Output = T;
    #[inline]
    fn div(self, other: Angle<T>) -> T {
        self.radians / other.radians
    }
}

impl<T: Div<T, Output = T>> Div<T> for Angle<T> {
    type Output = Angle<T>;
    #[inline]
    fn div(self, factor: T) -> Angle<T> {
        Angle::radians(self.radians / factor)
    }
}

impl<T: DivAssign<T>> DivAssign<T> for Angle<T> {
    fn div_assign(&mut self, factor: T) {
        self.radians /= factor;
    }
}

impl<T: Mul<T, Output = T>> Mul<T> for Angle<T> {
    type Output = Angle<T>;
    #[inline]
    fn mul(self, factor: T) -> Angle<T> {
        Angle::radians(self.radians * factor)
    }
}

impl<T: MulAssign<T>> MulAssign<T> for Angle<T> {
    fn mul_assign(&mut self, factor: T) {
        self.radians *= factor;
    }
}

impl<T: Neg<Output = T>> Neg for Angle<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Angle::radians(-self.radians)
    }
}

impl<T: ApproxEq<T>> ApproxEq<T> for Angle<T> {
    #[inline]
    fn approx_epsilon() -> T {
        T::approx_epsilon()
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Angle<T>, approx_epsilon: &T) -> bool {
        self.radians.approx_eq_eps(&other.radians, approx_epsilon)
    }
}

#[test]
fn wrap_angles() {
    use core::f32::consts::{FRAC_PI_2, PI};

    assert!(Angle::radians(0.0).positive().approx_eq(&Angle::zero()));
    assert!(Angle::radians(FRAC_PI_2)
        .positive()
        .approx_eq(&Angle::frac_pi_2()));
    assert!(Angle::radians(-FRAC_PI_2)
        .positive()
        .approx_eq(&Angle::radians(3.0 * FRAC_PI_2)));
    assert!(Angle::radians(3.0 * FRAC_PI_2)
        .positive()
        .approx_eq(&Angle::radians(3.0 * FRAC_PI_2)));
    assert!(Angle::radians(5.0 * FRAC_PI_2)
        .positive()
        .approx_eq(&Angle::frac_pi_2()));
    assert!(Angle::radians(2.0 * PI)
        .positive()
        .approx_eq(&Angle::zero()));
    assert!(Angle::radians(-2.0 * PI)
        .positive()
        .approx_eq(&Angle::zero()));
    assert!(Angle::radians(PI).positive().approx_eq(&Angle::pi()));
    assert!(Angle::radians(-PI).positive().approx_eq(&Angle::pi()));

    assert!(Angle::radians(FRAC_PI_2)
        .signed()
        .approx_eq(&Angle::frac_pi_2()));
    assert!(Angle::radians(3.0 * FRAC_PI_2)
        .signed()
        .approx_eq(&-Angle::frac_pi_2()));
    assert!(Angle::radians(5.0 * FRAC_PI_2)
        .signed()
        .approx_eq(&Angle::frac_pi_2()));
    assert!(Angle::radians(2.0 * PI).signed().approx_eq(&Angle::zero()));
    assert!(Angle::radians(-2.0 * PI).signed().approx_eq(&Angle::zero()));
    assert!(Angle::radians(-PI).signed().approx_eq(&Angle::pi()));
    assert!(Angle::radians(PI).signed().approx_eq(&Angle::pi()));
}

#[test]
fn lerp() {
    type A = Angle<f32>;

    let a = A::radians(1.0);
    let b = A::radians(2.0);
    assert!(a.lerp(b, 0.25).approx_eq(&Angle::radians(1.25)));
    assert!(a.lerp(b, 0.5).approx_eq(&Angle::radians(1.5)));
    assert!(a.lerp(b, 0.75).approx_eq(&Angle::radians(1.75)));
    assert!(a
        .lerp(b + A::two_pi(), 0.75)
        .approx_eq(&Angle::radians(1.75)));
    assert!(a
        .lerp(b - A::two_pi(), 0.75)
        .approx_eq(&Angle::radians(1.75)));
    assert!(a
        .lerp(b + A::two_pi() * 5.0, 0.75)
        .approx_eq(&Angle::radians(1.75)));
}

#[test]
fn sum() {
    type A = Angle<f32>;
    let angles = [A::radians(1.0), A::radians(2.0), A::radians(3.0)];
    let sum = A::radians(6.0);
    assert_eq!(angles.iter().sum::<A>(), sum);
}
