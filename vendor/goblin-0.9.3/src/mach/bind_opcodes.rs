//! Bind opcodes are interpreted by the dynamic linker to efficiently collect every symbol imported by this binary, and from which library using two-level namespacing
//!
//! Some uses of external symbols do not need to be bound immediately.
//! Instead they can be lazily bound on first use.  The lazy_bind
//! are contains a stream of BIND opcodes to bind all lazy symbols.
//! Normal use is that dyld ignores the lazy_bind section when
//! loading an image.  Instead the static linker arranged for a
//! lazy pointer to initially point to a helper function which
//! pushes the offset into the lazy_bind area for the symbol
//! needing to be bound, then jumps to dyld which simply adds
//! the offset to lazy_bind_off to get the information on what
//! to bind.

pub type Opcode = u8;

// The following are used to encode binding information
pub const BIND_TYPE_POINTER: u8 = 1;
pub const BIND_TYPE_TEXT_ABSOLUTE32: u8 = 2;
pub const BIND_TYPE_TEXT_PCREL32: u8 = 3;
pub const BIND_SPECIAL_DYLIB_SELF: u8 = 0;
pub const BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE: u8 = 0xf; // -1
pub const BIND_SPECIAL_DYLIB_FLAT_LOOKUP: u8 = 0xe; // -2
pub const BIND_SYMBOL_FLAGS_WEAK_IMPORT: u8 = 0x1;
pub const BIND_SYMBOL_FLAGS_NON_WEAK_DEFINITION: u8 = 0x8;
pub const BIND_OPCODE_MASK: u8 = 0xF0;
pub const BIND_IMMEDIATE_MASK: u8 = 0x0F;
pub const BIND_OPCODE_DONE: Opcode = 0x00;
pub const BIND_OPCODE_SET_DYLIB_ORDINAL_IMM: Opcode = 0x10;
pub const BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB: Opcode = 0x20;
pub const BIND_OPCODE_SET_DYLIB_SPECIAL_IMM: Opcode = 0x30;
pub const BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM: Opcode = 0x40;
pub const BIND_OPCODE_SET_TYPE_IMM: Opcode = 0x50;
pub const BIND_OPCODE_SET_ADDEND_SLEB: Opcode = 0x60;
pub const BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB: Opcode = 0x70;
pub const BIND_OPCODE_ADD_ADDR_ULEB: Opcode = 0x80;
pub const BIND_OPCODE_DO_BIND: Opcode = 0x90;
pub const BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB: Opcode = 0xA0;
pub const BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED: Opcode = 0xB0;
pub const BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB: Opcode = 0xC0;

pub fn opcode_to_str(opcode: Opcode) -> &'static str {
    match opcode {
        BIND_OPCODE_DONE => "BIND_OPCODE_DONE",
        BIND_OPCODE_SET_DYLIB_ORDINAL_IMM => "BIND_OPCODE_SET_DYLIB_ORDINAL_IMM",
        BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB => "BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB",
        BIND_OPCODE_SET_DYLIB_SPECIAL_IMM => "BIND_OPCODE_SET_DYLIB_SPECIAL_IMM",
        BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM => "BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM",
        BIND_OPCODE_SET_TYPE_IMM => "BIND_OPCODE_SET_TYPE_IMM",
        BIND_OPCODE_SET_ADDEND_SLEB => "BIND_OPCODE_SET_ADDEND_SLEB",
        BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB => "BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB",
        BIND_OPCODE_ADD_ADDR_ULEB => "BIND_OPCODE_ADD_ADDR_ULEB",
        BIND_OPCODE_DO_BIND => "BIND_OPCODE_DO_BIND",
        BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB => "BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB",
        BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED => "BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED",
        BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB => {
            "BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB"
        }
        _ => "UNKNOWN OPCODE",
    }
}
