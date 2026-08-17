// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::point::{Point2D, Point3D};
use crate::vector::{Vector2D, Vector3D};

use crate::num::{One, Zero};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::Div;
#[cfg(feature = "serde")]
use serde;

/// Homogeneous vector in 3D space.
#[repr(C)]
pub struct HomogeneousVector<T, U> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    #[doc(hidden)]
    pub _unit: PhantomData<U>,
}

impl<T: Copy, U> Copy for HomogeneousVector<T, U> {}

impl<T: Clone, U> Clone for HomogeneousVector<T, U> {
    fn clone(&self) -> Self {
        HomogeneousVector {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
            w: self.w.clone(),
            _unit: PhantomData,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T, U> serde::Deserialize<'de> for HomogeneousVector<T, U>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y, z, w) = serde::Deserialize::deserialize(deserializer)?;
        Ok(HomogeneousVector {
            x,
            y,
            z,
            w,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
impl<T, U> serde::Serialize for HomogeneousVector<T, U>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.x, &self.y, &self.z, &self.w).serialize(serializer)
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T, U> arbitrary::Arbitrary<'a> for HomogeneousVector<T, U>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (x, y, z, w) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(HomogeneousVector {
            x,
            y,
            z,
            w,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, U> Zeroable for HomogeneousVector<T, U> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, U: 'static> Pod for HomogeneousVector<T, U> {}

impl<T, U> Eq for HomogeneousVector<T, U> where T: Eq {}

impl<T, U> PartialEq for HomogeneousVector<T, U>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl<T, U> Hash for HomogeneousVector<T, U>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
        self.z.hash(h);
        self.w.hash(h);
    }
}

impl<T, U> HomogeneousVector<T, U> {
    /// Constructor taking scalar values directly.
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        HomogeneousVector {
            x,
            y,
            z,
            w,
            _unit: PhantomData,
        }
    }
}

impl<T: Copy + Div<T, Output = T> + Zero + PartialOrd, U> HomogeneousVector<T, U> {
    /// Convert into Cartesian 2D point.
    ///
    /// Returns `None` if the point is on or behind the W=0 hemisphere.
    #[inline]
    pub fn to_point2d(self) -> Option<Point2D<T, U>> {
        if self.w > T::zero() {
            Some(Point2D::new(self.x / self.w, self.y / self.w))
        } else {
            None
        }
    }

    /// Convert into Cartesian 3D point.
    ///
    /// Returns `None` if the point is on or behind the W=0 hemisphere.
    #[inline]
    pub fn to_point3d(self) -> Option<Point3D<T, U>> {
        if self.w > T::zero() {
            Some(Point3D::new(
                self.x / self.w,
                self.y / self.w,
                self.z / self.w,
            ))
        } else {
            None
        }
    }
}

impl<T: Zero, U> From<Vector2D<T, U>> for HomogeneousVector<T, U> {
    #[inline]
    fn from(v: Vector2D<T, U>) -> Self {
        HomogeneousVector::new(v.x, v.y, T::zero(), T::zero())
    }
}

impl<T: Zero, U> From<Vector3D<T, U>> for HomogeneousVector<T, U> {
    #[inline]
    fn from(v: Vector3D<T, U>) -> Self {
        HomogeneousVector::new(v.x, v.y, v.z, T::zero())
    }
}

impl<T: Zero + One, U> From<Point2D<T, U>> for HomogeneousVector<T, U> {
    #[inline]
    fn from(p: Point2D<T, U>) -> Self {
        HomogeneousVector::new(p.x, p.y, T::zero(), T::one())
    }
}

impl<T: One, U> From<Point3D<T, U>> for HomogeneousVector<T, U> {
    #[inline]
    fn from(p: Point3D<T, U>) -> Self {
        HomogeneousVector::new(p.x, p.y, p.z, T::one())
    }
}

impl<T: fmt::Debug, U> fmt::Debug for HomogeneousVector<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

#[cfg(test)]
mod homogeneous {
    use super::HomogeneousVector;
    use crate::default::{Point2D, Point3D};

    #[test]
    fn roundtrip() {
        assert_eq!(
            Some(Point2D::new(1.0, 2.0)),
            HomogeneousVector::from(Point2D::new(1.0, 2.0)).to_point2d()
        );
        assert_eq!(
            Some(Point3D::new(1.0, -2.0, 0.1)),
            HomogeneousVector::from(Point3D::new(1.0, -2.0, 0.1)).to_point3d()
        );
    }

    #[test]
    fn negative() {
        assert_eq!(
            None,
            HomogeneousVector::<f32, ()>::new(1.0, 2.0, 3.0, 0.0).to_point2d()
        );
        assert_eq!(
            None,
            HomogeneousVector::<f32, ()>::new(1.0, -2.0, -3.0, -2.0).to_point3d()
        );
    }
}
