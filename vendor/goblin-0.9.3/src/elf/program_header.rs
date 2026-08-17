/* Legal values for p_type (segment type).  */

/// Programg header table entry unused
pub const PT_NULL: u32 = 0;
/// Loadable program segment
pub const PT_LOAD: u32 = 1;
/// Dynamic linking information
pub const PT_DYNAMIC: u32 = 2;
/// Program interpreter
pub const PT_INTERP: u32 = 3;
/// Auxiliary information
pub const PT_NOTE: u32 = 4;
/// Reserved
pub const PT_SHLIB: u32 = 5;
/// Entry for header table itself
pub const PT_PHDR: u32 = 6;
/// Thread-local storage segment
pub const PT_TLS: u32 = 7;
/// Number of defined types
pub const PT_NUM: u32 = 8;
/// Start of OS-specific
pub const PT_LOOS: u32 = 0x6000_0000;
/// GCC .eh_frame_hdr segment
pub const PT_GNU_EH_FRAME: u32 = 0x6474_e550;
/// GNU property notes for linker and run-time loaders
pub const PT_GNU_PROPERTY: u32 = 0x6474_e553;
/// Indicates stack executability
pub const PT_GNU_STACK: u32 = 0x6474_e551;
/// Read-only after relocation
pub const PT_GNU_RELRO: u32 = 0x6474_e552;
/// Sun Specific segment
pub const PT_LOSUNW: u32 = 0x6fff_fffa;
/// Sun Specific segment
pub const PT_SUNWBSS: u32 = 0x6fff_fffa;
/// Stack segment
pub const PT_SUNWSTACK: u32 = 0x6fff_fffb;
/// End of OS-specific
pub const PT_HISUNW: u32 = 0x6fff_ffff;
/// End of OS-specific
pub const PT_HIOS: u32 = 0x6fff_ffff;
/// Start of processor-specific
pub const PT_LOPROC: u32 = 0x7000_0000;
/// ARM unwind segment
pub const PT_ARM_EXIDX: u32 = 0x7000_0001;
/// End of processor-specific
pub const PT_HIPROC: u32 = 0x7fff_ffff;

/* Legal values for p_flags (segment flags).  */

/// Segment is executable
pub const PF_X: u32 = 1;
/// Segment is writable
pub const PF_W: u32 = 1 << 1;
/// Segment is readable
pub const PF_R: u32 = 1 << 2;
/// Bits reserved for OS-specific usage
pub const PF_MASKOS: u32 = 0x0ff0_0000;
/// Bits reserved for processor-specific usage
pub const PF_MASKPROC: u32 = 0xf000_0000;

pub fn pt_to_str(pt: u32) -> &'static str {
    match pt {
        PT_NULL => "PT_NULL",
        PT_LOAD => "PT_LOAD",
        PT_DYNAMIC => "PT_DYNAMIC",
        PT_INTERP => "PT_INTERP",
        PT_NOTE => "PT_NOTE",
        PT_SHLIB => "PT_SHLIB",
        PT_PHDR => "PT_PHDR",
        PT_TLS => "PT_TLS",
        PT_NUM => "PT_NUM",
        PT_LOOS => "PT_LOOS",
        PT_GNU_EH_FRAME => "PT_GNU_EH_FRAME",
        PT_GNU_PROPERTY => "PT_GNU_PROPERTY",
        PT_GNU_STACK => "PT_GNU_STACK",
        PT_GNU_RELRO => "PT_GNU_RELRO",
        PT_SUNWBSS => "PT_SUNWBSS",
        PT_SUNWSTACK => "PT_SUNWSTACK",
        PT_HIOS => "PT_HIOS",
        PT_LOPROC => "PT_LOPROC",
        PT_HIPROC => "PT_HIPROC",
        PT_ARM_EXIDX => "PT_ARM_EXIDX",
        _ => "UNKNOWN_PT",
    }
}

if_alloc! {
    use core::fmt;
    use scroll::ctx;
    use core::result;
    use core::ops::Range;
    use crate::container::{Ctx, Container};
    use alloc::vec::Vec;

    #[derive(Default, PartialEq, Clone)]
    /// A unified ProgramHeader - convertable to and from 32-bit and 64-bit variants
    pub struct ProgramHeader {
        pub p_type  : u32,
        pub p_flags : u32,
        pub p_offset: u64,
        pub p_vaddr : u64,
        pub p_paddr : u64,
        pub p_filesz: u64,
        pub p_memsz : u64,
        pub p_align : u64,
    }

    impl ProgramHeader {
        /// Return the size of the underlying program header, given a `Ctx`
        #[inline]
        pub fn size(ctx: Ctx) -> usize {
            use scroll::ctx::SizeWith;
            Self::size_with(&ctx)
        }
        /// Create a new `PT_LOAD` ELF program header
        pub fn new() -> Self {
            ProgramHeader {
                p_type  : PT_LOAD,
                p_flags : 0,
                p_offset: 0,
                p_vaddr : 0,
                p_paddr : 0,
                p_filesz: 0,
                p_memsz : 0,
                //TODO: check if this is true for 32-bit pt_load
                p_align : 2 << 20,
            }
        }
        /// Returns this program header's file offset range
        pub fn file_range(&self) -> Range<usize> {
            self.p_offset as usize..self.p_offset.saturating_add(self.p_filesz) as usize
        }
        /// Returns this program header's virtual memory range
        pub fn vm_range(&self) -> Range<usize> {
            self.p_vaddr as usize..self.p_vaddr.saturating_add(self.p_memsz) as usize
        }
        /// Sets the executable flag
        pub fn executable(&mut self) {
            self.p_flags |= PF_X;
        }
        /// Sets the write flag
        pub fn write(&mut self) {
            self.p_flags |= PF_W;
        }
        /// Sets the read flag
        pub fn read(&mut self) {
            self.p_flags |= PF_R;
        }
        /// Whether this program header is executable
        pub fn is_executable(&self) -> bool {
            self.p_flags & PF_X != 0
        }
        /// Whether this program header is readable
        pub fn is_read(&self) -> bool {
            self.p_flags & PF_R != 0
        }
        /// Whether this program header is writable
        pub fn is_write(&self) -> bool {
            self.p_flags & PF_W != 0
        }
        #[cfg(feature = "endian_fd")]
        pub fn parse(bytes: &[u8], mut offset: usize, count: usize, ctx: Ctx) -> crate::error::Result<Vec<ProgramHeader>> {
            use scroll::Pread;
            // Sanity check to avoid OOM
            if count > bytes.len() / Self::size(ctx) {
                return Err(crate::error::Error::BufferTooShort(count, "program headers"));
            }
            let mut program_headers = Vec::with_capacity(count);
            for _ in 0..count {
                let phdr = bytes.gread_with(&mut offset, ctx)?;
                program_headers.push(phdr);
            }
            Ok(program_headers)
        }
    }

    impl fmt::Debug for ProgramHeader {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("ProgramHeader")
                .field("p_type", &pt_to_str(self.p_type))
                .field("p_flags", &format_args!("0x{:x}", self.p_flags))
                .field("p_offset", &format_args!("0x{:x}", self.p_offset))
                .field("p_vaddr", &format_args!("0x{:x}", self.p_vaddr))
                .field("p_paddr", &format_args!("0x{:x}", self.p_paddr))
                .field("p_filesz", &format_args!("0x{:x}", self.p_filesz))
                .field("p_memsz", &format_args!("0x{:x}", self.p_memsz))
                .field("p_align", &self.p_align)
                .finish()
        }
    }

    impl ctx::SizeWith<Ctx> for ProgramHeader {
        fn size_with(ctx: &Ctx) -> usize {
            match ctx.container {
                Container::Little => {
                    program_header32::SIZEOF_PHDR
                },
                Container::Big => {
                    program_header64::SIZEOF_PHDR
                },
            }
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Ctx> for ProgramHeader {
        type Error = crate::error::Error;
        fn try_from_ctx(bytes: &'a [u8], Ctx { container, le}: Ctx) -> result::Result<(Self, usize), Self::Error> {
            use scroll::Pread;
            let res = match container {
                Container::Little => {
                    (bytes.pread_with::<program_header32::ProgramHeader>(0, le)?.into(), program_header32::SIZEOF_PHDR)
                },
                Container::Big => {
                    (bytes.pread_with::<program_header64::ProgramHeader>(0, le)?.into(), program_header64::SIZEOF_PHDR)
                }
            };
            Ok(res)
        }
    }

    impl ctx::TryIntoCtx<Ctx> for ProgramHeader {
        type Error = crate::error::Error;
        fn try_into_ctx(self, bytes: &mut [u8], Ctx {container, le}: Ctx) -> result::Result<usize, Self::Error> {
            use scroll::Pwrite;
            match container {
                Container::Little => {
                    let phdr: program_header32::ProgramHeader = self.into();
                    Ok(bytes.pwrite_with(phdr, 0, le)?)
                },
                Container::Big => {
                    let phdr: program_header64::ProgramHeader = self.into();
                    Ok(bytes.pwrite_with(phdr, 0, le)?)
                }
            }
        }
    }
} // end if_alloc

macro_rules! elf_program_header_std_impl {
    ($size:ty) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn size_of() {
                assert_eq!(::std::mem::size_of::<ProgramHeader>(), SIZEOF_PHDR);
            }
        }

        if_alloc! {


            use crate::elf::program_header::ProgramHeader as ElfProgramHeader;
            #[cfg(any(feature = "std", feature = "endian_fd"))]
            use crate::error::Result;

            use plain::Plain;

            if_std! {
                use std::fs::File;
                use std::io::{Seek, Read};
                use std::io::SeekFrom::Start;
            }

            impl From<ProgramHeader> for ElfProgramHeader {
                fn from(ph: ProgramHeader) -> Self {
                    ElfProgramHeader {
                        p_type   : ph.p_type,
                        p_flags  : ph.p_flags,
                        p_offset : u64::from(ph.p_offset),
                        p_vaddr  : u64::from(ph.p_vaddr),
                        p_paddr  : u64::from(ph.p_paddr),
                        p_filesz : u64::from(ph.p_filesz),
                        p_memsz  : u64::from(ph.p_memsz),
                        p_align  : u64::from(ph.p_align),
                    }
                }
            }

            impl From<ElfProgramHeader> for ProgramHeader {
                fn from(ph: ElfProgramHeader) -> Self {
                    ProgramHeader {
                        p_type   : ph.p_type,
                        p_flags  : ph.p_flags,
                        p_offset : ph.p_offset as $size,
                        p_vaddr  : ph.p_vaddr  as $size,
                        p_paddr  : ph.p_paddr  as $size,
                        p_filesz : ph.p_filesz as $size,
                        p_memsz  : ph.p_memsz  as $size,
                        p_align  : ph.p_align  as $size,
                    }
                }
            }
        } // end if_alloc

        use core::fmt;
        use core::slice;

        impl fmt::Debug for ProgramHeader {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("ProgramHeader")
                    .field("p_type", &pt_to_str(self.p_type))
                    .field("p_flags", &format_args!("0x{:x}", self.p_flags))
                    .field("p_offset", &format_args!("0x{:x}", self.p_offset))
                    .field("p_vaddr", &format_args!("0x{:x}", self.p_vaddr))
                    .field("p_paddr", &format_args!("0x{:x}", self.p_paddr))
                    .field("p_filesz", &format_args!("0x{:x}", self.p_filesz))
                    .field("p_memsz", &format_args!("0x{:x}", self.p_memsz))
                    .field("p_align", &self.p_align)
                    .finish()
            }
        }

        impl ProgramHeader {
            #[cfg(feature = "endian_fd")]
            pub fn parse(
                bytes: &[u8],
                mut offset: usize,
                count: usize,
                ctx: ::scroll::Endian,
            ) -> Result<Vec<ProgramHeader>> {
                use scroll::Pread;
                let mut program_headers = vec![ProgramHeader::default(); count];
                let offset = &mut offset;
                bytes.gread_inout_with(offset, &mut program_headers, ctx)?;
                Ok(program_headers)
            }

            #[cfg(feature = "alloc")]
            pub fn from_bytes(bytes: &[u8], phnum: usize) -> Vec<ProgramHeader> {
                let mut phdrs = vec![ProgramHeader::default(); phnum];
                phdrs
                    .copy_from_bytes(bytes)
                    .expect("buffer is too short for given number of entries");
                phdrs
            }

            /// # Safety
            ///
            /// This function creates a `ProgramHeader` directly from a raw pointer
            pub unsafe fn from_raw_parts<'a>(
                phdrp: *const ProgramHeader,
                phnum: usize,
            ) -> &'a [ProgramHeader] {
                slice::from_raw_parts(phdrp, phnum)
            }

            #[cfg(feature = "std")]
            pub fn from_fd(fd: &mut File, offset: u64, count: usize) -> Result<Vec<ProgramHeader>> {
                let mut phdrs = vec![ProgramHeader::default(); count];
                fd.seek(Start(offset))?;
                unsafe {
                    fd.read_exact(plain::as_mut_bytes(&mut *phdrs))?;
                }
                Ok(phdrs)
            }
        }
    };
}

#[cfg(feature = "alloc")]
use scroll::{Pread, Pwrite, SizeWith};

pub mod program_header32 {
    pub use crate::elf::program_header::*;

    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Default)]
    #[cfg_attr(feature = "alloc", derive(Pread, Pwrite, SizeWith))]
    /// A 32-bit ProgramHeader typically specifies how to map executable and data segments into memory
    pub struct ProgramHeader {
        /// Segment type
        pub p_type: u32,
        /// Segment file offset
        pub p_offset: u32,
        /// Segment virtual address
        pub p_vaddr: u32,
        /// Segment physical address
        pub p_paddr: u32,
        /// Segment size in file
        pub p_filesz: u32,
        /// Segment size in memory
        pub p_memsz: u32,
        /// Segment flags
        pub p_flags: u32,
        /// Segment alignment
        pub p_align: u32,
    }

    pub const SIZEOF_PHDR: usize = 32;

    // Declare that this is a plain type.
    unsafe impl plain::Plain for ProgramHeader {}

    elf_program_header_std_impl!(u32);
}

pub mod program_header64 {
    pub use crate::elf::program_header::*;

    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Default)]
    #[cfg_attr(feature = "alloc", derive(Pread, Pwrite, SizeWith))]
    /// A 64-bit ProgramHeader typically specifies how to map executable and data segments into memory
    pub struct ProgramHeader {
        /// Segment type
        pub p_type: u32,
        /// Segment flags
        pub p_flags: u32,
        /// Segment file offset
        pub p_offset: u64,
        /// Segment virtual address
        pub p_vaddr: u64,
        /// Segment physical address
        pub p_paddr: u64,
        /// Segment size in file
        pub p_filesz: u64,
        /// Segment size in memory
        pub p_memsz: u64,
        /// Segment alignment
        pub p_align: u64,
    }

    pub const SIZEOF_PHDR: usize = 56;

    // Declare that this is a plain type.
    unsafe impl plain::Plain for ProgramHeader {}

    elf_program_header_std_impl!(u64);
}
