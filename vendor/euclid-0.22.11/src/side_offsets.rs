// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A group of side offsets, which correspond to top/left/bottom/right for borders, padding,
//! and margins in CSS.

use crate::length::Length;
use crate::num::Zero;
use crate::scale::Scale;
use crate::Vector2D;

use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A group of 2D side offsets, which correspond to top/right/bottom/left for borders, padding,
/// and margins in CSS, optionally tagged with a unit.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
)]
pub struct SideOffsets2D<T, U> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
    #[doc(hidden)]
    pub _unit: PhantomData<U>,
}

#[cfg(feature = "arbitrary")]
impl<'a, T, U> arbitrary::Arbitrary<'a> for SideOffsets2D<T, U>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (top, right, bottom, left) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(SideOffsets2D {
            top,
            right,
            bottom,
            left,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, U> Zeroable for SideOffsets2D<T, U> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, U: 'static> Pod for SideOffsets2D<T, U> {}

impl<T: Copy, U> Copy for SideOffsets2D<T, U> {}

impl<T: Clone, U> Clone for SideOffsets2D<T, U> {
    fn clone(&self) -> Self {
        SideOffsets2D {
            top: self.top.clone(),
            right: self.right.clone(),
            bottom: self.bottom.clone(),
            left: self.left.clone(),
            _unit: PhantomData,
        }
    }
}

impl<T, U> Eq for SideOffsets2D<T, U> where T: Eq {}

impl<T, U> PartialEq for SideOffsets2D<T, U>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top
            && self.right == other.right
            && self.bottom == other.bottom
            && self.left == other.left
    }
}

impl<T, U> Hash for SideOffsets2D<T, U>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.top.hash(h);
        self.right.hash(h);
        self.bottom.hash(h);
        self.left.hash(h);
    }
}

impl<T: fmt::Debug, U> fmt::Debug for SideOffsets2D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?},{:?},{:?},{:?})",
            self.top, self.right, self.bottom, self.left
        )
    }
}

impl<T: Default, U> Default for SideOffsets2D<T, U> {
    fn default() -> Self {
        SideOffsets2D {
            top: Default::default(),
            right: Default::default(),
            bottom: Default::default(),
            left: Default::default(),
            _unit: PhantomData,
        }
    }
}

impl<T, U> SideOffsets2D<T, U> {
    /// Constructor taking a scalar for each side.
    ///
    /// Sides are specified in top-right-bottom-left order following
    /// CSS's convention.
    pub const fn new(top: T, right: T, bottom: T, left: T) -> Self {
        SideOffsets2D {
            top,
            right,
            bottom,
            left,
            _unit: PhantomData,
        }
    }

    /// Constructor taking a typed Length for each side.
    ///
    /// Sides are specified in top-right-bottom-left order following
    /// CSS's convention.
    pub fn from_lengths(
        top: Length<T, U>,
        right: Length<T, U>,
        bottom: Length<T, U>,
        left: Length<T, U>,
    ) -> Self {
        SideOffsets2D::new(top.0, right.0, bottom.0, left.0)
    }

    /// Construct side offsets from min and a max vector offsets.
    ///
    /// The outer rect of the resulting side offsets is equivalent to translating
    /// a rectangle's upper-left corner with the min vector and translating the
    /// bottom-right corner with the max vector.
    pub fn from_vectors_outer(min: Vector2D<T, U>, max: Vector2D<T, U>) -> Self
    where
        T: Neg<Output = T>,
    {
        SideOffsets2D {
            left: -min.x,
            top: -min.y,
            right: max.x,
            bottom: max.y,
            _unit: PhantomData,
        }
    }

    /// Construct side offsets from min and a max vector offsets.
    ///
    /// The inner rect of the resulting side offsets is equivalent to translating
    /// a rectangle's upper-left corner with the min vector and translating the
    /// bottom-right corner with the max vector.
    pub fn from_vectors_inner(min: Vector2D<T, U>, max: Vector2D<T, U>) -> Self
    where
        T: Neg<Output = T>,
    {
        SideOffsets2D {
            left: min.x,
            top: min.y,
            right: -max.x,
            bottom: -max.y,
            _unit: PhantomData,
        }
    }

    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self
    where
        T: Zero,
    {
        SideOffsets2D::new(Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero())
    }

    /// Returns `true` if all side offsets are zero.
    pub fn is_zero(&self) -> bool
    where
        T: Zero + PartialEq,
    {
        let zero = T::zero();
        self.top == zero && self.right == zero && self.bottom == zero && self.left == zero
    }

    /// Constructor setting the same value to all sides, taking a scalar value directly.
    pub fn new_all_same(all: T) -> Self
    where
        T: Copy,
    {
        SideOffsets2D::new(all, all, all, all)
    }

    /// Constructor setting the same value to all sides, taking a typed Length.
    pub fn from_length_all_same(all: Length<T, U>) -> Self
    where
        T: Copy,
    {
        SideOffsets2D::new_all_same(all.0)
    }

    pub fn horizontal(&self) -> T
    where
        T: Copy + Add<T, Output = T>,
    {
        self.left + self.right
    }

    pub fn vertical(&self) -> T
    where
        T: Copy + Add<T, Output = T>,
    {
        self.top + self.bottom
    }
}

impl<T, U> Add for SideOffsets2D<T, U>
where
    T: Add<T, Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        SideOffsets2D::new(
            self.top + other.top,
            self.right + other.right,
            self.bottom + other.bottom,
            self.left + other.left,
        )
    }
}

impl<T, U> AddAssign<Self> for SideOffsets2D<T, U>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, other: Self) {
        self.top += other.top;
        self.right += other.right;
        self.bottom += other.bottom;
        self.left += other.left;
    }
}

impl<T, U> Sub for SideOffsets2D<T, U>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        SideOffsets2D::new(
            self.top - other.top,
            self.right - other.right,
            self.bottom - other.bottom,
            self.left - other.left,
        )
    }
}

impl<T, U> SubAssign<Self> for SideOffsets2D<T, U>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, other: Self) {
        self.top -= other.top;
        self.right -= other.right;
        self.bottom -= other.bottom;
        self.left -= other.left;
    }
}

impl<T, U> Neg for SideOffsets2D<T, U>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        SideOffsets2D {
            top: -self.top,
            right: -self.right,
            bottom: -self.bottom,
            left: -self.left,
            _unit: PhantomData,
        }
    }
}

impl<T: Copy + Mul, U> Mul<T> for SideOffsets2D<T, U> {
    type Output = SideOffsets2D<T::Output, U>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        SideOffsets2D::new(
            self.top * scale,
            self.right * scale,
            self.bottom * scale,
            self.left * scale,
        )
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for SideOffsets2D<T, U> {
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.top *= other;
        self.right *= other;
        self.bottom *= other;
        self.left *= other;
    }
}

impl<T: Copy + Mul, U1, U2> Mul<Scale<T, U1, U2>> for SideOffsets2D<T, U1> {
    type Output = SideOffsets2D<T::Output, U2>;

    #[inline]
    fn mul(self, scale: Scale<T, U1, U2>) -> Self::Output {
        SideOffsets2D::new(
            self.top * scale.0,
            self.right * scale.0,
            self.bottom * scale.0,
            self.left * scale.0,
        )
    }
}

impl<T: Copy + MulAssign, U> MulAssign<Scale<T, U, U>> for SideOffsets2D<T, U> {
    #[inline]
    fn mul_assign(&mut self, other: Scale<T, U, U>) {
        *self *= other.0;
    }
}

impl<T: Copy + Div, U> Div<T> for SideOffsets2D<T, U> {
    type Output = SideOffsets2D<T::Output, U>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        SideOffsets2D::new(
            self.top / scale,
            self.right / scale,
            self.bottom / scale,
            self.left / scale,
        )
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for SideOffsets2D<T, U> {
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.top /= other;
        self.right /= other;
        self.bottom /= other;
        self.left /= other;
    }
}

impl<T: Copy + Div, U1, U2> Div<Scale<T, U1, U2>> for SideOffsets2D<T, U2> {
    type Output = SideOffsets2D<T::Output, U1>;

    #[inline]
    fn div(self, scale: Scale<T, U1, U2>) -> Self::Output {
        SideOffsets2D::new(
            self.top / scale.0,
            self.right / scale.0,
            self.bottom / scale.0,
            self.left / scale.0,
        )
    }
}

impl<T: Copy + DivAssign, U> DivAssign<Scale<T, U, U>> for SideOffsets2D<T, U> {
    fn div_assign(&mut self, other: Scale<T, U, U>) {
        *self /= other.0;
    }
}

#[test]
fn from_vectors() {
    use crate::{point2, vec2};
    type Box2D = crate::default::Box2D<i32>;

    let b = Box2D {
        min: point2(10, 10),
        max: point2(20, 20),
    };

    let outer = b.outer_box(SideOffsets2D::from_vectors_outer(vec2(-1, -2), vec2(3, 4)));
    let inner = b.inner_box(SideOffsets2D::from_vectors_inner(vec2(1, 2), vec2(-3, -4)));

    assert_eq!(
        outer,
        Box2D {
            min: point2(9, 8),
            max: point2(23, 24)
        }
    );
    assert_eq!(
        inner,
        Box2D {
            min: point2(11, 12),
            max: point2(17, 16)
        }
    );
}

#[test]
fn test_is_zero() {
    let s1: SideOffsets2D<f32, ()> = SideOffsets2D::new_all_same(0.0);
    assert!(s1.is_zero());

    let s2: SideOffsets2D<f32, ()> = SideOffsets2D::new(1.0, 2.0, 3.0, 4.0);
    assert!(!s2.is_zero());
}

#[cfg(test)]
mod ops {
    use crate::Scale;

    pub enum Mm {}
    pub enum Cm {}

    type SideOffsets2D<T> = crate::default::SideOffsets2D<T>;
    type SideOffsets2DMm<T> = crate::SideOffsets2D<T, Mm>;
    type SideOffsets2DCm<T> = crate::SideOffsets2D<T, Cm>;

    #[test]
    fn test_mul_scalar() {
        let s = SideOffsets2D::new(1.0, 2.0, 3.0, 4.0);

        let result = s * 3.0;

        assert_eq!(result, SideOffsets2D::new(3.0, 6.0, 9.0, 12.0));
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut s = SideOffsets2D::new(1.0, 2.0, 3.0, 4.0);

        s *= 2.0;

        assert_eq!(s, SideOffsets2D::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn test_mul_scale() {
        let s = SideOffsets2DMm::new(0.0, 1.0, 3.0, 2.0);
        let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

        let result = s * cm_per_mm;

        assert_eq!(result, SideOffsets2DCm::new(0.0, 0.1, 0.3, 0.2));
    }

    #[test]
    fn test_mul_assign_scale() {
        let mut s = SideOffsets2DMm::new(2.0, 4.0, 6.0, 8.0);
        let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

        s *= scale;

        assert_eq!(s, SideOffsets2DMm::new(0.2, 0.4, 0.6, 0.8));
    }

    #[test]
    fn test_div_scalar() {
        let s = SideOffsets2D::new(10.0, 20.0, 30.0, 40.0);

        let result = s / 10.0;

        assert_eq!(result, SideOffsets2D::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut s = SideOffsets2D::new(10.0, 20.0, 30.0, 40.0);

        s /= 10.0;

        assert_eq!(s, SideOffsets2D::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_div_scale() {
        let s = SideOffsets2DCm::new(0.1, 0.2, 0.3, 0.4);
        let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

        let result = s / cm_per_mm;

        assert_eq!(result, SideOffsets2DMm::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_div_assign_scale() {
        let mut s = SideOffsets2DMm::new(0.1, 0.2, 0.3, 0.4);
        let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

        s /= scale;

        assert_eq!(s, SideOffsets2DMm::new(1.0, 2.0, 3.0, 4.0));
    }
}
