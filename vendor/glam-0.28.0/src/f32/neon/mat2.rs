// Generated from mat.rs.tera template. Edit the template, not the generated file.

use crate::{f32::math, swizzles::*, DMat2, Mat3, Mat3A, Vec2};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use core::arch::aarch64::*;

#[repr(C)]
union UnionCast {
    a: [f32; 4],
    v: Mat2,
}

/// Creates a 2x2 matrix from two column vectors.
#[inline(always)]
#[must_use]
pub const fn mat2(x_axis: Vec2, y_axis: Vec2) -> Mat2 {
    Mat2::from_cols(x_axis, y_axis)
}

/// A 2x2 column major matrix.
///
/// SIMD vector types are used for storage on supported platforms.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Mat2(pub(crate) float32x4_t);

impl Mat2 {
    /// A 2x2 matrix with all elements set to `0.0`.
    pub const ZERO: Self = Self::from_cols(Vec2::ZERO, Vec2::ZERO);

    /// A 2x2 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self::from_cols(Vec2::X, Vec2::Y);

    /// All NAN:s.
    pub const NAN: Self = Self::from_cols(Vec2::NAN, Vec2::NAN);

    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    #[must_use]
    const fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
        unsafe {
            UnionCast {
                a: [m00, m01, m10, m11],
            }
            .v
        }
    }

    /// Creates a 2x2 matrix from two column vectors.
    #[inline(always)]
    #[must_use]
    pub const fn from_cols(x_axis: Vec2, y_axis: Vec2) -> Self {
        unsafe {
            UnionCast {
                a: [x_axis.x, x_axis.y, y_axis.x, y_axis.y],
            }
            .v
        }
    }

    /// Creates a 2x2 matrix from a `[f32; 4]` array stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the returned
    /// matrix.
    #[inline]
    #[must_use]
    pub const fn from_cols_array(m: &[f32; 4]) -> Self {
        Self::new(m[0], m[1], m[2], m[3])
    }

    /// Creates a `[f32; 4]` array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    #[must_use]
    pub const fn to_cols_array(&self) -> [f32; 4] {
        unsafe { *(self as *const Self as *const [f32; 4]) }
    }

    /// Creates a 2x2 matrix from a `[[f32; 2]; 2]` 2D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline]
    #[must_use]
    pub const fn from_cols_array_2d(m: &[[f32; 2]; 2]) -> Self {
        Self::from_cols(Vec2::from_array(m[0]), Vec2::from_array(m[1]))
    }

    /// Creates a `[[f32; 2]; 2]` 2D array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    #[must_use]
    pub const fn to_cols_array_2d(&self) -> [[f32; 2]; 2] {
        unsafe { *(self as *const Self as *const [[f32; 2]; 2]) }
    }

    /// Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[doc(alias = "scale")]
    #[inline]
    #[must_use]
    pub const fn from_diagonal(diagonal: Vec2) -> Self {
        Self::new(diagonal.x, 0.0, 0.0, diagonal.y)
    }

    /// Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
    /// `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn from_scale_angle(scale: Vec2, angle: f32) -> Self {
        let (sin, cos) = math::sin_cos(angle);
        Self::new(cos * scale.x, sin * scale.x, -sin * scale.y, cos * scale.y)
    }

    /// Creates a 2x2 matrix containing a rotation of `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn from_angle(angle: f32) -> Self {
        let (sin, cos) = math::sin_cos(angle);
        Self::new(cos, sin, -sin, cos)
    }

    /// Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[inline]
    #[must_use]
    pub fn from_mat3(m: Mat3) -> Self {
        Self::from_cols(m.x_axis.xy(), m.y_axis.xy())
    }

    /// Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[inline]
    #[must_use]
    pub fn from_mat3a(m: Mat3A) -> Self {
        Self::from_cols(m.x_axis.xy(), m.y_axis.xy())
    }

    /// Creates a 2x2 matrix from the first 4 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline]
    #[must_use]
    pub const fn from_cols_slice(slice: &[f32]) -> Self {
        Self::new(slice[0], slice[1], slice[2], slice[3])
    }

    /// Writes the columns of `self` to the first 4 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline]
    pub fn write_cols_to_slice(self, slice: &mut [f32]) {
        slice[0] = self.x_axis.x;
        slice[1] = self.x_axis.y;
        slice[2] = self.y_axis.x;
        slice[3] = self.y_axis.y;
    }

    /// Returns the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 1.
    #[inline]
    #[must_use]
    pub fn col(&self, index: usize) -> Vec2 {
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
    pub fn col_mut(&mut self, index: usize) -> &mut Vec2 {
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
    #[must_use]
    pub fn row(&self, index: usize) -> Vec2 {
        match index {
            0 => Vec2::new(self.x_axis.x, self.y_axis.x),
            1 => Vec2::new(self.x_axis.y, self.y_axis.y),
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.x_axis.is_finite() && self.y_axis.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(&self) -> bool {
        self.x_axis.is_nan() || self.y_axis.is_nan()
    }

    /// Returns the transpose of `self`.
    #[inline]
    #[must_use]
    pub fn transpose(&self) -> Self {
        Self(unsafe {
            vsetq_lane_f32(
                vgetq_lane_f32(self.0, 2),
                vsetq_lane_f32(vgetq_lane_f32(self.0, 1), self.0, 2),
                1,
            )
        })
    }

    /// Returns the determinant of `self`.
    #[inline]
    #[must_use]
    pub fn determinant(&self) -> f32 {
        unsafe {
            let abcd = self.0;
            let badc = vrev64q_f32(abcd);
            let dcba = vextq_f32(badc, badc, 2);
            let prod = vmulq_f32(abcd, dcba);
            let det = vsubq_f32(prod, vdupq_laneq_f32(prod, 1));
            vgetq_lane_f32(det, 0)
        }
    }

    /// Returns the inverse of `self`.
    ///
    /// If the matrix is not invertible the returned matrix will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Self {
        unsafe {
            const SIGN: float32x4_t = crate::neon::f32x4_from_array([1.0, -1.0, -1.0, 1.0]);
            let abcd = self.0;
            let badc = vrev64q_f32(abcd);
            let dcba = vextq_f32(badc, badc, 2);
            let prod = vmulq_f32(abcd, dcba);
            let sub = vsubq_f32(prod, vdupq_laneq_f32(prod, 1));
            let det = vdupq_laneq_f32(sub, 0);
            let tmp = vdivq_f32(SIGN, det);
            glam_assert!(Mat2(tmp).is_finite());
            //let dbca = simd_swizzle!(abcd, [3, 1, 2, 0]);
            let dbca = vsetq_lane_f32(
                vgetq_lane_f32(abcd, 0),
                vsetq_lane_f32(vgetq_lane_f32(abcd, 3), abcd, 0),
                3,
            );
            Self(vmulq_f32(dbca, tmp))
        }
    }

    /// Transforms a 2D vector.
    #[inline]
    #[must_use]
    pub fn mul_vec2(&self, rhs: Vec2) -> Vec2 {
        unsafe {
            let abcd = self.0;
            let xxyy = vld1q_f32([rhs.x, rhs.x, rhs.y, rhs.y].as_ptr());
            let axbxcydy = vmulq_f32(abcd, xxyy);
            // let cydyaxbx = simd_swizzle!(axbxcydy, [2, 3, 0, 1]);
            let cydyaxbx = vextq_f32(axbxcydy, axbxcydy, 2);
            let result = vaddq_f32(axbxcydy, cydyaxbx);
            *(&result as *const float32x4_t as *const Vec2)
        }
    }

    /// Multiplies two 2x2 matrices.
    #[inline]
    #[must_use]
    pub fn mul_mat2(&self, rhs: &Self) -> Self {
        unsafe {
            let abcd = self.0;
            let xxyy0 = vzip1q_f32(rhs.0, rhs.0);
            let xxyy1 = vzip2q_f32(rhs.0, rhs.0);
            let axbxcydy0 = vmulq_f32(abcd, xxyy0);
            let axbxcydy1 = vmulq_f32(abcd, xxyy1);
            let cydyaxbx0 = vextq_f32(axbxcydy0, axbxcydy0, 2);
            let cydyaxbx1 = vextq_f32(axbxcydy1, axbxcydy1, 2);
            let result0 = vaddq_f32(axbxcydy0, cydyaxbx0);
            let result1 = vaddq_f32(axbxcydy1, cydyaxbx1);
            Self(vreinterpretq_f32_u64(vsetq_lane_u64(
                vgetq_lane_u64(vreinterpretq_u64_f32(result1), 0),
                vreinterpretq_u64_f32(result0),
                1,
            )))
        }
    }

    /// Adds two 2x2 matrices.
    #[inline]
    #[must_use]
    pub fn add_mat2(&self, rhs: &Self) -> Self {
        Self(unsafe { vaddq_f32(self.0, rhs.0) })
    }

    /// Subtracts two 2x2 matrices.
    #[inline]
    #[must_use]
    pub fn sub_mat2(&self, rhs: &Self) -> Self {
        Self(unsafe { vsubq_f32(self.0, rhs.0) })
    }

    /// Multiplies a 2x2 matrix by a scalar.
    #[inline]
    #[must_use]
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        Self(unsafe { vmulq_f32(self.0, vld1q_dup_f32(&rhs)) })
    }

    /// Divides a 2x2 matrix by a scalar.
    #[inline]
    #[must_use]
    pub fn div_scalar(&self, rhs: f32) -> Self {
        Self(unsafe { vdivq_f32(self.0, vld1q_dup_f32(&rhs)) })
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two matrices contain similar elements. It works best
    /// when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        self.x_axis.abs_diff_eq(rhs.x_axis, max_abs_diff)
            && self.y_axis.abs_diff_eq(rhs.y_axis, max_abs_diff)
    }

    /// Takes the absolute value of each element in `self`
    #[inline]
    #[must_use]
    pub fn abs(&self) -> Self {
        Self::from_cols(self.x_axis.abs(), self.y_axis.abs())
    }

    #[inline]
    pub fn as_dmat2(&self) -> DMat2 {
        DMat2::from_cols(self.x_axis.as_dvec2(), self.y_axis.as_dvec2())
    }
}

impl Default for Mat2 {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add<Mat2> for Mat2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.add_mat2(&rhs)
    }
}

impl AddAssign<Mat2> for Mat2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add_mat2(&rhs);
    }
}

impl Sub<Mat2> for Mat2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_mat2(&rhs)
    }
}

impl SubAssign<Mat2> for Mat2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub_mat2(&rhs);
    }
}

impl Neg for Mat2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self(unsafe { vnegq_f32(self.0) })
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_mat2(&rhs)
    }
}

impl MulAssign<Mat2> for Mat2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul_mat2(&rhs);
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        self.mul_vec2(rhs)
    }
}

impl Mul<Mat2> for f32 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: Mat2) -> Self::Output {
        rhs.mul_scalar(self)
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.mul_scalar(rhs)
    }
}

impl MulAssign<f32> for Mat2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.mul_scalar(rhs);
    }
}

impl Div<Mat2> for f32 {
    type Output = Mat2;
    #[inline]
    fn div(self, rhs: Mat2) -> Self::Output {
        rhs.div_scalar(self)
    }
}

impl Div<f32> for Mat2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.div_scalar(rhs)
    }
}

impl DivAssign<f32> for Mat2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = self.div_scalar(rhs);
    }
}

impl Sum<Self> for Mat2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Mat2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Mat2 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::IDENTITY, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Mat2 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}

impl PartialEq for Mat2 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.x_axis.eq(&rhs.x_axis) && self.y_axis.eq(&rhs.y_axis)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f32; 4]> for Mat2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Self as *const [f32; 4]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f32; 4]> for Mat2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 4]) }
    }
}

impl core::ops::Deref for Mat2 {
    type Target = crate::deref::Cols2<Vec2>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl core::ops::DerefMut for Mat2 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for Mat2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(stringify!(Mat2))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .finish()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "[{:.*}, {:.*}]", p, self.x_axis, p, self.y_axis)
        } else {
            write!(f, "[{}, {}]", self.x_axis, self.y_axis)
        }
    }
}
