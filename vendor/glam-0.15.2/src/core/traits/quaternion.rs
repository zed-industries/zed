use crate::core::{
    storage::XYZ,
    traits::{
        scalar::{FloatEx, NumEx},
        vector::*,
    },
};

pub trait Quaternion<T: FloatEx>: FloatVector4<T> {
    type SIMDVector3;

    #[inline]
    fn from_axis_angle(axis: XYZ<T>, angle: T) -> Self {
        glam_assert!(FloatVector3::is_normalized(axis));
        let (s, c) = (angle * T::HALF).sin_cos();
        let v = axis.mul_scalar(s);
        Self::new(v.x, v.y, v.z, c)
    }

    #[inline]
    fn from_rotation_x(angle: T) -> Self {
        let (s, c) = (angle * T::HALF).sin_cos();
        Self::new(s, T::ZERO, T::ZERO, c)
    }

    #[inline]
    fn from_rotation_y(angle: T) -> Self {
        let (s, c) = (angle * T::HALF).sin_cos();
        Self::new(T::ZERO, s, T::ZERO, c)
    }

    #[inline]
    fn from_rotation_z(angle: T) -> Self {
        let (s, c) = (angle * T::HALF).sin_cos();
        Self::new(T::ZERO, T::ZERO, s, c)
    }

    /// From the columns of a 3x3 rotation matrix.
    #[inline]
    fn from_rotation_axes(x_axis: XYZ<T>, y_axis: XYZ<T>, z_axis: XYZ<T>) -> Self {
        // Based on https://github.com/microsoft/DirectXMath `XM$quaternionRotationMatrix`
        // TODO: sse2 version
        let (m00, m01, m02) = x_axis.into_tuple();
        let (m10, m11, m12) = y_axis.into_tuple();
        let (m20, m21, m22) = z_axis.into_tuple();
        if m22 <= T::ZERO {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = T::ONE - m22;
            if dif10 <= T::ZERO {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = T::HALF / four_xsq.sqrt();
                Self::new(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = T::HALF / four_ysq.sqrt();
                Self::new(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = T::ONE + m22;
            if sum10 <= T::ZERO {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = T::HALF / four_zsq.sqrt();
                Self::new(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = T::HALF / four_wsq.sqrt();
                Self::new(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    fn to_axis_angle(self) -> (XYZ<T>, T) {
        // const EPSILON: f32 = 1.0e-8;
        // const EPSILON_SQUARED: f32 = EPSILON * EPSILON;
        let (x, y, z, w) = Vector4::into_tuple(self);
        let angle = w.acos_approx() * T::TWO;
        let scale_sq = NumEx::max(T::ONE - w * w, T::ZERO);
        // TODO: constants for epslions?
        if scale_sq >= T::from_f32(1.0e-8 * 1.0e-8) {
            (XYZ { x, y, z }.mul_scalar(scale_sq.sqrt().recip()), angle)
        } else {
            (Vector3Const::X, angle)
        }
    }

    #[inline]
    fn is_near_identity(self) -> bool {
        // Based on https://github.com/nfrechette/rtm `rtm::quat_near_identity`
        let threshold_angle = T::from_f64(0.002_847_144_6);
        // Because of floating point precision, we cannot represent very small rotations.
        // The closest f32 to 1.0 that is not 1.0 itself yields:
        // 0.99999994.acos() * 2.0  = 0.000690533954 rad
        //
        // An error threshold of 1.e-6 is used by default.
        // (1.0 - 1.e-6).acos() * 2.0 = 0.00284714461 rad
        // (1.0 - 1.e-7).acos() * 2.0 = 0.00097656250 rad
        //
        // We don't really care about the angle value itself, only if it's close to 0.
        // This will happen whenever quat.w is close to 1.0.
        // If the quat.w is close to -1.0, the angle will be near 2*PI which is close to
        // a negative 0 rotation. By forcing quat.w to be positive, we'll end up with
        // the shortest path.
        let positive_w_angle = self.as_ref_xyzw().w.abs().acos_approx() * T::TWO;
        positive_w_angle < threshold_angle
    }

    fn conjugate(self) -> Self;
    fn lerp(self, end: Self, s: T) -> Self;
    fn slerp(self, end: Self, s: T) -> Self;
    fn mul_quaternion(self, other: Self) -> Self;
    fn mul_vector3(self, other: XYZ<T>) -> XYZ<T>;
    fn mul_float4_as_vector3(self, other: Self::SIMDVector3) -> Self::SIMDVector3;
}
