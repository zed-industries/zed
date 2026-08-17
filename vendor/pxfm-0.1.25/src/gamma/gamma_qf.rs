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
use crate::exponents::core_expdf;
use crate::gamma::gamma_pf::core_gamma_pf;
use crate::gamma::lgamma_rf::lgamma_coref;
use crate::logs::fast_logf;

/// Regularized upper incomplete gamma
pub fn f_gamma_qf(a: f32, x: f32) -> f32 {
    let aa = a.to_bits();
    let ax = x.to_bits();

    if aa >= 0xffu32 << 23 || aa == 0 || ax >= 0xffu32 << 23 || ax == 0 {
        if (aa >> 31) != 0 || (ax >> 31) != 0 {
            // |a| < 0 or |b| < 0
            return f32::NAN;
        }
        if aa.wrapping_shl(1) == 0 {
            // |a| == 0
            return 1.0;
        }
        if ax.wrapping_shl(1) == 0 {
            // |x| == 0
            return 0.;
        }
        if a.is_infinite() {
            // |a| == infinity
            return f32::INFINITY;
        }
        if x.is_infinite() {
            // |x| == infinity
            return f32::INFINITY;
        }
        return a + f32::NAN;
    }

    const EPS: f64 = 1e-9;

    const BIG: f64 = 4503599627370496.0;
    const BIG_INV: f64 = 2.22044604925031308085e-16;

    if x < 1.0 || x <= a {
        return (1.0 - core_gamma_pf(a, x)) as f32;
    }

    let da = a as f64;
    let dx = x as f64;

    let ax = f_fmla(da, fast_logf(x), -dx - lgamma_coref(a).0);
    if ax <= -104. {
        if a < x {
            return 0.0;
        }
        return 1.0;
    }
    if ax >= 89. {
        return f32::INFINITY;
    }

    let mut y = 1.0 - da;
    let mut z = dx + y + 1.0;
    let mut c = 0.0;
    let mut pkm2 = 1.0;
    let mut qkm2 = dx;
    let mut pkm1 = dx + 1.0;
    let mut qkm1 = z * dx;
    let mut ans = pkm1 / qkm1;
    for _ in 0..200 {
        y += 1.0;
        z += 2.0;
        c += 1.0;
        let yc = y * c;
        let pk = pkm1 * z - pkm2 * yc;
        let qk = qkm1 * z - qkm2 * yc;

        pkm2 = pkm1;
        pkm1 = pk;
        qkm2 = qkm1;
        qkm1 = qk;

        if pk.abs() > BIG {
            pkm2 *= BIG_INV;
            pkm1 *= BIG_INV;
            qkm2 *= BIG_INV;
            qkm1 *= BIG_INV;
        }

        if qk != 0.0 {
            let r = pk / qk;
            let t = ((ans - r) / r).abs();
            ans = r;

            if t <= EPS {
                break;
            }
        }
    }
    (ans * core_expdf(ax)) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_f_beta_qf() {
        assert_eq!(f_gamma_qf(23.421, 41.), 0.001130525);
        assert_eq!(f_gamma_qf(0.764, 0.432123), 0.52247006);
        assert_eq!(f_gamma_qf(0.421, 1.), 0.12721314);
        assert!(f_gamma_qf(-1., 12.).is_nan());
        assert!(f_gamma_qf(1., -12.).is_nan());
        assert!(f_gamma_qf(f32::NAN, 12.).is_nan());
        assert!(f_gamma_qf(1., f32::NAN).is_nan());
        assert_eq!(f_gamma_qf(1., f32::INFINITY), f32::INFINITY);
        assert_eq!(f_gamma_qf(f32::INFINITY, f32::INFINITY), f32::INFINITY);
        assert_eq!(f_gamma_qf(f32::INFINITY, 5.32), f32::INFINITY);
    }
}
