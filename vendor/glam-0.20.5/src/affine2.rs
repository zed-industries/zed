use crate::core::storage::Columns3;
use crate::{DMat2, DMat3, DVec2, Mat2, Mat3, Mat3A, Vec2, Vec3A};
use core::ops::{Add, Deref, DerefMut, Mul, Sub};

#[cfg(not(feature = "std"))]
use num_traits::Float;

macro_rules! define_affine2_struct {
    ($affine2:ident, $matrix:ident, $column:ident) => {
        /// A 2D affine transform, which can represent translation, rotation, scaling and shear.
        #[derive(Copy, Clone)]
        #[repr(C)]
        pub struct $affine2 {
            pub matrix2: $matrix,
            pub translation: $column,
        }
    };
}

macro_rules! impl_affine2_methods {
    ($t:ty, $mat2:ident, $mat3:ident, $vec2:ident, $affine2:ident, $matrix:ident, $column:ident) => {
        impl $affine2 {
            /// The degenerate zero transform.
            ///
            /// This transforms any finite vector and point to zero.
            /// The zero transform is non-invertible.
            pub const ZERO: Self = Self {
                matrix2: $matrix::ZERO,
                translation: $column::ZERO,
            };

            /// The identity transform.
            ///
            /// Multiplying a vector with this returns the same vector.
            pub const IDENTITY: Self = Self {
                matrix2: $matrix::IDENTITY,
                translation: $column::ZERO,
            };

            /// All NAN:s.
            pub const NAN: Self = Self {
                matrix2: $matrix::NAN,
                translation: $column::NAN,
            };

            /// Creates an affine transform from three column vectors.
            #[inline(always)]
            pub fn from_cols(x_axis: $column, y_axis: $column, z_axis: $column) -> Self {
                Self {
                    matrix2: $matrix::from_cols(x_axis, y_axis),
                    translation: z_axis,
                }
            }

            /// Creates an affine transform from a `[S; 6]` array stored in column major order.
            /// If your data is stored in row major you will need to `transpose` the returned
            /// matrix.
            #[inline(always)]
            pub fn from_cols_array(m: &[$t; 6]) -> Self {
                Self {
                    matrix2: $matrix::from_cols_slice(&m[0..4]),
                    translation: $column::from_slice(&m[4..6]),
                }
            }

            /// Creates a `[S; 6]` array storing data in column major order.
            /// If you require data in row major order `transpose` the matrix first.
            #[inline(always)]
            pub fn to_cols_array(&self) -> [$t; 6] {
                let x = &self.matrix2.x_axis;
                let y = &self.matrix2.y_axis;
                let z = &self.translation;
                [x.x, x.y, y.x, y.y, z.x, z.y]
            }

            /// Creates an affine transform from a `[[S; 2]; 3]` 2D array stored in column major order.
            /// If your data is in row major order you will need to `transpose` the returned
            /// matrix.
            #[inline(always)]
            pub fn from_cols_array_2d(m: &[[$t; 2]; 3]) -> Self {
                Self {
                    matrix2: $matrix::from_cols(m[0].into(), m[1].into()),
                    translation: m[2].into(),
                }
            }

            /// Creates a `[[S; 2]; 3]` 2D array storing data in column major order.
            /// If you require data in row major order `transpose` the matrix first.
            #[inline(always)]
            pub fn to_cols_array_2d(&self) -> [[$t; 2]; 3] {
                [
                    self.matrix2.x_axis.into(),
                    self.matrix2.y_axis.into(),
                    self.translation.into(),
                ]
            }

            /// Creates an affine transform from the first 6 values in `slice`.
            ///
            /// # Panics
            ///
            /// Panics if `slice` is less than 6 elements long.
            #[inline(always)]
            pub fn from_cols_slice(slice: &[$t]) -> Self {
                Self {
                    matrix2: $matrix::from_cols_slice(&slice[0..4]),
                    translation: $column::from_slice(&slice[4..6]),
                }
            }

            /// Writes the columns of `self` to the first 12 elements in `slice`.
            ///
            /// # Panics
            ///
            /// Panics if `slice` is less than 12 elements long.
            #[inline(always)]
            pub fn write_cols_to_slice(self, slice: &mut [$t]) {
                self.matrix2.write_cols_to_slice(&mut slice[0..4]);
                self.translation.write_to_slice(&mut slice[4..6]);
            }

            /// Creates an affine transform that changes scale.
            /// Note that if any scale is zero the transform will be non-invertible.
            #[inline(always)]
            pub fn from_scale(scale: $vec2) -> Self {
                Self {
                    matrix2: $matrix::from_diagonal(scale),
                    translation: $column::ZERO,
                }
            }

            /// Creates an affine transform from the given rotation `angle`.
            #[inline(always)]
            pub fn from_angle(angle: $t) -> Self {
                Self {
                    matrix2: $matrix::from_angle(angle),
                    translation: $column::ZERO,
                }
            }

            /// Creates an affine transformation from the given 2D `translation`.
            #[inline(always)]
            pub fn from_translation(translation: $vec2) -> Self {
                Self {
                    matrix2: $matrix::IDENTITY,
                    translation,
                }
            }

            /// Creates an affine transform from a 2x2 matrix (expressing scale, shear and
            /// rotation)
            #[inline(always)]
            pub fn from_mat2(matrix2: $mat2) -> Self {
                Self {
                    matrix2,
                    translation: $column::ZERO,
                }
            }

            /// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
            /// and a translation vector.
            ///
            /// Equivalent to `Affine2::from_translation(translation) * Affine2::from_mat2(mat2)`
            #[inline(always)]
            pub fn from_mat2_translation(matrix2: $mat2, translation: $vec2) -> Self {
                Self {
                    matrix2,
                    translation,
                }
            }

            /// Creates an affine transform from the given 2D `scale`, rotation `angle` (in
            /// radians) and `translation`.
            ///
            /// Equivalent to `Affine2::from_translation(translation) *
            /// Affine2::from_angle(angle) * Affine2::from_scale(scale)`
            #[inline]
            pub fn from_scale_angle_translation(
                scale: $vec2,
                angle: $t,
                translation: $vec2,
            ) -> Self {
                let rotation = $matrix::from_angle(angle);
                Self {
                    matrix2: $matrix::from_cols(
                        rotation.x_axis * scale.x,
                        rotation.y_axis * scale.y,
                    ),
                    translation,
                }
            }

            /// Creates an affine transform from the given 2D rotation `angle` (in radians) and
            /// `translation`.
            ///
            /// Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle)`
            #[inline(always)]
            pub fn from_angle_translation(angle: $t, translation: $vec2) -> Self {
                Self {
                    matrix2: $matrix::from_angle(angle),
                    translation,
                }
            }

            /// The given `Mat3` must be an affine transform,
            #[inline]
            pub fn from_mat3(m: $mat3) -> Self {
                Self {
                    matrix2: $matrix::from_cols($vec2(m.x_axis.0.into()), $vec2(m.y_axis.0.into())),
                    translation: $vec2(m.z_axis.0.into()),
                }
            }

            /// Transforms the given 2D point, applying shear, scale, rotation and translation.
            #[inline(always)]
            pub fn transform_point2(&self, other: $vec2) -> $vec2 {
                self.matrix2 * other + self.translation
            }

            /// Transforms the given 2D vector, applying shear, scale and rotation (but NOT
            /// translation).
            ///
            /// To also apply translation, use [`Self::transform_point2`] instead.
            #[inline(always)]
            pub fn transform_vector2(&self, other: $vec2) -> $vec2 {
                self.matrix2 * other
            }

            /// Returns `true` if, and only if, all elements are finite.
            ///
            /// If any element is either `NaN`, positive or negative infinity, this will return
            /// `false`.
            #[inline]
            pub fn is_finite(&self) -> bool {
                self.matrix2.is_finite() && self.translation.is_finite()
            }

            /// Returns `true` if any elements are `NaN`.
            #[inline]
            pub fn is_nan(&self) -> bool {
                self.matrix2.is_nan() || self.translation.is_nan()
            }

            /// Returns true if the absolute difference of all elements between `self` and `other`
            /// is less than or equal to `max_abs_diff`.
            ///
            /// This can be used to compare if two 3x4 matrices contain similar elements. It works
            /// best when comparing with a known value. The `max_abs_diff` that should be used used
            /// depends on the values being compared against.
            ///
            /// For more see
            /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
            #[inline]
            pub fn abs_diff_eq(&self, other: Self, max_abs_diff: $t) -> bool {
                self.matrix2.abs_diff_eq(&other.matrix2, max_abs_diff)
                    && self
                        .translation
                        .abs_diff_eq(other.translation, max_abs_diff)
            }

            /// Return the inverse of this transform.
            ///
            /// Note that if the transform is not invertible the result will be invalid.
            #[must_use]
            #[inline]
            pub fn inverse(&self) -> Self {
                let matrix2 = self.matrix2.inverse();
                // transform negative translation by the 2x2 inverse:
                let translation = -(matrix2 * self.translation);

                Self {
                    matrix2,
                    translation,
                }
            }
        }
    };
}

macro_rules! impl_affine2_traits {
    ($t:ty, $mat2:ident, $mat3:ident, $vec2:ident, $affine2:ident, $matrix:ident, $column:ident, $deref:ident) => {
        impl Default for $affine2 {
            #[inline(always)]
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl Deref for $affine2 {
            type Target = $deref;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }

        impl DerefMut for $affine2 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut Self as *mut Self::Target) }
            }
        }

        impl PartialEq for $affine2 {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.matrix2.eq(&other.matrix2) && self.translation.eq(&other.translation)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl core::fmt::Debug for $affine2 {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                fmt.debug_struct(stringify!($affine2))
                    .field("matrix2", &self.matrix2)
                    .field("translation", &self.translation)
                    .finish()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl core::fmt::Display for $affine2 {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
            }
        }

        impl From<$affine2> for $mat3 {
            #[inline]
            fn from(m: $affine2) -> $mat3 {
                Self::from_cols(
                    m.matrix2.x_axis.extend(0.0),
                    m.matrix2.y_axis.extend(0.0),
                    m.translation.extend(1.0),
                )
            }
        }

        impl Mul for $affine2 {
            type Output = $affine2;

            #[inline(always)]
            fn mul(self, other: $affine2) -> Self::Output {
                Self {
                    matrix2: self.matrix2 * other.matrix2,
                    translation: self.matrix2 * other.translation + self.translation,
                }
            }
        }

        impl Mul<$affine2> for $t {
            type Output = $affine2;
            #[inline(always)]
            fn mul(self, other: $affine2) -> Self::Output {
                $affine2 {
                    matrix2: self * other.matrix2,
                    translation: self * other.translation,
                }
            }
        }

        impl Mul<$t> for $affine2 {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: $t) -> Self::Output {
                Self {
                    matrix2: self.matrix2 * other,
                    translation: self.translation * other,
                }
            }
        }

        impl Add<$affine2> for $affine2 {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: Self) -> Self::Output {
                Self {
                    matrix2: self.matrix2 + other.matrix2,
                    translation: self.translation + other.translation,
                }
            }
        }

        impl Sub<$affine2> for $affine2 {
            type Output = Self;
            #[inline(always)]
            fn sub(self, other: Self) -> Self::Output {
                Self {
                    matrix2: self.matrix2 - other.matrix2,
                    translation: self.translation - other.translation,
                }
            }
        }

        impl Mul<$mat3> for $affine2 {
            type Output = $mat3;

            #[inline(always)]
            fn mul(self, other: $mat3) -> Self::Output {
                $mat3::from(self) * other
            }
        }

        impl Mul<$affine2> for $mat3 {
            type Output = $mat3;

            #[inline(always)]
            fn mul(self, other: $affine2) -> Self::Output {
                self * $mat3::from(other)
            }
        }

        impl<'a> core::iter::Product<&'a Self> for $affine2 {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::IDENTITY, |a, &b| a * b)
            }
        }
    };
}

type TransformF32 = Mat2;
type TranslateF32 = Vec2;
type DerefTargetF32 = Columns3<crate::Vec2>;

define_affine2_struct!(Affine2, TransformF32, TranslateF32);
impl_affine2_methods!(f32, Mat2, Mat3, Vec2, Affine2, TransformF32, TranslateF32);
impl_affine2_traits!(
    f32,
    Mat2,
    Mat3,
    Vec2,
    Affine2,
    TransformF32,
    TranslateF32,
    DerefTargetF32
);

impl From<Affine2> for Mat3A {
    #[inline]
    fn from(m: Affine2) -> Mat3A {
        Self::from_cols(
            Vec3A::from((m.matrix2.x_axis, 0.0)),
            Vec3A::from((m.matrix2.y_axis, 0.0)),
            Vec3A::from((m.translation, 1.0)),
        )
    }
}

impl Mul<Mat3A> for Affine2 {
    type Output = Mat3A;

    #[inline(always)]
    fn mul(self, other: Mat3A) -> Self::Output {
        Mat3A::from(self) * other
    }
}

impl Mul<Affine2> for Mat3A {
    type Output = Mat3A;

    #[inline(always)]
    fn mul(self, other: Affine2) -> Self::Output {
        self * Mat3A::from(other)
    }
}

type TransformF64 = DMat2;
type TranslateF64 = DVec2;
type DerefTargetF64 = Columns3<DVec2>;

define_affine2_struct!(DAffine2, TransformF64, TranslateF64);
impl_affine2_methods!(
    f64,
    DMat2,
    DMat3,
    DVec2,
    DAffine2,
    TransformF64,
    TranslateF64
);
impl_affine2_traits!(
    f64,
    DMat2,
    DMat3,
    DVec2,
    DAffine2,
    TransformF64,
    TranslateF64,
    DerefTargetF64
);

#[cfg(all(
    not(feature = "cuda"),
    any(feature = "scalar-math", target_arch = "spirv")
))]
mod const_test_affine2 {
    const_assert_eq!(
        core::mem::align_of::<super::Vec2>(),
        core::mem::align_of::<super::Affine2>()
    );
    const_assert_eq!(24, core::mem::size_of::<super::Affine2>());
}

#[cfg(not(any(feature = "scalar-math", target_arch = "spirv")))]
mod const_test_affine2 {
    const_assert_eq!(16, core::mem::align_of::<super::Affine2>());
    const_assert_eq!(32, core::mem::size_of::<super::Affine2>());
}

mod const_test_daffine2 {
    const_assert_eq!(
        core::mem::align_of::<super::DVec2>(),
        core::mem::align_of::<super::DAffine2>()
    );
    const_assert_eq!(48, core::mem::size_of::<super::DAffine2>());
}
