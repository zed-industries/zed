# Implementation Plan: CRC-16 Hardware Acceleration

## Overview

This implementation adds hardware-accelerated CRC-16 support to the crc-fast library in two phases: first extending the key generator, then extending the algorithm module. The implementation leverages the existing CRC-32 infrastructure by scaling CRC-16 values to 32-bit space.

## Tasks

- [x] 1. Extend Key Generator for CRC-16
  - [x] 1.1 Add CRC-16 exponents constant and update keys() function
    - Add `CRC16_EXPONENTS` constant (same as CRC32_EXPONENTS)
    - Update `keys()` function to handle width=16
    - Update `key()` function to dispatch to `crc16_key()`
    - _Requirements: 1.3_

  - [x] 1.2 Implement crc16_key() function
    - Compute x^exponent mod P(x) for 17-bit CRC-16 polynomial
    - Handle both forward and reflected modes
    - Follow the pattern from reference/crc16f/crc16fg.cpp and reference/crc16r/crc16rg.cpp
    - _Requirements: 1.1, 1.2, 1.7_

  - [x] 1.3 Implement crc16_mu() function
    - Compute Barrett reduction constant floor(x^64/P(x))
    - Handle both forward and reflected modes
    - _Requirements: 1.4_

  - [x] 1.4 Implement crc16_polynomial() function
    - Format polynomial for forward mode: (poly << 16) | (1 << 32)
    - Format polynomial for reflected mode: (bit_reverse_16(poly) << 1) | 1
    - _Requirements: 1.5, 1.6_

  - [x] 1.5 Write property tests for CRC-16 key generation
    - **Property 1: Forward polynomial formatting**
    - **Property 2: Reflected polynomial formatting**
    - **Validates: Requirements 1.5, 1.6**

  - [x] 1.6 Add unit tests for CRC-16 key generation
    - Test generated keys match KEYS_8BB7_FORWARD for CRC-16/T10-DIF
    - Test generated keys match KEYS_1021_REVERSE for CRC-16/IBM-SDLC
    - Add CRC-16 test configs to TEST_ALL_CONFIGS
    - _Requirements: 1.1, 1.2, 4.1_

- [x] 2. Checkpoint - Verify key generation
  - Ensure all tests pass, ask the user if questions arise.

- [x] 3. Update CRC-16 Constants and Parameters
  - [x] 3.1 Update src/crc16/consts.rs with complete constants
    - Update KEYS_8BB7_FORWARD with 256-byte folding keys (indices 21-22)
    - Update KEYS_1021_REVERSE with 256-byte folding keys (indices 21-22)
    - Add SIMD_CONSTANTS if different from CRC-32
    - _Requirements: 5.1, 5.2, 5.4_

  - [x] 3.2 Add Width16 struct to src/structs.rs
    - Implement CrcWidth trait for Width16
    - _Requirements: 5.3_

  - [x] 3.3 Update CrcAlgorithm enum and CrcParams::new()
    - Add Crc16Custom variant to CrcAlgorithm enum
    - Update CrcParams::new() to handle width=16
    - Update cache module if needed for CRC-16 key caching
    - _Requirements: 5.5, 6.1_

- [x] 4. Implement CRC-16 Algorithm
  - [x] 4.1 Create src/crc16/algorithm.rs with EnhancedCrcWidth implementation
    - Implement EnhancedCrcWidth for Width16
    - Implement create_state() with proper scaling for forward mode
    - Implement extract_result() with proper scaling
    - Implement fold_16(), fold_width(), barrett_reduction()
    - Implement create_coefficient() and perform_final_reduction()
    - Implement get_last_bytes_table_ptr()
    - _Requirements: 2.3, 2.4, 2.5, 2.6, 2.7_

  - [x] 4.2 Implement process_0_to_15 for CRC-16
    - Handle small inputs (0-15 bytes) for CRC-16
    - Follow pattern from crc32/algorithm.rs
    - _Requirements: 2.1, 2.2_

  - [x] 4.3 Update architecture dispatcher for CRC-16
    - Update src/arch/mod.rs to handle width=16
    - Route to Width16 implementation
    - _Requirements: 2.1, 2.2_

  - [x] 4.4 Update software fallback for CRC-16
    - Update src/arch/software.rs to handle CRC-16 algorithms
    - Add CRC-16 reference implementations from crc crate
    - _Requirements: 2.1, 2.2_

- [x] 5. Checkpoint - Verify basic CRC-16 computation
  - Ensure all tests pass, ask the user if questions arise.

- [x] 6. Add Public API Support for CRC-16
  - [x] 6.1 Update src/lib.rs checksum() function
    - Add cases for Crc16IbmSdlc and Crc16T10Dif
    - _Requirements: 2.1, 2.2_

  - [x] 6.2 Update Digest and other public APIs
    - Ensure Digest works with CRC-16 algorithms
    - Ensure checksum_combine works with CRC-16
    - _Requirements: 6.2, 6.3, 6.4_

- [-] 7. Add Comprehensive Tests
  - [x] 7.1 Add CRC-16 check value tests
    - Test "123456789" produces 0x906E for CRC-16/IBM-SDLC
    - Test "123456789" produces 0xD0DB for CRC-16/T10-DIF
    - _Requirements: 2.1, 2.2, 4.2_

  - [x] 7.2 Write property test for CRC-16 computation matches reference
    - **Property 3: CRC-16 computation matches reference**
    - **Validates: Requirements 2.1, 2.2, 5.5, 6.1, 6.2, 6.3**

  - [x] 7.3 Write property test for backwards compatibility
    - **Property 4: CRC-32 and CRC-64 backwards compatibility**
    - **Validates: Requirements 3.1, 3.2, 3.4**

  - [x] 7.4 Write property test for CRC-16 checksum combination
    - **Property 5: CRC-16 checksum combination round-trip**
    - **Validates: Requirements 6.4**

  - [x] 7.5 Add CRC-16 to existing test suites
    - Add CRC-16 configs to TEST_ALL_CONFIGS in src/test/consts.rs
    - Ensure all length-based tests include CRC-16
    - _Requirements: 4.3, 4.4_

- [x] 8. Final Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
  - Run cargo fmt --check
  - Run cargo clippy -- -D warnings
  - Run cargo test

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
- The implementation leverages existing CRC-32 infrastructure to minimize code duplication
