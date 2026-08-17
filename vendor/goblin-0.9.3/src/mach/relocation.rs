// Format of a relocation entry of a Mach-O file.  Modified from the 4.3BSD
// format.  The modifications from the original format were changing the value
// of the r_symbolnum field for "local" (r_extern == 0) relocation entries.
// This modification is required to support symbols in an arbitrary number of
// sections not just the three sections (text, data and bss) in a 4.3BSD file.
// Also the last 4 bits have had the r_type tag added to them.

// The r_address is not really the address as it's name indicates but an offset.
// In 4.3BSD a.out objects this offset is from the start of the "segment" for
// which relocation entry is for (text or data).  For Mach-O object files it is
// also an offset but from the start of the "section" for which the relocation
// entry is for.  See comments in <mach-o/loader.h> about the r_address feild
// in images for used with the dynamic linker.

// In 4.3BSD a.out objects if r_extern is zero then r_symbolnum is an ordinal
// for the segment the symbol being relocated is in.  These ordinals are the
// symbol types N_TEXT, N_DATA, N_BSS or N_ABS.  In Mach-O object files these
// ordinals refer to the sections in the object file in the order their section
// structures appear in the headers of the object file they are in.  The first
// section has the ordinal 1, the second 2, and so on.  This means that the
// same ordinal in two different object files could refer to two different
// sections.  And further could have still different ordinals when combined
// by the link-editor.  The value R_ABS is used for relocation entries for
// absolute symbols which need no further relocation.
use crate::mach;
use core::fmt;
use scroll::{IOread, IOwrite, Pread, Pwrite, SizeWith};

// TODO: armv7 relocations are scattered, must and r_address with 0x8000_0000 to check if its scattered or not
#[derive(Copy, Clone, Pread, Pwrite, IOwrite, SizeWith, IOread)]
#[repr(C)]
pub struct RelocationInfo {
    /// Offset in the section to what is being relocated
    pub r_address: i32,
    /// Contains all of the relocation info as a bitfield.
    /// r_symbolnum, 24 bits, r_pcrel 1 bit, r_length 2 bits, r_extern 1 bit, r_type 4 bits
    pub r_info: u32,
}

pub const SIZEOF_RELOCATION_INFO: usize = 8;

impl RelocationInfo {
    /// Symbol index if `r_extern` == 1 or section ordinal if `r_extern` == 0. In bits :24
    #[inline]
    pub fn r_symbolnum(self) -> usize {
        (self.r_info & 0x00ff_ffffu32) as usize
    }
    /// Was relocated pc relative already, 1 bit
    #[inline]
    pub fn r_pcrel(self) -> u8 {
        ((self.r_info & 0x0100_0000u32) >> 24) as u8
    }
    /// The length of the relocation, 0=byte, 1=word, 2=long, 3=quad, 2 bits
    #[inline]
    pub fn r_length(self) -> u8 {
        ((self.r_info & 0x0600_0000u32) >> 25) as u8
    }
    /// Does not include value of sym referenced, 1 bit
    #[inline]
    pub fn r_extern(self) -> u8 {
        ((self.r_info & 0x0800_0000) >> 27) as u8
    }
    /// Ff not 0, machine specific relocation type, in bits :4
    #[inline]
    pub fn r_type(self) -> u8 {
        ((self.r_info & 0xf000_0000) >> 28) as u8
    }
    /// If true, this relocation is for a symbol; if false,  or a section ordinal otherwise
    #[inline]
    pub fn is_extern(self) -> bool {
        self.r_extern() == 1
    }
    /// If true, this is a PIC relocation
    #[inline]
    pub fn is_pic(self) -> bool {
        self.r_pcrel() > 0
    }
    /// Returns a string representation of this relocation, given the machine `cputype`
    pub fn to_str(self, cputype: mach::cputype::CpuType) -> &'static str {
        reloc_to_str(self.r_type(), cputype)
    }
}

/// Absolute relocation type for Mach-O files
pub const R_ABS: u8 = 0;

impl fmt::Debug for RelocationInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RelocationInfo")
            .field("r_address", &format_args!("{:#x}", &self.r_address))
            .field("r_info", &format_args!("{:#x}", &self.r_info))
            .field("r_symbolnum", &format_args!("{:#x}", &self.r_symbolnum()))
            .field("r_pcrel", &(self.r_pcrel()))
            .field("r_length", &self.r_length())
            .field("r_extern", &self.r_extern())
            .field("r_type", &self.r_type())
            .finish()
    }
}

pub type RelocType = u8;

/// Absolute address
pub const X86_64_RELOC_UNSIGNED: RelocType = 0;
/// Signed 32-bit displacement
pub const X86_64_RELOC_SIGNED: RelocType = 1;
/// A CALL/JMP instruction with 32-bit displacement
pub const X86_64_RELOC_BRANCH: RelocType = 2;
/// A MOVQ load of a GOT entry
pub const X86_64_RELOC_GOT_LOAD: RelocType = 3;
/// Other GOT references
pub const X86_64_RELOC_GOT: RelocType = 4;
/// Must be followed by a X86_64_RELOC_UNSIGNED relocation
pub const X86_64_RELOC_SUBTRACTOR: RelocType = 5;
/// for signed 32-bit displacement with a -1 addend
pub const X86_64_RELOC_SIGNED_1: RelocType = 6;
/// for signed 32-bit displacement with a -2 addend
pub const X86_64_RELOC_SIGNED_2: RelocType = 7;
/// for signed 32-bit displacement with a -4 addend
pub const X86_64_RELOC_SIGNED_4: RelocType = 8;
/// for thread local variables
pub const X86_64_RELOC_TLV: RelocType = 9;

// x86 relocations
pub const GENERIC_RELOC_VANILLA: RelocType = 0;
pub const GENERIC_RELOC_PAIR: RelocType = 1;
pub const GENERIC_RELOC_SECTDIFF: RelocType = 2;
pub const GENERIC_RELOC_PB_LA_PTR: RelocType = 3;
pub const GENERIC_RELOC_LOCAL_SECTDIFF: RelocType = 4;
pub const GENERIC_RELOC_TLV: RelocType = 5;

// arm relocations
pub const ARM_RELOC_VANILLA: RelocType = GENERIC_RELOC_VANILLA;
pub const ARM_RELOC_PAIR: RelocType = GENERIC_RELOC_PAIR;
pub const ARM_RELOC_SECTDIFF: RelocType = GENERIC_RELOC_SECTDIFF;
pub const ARM_RELOC_LOCAL_SECTDIFF: RelocType = 3;
pub const ARM_RELOC_PB_LA_PTR: RelocType = 4;
pub const ARM_RELOC_BR24: RelocType = 5;
pub const ARM_THUMB_RELOC_BR22: RelocType = 6;
/// Obsolete
pub const ARM_THUMB_32BIT_BRANCH: RelocType = 7;
pub const ARM_RELOC_HALF: RelocType = 8;
pub const ARM_RELOC_HALF_SECTDIFF: RelocType = 9;

/// For pointers.
pub const ARM64_RELOC_UNSIGNED: RelocType = 0;
/// Must be followed by an ARM64_RELOC_UNSIGNED
pub const ARM64_RELOC_SUBTRACTOR: RelocType = 1;
/// A B/BL instruction with 26-bit displacement.
pub const ARM64_RELOC_BRANCH26: RelocType = 2;
/// PC-rel distance to page of target.
pub const ARM64_RELOC_PAGE21: RelocType = 3;
/// Offset within page, scaled by r_length.
pub const ARM64_RELOC_PAGEOFF12: RelocType = 4;
/// PC-rel distance to page of GOT slot.
pub const ARM64_RELOC_GOT_LOAD_PAGE21: RelocType = 5;
/// Offset within page of GOT slot, scaled by r_length.
pub const ARM64_RELOC_GOT_LOAD_PAGEOFF12: RelocType = 6;
/// For pointers to GOT slots.
pub const ARM64_RELOC_POINTER_TO_GOT: RelocType = 7;
/// PC-rel distance to page of TLVP slot.
pub const ARM64_RELOC_TLVP_LOAD_PAGE21: RelocType = 8;
/// Offset within page of TLVP slot, scaled by r_length.
pub const ARM64_RELOC_TLVP_LOAD_PAGEOFF12: RelocType = 9;
/// Must be followed by ARM64_RELOC_PAGE21 or ARM64_RELOC_PAGEOFF12.
pub const ARM64_RELOC_ADDEND: RelocType = 10;

pub fn reloc_to_str(reloc: RelocType, cputype: mach::cputype::CpuType) -> &'static str {
    use crate::mach::constants::cputype::*;
    match cputype {
        CPU_TYPE_ARM64 | CPU_TYPE_ARM64_32 => match reloc {
            ARM64_RELOC_UNSIGNED => "ARM64_RELOC_UNSIGNED",
            ARM64_RELOC_SUBTRACTOR => "ARM64_RELOC_SUBTRACTOR",
            ARM64_RELOC_BRANCH26 => "ARM64_RELOC_BRANCH26",
            ARM64_RELOC_PAGE21 => "ARM64_RELOC_PAGE21",
            ARM64_RELOC_PAGEOFF12 => "ARM64_RELOC_PAGEOFF12",
            ARM64_RELOC_GOT_LOAD_PAGE21 => "ARM64_RELOC_GOT_LOAD_PAGE21",
            ARM64_RELOC_GOT_LOAD_PAGEOFF12 => "ARM64_RELOC_GOT_LOAD_PAGEOFF12",
            ARM64_RELOC_POINTER_TO_GOT => "ARM64_RELOC_POINTER_TO_GOT",
            ARM64_RELOC_TLVP_LOAD_PAGE21 => "ARM64_RELOC_TLVP_LOAD_PAGE21",
            ARM64_RELOC_TLVP_LOAD_PAGEOFF12 => "ARM64_RELOC_TLVP_LOAD_PAGEOFF12",
            ARM64_RELOC_ADDEND => "ARM64_RELOC_ADDEND",
            _ => "UNKNOWN",
        },
        CPU_TYPE_X86_64 => match reloc {
            X86_64_RELOC_UNSIGNED => "X86_64_RELOC_UNSIGNED",
            X86_64_RELOC_SIGNED => "X86_64_RELOC_SIGNED",
            X86_64_RELOC_BRANCH => "X86_64_RELOC_BRANCH",
            X86_64_RELOC_GOT_LOAD => "X86_64_RELOC_GOT_LOAD",
            X86_64_RELOC_GOT => "X86_64_RELOC_GOT",
            X86_64_RELOC_SUBTRACTOR => "X86_64_RELOC_SUBTRACTOR",
            X86_64_RELOC_SIGNED_1 => "X86_64_RELOC_SIGNED_1",
            X86_64_RELOC_SIGNED_2 => "X86_64_RELOC_SIGNED_2",
            X86_64_RELOC_SIGNED_4 => "X86_64_RELOC_SIGNED_4",
            X86_64_RELOC_TLV => "X86_64_RELOC_TLV",
            _ => "UNKNOWN",
        },
        CPU_TYPE_ARM => match reloc {
            ARM_RELOC_VANILLA => "ARM_RELOC_VANILLA",
            ARM_RELOC_PAIR => "ARM_RELOC_PAIR",
            ARM_RELOC_SECTDIFF => "ARM_RELOC_SECTDIFF",
            ARM_RELOC_LOCAL_SECTDIFF => "ARM_RELOC_LOCAL_SECTDIFF",
            ARM_RELOC_PB_LA_PTR => "ARM_RELOC_PB_LA_PTR",
            ARM_RELOC_BR24 => "ARM_RELOC_BR24",
            ARM_THUMB_RELOC_BR22 => "ARM_THUMB_RELOC_BR22",
            ARM_THUMB_32BIT_BRANCH => "ARM_THUMB_32BIT_BRANCH",
            ARM_RELOC_HALF => "ARM_RELOC_HALF",
            ARM_RELOC_HALF_SECTDIFF => "ARM_RELOC_HALF_SECTDIFF",
            _ => "UNKNOWN",
        },
        CPU_TYPE_X86 => match reloc {
            GENERIC_RELOC_VANILLA => "GENERIC_RELOC_VANILLA",
            GENERIC_RELOC_PAIR => "GENERIC_RELOC_PAIR",
            GENERIC_RELOC_SECTDIFF => "GENERIC_RELOC_SECTDIFF",
            GENERIC_RELOC_PB_LA_PTR => "GENERIC_RELOC_PB_LA_PTR",
            GENERIC_RELOC_LOCAL_SECTDIFF => "GENERIC_RELOC_LOCAL_SECTDIFF",
            GENERIC_RELOC_TLV => "GENERIC_RELOC_TLV",
            _ => "UNKNOWN",
        },
        _ => "BAD_CPUTYPE",
    }
}
