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

pub const fn roundf_ties_even(x: f32) -> f32 {
    const BIAS: i32 = 0x7f;
    const MANT_DIG: i32 = 24;
    const MAX_EXP: i32 = 2 * BIAS + 1;
    let mut ix: u32 = x.to_bits();
    let ux: u32 = ix & 0x7fffffff;
    let exponent: i32 = (ux >> (MANT_DIG - 1)) as i32;
    if exponent >= BIAS + MANT_DIG - 1 {
        /* Integer, infinity or NaN.  */
        if exponent == MAX_EXP {
            /* Infinity or NaN; quiet signaling NaNs.  */
            return x + x;
        } else {
            return x;
        }
    } else if exponent >= BIAS {
        /* At least 1; not necessarily an integer.  Locate the bits with
        exponents 0 and -1 (when the unbiased exponent is 0, the bit
        with exponent 0 is implicit, but as the bias is odd it is OK
        to take it from the low bit of the exponent).  */
        let int_pos: i32 = (BIAS + MANT_DIG - 1) - exponent;
        let half_pos: i32 = int_pos - 1;
        let half_bit: u32 = 1u32 << half_pos;
        let int_bit: u32 = 1u32 << int_pos;
        if (ix & (int_bit | (half_bit - 1))) != 0 {
            /* Carry into the exponent works correctly.  No need to test
            whether HALF_BIT is set.  */
            ix = ix.wrapping_add(half_bit);
        }
        ix &= !(int_bit - 1);
    } else if exponent == BIAS - 1 && ux > 0x3f000000 {
        /* Interval (0.5, 1).  */
        ix = (ix & 0x80000000) | 0x3f800000;
    } else {
        /* Rounds to 0.  */
        ix &= 0x80000000;
    }
    f32::from_bits(ix)
}

pub const fn round_ties_even(x: f64) -> f64 {
    let mut ix: u64 = x.to_bits();
    let ux = ix & 0x7fffffffffffffffu64;
    const BIAS: i32 = 0x3ff;
    const MANT_DIG: i32 = 53;
    const MAX_EXP: i32 = 2 * BIAS + 1;
    let exponent: i32 = (ux >> (MANT_DIG - 1)) as i32;
    if exponent >= BIAS + MANT_DIG - 1 {
        /* Integer, infinity or NaN.  */
        return if exponent == MAX_EXP {
            /* Infinity or NaN; quiet signaling NaNs.  */
            x + x
        } else {
            x
        };
    } else if exponent >= BIAS {
        /* At least 1; not necessarily an integer.  Locate the bits with
        exponents 0 and -1 (when the unbiased exponent is 0, the bit
        with exponent 0 is implicit, but as the bias is odd it is OK
        to take it from the low bit of the exponent).  */
        let int_pos: i32 = (BIAS + MANT_DIG - 1) - exponent;
        let half_pos: i32 = int_pos - 1;
        let half_bit: u64 = 1u64 << half_pos;
        let int_bit: u64 = 1u64 << int_pos;
        if (ix & (int_bit | (half_bit - 1))) != 0 {
            /* Carry into the exponent works correctly.  No need to test
            whether HALF_BIT is set.  */
            ix = ix.wrapping_add(half_bit);
        }
        ix &= !(int_bit - 1);
    } else if exponent == BIAS - 1 && ux > 0x3fe0000000000000u64 {
        /* Interval (0.5, 1).  */
        ix = (ix & 0x8000000000000000u64) | 0x3ff0000000000000u64;
    } else {
        /* Rounds to 0.  */
        ix &= 0x8000000000000000u64;
    }
    f64::from_bits(ix)
}

pub(crate) trait CpuRoundTiesEven {
    fn cpu_round_ties_even(self) -> Self;
}

impl CpuRoundTiesEven for f32 {
    #[inline]
    fn cpu_round_ties_even(self) -> Self {
        #[cfg(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        ))]
        {
            self.round_ties_even()
        }
        #[cfg(not(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        )))]
        {
            roundf_ties_even(self)
        }
    }
}

impl CpuRoundTiesEven for f64 {
    #[inline]
    fn cpu_round_ties_even(self) -> Self {
        #[cfg(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        ))]
        {
            self.round_ties_even()
        }
        #[cfg(not(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        )))]
        {
            round_ties_even(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundf_ties_even() {
        assert_eq!(roundf_ties_even(0f32), 0.0f32.round_ties_even());
        assert_eq!(roundf_ties_even(0.5f32), 0.5f32.round_ties_even());
        assert_eq!(roundf_ties_even(-0.5), (-0.5f32).round_ties_even());
        assert_eq!(roundf_ties_even(1f32), 1.0f32.round_ties_even());
        assert_eq!(roundf_ties_even(1.2f32), 1.2f32.round_ties_even());
        assert_eq!(roundf_ties_even(-1.2f32), (-1.2f32).round_ties_even());
        assert_eq!(roundf_ties_even(-1.6f32), (-1.6f32).round_ties_even());
        assert_eq!(roundf_ties_even(-1.5f32), (-1.5f32).round_ties_even());
        assert_eq!(roundf_ties_even(1.6f32), 1.6f32.round_ties_even());
        assert_eq!(roundf_ties_even(1.5f32), 1.5f32.round_ties_even());
        assert_eq!(roundf_ties_even(2.5f32), 2.5f32.round_ties_even());
        assert_eq!(
            roundf_ties_even(f32::INFINITY),
            f32::INFINITY.round_ties_even()
        );
        assert_eq!(
            roundf_ties_even(f32::NEG_INFINITY),
            f32::NEG_INFINITY.round_ties_even()
        );
        assert!(roundf_ties_even(f32::NAN).is_nan());
    }

    #[test]
    fn test_round_ties_even() {
        assert_eq!(
            round_ties_even(5.6916e-320),
            (5.6916e-320f64).round_ties_even()
        );
        assert_eq!(round_ties_even(3f64), 3f64.round_ties_even());
        assert_eq!(round_ties_even(2f64), 2f64.round_ties_even());
        assert_eq!(round_ties_even(0.), 0.0f64.round_ties_even());
        assert_eq!(round_ties_even(0.5), 0.5f64.round_ties_even());
        assert_eq!(round_ties_even(-0.5), (-0.5f64).round_ties_even());
        assert_eq!(round_ties_even(1.), 1.0f64.round_ties_even());
        assert_eq!(round_ties_even(1.2), 1.2f64.round_ties_even());
        assert_eq!(round_ties_even(-1.2), (-1.2f64).round_ties_even());
        assert_eq!(round_ties_even(-1.6), (-1.6f64).round_ties_even());
        assert_eq!(round_ties_even(-1.5), (-1.5f64).round_ties_even());
        assert_eq!(round_ties_even(1.6), 1.6f64.round_ties_even());
        assert_eq!(round_ties_even(1.5), 1.5f64.round_ties_even());
        assert_eq!(round_ties_even(2.5), 2.5f64.round_ties_even());
        assert_eq!(round_ties_even(-2.5), (-2.5f64).round_ties_even());
        assert_eq!(
            round_ties_even(f64::INFINITY),
            f64::INFINITY.round_ties_even()
        );
        assert_eq!(
            round_ties_even(f64::NEG_INFINITY),
            f64::NEG_INFINITY.round_ties_even()
        );
        assert!(round_ties_even(f64::NAN).is_nan());
    }
}
