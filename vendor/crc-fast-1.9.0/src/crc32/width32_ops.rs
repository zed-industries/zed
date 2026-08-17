// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! Shared operations for CRC widths that operate in 32-bit space (CRC-16, CRC-32).
//!
//! CRC-16 computation scales 16-bit values to 32-bit space, uses these operations,
//! then scales results back to 16 bits. CRC-32 uses these operations directly.
//!
//! This module eliminates code duplication between the CRC-16 and CRC-32 implementations,
//! which share nearly identical SIMD operations for folding, Barrett reduction, and
//! small input processing.

#![cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]

use crate::algorithm;
use crate::consts::CRC_CHUNK_SIZE;
use crate::crc32::consts::{PSHUFB_SHF_TABLE_FORWARD, PSHUFB_SHF_TABLE_REVERSE, SIMD_CONSTANTS};
use crate::enums::Reflector;
use crate::structs::CrcState;
use crate::traits::{ArchOps, EnhancedCrcWidth};

/// SIMD constants for reflected 32-bit-space CRC operations
pub const WIDTH32_CONSTANTS_REFLECTED: [[u64; 2]; 4] = [
    [0x08090a0b0c0d0e0f, 0x0001020304050607], // smask
    [0x8080808080808080, 0x8080808080808080], // mask1
    [0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF], // mask2 reverse
    [0x0000000000000000, 0x0000000000000000], // unused
];

/// SIMD constants for forward (non-reflected) 32-bit-space CRC operations
pub const WIDTH32_CONSTANTS_FORWARD: [[u64; 2]; 4] = [
    [0x08090a0b0c0d0e0f, 0x0001020304050607], // smask
    [0x8080808080808080, 0x8080808080808080], // mask1
    [0xffffffffffffffff, 0x00000000ffffffff], // mask2 forward
    [0x0000000000000000, 0x0000000000000000], // unused
];

/// Load SIMD constants for 32-bit-space CRC operations
#[inline(always)]
pub fn load_constants(reflected: bool) -> [[u64; 2]; 4] {
    if reflected {
        WIDTH32_CONSTANTS_REFLECTED
    } else {
        WIDTH32_CONSTANTS_FORWARD
    }
}

/// Fold 16 bytes for 32-bit-space CRC operations (used by CRC-16 and CRC-32)
#[inline(always)]
pub unsafe fn fold_16<T: ArchOps>(
    state: &mut CrcState<T::Vector>,
    coeff: T::Vector,
    data_to_xor: T::Vector,
    ops: &T,
) where
    T::Vector: Copy,
{
    let (h, l) = if state.reflected {
        (
            ops.carryless_mul_10(state.value, coeff),
            ops.carryless_mul_01(state.value, coeff),
        )
    } else {
        (
            ops.carryless_mul_00(state.value, coeff),
            ops.carryless_mul_11(state.value, coeff),
        )
    };
    state.value = ops.xor3_vectors(h, l, data_to_xor);
}

/// Fold to width for 32-bit-space CRC operations (used by CRC-16 and CRC-32)
#[inline(always)]
pub unsafe fn fold_width<T: ArchOps>(state: &mut CrcState<T::Vector>, high: u64, low: u64, ops: &T)
where
    T::Vector: Copy,
{
    let coeff_vector_low = ops.create_vector_from_u64_pair_non_reflected(0, low);
    let coeff_vector_high = ops.create_vector_from_u64_pair_non_reflected(high, 0);

    state.value = if state.reflected {
        ops.xor_vectors(
            ops.carryless_mul_00(state.value, coeff_vector_low),
            ops.shift_right_8(state.value),
        )
    } else {
        ops.xor_vectors(
            ops.carryless_mul_01(state.value, coeff_vector_low),
            ops.shift_left_8(state.value),
        )
    };

    let (clmul, masked) = if state.reflected {
        let mask2 = ops.load_aligned(&[0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF]);
        let masked = ops.and_vectors(state.value, mask2);
        let shifted = ops.shift_left_12(state.value);
        let clmul = ops.carryless_mul_11(shifted, coeff_vector_high);
        (clmul, masked)
    } else {
        let mask2 = ops.load_aligned(&[0xFFFFFFFFFFFFFFFF, 0x00000000FFFFFFFF]);
        let masked = ops.and_vectors(state.value, mask2);
        let shifted = ops.shift_right_12(state.value);
        let clmul = ops.carryless_mul_10(shifted, coeff_vector_high);
        (clmul, masked)
    };

    state.value = ops.xor_vectors(clmul, masked);
}

/// Barrett reduction for 32-bit-space CRC operations (used by CRC-16 and CRC-32)
///
/// Returns the raw u64 values from the vector; caller extracts appropriate bits:
/// - CRC-32: extracts 32 bits
/// - CRC-16: extracts 16 bits (after scaling back from 32-bit space)
#[inline(always)]
pub unsafe fn barrett_reduction<T: ArchOps>(
    state: &CrcState<T::Vector>,
    poly: u64,
    mu: u64,
    ops: &T,
) -> [u64; 2]
where
    T::Vector: Copy,
{
    let x = state.value;
    let mu_poly = ops.create_vector_from_u64_pair_non_reflected(poly, mu);

    if state.reflected {
        let clmul1 = ops.carryless_mul_00(x, mu_poly);
        let clmul2 = ops.carryless_mul_10(clmul1, mu_poly);
        let xorred = ops.xor_vectors(x, clmul2);
        ops.extract_u64s(xorred)
    } else {
        let clmul1 = ops.shift_left_4(ops.carryless_mul_01(x, mu_poly));
        let clmul2_shifted = ops.shift_left_4(ops.carryless_mul_11(clmul1, mu_poly));
        let final_xor = ops.xor_vectors(clmul2_shifted, x);
        ops.extract_u64s(final_xor)
    }
}

/// Create coefficient vector for 32-bit-space CRC operations (used by CRC-16 and CRC-32)
#[inline(always)]
pub unsafe fn create_coefficient<T: ArchOps>(high: u64, low: u64, ops: &T) -> T::Vector
where
    T::Vector: Copy,
{
    ops.create_vector_from_u64_pair_non_reflected(high, low)
}

/// Get shuffle table pointer for 32-bit-space CRC operations (used by CRC-16 and CRC-32)
#[inline(always)]
pub fn get_last_bytes_table_ptr(reflected: bool, remaining_len: usize) -> (*const u8, usize) {
    if reflected {
        let base_ptr = &PSHUFB_SHF_TABLE_REVERSE as *const _ as *const u8;
        (base_ptr, remaining_len)
    } else {
        let base_ptr = &PSHUFB_SHF_TABLE_FORWARD as *const _ as *const u8;
        (base_ptr, 16 - remaining_len)
    }
}

/// Perform final reduction for 32-bit-space CRC operations (used by CRC-16 and CRC-32)
///
/// Returns the raw u64 values; caller extracts appropriate bits.
#[inline(always)]
pub unsafe fn perform_final_reduction<T: ArchOps>(
    state: T::Vector,
    reflected: bool,
    keys: &[u64; 23],
    ops: &T,
) -> [u64; 2]
where
    T::Vector: Copy,
{
    let mut crc_state = CrcState {
        value: state,
        reflected,
    };

    // Fold 16 bytes into 8 bytes, then 8 bytes into 4 bytes
    fold_width(&mut crc_state, keys[6], keys[5], ops);

    // Perform Barrett reduction to finalize
    barrett_reduction(&crc_state, keys[8], keys[7], ops)
}

/// Process inputs smaller than 16 bytes for 32-bit-space CRC operations (used by CRC-16 and CRC-32)
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
pub unsafe fn process_0_to_15<T: ArchOps, W: EnhancedCrcWidth>(
    data: &[u8],
    state: &mut CrcState<T::Vector>,
    reflector: &Reflector<T::Vector>,
    keys: &[u64; 23],
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    let mut buffer = [0u8; CRC_CHUNK_SIZE];
    if state.reflected {
        buffer[CRC_CHUNK_SIZE - data.len()..].copy_from_slice(data);
    } else {
        buffer[..data.len()].copy_from_slice(data);
    }

    let len = data.len() as i32;
    let base = &PSHUFB_SHF_TABLE_REVERSE as *const _ as *const u8;

    let xmm7 = if state.reflected {
        let data = ops.load_bytes(buffer.as_ptr());
        let mask1 = ops.load_aligned(&SIMD_CONSTANTS[1]);

        let ptr = base.add(if len < 4 {
            8 + len as usize
        } else {
            len as usize
        });
        let mask = ops.load_bytes(ptr);
        let modified_mask = ops.xor_vectors(mask, mask1);
        let shuffled_crc = ops.shuffle_bytes(state.value, modified_mask);

        ops.xor_vectors(
            if len < 4 {
                ops.shift_right_8(data)
            } else {
                data
            },
            shuffled_crc,
        )
    } else {
        let data_arr = ops.load_bytes(buffer.as_ptr());
        let reflected_data = algorithm::reflect_bytes(reflector, data_arr, ops);
        let data_with_crc = ops.xor_vectors(reflected_data, state.value);

        if len < 4 {
            let result = match len {
                3 => ops.shift_right_5(data_with_crc),
                2 => ops.shift_right_6(data_with_crc),
                1 => ops.shift_right_7(data_with_crc),
                _ => data_with_crc,
            };

            return W::barrett_reduction(
                &CrcState {
                    value: result,
                    reflected: false,
                },
                keys[8],
                keys[7],
                ops,
            );
        }

        let base = &PSHUFB_SHF_TABLE_FORWARD as *const _ as *const u8;
        let ptr = base.add(16 - len as usize);
        let x0 = ops.load_bytes(ptr);
        let mask1 = ops.load_aligned(&SIMD_CONSTANTS[1]);
        let x0 = ops.xor_vectors(x0, mask1);

        if len < 8 {
            ops.shuffle_bytes(data_with_crc, x0)
        } else {
            let mut xmm7 = ops.load_bytes(buffer.as_ptr());
            if let Reflector::ForwardReflector { smask } = reflector {
                xmm7 = ops.shuffle_bytes(xmm7, *smask);
            }
            xmm7 = ops.xor_vectors(xmm7, state.value);
            let ptr = base.add(16 - len as usize);
            let x0 = ops.load_bytes(ptr);
            let xmm0 = ops.xor_vectors(x0, mask1);

            ops.shuffle_bytes(xmm7, xmm0)
        }
    };

    if len >= 4 {
        return W::perform_final_reduction(xmm7, state.reflected, keys, ops);
    }

    let final_state = CrcState {
        value: xmm7,
        reflected: state.reflected,
    };

    W::barrett_reduction(&final_state, keys[8], keys[7], ops)
}
