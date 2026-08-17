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
use crate::common::{is_integer, is_odd_integer};
use crate::gamma::lgamma::lgamma_core;

/// Computes log(gamma(x))
///
/// Returns gamma value + sign.
pub fn f_lgamma_r(x: f64) -> (f64, i32) {
    // filter out exceptional cases
    let xb = x.to_bits().wrapping_shl(1);
    if xb >= 0x7ffu64 << 53 || xb == 0 {
        if x.is_nan() {
            return (f64::NAN, 1);
        }
        if xb == 0 {
            return (f64::INFINITY, 1 - 2 * ((x.to_bits() >> 63) as i32));
        }
        if xb.wrapping_shl(11) == 0 {
            return (f64::INFINITY, 1);
        }
    }

    if is_integer(x) {
        if x == 2. || x == 1. {
            return (0., 1);
        }
        if x.is_sign_negative() {
            let is_odd = (!is_odd_integer(x)) as i32;
            return (f64::INFINITY, 1 - (is_odd << 1));
        }
    }

    let (r, sign_gam) = lgamma_core(x);
    (r.to_f64(), sign_gam)
}

#[cfg(test)]
mod tests {
    use crate::f_lgamma_r;

    #[test]
    fn test_lgamma_rf() {
        assert_eq!(f_lgamma_r(-0.), (f64::INFINITY, -1));
        assert_eq!(f_lgamma_r(f64::NEG_INFINITY), (f64::INFINITY, 1));
        assert_eq!(f_lgamma_r(f64::INFINITY), (f64::INFINITY, 1));
        assert_eq!(f_lgamma_r(0.), (f64::INFINITY, 1));
        assert_eq!(f_lgamma_r(5.), (3.1780538303479458, 1));
        assert_eq!(f_lgamma_r(-4.5), (-2.813084081769316, -1));
        assert_eq!(f_lgamma_r(-2.0015738), (5.759666328573806, -1));
    }
}
