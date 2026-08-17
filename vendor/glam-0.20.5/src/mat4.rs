use crate::core::{
    storage::{Columns4, XYZW},
    traits::{
        matrix::{FloatMatrix4x4, Matrix4x4, MatrixConst},
        projection::ProjectionMatrix,
    },
};
use crate::{DMat3, DQuat, DVec3, DVec4, EulerRot, Mat3, Quat, Vec3, Vec3A, Vec4};

#[cfg(all(
    target_feature = "sse2",
    not(feature = "scalar-math"),
    target_arch = "x86"
))]
use core::arch::x86::*;
#[cfg(all(
    target_feature = "sse2",
    not(feature = "scalar-math"),
    target_arch = "x86_64"
))]
use core::arch::x86_64::*;

#[cfg(target_feature = "simd128")]
use core::arch::wasm32::v128;

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign};

//macro_rules! define_mat4_struct {
//    ($mat4:ident, $inner:ident) => {
//        /// A 4x4 column major matrix.
//        ///
//        /// This 4x4 matrix type features convenience methods for creating and using affine
//        /// transforms and perspective projections.
//        ///
//        /// Affine transformations including 3D translation, rotation and scale can be created
//        /// using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
//        /// [`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
//        ///
//        /// Othographic projections can be created using the methods [`Self::orthographic_lh()`] for
//        /// left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
//        /// systems. The resulting matrix is also an affine transformation.
//        ///
//        /// The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
//        /// are provided for performing affine transformations on 3D vectors and points. These
//        /// multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
//        /// for vectors respectively. These methods assume that `Self` contains a valid affine
//        /// transform.
//        ///
//        /// Perspective projections can be created using methods such as
//        /// [`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
//        /// [`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
//        /// [`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
//        /// [`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
//        ///
//        /// The resulting perspective project can be use to transform 3D vectors as points with
//        /// perspective correction using the [`Self::project_point3()`] convenience method.
//        #[derive(Clone, Copy)]
//        #[repr(transparent)]
//        pub struct $mat4(pub(crate) $inner);
//    };
//}

macro_rules! impl_mat4_methods {
    ($t:ident, $vec4:ident, $vec3:ident, $mat3:ident, $quat:ident, $inner:ident) => {
        /// A 4x4 matrix with all elements set to `0.0`.
        pub const ZERO: Self = Self($inner::ZERO);

        /// A 4x4 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
        pub const IDENTITY: Self = Self($inner::IDENTITY);

        /// All NAN:s.
        pub const NAN: Self = Self(<$inner as crate::core::traits::scalar::NanConstEx>::NAN);

        /// Creates a 4x4 matrix from four column vectors.
        #[inline(always)]
        pub fn from_cols(x_axis: $vec4, y_axis: $vec4, z_axis: $vec4, w_axis: $vec4) -> Self {
            Self($inner::from_cols(x_axis.0, y_axis.0, z_axis.0, w_axis.0))
        }

        /// Creates a 4x4 matrix from a `[S; 16]` array stored in column major order.
        /// If your data is stored in row major you will need to `transpose` the returned
        /// matrix.
        #[inline(always)]
        pub fn from_cols_array(m: &[$t; 16]) -> Self {
            Self($inner::from_cols_array(m))
        }

        /// Creates a `[S; 16]` array storing data in column major order.
        /// If you require data in row major order `transpose` the matrix first.
        #[inline(always)]
        pub fn to_cols_array(&self) -> [$t; 16] {
            self.0.to_cols_array()
        }

        /// Creates a 4x4 matrix from a `[[S; 4]; 4]` 2D array stored in column major order.
        /// If your data is in row major order you will need to `transpose` the returned
        /// matrix.
        #[inline(always)]
        pub fn from_cols_array_2d(m: &[[$t; 4]; 4]) -> Self {
            Self($inner::from_cols_array_2d(m))
        }

        /// Creates a `[[S; 4]; 4]` 2D array storing data in column major order.
        /// If you require data in row major order `transpose` the matrix first.
        #[inline(always)]
        pub fn to_cols_array_2d(&self) -> [[$t; 4]; 4] {
            self.0.to_cols_array_2d()
        }

        /// Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
        #[doc(alias = "scale")]
        #[inline(always)]
        pub fn from_diagonal(diagonal: $vec4) -> Self {
            Self($inner::from_diagonal(diagonal.0.into()))
        }

        /// Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
        /// `translation`.
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        ///
        /// # Panics
        ///
        /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn from_scale_rotation_translation(
            scale: $vec3,
            rotation: $quat,
            translation: $vec3,
        ) -> Self {
            Self($inner::from_scale_quaternion_translation(
                scale.0,
                rotation.0,
                translation.0,
            ))
        }

        /// Creates an affine transformation matrix from the given 3D `translation`.
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        ///
        /// # Panics
        ///
        /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn from_rotation_translation(rotation: $quat, translation: $vec3) -> Self {
            Self($inner::from_quaternion_translation(
                rotation.0,
                translation.0,
            ))
        }

        /// Extracts `scale`, `rotation` and `translation` from `self`. The input matrix is
        /// expected to be a 3D affine transformation matrix otherwise the output will be invalid.
        ///
        /// # Panics
        ///
        /// Will panic if the determinant of `self` is zero or if the resulting scale vector
        /// contains any zero elements when `glam_assert` is enabled.
        #[inline(always)]
        pub fn to_scale_rotation_translation(&self) -> ($vec3, $quat, $vec3) {
            let (scale, rotation, translation) = self.0.to_scale_quaternion_translation();
            ($vec3(scale), $quat(rotation), $vec3(translation))
        }

        /// Creates an affine transformation matrix from the given `rotation` quaternion.
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        ///
        /// # Panics
        ///
        /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn from_quat(rotation: $quat) -> Self {
            Self($inner::from_quaternion(rotation.0))
        }

        /// Creates an affine transformation matrix from the given 3x3 linear transformation
        /// matrix.
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        #[inline(always)]
        pub fn from_mat3(m: $mat3) -> Self {
            Self::from_cols(
                (m.x_axis, 0.0).into(),
                (m.y_axis, 0.0).into(),
                (m.z_axis, 0.0).into(),
                $vec4::W,
            )
        }

        /// Creates an affine transformation matrix from the given 3D `translation`.
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        #[inline(always)]
        pub fn from_translation(translation: $vec3) -> Self {
            Self($inner::from_translation(translation.0))
        }

        /// Creates an affine transformation matrix containing a 3D rotation around a normalized
        /// rotation `axis` of `angle` (in radians).
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        ///
        /// # Panics
        ///
        /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn from_axis_angle(axis: $vec3, angle: $t) -> Self {
            Self($inner::from_axis_angle(axis.0, angle))
        }

        #[inline(always)]
        /// Creates a affine transformation matrix containing a rotation from the given euler
        /// rotation sequence and angles (in radians).
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        pub fn from_euler(order: EulerRot, a: $t, b: $t, c: $t) -> Self {
            let quat = $quat::from_euler(order, a, b, c);
            Self::from_quat(quat)
        }

        /// Creates an affine transformation matrix containing a 3D rotation around the x axis of
        /// `angle` (in radians).
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        #[inline(always)]
        pub fn from_rotation_x(angle: $t) -> Self {
            Self($inner::from_rotation_x(angle))
        }

        /// Creates an affine transformation matrix containing a 3D rotation around the y axis of
        /// `angle` (in radians).
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        #[inline(always)]
        pub fn from_rotation_y(angle: $t) -> Self {
            Self($inner::from_rotation_y(angle))
        }

        /// Creates an affine transformation matrix containing a 3D rotation around the z axis of
        /// `angle` (in radians).
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        #[inline(always)]
        pub fn from_rotation_z(angle: $t) -> Self {
            Self($inner::from_rotation_z(angle))
        }

        /// Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
        ///
        /// The resulting matrix can be used to transform 3D points and vectors. See
        /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
        ///
        /// # Panics
        ///
        /// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
        #[inline(always)]
        pub fn from_scale(scale: $vec3) -> Self {
            Self($inner::from_scale(scale.0))
        }

        /// Creates a 4x4 matrix from the first 16 values in `slice`.
        ///
        /// # Panics
        ///
        /// Panics if `slice` is less than 16 elements long.
        #[inline(always)]
        pub fn from_cols_slice(slice: &[$t]) -> Self {
            Self(Matrix4x4::from_cols_slice(slice))
        }

        /// Writes the columns of `self` to the first 16 elements in `slice`.
        ///
        /// # Panics
        ///
        /// Panics if `slice` is less than 16 elements long.
        #[inline(always)]
        pub fn write_cols_to_slice(self, slice: &mut [$t]) {
            Matrix4x4::write_cols_to_slice(&self.0, slice)
        }

        /// Returns the matrix column for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 3.
        #[inline]
        pub fn col(&self, index: usize) -> $vec4 {
            match index {
                0 => self.x_axis,
                1 => self.y_axis,
                2 => self.z_axis,
                3 => self.w_axis,
                _ => panic!("index out of bounds"),
            }
        }

        /// Returns a mutable reference to the matrix column for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 3.
        #[inline]
        pub fn col_mut(&mut self, index: usize) -> &mut $vec4 {
            match index {
                0 => &mut self.x_axis,
                1 => &mut self.y_axis,
                2 => &mut self.z_axis,
                3 => &mut self.w_axis,
                _ => panic!("index out of bounds"),
            }
        }

        /// Returns the matrix row for the given `index`.
        ///
        /// # Panics
        ///
        /// Panics if `index` is greater than 3.
        #[inline]
        pub fn row(&self, index: usize) -> $vec4 {
            match index {
                0 => $vec4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
                1 => $vec4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
                2 => $vec4::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
                3 => $vec4::new(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
                _ => panic!("index out of bounds"),
            }
        }

        /// Returns `true` if, and only if, all elements are finite.
        /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
        #[inline]
        pub fn is_finite(&self) -> bool {
            self.x_axis.is_finite()
                && self.y_axis.is_finite()
                && self.z_axis.is_finite()
                && self.w_axis.is_finite()
        }

        /// Returns `true` if any elements are `NaN`.
        #[inline]
        pub fn is_nan(&self) -> bool {
            self.x_axis.is_nan()
                || self.y_axis.is_nan()
                || self.z_axis.is_nan()
                || self.w_axis.is_nan()
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

        /// Creates a left-handed view matrix using a camera position, an up direction, and a focal
        /// point.
        /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
        ///
        /// # Panics
        ///
        /// Will panic if `up` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn look_at_lh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
            Self($inner::look_at_lh(eye.0, center.0, up.0))
        }

        /// Creates a right-handed view matrix using a camera position, an up direction, and a focal
        /// point.
        /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        ///
        /// # Panics
        ///
        /// Will panic if `up` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn look_at_rh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
            Self($inner::look_at_rh(eye.0, center.0, up.0))
        }

        /// Creates a right-handed perspective projection matrix with [-1,1] depth range.
        /// This is the same as the OpenGL `gluPerspective` function.
        /// See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
        #[inline(always)]
        pub fn perspective_rh_gl(
            fov_y_radians: $t,
            aspect_ratio: $t,
            z_near: $t,
            z_far: $t,
        ) -> Self {
            Self($inner::perspective_rh_gl(
                fov_y_radians,
                aspect_ratio,
                z_near,
                z_far,
            ))
        }

        /// Creates a left-handed perspective projection matrix with `[0,1]` depth range.
        ///
        /// # Panics
        ///
        /// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        /// enabled.
        #[inline(always)]
        pub fn perspective_lh(fov_y_radians: $t, aspect_ratio: $t, z_near: $t, z_far: $t) -> Self {
            Self($inner::perspective_lh(
                fov_y_radians,
                aspect_ratio,
                z_near,
                z_far,
            ))
        }

        /// Creates a right-handed perspective projection matrix with `[0,1]` depth range.
        ///
        /// # Panics
        ///
        /// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        /// enabled.
        #[inline(always)]
        pub fn perspective_rh(fov_y_radians: $t, aspect_ratio: $t, z_near: $t, z_far: $t) -> Self {
            Self($inner::perspective_rh(
                fov_y_radians,
                aspect_ratio,
                z_near,
                z_far,
            ))
        }

        /// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        ///
        /// # Panics
        ///
        /// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
        #[inline(always)]
        pub fn perspective_infinite_lh(fov_y_radians: $t, aspect_ratio: $t, z_near: $t) -> Self {
            Self($inner::perspective_infinite_lh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }

        /// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        ///
        /// # Panics
        ///
        /// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
        #[inline(always)]
        pub fn perspective_infinite_reverse_lh(
            fov_y_radians: $t,
            aspect_ratio: $t,
            z_near: $t,
        ) -> Self {
            Self($inner::perspective_infinite_reverse_lh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }

        /// Creates an infinite right-handed perspective projection matrix with
        /// `[0,1]` depth range.
        #[inline(always)]
        pub fn perspective_infinite_rh(fov_y_radians: $t, aspect_ratio: $t, z_near: $t) -> Self {
            Self($inner::perspective_infinite_rh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }

        /// Creates an infinite reverse right-handed perspective projection matrix
        /// with `[0,1]` depth range.
        #[inline(always)]
        pub fn perspective_infinite_reverse_rh(
            fov_y_radians: $t,
            aspect_ratio: $t,
            z_near: $t,
        ) -> Self {
            Self($inner::perspective_infinite_reverse_rh(
                fov_y_radians,
                aspect_ratio,
                z_near,
            ))
        }

        /// Creates a right-handed orthographic projection matrix with `[-1,1]` depth
        /// range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
        /// See
        /// <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
        #[inline(always)]
        pub fn orthographic_rh_gl(
            left: $t,
            right: $t,
            bottom: $t,
            top: $t,
            near: $t,
            far: $t,
        ) -> Self {
            Self($inner::orthographic_rh_gl(
                left, right, bottom, top, near, far,
            ))
        }

        /// Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
        #[inline(always)]
        pub fn orthographic_lh(
            left: $t,
            right: $t,
            bottom: $t,
            top: $t,
            near: $t,
            far: $t,
        ) -> Self {
            Self($inner::orthographic_lh(left, right, bottom, top, near, far))
        }

        /// Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
        #[inline(always)]
        pub fn orthographic_rh(
            left: $t,
            right: $t,
            bottom: $t,
            top: $t,
            near: $t,
            far: $t,
        ) -> Self {
            Self($inner::orthographic_rh(left, right, bottom, top, near, far))
        }

        /// Transforms a 4D vector.
        #[inline(always)]
        pub fn mul_vec4(&self, other: $vec4) -> $vec4 {
            $vec4(self.0.mul_vector(other.0))
        }

        /// Multiplies two 4x4 matrices.
        #[inline(always)]
        pub fn mul_mat4(&self, other: &Self) -> Self {
            Self(self.0.mul_matrix(&other.0))
        }

        /// Adds two 4x4 matrices.
        #[inline(always)]
        pub fn add_mat4(&self, other: &Self) -> Self {
            Self(self.0.add_matrix(&other.0))
        }

        /// Subtracts two 4x4 matrices.
        #[inline(always)]
        pub fn sub_mat4(&self, other: &Self) -> Self {
            Self(self.0.sub_matrix(&other.0))
        }

        /// Multiplies this matrix by a scalar value.
        #[inline(always)]
        pub fn mul_scalar(&self, other: $t) -> Self {
            Self(self.0.mul_scalar(other))
        }

        /// Transforms the given 3D vector as a point, applying perspective correction.
        ///
        /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
        /// The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
        ///
        /// This method assumes that `self` contains a projective transform.
        #[inline]
        pub fn project_point3(&self, other: $vec3) -> $vec3 {
            $vec3(self.0.project_point3(other.0))
        }

        /// Transforms the given 3D vector as a point.
        ///
        /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
        /// `1.0`.
        ///
        /// This method assumes that `self` contains a valid affine transform. It does not perform
        /// a persective divide, if `self` contains a perspective transform, or if you are unsure,
        /// the [`Self::project_point3()`] method should be used instead.
        ///
        /// # Panics
        ///
        /// Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
        #[inline]
        pub fn transform_point3(&self, other: $vec3) -> $vec3 {
            glam_assert!(self.row(3) == $vec4::W);
            $vec3(self.0.transform_point3(other.0))
        }

        /// Transforms the give 3D vector as a direction.
        ///
        /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
        /// `0.0`.
        ///
        /// This method assumes that `self` contains a valid affine transform.
        ///
        /// # Panics
        ///
        /// Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
        #[inline]
        pub fn transform_vector3(&self, other: $vec3) -> $vec3 {
            glam_assert!(self.row(3) == $vec4::W);
            $vec3(self.0.transform_vector3(other.0))
        }

        /// Returns true if the absolute difference of all elements between `self` and `other`
        /// is less than or equal to `max_abs_diff`.
        ///
        /// This can be used to compare if two 4x4 matrices contain similar elements. It works
        /// best when comparing with a known value. The `max_abs_diff` that should be used used
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

macro_rules! impl_mat4_traits {
    ($t:ty, $new:ident, $mat4:ident, $vec4:ident) => {
        /// Creates a 4x4 matrix from four column vectors.
        #[inline(always)]
        pub fn $new(x_axis: $vec4, y_axis: $vec4, z_axis: $vec4, w_axis: $vec4) -> $mat4 {
            $mat4::from_cols(x_axis, y_axis, z_axis, w_axis)
        }

        impl_matn_common_traits!($t, $mat4, $vec4);

        impl Deref for $mat4 {
            type Target = Columns4<$vec4>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }

        impl DerefMut for $mat4 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut Self as *mut Self::Target) }
            }
        }

        impl PartialEq for $mat4 {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.x_axis.eq(&other.x_axis)
                    && self.y_axis.eq(&other.y_axis)
                    && self.z_axis.eq(&other.z_axis)
                    && self.w_axis.eq(&other.w_axis)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsRef<[$t; 16]> for $mat4 {
            #[inline]
            fn as_ref(&self) -> &[$t; 16] {
                unsafe { &*(self as *const Self as *const [$t; 16]) }
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsMut<[$t; 16]> for $mat4 {
            #[inline]
            fn as_mut(&mut self) -> &mut [$t; 16] {
                unsafe { &mut *(self as *mut Self as *mut [$t; 16]) }
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $mat4 {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct(stringify!($mat4))
                    .field("x_axis", &self.x_axis)
                    .field("y_axis", &self.y_axis)
                    .field("z_axis", &self.z_axis)
                    .field("w_axis", &self.w_axis)
                    .finish()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $mat4 {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "[{}, {}, {}, {}]",
                    self.x_axis, self.y_axis, self.z_axis, self.w_axis
                )
            }
        }
    };
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type InnerF32 = Columns4<__m128>;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
type InnerF32 = Columns4<v128>;

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
type InnerF32 = Columns4<XYZW<f32>>;

/// A 4x4 column major matrix.
///
/// This 4x4 matrix type features convenience methods for creating and using affine transforms and
/// perspective projections. If you are primarily dealing with 3D affine transformations
/// considering using [`Affine3A`](crate::Affine3A) which is faster than a 4x4 matrix for some
/// affine operations.
///
/// Affine transformations including 3D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
/// [`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
///
/// Othographic projections can be created using the methods [`Self::orthographic_lh()`] for
/// left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
/// systems. The resulting matrix is also an affine transformation.
///
/// The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
/// are provided for performing affine transformations on 3D vectors and points. These
/// multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
/// for vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
///
/// Perspective projections can be created using methods such as
/// [`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
/// [`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
/// [`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
/// [`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
///
/// The resulting perspective project can be use to transform 3D vectors as points with
/// perspective correction using the [`Self::project_point3()`] convenience method.
#[derive(Clone, Copy)]
#[cfg_attr(
    any(
        not(any(feature = "scalar-math", target_arch = "spirv")),
        feature = "cuda"
    ),
    repr(C, align(16))
)]
#[cfg_attr(
    all(
        any(feature = "scalar-math", target_arch = "spirv"),
        not(feature = "cuda"),
    ),
    repr(transparent)
)]
pub struct Mat4(pub(crate) InnerF32);
// define_mat4_struct!(Mat4, InnerF32);

impl Mat4 {
    impl_mat4_methods!(f32, Vec4, Vec3, Mat3, Quat, InnerF32);

    /// Transforms the given `Vec3A` as 3D point.
    ///
    /// This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `1.0`.
    #[inline(always)]
    pub fn transform_point3a(&self, other: Vec3A) -> Vec3A {
        #[allow(clippy::useless_conversion)]
        Vec3A(self.0.transform_float4_as_point3(other.0.into()).into())
    }

    /// Transforms the give `Vec3A` as 3D vector.
    ///
    /// This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `0.0`.
    #[inline(always)]
    pub fn transform_vector3a(&self, other: Vec3A) -> Vec3A {
        #[allow(clippy::useless_conversion)]
        Vec3A(self.0.transform_float4_as_vector3(other.0.into()).into())
    }

    #[inline(always)]
    pub fn as_dmat4(&self) -> DMat4 {
        DMat4::from_cols(
            self.x_axis.as_dvec4(),
            self.y_axis.as_dvec4(),
            self.z_axis.as_dvec4(),
            self.w_axis.as_dvec4(),
        )
    }
}
impl_mat4_traits!(f32, mat4, Mat4, Vec4);

type InnerF64 = Columns4<XYZW<f64>>;

/// A 4x4 column major matrix.
///
/// This 4x4 matrix type features convenience methods for creating and using affine transforms and
/// perspective projections. If you are primarily dealing with 3D affine transformations
/// considering using [`DAffine3`](crate::DAffine3) which is faster than a 4x4 matrix for some
/// affine operations.
///
/// Affine transformations including 3D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
/// [`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
///
/// Othographic projections can be created using the methods [`Self::orthographic_lh()`] for
/// left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
/// systems. The resulting matrix is also an affine transformation.
///
/// The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
/// are provided for performing affine transformations on 3D vectors and points. These
/// multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
/// for vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
///
/// Perspective projections can be created using methods such as
/// [`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
/// [`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
/// [`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
/// [`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
///
/// The resulting perspective project can be use to transform 3D vectors as points with
/// perspective correction using the [`Self::project_point3()`] convenience method.
#[derive(Clone, Copy)]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
#[cfg_attr(feature = "cuda", repr(C, align(16)))]
pub struct DMat4(pub(crate) InnerF64);
// define_mat4_struct!(DMat4, InnerF64);

impl DMat4 {
    impl_mat4_methods!(f64, DVec4, DVec3, DMat3, DQuat, InnerF64);

    #[inline(always)]
    pub fn as_mat4(&self) -> Mat4 {
        Mat4::from_cols(
            self.x_axis.as_vec4(),
            self.y_axis.as_vec4(),
            self.z_axis.as_vec4(),
            self.w_axis.as_vec4(),
        )
    }
}
impl_mat4_traits!(f64, dmat4, DMat4, DVec4);

mod const_test_mat4 {
    const_assert_eq!(
        core::mem::align_of::<super::Vec4>(),
        core::mem::align_of::<super::Mat4>()
    );
    const_assert_eq!(64, core::mem::size_of::<super::Mat4>());
}

mod const_test_dmat4 {
    const_assert_eq!(
        core::mem::align_of::<super::DVec4>(),
        core::mem::align_of::<super::DMat4>()
    );
    const_assert_eq!(128, core::mem::size_of::<super::DMat4>());
}
