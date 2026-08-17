use crate::error;
use scroll::{Pread, Pwrite, SizeWith};

use crate::pe::data_directories;
use crate::pe::options;
use crate::pe::section_table;
use crate::pe::utils;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct DebugData<'a> {
    pub image_debug_directory: ImageDebugDirectory,
    pub codeview_pdb70_debug_info: Option<CodeviewPDB70DebugInfo<'a>>,
    pub codeview_pdb20_debug_info: Option<CodeviewPDB20DebugInfo<'a>>,
}

impl<'a> DebugData<'a> {
    pub fn parse(
        bytes: &'a [u8],
        dd: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
    ) -> error::Result<Self> {
        Self::parse_with_opts(
            bytes,
            dd,
            sections,
            file_alignment,
            &options::ParseOptions::default(),
        )
    }

    pub fn parse_with_opts(
        bytes: &'a [u8],
        dd: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
        opts: &options::ParseOptions,
    ) -> error::Result<Self> {
        let image_debug_directory =
            ImageDebugDirectory::parse_with_opts(bytes, dd, sections, file_alignment, opts)?;
        let codeview_pdb70_debug_info =
            CodeviewPDB70DebugInfo::parse_with_opts(bytes, &image_debug_directory, opts)?;
        let codeview_pdb20_debug_info =
            CodeviewPDB20DebugInfo::parse_with_opts(bytes, &image_debug_directory, opts)?;

        Ok(DebugData {
            image_debug_directory,
            codeview_pdb70_debug_info,
            codeview_pdb20_debug_info,
        })
    }

    /// Return this executable's debugging GUID, suitable for matching against a PDB file.
    pub fn guid(&self) -> Option<[u8; 16]> {
        self.codeview_pdb70_debug_info.map(|pdb70| pdb70.signature)
    }
}

// https://msdn.microsoft.com/en-us/library/windows/desktop/ms680307(v=vs.85).aspx
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pread, Pwrite, SizeWith)]
pub struct ImageDebugDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub data_type: u32,
    pub size_of_data: u32,
    pub address_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
}

pub const IMAGE_DEBUG_TYPE_UNKNOWN: u32 = 0;
pub const IMAGE_DEBUG_TYPE_COFF: u32 = 1;
pub const IMAGE_DEBUG_TYPE_CODEVIEW: u32 = 2;
pub const IMAGE_DEBUG_TYPE_FPO: u32 = 3;
pub const IMAGE_DEBUG_TYPE_MISC: u32 = 4;
pub const IMAGE_DEBUG_TYPE_EXCEPTION: u32 = 5;
pub const IMAGE_DEBUG_TYPE_FIXUP: u32 = 6;
pub const IMAGE_DEBUG_TYPE_BORLAND: u32 = 9;

impl ImageDebugDirectory {
    #[allow(unused)]
    fn parse(
        bytes: &[u8],
        dd: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
    ) -> error::Result<Self> {
        Self::parse_with_opts(
            bytes,
            dd,
            sections,
            file_alignment,
            &options::ParseOptions::default(),
        )
    }

    pub(crate) fn parse_with_opts(
        bytes: &[u8],
        dd: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
        opts: &options::ParseOptions,
    ) -> error::Result<Self> {
        let rva = dd.virtual_address as usize;
        let offset = utils::find_offset(rva, sections, file_alignment, opts).ok_or_else(|| {
            error::Error::Malformed(format!(
                "Cannot map ImageDebugDirectory rva {:#x} into offset",
                rva
            ))
        })?;
        let idd: Self = bytes.pread_with(offset, scroll::LE)?;
        Ok(idd)
    }
}

pub const CODEVIEW_PDB70_MAGIC: u32 = 0x5344_5352;
pub const CODEVIEW_PDB20_MAGIC: u32 = 0x3031_424e;
pub const CODEVIEW_CV50_MAGIC: u32 = 0x3131_424e;
pub const CODEVIEW_CV41_MAGIC: u32 = 0x3930_424e;

// http://llvm.org/doxygen/CVDebugRecord_8h_source.html
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct CodeviewPDB70DebugInfo<'a> {
    pub codeview_signature: u32,
    pub signature: [u8; 16],
    pub age: u32,
    pub filename: &'a [u8],
}

impl<'a> CodeviewPDB70DebugInfo<'a> {
    pub fn parse(bytes: &'a [u8], idd: &ImageDebugDirectory) -> error::Result<Option<Self>> {
        Self::parse_with_opts(bytes, idd, &options::ParseOptions::default())
    }

    pub fn parse_with_opts(
        bytes: &'a [u8],
        idd: &ImageDebugDirectory,
        opts: &options::ParseOptions,
    ) -> error::Result<Option<Self>> {
        if idd.data_type != IMAGE_DEBUG_TYPE_CODEVIEW {
            // not a codeview debug directory
            // that's not an error, but it's not a CodeviewPDB70DebugInfo either
            return Ok(None);
        }

        // ImageDebugDirectory.pointer_to_raw_data stores a raw offset -- not a virtual offset -- which we can use directly
        let mut offset: usize = match opts.resolve_rva {
            true => idd.pointer_to_raw_data as usize,
            false => idd.address_of_raw_data as usize,
        };

        // calculate how long the eventual filename will be, which doubles as a check of the record size
        let filename_length = idd.size_of_data as isize - 24;
        if filename_length < 0 {
            // the record is too short to be plausible
            return Err(error::Error::Malformed(format!(
                "ImageDebugDirectory size of data seems wrong: {:?}",
                idd.size_of_data
            )));
        }
        let filename_length = filename_length as usize;

        // check the codeview signature
        let codeview_signature: u32 = bytes.gread_with(&mut offset, scroll::LE)?;
        if codeview_signature != CODEVIEW_PDB70_MAGIC {
            return Ok(None);
        }

        // read the rest
        let mut signature: [u8; 16] = [0; 16];
        signature.copy_from_slice(bytes.gread_with(&mut offset, 16)?);
        let age: u32 = bytes.gread_with(&mut offset, scroll::LE)?;
        if let Some(filename) = bytes.get(offset..offset + filename_length) {
            Ok(Some(CodeviewPDB70DebugInfo {
                codeview_signature,
                signature,
                age,
                filename,
            }))
        } else {
            Err(error::Error::Malformed(format!(
                "ImageDebugDirectory seems corrupted: {:?}",
                idd
            )))
        }
    }
}

// http://llvm.org/doxygen/CVDebugRecord_8h_source.html
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct CodeviewPDB20DebugInfo<'a> {
    pub codeview_signature: u32,
    pub codeview_offset: u32,
    pub signature: u32,
    pub age: u32,
    pub filename: &'a [u8],
}

impl<'a> CodeviewPDB20DebugInfo<'a> {
    pub fn parse(bytes: &'a [u8], idd: &ImageDebugDirectory) -> error::Result<Option<Self>> {
        Self::parse_with_opts(bytes, idd, &options::ParseOptions::default())
    }

    pub fn parse_with_opts(
        bytes: &'a [u8],
        idd: &ImageDebugDirectory,
        opts: &options::ParseOptions,
    ) -> error::Result<Option<Self>> {
        if idd.data_type != IMAGE_DEBUG_TYPE_CODEVIEW {
            // not a codeview debug directory
            // that's not an error, but it's not a CodeviewPDB20DebugInfo either
            return Ok(None);
        }

        // ImageDebugDirectory.pointer_to_raw_data stores a raw offset -- not a virtual offset -- which we can use directly
        let mut offset: usize = match opts.resolve_rva {
            true => idd.pointer_to_raw_data as usize,
            false => idd.address_of_raw_data as usize,
        };

        // calculate how long the eventual filename will be, which doubles as a check of the record size
        let filename_length = idd.size_of_data as isize - 16;
        if filename_length < 0 {
            // the record is too short to be plausible
            return Err(error::Error::Malformed(format!(
                "ImageDebugDirectory size of data seems wrong: {:?}",
                idd.size_of_data
            )));
        }
        let filename_length = filename_length as usize;

        // check the codeview signature
        let codeview_signature: u32 = bytes.gread_with(&mut offset, scroll::LE)?;
        if codeview_signature != CODEVIEW_PDB20_MAGIC {
            return Ok(None);
        }
        let codeview_offset: u32 = bytes.gread_with(&mut offset, scroll::LE)?;

        // read the rest
        let signature: u32 = bytes.gread_with(&mut offset, scroll::LE)?;
        let age: u32 = bytes.gread_with(&mut offset, scroll::LE)?;
        if let Some(filename) = bytes.get(offset..offset + filename_length) {
            Ok(Some(CodeviewPDB20DebugInfo {
                codeview_signature,
                codeview_offset,
                signature,
                age,
                filename,
            }))
        } else {
            Err(error::Error::Malformed(format!(
                "ImageDebugDirectory seems corrupted: {:?}",
                idd
            )))
        }
    }
}
