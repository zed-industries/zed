// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Points are fixed positions in affine space with no length or direction. This
//! distinguishes them from vectors, which have a length and direction, but do
//! not have a fixed position.

use num_traits::{Bounded, Float, NumCast};
use std::fmt;
use std::mem;
use std::ops::*;

use structure::*;

use approx;
use num::{BaseFloat, BaseNum};
use vector::{Vector1, Vector2, Vector3, Vector4};

#[cfg(feature = "mint")]
use mint;

/// A point in 1-dimensional space.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point1<S> {
    pub x: S,
}

/// A point in 2-dimensional space.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point2<S> {
    pub x: S,
    pub y: S,
}

/// A point in 3-dimensional space.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S: BaseNum> Point3<S> {
    #[inline]
    pub fn from_homogeneous(v: Vector4<S>) -> Point3<S> {
        let e = v.truncate() * (S::one() / v.w);
        Point3::new(e.x, e.y, e.z) //FIXME
    }

    #[inline]
    pub fn to_homogeneous(self) -> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, S::one())
    }
}

macro_rules! impl_point {
    ($PointN:ident { $($field:ident),+ }, $VectorN:ident, $n:expr, $constructor:ident) => {
        impl<S> $PointN<S> {
            /// Construct a new point, using the provided values.
            #[inline]
            pub const fn new($($field: S),+) -> $PointN<S> {
                $PointN { $($field: $field),+ }
            }

            /// Perform the given operation on each field in the point, returning a new point
            /// constructed from the operations.
            #[inline]
            pub fn map<U, F>(self, mut f: F) -> $PointN<U>
                where F: FnMut(S) -> U
            {
                $PointN { $($field: f(self.$field)),+ }
            }

            /// Construct a new point where each component is the result of
            /// applying the given operation to each pair of components of the
            /// given points.
            #[inline]
            pub fn zip<S2, S3, F>(self, p2: $PointN<S2>, mut f: F) -> $PointN<S3>
            where F: FnMut(S, S2) -> S3
            {
                $PointN { $($field: f(self.$field, p2.$field)),+ }
            }
        }

        /// The short constructor.
        #[inline]
        pub const fn $constructor<S>($($field: S),+) -> $PointN<S> {
            $PointN::new($($field),+)
        }

        impl<S: BaseNum> Array for $PointN<S> {
            type Element = S;

            #[inline]
            fn len() -> usize {
                $n
            }

            #[inline]
            fn from_value(scalar: S) -> $PointN<S> {
                $PointN { $($field: scalar),+ }
            }

            #[inline]
            fn sum(self) -> S where S: Add<Output = S> {
                fold_array!(add, { $(self.$field),+ })
            }

            #[inline]
            fn product(self) -> S where S: Mul<Output = S> {
                fold_array!(mul, { $(self.$field),+ })
            }

            fn is_finite(&self) -> bool where S: Float {
                $(self.$field.is_finite())&&+
            }
        }

        impl<S: NumCast + Copy> $PointN<S> {
            /// Component-wise casting to another type
            #[inline]
            pub fn cast<T: NumCast>(&self) -> Option<$PointN<T>> {
                $(
                    let $field = match NumCast::from(self.$field) {
                        Some(field) => field,
                        None => return None
                    };
                )+
                Some($PointN { $($field),+ })
            }
        }

        impl<S: BaseFloat> MetricSpace for $PointN<S> {
            type Metric = S;

            #[inline]
            fn distance2(self, other: Self) -> S {
                (other - self).magnitude2()
            }
        }

        impl<S: BaseNum> EuclideanSpace for $PointN<S> {
            type Scalar = S;
            type Diff = $VectorN<S>;

            #[inline]
            fn origin() -> $PointN<S> {
                $PointN { $($field: S::zero()),+ }
            }

            #[inline]
            fn from_vec(v: $VectorN<S>) -> $PointN<S> {
                $PointN::new($(v.$field),+)
            }

            #[inline]
            fn to_vec(self) -> $VectorN<S> {
                $VectorN::new($(self.$field),+)
            }

            #[inline]
            fn dot(self, v: $VectorN<S>) -> S {
                $VectorN::new($(self.$field * v.$field),+).sum()
            }
        }

        impl<S: BaseFloat> approx::AbsDiffEq for $PointN<S> {
            type Epsilon = S::Epsilon;

            #[inline]
            fn default_epsilon() -> S::Epsilon {
                S::default_epsilon()
            }

            #[inline]
            fn abs_diff_eq(&self, other: &Self, epsilon: S::Epsilon)
            -> bool
            {
                $(S::abs_diff_eq(&self.$field, &other.$field, epsilon))&&+
            }
        }

        impl<S: BaseFloat> approx::RelativeEq for $PointN<S> {
            #[inline]
            fn default_max_relative() -> S::Epsilon {
                S::default_max_relative()
            }

            #[inline]
            fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
                $(S::relative_eq(&self.$field, &other.$field, epsilon, max_relative))&&+
            }
        }

        impl<S: BaseFloat> approx::UlpsEq for $PointN<S> {
            #[inline]
            fn default_max_ulps() -> u32 {
                S::default_max_ulps()
            }

            #[inline]
            fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
                $(S::ulps_eq(&self.$field, &other.$field, epsilon, max_ulps))&&+
            }
        }

        impl<S: Bounded> Bounded for $PointN<S> {
            #[inline]
            fn min_value() -> $PointN<S> {
                $PointN { $($field: S::min_value()),+ }
            }

            #[inline]
            fn max_value() -> $PointN<S> {
                $PointN { $($field: S::max_value()),+ }
            }
        }

        impl_operator!(<S: BaseNum> Add<$VectorN<S> > for $PointN<S> {
            fn add(lhs, rhs) -> $PointN<S> { $PointN::new($(lhs.$field + rhs.$field),+) }
        });
        impl_operator!(<S: BaseNum> Sub<$VectorN<S>> for $PointN<S> {
            fn sub(lhs, rhs) -> $PointN<S> { $PointN::new($(lhs.$field - rhs.$field),+) }
        });
        impl_assignment_operator!(<S: BaseNum> AddAssign<$VectorN<S> > for $PointN<S> {
            fn add_assign(&mut self, vector) { $(self.$field += vector.$field);+ }
        });
        impl_assignment_operator!(<S: BaseNum> SubAssign<$VectorN<S>> for $PointN<S> {
            fn sub_assign(&mut self, vector) { $(self.$field -= vector.$field);+ }
        });

        impl_operator!(<S: BaseNum> Sub<$PointN<S> > for $PointN<S> {
            fn sub(lhs, rhs) -> $VectorN<S> { $VectorN::new($(lhs.$field - rhs.$field),+) }
        });

        impl_operator!(<S: BaseNum> Mul<S> for $PointN<S> {
            fn mul(point, scalar) -> $PointN<S> { $PointN::new($(point.$field * scalar),+) }
        });
        impl_operator!(<S: BaseNum> Div<S> for $PointN<S> {
            fn div(point, scalar) -> $PointN<S> { $PointN::new($(point.$field / scalar),+) }
        });
        impl_operator!(<S: BaseNum> Rem<S> for $PointN<S> {
            fn rem(point, scalar) -> $PointN<S> { $PointN::new($(point.$field % scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> MulAssign<S> for $PointN<S> {
            fn mul_assign(&mut self, scalar) { $(self.$field *= scalar);+ }
        });
        impl_assignment_operator!(<S: BaseNum> DivAssign<S> for $PointN<S> {
            fn div_assign(&mut self, scalar) { $(self.$field /= scalar);+ }
        });
        impl_assignment_operator!(<S: BaseNum> RemAssign<S> for $PointN<S> {
            fn rem_assign(&mut self, scalar) { $(self.$field %= scalar);+ }
        });

        impl<S: BaseNum> ElementWise for $PointN<S> {
            #[inline] fn add_element_wise(self, rhs: $PointN<S>) -> $PointN<S> { $PointN::new($(self.$field + rhs.$field),+) }
            #[inline] fn sub_element_wise(self, rhs: $PointN<S>) -> $PointN<S> { $PointN::new($(self.$field - rhs.$field),+) }
            #[inline] fn mul_element_wise(self, rhs: $PointN<S>) -> $PointN<S> { $PointN::new($(self.$field * rhs.$field),+) }
            #[inline] fn div_element_wise(self, rhs: $PointN<S>) -> $PointN<S> { $PointN::new($(self.$field / rhs.$field),+) }
            #[inline] fn rem_element_wise(self, rhs: $PointN<S>) -> $PointN<S> { $PointN::new($(self.$field % rhs.$field),+) }

            #[inline] fn add_assign_element_wise(&mut self, rhs: $PointN<S>) { $(self.$field += rhs.$field);+ }
            #[inline] fn sub_assign_element_wise(&mut self, rhs: $PointN<S>) { $(self.$field -= rhs.$field);+ }
            #[inline] fn mul_assign_element_wise(&mut self, rhs: $PointN<S>) { $(self.$field *= rhs.$field);+ }
            #[inline] fn div_assign_element_wise(&mut self, rhs: $PointN<S>) { $(self.$field /= rhs.$field);+ }
            #[inline] fn rem_assign_element_wise(&mut self, rhs: $PointN<S>) { $(self.$field %= rhs.$field);+ }
        }

        impl<S: BaseNum> ElementWise<S> for $PointN<S> {
            #[inline] fn add_element_wise(self, rhs: S) -> $PointN<S> { $PointN::new($(self.$field + rhs),+) }
            #[inline] fn sub_element_wise(self, rhs: S) -> $PointN<S> { $PointN::new($(self.$field - rhs),+) }
            #[inline] fn mul_element_wise(self, rhs: S) -> $PointN<S> { $PointN::new($(self.$field * rhs),+) }
            #[inline] fn div_element_wise(self, rhs: S) -> $PointN<S> { $PointN::new($(self.$field / rhs),+) }
            #[inline] fn rem_element_wise(self, rhs: S) -> $PointN<S> { $PointN::new($(self.$field % rhs),+) }

            #[inline] fn add_assign_element_wise(&mut self, rhs: S) { $(self.$field += rhs);+ }
            #[inline] fn sub_assign_element_wise(&mut self, rhs: S) { $(self.$field -= rhs);+ }
            #[inline] fn mul_assign_element_wise(&mut self, rhs: S) { $(self.$field *= rhs);+ }
            #[inline] fn div_assign_element_wise(&mut self, rhs: S) { $(self.$field /= rhs);+ }
            #[inline] fn rem_assign_element_wise(&mut self, rhs: S) { $(self.$field %= rhs);+ }
        }

        impl_scalar_ops!($PointN<usize> { $($field),+ });
        impl_scalar_ops!($PointN<u8> { $($field),+ });
        impl_scalar_ops!($PointN<u16> { $($field),+ });
        impl_scalar_ops!($PointN<u32> { $($field),+ });
        impl_scalar_ops!($PointN<u64> { $($field),+ });
        impl_scalar_ops!($PointN<isize> { $($field),+ });
        impl_scalar_ops!($PointN<i8> { $($field),+ });
        impl_scalar_ops!($PointN<i16> { $($field),+ });
        impl_scalar_ops!($PointN<i32> { $($field),+ });
        impl_scalar_ops!($PointN<i64> { $($field),+ });
        impl_scalar_ops!($PointN<f32> { $($field),+ });
        impl_scalar_ops!($PointN<f64> { $($field),+ });

        impl_index_operators!($PointN<S>, $n, S, usize);
        impl_index_operators!($PointN<S>, $n, [S], Range<usize>);
        impl_index_operators!($PointN<S>, $n, [S], RangeTo<usize>);
        impl_index_operators!($PointN<S>, $n, [S], RangeFrom<usize>);
        impl_index_operators!($PointN<S>, $n, [S], RangeFull);
    }
}

macro_rules! impl_scalar_ops {
    ($PointN:ident<$S:ident> { $($field:ident),+ }) => {
        impl_operator!(Mul<$PointN<$S>> for $S {
            fn mul(scalar, point) -> $PointN<$S> { $PointN::new($(scalar * point.$field),+) }
        });
        impl_operator!(Div<$PointN<$S>> for $S {
            fn div(scalar, point) -> $PointN<$S> { $PointN::new($(scalar / point.$field),+) }
        });
        impl_operator!(Rem<$PointN<$S>> for $S {
            fn rem(scalar, point) -> $PointN<$S> { $PointN::new($(scalar % point.$field),+) }
        });
    };
}

impl_point!(Point1 { x }, Vector1, 1, point1);
impl_point!(Point2 { x, y }, Vector2, 2, point2);
impl_point!(Point3 { x, y, z }, Vector3, 3, point3);

impl<S: Copy> Point1<S> {
    impl_swizzle_functions!(Point1, Point2, Point3, S, x);
}

impl<S: Copy> Point2<S> {
    impl_swizzle_functions!(Point1, Point2, Point3, S, xy);
}

impl<S: Copy> Point3<S> {
    impl_swizzle_functions!(Point1, Point2, Point3, S, xyz);
}

impl_fixed_array_conversions!(Point1<S> { x: 0 }, 1);
impl_fixed_array_conversions!(Point2<S> { x: 0, y: 1 }, 2);
impl_fixed_array_conversions!(Point3<S> { x: 0, y: 1, z: 2 }, 3);

impl_tuple_conversions!(Point1<S> { x }, (S,));
impl_tuple_conversions!(Point2<S> { x, y }, (S, S));
impl_tuple_conversions!(Point3<S> { x, y, z }, (S, S, S));

#[cfg(feature = "mint")]
impl_mint_conversions!(Point2 { x, y }, Point2);
#[cfg(feature = "mint")]
impl_mint_conversions!(Point3 { x, y, z }, Point3);

impl<S: fmt::Debug> fmt::Debug for Point1<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point1 ")?;
        <[S; 1] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Point2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point2 ")?;
        <[S; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Point3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point3 ")?;
        <[S; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    mod point2 {
        use point::*;

        const POINT2: Point2<i32> = Point2 { x: 1, y: 2 };

        #[test]
        fn test_index() {
            assert_eq!(POINT2[0], POINT2.x);
            assert_eq!(POINT2[1], POINT2.y);
        }

        #[test]
        fn test_index_mut() {
            let mut p = POINT2;
            *&mut p[0] = 0;
            assert_eq!(p, [0, 2].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            POINT2[2];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&POINT2[..0], &[]);
            assert_eq!(&POINT2[..1], &[1]);
            assert_eq!(POINT2[..0].len(), 0);
            assert_eq!(POINT2[..1].len(), 1);
            assert_eq!(&POINT2[2..], &[]);
            assert_eq!(&POINT2[1..], &[2]);
            assert_eq!(POINT2[2..].len(), 0);
            assert_eq!(POINT2[1..].len(), 1);
            assert_eq!(&POINT2[..], &[1, 2]);
            assert_eq!(POINT2[..].len(), 2);
        }

        #[test]
        fn test_into() {
            let p = POINT2;
            {
                let p: [i32; 2] = p.into();
                assert_eq!(p, [1, 2]);
            }
            {
                let p: (i32, i32) = p.into();
                assert_eq!(p, (1, 2));
            }
        }

        #[test]
        fn test_as_ref() {
            let p = POINT2;
            {
                let p: &[i32; 2] = p.as_ref();
                assert_eq!(p, &[1, 2]);
            }
            {
                let p: &(i32, i32) = p.as_ref();
                assert_eq!(p, &(1, 2));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut p = POINT2;
            {
                let p: &mut [i32; 2] = p.as_mut();
                assert_eq!(p, &mut [1, 2]);
            }
            {
                let p: &mut (i32, i32) = p.as_mut();
                assert_eq!(p, &mut (1, 2));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Point2::from([1, 2]), POINT2);
            {
                let p = &[1, 2];
                let p: &Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
            {
                let p = &mut [1, 2];
                let p: &mut Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
            assert_eq!(Point2::from((1, 2)), POINT2);
            {
                let p = &(1, 2);
                let p: &Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
            {
                let p = &mut (1, 2);
                let p: &mut Point2<_> = From::from(p);
                assert_eq!(p, &POINT2);
            }
        }

        #[test]
        fn test_zip() {
            assert_eq!(
                Point2::new(true, false),
                Point2::new(-2, 1).zip(Point2::new(-1, -1), |a, b| a < b)
            );
        }
    }

    mod point3 {
        use point::*;

        const POINT3: Point3<i32> = Point3 { x: 1, y: 2, z: 3 };

        #[test]
        fn test_index() {
            assert_eq!(POINT3[0], POINT3.x);
            assert_eq!(POINT3[1], POINT3.y);
            assert_eq!(POINT3[2], POINT3.z);
        }

        #[test]
        fn test_index_mut() {
            let mut p = POINT3;
            *&mut p[1] = 0;
            assert_eq!(p, [1, 0, 3].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            POINT3[3];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&POINT3[..1], &[1]);
            assert_eq!(&POINT3[..2], &[1, 2]);
            assert_eq!(POINT3[..1].len(), 1);
            assert_eq!(POINT3[..2].len(), 2);
            assert_eq!(&POINT3[2..], &[3]);
            assert_eq!(&POINT3[1..], &[2, 3]);
            assert_eq!(POINT3[2..].len(), 1);
            assert_eq!(POINT3[1..].len(), 2);
            assert_eq!(&POINT3[..], &[1, 2, 3]);
            assert_eq!(POINT3[..].len(), 3);
        }

        #[test]
        fn test_into() {
            let p = POINT3;
            {
                let p: [i32; 3] = p.into();
                assert_eq!(p, [1, 2, 3]);
            }
            {
                let p: (i32, i32, i32) = p.into();
                assert_eq!(p, (1, 2, 3));
            }
        }

        #[test]
        fn test_as_ref() {
            let p = POINT3;
            {
                let p: &[i32; 3] = p.as_ref();
                assert_eq!(p, &[1, 2, 3]);
            }
            {
                let p: &(i32, i32, i32) = p.as_ref();
                assert_eq!(p, &(1, 2, 3));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut p = POINT3;
            {
                let p: &mut [i32; 3] = p.as_mut();
                assert_eq!(p, &mut [1, 2, 3]);
            }
            {
                let p: &mut (i32, i32, i32) = p.as_mut();
                assert_eq!(p, &mut (1, 2, 3));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Point3::from([1, 2, 3]), POINT3);
            {
                let p = &[1, 2, 3];
                let p: &Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
            {
                let p = &mut [1, 2, 3];
                let p: &mut Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
            assert_eq!(Point3::from((1, 2, 3)), POINT3);
            {
                let p = &(1, 2, 3);
                let p: &Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
            {
                let p = &mut (1, 2, 3);
                let p: &mut Point3<_> = From::from(p);
                assert_eq!(p, &POINT3);
            }
        }

        #[test]
        fn test_zip() {
            assert_eq!(
                Point3::new(true, false, false),
                Point3::new(-2, 1, 0).zip(Point3::new(-1, -1, -1), |a, b| a < b)
            );
        }
    }
}
