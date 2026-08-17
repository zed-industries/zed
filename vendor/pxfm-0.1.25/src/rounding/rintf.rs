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
pub const fn rintf(x: f32) -> f32 {
    /* Use generic implementation.  */
    static TWO23: [f32; 2] = [
        8.3886080000e+06,  /* 0x4b000000 */
        -8.3886080000e+06, /* 0xcb000000 */
    ];
    let mut i0: i32 = x.to_bits() as i32;
    let sx = (i0 >> 31) & 1;
    let j0 = ((i0 >> 23) & 0xff) - 0x7f;
    if j0 < 23 {
        if j0 < 0 {
            let w = TWO23[sx as usize] + x;
            let t = w - TWO23[sx as usize];
            i0 = t.to_bits() as i32;
            let u = (i0 & 0x7fffffff) | (sx << 31);
            return f32::from_bits(u as u32);
        }
    } else {
        return if j0 == 0x80 {
            x + x /* inf or NaN  */
        } else {
            x /* x is integral  */
        };
    }
    let w = TWO23[sx as usize] + x;
    w - TWO23[sx as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rintf() {
        assert_eq!(rintf(0f32), 0.0f32.round());
        assert_eq!(rintf(1f32), 1.0f32.round());
        assert_eq!(rintf(1.2f32), 1.2f32.round());
        assert_eq!(rintf(-1.2f32), (-1.2f32).round());
        assert_eq!(rintf(-1.6f32), (-1.6f32).round());
        assert_eq!(rintf(-1.5f32), (-1.5f32).round());
        assert_eq!(rintf(1.6f32), 1.6f32.round());
        assert_eq!(rintf(1.5f32), 1.5f32.round());
        assert_eq!(rintf(2.5f32), 2.0f32);
        assert_eq!(rintf(3.5f32), 4.0f32);
        assert_eq!(rintf(-2.5f32), -2.0f32);
        assert_eq!(rintf(-3.5f32), -4.0f32);
        assert_eq!(rintf(f32::INFINITY), f32::INFINITY);
        assert_eq!(rintf(f32::NEG_INFINITY), f32::NEG_INFINITY);
        assert!(rintf(f32::NAN).is_nan());
    }
}
