/*
 * // Copyright (c) Radzivon Bartoshyk 7/2025. All rights reserved.
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
use crate::bits::EXP_MASK;
use crate::common::f_fmla;
use crate::double_double::DoubleDouble;
use crate::sin::range_reduction_small;
use crate::sincos_reduce::LargeArgumentReduction;
use crate::tangent::tan::{tan_eval, tan_eval_dd};
use crate::tangent::tanpi_table::TAN_K_PI_OVER_128;

#[cold]
fn cot_accurate(y: DoubleDouble, tan_k: DoubleDouble) -> f64 {
    // Computes tan(x) through identities
    // tan(a+b) = (tan(a) + tan(b)) / (1 - tan(a)tan(b)) = (tan(y) + tan(k*pi/128)) / (1 - tan(y)*tan(k*pi/128))
    let tan_y = tan_eval_dd(y);

    // num = tan(y) + tan(k*pi/64)
    let num_dd = DoubleDouble::full_dd_add(tan_y, tan_k);
    // den = 1 - tan(y)*tan(k*pi/64)
    let den_dd = DoubleDouble::mul_add_f64(tan_y, -tan_k, 1.);

    let cot_x = DoubleDouble::div(den_dd, num_dd);
    cot_x.to_f64()
}

/// Cotangent in double precision
///
/// ULP 0.5
pub fn f_cot(x: f64) -> f64 {
    let x_e = (x.to_bits() >> 52) & 0x7ff;
    const E_BIAS: u64 = (1u64 << (11 - 1u64)) - 1u64;

    let y: DoubleDouble;
    let k;

    let mut argument_reduction = LargeArgumentReduction::default();

    if x_e < E_BIAS + 16 {
        // |x| < 2^16
        if x_e < E_BIAS - 7 {
            // |x| < 2^-7
            if x_e < E_BIAS - 27 {
                // |x| < 2^-27, |cot(x) - x| < ulp(x)/2.
                if x == 0.0 {
                    // Signed zeros.
                    return if x.is_sign_negative() {
                        f64::NEG_INFINITY
                    } else {
                        f64::INFINITY
                    };
                }

                if x_e < E_BIAS - 53 {
                    return 1. / x;
                }

                let dx = DoubleDouble::from_quick_recip(x);
                // taylor order 3
                return DoubleDouble::f64_mul_f64_add(x, f64::from_bits(0xbfd5555555555555), dx)
                    .to_f64();
            }
            // No range reduction needed.
            k = 0;
            y = DoubleDouble::new(0., x);
        } else {
            // Small range reduction.
            (y, k) = range_reduction_small(x);
        }
    } else {
        // Inf or NaN
        if x_e > 2 * E_BIAS {
            if x.is_nan() {
                return f64::NAN;
            }
            // tan(+-Inf) = NaN
            return x + f64::NAN;
        }

        // Large range reduction.
        (k, y) = argument_reduction.reduce(x);
    }

    let (tan_y, err) = tan_eval(y);

    // Computes tan(x) through identities.
    // tan(a+b) = (tan(a) + tan(b)) / (1 - tan(a)tan(b)) = (tan(y) + tan(k*pi/128)) / (1 - tan(y)*tan(k*pi/128))
    let tan_k = DoubleDouble::from_bit_pair(TAN_K_PI_OVER_128[(k & 255) as usize]);

    // num = tan(y) + tan(k*pi/64)
    let num_dd = DoubleDouble::add(tan_y, tan_k);
    // den = 1 - tan(y)*tan(k*pi/64)
    let den_dd = DoubleDouble::mul_add_f64(tan_y, -tan_k, 1.);

    // num and den shifted for cot
    let cot_x = DoubleDouble::div(den_dd, num_dd);

    // Simple error bound: |1 / den_dd| < 2^(1 + floor(-log2(den_dd)))).
    let den_inv = ((E_BIAS + 1) << (52 + 1)) - (den_dd.hi.to_bits() & EXP_MASK);
    // For tan_x = (num_dd + err) / (den_dd + err), the error is bounded by:
    //   | tan_x - num_dd / den_dd |  <= err * ( 1 + | tan_x * den_dd | ).
    let tan_err = err * f_fmla(f64::from_bits(den_inv), cot_x.hi.abs(), 1.0);

    let err_higher = cot_x.lo + tan_err;
    let err_lower = cot_x.lo - tan_err;

    let tan_upper = cot_x.hi + err_higher;
    let tan_lower = cot_x.hi + err_lower;

    // Ziv_s rounding test.
    if tan_upper == tan_lower {
        return tan_upper;
    }

    cot_accurate(y, tan_k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cot_test() {
        assert_eq!(f_cot(2.3006805685393681E-308), 4.346539948546049e307);
        assert_eq!(f_cot(5070552515158872000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.), 25.068466719883585);
        assert_eq!(f_cot(4.9406564584124654E-324), f64::INFINITY);
        assert_eq!(f_cot(0.0), f64::INFINITY);
        assert_eq!(f_cot(1.0), 0.6420926159343308);
        assert_eq!(f_cot(-0.5), -1.830487721712452);
        assert_eq!(f_cot(12.0), -1.5726734063976893);
        assert_eq!(f_cot(-12.0), 1.5726734063976893);
        assert!(f_cot(f64::INFINITY).is_nan());
        assert!(f_cot(f64::NEG_INFINITY).is_nan());
        assert!(f_cot(f64::NAN).is_nan());
    }
}
