/*
 * // Copyright (c) Radzivon Bartoshyk 6/2025. All rights reserved.
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
pub const fn roundf(x: f32) -> f32 {
    let mut i0 = x.to_bits() as i32;
    let j0 = ((i0 >> 23) & 0xff) - 0x7f;
    if j0 < 23 {
        if j0 < 0 {
            i0 &= 0x80000000u32 as i32;
            if j0 == -1 {
                i0 |= 0x3f800000;
            }
        } else {
            let i = 0x007fffff >> j0;
            if (i0 & i) == 0 {
                /* X is integral.  */
                return x;
            }

            i0 += 0x00400000 >> j0;
            i0 &= !i;
        }
    } else {
        return if j0 == 0x80 {
            /* Inf or NaN.  */
            x + x
        } else {
            x
        };
    }
    f32::from_bits(i0 as u32)
}

// infinity, NaNs are assumed already handled somewhere
#[inline]
pub(crate) fn froundf_finite(x: f32) -> f32 {
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1"
        ),
        target_arch = "aarch64"
    ))]
    {
        x.round()
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1"
        ),
        target_arch = "aarch64"
    )))]
    {
        roundf(x)
    }
}

#[inline]
pub const fn round(x: f64) -> f64 {
    let mut i0: i64 = x.to_bits() as i64;
    let j0: i32 = (((i0 >> 52) & 0x7ff) - 0x3ff) as i32;
    if j0 < 52 {
        if j0 < 0 {
            i0 &= 0x8000000000000000u64 as i64;
            if j0 == -1 {
                i0 |= 0x3ff0000000000000u64 as i64;
            }
        } else {
            let i = (0x000fffffffffffffu64 >> j0) as i64;
            if (i0 & i) == 0 {
                /* X is integral.  */
                return x;
            }

            i0 += (0x0008000000000000u64 >> j0) as i64;
            i0 &= !i;
        }
    } else {
        return if j0 == 0x400 {
            /* Inf or NaN.  */
            x + x
        } else {
            x
        };
    }
    f64::from_bits(i0 as u64)
}

// infinity, NaNs are assumed already handled somewhere
#[inline]
pub(crate) fn fround_finite(x: f64) -> f64 {
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1"
        ),
        target_arch = "aarch64"
    ))]
    {
        x.round()
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1"
        ),
        target_arch = "aarch64"
    )))]
    {
        round(x)
    }
}

pub(crate) trait CpuRound {
    fn cpu_round(self) -> Self;
}

impl CpuRound for f32 {
    #[inline]
    fn cpu_round(self) -> Self {
        froundf_finite(self)
    }
}

impl CpuRound for f64 {
    #[inline]
    fn cpu_round(self) -> Self {
        fround_finite(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundf() {
        assert_eq!(roundf(0f32), 0.0f32.round());
        assert_eq!(roundf(1f32), 1.0f32.round());
        assert_eq!(roundf(1.2f32), 1.2f32.round());
        assert_eq!(roundf(-1.2f32), (-1.2f32).round());
        assert_eq!(roundf(-1.6f32), (-1.6f32).round());
        assert_eq!(roundf(-1.5f32), (-1.5f32).round());
        assert_eq!(roundf(1.6f32), 1.6f32.round());
        assert_eq!(roundf(1.5f32), 1.5f32.round());
        assert_eq!(roundf(2.5f32), 2.5f32.round());
    }

    #[test]
    fn test_round() {
        assert_eq!(round(0.), 0.0f64.round());
        assert_eq!(round(1.), 1.0f64.round());
        assert_eq!(round(1.2), 1.2f64.round());
        assert_eq!(round(-1.2), (-1.2f64).round());
        assert_eq!(round(-1.6), (-1.6f64).round());
        assert_eq!(round(-1.5), (-1.5f64).round());
        assert_eq!(round(1.6), 1.6f64.round());
        assert_eq!(round(1.5), 1.5f64.round());
        assert_eq!(round(2.5), 2.5f64.round());
    }
}
