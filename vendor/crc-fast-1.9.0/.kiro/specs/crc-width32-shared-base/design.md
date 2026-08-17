# Design Document: CRC Width-32 Shared Base

## Overview

This design refactors the CRC-16 and CRC-32 algorithm implementations to share a common base module. Since CRC-16 operates in 32-bit space (scaling 16-bit values up for processing, then down for results), both widths use nearly identical SIMD operations. By extracting this shared logic into `src/crc32/width32_ops.rs`, we eliminate ~200 lines of duplicated code and prepare the codebase for future CRC width additions.

## Architecture

### Current Structure

```
src/
├── crc16/
│   ├── algorithm.rs    # Width16 impl + process_0_to_15 (duplicated)
│   ├── consts.rs
│   └── mod.rs
├── crc32/
│   ├── algorithm.rs    # Width32 impl + process_0_to_15 (duplicated)
│   ├── consts.rs
│   └── mod.rs
├── crc64/
│   ├── algorithm.rs    # Width64 impl + process_0_to_15 (unique)
│   ├── consts.rs
│   └── mod.rs
└── algorithm.rs        # Shared algorithm infrastructure
```

### Proposed Structure

```
src/
├── crc16/
│   ├── algorithm.rs    # Width16 impl (delegates to width32_ops)
│   ├── consts.rs
│   └── mod.rs
├── crc32/
│   ├── algorithm.rs    # Width32 impl (delegates to width32_ops)
│   ├── consts.rs
│   ├── mod.rs
│   └── width32_ops.rs  # NEW: Shared 32-bit-space operations
├── crc64/
│   ├── algorithm.rs    # Width64 impl + process_0_to_15 (unchanged)
│   ├── consts.rs
│   └── mod.rs
└── algorithm.rs        # Shared algorithm infrastructure
```

## Components and Interfaces

### 1. New Module: `src/crc32/width32_ops.rs`

This module contains all shared 32-bit-space operations:

```rust
//! Shared operations for CRC widths that operate in 32-bit space (CRC-16, CRC-32).
//!
//! CRC-16 computation scales 16-bit values to 32-bit space, uses these operations,
//! then scales results back to 16 bits. CRC-32 uses these operations directly.

use crate::algorithm;
use crate::consts::CRC_CHUNK_SIZE;
use crate::crc32::consts::{PSHUFB_SHF_TABLE_FORWARD, PSHUFB_SHF_TABLE_REVERSE, SIMD_CONSTANTS};
use crate::enums::Reflector;
use crate::structs::CrcState;
use crate::traits::{ArchOps, EnhancedCrcWidth};

/// SIMD constants shared by CRC-16 and CRC-32
pub const WIDTH32_CONSTANTS_REFLECTED: [[u64; 2]; 4] = [
    [0x08090a0b0c0d0e0f, 0x0001020304050607], // smask
    [0x8080808080808080, 0x8080808080808080], // mask1
    [0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF], // mask2 reverse
    [0x0000000000000000, 0x0000000000000000], // unused
];

pub const WIDTH32_CONSTANTS_FORWARD: [[u64; 2]; 4] = [
    [0x08090a0b0c0d0e0f, 0x0001020304050607], // smask
    [0x8080808080808080, 0x8080808080808080], // mask1
    [0xffffffffffffffff, 0x00000000ffffffff], // mask2 forward
    [0x0000000000000000, 0x0000000000000000], // unused
];

/// Load constants for 32-bit-space CRC operations
#[inline(always)]
pub fn load_constants(reflected: bool) -> [[u64; 2]; 4] {
    if reflected {
        WIDTH32_CONSTANTS_REFLECTED
    } else {
        WIDTH32_CONSTANTS_FORWARD
    }
}

/// Fold 16 bytes for 32-bit-space CRC operations
#[inline(always)]
pub unsafe fn fold_16<T: ArchOps>(
    state: &mut CrcState<T::Vector>,
    coeff: T::Vector,
    data_to_xor: T::Vector,
    ops: &T,
) where
    T::Vector: Copy,
{
    let (h, l) = if state.reflected {
        (
            ops.carryless_mul_10(state.value, coeff),
            ops.carryless_mul_01(state.value, coeff),
        )
    } else {
        (
            ops.carryless_mul_00(state.value, coeff),
            ops.carryless_mul_11(state.value, coeff),
        )
    };
    state.value = ops.xor3_vectors(h, l, data_to_xor);
}

/// Fold to width for 32-bit-space CRC operations
#[inline(always)]
pub unsafe fn fold_width<T: ArchOps>(
    state: &mut CrcState<T::Vector>,
    high: u64,
    low: u64,
    ops: &T,
) where
    T::Vector: Copy,
{
    let coeff_vector_low = ops.create_vector_from_u64_pair_non_reflected(0, low);
    let coeff_vector_high = ops.create_vector_from_u64_pair_non_reflected(high, 0);

    state.value = if state.reflected {
        ops.xor_vectors(
            ops.carryless_mul_00(state.value, coeff_vector_low),
            ops.shift_right_8(state.value),
        )
    } else {
        ops.xor_vectors(
            ops.carryless_mul_01(state.value, coeff_vector_low),
            ops.shift_left_8(state.value),
        )
    };

    let (clmul, masked) = if state.reflected {
        let mask2 = ops.load_aligned(&[0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF]);
        let masked = ops.and_vectors(state.value, mask2);
        let shifted = ops.shift_left_12(state.value);
        let clmul = ops.carryless_mul_11(shifted, coeff_vector_high);
        (clmul, masked)
    } else {
        let mask2 = ops.load_aligned(&[0xFFFFFFFFFFFFFFFF, 0x00000000FFFFFFFF]);
        let masked = ops.and_vectors(state.value, mask2);
        let shifted = ops.shift_right_12(state.value);
        let clmul = ops.carryless_mul_10(shifted, coeff_vector_high);
        (clmul, masked)
    };

    state.value = ops.xor_vectors(clmul, masked);
}

/// Barrett reduction for 32-bit-space CRC operations
/// Returns the full u64 result; caller extracts appropriate bits
#[inline(always)]
pub unsafe fn barrett_reduction<T: ArchOps>(
    state: &CrcState<T::Vector>,
    poly: u64,
    mu: u64,
    ops: &T,
) -> [u64; 2]
where
    T::Vector: Copy,
{
    let x = state.value;
    let mu_poly = ops.create_vector_from_u64_pair_non_reflected(poly, mu);

    if state.reflected {
        let clmul1 = ops.carryless_mul_00(x, mu_poly);
        let clmul2 = ops.carryless_mul_10(clmul1, mu_poly);
        let xorred = ops.xor_vectors(x, clmul2);
        ops.extract_u64s(xorred)
    } else {
        let clmul1 = ops.shift_left_4(ops.carryless_mul_01(x, mu_poly));
        let clmul2_shifted = ops.shift_left_4(ops.carryless_mul_11(clmul1, mu_poly));
        let final_xor = ops.xor_vectors(clmul2_shifted, x);
        ops.extract_u64s(final_xor)
    }
}

/// Create coefficient vector for 32-bit-space CRC operations
#[inline(always)]
pub unsafe fn create_coefficient<T: ArchOps>(
    high: u64,
    low: u64,
    ops: &T,
) -> T::Vector
where
    T::Vector: Copy,
{
    ops.create_vector_from_u64_pair_non_reflected(high, low)
}

/// Get shuffle table pointer for 32-bit-space CRC operations
#[inline(always)]
pub fn get_last_bytes_table_ptr(reflected: bool, remaining_len: usize) -> (*const u8, usize) {
    if reflected {
        let base_ptr = &PSHUFB_SHF_TABLE_REVERSE as *const _ as *const u8;
        (base_ptr, remaining_len)
    } else {
        let base_ptr = &PSHUFB_SHF_TABLE_FORWARD as *const _ as *const u8;
        (base_ptr, 16 - remaining_len)
    }
}

/// Process inputs smaller than 16 bytes for 32-bit-space CRC operations
#[inline]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "ssse3,sse4.1,pclmulqdq")
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
pub unsafe fn process_0_to_15<T: ArchOps, W: EnhancedCrcWidth>(
    data: &[u8],
    state: &mut CrcState<T::Vector>,
    reflector: &Reflector<T::Vector>,
    keys: [u64; 23],
    ops: &T,
) -> W::Value
where
    T::Vector: Copy,
{
    // Implementation shared by CRC-16 and CRC-32
    // ... (full implementation)
}
```

### 2. Updated `src/crc16/algorithm.rs`

The Width16 implementation delegates to shared operations:

```rust
impl EnhancedCrcWidth for Width16 {
    #[inline(always)]
    fn load_constants(reflected: bool) -> [[u64; 2]; 4] {
        crate::crc32::width32_ops::load_constants(reflected)
    }

    #[inline(always)]
    unsafe fn fold_16<T: ArchOps>(...) {
        crate::crc32::width32_ops::fold_16(state, coeff, data_to_xor, ops)
    }

    #[inline(always)]
    unsafe fn barrett_reduction<T: ArchOps>(...) -> Self::Value {
        let u64s = crate::crc32::width32_ops::barrett_reduction(state, poly, mu, ops);
        // Extract 16-bit result
        if state.reflected {
            u64s[1] as u16
        } else {
            ((u64s[0] >> 32) >> 16) as u16
        }
    }
    
    // ... other delegating methods
}

// process_0_to_15 delegates to shared implementation
pub(crate) unsafe fn process_0_to_15<T: ArchOps, W: EnhancedCrcWidth>(...) -> W::Value {
    crate::crc32::width32_ops::process_0_to_15::<T, W>(data, state, reflector, keys, ops)
}
```

### 3. Updated `src/crc32/algorithm.rs`

The Width32 implementation also delegates to shared operations:

```rust
impl EnhancedCrcWidth for Width32 {
    #[inline(always)]
    fn load_constants(reflected: bool) -> [[u64; 2]; 4] {
        crate::crc32::width32_ops::load_constants(reflected)
    }

    #[inline(always)]
    unsafe fn barrett_reduction<T: ArchOps>(...) -> Self::Value {
        let u64s = crate::crc32::width32_ops::barrett_reduction(state, poly, mu, ops);
        // Extract 32-bit result
        if state.reflected {
            u64s[1] as u32
        } else {
            (u64s[0] >> 32) as u32
        }
    }
    
    // ... other delegating methods
}

// process_0_to_15 delegates to shared implementation
pub(crate) unsafe fn process_0_to_15<T: ArchOps, W: EnhancedCrcWidth>(...) -> W::Value {
    crate::crc32::width32_ops::process_0_to_15::<T, W>(data, state, reflector, keys, ops)
}
```

### 4. Updated `src/crc32/mod.rs`

Export the new module:

```rust
pub mod algorithm;
pub mod consts;
pub(crate) mod width32_ops;  // NEW
```

## Data Models

No changes to data models. The refactoring only affects internal implementation.

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system.*

### Property 1: CRC-16 Computation Unchanged

*For any* input byte sequence and any CRC-16 parameters, the computed checksum after refactoring SHALL equal the checksum computed before refactoring.

**Validates: Requirements 4.1**

### Property 2: CRC-32 Computation Unchanged

*For any* input byte sequence and any CRC-32 parameters, the computed checksum after refactoring SHALL equal the checksum computed before refactoring.

**Validates: Requirements 4.2**

### Property 3: CRC-64 Computation Unchanged

*For any* input byte sequence and any CRC-64 parameters, the computed checksum after refactoring SHALL equal the checksum computed before refactoring.

**Validates: Requirements 4.3**

## Error Handling

No changes to error handling. The refactoring is purely internal.

## Testing Strategy

### Existing Tests

All existing tests serve as regression tests:
- CRC-16 check value tests (0x906E, 0xD0DB)
- CRC-32 check value tests
- CRC-64 check value tests
- Property-based tests comparing against `crc` crate
- Length-based tests for various input sizes

### Verification Approach

1. Run `cargo test` before refactoring to establish baseline
2. Perform refactoring
3. Run `cargo test` after refactoring to verify no regressions
4. Run `cargo fmt --check` and `cargo clippy -- -D warnings`

No new tests are needed since this is a pure refactoring with no behavior changes.
