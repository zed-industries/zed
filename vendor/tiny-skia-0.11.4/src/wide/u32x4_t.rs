// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Based on https://github.com/Lokathor/wide (Zlib)

cfg_if::cfg_if! {
    if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        // unused when AVX is available
        #[cfg(not(all(feature = "simd", target_feature = "avx2")))]
        use bytemuck::cast;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(16))]
        pub struct u32x4(__m128i);
    } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
        use core::arch::wasm32::*;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(16))]
        pub struct u32x4(v128);
    } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
        use core::arch::aarch64::*;

        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(16))]
        pub struct u32x4(uint32x4_t);
    } else {
        #[derive(Clone, Copy, Debug)]
        #[repr(C, align(16))]
        pub struct u32x4([u32; 4]);
    }
}

unsafe impl bytemuck::Zeroable for u32x4 {}
unsafe impl bytemuck::Pod for u32x4 {}

impl Default for u32x4 {
    fn default() -> Self {
        Self::splat(0)
    }
}

impl u32x4 {
    pub fn splat(n: u32) -> Self {
        bytemuck::cast([n, n, n, n])
    }

    // unused when AVX is available
    #[cfg(not(all(feature = "simd", target_feature = "avx2")))]
    pub fn cmp_eq(self, rhs: Self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_cmpeq_epi32(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(u32x4_eq(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vceqq_u32(self.0, rhs.0) })
            } else {
                Self([
                    if self.0[0] == rhs.0[0] { u32::MAX } else { 0 },
                    if self.0[1] == rhs.0[1] { u32::MAX } else { 0 },
                    if self.0[2] == rhs.0[2] { u32::MAX } else { 0 },
                    if self.0[3] == rhs.0[3] { u32::MAX } else { 0 },
                ])
            }
        }
    }

    // unused when AVX is available
    #[cfg(not(all(feature = "simd", target_feature = "avx2")))]
    pub fn shl<const RHS: i32>(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                let shift = cast([RHS as u64, 0]);
                Self(unsafe { _mm_sll_epi32(self.0, shift) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(u32x4_shl(self.0, RHS as _))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vshlq_n_u32::<RHS>(self.0) })
            } else {
                let u = RHS as u64;
                Self([
                    self.0[0] << u,
                    self.0[1] << u,
                    self.0[2] << u,
                    self.0[3] << u,
                ])
            }
        }
    }

    // unused when AVX is available
    #[cfg(not(all(feature = "simd", target_feature = "avx2")))]
    pub fn shr<const RHS: i32>(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                let shift: __m128i = cast([RHS as u64, 0]);
                Self(unsafe { _mm_srl_epi32(self.0, shift) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(u32x4_shr(self.0, RHS as _))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vshrq_n_u32::<RHS>(self.0) })
            } else {
                let u = RHS as u64;
                Self([
                    self.0[0] >> u,
                    self.0[1] >> u,
                    self.0[2] >> u,
                    self.0[3] >> u,
                ])
            }
        }
    }
}

impl core::ops::Not for u32x4 {
    type Output = Self;

    fn not(self) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                let all_bits = unsafe { _mm_set1_epi32(-1) };
                Self(unsafe { _mm_xor_si128(self.0, all_bits) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(v128_not(self.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vmvnq_u32(self.0) })
            } else {
                Self([
                    !self.0[0],
                    !self.0[1],
                    !self.0[2],
                    !self.0[3],
                ])
            }
        }
    }
}

impl core::ops::Add for u32x4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_add_epi32(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(u32x4_add(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vaddq_u32(self.0, rhs.0) })
            } else {
                Self([
                    self.0[0].wrapping_add(rhs.0[0]),
                    self.0[1].wrapping_add(rhs.0[1]),
                    self.0[2].wrapping_add(rhs.0[2]),
                    self.0[3].wrapping_add(rhs.0[3]),
                ])
            }
        }
    }
}

impl core::ops::BitAnd for u32x4 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "simd", target_feature = "sse2"))] {
                Self(unsafe { _mm_and_si128(self.0, rhs.0) })
            } else if #[cfg(all(feature = "simd", target_feature = "simd128"))] {
                Self(v128_and(self.0, rhs.0))
            } else if #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))] {
                Self(unsafe { vandq_u32(self.0, rhs.0) })
            } else {
                Self([
                    self.0[0] & rhs.0[0],
                    self.0[1] & rhs.0[1],
                    self.0[2] & rhs.0[2],
                    self.0[3] & rhs.0[3],
                ])
            }
        }
    }
}
