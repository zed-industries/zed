// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module contains the main algorithm for CRC calculation.
//!
//! This implementation is designed to work with both CRC-32 and CRC-64 algorithms.
//!
//! It uses SIMD instructions for performance optimization and is designed to be
//! platform-agnostic.
//!
//! The code is structured to allow for easy extension and modification for
//! different architectures and CRC algorithms.
//!
//! The main entry point is the `update` function, which takes the current CRC state,
//! the input data, CRC parameters, and architecture-specific operations.

#![cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]

use crate::consts::CRC_CHUNK_SIZE;
use crate::enums::{DataChunkProcessor, Reflector};
use crate::structs::CrcState;
use crate::traits::{ArchOps, EnhancedCrcWidth};
use crate::{crc16, crc32, crc64, CrcParams};

/// Extract keys from CrcParams using safe accessor methods
/// This ensures bounds checking and future compatibility
#[inline(always)]
fn extract_keys_array(params: &CrcParams) -> [u64; 23] {
    [
        params.get_key(0),
        params.get_key(1),
        params.get_key(2),
        params.get_key(3),
        params.get_key(4),
        params.get_key(5),
        params.get_key(6),
        params.get_key(7),
        params.get_key(8),
        params.get_key(9),
        params.get_key(10),
        params.get_key(11),
        params.get_key(12),
        params.get_key(13),
        params.get_key(14),
        params.get_key(15),
        params.get_key(16),
        params.get_key(17),
        params.get_key(18),
        params.get_key(19),
        params.get_key(20),
        params.get_key(21),
        params.get_key(22),
    ]
}

/// Main entry point that works for both CRC-32 and CRC-64
#[inline(always)]
pub unsafe fn update<T: ArchOps, W: EnhancedCrcWidth>(
    state: W::Value,
    bytes: &[u8],
    params: &CrcParams,
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    let len = bytes.len();
    if len == 0 {
        return state;
    }

    // Create the appropriate reflector based on CRC type
    let reflector = if params.refin {
        Reflector::NoReflector
    } else {
        // Load mask for byte-swapping operations
        let smask = ops.load_aligned(&W::load_constants(params.refin)[0] as *const [u64; 2]);
        Reflector::ForwardReflector { smask }
    };

    // Create initial CRC state
    let mut crc_state = W::create_state(state, params.refin, ops);

    // Extract keys once and pass by reference to avoid repeated stack copies
    let keys = extract_keys_array(params);

    // Process data differently based on length
    // On ARM M4 Max, ARM c8g, x86 c7a, and x86 c7i, using 128 bytes is a measurably faster
    // threshold than 256 bytes...
    if len < 128 {
        // Select processor based on input length
        let processor = DataChunkProcessor::for_length(len);
        return process_by_strategy::<T, W>(
            processor,
            bytes,
            &mut crc_state,
            reflector,
            &keys,
            ops,
        );
    }

    // Process large inputs with SIMD-optimized approach
    process_large_aligned::<T, W>(bytes, &mut crc_state, reflector, &keys, ops)
}

/// Process data with the selected strategy
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
unsafe fn process_by_strategy<T: ArchOps, W: EnhancedCrcWidth>(
    strategy: DataChunkProcessor,
    data: &[u8],
    state: &mut CrcState<T::Vector>,
    reflector: Reflector<T::Vector>,
    keys: &[u64; 23],
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    match strategy {
        DataChunkProcessor::From0To15 => match W::WIDTH {
            16 => crc16::algorithm::process_0_to_15::<T, W>(data, state, &reflector, keys, ops),
            32 => crc32::algorithm::process_0_to_15::<T, W>(data, state, &reflector, keys, ops),
            64 => crc64::algorithm::process_0_to_15::<T, W>(data, state, &reflector, keys, ops),
            _ => panic!("Unsupported CRC width"),
        },
        DataChunkProcessor::From16 => {
            process_exactly_16::<T, W>(data, state, &reflector, keys, ops)
        }
        DataChunkProcessor::From17To31 => {
            process_17_to_31::<T, W>(data, state, &reflector, keys, ops)
        }
        DataChunkProcessor::From32To255 => {
            process_32_to_255::<T, W>(data, state, &reflector, keys, ops)
        }
    }
}

/// Process large inputs with proper SIMD alignment
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
unsafe fn process_large_aligned<T: ArchOps, W: EnhancedCrcWidth>(
    bytes: &[u8],
    state: &mut CrcState<T::Vector>,
    reflector: Reflector<T::Vector>,
    keys: &[u64; 23],
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    // Align data for SIMD processing
    let (left, middle, right) = bytes.align_to::<[T::Vector; 8]>();

    if let Some((first, rest)) = middle.split_first() {
        if !left.is_empty() {
            let processor = DataChunkProcessor::for_length(left.len());
            // Process unaligned bytes at the start
            let left_crc =
                process_by_strategy::<T, W>(processor, left, state, reflector, keys, ops);
            // Update state with the result from processing left bytes
            *state = W::create_state(left_crc, state.reflected, ops);
        }

        // try to use the enhanced SIMD implementation first, fall back to non-enhanced if necessary
        if rest.is_empty()
            || !ops.process_enhanced_simd_blocks::<W>(state, first, rest, &reflector, keys)
        {
            process_simd_chunks::<T, W>(state, first, rest, &reflector, keys, ops);
        }

        // Process any unaligned bytes at the end
        if !right.is_empty() {
            let processor = DataChunkProcessor::for_length(right.len());
            // Use the current state to process the right bytes
            return process_by_strategy::<T, W>(processor, right, state, reflector, keys, ops);
        }

        // Extract the final result
        return W::extract_result(state.value, state.reflected, ops);
    }

    // Fall back to existing implementation if proper alignment isn't possible
    let processor = DataChunkProcessor::for_length(bytes.len());
    process_by_strategy::<T, W>(processor, bytes, state, reflector, keys, ops)
}

/// Process SIMD-aligned chunks of 128 bytes
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
unsafe fn process_simd_chunks<T: ArchOps, W: EnhancedCrcWidth>(
    state: &mut CrcState<T::Vector>,
    first: &[T::Vector; 8],
    rest: &[[T::Vector; 8]],
    reflector: &Reflector<T::Vector>,
    keys: &[u64; 23],
    ops: &T,
) where
    T::Vector: Copy,
{
    // Create a copy of the first 128 bytes
    let mut x = *first;

    // Apply initial reflection if needed
    for item in &mut x {
        *item = reflect_bytes(reflector, *item, ops);
    }

    // XOR initial CRC with the first 16 bytes
    x[0] = ops.xor_vectors(x[0], state.value);

    // Load the coefficient pair for folding
    let coeff = W::create_coefficient(keys[4], keys[3], state.reflected, ops);

    // Process remaining 128-byte chunks
    for chunk in rest {
        for (xi, yi) in x.iter_mut().zip(chunk.iter()) {
            // Load and reflect the new data if needed
            let yi = reflect_bytes(reflector, *yi, ops);

            // Create a temporary state for folding
            let mut temp_state = CrcState {
                value: *xi,
                reflected: state.reflected,
            };

            // Fold 16 bytes
            W::fold_16(&mut temp_state, coeff, yi, ops);

            // XOR with new data
            *xi = temp_state.value;
        }
    }

    // Fold the 8 xmm registers to 1 xmm register with different constants
    let mut res = x[7];

    // Create fold coefficients for different distances
    let fold_coefficients = [
        W::create_coefficient(keys[10], keys[9], state.reflected, ops), // 112 bytes
        W::create_coefficient(keys[12], keys[11], state.reflected, ops), // 96 bytes
        W::create_coefficient(keys[14], keys[13], state.reflected, ops), // 80 bytes
        W::create_coefficient(keys[16], keys[15], state.reflected, ops), // 64 bytes
        W::create_coefficient(keys[18], keys[17], state.reflected, ops), // 48 bytes
        W::create_coefficient(keys[20], keys[19], state.reflected, ops), // 32 bytes
        W::create_coefficient(keys[2], keys[1], state.reflected, ops),  // 16 bytes
    ];

    for (i, &coeff) in fold_coefficients.iter().enumerate() {
        let mut temp_state = CrcState {
            value: x[i],
            reflected: state.reflected,
        };
        W::fold_16(&mut temp_state, coeff, res, ops);

        res = temp_state.value
    }

    // Perform final reduction and update state
    let final_value = W::perform_final_reduction(res, state.reflected, keys, ops);
    *state = W::create_state(final_value, state.reflected, ops);
}

/// Process exactly 16 bytes
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
unsafe fn process_exactly_16<T: ArchOps, W: EnhancedCrcWidth>(
    data: &[u8],
    state: &mut CrcState<T::Vector>,
    reflector: &Reflector<T::Vector>,
    keys: &[u64; 23],
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    // Process 16 bytes and fold to width-specific size
    W::perform_final_reduction(
        process_16_byte_block(data.as_ptr(), state.value, reflector, ops),
        state.reflected,
        keys,
        ops,
    )
}

/// Process a 16-byte block of data
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
unsafe fn process_16_byte_block<T: ArchOps>(
    data_ptr: *const u8,
    initial_crc: T::Vector,
    reflector: &Reflector<T::Vector>,
    ops: &T,
) -> T::Vector
where
    T::Vector: Copy,
{
    // Load data, apply reflection, and XOR with initial CRC
    ops.xor_vectors(
        reflect_bytes(reflector, ops.load_bytes(data_ptr), ops),
        initial_crc,
    )
}

/// Reflect bytes according to the reflector strategy
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
pub(crate) unsafe fn reflect_bytes<T: ArchOps>(
    reflector: &Reflector<T::Vector>,
    data: T::Vector,
    ops: &T,
) -> T::Vector
where
    T::Vector: Copy,
{
    match reflector {
        Reflector::NoReflector => data,
        Reflector::ForwardReflector { smask } => ops.shuffle_bytes(data, *smask),
    }
}

/// Fold and XOR generic implementation
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
unsafe fn fold_and_xor<T: ArchOps, W: EnhancedCrcWidth>(
    current: T::Vector,
    coefficient: T::Vector,
    data_to_xor: T::Vector,
    reflected: bool,
    ops: &T,
) -> T::Vector
where
    T::Vector: Copy,
{
    // Create a temporary state for folding
    let mut temp_state = CrcState {
        value: current,
        reflected,
    };

    // Fold 16 bytes using width-specific method
    W::fold_16(&mut temp_state, coefficient, data_to_xor, ops);

    temp_state.value
}

/// Process inputs between 17 and 31 bytes
/// This implementation works for both CRC-32 and CRC-64 using the width-specific traits
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
unsafe fn process_17_to_31<T: ArchOps, W: EnhancedCrcWidth>(
    data: &[u8],
    state: &mut CrcState<T::Vector>,
    reflector: &Reflector<T::Vector>,
    keys: &[u64; 23],
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    // Process the first 16 bytes
    let xmm7 = process_16_byte_block(data.as_ptr(), state.value, reflector, ops);

    // Process the remaining bytes (1-15)
    let remaining_len = data.len() - CRC_CHUNK_SIZE;

    // Use the shared function to handle the last two chunks
    let final_xmm7 = get_last_two_xmms::<T, W>(
        DataRegion {
            full_data: data,
            offset: CRC_CHUNK_SIZE,
            remaining: remaining_len,
        },
        xmm7,
        keys,
        reflector,
        state.reflected,
        ops,
    );

    // Perform final reduction
    W::perform_final_reduction(final_xmm7, state.reflected, keys, ops)
}

// Process inputs between 32 and 255 bytes
/// This implementation works for both CRC-32 and CRC-64 using the width-specific traits
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
unsafe fn process_32_to_255<T: ArchOps, W: EnhancedCrcWidth>(
    data: &[u8],
    state: &mut CrcState<T::Vector>,
    reflector: &Reflector<T::Vector>,
    keys: &[u64; 23],
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    let mut current_pos = CRC_CHUNK_SIZE;
    let mut remaining_len = data.len() - CRC_CHUNK_SIZE;

    // Process first 16 bytes
    let mut xmm7 = process_16_byte_block(data.as_ptr(), state.value, reflector, ops);

    // Create coefficient for folding operations
    let rk01rk02 = W::create_coefficient(keys[2], keys[1], state.reflected, ops);

    // Main processing loop - 16 bytes at a time
    while remaining_len >= CRC_CHUNK_SIZE {
        // Load next 16 bytes of data
        let next_data = reflect_bytes(
            reflector,
            ops.load_bytes(data.as_ptr().add(current_pos)),
            ops,
        );

        // Fold and XOR
        xmm7 = fold_and_xor::<T, W>(xmm7, rk01rk02, next_data, state.reflected, ops);

        // Update position tracking
        current_pos += CRC_CHUNK_SIZE;
        remaining_len -= CRC_CHUNK_SIZE;
    }

    // Handle remaining bytes (if any)
    if remaining_len > 0 {
        // Use the shared get_last_two_xmms function to handle the remaining bytes
        xmm7 = get_last_two_xmms::<T, W>(
            DataRegion {
                full_data: data,
                offset: current_pos,
                remaining: remaining_len,
            },
            xmm7,
            keys,
            reflector,
            state.reflected,
            ops,
        );
    }

    // Perform final reduction
    W::perform_final_reduction(xmm7, state.reflected, keys, ops)
}

/// Data region descriptor for overlapping SIMD reads in CRC processing
///
/// # Safety Invariants
///
/// When this struct is used with overlapping SIMD reads:
/// - `offset` must be >= `CRC_CHUNK_SIZE` (16 bytes)
/// - `remaining` must be in range 1..=15
/// - `full_data` must contain at least `offset + remaining` bytes
struct DataRegion<'a> {
    full_data: &'a [u8],
    offset: usize,
    remaining: usize,
}

/// Handle the last two chunks of data using an overlapping SIMD read
///
/// # Safety
///
/// - `region.full_data` must contain at least `region.offset + region.remaining` bytes
/// - `region.offset` must be >= `CRC_CHUNK_SIZE` (16 bytes)
/// - `region.remaining` must be in range 1..=15
/// - Caller must ensure appropriate SIMD features are available
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
unsafe fn get_last_two_xmms<T: ArchOps, W: EnhancedCrcWidth>(
    region: DataRegion,
    current_state: T::Vector,
    keys: &[u64; 23],
    reflector: &Reflector<T::Vector>,
    reflected: bool,
    ops: &T,
) -> T::Vector
where
    T::Vector: Copy,
{
    debug_assert!(region.offset >= CRC_CHUNK_SIZE);
    debug_assert!(region.remaining > 0 && region.remaining < CRC_CHUNK_SIZE);
    debug_assert!(region.offset + region.remaining <= region.full_data.len());

    let coefficient = W::create_coefficient(keys[2], keys[1], reflected, ops);
    let const_mask = ops.set_all_bytes(0x80);
    let (table_ptr, offset) = W::get_last_bytes_table_ptr(reflected, region.remaining);

    if reflected {
        // Overlapping read: loads tail of previous chunk + remaining data
        let read_offset = region.offset - CRC_CHUNK_SIZE + region.remaining;
        let xmm1 = ops.load_bytes(region.full_data.as_ptr().add(read_offset));
        let mut xmm0 = ops.load_bytes(table_ptr.add(offset));
        let shuffled = ops.shuffle_bytes(current_state, xmm0);
        xmm0 = ops.xor_vectors(xmm0, const_mask);

        let shuffled_masked = ops.shuffle_bytes(current_state, xmm0);

        // CRC-16 and CRC-32 use the same logic (both operate in 32-bit space)
        let (xmm2_blended, mut temp_state) = if W::WIDTH <= 32 {
            let compare_mask = ops.create_compare_mask(xmm0);

            let xmm2_blended = ops.blend_vectors(xmm1, shuffled, compare_mask);

            let temp_state = CrcState {
                value: shuffled_masked,
                reflected,
            };

            (xmm2_blended, temp_state)
        } else {
            let xmm2_blended = ops.blend_vectors(shuffled_masked, xmm1, xmm0);

            let temp_state = CrcState {
                value: shuffled,
                reflected,
            };

            (xmm2_blended, temp_state)
        };

        W::fold_16(&mut temp_state, coefficient, xmm2_blended, ops);

        temp_state.value
    } else {
        let read_offset = region.offset - CRC_CHUNK_SIZE + region.remaining;
        let mut xmm1 = ops.load_bytes(region.full_data.as_ptr().add(read_offset));

        if let Reflector::ForwardReflector { smask } = reflector {
            xmm1 = ops.shuffle_bytes(xmm1, *smask);
        }

        let xmm0 = ops.load_bytes(table_ptr.add(offset));
        let shuffled = ops.shuffle_bytes(current_state, xmm0);
        let xmm0_masked = ops.xor_vectors(xmm0, const_mask);
        let shuffled_masked = ops.shuffle_bytes(current_state, xmm0_masked);
        let xmm2_blended = ops.blend_vectors(xmm1, shuffled, xmm0_masked);

        // Create a temporary state for folding
        let mut temp_state = CrcState {
            value: shuffled_masked,
            reflected,
        };

        W::fold_16(&mut temp_state, coefficient, xmm2_blended, ops);

        temp_state.value
    }
}
