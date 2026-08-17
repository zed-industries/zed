use crate::error::{self, Error};
use crate::pe::relocation;
use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use scroll::{ctx, Pread, Pwrite};

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Default)]
pub struct SectionTable {
    pub name: [u8; 8],
    pub real_name: Option<String>,
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

pub const SIZEOF_SECTION_TABLE: usize = 8 * 5;

// Based on https://github.com/llvm-mirror/llvm/blob/af7b1832a03ab6486c42a40d21695b2c03b2d8a3/lib/Object/COFFObjectFile.cpp#L70
// Decodes a string table entry in base 64 (//AAAAAA). Expects string without
// prefixed slashes.
fn base64_decode_string_entry(s: &str) -> Result<usize, ()> {
    assert!(s.len() <= 6, "String too long, possible overflow.");

    let mut val = 0;
    for c in s.bytes() {
        let v = if b'A' <= c && c <= b'Z' {
            // 00..=25
            c - b'A'
        } else if b'a' <= c && c <= b'z' {
            // 26..=51
            c - b'a' + 26
        } else if b'0' <= c && c <= b'9' {
            // 52..=61
            c - b'0' + 52
        } else if c == b'+' {
            // 62
            62
        } else if c == b'/' {
            // 63
            63
        } else {
            return Err(());
        };
        val = val * 64 + v as usize;
    }
    Ok(val)
}

impl SectionTable {
    pub fn parse(
        bytes: &[u8],
        offset: &mut usize,
        string_table_offset: usize,
    ) -> error::Result<Self> {
        let mut table = SectionTable::default();
        let mut name = [0u8; 8];
        name.copy_from_slice(bytes.gread_with(offset, 8)?);

        table.name = name;
        table.virtual_size = bytes.gread_with(offset, scroll::LE)?;
        table.virtual_address = bytes.gread_with(offset, scroll::LE)?;
        table.size_of_raw_data = bytes.gread_with(offset, scroll::LE)?;
        table.pointer_to_raw_data = bytes.gread_with(offset, scroll::LE)?;
        table.pointer_to_relocations = bytes.gread_with(offset, scroll::LE)?;
        table.pointer_to_linenumbers = bytes.gread_with(offset, scroll::LE)?;
        table.number_of_relocations = bytes.gread_with(offset, scroll::LE)?;
        table.number_of_linenumbers = bytes.gread_with(offset, scroll::LE)?;
        table.characteristics = bytes.gread_with(offset, scroll::LE)?;

        if let Some(idx) = table.name_offset()? {
            table.real_name = Some(bytes.pread::<&str>(string_table_offset + idx)?.to_string());
        }
        Ok(table)
    }

    pub fn data<'a, 'b: 'a>(&'a self, pe_bytes: &'b [u8]) -> error::Result<Option<Cow<[u8]>>> {
        let section_start: usize = self.pointer_to_raw_data.try_into().map_err(|_| {
            Error::Malformed(format!("Virtual address cannot fit in platform `usize`"))
        })?;

        // assert!(self.virtual_size <= self.size_of_raw_data);
        // if vsize > size_of_raw_data, the section is zero padded.
        let section_end: usize = section_start
            + usize::try_from(self.size_of_raw_data).map_err(|_| {
                Error::Malformed(format!("Virtual size cannot fit in platform `usize`"))
            })?;

        let original_bytes = pe_bytes.get(section_start..section_end).map(Cow::Borrowed);

        if original_bytes.is_some() && self.virtual_size > self.size_of_raw_data {
            let mut bytes: Vec<u8> = Vec::new();
            bytes.resize(self.size_of_raw_data.try_into()?, 0);
            bytes.copy_from_slice(&original_bytes.unwrap());
            bytes.resize(self.virtual_size.try_into()?, 0);

            Ok(Some(Cow::Owned(bytes)))
        } else {
            Ok(original_bytes)
        }
    }

    pub fn name_offset(&self) -> error::Result<Option<usize>> {
        // Based on https://github.com/llvm-mirror/llvm/blob/af7b1832a03ab6486c42a40d21695b2c03b2d8a3/lib/Object/COFFObjectFile.cpp#L1054
        if self.name[0] == b'/' {
            let idx: usize = if self.name[1] == b'/' {
                let b64idx = self.name.pread::<&str>(2)?;
                base64_decode_string_entry(b64idx).map_err(|_| {
                    Error::Malformed(format!(
                        "Invalid indirect section name //{}: base64 decoding failed",
                        b64idx
                    ))
                })?
            } else {
                let name = self.name.pread::<&str>(1)?;
                name.parse().map_err(|err| {
                    Error::Malformed(format!("Invalid indirect section name /{}: {}", name, err))
                })?
            };
            Ok(Some(idx))
        } else {
            Ok(None)
        }
    }

    #[allow(clippy::useless_let_if_seq)]
    pub fn set_name_offset(&mut self, mut idx: usize) -> error::Result<()> {
        if idx <= 9_999_999 {
            // 10^7 - 1
            // write!(&mut self.name[1..], "{}", idx) without using io::Write.
            // We write into a temporary since we calculate digits starting at the right.
            let mut name = [0; 7];
            let mut len = 0;
            if idx == 0 {
                name[6] = b'0';
                len = 1;
            } else {
                while idx != 0 {
                    let rem = (idx % 10) as u8;
                    idx /= 10;
                    name[6 - len] = b'0' + rem;
                    len += 1;
                }
            }
            self.name = [0; 8];
            self.name[0] = b'/';
            self.name[1..][..len].copy_from_slice(&name[7 - len..]);
            Ok(())
        } else if idx as u64 <= 0xfff_fff_fff {
            // 64^6 - 1
            self.name[0] = b'/';
            self.name[1] = b'/';
            for i in 0..6 {
                let rem = (idx % 64) as u8;
                idx /= 64;
                let c = match rem {
                    0..=25 => b'A' + rem,
                    26..=51 => b'a' + rem - 26,
                    52..=61 => b'0' + rem - 52,
                    62 => b'+',
                    63 => b'/',
                    _ => unreachable!(),
                };
                self.name[7 - i] = c;
            }
            Ok(())
        } else {
            Err(Error::Malformed(format!(
                "Invalid section name offset: {}",
                idx
            )))
        }
    }

    pub fn name(&self) -> error::Result<&str> {
        match self.real_name.as_ref() {
            Some(s) => Ok(s),
            None => Ok(self.name.pread(0)?),
        }
    }

    pub fn relocations<'a>(&self, bytes: &'a [u8]) -> error::Result<relocation::Relocations<'a>> {
        let offset = self.pointer_to_relocations as usize;
        let number = self.number_of_relocations as usize;
        relocation::Relocations::parse(bytes, offset, number)
    }

    /// Tests if `another_section` on-disk ranges will collide.
    pub fn overlaps_with(&self, another_section: &SectionTable) -> bool {
        let self_end = self.pointer_to_raw_data + self.size_of_raw_data;
        let another_end = another_section.pointer_to_raw_data + another_section.size_of_raw_data;

        !((self_end <= another_section.pointer_to_raw_data)
            || (another_end <= self.pointer_to_raw_data))
    }
}

impl ctx::SizeWith<scroll::Endian> for SectionTable {
    fn size_with(_ctx: &scroll::Endian) -> usize {
        SIZEOF_SECTION_TABLE
    }
}

impl ctx::TryIntoCtx<scroll::Endian> for &SectionTable {
    type Error = error::Error;
    fn try_into_ctx(self, bytes: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        bytes.gwrite(&self.name[..], offset)?;
        bytes.gwrite_with(self.virtual_size, offset, ctx)?;
        bytes.gwrite_with(self.virtual_address, offset, ctx)?;
        bytes.gwrite_with(self.size_of_raw_data, offset, ctx)?;
        bytes.gwrite_with(self.pointer_to_raw_data, offset, ctx)?;
        bytes.gwrite_with(self.pointer_to_relocations, offset, ctx)?;
        bytes.gwrite_with(self.pointer_to_linenumbers, offset, ctx)?;
        bytes.gwrite_with(self.number_of_relocations, offset, ctx)?;
        bytes.gwrite_with(self.number_of_linenumbers, offset, ctx)?;
        bytes.gwrite_with(self.characteristics, offset, ctx)?;
        Ok(SIZEOF_SECTION_TABLE)
    }
}

impl ctx::IntoCtx<scroll::Endian> for &SectionTable {
    fn into_ctx(self, bytes: &mut [u8], ctx: scroll::Endian) {
        bytes.pwrite_with(self, 0, ctx).unwrap();
    }
}

/// The section should not be padded to the next boundary. This flag is obsolete and is replaced
/// by `IMAGE_SCN_ALIGN_1BYTES`. This is valid only for object files.
pub const IMAGE_SCN_TYPE_NO_PAD: u32 = 0x0000_0008;
/// The section contains executable code.
pub const IMAGE_SCN_CNT_CODE: u32 = 0x0000_0020;
/// The section contains initialized data.
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 0x0000_0040;
///  The section contains uninitialized data.
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x0000_0080;
pub const IMAGE_SCN_LNK_OTHER: u32 = 0x0000_0100;
/// The section contains comments or other information. The .drectve section has this type.
/// This is valid for object files only.
pub const IMAGE_SCN_LNK_INFO: u32 = 0x0000_0200;
/// The section will not become part of the image. This is valid only for object files.
pub const IMAGE_SCN_LNK_REMOVE: u32 = 0x0000_0800;
/// The section contains COMDAT data. This is valid only for object files.
pub const IMAGE_SCN_LNK_COMDAT: u32 = 0x0000_1000;
/// The section contains data referenced through the global pointer (GP).
pub const IMAGE_SCN_GPREL: u32 = 0x0000_8000;
pub const IMAGE_SCN_MEM_PURGEABLE: u32 = 0x0002_0000;
pub const IMAGE_SCN_MEM_16BIT: u32 = 0x0002_0000;
pub const IMAGE_SCN_MEM_LOCKED: u32 = 0x0004_0000;
pub const IMAGE_SCN_MEM_PRELOAD: u32 = 0x0008_0000;

pub const IMAGE_SCN_ALIGN_1BYTES: u32 = 0x0010_0000;
pub const IMAGE_SCN_ALIGN_2BYTES: u32 = 0x0020_0000;
pub const IMAGE_SCN_ALIGN_4BYTES: u32 = 0x0030_0000;
pub const IMAGE_SCN_ALIGN_8BYTES: u32 = 0x0040_0000;
pub const IMAGE_SCN_ALIGN_16BYTES: u32 = 0x0050_0000;
pub const IMAGE_SCN_ALIGN_32BYTES: u32 = 0x0060_0000;
pub const IMAGE_SCN_ALIGN_64BYTES: u32 = 0x0070_0000;
pub const IMAGE_SCN_ALIGN_128BYTES: u32 = 0x0080_0000;
pub const IMAGE_SCN_ALIGN_256BYTES: u32 = 0x0090_0000;
pub const IMAGE_SCN_ALIGN_512BYTES: u32 = 0x00A0_0000;
pub const IMAGE_SCN_ALIGN_1024BYTES: u32 = 0x00B0_0000;
pub const IMAGE_SCN_ALIGN_2048BYTES: u32 = 0x00C0_0000;
pub const IMAGE_SCN_ALIGN_4096BYTES: u32 = 0x00D0_0000;
pub const IMAGE_SCN_ALIGN_8192BYTES: u32 = 0x00E0_0000;
pub const IMAGE_SCN_ALIGN_MASK: u32 = 0x00F0_0000;

/// The section contains extended relocations.
pub const IMAGE_SCN_LNK_NRELOC_OVFL: u32 = 0x0100_0000;
/// The section can be discarded as needed.
pub const IMAGE_SCN_MEM_DISCARDABLE: u32 = 0x0200_0000;
/// The section cannot be cached.
pub const IMAGE_SCN_MEM_NOT_CACHED: u32 = 0x0400_0000;
/// The section is not pageable.
pub const IMAGE_SCN_MEM_NOT_PAGED: u32 = 0x0800_0000;
/// The section can be shared in memory.
pub const IMAGE_SCN_MEM_SHARED: u32 = 0x1000_0000;
/// The section can be executed as code.
pub const IMAGE_SCN_MEM_EXECUTE: u32 = 0x2000_0000;
/// The section can be read.
pub const IMAGE_SCN_MEM_READ: u32 = 0x4000_0000;
/// The section can be written to.
pub const IMAGE_SCN_MEM_WRITE: u32 = 0x8000_0000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_name_offset() {
        let mut section = SectionTable::default();
        for &(offset, name) in [
            (0usize, b"/0\0\0\0\0\0\0"),
            (1, b"/1\0\0\0\0\0\0"),
            (9_999_999, b"/9999999"),
            (10_000_000, b"//AAmJaA"),
            #[cfg(target_pointer_width = "64")]
            (0xfff_fff_fff, b"////////"),
        ]
        .iter()
        {
            section.set_name_offset(offset).unwrap();
            assert_eq!(&section.name, name);
            assert_eq!(section.name_offset().unwrap(), Some(offset));
        }
        #[cfg(target_pointer_width = "64")]
        assert!(section.set_name_offset(0x1_000_000_000).is_err());
    }
}
