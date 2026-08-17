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
use crate::logs::{fast_log_d_to_dd, log1p_fast_dd};

/// Inverse logistic function
pub fn f_logit(x: f64) -> f64 {
    let ux = x.to_bits();
    if ux >= 0x3ff0000000000000u64 || ux == 0 {
        // |x| == 0, |x| == inf, |x| == NaN, x < 0, x >= 1
        if ux.wrapping_shl(1) == 0 {
            return 0.;
        }
        if ux == 0x3ff0000000000000u64 {
            // x == 1
            return f64::INFINITY;
        }
        return f64::NAN;
    }
    // logit(x) = log(x/(1-p)) = log(x) - log(1-p)
    let v0 = fast_log_d_to_dd(x);
    let v1 = log1p_fast_dd(-x);
    let product = DoubleDouble::quick_dd_sub(v0, v1);
    product.to_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logit() {
        assert_eq!(f_logit(0.70000000000007534), 0.8472978603875624);
        assert_eq!(f_logit(0.234), -1.1858610543828898);
        assert_eq!(f_logit(0.142112), -1.7978580757271523);
        assert_eq!(f_logit(0.86543), 1.8611419834868623);
        assert_eq!(f_logit(0.21312), -1.3062203716758034);
        assert_eq!(f_logit(0.128765), -1.91192270767172);
        assert_eq!(f_logit(1.), f64::INFINITY);
        assert!(f_logit(-0.00123).is_nan());
        assert!(f_logit(f64::NAN).is_nan());
    }
}
