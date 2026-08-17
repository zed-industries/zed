//! A header contains minimal architecture information, the binary kind, the number of load commands, as well as an endianness hint

use core::fmt;
use plain::Plain;
use scroll::ctx;
use scroll::ctx::SizeWith;
use scroll::{Pread, Pwrite, SizeWith};

use crate::container::{self, Container};
use crate::error;
use crate::mach::constants::cputype::{CpuSubType, CpuType, CPU_SUBTYPE_MASK};

// Constants for the flags field of the mach_header
/// the object file has no undefined references
pub const MH_NOUNDEFS: u32 = 0x1;
/// the object file is the output of an incremental link against a base file and can't be
/// link edited again
pub const MH_INCRLINK: u32 = 0x2;
/// the object file is input for the dynamic linker and can't be staticly link edited again
pub const MH_DYLDLINK: u32 = 0x4;
/// the object file's undefined references are bound by the dynamic linker when loaded.
pub const MH_BINDATLOAD: u32 = 0x8;
/// the file has its dynamic undefined references prebound.
pub const MH_PREBOUND: u32 = 0x10;
/// the file has its read-only and read-write segments split
pub const MH_SPLIT_SEGS: u32 = 0x20;
/// the shared library init routine is to be run lazily via catching memory faults to its writeable
/// segments (obsolete)
pub const MH_LAZY_INIT: u32 = 0x40;
/// the image is using two-level name space bindings
pub const MH_TWOLEVEL: u32 = 0x80;
/// the executable is forcing all images to use flat name space bindings
pub const MH_FORCE_FLAT: u32 = 0x100;
/// this umbrella guarantees no multiple defintions of symbols in its sub-images so the
/// two-level namespace hints can always be used.
pub const MH_NOMULTIDEFS: u32 = 0x200;
/// do not have dyld notify the prebinding agent about this executable
pub const MH_NOFIXPREBINDING: u32 = 0x400;
/// the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set.
pub const MH_PREBINDABLE: u32 = 0x800;
/// indicates that this binary binds to all two-level namespace modules of its dependent libraries.
/// Only used when MH_PREBINDABLE and MH_TWOLEVEL are both set.
pub const MH_ALLMODSBOUND: u32 = 0x1000;
/// safe to divide up the sections into sub-sections via symbols for dead code stripping
pub const MH_SUBSECTIONS_VIA_SYMBOLS: u32 = 0x2000;
/// the binary has been canonicalized via the unprebind operation
pub const MH_CANONICAL: u32 = 0x4000;
/// the final linked image contains external weak symbols
pub const MH_WEAK_DEFINES: u32 = 0x8000;
/// the final linked image uses weak symbols
pub const MH_BINDS_TO_WEAK: u32 = 0x10000;
/// When this bit is set, all stacks in the task will be given stack execution privilege.
/// Only used in MH_EXECUTE filetypes.
pub const MH_ALLOW_STACK_EXECUTION: u32 = 0x20000;
/// When this bit is set, the binary declares it is safe for use in processes with uid zero
pub const MH_ROOT_SAFE: u32 = 0x40000;
/// When this bit is set, the binary declares it is safe for use in processes when issetugid() is true
pub const MH_SETUID_SAFE: u32 = 0x80000;
/// When this bit is set on a dylib,  the static linker does not need to examine dependent dylibs to
/// see if any are re-exported
pub const MH_NO_REEXPORTED_DYLIBS: u32 = 0x0010_0000;
/// When this bit is set, the OS will load the main executable at a random address.
/// Only used in MH_EXECUTE filetypes.
pub const MH_PIE: u32 = 0x0020_0000;
/// Only for use on dylibs.  When linking against a dylib that has this bit set, the static linker
/// will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being
/// referenced from the dylib.
pub const MH_DEAD_STRIPPABLE_DYLIB: u32 = 0x0040_0000;
/// Contains a section of type S_THREAD_LOCAL_VARIABLES
pub const MH_HAS_TLV_DESCRIPTORS: u32 = 0x0080_0000;
/// When this bit is set, the OS will run the main executable with a non-executable heap even on
/// platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes.
pub const MH_NO_HEAP_EXECUTION: u32 = 0x0100_0000;

// TODO: verify this number is correct, it was previously 0x02000000 which could indicate a typo/data entry error
/// The code was linked for use in an application extension.
pub const MH_APP_EXTENSION_SAFE: u32 = 0x0200_0000;

#[inline(always)]
pub fn flag_to_str(flag: u32) -> &'static str {
    match flag {
        MH_NOUNDEFS => "MH_NOUNDEFS",
        MH_INCRLINK => "MH_INCRLINK",
        MH_DYLDLINK => "MH_DYLDLINK",
        MH_BINDATLOAD => "MH_BINDATLOAD",
        MH_PREBOUND => "MH_PREBOUND",
        MH_SPLIT_SEGS => "MH_SPLIT_SEGS",
        MH_LAZY_INIT => "MH_LAZY_INIT",
        MH_TWOLEVEL => "MH_TWOLEVEL",
        MH_FORCE_FLAT => "MH_FORCE_FLAT",
        MH_NOMULTIDEFS => "MH_NOMULTIDEFS",
        MH_NOFIXPREBINDING => "MH_NOFIXPREBINDING",
        MH_PREBINDABLE => "MH_PREBINDABLE ",
        MH_ALLMODSBOUND => "MH_ALLMODSBOUND",
        MH_SUBSECTIONS_VIA_SYMBOLS => "MH_SUBSECTIONS_VIA_SYMBOLS",
        MH_CANONICAL => "MH_CANONICAL",
        MH_WEAK_DEFINES => "MH_WEAK_DEFINES",
        MH_BINDS_TO_WEAK => "MH_BINDS_TO_WEAK",
        MH_ALLOW_STACK_EXECUTION => "MH_ALLOW_STACK_EXECUTION",
        MH_ROOT_SAFE => "MH_ROOT_SAFE",
        MH_SETUID_SAFE => "MH_SETUID_SAFE",
        MH_NO_REEXPORTED_DYLIBS => "MH_NO_REEXPORTED_DYLIBS",
        MH_PIE => "MH_PIE",
        MH_DEAD_STRIPPABLE_DYLIB => "MH_DEAD_STRIPPABLE_DYLIB",
        MH_HAS_TLV_DESCRIPTORS => "MH_HAS_TLV_DESCRIPTORS",
        MH_NO_HEAP_EXECUTION => "MH_NO_HEAP_EXECUTION",
        MH_APP_EXTENSION_SAFE => "MH_APP_EXTENSION_SAFE",
        _ => "UNKNOWN FLAG",
    }
}

/// Mach Header magic constant
pub const MH_MAGIC: u32 = 0xfeed_face;
pub const MH_CIGAM: u32 = 0xcefa_edfe;
/// Mach Header magic constant for 64-bit
pub const MH_MAGIC_64: u32 = 0xfeed_facf;
pub const MH_CIGAM_64: u32 = 0xcffa_edfe;

// Constants for the filetype field of the mach_header
/// relocatable object file
pub const MH_OBJECT: u32 = 0x1;
/// demand paged executable file
pub const MH_EXECUTE: u32 = 0x2;
/// fixed VM shared library file
pub const MH_FVMLIB: u32 = 0x3;
/// core file
pub const MH_CORE: u32 = 0x4;
/// preloaded executable file
pub const MH_PRELOAD: u32 = 0x5;
/// dynamically bound shared library
pub const MH_DYLIB: u32 = 0x6;
/// dynamic link editor
pub const MH_DYLINKER: u32 = 0x7;
/// dynamically bound bundle file
pub const MH_BUNDLE: u32 = 0x8;
/// shared library stub for static linking only, no section contents
pub const MH_DYLIB_STUB: u32 = 0x9;
/// companion file with only debug sections
pub const MH_DSYM: u32 = 0xa;
/// x86_64 kexts
pub const MH_KEXT_BUNDLE: u32 = 0xb;
/// set of mach-o's
pub const MH_FILESET: u32 = 0xc;

pub fn filetype_to_str(filetype: u32) -> &'static str {
    match filetype {
        MH_OBJECT => "OBJECT",
        MH_EXECUTE => "EXECUTE",
        MH_FVMLIB => "FVMLIB",
        MH_CORE => "CORE",
        MH_PRELOAD => "PRELOAD",
        MH_DYLIB => "DYLIB",
        MH_DYLINKER => "DYLINKER",
        MH_BUNDLE => "BUNDLE",
        MH_DYLIB_STUB => "DYLIB_STUB",
        MH_DSYM => "DSYM",
        MH_KEXT_BUNDLE => "KEXT_BUNDLE",
        MH_FILESET => "FILESET",
        _ => "UNKNOWN FILETYPE",
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Pread, Pwrite, SizeWith)]
/// A 32-bit Mach-o header
pub struct Header32 {
    /// mach magic number identifier
    pub magic: u32,
    /// cpu specifier
    pub cputype: u32,
    /// machine specifier
    pub cpusubtype: u32,
    /// type of file
    pub filetype: u32,
    /// number of load commands
    pub ncmds: u32,
    /// the size of all the load commands
    pub sizeofcmds: u32,
    /// flags
    pub flags: u32,
}

pub const SIZEOF_HEADER_32: usize = 0x1c;

unsafe impl Plain for Header32 {}

impl Header32 {
    /// Transmutes the given byte array into the corresponding 32-bit Mach-o header
    pub fn from_bytes(bytes: &[u8; SIZEOF_HEADER_32]) -> &Self {
        plain::from_bytes(bytes).unwrap()
    }
    pub fn size(&self) -> usize {
        SIZEOF_HEADER_32
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Pread, Pwrite, SizeWith)]
/// A 64-bit Mach-o header
pub struct Header64 {
    /// mach magic number identifier
    pub magic: u32,
    /// cpu specifier
    pub cputype: u32,
    /// machine specifier
    pub cpusubtype: u32,
    /// type of file
    pub filetype: u32,
    /// number of load commands
    pub ncmds: u32,
    /// the size of all the load commands
    pub sizeofcmds: u32,
    /// flags
    pub flags: u32,
    pub reserved: u32,
}

unsafe impl Plain for Header64 {}

pub const SIZEOF_HEADER_64: usize = 32;

impl Header64 {
    /// Transmutes the given byte array into the corresponding 64-bit Mach-o header
    pub fn from_bytes(bytes: &[u8; SIZEOF_HEADER_64]) -> &Self {
        plain::from_bytes(bytes).unwrap()
    }
    pub fn size(&self) -> usize {
        SIZEOF_HEADER_64
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
/// Generic sized header
pub struct Header {
    pub magic: u32,
    pub cputype: u32,
    pub cpusubtype: u32,
    /// type of file
    pub filetype: u32,
    /// number of load commands
    pub ncmds: usize,
    /// the size of all the load commands
    pub sizeofcmds: u32,
    /// flags
    pub flags: u32,
    pub reserved: u32,
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Header")
            .field("magic", &format_args!("0x{:x}", self.magic))
            .field("cputype", &self.cputype())
            .field("cpusubtype", &format_args!("0x{:x}", self.cpusubtype()))
            .field("filetype", &filetype_to_str(self.filetype))
            .field("ncmds", &self.ncmds)
            .field("sizeofcmds", &self.sizeofcmds)
            .field("flags", &format_args!("0x{:x}", self.flags))
            .field("reserved", &format_args!("0x{:x}", self.reserved))
            .finish()
    }
}

impl From<Header32> for Header {
    fn from(header: Header32) -> Self {
        Header {
            magic: header.magic,
            cputype: header.cputype,
            cpusubtype: header.cpusubtype,
            filetype: header.filetype,
            ncmds: header.ncmds as usize,
            sizeofcmds: header.sizeofcmds,
            flags: header.flags,
            reserved: 0,
        }
    }
}

impl From<Header> for Header32 {
    fn from(header: Header) -> Self {
        Header32 {
            magic: header.magic,
            cputype: header.cputype,
            cpusubtype: header.cpusubtype,
            filetype: header.filetype,
            ncmds: header.ncmds as u32,
            sizeofcmds: header.sizeofcmds,
            flags: header.flags,
        }
    }
}

impl From<Header64> for Header {
    fn from(header: Header64) -> Self {
        Header {
            magic: header.magic,
            cputype: header.cputype,
            cpusubtype: header.cpusubtype,
            filetype: header.filetype,
            ncmds: header.ncmds as usize,
            sizeofcmds: header.sizeofcmds,
            flags: header.flags,
            reserved: header.reserved,
        }
    }
}

impl From<Header> for Header64 {
    fn from(header: Header) -> Self {
        Header64 {
            magic: header.magic,
            cputype: header.cputype,
            cpusubtype: header.cpusubtype,
            filetype: header.filetype,
            ncmds: header.ncmds as u32,
            sizeofcmds: header.sizeofcmds,
            flags: header.flags,
            reserved: header.reserved,
        }
    }
}

impl Header {
    pub fn new(ctx: container::Ctx) -> Self {
        let mut header = Header::default();
        header.magic = if ctx.is_big() { MH_MAGIC_64 } else { MH_MAGIC };
        header
    }
    /// Returns the cpu type
    pub fn cputype(&self) -> CpuType {
        self.cputype
    }
    /// Returns the cpu subtype with the capabilities removed
    pub fn cpusubtype(&self) -> CpuSubType {
        self.cpusubtype & !CPU_SUBTYPE_MASK
    }
    /// Returns the capabilities of the CPU
    pub fn cpu_caps(&self) -> u32 {
        (self.cpusubtype & CPU_SUBTYPE_MASK) >> 24
    }
}

impl ctx::SizeWith<container::Ctx> for Header {
    fn size_with(container: &container::Ctx) -> usize {
        match container.container {
            Container::Little => SIZEOF_HEADER_32,
            Container::Big => SIZEOF_HEADER_64,
        }
    }
}

impl ctx::SizeWith<Container> for Header {
    fn size_with(container: &Container) -> usize {
        match container {
            Container::Little => SIZEOF_HEADER_32,
            Container::Big => SIZEOF_HEADER_64,
        }
    }
}

impl<'a> ctx::TryFromCtx<'a, container::Ctx> for Header {
    type Error = crate::error::Error;
    fn try_from_ctx(
        bytes: &'a [u8],
        container::Ctx { le, container }: container::Ctx,
    ) -> error::Result<(Self, usize)> {
        let size = bytes.len();
        if size < SIZEOF_HEADER_32 || size < SIZEOF_HEADER_64 {
            let error =
                error::Error::Malformed("bytes size is smaller than a Mach-o header".into());
            Err(error)
        } else {
            match container {
                Container::Little => {
                    let header = bytes.pread_with::<Header32>(0, le)?;
                    Ok((Header::from(header), SIZEOF_HEADER_32))
                }
                Container::Big => {
                    let header = bytes.pread_with::<Header64>(0, le)?;
                    Ok((Header::from(header), SIZEOF_HEADER_64))
                }
            }
        }
    }
}

impl ctx::TryIntoCtx<container::Ctx> for Header {
    type Error = crate::error::Error;
    fn try_into_ctx(self, bytes: &mut [u8], ctx: container::Ctx) -> error::Result<usize> {
        match ctx.container {
            Container::Little => {
                bytes.pwrite_with(Header32::from(self), 0, ctx.le)?;
            }
            Container::Big => {
                bytes.pwrite_with(Header64::from(self), 0, ctx.le)?;
            }
        };
        Ok(Header::size_with(&ctx))
    }
}

impl ctx::IntoCtx<container::Ctx> for Header {
    fn into_ctx(self, bytes: &mut [u8], ctx: container::Ctx) {
        bytes.pwrite_with(self, 0, ctx).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_parse_armv7_header() {
        use crate::mach::constants::cputype::CPU_TYPE_ARM;
        const CPU_SUBTYPE_ARM_V7: u32 = 9;
        use super::Header;
        use crate::container::{Container, Ctx, Endian};
        use scroll::Pread;
        let bytes = b"\xce\xfa\xed\xfe\x0c\x00\x00\x00\t\x00\x00\x00\n\x00\x00\x00\x06\x00\x00\x00\x8c\r\x00\x00\x00\x00\x00\x00\x1b\x00\x00\x00\x18\x00\x00\x00\xe0\xf7B\xbb\x1c\xf50w\xa6\xf7u\xa3\xba(";
        let header: Header = bytes
            .pread_with(0, Ctx::new(Container::Little, Endian::Little))
            .unwrap();
        assert_eq!(header.cputype, CPU_TYPE_ARM);
        assert_eq!(header.cpusubtype, CPU_SUBTYPE_ARM_V7);
    }

    #[test]
    fn sizeof_header32() {
        assert_eq!(SIZEOF_HEADER_32, size_of::<Header32>());
    }

    #[test]
    fn sizeof_header64() {
        assert_eq!(SIZEOF_HEADER_64, size_of::<Header64>());
    }
}
