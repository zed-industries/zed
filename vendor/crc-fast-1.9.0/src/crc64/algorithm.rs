// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides the CRC-64 implementation for areas where it differs from CRC-32.

#![cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]

use crate::algorithm;
use crate::consts::{CRC_CHUNK_SIZE, CRC_HALF_CHUNK_SIZE};
use crate::crc64::consts::SIMD_CONSTANTS;
use crate::enums::Reflector;
use crate::structs::CrcState;
use crate::traits::{ArchOps, EnhancedCrcWidth};

impl EnhancedCrcWidth for crate::structs::Width64 {
    #[inline(always)]
    fn load_constants(_reflected: bool) -> [[u64; 2]; 4] {
        [
            [0x08090a0b0c0d0e0f, 0x0001020304050607], // smask
            [0x8080808080808080, 0x8080808080808080], // mask1 (mask3 reflected)
            [0xffffffffffffffff, 0x00000000ffffffff], // mask2
            [0x0000000000000000, 0xffffffffffffffff], // mask3 (forward)
        ]
    }

    #[inline(always)]
    unsafe fn create_state<T: ArchOps>(
        value: Self::Value,
        reflected: bool,
        ops: &T,
    ) -> CrcState<T::Vector>
    where
        T::Vector: Copy,
    {
        let vector = if reflected {
            ops.create_vector_from_u64(value, false) // Set low 64 bits
        } else {
            ops.create_vector_from_u64(value, true) // Set high 64 bits
        };

        CrcState {
            value: vector,
            reflected,
        }
    }

    #[inline(always)]
    unsafe fn extract_result<T: ArchOps>(vector: T::Vector, reflected: bool, ops: &T) -> Self::Value
    where
        T::Vector: Copy,
    {
        let u64s = ops.extract_u64s(vector);
        if reflected {
            u64s[0] // Low 64 bits for reflected mode
        } else {
            u64s[1] // High 64 bits for non-reflected mode
        }
    }

    #[inline(always)]
    unsafe fn fold_16<T: ArchOps>(
        state: &mut CrcState<T::Vector>,
        coeff: T::Vector,
        data_to_xor: T::Vector,
        ops: &T,
    ) where
        T::Vector: Copy,
    {
        // CRC-64 specific implementation for folding 16 bytes
        state.value = {
            ops.xor3_vectors(
                ops.carryless_mul_00(state.value, coeff),
                ops.carryless_mul_11(state.value, coeff),
                data_to_xor,
            )
        };
    }

    #[inline(always)]
    unsafe fn fold_width<T: ArchOps>(state: &mut CrcState<T::Vector>, high: u64, low: u64, ops: &T)
    where
        T::Vector: Copy,
    {
        // CRC-64 specific implementation for folding 8 bytes
        let coeff = Self::create_coefficient(high, low, state.reflected, ops);

        if state.reflected {
            let h = ops.carryless_mul_01(coeff, state.value);
            let shifted = ops.shift_right_8(state.value);

            state.value = ops.xor_vectors(h, shifted);
        } else {
            let clmul = ops.carryless_mul_01(state.value, coeff);
            let shifted = ops.shift_left_8(state.value);

            state.value = ops.xor_vectors(clmul, shifted);
        }
    }

    #[inline(always)]
    unsafe fn barrett_reduction<T: ArchOps>(
        state: &CrcState<T::Vector>,
        poly: u64,
        mu: u64,
        ops: &T,
    ) -> Self::Value
    where
        T::Vector: Copy,
    {
        // CRC-64 Barrett reduction
        let x = state.value;
        let mu_poly = Self::create_coefficient(mu, poly, state.reflected, ops);

        if state.reflected {
            let clmul1 = ops.carryless_mul_00(x, mu_poly);
            let clmul2 = ops.carryless_mul_10(clmul1, mu_poly);
            let clmul1_shifted = ops.shift_left_8(clmul1);
            let final_xor = ops.xor3_vectors(clmul2, clmul1_shifted, x);

            ops.extract_u64s(final_xor)[1]
        } else {
            // Load mask3 for non-reflected mode
            let mask3 = ops.load_aligned(&[0x0000000000000000, 0xffffffffffffffff]);
            let x1 = ops.and_vectors(x, mask3);
            let clmul1 = ops.carryless_mul_11(x1, mu_poly);
            let clmul2 = ops.carryless_mul_01(ops.xor_vectors(clmul1, x1), mu_poly);
            let final_xor = ops.xor_vectors(clmul2, x);

            ops.extract_u64s(final_xor)[0]
        }
    }

    #[inline(always)]
    unsafe fn create_coefficient<T: ArchOps>(
        high: u64,
        low: u64,
        reflected: bool,
        ops: &T,
    ) -> T::Vector
    where
        T::Vector: Copy,
    {
        ops.create_vector_from_u64_pair(high, low, reflected)
    }

    #[inline(always)]
    unsafe fn perform_final_reduction<T: ArchOps>(
        state: T::Vector,
        reflected: bool,
        keys: &[u64; 23],
        ops: &T,
    ) -> Self::Value
    where
        T::Vector: Copy,
    {
        let mut crc_state = CrcState {
            value: state,
            reflected,
        };

        // Fold 16 bytes into 8 bytes
        Self::fold_width(&mut crc_state, keys[6], keys[5], ops);

        // Perform Barrett reduction to finalize
        Self::barrett_reduction(&crc_state, keys[8], keys[7], ops)
    }

    #[inline(always)]
    fn get_last_bytes_table_ptr(reflected: bool, remaining_len: usize) -> (*const u8, usize) {
        use crate::crc64::consts::{PSBTBL_FORWARD, PSBTBL_REVERSE};

        if reflected {
            // For reflected mode
            let base_ptr = &PSBTBL_REVERSE as *const _ as *const u8;
            let offset = if remaining_len <= CRC_CHUNK_SIZE {
                remaining_len
            } else {
                let real_remaining = remaining_len % CRC_CHUNK_SIZE;
                if real_remaining == 0 {
                    0
                } else {
                    real_remaining
                }
            };

            (base_ptr, offset)
        } else {
            // For non-reflected mode
            let base_ptr = &PSBTBL_FORWARD as *const _ as *const u8;
            let offset = if remaining_len <= CRC_CHUNK_SIZE {
                CRC_CHUNK_SIZE - remaining_len
            } else {
                let real_remaining = remaining_len % CRC_CHUNK_SIZE;
                if real_remaining == 0 {
                    0
                } else {
                    CRC_CHUNK_SIZE - real_remaining
                }
            };

            (base_ptr, offset)
        }
    }
}

/// Process inputs smaller than 16 bytes
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
pub(crate) unsafe fn process_0_to_15<T: ArchOps, W: EnhancedCrcWidth>(
    data: &[u8],
    state: &mut CrcState<T::Vector>,
    reflector: &Reflector<T::Vector>,
    keys: &[u64; 23],
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    // Create a zero-initialized aligned buffer
    let mut buffer = [0u8; CRC_CHUNK_SIZE];

    // Copy input data into the buffer
    buffer[..data.len()].copy_from_slice(data);

    // Get the length for processing
    let len = data.len() as i32;

    // Process the buffer
    let xmm7 = if state.reflected {
        // Get base pointer and standard offset from the trait method
        let (base_ptr, standard_offset) = W::get_last_bytes_table_ptr(true, len as usize);

        // Apply special adjustment for 1-7 bytes
        let ptr = if len < CRC_HALF_CHUNK_SIZE as i32 {
            // For 1-7 bytes: Add CRC_HALF_CHUNK_SIZE (8) to the offset
            base_ptr.add(standard_offset + CRC_HALF_CHUNK_SIZE)
        } else {
            // For 8-15 bytes: Use the standard offset directly
            base_ptr.add(standard_offset)
        };

        // Load shuffle mask
        let xmm0 = ops.load_bytes(ptr);

        // Apply shuffle to data XORed with initial state
        let data_with_crc = ops.xor_vectors(ops.load_bytes(buffer.as_ptr()), state.value);

        ops.shuffle_bytes(data_with_crc, xmm0)
    } else {
        // Calculate pointer offset for forward table
        // For non-reflected mode, we need to apply a special base offset
        let base_offset = if len < CRC_HALF_CHUNK_SIZE as i32 {
            24
        } else {
            16
        };

        // Use the trait method to get the base pointer and standard offset
        let (base_ptr, _) = W::get_last_bytes_table_ptr(false, len as usize);

        // Instead of using standard_offset directly, we use our special base_offset
        // This is the key difference that makes the code work correctly
        let ptr = base_ptr.add(base_offset - len as usize);

        // Load and reflect data
        let reflected_data =
            algorithm::reflect_bytes(reflector, ops.load_bytes(buffer.as_ptr()), ops);
        let data_with_crc = ops.xor_vectors(reflected_data, state.value);

        // Load shuffle mask
        let x0 = ops.load_bytes(ptr);

        if len >= CRC_HALF_CHUNK_SIZE as i32 {
            // For lengths 8-15, XOR with mask1
            let mask1 = ops.load_aligned(&SIMD_CONSTANTS[1]);
            ops.shuffle_bytes(data_with_crc, ops.xor_vectors(x0, mask1))
        } else {
            // For lengths 1-7, just return shuffled data
            ops.shuffle_bytes(data_with_crc, x0)
        }
    };

    if len >= CRC_HALF_CHUNK_SIZE as i32 {
        // For 8-15 bytes, perform additional folding
        return W::perform_final_reduction(xmm7, state.reflected, keys, ops);
    }

    let final_state = CrcState {
        value: xmm7,
        reflected: state.reflected,
    };

    // Barrett reduction to finalize
    W::barrett_reduction(&final_state, keys[8], keys[7], ops)
}
