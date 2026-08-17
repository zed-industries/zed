use crate::core::{
    storage::{Columns2, XY},
    traits::matrix::{FloatMatrix2x2, Matrix2x2, MatrixConst},
};
use crate::{DMat3, DVec2, Mat3, Vec2};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign};

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

#[cfg(target_feature = "simd128")]
use core::arch::wasm32::v128;

macro_rules! impl_mat2_methods {
    ($t:ty, $vec2:ident, $mat3:ident, $inner:ident) => {
        /// A 2x2 matrix with all elements set to `0.0`.
        pub const ZERO: Self = Self($inner::ZERO);

        /// A 2x2 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
        pub const IDENTITY: Self = Self($inner::IDENTITY);

        /// All NAN:s.
        pub const NAN: Self = Self(<$inner as crate::core::traits::scalar::NanConstEx>::NAN);

        /// Creates a 2x2 matrix from two column vectors.
        #[inline(always)]
        pub fn from_cols(x_axis: $vec2, y_axis: $vec2) -> Self {
            Self($inner::from_cols(x_axis.0, y_axis.0))
        }

        /// Creates a 2x2 matrix from a `[S; 4]` array stored in column major order.
        /// If your data is stored in row major you will need to `transpose` the returned
        /// matrix.
        #[inline(always)]
        pub fn from_cols_array(m: &[$t; 4]) -> Self {
            Self($inner::from_cols_array(m))
        }

        /// Creates a `[S; 4]` array storing data in column major order.
        /// If you require data in row major order `transpose` the matrix first.
        #[inline(always)]
        pub fn to_cols_array(&self) -> [$t; 4] {
            self.0.to_cols_array()
        }

        /// Creates a 2x2 matrix from a `[[S; 2]; 2]` 2D array stored in column major order.
        /// If your data is in row major order you will need to `transpose` the returned
        /// matrix.
        #[inline(always)]
        pub fn from_cols_array_2d(m: &[[$t; 2]; 2]) -> Self {
            Self($inner::from_cols_array_2d(m))
        }

        /// Creates a `[[S; 2]; 2]` 2D array storing data in column major order.
        /// If you require data in row major order `transpose` the matrix first.
        #[inline(always)]
        pub fn to_cols_array_2d(&self) -> [[$t; 2]; 2] {
            self.0.to_cols_array_2d()
        }

        /// Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
        #[doc(alias = "scale")]
        #[inline(always)]
        pub fn from_diagonal(diagonal: $vec2) -> Self {
            Self($inner::from_diagonal(diagonal.0))
        }

        /// Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
        /// `angle` (in radians).
        #[inline(always)]
        pub fn from_scale_angle(scale: $vec2, angle: $t) -> Self {
            Self($inner::from_scale_angle(scale.0, angle))
        }

        /// Creates a 2x2 matrix containing a rotation of `angle` (in radians).
        #[inline(always)]
        pub fn from_angle(angle: $t) -> Self {
            Self($inner::from_angle(angle))
        }

        /// Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
        #[inline(always)]
        pub fn from_mat3(m: $mat3) -> Self {
            Self::from_cols($vec2(m.x_axis.0.into()), $vec2(m.y_axis.0.into()))
        }

        /// Creates a 2x2 matrix from the first 4 values in `slice`.
        ///
        /// # Panics
        ///
        /// Panics if `slice` is less than 4 elements long.
        #[inline(always)]
        pub fn from_cols_slice(slice: &[$t]) -> Self {
            Self(Matrix2x2::from_cols_slice(slice))
        }

        /// Writes the columns of `self` to the first 4 elements in `slice`.
        ///
        /// # Panics
        ///
        /// Panics if `slice` is less than 4 elements long.
        #[inline(always)]
        pub fn write_cols_to_slice(self, slice: &mut [$t]) {
            Matrix2x2::write_cols_to_slice(&self.0, slice)
        }

        /// Returns the matrix column for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 1.
        #[inline]
        pub fn col(&self, index: usize) -> $vec2 {
            match index {
                0 => self.x_axis,
                1 => self.y_axis,
                _ => panic!("index out of bounds"),
            }
        }

        /// Returns a mutable reference to the matrix column for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 1.
        #[inline]
        pub fn col_mut(&mut self, index: usize) -> &mut $vec2 {
            match index {
                0 => &mut self.x_axis,
                1 => &mut self.y_axis,
                _ => panic!("index out of bounds"),
            }
        }

        /// Returns the matrix row for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 1.
        #[inline]
        pub fn row(&self, index: usize) -> $vec2 {
            match index {
                0 => $vec2::new(self.x_axis.x, self.y_axis.x),
                1 => $vec2::new(self.x_axis.y, self.y_axis.y),
                _ => panic!("index out of bounds"),
            }
        }

        /// Returns `true` if, and only if, all elements are finite.
        /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
        #[inline]
        pub fn is_finite(&self) -> bool {
            // TODO
            self.x_axis.is_finite() && self.y_axis.is_finite()
        }

        /// Returns `true` if any elements are `NaN`.
        #[inline]
        pub fn is_nan(&self) -> bool {
            self.x_axis.is_nan() || self.y_axis.is_nan()
        }

        /// Returns the transpose of `self`.
        #[must_use]
        #[inline(always)]
        pub fn transpose(&self) -> Self {
            Self(self.0.transpose())
        }

        /// Returns the determinant of `self`.
        #[inline(always)]
        pub fn determinant(&self) -> $t {
            self.0.determinant()
        }

        /// Returns the inverse of `self`.
        ///
        /// If the matrix is not invertible the returned matrix will be invalid.
        ///
        /// # Panics
        ///
        /// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
        #[must_use]
        #[inline(always)]
        pub fn inverse(&self) -> Self {
            Self(self.0.inverse())
        }

        /// Transforms a 2D vector.
        #[inline(always)]
        pub fn mul_vec2(&self, other: $vec2) -> $vec2 {
            $vec2(self.0.mul_vector(other.0))
        }

        /// Multiplies two 2x2 matrices.
        #[inline(always)]
        pub fn mul_mat2(&self, other: &Self) -> Self {
            Self(self.0.mul_matrix(&other.0))
        }

        /// Adds two 2x2 matrices.
        #[inline(always)]
        pub fn add_mat2(&self, other: &Self) -> Self {
            Self(self.0.add_matrix(&other.0))
        }

        /// Subtracts two 2x2 matrices.
        #[inline(always)]
        pub fn sub_mat2(&self, other: &Self) -> Self {
            Self(self.0.sub_matrix(&other.0))
        }

        /// Multiplies a 2x2 matrix by a scalar.
        #[inline(always)]
        pub fn mul_scalar(&self, other: $t) -> Self {
            Self(self.0.mul_scalar(other))
        }

        /// Returns true if the absolute difference of all elements between `self` and `other`
        /// is less than or equal to `max_abs_diff`.
        ///
        /// This can be used to compare if two matrices contain similar elements. It works best
        /// when comparing with a known value. The `max_abs_diff` that should be used used
        /// depends on the values being compared against.
        ///
        /// For more see
        /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
        #[inline(always)]
        pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: $t) -> bool {
            self.0.abs_diff_eq(&other.0, max_abs_diff)
        }
    };
}

macro_rules! impl_mat2_traits {
    ($t:ty, $new:ident, $mat2:ident, $vec2:ident) => {
        /// Creates a 2x2 matrix from two column vectors.
        #[inline(always)]
        pub fn $new(x_axis: $vec2, y_axis: $vec2) -> $mat2 {
            $mat2::from_cols(x_axis, y_axis)
        }

        impl_matn_common_traits!($t, $mat2, $vec2);

        impl PartialEq for $mat2 {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.x_axis.eq(&other.x_axis) && self.y_axis.eq(&other.y_axis)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsRef<[$t; 4]> for $mat2 {
            #[inline(always)]
            fn as_ref(&self) -> &[$t; 4] {
                unsafe { &*(self as *const Self as *const [$t; 4]) }
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsMut<[$t; 4]> for $mat2 {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut [$t; 4] {
                unsafe { &mut *(self as *mut Self as *mut [$t; 4]) }
            }
        }

        impl Deref for $mat2 {
            type Target = Columns2<$vec2>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }

        impl DerefMut for $mat2 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut Self as *mut Self::Target) }
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $mat2 {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct(stringify!($mat2))
                    .field("x_axis", &self.x_axis)
                    .field("y_axis", &self.y_axis)
                    .finish()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $mat2 {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{}, {}]", self.x_axis, self.y_axis)
            }
        }
    };
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type InnerF32 = __m128;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
type InnerF32 = v128;

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
type InnerF32 = crate::core::storage::Columns2<XY<f32>>;

/// A 2x2 column major matrix.
#[derive(Clone, Copy)]
#[cfg_attr(
    not(any(
        feature = "scalar-math",
        target_arch = "spirv",
        target_feature = "sse2",
        target_feature = "simd128"
    )),
    repr(C, align(16))
)]
#[cfg_attr(feature = "cuda", repr(C, align(8)))]
#[cfg_attr(
    all(
        any(
            feature = "scalar-math",
            target_arch = "spirv",
            target_feature = "sse2",
            target_feature = "simd128"
        ),
        not(feature = "cuda"),
    ),
    repr(transparent)
)]
pub struct Mat2(pub(crate) InnerF32);

impl Mat2 {
    impl_mat2_methods!(f32, Vec2, Mat3, InnerF32);

    #[inline(always)]
    pub fn as_dmat2(&self) -> DMat2 {
        DMat2::from_cols(self.x_axis.as_dvec2(), self.y_axis.as_dvec2())
    }
}
impl_mat2_traits!(f32, mat2, Mat2, Vec2);

type InnerF64 = crate::core::storage::Columns2<XY<f64>>;

/// A 2x2 column major matrix.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "cuda", repr(C, align(16)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
pub struct DMat2(pub(crate) InnerF64);

impl DMat2 {
    impl_mat2_methods!(f64, DVec2, DMat3, InnerF64);

    #[inline(always)]
    pub fn as_mat2(&self) -> Mat2 {
        Mat2::from_cols(self.x_axis.as_vec2(), self.y_axis.as_vec2())
    }
}
impl_mat2_traits!(f64, dmat2, DMat2, DVec2);

mod const_test_mat2 {
    #[cfg(any(feature = "scalar-math", target_arch = "spirv"))]
    const_assert_eq!(
        core::mem::align_of::<super::Vec2>(),
        core::mem::align_of::<super::Mat2>()
    );
    #[cfg(not(any(feature = "scalar-math", target_arch = "spirv")))]
    const_assert_eq!(16, core::mem::align_of::<super::Mat2>());
    const_assert_eq!(16, core::mem::size_of::<super::Mat2>());
}

mod const_test_dmat2 {
    const_assert_eq!(
        core::mem::align_of::<super::DVec2>(),
        core::mem::align_of::<super::DMat2>()
    );
    const_assert_eq!(32, core::mem::size_of::<super::DMat2>());
}
