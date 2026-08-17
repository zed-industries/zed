use crate::core::{
    storage::{XYZ, XYZW},
    traits::{quaternion::Quaternion, scalar::*, vector::*},
};

impl<T: FloatEx> Quaternion<T> for XYZW<T> {
    // fallback
    type SIMDVector3 = XYZ<T>;

    #[inline(always)]
    fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    #[inline]
    fn lerp(self, end: Self, s: T) -> Self {
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(end));

        let start = self;
        let end = end;
        let dot = start.dot(end);
        let bias = if dot >= T::ZERO { T::ONE } else { T::NEG_ONE };
        let interpolated = start.add(end.mul_scalar(bias).sub(start).mul_scalar(s));
        interpolated.normalize()
    }

    #[inline]
    fn slerp(self, mut end: Self, s: T) -> Self {
        // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/

        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(end));

        let mut dot = self.dot(end);

        // Note that a rotation can be represented by two quaternions: `q` and
        // `-q`. The slerp path between `q` and `end` will be different from the
        // path between `-q` and `end`. One path will take the long way around and
        // one will take the short way. In order to correct for this, the `dot`
        // product between `self` and `end` should be positive. If the `dot`
        // product is negative, slerp between `self` and `-end`.
        if dot < T::ZERO {
            end = end.mul_scalar(T::NEG_ONE);
            dot = -dot;
        }

        if dot > T::from_f32(0.9995) {
            // assumes lerp returns a normalized quaternion
            self.lerp(end, s)
        } else {
            // assumes scalar_acos clamps the input to [-1.0, 1.0]
            let theta = dot.acos_approx();
            let scale1 = (theta * (T::ONE - s)).sin();
            let scale2 = (theta * s).sin();
            let theta_sin = theta.sin();

            self.mul_scalar(scale1)
                .add(end.mul_scalar(scale2))
                .mul_scalar(theta_sin.recip())
        }
    }

    #[inline]
    fn mul_vector3(self, other: XYZ<T>) -> XYZ<T> {
        glam_assert!(FloatVector4::is_normalized(self));
        let w = self.w;
        let b = XYZ {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        let b2 = b.dot(b);
        other
            .mul_scalar(w * w - b2)
            .add(b.mul_scalar(other.dot(b) * T::TWO))
            .add(b.cross(other).mul_scalar(w * T::TWO))
    }

    #[inline]
    fn mul_quaternion(self, other: Self) -> Self {
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(other));
        let (x0, y0, z0, w0) = self.into_tuple();
        let (x1, y1, z1, w1) = other.into_tuple();
        Self::new(
            w0 * x1 + x0 * w1 + y0 * z1 - z0 * y1,
            w0 * y1 - x0 * z1 + y0 * w1 + z0 * x1,
            w0 * z1 + x0 * y1 - y0 * x1 + z0 * w1,
            w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
        )
    }

    #[inline(always)]
    fn mul_float4_as_vector3(self, other: XYZ<T>) -> XYZ<T> {
        self.mul_vector3(other)
    }
}
