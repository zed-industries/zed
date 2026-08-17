# CRC Width-32 Shared Base Refactoring

## Goal

Refactor the CRC-16 and CRC-32 algorithm implementations to share a common base, reducing code duplication and preparing for future CRC width additions (like CRC-8).

## Background

CRC-16 computation is performed by scaling 16-bit values to 32-bit space, using the CRC-32 algorithm infrastructure, and then scaling the result back to 16 bits. This means CRC-16 and CRC-32 share nearly identical implementations for:

- `process_0_to_15()` function (~100 lines each, nearly identical)
- `load_constants()` - identical constants
- `fold_16()` - identical logic
- `fold_width()` - identical logic
- `barrett_reduction()` - nearly identical (just different final extraction)
- `create_coefficient()` - identical
- `perform_final_reduction()` - identical
- `get_last_bytes_table_ptr()` - identical

## Approach

Create a shared module (`src/crc32/width32_ops.rs`) containing the common 32-bit-space operations that both CRC-16 and CRC-32 use. The Width16 and Width32 trait implementations will delegate to these shared functions, with Width16 performing the final 16-bit extraction where needed.

## Constraints

- Must maintain full backwards compatibility with existing CRC-16, CRC-32, and CRC-64 functionality
- All existing tests must continue to pass
- The refactoring should make it easier to add future CRC widths (like CRC-8)
