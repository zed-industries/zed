use crate::core::traits::vector::*;
use crate::{BVec2, DVec3, IVec3, UVec3, Vec3, XY};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

#[cfg(not(feature = "std"))]
use num_traits::Float;

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

        /// Creates a 3D vector from `self` and the given `z` value.
        #[inline(always)]
        pub fn extend(self, z: $t) -> $vec3 {
            $vec3::new(self.x, self.y, z)
        }

        /// `[x, y]`
        #[inline(always)]
        pub fn to_array(&self) -> [$t; 2] {
            [self.x, self.y]
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
        /// Also known as the wedge product, 2d cross product, and determinant.
        #[doc(alias = "wedge")]
        #[doc(alias = "cross")]
        #[doc(alias = "determinant")]
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

        impl Index<usize> for $vec2 {
            type Output = $t;
            #[inline(always)]
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!("index out of bounds"),
                }
            }
        }

        impl IndexMut<usize> for $vec2 {
            #[inline(always)]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => panic!("index out of bounds"),
                }
            }
        }
        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec2 {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{}, {}]", self.x, self.y)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec2 {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
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
#[cfg_attr(feature = "cuda", repr(C, align(8)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
pub struct Vec2(pub(crate) XYF32);

impl Vec2 {
    impl_vec2_float_methods!(f32, Vec2, Vec3, BVec2, XYF32);
    impl_as_dvec2!();
    impl_as_ivec2!();
    impl_as_uvec2!();
}
impl_vec2_signed_traits!(f32, vec2, Vec2, Vec3, BVec2, XYF32);

type XYF64 = XY<f64>;

/// A 2-dimensional vector.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "cuda", repr(C, align(16)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
pub struct DVec2(pub(crate) XYF64);

impl DVec2 {
    impl_vec2_float_methods!(f64, DVec2, DVec3, BVec2, XYF64);
    impl_as_vec2!();
    impl_as_ivec2!();
    impl_as_uvec2!();
}
impl_vec2_signed_traits!(f64, dvec2, DVec2, DVec3, BVec2, XYF64);

type XYI32 = XY<i32>;

/// A 2-dimensional vector.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "cuda", repr(C, align(8)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
pub struct IVec2(pub(crate) XYI32);

impl IVec2 {
    impl_vec2_signed_methods!(i32, IVec2, IVec3, BVec2, XYI32);
    impl_as_vec2!();
    impl_as_dvec2!();
    impl_as_uvec2!();
}
impl_vec2_signed_traits!(i32, ivec2, IVec2, IVec3, BVec2, XYI32);
impl_vecn_eq_hash_traits!(i32, 2, IVec2);

impl_vecn_scalar_shift_op_traits!(IVec2, i8, XYI32);
impl_vecn_scalar_shift_op_traits!(IVec2, i16, XYI32);
impl_vecn_scalar_shift_op_traits!(IVec2, i32, XYI32);
impl_vecn_scalar_shift_op_traits!(IVec2, u8, XYI32);
impl_vecn_scalar_shift_op_traits!(IVec2, u16, XYI32);
impl_vecn_scalar_shift_op_traits!(IVec2, u32, XYI32);

impl_vecn_shift_op_traits!(IVec2, IVec2, XYI32);
impl_vecn_shift_op_traits!(IVec2, UVec2, XYI32);

impl_vecn_scalar_bit_op_traits!(IVec2, i32, XYI32);

impl_vecn_bit_op_traits!(IVec2, XYI32);

type XYU32 = XY<u32>;

/// A 2-dimensional vector.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "cuda", repr(C, align(8)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
pub struct UVec2(pub(crate) XYU32);

impl UVec2 {
    impl_vec2_common_methods!(u32, UVec2, UVec3, BVec2, XYU32);
    impl_as_vec2!();
    impl_as_dvec2!();
    impl_as_ivec2!();
}
impl_vec2_unsigned_traits!(u32, uvec2, UVec2, UVec3, BVec2, XYU32);
impl_vecn_eq_hash_traits!(u32, 2, UVec2);

impl_vecn_scalar_shift_op_traits!(UVec2, i8, XYU32);
impl_vecn_scalar_shift_op_traits!(UVec2, i16, XYU32);
impl_vecn_scalar_shift_op_traits!(UVec2, i32, XYU32);
impl_vecn_scalar_shift_op_traits!(UVec2, u8, XYU32);
impl_vecn_scalar_shift_op_traits!(UVec2, u16, XYU32);
impl_vecn_scalar_shift_op_traits!(UVec2, u32, XYU32);

impl_vecn_shift_op_traits!(UVec2, IVec2, XYU32);
impl_vecn_shift_op_traits!(UVec2, UVec2, XYU32);

impl_vecn_scalar_bit_op_traits!(UVec2, u32, XYU32);

impl_vecn_bit_op_traits!(UVec2, XYU32);

mod const_test_vec2 {
    #[cfg(not(feature = "cuda"))]
    const_assert_eq!(
        core::mem::align_of::<f32>(),
        core::mem::align_of::<super::Vec2>()
    );
    #[cfg(feature = "cuda")]
    const_assert_eq!(8, core::mem::align_of::<super::Vec2>());
    const_assert_eq!(8, core::mem::size_of::<super::Vec2>());
}

mod const_test_dvec2 {
    #[cfg(not(feature = "cuda"))]
    const_assert_eq!(
        core::mem::align_of::<f64>(),
        core::mem::align_of::<super::DVec2>()
    );
    #[cfg(feature = "cuda")]
    const_assert_eq!(16, core::mem::align_of::<super::DVec2>());
    const_assert_eq!(16, core::mem::size_of::<super::DVec2>());
}

mod const_test_ivec2 {
    #[cfg(not(feature = "cuda"))]
    const_assert_eq!(
        core::mem::align_of::<i32>(),
        core::mem::align_of::<super::IVec2>()
    );
    #[cfg(feature = "cuda")]
    const_assert_eq!(8, core::mem::align_of::<super::IVec2>());
    const_assert_eq!(8, core::mem::size_of::<super::IVec2>());
}

mod const_test_uvec2 {
    #[cfg(not(feature = "cuda"))]
    const_assert_eq!(
        core::mem::align_of::<u32>(),
        core::mem::align_of::<super::UVec2>()
    );
    #[cfg(feature = "cuda")]
    const_assert_eq!(8, core::mem::align_of::<super::UVec2>());
    const_assert_eq!(8, core::mem::size_of::<super::UVec2>());
}
