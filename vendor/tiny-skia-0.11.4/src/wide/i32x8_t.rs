// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Based on https://github.com/Lokathor/wide (Zlib)

use bytemuck::cast;

use super::{f32x8, u32x8};

cfg_if::cfg_if! {
    if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(32))]
        pub struct i32x8(__m256i);
    } else {
        use super::i32x4;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(32))]
        pub struct i32x8(pub i32x4, pub i32x4);
    }
}

unsafe impl bytemuck::Zeroable for i32x8 {}
unsafe impl bytemuck::Pod for i32x8 {}

impl Default for i32x8 {
    fn default() -> Self {
        Self::splat(0)
    }
}

impl i32x8 {
    pub fn splat(n: i32) -> Self {
        cast([n, n, n, n, n, n, n, n])
    }

    pub fn blend(self, t: Self, f: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                Self(unsafe { _mm256_blendv_epi8(f.0, t.0, self.0) })
            } else {
                Self(self.0.blend(t.0, f.0), self.1.blend(t.1, f.1))
            }
        }
    }

    pub fn cmp_eq(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                Self(unsafe { _mm256_cmpeq_epi32(self.0, rhs.0) })
            } else {
                Self(self.0.cmp_eq(rhs.0), self.1.cmp_eq(rhs.1))
            }
        }
    }

    pub fn cmp_gt(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                Self(unsafe { _mm256_cmpgt_epi32(self.0, rhs.0) })
            } else {
                Self(self.0.cmp_gt(rhs.0), self.1.cmp_gt(rhs.1))
            }
        }
    }

    pub fn cmp_lt(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                // There is no `_mm256_cmpLT_epi32`, therefore we have to use
                // `_mm256_cmpGT_epi32` and then invert the result.
                let v = unsafe { _mm256_cmpgt_epi32(self.0, rhs.0) };
                let all_bits = unsafe { _mm256_set1_epi16(-1) };
                Self(unsafe { _mm256_xor_si256(v, all_bits) })
            } else {
                Self(self.0.cmp_lt(rhs.0), self.1.cmp_lt(rhs.1))
            }
        }
    }

    pub fn to_f32x8(self) -> f32x8 {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                cast(unsafe { _mm256_cvtepi32_ps(self.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "avx"))] {
                cast([self.0.to_f32x4(), self.1.to_f32x4()])
            } else {
                f32x8(self.0.to_f32x4(), self.1.to_f32x4())
            }
        }
    }

    pub fn to_u32x8_bitcast(self) -> u32x8 {
        bytemuck::cast(self)
    }

    pub fn to_f32x8_bitcast(self) -> f32x8 {
        bytemuck::cast(self)
    }
}

impl From<[i32; 8]> for i32x8 {
    fn from(v: [i32; 8]) -> Self {
        cast(v)
    }
}

impl From<i32x8> for [i32; 8] {
    fn from(v: i32x8) -> Self {
        cast(v)
    }
}

impl core::ops::Add for i32x8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                Self(unsafe { _mm256_add_epi32(self.0, rhs.0) })
            } else {
                Self(self.0 + rhs.0, self.1 + rhs.1)
            }
        }
    }
}

impl core::ops::BitAnd for i32x8 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                Self(unsafe { _mm256_and_si256(self.0, rhs.0) })
            } else {
                Self(self.0 & rhs.0, self.1 & rhs.1)
            }
        }
    }
}

impl core::ops::Mul for i32x8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                Self(unsafe { _mm256_mullo_epi32(self.0, rhs.0) })
            } else {
                Self(self.0 * rhs.0, self.1 * rhs.1)
            }
        }
    }
}

impl core::ops::BitOr for i32x8 {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                Self(unsafe { _mm256_or_si256(self.0, rhs.0) })
            } else {
                Self(self.0 | rhs.0, self.1 | rhs.1)
            }
        }
    }
}

impl core::ops::BitXor for i32x8 {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                Self(unsafe { _mm256_xor_si256(self.0, rhs.0) })
            } else {
                Self(self.0 ^ rhs.0, self.1 ^ rhs.1)
            }
        }
    }
}
