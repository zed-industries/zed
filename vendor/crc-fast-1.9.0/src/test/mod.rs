// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides tests and utilities for the CRC library.

#![cfg(test)]
#![allow(dead_code)]

pub(crate) mod consts;
pub(crate) mod enums;
mod future_proof_tests;
mod structs;

use proptest::test_runner::Config as ProptestConfig;

/// Returns a proptest config that works under Miri by disabling file-based failure persistence.
/// Miri runs with isolation enabled by default, which blocks getcwd() calls that proptest
/// uses for its failure persistence feature.
pub(crate) fn miri_compatible_proptest_config() -> ProptestConfig {
    let mut config = ProptestConfig::with_cases(100);

    if cfg!(miri) {
        // don't let Miri spend too much time on these
        config.cases = 1;

        // disable file-based failure persistence to avoid getcwd() calls
        config.failure_persistence = None;

        return config;
    }

    config
}

/// Creates a new aligned data vector from the input slice for testing.
pub(crate) fn create_aligned_data(input: &[u8]) -> Vec<u8> {
    // Size of our target alignment structure
    let align_size = std::mem::size_of::<[[u64; 4]; 2]>(); // 64 bytes

    // Create a zero-filled vector with padding to ensure we can find a properly aligned position
    let mut padded = vec![0; input.len() + align_size];

    // Find the first address that satisfies our alignment
    let start_addr = padded.as_ptr() as usize;
    let align_offset = (align_size - (start_addr % align_size)) % align_size;

    // Copy the input into the aligned position
    let aligned_start = &mut padded[align_offset..];
    aligned_start[..input.len()].copy_from_slice(input);

    // Return the exact slice we need
    aligned_start[..input.len()].to_vec()
}
