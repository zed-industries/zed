#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[repr(C)]
union UnionCast {
    u32x4: [u32; 4],
    f32x4: [f32; 4],
    m128: __m128,
}

pub const fn m128_from_f32x4(f32x4: [f32; 4]) -> __m128 {
    unsafe { UnionCast { f32x4 }.m128 }
}

const fn m128_from_u32x4(u32x4: [u32; 4]) -> __m128 {
    unsafe { UnionCast { u32x4 }.m128 }
}

const PS_ABS_MASK: __m128 = m128_from_u32x4([0x7fffffff; 4]);
const PS_INV_SIGN_MASK: __m128 = m128_from_u32x4([!0x8000_0000; 4]);
const PS_SIGN_MASK: __m128 = m128_from_u32x4([0x8000_0000; 4]);
const PS_NO_FRACTION: __m128 = m128_from_f32x4([8388608.0; 4]);
const PS_NEGATIVE_ZERO: __m128 = m128_from_u32x4([0x8000_0000; 4]);
const PS_PI: __m128 = m128_from_f32x4([core::f32::consts::PI; 4]);
const PS_HALF_PI: __m128 = m128_from_f32x4([core::f32::consts::FRAC_PI_2; 4]);
const PS_SIN_COEFFICIENTS0: __m128 =
    m128_from_f32x4([-0.16666667, 0.008_333_331, -0.00019840874, 2.752_556_2e-6]);
const PS_SIN_COEFFICIENTS1: __m128 = m128_from_f32x4([
    -2.388_985_9e-8,
    -0.16665852,      /*Est1*/
    0.008_313_95,     /*Est2*/
    -0.000_185_246_7, /*Est3*/
]);
const PS_ONE: __m128 = m128_from_f32x4([1.0; 4]);
const PS_TWO_PI: __m128 = m128_from_f32x4([core::f32::consts::TAU; 4]);
const PS_RECIPROCAL_TWO_PI: __m128 = m128_from_f32x4([0.159_154_94; 4]);

/// Calculates the vector 3 dot product and returns answer in x lane of __m128.
#[inline(always)]
pub(crate) unsafe fn dot3_in_x(lhs: __m128, rhs: __m128) -> __m128 {
    let x2_y2_z2_w2 = _mm_mul_ps(lhs, rhs);
    let y2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_01);
    let z2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_10);
    let x2y2_0_0_0 = _mm_add_ss(x2_y2_z2_w2, y2_0_0_0);
    _mm_add_ss(x2y2_0_0_0, z2_0_0_0)
}

/// Calculates the vector 4 dot product and returns answer in x lane of __m128.
#[inline(always)]
pub(crate) unsafe fn dot4_in_x(lhs: __m128, rhs: __m128) -> __m128 {
    let x2_y2_z2_w2 = _mm_mul_ps(lhs, rhs);
    let z2_w2_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_11_10);
    let x2z2_y2w2_0_0 = _mm_add_ps(x2_y2_z2_w2, z2_w2_0_0);
    let y2w2_0_0_0 = _mm_shuffle_ps(x2z2_y2w2_0_0, x2z2_y2w2_0_0, 0b00_00_00_01);
    _mm_add_ps(x2z2_y2w2_0_0, y2w2_0_0_0)
}

#[inline]
pub(crate) unsafe fn dot3(lhs: __m128, rhs: __m128) -> f32 {
    _mm_cvtss_f32(dot3_in_x(lhs, rhs))
}

#[inline]
pub(crate) unsafe fn dot3_into_m128(lhs: __m128, rhs: __m128) -> __m128 {
    let dot_in_x = dot3_in_x(lhs, rhs);
    _mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00)
}

#[inline]
pub(crate) unsafe fn dot4(lhs: __m128, rhs: __m128) -> f32 {
    _mm_cvtss_f32(dot4_in_x(lhs, rhs))
}

#[inline]
pub(crate) unsafe fn dot4_into_m128(lhs: __m128, rhs: __m128) -> __m128 {
    let dot_in_x = dot4_in_x(lhs, rhs);
    _mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00)
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

#[inline]
pub(crate) unsafe fn m128_abs(v: __m128) -> __m128 {
    _mm_and_ps(v, _mm_castsi128_ps(_mm_set1_epi32(0x7f_ff_ff_ff)))
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
pub(crate) unsafe fn m128_trunc(v: __m128) -> __m128 {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorTruncate`
    // To handle NAN, INF and numbers greater than 8388608, use masking
    // Get the abs value
    let mut vtest = _mm_and_si128(_mm_castps_si128(v), _mm_castps_si128(PS_ABS_MASK));
    // Test for greater than 8388608 (All floats with NO fractionals, NAN and INF
    vtest = _mm_cmplt_epi32(vtest, _mm_castps_si128(PS_NO_FRACTION));
    // Convert to int and back to float for rounding with truncation
    let vint = _mm_cvttps_epi32(v);
    // Convert back to floats
    let mut vresult = _mm_cvtepi32_ps(vint);
    // All numbers less than 8388608 will use the round to int
    vresult = _mm_and_ps(vresult, _mm_castsi128_ps(vtest));
    // All others, use the ORIGINAL value
    vtest = _mm_andnot_si128(vtest, _mm_castps_si128(v));
    _mm_or_ps(vresult, _mm_castsi128_ps(vtest))
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

#[test]
fn test_sse2_m128_sin() {
    use crate::Vec4;
    use core::f32::consts::PI;

    fn test_sse2_m128_sin_angle(a: f32) {
        let v = unsafe { m128_sin(_mm_set_ps1(a)) };
        let v = Vec4(v);
        let a_sin = a.sin();
        // dbg!((a, a_sin, v));
        assert!(v.abs_diff_eq(Vec4::splat(a_sin), 1e-6));
    }

    let mut a = -PI;
    let end = PI;
    let step = PI / 8192.0;

    while a <= end {
        test_sse2_m128_sin_angle(a);
        a += step;
    }
}
