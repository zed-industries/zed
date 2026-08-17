use crate::core::{
    storage::{Vector3x3, XYZ},
    traits::matrix::{FloatMatrix3x3, Matrix3x3, MatrixConst},
};
use crate::{DMat4, DQuat, DVec2, DVec3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec3Swizzles};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{
    cmp::Ordering,
    ops::{Add, Deref, DerefMut, Mul, Sub},
};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

macro_rules! define_mat3_struct {
    ($mat3:ident, $inner:ident) => {
        /// A 3x3 column major matrix.
        ///
        /// This 3x3 matrix type features convenience methods for creating and using linear and
        /// affine transformations.
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
    ($t:ty, $vec3: ident, $vec2:ident, $quat:ident, $inner:ident) => {
        /// A 3x3 matrix with all elements set to `0.0`.
        pub const ZERO: Self = Self($inner::ZERO);

        /// A 3x3 identity matrix, where all diagonal elements are `1`, and all off-diagonal
        /// elements are `0`.
        pub const IDENTITY: Self = Self($inner::IDENTITY);

        /// Creates a 3x3 matrix with all elements set to `0.0`.
        #[deprecated = "use Mat3::ZERO instead"]
        #[inline(always)]
        pub const fn zero() -> Self {
            Self::ZERO
        }

        /// Creates a 3x3 identity matrix.
        #[deprecated = "use Mat3::IDENTITY instead"]
        #[inline(always)]
        pub const fn identity() -> Self {
            Self::IDENTITY
        }

        /// Creates a 3x3 matrix from three column vectors.
        #[inline(always)]
        pub fn from_cols(x_axis: $vec3, y_axis: $vec3, z_axis: $vec3) -> Self {
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
        #[cfg_attr(docsrs, doc(alias = "scale"))]
        #[inline(always)]
        pub fn from_diagonal(diagonal: $vec3) -> Self {
            Self($inner::from_diagonal(diagonal.0))
        }

        #[inline(always)]
        /// Creates a 3D rotation matrix from the given quaternion.
        pub fn from_quat(rotation: $quat) -> Self {
            // TODO: SIMD?
            Self($inner::from_quaternion(rotation.0.into()))
        }

        /// Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
        /// radians).
        #[inline(always)]
        pub fn from_axis_angle(axis: $vec3, angle: $t) -> Self {
            Self(FloatMatrix3x3::from_axis_angle(axis.0, angle))
        }

        /// Creates a 3D rotation matrix from the given yaw (around y), pitch (around x) and roll
        /// (around z) in radians.
        #[inline(always)]
        pub fn from_rotation_ypr(yaw: $t, pitch: $t, roll: $t) -> Self {
            let quat = $quat::from_rotation_ypr(yaw, pitch, roll);
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
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
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
        #[inline(always)]
        pub fn from_scale(scale: $vec2) -> Self {
            Self(Matrix3x3::from_scale(scale.0))
        }

        /// Returns the matrix column for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 2.
        #[inline]
        pub fn col(&self, index: usize) -> $vec3 {
            match index {
                0 => self.x_axis,
                1 => self.y_axis,
                2 => self.z_axis,
                _ => panic!(
                    "index out of bounds: the len is 3 but the index is {}",
                    index
                ),
            }
        }

        /// Returns the matrix row for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 2.
        #[inline]
        pub fn row(&self, index: usize) -> $vec3 {
            match index {
                0 => $vec3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
                1 => $vec3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
                2 => $vec3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
                _ => panic!(
                    "index out of bounds: the len is 3 but the index is {}",
                    index
                ),
            }
        }

        // #[inline]
        // pub(crate) fn col_mut(&mut self, index: usize) -> &mut $vec3 {
        //     match index {
        //         0 => &mut self.x_axis,
        //         1 => &mut self.y_axis,
        //         2 => &mut self.z_axis,
        //         _ => panic!(
        //             "index out of bounds: the len is 3 but the index is {}",
        //             index
        //         ),
        //     }
        // }

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
        #[inline(always)]
        pub fn transpose(&self) -> Self {
            Self(self.0.transpose())
            // {
            //     #[cfg(target_arch = "x86")]
            //     use core::arch::x86::*;
            //     #[cfg(target_arch = "x86_64")]
            //     use core::arch::x86_64::*;
            //     unsafe {
            //         let tmp0 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b01_00_01_00);
            //         let tmp1 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b11_10_11_10);

            //         Self {
            //             x_axis: _mm_shuffle_ps(tmp0, self.z_axis.0, 0b00_00_10_00).into(),
            //             y_axis: _mm_shuffle_ps(tmp0, self.z_axis.0, 0b01_01_11_01).into(),
            //             z_axis: _mm_shuffle_ps(tmp1, self.z_axis.0, 0b10_10_10_00).into(),
            //         }
            //     }
            // }
        }

        /// Returns the determinant of `self`.
        #[inline(always)]
        pub fn determinant(&self) -> $t {
            self.0.determinant()
        }

        /// Returns the inverse of `self`.
        ///
        /// If the matrix is not invertible the returned matrix will be invalid.
        #[inline(always)]
        pub fn inverse(&self) -> Self {
            Self(self.0.inverse())
        }

        /// Transforms a 3D vector.
        #[inline(always)]
        pub fn mul_vec3(&self, other: $vec3) -> $vec3 {
            self.mul_vec3_as_vec3a(other)
        }

        /// Multiplies two 3x3 matrices.
        #[inline]
        pub fn mul_mat3(&self, other: &Self) -> Self {
            Self::from_cols(
                self.mul_vec3(other.x_axis),
                self.mul_vec3(other.y_axis),
                self.mul_vec3(other.z_axis),
            )
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
            self.transform_point2_as_vec3a(other)
        }

        /// Rotates the given 2D vector.
        ///
        /// This is the equivalent of multiplying `other` as a 3D vector where `z` is `0`.
        ///
        /// This method assumes that `self` contains a valid affine transform.
        #[inline(always)]
        pub fn transform_vector2(&self, other: $vec2) -> $vec2 {
            self.transform_vector2_as_vec3a(other)
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
    ($t:ty, $new:ident, $mat3:ident, $mat4:ident, $vec3: ident) => {
        /// Creates a 3x3 matrix from three column vectors.
        #[inline(always)]
        pub fn $new(x_axis: $vec3, y_axis: $vec3, z_axis: $vec3) -> $mat3 {
            $mat3::from_cols(x_axis, y_axis, z_axis)
        }

        impl Default for $mat3 {
            #[inline(always)]
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl PartialEq for $mat3 {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.x_axis.eq(&other.x_axis)
                    && self.y_axis.eq(&other.y_axis)
                    && self.z_axis.eq(&other.z_axis)
            }
        }

        impl PartialOrd for $mat3 {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.as_ref().partial_cmp(other.as_ref())
            }
        }

        impl Deref for $mat3 {
            type Target = Vector3x3<$vec3>;
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
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $mat3 {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.debug_struct("$mat3")
                    .field("x_axis", &self.x_axis)
                    .field("y_axis", &self.y_axis)
                    .field("z_axis", &self.z_axis)
                    .finish()
            }
        }
        impl AsRef<[$t; 9]> for $mat3 {
            #[inline(always)]
            fn as_ref(&self) -> &[$t; 9] {
                unsafe { &*(self as *const Self as *const [$t; 9]) }
            }
        }

        impl AsMut<[$t; 9]> for $mat3 {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut [$t; 9] {
                unsafe { &mut *(self as *mut Self as *mut [$t; 9]) }
            }
        }

        impl Add<$mat3> for $mat3 {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: Self) -> Self {
                self.add_mat3(&other)
            }
        }

        impl Sub<$mat3> for $mat3 {
            type Output = Self;
            #[inline(always)]
            fn sub(self, other: Self) -> Self {
                self.sub_mat3(&other)
            }
        }

        impl Mul<$mat3> for $mat3 {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: Self) -> Self {
                self.mul_mat3(&other)
            }
        }

        impl Mul<$vec3> for $mat3 {
            type Output = $vec3;
            #[inline(always)]
            fn mul(self, other: $vec3) -> $vec3 {
                self.mul_vec3(other)
            }
        }

        impl Mul<$mat3> for $t {
            type Output = $mat3;
            #[inline(always)]
            fn mul(self, other: $mat3) -> $mat3 {
                other.mul_scalar(self)
            }
        }

        impl Mul<$t> for $mat3 {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: $t) -> Self {
                self.mul_scalar(other)
            }
        }

        impl From<$mat4> for $mat3 {
            /// Creates a 3x3 matrix from the top left submatrix of the given 4x4 matrix.
            fn from(m: $mat4) -> $mat3 {
                $mat3::from_cols(m.x_axis.into(), m.y_axis.into(), m.z_axis.into())
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Sum<&'a Self> for $mat3 {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold($mat3::ZERO, |a, &b| Self::add(a, b))
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Product<&'a Self> for $mat3 {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold($mat3::IDENTITY, |a, &b| Self::mul(a, b))
            }
        }
    };
}

type InnerF32 = Vector3x3<XYZ<f32>>;
define_mat3_struct!(Mat3, InnerF32);

impl Mat3 {
    impl_mat3_methods!(f32, Vec3, Vec2, Quat, InnerF32);

    /// Transforms a `Vec3A`.
    #[inline]
    pub fn mul_vec3a(&self, other: Vec3A) -> Vec3A {
        let mut res = Vec3A::from(self.x_axis) * other.xxx();
        res = Vec3A::from(self.y_axis).mul_add(other.yyy(), res);
        res = Vec3A::from(self.z_axis).mul_add(other.zzz(), res);
        res
    }

    /// Transforms a `Vec3`.
    #[inline(always)]
    fn mul_vec3_as_vec3a(&self, other: Vec3) -> Vec3 {
        Vec3::from(self.mul_vec3a(Vec3A::from(other)))
    }

    #[inline]
    fn transform_point2_as_vec3a(&self, other: Vec2) -> Vec2 {
        let mut res = Vec3A::from(self.x_axis).mul(Vec3A::splat(other.x));
        res = Vec3A::from(self.y_axis).mul_add(Vec3A::splat(other.y), res);
        res = Vec3A::from(self.z_axis).add(res);
        res.xy()
    }

    #[inline]
    fn transform_vector2_as_vec3a(&self, other: Vec2) -> Vec2 {
        let mut res = Vec3A::from(self.x_axis).mul(Vec3A::splat(other.x));
        res = Vec3A::from(self.y_axis).mul_add(Vec3A::splat(other.y), res);
        res.xy()
    }

    #[inline(always)]
    pub fn as_f64(&self) -> DMat3 {
        DMat3::from_cols(
            self.x_axis.as_f64(),
            self.y_axis.as_f64(),
            self.z_axis.as_f64(),
        )
    }
}
impl_mat3_traits!(f32, mat3, Mat3, Mat4, Vec3);

impl Mul<Vec3A> for Mat3 {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, other: Vec3A) -> Vec3A {
        self.mul_vec3a(other)
    }
}

type InnerF64 = Vector3x3<XYZ<f64>>;
define_mat3_struct!(DMat3, InnerF64);

impl DMat3 {
    impl_mat3_methods!(f64, DVec3, DVec2, DQuat, InnerF64);

    #[inline(always)]
    fn mul_vec3_as_vec3a(&self, other: DVec3) -> DVec3 {
        DVec3(self.0.mul_vector(other.0))
    }

    #[inline(always)]
    fn transform_point2_as_vec3a(&self, other: DVec2) -> DVec2 {
        DVec2(self.0.transform_point2(other.0))
    }

    #[inline(always)]
    fn transform_vector2_as_vec3a(&self, other: DVec2) -> DVec2 {
        DVec2(self.0.transform_vector2(other.0))
    }

    #[inline(always)]
    pub fn as_f32(&self) -> Mat3 {
        Mat3::from_cols(
            self.x_axis.as_f32(),
            self.y_axis.as_f32(),
            self.z_axis.as_f32(),
        )
    }
}
impl_mat3_traits!(f64, dmat3, DMat3, DMat4, DVec3);
