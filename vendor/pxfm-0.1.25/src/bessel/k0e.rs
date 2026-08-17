/*
 * // Copyright (c) Radzivon Bartoshyk 8/2025. All rights reserved.
 * //
 * // Redistribution and use in source and binary forms, with or without modification,
 * // are permitted provided that the following conditions are met:
 * //
 * // 1.  Redistributions of source code must retain the above copyright notice, this
 * // list of conditions and the following disclaimer.
 * //
 * // 2.  Redistributions in binary form must reproduce the above copyright notice,
 * // this list of conditions and the following disclaimer in the documentation
 * // and/or other materials provided with the distribution.
 * //
 * // 3.  Neither the name of the copyright holder nor the names of its
 * // contributors may be used to endorse or promote products derived from
 * // this software without specific prior written permission.
 * //
 * // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * // AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * // IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * // DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * // FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * // DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * // SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * // CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * // OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * // OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use crate::bessel::i0::bessel_rsqrt_hard;
use crate::bessel::i0_exp;
use crate::bessel::k0::k0_small_dd;
use crate::double_double::DoubleDouble;
use crate::dyadic_float::{DyadicFloat128, DyadicSign};

/// Modified exponentially scaled Bessel of the first kind of order 0
///
/// Computes K0(x)exp(x)
pub fn f_k0e(x: f64) -> f64 {
    let ix = x.to_bits();

    if ix >= 0x7ffu64 << 52 || ix == 0 {
        // |x| == NaN, x == inf, |x| == 0, x < 0
        if ix.wrapping_shl(1) == 0 {
            // |x| == 0
            return f64::INFINITY;
        }
        if x.is_infinite() {
            return if x.is_sign_positive() { 0. } else { f64::NAN };
        }
        return x + f64::NAN; // x == NaN
    }

    let xb = x.to_bits();

    if xb <= 0x3ff0000000000000 {
        // x <= 1
        let v_k0 = k0_small_dd(x);
        let v_exp = i0_exp(x);
        return DoubleDouble::quick_mult(v_exp, v_k0).to_f64();
    }

    k0e_asympt(x)
}

/**
Generated in Wolfram

Computes sqrt(x)*exp(x)*K0(x)=Pn(1/x)/Qm(1/x)
hence
K0(x)exp(x) = Pn(1/x)/Qm(1/x) / sqrt(x)

```text
<<FunctionApproximations`
ClearAll["Global`*"]
f[x_]:=Sqrt[x] Exp[x] BesselK[0,x]
g[z_]:=f[1/z]
{err, approx}=MiniMaxApproximation[g[z],{z,{0.0000000000001,1},11,11},WorkingPrecision->60]
poly=Numerator[approx][[1]];
coeffs=CoefficientList[poly,z];
TableForm[Table[Row[{"'",NumberForm[coeffs[[i+1]],{50,50},ExponentFunction->(Null&)],"',"}],{i,0,Length[coeffs]-1}]]
poly=Denominator[approx][[1]];
coeffs=CoefficientList[poly,z];
TableForm[Table[Row[{"'",NumberForm[coeffs[[i+1]],{50,50},ExponentFunction->(Null&)],"',"}],{i,0,Length[coeffs]-1}]]
```
**/
#[inline]
fn k0e_asympt(x: f64) -> f64 {
    let recip = DoubleDouble::from_quick_recip(x);
    let r_sqrt = DoubleDouble::from_sqrt(x);

    const P: [(u64, u64); 12] = [
        (0xbc9a6a11d237114e, 0x3ff40d931ff62706),
        (0x3cdd614ddf4929e5, 0x4040645168c3e483),
        (0xbd1ecf9ea0af6ab2, 0x40757419a703a2ab),
        (0xbd3da3551fb27770, 0x409d4e65365522a2),
        (0xbd564d58855d1a46, 0x40b6dd32f5a199d9),
        (0xbd6cf055ca963a8e, 0x40c4fd2368f19618),
        (0x3d4b6cdfbdc058df, 0x40c68faa11ebcd59),
        (0x3d5b4ce4665bfa46, 0x40bb6fbe08e0a8ea),
        (0xbd4316909063be15, 0x40a1953103a5be31),
        (0x3d12f3f8edf41af0, 0x4074d2cb001e175c),
        (0xbcd7bba36540264f, 0x40316cffcad5f8f9),
        (0xbc6bf28dfdd5d37d, 0x3fc2f487fe78b8d7),
    ];

    let x2 = DoubleDouble::quick_mult(recip, recip);
    let x4 = DoubleDouble::quick_mult(x2, x2);
    let x8 = DoubleDouble::quick_mult(x4, x4);

    let e0 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(P[1]),
        DoubleDouble::from_bit_pair(P[0]),
    );
    let e1 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(P[3]),
        DoubleDouble::from_bit_pair(P[2]),
    );
    let e2 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(P[5]),
        DoubleDouble::from_bit_pair(P[4]),
    );
    let e3 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(P[7]),
        DoubleDouble::from_bit_pair(P[6]),
    );
    let e4 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(P[9]),
        DoubleDouble::from_bit_pair(P[8]),
    );
    let e5 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(P[11]),
        DoubleDouble::from_bit_pair(P[10]),
    );

    let f0 = DoubleDouble::mul_add(x2, e1, e0);
    let f1 = DoubleDouble::mul_add(x2, e3, e2);
    let f2 = DoubleDouble::mul_add(x2, e5, e4);

    let g0 = DoubleDouble::mul_add(x4, f1, f0);

    let p_num = DoubleDouble::mul_add(x8, f2, g0);

    const Q: [(u64, u64); 12] = [
        (0x0000000000000000, 0x3ff0000000000000),
        (0xbcb9e8a5b17e696a, 0x403a485acd64d64a),
        (0x3cd2e2e9c87f71f7, 0x4071518092320ecb),
        (0xbd0d05bdb9431a2f, 0x4097e57e4c22c08e),
        (0x3d5207068ab19ba9, 0x40b2ebadb2db62f9),
        (0xbd64e37674083471, 0x40c1c0e4e9d6493d),
        (0x3d3efb7a9a62b020, 0x40c3b94e8d62cdc7),
        (0x3d47d6ce80a2114b, 0x40b93c2fd39e868e),
        (0xbd1dfda61f525861, 0x40a1877a53a7f8d8),
        (0x3d1236ff523dfcfa, 0x4077c3a10c2827de),
        (0xbcc889997c9b0fe7, 0x4039a1d80b11c4a1),
        (0x3c7ded0e8d73dddc, 0x3fdafe4913722123),
    ];

    let e0 = DoubleDouble::mul_add_f64(
        recip,
        DoubleDouble::from_bit_pair(Q[1]),
        f64::from_bits(0x3ff0000000000000),
    );
    let e1 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(Q[3]),
        DoubleDouble::from_bit_pair(Q[2]),
    );
    let e2 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(Q[5]),
        DoubleDouble::from_bit_pair(Q[4]),
    );
    let e3 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(Q[7]),
        DoubleDouble::from_bit_pair(Q[6]),
    );
    let e4 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(Q[9]),
        DoubleDouble::from_bit_pair(Q[8]),
    );
    let e5 = DoubleDouble::mul_add(
        recip,
        DoubleDouble::from_bit_pair(Q[11]),
        DoubleDouble::from_bit_pair(Q[10]),
    );

    let f0 = DoubleDouble::mul_add(x2, e1, e0);
    let f1 = DoubleDouble::mul_add(x2, e3, e2);
    let f2 = DoubleDouble::mul_add(x2, e5, e4);

    let g0 = DoubleDouble::mul_add(x4, f1, f0);

    let p_den = DoubleDouble::mul_add(x8, f2, g0);

    let z = DoubleDouble::div(p_num, p_den);

    let r = DoubleDouble::div(z, r_sqrt);

    let err = r.hi * f64::from_bits(0x3c10000000000000); // 2^-62
    let ub = r.hi + (r.lo + err);
    let lb = r.hi + (r.lo - err);
    if ub != lb {
        return k0e_asympt_hard(x);
    }
    r.to_f64()
}

/**
Generated in Wolfram

Computes sqrt(x)*exp(x)*K0(x)=Pn(1/x)/Qm(1/x)
hence
K0(x)exp(x) = Pn(1/x)/Qm(1/x) / sqrt(x)

```text
<<FunctionApproximations`
ClearAll["Global`*"]
f[x_]:=Sqrt[x] Exp[x] BesselK[0,x]
g[z_]:=f[1/z]
{err, approx}=MiniMaxApproximation[g[z],{z,{0.0000000000001,1},14,14},WorkingPrecision->90]
poly=Numerator[approx][[1]];
coeffs=CoefficientList[poly,z];
TableForm[Table[Row[{"'",NumberForm[coeffs[[i+1]],{50,50},ExponentFunction->(Null&)],"',"}],{i,0,Length[coeffs]-1}]]
poly=Denominator[approx][[1]];
coeffs=CoefficientList[poly,z];
TableForm[Table[Row[{"'",NumberForm[coeffs[[i+1]],{50,50},ExponentFunction->(Null&)],"',"}],{i,0,Length[coeffs]-1}]]
```
**/
#[inline(never)]
#[cold]
fn k0e_asympt_hard(x: f64) -> f64 {
    static P: [DyadicFloat128; 15] = [
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -127,
            mantissa: 0xa06c98ff_b1382cb2_be520f51_a7b8f970_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -122,
            mantissa: 0xc84d8d0c_7faeef84_e56abccc_3d70f8a2_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -118,
            mantissa: 0xd1a71096_3da22280_35768c9e_0d3ddf42_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -115,
            mantissa: 0xf202e474_3698aabb_05688da0_ea1a088d_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -112,
            mantissa: 0xaaa01830_8138af4d_1137b2dd_11a216f5_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -110,
            mantissa: 0x99e0649f_320bca1a_c7adadb3_f5d8498e_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -109,
            mantissa: 0xb4d81657_de1baf00_918cbc76_c6974e96_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -108,
            mantissa: 0x8a9a28c8_a61c2c7a_12416d56_51c0b3d3_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -108,
            mantissa: 0x88a079f1_d9bd4582_6353316c_3aeb9dc9_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -109,
            mantissa: 0xa82e10eb_9dc6225a_ef6223e7_54aa254d_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -111,
            mantissa: 0xf5fc07fe_6b652e8a_0b9e74ba_d0c56118_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -113,
            mantissa: 0xc5288444_c7354b24_4a4e1663_86488928_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -116,
            mantissa: 0x96d3d226_a220ae6e_d6cca1ae_40f01e27_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -121,
            mantissa: 0xa7ab931b_499c4964_499c1091_4ab9673d_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -129,
            mantissa: 0xf8373d1a_9ff3f9c6_e5cfbe0a_85ccc131_u128,
        },
    ];

    static Q: [DyadicFloat128; 15] = [
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -127,
            mantissa: 0x80000000_00000000_00000000_00000000_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -122,
            mantissa: 0xa05190f4_dcf0d35c_277e0f21_0635c538_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -118,
            mantissa: 0xa8837381_94c38992_86c0995d_5e5fa474_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -115,
            mantissa: 0xc3a4f279_9297e905_f59cc065_75959de8_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -112,
            mantissa: 0x8b05ade4_03432e06_881ce37d_a907216d_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -111,
            mantissa: 0xfd77f85e_35626f21_355ae728_01b78cbe_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -109,
            mantissa: 0x972ed117_254970eb_661121dc_a4462d2f_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -109,
            mantissa: 0xec9d204a_9294ab57_2ef500d5_59d701b7_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -109,
            mantissa: 0xf033522d_cae45860_53a01453_c56da895_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -109,
            mantissa: 0x9a33640c_9896ead5_1ce040d7_b36544f3_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -111,
            mantissa: 0xefe714fa_49da0166_fdf8bc68_57b77fa0_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -113,
            mantissa: 0xd323b84c_214196b0_e25b8931_930fea0d_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -116,
            mantissa: 0xbbb5240b_346642d8_010383cb_1e8a607e_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -120,
            mantissa: 0x88dcfa2a_f9f7d2ab_dd017994_8fae7e87_u128,
        },
        DyadicFloat128 {
            sign: DyadicSign::Pos,
            exponent: -127,
            mantissa: 0xc891477c_526e0f5e_74c4ae9f_9d8732b5_u128,
        },
    ];

    let recip = DyadicFloat128::accurate_reciprocal(x);
    let r_sqrt = bessel_rsqrt_hard(x, recip);

    let mut p0 = P[14];
    for i in (0..14).rev() {
        p0 = recip * p0 + P[i];
    }

    let mut q = Q[14];
    for i in (0..14).rev() {
        q = recip * q + Q[i];
    }

    let v = p0 * q.reciprocal();
    let r = v * r_sqrt;
    r.fast_as_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k0() {
        assert_eq!(f_k0e(0.00060324324), 7.533665613459802);
        assert_eq!(f_k0e(0.11), 2.6045757643537244);
        assert_eq!(f_k0e(0.643), 1.3773725807788395);
        assert_eq!(f_k0e(0.964), 1.1625987432322884);
        assert_eq!(f_k0e(2.964), 0.7017119941259377);
        assert_eq!(f_k0e(423.43), 0.06088931243251448);
        assert_eq!(f_k0e(4324235240321.43), 6.027056776336986e-7);
        assert_eq!(k0e_asympt_hard(423.43), 0.06088931243251448);
        assert_eq!(f_k0e(0.), f64::INFINITY);
        assert_eq!(f_k0e(-0.), f64::INFINITY);
        assert!(f_k0e(-0.5).is_nan());
        assert!(f_k0e(f64::NEG_INFINITY).is_nan());
        assert_eq!(f_k0e(f64::INFINITY), 0.);
    }
}
