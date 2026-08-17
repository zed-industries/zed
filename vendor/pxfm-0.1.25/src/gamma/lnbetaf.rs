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

use crate::gamma::lgamma_rf::lgamma_coref;

/// Computes log(beta(x)) function
pub fn f_lnbetaf(a: f32, b: f32) -> f32 {
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
            return f32::NEG_INFINITY;
        }
        return a + f32::NAN; // nan
    }

    let (mut y, _) = lgamma_coref(a + b);
    let (y1, _) = lgamma_coref(b);
    y = y1 - y;
    let (y1, _) = lgamma_coref(a);
    y += y1;
    y as f32
}

pub(crate) fn lnbetaf_core(a: f32, b: f32) -> f64 {
    let (mut y, _) = lgamma_coref(a + b);
    let (y1, _) = lgamma_coref(b);
    y = y1 - y;
    let (y1, _) = lgamma_coref(a);
    y += y1;
    y
}

#[cfg(test)]
mod tests {
    use crate::f_lnbetaf;

    #[test]
    fn test_betaf() {
        assert_eq!(f_lnbetaf(1., f32::INFINITY), f32::NEG_INFINITY);
        assert_eq!(f_lnbetaf(f32::INFINITY, 0.), f32::INFINITY);
        assert!(f_lnbetaf(0., 0.).is_nan());
        assert_eq!(f_lnbetaf(0., f32::INFINITY), f32::INFINITY);
        assert!(f_lnbetaf(-5., 15.).is_nan());
        assert!(f_lnbetaf(5., -15.).is_nan());
        assert!(f_lnbetaf(f32::NAN, 15.).is_nan());
        assert!(f_lnbetaf(15., f32::NAN).is_nan());
        assert_eq!(f_lnbetaf(f32::INFINITY, 1.), f32::NEG_INFINITY);
        assert_eq!(f_lnbetaf(5., 3.), -4.65396);
        assert_eq!(f_lnbetaf(3., 5.), -4.65396);
        assert_eq!(f_lnbetaf(12., 23.), -22.607338);
    }
}
