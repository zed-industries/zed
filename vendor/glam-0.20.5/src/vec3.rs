use crate::core::traits::vector::*;
#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
use crate::BVec3A;
use crate::{BVec3, DVec2, DVec4, IVec2, IVec4, UVec2, UVec4, Vec2, Vec4, XYZ};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

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

macro_rules! impl_vec3_common_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        /// All zeroes.
        pub const ZERO: Self = Self(VectorConst::ZERO);

        /// All ones.
        pub const ONE: Self = Self(VectorConst::ONE);

        /// `[1, 0, 0]`: a unit-length vector pointing along the positive X axis.
        pub const X: Self = Self(Vector3Const::X);

        /// `[0, 1, 0]`: a unit-length vector pointing along the positive Y axis.
        pub const Y: Self = Self(Vector3Const::Y);

        /// `[0, 0, 1]`: a unit-length vector pointing along the positive Z axis.
        pub const Z: Self = Self(Vector3Const::Z);

        /// The unit axes.
        pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

        /// Creates a new 3D vector.
        #[inline(always)]
        pub fn new(x: $t, y: $t, z: $t) -> Self {
            Self(Vector3::new(x, y, z))
        }

        /// Creates a 4D vector from `self` and the given `w` value.
        #[inline(always)]
        pub fn extend(self, w: $t) -> $vec4 {
            // TODO: Optimize?
            $vec4(Vector4::new(self.x, self.y, self.z, w))
        }

        /// Creates a `Vec2` from the `x` and `y` elements of `self`, discarding `z`.
        ///
        /// Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
        #[inline(always)]
        pub fn truncate(self) -> $vec2 {
            $vec2(Vector3::into_xy(self.0))
        }

        /// Returns the dot product result in all elements of the vector
        #[inline(always)]
        #[allow(dead_code)]
        pub(crate) fn dot_as_vec3(self, other: Self) -> Self {
            Self(Vector3::dot_into_vec(self.0, other.0))
        }

        /// Computes the cross product of `self` and `other`.
        #[inline(always)]
        pub fn cross(self, other: Self) -> Self {
            Self(self.0.cross(other.0))
        }

        /// `[x, y, z]`
        #[inline(always)]
        pub fn to_array(&self) -> [$t; 3] {
            [self.x, self.y, self.z]
        }

        impl_vecn_common_methods!($t, $vec3, $mask, $inner, Vector3);
    };
}

macro_rules! impl_vec3_common_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $inner:ident) => {
        /// Creates a 3-dimensional vector.
        #[inline(always)]
        pub fn $new(x: $t, y: $t, z: $t) -> $vec3 {
            $vec3::new(x, y, z)
        }

        impl Index<usize> for $vec3 {
            type Output = $t;
            #[inline(always)]
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!("index out of bounds"),
                }
            }
        }

        impl IndexMut<usize> for $vec3 {
            #[inline(always)]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => panic!("index out of bounds"),
                }
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec3 {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple(stringify!($vec3))
                    .field(&self.x)
                    .field(&self.y)
                    .field(&self.z)
                    .finish()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec3 {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
            }
        }

        impl From<($vec2, $t)> for $vec3 {
            #[inline(always)]
            fn from((v, z): ($vec2, $t)) -> Self {
                Self::new(v.x, v.y, z)
            }
        }

        impl From<($t, $t, $t)> for $vec3 {
            #[inline(always)]
            fn from(t: ($t, $t, $t)) -> Self {
                Self(Vector3::from_tuple(t))
            }
        }

        impl From<$vec3> for ($t, $t, $t) {
            #[inline(always)]
            fn from(v: $vec3) -> Self {
                v.into_tuple()
            }
        }

        impl Deref for $vec3 {
            type Target = XYZ<$t>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref_xyz()
            }
        }

        impl DerefMut for $vec3 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0.as_mut_xyz()
            }
        }

        impl_vecn_common_traits!($t, 3, $vec3, $inner, Vector3);
    };
}

macro_rules! impl_vec3_signed_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl_vec3_common_methods!($t, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_signed_methods!($t, $vec3, $mask, $inner, SignedVector3);
    };
}

macro_rules! impl_vec3_float_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl_vec3_signed_methods!($t, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_float_methods!($t, $vec3, $mask, $inner, FloatVector3);

        /// Returns the angle (in radians) between two vectors.
        ///
        /// The input vectors do not need to be unit length however they must be non-zero.
        #[inline(always)]
        pub fn angle_between(self, other: Self) -> $t {
            self.0.angle_between(other.0)
        }

        /// Returns some vector that is orthogonal to the given one.
        ///
        /// The input vector must be finite and non-zero.
        ///
        /// The output vector is not necessarily unit-length.
        /// For that use [`Self::any_orthonormal_vector`] instead.
        #[inline]
        pub fn any_orthogonal_vector(&self) -> Self {
            // This can probably be optimized
            if self.x.abs() > self.y.abs() {
                Self::new(-self.z, 0.0, self.x) // self.cross(Self::Y)
            } else {
                Self::new(0.0, self.z, -self.y) // self.cross(Self::X)
            }
        }

        /// Returns any unit-length vector that is orthogonal to the given one.
        /// The input vector must be finite and non-zero.
        ///
        /// # Panics
        ///
        /// Will panic if `self` is not normalized when `glam_assert` is enabled.
        #[inline]
        pub fn any_orthonormal_vector(&self) -> Self {
            glam_assert!(self.is_normalized());
            // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
            #[cfg(feature = "std")]
            let sign = (1.0 as $t).copysign(self.z);
            #[cfg(not(feature = "std"))]
            let sign = self.z.signum();
            let a = -1.0 / (sign + self.z);
            let b = self.x * self.y * a;
            Self::new(b, sign + self.y * self.y * a, -self.y)
        }

        /// Given a unit-length vector return two other vectors that together form an orthonormal
        /// basis.  That is, all three vectors are orthogonal to each other and are normalized.
        ///
        /// # Panics
        ///
        /// Will panic if `self` is not normalized when `glam_assert` is enabled.
        #[inline]
        pub fn any_orthonormal_pair(&self) -> (Self, Self) {
            glam_assert!(self.is_normalized());
            // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
            #[cfg(feature = "std")]
            let sign = (1.0 as $t).copysign(self.z);
            #[cfg(not(feature = "std"))]
            let sign = self.z.signum();
            let a = -1.0 / (sign + self.z);
            let b = self.x * self.y * a;
            (
                Self::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x),
                Self::new(b, sign + self.y * self.y * a, -self.y),
            )
        }
    };
}

// implements traits that are common between `Vec3`, `Vec3A` and `Vec4` types.
macro_rules! impl_vec3_float_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $inner:ident) => {
        impl_vec3_common_traits!($t, $new, $vec2, $vec3, $vec4, $inner);
        impl_vecn_signed_traits!($t, 3, $vec3, $inner, SignedVector3);
    };
}

// implements f32 functionality common between `Vec3` and `Vec3A` types.
macro_rules! impl_f32_vec3 {
    ($new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl $vec3 {
            impl_vec3_float_methods!(f32, $vec2, $vec3, $vec4, $mask, $inner);
            impl_as_dvec3!();
            impl_as_ivec3!();
            impl_as_uvec3!();
        }
        impl_vec3_float_traits!(f32, $new, $vec2, $vec3, $vec4, $inner);
    };
}

type XYZF32 = XYZ<f32>;

/// A 3-dimensional vector without SIMD support.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3(pub(crate) XYZF32);
impl_f32_vec3!(vec3, Vec2, Vec3, Vec4, BVec3, XYZF32);

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type XYZF32A = __m128;
#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
type XYZF32A = v128;

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
type XYZF32A = crate::core::storage::XYZF32A16;

/// A 3-dimensional vector with SIMD support.
///
/// This type is 16 byte aligned. A SIMD vector type is used for storage on supported platforms for
/// better performance than the `Vec3` type.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3A(pub(crate) XYZF32A);

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
impl_f32_vec3!(vec3a, Vec2, Vec3A, Vec4, BVec3A, XYZF32A);

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
impl_f32_vec3!(vec3a, Vec2, Vec3A, Vec4, BVec3, XYZF32A);

impl From<Vec3> for Vec3A {
    #[inline(always)]
    fn from(v: Vec3) -> Self {
        Self(v.0.into())
    }
}

impl From<Vec3A> for Vec3 {
    #[inline(always)]
    fn from(v: Vec3A) -> Self {
        Self(v.0.into())
    }
}

type XYZF64 = XYZ<f64>;

/// A 3-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DVec3(pub(crate) XYZF64);

impl DVec3 {
    impl_vec3_float_methods!(f64, DVec2, DVec3, DVec4, BVec3, XYZF64);
    impl_as_vec3!();
    impl_as_ivec3!();
    impl_as_uvec3!();
}
impl_vec3_float_traits!(f64, dvec3, DVec2, DVec3, DVec4, XYZF64);

type XYZI32 = XYZ<i32>;

/// A 3-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IVec3(pub(crate) XYZI32);

impl IVec3 {
    impl_vec3_common_methods!(i32, IVec2, IVec3, IVec4, BVec3, XYZI32);
    impl_vecn_signed_methods!(i32, IVec3, BVec3, XYZI32, SignedVector3);
    impl_as_vec3!();
    impl_as_dvec3!();
    impl_as_uvec3!();
}
impl_vec3_common_traits!(i32, ivec3, IVec2, IVec3, IVec4, XYZI32);
impl_vecn_signed_traits!(i32, 3, IVec3, XYZI32, SignedVector3);
impl_vecn_eq_hash_traits!(i32, 3, IVec3);

impl_vecn_scalar_shift_op_traits!(IVec3, i8, XYZI32);
impl_vecn_scalar_shift_op_traits!(IVec3, i16, XYZI32);
impl_vecn_scalar_shift_op_traits!(IVec3, i32, XYZI32);
impl_vecn_scalar_shift_op_traits!(IVec3, u8, XYZI32);
impl_vecn_scalar_shift_op_traits!(IVec3, u16, XYZI32);
impl_vecn_scalar_shift_op_traits!(IVec3, u32, XYZI32);

impl_vecn_shift_op_traits!(IVec3, IVec3, XYZI32);
impl_vecn_shift_op_traits!(IVec3, UVec3, XYZI32);

impl_vecn_scalar_bit_op_traits!(IVec3, i32, XYZI32);

impl_vecn_bit_op_traits!(IVec3, XYZI32);

type XYZU32 = XYZ<u32>;

/// A 3-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec3(pub(crate) XYZU32);

impl UVec3 {
    impl_vec3_common_methods!(u32, UVec2, UVec3, UVec4, BVec3, XYZU32);
    impl_as_vec3!();
    impl_as_dvec3!();
    impl_as_ivec3!();
}
impl_vec3_common_traits!(u32, uvec3, UVec2, UVec3, UVec4, XYZU32);
impl_vecn_eq_hash_traits!(u32, 3, UVec3);

impl_vecn_scalar_shift_op_traits!(UVec3, i8, XYZU32);
impl_vecn_scalar_shift_op_traits!(UVec3, i16, XYZU32);
impl_vecn_scalar_shift_op_traits!(UVec3, i32, XYZU32);
impl_vecn_scalar_shift_op_traits!(UVec3, u8, XYZU32);
impl_vecn_scalar_shift_op_traits!(UVec3, u16, XYZU32);
impl_vecn_scalar_shift_op_traits!(UVec3, u32, XYZU32);

impl_vecn_shift_op_traits!(UVec3, IVec3, XYZU32);
impl_vecn_shift_op_traits!(UVec3, UVec3, XYZU32);

impl_vecn_scalar_bit_op_traits!(UVec3, u32, XYZU32);

impl_vecn_bit_op_traits!(UVec3, XYZU32);

mod const_test_vec3 {
    const_assert_eq!(
        core::mem::align_of::<f32>(),
        core::mem::align_of::<super::Vec3>()
    );
    const_assert_eq!(12, core::mem::size_of::<super::Vec3>());
}

mod const_test_vec3a {
    const_assert_eq!(16, core::mem::align_of::<super::Vec3A>());
    const_assert_eq!(16, core::mem::size_of::<super::Vec3A>());
}

mod const_test_dvec3 {
    const_assert_eq!(
        core::mem::align_of::<f64>(),
        core::mem::align_of::<super::DVec3>()
    );
    const_assert_eq!(24, core::mem::size_of::<super::DVec3>());
}

mod const_test_ivec3 {
    const_assert_eq!(
        core::mem::align_of::<i32>(),
        core::mem::align_of::<super::IVec3>()
    );
    const_assert_eq!(12, core::mem::size_of::<super::IVec3>());
}

mod const_test_uvec3 {
    const_assert_eq!(
        core::mem::align_of::<u32>(),
        core::mem::align_of::<super::UVec3>()
    );
    const_assert_eq!(12, core::mem::size_of::<super::UVec3>());
}
