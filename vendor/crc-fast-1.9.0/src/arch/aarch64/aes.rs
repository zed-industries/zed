// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides AArch64-specific implementations of the ArchOps trait for architectures
//! with AES support.

#![cfg(target_arch = "aarch64")]

use crate::traits::ArchOps;
use core::arch::aarch64::*;

// Tier-specific ArchOps implementations for AArch64

/// Base AArch64 implementation with AES+NEON optimizations
/// NEON is implicit with AES support on AArch64
#[derive(Debug, Copy, Clone)]
pub struct Aarch64AesOps;

// Base implementation for AArch64 AES tier
impl ArchOps for Aarch64AesOps {
    type Vector = uint8x16_t;

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn create_vector_from_u64_pair(
        &self,
        high: u64,
        low: u64,
        reflected: bool,
    ) -> Self::Vector {
        // Note: AArch64 switches the order for reflected mode
        if reflected {
            self.load_key_pair(high, low)
        } else {
            self.load_key_pair(low, high)
        }
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn create_vector_from_u64_pair_non_reflected(
        &self,
        high: u64,
        low: u64,
    ) -> Self::Vector {
        self.load_key_pair(low, high)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn create_vector_from_u64(&self, value: u64, high: bool) -> Self::Vector {
        let mut result = vdupq_n_u64(0);
        if high {
            result = vsetq_lane_u64(value, result, 1); // Set high 64 bits
        } else {
            result = vsetq_lane_u64(value, result, 0); // Set low 64 bits
        }

        vreinterpretq_u8_u64(result)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn extract_u64s(&self, vector: Self::Vector) -> [u64; 2] {
        let x = vreinterpretq_u64_u8(vector);
        [vgetq_lane_u64(x, 0), vgetq_lane_u64(x, 1)]
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn extract_poly64s(&self, vector: Self::Vector) -> [u64; 2] {
        let x = vreinterpretq_p64_u8(vector);
        [vgetq_lane_p64(x, 0), vgetq_lane_p64(x, 1)]
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn xor_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        veorq_u8(a, b)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn load_bytes(&self, ptr: *const u8) -> Self::Vector {
        vld1q_u8(ptr)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn load_aligned(&self, ptr: *const [u64; 2]) -> Self::Vector {
        vreinterpretq_u8_u64(vld1q_u64(ptr as *const u64))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shuffle_bytes(&self, data: Self::Vector, mask: Self::Vector) -> Self::Vector {
        vqtbl1q_u8(data, mask)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn blend_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        mask: Self::Vector,
    ) -> Self::Vector {
        let msb_mask = vcltq_s8(vreinterpretq_s8_u8(mask), vdupq_n_s8(0));
        vbslq_u8(msb_mask, b, a)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_left_8(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vdupq_n_u8(0), vector, 8)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn set_all_bytes(&self, value: u8) -> Self::Vector {
        vdupq_n_u8(value)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn create_compare_mask(&self, vector: Self::Vector) -> Self::Vector {
        vcltq_s8(vreinterpretq_s8_u8(vector), vdupq_n_s8(0))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn and_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        vandq_u8(a, b)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_right_32(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vector, vdupq_n_u8(0), 4)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_left_32(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vdupq_n_u8(0), vector, 12) // 16-4=12
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn create_vector_from_u32(&self, value: u32, high: bool) -> Self::Vector {
        let mut result = vdupq_n_u64(0);
        if high {
            // For high=true, place in the high 32 bits of the high 64 bits
            result = vreinterpretq_u64_u32(vsetq_lane_u32(value, vreinterpretq_u32_u64(result), 3));
        } else {
            // For high=false, place in the low 32 bits of the low 64 bits
            result = vreinterpretq_u64_u32(vsetq_lane_u32(value, vreinterpretq_u32_u64(result), 0));
        }
        vreinterpretq_u8_u64(result)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_left_4(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vdupq_n_u8(0), vector, 12) // 16-4=12
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_right_4(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vector, vdupq_n_u8(0), 4)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_right_8(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vector, vdupq_n_u8(0), 8)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_right_5(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vector, vdupq_n_u8(0), 5)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_right_6(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vector, vdupq_n_u8(0), 6)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_right_7(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vector, vdupq_n_u8(0), 7)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_right_12(&self, vector: Self::Vector) -> Self::Vector {
        vextq_u8(vector, vdupq_n_u8(0), 12)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shift_left_12(&self, vector: Self::Vector) -> Self::Vector {
        let low_32 = vgetq_lane_u32(vreinterpretq_u32_u8(vector), 0);
        let result = vsetq_lane_u32(low_32, vdupq_n_u32(0), 3);
        vreinterpretq_u8_u32(result)
    }

    #[inline]
    #[target_feature(enable = "aes")]
    unsafe fn carryless_mul_00(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        vreinterpretq_u8_p128(vmull_p64(
            vgetq_lane_p64(vreinterpretq_p64_u8(a), 0),
            vgetq_lane_p64(vreinterpretq_p64_u8(b), 0),
        ))
    }

    #[inline]
    #[target_feature(enable = "aes")]
    unsafe fn carryless_mul_01(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        let a_low = vgetq_lane_p64(vreinterpretq_p64_u8(a), 1);
        let b_high = vgetq_lane_p64(vreinterpretq_p64_u8(b), 0);
        vreinterpretq_u8_p128(vmull_p64(a_low, b_high))
    }

    #[inline]
    #[target_feature(enable = "aes")]
    unsafe fn carryless_mul_10(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        vreinterpretq_u8_p128(vmull_p64(
            vgetq_lane_p64(vreinterpretq_p64_u8(a), 0),
            vgetq_lane_p64(vreinterpretq_p64_u8(b), 1),
        ))
    }

    #[inline]
    #[target_feature(enable = "aes")]
    unsafe fn carryless_mul_11(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        vreinterpretq_u8_p128(vmull_p64(
            vgetq_lane_p64(vreinterpretq_p64_u8(a), 1),
            vgetq_lane_p64(vreinterpretq_p64_u8(b), 1),
        ))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn xor3_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        c: Self::Vector,
    ) -> Self::Vector {
        veorq_u8(veorq_u8(a, b), c)
    }
}

impl Aarch64AesOps {
    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn load_key_pair(&self, idx1: u64, idx2: u64) -> uint8x16_t {
        vreinterpretq_u8_u64(vld1q_u64([idx1, idx2].as_ptr()))
    }
}
