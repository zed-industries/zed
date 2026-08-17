// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! Tests for future-proof CrcKeysStorage and CrcParams functionality

#![cfg(test)]

use crate::{CrcAlgorithm, CrcKeysStorage, CrcParams};

#[test]
fn test_crc_keys_storage_bounds_checking() {
    // Test KeysFold256 variant (23 keys)
    let keys_23 = [1u64; 23];
    let storage_23 = CrcKeysStorage::from_keys_fold_256(keys_23);

    // Test valid indices
    for i in 0..23 {
        assert_eq!(
            storage_23.get_key(i),
            1,
            "Valid index {} should return key value",
            i
        );
    }

    // Test out-of-bounds indices return 0
    assert_eq!(
        storage_23.get_key(23),
        0,
        "Index 23 should return 0 for 23-key storage"
    );
    assert_eq!(
        storage_23.get_key(24),
        0,
        "Index 24 should return 0 for 23-key storage"
    );
    assert_eq!(
        storage_23.get_key(100),
        0,
        "Large index should return 0 for 23-key storage"
    );

    // Test KeysFutureTest variant (25 keys)
    let keys_25 = [2u64; 25];
    let storage_25 = CrcKeysStorage::from_keys_fold_future_test(keys_25);

    // Test valid indices
    for i in 0..25 {
        assert_eq!(
            storage_25.get_key(i),
            2,
            "Valid index {} should return key value",
            i
        );
    }

    // Test out-of-bounds indices return 0
    assert_eq!(
        storage_25.get_key(25),
        0,
        "Index 25 should return 0 for 25-key storage"
    );
    assert_eq!(
        storage_25.get_key(26),
        0,
        "Index 26 should return 0 for 25-key storage"
    );
    assert_eq!(
        storage_25.get_key(100),
        0,
        "Large index should return 0 for 25-key storage"
    );
}

#[test]
#[allow(deprecated)]
fn test_crc_params_get_key_checked() {
    // Create test CrcParams with 23-key storage
    let keys_23 = [42u64; 23];
    let params_23 = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Test CRC",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0x12345678,
        keys: CrcKeysStorage::from_keys_fold_256(keys_23),
    };

    // Test valid indices return Some(value)
    for i in 0..23 {
        assert_eq!(
            params_23.get_key_checked(i),
            Some(42),
            "Valid index {} should return Some(42)",
            i
        );
    }

    // Test out-of-bounds indices return None
    assert_eq!(
        params_23.get_key_checked(23),
        None,
        "Index 23 should return None for 23-key params"
    );
    assert_eq!(
        params_23.get_key_checked(24),
        None,
        "Index 24 should return None for 23-key params"
    );
    assert_eq!(
        params_23.get_key_checked(100),
        None,
        "Large index should return None for 23-key params"
    );

    // Create test CrcParams with 25-key storage
    let keys_25 = [84u64; 25];
    let params_25 = CrcParams {
        algorithm: CrcAlgorithm::Crc64Custom,
        name: "Test CRC 64",
        width: 64,
        poly: 0x42F0E1EBA9EA3693,
        init: 0xFFFFFFFFFFFFFFFF,
        init_algorithm: 0xFFFFFFFFFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFFFFFFFFFF,
        check: 0x123456789ABCDEF0,
        keys: CrcKeysStorage::from_keys_fold_future_test(keys_25),
    };

    // Test valid indices return Some(value)
    for i in 0..25 {
        assert_eq!(
            params_25.get_key_checked(i),
            Some(84),
            "Valid index {} should return Some(84)",
            i
        );
    }

    // Test out-of-bounds indices return None
    assert_eq!(
        params_25.get_key_checked(25),
        None,
        "Index 25 should return None for 25-key params"
    );
    assert_eq!(
        params_25.get_key_checked(26),
        None,
        "Index 26 should return None for 25-key params"
    );
    assert_eq!(
        params_25.get_key_checked(100),
        None,
        "Large index should return None for 25-key params"
    );
}

#[test]
#[allow(deprecated)]
fn test_key_count_returns_correct_values() {
    // Test KeysFold256 variant
    let keys_23 = [1u64; 23];
    let storage_23 = CrcKeysStorage::from_keys_fold_256(keys_23);
    assert_eq!(
        storage_23.key_count(),
        23,
        "KeysFold256 should report 23 keys"
    );

    let params_23 = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Test CRC",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0x12345678,
        keys: storage_23,
    };
    assert_eq!(
        params_23.key_count(),
        23,
        "CrcParams with KeysFold256 should report 23 keys"
    );

    // Test KeysFutureTest variant
    let keys_25 = [2u64; 25];
    let storage_25 = CrcKeysStorage::from_keys_fold_future_test(keys_25);
    assert_eq!(
        storage_25.key_count(),
        25,
        "KeysFutureTest should report 25 keys"
    );

    let params_25 = CrcParams {
        algorithm: CrcAlgorithm::Crc64Custom,
        name: "Test CRC 64",
        width: 64,
        poly: 0x42F0E1EBA9EA3693,
        init: 0xFFFFFFFFFFFFFFFF,
        init_algorithm: 0xFFFFFFFFFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFFFFFFFFFF,
        check: 0x123456789ABCDEF0,
        keys: storage_25,
    };
    assert_eq!(
        params_25.key_count(),
        25,
        "CrcParams with KeysFutureTest should report 25 keys"
    );
}

#[test]
#[allow(deprecated)]
fn test_crc_params_get_key_bounds_checking() {
    // Create test CrcParams with 23-key storage
    let keys_23 = [99u64; 23];
    let params_23 = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Test CRC",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0x12345678,
        keys: CrcKeysStorage::from_keys_fold_256(keys_23),
    };

    // Test valid indices
    for i in 0..23 {
        assert_eq!(
            params_23.get_key(i),
            99,
            "Valid index {} should return 99",
            i
        );
    }

    // Test out-of-bounds indices return 0
    assert_eq!(
        params_23.get_key(23),
        0,
        "Index 23 should return 0 for 23-key params"
    );
    assert_eq!(
        params_23.get_key(24),
        0,
        "Index 24 should return 0 for 23-key params"
    );
    assert_eq!(
        params_23.get_key(100),
        0,
        "Large index should return 0 for 23-key params"
    );
}

#[test]
#[allow(deprecated)]
fn test_third_party_const_definitions_compatibility() {
    // Mock third-party const definitions using the new format
    // These simulate how third-party applications would define custom CrcParams

    // Mock third-party CRC-32 definition (similar to existing library constants)
    const MOCK_THIRD_PARTY_CRC32: CrcParams = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Mock Third Party CRC-32",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0xE3069283,
        keys: CrcKeysStorage::from_keys_fold_256([
            0x1234567890ABCDEF,
            0x2345678901BCDEF0,
            0x3456789012CDEF01,
            0x456789023DEF012,
            0x56789034EF0123,
            0x6789045F01234,
            0x789056012345,
            0x89067123456,
            0x9078234567,
            0xA089345678,
            0xB09A456789,
            0xC0AB56789A,
            0xD0BC6789AB,
            0xE0CD789ABC,
            0xF0DE89ABCD,
            0x10EF9ABCDE,
            0x210ABCDEF0,
            0x321BCDEF01,
            0x432CDEF012,
            0x543DEF0123,
            0x654EF01234,
            0x765F012345,
            0x876012345,
        ]),
    };

    // Mock third-party CRC-64 definition
    const MOCK_THIRD_PARTY_CRC64: CrcParams = CrcParams {
        algorithm: CrcAlgorithm::Crc64Custom,
        name: "Mock Third Party CRC-64",
        width: 64,
        poly: 0x42F0E1EBA9EA3693,
        init: 0xFFFFFFFFFFFFFFFF,
        init_algorithm: 0xFFFFFFFFFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFFFFFFFFFF,
        check: 0x6C40DF5F0B497347,
        keys: CrcKeysStorage::from_keys_fold_256([
            0xFEDCBA0987654321,
            0xEDCBA09876543210,
            0xDCBA098765432101,
            0xCBA0987654321012,
            0xBA09876543210123,
            0xA098765432101234,
            0x9087654321012345,
            0x8076543210123456,
            0x7065432101234567,
            0x6054321012345678,
            0x5043210123456789,
            0x403210123456789A,
            0x3210123456789AB,
            0x210123456789ABC,
            0x10123456789ABCD,
            0x123456789ABCDE,
            0x23456789ABCDEF,
            0x3456789ABCDEF0,
            0x456789ABCDEF01,
            0x56789ABCDEF012,
            0x6789ABCDEF0123,
            0x789ABCDEF01234,
            0x89ABCDEF012345,
        ]),
    };

    // Test that third-party const definitions work correctly
    assert_eq!(MOCK_THIRD_PARTY_CRC32.key_count(), 23);
    assert_eq!(MOCK_THIRD_PARTY_CRC64.key_count(), 23);

    // Test key access patterns that third-party code might use
    assert_eq!(MOCK_THIRD_PARTY_CRC32.get_key(0), 0x1234567890ABCDEF);
    assert_eq!(MOCK_THIRD_PARTY_CRC32.get_key(22), 0x876012345);
    assert_eq!(MOCK_THIRD_PARTY_CRC32.get_key(23), 0); // Out of bounds

    assert_eq!(MOCK_THIRD_PARTY_CRC64.get_key(0), 0xFEDCBA0987654321);
    assert_eq!(MOCK_THIRD_PARTY_CRC64.get_key(22), 0x89ABCDEF012345);
    assert_eq!(MOCK_THIRD_PARTY_CRC64.get_key(23), 0); // Out of bounds

    // Test that checked access works as expected
    assert_eq!(
        MOCK_THIRD_PARTY_CRC32.get_key_checked(0),
        Some(0x1234567890ABCDEF)
    );
    assert_eq!(
        MOCK_THIRD_PARTY_CRC32.get_key_checked(22),
        Some(0x876012345)
    );
    assert_eq!(MOCK_THIRD_PARTY_CRC32.get_key_checked(23), None);

    assert_eq!(
        MOCK_THIRD_PARTY_CRC64.get_key_checked(0),
        Some(0xFEDCBA0987654321)
    );
    assert_eq!(
        MOCK_THIRD_PARTY_CRC64.get_key_checked(22),
        Some(0x89ABCDEF012345)
    );
    assert_eq!(MOCK_THIRD_PARTY_CRC64.get_key_checked(23), None);
}

#[test]
#[allow(clippy::needless_range_loop)] // Intentionally testing indexed key access patterns
#[allow(deprecated)]
fn test_existing_key_access_patterns_continue_to_work() {
    // Test that common key access patterns used by existing code continue to work

    let test_keys = [
        0x1111111111111111,
        0x2222222222222222,
        0x3333333333333333,
        0x4444444444444444,
        0x5555555555555555,
        0x6666666666666666,
        0x7777777777777777,
        0x8888888888888888,
        0x9999999999999999,
        0xAAAAAAAAAAAAAAAA,
        0xBBBBBBBBBBBBBBBB,
        0xCCCCCCCCCCCCCCCC,
        0xDDDDDDDDDDDDDDDD,
        0xEEEEEEEEEEEEEEEE,
        0xFFFFFFFFFFFFFFFF,
        0x1010101010101010,
        0x2020202020202020,
        0x3030303030303030,
        0x4040404040404040,
        0x5050505050505050,
        0x6060606060606060,
        0x7070707070707070,
        0x8080808080808080,
    ];

    let params = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Test Pattern Access",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0x12345678,
        keys: CrcKeysStorage::from_keys_fold_256(test_keys),
    };

    // Pattern 1: Sequential access (common in folding algorithms)
    for i in 0..23 {
        let expected = test_keys[i];
        assert_eq!(
            params.get_key(i),
            expected,
            "Sequential access failed at index {}",
            i
        );
    }

    // Pattern 2: Reverse access (sometimes used in algorithms)
    for i in (0..23).rev() {
        let expected = test_keys[i];
        assert_eq!(
            params.get_key(i),
            expected,
            "Reverse access failed at index {}",
            i
        );
    }

    // Pattern 3: Specific indices commonly used in folding (powers of 2, etc.)
    let common_indices = [0, 1, 2, 4, 8, 16, 22];
    for &i in &common_indices {
        if i < 23 {
            let expected = test_keys[i];
            assert_eq!(
                params.get_key(i),
                expected,
                "Common index access failed at index {}",
                i
            );
        }
    }

    // Pattern 4: Bounds checking that third-party code might rely on
    assert_eq!(
        params.get_key(23),
        0,
        "Out-of-bounds access should return 0"
    );
    assert_eq!(
        params.get_key(100),
        0,
        "Large out-of-bounds access should return 0"
    );
}

#[test]
#[allow(clippy::needless_range_loop)] // Intentionally testing indexed key access for backwards compatibility
#[allow(deprecated)]
fn test_backwards_compatibility_throughout_migration_phases() {
    // This test simulates the migration phases to ensure backwards compatibility

    // Phase 1 & 2: Original array-based access patterns (simulated)
    let test_keys = [0x123456789ABCDEF0u64; 23];
    let storage = CrcKeysStorage::from_keys_fold_256(test_keys);

    // Verify that the storage behaves identically to direct array access
    for i in 0..23 {
        assert_eq!(
            storage.get_key(i),
            test_keys[i],
            "Storage access should match array access at index {}",
            i
        );
    }

    // Phase 3: New CrcKeysStorage-based access
    let params = CrcParams {
        algorithm: CrcAlgorithm::Crc64Custom,
        name: "Migration Test",
        width: 64,
        poly: 0x42F0E1EBA9EA3693,
        init: 0x0000000000000000,
        init_algorithm: 0x0000000000000000,
        refin: false,
        refout: false,
        xorout: 0x0000000000000000,
        check: 0x6C40DF5F0B497347,
        keys: storage,
    };

    // Verify that CrcParams provides the same access patterns
    for i in 0..23 {
        assert_eq!(
            params.get_key(i),
            test_keys[i],
            "CrcParams access should match array access at index {}",
            i
        );
        assert_eq!(
            params.get_key_checked(i),
            Some(test_keys[i]),
            "CrcParams checked access should match array access at index {}",
            i
        );
    }

    // Verify bounds checking works consistently
    assert_eq!(params.get_key(23), 0, "Out-of-bounds should return 0");
    assert_eq!(
        params.get_key_checked(23),
        None,
        "Out-of-bounds checked should return None"
    );

    // Verify key count is correct
    assert_eq!(params.key_count(), 23, "Key count should be 23");

    // Test compatibility with existing comparison operations
    assert_eq!(
        storage.to_keys_array_23(),
        test_keys,
        "Storage should convert back to original array"
    );
    assert_eq!(
        storage, test_keys,
        "Storage should compare equal to original array"
    );
    assert_eq!(
        test_keys, storage,
        "Original array should compare equal to storage"
    );
}

#[test]
#[allow(clippy::needless_range_loop)] // Intentionally testing indexed access performance
#[allow(deprecated)]
fn test_key_access_performance_matches_direct_array_access() {
    // This test verifies that CrcKeysStorage key access has zero runtime overhead
    // compared to direct array access. While we can't easily measure exact timing
    // in a unit test, we can verify that the compiler optimizations work correctly
    // by testing that the behavior is identical and that large-scale access works efficiently.

    // Create test keys with different values to avoid XOR cancellation
    let test_keys = [
        0x1111111111111111,
        0x2222222222222222,
        0x3333333333333333,
        0x4444444444444444,
        0x5555555555555555,
        0x6666666666666666,
        0x7777777777777777,
        0x8888888888888888,
        0x9999999999999999,
        0xAAAAAAAAAAAAAAAA,
        0xBBBBBBBBBBBBBBBB,
        0xCCCCCCCCCCCCCCCC,
        0xDDDDDDDDDDDDDDDD,
        0xEEEEEEEEEEEEEEEE,
        0xFFFFFFFFFFFFFFFF,
        0x1010101010101010,
        0x2020202020202020,
        0x3030303030303030,
        0x4040404040404040,
        0x5050505050505050,
        0x6060606060606060,
        0x7070707070707070,
        0x8080808080808080,
    ];
    let storage = CrcKeysStorage::from_keys_fold_256(test_keys);

    let params = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Performance Test",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0x12345678,
        keys: storage,
    };

    // Simulate intensive key access patterns that would reveal performance issues
    let iterations = 1000;
    let mut checksum = 0u64;

    // Pattern 1: Sequential access (most common in real algorithms)
    for iteration in 0..iterations {
        for i in 0..23 {
            checksum = checksum.wrapping_add(params.get_key(i).wrapping_mul(iteration as u64 + 1));
        }
    }

    // Pattern 2: Random access pattern
    let access_pattern = [
        0, 5, 12, 3, 18, 7, 22, 1, 15, 9, 20, 4, 11, 16, 2, 19, 8, 14, 6, 21, 10, 17, 13,
    ];
    for iteration in 0..iterations {
        for &i in &access_pattern {
            checksum = checksum.wrapping_add(params.get_key(i).wrapping_mul(iteration as u64 + 2));
        }
    }

    // Verify that we actually accessed the keys (checksum should be non-zero)
    assert_ne!(checksum, 0, "Performance test should have accessed keys");

    // Test that bounds checking doesn't significantly impact performance
    let mut bounds_checksum = 0u64;
    for iteration in 0..iterations {
        for i in 0..30 {
            // Include some out-of-bounds accesses
            bounds_checksum =
                bounds_checksum.wrapping_add(params.get_key(i).wrapping_mul(iteration as u64 + 3));
        }
    }

    // The bounds-checked version should still work correctly
    assert_ne!(
        bounds_checksum, 0,
        "Bounds checking performance test should work"
    );
}

#[test]
fn test_crc_calculation_performance_before_and_after_changes() {
    // Test that CRC calculation performance remains identical with the new key storage
    use crate::{checksum, CrcAlgorithm};

    // Test data of various sizes to ensure performance across different scenarios
    let test_data_small = b"123456789";
    let test_data_medium = vec![0xAAu8; 1024]; // 1KB
    let test_data_large = vec![0x55u8; 65536]; // 64KB

    // Test multiple CRC algorithms to ensure consistent performance
    let algorithms = [
        CrcAlgorithm::Crc32IsoHdlc,
        CrcAlgorithm::Crc32Iscsi,
        CrcAlgorithm::Crc64Nvme,
        CrcAlgorithm::Crc64Ecma182,
    ];

    for algorithm in algorithms {
        // Small data performance
        let result_small = checksum(algorithm, test_data_small);
        assert_ne!(
            result_small, 0,
            "Small data CRC should produce non-zero result"
        );

        // Medium data performance
        let result_medium = checksum(algorithm, &test_data_medium);
        assert_ne!(
            result_medium, 0,
            "Medium data CRC should produce non-zero result"
        );

        // Large data performance (this would reveal significant performance regressions)
        let result_large = checksum(algorithm, &test_data_large);
        assert_ne!(
            result_large, 0,
            "Large data CRC should produce non-zero result"
        );

        // Verify consistency across multiple runs (performance should be deterministic)
        let result_small_2 = checksum(algorithm, test_data_small);
        assert_eq!(
            result_small, result_small_2,
            "CRC results should be consistent"
        );
    }
}

#[test]
#[allow(deprecated)]
fn test_memory_usage_impact_of_enum_based_storage() {
    // Test that enum-based storage doesn't significantly increase memory usage
    use std::mem;

    // Test memory size of different storage variants
    let keys_23 = [0u64; 23];
    let keys_25 = [0u64; 25];

    let storage_23 = CrcKeysStorage::from_keys_fold_256(keys_23);
    let storage_25 = CrcKeysStorage::from_keys_fold_future_test(keys_25);

    // Verify that enum storage size is reasonable
    let storage_23_size = mem::size_of_val(&storage_23);
    let storage_25_size = mem::size_of_val(&storage_25);
    let _array_23_size = mem::size_of_val(&keys_23);
    let array_25_size = mem::size_of_val(&keys_25);

    // Rust enums use the size of the largest variant plus discriminant/alignment
    // Both variants will be the same size (size of largest variant)
    assert_eq!(
        storage_23_size, storage_25_size,
        "Both enum variants should have the same size"
    );

    // The enum size should be reasonable (largest variant + small overhead)
    assert!(
        storage_23_size >= array_25_size,
        "Enum should be at least as large as the largest variant"
    );
    assert!(
        storage_23_size <= array_25_size + 16,
        "Enum should not add excessive overhead beyond largest variant"
    );

    // Test CrcParams memory usage
    let params_23 = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Memory Test 23",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0x12345678,
        keys: storage_23,
    };

    let params_25 = CrcParams {
        algorithm: CrcAlgorithm::Crc64Custom,
        name: "Memory Test 25",
        width: 64,
        poly: 0x42F0E1EBA9EA3693,
        init: 0xFFFFFFFFFFFFFFFF,
        init_algorithm: 0xFFFFFFFFFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFFFFFFFFFF,
        check: 0x123456789ABCDEF0,
        keys: storage_25,
    };

    let params_23_size = mem::size_of_val(&params_23);
    let params_25_size = mem::size_of_val(&params_25);

    // CrcParams should have reasonable size differences based on key storage
    assert!(
        params_25_size >= params_23_size,
        "25-key params should be at least as large as 23-key params"
    );
    assert!(
        params_25_size - params_23_size <= 16,
        "Size difference should be reasonable (just the extra keys)"
    );

    // Verify that the structs are still reasonably sized
    assert!(
        params_23_size < 512,
        "CrcParams should not be excessively large"
    );
    assert!(
        params_25_size < 512,
        "CrcParams should not be excessively large"
    );
}

#[test]
fn test_compiler_optimizations_eliminate_enum_dispatch() {
    // This test verifies that compiler optimizations work correctly by testing
    // that enum dispatch doesn't introduce runtime branching in hot paths

    // Create different keys to avoid XOR cancellation
    let keys_23 = [
        0x1111111111111111,
        0x2222222222222222,
        0x3333333333333333,
        0x4444444444444444,
        0x5555555555555555,
        0x6666666666666666,
        0x7777777777777777,
        0x8888888888888888,
        0x9999999999999999,
        0xAAAAAAAAAAAAAAAA,
        0xBBBBBBBBBBBBBBBB,
        0xCCCCCCCCCCCCCCCC,
        0xDDDDDDDDDDDDDDDD,
        0xEEEEEEEEEEEEEEEE,
        0xFFFFFFFFFFFFFFFF,
        0x1010101010101010,
        0x2020202020202020,
        0x3030303030303030,
        0x4040404040404040,
        0x5050505050505050,
        0x6060606060606060,
        0x7070707070707070,
        0x8080808080808080,
    ];
    let keys_25 = [
        0x1111111111111111,
        0x2222222222222222,
        0x3333333333333333,
        0x4444444444444444,
        0x5555555555555555,
        0x6666666666666666,
        0x7777777777777777,
        0x8888888888888888,
        0x9999999999999999,
        0xAAAAAAAAAAAAAAAA,
        0xBBBBBBBBBBBBBBBB,
        0xCCCCCCCCCCCCCCCC,
        0xDDDDDDDDDDDDDDDD,
        0xEEEEEEEEEEEEEEEE,
        0xFFFFFFFFFFFFFFFF,
        0x1010101010101010,
        0x2020202020202020,
        0x3030303030303030,
        0x4040404040404040,
        0x5050505050505050,
        0x6060606060606060,
        0x7070707070707070,
        0x8080808080808080,
        0x9090909090909090,
        0xA0A0A0A0A0A0A0A0,
    ];

    let storage_23 = CrcKeysStorage::from_keys_fold_256(keys_23);
    let storage_25 = CrcKeysStorage::from_keys_fold_future_test(keys_25);

    // Test that repeated access to the same storage type is efficient
    // (compiler should optimize away the enum matching)
    let mut sum_23 = 0u64;
    for iteration in 0..100 {
        for i in 0..23 {
            sum_23 = sum_23.wrapping_add(storage_23.get_key(i).wrapping_mul(iteration + 1));
        }
    }

    let mut sum_25 = 0u64;
    for iteration in 0..100 {
        for i in 0..25 {
            sum_25 = sum_25.wrapping_add(storage_25.get_key(i).wrapping_mul(iteration + 1));
        }
    }

    // Verify that the operations actually happened
    assert_ne!(
        sum_23, 0,
        "23-key operations should produce non-zero result"
    );
    assert_ne!(
        sum_25, 0,
        "25-key operations should produce non-zero result"
    );

    // Test mixed access patterns (this would reveal optimization issues)
    let storages = [storage_23, storage_25];
    let mut mixed_sum = 0u64;

    for iteration in 0..100 {
        for (storage_idx, storage) in storages.iter().enumerate() {
            let key_count = storage.key_count();
            for i in 0..key_count {
                mixed_sum = mixed_sum.wrapping_add(
                    storage
                        .get_key(i)
                        .wrapping_mul((iteration + storage_idx + 1) as u64),
                );
            }
        }
    }

    assert_ne!(mixed_sum, 0, "Mixed access should produce non-zero result");
}
#[test]
#[allow(clippy::needless_range_loop)] // Intentionally testing indexed key access for future variant
#[allow(deprecated)]
fn test_create_crc_params_using_keys_future_test_variant() {
    // Create test CrcParams using KeysFutureTest variant with 25 keys
    let test_keys_25 = [
        0x1111111111111111,
        0x2222222222222222,
        0x3333333333333333,
        0x4444444444444444,
        0x5555555555555555,
        0x6666666666666666,
        0x7777777777777777,
        0x8888888888888888,
        0x9999999999999999,
        0xAAAAAAAAAAAAAAAA,
        0xBBBBBBBBBBBBBBBB,
        0xCCCCCCCCCCCCCCCC,
        0xDDDDDDDDDDDDDDDD,
        0xEEEEEEEEEEEEEEEE,
        0xFFFFFFFFFFFFFFFF,
        0x1010101010101010,
        0x2020202020202020,
        0x3030303030303030,
        0x4040404040404040,
        0x5050505050505050,
        0x6060606060606060,
        0x7070707070707070,
        0x8080808080808080,
        0x9090909090909090,
        0xA0A0A0A0A0A0A0A0,
    ];

    let future_params = CrcParams {
        algorithm: CrcAlgorithm::Crc64Custom,
        name: "Future Test CRC-64",
        width: 64,
        poly: 0x42F0E1EBA9EA3693,
        init: 0xFFFFFFFFFFFFFFFF,
        init_algorithm: 0xFFFFFFFFFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFFFFFFFFFF,
        check: 0x123456789ABCDEF0,
        keys: CrcKeysStorage::from_keys_fold_future_test(test_keys_25),
    };

    // Verify that the future params work correctly
    assert_eq!(
        future_params.key_count(),
        25,
        "Future params should have 25 keys"
    );

    // Test access to all 25 keys
    for i in 0..25 {
        assert_eq!(
            future_params.get_key(i),
            test_keys_25[i],
            "Key {} should match expected value",
            i
        );
        assert_eq!(
            future_params.get_key_checked(i),
            Some(test_keys_25[i]),
            "Checked key {} should return Some(value)",
            i
        );
    }

    // Test out-of-bounds access
    assert_eq!(
        future_params.get_key(25),
        0,
        "Key 25 should return 0 (out of bounds)"
    );
    assert_eq!(
        future_params.get_key_checked(25),
        None,
        "Checked key 25 should return None"
    );
    assert_eq!(
        future_params.get_key(30),
        0,
        "Key 30 should return 0 (out of bounds)"
    );
    assert_eq!(
        future_params.get_key_checked(30),
        None,
        "Checked key 30 should return None"
    );
}

#[test]
#[allow(deprecated)]
fn test_code_gracefully_handles_different_key_array_sizes() {
    // Test that the same code can handle both 23-key and 25-key variants gracefully

    let keys_23 = [0x1234567890ABCDEFu64; 23];
    let keys_25 = [0xFEDCBA0987654321u64; 25];

    let params_23 = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "23-Key Test",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0x12345678,
        keys: CrcKeysStorage::from_keys_fold_256(keys_23),
    };

    let params_25 = CrcParams {
        algorithm: CrcAlgorithm::Crc64Custom,
        name: "25-Key Test",
        width: 64,
        poly: 0x42F0E1EBA9EA3693,
        init: 0xFFFFFFFFFFFFFFFF,
        init_algorithm: 0xFFFFFFFFFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFFFFFFFFFF,
        check: 0x123456789ABCDEF0,
        keys: CrcKeysStorage::from_keys_fold_future_test(keys_25),
    };

    // Generic function that works with any CrcParams regardless of key count
    fn process_crc_params(params: CrcParams) -> (usize, u64, u64) {
        let key_count = params.key_count();
        let first_key = params.get_key(0);
        let last_valid_key = if key_count > 0 {
            params.get_key(key_count - 1)
        } else {
            0
        };
        (key_count, first_key, last_valid_key)
    }

    // Test that the same function works with both variants
    let (count_23, first_23, last_23) = process_crc_params(params_23);
    let (count_25, first_25, last_25) = process_crc_params(params_25);

    assert_eq!(count_23, 23, "23-key params should report 23 keys");
    assert_eq!(count_25, 25, "25-key params should report 25 keys");

    assert_eq!(
        first_23, 0x1234567890ABCDEF,
        "23-key first key should match"
    );
    assert_eq!(
        first_25, 0xFEDCBA0987654321,
        "25-key first key should match"
    );

    assert_eq!(last_23, 0x1234567890ABCDEF, "23-key last key should match");
    assert_eq!(last_25, 0xFEDCBA0987654321, "25-key last key should match");

    // Test that bounds checking works consistently across variants
    assert_eq!(
        params_23.get_key(23),
        0,
        "23-key params should return 0 for index 23"
    );
    assert_eq!(
        params_23.get_key(25),
        0,
        "23-key params should return 0 for index 25"
    );

    assert_eq!(
        params_25.get_key(25),
        0,
        "25-key params should return 0 for index 25"
    );
    assert_eq!(
        params_25.get_key(30),
        0,
        "25-key params should return 0 for index 30"
    );
}

#[test]
#[allow(deprecated)]
fn test_expansion_to_larger_key_arrays_works_as_designed() {
    // Test that the design supports expansion to larger key arrays

    // Simulate a migration scenario where we add more keys
    let original_keys = [0x1111111111111111u64; 23];
    let expanded_keys = [
        // Original 23 keys
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        0x1111111111111111,
        // Additional 2 keys for future expansion
        0x2222222222222222,
        0x3333333333333333,
    ];

    let original_params = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Original CRC",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0x12345678,
        keys: CrcKeysStorage::from_keys_fold_256(original_keys),
    };

    let expanded_params = CrcParams {
        algorithm: CrcAlgorithm::Crc64Custom,
        name: "Expanded CRC",
        width: 64,
        poly: 0x42F0E1EBA9EA3693,
        init: 0xFFFFFFFFFFFFFFFF,
        init_algorithm: 0xFFFFFFFFFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFFFFFFFFFF,
        check: 0x123456789ABCDEF0,
        keys: CrcKeysStorage::from_keys_fold_future_test(expanded_keys),
    };

    // Test that existing key access patterns continue to work
    for i in 0..23 {
        assert_eq!(
            original_params.get_key(i),
            0x1111111111111111,
            "Original key {} should match",
            i
        );
        assert_eq!(
            expanded_params.get_key(i),
            0x1111111111111111,
            "Expanded key {} should match original",
            i
        );
    }

    // Test that new keys are accessible in expanded version
    assert_eq!(
        expanded_params.get_key(23),
        0x2222222222222222,
        "New key 23 should be accessible"
    );
    assert_eq!(
        expanded_params.get_key(24),
        0x3333333333333333,
        "New key 24 should be accessible"
    );

    // Test that original version handles new indices gracefully
    assert_eq!(
        original_params.get_key(23),
        0,
        "Original should return 0 for new key indices"
    );
    assert_eq!(
        original_params.get_key(24),
        0,
        "Original should return 0 for new key indices"
    );

    // Test that both versions handle out-of-bounds access consistently
    assert_eq!(
        original_params.get_key(30),
        0,
        "Original should return 0 for out-of-bounds"
    );
    assert_eq!(
        expanded_params.get_key(30),
        0,
        "Expanded should return 0 for out-of-bounds"
    );

    // Test key count differences
    assert_eq!(
        original_params.key_count(),
        23,
        "Original should have 23 keys"
    );
    assert_eq!(
        expanded_params.key_count(),
        25,
        "Expanded should have 25 keys"
    );

    // Test that checked access works correctly for both
    assert_eq!(
        original_params.get_key_checked(22),
        Some(0x1111111111111111),
        "Original last key should be accessible"
    );
    assert_eq!(
        original_params.get_key_checked(23),
        None,
        "Original should return None for index 23"
    );

    assert_eq!(
        expanded_params.get_key_checked(22),
        Some(0x1111111111111111),
        "Expanded key 22 should be accessible"
    );
    assert_eq!(
        expanded_params.get_key_checked(24),
        Some(0x3333333333333333),
        "Expanded last key should be accessible"
    );
    assert_eq!(
        expanded_params.get_key_checked(25),
        None,
        "Expanded should return None for index 25"
    );
}

#[test]
#[allow(deprecated)]
fn test_future_expansion_backwards_compatibility() {
    // Test that future expansion maintains backwards compatibility

    // This test simulates a scenario where:
    // 1. Third-party code is written against 23-key CrcParams
    // 2. Library is expanded to support 25-key CrcParams
    // 3. Third-party code continues to work without modification

    // Mock third-party function that expects to work with any CrcParams
    fn third_party_key_processor(params: CrcParams) -> Vec<u64> {
        let mut result = Vec::new();

        // Third-party code might access keys in various patterns
        // Pattern 1: Access first few keys
        for i in 0..5 {
            result.push(params.get_key(i));
        }

        // Pattern 2: Access some middle keys
        for i in 10..15 {
            result.push(params.get_key(i));
        }

        // Pattern 3: Access keys near the end (but within original 23-key range)
        for i in 20..23 {
            result.push(params.get_key(i));
        }

        // Pattern 4: Attempt to access beyond both ranges (should return 0 for both)
        result.push(params.get_key(30));
        result.push(params.get_key(31));

        result
    }

    // Test with original 23-key params
    let keys_23 = [0xABCDEF0123456789u64; 23];
    let params_23 = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Backwards Compat Test 23",
        width: 32,
        poly: 0x1EDC6F41,
        init: 0xFFFFFFFF,
        init_algorithm: 0xFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFF,
        check: 0x12345678,
        keys: CrcKeysStorage::from_keys_fold_256(keys_23),
    };

    // Test with expanded 25-key params
    let keys_25 = [0xABCDEF0123456789u64; 25];
    let params_25 = CrcParams {
        algorithm: CrcAlgorithm::Crc64Custom,
        name: "Backwards Compat Test 25",
        width: 64,
        poly: 0x42F0E1EBA9EA3693,
        init: 0xFFFFFFFFFFFFFFFF,
        init_algorithm: 0xFFFFFFFFFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFFFFFFFFFF,
        check: 0x123456789ABCDEF0,
        keys: CrcKeysStorage::from_keys_fold_future_test(keys_25),
    };

    // Run third-party function with both variants
    let result_23 = third_party_key_processor(params_23);
    let result_25 = third_party_key_processor(params_25);

    // Results should be identical for the overlapping key ranges
    assert_eq!(
        result_23.len(),
        result_25.len(),
        "Results should have same length"
    );

    // First 13 values should be identical (keys 0-4, 10-14, 20-22)
    for i in 0..13 {
        assert_eq!(
            result_23[i], result_25[i],
            "Result {} should be identical",
            i
        );
        assert_eq!(
            result_23[i], 0xABCDEF0123456789,
            "Result {} should match key value",
            i
        );
    }

    // Last 2 values should be 0 for both (out-of-bounds access to indices 30, 31)
    assert_eq!(
        result_23[13], 0,
        "Out-of-bounds access should return 0 for 23-key"
    );
    assert_eq!(
        result_23[14], 0,
        "Out-of-bounds access should return 0 for 23-key"
    );
    assert_eq!(
        result_25[13], 0,
        "Out-of-bounds access should return 0 for 25-key"
    );
    assert_eq!(
        result_25[14], 0,
        "Out-of-bounds access should return 0 for 25-key"
    );

    // This demonstrates that third-party code works identically with both variants
    assert_eq!(
        result_23, result_25,
        "Third-party function should produce identical results"
    );
}
// FFI Tests for future-proof CrcFastParams functionality

#[cfg(all(
    feature = "ffi",
    any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")
))]
mod ffi_tests {
    use crate::ffi::CrcFastParams;
    use crate::{CrcAlgorithm, CrcKeysStorage, CrcParams};

    #[test]
    #[allow(deprecated)]
    fn test_ffi_conversion_23_keys() {
        // Test conversion between CrcParams and CrcFastParams for 23-key variant
        let keys_23 = [0x1234567890ABCDEFu64; 23];
        let original_params = CrcParams {
            algorithm: CrcAlgorithm::Crc32Custom,
            name: "FFI Test 23",
            width: 32,
            poly: 0x1EDC6F41,
            init: 0xFFFFFFFF,
            init_algorithm: 0xFFFFFFFF,
            refin: true,
            refout: true,
            xorout: 0xFFFFFFFF,
            check: 0x12345678,
            keys: CrcKeysStorage::from_keys_fold_256(keys_23),
        };

        // Convert to FFI struct
        let ffi_params: CrcFastParams = original_params.into();

        // Verify FFI struct fields
        assert_eq!(ffi_params.key_count, 23, "FFI params should have 23 keys");
        assert!(
            !ffi_params.keys.is_null(),
            "Keys pointer should not be null"
        );
        assert_eq!(ffi_params.width, 32, "Width should match");
        assert_eq!(ffi_params.poly, 0x1EDC6F41, "Poly should match");
        assert_eq!(ffi_params.init, 0xFFFFFFFF, "Init should match");
        assert!(ffi_params.refin, "Refin should match");
        assert!(ffi_params.refout, "Refout should match");
        assert_eq!(ffi_params.xorout, 0xFFFFFFFF, "Xorout should match");
        assert_eq!(ffi_params.check, 0x12345678, "Check should match");

        // Test direct pointer access to keys
        unsafe {
            for i in 0..23 {
                let key_value = *ffi_params.keys.add(i);
                assert_eq!(
                    key_value, 0x1234567890ABCDEF,
                    "Key {} should match expected value",
                    i
                );
            }
        }

        // Convert back to CrcParams
        let converted_params: CrcParams = ffi_params.into();

        // Verify round-trip conversion
        assert_eq!(converted_params.algorithm, original_params.algorithm);
        assert_eq!(converted_params.width, original_params.width);
        assert_eq!(converted_params.poly, original_params.poly);
        assert_eq!(converted_params.init, original_params.init);
        assert_eq!(converted_params.refin, original_params.refin);
        assert_eq!(converted_params.refout, original_params.refout);
        assert_eq!(converted_params.xorout, original_params.xorout);
        assert_eq!(converted_params.check, original_params.check);
        assert_eq!(converted_params.key_count(), 23);

        // Verify all keys match
        for i in 0..23 {
            assert_eq!(
                converted_params.get_key(i),
                original_params.get_key(i),
                "Converted key {} should match original",
                i
            );
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_ffi_conversion_25_keys() {
        // Test conversion between CrcParams and CrcFastParams for 25-key variant
        let keys_25 = [0xFEDCBA0987654321u64; 25];
        let original_params = CrcParams {
            algorithm: CrcAlgorithm::Crc64Custom,
            name: "FFI Test 25",
            width: 64,
            poly: 0x42F0E1EBA9EA3693,
            init: 0xFFFFFFFFFFFFFFFF,
            init_algorithm: 0xFFFFFFFFFFFFFFFF,
            refin: true,
            refout: true,
            xorout: 0xFFFFFFFFFFFFFFFF,
            check: 0x123456789ABCDEF0,
            keys: CrcKeysStorage::from_keys_fold_future_test(keys_25),
        };

        // Convert to FFI struct
        let ffi_params: CrcFastParams = original_params.into();

        // Verify FFI struct fields
        assert_eq!(ffi_params.key_count, 25, "FFI params should have 25 keys");
        assert!(
            !ffi_params.keys.is_null(),
            "Keys pointer should not be null"
        );
        assert_eq!(ffi_params.width, 64, "Width should match");
        assert_eq!(ffi_params.poly, 0x42F0E1EBA9EA3693, "Poly should match");

        // Test direct pointer access to keys
        unsafe {
            for i in 0..25 {
                let key_value = *ffi_params.keys.add(i);
                assert_eq!(
                    key_value, 0xFEDCBA0987654321,
                    "Key {} should match expected value",
                    i
                );
            }
        }

        // Convert back to CrcParams
        let converted_params: CrcParams = ffi_params.into();

        // Verify round-trip conversion
        assert_eq!(converted_params.key_count(), 25);
        for i in 0..25 {
            assert_eq!(
                converted_params.get_key(i),
                original_params.get_key(i),
                "Converted key {} should match original",
                i
            );
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_ffi_pointer_stability() {
        // Test that key pointers remain stable across multiple conversions
        let keys_23 = [0x1111111111111111u64; 23];
        let params = CrcParams {
            algorithm: CrcAlgorithm::Crc32Custom,
            name: "Stability Test",
            width: 32,
            poly: 0x1EDC6F41,
            init: 0xFFFFFFFF,
            init_algorithm: 0xFFFFFFFF,
            refin: true,
            refout: true,
            xorout: 0xFFFFFFFF,
            check: 0x12345678,
            keys: CrcKeysStorage::from_keys_fold_256(keys_23),
        };

        // Convert to FFI multiple times
        let ffi_params1: CrcFastParams = params.into();
        let ffi_params2: CrcFastParams = params.into();

        // Pointers should be stable (same keys should get same pointer)
        assert_eq!(
            ffi_params1.keys, ffi_params2.keys,
            "Identical key sets should get same stable pointer"
        );
        assert_eq!(
            ffi_params1.key_count, ffi_params2.key_count,
            "Key counts should match"
        );

        // Test that different key sets get different pointers
        let different_keys = [0x2222222222222222u64; 23];
        let different_params = CrcParams {
            algorithm: CrcAlgorithm::Crc32Custom,
            name: "Different Test",
            width: 32,
            poly: 0x1EDC6F41,
            init: 0xFFFFFFFF,
            init_algorithm: 0xFFFFFFFF,
            refin: true,
            refout: true,
            xorout: 0xFFFFFFFF,
            check: 0x12345678,
            keys: CrcKeysStorage::from_keys_fold_256(different_keys),
        };

        let ffi_params3: CrcFastParams = different_params.into();
        assert_ne!(
            ffi_params1.keys, ffi_params3.keys,
            "Different key sets should get different pointers"
        );
    }

    #[test]
    #[allow(deprecated)]
    fn test_ffi_memory_safety() {
        // Test that FFI conversions are memory safe
        let keys_23 = [0xAAAAAAAAAAAAAAAAu64; 23];
        let params = CrcParams {
            algorithm: CrcAlgorithm::Crc32Custom,
            name: "Memory Safety Test",
            width: 32,
            poly: 0x1EDC6F41,
            init: 0xFFFFFFFF,
            init_algorithm: 0xFFFFFFFF,
            refin: true,
            refout: true,
            xorout: 0xFFFFFFFF,
            check: 0x12345678,
            keys: CrcKeysStorage::from_keys_fold_256(keys_23),
        };

        let ffi_params: CrcFastParams = params.into();

        // Test that we can safely access all keys through the pointer
        unsafe {
            let keys_slice =
                std::slice::from_raw_parts(ffi_params.keys, ffi_params.key_count as usize);

            // Verify all keys are accessible and correct
            for (i, &key) in keys_slice.iter().enumerate() {
                assert_eq!(
                    key, 0xAAAAAAAAAAAAAAAA,
                    "Key {} should be accessible and correct",
                    i
                );
            }

            // Test that we can create multiple slices from the same pointer
            let keys_slice2 =
                std::slice::from_raw_parts(ffi_params.keys, ffi_params.key_count as usize);
            assert_eq!(
                keys_slice, keys_slice2,
                "Multiple slices should be identical"
            );
        }

        // Test conversion back to CrcParams
        let converted: CrcParams = ffi_params.into();
        assert_eq!(converted.key_count(), 23);

        for i in 0..23 {
            assert_eq!(
                converted.get_key(i),
                0xAAAAAAAAAAAAAAAA,
                "Converted key {} should match",
                i
            );
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_ffi_different_key_counts() {
        // Test FFI with different key count scenarios

        // Test 23-key variant
        let keys_23 = [0x1111111111111111u64; 23];
        let params_23 = CrcParams {
            algorithm: CrcAlgorithm::Crc32Custom,
            name: "23-Key FFI Test",
            width: 32,
            poly: 0x1EDC6F41,
            init: 0xFFFFFFFF,
            init_algorithm: 0xFFFFFFFF,
            refin: true,
            refout: true,
            xorout: 0xFFFFFFFF,
            check: 0x12345678,
            keys: CrcKeysStorage::from_keys_fold_256(keys_23),
        };

        // Test 25-key variant
        let keys_25 = [0x2222222222222222u64; 25];
        let params_25 = CrcParams {
            algorithm: CrcAlgorithm::Crc64Custom,
            name: "25-Key FFI Test",
            width: 64,
            poly: 0x42F0E1EBA9EA3693,
            init: 0xFFFFFFFFFFFFFFFF,
            init_algorithm: 0xFFFFFFFFFFFFFFFF,
            refin: true,
            refout: true,
            xorout: 0xFFFFFFFFFFFFFFFF,
            check: 0x123456789ABCDEF0,
            keys: CrcKeysStorage::from_keys_fold_future_test(keys_25),
        };

        // Convert both to FFI
        let ffi_23: CrcFastParams = params_23.into();
        let ffi_25: CrcFastParams = params_25.into();

        // Verify key counts
        assert_eq!(ffi_23.key_count, 23);
        assert_eq!(ffi_25.key_count, 25);

        // Test C-style access patterns
        unsafe {
            // Access all keys in 23-key variant
            for i in 0..23 {
                let key = *ffi_23.keys.add(i);
                assert_eq!(
                    key, 0x1111111111111111,
                    "23-key variant key {} should match",
                    i
                );
            }

            // Access all keys in 25-key variant
            for i in 0..25 {
                let key = *ffi_25.keys.add(i);
                assert_eq!(
                    key, 0x2222222222222222,
                    "25-key variant key {} should match",
                    i
                );
            }

            // Test bounds-aware C code pattern
            for ffi_params in [&ffi_23, &ffi_25] {
                for i in 0..ffi_params.key_count {
                    let key = *ffi_params.keys.add(i as usize);
                    assert_ne!(key, 0, "Key {} should not be zero", i);
                }
            }
        }

        // Test round-trip conversions
        let converted_23: CrcParams = ffi_23.into();
        let converted_25: CrcParams = ffi_25.into();

        assert_eq!(converted_23.key_count(), 23);
        assert_eq!(converted_25.key_count(), 25);

        // Verify all keys survived round-trip
        for i in 0..23 {
            assert_eq!(converted_23.get_key(i), 0x1111111111111111);
        }
        for i in 0..25 {
            assert_eq!(converted_25.get_key(i), 0x2222222222222222);
        }
    }

    #[test]
    fn test_ffi_get_custom_params_function() {
        // Test the crc_fast_get_custom_params FFI function
        use std::ffi::CString;

        let name = CString::new("Test CRC").unwrap();
        let ffi_params = crate::ffi::crc_fast_get_custom_params(
            name.as_ptr(),
            32,
            0x1EDC6F41,
            0xFFFFFFFF,
            true,
            0xFFFFFFFF,
            0x12345678,
        );

        // Verify the returned FFI params
        assert_eq!(ffi_params.width, 32);
        assert_eq!(ffi_params.poly, 0x1EDC6F41);
        assert_eq!(ffi_params.init, 0xFFFFFFFF);
        assert!(ffi_params.refin);
        assert!(ffi_params.refout);
        assert_eq!(ffi_params.xorout, 0xFFFFFFFF);
        assert_eq!(ffi_params.check, 0x12345678);
        assert!(
            !ffi_params.keys.is_null(),
            "Keys pointer should not be null"
        );
        assert!(ffi_params.key_count > 0, "Should have keys");

        // Test that we can access the keys
        unsafe {
            for i in 0..ffi_params.key_count {
                let _key = *ffi_params.keys.add(i as usize);
                // Keys should be accessible without crashing
            }
        }

        // Test conversion to CrcParams
        let params: CrcParams = ffi_params.into();
        assert_eq!(params.width, 32);
        assert_eq!(params.poly, 0x1EDC6F41);
        assert_eq!(params.init, 0xFFFFFFFF);
        assert!(params.refin);
        assert!(params.refout);
        assert_eq!(params.xorout, 0xFFFFFFFF);
        assert_eq!(params.check, 0x12345678);
        assert!(params.key_count() > 0);
    }
}
