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

#[inline]
pub const fn truncf(x: f32) -> f32 {
    let i0 = x.to_bits() as i32;
    let sx = i0 & (0x80000000u32 as i32);
    let j0 = ((i0 >> 23) & 0xff) - 0x7f;
    if j0 < 23 {
        return if j0 < 0 {
            /* The magnitude of the number is < 1 so the result is +-0.  */
            f32::from_bits(sx as u32)
        } else {
            f32::from_bits((sx | (i0 & !(0x007fffff >> j0))) as u32)
        };
    } else if j0 == 0x80 {
        /* x is inf or NaN.  */
        return x + x;
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncf() {
        assert_eq!(truncf(-1.0), -1.0);
        assert_eq!(truncf(1.0), 1.0);
        assert_eq!(truncf(1.234211), 1.0);
        assert_eq!(truncf(-1.234211), -1.0);
        assert_eq!(truncf(f32::INFINITY), f32::INFINITY);
        assert_eq!(truncf(f32::NEG_INFINITY), f32::NEG_INFINITY);
        assert!(truncf(f32::NAN).is_nan());
    }
}
