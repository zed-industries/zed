// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![allow(dead_code)]

use crate::traits::{CrcCalculator, CrcWidth};
use crate::{arch, cache, CrcAlgorithm, CrcParams};

/// CRC-16 width implementation
#[derive(Clone, Copy)]
pub struct Width16;

impl CrcWidth for Width16 {
    const WIDTH: u32 = 16;
    type Value = u16;
}

/// CRC-32 width implementation
#[derive(Clone, Copy)]
pub struct Width32;

impl CrcWidth for Width32 {
    const WIDTH: u32 = 32;
    type Value = u32;
}

/// CRC-64 width implementation
#[derive(Clone, Copy)]
pub struct Width64;

impl CrcWidth for Width64 {
    const WIDTH: u32 = 64;
    type Value = u64;
}

/// CRC State wrapper to manage the SIMD operations and reflection mode
#[derive(Debug, Clone, Copy)]
pub struct CrcState<T> {
    pub value: T,
    pub reflected: bool,
}

pub(crate) struct Calculator {}

impl CrcCalculator for Calculator {
    #[inline(always)]
    fn calculate(state: u64, data: &[u8], params: &CrcParams) -> u64 {
        unsafe { arch::update(state, data, params) }
    }
}

impl CrcParams {
    /// Creates custom CRC parameters for a given set of Rocksoft CRC parameters.
    ///
    /// Uses an internal cache to avoid regenerating folding keys for identical parameter sets.
    /// The first call with a given set of parameters will generate and cache the keys, while
    /// subsequent calls with the same parameters will use the cached keys for optimal performance.
    ///
    /// Does not support mis-matched refin/refout parameters, so both must be true or both false.
    ///
    /// Rocksoft parameters for lots of variants: https://reveng.sourceforge.io/crc-catalogue/all.htm
    pub fn new(
        name: &'static str,
        width: u8,
        poly: u64,
        init: u64,
        reflected: bool,
        xorout: u64,
        check: u64,
    ) -> Self {
        let keys_array = cache::get_or_generate_keys(width, poly, reflected);
        let keys = crate::CrcKeysStorage::from_keys_fold_256(keys_array);

        // Validate width is supported
        if width != 16 && width != 32 && width != 64 {
            panic!("Unsupported width: {width}");
        }

        // For reflected CRC-16, bit-reverse the init value for the SIMD algorithm
        let init_algorithm = if width == 16 && reflected {
            (init as u16).reverse_bits() as u64
        } else {
            init
        };

        Self {
            algorithm: CrcAlgorithm::CrcCustom,
            name,
            width,
            poly,
            init,
            init_algorithm,
            refin: reflected,
            refout: reflected,
            xorout,
            check,
            keys,
        }
    }

    /// Gets a key at the specified index, returning 0 if out of bounds.
    /// This provides safe access regardless of internal key storage format.
    #[inline(always)]
    pub fn get_key(&self, index: usize) -> u64 {
        self.keys.get_key(index)
    }

    /// Gets a key at the specified index, returning None if out of bounds.
    /// This provides optional key access for cases where bounds checking is needed.
    #[inline(always)]
    pub fn get_key_checked(&self, index: usize) -> Option<u64> {
        if index < self.keys.key_count() {
            Some(self.keys.get_key(index))
        } else {
            None
        }
    }

    /// Returns the number of keys available in this CrcParams instance.
    #[inline(always)]
    pub fn key_count(&self) -> usize {
        self.keys.key_count()
    }
}
