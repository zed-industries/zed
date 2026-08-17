use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::fmt::{Debug, LowerHex};

use crate::error;
use scroll::ctx::TryFromCtx;
use scroll::{Pread, Pwrite, SizeWith};

use crate::pe::data_directories;
use crate::pe::options;
use crate::pe::section_table;
use crate::pe::utils;

use log::{debug, warn};

pub const IMPORT_BY_ORDINAL_32: u32 = 0x8000_0000;
pub const IMPORT_BY_ORDINAL_64: u64 = 0x8000_0000_0000_0000;
pub const IMPORT_RVA_MASK_32: u32 = 0x7fff_ffff;
pub const IMPORT_RVA_MASK_64: u64 = 0x0000_0000_7fff_ffff;

pub trait Bitfield<'a>:
    Into<u64>
    + PartialEq
    + Eq
    + LowerHex
    + Debug
    + TryFromCtx<'a, scroll::Endian, Error = scroll::Error>
{
    fn is_ordinal(&self) -> bool;
    fn to_ordinal(&self) -> u16;
    fn to_rva(&self) -> u32;
    fn size_of() -> usize;
    fn is_zero(&self) -> bool;
}

impl<'a> Bitfield<'a> for u64 {
    fn is_ordinal(&self) -> bool {
        self & IMPORT_BY_ORDINAL_64 == IMPORT_BY_ORDINAL_64
    }
    fn to_ordinal(&self) -> u16 {
        (0xffff & self) as u16
    }
    fn to_rva(&self) -> u32 {
        (self & IMPORT_RVA_MASK_64) as u32
    }
    fn size_of() -> usize {
        8
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl<'a> Bitfield<'a> for u32 {
    fn is_ordinal(&self) -> bool {
        self & IMPORT_BY_ORDINAL_32 == IMPORT_BY_ORDINAL_32
    }
    fn to_ordinal(&self) -> u16 {
        (0xffff & self) as u16
    }
    fn to_rva(&self) -> u32 {
        (self & IMPORT_RVA_MASK_32) as u32
    }
    fn size_of() -> usize {
        4
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

#[derive(Debug, Clone)]
pub struct HintNameTableEntry<'a> {
    pub hint: u16,
    pub name: &'a str,
}

impl<'a> HintNameTableEntry<'a> {
    fn parse(bytes: &'a [u8], mut offset: usize) -> error::Result<Self> {
        let offset = &mut offset;
        let hint = bytes.gread_with(offset, scroll::LE)?;
        let name = bytes.pread::<&'a str>(*offset)?;
        Ok(HintNameTableEntry { hint, name })
    }
}

#[derive(Debug, Clone)]
pub enum SyntheticImportLookupTableEntry<'a> {
    OrdinalNumber(u16),
    HintNameTableRVA((u32, HintNameTableEntry<'a>)), // [u8; 31] bitfield :/
}

pub type ImportLookupTable<'a> = Vec<SyntheticImportLookupTableEntry<'a>>;

impl<'a> SyntheticImportLookupTableEntry<'a> {
    pub fn parse<T: Bitfield<'a>>(
        bytes: &'a [u8],
        offset: usize,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
    ) -> error::Result<ImportLookupTable<'a>> {
        Self::parse_with_opts::<T>(
            bytes,
            offset,
            sections,
            file_alignment,
            &options::ParseOptions::default(),
        )
    }

    pub fn parse_with_opts<T: Bitfield<'a>>(
        bytes: &'a [u8],
        mut offset: usize,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
        opts: &options::ParseOptions,
    ) -> error::Result<ImportLookupTable<'a>> {
        let le = scroll::LE;
        let offset = &mut offset;
        let mut table = Vec::new();
        loop {
            let bitfield: T = bytes.gread_with(offset, le)?;
            if bitfield.is_zero() {
                debug!("imports done");
                break;
            } else {
                let entry = {
                    debug!("bitfield {:#x}", bitfield);
                    use self::SyntheticImportLookupTableEntry::*;
                    if bitfield.is_ordinal() {
                        let ordinal = bitfield.to_ordinal();
                        debug!("importing by ordinal {:#x} ({})", ordinal, ordinal);
                        OrdinalNumber(ordinal)
                    } else {
                        let rva = bitfield.to_rva();
                        let hentry = {
                            debug!("searching for RVA {:#x}", rva);
                            if let Some(offset) =
                                utils::find_offset(rva as usize, sections, file_alignment, opts)
                            {
                                debug!("offset {:#x}", offset);
                                HintNameTableEntry::parse(bytes, offset)?
                            } else {
                                warn!("Entry {} has bad RVA: {:#x}", table.len(), rva);
                                continue;
                            }
                        };
                        HintNameTableRVA((rva, hentry))
                    }
                };
                table.push(entry);
            }
        }
        Ok(table)
    }
}

// get until entry is 0
pub type ImportAddressTable = Vec<u64>;

#[repr(C)]
#[derive(Debug, Pread, Pwrite, SizeWith)]
pub struct ImportDirectoryEntry {
    pub import_lookup_table_rva: u32,
    pub time_date_stamp: u32,
    pub forwarder_chain: u32,
    pub name_rva: u32,
    pub import_address_table_rva: u32,
}

pub const SIZEOF_IMPORT_DIRECTORY_ENTRY: usize = 20;

impl ImportDirectoryEntry {
    /// Whether the entire fields are set to zero
    pub fn is_null(&self) -> bool {
        (self.import_lookup_table_rva == 0)
            && (self.time_date_stamp == 0)
            && (self.forwarder_chain == 0)
            && (self.name_rva == 0)
            && (self.import_address_table_rva == 0)
    }

    /// Whether the entry is _possibly_ valid.
    ///
    /// Both [`Self::name_rva`] and [`Self::import_address_table_rva`] must be non-zero
    pub fn is_possibly_valid(&self) -> bool {
        self.name_rva != 0 && self.import_address_table_rva != 0
    }
}

#[derive(Debug)]
pub struct SyntheticImportDirectoryEntry<'a> {
    pub import_directory_entry: ImportDirectoryEntry,
    /// Computed
    pub name: &'a str,
    /// The import lookup table is a vector of either ordinals, or RVAs + import names
    pub import_lookup_table: Option<ImportLookupTable<'a>>,
    /// Computed
    pub import_address_table: ImportAddressTable,
}

impl<'a> SyntheticImportDirectoryEntry<'a> {
    pub fn parse<T: Bitfield<'a>>(
        bytes: &'a [u8],
        import_directory_entry: ImportDirectoryEntry,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
    ) -> error::Result<SyntheticImportDirectoryEntry<'a>> {
        Self::parse_with_opts::<T>(
            bytes,
            import_directory_entry,
            sections,
            file_alignment,
            &options::ParseOptions::default(),
        )
    }

    pub fn parse_with_opts<T: Bitfield<'a>>(
        bytes: &'a [u8],
        import_directory_entry: ImportDirectoryEntry,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
        opts: &options::ParseOptions,
    ) -> error::Result<SyntheticImportDirectoryEntry<'a>> {
        const LE: scroll::Endian = scroll::LE;
        let name_rva = import_directory_entry.name_rva;
        let name = utils::try_name(bytes, name_rva as usize, sections, file_alignment, opts)?;
        let import_lookup_table = {
            let import_lookup_table_rva = import_directory_entry.import_lookup_table_rva;
            let import_address_table_rva = import_directory_entry.import_address_table_rva;
            if let Some(import_lookup_table_offset) = utils::find_offset(
                import_lookup_table_rva as usize,
                sections,
                file_alignment,
                opts,
            ) {
                debug!("Synthesizing lookup table imports for {} lib, with import lookup table rva: {:#x}", name, import_lookup_table_rva);
                let import_lookup_table = SyntheticImportLookupTableEntry::parse_with_opts::<T>(
                    bytes,
                    import_lookup_table_offset,
                    sections,
                    file_alignment,
                    opts,
                )?;
                debug!(
                    "Successfully synthesized import lookup table entry from lookup table: {:#?}",
                    import_lookup_table
                );
                Some(import_lookup_table)
            } else if let Some(import_address_table_offset) = utils::find_offset(
                import_address_table_rva as usize,
                sections,
                file_alignment,
                opts,
            ) {
                debug!("Synthesizing lookup table imports for {} lib, with import address table rva: {:#x}", name, import_lookup_table_rva);
                let import_address_table = SyntheticImportLookupTableEntry::parse_with_opts::<T>(
                    bytes,
                    import_address_table_offset,
                    sections,
                    file_alignment,
                    opts,
                )?;
                debug!(
                    "Successfully synthesized import lookup table entry from IAT: {:#?}",
                    import_address_table
                );
                Some(import_address_table)
            } else {
                None
            }
        };

        let rva = match import_directory_entry.import_address_table_rva.is_zero() {
            true => import_directory_entry.import_lookup_table_rva,
            false => import_directory_entry.import_address_table_rva,
        };

        let import_address_table_offset =
            &mut utils::find_offset(rva as usize, sections, file_alignment, opts).ok_or_else(
                || {
                    let target = if import_directory_entry.import_address_table_rva.is_zero() {
                        "import_lookup_table_rva"
                    } else {
                        "import_address_table_rva"
                    };
                    error::Error::Malformed(format!(
                        "Cannot map {} {:#x} into offset for {}",
                        target, rva, name
                    ))
                },
            )?;
        let mut import_address_table = Vec::new();
        loop {
            let import_address = bytes
                .gread_with::<T>(import_address_table_offset, LE)?
                .into();
            if import_address == 0 {
                break;
            } else {
                import_address_table.push(import_address);
            }
        }
        Ok(SyntheticImportDirectoryEntry {
            import_directory_entry,
            name,
            import_lookup_table,
            import_address_table,
        })
    }
}

#[derive(Debug)]
/// Contains a list of synthesized import data for this binary, e.g., which symbols from which libraries it is importing from
pub struct ImportData<'a> {
    pub import_data: Vec<SyntheticImportDirectoryEntry<'a>>,
}

impl<'a> ImportData<'a> {
    pub fn parse<T: Bitfield<'a>>(
        bytes: &'a [u8],
        dd: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
    ) -> error::Result<ImportData<'a>> {
        Self::parse_with_opts::<T>(
            bytes,
            dd,
            sections,
            file_alignment,
            &options::ParseOptions::default(),
        )
    }

    pub fn parse_with_opts<T: Bitfield<'a>>(
        bytes: &'a [u8],
        dd: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
        opts: &options::ParseOptions,
    ) -> error::Result<ImportData<'a>> {
        let import_directory_table_rva = dd.virtual_address as usize;
        debug!(
            "import_directory_table_rva {:#x}",
            import_directory_table_rva
        );
        let offset =
            &mut utils::find_offset(import_directory_table_rva, sections, file_alignment, opts)
                .ok_or_else(|| {
                    error::Error::Malformed(format!(
                "Cannot create ImportData; cannot map import_directory_table_rva {:#x} into offset",
                import_directory_table_rva
            ))
                })?;
        debug!("import data offset {:#x}", offset);
        let mut import_data = Vec::new();
        loop {
            let import_directory_entry: ImportDirectoryEntry =
                bytes.gread_with(offset, scroll::LE)?;
            debug!("{:#?} at {:#x}", import_directory_entry, offset);
            if import_directory_entry.is_null() || !import_directory_entry.is_possibly_valid() {
                break;
            } else {
                let entry = SyntheticImportDirectoryEntry::parse_with_opts::<T>(
                    bytes,
                    import_directory_entry,
                    sections,
                    file_alignment,
                    opts,
                )?;
                debug!("entry {:#?} at {:#x}", entry, offset);
                import_data.push(entry);
            }
        }
        debug!("finished ImportData");
        Ok(ImportData { import_data })
    }
}

#[derive(Debug)]
/// A synthesized symbol import, the name is pre-indexed, and the binary offset is computed, as well as which dll it belongs to
pub struct Import<'a> {
    pub name: Cow<'a, str>,
    pub dll: &'a str,
    pub ordinal: u16,
    pub offset: usize,
    pub rva: usize,
    pub size: usize,
}

impl<'a> Import<'a> {
    pub fn parse<T: Bitfield<'a>>(
        _bytes: &'a [u8],
        import_data: &ImportData<'a>,
        _sections: &[section_table::SectionTable],
    ) -> error::Result<Vec<Import<'a>>> {
        let mut imports = Vec::new();
        for data in &import_data.import_data {
            if let Some(ref import_lookup_table) = data.import_lookup_table {
                let dll = data.name;
                let import_base = data.import_directory_entry.import_address_table_rva as usize;
                debug!("Getting imports from {}", &dll);
                for (i, entry) in import_lookup_table.iter().enumerate() {
                    let offset = import_base + (i * T::size_of());
                    use self::SyntheticImportLookupTableEntry::*;
                    let (rva, name, ordinal) = match *entry {
                        HintNameTableRVA((rva, ref hint_entry)) => {
                            // if hint_entry.name = "" && hint_entry.hint = 0 {
                            //     println!("<PE.Import> warning hint/name table rva from {} without hint {:#x}", dll, rva);
                            // }
                            (rva, Cow::Borrowed(hint_entry.name), hint_entry.hint)
                        }
                        OrdinalNumber(ordinal) => {
                            let name = format!("ORDINAL {}", ordinal);
                            (0x0, Cow::Owned(name), ordinal)
                        }
                    };
                    let import = Import {
                        name,
                        ordinal,
                        dll,
                        size: T::size_of(),
                        offset,
                        rva: rva as usize,
                    };
                    imports.push(import);
                }
            }
        }
        Ok(imports)
    }
}

#[cfg(test)]
mod tests {
    const NOT_WELL_FORMED_IMPORT: &[u8] =
        include_bytes!("../../tests/bins/pe/not_well_formed_import.exe.bin");
    const WELL_FORMED_IMPORT: &[u8] =
        include_bytes!("../../tests/bins/pe/well_formed_import.exe.bin");

    #[test]
    fn parse_non_well_formed_import_table() {
        let binary = crate::pe::PE::parse(NOT_WELL_FORMED_IMPORT).expect("Unable to parse binary");
        assert_eq!(binary.import_data.is_some(), true);
        assert_eq!(binary.imports.len(), 1);
        assert_eq!(binary.imports[0].name, "ORDINAL 51398");
        assert_eq!(binary.imports[0].dll, "abcd.dll");
        assert_eq!(binary.imports[0].ordinal, 51398);
        assert_eq!(binary.imports[0].offset, 0x7014);
        assert_eq!(binary.imports[0].rva, 0);
        assert_eq!(binary.imports[0].size, 8);
    }

    #[test]
    fn parse_well_formed_import_table() {
        let binary = crate::pe::PE::parse(WELL_FORMED_IMPORT).expect("Unable to parse binary");
        assert_eq!(binary.import_data.is_some(), true);
        assert_eq!(binary.imports.len(), 1);
        assert_eq!(binary.imports[0].name, "GetLastError");
        assert_eq!(binary.imports[0].dll, "KERNEL32.dll");
        assert_eq!(binary.imports[0].ordinal, 647);
        assert_eq!(binary.imports[0].offset, 0x2000);
        assert_eq!(binary.imports[0].rva, 0x21B8);
        assert_eq!(binary.imports[0].size, 8);
    }
}
