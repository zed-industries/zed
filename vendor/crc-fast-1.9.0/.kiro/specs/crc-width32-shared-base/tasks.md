# Implementation Plan: CRC Width-32 Shared Base

## Overview

This implementation refactors CRC-16 and CRC-32 to share common 32-bit-space operations, eliminating ~200 lines of duplicated code. The refactoring creates a new `width32_ops.rs` module and updates both Width16 and Width32 implementations to delegate to it.

## Tasks

- [x] 1. Create shared width32_ops module
  - [x] 1.1 Create src/crc32/width32_ops.rs with shared constants
    - Add module documentation explaining purpose
    - Add WIDTH32_CONSTANTS_REFLECTED and WIDTH32_CONSTANTS_FORWARD
    - Add load_constants() function
    - _Requirements: 1.1, 1.4_

  - [x] 1.2 Add shared fold operations to width32_ops
    - Implement fold_16() function
    - Implement fold_width() function
    - _Requirements: 1.3_

  - [x] 1.3 Add shared barrett_reduction to width32_ops
    - Implement barrett_reduction() returning [u64; 2] for caller to extract bits
    - _Requirements: 1.3_

  - [x] 1.4 Add shared helper functions to width32_ops
    - Implement create_coefficient() function
    - Implement get_last_bytes_table_ptr() function
    - _Requirements: 1.3, 1.5_

  - [x] 1.5 Add shared process_0_to_15 to width32_ops
    - Move the shared implementation from crc32/algorithm.rs
    - _Requirements: 1.2_

  - [x] 1.6 Update src/crc32/mod.rs to export width32_ops
    - Add `pub(crate) mod width32_ops;`
    - _Requirements: 1.1_

- [x] 2. Checkpoint - Verify module compiles
  - Ensure `cargo check` passes

- [x] 3. Refactor Width32 implementation
  - [x] 3.1 Update Width32 load_constants to delegate
    - Call width32_ops::load_constants()
    - _Requirements: 3.1_

  - [x] 3.2 Update Width32 fold_16 to delegate
    - Call width32_ops::fold_16()
    - _Requirements: 3.1_

  - [x] 3.3 Update Width32 fold_width to delegate
    - Call width32_ops::fold_width()
    - _Requirements: 3.2_

  - [x] 3.4 Update Width32 barrett_reduction to delegate
    - Call width32_ops::barrett_reduction() and extract 32-bit result
    - _Requirements: 3.6_

  - [x] 3.5 Update Width32 create_coefficient to delegate
    - Call width32_ops::create_coefficient()
    - _Requirements: 3.3_

  - [x] 3.6 Update Width32 perform_final_reduction to delegate
    - Call width32_ops::fold_width() and width32_ops::barrett_reduction()
    - _Requirements: 3.4_

  - [x] 3.7 Update Width32 get_last_bytes_table_ptr to delegate
    - Call width32_ops::get_last_bytes_table_ptr()
    - _Requirements: 3.5_

  - [x] 3.8 Update crc32::algorithm::process_0_to_15 to delegate
    - Call width32_ops::process_0_to_15()
    - _Requirements: 3.7_

- [x] 4. Checkpoint - Verify CRC-32 still works
  - Run `cargo test` to ensure CRC-32 tests pass

- [x] 5. Refactor Width16 implementation
  - [x] 5.1 Update Width16 load_constants to delegate
    - Call width32_ops::load_constants()
    - _Requirements: 2.1_

  - [x] 5.2 Update Width16 fold_16 to delegate
    - Call width32_ops::fold_16()
    - _Requirements: 2.1_

  - [x] 5.3 Update Width16 fold_width to delegate
    - Call width32_ops::fold_width()
    - _Requirements: 2.2_

  - [x] 5.4 Update Width16 barrett_reduction to delegate
    - Call width32_ops::barrett_reduction() and extract 16-bit result
    - _Requirements: 2.6_

  - [x] 5.5 Update Width16 create_coefficient to delegate
    - Call width32_ops::create_coefficient()
    - _Requirements: 2.3_

  - [x] 5.6 Update Width16 perform_final_reduction to delegate
    - Call width32_ops::fold_width() and width32_ops::barrett_reduction()
    - _Requirements: 2.4_

  - [x] 5.7 Update Width16 get_last_bytes_table_ptr to delegate
    - Call width32_ops::get_last_bytes_table_ptr()
    - _Requirements: 2.5_

  - [x] 5.8 Update crc16::algorithm::process_0_to_15 to delegate
    - Call width32_ops::process_0_to_15()
    - _Requirements: 2.7_

- [x] 6. Checkpoint - Verify CRC-16 still works
  - Run `cargo test` to ensure CRC-16 tests pass

- [x] 7. Final verification and cleanup
  - [x] 7.1 Run full test suite
    - Run `cargo test` to verify all tests pass
    - _Requirements: 4.5_

  - [x] 7.2 Run code quality checks
    - Run `cargo fmt --check`
    - Run `cargo clippy -- -D warnings`
    - _Requirements: 5.1, 5.2, 5.3_

  - [x] 7.3 Verify CRC-64 unchanged
    - Confirm CRC-64 tests still pass
    - _Requirements: 4.3_

## Notes

- This is a pure refactoring with no behavior changes
- All existing tests serve as regression tests
- The shared module is placed in crc32/ since CRC-32 is the "native" width for these operations
- CRC-16 scales to 32-bit space, uses shared ops, then scales back to 16 bits
- CRC-64 remains unchanged as it has different SIMD operations
