use core::arch::wasm32::*;

macro_rules! const_u32x4 {
    ($ux4:expr) => {
        unsafe { $crate::cast::UVec4Cast { ux4: $ux4 }.v128 }
    };
}

const PS_NEGATIVE_ZERO: v128 = const_u32x4!([0x8000_0000; 4]);
const PS_PI: v128 = const_f32x4!([core::f32::consts::PI; 4]);
const PS_HALF_PI: v128 = const_f32x4!([core::f32::consts::FRAC_PI_2; 4]);
const PS_SIN_COEFFICIENTS0: v128 =
    const_f32x4!([-0.16666667, 0.008_333_331, -0.00019840874, 2.752_556_2e-6]);
const PS_SIN_COEFFICIENTS1: v128 = const_f32x4!([
    -2.388_985_9e-8,
    -0.16665852,      /*Est1*/
    0.008_313_95,     /*Est2*/
    -0.000_185_246_7  /*Est3*/
]);
const PS_ONE: v128 = const_f32x4!([1.0; 4]);
const PS_TWO_PI: v128 = const_f32x4!([core::f32::consts::TAU; 4]);
const PS_RECIPROCAL_TWO_PI: v128 = const_f32x4!([0.159_154_94; 4]);

#[inline(always)]
pub(crate) fn v128_mul_add(a: v128, b: v128, c: v128) -> v128 {
    f32x4_add(f32x4_mul(a, b), c)
}

#[inline(always)]
pub(crate) fn v128_neg_mul_sub(a: v128, b: v128, c: v128) -> v128 {
    f32x4_sub(c, f32x4_mul(a, b))
}

/// Returns a vector whose components are the corresponding components of Angles modulo 2PI.
#[inline]
pub(crate) fn v128_mod_angles(angles: v128) -> v128 {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorModAngles`
    let v = f32x4_mul(angles, PS_RECIPROCAL_TWO_PI);
    let v = f32x4_nearest(v);
    v128_neg_mul_sub(PS_TWO_PI, v, angles)
}

/// Computes the sine of the angle in each lane of `v`. Values outside
/// the bounds of PI may produce an increasing error as the input angle
/// drifts from `[-PI, PI]`.
#[inline]
pub(crate) fn v128_sin(v: v128) -> v128 {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorSin`

    // 11-degree minimax approximation

    // Force the value within the bounds of pi
    let mut x = v128_mod_angles(v);

    // Map in [-pi/2,pi/2] with sin(y) = sin(x).
    let sign = v128_and(x, PS_NEGATIVE_ZERO);
    // pi when x >= 0, -pi when x < 0
    let c = v128_or(PS_PI, sign);
    // |x|
    let absx = v128_andnot(sign, x);
    let rflx = f32x4_sub(c, x);
    let comp = f32x4_le(absx, PS_HALF_PI);
    let select0 = v128_and(comp, x);
    let select1 = v128_andnot(comp, rflx);
    x = v128_or(select0, select1);

    let x2 = f32x4_mul(x, x);

    // Compute polynomial approximation
    const SC1: v128 = PS_SIN_COEFFICIENTS1;
    let v_constants_b = i32x4_shuffle::<0, 0, 4, 4>(SC1, SC1);

    const SC0: v128 = PS_SIN_COEFFICIENTS0;
    let mut v_constants = i32x4_shuffle::<3, 3, 7, 7>(SC0, SC0);
    let mut result = v128_mul_add(v_constants_b, x2, v_constants);

    v_constants = i32x4_shuffle::<2, 2, 6, 6>(SC0, SC0);
    result = v128_mul_add(result, x2, v_constants);

    v_constants = i32x4_shuffle::<1, 1, 5, 5>(SC0, SC0);
    result = v128_mul_add(result, x2, v_constants);

    v_constants = i32x4_shuffle::<0, 0, 4, 4>(SC0, SC0);
    result = v128_mul_add(result, x2, v_constants);

    result = v128_mul_add(result, x2, PS_ONE);
    result = f32x4_mul(result, x);

    result
}

#[test]
fn test_wasm32_v128_sin() {
    use crate::core::traits::vector::*;
    use core::f32::consts::PI;

    fn test_wasm32_v128_sin_angle(a: f32) {
        let v = v128_sin(f32x4_splat(a));
        let v = v.as_ref_xyzw();
        let a_sin = a.sin();
        // dbg!((a, a_sin, v));
        assert!(v.abs_diff_eq(Vector::splat(a_sin), 1e-6));
    }

    let mut a = -PI;
    let end = PI;
    let step = PI / 8192.0;

    while a <= end {
        test_wasm32_v128_sin_angle(a);
        a += step;
    }
}
