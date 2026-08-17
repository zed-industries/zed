# Requirements Document

## Introduction

This feature refactors the CRC-16 and CRC-32 algorithm implementations to share a common base module, reducing code duplication. Since CRC-16 operates in 32-bit space (scaling values up and down), both widths use nearly identical SIMD operations. Extracting this shared logic into a common module improves maintainability and prepares the codebase for future CRC width additions.

## Glossary

- **Width32_Ops**: The shared module containing 32-bit-space SIMD operations used by both CRC-16 and CRC-32
- **EnhancedCrcWidth**: The trait that defines width-specific CRC operations
- **Width16**: The CRC-16 width implementation that operates in 32-bit space
- **Width32**: The CRC-32 width implementation
- **Width64**: The CRC-64 width implementation (unchanged by this refactoring)
- **process_0_to_15**: The function handling small inputs (0-15 bytes)
- **Barrett_Reduction**: The algorithm for fast modular reduction used in final CRC computation

## Requirements

### Requirement 1: Create Shared Width32 Operations Module

**User Story:** As a library maintainer, I want shared 32-bit-space operations in a common module, so that CRC-16 and CRC-32 implementations don't duplicate code.

#### Acceptance Criteria

1. THE system SHALL create a new module `src/crc32/width32_ops.rs` containing shared 32-bit-space operations
2. THE Width32_Ops module SHALL export a `process_0_to_15_width32()` function that handles small inputs for both CRC-16 and CRC-32
3. THE Width32_Ops module SHALL export helper functions for `fold_16`, `fold_width`, and `barrett_reduction` operations
4. THE Width32_Ops module SHALL export the shared `load_constants()` values
5. THE Width32_Ops module SHALL export the shared `get_last_bytes_table_ptr()` logic

### Requirement 2: Refactor Width16 Implementation

**User Story:** As a library maintainer, I want the Width16 implementation to delegate to shared operations, so that code duplication is eliminated.

#### Acceptance Criteria

1. THE Width16 `EnhancedCrcWidth` implementation SHALL delegate `fold_16()` to the shared Width32_Ops module
2. THE Width16 `EnhancedCrcWidth` implementation SHALL delegate `fold_width()` to the shared Width32_Ops module
3. THE Width16 `EnhancedCrcWidth` implementation SHALL delegate `create_coefficient()` to the shared Width32_Ops module
4. THE Width16 `EnhancedCrcWidth` implementation SHALL delegate `perform_final_reduction()` to the shared Width32_Ops module
5. THE Width16 `EnhancedCrcWidth` implementation SHALL delegate `get_last_bytes_table_ptr()` to the shared Width32_Ops module
6. THE Width16 `barrett_reduction()` SHALL call the shared implementation and extract the 16-bit result
7. THE crc16::algorithm::process_0_to_15 function SHALL be removed and replaced with a call to the shared implementation

### Requirement 3: Refactor Width32 Implementation

**User Story:** As a library maintainer, I want the Width32 implementation to use shared operations, so that code duplication is eliminated.

#### Acceptance Criteria

1. THE Width32 `EnhancedCrcWidth` implementation SHALL delegate `fold_16()` to the shared Width32_Ops module
2. THE Width32 `EnhancedCrcWidth` implementation SHALL delegate `fold_width()` to the shared Width32_Ops module
3. THE Width32 `EnhancedCrcWidth` implementation SHALL delegate `create_coefficient()` to the shared Width32_Ops module
4. THE Width32 `EnhancedCrcWidth` implementation SHALL delegate `perform_final_reduction()` to the shared Width32_Ops module
5. THE Width32 `EnhancedCrcWidth` implementation SHALL delegate `get_last_bytes_table_ptr()` to the shared Width32_Ops module
6. THE Width32 `barrett_reduction()` SHALL call the shared implementation and extract the 32-bit result
7. THE crc32::algorithm::process_0_to_15 function SHALL call the shared implementation

### Requirement 4: Backwards Compatibility

**User Story:** As an existing library user, I want all CRC functionality to remain unchanged, so that my existing code continues to work correctly.

#### Acceptance Criteria

1. WHEN computing CRC-16 checksums after refactoring, THE system SHALL produce identical results to the current implementation
2. WHEN computing CRC-32 checksums after refactoring, THE system SHALL produce identical results to the current implementation
3. WHEN computing CRC-64 checksums after refactoring, THE system SHALL produce identical results to the current implementation
4. THE public API SHALL remain unchanged with no breaking changes
5. ALL existing tests SHALL continue to pass without modification

### Requirement 5: Code Quality

**User Story:** As a library maintainer, I want the refactored code to be clean and maintainable, so that future development is easier.

#### Acceptance Criteria

1. THE refactored code SHALL pass `cargo fmt --check` without errors
2. THE refactored code SHALL pass `cargo clippy -- -D warnings` without warnings
3. THE refactored code SHALL pass all existing tests via `cargo test`
4. THE shared module SHALL have clear documentation explaining its purpose
5. THE Width16 and Width32 implementations SHALL have comments indicating delegation to shared operations where applicable
