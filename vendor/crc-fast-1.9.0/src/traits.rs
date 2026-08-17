// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![allow(dead_code)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
use crate::enums::Reflector;

use crate::CrcParams;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
use crate::structs::CrcState;

use core::ops::BitXor;

/// Marker trait for CRC width
pub trait CrcWidth {
    /// The width in bits
    const WIDTH: u32;
    /// The natural value type for this width
    type Value: Copy + BitXor<Output = Self::Value>;
}

pub(crate) trait CrcCalculator {
    fn update(data: &[u8], state: u64, params: &CrcParams) -> u64 {
        Self::calculate(state, data, params)
    }

    fn checksum(data: &[u8], params: &CrcParams) -> u64 {
        Self::calculate(params.init, data, params) ^ params.xorout
    }

    fn calculate(state: u64, data: &[u8], params: &CrcParams) -> u64;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
/// Trait defining architecture-specific SIMD operations for CRC calculation
pub trait ArchOps: Sized + Copy + Clone {
    /// The SIMD vector type used by this architecture
    type Vector;

    /// Process aligned blocks using potentially accelerated SIMD operations
    ///
    /// Returns true if the operation was handled by the accelerated path (for example,
    /// using VPCLMULQDQ)
    unsafe fn process_enhanced_simd_blocks<W: EnhancedCrcWidth>(
        &self,
        _state: &mut CrcState<Self::Vector>,
        _first: &[Self::Vector; 8],
        _rest: &[[Self::Vector; 8]],
        _reflector: &Reflector<Self::Vector>,
        _keys: &[u64; 23],
    ) -> bool
    where
        Self::Vector: Copy,
    {
        // Default implementation just returns false
        // indicating the non-enhanced algorithm should be used
        false
    }

    /// Create a SIMD vector from a u64 pair
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn create_vector_from_u64_pair(
        &self,
        high: u64,
        low: u64,
        reflected: bool,
    ) -> Self::Vector;

    /// Create a SIMD vector from a u64 pair without reflection
    ///
    /// TODO: I have no idea (yet) why CRC-32 doesn't use reflection, but CRC-64 does.
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn create_vector_from_u64_pair_non_reflected(&self, high: u64, low: u64)
        -> Self::Vector;

    /// Create a SIMD vector with a single u64 value
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn create_vector_from_u64(&self, value: u64, high: bool) -> Self::Vector;

    /// Extract two u64 values from a SIMD vector
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn extract_u64s(&self, vector: Self::Vector) -> [u64; 2];

    /// Extract two polynomial values (for carryless multiplication)
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn extract_poly64s(&self, vector: Self::Vector) -> [u64; 2];

    /// XOR two SIMD vectors
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn xor_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector;

    /// Load bytes from memory into a SIMD vector
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn load_bytes(&self, ptr: *const u8) -> Self::Vector;

    /// Load aligned bytes from memory
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn load_aligned(&self, ptr: *const [u64; 2]) -> Self::Vector;

    //unsafe fn load_aligned(&self, ptr: &[u64]) -> Self::Vector;

    //unsafe fn load_aligned_const(&self, ptr: *const [u64; 2]) -> Self::Vector;

    /// Shuffle/permute bytes according to a mask
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn shuffle_bytes(&self, data: Self::Vector, mask: Self::Vector) -> Self::Vector;

    /// Blend two vectors using a mask (select from a or b based on mask bits)
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn blend_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        mask: Self::Vector,
    ) -> Self::Vector;

    /// Shift a vector left by 8 bytes
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn shift_left_8(&self, vector: Self::Vector) -> Self::Vector;

    /// Create a vector with all bytes set to the same value
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn set_all_bytes(&self, value: u8) -> Self::Vector;

    /// Create a comparison mask (for blending operations)
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn create_compare_mask(&self, vector: Self::Vector) -> Self::Vector;

    /// AND two vectors
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn and_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector;

    /// Shift a vector right by 32 bits (4 bytes)
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn shift_right_32(&self, vector: Self::Vector) -> Self::Vector;

    /// Shift a vector left by 32 bits (4 bytes)
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn shift_left_32(&self, vector: Self::Vector) -> Self::Vector;

    /// Create a SIMD vector with a single u32 value
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn create_vector_from_u32(&self, value: u32, high: bool) -> Self::Vector;

    /// Shift a vector left by 4 bytes (32 bits)
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn shift_left_4(&self, vector: Self::Vector) -> Self::Vector;

    /// Shift a vector right by 4 bytes (32 bits)
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn shift_right_4(&self, vector: Self::Vector) -> Self::Vector;

    /// Shift a vector right by 8 bytes (64 bits)
    ///
    /// # Safety
    /// May use native CPU features
    unsafe fn shift_right_8(&self, vector: Self::Vector) -> Self::Vector;

    /// Shift a vector right by 5 bytes
    unsafe fn shift_right_5(&self, vector: Self::Vector) -> Self::Vector;

    /// Shift a vector right by 6 bytes
    unsafe fn shift_right_6(&self, vector: Self::Vector) -> Self::Vector;

    /// Shift a vector right by 7 bytes
    unsafe fn shift_right_7(&self, vector: Self::Vector) -> Self::Vector;

    /// Shift a vector right by 12 bytes
    unsafe fn shift_right_12(&self, vector: Self::Vector) -> Self::Vector;

    /// Shift a vector left by 12 bytes
    unsafe fn shift_left_12(&self, vector: Self::Vector) -> Self::Vector;

    /// Perform carryless multiplication with immediate value 0x00 (low 64 bits of both vectors)
    unsafe fn carryless_mul_00(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector;

    /// Perform carryless multiplication with immediate value 0x01 (low 64 bits of a, high 64 bits of b)
    unsafe fn carryless_mul_01(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector;

    /// Perform carryless multiplication with immediate value 0x10 (high 64 bits of a, low 64 bits of b)
    unsafe fn carryless_mul_10(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector;

    /// Perform carryless multiplication with immediate value 0x11 (high 64 bits of both vectors)
    unsafe fn carryless_mul_11(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector;

    /// XOR three vectors together: a XOR b XOR c
    /// Uses native XOR3 instructions when available, falls back to two XOR operations otherwise
    unsafe fn xor3_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        c: Self::Vector,
    ) -> Self::Vector;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
/// Enhanced CrcWidth trait with additional operations for generic CRC implementation
pub trait EnhancedCrcWidth: CrcWidth {
    /// Load constants specific to CRC width
    fn load_constants(reflected: bool) -> [[u64; 2]; 4];

    /// Create a CRC state with the initial value positioned correctly for the width
    unsafe fn create_state<T: ArchOps>(
        value: Self::Value,
        reflected: bool,
        ops: &T,
    ) -> CrcState<T::Vector>
    where
        T::Vector: Copy;

    /// Extract the final CRC result from a SIMD vector
    unsafe fn extract_result<T: ArchOps>(
        vector: T::Vector,
        reflected: bool,
        ops: &T,
    ) -> Self::Value
    where
        T::Vector: Copy;

    /// Perform width-specific folding operations using CLMUL and two XOR operations (or one XOR3)
    unsafe fn fold_16<T: ArchOps>(
        state: &mut CrcState<T::Vector>,
        coefficient: T::Vector,
        data_to_xor: T::Vector,
        ops: &T,
    ) where
        T::Vector: Copy;

    /// Fold width-specific number of bytes
    unsafe fn fold_width<T: ArchOps>(state: &mut CrcState<T::Vector>, high: u64, low: u64, ops: &T)
    where
        T::Vector: Copy;

    /// Width-specific Barrett reduction
    unsafe fn barrett_reduction<T: ArchOps>(
        state: &CrcState<T::Vector>,
        poly: u64,
        mu: u64,
        ops: &T,
    ) -> Self::Value
    where
        T::Vector: Copy;

    /// Create a coefficient vector for folding operations
    unsafe fn create_coefficient<T: ArchOps>(
        high: u64,
        low: u64,
        reflected: bool,
        ops: &T,
    ) -> T::Vector
    where
        T::Vector: Copy;

    /// Perform final reduction for the specific width
    unsafe fn perform_final_reduction<T: ArchOps>(
        state: T::Vector,
        reflected: bool,
        keys: &[u64; 23],
        ops: &T,
    ) -> Self::Value
    where
        T::Vector: Copy;

    /// Get the appropriate shuffle table pointer and offset for handling last bytes
    fn get_last_bytes_table_ptr(reflected: bool, remaining_len: usize) -> (*const u8, usize);
}
