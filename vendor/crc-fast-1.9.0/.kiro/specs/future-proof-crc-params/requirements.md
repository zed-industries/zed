# Requirements Document

## Introduction

This feature implements a future-proof CrcParams structure that can expand to support additional folding keys (for example, larger folding distances) without breaking API compatibility for third-party applications. The solution maintains the simplicity of const definitions while providing safe key access and internal flexibility for future expansion.

## Requirements

### Requirement 1

**User Story:** As a library maintainer, I want to expand CRC folding key support from 23 to 24+ keys for potentially larger folding distances in the future, so that I can improve performance for large data processing.

#### Acceptance Criteria

1. WHEN I add support for larger folding distances THEN existing third-party applications with hardcoded 23-key CrcParams SHALL continue to compile and function correctly
2. WHEN I expand key arrays from 23 to 24+ elements THEN existing const definitions SHALL require minimal changes (only the keys field)

### Requirement 2

**User Story:** As a third-party application developer, I want to define custom CrcParams as const definitions, so that I can embed CRC configurations directly in my code without runtime overhead.

#### Acceptance Criteria

1. WHEN I define a custom CrcParams const THEN I SHALL be able to use the same simple struct literal syntax as currently exists
2. WHEN I access CRC keys through the CrcParams interface THEN the performance SHALL be identical to direct array access (zero runtime overhead)
3. WHEN the library expands key support THEN my existing const definitions SHALL continue to work without modification

### Requirement 3

**User Story:** As a library maintainer, I want safe key access methods that prevent array bounds panics, so that the library is robust against future expansion and misuse.

#### Acceptance Criteria

1. WHEN architecture code accesses CRC keys THEN it SHALL use bounds-checked methods instead of direct array indexing
2. WHEN code requests key count information THEN it SHALL receive the actual number of available keys for that CrcParams instance

### Requirement 4

**User Story:** As a library maintainer, I want internal flexibility to support different key array sizes, so that I can optimize different CRC algorithms with varying folding distance requirements.

#### Acceptance Criteria

1. WHEN I create CrcParams with 23 keys THEN the system SHALL store and access exactly 23 keys efficiently
2. WHEN I create CrcParams with 25 keys THEN the system SHALL store and access exactly 25 keys efficiently  
4. WHEN the compiler optimizes the code THEN enum dispatch for key access SHALL be eliminated (zero runtime overhead)

### Requirement 5

**User Story:** As a library maintainer, I want to migrate existing code gradually, so that I can implement the changes in phases without breaking the build at any point.

#### Acceptance Criteria

1. WHEN I add the new CrcKeysStorage types THEN existing code SHALL continue to compile and function
2. WHEN I update architecture code to use safe accessors THEN the change SHALL be backward compatible
3. WHEN I switch CrcParams to use CrcKeysStorage THEN the migration SHALL require only updating const definitions
4. WHEN each phase is complete THEN all existing tests SHALL continue to pass

### Requirement 6

**User Story:** As a third-party application developer, I want the `get-custom-params` binary to output the updated `CrcParams` const definition using the new key storage approach, so that I can easily generate future-proof custom CRC parameter definitions.

#### Acceptance Criteria

1. WHEN I run the `get-custom-params` binary THEN it SHALL output CrcParams const definitions using CrcKeysStorage::from_keys_fold_256()
2. WHEN I copy the generated const definition THEN it SHALL compile and work correctly with the new CrcParams structure
3. WHEN the output format changes THEN the generated code SHALL remain compatible with the current CrcParams API

### Requirement 7

**User Story:** As a C/C++ application developer, I want the FFI interface to be future-proof for key expansion, so that my applications can benefit from future performance improvements without requiring code changes.

#### Acceptance Criteria

1. WHEN the library adds support for larger key arrays THEN existing C code using CrcFastParams SHALL continue to compile and function correctly
2. WHEN I create custom CRC parameters in C THEN I SHALL be able to specify the key count and keys dynamically
3. WHEN I access CRC functionality through the C API THEN the performance SHALL remain identical to direct Rust usage

### Requirement 8

**User Story:** As a library maintainer, I want the C FFI interface to support different key array sizes internally, so that C users can benefit from future CRC algorithm improvements.

#### Acceptance Criteria

1. WHEN I add new CrcKeysStorage variants with different key counts THEN the C API SHALL automatically support them
2. WHEN C code specifies custom key arrays THEN they SHALL be automatically converted to the appropriate internal storage format
3. WHEN C code queries key information THEN it SHALL receive accurate key count and key data for the specific CRC algorithm being used
