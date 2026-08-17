//! This module contains constants that are shared between the codegen and the meta crate, so they
//! are kept in sync.

// Numbering scheme for value types:
//
// 0: Void
// 0x01-0x6f: Special types
// 0x70-0x7d: Lane types
// 0x7e-0x7f: Reference types
// 0x80-0xff: Vector types
// 0x100-0x17f: Dynamic Vector types
//
// Vector types are encoded with the lane type in the low 4 bits and log2(lanes)
// in the next highest 4 bits, giving a range of 2-256 lanes.

// Dynamic vector types are encoded similarly.

/// Start of the lane types.
pub const LANE_BASE: u16 = 0x70;

/// Base for reference types.
pub const REFERENCE_BASE: u16 = 0x7E;

/// Start of the 2-lane vector types.
pub const VECTOR_BASE: u16 = 0x80;

/// Start of the dynamic vector types.
pub const DYNAMIC_VECTOR_BASE: u16 = 0x100;
