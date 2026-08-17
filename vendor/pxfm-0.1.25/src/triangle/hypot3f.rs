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
use crate::common::EXP_MASK_F32;

pub fn f_hypot3f(x: f32, y: f32, z: f32) -> f32 {
    let x_abs = f32::from_bits(x.to_bits() & 0x7fff_ffffu32);
    let y_abs = f32::from_bits(y.to_bits() & 0x7fff_ffffu32);
    let z_abs = f32::from_bits(z.to_bits() & 0x7fff_ffffu32);

    let a_bits = x_abs.to_bits().max(y_abs.to_bits()).max(z_abs.to_bits());
    let b_bits = x_abs.to_bits().min(y_abs.to_bits()).min(z_abs.to_bits());

    let a_u = a_bits;
    let b_u = b_bits;

    if a_u >= EXP_MASK_F32 {
        // x or y is inf or nan
        if x_abs.is_nan() || y_abs.is_nan() || z_abs.is_nan() {
            return f32::NAN;
        }
        if x_abs.is_infinite() || y_abs.is_infinite() || z_abs.is_infinite() {
            return f32::INFINITY;
        }
        return f32::from_bits(a_bits);
    }

    if a_u.wrapping_sub(b_u) >= ((23u32 + 2) << 23) {
        return x_abs + y_abs + z_abs;
    }

    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    ))]
    {
        let ad = x as f64;
        let bd = y as f64;
        let cd = z as f64;
        use crate::common::f_fmla;
        // for FMA environment we're using Kahan style summation which is short and reliable.
        let w = bd * bd; // RN(bc)
        let e = f_fmla(-bd, bd, w); // RN(w − bc)
        let f = f_fmla(ad, ad, w); // RN(ad + w)
        let f0 = f_fmla(cd, cd, f); // RN(cd + f)
        let r = e + f0; // RN(f + e)
        let hyp = r.sqrt(); // sqrt(x^2 + y^2)
        hyp as f32
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    )))]
    {
        let ad = x_abs as f64;
        let bd = y_abs as f64;
        let cd = z_abs as f64;
        use crate::double_double::DoubleDouble;
        let da = DoubleDouble::from_exact_mult(bd, bd);
        let db = DoubleDouble::from_exact_mult(ad, ad);
        let dc = DoubleDouble::from_exact_mult(cd, cd);
        let f = DoubleDouble::add(DoubleDouble::add(da, db), dc);
        let cath = f.to_f64().sqrt();
        cath as f32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hypot3f() {
        assert_eq!(f_hypot3f(3.0, 4.0, 12.0), 13.0);
        assert_eq!(f_hypot3f(6.0, 8.0, 24.0), 26.0);
        assert_eq!(f_hypot3f(5.0, 12.0, 84.0), 85.0);
        assert_eq!(f_hypot3f(9.0, 12.0, 20.0), 25.0);
        assert_eq!(f_hypot3f(1e20, 3.0, 4.0), 1e20);
        assert_eq!(f_hypot3f(1e-20, 1e-20, 1.0), 1.);
        assert_eq!(
            f_hypot3f(f32::MIN_POSITIVE, f32::MIN_POSITIVE, 0.0),
            1.6624e-38
        );
        assert_eq!(f_hypot3f(f32::MAX, f32::MAX, 0.), f32::INFINITY);
        assert_eq!(f_hypot3f(f32::MAX, 0., 0.), 3.4028235e38);
        assert_eq!(f_hypot3f(f32::INFINITY, 0., 0.), f32::INFINITY);
        assert_eq!(f_hypot3f(0., f32::INFINITY, 0.), f32::INFINITY);
        assert_eq!(f_hypot3f(0., 0., f32::INFINITY), f32::INFINITY);
        assert!(f_hypot3f(f32::NAN, 0., 0.).is_nan());
        assert!(f_hypot3f(0., f32::NAN, 0.).is_nan());
        assert!(f_hypot3f(0., 0., f32::NAN).is_nan());
    }
}
