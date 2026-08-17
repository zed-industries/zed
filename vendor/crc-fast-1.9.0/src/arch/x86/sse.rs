// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides x86-specific implementations of the ArchOps trait.
//!
//! This module is designed to work with both x86 and x86_64 architectures.
//!
//! It uses the SSE4.1 instruction sets for SIMD operations.

#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::traits::ArchOps;

// Tier-specific ArchOps implementations for x86/x86_64

/// Base x86/x86_64 SSE+PCLMULQDQ implementation - baseline performance for both architectures
/// Uses SSE4.1 and PCLMULQDQ instructions
#[derive(Debug, Copy, Clone)]
pub struct X86SsePclmulqdqOps;

// Base implementation for x86/x86_64 SSE+PCLMULQDQ tier
impl ArchOps for X86SsePclmulqdqOps {
    type Vector = __m128i;

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn create_vector_from_u64_pair(
        &self,
        high: u64,
        low: u64,
        reflected: bool,
    ) -> Self::Vector {
        // Note order is different from AArch64
        if reflected {
            self.set_epi64x(low, high)
        } else {
            self.set_epi64x(high, low)
        }
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn create_vector_from_u64_pair_non_reflected(
        &self,
        high: u64,
        low: u64,
    ) -> Self::Vector {
        // Note order is different from AArch64
        self.set_epi64x(high, low)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn create_vector_from_u64(&self, value: u64, high: bool) -> Self::Vector {
        self.create_u64_vector(value, high)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn extract_u64s(&self, vector: Self::Vector) -> [u64; 2] {
        [self.extract_u64_low(vector), self.extract_u64_high(vector)]
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn extract_poly64s(&self, vector: Self::Vector) -> [u64; 2] {
        // On x86, poly64s and u64s extraction is the same
        self.extract_u64s(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn xor_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        _mm_xor_si128(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn load_bytes(&self, ptr: *const u8) -> Self::Vector {
        _mm_loadu_si128(ptr as *const __m128i)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn load_aligned(&self, ptr: *const [u64; 2]) -> Self::Vector {
        _mm_loadu_si128(ptr as *const __m128i)
    }

    #[inline]
    #[target_feature(enable = "ssse3")]
    unsafe fn shuffle_bytes(&self, data: Self::Vector, mask: Self::Vector) -> Self::Vector {
        _mm_shuffle_epi8(data, mask)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn blend_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        mask: Self::Vector,
    ) -> Self::Vector {
        _mm_blendv_epi8(a, b, mask)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_8(&self, vector: Self::Vector) -> Self::Vector {
        _mm_slli_si128(vector, 8)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn set_all_bytes(&self, value: u8) -> Self::Vector {
        _mm_set1_epi8(value as i8)
    }

    #[inline(always)]
    unsafe fn create_compare_mask(&self, vector: Self::Vector) -> Self::Vector {
        // On x86, MSB is already used for blending, so we just return the vector
        vector
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn and_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        _mm_and_si128(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_32(&self, vector: Self::Vector) -> Self::Vector {
        _mm_srli_si128(vector, 4)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_32(&self, vector: Self::Vector) -> Self::Vector {
        _mm_slli_si128(vector, 4)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn create_vector_from_u32(&self, value: u32, high: bool) -> Self::Vector {
        if high {
            _mm_insert_epi32(_mm_set1_epi32(0), value as i32, 3)
        } else {
            _mm_set_epi32(0, 0, 0, value as i32)
        }
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_4(&self, vector: Self::Vector) -> Self::Vector {
        _mm_slli_si128(vector, 4)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_4(&self, vector: Self::Vector) -> Self::Vector {
        _mm_srli_si128(vector, 4)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_8(&self, vector: Self::Vector) -> Self::Vector {
        _mm_srli_si128(vector, 8)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_5(&self, vector: Self::Vector) -> Self::Vector {
        _mm_srli_si128(vector, 5)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_6(&self, vector: Self::Vector) -> Self::Vector {
        _mm_srli_si128(vector, 6)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_7(&self, vector: Self::Vector) -> Self::Vector {
        _mm_srli_si128(vector, 7)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_12(&self, vector: Self::Vector) -> Self::Vector {
        _mm_srli_si128(vector, 12)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_12(&self, vector: Self::Vector) -> Self::Vector {
        _mm_slli_si128(vector, 12)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_00(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        _mm_clmulepi64_si128(a, b, 0x00)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_01(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        _mm_clmulepi64_si128(a, b, 0x01)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_10(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        _mm_clmulepi64_si128(a, b, 0x10)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_11(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        _mm_clmulepi64_si128(a, b, 0x11)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn xor3_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        c: Self::Vector,
    ) -> Self::Vector {
        // SSE tier always uses two XOR operations
        _mm_xor_si128(_mm_xor_si128(a, b), c)
    }
}

impl X86SsePclmulqdqOps {
    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn set_epi64x(&self, e1: u64, e0: u64) -> __m128i {
        #[cfg(target_arch = "x86_64")]
        {
            _mm_set_epi64x(e1 as i64, e0 as i64)
        }

        #[cfg(target_arch = "x86")]
        {
            // _mm_set_epi32 takes (highest, higher, lower, lowest)
            // We need to ensure e0 is in the lower 64 bits and e1 in the higher 64 bits
            let lo = _mm_set_epi32(0, 0, (e0 >> 32) as i32, e0 as i32);
            let hi = _mm_set_epi32(0, 0, (e1 >> 32) as i32, e1 as i32);

            _mm_unpacklo_epi64(lo, hi)
        }
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn create_u64_vector(&self, value: u64, high: bool) -> __m128i {
        if high {
            self.set_epi64x(value, 0)
        } else {
            self.set_epi64x(0, value)
        }
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn extract_u64_low(&self, v: __m128i) -> u64 {
        #[cfg(target_arch = "x86_64")]
        {
            _mm_cvtsi128_si64(v) as u64
        }

        #[cfg(target_arch = "x86")]
        {
            let lo = _mm_cvtsi128_si32(v) as u32 as u64;
            let hi = _mm_cvtsi128_si32(_mm_srli_si128(v, 4)) as u32 as u64;
            lo | (hi << 32)
        }
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn extract_u64_high(&self, v: __m128i) -> u64 {
        #[cfg(target_arch = "x86_64")]
        {
            _mm_cvtsi128_si64(_mm_srli_si128(v, 8)) as u64
        }

        #[cfg(target_arch = "x86")]
        {
            let lo = _mm_cvtsi128_si32(_mm_srli_si128(v, 8)) as u32 as u64;
            let hi = _mm_cvtsi128_si32(_mm_srli_si128(v, 12)) as u32 as u64;
            lo | (hi << 32)
        }
    }
}
