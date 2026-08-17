//! A Mach-o fat binary is a multi-architecture binary container

use core::fmt;

if_std! {
    use std::fs::File;
    use std::io::{self, Read};
}

use crate::error;
use crate::mach::constants::cputype::{CpuSubType, CpuType, CPU_ARCH_ABI64, CPU_SUBTYPE_MASK};
use scroll::{Pread, Pwrite, SizeWith};

pub const FAT_MAGIC: u32 = 0xcafe_babe;
pub const FAT_CIGAM: u32 = 0xbeba_feca;

#[repr(C)]
#[derive(Clone, Copy, Default, Pread, Pwrite, SizeWith)]
/// The Mach-o `FatHeader` always has its data bigendian
pub struct FatHeader {
    /// The magic number, `cafebabe`
    pub magic: u32,
    /// How many fat architecture headers there are
    pub nfat_arch: u32,
}

pub const SIZEOF_FAT_HEADER: usize = 8;

impl fmt::Debug for FatHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FatHeader")
            .field("magic", &format_args!("0x{:x}", self.magic))
            .field("nfat_arch", &self.nfat_arch)
            .finish()
    }
}

impl FatHeader {
    /// Reinterpret a `FatHeader` from `bytes`
    pub fn from_bytes(bytes: [u8; SIZEOF_FAT_HEADER]) -> FatHeader {
        let mut offset = 0;
        let magic = bytes.gread_with(&mut offset, scroll::BE).unwrap();
        let nfat_arch = bytes.gread_with(&mut offset, scroll::BE).unwrap();
        FatHeader { magic, nfat_arch }
    }

    /// Reads a `FatHeader` from a `File` on disk
    #[cfg(feature = "std")]
    pub fn from_fd(fd: &mut File) -> io::Result<FatHeader> {
        let mut header = [0; SIZEOF_FAT_HEADER];
        fd.read_exact(&mut header)?;
        Ok(FatHeader::from_bytes(header))
    }

    /// Parse a mach-o fat header from the `bytes`
    pub fn parse(bytes: &[u8]) -> error::Result<FatHeader> {
        Ok(bytes.pread_with::<FatHeader>(0, scroll::BE)?)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Pread, Pwrite, SizeWith)]
/// The Mach-o `FatArch` always has its data bigendian
pub struct FatArch {
    /// What kind of CPU this binary is
    pub cputype: u32,
    pub cpusubtype: u32,
    /// Where in the fat binary it starts
    pub offset: u32,
    /// How big the binary is
    pub size: u32,
    pub align: u32,
}

pub const SIZEOF_FAT_ARCH: usize = 20;

impl fmt::Debug for FatArch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("FatArch")
            .field("cputype", &self.cputype())
            .field("cmdsize", &self.cpusubtype())
            .field("offset", &format_args!("{:#x}", &self.offset))
            .field("size", &self.size)
            .field("align", &self.align)
            .finish()
    }
}

impl FatArch {
    /// Get the slice of bytes this header describes from `bytes`
    pub fn slice<'a>(&self, bytes: &'a [u8]) -> &'a [u8] {
        // FIXME: This function should ideally validate the inputs and return a `Result`.
        // Best we can do for now without `panic`ing is return an empty slice.
        let start = self.offset as usize;
        match start
            .checked_add(self.size as usize)
            .and_then(|end| bytes.get(start..end))
        {
            Some(slice) => slice,
            None => {
                log::warn!("invalid `FatArch` offset");
                &[]
            }
        }
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

    /// Whether this fat architecture header describes a 64-bit binary
    pub fn is_64(&self) -> bool {
        (self.cputype & CPU_ARCH_ABI64) == CPU_ARCH_ABI64
    }

    /// Parse a `FatArch` header from `bytes` at `offset`
    pub fn parse(bytes: &[u8], offset: usize) -> error::Result<Self> {
        let arch = bytes.pread_with::<FatArch>(offset, scroll::BE)?;
        Ok(arch)
    }
}
