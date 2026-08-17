// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Based on https://github.com/Lokathor/wide (Zlib)

use super::{f32x8, i32x8};

cfg_if::cfg_if! {
    if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        use bytemuck::cast;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(32))]
        pub struct u32x8(__m256i);
    } else {
        use super::u32x4;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(32))]
        pub struct u32x8(u32x4, u32x4);
    }
}

unsafe impl bytemuck::Zeroable for u32x8 {}
unsafe impl bytemuck::Pod for u32x8 {}

impl Default for u32x8 {
    fn default() -> Self {
        Self::splat(0)
    }
}

impl u32x8 {
    pub fn splat(n: u32) -> Self {
        bytemuck::cast([n, n, n, n, n, n, n, n])
    }

    pub fn to_i32x8_bitcast(self) -> i32x8 {
        bytemuck::cast(self)
    }

    pub fn to_f32x8_bitcast(self) -> f32x8 {
        bytemuck::cast(self)
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

    pub fn shl<const RHS: i32>(self) -> Self {
        cfg_if::cfg_if! {
           if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                let shift: __m128i = cast([RHS as u64, 0]);
                Self(unsafe { _mm256_sll_epi32(self.0, shift) })
            } else {
                Self(self.0.shl::<RHS>(), self.1.shl::<RHS>())
            }
        }
    }

    pub fn shr<const RHS: i32>(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                let shift: __m128i = cast([RHS as u64, 0]);
                Self(unsafe { _mm256_srl_epi32(self.0, shift) })
            } else {
                Self(self.0.shr::<RHS>(), self.1.shr::<RHS>())
            }
        }
    }
}

impl core::ops::Not for u32x8 {
    type Output = Self;

    fn not(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "avx2"))] {
                let all_bits = unsafe { _mm256_set1_epi16(-1) };
                Self(unsafe { _mm256_xor_si256(self.0, all_bits) })
            } else {
                Self(!self.0, !self.1)
            }
        }
    }
}

impl core::ops::Add for u32x8 {
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

impl core::ops::BitAnd for u32x8 {
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
