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
use crate::gamma::lnbetaf::lnbetaf_core;
use crate::{f_exp, f_log, f_log1p, f_pow, f_powf};

/// Regularized incomplete beta
pub fn f_betainc_regf(a: f32, b: f32, x: f32) -> f32 {
    let aa = a.to_bits();
    let ab = b.to_bits();
    let ax = x.to_bits();

    if aa >= 0xffu32 << 23
        || aa == 0
        || ab >= 0xffu32 << 23
        || ab == 0
        || ax == 0
        || ax >= 0x3f800000
    {
        if (aa >> 31) != 0 || (ab >> 31) != 0 || (ax >> 31) != 0 {
            // |a| < 0 or |b| < 0
            return f32::NAN;
        }
        if ax >= 0x3f800000 {
            // |x| > 1
            if ax == 0x3f800000 {
                // |x| == 1
                return 1.;
            }
            return f32::NAN;
        }
        if aa.wrapping_shl(1) == 0 {
            // |a| == 0
            return 1.0;
        }
        if ab.wrapping_shl(1) == 0 {
            // |b| == 0
            return 0.;
        }
        if ax.wrapping_shl(1) == 0 {
            // |x| == 0
            return 0.;
        }
        if a.is_infinite() {
            // |a| == inf
            return 0.;
        }
        if b.is_infinite() {
            // |b| == inf
            return 1.;
        }
        return a + f32::NAN; // nan
    }

    if aa == 0x3f800000 {
        // a == 1
        return (1. - f_pow(1. - x as f64, b as f64)) as f32;
    }

    if ab == 0x3f800000 {
        // b == 1
        return f_powf(x, a);
    }

    /*The continued fraction converges nicely for x < (a+1)/(a+b+2)*/
    /*Use the fact that beta is symmetrical.*/
    let mut return_inverse = false;
    let mut dx = x as f64;
    let mut a = a;
    let mut b = b;
    if x > (a + 1.0) / (a + b + 2.0) {
        std::mem::swap(&mut a, &mut b);
        dx = 1.0 - dx;
        return_inverse = true;
    }
    /*Find the first part before the continued fraction.*/
    let da = a as f64;
    let db = b as f64;
    let w0 = f_fmla(f_log(dx), da, f_fmla(f_log1p(-dx), db, -lnbetaf_core(a, b)));
    let front = f_exp(w0) / da;

    /*Use Lentz's algorithm to evaluate the continued fraction.*/
    let mut f = 1.0;
    let mut c = 1.0;
    let mut d = 0.0;

    const TINY: f64 = 1.0e-30;
    const STOP: f64 = 1.0e-8;

    for i in 0..200 {
        let m = i / 2;
        let numerator: f64 = if i == 0 {
            1.0 /*First numerator is 1.0.*/
        } else if i % 2 == 0 {
            let m = m as f64;
            let c0 = f_fmla(2.0, m, da);
            (m * (db - m) * dx) / ((c0 - 1.0) * c0) /*Even term.*/
        } else {
            let m = m as f64;
            let c0 = f_fmla(2.0, m, da);
            -((da + m) * (da + db + m) * dx) / (c0 * (c0 + 1.)) /*Odd term.*/
        };

        /*Do an iteration of Lentz's algorithm.*/
        d = f_fmla(numerator, d, 1.0);
        if d.abs() < TINY {
            d = TINY;
        }
        d = 1.0 / d;

        c = 1.0 + numerator / c;
        if c.abs() < TINY {
            c = TINY;
        }

        let cd = c * d;
        f *= cd;

        /*Check for stop.*/
        if (1.0 - cd).abs() < STOP {
            return if return_inverse {
                f_fmla(-front, f - 1.0, 1.) as f32
            } else {
                (front * (f - 1.0)) as f32
            };
        }
    }

    if return_inverse {
        f_fmla(-front, f - 1.0, 1.) as f32
    } else {
        (front * (f - 1.0)) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_betaincf() {
        assert_eq!(f_betainc_regf(0.5, 2.5, 0.5), 0.9244131815783875);
        assert_eq!(f_betainc_regf(2.5, 1.0, 0.5), 0.1767766952966368811);
        assert_eq!(f_betainc_regf(0.5, 0.5, 0.5), 0.5);
        assert_eq!(f_betainc_regf(54221., 23124., 0.64534), 0.0);
        assert_eq!(f_betainc_regf(5., 1.4324, 0.1312), 8.872578e-5);
        assert_eq!(f_betainc_regf(7., 42., 0.4324), 0.99999547);
        assert_eq!(f_betainc_regf(0.5, 0., 1.), 1.);
        assert_eq!(f_betainc_regf(5., 2., 1.), 1.);
        assert_eq!(f_betainc_regf(5., 2., 0.), 0.);
        assert_eq!(f_betainc_regf(5., 2., 0.5), 0.109375);
        assert!(f_betainc_regf(5., 2., -1.).is_nan());
        assert!(f_betainc_regf(5., 2., 1.1).is_nan());
        assert!(f_betainc_regf(5., 2., f32::INFINITY).is_nan());
        assert!(f_betainc_regf(5., 2., f32::NEG_INFINITY).is_nan());
        assert!(f_betainc_regf(5., 2., f32::NAN).is_nan());
        assert!(f_betainc_regf(-5., 2., 0.432).is_nan());
        assert!(f_betainc_regf(5., -2., 0.432).is_nan());
        assert!(f_betainc_regf(5., 2., -0.432).is_nan());
    }
}
