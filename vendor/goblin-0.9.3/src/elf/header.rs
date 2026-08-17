include!("constants_header.rs");

macro_rules! elf_header {
    ($size:ident) => {
        use core::fmt;

        #[repr(C)]
        #[derive(Clone, Copy, Default, PartialEq)]
        pub struct Header {
            /// Magic number and other info
            pub e_ident: [u8; SIZEOF_IDENT],
            /// Object file type
            pub e_type: u16,
            /// Architecture
            pub e_machine: u16,
            /// Object file version
            pub e_version: u32,
            /// Entry point virtual address
            pub e_entry: $size,
            /// Program header table file offset
            pub e_phoff: $size,
            /// Section header table file offset
            pub e_shoff: $size,
            /// Processor-specific flags
            pub e_flags: u32,
            /// ELF header size in bytes
            pub e_ehsize: u16,
            /// Program header table entry size
            pub e_phentsize: u16,
            /// Program header table entry count
            pub e_phnum: u16,
            /// Section header table entry size
            pub e_shentsize: u16,
            /// Section header table entry count
            pub e_shnum: u16,
            /// Section header string table index
            pub e_shstrndx: u16,
        }

        use plain;
        // Declare that this is a plain type.
        unsafe impl plain::Plain for Header {}

        impl Header {
            /// Returns the corresponding ELF header from the given byte array.
            pub fn from_bytes(bytes: &[u8; SIZEOF_EHDR]) -> &Header {
                // FIXME: Length is ensured correct because it's encoded in the type,
                // but it can still panic due to invalid alignment.
                plain::from_bytes(bytes).unwrap()
            }
        }
        impl fmt::Debug for Header {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("Header")
                    .field("e_ident", &format_args!("{:?}", self.e_ident))
                    .field("e_type", &et_to_str(self.e_type))
                    .field("e_machine", &format_args!("0x{:x}", self.e_machine))
                    .field("e_version", &format_args!("0x{:x}", self.e_version))
                    .field("e_entry", &format_args!("0x{:x}", self.e_entry))
                    .field("e_phoff", &format_args!("0x{:x}", self.e_phoff))
                    .field("e_shoff", &format_args!("0x{:x}", self.e_shoff))
                    .field("e_flags", &format_args!("{:x}", self.e_flags))
                    .field("e_ehsize", &self.e_ehsize)
                    .field("e_phentsize", &self.e_phentsize)
                    .field("e_phnum", &self.e_phnum)
                    .field("e_shentsize", &self.e_shentsize)
                    .field("e_shnum", &self.e_shnum)
                    .field("e_shstrndx", &self.e_shstrndx)
                    .finish()
            }
        }
    };
}

/// No file type.
pub const ET_NONE: u16 = 0;
/// Relocatable file.
pub const ET_REL: u16 = 1;
/// Executable file.
pub const ET_EXEC: u16 = 2;
/// Shared object file.
pub const ET_DYN: u16 = 3;
/// Core file.
pub const ET_CORE: u16 = 4;
/// Number of defined types.
pub const ET_NUM: u16 = 5;
/// OS-specific range start
pub const ET_LOOS: u16 = 0xfe00;
/// OS-specific range end
pub const ET_HIOS: u16 = 0xfeff;
/// Processor-specific range start
pub const ET_LOPROC: u16 = 0xff00;
/// Processor-specific range end
pub const ET_HIPROC: u16 = 0xffff;

/// The ELF magic number.
pub const ELFMAG: &[u8; 4] = b"\x7FELF";
/// Sizeof ELF magic number.
pub const SELFMAG: usize = 4;

/// File class byte index.
pub const EI_CLASS: usize = 4;
/// Invalid class.
pub const ELFCLASSNONE: u8 = 0;
/// 32-bit objects.
pub const ELFCLASS32: u8 = 1;
/// 64-bit objects.
pub const ELFCLASS64: u8 = 2;
/// ELF class number.
pub const ELFCLASSNUM: u8 = 3;

/// Data encoding byte index.
pub const EI_DATA: usize = 5;
/// Invalid data encoding.
pub const ELFDATANONE: u8 = 0;
/// 2's complement, little endian.
pub const ELFDATA2LSB: u8 = 1;
/// 2's complement, big endian.
pub const ELFDATA2MSB: u8 = 2;

/// File version byte index.
pub const EI_VERSION: usize = 6;
/// Current ELF version.
pub const EV_CURRENT: u8 = 1;

/// OS ABI byte index.
pub const EI_OSABI: usize = 7;
/// UNIX System V ABI.
pub const ELFOSABI_NONE: u8 = 0;
/// UNIX System V ABI.
///
/// Alias.
pub const ELFOSABI_SYSV: u8 = ELFOSABI_NONE;
/// HP-UX.
pub const ELFOSABI_HPUX: u8 = 1;
/// NetBSD.
pub const ELFOSABI_NETBSD: u8 = 2;
/// Object uses GNU ELF extensions.
pub const ELFOSABI_GNU: u8 = 3;
/// Object uses GNU ELF extensions.
///
/// Alias.
pub const ELFOSABI_LINUX: u8 = ELFOSABI_GNU;
/// Sun Solaris.
pub const ELFOSABI_SOLARIS: u8 = 6;
/// IBM AIX.
pub const ELFOSABI_AIX: u8 = 7;
/// SGI Irix.
pub const ELFOSABI_IRIX: u8 = 8;
/// FreeBSD
pub const ELFOSABI_FREEBSD: u8 = 9;
/// Compaq TRU64 UNIX.
pub const ELFOSABI_TRU64: u8 = 10;
/// Novell Modesto.
pub const ELFOSABI_MODESTO: u8 = 11;
/// OpenBSD.
pub const ELFOSABI_OPENBSD: u8 = 12;
/// ARM EABI.
pub const ELFOSABI_ARM_AEABI: u8 = 64;
/// ARM.
pub const ELFOSABI_ARM: u8 = 97;
/// Standalone (embedded) application.
pub const ELFOSABI_STANDALONE: u8 = 255;

/// ABI version byte index.
pub const EI_ABIVERSION: usize = 8;

/// Number of bytes in an identifier.
pub const SIZEOF_IDENT: usize = 16;

/// Convert a ELF class byte to the associated string.
#[inline]
pub fn class_to_str(et: u8) -> &'static str {
    match et {
        ELFCLASSNONE => "NONE",
        ELFCLASS32 => "ELF32",
        ELFCLASS64 => "ELF64",
        _ => "UNKNOWN_CLASS",
    }
}

/// Convert an ET value to their associated string.
#[inline]
pub fn et_to_str(et: u16) -> &'static str {
    match et {
        ET_NONE => "NONE",
        ET_REL => "REL",
        ET_EXEC => "EXEC",
        ET_DYN => "DYN",
        ET_CORE => "CORE",
        ET_NUM => "NUM",
        _ => "UNKNOWN_ET",
    }
}

if_alloc! {
    use crate::error;
    use scroll::{ctx, Endian};
    use core::fmt;
    use crate::container::{Ctx, Container};
    use alloc::string::ToString;

    #[derive(Copy, Clone, PartialEq)]
    /// An ELF header
    pub struct Header {
        pub e_ident           : [u8; SIZEOF_IDENT],
        pub e_type            : u16,
        pub e_machine         : u16,
        pub e_version         : u32,
        pub e_entry           : u64,
        pub e_phoff           : u64,
        pub e_shoff           : u64,
        pub e_flags           : u32,
        pub e_ehsize          : u16,
        pub e_phentsize       : u16,
        pub e_phnum           : u16,
        pub e_shentsize       : u16,
        pub e_shnum           : u16,
        pub e_shstrndx        : u16,
    }

    impl Header {
        /// Return the size of the underlying program header, given a `container`
        #[inline]
        pub fn size(ctx: Ctx) -> usize {
            use scroll::ctx::SizeWith;
            Self::size_with(&ctx)
        }
        /// Returns the container type this header specifies
        pub fn container(&self) -> error::Result<Container> {
            use crate::error::Error;
            match self.e_ident[EI_CLASS] {
                ELFCLASS32 => { Ok(Container::Little) },
                ELFCLASS64 => { Ok(Container::Big) },
                class => Err(Error::Malformed(format!("Invalid class in Header: {}", class)))
            }
        }
        /// Returns the byte order this header specifies
        pub fn endianness(&self) -> error::Result<scroll::Endian> {
            use crate::error::Error;
            match self.e_ident[EI_DATA] {
                ELFDATA2LSB => { Ok(scroll::LE) },
                ELFDATA2MSB => { Ok(scroll::BE) },
                class => Err(Error::Malformed(format!("Invalid endianness in Header: {}", class)))
            }
        }
        pub fn new(ctx: Ctx) -> Self {
            use crate::elf32;
            use crate::elf64;
            let (typ, ehsize, phentsize, shentsize) = match ctx.container {
                Container::Little => {
                    (ELFCLASS32, header32::SIZEOF_EHDR,
                     elf32::program_header::SIZEOF_PHDR,
                     elf32::section_header::SIZEOF_SHDR)
                },
                Container::Big => {
                    (ELFCLASS64, header64::SIZEOF_EHDR,
                     elf64::program_header::SIZEOF_PHDR,
                     elf64::section_header::SIZEOF_SHDR)
                }
            };
            let byteorder = match ctx.le { Endian::Little => ELFDATA2LSB, Endian::Big => ELFDATA2MSB };
            Header {
                e_ident: [
                    127,
                    69,
                    76,
                    70,
                    typ,
                    byteorder,
                    EV_CURRENT,
                    ELFOSABI_NONE,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0
                ],
                e_type: ET_DYN,
                e_machine: EM_NONE,
                e_version: 1,
                e_entry: 0x0,
                e_phoff: 0x0,
                e_shoff: 0x0,
                e_flags: 0,
                e_ehsize: ehsize as u16,
                e_phentsize: phentsize as u16,
                e_phnum: 0,
                e_shentsize: shentsize as u16,
                e_shnum: 0,
                e_shstrndx: 0,
            }
        }
    }

    impl fmt::Debug for Header {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Header")
               .field("e_ident", &format_args!("{:?}", self.e_ident))
               .field("e_type", &et_to_str(self.e_type))
               .field("e_machine", &format_args!("0x{:x}", self.e_machine))
               .field("e_version", &format_args!("0x{:x}", self.e_version))
               .field("e_entry", &format_args!("0x{:x}", self.e_entry))
               .field("e_phoff", &format_args!("0x{:x}", self.e_phoff))
               .field("e_shoff", &format_args!("0x{:x}", self.e_shoff))
               .field("e_flags", &format_args!("{:x}", self.e_flags))
               .field("e_ehsize", &self.e_ehsize)
               .field("e_phentsize", &self.e_phentsize)
               .field("e_phnum", &self.e_phnum)
               .field("e_shentsize", &self.e_shentsize)
               .field("e_shnum", &self.e_shnum)
               .field("e_shstrndx", &self.e_shstrndx)
               .finish()
        }
    }

    impl ctx::SizeWith<crate::container::Ctx> for Header {
        fn size_with(ctx: &crate::container::Ctx) -> usize {
            match ctx.container {
                Container::Little => {
                    header32::SIZEOF_EHDR
                },
                Container::Big => {
                    header64::SIZEOF_EHDR
                },
            }
        }
    }

    impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for Header {
        type Error = crate::error::Error;
        fn try_from_ctx(bytes: &'a [u8], _ctx: scroll::Endian) -> error::Result<(Self, usize)> {
            use scroll::Pread;
            if bytes.len() < SIZEOF_IDENT {
                return Err(error::Error::Malformed("Too small".to_string()));
            }
            let ident: &[u8] = &bytes[..SIZEOF_IDENT];
            if &ident[0..SELFMAG] != ELFMAG {
                let magic: u64 = ident.pread_with(0, scroll::LE)?;
                return Err(error::Error::BadMagic(magic));
            }
            let class = ident[EI_CLASS];
            match class {
                ELFCLASS32 => {
                    Ok((Header::from(bytes.pread::<header32::Header>(0)?), header32::SIZEOF_EHDR))
                },
                ELFCLASS64 => {
                    Ok((Header::from(bytes.pread::<header64::Header>(0)?), header64::SIZEOF_EHDR))
                },
                _ => {
                    Err(error::Error::Malformed(format!("invalid ELF class {:x}", class)))
                }
            }
        }
    }

    impl ctx::TryIntoCtx<scroll::Endian> for Header {
        type Error = crate::error::Error;
        fn try_into_ctx(self, bytes: &mut [u8], _ctx: scroll::Endian) -> Result<usize, Self::Error> {
            use scroll::Pwrite;
            match self.container()? {
                Container::Little => {
                    bytes.pwrite(header32::Header::from(self), 0)
                },
                Container::Big => {
                    bytes.pwrite(header64::Header::from(self), 0)
                }
            }
        }
    }
    impl ctx::IntoCtx<crate::container::Ctx> for Header {
        fn into_ctx(self, bytes: &mut [u8], ctx: crate::container::Ctx) {
            use scroll::Pwrite;
            match ctx.container {
                Container::Little => {
                    bytes.pwrite_with(header32::Header::from(self), 0, ctx.le).unwrap()
                },
                Container::Big => {
                    bytes.pwrite_with(header64::Header::from(self), 0, ctx.le).unwrap()
                }
            };
        }
    }
} // end if_alloc

macro_rules! elf_header_std_impl {
    ($size:expr, $width:ty) => {

        if_alloc! {
            use crate::elf::header::Header as ElfHeader;
            use crate::error::Error;
            #[cfg(any(feature = "std", feature = "endian_fd"))]
            use crate::error::Result;

            use scroll::{ctx, Pread};

            use core::result;

            if_std! {
                use std::fs::File;
                use std::io::{Read};
            }

            impl From<ElfHeader> for Header {
                fn from(eh: ElfHeader) -> Self {
                    Header {
                        e_ident: eh.e_ident,
                        e_type: eh.e_type,
                        e_machine: eh.e_machine,
                        e_version: eh.e_version,
                        e_entry: eh.e_entry as $width,
                        e_phoff: eh.e_phoff as $width,
                        e_shoff: eh.e_shoff as $width,
                        e_flags: eh.e_flags,
                        e_ehsize: eh.e_ehsize,
                        e_phentsize: eh.e_phentsize,
                        e_phnum: eh.e_phnum,
                        e_shentsize: eh.e_shentsize,
                        e_shnum: eh.e_shnum,
                        e_shstrndx: eh.e_shstrndx,
                    }
                }
            }

            impl From<Header> for ElfHeader {
                fn from(eh: Header) -> Self {
                    ElfHeader {
                        e_ident: eh.e_ident,
                        e_type: eh.e_type,
                        e_machine: eh.e_machine,
                        e_version: eh.e_version,
                        e_entry: u64::from(eh.e_entry),
                        e_phoff: u64::from(eh.e_phoff),
                        e_shoff: u64::from(eh.e_shoff),
                        e_flags: eh.e_flags,
                        e_ehsize: eh.e_ehsize,
                        e_phentsize: eh.e_phentsize,
                        e_phnum: eh.e_phnum,
                        e_shentsize: eh.e_shentsize,
                        e_shnum: eh.e_shnum,
                        e_shstrndx: eh.e_shstrndx,
                    }
                }
            }

            impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for Header {
                type Error = crate::error::Error;
                fn try_from_ctx(bytes: &'a [u8], _: scroll::Endian) -> result::Result<(Self, usize), Self::Error> {
                    let mut elf_header = Header::default();
                    let offset = &mut 0;
                    bytes.gread_inout(offset, &mut elf_header.e_ident)?;
                    let endianness =
                        match elf_header.e_ident[EI_DATA] {
                            ELFDATA2LSB => scroll::LE,
                            ELFDATA2MSB => scroll::BE,
                            d => return Err(Error::Malformed(format!("invalid ELF endianness DATA type {:x}", d)).into()),
                        };
                    elf_header.e_type =      bytes.gread_with(offset, endianness)?;
                    elf_header.e_machine =   bytes.gread_with(offset, endianness)?;
                    elf_header.e_version =   bytes.gread_with(offset, endianness)?;
                    elf_header.e_entry =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_phoff =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_shoff =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_flags =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_ehsize =    bytes.gread_with(offset, endianness)?;
                    elf_header.e_phentsize = bytes.gread_with(offset, endianness)?;
                    elf_header.e_phnum =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_shentsize = bytes.gread_with(offset, endianness)?;
                    elf_header.e_shnum =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_shstrndx =  bytes.gread_with(offset, endianness)?;
                    Ok((elf_header, SIZEOF_EHDR))
                }
            }

            impl ctx::TryIntoCtx<scroll::Endian> for Header {
                type Error = crate::error::Error;
                /// a Pwrite impl for Header: **note** we use the endianness value in the header, and not a parameter
                fn try_into_ctx(self, bytes: &mut [u8], _endianness: scroll::Endian) -> result::Result<usize, Self::Error> {
                    use scroll::{Pwrite};
                    let offset = &mut 0;
                    let endianness =
                        match self.e_ident[EI_DATA] {
                            ELFDATA2LSB => scroll::LE,
                            ELFDATA2MSB => scroll::BE,
                            d => return Err(Error::Malformed(format!("invalid ELF DATA type {:x}", d)).into()),
                        };
                    for i in 0..self.e_ident.len() {
                        bytes.gwrite(self.e_ident[i], offset)?;
                    }
                    bytes.gwrite_with(self.e_type      , offset, endianness)?;
                    bytes.gwrite_with(self.e_machine   , offset, endianness)?;
                    bytes.gwrite_with(self.e_version   , offset, endianness)?;
                    bytes.gwrite_with(self.e_entry     , offset, endianness)?;
                    bytes.gwrite_with(self.e_phoff     , offset, endianness)?;
                    bytes.gwrite_with(self.e_shoff     , offset, endianness)?;
                    bytes.gwrite_with(self.e_flags     , offset, endianness)?;
                    bytes.gwrite_with(self.e_ehsize    , offset, endianness)?;
                    bytes.gwrite_with(self.e_phentsize , offset, endianness)?;
                    bytes.gwrite_with(self.e_phnum     , offset, endianness)?;
                    bytes.gwrite_with(self.e_shentsize , offset, endianness)?;
                    bytes.gwrite_with(self.e_shnum     , offset, endianness)?;
                    bytes.gwrite_with(self.e_shstrndx  , offset, endianness)?;
                    Ok(SIZEOF_EHDR)
                }
            }

            impl Header {
                /// Load a header from a file. **You must** ensure the seek is at the correct position.
                #[cfg(feature = "std")]
                pub fn from_fd(bytes: &mut File) -> Result<Header> {
                    let mut elf_header = [0; $size];
                    bytes.read_exact(&mut elf_header)?;
                    Ok(*Header::from_bytes(&elf_header))
                }

                #[cfg(feature = "endian_fd")]
                /// Parses an ELF header from the given bytes
                pub fn parse(bytes: &[u8]) -> Result<Header> {
                    use super::{EI_DATA, ELFDATA2LSB, ELFDATA2MSB, SIZEOF_IDENT};

                    let mut elf_header = Header::default();
                    let mut offset = &mut 0;
                    for i in 0..SIZEOF_IDENT {
                        elf_header.e_ident[i] = bytes.gread(&mut offset)?;
                    }
                    let endianness =
                        match elf_header.e_ident[EI_DATA] {
                            ELFDATA2LSB => scroll::LE,
                            ELFDATA2MSB => scroll::BE,
                            d => return Err(Error::Malformed(format!("invalid ELF DATA type {:x}", d)).into()),
                        };
                    elf_header.e_type =      bytes.gread_with(offset, endianness)?;
                    elf_header.e_machine =   bytes.gread_with(offset, endianness)?;
                    elf_header.e_version =   bytes.gread_with(offset, endianness)?;
                    elf_header.e_entry =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_phoff =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_shoff =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_flags =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_ehsize =    bytes.gread_with(offset, endianness)?;
                    elf_header.e_phentsize = bytes.gread_with(offset, endianness)?;
                    elf_header.e_phnum =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_shentsize = bytes.gread_with(offset, endianness)?;
                    elf_header.e_shnum =     bytes.gread_with(offset, endianness)?;
                    elf_header.e_shstrndx =  bytes.gread_with(offset, endianness)?;
                    Ok(elf_header)
                }
            }
        } // end if_alloc
    };
}

// tests

macro_rules! elf_header_test {
    ($class:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::container::{Container, Ctx};
            use crate::elf::header::Header as ElfHeader;
            use alloc::vec::Vec;
            use scroll::{Pread, Pwrite};
            #[test]
            fn size_of() {
                assert_eq!(::std::mem::size_of::<Header>(), SIZEOF_EHDR);
            }
            #[test]
            fn header_read_write() {
                let crt1: Vec<u8> = if $class == ELFCLASS64 {
                    include!("../../etc/crt1.rs")
                } else {
                    include!("../../etc/crt132.rs")
                };
                let header: Header = crt1.pread(0).unwrap();
                assert_eq!(header.e_type, ET_REL);
                println!("header: {:?}", &header);
                let mut bytes = [0u8; SIZEOF_EHDR];
                bytes.pwrite(header, 0).unwrap();
                let header2: Header = bytes.pread(0).unwrap();
                assert_eq!(header, header2);
            }
            #[test]
            fn elfheader_read_write() {
                let (container, crt1): (Container, Vec<u8>) = if $class == ELFCLASS64 {
                    (Container::Big, include!("../../etc/crt1.rs"))
                } else {
                    (Container::Little, include!("../../etc/crt132.rs"))
                };
                let header: Header = crt1.pread(0).unwrap();
                assert_eq!(header.e_type, ET_REL);
                println!("header: {:?}", &header);
                let mut bytes = [0u8; SIZEOF_EHDR];
                let header_ = Header::from(header.clone());
                bytes.pwrite(header_, 0).unwrap();
                let header2: Header = bytes.pread(0).unwrap();
                assert_eq!(header, header2);
                let header = ElfHeader::new(Ctx::from(container));
                println!("header: {:?}", &header);

                let mut bytes = vec![0; 100];
                bytes.pwrite(header, 0).unwrap();
            }
        }
    };
}

pub mod header32 {
    pub use super::*;

    pub const SIZEOF_EHDR: usize = 52;
    pub const ELFCLASS: u8 = ELFCLASS32;

    elf_header!(u32);
    elf_header_std_impl!(SIZEOF_EHDR, u32);
    elf_header_test!(ELFCLASS);
}

pub mod header64 {
    pub use super::*;

    pub const SIZEOF_EHDR: usize = 64;
    pub const ELFCLASS: u8 = ELFCLASS64;

    elf_header!(u64);
    elf_header_std_impl!(SIZEOF_EHDR, u64);
    elf_header_test!(ELFCLASS);
}
