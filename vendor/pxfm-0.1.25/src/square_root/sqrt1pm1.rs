/*
 * // Copyright (c) Radzivon Bartoshyk 9/2025. All rights reserved.
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
use crate::common::f_fmla;
use crate::double_double::DoubleDouble;
use crate::polyeval::f_polyeval6;

#[cold]
fn sqrt1pm1_near_zero_hard(x: f64) -> f64 {
    const C: [(u64, u64); 10] = [
        (0x3af2bd7b2a000000, 0x3fe0000000000000),
        (0xbb1e743d7fe00000, 0xbfc0000000000000),
        (0xbc18d267496ae7f8, 0x3fb0000000000000),
        (0x3c2d7a2a887e1af8, 0xbfa4000000000000),
        (0xbc3a4965117e40c8, 0x3f9c00000000150b),
        (0x3c3dc057cb5bf82c, 0xbf9500000000209e),
        (0x3c02d0756979aa48, 0x3f907ffff9c1db3d),
        (0xbbe30b3d9b8a1020, 0xbf8acffff10fbdbc),
        (0xbc16d26d5f2efc04, 0x3f8659830d1bf014),
        (0x3c212aabd12c483e, 0xbf82ff830f9799c4),
    ];
    let mut p = DoubleDouble::quick_mul_f64_add(
        DoubleDouble::from_bit_pair(C[9]),
        x,
        DoubleDouble::from_bit_pair(C[8]),
    );
    p = DoubleDouble::quick_mul_f64_add(p, x, DoubleDouble::from_bit_pair(C[7]));
    p = DoubleDouble::quick_mul_f64_add(p, x, DoubleDouble::from_bit_pair(C[6]));
    p = DoubleDouble::quick_mul_f64_add(p, x, DoubleDouble::from_bit_pair(C[5]));
    p = DoubleDouble::quick_mul_f64_add(p, x, DoubleDouble::from_bit_pair(C[4]));
    p = DoubleDouble::quick_mul_f64_add(p, x, DoubleDouble::from_bit_pair(C[3]));
    p = DoubleDouble::quick_mul_f64_add(p, x, DoubleDouble::from_bit_pair(C[2]));
    p = DoubleDouble::quick_mul_f64_add(p, x, DoubleDouble::from_bit_pair(C[1]));
    p = DoubleDouble::quick_mul_f64_add(p, x, DoubleDouble::from_bit_pair(C[0]));
    p = DoubleDouble::quick_mult_f64(p, x);
    p.to_f64()
}

/// Computes sqrt(1+x) - 1
pub fn f_sqrt1pm1(x: f64) -> f64 {
    let ix = x.to_bits();
    let ux = ix.wrapping_shl(1);

    if ux >= 0x7ffu64 << 53 || ux <= 0x7960000000000000u64 {
        // |x| == NaN, x == inf, |x| == 0, |x| <= f64::EPSILON
        if ux == 0 {
            // |x| == 0
            return 0.;
        }
        if ux.wrapping_shl(11) == 0 {
            // |x| == Inf
            return if x.is_sign_negative() {
                f64::NAN
            } else {
                f64::INFINITY
            };
        }

        if ux <= 0x7960000000000000u64 {
            // |x| <= f64::EPSILON
            #[cfg(any(
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "fma"
                ),
                target_arch = "aarch64"
            ))]
            {
                return f_fmla(-0.25 * x, 0.25 * x, x * 0.5);
            }
            #[cfg(not(any(
                all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "fma"
                ),
                target_arch = "aarch64"
            )))]
            {
                use crate::common::dyad_fmla;
                return if x < 1e-150 {
                    dyad_fmla(-0.25 * x, 0.25 * x, x * 0.5)
                } else {
                    f_fmla(-0.25 * x, 0.25 * x, x * 0.5)
                };
            }
        }

        return f64::NAN; // x == NaN
    }

    if (ix >> 63) != 0 && ux >= 0x7fe0000000000000u64 {
        // x < 0 and x >= 1
        if ux == 0x7fe0000000000000u64 {
            // x == -1
            return -1.;
        }
        return f64::NAN;
    }

    if ux <= 0x7f1126e978d4fdf4u64 {
        // |x| <= 0.012

        // Polynomial generated by Sollya:
        // d = [-0.012, 0.012];
        // sqrt1pm1 = sqrt(x + 1) - 1;
        // Q = fpminimax(sqrt1pm1, [|1,2,3,4,5,6,7|], [|1, D...|], d);
        const C: [u64; 7] = [
            0x3fe0000000000000,
            0xbfc000000000009a,
            0x3fb0000000000202,
            0xbfa3fffffdf5a853,
            0x3f9bfffffb3c75fe,
            0xbf9500dd6aee8501,
            0x3f9080d2ece21348,
        ];
        let p = f_polyeval6(
            x,
            f64::from_bits(C[1]),
            f64::from_bits(C[2]),
            f64::from_bits(C[3]),
            f64::from_bits(C[4]),
            f64::from_bits(C[5]),
            f64::from_bits(C[6]),
        ) * x;
        let r = DoubleDouble::from_exact_add(f64::from_bits(C[0]), p);
        let q = DoubleDouble::quick_mult_f64(r, x);
        let err = f_fmla(
            q.hi,
            f64::from_bits(0x3c50000000000000), // 2^-58
            f64::from_bits(0x3bf0000000000000), // 2^-64
        );
        let ub = q.hi + (q.lo + err);
        let lb = q.hi + (q.lo - err);
        // Ziv's accuracy test
        if ub == lb {
            return ub;
        }
        return sqrt1pm1_near_zero_hard(x);
    }
    // |x| > 0.012

    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    ))]
    {
        let r = DoubleDouble::from_full_exact_add(x, 1.0);
        let v_sqrt = r.fast_sqrt();
        let q = DoubleDouble::full_add_f64(v_sqrt, -1.0);
        q.to_f64()
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    )))]
    {
        use crate::double_double::two_product_compatible;
        if !two_product_compatible(x) {
            // x is very big, thus adding + 1 is negligible in ulp terms
            let r = x + 1.;
            let v_sqrt = r.sqrt();
            DoubleDouble::from_full_exact_sub(v_sqrt, 1.0).to_f64()
        } else {
            let r = DoubleDouble::from_full_exact_add(x, 1.0);
            let v_sqrt = r.fast_sqrt();
            let q = DoubleDouble::full_add_f64(v_sqrt, -1.0);
            q.to_f64()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt1pm1() {
        assert_eq!(f_sqrt1pm1(15.), 3.0);
        assert_eq!(f_sqrt1pm1(24.), 4.0);
        assert_eq!(f_sqrt1pm1(8.), 2.0);
        assert_eq!(f_sqrt1pm1(-0.75), -0.5);
        assert_eq!(f_sqrt1pm1(0.5), 0.22474487139158905);
        assert_eq!(f_sqrt1pm1(0.0005233212), 0.00026162637581973774);
        assert_eq!(f_sqrt1pm1(-0.0005233212), -0.0002616948420951896);
        assert_eq!(f_sqrt1pm1(-0.00000000000000000000005233212), -2.616606e-23);
        assert_eq!(f_sqrt1pm1(0.00000000000000000000005233212), 2.616606e-23);
        assert_eq!(f_sqrt1pm1(0.), 0.);
        assert_eq!(f_sqrt1pm1(-0.), 0.);
        assert_eq!(f_sqrt1pm1(f64::INFINITY), f64::INFINITY);
        assert!(f_sqrt1pm1(f64::NEG_INFINITY).is_nan());
        assert!(f_sqrt1pm1(f64::NAN).is_nan());
        assert!(f_sqrt1pm1(-1.0001).is_nan());
    }
}
