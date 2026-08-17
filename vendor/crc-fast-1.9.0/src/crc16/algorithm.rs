// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides the CRC-16 algorithm implementations.
//!
//! CRC-16 computation is performed by scaling 16-bit values to 32-bit space,
//! using the shared width32_ops module, and then scaling the result back to 16 bits.

#![cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]

use crate::enums::Reflector;
use crate::structs::CrcState;
use crate::traits::{ArchOps, EnhancedCrcWidth};

impl EnhancedCrcWidth for crate::structs::Width16 {
    #[inline(always)]
    fn load_constants(reflected: bool) -> [[u64; 2]; 4] {
        crate::crc32::width32_ops::load_constants(reflected)
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
            // For reflected mode, state goes in the low 32 bits (no scaling needed)
            ops.create_vector_from_u32(value as u32, false)
        } else {
            // For non-reflected mode, scale to 32 bits and position in high bytes
            // CRC-16 forward: shift left 16 bits to scale to 32-bit space
            let scaled = (value as u32) << 16;
            ops.create_vector_from_u32(scaled, true)
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
            // In reflected mode, the result is in the low 32 bits of the low 64 bits
            // Extract the 16-bit result
            u64s[0] as u16
        } else {
            // In non-reflected mode, the result is in the high 32 bits of the low 64 bits
            // Scale back from 32 bits to 16 bits
            ((u64s[1] >> 32) >> 16) as u16
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
        crate::crc32::width32_ops::fold_16(state, coeff, data_to_xor, ops)
    }

    #[inline(always)]
    unsafe fn fold_width<T: ArchOps>(state: &mut CrcState<T::Vector>, high: u64, low: u64, ops: &T)
    where
        T::Vector: Copy,
    {
        crate::crc32::width32_ops::fold_width(state, high, low, ops)
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
        let u64s = crate::crc32::width32_ops::barrett_reduction(state, poly, mu, ops);
        if state.reflected {
            u64s[1] as u16
        } else {
            // Scale back from 32 bits to 16 bits
            ((u64s[0] >> 32) >> 16) as u16
        }
    }

    #[inline(always)]
    unsafe fn create_coefficient<T: ArchOps>(
        high: u64,
        low: u64,
        _reflected: bool,
        ops: &T,
    ) -> T::Vector
    where
        T::Vector: Copy,
    {
        crate::crc32::width32_ops::create_coefficient(high, low, ops)
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
        let u64s = crate::crc32::width32_ops::perform_final_reduction(state, reflected, keys, ops);
        if reflected {
            u64s[1] as u16
        } else {
            // Scale back from 32 bits to 16 bits
            ((u64s[0] >> 32) >> 16) as u16
        }
    }

    #[inline(always)]
    fn get_last_bytes_table_ptr(reflected: bool, remaining_len: usize) -> (*const u8, usize) {
        crate::crc32::width32_ops::get_last_bytes_table_ptr(reflected, remaining_len)
    }
}

/// Process inputs smaller than 16 bytes for CRC-16
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
    crate::crc32::width32_ops::process_0_to_15::<T, W>(data, state, reflector, keys, ops)
}
