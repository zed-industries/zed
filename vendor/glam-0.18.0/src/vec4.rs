use crate::core::traits::vector::*;

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
use crate::BVec4A;
use crate::{BVec4, DVec2, DVec3, IVec2, IVec3, UVec2, UVec3, Vec2, Vec3, Vec3A, XYZW};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::ops::*;

#[cfg(not(feature = "std"))]
use num_traits::Float;

#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86::*;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86_64::*;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
use core::arch::wasm32::v128;

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

use core::f32;

macro_rules! impl_vec4_common_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        /// All zeroes.
        pub const ZERO: Self = Self(VectorConst::ZERO);

        /// All ones.
        pub const ONE: Self = Self(VectorConst::ONE);

        /// `[1, 0, 0, 0]`: a unit-length vector pointing along the positive X axis.
        pub const X: Self = Self(Vector4Const::X);

        /// `[0, 1, 0, 0]`: a unit-length vector pointing along the positive Y axis.
        pub const Y: Self = Self(Vector4Const::Y);

        /// `[0, 0, 1, 0]`: a unit-length vector pointing along the positive Z axis.
        pub const Z: Self = Self(Vector4Const::Z);

        /// `[0, 0, 0, 1]`: a unit-length vector pointing along the positive W axis.
        pub const W: Self = Self(Vector4Const::W);

        /// The unit axes.
        pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];

        /// Creates a new 4D vector.
        #[inline(always)]
        pub fn new(x: $t, y: $t, z: $t, w: $t) -> Self {
            Self(Vector4::new(x, y, z, w))
        }

        /// Creates a `Vec3` from the `x`, `y` and `z` elements of `self`, discarding `w`.
        ///
        /// Truncation to `Vec3` may also be performed by using `self.xyz()` or `Vec3::from()`.
        ///
        /// To truncate to `Vec3A` use `Vec3A::from()`.
        #[inline(always)]
        pub fn truncate(self) -> $vec3 {
            $vec3::new(self.x, self.y, self.z)
        }

        /// `[x, y, z, w]`
        #[inline(always)]
        pub fn to_array(&self) -> [$t; 4] {
            [self.x, self.y, self.z, self.w]
        }

        impl_vecn_common_methods!($t, $vec4, $mask, $inner, Vector4);
    };
}

macro_rules! impl_vec4_common_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        /// Creates a 4-dimensional vector.
        #[inline(always)]
        pub fn $new(x: $t, y: $t, z: $t, w: $t) -> $vec4 {
            $vec4::new(x, y, z, w)
        }

        impl Index<usize> for $vec4 {
            type Output = $t;
            #[inline(always)]
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    3 => &self.w,
                    _ => panic!("index out of bounds"),
                }
            }
        }

        impl IndexMut<usize> for $vec4 {
            #[inline(always)]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    3 => &mut self.w,
                    _ => panic!("index out of bounds"),
                }
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec4 {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple(stringify!($vec4))
                    .field(&self.x)
                    .field(&self.y)
                    .field(&self.z)
                    .field(&self.w)
                    .finish()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec4 {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(fmt, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
            }
        }

        impl From<($t, $t, $t, $t)> for $vec4 {
            #[inline(always)]
            fn from(t: ($t, $t, $t, $t)) -> Self {
                Self(Vector4::from_tuple(t))
            }
        }

        impl From<$vec4> for ($t, $t, $t, $t) {
            #[inline(always)]
            fn from(v: $vec4) -> Self {
                Vector4::into_tuple(v.0)
            }
        }

        impl From<($vec3, $t)> for $vec4 {
            #[inline(always)]
            fn from((v, w): ($vec3, $t)) -> Self {
                Self::new(v.x, v.y, v.z, w)
            }
        }

        impl From<($t, $vec3)> for $vec4 {
            #[inline(always)]
            fn from((x, v): ($t, $vec3)) -> Self {
                Self::new(x, v.x, v.y, v.z)
            }
        }

        impl From<($vec2, $t, $t)> for $vec4 {
            #[inline(always)]
            fn from((v, z, w): ($vec2, $t, $t)) -> Self {
                Self::new(v.x, v.y, z, w)
            }
        }

        impl From<($vec2, $vec2)> for $vec4 {
            #[inline(always)]
            fn from((v, u): ($vec2, $vec2)) -> Self {
                Self::new(v.x, v.y, u.x, u.y)
            }
        }

        impl From<$vec4> for $vec3 {
            /// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
            #[inline(always)]
            fn from(v: $vec4) -> Self {
                Self(v.into_xyz())
            }
        }

        impl From<$vec4> for $vec2 {
            /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z` and
            /// `w`.
            #[inline(always)]
            fn from(v: $vec4) -> Self {
                Self(v.into_xy())
            }
        }

        impl Deref for $vec4 {
            type Target = XYZW<$t>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref_xyzw()
            }
        }

        impl DerefMut for $vec4 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0.as_mut_xyzw()
            }
        }

        impl_vecn_common_traits!($t, 4, $vec4, $inner, Vector4);
    };
}

macro_rules! impl_vec4_signed_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl_vec4_common_methods!($t, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_signed_methods!($t, $vec4, $mask, $inner, SignedVector4);
    };
}

macro_rules! impl_vec4_signed_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl_vec4_common_traits!($t, $new, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_signed_traits!($t, 4, $vec4, $inner, SignedVector4);
    };
}

macro_rules! impl_vec4_float_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl_vec4_signed_methods!($t, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_float_methods!($t, $vec4, $mask, $inner, FloatVector4);
    };
}

// implement `Vec4` functionality
macro_rules! impl_f32_vec4 {
    ($new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl $vec4 {
            impl_vec4_float_methods!(f32, $vec2, $vec3, $vec4, $mask, $inner);
            impl_as_dvec4!();
            impl_as_ivec4!();
            impl_as_uvec4!();
        }
        impl_vec4_signed_traits!(f32, $new, $vec2, $vec3, $vec4, $mask, $inner);
    };
}

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
type XYZWF32 = XYZW<f32>;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type XYZWF32 = __m128;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
type XYZWF32 = v128;

/// A 4-dimensional vector.
///
/// This type uses 16 byte aligned SIMD vector type for storage on supported platforms.
#[derive(Clone, Copy)]
#[cfg_attr(
    not(any(feature = "scalar-math", target_arch = "spriv")),
    repr(align(16))
)]
#[cfg_attr(any(feature = "scalar-math", target_arch = "spriv"), repr(transparent))]
pub struct Vec4(pub(crate) XYZWF32);

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
impl_f32_vec4!(vec4, Vec2, Vec3, Vec4, BVec4, XYZWF32);

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
impl_f32_vec4!(vec4, Vec2, Vec3, Vec4, BVec4A, XYZWF32);

impl From<Vec4> for Vec3A {
    /// Creates a `Vec3A` from the `x`, `y` and `z` elements of `self` discarding `w`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on x86_64 this conversion is a noop.
    #[inline(always)]
    fn from(v: Vec4) -> Self {
        #[allow(clippy::useless_conversion)]
        Self(v.0.into())
    }
}

impl From<(Vec3A, f32)> for Vec4 {
    #[inline(always)]
    fn from((v, w): (Vec3A, f32)) -> Self {
        v.extend(w)
    }
}

impl From<(f32, Vec3A)> for Vec4 {
    #[inline(always)]
    fn from((x, v): (f32, Vec3A)) -> Self {
        Self::new(x, v.x, v.y, v.z)
    }
}

type XYZWF64 = XYZW<f64>;

/// A 4-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DVec4(pub(crate) XYZWF64);

impl DVec4 {
    impl_vec4_float_methods!(f64, DVec2, DVec3, DVec4, BVec4, XYZWF64);
    impl_as_vec4!();
    impl_as_ivec4!();
    impl_as_uvec4!();
}
impl_vec4_signed_traits!(f64, dvec4, DVec2, DVec3, DVec4, BVec4, XYZWF64);

type XYZWI32 = XYZW<i32>;

/// A 4-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IVec4(pub(crate) XYZWI32);

impl IVec4 {
    impl_vec4_signed_methods!(i32, IVec2, IVec3, IVec4, BVec4, XYZWI32);
    impl_as_vec4!();
    impl_as_dvec4!();
    impl_as_uvec4!();
}
impl_vec4_signed_traits!(i32, ivec4, IVec2, IVec3, IVec4, BVec4, XYZWI32);
impl_vecn_eq_hash_traits!(i32, 4, IVec4);

type XYZWU32 = XYZW<u32>;

/// A 4-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec4(pub(crate) XYZWU32);

impl UVec4 {
    impl_vec4_common_methods!(u32, UVec2, UVec3, UVec4, BVec4, XYZWU32);
    impl_as_vec4!();
    impl_as_dvec4!();
    impl_as_ivec4!();
}
impl_vec4_common_traits!(u32, uvec4, UVec2, UVec3, UVec4, BVec4, XYZWU32);
impl_vecn_eq_hash_traits!(u32, 4, UVec4);

#[test]
fn test_vec4_private() {
    assert_eq!(
        vec4(1.0, 1.0, 1.0, 1.0).mul_add(vec4(0.5, 2.0, -4.0, 0.0), vec4(-1.0, -1.0, -1.0, -1.0)),
        vec4(-0.5, 1.0, -5.0, -1.0)
    );
}

#[cfg(test)]
mod tests {
    use super::{vec4, Vec3};

    #[test]
    fn from_vec3() {
        assert_eq!(
            vec4(1.0, 2.0, 3.0, 4.0),
            (Vec3::new(1.0, 2.0, 3.0), 4.0).into()
        );
    }
}

#[cfg(any(feature = "scalar-math", target_arch = "spriv"))]
mod const_test_vec4 {
    const_assert_eq!(
        core::mem::align_of::<f32>(),
        core::mem::align_of::<super::Vec4>()
    );
    const_assert_eq!(16, core::mem::size_of::<super::Vec4>());
}

#[cfg(not(any(feature = "scalar-math", target_arch = "spriv")))]
mod const_test_vec4 {
    const_assert_eq!(16, core::mem::align_of::<super::Vec4>());
    const_assert_eq!(16, core::mem::size_of::<super::Vec4>());
}

mod const_test_dvec4 {
    const_assert_eq!(
        core::mem::align_of::<f64>(),
        core::mem::align_of::<super::DVec4>()
    );
    const_assert_eq!(32, core::mem::size_of::<super::DVec4>());
}

mod const_test_ivec4 {
    const_assert_eq!(
        core::mem::align_of::<i32>(),
        core::mem::align_of::<super::IVec4>()
    );
    const_assert_eq!(16, core::mem::size_of::<super::IVec4>());
}

mod const_test_uvec4 {
    const_assert_eq!(
        core::mem::align_of::<u32>(),
        core::mem::align_of::<super::UVec4>()
    );
    const_assert_eq!(16, core::mem::size_of::<super::UVec4>());
}
