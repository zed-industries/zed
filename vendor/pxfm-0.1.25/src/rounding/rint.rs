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
pub const fn rint(x: f64) -> f64 {
    /* Use generic implementation.  */
    static TWO52: [f64; 2] = [
        4.50359962737049600000e+15,  /* 0x43300000, 0x00000000 */
        -4.50359962737049600000e+15, /* 0xC3300000, 0x00000000 */
    ];
    let mut i0: i64 = x.to_bits() as i64;
    let sx = (i0 >> 63) & 1;
    let j0: i32 = (((i0 >> 52) & 0x7ff) - 0x3ff) as i32;
    if j0 < 52 {
        if j0 < 0 {
            let w = TWO52[sx as usize] + x;
            let t = w - TWO52[sx as usize];
            i0 = t.to_bits() as i64;
            let u = (i0 & (0x7fffffffffffffffu64 as i64)) | (sx << 63);
            return f64::from_bits(u as u64);
        }
    } else {
        return if j0 == 0x400 {
            x + x /* inf or NaN  */
        } else {
            x /* x is integral  */
        };
    }
    let w = TWO52[sx as usize] + x;
    w - TWO52[sx as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rintf() {
        assert_eq!(rint(0f64), 0.0f64.round());
        assert_eq!(rint(1f64), 1.0f64.round());
        assert_eq!(rint(1.2f64), 1.2f64.round());
        assert_eq!(rint(-1.2f64), (-1.2f64).round());
        assert_eq!(rint(-1.6f64), (-1.6f64).round());
        assert_eq!(rint(-1.5f64), (-1.5f64).round());
        assert_eq!(rint(1.6f64), 1.6f64.round());
        assert_eq!(rint(1.5f64), 1.5f64.round());
        assert_eq!(rint(2.5f64), 2.0f64);
        assert_eq!(rint(3.5f64), 4.0f64);
        assert_eq!(rint(-2.5f64), -2.0f64);
        assert_eq!(rint(-3.5f64), -4.0f64);
        assert_eq!(rint(f64::INFINITY), f64::INFINITY);
        assert_eq!(rint(f64::NEG_INFINITY), f64::NEG_INFINITY);
        assert!(rint(f64::NAN).is_nan());
    }
}
