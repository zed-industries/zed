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
use crate::double_double::DoubleDouble;
use crate::gamma::lgamma::lgamma_core;

/// Computes log(beta(x)) function
pub fn f_lnbeta(a: f64, b: f64) -> f64 {
    let ax = a.to_bits();
    let bx = b.to_bits();

    if ax >= 0x7ffu64 << 52 || ax == 0 || bx >= 0x7ffu64 << 52 || bx == 0 {
        if (ax >> 63) != 0 || (bx >> 63) != 0 {
            // |a| < 0 or |b| < 0
            return f64::NAN;
        }
        if ax.wrapping_shl(1) == 0 || bx.wrapping_shl(1) == 0 {
            // |a| == 0 || |b| == 0
            if ax == 0 || bx == 0 {
                // |a| == 0 || |b| == 0
                if ax.wrapping_shl(1) != 0 || bx.wrapping_shl(1) != 0 {
                    return f64::INFINITY;
                }
                return f64::NAN;
            }
            return f64::NAN;
        }
        if a.is_infinite() || b.is_infinite() {
            // |a| == inf or |b| == inf
            return f64::NEG_INFINITY;
        }
        return a + f64::NAN; // nan
    }

    let (mut y, _) = lgamma_core(DoubleDouble::from_full_exact_add(a, b).to_f64());
    let (y1, _) = lgamma_core(b);
    y = DoubleDouble::quick_dd_sub(y1, y);
    let (y1, _) = lgamma_core(a);
    y = DoubleDouble::quick_dd_add(y1, y);
    y.to_f64()
}

pub(crate) fn lnbeta_core(a: f64, b: f64) -> DoubleDouble {
    let (mut y, _) = lgamma_core(DoubleDouble::from_full_exact_add(a, b).to_f64());
    let (y1, _) = lgamma_core(b);
    y = DoubleDouble::quick_dd_sub(y1, y);
    let (y1, _) = lgamma_core(a);
    DoubleDouble::quick_dd_add(y1, y)
}

#[cfg(test)]
mod tests {
    use crate::f_lnbeta;

    #[test]
    fn test_beta() {
        assert_eq!(f_lnbeta(f64::INFINITY, 0.), f64::INFINITY);
        assert!(f_lnbeta(0., 0.).is_nan());
        assert_eq!(f_lnbeta(0., f64::INFINITY), f64::INFINITY);
        assert!(f_lnbeta(-5., 15.).is_nan());
        assert!(f_lnbeta(5., -15.).is_nan());
        assert!(f_lnbeta(f64::NAN, 15.).is_nan());
        assert!(f_lnbeta(15., f64::NAN).is_nan());
        assert_eq!(f_lnbeta(f64::INFINITY, 1.), f64::NEG_INFINITY);
        assert_eq!(f_lnbeta(1., f64::INFINITY), f64::NEG_INFINITY);
        assert_eq!(f_lnbeta(5., 3.), -4.653960350157523);
        assert_eq!(f_lnbeta(3., 5.), -4.653960350157523);
        assert_eq!(f_lnbeta(12., 23.), -22.607338344488568);
    }
}
