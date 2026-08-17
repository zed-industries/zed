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
use crate::bessel::i0_exp;
use crate::double_double::DoubleDouble;

/// Logistic function
///
/// Computes 1/(1+exp(-x))
pub fn f_logistic(x: f64) -> f64 {
    let x_u = x.to_bits();
    let x_abs = x_u & 0x7fff_ffff_ffff_ffffu64;
    if x_abs <= 0x3c90000000000000u64 {
        // |x| <= 5.55112e-17
        return 1. / (2. + -x);
    }
    if x_abs >= 0x4043800000000000u64 {
        // |x| >= 39.0
        if x_abs > 0x7ff0000000000000u64 {
            // x == NaN
            return x + x;
        }
        if x_abs == 0x7ff0000000000000u64 {
            // |x| = inf
            return if (x.to_bits() >> 63) != 0 {
                0.0 // x = -inf
            } else {
                1.0 // x = inf
            };
        }
        if (x.to_bits() >> 63) == 0 {
            // x >= 39.0
            return 1.0;
        }
        if x_abs >= 0x40874910d52d3052u64 {
            // x <= -745.133
            return 1.0;
        }
    }
    // e^x/(1+e^x)
    let num = i0_exp(x);
    let den = DoubleDouble::full_add_f64(num, 1.0);
    let v = DoubleDouble::div(num, den);
    v.to_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logistic() {
        assert_eq!(f_logistic(6.), 0.9975273768433652);
        assert_eq!(f_logistic(39.), 1.0);
        assert_eq!(f_logistic(35.), 0.9999999999999993);
        assert_eq!(f_logistic(30.), 0.9999999999999064);
        assert_eq!(f_logistic(40.), 1.);
        assert_eq!(f_logistic(-89.), 2.2273635617957438e-39);
        assert_eq!(f_logistic(-104.), 6.813556821545298e-46);
        assert_eq!(f_logistic(-103.), 1.8521167695179754e-45);
        assert_eq!(f_logistic(-88.9), 2.4616174324780325e-39);
        assert_eq!(f_logistic(-88.), 6.054601895401186e-39);
        assert_eq!(f_logistic(-80.), 1.8048513878454153e-35);
        assert_eq!(f_logistic(-60.), 8.75651076269652e-27);
        assert_eq!(f_logistic(-40.), 4.248354255291589e-18);
        assert_eq!(f_logistic(-20.), 2.0611536181902037e-9);
        assert_eq!(f_logistic(-1.591388e29), 1.0);
        assert_eq!(f_logistic(-3.), 0.04742587317756678);
        assert_eq!(f_logistic(3.), 0.9525741268224333);
        assert_eq!(f_logistic(20.), 0.9999999979388464);
        assert_eq!(f_logistic(55.), 1.);
        assert_eq!(f_logistic(-104.), 6.813556821545298e-46);
        assert_eq!(f_logistic(-90.), 8.194012623990515e-40);
        assert_eq!(f_logistic(0.00000000000524323), 0.5000000000013108);
        assert_eq!(f_logistic(0.00000000524323), 0.5000000013108075);
        assert_eq!(f_logistic(0.02), 0.5049998333399998);
        assert_eq!(f_logistic(-0.02), 0.4950001666600003);
        assert_eq!(f_logistic(90.), 1.);
        assert_eq!(f_logistic(105.), 1.);
        assert_eq!(f_logistic(f64::INFINITY), 1.0);
        assert_eq!(f_logistic(f64::NEG_INFINITY), 0.);
        assert!(f_logistic(f64::NAN).is_nan());
    }
}
