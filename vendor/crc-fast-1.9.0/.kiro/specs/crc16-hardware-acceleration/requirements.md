# Requirements Document

## Introduction

This feature adds hardware-accelerated CRC-16 calculation support to the crc-fast library, which already has working hardware-accelerated CRC-32 and CRC-64 implementations. The implementation will leverage PCLMULQDQ/PMULL SIMD instructions for high-performance CRC-16 computation, supporting any CRC-16 variant (both forward/non-reflected and reflected). While CRC-16/IBM-SDLC and CRC-16/T10-DIF serve as reference variants for initial validation, the final implementation must support custom CRC-16 variants with arbitrary polynomials, just like the existing CRC-32 and CRC-64 support. The feature must maintain full backwards compatibility with existing CRC-32 and CRC-64 functionality.

## Glossary

- **CRC-16**: A 16-bit Cyclic Redundancy Check algorithm used for error detection
- **Key_Generator**: The module responsible for generating PCLMULQDQ folding keys from CRC polynomials
- **Folding_Keys**: Precomputed constants (x^n mod P(x)) used for parallel CRC computation via PCLMULQDQ
- **PCLMULQDQ**: Intel's carryless multiplication instruction used for hardware-accelerated CRC computation
- **PMULL**: ARM's polynomial multiplication instruction, equivalent to PCLMULQDQ
- **Forward_CRC**: A CRC variant where data is processed MSB-first (refin=false)
- **Reflected_CRC**: A CRC variant where data is processed LSB-first (refin=true)
- **CRC-16/T10-DIF**: A forward (non-reflected) CRC-16 variant with polynomial 0x8BB7
- **CRC-16/IBM-SDLC**: A reflected CRC-16 variant with polynomial 0x1021
- **Barrett_Reduction**: An algorithm for fast modular reduction without division, used in final CRC computation
- **Mu_Constant**: The Barrett reduction constant (μ = floor(x^64/P(x))) used for final reduction
- **Algorithm_Module**: The SIMD-based CRC calculation module that processes data using folding keys
- **Check_Value**: The expected CRC result for the industry standard test string "123456789"
- **256-Byte_Folding_Keys**: Extended folding keys (indices 21-22) for AVX-512 VPCLMULQDQ 256-byte chunk processing

## Requirements

### Requirement 1: CRC-16 Key Generation

**User Story:** As a library maintainer, I want the key generator to produce correct CRC-16 folding keys for any CRC-16 polynomial, so that hardware-accelerated CRC-16 computation produces correct results for all variants.

#### Acceptance Criteria

1. WHEN the Key_Generator receives a 16-bit polynomial and forward reflection mode, THE Key_Generator SHALL produce folding keys that match the reference values in KEYS_8BB7_FORWARD for CRC-16/T10-DIF
2. WHEN the Key_Generator receives a 16-bit polynomial and reflected mode, THE Key_Generator SHALL produce folding keys that match the reference values in KEYS_1021_REVERSE for CRC-16/IBM-SDLC
3. WHEN generating CRC-16 keys for any 16-bit polynomial, THE Key_Generator SHALL use exponents based on 32-bit distances (32*N) matching the CRC-32 exponent pattern
4. WHEN generating the Mu_Constant for any CRC-16 polynomial, THE Key_Generator SHALL compute floor(x^64/P(x)) where P(x) is the 17-bit polynomial (16-bit poly with implicit leading 1)
5. WHEN generating the polynomial key for any forward CRC-16, THE Key_Generator SHALL shift the polynomial left by 16 bits and set bit 32
6. WHEN generating the polynomial key for any reflected CRC-16, THE Key_Generator SHALL bit-reverse the 16-bit polynomial, shift left by 1, and set the LSB
7. WHEN generating 256-byte folding keys (indices 21-22) for any CRC-16 polynomial, THE Key_Generator SHALL compute keys for distances 32*63 and 32*65 bits

### Requirement 2: CRC-16 Algorithm Implementation

**User Story:** As a developer, I want to compute CRC-16 checksums using hardware acceleration for any CRC-16 variant, so that I can achieve high-performance CRC-16 calculations.

#### Acceptance Criteria

1. WHEN computing CRC-16 for the industry standard string "123456789", THE Algorithm_Module SHALL produce the Check_Value 0x906E for CRC-16/IBM-SDLC
2. WHEN computing CRC-16 for the industry standard string "123456789", THE Algorithm_Module SHALL produce the Check_Value 0xD0DB for CRC-16/T10-DIF
3. WHEN processing data for any forward CRC-16 variant, THE Algorithm_Module SHALL byte-reflect input data using pshufb before folding operations
4. WHEN processing data for any reflected CRC-16 variant, THE Algorithm_Module SHALL process input data without byte reflection
5. WHEN performing the final Barrett reduction for any CRC-16 variant, THE Algorithm_Module SHALL extract the 16-bit result from the appropriate register position
6. WHEN the initial CRC value is provided for any forward CRC-16 variant, THE Algorithm_Module SHALL scale it to 32 bits by shifting left 16 bits before processing
7. WHEN the final CRC is computed for any forward CRC-16 variant, THE Algorithm_Module SHALL scale the result back to 16 bits by shifting right 16 bits

### Requirement 3: Backwards Compatibility

**User Story:** As an existing library user, I want CRC-32 and CRC-64 functionality to remain unchanged, so that my existing code continues to work correctly.

#### Acceptance Criteria

1. WHEN computing CRC-32 checksums after CRC-16 support is added, THE Algorithm_Module SHALL produce identical results to the current implementation
2. WHEN computing CRC-64 checksums after CRC-16 support is added, THE Algorithm_Module SHALL produce identical results to the current implementation
3. WHEN using the existing public API functions (checksum, Digest, checksum_combine), THE API SHALL maintain the same function signatures and behavior
4. WHEN generating CRC-32 or CRC-64 keys, THE Key_Generator SHALL produce identical keys to the current implementation

### Requirement 4: Test Coverage

**User Story:** As a library maintainer, I want comprehensive tests for CRC-16 functionality, so that I can ensure correctness and prevent regressions.

#### Acceptance Criteria

1. WHEN testing CRC-16 key generation, THE Test_Suite SHALL verify generated keys match the known good reference values from the reference implementation
2. WHEN testing CRC-16 computation, THE Test_Suite SHALL verify the Check_Value for the industry standard "123456789" string for both CRC-16/IBM-SDLC and CRC-16/T10-DIF
3. WHEN testing CRC-16 computation, THE Test_Suite SHALL compare results against the `crc` crate reference implementation for various input sizes
4. WHEN testing backwards compatibility, THE Test_Suite SHALL verify all existing CRC-32 and CRC-64 tests continue to pass

### Requirement 5: CRC-16 Constants and Parameters

**User Story:** As a library maintainer, I want properly defined CRC-16 constants and parameters, so that the implementation is consistent with industry standards and supports custom variants.

#### Acceptance Criteria

1. THE CRC-16 module SHALL define CRC16_IBM_SDLC with polynomial 0x1021, init 0xFFFF, refin=true, refout=true, xorout=0xFFFF, check=0x906E
2. THE CRC-16 module SHALL define CRC16_T10_DIF with polynomial 0x8BB7, init 0x0000, refin=false, refout=false, xorout=0x0000, check=0xD0DB
3. WHEN storing CRC-16 parameters, THE CrcParams structure SHALL use width=16 to distinguish from CRC-32 and CRC-64 variants
4. THE CRC-16 module SHALL include SIMD constants (smask, mask1, mask2) appropriate for 16-bit CRC computation
5. WHEN creating custom CRC-16 parameters via CrcParams::new(), THE Key_Generator SHALL dynamically generate folding keys for the provided polynomial

### Requirement 6: Custom CRC-16 Variant Support

**User Story:** As a developer, I want to use custom CRC-16 parameters with any polynomial, so that I can compute CRC-16 checksums for non-standard variants.

#### Acceptance Criteria

1. WHEN CrcParams::new() is called with width=16 and any valid polynomial, THE system SHALL generate correct folding keys for that polynomial
2. WHEN checksum_with_params() is called with custom CRC-16 parameters, THE Algorithm_Module SHALL compute the correct CRC-16 checksum
3. WHEN Digest::new_with_params() is called with custom CRC-16 parameters, THE Digest SHALL correctly accumulate and finalize CRC-16 checksums
4. WHEN checksum_combine_with_params() is called with custom CRC-16 parameters, THE system SHALL correctly combine two CRC-16 checksums
