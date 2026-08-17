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
use crate::gamma::lgamma::lgamma_core;
use crate::logs::fast_log_d_to_dd;

/// Regularized lower incomplete gamma
pub fn f_gamma_p(a: f64, x: f64) -> f64 {
    let aa = a.to_bits();
    let ax = x.to_bits();

    if aa >= 0x7ffu64 << 52 || aa == 0 || ax >= 0x7ffu64 << 52 || ax == 0 {
        if (aa >> 63) != 0 || (ax >> 63) != 0 {
            // |a| < 0 or |b| < 0
            return f64::NAN;
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
            return f64::INFINITY;
        }
        if x.is_infinite() {
            // |x| == infinity
            return f64::INFINITY;
        }
        return a + f64::NAN;
    }

    let big = 4503599627370496.0;
    let big_inv = 2.22044604925031308085e-16;

    const EPS: f64 = f64::EPSILON;

    let da = a;
    let dx = x;

    let r = DoubleDouble::full_add_f64(-lgamma_core(a).0, -dx);
    let ax = DoubleDouble::mul_f64_add(fast_log_d_to_dd(x), da, r).to_f64();

    if ax <= -709.78271289338399 {
        if a < x {
            return 1.0;
        }
        return 0.0;
    }
    if ax >= 709.783 {
        return f64::INFINITY;
    }

    if x <= 1.0 || x <= a {
        let mut r2 = DoubleDouble::new(0., da);
        let mut c2 = DoubleDouble::new(0., 1.0);
        let mut ans2 = DoubleDouble::new(0., 1.0);
        let v_e = i0_exp(ax);
        for _ in 0..200 {
            r2 = DoubleDouble::full_add_f64(r2, 1.0);
            c2 = DoubleDouble::quick_mult(DoubleDouble::from_f64_div_dd(dx, r2), c2);
            c2 = DoubleDouble::from_exact_add(c2.hi, c2.lo);
            ans2 = DoubleDouble::add(ans2, c2);

            if c2.hi / ans2.hi <= EPS {
                break;
            }
        }
        let v0 = DoubleDouble::quick_mult(v_e, ans2);
        return DoubleDouble::div_dd_f64(v0, da).to_f64();
    }

    let v_e = i0_exp(ax);

    let mut y = 1.0 - da;
    let mut z = dx + y + 1.0;
    let mut c = 0i32;

    let mut p3 = 1.0;
    let mut q3 = dx;
    let mut p2 = dx + 1.0;
    let mut q2 = z * dx;
    let mut ans = p2 / q2;

    for _ in 0..200 {
        y += 1.0;
        z += 2.0;
        c += 1;
        let yc = y * c as f64;

        let p = p2 * z - p3 * yc;
        let q = q2 * z - q3 * yc;

        p3 = p2;
        p2 = p;
        q3 = q2;
        q2 = q;

        if p.abs() > big {
            p3 *= big_inv;
            p2 *= big_inv;
            q3 *= big_inv;
            q2 *= big_inv;
        }

        if q != 0.0 {
            let nextans = p / q;
            let error = ((ans - nextans) / nextans).abs();
            ans = nextans;

            if error <= EPS {
                break;
            }
        }
    }

    DoubleDouble::mul_f64_add_f64(-v_e, ans, 1.0).to_f64()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_f_beta_pf() {
        assert_eq!(f_gamma_p(1., f64::INFINITY), f64::INFINITY);
        assert_eq!(f_gamma_p(23.421, 41.), 0.9988694746117834);
        assert_eq!(f_gamma_p(0.764, 0.432123), 0.47752996395412817);
        assert_eq!(f_gamma_p(0.421, 1.), 0.8727868618082306);
        assert!(f_gamma_p(-1., 12.).is_nan());
        assert!(f_gamma_p(1., -12.).is_nan());
        assert!(f_gamma_p(f64::NAN, 12.).is_nan());
        assert!(f_gamma_p(1., f64::NAN).is_nan());
        assert_eq!(f_gamma_p(f64::INFINITY, f64::INFINITY), f64::INFINITY);
        assert_eq!(f_gamma_p(f64::INFINITY, 5.32), f64::INFINITY);
    }
}
