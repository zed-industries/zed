//! A PE32 and PE32+ parser
//!

// TODO: panics with unwrap on None for apisetschema.dll, fhuxgraphics.dll and some others

use core::cmp::max;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use log::warn;

pub mod authenticode;
pub mod certificate_table;
pub mod characteristic;
pub mod data_directories;
pub mod debug;
pub mod dll_characteristic;
pub mod exception;
pub mod export;
pub mod header;
pub mod import;
pub mod optional_header;
pub mod options;
pub mod relocation;
pub mod section_table;
pub mod subsystem;
pub mod symbol;
pub mod tls;
pub mod utils;

use crate::container;
use crate::error;
use crate::pe::utils::pad;
use crate::strtab;

use scroll::{ctx, Pwrite};

use log::debug;

#[derive(Debug)]
/// An analyzed PE32/PE32+ binary
pub struct PE<'a> {
    /// Underlying bytes
    bytes: &'a [u8],
    authenticode_excluded_sections: Option<authenticode::ExcludedSections>,
    /// The PE header
    pub header: header::Header<'a>,
    /// A list of the sections in this PE binary
    pub sections: Vec<section_table::SectionTable>,
    /// The size of the binary
    pub size: usize,
    /// The name of this `dll`, if it has one
    pub name: Option<&'a str>,
    /// Whether this is a `dll` or not
    pub is_lib: bool,
    /// Whether the binary is 64-bit (PE32+)
    pub is_64: bool,
    /// the entry point of the binary
    pub entry: usize,
    /// The binary's RVA, or image base - useful for computing virtual addreses
    pub image_base: usize,
    /// Data about any exported symbols in this binary (e.g., if it's a `dll`)
    pub export_data: Option<export::ExportData<'a>>,
    /// Data for any imported symbols, and from which `dll`, etc., in this binary
    pub import_data: Option<import::ImportData<'a>>,
    /// The list of exported symbols in this binary, contains synthetic information for easier analysis
    pub exports: Vec<export::Export<'a>>,
    /// The list symbols imported by this binary from other `dll`s
    pub imports: Vec<import::Import<'a>>,
    /// The list of libraries which this binary imports symbols from
    pub libraries: Vec<&'a str>,
    /// Debug information, if any, contained in the PE header
    pub debug_data: Option<debug::DebugData<'a>>,
    /// TLS information, if any, contained in the PE header
    pub tls_data: Option<tls::TlsData<'a>>,
    /// Exception handling and stack unwind information, if any, contained in the PE header
    pub exception_data: Option<exception::ExceptionData<'a>>,
    /// Certificates present, if any, described by the Certificate Table
    pub certificates: certificate_table::CertificateDirectoryTable<'a>,
}

impl<'a> PE<'a> {
    /// Reads a PE binary from the underlying `bytes`
    pub fn parse(bytes: &'a [u8]) -> error::Result<Self> {
        Self::parse_with_opts(bytes, &options::ParseOptions::default())
    }

    /// Reads a PE binary from the underlying `bytes`
    pub fn parse_with_opts(bytes: &'a [u8], opts: &options::ParseOptions) -> error::Result<Self> {
        let header = header::Header::parse(bytes)?;
        let mut authenticode_excluded_sections = None;

        debug!("{:#?}", header);
        let optional_header_offset = header.dos_header.pe_pointer as usize
            + header::SIZEOF_PE_MAGIC
            + header::SIZEOF_COFF_HEADER;
        let offset =
            &mut (optional_header_offset + header.coff_header.size_of_optional_header as usize);

        let sections = header.coff_header.sections(bytes, offset)?;
        let is_lib = characteristic::is_dll(header.coff_header.characteristics);
        let mut entry = 0;
        let mut image_base = 0;
        let mut exports = vec![];
        let mut export_data = None;
        let mut name = None;
        let mut imports = vec![];
        let mut import_data = None;
        let mut libraries = vec![];
        let mut debug_data = None;
        let mut tls_data = None;
        let mut exception_data = None;
        let mut certificates = Default::default();
        let mut is_64 = false;
        if let Some(optional_header) = header.optional_header {
            // Sections we are assembling through the parsing, eventually, it will be passed
            // to the authenticode_sections attribute of `PE`.
            let (checksum, datadir_entry_certtable) = match optional_header.standard_fields.magic {
                optional_header::MAGIC_32 => {
                    let standard_field_offset =
                        optional_header_offset + optional_header::SIZEOF_STANDARD_FIELDS_32;
                    let checksum_field_offset =
                        standard_field_offset + optional_header::OFFSET_WINDOWS_FIELDS_32_CHECKSUM;
                    (
                        checksum_field_offset..checksum_field_offset + 4,
                        optional_header_offset + 128..optional_header_offset + 136,
                    )
                }
                optional_header::MAGIC_64 => {
                    let standard_field_offset =
                        optional_header_offset + optional_header::SIZEOF_STANDARD_FIELDS_64;
                    let checksum_field_offset =
                        standard_field_offset + optional_header::OFFSET_WINDOWS_FIELDS_64_CHECKSUM;

                    (
                        checksum_field_offset..checksum_field_offset + 4,
                        optional_header_offset + 144..optional_header_offset + 152,
                    )
                }
                magic => {
                    return Err(error::Error::Malformed(format!(
                        "Unsupported header magic ({:#x})",
                        magic
                    )))
                }
            };

            entry = optional_header.standard_fields.address_of_entry_point as usize;
            image_base = optional_header.windows_fields.image_base as usize;
            is_64 = optional_header.container()? == container::Container::Big;
            debug!(
                "entry {:#x} image_base {:#x} is_64: {}",
                entry, image_base, is_64
            );
            let file_alignment = optional_header.windows_fields.file_alignment;
            if let Some(&export_table) = optional_header.data_directories.get_export_table() {
                if let Ok(ed) = export::ExportData::parse_with_opts(
                    bytes,
                    export_table,
                    &sections,
                    file_alignment,
                    opts,
                ) {
                    debug!("export data {:#?}", ed);
                    exports = export::Export::parse_with_opts(
                        bytes,
                        &ed,
                        &sections,
                        file_alignment,
                        opts,
                    )?;
                    name = ed.name;
                    debug!("name: {:#?}", name);
                    export_data = Some(ed);
                }
            }
            debug!("exports: {:#?}", exports);
            if let Some(&import_table) = optional_header.data_directories.get_import_table() {
                let id = if is_64 {
                    import::ImportData::parse_with_opts::<u64>(
                        bytes,
                        import_table,
                        &sections,
                        file_alignment,
                        opts,
                    )?
                } else {
                    import::ImportData::parse_with_opts::<u32>(
                        bytes,
                        import_table,
                        &sections,
                        file_alignment,
                        opts,
                    )?
                };
                debug!("import data {:#?}", id);
                if is_64 {
                    imports = import::Import::parse::<u64>(bytes, &id, &sections)?
                } else {
                    imports = import::Import::parse::<u32>(bytes, &id, &sections)?
                }
                libraries = id
                    .import_data
                    .iter()
                    .map(|data| data.name)
                    .collect::<Vec<&'a str>>();
                libraries.sort();
                libraries.dedup();
                import_data = Some(id);
            }
            debug!("imports: {:#?}", imports);
            if let Some(&debug_table) = optional_header.data_directories.get_debug_table() {
                debug_data = Some(debug::DebugData::parse_with_opts(
                    bytes,
                    debug_table,
                    &sections,
                    file_alignment,
                    opts,
                )?);
            }

            if let Some(tls_table) = optional_header.data_directories.get_tls_table() {
                tls_data = if is_64 {
                    tls::TlsData::parse_with_opts::<u64>(
                        bytes,
                        image_base,
                        tls_table,
                        &sections,
                        file_alignment,
                        opts,
                    )?
                } else {
                    tls::TlsData::parse_with_opts::<u32>(
                        bytes,
                        image_base,
                        &tls_table,
                        &sections,
                        file_alignment,
                        opts,
                    )?
                };
                debug!("tls data: {:#?}", tls_data);
            }

            if header.coff_header.machine == header::COFF_MACHINE_X86_64 {
                // currently only x86_64 is supported
                debug!("exception data: {:#?}", exception_data);
                if let Some(&exception_table) =
                    optional_header.data_directories.get_exception_table()
                {
                    exception_data = Some(exception::ExceptionData::parse_with_opts(
                        bytes,
                        exception_table,
                        &sections,
                        file_alignment,
                        opts,
                    )?);
                }
            }

            // Parse attribute certificates unless opted out of
            let certificate_table_size = if opts.parse_attribute_certificates {
                if let Some(&certificate_table) =
                    optional_header.data_directories.get_certificate_table()
                {
                    certificates = certificate_table::enumerate_certificates(
                        bytes,
                        certificate_table.virtual_address,
                        certificate_table.size,
                    )?;

                    certificate_table.size as usize
                } else {
                    0
                }
            } else {
                0
            };

            authenticode_excluded_sections = Some(authenticode::ExcludedSections::new(
                checksum,
                datadir_entry_certtable,
                certificate_table_size,
                optional_header.windows_fields.size_of_headers as usize,
            ));
        }
        Ok(PE {
            bytes,
            authenticode_excluded_sections,
            header,
            sections,
            size: 0,
            name,
            is_lib,
            is_64,
            entry,
            image_base,
            export_data,
            import_data,
            exports,
            imports,
            libraries,
            debug_data,
            tls_data,
            exception_data,
            certificates,
        })
    }

    pub fn write_sections(
        &self,
        bytes: &mut [u8],
        offset: &mut usize,
        file_alignment: Option<usize>,
        ctx: scroll::Endian,
    ) -> Result<usize, error::Error> {
        // sections table and data
        debug_assert!(
            self.sections
                .iter()
                .flat_map(|section_a| {
                    self.sections
                        .iter()
                        .map(move |section_b| (section_a, section_b))
                })
                // given sections = (s_1, …, s_n)
                // for all (s_i, s_j), i != j, verify that s_i does not overlap with s_j and vice versa.
                .all(|(section_i, section_j)| section_i == section_j
                    || !section_i.overlaps_with(section_j)),
            "Overlapping sections were found, this is not supported."
        );

        for section in &self.sections {
            let section_data = section.data(&self.bytes)?.ok_or_else(|| {
                error::Error::Malformed(format!(
                    "Section data `{}` is malformed",
                    section.name().unwrap_or("unknown name")
                ))
            })?;
            let file_section_offset =
                usize::try_from(section.pointer_to_raw_data).map_err(|_| {
                    error::Error::Malformed(format!(
                        "Section `{}`'s pointer to raw data does not fit in platform `usize`",
                        section.name().unwrap_or("unknown name")
                    ))
                })?;
            let vsize: usize = section.virtual_size.try_into()?;
            let ondisk_size: usize = section.size_of_raw_data.try_into()?;
            let section_name = String::from(section.name().unwrap_or("unknown name"));

            let mut file_offset = file_section_offset;
            // `file_section_offset` is a on-disk offset which can be anywhere in the file.
            // Write section data first to avoid the final consumption.
            match section_data {
                Cow::Borrowed(borrowed) => bytes.gwrite(borrowed, &mut file_offset)?,
                Cow::Owned(owned) => bytes.gwrite(owned.as_slice(), &mut file_offset)?,
            };

            // Section tables follows the header.
            bytes.gwrite_with(section, offset, ctx)?;

            // for size size_of_raw_data
            // if < virtual_size, pad with 0
            // Pad with zeros if necessary
            if file_offset < vsize {
                bytes.gwrite(vec![0u8; vsize - file_offset].as_slice(), &mut file_offset)?;
            }

            // Align on a boundary as per file alignement field.
            if let Some(pad) = pad(file_offset - file_section_offset, file_alignment) {
                debug!(
                    "aligning `{}` {:#x} -> {:#x} bytes'",
                    section_name,
                    file_offset - file_section_offset,
                    file_offset - file_section_offset + pad.len()
                );
                bytes.gwrite(pad.as_slice(), &mut file_offset)?;
            }

            let written_data_size = file_offset - file_section_offset;
            if ondisk_size != written_data_size {
                warn!("Original PE is inefficient or bug (on-disk data size in PE: {:#x}), we wrote {:#x} bytes",
                    ondisk_size,
                    written_data_size);
            }
        }

        Ok(*offset)
    }

    pub fn write_certificates(
        &self,
        bytes: &mut [u8],
        ctx: scroll::Endian,
    ) -> Result<usize, error::Error> {
        let opt_header = self
            .header
            .optional_header
            .ok_or(error::Error::Malformed(format!(
                "This PE binary has no optional header; it is required to write certificates"
            )))?;
        let mut max_offset = 0;

        if let Some(certificate_directory) = opt_header.data_directories.get_certificate_table() {
            let mut certificate_start = certificate_directory.virtual_address.try_into()?;
            for certificate in &self.certificates {
                bytes.gwrite_with(certificate, &mut certificate_start, ctx)?;
                max_offset = max(certificate_start, max_offset);
            }
        }

        Ok(max_offset)
    }
}

impl<'a> ctx::TryIntoCtx<scroll::Endian> for PE<'a> {
    type Error = error::Error;

    fn try_into_ctx(self, bytes: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        let mut offset = 0;
        // We need to maintain a `max_offset` because
        // we could be writing sections in the wrong order (i.e. not an increasing order for the
        // pointer on raw disk)
        // and there could be holes between sections.
        // If we don't re-layout sections, we cannot fix that ourselves.
        // Same can be said about the certificate table, there could be a hole between sections
        // and the certificate data.
        // To avoid those troubles, we maintain the max over all offsets we see so far.
        let mut max_offset = 0;
        let file_alignment: Option<usize> = match self.header.optional_header {
            Some(opt_header) => {
                debug_assert!(
                    opt_header.windows_fields.file_alignment.count_ones() == 1,
                    "file alignment should be a power of 2"
                );
                Some(opt_header.windows_fields.file_alignment.try_into()?)
            }
            _ => None,
        };

        bytes.gwrite_with(self.header, &mut offset, ctx)?;
        max_offset = max(offset, max_offset);
        self.write_sections(bytes, &mut offset, file_alignment, ctx)?;
        // We want the section offset for which we have the highest pointer on disk.
        // The next offset is reserved for debug tables (outside of sections) and/or certificate
        // tables.
        max_offset = max(
            self.sections
                .iter()
                .max_by_key(|section| section.pointer_to_raw_data as usize)
                .map(|section| (section.pointer_to_raw_data + section.size_of_raw_data) as usize)
                .unwrap_or(offset),
            max_offset,
        );

        // COFF Symbol Table
        // Auxiliary Symbol Records
        // COFF String Table
        assert!(
            self.header.coff_header.pointer_to_symbol_table == 0,
            "Symbol tables in PE are deprecated and not supported to write"
        );

        // The following data directories are
        // taken care inside a section:
        // - export table (.edata)
        // - import table (.idata)
        // - bound import table
        // - import address table
        // - delay import tables
        // - resource table (.rsrc)
        // - exception table (.pdata)
        // - base relocation table (.reloc)
        // - debug table (.debug) <- this one is special, it can be outside of a
        // section.
        // - load config table
        // - tls table (.tls)
        // - architecture (reserved, 0 for now)
        // - global ptr is a "empty" data directory (header-only)
        // - clr runtime header (.cormeta is object-only)
        //
        // Nonetheless, we need to write the attribute certificate table one.
        max_offset = max(max_offset, self.write_certificates(bytes, ctx)?);

        // TODO: we would like to support debug table outside of a section.
        // i.e. debug tables that are never mapped in memory
        // See https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#debug-directory-image-only
        // > The debug directory can be in a discardable .debug section (if one exists), or it can be included in any other section in the image file, or not be in a section at all.
        // In case it's not in a section at all, we need to find a way
        // to rewrite it again.
        // and we need to respect the ordering between attribute certificates
        // and debug table.

        Ok(max_offset)
    }
}

/// An analyzed TE binary
///
/// A TE binary is a PE/PE32+ binary that has had it's header stripped and
/// re-formatted to the TE specification. This presents a challenge for
/// parsing, as all relative addresses (RVAs) are not updated to take this into
/// account, and are thus incorrect. The parsing of a TE must take this into
/// account by using the [header::TeHeader::stripped_size`] field of the TE
/// header to adjust the RVAs during parsing.
#[cfg(feature = "te")]
#[derive(Debug)]
pub struct TE<'a> {
    /// The TE header
    pub header: header::TeHeader,
    /// A list of the sections in this TE binary
    pub sections: Vec<section_table::SectionTable>,
    /// Debug information, contained in the PE header
    pub debug_data: debug::DebugData<'a>,
    /// The offset to apply to addresses not parsed by the TE parser
    /// itself: [header::TeHeader::stripped_size] - size_of::<[header::TeHeader]>()
    pub rva_offset: usize,
}

#[cfg(feature = "te")]
impl<'a> TE<'a> {
    /// Reads a TE binary from the underlying `bytes`
    pub fn parse(bytes: &'a [u8]) -> error::Result<Self> {
        let opts = &options::ParseOptions {
            resolve_rva: false,
            parse_attribute_certificates: false,
        };

        let mut offset = 0;

        // Parse the TE header and adjust the offsets
        let header = header::TeHeader::parse(bytes, &mut offset)?;
        let rva_offset = header.stripped_size as usize - core::mem::size_of::<header::TeHeader>();

        // Parse the sections and adjust the offsets
        let sections = header.sections(bytes, &mut offset)?;

        // Parse the debug data. Must adjust offsets before parsing the image_debug_directory
        let mut debug_data = debug::DebugData::default();
        debug_data.image_debug_directory = debug::ImageDebugDirectory::parse_with_opts(
            bytes,
            header.debug_dir,
            &sections,
            0,
            opts,
        )?;
        TE::fixup_debug_data(&mut debug_data, rva_offset as u32);
        debug_data.codeview_pdb70_debug_info = debug::CodeviewPDB70DebugInfo::parse_with_opts(
            bytes,
            &debug_data.image_debug_directory,
            opts,
        )?;

        Ok(TE {
            header,
            sections,
            debug_data,
            rva_offset,
        })
    }

    /// Adjust all addresses in the TE binary debug data.
    fn fixup_debug_data(dd: &mut debug::DebugData, rva_offset: u32) {
        debug!(
            "ImageDebugDirectory address of raw data fixed up from: 0x{:X} to 0x{:X}",
            dd.image_debug_directory.address_of_raw_data,
            dd.image_debug_directory
                .address_of_raw_data
                .wrapping_sub(rva_offset),
        );
        dd.image_debug_directory.address_of_raw_data = dd
            .image_debug_directory
            .address_of_raw_data
            .wrapping_sub(rva_offset);

        debug!(
            "ImageDebugDirectory pointer to raw data fixed up from: 0x{:X} to 0x{:X}",
            dd.image_debug_directory.pointer_to_raw_data,
            dd.image_debug_directory
                .pointer_to_raw_data
                .wrapping_sub(rva_offset),
        );
        dd.image_debug_directory.pointer_to_raw_data = dd
            .image_debug_directory
            .pointer_to_raw_data
            .wrapping_sub(rva_offset);
    }
}

/// An analyzed COFF object
#[derive(Debug)]
pub struct Coff<'a> {
    /// The COFF header
    pub header: header::CoffHeader,
    /// A list of the sections in this COFF binary
    pub sections: Vec<section_table::SectionTable>,
    /// The COFF symbol table, they are not guaranteed to exist.
    /// For an image, this is expected to be None as COFF debugging information
    /// has been deprecated.
    pub symbols: Option<symbol::SymbolTable<'a>>,
    /// The string table, they don't exist if COFF symbol table does not exist.
    pub strings: Option<strtab::Strtab<'a>>,
}

impl<'a> Coff<'a> {
    /// Reads a COFF object from the underlying `bytes`
    pub fn parse(bytes: &'a [u8]) -> error::Result<Self> {
        let offset = &mut 0;
        let header = header::CoffHeader::parse(bytes, offset)?;
        debug!("{:#?}", header);
        // TODO: maybe parse optional header, but it isn't present for Windows.
        *offset += header.size_of_optional_header as usize;
        let sections = header.sections(bytes, offset)?;
        let symbols = header.symbols(bytes)?;
        let strings = header.strings(bytes)?;
        Ok(Coff {
            header,
            sections,
            symbols,
            strings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Coff;
    use super::PE;

    static INVALID_DOS_SIGNATURE: [u8; 512] = [
        0x3D, 0x5A, 0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x00, 0x0E, 0x1F, 0xBA, 0x0E, 0x00, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01,
        0x4C, 0xCD, 0x21, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D,
        0x20, 0x63, 0x61, 0x6E, 0x6E, 0x6F, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20,
        0x69, 0x6E, 0x20, 0x44, 0x4F, 0x53, 0x20, 0x6D, 0x6F, 0x64, 0x65, 0x2E, 0x0D, 0x0D, 0x0A,
        0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x31, 0xE2, 0xB1, 0x67, 0x50, 0x8C,
        0xE2, 0x67, 0x50, 0x8C, 0xE2, 0x67, 0x50, 0x8C, 0xE2, 0x3C, 0x38, 0x88, 0xE3, 0x6D, 0x50,
        0x8C, 0xE2, 0x3C, 0x38, 0x8F, 0xE3, 0x62, 0x50, 0x8C, 0xE2, 0x3C, 0x38, 0x89, 0xE3, 0xE0,
        0x50, 0x8C, 0xE2, 0xAC, 0x3F, 0x89, 0xE3, 0x42, 0x50, 0x8C, 0xE2, 0xAC, 0x3F, 0x88, 0xE3,
        0x77, 0x50, 0x8C, 0xE2, 0xAC, 0x3F, 0x8F, 0xE3, 0x6E, 0x50, 0x8C, 0xE2, 0x3C, 0x38, 0x8D,
        0xE3, 0x64, 0x50, 0x8C, 0xE2, 0x67, 0x50, 0x8D, 0xE2, 0x3F, 0x50, 0x8C, 0xE2, 0xE1, 0x20,
        0x85, 0xE3, 0x66, 0x50, 0x8C, 0xE2, 0xE1, 0x20, 0x73, 0xE2, 0x66, 0x50, 0x8C, 0xE2, 0xE1,
        0x20, 0x8E, 0xE3, 0x66, 0x50, 0x8C, 0xE2, 0x52, 0x69, 0x63, 0x68, 0x67, 0x50, 0x8C, 0xE2,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x50, 0x45, 0x00, 0x00, 0x64, 0x86, 0x07, 0x00, 0x5F, 0x41, 0xFC, 0x5E, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x22, 0x00, 0x0B, 0x02, 0x0E, 0x1A, 0x00,
        0xFC, 0x00, 0x00, 0x00, 0xD6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE4, 0x14, 0x00, 0x00,
        0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x01, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x02, 0x00, 0x00, 0x04, 0x00, 0x00, 0xE0,
        0x68, 0x02, 0x00, 0x03, 0x00, 0x60, 0x81, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xA3, 0x01, 0x00, 0x28,
        0x00, 0x00, 0x00, 0x00, 0xF0, 0x01, 0x00, 0xE0, 0x01, 0x00, 0x00, 0x00, 0xD0, 0x01, 0x00,
        0x60, 0x0F, 0x00, 0x00, 0x00, 0xC4, 0x01, 0x00, 0xF8, 0x46, 0x00, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x54, 0x06, 0x00, 0x00, 0xF0, 0x91, 0x01, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x92, 0x01, 0x00, 0x30, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x01, 0x00, 0x48, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];

    static INVALID_PE_SIGNATURE: [u8; 512] = [
        0x4D, 0x5A, 0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x00, 0x0E, 0x1F, 0xBA, 0x0E, 0x00, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01,
        0x4C, 0xCD, 0x21, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D,
        0x20, 0x63, 0x61, 0x6E, 0x6E, 0x6F, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20,
        0x69, 0x6E, 0x20, 0x44, 0x4F, 0x53, 0x20, 0x6D, 0x6F, 0x64, 0x65, 0x2E, 0x0D, 0x0D, 0x0A,
        0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x31, 0xE2, 0xB1, 0x67, 0x50, 0x8C,
        0xE2, 0x67, 0x50, 0x8C, 0xE2, 0x67, 0x50, 0x8C, 0xE2, 0x3C, 0x38, 0x88, 0xE3, 0x6D, 0x50,
        0x8C, 0xE2, 0x3C, 0x38, 0x8F, 0xE3, 0x62, 0x50, 0x8C, 0xE2, 0x3C, 0x38, 0x89, 0xE3, 0xE0,
        0x50, 0x8C, 0xE2, 0xAC, 0x3F, 0x89, 0xE3, 0x42, 0x50, 0x8C, 0xE2, 0xAC, 0x3F, 0x88, 0xE3,
        0x77, 0x50, 0x8C, 0xE2, 0xAC, 0x3F, 0x8F, 0xE3, 0x6E, 0x50, 0x8C, 0xE2, 0x3C, 0x38, 0x8D,
        0xE3, 0x64, 0x50, 0x8C, 0xE2, 0x67, 0x50, 0x8D, 0xE2, 0x3F, 0x50, 0x8C, 0xE2, 0xE1, 0x20,
        0x85, 0xE3, 0x66, 0x50, 0x8C, 0xE2, 0xE1, 0x20, 0x73, 0xE2, 0x66, 0x50, 0x8C, 0xE2, 0xE1,
        0x20, 0x8E, 0xE3, 0x66, 0x50, 0x8C, 0xE2, 0x52, 0x69, 0x63, 0x68, 0x67, 0x50, 0x8C, 0xE2,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x50, 0x05, 0x00, 0x00, 0x64, 0x86, 0x07, 0x00, 0x5F, 0x41, 0xFC, 0x5E, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x22, 0x00, 0x0B, 0x02, 0x0E, 0x1A, 0x00,
        0xFC, 0x00, 0x00, 0x00, 0xD6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE4, 0x14, 0x00, 0x00,
        0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x01, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x02, 0x00, 0x00, 0x04, 0x00, 0x00, 0xE0,
        0x68, 0x02, 0x00, 0x03, 0x00, 0x60, 0x81, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xA3, 0x01, 0x00, 0x28,
        0x00, 0x00, 0x00, 0x00, 0xF0, 0x01, 0x00, 0xE0, 0x01, 0x00, 0x00, 0x00, 0xD0, 0x01, 0x00,
        0x60, 0x0F, 0x00, 0x00, 0x00, 0xC4, 0x01, 0x00, 0xF8, 0x46, 0x00, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x54, 0x06, 0x00, 0x00, 0xF0, 0x91, 0x01, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x92, 0x01, 0x00, 0x30, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x01, 0x00, 0x48, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];

    // The assembler program used to generate this string is as follows:
    //
    // bits 64
    // default rel
    // segment .text
    // global main
    // extern ExitProcess
    // main:
    //      xor rax, rax
    //      call ExitProcess
    //
    //
    // The code can be compiled using nasm (https://nasm.us) with the command below:
    //      nasm -f win64 <filename>.asm -o <filename>.obj
    static COFF_FILE_SINGLE_STRING_IN_STRING_TABLE: [u8; 220] = [
        0x64, 0x86, 0x1, 0x0, 0xb5, 0x39, 0x91, 0x62, 0x4e, 0x0, 0x0, 0x0, 0x7, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x8, 0x0, 0x0, 0x0, 0x3c, 0x0, 0x0, 0x0, 0x44, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x1, 0x0, 0x0, 0x0, 0x20, 0x0, 0x50, 0x60, 0x48, 0x31, 0xc0, 0xe8, 0x0, 0x0, 0x0, 0x0, 0x4,
        0x0, 0x0, 0x0, 0x5, 0x0, 0x0, 0x0, 0x4, 0x0, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0xfe, 0xff, 0x0, 0x0, 0x67, 0x1, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
        0x73, 0x2e, 0x61, 0x73, 0x6d, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2e, 0x74, 0x65, 0x78,
        0x74, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x3, 0x1, 0x8, 0x0, 0x0, 0x0,
        0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2e, 0x61, 0x62,
        0x73, 0x6f, 0x6c, 0x75, 0x74, 0x0, 0x0, 0x0, 0x0, 0xff, 0xff, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x6d, 0x61,
        0x69, 0x6e, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x2, 0x0, 0x10,
        0x0, 0x0, 0x0, 0x45, 0x78, 0x69, 0x74, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x0,
    ];

    #[test]
    fn string_table_excludes_length() {
        let coff = Coff::parse(&&COFF_FILE_SINGLE_STRING_IN_STRING_TABLE[..]).unwrap();
        let string_table = coff.strings.unwrap().to_vec().unwrap();

        assert!(string_table == vec!["ExitProcess"]);
    }

    #[test]
    fn symbol_name_excludes_length() {
        let coff = Coff::parse(&COFF_FILE_SINGLE_STRING_IN_STRING_TABLE).unwrap();
        let strings = coff.strings.unwrap();
        let symbols = coff
            .symbols
            .unwrap()
            .iter()
            .filter(|(_, name, _)| name.is_none())
            .map(|(_, _, sym)| sym.name(&strings).unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(symbols, vec!["ExitProcess"])
    }

    #[test]
    fn invalid_dos_header() {
        if let Ok(_) = PE::parse(&INVALID_DOS_SIGNATURE) {
            panic!("must not parse PE with invalid DOS header");
        }
    }

    #[test]
    fn invalid_pe_header() {
        if let Ok(_) = PE::parse(&INVALID_PE_SIGNATURE) {
            panic!("must not parse PE with invalid PE header");
        }
    }
}
