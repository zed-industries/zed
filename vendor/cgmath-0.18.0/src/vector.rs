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

use num_traits::{Bounded, Float, NumCast};
#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;
use std::iter;
use std::mem;
use std::ops::*;

use structure::*;

use angle::Rad;
use approx;
use num::{BaseFloat, BaseNum};

#[cfg(feature = "mint")]
use mint;

/// A 1-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector1<S> {
    /// The x component of the vector.
    pub x: S,
}

/// A 2-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector2<S> {
    /// The x component of the vector.
    pub x: S,
    /// The y component of the vector.
    pub y: S,
}

/// A 3-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector3<S> {
    /// The x component of the vector.
    pub x: S,
    /// The y component of the vector.
    pub y: S,
    /// The z component of the vector.
    pub z: S,
}

/// A 4-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector4<S> {
    /// The x component of the vector.
    pub x: S,
    /// The y component of the vector.
    pub y: S,
    /// The z component of the vector.
    pub z: S,
    /// The w component of the vector.
    pub w: S,
}

// Utility macro for generating associated functions for the vectors
macro_rules! impl_vector {
    ($VectorN:ident { $($field:ident),+ }, $n:expr, $constructor:ident) => {
        impl<S> $VectorN<S> {
            /// Construct a new vector, using the provided values.
            #[inline]
            pub const fn new($($field: S),+) -> $VectorN<S> {
                $VectorN { $($field: $field),+ }
            }

            /// Perform the given operation on each field in the vector, returning a new point
            /// constructed from the operations.
            #[inline]
            pub fn map<U, F>(self, mut f: F) -> $VectorN<U>
                where F: FnMut(S) -> U
            {
                $VectorN { $($field: f(self.$field)),+ }
            }

            /// Construct a new vector where each component is the result of
            /// applying the given operation to each pair of components of the
            /// given vectors.
            #[inline]
            pub fn zip<S2, S3, F>(self, v2: $VectorN<S2>, mut f: F) -> $VectorN<S3>
                where F: FnMut(S, S2) -> S3
            {
                $VectorN { $($field: f(self.$field, v2.$field)),+ }
            }
        }

        /// The short constructor.
        #[inline]
        pub const fn $constructor<S>($($field: S),+) -> $VectorN<S> {
            $VectorN::new($($field),+)
        }

        impl<S: NumCast + Copy> $VectorN<S> {
            /// Component-wise casting to another type.
            #[inline]
            pub fn cast<T: NumCast>(&self) -> Option<$VectorN<T>> {
                $(
                    let $field = match NumCast::from(self.$field) {
                        Some(field) => field,
                        None => return None
                    };
                )+
                Some($VectorN { $($field),+ })
            }
        }

        impl<S: BaseNum> MetricSpace for $VectorN<S> {
            type Metric = S;

            #[inline]
            fn distance2(self, other: Self) -> S {
                (other - self).magnitude2()
            }
        }

        impl<S: Copy> Array for $VectorN<S> {
            type Element = S;

            #[inline]
            fn len() -> usize {
                $n
            }

            #[inline]
            fn from_value(scalar: S) -> $VectorN<S> {
                $VectorN { $($field: scalar),+ }
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

        impl<S: BaseNum> Zero for $VectorN<S> {
            #[inline]
            fn zero() -> $VectorN<S> {
                $VectorN::from_value(S::zero())
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == $VectorN::zero()
            }
        }

        impl<S: BaseNum> iter::Sum<$VectorN<S>> for $VectorN<S> {
            #[inline]
            fn sum<I: Iterator<Item=$VectorN<S>>>(iter: I) -> $VectorN<S> {
                iter.fold($VectorN::zero(), Add::add)
            }
        }

        impl<'a, S: 'a + BaseNum> iter::Sum<&'a $VectorN<S>> for $VectorN<S> {
            #[inline]
            fn sum<I: Iterator<Item=&'a $VectorN<S>>>(iter: I) -> $VectorN<S> {
                iter.fold($VectorN::zero(), Add::add)
            }
        }

        impl<S: BaseNum> VectorSpace for $VectorN<S> {
            type Scalar = S;
        }

        impl<S: Neg<Output = S>> Neg for $VectorN<S> {
            type Output = $VectorN<S>;

            #[inline]
            default_fn!( neg(self) -> $VectorN<S> { $VectorN::new($(-self.$field),+) } );
        }

        impl<S: BaseFloat> approx::AbsDiffEq for $VectorN<S> {
            type Epsilon = S::Epsilon;

            #[inline]
            fn default_epsilon() -> S::Epsilon {
                S::default_epsilon()
            }

            #[inline]
            fn abs_diff_eq(&self, other: &Self, epsilon: S::Epsilon) -> bool {
                $(S::abs_diff_eq(&self.$field, &other.$field, epsilon))&&+
            }
        }

        impl<S: BaseFloat> approx::RelativeEq for $VectorN<S> {
            #[inline]
            fn default_max_relative() -> S::Epsilon {
                S::default_max_relative()
            }

            #[inline]
            fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
                $(S::relative_eq(&self.$field, &other.$field, epsilon, max_relative))&&+
            }
        }

        impl<S: BaseFloat> approx::UlpsEq for $VectorN<S> {
            #[inline]
            fn default_max_ulps() -> u32 {
                S::default_max_ulps()
            }

            #[inline]
            fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
                $(S::ulps_eq(&self.$field, &other.$field, epsilon, max_ulps))&&+
            }
        }

        #[cfg(feature = "rand")]
        impl<S> Distribution<$VectorN<S>> for Standard
            where Standard: Distribution<S>,
                S: BaseFloat {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $VectorN<S> {
                $VectorN { $($field: rng.gen()),+ }
            }
        }

        impl<S: Bounded> Bounded for $VectorN<S> {
            #[inline]
            fn min_value() -> $VectorN<S> {
                $VectorN { $($field: S::min_value()),+ }
            }

            #[inline]
            fn max_value() -> $VectorN<S> {
                $VectorN { $($field: S::max_value()),+ }
            }
        }

        impl_operator!(<S: BaseNum> Add<$VectorN<S> > for $VectorN<S> {
            fn add(lhs, rhs) -> $VectorN<S> { $VectorN::new($(lhs.$field + rhs.$field),+) }
        });
        impl_assignment_operator!(<S: BaseNum> AddAssign<$VectorN<S> > for $VectorN<S> {
            fn add_assign(&mut self, other) { $(self.$field += other.$field);+ }
        });

        impl_operator!(<S: BaseNum> Sub<$VectorN<S> > for $VectorN<S> {
            fn sub(lhs, rhs) -> $VectorN<S> { $VectorN::new($(lhs.$field - rhs.$field),+) }
        });
        impl_assignment_operator!(<S: BaseNum> SubAssign<$VectorN<S> > for $VectorN<S> {
            fn sub_assign(&mut self, other) { $(self.$field -= other.$field);+ }
        });

        impl_operator!(<S: BaseNum> Mul<S> for $VectorN<S> {
            fn mul(vector, scalar) -> $VectorN<S> { $VectorN::new($(vector.$field * scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> MulAssign<S> for $VectorN<S> {
            fn mul_assign(&mut self, scalar) { $(self.$field *= scalar);+ }
        });

        impl_operator!(<S: BaseNum> Div<S> for $VectorN<S> {
            fn div(vector, scalar) -> $VectorN<S> { $VectorN::new($(vector.$field / scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> DivAssign<S> for $VectorN<S> {
            fn div_assign(&mut self, scalar) { $(self.$field /= scalar);+ }
        });

        impl_operator!(<S: BaseNum> Rem<S> for $VectorN<S> {
            fn rem(vector, scalar) -> $VectorN<S> { $VectorN::new($(vector.$field % scalar),+) }
        });
        impl_assignment_operator!(<S: BaseNum> RemAssign<S> for $VectorN<S> {
            fn rem_assign(&mut self, scalar) { $(self.$field %= scalar);+ }
        });

        impl<S: BaseNum> ElementWise for $VectorN<S> {
            #[inline] default_fn!( add_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field + rhs.$field),+) } );
            #[inline] default_fn!( sub_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field - rhs.$field),+) } );
            #[inline] default_fn!( mul_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field * rhs.$field),+) } );
            #[inline] default_fn!( div_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field / rhs.$field),+) } );
            #[inline] fn rem_element_wise(self, rhs: $VectorN<S>) -> $VectorN<S> { $VectorN::new($(self.$field % rhs.$field),+) }

            #[inline] default_fn!( add_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field += rhs.$field);+ } );
            #[inline] default_fn!( sub_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field -= rhs.$field);+ } );
            #[inline] default_fn!( mul_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field *= rhs.$field);+ } );
            #[inline] default_fn!( div_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field /= rhs.$field);+ } );
            #[inline] fn rem_assign_element_wise(&mut self, rhs: $VectorN<S>) { $(self.$field %= rhs.$field);+ }
        }

        impl<S: BaseNum> ElementWise<S> for $VectorN<S> {
            #[inline] default_fn!( add_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field + rhs),+) } );
            #[inline] default_fn!( sub_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field - rhs),+) } );
            #[inline] default_fn!( mul_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field * rhs),+) } );
            #[inline] default_fn!( div_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field / rhs),+) } );
            #[inline] fn rem_element_wise(self, rhs: S) -> $VectorN<S> { $VectorN::new($(self.$field % rhs),+) }

            #[inline] default_fn!( add_assign_element_wise(&mut self, rhs: S) { $(self.$field += rhs);+ } );
            #[inline] default_fn!( sub_assign_element_wise(&mut self, rhs: S) { $(self.$field -= rhs);+ } );
            #[inline] default_fn!( mul_assign_element_wise(&mut self, rhs: S) { $(self.$field *= rhs);+ } );
            #[inline] default_fn!( div_assign_element_wise(&mut self, rhs: S) { $(self.$field /= rhs);+ } );
            #[inline] fn rem_assign_element_wise(&mut self, rhs: S) { $(self.$field %= rhs);+ }
        }

        impl_scalar_ops!($VectorN<usize> { $($field),+ });
        impl_scalar_ops!($VectorN<u8> { $($field),+ });
        impl_scalar_ops!($VectorN<u16> { $($field),+ });
        impl_scalar_ops!($VectorN<u32> { $($field),+ });
        impl_scalar_ops!($VectorN<u64> { $($field),+ });
        impl_scalar_ops!($VectorN<isize> { $($field),+ });
        impl_scalar_ops!($VectorN<i8> { $($field),+ });
        impl_scalar_ops!($VectorN<i16> { $($field),+ });
        impl_scalar_ops!($VectorN<i32> { $($field),+ });
        impl_scalar_ops!($VectorN<i64> { $($field),+ });
        impl_scalar_ops!($VectorN<f32> { $($field),+ });
        impl_scalar_ops!($VectorN<f64> { $($field),+ });

        impl_index_operators!($VectorN<S>, $n, S, usize);
        impl_index_operators!($VectorN<S>, $n, [S], Range<usize>);
        impl_index_operators!($VectorN<S>, $n, [S], RangeTo<usize>);
        impl_index_operators!($VectorN<S>, $n, [S], RangeFrom<usize>);
        impl_index_operators!($VectorN<S>, $n, [S], RangeFull);
    }
}

macro_rules! impl_scalar_ops {
    ($VectorN:ident<$S:ident> { $($field:ident),+ }) => {
        impl_operator!(Mul<$VectorN<$S>> for $S {
            fn mul(scalar, vector) -> $VectorN<$S> { $VectorN::new($(scalar * vector.$field),+) }
        });
        impl_operator!(Div<$VectorN<$S>> for $S {
            fn div(scalar, vector) -> $VectorN<$S> { $VectorN::new($(scalar / vector.$field),+) }
        });
        impl_operator!(Rem<$VectorN<$S>> for $S {
            fn rem(scalar, vector) -> $VectorN<$S> { $VectorN::new($(scalar % vector.$field),+) }
        });
    };
}

impl_vector!(Vector1 { x }, 1, vec1);
impl_vector!(Vector2 { x, y }, 2, vec2);
impl_vector!(Vector3 { x, y, z }, 3, vec3);
impl_vector!(Vector4 { x, y, z, w }, 4, vec4);

impl_fixed_array_conversions!(Vector1<S> { x: 0 }, 1);
impl_fixed_array_conversions!(Vector2<S> { x: 0, y: 1 }, 2);
impl_fixed_array_conversions!(Vector3<S> { x: 0, y: 1, z: 2 }, 3);
impl_fixed_array_conversions!(Vector4<S> { x: 0, y: 1, z: 2, w: 3 }, 4);

impl_tuple_conversions!(Vector1<S> { x }, (S,));
impl_tuple_conversions!(Vector2<S> { x, y }, (S, S));
impl_tuple_conversions!(Vector3<S> { x, y, z }, (S, S, S));
impl_tuple_conversions!(Vector4<S> { x, y, z, w }, (S, S, S, S));

impl<S: BaseNum> Vector1<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector1<S> {
        Vector1::new(S::one())
    }

    impl_swizzle_functions!(Vector1, Vector2, Vector3, Vector4, S, x);
}

impl<S: BaseNum> Vector2<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector2<S> {
        Vector2::new(S::one(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Vector2<S> {
        Vector2::new(S::zero(), S::one())
    }

    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(self, other: Vector2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }

    /// Create a `Vector3`, using the `x` and `y` values from this vector, and the
    /// provided `z`.
    #[inline]
    pub fn extend(self, z: S) -> Vector3<S> {
        Vector3::new(self.x, self.y, z)
    }

    impl_swizzle_functions!(Vector1, Vector2, Vector3, Vector4, S, xy);
}

impl<S: BaseNum> Vector3<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector3<S> {
        Vector3::new(S::one(), S::zero(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Vector3<S> {
        Vector3::new(S::zero(), S::one(), S::zero())
    }

    /// A unit vector in the `z` direction.
    #[inline]
    pub fn unit_z() -> Vector3<S> {
        Vector3::new(S::zero(), S::zero(), S::one())
    }

    /// Returns the cross product of the vector and `other`.
    #[inline]
    pub fn cross(self, other: Vector3<S>) -> Vector3<S> {
        Vector3::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }

    /// Create a `Vector4`, using the `x`, `y` and `z` values from this vector, and the
    /// provided `w`.
    #[inline]
    pub fn extend(self, w: S) -> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, w)
    }

    /// Create a `Vector2`, dropping the `z` value.
    #[inline]
    pub fn truncate(self) -> Vector2<S> {
        Vector2::new(self.x, self.y)
    }

    impl_swizzle_functions!(Vector1, Vector2, Vector3, Vector4, S, xyz);
}

impl<S: BaseNum> Vector4<S> {
    /// A unit vector in the `x` direction.
    #[inline]
    pub fn unit_x() -> Vector4<S> {
        Vector4::new(S::one(), S::zero(), S::zero(), S::zero())
    }

    /// A unit vector in the `y` direction.
    #[inline]
    pub fn unit_y() -> Vector4<S> {
        Vector4::new(S::zero(), S::one(), S::zero(), S::zero())
    }

    /// A unit vector in the `z` direction.
    #[inline]
    pub fn unit_z() -> Vector4<S> {
        Vector4::new(S::zero(), S::zero(), S::one(), S::zero())
    }

    /// A unit vector in the `w` direction.
    #[inline]
    pub fn unit_w() -> Vector4<S> {
        Vector4::new(S::zero(), S::zero(), S::zero(), S::one())
    }

    /// Create a `Vector3`, dropping the `w` value.
    #[inline]
    pub fn truncate(self) -> Vector3<S> {
        Vector3::new(self.x, self.y, self.z)
    }

    /// Create a `Vector3`, dropping the nth element.
    #[inline]
    pub fn truncate_n(&self, n: isize) -> Vector3<S> {
        match n {
            0 => Vector3::new(self.y, self.z, self.w),
            1 => Vector3::new(self.x, self.z, self.w),
            2 => Vector3::new(self.x, self.y, self.w),
            3 => Vector3::new(self.x, self.y, self.z),
            _ => panic!("{:?} is out of range", n),
        }
    }

    impl_swizzle_functions!(Vector1, Vector2, Vector3, Vector4, S, xyzw);
}

/// Dot product of two vectors.
#[inline]
pub fn dot<V: InnerSpace>(a: V, b: V) -> V::Scalar
where
    V::Scalar: BaseFloat,
{
    V::dot(a, b)
}

impl<S: BaseNum> InnerSpace for Vector1<S> {
    #[inline]
    fn dot(self, other: Vector1<S>) -> S {
        Vector1::mul_element_wise(self, other).sum()
    }
}

impl<S: BaseNum> InnerSpace for Vector2<S> {
    #[inline]
    fn dot(self, other: Vector2<S>) -> S {
        Vector2::mul_element_wise(self, other).sum()
    }

    #[inline]
    fn angle(self, other: Vector2<S>) -> Rad<S>
    where
        S: BaseFloat,
    {
        Rad::atan2(Self::perp_dot(self, other), Self::dot(self, other))
    }
}

impl<S: BaseNum> InnerSpace for Vector3<S> {
    #[inline]
    fn dot(self, other: Vector3<S>) -> S {
        Vector3::mul_element_wise(self, other).sum()
    }

    #[inline]
    fn angle(self, other: Vector3<S>) -> Rad<S>
    where
        S: BaseFloat,
    {
        Rad::atan2(self.cross(other).magnitude(), Self::dot(self, other))
    }
}

impl<S: BaseNum> InnerSpace for Vector4<S> {
    #[inline]
    fn dot(self, other: Vector4<S>) -> S {
        Vector4::mul_element_wise(self, other).sum()
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector1<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector1 ")?;
        <[S; 1] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector2 ")?;
        <[S; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector3 ")?;
        <[S; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Vector4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector4 ")?;
        <[S; 4] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

#[cfg(feature = "mint")]
impl_mint_conversions!(Vector2 { x, y }, Vector2);
#[cfg(feature = "mint")]
impl_mint_conversions!(Vector3 { x, y, z }, Vector3);
#[cfg(feature = "mint")]
impl_mint_conversions!(Vector4 { x, y, z, w }, Vector4);

#[cfg(test)]
mod tests {
    mod vector2 {
        use vector::*;

        const VECTOR2: Vector2<i32> = Vector2 { x: 1, y: 2 };

        #[test]
        fn test_index() {
            assert_eq!(VECTOR2[0], VECTOR2.x);
            assert_eq!(VECTOR2[1], VECTOR2.y);
        }

        #[test]
        fn test_index_mut() {
            let mut v = VECTOR2;
            *&mut v[0] = 0;
            assert_eq!(v, [0, 2].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            VECTOR2[2];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&VECTOR2[..0], &[]);
            assert_eq!(&VECTOR2[..1], &[1]);
            assert_eq!(VECTOR2[..0].len(), 0);
            assert_eq!(VECTOR2[..1].len(), 1);
            assert_eq!(&VECTOR2[2..], &[]);
            assert_eq!(&VECTOR2[1..], &[2]);
            assert_eq!(VECTOR2[2..].len(), 0);
            assert_eq!(VECTOR2[1..].len(), 1);
            assert_eq!(&VECTOR2[..], &[1, 2]);
            assert_eq!(VECTOR2[..].len(), 2);
        }

        #[test]
        fn test_into() {
            let v = VECTOR2;
            {
                let v: [i32; 2] = v.into();
                assert_eq!(v, [1, 2]);
            }
            {
                let v: (i32, i32) = v.into();
                assert_eq!(v, (1, 2));
            }
        }

        #[test]
        fn test_as_ref() {
            let v = VECTOR2;
            {
                let v: &[i32; 2] = v.as_ref();
                assert_eq!(v, &[1, 2]);
            }
            {
                let v: &(i32, i32) = v.as_ref();
                assert_eq!(v, &(1, 2));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut v = VECTOR2;
            {
                let v: &mut [i32; 2] = v.as_mut();
                assert_eq!(v, &mut [1, 2]);
            }
            {
                let v: &mut (i32, i32) = v.as_mut();
                assert_eq!(v, &mut (1, 2));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Vector2::from([1, 2]), VECTOR2);
            {
                let v = &[1, 2];
                let v: &Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
            {
                let v = &mut [1, 2];
                let v: &mut Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
            assert_eq!(Vector2::from((1, 2)), VECTOR2);
            {
                let v = &(1, 2);
                let v: &Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
            {
                let v = &mut (1, 2);
                let v: &mut Vector2<_> = From::from(v);
                assert_eq!(v, &VECTOR2);
            }
        }

        #[test]
        fn test_is_finite() {
            use num_traits::Float;
            assert!(!Vector2::from([Float::nan(), 1.0]).is_finite());
            assert!(!Vector2::from([1.0, Float::infinity()]).is_finite());
            assert!(Vector2::from([-1.0, 1.0]).is_finite());
        }

        #[test]
        fn test_zip() {
            assert_eq!(
                Vector2::new(true, false),
                Vector2::new(-2, 1).zip(Vector2::new(-1, -1), |a, b| a < b)
            );
        }
    }

    mod vector3 {
        use vector::*;

        const VECTOR3: Vector3<i32> = Vector3 { x: 1, y: 2, z: 3 };

        #[test]
        fn test_index() {
            assert_eq!(VECTOR3[0], VECTOR3.x);
            assert_eq!(VECTOR3[1], VECTOR3.y);
            assert_eq!(VECTOR3[2], VECTOR3.z);
        }

        #[test]
        fn test_index_mut() {
            let mut v = VECTOR3;
            *&mut v[1] = 0;
            assert_eq!(v, [1, 0, 3].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            VECTOR3[3];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&VECTOR3[..1], &[1]);
            assert_eq!(&VECTOR3[..2], &[1, 2]);
            assert_eq!(VECTOR3[..1].len(), 1);
            assert_eq!(VECTOR3[..2].len(), 2);
            assert_eq!(&VECTOR3[2..], &[3]);
            assert_eq!(&VECTOR3[1..], &[2, 3]);
            assert_eq!(VECTOR3[2..].len(), 1);
            assert_eq!(VECTOR3[1..].len(), 2);
            assert_eq!(&VECTOR3[..], &[1, 2, 3]);
            assert_eq!(VECTOR3[..].len(), 3);
        }

        #[test]
        fn test_into() {
            let v = VECTOR3;
            {
                let v: [i32; 3] = v.into();
                assert_eq!(v, [1, 2, 3]);
            }
            {
                let v: (i32, i32, i32) = v.into();
                assert_eq!(v, (1, 2, 3));
            }
        }

        #[test]
        fn test_as_ref() {
            let v = VECTOR3;
            {
                let v: &[i32; 3] = v.as_ref();
                assert_eq!(v, &[1, 2, 3]);
            }
            {
                let v: &(i32, i32, i32) = v.as_ref();
                assert_eq!(v, &(1, 2, 3));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut v = VECTOR3;
            {
                let v: &mut [i32; 3] = v.as_mut();
                assert_eq!(v, &mut [1, 2, 3]);
            }
            {
                let v: &mut (i32, i32, i32) = v.as_mut();
                assert_eq!(v, &mut (1, 2, 3));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Vector3::from([1, 2, 3]), VECTOR3);
            {
                let v = &[1, 2, 3];
                let v: &Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
            {
                let v = &mut [1, 2, 3];
                let v: &mut Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
            assert_eq!(Vector3::from((1, 2, 3)), VECTOR3);
            {
                let v = &(1, 2, 3);
                let v: &Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
            {
                let v = &mut (1, 2, 3);
                let v: &mut Vector3<_> = From::from(v);
                assert_eq!(v, &VECTOR3);
            }
        }

        #[test]
        fn test_is_finite() {
            use num_traits::Float;
            assert!(!Vector3::from([Float::nan(), 1.0, 1.0]).is_finite());
            assert!(!Vector3::from([1.0, 1.0, Float::infinity()]).is_finite());
            assert!(Vector3::from([-1.0, 1.0, 1.0]).is_finite());
        }

        #[test]
        fn test_zip() {
            assert_eq!(
                Vector3::new(true, false, false),
                Vector3::new(-2, 1, 0).zip(Vector3::new(-1, -1, -1), |a, b| a < b)
            );
        }
    }

    mod vector4 {
        use vector::*;

        const VECTOR4: Vector4<i32> = Vector4 {
            x: 1,
            y: 2,
            z: 3,
            w: 4,
        };

        #[test]
        fn test_index() {
            assert_eq!(VECTOR4[0], VECTOR4.x);
            assert_eq!(VECTOR4[1], VECTOR4.y);
            assert_eq!(VECTOR4[2], VECTOR4.z);
            assert_eq!(VECTOR4[3], VECTOR4.w);
        }

        #[test]
        fn test_index_mut() {
            let mut v = VECTOR4;
            *&mut v[2] = 0;
            assert_eq!(v, [1, 2, 0, 4].into());
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds() {
            VECTOR4[4];
        }

        #[test]
        fn test_index_range() {
            assert_eq!(&VECTOR4[..2], &[1, 2]);
            assert_eq!(&VECTOR4[..3], &[1, 2, 3]);
            assert_eq!(VECTOR4[..2].len(), 2);
            assert_eq!(VECTOR4[..3].len(), 3);
            assert_eq!(&VECTOR4[2..], &[3, 4]);
            assert_eq!(&VECTOR4[1..], &[2, 3, 4]);
            assert_eq!(VECTOR4[2..].len(), 2);
            assert_eq!(VECTOR4[1..].len(), 3);
            assert_eq!(&VECTOR4[..], &[1, 2, 3, 4]);
            assert_eq!(VECTOR4[..].len(), 4);
        }

        #[test]
        fn test_into() {
            let v = VECTOR4;
            {
                let v: [i32; 4] = v.into();
                assert_eq!(v, [1, 2, 3, 4]);
            }
            {
                let v: (i32, i32, i32, i32) = v.into();
                assert_eq!(v, (1, 2, 3, 4));
            }
        }

        #[test]
        fn test_as_ref() {
            let v = VECTOR4;
            {
                let v: &[i32; 4] = v.as_ref();
                assert_eq!(v, &[1, 2, 3, 4]);
            }
            {
                let v: &(i32, i32, i32, i32) = v.as_ref();
                assert_eq!(v, &(1, 2, 3, 4));
            }
        }

        #[test]
        fn test_as_mut() {
            let mut v = VECTOR4;
            {
                let v: &mut [i32; 4] = v.as_mut();
                assert_eq!(v, &mut [1, 2, 3, 4]);
            }
            {
                let v: &mut (i32, i32, i32, i32) = v.as_mut();
                assert_eq!(v, &mut (1, 2, 3, 4));
            }
        }

        #[test]
        fn test_from() {
            assert_eq!(Vector4::from([1, 2, 3, 4]), VECTOR4);
            {
                let v = &[1, 2, 3, 4];
                let v: &Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
            {
                let v = &mut [1, 2, 3, 4];
                let v: &mut Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
            assert_eq!(Vector4::from((1, 2, 3, 4)), VECTOR4);
            {
                let v = &(1, 2, 3, 4);
                let v: &Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
            {
                let v = &mut (1, 2, 3, 4);
                let v: &mut Vector4<_> = From::from(v);
                assert_eq!(v, &VECTOR4);
            }
        }

        #[test]
        fn test_is_finite() {
            use num_traits::Float;
            assert!(!Vector4::from([0.0, Float::nan(), 1.0, 1.0]).is_finite());
            assert!(!Vector4::from([1.0, 1.0, Float::neg_infinity(), 0.0]).is_finite());
            assert!(Vector4::from([-1.0, 0.0, 1.0, 1.0]).is_finite());
        }

        #[test]
        fn test_zip() {
            assert_eq!(
                Vector4::new(true, false, false, false),
                Vector4::new(-2, 1, 0, 1).zip(Vector4::new(-1, -1, -1, -1), |a, b| a < b)
            );
        }

        #[test]
        fn test_dot() {
            assert_eq!(vec3(1.0, 2.0, 3.0).dot(vec3(4.0, 5.0, 6.0)), 32.0);
            assert_eq!(vec3(1, 2, 3).dot(vec3(4, 5, 6)), 32);
        }
    }
}
