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
use crate::common::is_integerf;
use crate::gamma::lgamma_rf::lgamma_coref;

/// Computes log(gamma(x))
///
/// ulp 0.5
pub fn f_lgammaf(x: f32) -> f32 {
    let xb = x.to_bits();
    if xb >= 0xffu32 << 23 || xb == 0 {
        if x.is_infinite() {
            return f32::INFINITY;
        }
        if x.is_nan() {
            return f32::NAN;
        }
        if xb.wrapping_shl(1) == 0 {
            return f32::INFINITY;
        }
    }

    if is_integerf(x) {
        if x == 2. || x == 1. {
            return 0.;
        }
        if x.is_sign_negative() {
            return f32::INFINITY;
        }
    }

    lgamma_coref(x).0 as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lgammaf() {
        assert_eq!(f_lgammaf(0.006919047), 4.969523);
        assert_eq!(
            f_lgammaf(0.000000000000000000000000000000000000000015425),
            93.97255
        );
        assert_eq!(f_lgammaf(0.0), f32::INFINITY);
        assert_eq!(f_lgammaf(1.7506484), -0.0842405);
        assert_eq!(f_lgammaf(7.095007e-8), 16.461288);
        assert!(f_lgammaf(-12.).is_infinite());
        assert_eq!(f_lgammaf(2.), 0.);
        assert_eq!(f_lgammaf(1.), 0.);
        assert_eq!(f_lgammaf(0.53), 0.5156078);
        assert_eq!(f_lgammaf(1.53), -0.11927056);
        assert_eq!(f_lgammaf(4.53), 2.4955146);
        assert_eq!(f_lgammaf(11.77), 16.94281);
        assert_eq!(f_lgammaf(22.77), 47.756233);
        assert_eq!(f_lgammaf(-0.53), 1.2684484);
        assert_eq!(f_lgammaf(-1.53), 0.84318066);
        assert_eq!(f_lgammaf(-4.53), -2.8570588);
        assert_eq!(f_lgammaf(-11.77), -17.850103);
        assert_eq!(f_lgammaf(-22.77), -49.323418);
        assert_eq!(f_lgammaf(f32::NEG_INFINITY), f32::INFINITY);
    }
}
