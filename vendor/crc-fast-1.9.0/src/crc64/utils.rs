// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]

// SIMD intrinsics for debug printing (std only)
#[cfg(all(feature = "std", target_arch = "aarch64"))]
use core::arch::aarch64::*;

#[cfg(all(feature = "std", target_arch = "x86"))]
use core::arch::x86::*;

#[cfg(all(feature = "std", target_arch = "x86_64"))]
use core::arch::x86_64::*;

// ArchOps trait for generic vector printing (std only)
#[cfg(feature = "std")]
use crate::traits::ArchOps;

#[cfg(all(feature = "std", target_arch = "aarch64"))]
#[allow(dead_code)]
#[target_feature(enable = "aes")]
pub(crate) unsafe fn print_xmm_hex(prefix: &str, xmm: uint8x16_t) {
    let mut temp = [0u64; 2];
    vst1q_u64(temp.as_mut_ptr(), vreinterpretq_u64_u8(xmm));
    println!("{}={:#016x}{:016x}", prefix, temp[1], temp[0]);
}

#[cfg(all(feature = "std", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(dead_code)]
#[target_feature(enable = "sse4.1")]
pub(crate) unsafe fn print_xmm_hex(prefix: &str, xmm: __m128i) {
    let mut temp = [0u64; 2];
    _mm_storeu_si128(temp.as_mut_ptr() as *mut __m128i, xmm);
    println!("{}={:#016x}{:016x}", prefix, temp[1], temp[0]);
}

#[cfg(feature = "std")]
#[allow(dead_code)]
pub(crate) unsafe fn print_vector_hex<T: ArchOps>(prefix: &str, vector: T::Vector, ops: &T) {
    // Extract the u64 values from the vector using the ArchOps trait
    let values = ops.extract_u64s(vector);

    // Print in the same format as your original functions
    println!("{}={:#016x}_{:016x}", prefix, values[1], values[0]);
}

/// Print a vector as u8 array (useful for byte-level debugging)
#[cfg(feature = "std")]
#[allow(dead_code)]
pub(crate) unsafe fn print_vector_bytes<T: ArchOps>(prefix: &str, vector: T::Vector, ops: &T) {
    // Extract the u64 values
    let values = ops.extract_u64s(vector);

    // Convert to bytes for detailed inspection
    let bytes: [u8; 16] = core::mem::transmute([values[0], values[1]]);

    println!("{}=[{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x},{:02x}]",
             prefix,
             bytes[0], bytes[1], bytes[2], bytes[3],
             bytes[4], bytes[5], bytes[6], bytes[7],
             bytes[8], bytes[9], bytes[10], bytes[11],
             bytes[12], bytes[13], bytes[14], bytes[15]
    );
}
