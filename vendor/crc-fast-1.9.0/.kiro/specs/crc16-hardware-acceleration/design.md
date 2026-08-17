# Design Document: CRC-16 Hardware Acceleration

## Overview

This design extends the crc-fast library to support hardware-accelerated CRC-16 computation using the same PCLMULQDQ/PMULL-based approach already implemented for CRC-32 and CRC-64. The key insight from the reference implementations is that CRC-16 computation can be performed by scaling the 16-bit values to 32-bit space, using the existing CRC-32 algorithm infrastructure, and then scaling the result back to 16 bits.

The implementation follows a two-phase approach:
1. **Phase 1**: Extend the key generator (`generate.rs`) to produce correct CRC-16 folding keys
2. **Phase 2**: Extend the algorithm module to handle width=16 by leveraging the CRC-32 code path with appropriate scaling

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Public API                                   │
│  checksum(), Digest, checksum_with_params(), CrcParams::new()       │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Width Dispatcher                                │
│  arch/mod.rs: routes to Width16, Width32, or Width64                │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
              ┌──────────┐   ┌──────────┐   ┌──────────┐
              │ Width16  │   │ Width32  │   │ Width64  │
              │ (NEW)    │   │(existing)│   │(existing)│
              └──────────┘   └──────────┘   └──────────┘
                    │               │               │
                    └───────────────┼───────────────┘
                                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    SIMD Algorithm Core                               │
│  algorithm.rs: fold-by-8, Barrett reduction                         │
└─────────────────────────────────────────────────────────────────────┘
```

### CRC-16 Scaling Strategy

The reference assembly implementations reveal that CRC-16 uses the same PCLMULQDQ algorithm as CRC-32, but with values scaled to 32-bit space:

**Forward (non-reflected) CRC-16:**
1. Scale initial CRC: `initial_crc << 16` (shift left 16 bits)
2. Process using CRC-32 algorithm with CRC-16 polynomial scaled to 32 bits
3. Scale result: `result >> 16` (shift right 16 bits)

**Reflected CRC-16:**
1. Initial CRC used directly (no scaling needed for reflected)
2. Process using CRC-32 algorithm with reflected CRC-16 polynomial
3. Result extracted from appropriate register position

## Components and Interfaces

### 1. Key Generator Extension (`src/generate.rs`)

The key generator must be extended to handle width=16. CRC-16 uses the same exponents as CRC-32 (32*N distances) because the folding algorithm operates on 128-bit SIMD registers regardless of CRC width.

```rust
/// Exponents for CRC-16 key generation (same as CRC-32)
const CRC16_EXPONENTS: [u64; 23] = CRC32_EXPONENTS;

/// Generates folding keys for CRC-16
pub fn keys(width: u8, poly: u64, reflected: bool) -> [u64; 23] {
    let exponents = match width {
        16 => CRC16_EXPONENTS,  // NEW
        32 => CRC32_EXPONENTS,
        64 => CRC64_EXPONENTS,
        _ => panic!("Unsupported width: {width}"),
    };
    // ... rest of implementation
}
```

**Key Generation Functions:**

```rust
/// Computes a CRC-16 folding key for a given bit distance
fn crc16_key(exponent: u64, reflected: bool, polynomial: u64) -> u64 {
    // CRC-16 uses 17-bit polynomial (16 bits + implicit leading 1)
    // Algorithm similar to CRC-32 but with different bit positions
    if exponent < 32 {
        return 0;
    }
    
    let mut n: u64 = 0x080000000;  // Start at x^32 (same as CRC-32)
    let e = exponent - 31;
    
    for _ in 0..e {
        n <<= 1;
        if (n & 0x100000000) != 0 {
            n ^= polynomial;  // polynomial already has bit 32 set
        }
    }
    
    if reflected {
        bit_reverse_16(n) >> 31  // Reverse 16 bits, align
    } else {
        n << 32  // Shift to upper 32 bits
    }
}

/// Computes Barrett reduction constant (μ) for CRC-16
fn crc16_mu(polynomial: u64, reflected: bool) -> u64 {
    // Same algorithm as CRC-32 mu
    let mut n: u64 = 0x100000000;
    let mut q: u64 = 0;
    
    for _ in 0..33 {
        q <<= 1;
        if n & 0x100000000 != 0 {
            q |= 1;
            n ^= polynomial;
        }
        n <<= 1;
    }
    
    if reflected {
        bit_reverse(q) >> 31
    } else {
        q
    }
}

/// Formats CRC-16 polynomial for PCLMULQDQ operations
fn crc16_polynomial(polynomial: u64, reflected: bool) -> u64 {
    if !reflected {
        // Forward: polynomial << 16 with bit 32 set
        (polynomial << 16) | (1u64 << 32)
    } else {
        // Reflected: bit-reverse 16-bit poly, shift left 1, set LSB
        let reversed = bit_reverse_16(polynomial as u16);
        ((reversed as u64) << 1) | 1
    }
}
```

### 2. Width16 Implementation (`src/structs.rs`)

Add a new width type for CRC-16:

```rust
/// CRC-16 width implementation
#[derive(Clone, Copy)]
pub struct Width16;

impl CrcWidth for Width16 {
    const WIDTH: u32 = 16;
    type Value = u16;
}
```

### 3. EnhancedCrcWidth for Width16 (`src/crc16/algorithm.rs`)

The CRC-16 implementation leverages the CRC-32 infrastructure with scaling:

```rust
impl EnhancedCrcWidth for Width16 {
    fn load_constants(reflected: bool) -> [[u64; 2]; 4] {
        // Use CRC-32 constants - same SIMD operations
        crc32::consts::SIMD_CONSTANTS
    }

    unsafe fn create_state<T: ArchOps>(
        value: Self::Value,
        reflected: bool,
        ops: &T,
    ) -> CrcState<T::Vector> {
        if reflected {
            // Reflected: value in low position
            CrcState {
                value: ops.create_vector_from_u32(value as u32, false),
                reflected,
            }
        } else {
            // Forward: scale to 32 bits, position in high bytes
            let scaled = (value as u32) << 16;
            CrcState {
                value: ops.create_vector_from_u32(scaled, true),
                reflected,
            }
        }
    }

    unsafe fn extract_result<T: ArchOps>(
        vector: T::Vector,
        reflected: bool,
        ops: &T,
    ) -> Self::Value {
        let [low, _high] = ops.extract_u64s(vector);
        if reflected {
            // Reflected: extract from appropriate position
            ((low >> 32) & 0xFFFF) as u16
        } else {
            // Forward: scale back from 32 bits
            ((low >> 48) & 0xFFFF) as u16
        }
    }

    // fold_16, fold_width, barrett_reduction delegate to CRC-32 implementations
    // with appropriate value scaling
}
```

### 4. Architecture Dispatcher Update (`src/arch/mod.rs`)

Update the dispatcher to handle width=16:

```rust
pub(crate) unsafe fn update(state: u64, bytes: &[u8], params: &CrcParams) -> u64 {
    match params.width {
        16 => algorithm::update::<_, Width16>(state as u16, bytes, params, ops) as u64,
        32 => algorithm::update::<_, Width32>(state as u32, bytes, params, ops) as u64,
        64 => algorithm::update::<_, Width64>(state, bytes, params, ops),
        _ => panic!("Unsupported CRC width: {}", params.width),
    }
}
```

### 5. CRC-16 Constants (`src/crc16/consts.rs`)

Update the existing stub with complete constants:

```rust
pub const CRC16_IBM_SDLC: CrcParams = CrcParams {
    name: NAME_CRC16_IBM_SDLC,
    algorithm: CrcAlgorithm::Crc16IbmSdlc,
    width: 16,
    poly: 0x1021,
    init: 0xFFFF,
    refin: true,
    refout: true,
    xorout: 0xFFFF,
    check: 0x906E,
    keys: CrcKeysStorage::from_keys_fold_256(KEYS_1021_REVERSE),
};

pub const CRC16_T10_DIF: CrcParams = CrcParams {
    name: NAME_CRC16_T10_DIF,
    algorithm: CrcAlgorithm::Crc16T10Dif,
    width: 16,
    poly: 0x8BB7,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xD0DB,
    keys: CrcKeysStorage::from_keys_fold_256(KEYS_8BB7_FORWARD),
};
```

## Data Models

### CRC-16 Polynomial Representation

| Variant | Polynomial | Scaled Polynomial (32-bit) | Reflected Polynomial |
|---------|------------|---------------------------|---------------------|
| T10-DIF | 0x8BB7 | 0x18BB70000 | N/A (forward) |
| IBM-SDLC | 0x1021 | 0x110210000 | 0x10811 |

### Key Array Structure

The 23-key array structure remains unchanged:

| Index | Purpose | CRC-16 Value |
|-------|---------|--------------|
| 0 | Unused | 0 |
| 1-2 | 16-byte fold (32*3, 32*5) | Computed |
| 3-4 | 128-byte fold (32*31, 32*33) | Computed |
| 5-6 | Short fold (32*3, 32*2) | Computed |
| 7 | μ (Barrett constant) | Computed |
| 8 | Polynomial | Scaled poly |
| 9-20 | Progressive fold distances | Computed |
| 21-22 | 256-byte fold (32*63, 32*65) | Computed |



## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Forward CRC-16 Polynomial Formatting

*For any* 16-bit polynomial value P, when generating the polynomial key for forward (non-reflected) CRC-16, the result SHALL equal `(P << 16) | (1 << 32)`.

**Validates: Requirements 1.5**

### Property 2: Reflected CRC-16 Polynomial Formatting

*For any* 16-bit polynomial value P, when generating the polynomial key for reflected CRC-16, the result SHALL equal `(bit_reverse_16(P) << 1) | 1`.

**Validates: Requirements 1.6**

### Property 3: CRC-16 Computation Matches Reference

*For any* valid CRC-16 parameters (polynomial, init, refin, refout, xorout) and any input byte sequence, the computed CRC-16 checksum SHALL match the result from the `crc` crate reference implementation.

**Validates: Requirements 2.1, 2.2, 5.5, 6.1, 6.2, 6.3**

### Property 4: CRC-32 and CRC-64 Backwards Compatibility

*For any* existing CRC-32 or CRC-64 configuration and any input byte sequence, the computed checksum SHALL match the result from the `crc` crate reference implementation, ensuring no regression from CRC-16 changes.

**Validates: Requirements 3.1, 3.2, 3.4**

### Property 5: CRC-16 Checksum Combination Round-Trip

*For any* valid CRC-16 parameters and any two input byte sequences A and B, `checksum_combine(checksum(A), checksum(B), len(B))` SHALL equal `checksum(A + B)`.

**Validates: Requirements 6.4**

## Error Handling

### Invalid Width

When `CrcParams::new()` is called with an unsupported width (not 16, 32, or 64), the function SHALL panic with a descriptive error message.

### Invalid Polynomial

CRC-16 polynomials must fit in 16 bits. If a polynomial larger than 0xFFFF is provided for width=16, the behavior is undefined (caller's responsibility to provide valid parameters).

### Empty Input

Empty input (`&[]`) is valid and SHALL return the initial CRC value XORed with xorout, consistent with existing CRC-32 and CRC-64 behavior.

## Testing Strategy

### Unit Tests

Unit tests will verify specific examples and edge cases:

1. **Check Value Tests**: Verify "123456789" produces correct check values for CRC-16/IBM-SDLC (0x906E) and CRC-16/T10-DIF (0xD0DB)
2. **Key Generation Tests**: Verify generated keys match reference values in `KEYS_8BB7_FORWARD` and `KEYS_1021_REVERSE`
3. **Empty Input Tests**: Verify empty input returns expected value
4. **Constant Definition Tests**: Verify CRC16_IBM_SDLC and CRC16_T10_DIF have correct parameter values

### Property-Based Tests

Property-based tests will use the `proptest` crate (already used in the project) with minimum 100 iterations per property:

1. **Property 1 & 2**: Generate random 16-bit polynomials and verify polynomial key formatting
2. **Property 3**: Generate random CRC-16 parameters and random byte sequences, compare against `crc` crate
3. **Property 4**: Use existing CRC-32/64 configurations with random byte sequences, compare against `crc` crate
4. **Property 5**: Generate random CRC-16 parameters and two random byte sequences, verify combination

### Test Configuration

```rust
// Property test configuration
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    // Feature: crc16-hardware-acceleration, Property 1: Forward polynomial formatting
    #[test]
    fn prop_forward_polynomial_formatting(poly in 0u16..=0xFFFFu16) {
        let result = crc16_polynomial(poly as u64, false);
        let expected = ((poly as u64) << 16) | (1u64 << 32);
        prop_assert_eq!(result, expected);
    }
}
```

### Regression Testing

All existing tests in `src/arch/mod.rs`, `src/test/`, and `tests/` must continue to pass after CRC-16 implementation.
