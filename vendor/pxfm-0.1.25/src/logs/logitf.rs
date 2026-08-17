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
use crate::logs::log1pf::{core_log1pf, core_logf};

/// Inverse logistic function
pub fn f_logitf(x: f32) -> f32 {
    let ux = x.to_bits();
    if ux >= 0x3f80_0000u32 || ux == 0 {
        // |x| == 0, |x| == inf, |x| == NaN, x < 0, x >= 1
        if ux.wrapping_shl(1) == 0 {
            return 0.;
        }
        if ux == 0x3f80_0000u32 {
            // x == 1
            return f32::INFINITY;
        }
        return f32::NAN;
    }
    // logit(x) = log(x/(1-p)) = log(x) - log(1-p)
    let v0 = core_logf(x as f64);
    let v1 = core_log1pf(-x);
    (v0 - v1) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logit() {
        assert_eq!(f_logitf(0.234), -1.1858611);
        assert_eq!(f_logitf(0.142112), -1.7978581);
        assert_eq!(f_logitf(0.86543), 1.8611419);
        assert_eq!(f_logitf(0.21312), -1.3062204);
        assert_eq!(f_logitf(0.128765), -1.9119227);
        assert_eq!(f_logitf(1.), f32::INFINITY);
        assert!(f_logitf(-0.00123).is_nan());
        assert!(f_logitf(f32::NAN).is_nan());
    }
}
