// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides AVX-512 and VPCLMULQDQ-specific implementations of the ArchOps trait.
//!
//! It performs folding using 4 x ZMM registers of 512-bits each.

#![cfg(target_arch = "x86_64")]

#[rustversion::since(1.89)]
use crate::arch::x86::sse::X86SsePclmulqdqOps;

#[rustversion::since(1.89)]
use crate::enums::Reflector;

#[rustversion::since(1.89)]
use crate::structs::CrcState;

#[rustversion::since(1.89)]
use crate::traits::{ArchOps, EnhancedCrcWidth};

#[rustversion::since(1.89)]
use core::arch::x86_64::*;

#[rustversion::since(1.89)]
use core::ops::BitXor;

/// Implements the ArchOps trait using 512-bit AVX-512 and VPCLMULQDQ instructions at 512 bits.
/// Delegates to X86SsePclmulqdqOps for standard 128-bit operations
#[rustversion::since(1.89)]
#[derive(Debug, Copy, Clone)]
pub struct X86_64Avx512VpclmulqdqOps(X86SsePclmulqdqOps);

#[rustversion::since(1.89)]
impl X86_64Avx512VpclmulqdqOps {
    #[inline(always)]
    pub fn new() -> Self {
        Self(X86SsePclmulqdqOps)
    }
}

// Wrapper for __m512i to make it easier to work with
#[rustversion::since(1.89)]
#[derive(Debug, Copy, Clone)]
struct Simd512(__m512i);

#[rustversion::since(1.89)]
impl Simd512 {
    #[inline]
    #[target_feature(enable = "avx512f")]
    #[allow(clippy::too_many_arguments)]
    unsafe fn new(x7: u64, x6: u64, x5: u64, x4: u64, x3: u64, x2: u64, x1: u64, x0: u64) -> Self {
        Self(_mm512_set_epi64(
            x7 as i64, x6 as i64, x5 as i64, x4 as i64, x3 as i64, x2 as i64, x1 as i64, x0 as i64,
        ))
    }

    #[inline]
    #[target_feature(enable = "avx512vl,vpclmulqdq")]
    unsafe fn fold_64(&self, coeff: &Self, new_data: &Self) -> Self {
        // Use 512-bit ternary logic XOR3 with carryless multiplication
        Self(_mm512_ternarylogic_epi64(
            _mm512_clmulepi64_epi128(self.0, coeff.0, 0), // Low parts
            _mm512_clmulepi64_epi128(self.0, coeff.0, 17), // High parts
            new_data.0,
            0x96, // XOR3 operation
        ))
    }

    #[cfg(feature = "std")]
    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn extract_u64s(&self) -> [u64; 8] {
        let mut result = [0u64; 8];
        _mm512_storeu_si512(result.as_mut_ptr().cast(), self.0);

        result
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn load_from_ptr(ptr: *const u8) -> Self {
        Self(_mm512_loadu_si512(ptr as *const __m512i))
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn to_128i_extract<const INDEX: i32>(self) -> __m128i {
        _mm512_extracti32x4_epi32(self.0, INDEX)
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn xor(&self, other: &Self) -> Self {
        Self(_mm512_xor_si512(self.0, other.0))
    }

    #[cfg(feature = "std")]
    #[inline]
    #[target_feature(enable = "avx512f")]
    #[allow(unused)]
    unsafe fn print_hex(&self, prefix: &str) {
        let values = self.extract_u64s();
        println!(
            "{}={:#016x}_{:016x}_{:016x}_{:016x}_{:016x}_{:016x}_{:016x}_{:016x}",
            prefix,
            values[7],
            values[6],
            values[5],
            values[4],
            values[3],
            values[2],
            values[1],
            values[0]
        );
    }
}

#[rustversion::since(1.89)]
impl X86_64Avx512VpclmulqdqOps {
    /// Process aligned blocks using VPCLMULQDQ with 4 x 512-bit registers
    ///
    /// Note that #[inline(always)] loses the inlining performance boost, despite no native
    /// target_features being used directly. Odd since that's not how Rust's docs make it sound...
    #[inline]
    #[target_feature(enable = "avx512vl,avx512bw,vpclmulqdq")]
    unsafe fn process_blocks<W: EnhancedCrcWidth>(
        &self,
        state: &mut CrcState<<X86_64Avx512VpclmulqdqOps as ArchOps>::Vector>,
        first: &[__m128i; 8],
        rest: &[[__m128i; 8]],
        keys: &[u64; 23],
        reflected: bool,
    ) -> W::Value
    where
        W::Value: Copy + BitXor<Output = W::Value>,
    {
        let state_u64s = self.extract_u64s(state.value);

        let positioned_state = if reflected {
            Simd512::new(0, 0, 0, 0, 0, 0, 0, state_u64s[0])
        } else {
            Simd512::new(state_u64s[1], 0, 0, 0, 0, 0, 0, 0)
        };

        let reflector = create_reflector512(reflected);

        // Load first 256 bytes (2nd half is rest[0] since these are 128-byte blocks)
        let first_ptr = first.as_ptr() as *const u8;
        let first_rest_ptr = rest[0].as_ptr() as *const u8;

        let mut x = [
            reflect_bytes512(&reflector, Simd512::load_from_ptr(first_ptr)),
            reflect_bytes512(&reflector, Simd512::load_from_ptr(first_ptr.add(64))),
            reflect_bytes512(&reflector, Simd512::load_from_ptr(first_rest_ptr)),
            reflect_bytes512(&reflector, Simd512::load_from_ptr(first_rest_ptr.add(64))),
        ];

        x[0] = positioned_state.xor(&x[0]);

        let coeff = self.create_avx512_256byte_coefficient(keys, reflected);

        let remaining_rest = &rest[1..];
        let pair_count = remaining_rest.len() / 2;

        for i in 0..pair_count {
            let block1_ptr = remaining_rest[i * 2].as_ptr() as *const u8;
            let block2_ptr = remaining_rest[i * 2 + 1].as_ptr() as *const u8;

            x[0] = x[0].fold_64(
                &coeff,
                &reflect_bytes512(&reflector, Simd512::load_from_ptr(block1_ptr)),
            );
            x[1] = x[1].fold_64(
                &coeff,
                &reflect_bytes512(&reflector, Simd512::load_from_ptr(block1_ptr.add(64))),
            );
            x[2] = x[2].fold_64(
                &coeff,
                &reflect_bytes512(&reflector, Simd512::load_from_ptr(block2_ptr)),
            );
            x[3] = x[3].fold_64(
                &coeff,
                &reflect_bytes512(&reflector, Simd512::load_from_ptr(block2_ptr.add(64))),
            );
        }

        let processed_pairs = pair_count * 2;
        let remaining_single_count = remaining_rest.len() - processed_pairs;

        if remaining_single_count > 0 {
            // We have 1 unprocessed block (128 bytes)
            // Fold 4×512 down to 2×512 and process the remaining block with 2-register mode
            let folded_2reg = self.fold_from_4x512_to_2x256(x, keys, reflected);
            let coeff_2reg = self.create_avx512_128byte_coefficient(keys, reflected);

            let last_block_ptr = remaining_rest[processed_pairs].as_ptr() as *const u8;

            let final_x = [
                folded_2reg[0].fold_64(
                    &coeff_2reg,
                    &reflect_bytes512(&reflector, Simd512::load_from_ptr(last_block_ptr)),
                ),
                folded_2reg[1].fold_64(
                    &coeff_2reg,
                    &reflect_bytes512(&reflector, Simd512::load_from_ptr(last_block_ptr.add(64))),
                ),
            ];

            let folded = self.fold_from_2x512_to_1x128(final_x, keys, reflected);

            return W::perform_final_reduction(folded, reflected, keys, self);
        }

        // All blocks processed in pairs - fold from 4 x 512-bit to 1 x 128-bit
        let folded = self.fold_from_4x512_to_1x128(x, keys, reflected);

        W::perform_final_reduction(folded, reflected, keys, self)
    }

    /// Create a folding coefficient for AVX-512 for 128-byte folding distances
    #[inline(always)]
    unsafe fn create_avx512_128byte_coefficient(
        &self,
        keys: &[u64; 23],
        reflected: bool,
    ) -> Simd512 {
        let (k1, k2) = if reflected {
            (keys[3], keys[4])
        } else {
            (keys[4], keys[3])
        };

        // Replicate the coefficient pair
        Simd512::new(k1, k2, k1, k2, k1, k2, k1, k2)
    }

    /// Create a folding coefficient for AVX-512 for 256-byte folding distances
    #[inline(always)]
    unsafe fn create_avx512_256byte_coefficient(
        &self,
        keys: &[u64; 23],
        reflected: bool,
    ) -> Simd512 {
        let (k1, k2) = if reflected {
            (keys[21], keys[22])
        } else {
            (keys[22], keys[21])
        };

        // Replicate the coefficient pair
        Simd512::new(k1, k2, k1, k2, k1, k2, k1, k2)
    }

    /// Fold from 4 x 512-bit to 1 x 128-bit
    #[inline(always)]
    unsafe fn fold_from_4x512_to_1x128(
        &self,
        x: [Simd512; 4],
        keys: &[u64; 23],
        reflected: bool,
    ) -> __m128i {
        // Step 1: Fold 4 x 512-bit to 2 x 512-bit
        let x2 = self.fold_from_4x512_to_2x256(x, keys, reflected);

        // Step 2: Fold 2 x 512-bit to 1 x 128-bit
        self.fold_from_2x512_to_1x128(x2, keys, reflected)
    }

    /// Fold from 4 x 512-bit to 2 x 512-bit
    #[inline(always)]
    unsafe fn fold_from_4x512_to_2x256(
        &self,
        x: [Simd512; 4],
        keys: &[u64; 23],
        reflected: bool,
    ) -> [Simd512; 2] {
        // This folds registers that are 128 bytes apart (x[0] with x[2], x[1] with x[3])
        let coeff = self.create_avx512_128byte_coefficient(keys, reflected);

        // Fold pairs:
        // x[0] (bytes 0-63) + x[2] (bytes 128-191) → result[0]
        // x[1] (bytes 64-127) + x[3] (bytes 192-255) → result[1]
        [x[0].fold_64(&coeff, &x[2]), x[1].fold_64(&coeff, &x[3])]
    }

    /// Fold from 2 x 512-bit to 1 x 128-bit
    #[inline(always)]
    unsafe fn fold_from_2x512_to_1x128(
        &self,
        x: [Simd512; 2],
        keys: &[u64; 23],
        reflected: bool,
    ) -> __m128i {
        // Create the fold coefficients for different distances
        let fold_coefficients = [
            self.create_vector_from_u64_pair(keys[10], keys[9], reflected), // 112 bytes
            self.create_vector_from_u64_pair(keys[12], keys[11], reflected), // 96 bytes
            self.create_vector_from_u64_pair(keys[14], keys[13], reflected), // 80 bytes
            self.create_vector_from_u64_pair(keys[16], keys[15], reflected), // 64 bytes
            self.create_vector_from_u64_pair(keys[18], keys[17], reflected), // 48 bytes
            self.create_vector_from_u64_pair(keys[20], keys[19], reflected), // 32 bytes
            self.create_vector_from_u64_pair(keys[2], keys[1], reflected),  // 16 bytes
        ];

        // Extract the 8 x 128-bit vectors from the 2 x 512-bit vectors (this is faster than
        // using 256-bit intrinsics for 1KiB payloads)
        let v128 = if reflected {
            [
                x[0].to_128i_extract::<0>(), // 256-x0.low
                x[0].to_128i_extract::<1>(), // 256-x0.high
                x[0].to_128i_extract::<2>(), // 256-x1.low
                x[0].to_128i_extract::<3>(), // 256-x1.high
                x[1].to_128i_extract::<0>(), // 256-x2.low
                x[1].to_128i_extract::<1>(), // 256-x2.high
                x[1].to_128i_extract::<2>(), // 256-x3.low
                x[1].to_128i_extract::<3>(), // 256-x3.high
            ]
        } else {
            [
                x[0].to_128i_extract::<3>(), // 256-x1.high
                x[0].to_128i_extract::<2>(), // 256-x1.low
                x[0].to_128i_extract::<1>(), // 256-x0.high
                x[0].to_128i_extract::<0>(), // 256-x0.low
                x[1].to_128i_extract::<3>(), // 256-x3.high
                x[1].to_128i_extract::<2>(), // 256-x3.low
                x[1].to_128i_extract::<1>(), // 256-x2.high
                x[1].to_128i_extract::<0>(), // 256-x2.low
            ]
        };

        // Fold the 8 xmm registers to 1 xmm register
        let mut res = v128[7];

        for (i, &coeff) in fold_coefficients.iter().enumerate() {
            let folded_h = self.carryless_mul_00(v128[i], coeff);
            let folded_l = self.carryless_mul_11(v128[i], coeff);
            res = self.xor3_vectors(folded_h, folded_l, res);
        }

        res
    }
}

// 512-bit version of the Reflector
#[rustversion::since(1.89)]
#[derive(Clone, Copy)]
enum Reflector512 {
    NoReflector,
    ForwardReflector { smask: Simd512 },
}

// Function to create the appropriate reflector based on CRC parameters
#[rustversion::since(1.89)]
#[inline(always)]
unsafe fn create_reflector512(reflected: bool) -> Reflector512 {
    if reflected {
        Reflector512::NoReflector
    } else {
        // Load shuffle mask
        let smask = Simd512::new(
            0x08090a0b0c0d0e0f,
            0x0001020304050607,
            0x08090a0b0c0d0e0f,
            0x0001020304050607,
            0x08090a0b0c0d0e0f,
            0x0001020304050607,
            0x08090a0b0c0d0e0f,
            0x0001020304050607,
        );
        Reflector512::ForwardReflector { smask }
    }
}

// Function to apply reflection to a 512-bit vector
#[rustversion::since(1.89)]
#[inline(always)]
unsafe fn reflect_bytes512(reflector: &Reflector512, data: Simd512) -> Simd512 {
    match reflector {
        Reflector512::NoReflector => data,
        Reflector512::ForwardReflector { smask } => shuffle_bytes512(data, *smask),
    }
}

// pre-compute the reverse indices for 512-bit shuffling
#[rustversion::since(1.89)]
static REVERSE_INDICES_512: __m512i =
    unsafe { core::mem::transmute([7u64, 6u64, 5u64, 4u64, 3u64, 2u64, 1u64, 0u64]) };

// Implement a 512-bit byte shuffle function
#[rustversion::since(1.89)]
#[inline]
#[target_feature(enable = "avx512f,avx512bw")]
unsafe fn shuffle_bytes512(data: Simd512, mask: Simd512) -> Simd512 {
    Simd512(_mm512_permutexvar_epi64(
        // Reverse the order using 512-bit permutation
        REVERSE_INDICES_512,                 // reverse indices
        _mm512_shuffle_epi8(data.0, mask.0), // shuffled data
    ))
}

// Delegate all ArchOps methods to the inner X86SsePclmulqdqOps instance
#[rustversion::since(1.89)]
impl ArchOps for X86_64Avx512VpclmulqdqOps {
    type Vector = __m128i;

    #[inline(always)]
    unsafe fn process_enhanced_simd_blocks<W: EnhancedCrcWidth>(
        &self,
        state: &mut CrcState<Self::Vector>,
        first: &[Self::Vector; 8],
        rest: &[[Self::Vector; 8]],
        _reflector: &Reflector<Self::Vector>,
        keys: &[u64; 23],
    ) -> bool
    where
        Self::Vector: Copy,
    {
        // Update the state with the result
        *state = W::create_state(
            self.process_blocks::<W>(state, first, rest, keys, state.reflected),
            state.reflected,
            self,
        );

        // Return true to indicate we handled it
        true
    }

    // Delegate all other methods to X86SsePclmulqdqOps
    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn create_vector_from_u64_pair(
        &self,
        high: u64,
        low: u64,
        reflected: bool,
    ) -> Self::Vector {
        self.0.create_vector_from_u64_pair(high, low, reflected)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn create_vector_from_u64_pair_non_reflected(
        &self,
        high: u64,
        low: u64,
    ) -> Self::Vector {
        self.0.create_vector_from_u64_pair_non_reflected(high, low)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn create_vector_from_u64(&self, value: u64, high: bool) -> Self::Vector {
        self.0.create_vector_from_u64(value, high)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn extract_u64s(&self, vector: Self::Vector) -> [u64; 2] {
        self.0.extract_u64s(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn extract_poly64s(&self, vector: Self::Vector) -> [u64; 2] {
        self.0.extract_poly64s(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn xor_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.xor_vectors(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn load_bytes(&self, ptr: *const u8) -> Self::Vector {
        self.0.load_bytes(ptr)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn load_aligned(&self, ptr: *const [u64; 2]) -> Self::Vector {
        self.0.load_aligned(ptr)
    }

    #[inline]
    #[target_feature(enable = "ssse3")]
    unsafe fn shuffle_bytes(&self, data: Self::Vector, mask: Self::Vector) -> Self::Vector {
        self.0.shuffle_bytes(data, mask)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn blend_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        mask: Self::Vector,
    ) -> Self::Vector {
        self.0.blend_vectors(a, b, mask)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_8(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_8(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn set_all_bytes(&self, value: u8) -> Self::Vector {
        self.0.set_all_bytes(value)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn create_compare_mask(&self, vector: Self::Vector) -> Self::Vector {
        self.0.create_compare_mask(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn and_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.and_vectors(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_32(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_32(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_32(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_32(vector)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn create_vector_from_u32(&self, value: u32, high: bool) -> Self::Vector {
        self.0.create_vector_from_u32(value, high)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_4(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_4(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_4(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_4(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_8(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_8(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_5(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_5(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_6(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_6(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_7(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_7(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_12(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_12(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_12(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_12(vector)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_00(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_00(a, b)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_01(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_01(a, b)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_10(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_10(a, b)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_11(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_11(a, b)
    }

    #[inline]
    #[target_feature(enable = "avx512vl")]
    unsafe fn xor3_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        c: Self::Vector,
    ) -> Self::Vector {
        // VPCLMULQDQ implementation always uses AVX512 ternary logic
        // Feature detection is handled at the dispatch level
        _mm_ternarylogic_epi64(
            a, b, c, 0x96, // XOR3 operation
        )
    }
}
