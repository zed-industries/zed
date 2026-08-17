/*
 * // Copyright (c) Radzivon Bartoshyk 6/2025. All rights reserved.
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
use crate::common::{dyad_fmla, f_fmla, min_normal_f64};
use crate::double_double::DoubleDouble;
use crate::dyadic_float::{DyadicFloat128, DyadicSign};
use crate::rounding::CpuRound;
use crate::sin_helper::sincos_eval_dd;
use crate::sin_table::SIN_K_PI_OVER_128;
use crate::sincos_dyadic::SIN_K_PI_OVER_128_F128;
use crate::sincos_reduce::LargeArgumentReduction;

// For 2^-7 < |x| < 2^16, return k and u such that:
//   k = round(x * 128/pi)
//   x mod pi/128 = x - k * pi/128 ~ u.hi + u.lo
// Error bound:
//   |(x - k * pi/128) - (u_hi + u_lo)| <= max(ulp(ulp(u_hi)), 2^-119)
//                                      <= 2^-111.
#[inline]
pub(crate) fn range_reduction_small(x: f64) -> (DoubleDouble, u64) {
    const MPI_OVER_128: [u64; 3] = [0xbf9921fb54400000, 0xbd70b4611a600000, 0xbb43198a2e037073];
    const ONE_TWENTY_EIGHT_OVER_PI_D: f64 = f64::from_bits(0x40445f306dc9c883);
    let prod_hi = x * ONE_TWENTY_EIGHT_OVER_PI_D;
    let kd = prod_hi.cpu_round();

    // Let y = x - k * (pi/128)
    // Then |y| < pi / 256
    // With extra rounding errors, we can bound |y| < 1.6 * 2^-7.
    let y_hi = f_fmla(kd, f64::from_bits(MPI_OVER_128[0]), x); // Exact
    // |u.hi| < 1.6*2^-7
    let u_hi = f_fmla(kd, f64::from_bits(MPI_OVER_128[1]), y_hi);

    let u0 = y_hi - u_hi; // Exact
    // |u.lo| <= max(ulp(u.hi), |kd * MPI_OVER_128[2]|)
    let u1 = f_fmla(kd, f64::from_bits(MPI_OVER_128[1]), u0); // Exact
    let u_lo = f_fmla(kd, f64::from_bits(MPI_OVER_128[2]), u1);
    // Error bound:
    // |x - k * pi/128| - (u.hi + u.lo) <= ulp(u.lo)
    //                                  <= ulp(max(ulp(u.hi), kd*MPI_OVER_128[2]))
    //                                  <= 2^(-7 - 104) = 2^-111.
    (DoubleDouble::new(u_lo, u_hi), unsafe {
        kd.to_int_unchecked::<i64>() as u64 // indeterminate values is always filtered out before this call, as well only lowest bits are used
    })
}

#[inline]
pub(crate) fn get_sin_k_rational(kk: u64) -> DyadicFloat128 {
    let idx = if (kk & 64) != 0 {
        64 - (kk & 63)
    } else {
        kk & 63
    };
    let mut ans = SIN_K_PI_OVER_128_F128[idx as usize];
    if (kk & 128) != 0 {
        ans.sign = DyadicSign::Neg;
    }
    ans
}

pub(crate) struct SinCos {
    pub(crate) v_sin: DoubleDouble,
    pub(crate) v_cos: DoubleDouble,
    pub(crate) err: f64,
}

#[inline]
pub(crate) fn sincos_eval(u: DoubleDouble) -> SinCos {
    // Evaluate sin(y) = sin(x - k * (pi/128))
    // We use the degree-7 Taylor approximation:
    //   sin(y) ~ y - y^3/3! + y^5/5! - y^7/7!
    // Then the error is bounded by:
    //   |sin(y) - (y - y^3/3! + y^5/5! - y^7/7!)| < |y|^9/9! < 2^-54/9! < 2^-72.
    // For y ~ u_hi + u_lo, fully expanding the polynomial and drop any terms
    // < ulp(u_hi^3) gives us:
    //   y - y^3/3! + y^5/5! - y^7/7! = ...
    // ~ u_hi + u_hi^3 * (-1/6 + u_hi^2 * (1/120 - u_hi^2 * 1/5040)) +
    //        + u_lo (1 + u_hi^2 * (-1/2 + u_hi^2 / 24))
    let u_hi_sq = u.hi * u.hi; // Error < ulp(u_hi^2) < 2^(-6 - 52) = 2^-58.
    // p1 ~ 1/120 + u_hi^2 / 5040.
    let p1 = f_fmla(
        u_hi_sq,
        f64::from_bits(0xbf2a01a01a01a01a),
        f64::from_bits(0x3f81111111111111),
    );
    // q1 ~ -1/2 + u_hi^2 / 24.
    let q1 = f_fmla(
        u_hi_sq,
        f64::from_bits(0x3fa5555555555555),
        f64::from_bits(0xbfe0000000000000),
    );
    let u_hi_3 = u_hi_sq * u.hi;
    // p2 ~ -1/6 + u_hi^2 (1/120 - u_hi^2 * 1/5040)
    let p2 = f_fmla(u_hi_sq, p1, f64::from_bits(0xbfc5555555555555));
    // q2 ~ 1 + u_hi^2 (-1/2 + u_hi^2 / 24)
    let q2 = f_fmla(u_hi_sq, q1, 1.0);
    let sin_lo = f_fmla(u_hi_3, p2, u.lo * q2);
    // Overall, |sin(y) - (u_hi + sin_lo)| < 2*ulp(u_hi^3) < 2^-69.

    // Evaluate cos(y) = cos(x - k * (pi/128))
    // We use the degree-8 Taylor approximation:
    //   cos(y) ~ 1 - y^2/2 + y^4/4! - y^6/6! + y^8/8!
    // Then the error is bounded by:
    //   |cos(y) - (...)| < |y|^10/10! < 2^-81
    // For y ~ u_hi + u_lo, fully expanding the polynomial and drop any terms
    // < ulp(u_hi^3) gives us:
    //   1 - y^2/2 + y^4/4! - y^6/6! + y^8/8! = ...
    // ~ 1 - u_hi^2/2 + u_hi^4(1/24 + u_hi^2 (-1/720 + u_hi^2/40320)) +
    //     + u_hi u_lo (-1 + u_hi^2/6)
    // We compute 1 - u_hi^2 accurately:
    //   v_hi + v_lo ~ 1 - u_hi^2/2
    // with error <= 2^-105.
    let u_hi_neg_half = (-0.5) * u.hi;

    let (mut v_lo, v_hi);

    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    ))]
    {
        v_hi = f_fmla(u.hi, u_hi_neg_half, 1.0);
        v_lo = 1.0 - v_hi; // Exact
        v_lo = f_fmla(u.hi, u_hi_neg_half, v_lo);
    }

    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    )))]
    {
        let u_hi_sq_neg_half = DoubleDouble::from_exact_mult(u.hi, u_hi_neg_half);
        let v = DoubleDouble::from_exact_add(1.0, u_hi_sq_neg_half.hi);
        v_lo = v.lo;
        v_lo += u_hi_sq_neg_half.lo;
        v_hi = v.hi;
    }

    // r1 ~ -1/720 + u_hi^2 / 40320
    let r1 = f_fmla(
        u_hi_sq,
        f64::from_bits(0x3efa01a01a01a01a),
        f64::from_bits(0xbf56c16c16c16c17),
    );
    // s1 ~ -1 + u_hi^2 / 6
    let s1 = f_fmla(u_hi_sq, f64::from_bits(0x3fc5555555555555), -1.0);
    let u_hi_4 = u_hi_sq * u_hi_sq;
    let u_hi_u_lo = u.hi * u.lo;
    // r2 ~ 1/24 + u_hi^2 (-1/720 + u_hi^2 / 40320)
    let r2 = f_fmla(u_hi_sq, r1, f64::from_bits(0x3fa5555555555555));
    // s2 ~ v_lo + u_hi * u_lo * (-1 + u_hi^2 / 6)
    let s2 = f_fmla(u_hi_u_lo, s1, v_lo);
    let cos_lo = f_fmla(u_hi_4, r2, s2);
    // Overall, |cos(y) - (v_hi + cos_lo)| < 2*ulp(u_hi^4) < 2^-75.

    let sin_u = DoubleDouble::from_exact_add(u.hi, sin_lo);
    let cos_u = DoubleDouble::from_exact_add(v_hi, cos_lo);

    let err = f_fmla(
        u_hi_3,
        f64::from_bits(0x3cc0000000000000),
        f64::from_bits(0x3960000000000000),
    );

    SinCos {
        v_sin: sin_u,
        v_cos: cos_u,
        err,
    }
}

#[cold]
#[inline(never)]
fn sin_accurate(y: DoubleDouble, sin_k: DoubleDouble, cos_k: DoubleDouble) -> f64 {
    let r_sincos = sincos_eval_dd(y);

    // k is an integer and -pi / 256 <= y <= pi / 256.
    // Then sin(x) = sin((k * pi/128 + y)
    //             = sin(y) * cos(k*pi/128) + cos(y) * sin(k*pi/128)

    let sin_k_cos_y = DoubleDouble::quick_mult(r_sincos.v_cos, sin_k);
    let cos_k_sin_y = DoubleDouble::quick_mult(r_sincos.v_sin, cos_k);

    let rr = DoubleDouble::full_dd_add(sin_k_cos_y, cos_k_sin_y);
    rr.to_f64()
}

#[cold]
pub(crate) fn sin_accurate_near_zero(x: f64) -> f64 {
    // Generated by Sollya:
    // d = [0, 0.0490874];
    // f_sin = sin(y)/y;
    // Q = fpminimax(f_sin, [|0, 2, 4, 6, 8, 10, 12|], [|107...|], d, relative, floating);
    const C: [(u64, u64); 7] = [
        (0x0000000000000000, 0x3ff0000000000000),
        (0xbc65555555555217, 0xbfc5555555555555),
        (0x3c011110f7d49e8c, 0x3f81111111111111),
        (0xbb65e5b30986fc80, 0xbf2a01a01a01a01a),
        (0xbb689e86245bd566, 0x3ec71de3a556c6e5),
        (0xbaccb3d55ccfca58, 0xbe5ae64567954935),
        (0x3a6333ef7a00ce40, 0x3de6120ca00ce3a2),
    ];
    let x2 = DoubleDouble::from_exact_mult(x, x);
    let mut p = DoubleDouble::quick_mul_add(
        x2,
        DoubleDouble::from_bit_pair(C[6]),
        DoubleDouble::from_bit_pair(C[5]),
    );
    p = DoubleDouble::quick_mul_add(x2, p, DoubleDouble::from_bit_pair(C[4]));
    p = DoubleDouble::quick_mul_add(x2, p, DoubleDouble::from_bit_pair(C[3]));
    p = DoubleDouble::quick_mul_add(x2, p, DoubleDouble::from_bit_pair(C[2]));
    p = DoubleDouble::quick_mul_add(x2, p, DoubleDouble::from_bit_pair(C[1]));
    p = DoubleDouble::quick_mul_add_f64(x2, p, f64::from_bits(0x3ff0000000000000));
    p = DoubleDouble::quick_mult_f64(p, x);
    p.to_f64()
}

/// Sine for double precision
///
/// ULP 0.5
pub fn f_sin(x: f64) -> f64 {
    let x_e = (x.to_bits() >> 52) & 0x7ff;
    const E_BIAS: u64 = (1u64 << (11 - 1u64)) - 1u64;

    let y: DoubleDouble;
    let k;

    let mut argument_reduction = LargeArgumentReduction::default();

    if x_e < E_BIAS + 16 {
        // |x| < 2^32 (with FMA) or |x| < 2^23 (w/o FMA)
        let ax = x.to_bits() & 0x7fff_ffff_ffff_ffff;
        if ax <= 0x3fa921fbd34a9550 {
            // |x| <= 0.0490874
            if x_e < E_BIAS - 26 {
                // |x| < 2^-26
                if x == 0.0 {
                    // Signed zeros.
                    return x;
                }

                // For |x| < 2^-26, |sin(x) - x| < ulp(x)/2.
                return dyad_fmla(x, f64::from_bits(0xbc90000000000000), x);
            }

            // Polynomial for sin(x)/x
            // Generated by Sollya:
            // d = [0, 0.0490874];
            // f_sin = sin(y)/y;
            // Q = fpminimax(f_sin, [|0, 2, 4, 6, 8|], [|1, D...|], d, relative, floating);

            let x2 = x * x;
            let x4 = x2 * x2;
            const C: [u64; 4] = [
                0xbfc5555555555555,
                0x3f8111111110e45a,
                0xbf2a019ffd7fdaaf,
                0x3ec71819b9bf01ef,
            ];
            let p01 = f_fmla(x2, f64::from_bits(C[1]), f64::from_bits(C[0]));
            let p23 = f_fmla(x2, f64::from_bits(C[3]), f64::from_bits(C[2]));
            let w0 = f_fmla(x4, p23, p01);
            let w1 = x2 * w0 * x;
            let r = DoubleDouble::from_exact_add(x, w1);
            let err = f_fmla(
                x2,
                f64::from_bits(0x3cb0000000000000), // 2^-52
                f64::from_bits(0x3be0000000000000), // 2^-65
            );
            let ub = r.hi + (r.lo + err);
            let lb = r.hi + (r.lo - err);
            if ub == lb {
                return ub;
            }
            return sin_accurate_near_zero(x);
        }

        // Small range reduction.
        (y, k) = range_reduction_small(x);
    } else {
        // Inf or NaN
        if x_e > 2 * E_BIAS {
            // sin(+-Inf) = NaN
            return x + f64::NAN;
        }

        // Large range reduction.
        (k, y) = argument_reduction.reduce(x);
    }

    let r_sincos = sincos_eval(y);

    // cos(k * pi/128) = sin(k * pi/128 + pi/2) = sin((k + 64) * pi/128).
    let sk = SIN_K_PI_OVER_128[(k & 255) as usize];
    let ck = SIN_K_PI_OVER_128[((k.wrapping_add(64)) & 255) as usize];

    let sin_k = DoubleDouble::from_bit_pair(sk);
    let cos_k = DoubleDouble::from_bit_pair(ck);

    let sin_k_cos_y = DoubleDouble::quick_mult(r_sincos.v_cos, sin_k);
    let cos_k_sin_y = DoubleDouble::quick_mult(r_sincos.v_sin, cos_k);

    // sin_k_cos_y is always >> cos_k_sin_y
    let mut rr = DoubleDouble::from_exact_add(sin_k_cos_y.hi, cos_k_sin_y.hi);
    rr.lo += sin_k_cos_y.lo + cos_k_sin_y.lo;

    let rlp = rr.lo + r_sincos.err;
    let rlm = rr.lo - r_sincos.err;

    let r_upper = rr.hi + rlp; // (rr.lo + ERR);
    let r_lower = rr.hi + rlm; // (rr.lo - ERR);

    // Ziv's accuracy test
    if r_upper == r_lower {
        return rr.to_f64();
    }

    sin_accurate(y, sin_k, cos_k)
}

#[cold]
#[inline(never)]
fn cos_accurate(y: DoubleDouble, msin_k: DoubleDouble, cos_k: DoubleDouble) -> f64 {
    let r_sincos = sincos_eval_dd(y);

    // After range reduction, k = round(x * 128 / pi) and y = x - k * (pi / 128).
    // So k is an integer and -pi / 256 <= y <= pi / 256.
    // Then sin(x) = sin((k * pi/128 + y)
    //             = sin(y) * cos(k*pi/128) + cos(y) * sin(k*pi/128)

    let sin_k_cos_y = DoubleDouble::quick_mult(r_sincos.v_cos, cos_k);
    let cos_k_sin_y = DoubleDouble::quick_mult(r_sincos.v_sin, msin_k);

    let rr = DoubleDouble::full_dd_add(sin_k_cos_y, cos_k_sin_y);
    rr.to_f64()
}

#[cold]
pub(crate) fn cos_accurate_near_zero(x: f64) -> f64 {
    // Generated by Sollya:
    // d = [0, 0.0490874];
    // f_sin = sin(y)/y;
    // Q = fpminimax(f_sin, [|0, 2, 4, 6, 8, 10|], [|107...|], d, relative, floating);
    const C: [(u64, u64); 6] = [
        (0xba2fa1c000000000, 0x3ff0000000000000),
        (0x3b1cd6ead3800000, 0xbfe0000000000000),
        (0x3c45112931063916, 0x3fa5555555555555),
        (0xbba1c1e3ab3b09e0, 0xbf56c16c16c16b2b),
        (0x3b937bc2f4ea7db6, 0x3efa01a019495fca),
        (0x3b307fd5e1b021b3, 0xbe927e0d57d894f8),
    ];
    let x2 = DoubleDouble::from_exact_mult(x, x);
    let mut p = DoubleDouble::quick_mul_add(
        x2,
        DoubleDouble::from_bit_pair(C[5]),
        DoubleDouble::from_bit_pair(C[4]),
    );
    p = DoubleDouble::quick_mul_add(x2, p, DoubleDouble::from_bit_pair(C[3]));
    p = DoubleDouble::quick_mul_add(x2, p, DoubleDouble::from_bit_pair(C[2]));
    p = DoubleDouble::quick_mul_add(x2, p, DoubleDouble::from_bit_pair(C[1]));
    p = DoubleDouble::quick_mul_add(x2, p, DoubleDouble::from_bit_pair(C[0]));
    p.to_f64()
}

/// Cosine for double precision
///
/// ULP 0.5
pub fn f_cos(x: f64) -> f64 {
    let x_e = (x.to_bits() >> 52) & 0x7ff;
    const E_BIAS: u64 = (1u64 << (11 - 1u64)) - 1u64;

    let y: DoubleDouble;
    let k;

    let mut argument_reduction = LargeArgumentReduction::default();

    if x_e < E_BIAS + 16 {
        // |x| < 2^32 (with FMA) or |x| < 2^23 (w/o FMA)
        let ax = x.to_bits() & 0x7fff_ffff_ffff_ffff;
        if ax <= 0x3fa921fbd34a9550 {
            // |x| <= 0.0490874
            if x_e < E_BIAS - 27 {
                // |x| < 2^-27
                if x == 0.0 {
                    // Signed zeros.
                    return 1.0;
                }
                // For |x| < 2^-26, |sin(x) - x| < ulp(x)/2.
                return 1.0 - min_normal_f64();
            }

            // Polynomial for cos(x)
            // Generated by Sollya:
            // d = [0, 0.0490874];
            // f_cos = cos(y);
            // Q = fpminimax(f_cos, [|0, 2, 4, 6, 8|], [|1, D...|], d, relative, floating);

            let x2 = x * x;
            let x4 = x2 * x2;
            const C: [u64; 4] = [
                0xbfe0000000000000,
                0x3fa55555555554a4,
                0xbf56c16c1619b84a,
                0x3efa013d3d01cf7f,
            ];
            let p01 = f_fmla(x2, f64::from_bits(C[1]), f64::from_bits(C[0]));
            let p23 = f_fmla(x2, f64::from_bits(C[3]), f64::from_bits(C[2]));
            let w0 = f_fmla(x4, p23, p01);
            let w1 = x2 * w0;
            let r = DoubleDouble::from_exact_add(1., w1);
            let err = f_fmla(
                x2,
                f64::from_bits(0x3cb0000000000000), // 2^-52
                f64::from_bits(0x3be0000000000000), // 2^-65
            );

            let ub = r.hi + (r.lo + err);
            let lb = r.hi + (r.lo - err);
            if ub == lb {
                return ub;
            }
            return cos_accurate_near_zero(x);
        } else {
            // // Small range reduction.
            (y, k) = range_reduction_small(x);
        }
    } else {
        // Inf or NaN
        if x_e > 2 * E_BIAS {
            // cos(+-Inf) = NaN
            return x + f64::NAN;
        }

        // Large range reduction.
        (k, y) = argument_reduction.reduce(x);
    }
    let r_sincos = sincos_eval(y);

    // After range reduction, k = round(x * 128 / pi) and y = x - k * (pi / 128).
    // So k is an integer and -pi / 256 <= y <= pi / 256.
    // Then cos(x) = cos((k * pi/128 + y)
    //             = cos(y) * cos(k*pi/128) - sin(y) * sin(k*pi/128)
    let sk = SIN_K_PI_OVER_128[(k.wrapping_add(128) & 255) as usize];
    let ck = SIN_K_PI_OVER_128[((k.wrapping_add(64)) & 255) as usize];
    let msin_k = DoubleDouble::from_bit_pair(sk);
    let cos_k = DoubleDouble::from_bit_pair(ck);

    let cos_k_cos_y = DoubleDouble::quick_mult(r_sincos.v_cos, cos_k);
    let cos_k_msin_y = DoubleDouble::quick_mult(r_sincos.v_sin, msin_k);

    // cos_k_cos_y is always >> cos_k_msin_y
    let mut rr = DoubleDouble::from_exact_add(cos_k_cos_y.hi, cos_k_msin_y.hi);
    rr.lo += cos_k_cos_y.lo + cos_k_msin_y.lo;
    let rlp = rr.lo + r_sincos.err;
    let rlm = rr.lo - r_sincos.err;

    let r_upper = rr.hi + rlp; // (rr.lo + ERR);
    let r_lower = rr.hi + rlm; // (rr.lo - ERR);

    // Ziv's accuracy test
    if r_upper == r_lower {
        return rr.to_f64();
    }
    cos_accurate(y, msin_k, cos_k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cos_test() {
        assert_eq!(f_cos(0.0), 1.0);
        assert_eq!(f_cos(0.0024312312), 0.9999970445588818);
        assert_eq!(f_cos(1.0), 0.5403023058681398);
        assert_eq!(f_cos(-0.5), 0.8775825618903728);
        assert!(f_cos(f64::INFINITY).is_nan());
        assert!(f_cos(f64::NEG_INFINITY).is_nan());
        assert!(f_cos(f64::NAN).is_nan());
    }

    #[test]
    fn sin_test() {
        assert_eq!(f_sin(0.00000002149119332890786), 0.00000002149119332890786);
        assert_eq!(f_sin(2.8477476437362989E-306), 2.8477476437362989E-306);
        assert_eq!(f_sin(0.0024312312), 0.002431228804879309);
        assert_eq!(f_sin(0.0), 0.0);
        assert_eq!(f_sin(1.0), 0.8414709848078965);
        assert_eq!(f_sin(-0.5), -0.479425538604203);
        assert!(f_sin(f64::INFINITY).is_nan());
        assert!(f_sin(f64::NEG_INFINITY).is_nan());
        assert!(f_sin(f64::NAN).is_nan());
    }
}
