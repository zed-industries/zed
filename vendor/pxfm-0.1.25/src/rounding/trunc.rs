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
pub const fn trunc(x: f64) -> f64 {
    let i0 = x.to_bits() as i64;
    let sx = i0 & (0x8000000000000000u64 as i64);
    let j0 = ((i0 >> 52) & 0x7ff) - 0x3ff;
    if j0 < 52 {
        return if j0 < 0 {
            /* The magnitude of the number is < 1 so the result is +-0.  */
            f64::from_bits(sx as u64)
        } else {
            f64::from_bits((sx | (i0 & (!(0x000fffffffffffffu64 >> j0) as i64))) as u64)
        };
    } else if j0 == 0x400 {
        /* x is inf or NaN.  */
        return x + x;
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trunc() {
        assert_eq!(trunc(-1.0), -1.0);
        assert_eq!(trunc(1.0), 1.0);
        assert_eq!(trunc(1.234211), 1.0);
        assert_eq!(trunc(-1.234211), -1.0);
        assert_eq!(trunc(f64::INFINITY), f64::INFINITY);
        assert_eq!(trunc(f64::NEG_INFINITY), f64::NEG_INFINITY);
        assert!(trunc(f64::NAN).is_nan());
    }
}
