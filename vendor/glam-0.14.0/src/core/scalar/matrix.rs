use crate::core::{
    storage::{Vector2x2, Vector3x3, Vector4x4, XY, XYZ, XYZW},
    traits::{
        matrix::{
            FloatMatrix2x2, FloatMatrix3x3, FloatMatrix4x4, Matrix, Matrix2x2, Matrix3x3,
            Matrix4x4, MatrixConst,
        },
        projection::ProjectionMatrix,
        scalar::{FloatEx, NumEx},
        vector::*,
    },
};

impl<T: NumEx> MatrixConst for Vector2x2<XY<T>> {
    const ZERO: Self = Self {
        x_axis: XY::ZERO,
        y_axis: XY::ZERO,
    };
    const IDENTITY: Self = Self {
        x_axis: XY::X,
        y_axis: XY::Y,
    };
}

impl<T: NumEx> Matrix<T> for Vector2x2<XY<T>> {}

impl<T: NumEx> Matrix2x2<T, XY<T>> for Vector2x2<XY<T>> {
    #[inline(always)]
    fn from_cols(x_axis: XY<T>, y_axis: XY<T>) -> Self {
        Self { x_axis, y_axis }
    }

    #[inline(always)]
    fn as_ref_vector2x2(&self) -> &Vector2x2<XY<T>> {
        self
    }

    #[inline(always)]
    fn as_mut_vector2x2(&mut self) -> &mut Vector2x2<XY<T>> {
        self
    }

    #[inline]
    fn determinant(&self) -> T {
        self.x_axis.x * self.y_axis.y - self.x_axis.y * self.y_axis.x
    }

    #[inline(always)]
    fn transpose(&self) -> Self {
        Self::new(self.x_axis.x, self.y_axis.x, self.x_axis.y, self.y_axis.y)
    }

    #[inline]
    fn mul_vector(&self, other: XY<T>) -> XY<T> {
        Vector2::new(
            (self.x_axis.x * other.x) + (self.y_axis.x * other.y),
            (self.x_axis.y * other.x) + (self.y_axis.y * other.y),
        )
    }

    #[inline]
    fn mul_matrix(&self, other: &Self) -> Self {
        Self::from_cols(self.mul_vector(other.x_axis), self.mul_vector(other.y_axis))
    }

    #[inline]
    fn mul_scalar(&self, other: T) -> Self {
        Self::from_cols(self.x_axis.mul_scalar(other), self.y_axis.mul_scalar(other))
    }

    #[inline]
    fn add_matrix(&self, other: &Self) -> Self {
        Self::from_cols(self.x_axis.add(other.x_axis), self.y_axis.add(other.y_axis))
    }

    #[inline]
    fn sub_matrix(&self, other: &Self) -> Self {
        Self::from_cols(self.x_axis.sub(other.x_axis), self.y_axis.sub(other.y_axis))
    }
}

impl<T: FloatEx> FloatMatrix2x2<T, XY<T>> for Vector2x2<XY<T>> {
    #[inline]
    fn inverse(&self) -> Self {
        let inv_det = {
            let det = self.determinant();
            glam_assert!(det != T::ZERO);
            det.recip()
        };
        Self::new(
            self.y_axis.y * inv_det,
            self.x_axis.y * -inv_det,
            self.y_axis.x * -inv_det,
            self.x_axis.x * inv_det,
        )
    }
}

impl<T: NumEx> MatrixConst for Vector3x3<XYZ<T>> {
    const ZERO: Self = Self {
        x_axis: XYZ::ZERO,
        y_axis: XYZ::ZERO,
        z_axis: XYZ::ZERO,
    };
    const IDENTITY: Self = Self {
        x_axis: XYZ::X,
        y_axis: XYZ::Y,
        z_axis: XYZ::Z,
    };
}

impl<T: NumEx> Matrix<T> for Vector3x3<XYZ<T>> {}

impl<T: NumEx> Matrix3x3<T, XYZ<T>> for Vector3x3<XYZ<T>> {
    #[inline(always)]
    fn from_cols(x_axis: XYZ<T>, y_axis: XYZ<T>, z_axis: XYZ<T>) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    fn as_ref_vector3x3(&self) -> &Vector3x3<XYZ<T>> {
        self
    }

    #[inline(always)]
    fn as_mut_vector3x3(&mut self) -> &mut Vector3x3<XYZ<T>> {
        self
    }

    #[inline]
    fn determinant(&self) -> T {
        self.z_axis.dot(self.x_axis.cross(self.y_axis))
    }

    #[inline(always)]
    fn transpose(&self) -> Self {
        Self::from_cols(
            XYZ {
                x: self.x_axis.x,
                y: self.y_axis.x,
                z: self.z_axis.x,
            },
            XYZ {
                x: self.x_axis.y,
                y: self.y_axis.y,
                z: self.z_axis.y,
            },
            XYZ {
                x: self.x_axis.z,
                y: self.y_axis.z,
                z: self.z_axis.z,
            },
        )
    }

    #[inline]
    fn mul_vector(&self, other: XYZ<T>) -> XYZ<T> {
        let mut res = self.x_axis.mul_scalar(other.x);
        res = self.y_axis.mul_scalar(other.y).add(res);
        res = self.z_axis.mul_scalar(other.z).add(res);
        res
    }

    #[inline]
    fn mul_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.mul_vector(other.x_axis),
            self.mul_vector(other.y_axis),
            self.mul_vector(other.z_axis),
        )
    }

    #[inline]
    fn mul_scalar(&self, other: T) -> Self {
        Self::from_cols(
            self.x_axis.mul_scalar(other),
            self.y_axis.mul_scalar(other),
            self.z_axis.mul_scalar(other),
        )
    }

    #[inline]
    fn add_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.x_axis.add(other.x_axis),
            self.y_axis.add(other.y_axis),
            self.z_axis.add(other.z_axis),
        )
    }

    #[inline]
    fn sub_matrix(&self, other: &Self) -> Self {
        Self::from_cols(
            self.x_axis.sub(other.x_axis),
            self.y_axis.sub(other.y_axis),
            self.z_axis.sub(other.z_axis),
        )
    }
}

impl<T: FloatEx> FloatMatrix3x3<T, XYZ<T>> for Vector3x3<XYZ<T>> {
    #[inline]
    fn inverse(&self) -> Self {
        let tmp0 = self.y_axis.cross(self.z_axis);
        let tmp1 = self.z_axis.cross(self.x_axis);
        let tmp2 = self.x_axis.cross(self.y_axis);
        let det = self.z_axis.dot_into_vec(tmp2);
        glam_assert!(det.cmpne(XYZ::ZERO).all());
        let inv_det = det.recip();
        // TODO: Work out if it's possible to get rid of the transpose
        Self::from_cols(tmp0.mul(inv_det), tmp1.mul(inv_det), tmp2.mul(inv_det)).transpose()
    }

    #[inline]
    fn transform_point2(&self, other: XY<T>) -> XY<T> {
        let mut res = self.x_axis.mul_scalar(other.x);
        res = self.y_axis.mul_scalar(other.y).add(res);
        res = self.z_axis.add(res);
        res.into()
    }

    #[inline]
    fn transform_vector2(&self, other: XY<T>) -> XY<T> {
        let mut res = self.x_axis.mul_scalar(other.x);
        res = self.y_axis.mul_scalar(other.y).add(res);
        res.into()
    }
}

impl<T: NumEx> MatrixConst for Vector4x4<XYZW<T>> {
    const ZERO: Self = Self {
        x_axis: XYZW::ZERO,
        y_axis: XYZW::ZERO,
        z_axis: XYZW::ZERO,
        w_axis: XYZW::ZERO,
    };
    const IDENTITY: Self = Self {
        x_axis: XYZW::X,
        y_axis: XYZW::Y,
        z_axis: XYZW::Z,
        w_axis: XYZW::W,
    };
}

impl<T: NumEx> Matrix<T> for Vector4x4<XYZW<T>> {}

impl<T: NumEx> Matrix4x4<T, XYZW<T>> for Vector4x4<XYZW<T>> {
    #[rustfmt::skip]
    #[inline(always)]
    fn from_cols(x_axis: XYZW<T>, y_axis: XYZW<T>, z_axis: XYZW<T>, w_axis: XYZW<T>) -> Self {
        Self { x_axis, y_axis, z_axis, w_axis }
    }

    #[inline(always)]
    fn x_axis(&self) -> &XYZW<T> {
        &self.x_axis
    }

    #[inline(always)]
    fn y_axis(&self) -> &XYZW<T> {
        &self.y_axis
    }

    #[inline(always)]
    fn z_axis(&self) -> &XYZW<T> {
        &self.z_axis
    }

    #[inline(always)]
    fn w_axis(&self) -> &XYZW<T> {
        &self.w_axis
    }

    #[inline(always)]
    fn as_ref_vector4x4(&self) -> &Vector4x4<XYZW<T>> {
        self
    }

    #[inline(always)]
    fn as_mut_vector4x4(&mut self) -> &mut Vector4x4<XYZW<T>> {
        self
    }

    #[inline]
    fn determinant(&self) -> T {
        let (m00, m01, m02, m03) = self.x_axis.into_tuple();
        let (m10, m11, m12, m13) = self.y_axis.into_tuple();
        let (m20, m21, m22, m23) = self.z_axis.into_tuple();
        let (m30, m31, m32, m33) = self.w_axis.into_tuple();

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
        let (m00, m01, m02, m03) = self.x_axis.into_tuple();
        let (m10, m11, m12, m13) = self.y_axis.into_tuple();
        let (m20, m21, m22, m23) = self.z_axis.into_tuple();
        let (m30, m31, m32, m33) = self.w_axis.into_tuple();

        Self {
            x_axis: Vector4::new(m00, m10, m20, m30),
            y_axis: Vector4::new(m01, m11, m21, m31),
            z_axis: Vector4::new(m02, m12, m22, m32),
            w_axis: Vector4::new(m03, m13, m23, m33),
        }
    }
}

impl<T: FloatEx> FloatMatrix4x4<T, XYZW<T>> for Vector4x4<XYZW<T>> {
    type SIMDVector3 = XYZ<T>;

    #[inline(always)]
    fn transform_float4_as_point3(&self, other: XYZ<T>) -> XYZ<T> {
        self.transform_point3(other)
    }

    #[inline(always)]
    fn transform_float4_as_vector3(&self, other: XYZ<T>) -> XYZ<T> {
        self.transform_vector3(other)
    }

    #[inline(always)]
    fn project_float4_as_point3(&self, other: XYZ<T>) -> XYZ<T> {
        self.project_point3(other)
    }

    fn inverse(&self) -> Self {
        let (m00, m01, m02, m03) = self.x_axis.into_tuple();
        let (m10, m11, m12, m13) = self.y_axis.into_tuple();
        let (m20, m21, m22, m23) = self.z_axis.into_tuple();
        let (m30, m31, m32, m33) = self.w_axis.into_tuple();

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

        let fac0: XYZW<T> = Vector4::new(coef00, coef00, coef02, coef03);
        let fac1: XYZW<T> = Vector4::new(coef04, coef04, coef06, coef07);
        let fac2: XYZW<T> = Vector4::new(coef08, coef08, coef10, coef11);
        let fac3: XYZW<T> = Vector4::new(coef12, coef12, coef14, coef15);
        let fac4: XYZW<T> = Vector4::new(coef16, coef16, coef18, coef19);
        let fac5: XYZW<T> = Vector4::new(coef20, coef20, coef22, coef23);

        let vec0: XYZW<T> = Vector4::new(m10, m00, m00, m00);
        let vec1: XYZW<T> = Vector4::new(m11, m01, m01, m01);
        let vec2: XYZW<T> = Vector4::new(m12, m02, m02, m02);
        let vec3: XYZW<T> = Vector4::new(m13, m03, m03, m03);

        let inv0 = vec1.mul(fac0).sub(vec2.mul(fac1)).add(vec3.mul(fac2));
        let inv1 = vec0.mul(fac0).sub(vec2.mul(fac3)).add(vec3.mul(fac4));
        let inv2 = vec0.mul(fac1).sub(vec1.mul(fac3)).add(vec3.mul(fac5));
        let inv3 = vec0.mul(fac2).sub(vec1.mul(fac4)).add(vec2.mul(fac5));

        let sign_a = Vector4::new(T::ONE, -T::ONE, T::ONE, -T::ONE);
        let sign_b = Vector4::new(-T::ONE, T::ONE, -T::ONE, T::ONE);

        let inverse = Self {
            x_axis: inv0.mul(sign_a),
            y_axis: inv1.mul(sign_b),
            z_axis: inv2.mul(sign_a),
            w_axis: inv3.mul(sign_b),
        };

        let col0 = Vector4::new(
            inverse.x_axis.x,
            inverse.y_axis.x,
            inverse.z_axis.x,
            inverse.w_axis.x,
        );

        let dot0 = self.x_axis().mul(col0);
        let dot1 = dot0.x + dot0.y + dot0.z + dot0.w;

        glam_assert!(dot1 != T::ZERO);

        let rcp_det = dot1.recip();
        inverse.mul_scalar(rcp_det)
    }
}

impl<T: FloatEx> ProjectionMatrix<T, XYZW<T>> for Vector4x4<XYZW<T>> {}
