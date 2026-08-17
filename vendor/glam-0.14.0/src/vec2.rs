use crate::core::traits::vector::*;
use crate::{BVec2, DVec3, IVec3, UVec3, Vec3, XY};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{cmp::Ordering, f32, ops::*};

#[cfg(not(feature = "std"))]
use num_traits::Float;

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

macro_rules! impl_vec2_common_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $mask:ident, $inner:ident) => {
        /// All zeroes.
        pub const ZERO: Self = Self($inner::ZERO);

        /// All ones.
        pub const ONE: Self = Self($inner::ONE);

        /// `[1, 0]`: a unit-length vector pointing along the positive X axis.
        pub const X: Self = Self($inner::X);

        /// `[0, 1]`: a unit-length vector pointing along the positive Y axis.
        pub const Y: Self = Self($inner::Y);

        /// The unit axes.
        pub const AXES: [Self; 2] = [Self::X, Self::Y];

        /// Creates a new vector.
        #[inline(always)]
        pub fn new(x: $t, y: $t) -> $vec2 {
            Self(Vector2::new(x, y))
        }

        /// Creates a vector with values `[x: 1.0, y: 0.0]`.
        #[deprecated = "Use Vec2::X instead"]
        #[inline(always)]
        pub const fn unit_x() -> $vec2 {
            Self($inner::X)
        }

        /// Creates a vector with values `[x: 0.0, y: 1.0]`.
        #[deprecated = "Use Vec2::Y instead"]
        #[inline(always)]
        pub const fn unit_y() -> $vec2 {
            Self($inner::Y)
        }

        /// Creates a 3D vector from `self` and the given `z` value.
        #[inline(always)]
        pub fn extend(self, z: $t) -> $vec3 {
            $vec3::new(self.x, self.y, z)
        }

        impl_vecn_common_methods!($t, $vec2, $mask, $inner, Vector2);
    };
}

macro_rules! impl_vec2_signed_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $mask:ident, $inner:ident) => {
        impl_vec2_common_methods!($t, $vec2, $vec3, $mask, $inner);
        impl_vecn_signed_methods!($t, $vec2, $mask, $inner, SignedVector2);

        /// Returns a vector that is equal to `self` rotated by 90 degrees.
        #[inline(always)]
        pub fn perp(self) -> Self {
            Self(self.0.perp())
        }

        /// The perpendicular dot product of `self` and `other`.
        #[inline(always)]
        pub fn perp_dot(self, other: $vec2) -> $t {
            self.0.perp_dot(other.0)
        }
    };
}

macro_rules! impl_vec2_float_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $mask:ident, $inner:ident) => {
        impl_vec2_signed_methods!($t, $vec2, $vec3, $mask, $inner);
        impl_vecn_float_methods!($t, $vec2, $mask, $inner, FloatVector2);

        /// Returns the angle (in radians) between `self` and `other`.
        ///
        /// The input vectors do not need to be unit length however they must be non-zero.
        #[inline(always)]
        pub fn angle_between(self, other: Self) -> $t {
            self.0.angle_between(other.0)
        }
    };
}

macro_rules! impl_vec2_common_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $inner:ident) => {
        /// Creates a 2-dimensional vector.
        #[inline(always)]
        pub fn $new(x: $t, y: $t) -> $vec2 {
            $vec2::new(x, y)
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec2 {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}, {}]", self.x, self.y)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec2 {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.debug_tuple(stringify!($vec2))
                    .field(&self.x)
                    .field(&self.y)
                    .finish()
            }
        }

        impl From<($t, $t)> for $vec2 {
            #[inline(always)]
            fn from(t: ($t, $t)) -> Self {
                Self($inner::from_tuple(t))
            }
        }

        impl From<$vec2> for ($t, $t) {
            #[inline(always)]
            fn from(v: $vec2) -> Self {
                v.0.into_tuple()
            }
        }

        impl Deref for $vec2 {
            type Target = XY<$t>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref_xy()
            }
        }

        impl DerefMut for $vec2 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0.as_mut_xy()
            }
        }

        impl_vecn_common_traits!($t, 2, $vec2, $inner, Vector2);
    };
}

macro_rules! impl_vec2_unsigned_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $inner:ident) => {
        impl_vec2_common_traits!($t, $new, $vec2, $vec3, $mask, $inner);
    };
}

macro_rules! impl_vec2_signed_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $inner:ident) => {
        impl_vec2_common_traits!($t, $new, $vec2, $vec3, $mask, $inner);
        impl_vecn_signed_traits!($t, 2, $vec2, $inner, SignedVector2);
    };
}

type XYF32 = XY<f32>;

/// A 2-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec2(pub(crate) XYF32);

impl Vec2 {
    impl_vec2_float_methods!(f32, Vec2, Vec3, BVec2, XYF32);
    impl_vecn_as_f64!(DVec2, x, y);
    impl_vecn_as_i32!(IVec2, x, y);
    impl_vecn_as_u32!(UVec2, x, y);
}
impl_vec2_signed_traits!(f32, vec2, Vec2, Vec3, BVec2, XYF32);

type XYF64 = XY<f64>;

/// A 2-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DVec2(pub(crate) XYF64);

impl DVec2 {
    impl_vec2_float_methods!(f64, DVec2, DVec3, BVec2, XYF64);
    impl_vecn_as_f32!(Vec2, x, y);
    impl_vecn_as_i32!(IVec2, x, y);
    impl_vecn_as_u32!(UVec2, x, y);
}
impl_vec2_signed_traits!(f64, dvec2, DVec2, DVec3, BVec2, XYF64);

type XYI32 = XY<i32>;

/// A 2-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IVec2(pub(crate) XYI32);

impl IVec2 {
    impl_vec2_signed_methods!(i32, IVec2, IVec3, BVec2, XYI32);
    impl_vecn_as_f32!(Vec2, x, y);
    impl_vecn_as_f64!(DVec2, x, y);
    impl_vecn_as_u32!(UVec2, x, y);
}
impl_vec2_signed_traits!(i32, ivec2, IVec2, IVec3, BVec2, XYI32);
impl_vecn_eq_hash_traits!(i32, 2, IVec2);

type XYU32 = XY<u32>;

/// A 2-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec2(pub(crate) XYU32);

impl UVec2 {
    impl_vec2_common_methods!(u32, UVec2, UVec3, BVec2, XYU32);
    impl_vecn_as_f32!(Vec2, x, y);
    impl_vecn_as_f64!(DVec2, x, y);
    impl_vecn_as_i32!(IVec2, x, y);
}
impl_vec2_unsigned_traits!(u32, uvec2, UVec2, UVec3, BVec2, XYU32);
impl_vecn_eq_hash_traits!(u32, 2, UVec2);
