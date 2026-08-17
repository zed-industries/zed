#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

macro_rules! const_u32x4 {
    ($ux4:expr) => {
        unsafe { $crate::cast::UVec4Cast { ux4: $ux4 }.m128 }
    };
}

const PS_INV_SIGN_MASK: __m128 = const_u32x4!([!0x8000_0000; 4]);
const PS_SIGN_MASK: __m128 = const_u32x4!([0x8000_0000; 4]);
const PS_NO_FRACTION: __m128 = const_f32x4!([8388608.0; 4]);
const PS_NEGATIVE_ZERO: __m128 = const_u32x4!([0x8000_0000; 4]);
const PS_PI: __m128 = const_f32x4!([core::f32::consts::PI; 4]);
const PS_HALF_PI: __m128 = const_f32x4!([core::f32::consts::FRAC_PI_2; 4]);
const PS_SIN_COEFFICIENTS0: __m128 =
    const_f32x4!([-0.16666667, 0.008_333_331, -0.00019840874, 2.752_556_2e-6]);
const PS_SIN_COEFFICIENTS1: __m128 = const_f32x4!([
    -2.388_985_9e-8,
    -0.16665852,      /*Est1*/
    0.008_313_95,     /*Est2*/
    -0.000_185_246_7  /*Est3*/
]);
const PS_ONE: __m128 = const_f32x4!([1.0; 4]);
const PS_TWO_PI: __m128 = const_f32x4!([core::f32::consts::TAU; 4]);
const PS_RECIPROCAL_TWO_PI: __m128 = const_f32x4!([0.159_154_94; 4]);

#[inline]
pub(crate) unsafe fn m128_abs(v: __m128) -> __m128 {
    _mm_and_ps(v, _mm_castsi128_ps(_mm_set1_epi32(0x7f_ff_ff_ff)))
}

#[inline]
pub(crate) unsafe fn m128_round(v: __m128) -> __m128 {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorRound`
    let sign = _mm_and_ps(v, PS_SIGN_MASK);
    let s_magic = _mm_or_ps(PS_NO_FRACTION, sign);
    let r1 = _mm_add_ps(v, s_magic);
    let r1 = _mm_sub_ps(r1, s_magic);
    let r2 = _mm_and_ps(v, PS_INV_SIGN_MASK);
    let mask = _mm_cmple_ps(r2, PS_NO_FRACTION);
    let r2 = _mm_andnot_ps(mask, v);
    let r1 = _mm_and_ps(r1, mask);
    _mm_xor_ps(r1, r2)
}

#[inline]
pub(crate) unsafe fn m128_floor(v: __m128) -> __m128 {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorFloor`
    // To handle NAN, INF and numbers greater than 8388608, use masking
    let test = _mm_and_si128(_mm_castps_si128(v), _mm_castps_si128(PS_INV_SIGN_MASK));
    let test = _mm_cmplt_epi32(test, _mm_castps_si128(PS_NO_FRACTION));
    // Truncate
    let vint = _mm_cvttps_epi32(v);
    let result = _mm_cvtepi32_ps(vint);
    let larger = _mm_cmpgt_ps(result, v);
    // 0 -> 0, 0xffffffff -> -1.0f
    let larger = _mm_cvtepi32_ps(_mm_castps_si128(larger));
    let result = _mm_add_ps(result, larger);
    // All numbers less than 8388608 will use the round to int
    let result = _mm_and_ps(result, _mm_castsi128_ps(test));
    // All others, use the ORIGINAL value
    let test = _mm_andnot_si128(test, _mm_castps_si128(v));
    _mm_or_ps(result, _mm_castsi128_ps(test))
}

#[inline]
pub(crate) unsafe fn m128_ceil(v: __m128) -> __m128 {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorCeil`
    // To handle NAN, INF and numbers greater than 8388608, use masking
    let test = _mm_and_si128(_mm_castps_si128(v), _mm_castps_si128(PS_INV_SIGN_MASK));
    let test = _mm_cmplt_epi32(test, _mm_castps_si128(PS_NO_FRACTION));
    // Truncate
    let vint = _mm_cvttps_epi32(v);
    let result = _mm_cvtepi32_ps(vint);
    let smaller = _mm_cmplt_ps(result, v);
    // 0 -> 0, 0xffffffff -> -1.0f
    let smaller = _mm_cvtepi32_ps(_mm_castps_si128(smaller));
    let result = _mm_sub_ps(result, smaller);
    // All numbers less than 8388608 will use the round to int
    let result = _mm_and_ps(result, _mm_castsi128_ps(test));
    // All others, use the ORIGINAL value
    let test = _mm_andnot_si128(test, _mm_castps_si128(v));
    _mm_or_ps(result, _mm_castsi128_ps(test))
}

#[inline(always)]
pub(crate) unsafe fn m128_mul_add(a: __m128, b: __m128, c: __m128) -> __m128 {
    // Only enable fused multiply-adds here if "fast-math" is enabled and the
    // platform supports it. Otherwise this may break cross-platform determinism.
    #[cfg(all(feature = "fast-math", target_feature = "fma"))]
    {
        _mm_fmadd_ps(a, b, c)
    }

    #[cfg(any(not(feature = "fast-math"), not(target_feature = "fma")))]
    {
        _mm_add_ps(_mm_mul_ps(a, b), c)
    }
}

#[inline(always)]
pub(crate) unsafe fn m128_neg_mul_sub(a: __m128, b: __m128, c: __m128) -> __m128 {
    _mm_sub_ps(c, _mm_mul_ps(a, b))
}

/// Returns a vector whose components are the corresponding components of Angles modulo 2PI.
#[inline]
pub(crate) unsafe fn m128_mod_angles(angles: __m128) -> __m128 {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorModAngles`
    let v = _mm_mul_ps(angles, PS_RECIPROCAL_TWO_PI);
    let v = m128_round(v);
    m128_neg_mul_sub(PS_TWO_PI, v, angles)
}

/// Computes the sine of the angle in each lane of `v`. Values outside
/// the bounds of PI may produce an increasing error as the input angle
/// drifts from `[-PI, PI]`.
#[inline]
pub(crate) unsafe fn m128_sin(v: __m128) -> __m128 {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorSin`

    // 11-degree minimax approximation

    // Force the value within the bounds of pi
    let mut x = m128_mod_angles(v);

    // Map in [-pi/2,pi/2] with sin(y) = sin(x).
    let sign = _mm_and_ps(x, PS_NEGATIVE_ZERO);
    // pi when x >= 0, -pi when x < 0
    let c = _mm_or_ps(PS_PI, sign);
    // |x|
    let absx = _mm_andnot_ps(sign, x);
    let rflx = _mm_sub_ps(c, x);
    let comp = _mm_cmple_ps(absx, PS_HALF_PI);
    let select0 = _mm_and_ps(comp, x);
    let select1 = _mm_andnot_ps(comp, rflx);
    x = _mm_or_ps(select0, select1);

    let x2 = _mm_mul_ps(x, x);

    // Compute polynomial approximation
    const SC1: __m128 = PS_SIN_COEFFICIENTS1;
    let v_constants_b = _mm_shuffle_ps(SC1, SC1, 0b00_00_00_00);

    const SC0: __m128 = PS_SIN_COEFFICIENTS0;
    let mut v_constants = _mm_shuffle_ps(SC0, SC0, 0b11_11_11_11);
    let mut result = m128_mul_add(v_constants_b, x2, v_constants);

    v_constants = _mm_shuffle_ps(SC0, SC0, 0b10_10_10_10);
    result = m128_mul_add(result, x2, v_constants);

    v_constants = _mm_shuffle_ps(SC0, SC0, 0b01_01_01_01);
    result = m128_mul_add(result, x2, v_constants);

    v_constants = _mm_shuffle_ps(SC0, SC0, 0b00_00_00_00);
    result = m128_mul_add(result, x2, v_constants);

    result = m128_mul_add(result, x2, PS_ONE);
    result = _mm_mul_ps(result, x);

    result
}

// Based on http://gruntthepeon.free.fr/ssemath/sse_mathfun.h
// #[cfg(target_feature = "sse2")]
// unsafe fn sin_cos_sse2(x: __m128) -> (__m128, __m128) {
//     let mut sign_bit_sin = x;
//     // take the absolute value
//     let mut x = _mm_and_ps(x, PS_INV_SIGN_MASK.m128);
//     // extract the sign bit (upper one)
//     sign_bit_sin = _mm_and_ps(sign_bit_sin, PS_SIGN_MASK.m128);

//     // scale by 4/Pi
//     let mut y = _mm_mul_ps(x, PS_CEPHES_FOPI.m128);

//     // store the integer part of y in emm2
//     let mut emm2 = _mm_cvttps_epi32(y);

//     // j=(j+1) & (~1) (see the cephes sources)
//     emm2 = _mm_add_epi32(emm2, PI32_1.m128i);
//     emm2 = _mm_and_si128(emm2, PI32_INV_1.m128i);
//     y = _mm_cvtepi32_ps(emm2);

//     let mut emm4 = emm2;

//     /* get the swap sign flag for the sine */
//     let mut emm0 = _mm_and_si128(emm2, PI32_4.m128i);
//     emm0 = _mm_slli_epi32(emm0, 29);
//     let swap_sign_bit_sin = _mm_castsi128_ps(emm0);

//     /* get the polynom selection mask for the sine*/
//     emm2 = _mm_and_si128(emm2, PI32_2.m128i);
//     emm2 = _mm_cmpeq_epi32(emm2, _mm_setzero_si128());
//     let poly_mask = _mm_castsi128_ps(emm2);

//     /* The magic pass: "Extended precision modular arithmetic"
//     x = ((x - y * DP1) - y * DP2) - y * DP3; */
//     let mut xmm1 = PS_MINUS_CEPHES_DP1.m128;
//     let mut xmm2 = PS_MINUS_CEPHES_DP2.m128;
//     let mut xmm3 = PS_MINUS_CEPHES_DP3.m128;
//     xmm1 = _mm_mul_ps(y, xmm1);
//     xmm2 = _mm_mul_ps(y, xmm2);
//     xmm3 = _mm_mul_ps(y, xmm3);
//     x = _mm_add_ps(x, xmm1);
//     x = _mm_add_ps(x, xmm2);
//     x = _mm_add_ps(x, xmm3);

//     emm4 = _mm_sub_epi32(emm4, PI32_2.m128i);
//     emm4 = _mm_andnot_si128(emm4, PI32_4.m128i);
//     emm4 = _mm_slli_epi32(emm4, 29);
//     let sign_bit_cos = _mm_castsi128_ps(emm4);

//     sign_bit_sin = _mm_xor_ps(sign_bit_sin, swap_sign_bit_sin);

//     // Evaluate the first polynom  (0 <= x <= Pi/4)
//     let z = _mm_mul_ps(x, x);
//     y = PS_COSCOF_P0.m128;

//     y = _mm_mul_ps(y, z);
//     y = _mm_add_ps(y, PS_COSCOF_P1.m128);
//     y = _mm_mul_ps(y, z);
//     y = _mm_add_ps(y, PS_COSCOF_P2.m128);
//     y = _mm_mul_ps(y, z);
//     y = _mm_mul_ps(y, z);
//     let tmp = _mm_mul_ps(z, PS_0_5.m128);
//     y = _mm_sub_ps(y, tmp);
//     y = _mm_add_ps(y, PS_1_0.m128);

//     // Evaluate the second polynom  (Pi/4 <= x <= 0)
//     let mut y2 = PS_SINCOF_P0.m128;
//     y2 = _mm_mul_ps(y2, z);
//     y2 = _mm_add_ps(y2, PS_SINCOF_P1.m128);
//     y2 = _mm_mul_ps(y2, z);
//     y2 = _mm_add_ps(y2, PS_SINCOF_P2.m128);
//     y2 = _mm_mul_ps(y2, z);
//     y2 = _mm_mul_ps(y2, x);
//     y2 = _mm_add_ps(y2, x);

//     // select the correct result from the two polynoms
//     xmm3 = poly_mask;
//     let ysin2 = _mm_and_ps(xmm3, y2);
//     let ysin1 = _mm_andnot_ps(xmm3, y);
//     y2 = _mm_sub_ps(y2, ysin2);
//     y = _mm_sub_ps(y, ysin1);

//     xmm1 = _mm_add_ps(ysin1, ysin2);
//     xmm2 = _mm_add_ps(y, y2);

//     // update the sign
//     (
//         _mm_xor_ps(xmm1, sign_bit_sin),
//         _mm_xor_ps(xmm2, sign_bit_cos),
//     )
// }

#[test]
fn test_sse2_m128_sin() {
    use crate::core::traits::vector::*;
    use core::f32::consts::PI;

    fn test_sse2_m128_sin_angle(a: f32) {
        let v = unsafe { m128_sin(_mm_set_ps1(a)) };
        let v = v.as_ref_xyzw();
        let a_sin = a.sin();
        // dbg!((a, a_sin, v));
        assert!(v.abs_diff_eq(Vector::splat(a_sin), 1e-6));
    }

    let mut a = -PI;
    let end = PI;
    let step = PI / 8192.0;

    while a <= end {
        test_sse2_m128_sin_angle(a);
        a += step;
    }
}
