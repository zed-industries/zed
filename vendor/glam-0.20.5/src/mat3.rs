use crate::core::{
    storage::{Columns3, XYZ},
    traits::matrix::{FloatMatrix3x3, Matrix3x3, MatrixConst},
};
use crate::{DMat2, DMat4, DQuat, DVec2, DVec3, EulerRot, Mat2, Mat4, Quat, Vec2, Vec3, Vec3A};
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

macro_rules! define_mat3_struct {
    ($mat3:ident, $inner:ident) => {
        /// A 3x3 column major matrix.
        ///
        /// This 3x3 matrix type features convenience methods for creating and using linear and
        /// affine transformations. If you are primarily dealing with 2D affine transformations the
        /// [`Affine2`](crate::Affine2) type is much faster and more space efficient than using a
        /// 3x3 matrix.
        ///
        /// Linear transformations including 3D rotation and scale can be created using methods
        /// such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
        /// [`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
        /// [`Self::from_rotation_z()`].
        ///
        /// The resulting matrices can be use to transform 3D vectors using regular vector
        /// multiplication.
        ///
        /// Affine transformations including 2D translation, rotation and scale can be created
        /// using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
        /// [`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
        ///
        /// The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
        /// are provided for performing affine transforms on 2D vectors and points. These multiply
        /// 2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
        /// vectors respectively. These methods assume that `Self` contains a valid affine
        /// transform.
        #[derive(Clone, Copy)]
        #[cfg_attr(not(target_arch = "spirv"), repr(C))]
        pub struct $mat3(pub(crate) $inner);
    };
}

macro_rules! impl_mat3_methods {
    ($t:ty, $vec3:ident, $vec3a:ident, $vec2:ident, $quat:ident, $mat2:ident, $mat4:ident, $inner:ident) => {
        /// A 3x3 matrix with all elements set to `0.0`.
        pub const ZERO: Self = Self($inner::ZERO);

        /// A 3x3 identity matrix, where all diagonal elements are `1`, and all off-diagonal
        /// elements are `0`.
        pub const IDENTITY: Self = Self($inner::IDENTITY);

        /// All NAN:s.
        pub const NAN: Self = Self(<$inner as crate::core::traits::scalar::NanConstEx>::NAN);

        /// Creates a 3x3 matrix from three column vectors.
        #[inline(always)]
        pub fn from_cols(x_axis: $vec3a, y_axis: $vec3a, z_axis: $vec3a) -> Self {
            Self(Matrix3x3::from_cols(x_axis.0, y_axis.0, z_axis.0))
        }

        /// Creates a 3x3 matrix from a `[S; 9]` array stored in column major order.
        /// If your data is stored in row major you will need to `transpose` the returned
        /// matrix.
        #[inline(always)]
        pub fn from_cols_array(m: &[$t; 9]) -> Self {
            Self(Matrix3x3::from_cols_array(m))
        }

        /// Creates a `[S; 9]` array storing data in column major order.
        /// If you require data in row major order `transpose` the matrix first.
        #[inline(always)]
        pub fn to_cols_array(&self) -> [$t; 9] {
            self.0.to_cols_array()
        }

        /// Creates a 3x3 matrix from a `[[S; 3]; 3]` 2D array stored in column major order.
        /// If your data is in row major order you will need to `transpose` the returned
        /// matrix.
        #[inline(always)]
        pub fn from_cols_array_2d(m: &[[$t; 3]; 3]) -> Self {
            Self(Matrix3x3::from_cols_array_2d(m))
        }

        /// Creates a `[[S; 3]; 3]` 2D array storing data in column major order.
        /// If you require data in row major order `transpose` the matrix first.
        #[inline(always)]
        pub fn to_cols_array_2d(&self) -> [[$t; 3]; 3] {
            self.0.to_cols_array_2d()
        }

        /// Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
        /// The resulting matrix is a 3D scale transfom.
        #[doc(alias = "scale")]
        #[inline(always)]
        pub fn from_diagonal(diagonal: $vec3) -> Self {
            Self($inner::from_diagonal(diagonal.0))
        }

        /// Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
        pub fn from_mat4(m: $mat4) -> Self {
            Self::from_cols(
                $vec3a(m.x_axis.0.into()),
                $vec3a(m.y_axis.0.into()),
                $vec3a(m.z_axis.0.into()),
            )
        }

        /// Creates a 3D rotation matrix from the given quaternion.
        ///
        /// # Panics
        ///
        /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn from_quat(rotation: $quat) -> Self {
            // TODO: SIMD?
            Self($inner::from_quaternion(rotation.0.into()))
        }

        /// Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
        /// radians).
        ///
        /// # Panics
        ///
        /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn from_axis_angle(axis: $vec3, angle: $t) -> Self {
            Self(FloatMatrix3x3::from_axis_angle(axis.0, angle))
        }

        #[inline(always)]
        /// Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
        /// radians).
        pub fn from_euler(order: EulerRot, a: $t, b: $t, c: $t) -> Self {
            let quat = $quat::from_euler(order, a, b, c);
            Self::from_quat(quat)
        }

        /// Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
        #[inline(always)]
        pub fn from_rotation_x(angle: $t) -> Self {
            Self($inner::from_rotation_x(angle))
        }

        /// Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
        #[inline(always)]
        pub fn from_rotation_y(angle: $t) -> Self {
            Self($inner::from_rotation_y(angle))
        }

        /// Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
        #[inline(always)]
        pub fn from_rotation_z(angle: $t) -> Self {
            Self($inner::from_rotation_z(angle))
        }

        /// Creates an affine transformation matrix from the given 2D `translation`.
        ///
        /// The resulting matrix can be used to transform 2D points and vectors. See
        /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
        #[inline(always)]
        pub fn from_translation(translation: $vec2) -> Self {
            Self(Matrix3x3::from_translation(translation.0))
        }

        /// Creates an affine transformation matrix from the given 2D rotation `angle` (in
        /// radians).
        ///
        /// The resulting matrix can be used to transform 2D points and vectors. See
        /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
        #[inline(always)]
        pub fn from_angle(angle: $t) -> Self {
            Self(FloatMatrix3x3::from_angle(angle))
        }

        /// Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
        /// radians) and `translation`.
        ///
        /// The resulting matrix can be used to transform 2D points and vectors. See
        /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
        #[inline(always)]
        pub fn from_scale_angle_translation(scale: $vec2, angle: $t, translation: $vec2) -> Self {
            Self(FloatMatrix3x3::from_scale_angle_translation(
                scale.0,
                angle,
                translation.0,
            ))
        }

        /// Creates an affine transformation matrix from the given non-uniform 2D `scale`.
        ///
        /// The resulting matrix can be used to transform 2D points and vectors. See
        /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
        ///
        /// # Panics
        ///
        /// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
        #[inline(always)]
        pub fn from_scale(scale: $vec2) -> Self {
            Self(Matrix3x3::from_scale(scale.0))
        }

        /// Creates an affine transformation matrix from the given 2x2 matrix.
        ///
        /// The resulting matrix can be used to transform 2D points and vectors. See
        /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
        #[inline(always)]
        pub fn from_mat2(m: $mat2) -> Self {
            Self::from_cols((m.x_axis, 0.0).into(), (m.y_axis, 0.0).into(), $vec3a::Z)
        }

        /// Creates a 3x3 matrix from the first 9 values in `slice`.
        ///
        /// # Panics
        ///
        /// Panics if `slice` is less than 9 elements long.
        #[inline(always)]
        pub fn from_cols_slice(slice: &[$t]) -> Self {
            Self(Matrix3x3::from_cols_slice(slice))
        }

        /// Writes the columns of `self` to the first 9 elements in `slice`.
        ///
        /// # Panics
        ///
        /// Panics if `slice` is less than 9 elements long.
        #[inline(always)]
        pub fn write_cols_to_slice(self, slice: &mut [$t]) {
            Matrix3x3::write_cols_to_slice(&self.0, slice)
        }

        /// Returns the matrix column for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 2.
        #[inline]
        pub fn col(&self, index: usize) -> $vec3a {
            match index {
                0 => self.x_axis,
                1 => self.y_axis,
                2 => self.z_axis,
                _ => panic!("index out of bounds"),
            }
        }

        /// Returns a mutable reference to the matrix column for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 2.
        #[inline]
        pub fn col_mut(&mut self, index: usize) -> &mut $vec3a {
            match index {
                0 => &mut self.x_axis,
                1 => &mut self.y_axis,
                2 => &mut self.z_axis,
                _ => panic!("index out of bounds"),
            }
        }

        /// Returns the matrix row for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 2.
        #[inline]
        pub fn row(&self, index: usize) -> $vec3a {
            match index {
                0 => $vec3a::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
                1 => $vec3a::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
                2 => $vec3a::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
                _ => panic!("index out of bounds"),
            }
        }

        /// Returns `true` if, and only if, all elements are finite.
        /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
        #[inline]
        pub fn is_finite(&self) -> bool {
            self.x_axis.is_finite() && self.y_axis.is_finite() && self.z_axis.is_finite()
        }

        /// Returns `true` if any elements are `NaN`.
        #[inline]
        pub fn is_nan(&self) -> bool {
            self.x_axis.is_nan() || self.y_axis.is_nan() || self.z_axis.is_nan()
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

        /// Transforms a 3D vector.
        #[inline(always)]
        pub fn mul_vec3(&self, other: $vec3) -> $vec3 {
            $vec3(self.0.mul_vector(other.0.into()).into())
        }

        /// Multiplies two 3x3 matrices.
        #[inline]
        pub fn mul_mat3(&self, other: &Self) -> Self {
            Self(self.0.mul_matrix(&other.0))
        }

        /// Adds two 3x3 matrices.
        #[inline(always)]
        pub fn add_mat3(&self, other: &Self) -> Self {
            Self(self.0.add_matrix(&other.0))
        }

        /// Subtracts two 3x3 matrices.
        #[inline(always)]
        pub fn sub_mat3(&self, other: &Self) -> Self {
            Self(self.0.sub_matrix(&other.0))
        }

        /// Multiplies a 3x3 matrix by a scalar.
        #[inline(always)]
        pub fn mul_scalar(&self, other: $t) -> Self {
            Self(self.0.mul_scalar(other))
        }

        /// Transforms the given 2D vector as a point.
        ///
        /// This is the equivalent of multiplying `other` as a 3D vector where `z` is `1`.
        ///
        /// This method assumes that `self` contains a valid affine transform.
        #[inline(always)]
        pub fn transform_point2(&self, other: $vec2) -> $vec2 {
            $mat2::from_cols($vec2(self.x_axis.0.into()), $vec2(self.y_axis.0.into())) * other
                + $vec2(self.z_axis.0.into())
        }

        /// Rotates the given 2D vector.
        ///
        /// This is the equivalent of multiplying `other` as a 3D vector where `z` is `0`.
        ///
        /// This method assumes that `self` contains a valid affine transform.
        #[inline(always)]
        pub fn transform_vector2(&self, other: $vec2) -> $vec2 {
            $mat2::from_cols($vec2(self.x_axis.0.into()), $vec2(self.y_axis.0.into())) * other
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
        pub fn abs_diff_eq(&self, other: Self, max_abs_diff: $t) -> bool {
            self.0.abs_diff_eq(&other.0, max_abs_diff)
        }
    };
}

macro_rules! impl_mat3_traits {
    ($t:ty, $new:ident, $mat3:ident, $vec3:ident, $vec3a:ident) => {
        /// Creates a 3x3 matrix from three column vectors.
        #[inline(always)]
        pub fn $new(x_axis: $vec3a, y_axis: $vec3a, z_axis: $vec3a) -> $mat3 {
            $mat3::from_cols(x_axis, y_axis, z_axis)
        }

        impl_matn_common_traits!($t, $mat3, $vec3a);

        impl PartialEq for $mat3 {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.x_axis.eq(&other.x_axis)
                    && self.y_axis.eq(&other.y_axis)
                    && self.z_axis.eq(&other.z_axis)
            }
        }

        impl Deref for $mat3 {
            type Target = Columns3<$vec3a>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }

        impl DerefMut for $mat3 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut Self as *mut Self::Target) }
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $mat3 {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $mat3 {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("$mat3")
                    .field("x_axis", &self.x_axis)
                    .field("y_axis", &self.y_axis)
                    .field("z_axis", &self.z_axis)
                    .finish()
            }
        }
    };
}

macro_rules! impl_mat3_traits_unsafe {
    ($t:ty, $mat3:ident) => {
        #[cfg(not(target_arch = "spirv"))]
        impl AsRef<[$t; 9]> for $mat3 {
            #[inline(always)]
            fn as_ref(&self) -> &[$t; 9] {
                unsafe { &*(self as *const Self as *const [$t; 9]) }
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsMut<[$t; 9]> for $mat3 {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut [$t; 9] {
                unsafe { &mut *(self as *mut Self as *mut [$t; 9]) }
            }
        }
    };
}

type InnerF32 = Columns3<XYZ<f32>>;
define_mat3_struct!(Mat3, InnerF32);

impl Mat3 {
    impl_mat3_methods!(f32, Vec3, Vec3, Vec2, Quat, Mat2, Mat4, InnerF32);

    /// Transforms a `Vec3A`.
    #[inline]
    pub fn mul_vec3a(&self, other: Vec3A) -> Vec3A {
        self.mul_vec3(other.into()).into()
    }

    #[inline(always)]
    pub fn as_dmat3(&self) -> DMat3 {
        DMat3::from_cols(
            self.x_axis.as_dvec3(),
            self.y_axis.as_dvec3(),
            self.z_axis.as_dvec3(),
        )
    }
}
impl_mat3_traits!(f32, mat3, Mat3, Vec3, Vec3);
impl_mat3_traits_unsafe!(f32, Mat3);

impl Mul<Vec3A> for Mat3 {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, other: Vec3A) -> Vec3A {
        self.mul_vec3a(other)
    }
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type InnerF32A = Columns3<__m128>;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
type InnerF32A = Columns3<v128>;

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
type InnerF32A = Columns3<crate::core::storage::XYZF32A16>;
define_mat3_struct!(Mat3A, InnerF32A);

impl Mat3A {
    impl_mat3_methods!(f32, Vec3, Vec3A, Vec2, Quat, Mat2, Mat4, InnerF32A);

    /// Transforms a `Vec3A`.
    #[inline]
    pub fn mul_vec3a(&self, other: Vec3A) -> Vec3A {
        Vec3A(self.0.mul_vector(other.0))
    }

    #[inline(always)]
    pub fn as_dmat3(&self) -> DMat3 {
        DMat3::from_cols(
            self.x_axis.as_dvec3(),
            self.y_axis.as_dvec3(),
            self.z_axis.as_dvec3(),
        )
    }
}
impl_mat3_traits!(f32, mat3a, Mat3A, Vec3, Vec3A);

impl Mul<Vec3> for Mat3A {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        self.mul_vec3(other)
    }
}

impl From<Mat3> for Mat3A {
    #[inline(always)]
    fn from(m: Mat3) -> Self {
        Self(m.0.into())
    }
}

impl From<Mat3A> for Mat3 {
    #[inline(always)]
    fn from(m: Mat3A) -> Self {
        Self(m.0.into())
    }
}

type InnerF64 = Columns3<XYZ<f64>>;
define_mat3_struct!(DMat3, InnerF64);

impl DMat3 {
    impl_mat3_methods!(f64, DVec3, DVec3, DVec2, DQuat, DMat2, DMat4, InnerF64);

    #[inline(always)]
    pub fn as_mat3(&self) -> Mat3 {
        Mat3::from_cols(
            self.x_axis.as_vec3(),
            self.y_axis.as_vec3(),
            self.z_axis.as_vec3(),
        )
    }
}
impl_mat3_traits!(f64, dmat3, DMat3, DVec3, DVec3);
impl_mat3_traits_unsafe!(f64, DMat3);

mod const_test_mat3 {
    const_assert_eq!(
        core::mem::align_of::<f32>(),
        core::mem::align_of::<super::Mat3>()
    );
    const_assert_eq!(36, core::mem::size_of::<super::Mat3>());
}

mod const_test_mat3a {
    const_assert_eq!(16, core::mem::align_of::<super::Mat3A>());
    const_assert_eq!(48, core::mem::size_of::<super::Mat3A>());
}

mod const_test_dmat3 {
    const_assert_eq!(
        core::mem::align_of::<f64>(),
        core::mem::align_of::<super::DMat3>()
    );
    const_assert_eq!(72, core::mem::size_of::<super::DMat3>());
}
