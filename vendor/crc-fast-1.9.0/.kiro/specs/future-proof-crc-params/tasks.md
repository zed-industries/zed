# Implementation Plan

- [x] 1. Phase 1: Add CrcKeysStorage enum and helper methods
  - Add CrcKeysStorage enum with KeysFold256 and KeysFutureTest variants
  - Implement get_key() and key_count() methods on CrcKeysStorage
  - Add const constructor methods from_keys_fold_256() and from_keys_fold_future_test()
  - Add safe accessor methods to CrcParams that delegate to existing keys field
  - Write comprehensive unit tests for CrcKeysStorage functionality
  - _Requirements: 4.1, 4.2, 4.4, 5.1_

- [x] 2. Phase 2: Update architecture code to use safe accessors
  - [x] 2.1 Update SIMD folding code in src/arch/ to use params.get_key()
    - Replace direct keys[index] access with params.get_key(index) in algorithm.rs
    - Update VPCLMULQDQ code to use safe key access methods
    - Update aarch64 and x86 architecture-specific code
    - _Requirements: 3.1, 5.2_

  - [x] 2.2 Update CRC32 algorithm code to use safe accessors
    - Modify src/crc32/algorithm.rs to use params.get_key() instead of keys[index]
    - Update fusion code in src/crc32/fusion/ if it accesses keys directly
    - _Requirements: 3.1, 5.2_

  - [x] 2.3 Update CRC64 algorithm code to use safe accessors
    - Modify src/crc64/algorithm.rs to use params.get_key() instead of keys[index]
    - Update any other CRC64-specific code that accesses keys directly
    - _Requirements: 3.1, 5.2_

  - [x] 2.4 Run performance benchmarks to verify zero overhead
    - Benchmark key access performance before and after changes
    - Verify compiler optimizations eliminate any performance regression
    - Document that performance remains identical to direct array access
    - _Requirements: 2.2, 4.4_

- [x] 3. Phase 3: Switch CrcParams to use CrcKeysStorage
  - [x] 3.1 Update CrcParams struct definition
    - Change keys field from [u64; 23] to CrcKeysStorage
    - Update CrcParams accessor methods to delegate to CrcKeysStorage
    - Remove temporary delegation methods added in Phase 1
    - _Requirements: 5.3_

  - [x] 3.2 Update all CRC32 const definitions
    - Update src/crc32/consts.rs to use CrcKeysStorage::from_keys_fold_256()
    - Modify all CRC32_* const definitions to use new key storage format
    - Ensure all existing key arrays are properly wrapped
    - _Requirements: 1.2, 2.1_

  - [x] 3.3 Update all CRC64 const definitions
    - Update src/crc64/consts.rs to use CrcKeysStorage::from_keys_fold_256()
    - Modify all CRC64_* const definitions to use new key storage format
    - Ensure all existing key arrays are properly wrapped
    - _Requirements: 1.2, 2.1_

  - [x] 3.4 Update get-custom-params binary output
    - Modify src/bin/get-custom-params.rs to output CrcKeysStorage format
    - Update output template to use CrcKeysStorage::from_keys_fold_256()
    - Test that generated const definitions compile and work correctly
    - _Requirements: 6.1, 6.2, 6.3_

  - [x] 3.5 Update cache system for new CrcParams structure
    - Modify src/cache.rs to work with CrcKeysStorage-based CrcParams
    - Update CrcParams::new() method to use new key storage format
    - Ensure cache functionality remains intact after structural changes
    - _Requirements: 2.3, 5.3_

- [x] 4. Create comprehensive test suite for future-proof functionality
  - [x] 4.1 Add unit tests for bounds checking behavior
    - Test that get_key() returns 0 for out-of-bounds indices
    - Test that get_key_checked() returns None for invalid indices
    - Verify key_count() returns correct values for different storage variants
    - _Requirements: 3.2_

  - [x] 4.2 Add integration tests for third-party compatibility
    - Create mock third-party const definitions using new format
    - Test that existing key access patterns continue to work
    - Verify backwards compatibility throughout migration phases
    - _Requirements: 1.1, 2.3_

  - [x] 4.3 Add performance regression tests
    - Benchmark CRC calculation performance before and after changes
    - Verify that key access performance matches direct array access
    - Test memory usage impact of enum-based storage
    - _Requirements: 2.2, 4.4_

  - [x] 4.4 Add future expansion simulation tests
    - Create test CrcParams using KeysFutureTest variant with 25 keys
    - Test that code gracefully handles different key array sizes
    - Verify that expansion to larger key arrays works as designed
    - _Requirements: 1.1, 4.2_

- [x] 5. Validate migration and run full test suite
  - Run cargo test to ensure all existing tests pass
  - Run cargo clippy to ensure code quality standards
  - Run cargo fmt to ensure consistent formatting
  - Verify that all CRC calculations produce identical results
  - Test that third-party usage patterns remain functional
  - _Requirements: 5.4_

- [x] 6. Implement FFI future-proofing for C/C++ compatibility
  - [x] 6.1 Update CrcFastParams struct to use pointer-based keys
    - Change keys field from [u64; 23] to const uint64_t *keys pointer
    - Add key_count field to track number of available keys
    - Update From<CrcFastParams> and From<CrcParams> conversion implementations
    - _Requirements: 7.2, 8.1_

  - [x] 6.2 Implement stable key pointer management
    - Add create_stable_key_pointer() helper function for CrcKeysStorage
    - Ensure key pointers remain valid for the lifetime of CrcFastParams
    - Handle memory management safely between Rust and C boundaries
    - _Requirements: 8.2, 8.3_

  - [x] 6.3 Update FFI functions to use new CrcFastParams structure
    - Update existing FFI functions to use new pointer-based CrcFastParams
    - Ensure all FFI functions handle variable key counts correctly
    - Test conversion between CrcKeysStorage variants and C pointer access
    - _Requirements: 7.1, 7.3_

  - [x] 6.4 Update C header file and add comprehensive FFI tests
    - Update CrcFastParams struct definition in libcrc_fast.h to use pointer
    - Create FFI tests for direct pointer access with different key counts
    - Test future expansion scenarios with different key counts (23, 25, etc.)
    - Verify memory safety and pointer stability across FFI boundary
    - _Requirements: 7.1, 8.1_