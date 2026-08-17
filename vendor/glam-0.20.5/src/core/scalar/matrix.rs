use crate::core::{
    storage::{Columns2, Columns3, Columns4, XY, XYZ, XYZF32A16, XYZW},
    traits::{
        matrix::{
            FloatMatrix2x2, FloatMatrix3x3, FloatMatrix4x4, Matrix, Matrix2x2, Matrix3x3,
            Matrix4x4, MatrixConst,
        },
        projection::ProjectionMatrix,
        scalar::{FloatEx, NanConstEx, NumEx},
        vector::*,
    },
};

impl<T: NumEx> MatrixConst for Columns2<XY<T>> {
    const ZERO: Self = Self {
        x_axis: XY::ZERO,
        y_axis: XY::ZERO,
    };
    const IDENTITY: Self = Self {
        x_axis: XY::X,
        y_axis: XY::Y,
    };
}

impl<T: NanConstEx> NanConstEx for Columns2<XY<T>> {
    const NAN: Self = Self {
        x_axis: XY::NAN,
        y_axis: XY::NAN,
    };
}

impl<T: NumEx> Matrix<T> for Columns2<XY<T>> {}

impl<T: NumEx> Matrix2x2<T, XY<T>> for Columns2<XY<T>> {
    #[inline(always)]
    fn from_cols(x_axis: XY<T>, y_axis: XY<T>) -> Self {
        Self { x_axis, y_axis }
    }

    #[inline(always)]
    fn x_axis(&self) -> &XY<T> {
        &self.x_axis
    }

    #[inline(always)]
    fn y_axis(&self) -> &XY<T> {
        &self.y_axis
    }
}

impl<T: FloatEx> FloatMatrix2x2<T, XY<T>> for Columns2<XY<T>> {}

impl<T: NumEx> MatrixConst for Columns3<XYZ<T>> {
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

impl<T: NanConstEx> NanConstEx for Columns3<XYZ<T>> {
    const NAN: Self = Self {
        x_axis: XYZ::NAN,
        y_axis: XYZ::NAN,
        z_axis: XYZ::NAN,
    };
}

impl<T: NumEx> Matrix<T> for Columns3<XYZ<T>> {}

impl<T: NumEx> Matrix3x3<T, XYZ<T>> for Columns3<XYZ<T>> {
    #[inline(always)]
    fn from_cols(x_axis: XYZ<T>, y_axis: XYZ<T>, z_axis: XYZ<T>) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    fn x_axis(&self) -> &XYZ<T> {
        &self.x_axis
    }

    #[inline(always)]
    fn y_axis(&self) -> &XYZ<T> {
        &self.y_axis
    }

    #[inline(always)]
    fn z_axis(&self) -> &XYZ<T> {
        &self.z_axis
    }

    #[inline]
    fn mul_vector(&self, other: XYZ<T>) -> XYZ<T> {
        // default implementation uses splat_x etc, which might not be optimal. Need to check.
        let mut res = self.x_axis.mul_scalar(other.x);
        res = self.y_axis.mul_scalar(other.y).add(res);
        res = self.z_axis.mul_scalar(other.z).add(res);
        res
    }
}

impl<T: FloatEx> FloatMatrix3x3<T, XYZ<T>> for Columns3<XYZ<T>> {
    #[inline]
    fn transform_point2(&self, other: XY<T>) -> XY<T> {
        // TODO: This is untested, probably slower than the high level code that uses a SIMD mat2
        Columns2::from_cols(self.x_axis.into(), self.y_axis.into())
            .mul_vector(other)
            .add(self.z_axis.into())
    }

    #[inline]
    fn transform_vector2(&self, other: XY<T>) -> XY<T> {
        // TODO: This is untested, probably slower than the high level code that uses a SIMD mat2
        Columns2::from_cols(self.x_axis.into(), self.y_axis.into()).mul_vector(other)
    }
}

impl MatrixConst for Columns3<XYZF32A16> {
    const ZERO: Self = Self {
        x_axis: XYZF32A16::ZERO,
        y_axis: XYZF32A16::ZERO,
        z_axis: XYZF32A16::ZERO,
    };
    const IDENTITY: Self = Self {
        x_axis: XYZF32A16::X,
        y_axis: XYZF32A16::Y,
        z_axis: XYZF32A16::Z,
    };
}

impl NanConstEx for Columns3<XYZF32A16> {
    const NAN: Self = Self {
        x_axis: XYZF32A16::NAN,
        y_axis: XYZF32A16::NAN,
        z_axis: XYZF32A16::NAN,
    };
}

impl Matrix<f32> for Columns3<XYZF32A16> {}

impl Matrix3x3<f32, XYZF32A16> for Columns3<XYZF32A16> {
    #[inline(always)]
    fn from_cols(x_axis: XYZF32A16, y_axis: XYZF32A16, z_axis: XYZF32A16) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline(always)]
    fn x_axis(&self) -> &XYZF32A16 {
        &self.x_axis
    }

    #[inline(always)]
    fn y_axis(&self) -> &XYZF32A16 {
        &self.y_axis
    }

    #[inline(always)]
    fn z_axis(&self) -> &XYZF32A16 {
        &self.z_axis
    }
}

impl FloatMatrix3x3<f32, XYZF32A16> for Columns3<XYZF32A16> {
    #[inline]
    fn transform_point2(&self, other: XY<f32>) -> XY<f32> {
        // TODO: This is untested, probably slower than the high level code that uses a SIMD mat2
        Columns2::from_cols(self.x_axis.into_xy(), self.y_axis.into_xy())
            .mul_vector(other)
            .add(self.z_axis.into_xy())
    }

    #[inline]
    fn transform_vector2(&self, other: XY<f32>) -> XY<f32> {
        // TODO: This is untested, probably slower than the high level code that uses a SIMD mat2
        Columns2::from_cols(self.x_axis.into_xy(), self.y_axis.into_xy()).mul_vector(other)
    }
}

impl<T: NumEx> MatrixConst for Columns4<XYZW<T>> {
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

impl<T: NanConstEx> NanConstEx for Columns4<XYZW<T>> {
    const NAN: Self = Self {
        x_axis: XYZW::NAN,
        y_axis: XYZW::NAN,
        z_axis: XYZW::NAN,
        w_axis: XYZW::NAN,
    };
}

impl<T: NumEx> Matrix<T> for Columns4<XYZW<T>> {}

impl<T: NumEx> Matrix4x4<T, XYZW<T>> for Columns4<XYZW<T>> {
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
}

impl<T: FloatEx> FloatMatrix4x4<T, XYZW<T>> for Columns4<XYZW<T>> {
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
}

impl<T: FloatEx> ProjectionMatrix<T, XYZW<T>> for Columns4<XYZW<T>> {}

impl From<Columns3<XYZ<f32>>> for Columns3<XYZF32A16> {
    fn from(v: Columns3<XYZ<f32>>) -> Columns3<XYZF32A16> {
        Self {
            x_axis: v.x_axis.into(),
            y_axis: v.y_axis.into(),
            z_axis: v.z_axis.into(),
        }
    }
}

impl From<Columns3<XYZF32A16>> for Columns3<XYZ<f32>> {
    fn from(v: Columns3<XYZF32A16>) -> Columns3<XYZ<f32>> {
        Self {
            x_axis: v.x_axis.into(),
            y_axis: v.y_axis.into(),
            z_axis: v.z_axis.into(),
        }
    }
}
