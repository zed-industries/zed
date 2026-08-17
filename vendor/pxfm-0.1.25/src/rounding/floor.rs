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

/// Round to integer towards minus infinity
#[inline]
pub const fn floorf(x: f32) -> f32 {
    let mut i0 = x.to_bits() as i32;
    let j0 = ((i0 >> 23) & 0xff) - 0x7f;
    if j0 < 23 {
        if j0 < 0 {
            /* return 0 * sign (x) if |x| < 1  */
            if i0 >= 0 {
                i0 = 0;
            } else if (i0 & 0x7fffffff) != 0 {
                i0 = 0xbf800000u32 as i32;
            }
        } else {
            let i = (0x007fffff) >> j0;
            if (i0 & i) == 0 {
                return x; /* x is integral  */
            }
            if i0 < 0 {
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

/// Floors value
#[inline]
pub const fn floor(x: f64) -> f64 {
    let mut i0 = x.to_bits() as i64;
    let j0 = (((i0 >> 52) & 0x7ff) as i32) - 0x3ff;
    let mut x = x;
    if j0 < 52 {
        if j0 < 0 {
            /* return 0 * sign (x) if |x| < 1  */
            if i0 >= 0 {
                i0 = 0;
            } else if (i0 & 0x7fffffffffffffffi64) != 0 {
                i0 = 0xbff0000000000000u64 as i64;
            }
        } else {
            let i = 0x000fffffffffffffi64 >> j0;
            if (i0 & i) == 0 {
                return x; /* x is integral */
            }
            if i0 < 0 {
                i0 = i0.wrapping_add((0x0010000000000000u64 >> j0) as i64);
            }
            i0 &= !i;
        }
        x = f64::from_bits(i0 as u64);
    } else if j0 == 0x400 {
        return x + x; /* inf or NaN */
    }
    x
}

pub(crate) trait CpuFloor {
    fn cpu_floor(self) -> Self;
}

impl CpuFloor for f32 {
    #[inline]
    fn cpu_floor(self) -> Self {
        #[cfg(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        ))]
        {
            self.floor()
        }
        #[cfg(not(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        )))]
        {
            floorf(self)
        }
    }
}

impl CpuFloor for f64 {
    #[inline]
    fn cpu_floor(self) -> Self {
        #[cfg(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        ))]
        {
            self.floor()
        }
        #[cfg(not(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "sse4.1"
            ),
            target_arch = "aarch64"
        )))]
        {
            floor(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floorf() {
        assert_eq!(floorf(-1.0), -1.0);
        assert_eq!(floorf(1.0), 1.0);
        assert_eq!(floorf(0.0), 0.0);
        assert_eq!(floorf(0.324), 0.0);
        assert_eq!(floorf(-0.324), -1.0);
        assert_eq!(floorf(1.234211), 1.0);
        assert_eq!(floorf(-1.234211), -2.0);
        assert_eq!(floorf(f32::INFINITY), f32::INFINITY);
        assert_eq!(floorf(f32::NEG_INFINITY), f32::NEG_INFINITY);
        assert!(floorf(f32::NAN).is_nan());
    }

    #[test]
    fn test_floor() {
        assert_eq!(floor(-1.0), -1.0);
        assert_eq!(floor(1.0), 1.0);
        assert_eq!(floor(0.0), 0.0);
        assert_eq!(floor(0.324), 0.0);
        assert_eq!(floor(-0.324), -1.0);
        assert_eq!(floor(1.234211), 1.0);
        assert_eq!(floor(-1.234211), -2.0);
        assert_eq!(floor(f64::INFINITY), f64::INFINITY);
        assert_eq!(floor(f64::NEG_INFINITY), f64::NEG_INFINITY);
        assert!(floor(f64::NAN).is_nan());
    }
}
