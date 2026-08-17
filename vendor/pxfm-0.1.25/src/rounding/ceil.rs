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
pub const fn ceilf(x: f32) -> f32 {
    /* Use generic implementation.  */
    let mut i0: i32 = x.to_bits() as i32;
    let j0 = ((i0 >> 23) & 0xff) - 0x7f;
    if j0 < 23 {
        if j0 < 0 {
            /* return 0 * sign (x) if |x| < 1  */
            if i0 < 0 {
                i0 = 0x80000000u32 as i32;
            } else if i0 != 0 {
                i0 = 0x3f800000u32 as i32;
            }
        } else {
            let i = (0x007fffff) >> j0;
            if (i0 & i) == 0 {
                return x; /* x is integral  */
            }
            if i0 > 0 {
                i0 = i0.wrapping_add((0x00800000) >> j0);
            }
            i0 &= !i;
        }
    } else {
        return if j0 == 0x80 {
            x + x /* inf or NaN  */
        } else {
            x /* x is integral  */
        };
    }
    f32::from_bits(i0 as u32)
}

#[inline]
pub const fn ceil(x: f64) -> f64 {
    let mut i0: i64 = x.to_bits() as i64;
    let j0: i32 = (((i0 >> 52) & 0x7ff) - 0x3ff) as i32;
    if j0 <= 51 {
        if j0 < 0 {
            /* return 0 * sign(x) if |x| < 1  */
            if i0 < 0 {
                i0 = 0x8000000000000000u64 as i64;
            } else if i0 != 0 {
                i0 = 0x3ff0000000000000u64 as i64;
            }
        } else {
            let i = (0x000fffffffffffffu64 as i64) >> j0;
            if (i0 & i) == 0 {
                return x; /* x is integral  */
            }
            if i0 > 0 {
                i0 = i0.wrapping_add((0x0010000000000000u64 >> j0) as i64);
            }
            i0 &= !i;
        }
    } else {
        return if j0 == 0x400 {
            x + x /* inf or NaN  */
        } else {
            x /* x is integral  */
        };
    }
    f64::from_bits(i0 as u64)
}

pub(crate) trait CpuCeil {
    fn cpu_ceil(self) -> Self;
}

impl CpuCeil for f32 {
    #[inline]
    fn cpu_ceil(self) -> Self {
        #[cfg(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        ))]
        {
            self.ceil()
        }
        #[cfg(not(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        )))]
        {
            ceilf(self)
        }
    }
}

impl CpuCeil for f64 {
    #[inline]
    fn cpu_ceil(self) -> Self {
        #[cfg(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        ))]
        {
            self.ceil()
        }
        #[cfg(not(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        )))]
        {
            ceil(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceilf() {
        assert_eq!(ceilf(0.0), 0.0);
        assert_eq!(ceilf(10.0), 10.0);
        assert_eq!(ceilf(10.1), 11.0);
        assert_eq!(ceilf(-9.0), -9.0);
        assert_eq!(ceilf(-9.5), -9.0);
    }

    #[test]
    fn test_ceil() {
        assert_eq!(ceil(0.0), 0.0);
        assert_eq!(ceil(10.0), 10.0);
        assert_eq!(ceil(10.1), 11.0);
        assert_eq!(ceil(-9.0), -9.0);
        assert_eq!(ceil(-9.5), -9.0);
    }
}
