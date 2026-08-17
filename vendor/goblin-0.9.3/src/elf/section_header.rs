macro_rules! elf_section_header {
    ($size:ident) => {
        // XXX: Do not import scroll traits here.
        // See: https://github.com/rust-lang/rust/issues/65090#issuecomment-538668155

        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq, Default)]
        #[cfg_attr(
            feature = "alloc",
            derive(scroll::Pread, scroll::Pwrite, scroll::SizeWith)
        )]
        /// Section Headers are typically used by humans and static linkers for additional information or how to relocate the object
        ///
        /// **NOTE** section headers are strippable from a binary without any loss of portability/executability; _do not_ rely on them being there!
        pub struct SectionHeader {
            /// Section name (string tbl index)
            pub sh_name: u32,
            /// Section type
            pub sh_type: u32,
            /// Section flags
            pub sh_flags: $size,
            /// Section virtual addr at execution
            pub sh_addr: $size,
            /// Section file offset
            pub sh_offset: $size,
            /// Section size in bytes
            pub sh_size: $size,
            /// Link to another section
            pub sh_link: u32,
            /// Additional section information
            pub sh_info: u32,
            /// Section alignment
            pub sh_addralign: $size,
            /// Entry size if section holds table
            pub sh_entsize: $size,
        }

        use plain;
        // Declare that this is a plain type.
        unsafe impl plain::Plain for SectionHeader {}

        impl ::core::fmt::Debug for SectionHeader {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct("SectionHeader")
                    .field("sh_name", &self.sh_name)
                    .field("sh_type", &sht_to_str(self.sh_type))
                    .field("sh_flags", &format_args!("0x{:x}", self.sh_flags))
                    .field("sh_addr", &format_args!("0x{:x}", self.sh_addr))
                    .field("sh_offset", &format_args!("0x{:x}", self.sh_offset))
                    .field("sh_size", &format_args!("0x{:x}", self.sh_size))
                    .field("sh_link", &format_args!("0x{:x}", self.sh_link))
                    .field("sh_info", &format_args!("0x{:x}", self.sh_info))
                    .field("sh_addralign", &format_args!("0x{:x}", self.sh_addralign))
                    .field("sh_entsize", &format_args!("0x{:x}", self.sh_entsize))
                    .finish()
            }
        }
    };
}

/// Undefined section.
pub const SHN_UNDEF: u32 = 0;
/// Start of reserved indices.
pub const SHN_LORESERVE: u32 = 0xff00;
/// Start of processor-specific.
pub const SHN_LOPROC: u32 = 0xff00;
/// Order section before all others (Solaris).
pub const SHN_BEFORE: u32 = 0xff00;
/// Order section after all others (Solaris).
pub const SHN_AFTER: u32 = 0xff01;
/// End of processor-specific.
pub const SHN_HIPROC: u32 = 0xff1f;
/// Start of OS-specific.
pub const SHN_LOOS: u32 = 0xff20;
/// End of OS-specific.
pub const SHN_HIOS: u32 = 0xff3f;
/// Associated symbol is absolute.
pub const SHN_ABS: u32 = 0xfff1;
/// Associated symbol is common.
pub const SHN_COMMON: u32 = 0xfff2;
/// Index is in extra table.
pub const SHN_XINDEX: u32 = 0xffff;
/// End of reserved indices.
pub const SHN_HIRESERVE: u32 = 0xffff;

// === Legal values for sh_type (section type). ===
/// Section header table entry unused.
pub const SHT_NULL: u32 = 0;
/// Program data.
pub const SHT_PROGBITS: u32 = 1;
/// Symbol table.
pub const SHT_SYMTAB: u32 = 2;
/// String table.
pub const SHT_STRTAB: u32 = 3;
/// Relocation entries with addends.
pub const SHT_RELA: u32 = 4;
/// Symbol hash table.
pub const SHT_HASH: u32 = 5;
/// Dynamic linking information.
pub const SHT_DYNAMIC: u32 = 6;
/// Notes.
pub const SHT_NOTE: u32 = 7;
/// Program space with no data (bss).
pub const SHT_NOBITS: u32 = 8;
/// Relocation entries, no addends.
pub const SHT_REL: u32 = 9;
/// Reserved.
pub const SHT_SHLIB: u32 = 10;
/// Dynamic linker symbol table.
pub const SHT_DYNSYM: u32 = 11;
/// Array of constructors.
pub const SHT_INIT_ARRAY: u32 = 14;
/// Array of destructors.
pub const SHT_FINI_ARRAY: u32 = 15;
/// Array of pre-constructors.
pub const SHT_PREINIT_ARRAY: u32 = 16;
/// Section group.
pub const SHT_GROUP: u32 = 17;
/// Extended section indeces.
pub const SHT_SYMTAB_SHNDX: u32 = 18;
/// Number of defined types.
pub const SHT_NUM: u32 = 19;
/// Start OS-specific.
pub const SHT_LOOS: u32 = 0x6000_0000;
/// Object attributes.
pub const SHT_GNU_ATTRIBUTES: u32 = 0x6fff_fff5;
/// GNU-style hash table.
pub const SHT_GNU_HASH: u32 = 0x6fff_fff6;
/// Prelink library list.
pub const SHT_GNU_LIBLIST: u32 = 0x6fff_fff7;
/// Checksum for DSO content.
pub const SHT_CHECKSUM: u32 = 0x6fff_fff8;
/// Sun-specific low bound.
pub const SHT_LOSUNW: u32 = 0x6fff_fffa;
pub const SHT_SUNW_MOVE: u32 = 0x6fff_fffa;
pub const SHT_SUNW_COMDAT: u32 = 0x6fff_fffb;
pub const SHT_SUNW_SYMINFO: u32 = 0x6fff_fffc;
/// Version definition section.
pub const SHT_GNU_VERDEF: u32 = 0x6fff_fffd;
/// Version needs section.
pub const SHT_GNU_VERNEED: u32 = 0x6fff_fffe;
/// Version symbol table.
pub const SHT_GNU_VERSYM: u32 = 0x6fff_ffff;
/// Sun-specific high bound.
pub const SHT_HISUNW: u32 = 0x6fff_ffff;
/// End OS-specific type.
pub const SHT_HIOS: u32 = 0x6fff_ffff;
/// Start of processor-specific.
pub const SHT_LOPROC: u32 = 0x7000_0000;
/// X86-64 unwind information.
pub const SHT_X86_64_UNWIND: u32 = 0x7000_0001;
/// End of processor-specific.
pub const SHT_HIPROC: u32 = 0x7fff_ffff;
/// Start of application-specific.
pub const SHT_LOUSER: u32 = 0x8000_0000;
/// End of application-specific.
pub const SHT_HIUSER: u32 = 0x8fff_ffff;

// Legal values for sh_flags (section flags)
/// Writable.
pub const SHF_WRITE: u32 = 0x1;
/// Occupies memory during execution.
pub const SHF_ALLOC: u32 = 0x2;
/// Executable.
pub const SHF_EXECINSTR: u32 = 0x4;
/// Might be merged.
pub const SHF_MERGE: u32 = 0x10;
/// Contains nul-terminated strings.
pub const SHF_STRINGS: u32 = 0x20;
/// `sh_info' contains SHT index.
pub const SHF_INFO_LINK: u32 = 0x40;
/// Preserve order after combining.
pub const SHF_LINK_ORDER: u32 = 0x80;
/// Non-standard OS specific handling required.
pub const SHF_OS_NONCONFORMING: u32 = 0x100;
/// Section is member of a group.
pub const SHF_GROUP: u32 = 0x200;
/// Section hold thread-local data.
pub const SHF_TLS: u32 = 0x400;
/// Section with compressed data.
pub const SHF_COMPRESSED: u32 = 0x800;
/// OS-specific..
pub const SHF_MASKOS: u32 = 0x0ff0_0000;
/// Processor-specific.
pub const SHF_MASKPROC: u32 = 0xf000_0000;
/// Special ordering requirement (Solaris).
pub const SHF_ORDERED: u32 = 1 << 30;
/// Number of "regular" section header flags
pub const SHF_NUM_REGULAR_FLAGS: usize = 12;
/// Section is excluded unless referenced or allocated (Solaris).
pub const SHF_EXCLUDE: u32 = 0x80000000; // 1U << 31

pub const SHF_FLAGS: [u32; SHF_NUM_REGULAR_FLAGS] = [
    SHF_WRITE,
    SHF_ALLOC,
    SHF_EXECINSTR,
    SHF_MERGE,
    SHF_STRINGS,
    SHF_INFO_LINK,
    SHF_LINK_ORDER,
    SHF_OS_NONCONFORMING,
    SHF_GROUP,
    SHF_TLS,
    SHF_COMPRESSED,
    SHF_ORDERED,
];

pub fn sht_to_str(sht: u32) -> &'static str {
    match sht {
        SHT_NULL => "SHT_NULL",
        SHT_PROGBITS => "SHT_PROGBITS",
        SHT_SYMTAB => "SHT_SYMTAB",
        SHT_STRTAB => "SHT_STRTAB",
        SHT_RELA => "SHT_RELA",
        SHT_HASH => "SHT_HASH",
        SHT_DYNAMIC => "SHT_DYNAMIC",
        SHT_NOTE => "SHT_NOTE",
        SHT_NOBITS => "SHT_NOBITS",
        SHT_REL => "SHT_REL",
        SHT_SHLIB => "SHT_SHLIB",
        SHT_DYNSYM => "SHT_DYNSYM",
        SHT_INIT_ARRAY => "SHT_INIT_ARRAY",
        SHT_FINI_ARRAY => "SHT_FINI_ARRAY",
        SHT_PREINIT_ARRAY => "SHT_PREINIT_ARRAY",
        SHT_GROUP => "SHT_GROUP",
        SHT_SYMTAB_SHNDX => "SHT_SYMTAB_SHNDX",
        SHT_NUM => "SHT_NUM",
        SHT_LOOS => "SHT_LOOS",
        SHT_GNU_ATTRIBUTES => "SHT_GNU_ATTRIBUTES",
        SHT_GNU_HASH => "SHT_GNU_HASH",
        SHT_GNU_LIBLIST => "SHT_GNU_LIBLIST",
        SHT_CHECKSUM => "SHT_CHECKSUM",
        SHT_SUNW_MOVE => "SHT_SUNW_MOVE",
        SHT_SUNW_COMDAT => "SHT_SUNW_COMDAT",
        SHT_SUNW_SYMINFO => "SHT_SUNW_SYMINFO",
        SHT_GNU_VERDEF => "SHT_GNU_VERDEF",
        SHT_GNU_VERNEED => "SHT_GNU_VERNEED",
        SHT_GNU_VERSYM => "SHT_GNU_VERSYM",
        SHT_LOPROC => "SHT_LOPROC",
        SHT_X86_64_UNWIND => "SHT_X86_64_UNWIND",
        SHT_HIPROC => "SHT_HIPROC",
        SHT_LOUSER => "SHT_LOUSER",
        SHT_HIUSER => "SHT_HIUSER",
        _ => "UNKNOWN_SHT",
    }
}

pub fn shf_to_str(shf: u32) -> &'static str {
    match shf {
        SHF_WRITE => "SHF_WRITE",
        SHF_ALLOC => "SHF_ALLOC",
        SHF_EXECINSTR => "SHF_EXECINSTR",
        SHF_MERGE => "SHF_MERGE",
        SHF_STRINGS => "SHF_STRINGS",
        SHF_INFO_LINK => "SHF_INFO_LINK",
        SHF_LINK_ORDER => "SHF_LINK_ORDER",
        SHF_OS_NONCONFORMING => "SHF_OS_NONCONFORMING",
        SHF_GROUP => "SHF_GROUP",
        SHF_TLS => "SHF_TLS",
        SHF_COMPRESSED => "SHF_COMPRESSED",
        //SHF_MASKOS..SHF_MASKPROC => "SHF_OSFLAG",
        SHF_ORDERED => "SHF_ORDERED",
        _ => "SHF_UNKNOWN",
    }
}

macro_rules! elf_section_header_std_impl { ($size:ty) => {

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn size_of() {
            assert_eq!(::std::mem::size_of::<SectionHeader>(), SIZEOF_SHDR);
        }
    }

    if_alloc! {
        use crate::elf::section_header::SectionHeader as ElfSectionHeader;

        use plain::Plain;
        use alloc::vec::Vec;

        if_std! {
            use crate::error::Result;

            use std::fs::File;
            use std::io::{Read, Seek};
            use std::io::SeekFrom::Start;
        }

        impl From<SectionHeader> for ElfSectionHeader {
            fn from(sh: SectionHeader) -> Self {
                ElfSectionHeader {
                    sh_name: sh.sh_name as usize,
                    sh_type: sh.sh_type,
                    sh_flags: u64::from(sh.sh_flags),
                    sh_addr: u64::from(sh.sh_addr),
                    sh_offset: u64::from(sh.sh_offset),
                    sh_size: u64::from(sh.sh_size),
                    sh_link: sh.sh_link,
                    sh_info: sh.sh_info,
                    sh_addralign: u64::from(sh.sh_addralign),
                    sh_entsize: u64::from(sh.sh_entsize),
                }
            }
        }
        impl From<ElfSectionHeader> for SectionHeader {
            fn from(sh: ElfSectionHeader) -> Self {
                SectionHeader {
                    sh_name     : sh.sh_name as u32,
                    sh_type     : sh.sh_type,
                    sh_flags    : sh.sh_flags as $size,
                    sh_addr     : sh.sh_addr as $size,
                    sh_offset   : sh.sh_offset as $size,
                    sh_size     : sh.sh_size as $size,
                    sh_link     : sh.sh_link,
                    sh_info     : sh.sh_info,
                    sh_addralign: sh.sh_addralign as $size,
                    sh_entsize  : sh.sh_entsize as $size,
                }
            }
        }

        impl SectionHeader {
            // FIXME: > 65535 sections
            pub fn from_bytes(bytes: &[u8], shnum: usize) -> Vec<SectionHeader> {
                let mut shdrs = vec![SectionHeader::default(); shnum];
                shdrs.copy_from_bytes(bytes).expect("buffer is too short for given number of entries");
                shdrs
            }

            #[cfg(feature = "std")]
            // FIXME: > 65535 sections
            pub fn from_fd(fd: &mut File, offset: u64, shnum: usize) -> Result<Vec<SectionHeader>> {
                let mut shdrs = vec![SectionHeader::default(); shnum];
                fd.seek(Start(offset))?;
                unsafe {
                    fd.read_exact(plain::as_mut_bytes(&mut *shdrs))?;
                }
                Ok(shdrs)
            }
        }
    } // end if_alloc
};}

pub mod section_header32 {
    pub use crate::elf::section_header::*;

    elf_section_header!(u32);

    pub const SIZEOF_SHDR: usize = 40;

    elf_section_header_std_impl!(u32);
}

pub mod section_header64 {

    pub use crate::elf::section_header::*;

    elf_section_header!(u64);

    pub const SIZEOF_SHDR: usize = 64;

    elf_section_header_std_impl!(u64);
}

///////////////////////////////
// Std/analysis/Unified Structs
///////////////////////////////

if_alloc! {
    use crate::error;
    use core::fmt;
    use core::result;
    use core::ops::Range;
    use scroll::ctx;
    use crate::container::{Container, Ctx};

    #[cfg(feature = "endian_fd")]
    use alloc::vec::Vec;

    #[derive(Default, PartialEq, Clone)]
    /// A unified SectionHeader - convertable to and from 32-bit and 64-bit variants
    pub struct SectionHeader {
        /// Section name (string tbl index)
        pub sh_name: usize,
        /// Section type
        pub sh_type: u32,
        /// Section flags
        pub sh_flags: u64,
        /// Section virtual addr at execution
        pub sh_addr: u64,
        /// Section file offset
        pub sh_offset: u64,
        /// Section size in bytes
        pub sh_size: u64,
        /// Link to another section
        pub sh_link: u32,
        /// Additional section information
        pub sh_info: u32,
        /// Section alignment
        pub sh_addralign: u64,
        /// Entry size if section holds table
        pub sh_entsize: u64,
    }

    impl SectionHeader {
        /// Return the size of the underlying section header, given a `Ctx`
        #[inline]
        pub fn size(ctx: Ctx) -> usize {
            use scroll::ctx::SizeWith;
            Self::size_with(&ctx)
        }
        pub fn new() -> Self {
            SectionHeader {
                sh_name: 0,
                sh_type: SHT_PROGBITS,
                sh_flags: u64::from(SHF_ALLOC),
                sh_addr: 0,
                sh_offset: 0,
                sh_size: 0,
                sh_link: 0,
                sh_info: 0,
                sh_addralign: 2 << 8,
                sh_entsize: 0,
            }
        }
        /// Returns this section header's file offset range,
        /// if the section occupies space in fhe file.
        pub fn file_range(&self) -> Option<Range<usize>> {
            // Sections with type SHT_NOBITS have no data in the file itself,
            // they only exist in memory.
            if self.sh_type == SHT_NOBITS {
                None
            } else {
                Some(self.sh_offset as usize..(self.sh_offset as usize).saturating_add(self.sh_size as usize))
            }
        }
        /// Returns this section header's virtual memory range
        pub fn vm_range(&self) -> Range<usize> {
            self.sh_addr as usize..(self.sh_addr as usize).saturating_add(self.sh_size as usize)
        }
        /// Parse `count` section headers from `bytes` at `offset`, using the given `ctx`
        /// Assuming this is read from the whole file, it will check offset.
        #[cfg(feature = "endian_fd")]
        pub fn parse(bytes: &[u8], offset: usize, count: usize, ctx: Ctx) -> error::Result<Vec<SectionHeader>> {
            // Zero offset means no section headers, not even the null section header.
            if offset == 0 {
                return Ok(Vec::new());
            }
            Self::parse_from(bytes, offset, count, ctx)
        }
        /// Parse `count` section headers from `bytes` at `offset`, using the given `ctx`
        /// without performing any offset checking to allow parsing relatively
        #[cfg(feature = "endian_fd")]
        pub fn parse_from(bytes: &[u8], mut offset: usize, mut count: usize, ctx: Ctx) -> error::Result<Vec<SectionHeader>> {
            use scroll::Pread;
            let empty_sh = bytes.gread_with::<SectionHeader>(&mut offset, ctx)?;
            if count == 0 as usize {
                // Zero count means either no section headers or the number of section headers
                // overflows SHN_LORESERVE, in which case the count is stored in the sh_size field
                // of the null section header.
                count = empty_sh.sh_size as usize;
            }

            // Sanity check to avoid OOM
            if count > bytes.len() / Self::size(ctx) {
                return Err(error::Error::BufferTooShort(count, "section headers"));
            }
            let mut section_headers = Vec::with_capacity(count);
            section_headers.push(empty_sh);
            for _ in 1..count {
                let shdr = bytes.gread_with(&mut offset, ctx)?;
                section_headers.push(shdr);
            }
            Ok(section_headers)
        }
        pub fn check_size(&self, size: usize) -> error::Result<()> {
            if self.sh_type == SHT_NOBITS || self.sh_size == 0 {
                return Ok(());
            }
            let (end, overflow) = self.sh_offset.overflowing_add(self.sh_size);
            if overflow || end > size as u64 {
                let message = format!("Section {} size ({}) + offset ({}) is out of bounds. Overflowed: {}",
                    self.sh_name, self.sh_offset, self.sh_size, overflow);
                return Err(error::Error::Malformed(message));
            }
            let (_, overflow) = self.sh_addr.overflowing_add(self.sh_size);
            if overflow {
                let message = format!("Section {} size ({}) + addr ({}) is out of bounds. Overflowed: {}",
                    self.sh_name, self.sh_addr, self.sh_size, overflow);
                return Err(error::Error::Malformed(message));
            }
            Ok(())
        }
        pub fn is_relocation(&self) -> bool {
            self.sh_type == SHT_RELA
        }
        pub fn is_executable(&self) -> bool {
            self.is_alloc() && self.sh_flags as u32 & SHF_EXECINSTR == SHF_EXECINSTR
        }
        pub fn is_writable(&self) -> bool {
            self.is_alloc() && self.sh_flags as u32 & SHF_WRITE == SHF_WRITE
        }
        pub fn is_alloc(&self) -> bool {
            self.sh_flags as u32 & SHF_ALLOC == SHF_ALLOC
        }
    }

    impl fmt::Debug for SectionHeader {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("SectionHeader")
                .field("sh_name", &self.sh_name)
                .field("sh_type", &sht_to_str(self.sh_type))
                .field("sh_flags", &format_args!("0x{:x}", self.sh_flags))
                .field("sh_addr", &format_args!("0x{:x}", self.sh_addr))
                .field("sh_offset", &format_args!("0x{:x}", self.sh_offset))
                .field("sh_size", &format_args!("0x{:x}", self.sh_size))
                .field("sh_link", &format_args!("0x{:x}", self.sh_link))
                .field("sh_info", &format_args!("0x{:x}", self.sh_info))
                .field("sh_addralign", &format_args!("0x{:x}", self.sh_addralign))
                .field("sh_entsize", &format_args!("0x{:x}", self.sh_entsize))
                .finish()
        }
    }

    impl ctx::SizeWith<Ctx> for SectionHeader {
        fn size_with( &Ctx { container, .. }: &Ctx) -> usize {
            match container {
                Container::Little => {
                    section_header32::SIZEOF_SHDR
                },
                Container::Big => {
                    section_header64::SIZEOF_SHDR
                },
            }
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Ctx> for SectionHeader {
        type Error = crate::error::Error;
        fn try_from_ctx(bytes: &'a [u8], Ctx {container, le}: Ctx) -> result::Result<(Self, usize), Self::Error> {
            use scroll::Pread;
            let res = match container {
                Container::Little => {
                    (bytes.pread_with::<section_header32::SectionHeader>(0, le)?.into(), section_header32::SIZEOF_SHDR)
                },
                Container::Big => {
                    (bytes.pread_with::<section_header64::SectionHeader>(0, le)?.into(), section_header64::SIZEOF_SHDR)
                }
            };
            Ok(res)
        }
    }

    impl ctx::TryIntoCtx<Ctx> for SectionHeader {
        type Error = crate::error::Error;
        fn try_into_ctx(self, bytes: &mut [u8], Ctx {container, le}: Ctx) -> result::Result<usize, Self::Error> {
            use scroll::Pwrite;
            match container {
                Container::Little => {
                    let shdr: section_header32::SectionHeader = self.into();
                    Ok(bytes.pwrite_with(shdr, 0, le)?)
                },
                Container::Big => {
                    let shdr: section_header64::SectionHeader = self.into();
                    Ok(bytes.pwrite_with(shdr, 0, le)?)
                }
            }
        }
    }
    impl ctx::IntoCtx<Ctx> for SectionHeader {
        fn into_ctx(self, bytes: &mut [u8], Ctx {container, le}: Ctx) {
            use scroll::Pwrite;
            match container {
                Container::Little => {
                    let shdr: section_header32::SectionHeader = self.into();
                    bytes.pwrite_with(shdr, 0, le).unwrap();
                },
                Container::Big => {
                    let shdr: section_header64::SectionHeader = self.into();
                    bytes.pwrite_with(shdr, 0, le).unwrap();
                }
            }
        }
    }
} // end if_alloc
