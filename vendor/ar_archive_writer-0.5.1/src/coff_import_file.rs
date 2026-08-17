// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result, Seek, Write};
use std::mem::{offset_of, size_of};
use std::path::PathBuf;
use std::str::from_utf8;

use object::pe::{
    IMAGE_FILE_32BIT_MACHINE, IMAGE_REL_AMD64_ADDR32NB, IMAGE_REL_ARM_ADDR32NB,
    IMAGE_REL_ARM64_ADDR32NB, IMAGE_REL_I386_DIR32NB, IMAGE_SCN_ALIGN_2BYTES,
    IMAGE_SCN_ALIGN_4BYTES, IMAGE_SCN_ALIGN_8BYTES, IMAGE_SCN_CNT_INITIALIZED_DATA,
    IMAGE_SCN_LNK_INFO, IMAGE_SCN_LNK_REMOVE, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE,
    IMAGE_SYM_CLASS_EXTERNAL, IMAGE_SYM_CLASS_NULL, IMAGE_SYM_CLASS_SECTION,
    IMAGE_SYM_CLASS_STATIC, IMAGE_SYM_CLASS_WEAK_EXTERNAL, IMAGE_WEAK_EXTERN_SEARCH_ALIAS,
    ImageFileHeader, ImageImportDescriptor, ImageRelocation, ImageSectionHeader, ImageSymbol,
    ImportObjectHeader,
};
use object::pod::bytes_of;

use crate::coff::{ImportNameType, ImportType, MachineTypes, is_arm64ec};
use crate::mangler::{get_arm64ec_demangled_function_name, get_arm64ec_mangled_function_name};
use crate::{ArchiveKind, DEFAULT_OBJECT_READER, NewArchiveMember, write_archive_to_stream};

pub(crate) const IMPORT_DESCRIPTOR_PREFIX: &[u8] = b"__IMPORT_DESCRIPTOR_";
pub(crate) const NULL_IMPORT_DESCRIPTOR_SYMBOL_NAME: &[u8] = b"__NULL_IMPORT_DESCRIPTOR";
pub(crate) const NULL_IMPORT_DESCRIPTOR_SYMBOL_NAME_PREFIX: &[u8] = b"__NULL_IMPORT_DESCRIPTOR_";
pub(crate) const NULL_THUNK_DATA_PREFIX: &[u8] = b"\x7f";
pub(crate) const NULL_THUNK_DATA_SUFFIX: &[u8] = b"_NULL_THUNK_DATA";

macro_rules! u16 {
    ($val:expr_2021) => {
        object::U16::new(object::LittleEndian, $val)
    };
}

macro_rules! u32 {
    ($val:expr_2021) => {
        object::U32::new(object::LittleEndian, $val)
    };
}

// Derived from COFFImportFile::printSymbolName and COFFImportFile::symbol_end.
pub(crate) fn get_short_import_symbol(
    buf: &[u8],
    f: &mut dyn FnMut(&[u8]) -> Result<()>,
) -> Result<bool> {
    let mut offset = 0;
    let header = ImportObjectHeader::parse(buf, &mut offset).map_err(Error::other)?;
    let data = header.parse_data(buf, &mut offset).map_err(Error::other)?;
    let is_ec = header.machine.get(object::LittleEndian) == object::pe::IMAGE_FILE_MACHINE_ARM64EC;

    let name = data.symbol();
    let demangled_name = is_ec
        .then(|| get_arm64ec_demangled_function_name(from_utf8(name).unwrap()))
        .flatten()
        .map_or_else(
            || Cow::Borrowed(name),
            |demangled_name| Cow::Owned(demangled_name.into_bytes()),
        );

    // Import symbol is first.
    const IMP_PREFIX: &[u8] = b"__imp_";
    f(&IMP_PREFIX
        .iter()
        .chain(demangled_name.as_ref())
        .copied()
        .collect::<Vec<_>>())?;

    // For data, only the import symbol is needed.
    if header.import_type() == ImportType::Data.into() {
        return Ok(true);
    }

    // Next, thunk.
    f(demangled_name.as_ref())?;

    // For Arm64EC, also the EC import symbol and thunk.
    if header.machine.get(object::LittleEndian) == object::pe::IMAGE_FILE_MACHINE_ARM64EC {
        const IMP_PREFIX: &[u8] = b"__imp_aux_";
        f(&IMP_PREFIX
            .iter()
            .chain(demangled_name.as_ref())
            .copied()
            .collect::<Vec<_>>())?;

        f(name)?;
    }

    Ok(true)
}

const READER_FOR_SHORT_IMPORT: crate::ObjectReader = crate::ObjectReader {
    get_symbols: get_short_import_symbol,
    ..crate::DEFAULT_OBJECT_READER
};

pub struct COFFShortExport {
    /// The name of the export as specified in the .def file or on the command
    /// line, i.e. "foo" in "/EXPORT:foo", and "bar" in "/EXPORT:foo=bar". This
    /// may lack mangling, such as underscore prefixing and stdcall suffixing.
    pub name: String,

    /// The external, exported name. Only non-empty when export renaming is in
    /// effect, i.e. "foo" in "/EXPORT:foo=bar".
    pub ext_name: Option<String>,

    /// The real, mangled symbol name from the object file. Given
    /// "/export:foo=bar", this could be "_bar@8" if bar is stdcall.
    pub symbol_name: Option<String>,

    /// Creates an import library entry that imports from a DLL export with a
    /// different name. This is the name of the DLL export that should be
    /// referenced when linking against this import library entry. In a .def
    /// file, this is "baz" in "EXPORTS\nfoo = bar == baz".
    pub import_name: Option<String>,

    /// Specifies EXPORTAS name. In a .def file, this is "bar" in
    /// "EXPORTS\nfoo EXPORTAS bar".
    pub export_as: Option<String>,

    pub ordinal: u16,
    pub noname: bool,
    pub data: bool,
    pub private: bool,
    pub constant: bool,
}

fn set_name_to_string_table_entry(symbol: &mut ImageSymbol, offset: usize) {
    // If first 4 bytes are 0, then second 4 bytes are offset into string table.
    symbol.name[..4].copy_from_slice(&[0; 4]);
    symbol.name[4..].copy_from_slice(&u32::try_from(offset).unwrap().to_le_bytes());
}

fn apply_name_type(import_type: ImportNameType, name: &str) -> &str {
    fn ltrim1<'a>(name: &'a str, chars: &str) -> &'a str {
        if let Some((first_char, rest)) = name.split_at_checked(1)
            && chars.contains(first_char)
        {
            return rest;
        }
        name
    }

    match import_type {
        ImportNameType::NameNoprefix => ltrim1(name, "?@_"),
        ImportNameType::NameUndecorate => {
            let name = ltrim1(name, "?@_");
            &name[..name.find('@').unwrap_or(name.len())]
        }
        _ => name,
    }
}

fn get_img_rel_relocation(machine: MachineTypes) -> object::U16<object::LittleEndian> {
    u16!(match machine {
        MachineTypes::AMD64 => IMAGE_REL_AMD64_ADDR32NB,
        MachineTypes::ARMNT => IMAGE_REL_ARM_ADDR32NB,
        MachineTypes::ARM64 | MachineTypes::ARM64EC | MachineTypes::ARM64X =>
            IMAGE_REL_ARM64_ADDR32NB,
        MachineTypes::I386 => IMAGE_REL_I386_DIR32NB,
    })
}

fn write_string_table(b: &mut Vec<u8>, strings: &[&[u8]]) -> Result<()> {
    // The COFF string table consists of a 4-byte value which is the size of the
    // table, including the length field itself.  This value is followed by the
    // string content itself, which is an array of null-terminated C-style
    // strings.  The termination is important as they are referenced to by offset
    // by the symbol entity in the file format.

    let offset = b.len();

    // Skip over the length field, we will fill it in later as we will have
    // computed the length while emitting the string content itself.
    b.extend(0u32.to_le_bytes());

    for s in strings {
        b.write_all(s)?;
        b.write_all(&[0])?;
    }

    // Backfill the length of the table now that it has been computed.
    let length: u32 = (b.len() - offset).try_into().unwrap();
    b[offset..offset + size_of::<u32>()].copy_from_slice(&length.to_le_bytes());

    Ok(())
}

fn get_name_type(sym: &str, ext_name: &str, machine: MachineTypes, mingw: bool) -> ImportNameType {
    // A decorated stdcall function in MSVC is exported with the
    // type IMPORT_NAME, and the exported function name includes the
    // the leading underscore. In MinGW on the other hand, a decorated
    // stdcall function still omits the underscore (IMPORT_NAME_NOPREFIX).
    // See the comment in isDecorated in COFFModuleDefinition.cpp for more
    // details.
    if ext_name.starts_with('_') && ext_name.contains('@') && !mingw {
        ImportNameType::Name
    } else if sym != ext_name {
        ImportNameType::NameUndecorate
    } else if machine == MachineTypes::I386 && sym.starts_with('_') {
        ImportNameType::NameNoprefix
    } else {
        ImportNameType::Name
    }
}

fn replace(s: &str, mut from: &str, mut to: &str) -> Result<String> {
    if let Some((before, after)) = s.split_once(from) {
        return Ok(format!("{before}{to}{after}"));
    }

    // From and To may be mangled, but substrings in S may not.
    if from.starts_with('_') && to.starts_with('_') {
        from = &from[1..];
        to = &to[1..];
        if let Some((before, after)) = s.split_once(from) {
            return Ok(format!("{before}{to}{after}"));
        }
    }

    Err(Error::other(format!(
        "{s}: replacing '{from}' with '{to}' failed"
    )))
}

/// This class constructs various small object files necessary to support linking
/// symbols imported from a DLL.  The contents are pretty strictly defined and
/// nearly entirely static.  The details of the structures files are defined in
/// WINNT.h and the PE/COFF specification.
struct ObjectFactory<'a> {
    native_machine: MachineTypes,
    import_name: &'a str,
    import_descriptor_symbol_name: Vec<u8>,
    null_thunk_symbol_name: Vec<u8>,
    null_import_descriptor_symbol_name: Vec<u8>,
}

impl<'a> ObjectFactory<'a> {
    fn new(s: &'a str, m: MachineTypes, whole_archive_compat: bool) -> Result<Self> {
        let import_as_path = PathBuf::from(s);
        let library = import_as_path
            .file_stem()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidInput,
                    "Import name did not end with a file name",
                )
            })?
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Import name is not valid UTF-8"))?;
        let library = library.as_bytes();
        Ok(Self {
            native_machine: m,
            import_name: s,
            import_descriptor_symbol_name: IMPORT_DESCRIPTOR_PREFIX
                .iter()
                .chain(library)
                .copied()
                .collect(),
            null_thunk_symbol_name: NULL_THUNK_DATA_PREFIX
                .iter()
                .chain(library)
                .chain(NULL_THUNK_DATA_SUFFIX)
                .copied()
                .collect(),
            null_import_descriptor_symbol_name: if whole_archive_compat {
                NULL_IMPORT_DESCRIPTOR_SYMBOL_NAME_PREFIX
                    .iter()
                    .chain(library)
                    .copied()
                    .collect()
            } else {
                NULL_IMPORT_DESCRIPTOR_SYMBOL_NAME.into()
            },
        })
    }

    fn is_64_bit(&self) -> bool {
        crate::coff::is_64_bit(self.native_machine)
    }

    /// Creates an Import Descriptor.  This is a small object file which contains a
    /// reference to the terminators and contains the library name (entry) for the
    /// import name table.  It will force the linker to construct the necessary
    /// structure to import symbols from the DLL.
    fn create_import_descriptor(&self) -> Result<NewArchiveMember<'_>> {
        let mut buffer = Vec::new();

        const NUMBER_OF_SECTIONS: usize = 2;
        const NUMBER_OF_SYMBOLS: usize = 7;
        const NUMBER_OF_RELOCATIONS: usize = 3;

        // COFF Header
        let header = ImageFileHeader {
            machine: u16!(self.native_machine.into()),
            number_of_sections: u16!(NUMBER_OF_SECTIONS.try_into().unwrap()),
            time_date_stamp: u32!(0),
            pointer_to_symbol_table: u32!((size_of::<ImageFileHeader>() + (NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>()) +
                // .idata$2
                size_of::<ImageImportDescriptor>() +
                NUMBER_OF_RELOCATIONS * size_of::<ImageRelocation>() +
                // .idata$4
                (self.import_name.len() + 1)).try_into().unwrap()),
            number_of_symbols: u32!(NUMBER_OF_SYMBOLS.try_into().unwrap()),
            size_of_optional_header: u16!(0),
            characteristics: u16!(if self.is_64_bit() {
                0
            } else {
                IMAGE_FILE_32BIT_MACHINE
            }),
        };
        buffer.write_all(bytes_of(&header))?;

        // Section Header Table
        let section_table: [_; NUMBER_OF_SECTIONS] = [
            ImageSectionHeader {
                name: *b".idata$2",
                virtual_size: u32!(0),
                virtual_address: u32!(0),
                size_of_raw_data: u32!(size_of::<ImageImportDescriptor>().try_into().unwrap()),
                pointer_to_raw_data: u32!(
                    (size_of::<ImageFileHeader>()
                        + NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>())
                    .try_into()
                    .unwrap()
                ),
                pointer_to_relocations: u32!(
                    (size_of::<ImageFileHeader>()
                        + NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>()
                        + size_of::<ImageImportDescriptor>())
                    .try_into()
                    .unwrap()
                ),
                pointer_to_linenumbers: u32!(0),
                number_of_relocations: u16!(NUMBER_OF_RELOCATIONS.try_into().unwrap()),
                number_of_linenumbers: u16!(0),
                characteristics: u32!(
                    IMAGE_SCN_ALIGN_4BYTES
                        | IMAGE_SCN_CNT_INITIALIZED_DATA
                        | IMAGE_SCN_MEM_READ
                        | IMAGE_SCN_MEM_WRITE
                ),
            },
            ImageSectionHeader {
                name: *b".idata$6",
                virtual_size: u32!(0),
                virtual_address: u32!(0),
                size_of_raw_data: u32!((self.import_name.len() + 1).try_into().unwrap()),
                pointer_to_raw_data: u32!(
                    (size_of::<ImageFileHeader>()
                        + NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>()
                        + size_of::<ImageImportDescriptor>()
                        + NUMBER_OF_RELOCATIONS * size_of::<ImageRelocation>())
                    .try_into()
                    .unwrap()
                ),
                pointer_to_relocations: u32!(0),
                pointer_to_linenumbers: u32!(0),
                number_of_relocations: u16!(0),
                number_of_linenumbers: u16!(0),
                characteristics: u32!(
                    IMAGE_SCN_ALIGN_2BYTES
                        | IMAGE_SCN_CNT_INITIALIZED_DATA
                        | IMAGE_SCN_MEM_READ
                        | IMAGE_SCN_MEM_WRITE
                ),
            },
        ];
        buffer.write_all(bytes_of(&section_table))?;

        // .idata$2
        let import_descriptor = ImageImportDescriptor {
            original_first_thunk: u32!(0),
            time_date_stamp: u32!(0),
            forwarder_chain: u32!(0),
            name: u32!(0),
            first_thunk: u32!(0),
        };
        buffer.write_all(bytes_of(&import_descriptor))?;

        let relocation_table: [_; NUMBER_OF_RELOCATIONS] = [
            ImageRelocation {
                virtual_address: u32!(
                    (offset_of!(ImageImportDescriptor, name))
                        .try_into()
                        .unwrap()
                ),
                symbol_table_index: u32!(2),
                typ: get_img_rel_relocation(self.native_machine),
            },
            ImageRelocation {
                virtual_address: u32!(
                    offset_of!(ImageImportDescriptor, original_first_thunk)
                        .try_into()
                        .unwrap()
                ),
                symbol_table_index: u32!(3),
                typ: get_img_rel_relocation(self.native_machine),
            },
            ImageRelocation {
                virtual_address: u32!(
                    offset_of!(ImageImportDescriptor, first_thunk)
                        .try_into()
                        .unwrap()
                ),
                symbol_table_index: u32!(4),
                typ: get_img_rel_relocation(self.native_machine),
            },
        ];
        buffer.write_all(bytes_of(&relocation_table))?;

        // .idata$6
        buffer.write_all(self.import_name.as_bytes())?;
        buffer.write_all(&[0])?;

        // Symbol Table
        let mut symbol_table: [_; NUMBER_OF_SYMBOLS] = [
            ImageSymbol {
                name: [0; 8],
                value: u32!(0),
                section_number: u16!(1),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                number_of_aux_symbols: 0,
            },
            ImageSymbol {
                name: *b".idata$2",
                value: u32!(0),
                section_number: u16!(1),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_SECTION,
                number_of_aux_symbols: 0,
            },
            ImageSymbol {
                name: *b".idata$6",
                value: u32!(0),
                section_number: u16!(2),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_STATIC,
                number_of_aux_symbols: 0,
            },
            ImageSymbol {
                name: *b".idata$4",
                value: u32!(0),
                section_number: u16!(0),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_SECTION,
                number_of_aux_symbols: 0,
            },
            ImageSymbol {
                name: *b".idata$5",
                value: u32!(0),
                section_number: u16!(0),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_SECTION,
                number_of_aux_symbols: 0,
            },
            ImageSymbol {
                name: [0; 8],
                value: u32!(0),
                section_number: u16!(0),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                number_of_aux_symbols: 0,
            },
            ImageSymbol {
                name: [0; 8],
                value: u32!(0),
                section_number: u16!(0),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                number_of_aux_symbols: 0,
            },
        ];
        // TODO: Name.Offset.Offset here and in the all similar places below
        // suggests a names refactoring. Maybe StringTableOffset.Value?
        set_name_to_string_table_entry(&mut symbol_table[0], size_of::<u32>());
        set_name_to_string_table_entry(
            &mut symbol_table[5],
            size_of::<u32>() + self.import_descriptor_symbol_name.len() + 1,
        );
        set_name_to_string_table_entry(
            &mut symbol_table[6],
            size_of::<u32>()
                + self.import_descriptor_symbol_name.len()
                + 1
                + self.null_import_descriptor_symbol_name.len()
                + 1,
        );
        buffer.write_all(bytes_of(&symbol_table))?;

        // String Table
        write_string_table(
            &mut buffer,
            &[
                &self.import_descriptor_symbol_name,
                &self.null_import_descriptor_symbol_name,
                &self.null_thunk_symbol_name,
            ],
        )?;

        Ok(NewArchiveMember::new(
            buffer.into_boxed_slice(),
            &DEFAULT_OBJECT_READER,
            self.import_name.to_string(),
        ))
    }

    /// Creates a NULL import descriptor.  This is a small object file whcih
    /// contains a NULL import descriptor.  It is used to terminate the imports
    /// from a specific DLL.
    fn create_null_import_descriptor(&self) -> Result<NewArchiveMember<'_>> {
        let mut buffer = Vec::new();

        const NUMBER_OF_SECTIONS: usize = 1;
        const NUMBER_OF_SYMBOLS: usize = 1;

        // COFF Header
        let header = ImageFileHeader {
            machine: u16!(self.native_machine.into()),
            number_of_sections: u16!(NUMBER_OF_SECTIONS.try_into().unwrap()),
            time_date_stamp: u32!(0),
            pointer_to_symbol_table: u32!((size_of::<ImageFileHeader>() + (NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>()) +
                // .idata$3
                size_of::<ImageImportDescriptor>()).try_into().unwrap()),
            number_of_symbols: u32!(NUMBER_OF_SYMBOLS.try_into().unwrap()),
            size_of_optional_header: u16!(0),
            characteristics: u16!(if self.is_64_bit() {
                0
            } else {
                IMAGE_FILE_32BIT_MACHINE
            }),
        };
        buffer.write_all(bytes_of(&header))?;

        // Section Header Table
        let section_table: [_; NUMBER_OF_SECTIONS] = [ImageSectionHeader {
            name: *b".idata$3",
            virtual_size: u32!(0),
            virtual_address: u32!(0),
            size_of_raw_data: u32!(size_of::<ImageImportDescriptor>().try_into().unwrap()),
            pointer_to_raw_data: u32!(
                (size_of::<ImageFileHeader>()
                    + NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>())
                .try_into()
                .unwrap()
            ),
            pointer_to_relocations: u32!(0),
            pointer_to_linenumbers: u32!(0),
            number_of_relocations: u16!(0),
            number_of_linenumbers: u16!(0),
            characteristics: u32!(
                IMAGE_SCN_ALIGN_4BYTES
                    | IMAGE_SCN_CNT_INITIALIZED_DATA
                    | IMAGE_SCN_MEM_READ
                    | IMAGE_SCN_MEM_WRITE
            ),
        }];
        buffer.write_all(bytes_of(&section_table))?;

        // .idata$3
        let import_descriptor = ImageImportDescriptor {
            original_first_thunk: u32!(0),
            time_date_stamp: u32!(0),
            forwarder_chain: u32!(0),
            name: u32!(0),
            first_thunk: u32!(0),
        };
        buffer.write_all(bytes_of(&import_descriptor))?;

        // Symbol Table
        let mut symbol_table: [_; NUMBER_OF_SYMBOLS] = [ImageSymbol {
            name: [0; 8],
            value: u32!(0),
            section_number: u16!(1),
            typ: u16!(0),
            storage_class: IMAGE_SYM_CLASS_EXTERNAL,
            number_of_aux_symbols: 0,
        }];
        set_name_to_string_table_entry(&mut symbol_table[0], size_of::<u32>());
        buffer.write_all(bytes_of(&symbol_table))?;

        // String Table
        write_string_table(&mut buffer, &[&self.null_import_descriptor_symbol_name])?;

        Ok(NewArchiveMember::new(
            buffer.into_boxed_slice(),
            &DEFAULT_OBJECT_READER,
            self.import_name.to_string(),
        ))
    }

    /// Create a NULL Thunk Entry.  This is a small object file which contains a
    /// NULL Import Address Table entry and a NULL Import Lookup Table Entry.  It
    /// is used to terminate the IAT and ILT.
    fn create_null_thunk(&self) -> Result<NewArchiveMember<'_>> {
        let mut buffer = Vec::new();

        const NUMBER_OF_SECTIONS: usize = 2;
        const NUMBER_OF_SYMBOLS: usize = 1;
        let va_size = if self.is_64_bit() { 8 } else { 4 };

        // COFF Header
        let header = ImageFileHeader {
            machine: u16!(self.native_machine.into()),
            number_of_sections: u16!(NUMBER_OF_SECTIONS.try_into().unwrap()),
            time_date_stamp: u32!(0),
            pointer_to_symbol_table: u32!((size_of::<ImageFileHeader>() + (NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>()) +
                // .idata$5
                va_size +
                // .idata$4
                va_size).try_into().unwrap()),
            number_of_symbols: u32!(NUMBER_OF_SYMBOLS.try_into().unwrap()),
            size_of_optional_header: u16!(0),
            characteristics: u16!(if self.is_64_bit() {
                0
            } else {
                IMAGE_FILE_32BIT_MACHINE
            }),
        };
        buffer.write_all(bytes_of(&header))?;

        // Section Header Table
        let alignment = if self.is_64_bit() {
            IMAGE_SCN_ALIGN_8BYTES
        } else {
            IMAGE_SCN_ALIGN_4BYTES
        };
        let section_table: [_; NUMBER_OF_SECTIONS] = [
            ImageSectionHeader {
                name: *b".idata$5",
                virtual_size: u32!(0),
                virtual_address: u32!(0),
                size_of_raw_data: u32!(va_size.try_into().unwrap()),
                pointer_to_raw_data: u32!(
                    (size_of::<ImageFileHeader>()
                        + NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>())
                    .try_into()
                    .unwrap()
                ),
                pointer_to_relocations: u32!(0),
                pointer_to_linenumbers: u32!(0),
                number_of_relocations: u16!(0),
                number_of_linenumbers: u16!(0),
                characteristics: u32!(
                    alignment
                        | IMAGE_SCN_CNT_INITIALIZED_DATA
                        | IMAGE_SCN_MEM_READ
                        | IMAGE_SCN_MEM_WRITE
                ),
            },
            ImageSectionHeader {
                name: *b".idata$4",
                virtual_size: u32!(0),
                virtual_address: u32!(0),
                size_of_raw_data: u32!(va_size.try_into().unwrap()),
                pointer_to_raw_data: u32!(
                    (size_of::<ImageFileHeader>()
                        + NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>()
                        + va_size)
                        .try_into()
                        .unwrap()
                ),
                pointer_to_relocations: u32!(0),
                pointer_to_linenumbers: u32!(0),
                number_of_relocations: u16!(0),
                number_of_linenumbers: u16!(0),
                characteristics: u32!(
                    alignment
                        | IMAGE_SCN_CNT_INITIALIZED_DATA
                        | IMAGE_SCN_MEM_READ
                        | IMAGE_SCN_MEM_WRITE
                ),
            },
        ];
        buffer.write_all(bytes_of(&section_table))?;

        // .idata$5, ILT
        buffer.write_all(&vec![0; va_size])?;

        // .idata$4, IAT
        buffer.write_all(&vec![0; va_size])?;

        // Symbol Table
        let mut symbol_table: [_; NUMBER_OF_SYMBOLS] = [ImageSymbol {
            name: [0; 8],
            value: u32!(0),
            section_number: u16!(1),
            typ: u16!(0),
            storage_class: IMAGE_SYM_CLASS_EXTERNAL,
            number_of_aux_symbols: 0,
        }];
        set_name_to_string_table_entry(&mut symbol_table[0], size_of::<u32>());
        buffer.write_all(bytes_of(&symbol_table))?;

        // String Table
        write_string_table(&mut buffer, &[&self.null_thunk_symbol_name])?;

        Ok(NewArchiveMember::new(
            buffer.into_boxed_slice(),
            &DEFAULT_OBJECT_READER,
            self.import_name.to_string(),
        ))
    }

    /// Create a short import file which is described in PE/COFF spec 7. Import
    /// Library Format.
    fn create_short_import(
        &self,
        sym: &str,
        ordinal: u16,
        import_type: ImportType,
        name_type: ImportNameType,
        export_name: Option<&str>,
        machine: MachineTypes,
    ) -> Result<NewArchiveMember<'_>> {
        let mut imp_size = self.import_name.len() + sym.len() + 2; // +2 for NULs
        if let Some(export_name) = export_name {
            imp_size += export_name.len() + 1;
        }
        let size = size_of::<ImportObjectHeader>() + imp_size;
        let mut buf = Vec::new();
        buf.reserve_exact(size);

        // Write short import library.
        let imp = ImportObjectHeader {
            sig1: u16!(0),
            sig2: u16!(0xFFFF),
            version: u16!(0),
            machine: u16!(machine.into()),
            time_date_stamp: u32!(0),
            size_of_data: u32!(imp_size.try_into().unwrap()),
            ordinal_or_hint: u16!(ordinal),
            name_type: u16!((u16::from(name_type) << 2) | u16::from(import_type)),
        };
        buf.write_all(bytes_of(&imp))?;

        // Write symbol name and DLL name.
        buf.write_all(sym.as_bytes())?;
        buf.write_all(&[0])?;
        buf.write_all(self.import_name.as_bytes())?;
        buf.write_all(&[0])?;
        if let Some(export_name) = export_name {
            buf.write_all(export_name.as_bytes())?;
            buf.write_all(&[0])?;
        }

        Ok(NewArchiveMember::new(
            buf.into_boxed_slice(),
            &READER_FOR_SHORT_IMPORT,
            self.import_name.to_string(),
        ))
    }

    /// Create a weak external file which is described in PE/COFF Aux Format 3.
    fn create_weak_external(
        &self,
        sym: &str,
        weak: &str,
        imp: bool,
        machine: MachineTypes,
    ) -> Result<NewArchiveMember<'_>> {
        let mut buffer = Vec::new();
        const NUMBER_OF_SECTIONS: usize = 1;
        const NUMBER_OF_SYMBOLS: usize = 5;

        // COFF Header
        let header = ImageFileHeader {
            machine: u16!(machine.into()),
            number_of_sections: u16!(NUMBER_OF_SECTIONS.try_into().unwrap()),
            time_date_stamp: u32!(0),
            pointer_to_symbol_table: u32!(
                (size_of::<ImageFileHeader>()
                    + (NUMBER_OF_SECTIONS * size_of::<ImageSectionHeader>()))
                .try_into()
                .unwrap()
            ),
            number_of_symbols: u32!(NUMBER_OF_SYMBOLS.try_into().unwrap()),
            size_of_optional_header: u16!(0),
            characteristics: u16!(0),
        };
        buffer.write_all(bytes_of(&header))?;

        // Section Header Table
        let section_table: [_; NUMBER_OF_SECTIONS] = [ImageSectionHeader {
            name: *b".drectve",
            virtual_size: u32!(0),
            virtual_address: u32!(0),
            size_of_raw_data: u32!(0),
            pointer_to_raw_data: u32!(0),
            pointer_to_relocations: u32!(0),
            pointer_to_linenumbers: u32!(0),
            number_of_relocations: u16!(0),
            number_of_linenumbers: u16!(0),
            characteristics: u32!(IMAGE_SCN_LNK_INFO | IMAGE_SCN_LNK_REMOVE),
        }];
        buffer.write_all(bytes_of(&section_table))?;

        // Symbol Table
        let mut symbol_table: [_; NUMBER_OF_SYMBOLS] = [
            ImageSymbol {
                name: *b"@comp.id",
                value: u32!(0),
                section_number: u16!(0xFFFF),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_STATIC,
                number_of_aux_symbols: 0,
            },
            ImageSymbol {
                name: *b"@feat.00",
                value: u32!(0),
                section_number: u16!(0xFFFF),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_STATIC,
                number_of_aux_symbols: 0,
            },
            ImageSymbol {
                name: [0; 8],
                value: u32!(0),
                section_number: u16!(0),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                number_of_aux_symbols: 0,
            },
            ImageSymbol {
                name: [0; 8],
                value: u32!(0),
                section_number: u16!(0),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_WEAK_EXTERNAL,
                number_of_aux_symbols: 1,
            },
            ImageSymbol {
                name: [2, 0, 0, 0, IMAGE_WEAK_EXTERN_SEARCH_ALIAS as u8, 0, 0, 0],
                value: u32!(0),
                section_number: u16!(0),
                typ: u16!(0),
                storage_class: IMAGE_SYM_CLASS_NULL,
                number_of_aux_symbols: 0,
            },
        ];
        set_name_to_string_table_entry(&mut symbol_table[2], size_of::<u32>());

        //__imp_ String Table
        let prefix: &[u8] = if imp { b"__imp_" } else { b"" };
        set_name_to_string_table_entry(
            &mut symbol_table[3],
            size_of::<u32>() + sym.len() + prefix.len() + 1,
        );
        buffer.write_all(bytes_of(&symbol_table))?;

        write_string_table(
            &mut buffer,
            &[
                &prefix
                    .iter()
                    .chain(sym.as_bytes())
                    .copied()
                    .collect::<Vec<_>>(),
                &prefix
                    .iter()
                    .chain(weak.as_bytes())
                    .copied()
                    .collect::<Vec<_>>(),
            ],
        )?;

        // Copied here so we can still use writeStringTable
        Ok(NewArchiveMember::new(
            buffer.into_boxed_slice(),
            &DEFAULT_OBJECT_READER,
            self.import_name.to_string(),
        ))
    }
}

/// Writes a COFF import library containing entries described by the Exports
/// array.
///
/// For hybrid targets such as ARM64EC, additional native entry points can be
/// exposed using the NativeExports parameter. When NativeExports is used, the
/// output import library will expose these native ARM64 imports alongside the
/// entries described in the Exports array. Such a library can be used for
/// linking both ARM64EC and pure ARM64 objects, and the linker will pick only
/// the exports relevant to the target platform. For non-hybrid targets,
/// the NativeExports parameter should not be used.
pub fn write_import_library<W: Write + Seek>(
    w: &mut W,
    import_name: &str,
    exports: &[COFFShortExport],
    mut machine: MachineTypes,
    mingw: bool,
    whole_archive_compat: bool,
    native_exports: &[COFFShortExport],
) -> Result<()> {
    let native_machine = if is_arm64ec(machine) {
        machine = MachineTypes::ARM64EC;
        MachineTypes::ARM64
    } else {
        machine
    };

    let of = ObjectFactory::new(import_name, native_machine, whole_archive_compat)?;
    let mut members = Vec::new();

    members.push(of.create_import_descriptor()?);

    members.push(of.create_null_import_descriptor()?);

    members.push(of.create_null_thunk()?);

    let mut add_exports = |exports: &[COFFShortExport], machine_type: MachineTypes| -> Result<()> {
        struct Deferred<'a> {
            name: String,
            import_type: ImportType,
            export: &'a COFFShortExport,
        }
        // Determinism: Safe to use HashMap here as we never iterate over it.
        let mut regular_imports = HashMap::new();
        let mut renames = Vec::new();
        for e in exports {
            if e.private {
                continue;
            }

            let mut import_type = ImportType::Code;
            if e.data {
                import_type = ImportType::Data;
            }
            if e.constant {
                import_type = ImportType::Const;
            }

            let symbol_name = if let Some(symbol_name) = e.symbol_name.as_ref() {
                symbol_name
            } else {
                &e.name
            };

            let mut name: Cow<'_, str> = if let Some(ext_name) = e.ext_name.as_ref() {
                Cow::Owned(replace(symbol_name, &e.name, ext_name)?)
            } else {
                Cow::Borrowed(symbol_name)
            };

            let mut export_name = None;
            let mut name_type = if e.noname {
                ImportNameType::Ordinal
            } else if let Some(export_as) = e.export_as.as_deref() {
                export_name = Some(Cow::Borrowed(export_as));
                ImportNameType::NameExportas
            } else if let Some(import_name) = e.import_name.as_deref() {
                // If we need to import from a specific ImportName, we may need to use
                // a weak alias (which needs another import to point at). But if we can
                // express ImportName based on the symbol name and a specific NameType,
                // prefer that over an alias.
                if machine_type == MachineTypes::I386
                    && apply_name_type(ImportNameType::NameUndecorate, &name) == import_name
                {
                    ImportNameType::NameUndecorate
                } else if machine_type == MachineTypes::I386
                    && apply_name_type(ImportNameType::NameNoprefix, &name) == import_name
                {
                    ImportNameType::NameNoprefix
                } else if is_arm64ec(machine_type) {
                    export_name = Some(Cow::Borrowed(import_name));
                    ImportNameType::NameExportas
                } else if name == import_name {
                    ImportNameType::Name
                } else {
                    renames.push(Deferred {
                        name: name.into_owned(),
                        import_type,
                        export: e,
                    });
                    continue;
                }
            } else {
                get_name_type(symbol_name, &e.name, machine_type, mingw)
            };

            // On ARM64EC, use EXPORTAS to import demangled name for mangled symbols.
            if import_type == ImportType::Code && crate::coff::is_arm64ec(machine_type) {
                match get_arm64ec_mangled_function_name(&name) {
                    Ok(Some(mangled_name)) => {
                        if !e.noname && export_name.is_none() {
                            name_type = ImportNameType::NameExportas;
                            export_name = Some(name);
                        }
                        name = Cow::Owned(mangled_name);
                    }
                    Err(_) if !e.noname && export_name.is_none() => {
                        return Err(Error::other(format!(
                            "Functions on Arm64EC must use the Arm64EC mangling scheme, but the function \"{name}\" does not. Either use the Arm64EC mangled name or set the `export_name`."
                        )));
                    }
                    Ok(None) if !e.noname && export_name.is_none() => {
                        let demangled_name = get_arm64ec_demangled_function_name(&name)
                            .ok_or_else(|| {
                                Error::other(format!("Invalid ARM64EC function name \"{name}\""))
                            })?;
                        name_type = ImportNameType::NameExportas;
                        export_name = Some(Cow::Owned(demangled_name));
                    }
                    _ => {}
                }
            }

            let key = apply_name_type(name_type, &name).to_string();
            regular_imports.insert(key, name.to_string());
            members.push(of.create_short_import(
                name.as_ref(),
                e.ordinal,
                import_type,
                name_type,
                export_name.as_deref(),
                machine_type,
            )?);
        }
        for d in renames {
            if let Some(symbol) = d
                .export
                .import_name
                .as_ref()
                .and_then(|import_name| regular_imports.get(import_name.as_str()))
            {
                // We have a regular import entry for a symbol with the name we
                // want to reference; produce an alias pointing at that.
                if d.import_type == ImportType::Code {
                    members.push(of.create_weak_external(symbol, &d.name, false, machine_type)?);
                }
                members.push(of.create_weak_external(symbol, &d.name, true, machine_type)?);
            } else {
                members.push(of.create_short_import(
                    &d.name,
                    d.export.ordinal,
                    d.import_type,
                    ImportNameType::NameExportas,
                    d.export.import_name.as_deref(),
                    machine_type,
                )?);
            }
        }

        Ok(())
    };

    add_exports(exports, machine)?;
    add_exports(native_exports, native_machine)?;

    write_archive_to_stream(
        w,
        &members,
        ArchiveKind::Coff,
        false,
        Some(is_arm64ec(machine)),
    )
}
