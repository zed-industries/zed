use core::arch::wasm32::*;

// use super::float::*;
use crate::core::{
    storage::XYZ,
    traits::{quaternion::Quaternion, scalar::*, vector::*},
};

impl Quaternion<f32> for v128 {
    type SIMDVector3 = v128;

    #[inline(always)]
    fn conjugate(self) -> Self {
        const SIGN: v128 = const_f32x4!([-1.0, -1.0, -1.0, 1.0]);
        f32x4_mul(self, SIGN)
    }

    #[inline]
    fn lerp(self, end: Self, s: f32) -> Self {
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(end));

        const NEG_ZERO: v128 = const_f32x4!([-0.0; 4]);
        let start = self;
        let end = end;
        let dot = Vector4::dot_into_vec(start, end);
        // Calculate the bias, if the dot product is positive or zero, there is no bias
        // but if it is negative, we want to flip the 'end' rotation XYZW components
        let bias = v128_and(dot, NEG_ZERO);
        let interpolated = f32x4_add(
            f32x4_mul(f32x4_sub(v128_xor(end, bias), start), f32x4_splat(s)),
            start,
        );
        FloatVector4::normalize(interpolated)
    }

    #[inline]
    fn slerp(self, end: Self, s: f32) -> Self {
        // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(end));

        const DOT_THRESHOLD: f32 = 0.9995;

        let dot = Vector4::dot(self, end);

        if dot > DOT_THRESHOLD {
            // assumes lerp returns a normalized quaternion
            self.lerp(end, s)
        } else {
            // assumes scalar_acos clamps the input to [-1.0, 1.0]
            let theta = dot.acos_approx();

            // TODO: v128_sin is broken
            // let x = 1.0 - s;
            // let y = s;
            // let z = 1.0;
            // let w = 0.0;
            // let tmp = f32x4_mul(f32x4_splat(theta), f32x4(x, y, z, w));
            // let tmp = v128_sin(tmp);
            let x = (theta * (1.0 - s)).sin();
            let y = (theta * s).sin();
            let z = theta.sin();
            let w = 0.0;
            let tmp = f32x4(x, y, z, w);

            let scale1 = i32x4_shuffle::<0, 0, 4, 4>(tmp, tmp);
            let scale2 = i32x4_shuffle::<1, 1, 5, 5>(tmp, tmp);
            let theta_sin = i32x4_shuffle::<2, 2, 6, 6>(tmp, tmp);

            self.mul(scale1).add(end.mul(scale2)).div(theta_sin)
        }
    }

    #[inline]
    fn mul_quaternion(self, other: Self) -> Self {
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(other));
        // Based on https://github.com/nfrechette/rtm `rtm::quat_mul`
        let lhs = self;
        let rhs = other;

        const CONTROL_WZYX: v128 = const_f32x4!([1.0, -1.0, 1.0, -1.0]);
        const CONTROL_ZWXY: v128 = const_f32x4!([1.0, 1.0, -1.0, -1.0]);
        const CONTROL_YXWZ: v128 = const_f32x4!([-1.0, 1.0, 1.0, -1.0]);

        let r_xxxx = i32x4_shuffle::<0, 0, 4, 4>(lhs, lhs);
        let r_yyyy = i32x4_shuffle::<1, 1, 5, 5>(lhs, lhs);
        let r_zzzz = i32x4_shuffle::<2, 2, 6, 6>(lhs, lhs);
        let r_wwww = i32x4_shuffle::<3, 3, 7, 7>(lhs, lhs);

        let lxrw_lyrw_lzrw_lwrw = f32x4_mul(r_wwww, rhs);
        let l_wzyx = i32x4_shuffle::<3, 2, 5, 4>(rhs, rhs);

        let lwrx_lzrx_lyrx_lxrx = f32x4_mul(r_xxxx, l_wzyx);
        let l_zwxy = i32x4_shuffle::<1, 0, 7, 6>(l_wzyx, l_wzyx);

        let lwrx_nlzrx_lyrx_nlxrx = f32x4_mul(lwrx_lzrx_lyrx_lxrx, CONTROL_WZYX);

        let lzry_lwry_lxry_lyry = f32x4_mul(r_yyyy, l_zwxy);
        let l_yxwz = i32x4_shuffle::<3, 2, 5, 4>(l_zwxy, l_zwxy);

        let lzry_lwry_nlxry_nlyry = f32x4_mul(lzry_lwry_lxry_lyry, CONTROL_ZWXY);

        let lyrz_lxrz_lwrz_lzrz = f32x4_mul(r_zzzz, l_yxwz);
        let result0 = f32x4_add(lxrw_lyrw_lzrw_lwrw, lwrx_nlzrx_lyrx_nlxrx);

        let nlyrz_lxrz_lwrz_wlzrz = f32x4_mul(lyrz_lxrz_lwrz_lzrz, CONTROL_YXWZ);
        let result1 = f32x4_add(lzry_lwry_nlxry_nlyry, nlyrz_lxrz_lwrz_wlzrz);
        f32x4_add(result0, result1)
    }

    #[inline]
    fn mul_vector3(self, other: XYZ<f32>) -> XYZ<f32> {
        self.mul_float4_as_vector3(other.into()).into()
    }

    #[inline]
    fn mul_float4_as_vector3(self, other: v128) -> v128 {
        glam_assert!(FloatVector4::is_normalized(self));
        const TWO: v128 = const_f32x4!([2.0; 4]);
        let w = i32x4_shuffle::<3, 3, 7, 7>(self, self);
        let b = self;
        let b2 = Vector3::dot_into_vec(b, b);
        other
            .mul(w.mul(w).sub(b2))
            .add(b.mul(Vector3::dot_into_vec(other, b).mul(TWO)))
            .add(b.cross(other).mul(w.mul(TWO)))
    }
}
