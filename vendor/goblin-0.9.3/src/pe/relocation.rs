use crate::error;
use scroll::{IOread, IOwrite, Pread, Pwrite, SizeWith};

/// Size of a single COFF relocation.
pub const COFF_RELOCATION_SIZE: usize = 10;

// x86 relocations.

/// The relocation is ignored.
pub const IMAGE_REL_I386_ABSOLUTE: u16 = 0x0000;
/// Not supported.
pub const IMAGE_REL_I386_DIR16: u16 = 0x0001;
/// Not supported.
pub const IMAGE_REL_I386_REL16: u16 = 0x0002;
/// The target's 32-bit VA.
pub const IMAGE_REL_I386_DIR32: u16 = 0x0006;
/// The target's 32-bit RVA.
pub const IMAGE_REL_I386_DIR32NB: u16 = 0x0007;
/// Not supported.
pub const IMAGE_REL_I386_SEG12: u16 = 0x0009;
/// The 16-bit section index of the section that contains the target.
///
/// This is used to support debugging information.
pub const IMAGE_REL_I386_SECTION: u16 = 0x000A;
/// The 32-bit offset of the target from the beginning of its section.
///
/// This is used to support debugging information and static thread local storage.
pub const IMAGE_REL_I386_SECREL: u16 = 0x000B;
/// The CLR token.
pub const IMAGE_REL_I386_TOKEN: u16 = 0x000C;
/// A 7-bit offset from the base of the section that contains the target.
pub const IMAGE_REL_I386_SECREL7: u16 = 0x000D;
/// The 32-bit relative displacement to the target.
///
/// This supports the x86 relative branch and call instructions.
pub const IMAGE_REL_I386_REL32: u16 = 0x0014;

// x86-64 relocations.

/// The relocation is ignored.
pub const IMAGE_REL_AMD64_ABSOLUTE: u16 = 0x0000;
/// The 64-bit VA of the relocation target.
pub const IMAGE_REL_AMD64_ADDR64: u16 = 0x0001;
/// The 32-bit VA of the relocation target.
pub const IMAGE_REL_AMD64_ADDR32: u16 = 0x0002;
/// The 32-bit address without an image base (RVA).
pub const IMAGE_REL_AMD64_ADDR32NB: u16 = 0x0003;
/// The 32-bit relative address from the byte following the relocation.
pub const IMAGE_REL_AMD64_REL32: u16 = 0x0004;
/// The 32-bit address relative to byte distance 1 from the relocation.
pub const IMAGE_REL_AMD64_REL32_1: u16 = 0x0005;
/// The 32-bit address relative to byte distance 2 from the relocation.
pub const IMAGE_REL_AMD64_REL32_2: u16 = 0x0006;
/// The 32-bit address relative to byte distance 3 from the relocation.
pub const IMAGE_REL_AMD64_REL32_3: u16 = 0x0007;
/// The 32-bit address relative to byte distance 4 from the relocation.
pub const IMAGE_REL_AMD64_REL32_4: u16 = 0x0008;
/// The 32-bit address relative to byte distance 5 from the relocation.
pub const IMAGE_REL_AMD64_REL32_5: u16 = 0x0009;
/// The 16-bit section index of the section that contains the target.
///
/// This is used to support debugging information.
pub const IMAGE_REL_AMD64_SECTION: u16 = 0x000A;
/// The 32-bit offset of the target from the beginning of its section.
///
/// This is used to support debugging information and static thread local storage.
pub const IMAGE_REL_AMD64_SECREL: u16 = 0x000B;
/// A 7-bit unsigned offset from the base of the section that contains the target.
pub const IMAGE_REL_AMD64_SECREL7: u16 = 0x000C;
/// CLR tokens.
pub const IMAGE_REL_AMD64_TOKEN: u16 = 0x000D;
/// A 32-bit signed span-dependent value emitted into the object.
pub const IMAGE_REL_AMD64_SREL32: u16 = 0x000E;
/// A pair that must immediately follow every span-dependent value.
pub const IMAGE_REL_AMD64_PAIR: u16 = 0x000F;
/// A 32-bit signed span-dependent value that is applied at link time.
pub const IMAGE_REL_AMD64_SSPAN32: u16 = 0x0010;

/// A COFF relocation.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct Relocation {
    /// The address of the item to which relocation is applied.
    ///
    /// This is the offset from the beginning of the section, plus the
    /// value of the section's `virtual_address` field.
    pub virtual_address: u32,
    /// A zero-based index into the symbol table.
    ///
    /// This symbol gives the address that is to be used for the relocation. If the specified
    /// symbol has section storage class, then the symbol's address is the address with the
    /// first section of the same name.
    pub symbol_table_index: u32,
    /// A value that indicates the kind of relocation that should be performed.
    ///
    /// Valid relocation types depend on machine type.
    pub typ: u16,
}

/// An iterator for COFF relocations.
#[derive(Default)]
pub struct Relocations<'a> {
    offset: usize,
    relocations: &'a [u8],
}

impl<'a> Relocations<'a> {
    /// Parse a COFF relocation table at the given offset.
    ///
    /// The offset and number of relocations should be from the COFF section header.
    pub fn parse(bytes: &'a [u8], offset: usize, number: usize) -> error::Result<Relocations<'a>> {
        let relocations = bytes.pread_with(offset, number * COFF_RELOCATION_SIZE)?;
        Ok(Relocations {
            offset: 0,
            relocations,
        })
    }
}

impl<'a> Iterator for Relocations<'a> {
    type Item = Relocation;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.relocations.len() {
            None
        } else {
            Some(
                self.relocations
                    .gread_with(&mut self.offset, scroll::LE)
                    .unwrap(),
            )
        }
    }
}
