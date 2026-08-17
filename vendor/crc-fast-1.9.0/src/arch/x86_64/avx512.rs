// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides x86_64-specific implementations of the ArchOps trait.
//!
//! It uses the AVX-512 instruction sets for SIMD operations.

#![cfg(target_arch = "x86_64")]

#[rustversion::since(1.89)]
use core::arch::x86_64::*;

#[rustversion::since(1.89)]
use crate::arch::x86::sse::X86SsePclmulqdqOps;
#[rustversion::since(1.89)]
use crate::traits::ArchOps;

/// x86_64-only AVX512+PCLMULQDQ tier - delegates to SSE tier and overrides XOR3 operations
/// Uses AVX512 ternary logic for XOR3 operations with PCLMULQDQ
#[rustversion::since(1.89)]
#[derive(Debug, Copy, Clone)]
pub struct X86_64Avx512PclmulqdqOps(X86SsePclmulqdqOps);

#[rustversion::since(1.89)]
impl X86_64Avx512PclmulqdqOps {
    #[inline(always)]
    pub fn new() -> Self {
        Self(X86SsePclmulqdqOps)
    }
}

#[rustversion::since(1.89)]
impl ArchOps for X86_64Avx512PclmulqdqOps {
    type Vector = __m128i;

    // Delegate all methods to the base SSE implementation
    #[inline(always)]
    unsafe fn create_vector_from_u64_pair(
        &self,
        high: u64,
        low: u64,
        reflected: bool,
    ) -> Self::Vector {
        self.0.create_vector_from_u64_pair(high, low, reflected)
    }

    #[inline(always)]
    unsafe fn create_vector_from_u64_pair_non_reflected(
        &self,
        high: u64,
        low: u64,
    ) -> Self::Vector {
        self.0.create_vector_from_u64_pair_non_reflected(high, low)
    }

    #[inline(always)]
    unsafe fn create_vector_from_u64(&self, value: u64, high: bool) -> Self::Vector {
        self.0.create_vector_from_u64(value, high)
    }

    #[inline(always)]
    unsafe fn extract_u64s(&self, vector: Self::Vector) -> [u64; 2] {
        self.0.extract_u64s(vector)
    }

    #[inline(always)]
    unsafe fn extract_poly64s(&self, vector: Self::Vector) -> [u64; 2] {
        self.0.extract_poly64s(vector)
    }

    #[inline(always)]
    unsafe fn xor_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.xor_vectors(a, b)
    }

    #[inline(always)]
    unsafe fn load_bytes(&self, ptr: *const u8) -> Self::Vector {
        self.0.load_bytes(ptr)
    }

    #[inline(always)]
    unsafe fn load_aligned(&self, ptr: *const [u64; 2]) -> Self::Vector {
        self.0.load_aligned(ptr)
    }

    #[inline(always)]
    unsafe fn shuffle_bytes(&self, data: Self::Vector, mask: Self::Vector) -> Self::Vector {
        self.0.shuffle_bytes(data, mask)
    }

    #[inline(always)]
    unsafe fn blend_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        mask: Self::Vector,
    ) -> Self::Vector {
        self.0.blend_vectors(a, b, mask)
    }

    #[inline(always)]
    unsafe fn shift_left_8(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_8(vector)
    }

    #[inline(always)]
    unsafe fn set_all_bytes(&self, value: u8) -> Self::Vector {
        self.0.set_all_bytes(value)
    }

    #[inline(always)]
    unsafe fn create_compare_mask(&self, vector: Self::Vector) -> Self::Vector {
        self.0.create_compare_mask(vector)
    }

    #[inline(always)]
    unsafe fn and_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.and_vectors(a, b)
    }

    #[inline(always)]
    unsafe fn shift_right_32(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_32(vector)
    }

    #[inline(always)]
    unsafe fn shift_left_32(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_32(vector)
    }

    #[inline(always)]
    unsafe fn create_vector_from_u32(&self, value: u32, high: bool) -> Self::Vector {
        self.0.create_vector_from_u32(value, high)
    }

    #[inline(always)]
    unsafe fn shift_left_4(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_4(vector)
    }

    #[inline(always)]
    unsafe fn shift_right_4(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_4(vector)
    }

    #[inline(always)]
    unsafe fn shift_right_8(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_8(vector)
    }

    #[inline(always)]
    unsafe fn shift_right_5(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_5(vector)
    }

    #[inline(always)]
    unsafe fn shift_right_6(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_6(vector)
    }

    #[inline(always)]
    unsafe fn shift_right_7(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_7(vector)
    }

    #[inline(always)]
    unsafe fn shift_right_12(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_12(vector)
    }

    #[inline(always)]
    unsafe fn shift_left_12(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_12(vector)
    }

    #[inline(always)]
    unsafe fn carryless_mul_00(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_00(a, b)
    }

    #[inline(always)]
    unsafe fn carryless_mul_01(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_01(a, b)
    }

    #[inline(always)]
    unsafe fn carryless_mul_10(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_10(a, b)
    }

    #[inline(always)]
    unsafe fn carryless_mul_11(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_11(a, b)
    }

    #[rustversion::since(1.89)]
    #[inline]
    #[target_feature(enable = "avx512vl")]
    unsafe fn xor3_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        c: Self::Vector,
    ) -> Self::Vector {
        // AVX512 tier always uses ternary logic
        _mm_ternarylogic_epi64(a, b, c, 0x96) // XOR3 operation
    }

    // Fallback for older Rust versions
    #[rustversion::before(1.89)]
    #[inline(always)]
    unsafe fn xor3_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        c: Self::Vector,
    ) -> Self::Vector {
        // Rust < 1.89 doesn't have _mm_ternarylogic_epi64, fall back to SSE
        self.0.xor3_vectors(a, b, c)
    }
}
