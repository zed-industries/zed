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
use crate::exponents::core_expdf;
use crate::gamma::lgamma_rf::lgamma_coref;

/// Computes beta function
pub fn f_betaf(a: f32, b: f32) -> f32 {
    let ax = a.to_bits();
    let bx = b.to_bits();

    if ax >= 0xffu32 << 23 || ax == 0 || bx >= 0xffu32 << 23 || bx == 0 {
        if (ax >> 31) != 0 || (bx >> 31) != 0 {
            // |a| < 0 or |b| < 0
            return f32::NAN;
        }
        if ax.wrapping_shl(1) == 0 || bx.wrapping_shl(1) == 0 {
            // |a| == 0 || |b| == 0
            if ax.wrapping_shl(1) != 0 || bx.wrapping_shl(1) != 0 {
                return f32::INFINITY;
            }
            return f32::NAN;
        }
        if a.is_infinite() || b.is_infinite() {
            // |a| == inf or |b| == inf
            return 0.;
        }
        return a + f32::NAN; // nan
    }

    let mut sign = 1i32;
    let (mut y, sgngamf) = lgamma_coref(a + b);
    sign *= sgngamf; /* keep track of the sign */
    let (y1, sgngamf) = lgamma_coref(b);
    y = y1 - y;
    sign *= sgngamf;
    let (y1, sgngamf) = lgamma_coref(a);
    y += y1;
    sign *= sgngamf;
    if y <= -104. {
        return 0.;
    }
    if y >= 89. {
        // x > 89
        return f32::INFINITY;
    }
    (core_expdf(y) * (sign as f64)) as f32
}

#[cfg(test)]
mod tests {
    use crate::f_betaf;

    #[test]
    fn test_betaf() {
        assert_eq!(f_betaf(f32::INFINITY, 1.), 0.);
        assert_eq!(f_betaf(f32::INFINITY, 0.), f32::INFINITY);
        assert!(f_betaf(0., 0.).is_nan());
        assert_eq!(f_betaf(0., f32::INFINITY), f32::INFINITY);
        assert!(f_betaf(-5., 15.).is_nan());
        assert!(f_betaf(5., -15.).is_nan());
        assert!(f_betaf(f32::NAN, 15.).is_nan());
        assert!(f_betaf(15., f32::NAN).is_nan());
        assert_eq!(f_betaf(f32::INFINITY, 1.), 0.);
        assert_eq!(f_betaf(1., f32::INFINITY), 0.);
        assert_eq!(f_betaf(5., 3.), 0.00952381);
        assert_eq!(f_betaf(3., 5.), 0.00952381);
        assert_eq!(f_betaf(12., 23.), 1.5196995e-10);
    }
}
