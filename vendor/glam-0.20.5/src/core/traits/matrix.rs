use crate::core::{
    storage::{XY, XYZ, XYZW},
    traits::{
        quaternion::Quaternion,
        scalar::{FloatEx, NumEx},
        vector::*,
    },
};

pub trait MatrixConst {
    const ZERO: Self;
    const IDENTITY: Self;
}

/// Base matrix trait that sets up trait bounds
pub trait Matrix<T: NumEx>: Sized + Copy + Clone {}

/// 2x2 Matrix trait for all types of T
pub trait Matrix2x2<T: NumEx, V2: Vector2<T>>: Matrix<T> {
    #[inline(always)]
    fn new(m00: T, m01: T, m10: T, m11: T) -> Self {
        Self::from_cols(V2::new(m00, m01), V2::new(m10, m11))
    }

    fn from_cols(x_axis: V2, y_axis: V2) -> Self;

    fn x_axis(&self) -> &V2;
    fn y_axis(&self) -> &V2;

    #[inline(always)]
    fn from_cols_array(m: &[T; 4]) -> Self {
        Self::new(m[0], m[1], m[2], m[3])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn to_cols_array(&self) -> [T; 4] {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        [x_axis.x(), x_axis.y(),
         y_axis.x(), y_axis.y()]
    }

    #[inline(always)]
    fn from_cols_array_2d(m: &[[T; 2]; 2]) -> Self {
        Self::from_cols(V2::from_array(m[0]), V2::from_array(m[1]))
    }

    #[inline(always)]
    fn to_cols_array_2d(&self) -> [[T; 2]; 2] {
        [self.x_axis().into_array(), self.y_axis().into_array()]
    }

    #[inline(always)]
    fn from_cols_slice(m: &[T]) -> Self {
        Self::new(m[0], m[1], m[2], m[3])
    }

    #[inline(always)]
    fn write_cols_to_slice(&self, slice: &mut [T]) {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        slice[0] = x_axis.x();
        slice[1] = x_axis.y();
        slice[2] = y_axis.x();
        slice[3] = y_axis.y();
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn from_diagonal(diagonal: XY<T>) -> Self {
        Self::new(
            diagonal.x, T::ZERO,
            T::ZERO, diagonal.y)
    }

    #[inline]
    fn determinant(&self) -> T {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        x_axis.x() * y_axis.y() - x_axis.y() * y_axis.x()
    }

    #[inline(always)]
    fn transpose(&self) -> Self {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        Self::new(x_axis.x(), y_axis.x(), x_axis.y(), y_axis.y())
    }

    #[inline]
    fn mul_vector(&self, other: V2) -> V2 {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        #[allow(clippy::suspicious_operation_groupings)]
        V2::new(
            (x_axis.x() * other.x()) + (y_axis.x() * other.y()),
            (x_axis.y() * other.x()) + (y_axis.y() * other.y()),
        )
    }

    #[inline]
    fn mul_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.mul_vector(*other.x_axis()),
            self.mul_vector(*other.y_axis()),
        )
    }

    #[inline]
    fn mul_scalar(&self, other: T) -> Self {
        Self::from_cols(
            self.x_axis().mul_scalar(other),
            self.y_axis().mul_scalar(other),
        )
    }

    #[inline]
    fn add_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.x_axis().add(*other.x_axis()),
            self.y_axis().add(*other.y_axis()),
        )
    }

    #[inline]
    fn sub_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.x_axis().sub(*other.x_axis()),
            self.y_axis().sub(*other.y_axis()),
        )
    }
}

/// 2x2 matrix trait for float types of T
pub trait FloatMatrix2x2<T: FloatEx, V2: FloatVector2<T>>: Matrix2x2<T, V2> {
    #[inline]
    fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool
    where
        <V2 as Vector<T>>::Mask: MaskVector2,
    {
        self.x_axis().abs_diff_eq(*other.x_axis(), max_abs_diff)
            && self.y_axis().abs_diff_eq(*other.y_axis(), max_abs_diff)
    }

    #[inline]
    fn neg_matrix(&self) -> Self {
        Self::from_cols(self.x_axis().neg(), self.y_axis().neg())
    }

    #[inline]
    fn from_scale_angle(scale: V2, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        let (scale_x, scale_y) = scale.into_tuple();
        Self::new(cos * scale_x, sin * scale_x, -sin * scale_y, cos * scale_y)
    }

    #[inline]
    fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(cos, sin, -sin, cos)
    }

    #[inline]
    fn inverse(&self) -> Self {
        let inv_det = {
            let det = self.determinant();
            glam_assert!(det != T::ZERO);
            det.recip()
        };
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        Self::new(
            y_axis.y() * inv_det,
            x_axis.y() * -inv_det,
            y_axis.x() * -inv_det,
            x_axis.x() * inv_det,
        )
    }
}

pub trait Matrix3x3<T: NumEx, V3: Vector3<T>>: Matrix<T> {
    fn from_cols(x_axis: V3, y_axis: V3, z_axis: V3) -> Self;

    fn x_axis(&self) -> &V3;
    fn y_axis(&self) -> &V3;
    fn z_axis(&self) -> &V3;

    #[rustfmt::skip]
    #[inline(always)]
    fn from_cols_array(m: &[T; 9]) -> Self {
        Self::from_cols(
            V3::new(m[0], m[1], m[2]),
            V3::new(m[3], m[4], m[5]),
            V3::new(m[6], m[7], m[8]))
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn to_cols_array(&self) -> [T; 9] {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        let z_axis = self.z_axis();
        [
            x_axis.x(), x_axis.y(), x_axis.z(),
            y_axis.x(), y_axis.y(), y_axis.z(),
            z_axis.x(), z_axis.y(), z_axis.z(),
        ]
    }

    #[inline(always)]
    fn from_cols_array_2d(m: &[[T; 3]; 3]) -> Self {
        Self::from_cols(
            V3::from_array(m[0]),
            V3::from_array(m[1]),
            V3::from_array(m[2]),
        )
    }

    #[inline(always)]
    fn to_cols_array_2d(&self) -> [[T; 3]; 3] {
        [
            self.x_axis().into_array(),
            self.y_axis().into_array(),
            self.z_axis().into_array(),
        ]
    }

    #[inline(always)]
    fn from_cols_slice(m: &[T]) -> Self {
        Self::from_cols(
            V3::new(m[0], m[1], m[2]),
            V3::new(m[3], m[4], m[5]),
            V3::new(m[6], m[7], m[8]),
        )
    }

    #[inline(always)]
    fn write_cols_to_slice(&self, slice: &mut [T]) {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        let z_axis = self.z_axis();
        slice[0] = x_axis.x();
        slice[1] = x_axis.y();
        slice[2] = x_axis.z();
        slice[3] = y_axis.x();
        slice[4] = y_axis.y();
        slice[5] = y_axis.z();
        slice[6] = z_axis.x();
        slice[7] = z_axis.y();
        slice[8] = z_axis.z();
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn from_diagonal(diagonal: XYZ<T>) -> Self {
        Self::from_cols(
            V3::new(diagonal.x, T::ZERO, T::ZERO),
            V3::new(T::ZERO, diagonal.y, T::ZERO),
            V3::new(T::ZERO, T::ZERO, diagonal.z),
        )
    }

    #[inline(always)]
    fn from_scale(scale: XY<T>) -> Self {
        // Do not panic as long as any component is non-zero
        glam_assert!(scale.cmpne(XY::<T>::ZERO).any());
        Self::from_cols(
            V3::new(scale.x, T::ZERO, T::ZERO),
            V3::new(T::ZERO, scale.y, T::ZERO),
            V3::Z,
        )
    }

    #[inline(always)]
    fn from_translation(translation: XY<T>) -> Self {
        Self::from_cols(V3::X, V3::Y, V3::new(translation.x, translation.y, T::ONE))
    }

    #[inline]
    fn determinant(&self) -> T {
        self.z_axis().dot(self.x_axis().cross(*self.y_axis()))
    }

    #[inline]
    fn transpose(&self) -> Self {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        let z_axis = self.z_axis();
        Self::from_cols(
            V3::new(x_axis.x(), y_axis.x(), z_axis.x()),
            V3::new(x_axis.y(), y_axis.y(), z_axis.y()),
            V3::new(x_axis.z(), y_axis.z(), z_axis.z()),
        )
    }

    #[inline]
    fn mul_vector(&self, other: V3) -> V3 {
        let mut res = self.x_axis().mul(other.splat_x());
        res = res.add(self.y_axis().mul(other.splat_y()));
        res = res.add(self.z_axis().mul(other.splat_z()));
        res
    }

    #[inline]
    fn mul_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.mul_vector(*other.x_axis()),
            self.mul_vector(*other.y_axis()),
            self.mul_vector(*other.z_axis()),
        )
    }

    #[inline]
    fn mul_scalar(&self, other: T) -> Self {
        Self::from_cols(
            self.x_axis().mul_scalar(other),
            self.y_axis().mul_scalar(other),
            self.z_axis().mul_scalar(other),
        )
    }

    #[inline]
    fn add_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.x_axis().add(*other.x_axis()),
            self.y_axis().add(*other.y_axis()),
            self.z_axis().add(*other.z_axis()),
        )
    }

    #[inline]
    fn sub_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.x_axis().sub(*other.x_axis()),
            self.y_axis().sub(*other.y_axis()),
            self.z_axis().sub(*other.z_axis()),
        )
    }
}

pub trait FloatMatrix3x3<T: FloatEx, V3: FloatVector3<T>>: Matrix3x3<T, V3> {
    #[inline]
    fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool
    where
        <V3 as Vector<T>>::Mask: MaskVector3,
    {
        self.x_axis().abs_diff_eq(*other.x_axis(), max_abs_diff)
            && self.y_axis().abs_diff_eq(*other.y_axis(), max_abs_diff)
            && self.z_axis().abs_diff_eq(*other.z_axis(), max_abs_diff)
    }

    #[inline]
    fn neg_matrix(&self) -> Self {
        Self::from_cols(
            self.x_axis().neg(),
            self.y_axis().neg(),
            self.z_axis().neg(),
        )
    }

    #[inline]
    fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_cols(
            V3::new(cos, sin, T::ZERO),
            V3::new(-sin, cos, T::ZERO),
            V3::Z,
        )
    }
    #[inline]
    fn from_scale_angle_translation(scale: XY<T>, angle: T, translation: XY<T>) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_cols(
            V3::new(cos * scale.x, sin * scale.x, T::ZERO),
            V3::new(-sin * scale.y, cos * scale.y, T::ZERO),
            V3::new(translation.x, translation.y, T::ONE),
        )
    }

    #[inline]
    fn from_axis_angle(axis: XYZ<T>, angle: T) -> Self {
        glam_assert!(axis.is_normalized());
        let (sin, cos) = angle.sin_cos();
        let (xsin, ysin, zsin) = axis.mul_scalar(sin).into_tuple();
        let (x, y, z) = axis.into_tuple();
        let (x2, y2, z2) = axis.mul(axis).into_tuple();
        let omc = T::ONE - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;
        Self::from_cols(
            V3::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
            V3::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
            V3::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
        )
    }

    #[inline]
    fn from_quaternion(rotation: XYZW<T>) -> Self {
        glam_assert!(rotation.is_normalized());
        let x2 = rotation.x + rotation.x;
        let y2 = rotation.y + rotation.y;
        let z2 = rotation.z + rotation.z;
        let xx = rotation.x * x2;
        let xy = rotation.x * y2;
        let xz = rotation.x * z2;
        let yy = rotation.y * y2;
        let yz = rotation.y * z2;
        let zz = rotation.z * z2;
        let wx = rotation.w * x2;
        let wy = rotation.w * y2;
        let wz = rotation.w * z2;

        Self::from_cols(
            V3::new(T::ONE - (yy + zz), xy + wz, xz - wy),
            V3::new(xy - wz, T::ONE - (xx + zz), yz + wx),
            V3::new(xz + wy, yz - wx, T::ONE - (xx + yy)),
        )
    }

    #[inline]
    fn from_rotation_x(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            V3::X,
            V3::new(T::ZERO, cosa, sina),
            V3::new(T::ZERO, -sina, cosa),
        )
    }

    #[inline]
    fn from_rotation_y(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            V3::new(cosa, T::ZERO, -sina),
            V3::Y,
            V3::new(sina, T::ZERO, cosa),
        )
    }

    #[inline]
    fn from_rotation_z(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            V3::new(cosa, sina, T::ZERO),
            V3::new(-sina, cosa, T::ZERO),
            V3::Z,
        )
    }

    fn transform_point2(&self, other: XY<T>) -> XY<T>;
    fn transform_vector2(&self, other: XY<T>) -> XY<T>;

    #[inline]
    fn inverse(&self) -> Self
    where
        <V3 as Vector<T>>::Mask: MaskVector3,
    {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        let z_axis = self.z_axis();
        let tmp0 = y_axis.cross(*z_axis);
        let tmp1 = z_axis.cross(*x_axis);
        let tmp2 = x_axis.cross(*y_axis);
        let det = z_axis.dot_into_vec(tmp2);
        glam_assert!(det.cmpne(V3::ZERO).all());
        let inv_det = det.recip();
        // TODO: Work out if it's possible to get rid of the transpose
        Self::from_cols(tmp0.mul(inv_det), tmp1.mul(inv_det), tmp2.mul(inv_det)).transpose()
    }

    #[inline]
    fn is_finite(&self) -> bool {
        self.x_axis().is_finite() && self.y_axis().is_finite() && self.z_axis().is_finite()
    }

    #[inline]
    fn is_nan(&self) -> bool {
        self.x_axis().is_nan() || self.y_axis().is_nan() || self.z_axis().is_nan()
    }
}

pub trait Matrix4x4<T: NumEx, V4: Vector4<T>>: Matrix<T> {
    fn from_cols(x_axis: V4, y_axis: V4, z_axis: V4, w_axis: V4) -> Self;

    fn x_axis(&self) -> &V4;
    fn y_axis(&self) -> &V4;
    fn z_axis(&self) -> &V4;
    fn w_axis(&self) -> &V4;

    #[rustfmt::skip]
    #[inline(always)]
    fn from_cols_array(m: &[T; 16]) -> Self {
        Self::from_cols(
            V4::new( m[0],  m[1],  m[2],  m[3]),
            V4::new( m[4],  m[5],  m[6],  m[7]),
            V4::new( m[8],  m[9], m[10], m[11]),
            V4::new(m[12], m[13], m[14], m[15]))
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn to_cols_array(&self) -> [T; 16] {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        let z_axis = self.z_axis();
        let w_axis = self.w_axis();
        [
            x_axis.x(), x_axis.y(), x_axis.z(), x_axis.w(),
            y_axis.x(), y_axis.y(), y_axis.z(), y_axis.w(),
            z_axis.x(), z_axis.y(), z_axis.z(), z_axis.w(),
            w_axis.x(), w_axis.y(), w_axis.z(), w_axis.w(),
        ]
    }

    #[inline(always)]
    fn from_cols_array_2d(m: &[[T; 4]; 4]) -> Self {
        Self::from_cols(
            Vector4::from_array(m[0]),
            Vector4::from_array(m[1]),
            Vector4::from_array(m[2]),
            Vector4::from_array(m[3]),
        )
    }

    #[inline(always)]
    fn to_cols_array_2d(&self) -> [[T; 4]; 4] {
        [
            self.x_axis().into_array(),
            self.y_axis().into_array(),
            self.z_axis().into_array(),
            self.w_axis().into_array(),
        ]
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn from_cols_slice(m: &[T]) -> Self {
        Self::from_cols(
            V4::new( m[0],  m[1],  m[2],  m[3]),
            V4::new( m[4],  m[5],  m[6],  m[7]),
            V4::new( m[8],  m[9], m[10], m[11]),
            V4::new(m[12], m[13], m[14], m[15]))
    }

    #[inline(always)]
    fn write_cols_to_slice(&self, slice: &mut [T]) {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        let z_axis = self.z_axis();
        let w_axis = self.w_axis();
        slice[0] = x_axis.x();
        slice[1] = x_axis.y();
        slice[2] = x_axis.z();
        slice[3] = x_axis.w();

        slice[4] = y_axis.x();
        slice[5] = y_axis.y();
        slice[6] = y_axis.z();
        slice[7] = y_axis.w();

        slice[8] = z_axis.x();
        slice[9] = z_axis.y();
        slice[10] = z_axis.z();
        slice[11] = z_axis.w();

        slice[12] = w_axis.x();
        slice[13] = w_axis.y();
        slice[14] = w_axis.z();
        slice[15] = w_axis.w();
    }

    #[inline(always)]
    fn from_diagonal(diagonal: XYZW<T>) -> Self {
        Self::from_cols(
            V4::new(diagonal.x, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, diagonal.y, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, diagonal.z, T::ZERO),
            V4::new(T::ZERO, T::ZERO, T::ZERO, diagonal.w),
        )
    }

    #[inline(always)]
    fn from_scale(scale: XYZ<T>) -> Self {
        // Do not panic as long as any component is non-zero
        glam_assert!(scale.cmpne(XYZ::<T>::ZERO).any());
        Self::from_cols(
            V4::new(scale.x, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, scale.y, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, scale.z, T::ZERO),
            V4::W,
        )
    }

    #[inline(always)]
    fn from_translation(translation: XYZ<T>) -> Self {
        Self::from_cols(
            V4::X,
            V4::Y,
            V4::Z,
            V4::new(translation.x, translation.y, translation.z, T::ONE),
        )
    }

    #[inline]
    fn determinant(&self) -> T {
        let (m00, m01, m02, m03) = self.x_axis().into_tuple();
        let (m10, m11, m12, m13) = self.y_axis().into_tuple();
        let (m20, m21, m22, m23) = self.z_axis().into_tuple();
        let (m30, m31, m32, m33) = self.w_axis().into_tuple();

        let a2323 = m22 * m33 - m23 * m32;
        let a1323 = m21 * m33 - m23 * m31;
        let a1223 = m21 * m32 - m22 * m31;
        let a0323 = m20 * m33 - m23 * m30;
        let a0223 = m20 * m32 - m22 * m30;
        let a0123 = m20 * m31 - m21 * m30;

        m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
            - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
            + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
            - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123)
    }

    #[inline]
    fn transpose(&self) -> Self {
        let (m00, m01, m02, m03) = self.x_axis().into_tuple();
        let (m10, m11, m12, m13) = self.y_axis().into_tuple();
        let (m20, m21, m22, m23) = self.z_axis().into_tuple();
        let (m30, m31, m32, m33) = self.w_axis().into_tuple();

        Self::from_cols(
            V4::new(m00, m10, m20, m30),
            V4::new(m01, m11, m21, m31),
            V4::new(m02, m12, m22, m32),
            V4::new(m03, m13, m23, m33),
        )
    }

    #[inline]
    fn mul_vector(&self, other: V4) -> V4 {
        let mut res = self.x_axis().mul(other.splat_x());
        res = res.add(self.y_axis().mul(other.splat_y()));
        res = res.add(self.z_axis().mul(other.splat_z()));
        res = res.add(self.w_axis().mul(other.splat_w()));
        res
    }

    #[inline]
    fn mul_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.mul_vector(*other.x_axis()),
            self.mul_vector(*other.y_axis()),
            self.mul_vector(*other.z_axis()),
            self.mul_vector(*other.w_axis()),
        )
    }

    #[inline]
    fn mul_scalar(&self, other: T) -> Self {
        Self::from_cols(
            self.x_axis().mul_scalar(other),
            self.y_axis().mul_scalar(other),
            self.z_axis().mul_scalar(other),
            self.w_axis().mul_scalar(other),
        )
    }

    #[inline]
    fn add_matrix(&self, other: &Self) -> Self {
        // TODO: Make Vector4::add take a ref?
        Self::from_cols(
            self.x_axis().add(*other.x_axis()),
            self.y_axis().add(*other.y_axis()),
            self.z_axis().add(*other.z_axis()),
            self.w_axis().add(*other.w_axis()),
        )
    }

    #[inline]
    fn sub_matrix(&self, other: &Self) -> Self {
        // TODO: Make Vector4::sub take a ref?
        Self::from_cols(
            self.x_axis().sub(*other.x_axis()),
            self.y_axis().sub(*other.y_axis()),
            self.z_axis().sub(*other.z_axis()),
            self.w_axis().sub(*other.w_axis()),
        )
    }
}

pub trait FloatMatrix4x4<T: FloatEx, V4: FloatVector4<T> + Quaternion<T>>:
    Matrix4x4<T, V4>
{
    // Vector3 represented by a SIMD type if available
    type SIMDVector3;

    #[inline]
    fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool
    where
        <V4 as Vector<T>>::Mask: MaskVector4,
    {
        self.x_axis().abs_diff_eq(*other.x_axis(), max_abs_diff)
            && self.y_axis().abs_diff_eq(*other.y_axis(), max_abs_diff)
            && self.z_axis().abs_diff_eq(*other.z_axis(), max_abs_diff)
            && self.w_axis().abs_diff_eq(*other.w_axis(), max_abs_diff)
    }

    #[inline]
    fn neg_matrix(&self) -> Self {
        Self::from_cols(
            self.x_axis().neg(),
            self.y_axis().neg(),
            self.z_axis().neg(),
            self.w_axis().neg(),
        )
    }

    #[inline]
    fn quaternion_to_axes(rotation: V4) -> (V4, V4, V4) {
        glam_assert!(rotation.is_normalized());
        let (x, y, z, w) = rotation.into_tuple();
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let x_axis = V4::new(T::ONE - (yy + zz), xy + wz, xz - wy, T::ZERO);
        let y_axis = V4::new(xy - wz, T::ONE - (xx + zz), yz + wx, T::ZERO);
        let z_axis = V4::new(xz + wy, yz - wx, T::ONE - (xx + yy), T::ZERO);
        (x_axis, y_axis, z_axis)
    }

    #[inline]
    fn from_quaternion(rotation: V4) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quaternion_to_axes(rotation);
        Self::from_cols(x_axis, y_axis, z_axis, V4::W)
    }

    fn to_scale_quaternion_translation(&self) -> (XYZ<T>, V4, XYZ<T>) {
        let det = self.determinant();
        glam_assert!(det != T::ZERO);

        let scale: XYZ<T> = Vector3::new(
            self.x_axis().length() * det.signum(),
            self.y_axis().length(),
            self.z_axis().length(),
        );

        glam_assert!(scale.cmpne(XYZ::<T>::ZERO).all());

        let inv_scale = scale.recip();

        let rotation = Quaternion::from_rotation_axes(
            self.x_axis().mul_scalar(inv_scale.x).into_xyz(),
            self.y_axis().mul_scalar(inv_scale.y).into_xyz(),
            self.z_axis().mul_scalar(inv_scale.z).into_xyz(),
        );

        let translation = self.w_axis().into_xyz();

        (scale, rotation, translation)
    }

    #[inline]
    fn from_scale_quaternion_translation(scale: XYZ<T>, rotation: V4, translation: XYZ<T>) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quaternion_to_axes(rotation);
        Self::from_cols(
            x_axis.mul_scalar(scale.x),
            y_axis.mul_scalar(scale.y),
            z_axis.mul_scalar(scale.z),
            V4::from_xyz(translation, T::ONE),
        )
    }

    #[inline]
    fn from_quaternion_translation(rotation: V4, translation: XYZ<T>) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quaternion_to_axes(rotation);
        Self::from_cols(x_axis, y_axis, z_axis, V4::from_xyz(translation, T::ONE))
    }

    #[inline]
    fn from_axis_angle(axis: XYZ<T>, angle: T) -> Self {
        glam_assert!(axis.is_normalized());
        let (sin, cos) = angle.sin_cos();
        let axis_sin = axis.mul_scalar(sin);
        let axis_sq = axis.mul(axis);
        let omc = T::ONE - cos;
        let xyomc = axis.x * axis.y * omc;
        let xzomc = axis.x * axis.z * omc;
        let yzomc = axis.y * axis.z * omc;
        Self::from_cols(
            V4::new(
                axis_sq.x * omc + cos,
                xyomc + axis_sin.z,
                xzomc - axis_sin.y,
                T::ZERO,
            ),
            V4::new(
                xyomc - axis_sin.z,
                axis_sq.y * omc + cos,
                yzomc + axis_sin.x,
                T::ZERO,
            ),
            V4::new(
                xzomc + axis_sin.y,
                yzomc - axis_sin.x,
                axis_sq.z * omc + cos,
                T::ZERO,
            ),
            V4::W,
        )
    }

    #[inline]
    fn from_rotation_x(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            V4::X,
            V4::new(T::ZERO, cosa, sina, T::ZERO),
            V4::new(T::ZERO, -sina, cosa, T::ZERO),
            V4::W,
        )
    }

    #[inline]
    fn from_rotation_y(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            V4::new(cosa, T::ZERO, -sina, T::ZERO),
            V4::Y,
            V4::new(sina, T::ZERO, cosa, T::ZERO),
            V4::W,
        )
    }

    #[inline]
    fn from_rotation_z(angle: T) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            V4::new(cosa, sina, T::ZERO, T::ZERO),
            V4::new(-sina, cosa, T::ZERO, T::ZERO),
            V4::Z,
            V4::W,
        )
    }

    #[inline]
    fn look_to_lh(eye: XYZ<T>, dir: XYZ<T>, up: XYZ<T>) -> Self {
        let f = dir.normalize();
        let s = up.cross(f).normalize();
        let u = f.cross(s);
        Self::from_cols(
            V4::new(s.x, u.x, f.x, T::ZERO),
            V4::new(s.y, u.y, f.y, T::ZERO),
            V4::new(s.z, u.z, f.z, T::ZERO),
            V4::new(-s.dot(eye), -u.dot(eye), -f.dot(eye), T::ONE),
        )
    }

    #[inline]
    fn look_at_lh(eye: XYZ<T>, center: XYZ<T>, up: XYZ<T>) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, center.sub(eye), up)
    }

    #[inline]
    fn look_at_rh(eye: XYZ<T>, center: XYZ<T>, up: XYZ<T>) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, eye.sub(center), up)
    }

    #[inline]
    fn transform_point3(&self, other: XYZ<T>) -> XYZ<T> {
        let mut res = self.x_axis().mul_scalar(other.x);
        res = self.y_axis().mul_scalar(other.y).add(res);
        res = self.z_axis().mul_scalar(other.z).add(res);
        res = self.w_axis().add(res);
        res.into_xyz()
    }

    #[inline]
    fn transform_vector3(&self, other: XYZ<T>) -> XYZ<T> {
        let mut res = self.x_axis().mul_scalar(other.x);
        res = self.y_axis().mul_scalar(other.y).add(res);
        res = self.z_axis().mul_scalar(other.z).add(res);
        res.into_xyz()
    }

    #[inline]
    fn project_point3(&self, other: XYZ<T>) -> XYZ<T> {
        let mut res = self.x_axis().mul_scalar(other.x);
        res = self.y_axis().mul_scalar(other.y).add(res);
        res = self.z_axis().mul_scalar(other.z).add(res);
        res = self.w_axis().add(res);
        res = res.mul(res.splat_w().recip());
        res.into_xyz()
    }

    fn transform_float4_as_point3(&self, other: Self::SIMDVector3) -> Self::SIMDVector3;
    fn transform_float4_as_vector3(&self, other: Self::SIMDVector3) -> Self::SIMDVector3;
    fn project_float4_as_point3(&self, other: Self::SIMDVector3) -> Self::SIMDVector3;

    fn inverse(&self) -> Self {
        let (m00, m01, m02, m03) = self.x_axis().into_tuple();
        let (m10, m11, m12, m13) = self.y_axis().into_tuple();
        let (m20, m21, m22, m23) = self.z_axis().into_tuple();
        let (m30, m31, m32, m33) = self.w_axis().into_tuple();

        let coef00 = m22 * m33 - m32 * m23;
        let coef02 = m12 * m33 - m32 * m13;
        let coef03 = m12 * m23 - m22 * m13;

        let coef04 = m21 * m33 - m31 * m23;
        let coef06 = m11 * m33 - m31 * m13;
        let coef07 = m11 * m23 - m21 * m13;

        let coef08 = m21 * m32 - m31 * m22;
        let coef10 = m11 * m32 - m31 * m12;
        let coef11 = m11 * m22 - m21 * m12;

        let coef12 = m20 * m33 - m30 * m23;
        let coef14 = m10 * m33 - m30 * m13;
        let coef15 = m10 * m23 - m20 * m13;

        let coef16 = m20 * m32 - m30 * m22;
        let coef18 = m10 * m32 - m30 * m12;
        let coef19 = m10 * m22 - m20 * m12;

        let coef20 = m20 * m31 - m30 * m21;
        let coef22 = m10 * m31 - m30 * m11;
        let coef23 = m10 * m21 - m20 * m11;

        let fac0 = V4::new(coef00, coef00, coef02, coef03);
        let fac1 = V4::new(coef04, coef04, coef06, coef07);
        let fac2 = V4::new(coef08, coef08, coef10, coef11);
        let fac3 = V4::new(coef12, coef12, coef14, coef15);
        let fac4 = V4::new(coef16, coef16, coef18, coef19);
        let fac5 = V4::new(coef20, coef20, coef22, coef23);

        let vec0 = V4::new(m10, m00, m00, m00);
        let vec1 = V4::new(m11, m01, m01, m01);
        let vec2 = V4::new(m12, m02, m02, m02);
        let vec3 = V4::new(m13, m03, m03, m03);

        let inv0 = vec1.mul(fac0).sub(vec2.mul(fac1)).add(vec3.mul(fac2));
        let inv1 = vec0.mul(fac0).sub(vec2.mul(fac3)).add(vec3.mul(fac4));
        let inv2 = vec0.mul(fac1).sub(vec1.mul(fac3)).add(vec3.mul(fac5));
        let inv3 = vec0.mul(fac2).sub(vec1.mul(fac4)).add(vec2.mul(fac5));

        let sign_a = Vector4::new(T::ONE, -T::ONE, T::ONE, -T::ONE);
        let sign_b = Vector4::new(-T::ONE, T::ONE, -T::ONE, T::ONE);

        let inverse = Self::from_cols(
            inv0.mul(sign_a),
            inv1.mul(sign_b),
            inv2.mul(sign_a),
            inv3.mul(sign_b),
        );

        let col0 = V4::new(
            inverse.x_axis().x(),
            inverse.y_axis().x(),
            inverse.z_axis().x(),
            inverse.w_axis().x(),
        );

        let dot0 = self.x_axis().mul(col0);
        let dot1 = dot0.x() + dot0.y() + dot0.z() + dot0.w();

        glam_assert!(dot1 != T::ZERO);

        let rcp_det = dot1.recip();
        inverse.mul_scalar(rcp_det)
    }
}
