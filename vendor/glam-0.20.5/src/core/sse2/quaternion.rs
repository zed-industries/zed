#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use super::float::*;
use crate::core::{
    storage::XYZ,
    traits::{quaternion::Quaternion, scalar::*, vector::*},
};

impl Quaternion<f32> for __m128 {
    type SIMDVector3 = __m128;

    #[inline(always)]
    fn conjugate(self) -> Self {
        const SIGN: __m128 = const_f32x4!([-0.0, -0.0, -0.0, 0.0]);
        unsafe { _mm_xor_ps(self, SIGN) }
    }

    #[inline]
    fn lerp(self, end: Self, s: f32) -> Self {
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(end));

        unsafe {
            const NEG_ZERO: __m128 = const_f32x4!([-0.0; 4]);
            let start = self;
            let end = end;
            let dot = Vector4::dot_into_vec(start, end);
            // Calculate the bias, if the dot product is positive or zero, there is no bias
            // but if it is negative, we want to flip the 'end' rotation XYZW components
            let bias = _mm_and_ps(dot, NEG_ZERO);
            let interpolated = _mm_add_ps(
                _mm_mul_ps(_mm_sub_ps(_mm_xor_ps(end, bias), start), _mm_set_ps1(s)),
                start,
            );
            FloatVector4::normalize(interpolated)
        }
    }

    #[inline]
    fn slerp(self, mut end: Self, s: f32) -> Self {
        // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(end));

        const DOT_THRESHOLD: f32 = 0.9995;

        let mut dot = Vector4::dot(self, end);

        // Note that a rotation can be represented by two quaternions: `q` and
        // `-q`. The slerp path between `q` and `end` will be different from the
        // path between `-q` and `end`. One path will take the long way around and
        // one will take the short way. In order to correct for this, the `dot`
        // product between `self` and `end` should be positive. If the `dot`
        // product is negative, slerp between `self` and `-end`.
        if dot < 0.0 {
            const PS_NEGATIVE_ONE: __m128 = const_f32x4!([-1.0; 4]);
            end = unsafe { _mm_mul_ps(PS_NEGATIVE_ONE, end) };
            dot = -dot;
        }

        if dot > DOT_THRESHOLD {
            // assumes lerp returns a normalized quaternion
            self.lerp(end, s)
        } else {
            // assumes scalar_acos clamps the input to [-1.0, 1.0]
            let theta = dot.acos_approx();

            let x = 1.0 - s;
            let y = s;
            let z = 1.0;

            unsafe {
                let tmp = _mm_mul_ps(_mm_set_ps1(theta), _mm_set_ps(0.0, z, y, x));
                let tmp = m128_sin(tmp);

                let scale1 = _mm_shuffle_ps(tmp, tmp, 0b00_00_00_00);
                let scale2 = _mm_shuffle_ps(tmp, tmp, 0b01_01_01_01);
                let theta_sin = _mm_shuffle_ps(tmp, tmp, 0b10_10_10_10);

                self.mul(scale1).add(end.mul(scale2)).div(theta_sin)
            }
        }
    }

    #[inline]
    fn mul_quaternion(self, other: Self) -> Self {
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(other));
        unsafe {
            // Based on https://github.com/nfrechette/rtm `rtm::quat_mul`
            let lhs = self;
            let rhs = other;

            const CONTROL_WZYX: __m128 = const_f32x4!([1.0, -1.0, 1.0, -1.0]);
            const CONTROL_ZWXY: __m128 = const_f32x4!([1.0, 1.0, -1.0, -1.0]);
            const CONTROL_YXWZ: __m128 = const_f32x4!([-1.0, 1.0, 1.0, -1.0]);

            let r_xxxx = _mm_shuffle_ps(lhs, lhs, 0b00_00_00_00);
            let r_yyyy = _mm_shuffle_ps(lhs, lhs, 0b01_01_01_01);
            let r_zzzz = _mm_shuffle_ps(lhs, lhs, 0b10_10_10_10);
            let r_wwww = _mm_shuffle_ps(lhs, lhs, 0b11_11_11_11);

            let lxrw_lyrw_lzrw_lwrw = _mm_mul_ps(r_wwww, rhs);
            let l_wzyx = _mm_shuffle_ps(rhs, rhs, 0b00_01_10_11);

            let lwrx_lzrx_lyrx_lxrx = _mm_mul_ps(r_xxxx, l_wzyx);
            let l_zwxy = _mm_shuffle_ps(l_wzyx, l_wzyx, 0b10_11_00_01);

            let lwrx_nlzrx_lyrx_nlxrx = _mm_mul_ps(lwrx_lzrx_lyrx_lxrx, CONTROL_WZYX);

            let lzry_lwry_lxry_lyry = _mm_mul_ps(r_yyyy, l_zwxy);
            let l_yxwz = _mm_shuffle_ps(l_zwxy, l_zwxy, 0b00_01_10_11);

            let lzry_lwry_nlxry_nlyry = _mm_mul_ps(lzry_lwry_lxry_lyry, CONTROL_ZWXY);

            let lyrz_lxrz_lwrz_lzrz = _mm_mul_ps(r_zzzz, l_yxwz);
            let result0 = _mm_add_ps(lxrw_lyrw_lzrw_lwrw, lwrx_nlzrx_lyrx_nlxrx);

            let nlyrz_lxrz_lwrz_wlzrz = _mm_mul_ps(lyrz_lxrz_lwrz_lzrz, CONTROL_YXWZ);
            let result1 = _mm_add_ps(lzry_lwry_nlxry_nlyry, nlyrz_lxrz_lwrz_wlzrz);
            _mm_add_ps(result0, result1)
        }
    }

    #[inline]
    fn mul_vector3(self, other: XYZ<f32>) -> XYZ<f32> {
        self.mul_float4_as_vector3(other.into()).into()
    }

    #[inline]
    fn mul_float4_as_vector3(self, other: __m128) -> __m128 {
        glam_assert!(FloatVector4::is_normalized(self));
        unsafe {
            const TWO: __m128 = const_f32x4!([2.0; 4]);
            let w = _mm_shuffle_ps(self, self, 0b11_11_11_11);
            let b = self;
            let b2 = Vector3::dot_into_vec(b, b);
            other
                .mul(w.mul(w).sub(b2))
                .add(b.mul(Vector3::dot_into_vec(other, b).mul(TWO)))
                .add(b.cross(other).mul(w.mul(TWO)))
        }
    }
}
