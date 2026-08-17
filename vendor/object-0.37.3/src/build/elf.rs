//! This module provides a [`Builder`] for reading, modifying, and then writing ELF files.
use alloc::vec::Vec;
use core::convert::TryInto;
use core::fmt;
use core::marker::PhantomData;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

use crate::build::{ByteString, Bytes, Error, Id, IdPrivate, Item, Result, Table};
use crate::elf;
use crate::read::elf::{Dyn, FileHeader, ProgramHeader, Rela, SectionHeader, Sym};
use crate::read::{self, FileKind, ReadRef};
use crate::write;
use crate::Endianness;

/// A builder for reading, modifying, and then writing ELF files.
///
/// Public fields are available for modifying the values that will be written.
/// Methods are available to add elements to tables, and elements can be deleted
/// from tables by setting the `delete` field in the element.
#[derive(Debug)]
pub struct Builder<'data> {
    /// The endianness.
    ///
    /// Used to set the data encoding when writing the ELF file.
    pub endian: Endianness,
    /// Whether file is 64-bit.
    ///
    /// Use to set the file class when writing the ELF file.
    pub is_64: bool,
    /// The alignment of [`elf::PT_LOAD`] segments.
    ///
    /// This is an informational field and is not used when writing the ELF file.
    /// It can optionally be used when calling [`Segments::add_load_segment`].
    ///
    /// It is determined heuristically when reading the ELF file. Currently,
    /// if all load segments have the same alignment, that alignment is used,
    /// otherwise it is set to 1.
    pub load_align: u64,
    /// The file header.
    pub header: Header,
    /// The segment table.
    pub segments: Segments<'data>,
    /// The section table.
    pub sections: Sections<'data>,
    /// The symbol table.
    pub symbols: Symbols<'data>,
    /// The dynamic symbol table.
    pub dynamic_symbols: DynamicSymbols<'data>,
    /// The base version for the GNU version definitions.
    ///
    /// This will be written as a version definition with index 1.
    pub version_base: Option<ByteString<'data>>,
    /// The GNU version definitions and dependencies.
    pub versions: Versions<'data>,
    /// The filenames used in the GNU version definitions.
    pub version_files: VersionFiles<'data>,
    /// The bucket count parameter for the hash table.
    pub hash_bucket_count: u32,
    /// The bloom shift parameter for the GNU hash table.
    pub gnu_hash_bloom_shift: u32,
    /// The bloom count parameter for the GNU hash table.
    pub gnu_hash_bloom_count: u32,
    /// The bucket count parameter for the GNU hash table.
    pub gnu_hash_bucket_count: u32,
    marker: PhantomData<()>,
}

impl<'data> Builder<'data> {
    /// Create a new ELF builder.
    pub fn new(endian: Endianness, is_64: bool) -> Self {
        Self {
            endian,
            is_64,
            load_align: 1,
            header: Header::default(),
            segments: Segments::new(),
            sections: Sections::new(),
            symbols: Symbols::new(),
            dynamic_symbols: Symbols::new(),
            version_base: None,
            versions: Versions::new(),
            version_files: VersionFiles::new(),
            hash_bucket_count: 0,
            gnu_hash_bloom_shift: 0,
            gnu_hash_bloom_count: 0,
            gnu_hash_bucket_count: 0,
            marker: PhantomData,
        }
    }

    /// Read the ELF file from file data.
    pub fn read<R: ReadRef<'data>>(data: R) -> Result<Self> {
        match FileKind::parse(data)? {
            FileKind::Elf32 => Self::read32(data),
            FileKind::Elf64 => Self::read64(data),
            #[allow(unreachable_patterns)]
            _ => Err(Error::new("Not an ELF file")),
        }
    }

    /// Read a 32-bit ELF file from file data.
    pub fn read32<R: ReadRef<'data>>(data: R) -> Result<Self> {
        Self::read_file::<elf::FileHeader32<Endianness>, R>(data)
    }

    /// Read a 64-bit ELF file from file data.
    pub fn read64<R: ReadRef<'data>>(data: R) -> Result<Self> {
        Self::read_file::<elf::FileHeader64<Endianness>, R>(data)
    }

    fn read_file<Elf, R>(data: R) -> Result<Self>
    where
        Elf: FileHeader<Endian = Endianness>,
        R: ReadRef<'data>,
    {
        let header = Elf::parse(data)?;
        let endian = header.endian()?;
        let is_mips64el = header.is_mips64el(endian);
        let section_strings_index = header.section_strings_index(endian, data)?;
        let segments = header.program_headers(endian, data)?;
        let sections = header.sections(endian, data)?;
        let symbols = sections.symbols(endian, data, elf::SHT_SYMTAB)?;
        let dynamic_symbols = sections.symbols(endian, data, elf::SHT_DYNSYM)?;

        let mut builder = Builder {
            endian,
            is_64: header.is_type_64(),
            load_align: 0,
            header: Header {
                os_abi: header.e_ident().os_abi,
                abi_version: header.e_ident().abi_version,
                e_type: header.e_type(endian),
                e_machine: header.e_machine(endian),
                e_entry: header.e_entry(endian).into(),
                e_flags: header.e_flags(endian),
                e_phoff: header.e_phoff(endian).into(),
            },
            segments: Segments::new(),
            sections: Sections::new(),
            symbols: Symbols::new(),
            dynamic_symbols: Symbols::new(),
            version_base: None,
            versions: Versions::new(),
            version_files: VersionFiles::new(),
            hash_bucket_count: 0,
            gnu_hash_bloom_shift: 0,
            gnu_hash_bloom_count: 0,
            gnu_hash_bucket_count: 0,
            marker: PhantomData,
        };

        for segment in segments {
            if segment.p_type(endian) == elf::PT_LOAD {
                let p_align = segment.p_align(endian).into();
                if builder.load_align == 0 {
                    builder.load_align = p_align;
                } else if builder.load_align != p_align {
                    builder.load_align = 1;
                }
            }

            let id = builder.segments.next_id();
            builder.segments.push(Segment {
                id,
                delete: false,
                p_type: segment.p_type(endian),
                p_flags: segment.p_flags(endian),
                p_offset: segment.p_offset(endian).into(),
                p_vaddr: segment.p_vaddr(endian).into(),
                p_paddr: segment.p_paddr(endian).into(),
                p_filesz: segment.p_filesz(endian).into(),
                p_memsz: segment.p_memsz(endian).into(),
                p_align: segment.p_align(endian).into(),
                sections: Vec::new(),
                marker: PhantomData,
            });
        }
        if builder.load_align == 0 {
            builder.load_align = 1;
        }

        for (index, section) in sections.enumerate().skip(1) {
            let id = SectionId(index.0 - 1);
            let relocations = if let Some((rels, link)) = section.rel(endian, data)? {
                Self::read_relocations(
                    index,
                    endian,
                    is_mips64el,
                    section,
                    rels,
                    link,
                    &symbols,
                    &dynamic_symbols,
                )?
            } else if let Some((rels, link)) = section.rela(endian, data)? {
                Self::read_relocations(
                    index,
                    endian,
                    is_mips64el,
                    section,
                    rels,
                    link,
                    &symbols,
                    &dynamic_symbols,
                )?
            } else {
                SectionData::Data(Bytes::default())
            };
            if let Some(hash) = section.hash_header(endian, data)? {
                builder.hash_bucket_count = hash.bucket_count.get(endian);
            }
            if let Some(hash) = section.gnu_hash_header(endian, data)? {
                builder.gnu_hash_bloom_shift = hash.bloom_shift.get(endian);
                builder.gnu_hash_bloom_count = hash.bloom_count.get(endian);
                builder.gnu_hash_bucket_count = hash.bucket_count.get(endian);
            }
            let name = sections.section_name(endian, section)?;
            let data = match section.sh_type(endian) {
                elf::SHT_NOBITS => SectionData::UninitializedData(section.sh_size(endian).into()),
                // Section types that we treat as opaque data. In future, some of these could be
                // changed to a parsed variant if we need to modify their contents.
                elf::SHT_PROGBITS
                | elf::SHT_INIT_ARRAY
                | elf::SHT_FINI_ARRAY
                | elf::SHT_PREINIT_ARRAY
                | elf::SHT_RELR
                | elf::SHT_CREL
                | elf::SHT_LLVM_DEPENDENT_LIBRARIES => {
                    SectionData::Data(section.data(endian, data)?.into())
                }
                elf::SHT_REL | elf::SHT_RELA => relocations,
                elf::SHT_SYMTAB => {
                    if index == symbols.section() {
                        SectionData::Symbol
                    } else {
                        return Err(Error(format!(
                            "Unsupported SHT_SYMTAB section at index {}",
                            index
                        )));
                    }
                }
                elf::SHT_SYMTAB_SHNDX => {
                    if index == symbols.shndx_section() {
                        SectionData::SymbolSectionIndex
                    } else {
                        return Err(Error(format!(
                            "Unsupported SHT_SYMTAB_SHNDX section at index {}",
                            index
                        )));
                    }
                }
                elf::SHT_DYNSYM => {
                    if index == dynamic_symbols.section() {
                        SectionData::DynamicSymbol
                    } else {
                        return Err(Error(format!(
                            "Unsupported SHT_DYNSYM section at index {}",
                            index
                        )));
                    }
                }
                elf::SHT_STRTAB => {
                    if index == symbols.string_section() {
                        SectionData::String
                    } else if index == dynamic_symbols.string_section() {
                        SectionData::DynamicString
                    } else if index == section_strings_index {
                        SectionData::SectionString
                    } else if name == b".annobin.notes" {
                        // Not actually a string table because nothing references the strings.
                        // We simply need to preserve the data (similar to a .comment section).
                        SectionData::Data(section.data(endian, data)?.into())
                    } else {
                        return Err(Error(format!(
                            "Unsupported SHT_STRTAB section at index {}",
                            index
                        )));
                    }
                }
                elf::SHT_NOTE => SectionData::Note(section.data(endian, data)?.into()),
                elf::SHT_DYNAMIC => {
                    let (dyns, link) = section.dynamic(endian, data)?.unwrap();
                    let dynamic_strings = sections.strings(endian, data, link)?;
                    Self::read_dynamics::<Elf, _>(endian, dyns, dynamic_strings)?
                }
                elf::SHT_GNU_ATTRIBUTES => {
                    let attributes = section.attributes(endian, data)?;
                    Self::read_attributes(index, attributes, sections.len(), symbols.len())?
                }
                elf::SHT_HASH => SectionData::Hash,
                elf::SHT_GNU_HASH => SectionData::GnuHash,
                elf::SHT_GNU_VERSYM => SectionData::GnuVersym,
                elf::SHT_GNU_VERDEF => SectionData::GnuVerdef,
                elf::SHT_GNU_VERNEED => SectionData::GnuVerneed,
                other => match (builder.header.e_machine, other) {
                    (elf::EM_ARM, elf::SHT_ARM_ATTRIBUTES)
                    | (elf::EM_AARCH64, elf::SHT_AARCH64_ATTRIBUTES)
                    | (elf::EM_CSKY, elf::SHT_CSKY_ATTRIBUTES)
                    | (elf::EM_RISCV, elf::SHT_RISCV_ATTRIBUTES) => {
                        let attributes = section.attributes(endian, data)?;
                        Self::read_attributes(index, attributes, sections.len(), symbols.len())?
                    }
                    // Some section types that we can't parse but that are safe to copy.
                    // Lots of types missing, add as needed. We can't default to copying
                    // everything because some types are not safe to copy.
                    (elf::EM_ARM, elf::SHT_ARM_EXIDX)
                    | (elf::EM_IA_64, elf::SHT_IA_64_UNWIND)
                    | (elf::EM_MIPS, elf::SHT_MIPS_REGINFO)
                    | (elf::EM_MIPS, elf::SHT_MIPS_DWARF)
                    | (elf::EM_X86_64, elf::SHT_X86_64_UNWIND) => {
                        SectionData::Data(section.data(endian, data)?.into())
                    }
                    _ => return Err(Error(format!("Unsupported section type {:x}", other))),
                },
            };
            let sh_flags = section.sh_flags(endian).into();
            let sh_link = section.sh_link(endian);
            let sh_link_section = if sh_link == 0 {
                None
            } else {
                if sh_link as usize >= sections.len() {
                    return Err(Error(format!(
                        "Invalid sh_link {} in section at index {}",
                        sh_link, index
                    )));
                }
                Some(SectionId(sh_link as usize - 1))
            };
            let sh_info = section.sh_info(endian);
            let sh_info_section = if sh_info == 0 || sh_flags & u64::from(elf::SHF_INFO_LINK) == 0 {
                None
            } else {
                if sh_info as usize >= sections.len() {
                    return Err(Error(format!(
                        "Invalid sh_info link {} in section at index {}",
                        sh_info, index
                    )));
                }
                Some(SectionId(sh_info as usize - 1))
            };
            let sh_flags = section.sh_flags(endian).into();
            let sh_addr = section.sh_addr(endian).into();
            if sh_flags & u64::from(elf::SHF_ALLOC) != 0 {
                for segment in &mut builder.segments {
                    if segment.contains_address(sh_addr) {
                        segment.sections.push(id);
                    }
                }
            }
            builder.sections.push(Section {
                id,
                delete: false,
                name: name.into(),
                sh_type: section.sh_type(endian),
                sh_flags,
                sh_addr,
                sh_offset: section.sh_offset(endian).into(),
                sh_size: section.sh_size(endian).into(),
                sh_link_section,
                sh_info,
                sh_info_section,
                sh_addralign: section.sh_addralign(endian).into(),
                sh_entsize: section.sh_entsize(endian).into(),
                data,
            });
        }

        Self::read_symbols(
            endian,
            &symbols,
            &mut builder.symbols,
            builder.sections.len(),
        )?;
        Self::read_symbols(
            endian,
            &dynamic_symbols,
            &mut builder.dynamic_symbols,
            builder.sections.len(),
        )?;
        builder.read_gnu_versions(endian, data, &sections, &dynamic_symbols)?;

        Ok(builder)
    }

    #[allow(clippy::too_many_arguments)]
    fn read_relocations<Elf, Rel, R>(
        index: read::SectionIndex,
        endian: Elf::Endian,
        is_mips64el: bool,
        section: &'data Elf::SectionHeader,
        rels: &'data [Rel],
        link: read::SectionIndex,
        symbols: &read::elf::SymbolTable<'data, Elf, R>,
        dynamic_symbols: &read::elf::SymbolTable<'data, Elf, R>,
    ) -> Result<SectionData<'data>>
    where
        Elf: FileHeader<Endian = Endianness>,
        Rel: Copy + Into<Elf::Rela>,
        R: ReadRef<'data>,
    {
        if link == dynamic_symbols.section() {
            Self::read_relocations_impl::<Elf, Rel, true>(
                index,
                endian,
                is_mips64el,
                rels,
                dynamic_symbols.len(),
            )
            .map(SectionData::DynamicRelocation)
        } else if link.0 == 0 || section.sh_flags(endian).into() & u64::from(elf::SHF_ALLOC) != 0 {
            // If there's no link, then none of the relocations may reference symbols.
            // Assume that these are dynamic relocations, but don't use the dynamic
            // symbol table when parsing.
            //
            // Additionally, sometimes there is an allocated section that links to
            // the static symbol table. We don't currently support this case in general,
            // but if none of the relocation entries reference a symbol then it is
            // safe to treat it as a dynamic relocation section.
            //
            // For both of these cases, if there is a reference to a symbol then
            // an error will be returned when parsing the relocations.
            Self::read_relocations_impl::<Elf, Rel, true>(index, endian, is_mips64el, rels, 0)
                .map(SectionData::DynamicRelocation)
        } else if link == symbols.section() {
            Self::read_relocations_impl::<Elf, Rel, false>(
                index,
                endian,
                is_mips64el,
                rels,
                symbols.len(),
            )
            .map(SectionData::Relocation)
        } else {
            return Err(Error(format!(
                "Invalid sh_link {} in relocation section at index {}",
                link.0, index,
            )));
        }
    }

    fn read_relocations_impl<Elf, Rel, const DYNAMIC: bool>(
        index: read::SectionIndex,
        endian: Elf::Endian,
        is_mips64el: bool,
        rels: &'data [Rel],
        symbols_len: usize,
    ) -> Result<Vec<Relocation<DYNAMIC>>>
    where
        Elf: FileHeader<Endian = Endianness>,
        Rel: Copy + Into<Elf::Rela>,
    {
        let mut relocations = Vec::new();
        for rel in rels {
            let rel = (*rel).into();
            let symbol = if let Some(symbol) = rel.symbol(endian, is_mips64el) {
                if symbol.0 >= symbols_len {
                    return Err(Error(format!(
                        "Invalid symbol index {} in relocation section at index {}",
                        symbol, index,
                    )));
                }
                Some(SymbolId(symbol.0 - 1))
            } else {
                None
            };
            relocations.push(Relocation {
                r_offset: rel.r_offset(endian).into(),
                symbol,
                r_type: rel.r_type(endian, is_mips64el),
                r_addend: rel.r_addend(endian).into(),
            });
        }
        Ok(relocations)
    }

    fn read_dynamics<Elf, R>(
        endian: Elf::Endian,
        dyns: &'data [Elf::Dyn],
        strings: read::StringTable<'data, R>,
    ) -> Result<SectionData<'data>>
    where
        Elf: FileHeader<Endian = Endianness>,
        R: ReadRef<'data>,
    {
        let mut dynamics = Vec::with_capacity(dyns.len());
        for d in dyns {
            let tag = d.d_tag(endian).into().try_into().map_err(|_| {
                Error(format!(
                    "Unsupported dynamic tag 0x{:x}",
                    d.d_tag(endian).into()
                ))
            })?;
            if tag == elf::DT_NULL {
                break;
            }
            let val = d.d_val(endian).into();
            dynamics.push(if d.is_string(endian) {
                let val =
                    strings
                        .get(val.try_into().map_err(|_| {
                            Error(format!("Unsupported dynamic string 0x{:x}", val))
                        })?)
                        .map_err(|_| Error(format!("Invalid dynamic string 0x{:x}", val)))?;
                Dynamic::String {
                    tag,
                    val: val.into(),
                }
            } else {
                match tag {
                    elf::DT_SYMTAB
                    | elf::DT_STRTAB
                    | elf::DT_STRSZ
                    | elf::DT_HASH
                    | elf::DT_GNU_HASH
                    | elf::DT_VERSYM
                    | elf::DT_VERDEF
                    | elf::DT_VERDEFNUM
                    | elf::DT_VERNEED
                    | elf::DT_VERNEEDNUM => Dynamic::Auto { tag },
                    _ => Dynamic::Integer { tag, val },
                }
            });
        }
        Ok(SectionData::Dynamic(dynamics))
    }

    fn read_symbols<Elf, R, const DYNAMIC: bool>(
        endian: Elf::Endian,
        symbols: &read::elf::SymbolTable<'data, Elf, R>,
        builder_symbols: &mut Symbols<'data, DYNAMIC>,
        sections_len: usize,
    ) -> Result<()>
    where
        Elf: FileHeader<Endian = Endianness>,
        R: ReadRef<'data>,
    {
        for (index, symbol) in symbols.enumerate().skip(1) {
            let id = SymbolId(index.0 - 1);
            let section =
                if let Some(section_index) = symbols.symbol_section(endian, symbol, index)? {
                    let section_id = section_index.0.wrapping_sub(1);
                    if section_id >= sections_len {
                        return Err(Error::new("Invalid symbol section index"));
                    }
                    Some(SectionId(section_id))
                } else {
                    None
                };
            builder_symbols.push(Symbol {
                id,
                delete: false,
                name: symbols.symbol_name(endian, symbol)?.into(),
                section,
                st_info: symbol.st_info(),
                st_other: symbol.st_other(),
                st_shndx: symbol.st_shndx(endian),
                st_value: symbol.st_value(endian).into(),
                st_size: symbol.st_size(endian).into(),
                version: VersionId::local(),
                version_hidden: false,
            });
        }
        Ok(())
    }

    fn read_attributes<Elf>(
        index: read::SectionIndex,
        attributes: read::elf::AttributesSection<'data, Elf>,
        sections_len: usize,
        symbols_len: usize,
    ) -> Result<SectionData<'data>>
    where
        Elf: FileHeader<Endian = Endianness>,
    {
        let mut builder_attributes = AttributesSection::new();
        let mut subsections = attributes.subsections()?;
        while let Some(subsection) = subsections.next()? {
            let mut builder_subsection = AttributesSubsection::new(subsection.vendor().into());
            let mut subsubsections = subsection.subsubsections();
            while let Some(subsubsection) = subsubsections.next()? {
                let tag = match subsubsection.tag() {
                    elf::Tag_File => AttributeTag::File,
                    elf::Tag_Section => {
                        let mut tag_sections = Vec::new();
                        let mut indices = subsubsection.indices();
                        while let Some(index) = indices.next()? {
                            let index = index as usize;
                            if index >= sections_len {
                                return Err(Error(format!(
                                    "Invalid section index {} in attribute",
                                    index
                                )));
                            }
                            tag_sections.push(SectionId(index - 1));
                        }
                        AttributeTag::Section(tag_sections)
                    }
                    elf::Tag_Symbol => {
                        let mut tag_symbols = Vec::new();
                        let mut indices = subsubsection.indices();
                        while let Some(index) = indices.next()? {
                            let index = index as usize;
                            if index >= symbols_len {
                                return Err(Error(format!(
                                    "Invalid symbol index {} in attribute",
                                    index
                                )));
                            }
                            tag_symbols.push(SymbolId(index - 1));
                        }
                        AttributeTag::Symbol(tag_symbols)
                    }
                    tag => {
                        return Err(Error(format!(
                            "Unsupported attribute tag 0x{:x} in section at index {}",
                            tag, index,
                        )))
                    }
                };
                let data = subsubsection.attributes_data().into();
                builder_subsection
                    .subsubsections
                    .push(AttributesSubsubsection { tag, data });
            }
            builder_attributes.subsections.push(builder_subsection);
        }
        Ok(SectionData::Attributes(builder_attributes))
    }

    fn read_gnu_versions<Elf, R>(
        &mut self,
        endian: Elf::Endian,
        data: R,
        sections: &read::elf::SectionTable<'data, Elf, R>,
        dynamic_symbols: &read::elf::SymbolTable<'data, Elf, R>,
    ) -> Result<()>
    where
        Elf: FileHeader<Endian = Endianness>,
        R: ReadRef<'data>,
    {
        let strings = dynamic_symbols.strings();
        let mut ids = HashMap::new();
        ids.insert(0, VersionId::local());
        ids.insert(1, VersionId::global());

        if let Some((mut verdefs, link)) = sections.gnu_verdef(endian, data)? {
            if link != dynamic_symbols.string_section() {
                return Err(Error::new("Invalid SHT_GNU_VERDEF section"));
            }
            while let Some((verdef, mut verdauxs)) = verdefs.next()? {
                let flags = verdef.vd_flags.get(endian);
                if flags & elf::VER_FLG_BASE != 0 {
                    if flags != elf::VER_FLG_BASE
                        || verdef.vd_ndx.get(endian) != 1
                        || verdef.vd_cnt.get(endian) != 1
                    {
                        return Err(Error::new("Unsupported VER_FLG_BASE in SHT_GNU_VERDEF"));
                    }
                    if self.version_base.is_some() {
                        return Err(Error::new("Duplicate VER_FLG_BASE in SHT_GNU_VERDEF"));
                    }
                    let verdaux = verdauxs.next()?.ok_or_else(|| {
                        Error::new("Missing name for VER_FLG_BASE in SHT_GNU_VERDEF")
                    })?;
                    self.version_base = Some(verdaux.name(endian, strings)?.into());
                    continue;
                }

                let index = verdef.vd_ndx.get(endian) & elf::VERSYM_VERSION;
                let id = self.versions.next_id();
                if ids.insert(index, id).is_some() {
                    return Err(Error(format!("Duplicate SHT_GNU_VERDEF index {}", index)));
                }

                let mut names = Vec::new();
                while let Some(verdaux) = verdauxs.next()? {
                    names.push(verdaux.name(endian, strings)?.into());
                }

                let data = VersionData::Def(VersionDef { flags, names });
                self.versions.push(Version {
                    id,
                    delete: false,
                    data,
                });
            }
        }

        if let Some((mut verneeds, link)) = sections.gnu_verneed(endian, data)? {
            if link != dynamic_symbols.string_section() {
                return Err(Error::new("Invalid SHT_GNU_VERNEED section"));
            }
            while let Some((verneed, mut vernauxs)) = verneeds.next()? {
                let file = VersionFileId(self.version_files.len());
                self.version_files.push(VersionFile {
                    id: file,
                    delete: false,
                    name: verneed.file(endian, strings)?.into(),
                });
                while let Some(vernaux) = vernauxs.next()? {
                    let index = vernaux.vna_other.get(endian) & elf::VERSYM_VERSION;
                    let id = self.versions.next_id();
                    if ids.insert(index, id).is_some() {
                        return Err(Error(format!("Duplicate SHT_GNU_VERNEED index {}", index)));
                    }

                    let data = VersionData::Need(VersionNeed {
                        flags: vernaux.vna_flags.get(endian),
                        name: vernaux.name(endian, strings)?.into(),
                        file,
                    });
                    self.versions.push(Version {
                        id,
                        delete: false,
                        data,
                    });
                }
            }
        }

        if let Some((versyms, link)) = sections.gnu_versym(endian, data)? {
            if versyms.len() != dynamic_symbols.len() || link != dynamic_symbols.section() {
                return Err(Error::new("Invalid SHT_GNU_VERSYM section"));
            }
            for (id, versym) in versyms.iter().skip(1).enumerate() {
                let index = versym.0.get(endian);
                let symbol = self.dynamic_symbols.get_mut(SymbolId(id));
                symbol.version = *ids
                    .get(&(index & elf::VERSYM_VERSION))
                    .ok_or_else(|| Error(format!("Invalid SHT_GNU_VERSYM index {:x}", index)))?;
                symbol.version_hidden = index & elf::VERSYM_HIDDEN != 0;
            }
        }
        Ok(())
    }

    /// Write the ELF file to the buffer.
    pub fn write(mut self, buffer: &mut dyn write::WritableBuffer) -> Result<()> {
        struct SectionOut {
            id: SectionId,
            name: Option<write::StringId>,
            offset: usize,
            attributes: Vec<u8>,
        }

        struct SymbolOut {
            id: SymbolId,
            name: Option<write::StringId>,
        }

        struct DynamicSymbolOut {
            id: DynamicSymbolId,
            name: Option<write::StringId>,
            hash: Option<u32>,
            gnu_hash: Option<u32>,
        }

        #[derive(Default, Clone)]
        struct VersionFileOut {
            versions: Vec<VersionId>,
        }

        // TODO: require the caller to do this?
        self.delete_orphans();
        self.delete_unused_versions();

        let mut writer = write::elf::Writer::new(self.endian, self.is_64, buffer);

        // Find metadata sections, and assign section indices.
        let mut shstrtab_id = None;
        let mut symtab_id = None;
        let mut symtab_shndx_id = None;
        let mut strtab_id = None;
        let mut dynsym_id = None;
        let mut dynstr_id = None;
        let mut hash_id = None;
        let mut gnu_hash_id = None;
        let mut gnu_versym_id = None;
        let mut gnu_verdef_id = None;
        let mut gnu_verneed_id = None;
        let mut out_sections = Vec::with_capacity(self.sections.len());
        let mut out_sections_index = vec![None; self.sections.len()];
        if !self.sections.is_empty() {
            writer.reserve_null_section_index();
        }
        for section in &self.sections {
            let index = match &section.data {
                SectionData::Data(_)
                | SectionData::UninitializedData(_)
                | SectionData::Relocation(_)
                | SectionData::DynamicRelocation(_)
                | SectionData::Note(_)
                | SectionData::Dynamic(_)
                | SectionData::Attributes(_) => writer.reserve_section_index(),
                SectionData::SectionString => {
                    if shstrtab_id.is_some() {
                        return Err(Error::new("Multiple .shstrtab sections"));
                    }
                    shstrtab_id = Some(section.id);
                    writer.reserve_shstrtab_section_index_with_name(&section.name)
                }
                SectionData::Symbol => {
                    if symtab_id.is_some() {
                        return Err(Error::new("Multiple .symtab sections"));
                    }
                    symtab_id = Some(section.id);
                    writer.reserve_symtab_section_index_with_name(&section.name)
                }
                SectionData::SymbolSectionIndex => {
                    if symtab_shndx_id.is_some() {
                        return Err(Error::new("Multiple .symtab_shndx sections"));
                    }
                    symtab_shndx_id = Some(section.id);
                    writer.reserve_symtab_shndx_section_index_with_name(&section.name)
                }
                SectionData::String => {
                    if strtab_id.is_some() {
                        return Err(Error::new("Multiple .strtab sections"));
                    }
                    strtab_id = Some(section.id);
                    writer.reserve_strtab_section_index_with_name(&section.name)
                }
                SectionData::DynamicSymbol => {
                    if dynsym_id.is_some() {
                        return Err(Error::new("Multiple .dynsym sections"));
                    }
                    dynsym_id = Some(section.id);
                    writer.reserve_dynsym_section_index_with_name(&section.name)
                }
                SectionData::DynamicString => {
                    if dynstr_id.is_some() {
                        return Err(Error::new("Multiple .dynstr sections"));
                    }
                    dynstr_id = Some(section.id);
                    writer.reserve_dynstr_section_index_with_name(&section.name)
                }
                SectionData::Hash => {
                    if hash_id.is_some() {
                        return Err(Error::new("Multiple .hash sections"));
                    }
                    hash_id = Some(section.id);
                    writer.reserve_hash_section_index_with_name(&section.name)
                }
                SectionData::GnuHash => {
                    if gnu_hash_id.is_some() {
                        return Err(Error::new("Multiple .gnu.hash sections"));
                    }
                    gnu_hash_id = Some(section.id);
                    writer.reserve_gnu_hash_section_index_with_name(&section.name)
                }
                SectionData::GnuVersym => {
                    if gnu_versym_id.is_some() {
                        return Err(Error::new("Multiple .gnu.version sections"));
                    }
                    gnu_versym_id = Some(section.id);
                    writer.reserve_gnu_versym_section_index_with_name(&section.name)
                }
                SectionData::GnuVerdef => {
                    if gnu_verdef_id.is_some() {
                        return Err(Error::new("Multiple .gnu.version_d sections"));
                    }
                    gnu_verdef_id = Some(section.id);
                    writer.reserve_gnu_verdef_section_index_with_name(&section.name)
                }
                SectionData::GnuVerneed => {
                    if gnu_verneed_id.is_some() {
                        return Err(Error::new("Multiple .gnu.version_r sections"));
                    }
                    gnu_verneed_id = Some(section.id);
                    writer.reserve_gnu_verneed_section_index_with_name(&section.name)
                }
            };
            out_sections_index[section.id.0] = Some(index);

            let name = if section.name.is_empty() {
                None
            } else {
                Some(writer.add_section_name(&section.name))
            };
            out_sections.push(SectionOut {
                id: section.id,
                name,
                offset: 0,
                attributes: Vec::new(),
            });
        }

        // Assign dynamic strings.
        for section in &self.sections {
            if let SectionData::Dynamic(dynamics) = &section.data {
                for dynamic in dynamics {
                    if let Dynamic::String { val, .. } = dynamic {
                        writer.add_dynamic_string(val);
                    }
                }
            }
        }

        // Assign dynamic symbol indices.
        let mut out_dynsyms = Vec::with_capacity(self.dynamic_symbols.len());
        // Local symbols must come before global.
        let local_symbols = self
            .dynamic_symbols
            .into_iter()
            .filter(|symbol| symbol.st_bind() == elf::STB_LOCAL);
        let global_symbols = self
            .dynamic_symbols
            .into_iter()
            .filter(|symbol| symbol.st_bind() != elf::STB_LOCAL);
        for symbol in local_symbols.chain(global_symbols) {
            let mut name = None;
            let mut hash = None;
            let mut gnu_hash = None;
            if !symbol.name.is_empty() {
                name = Some(writer.add_dynamic_string(&symbol.name));
                if hash_id.is_some() {
                    hash = Some(elf::hash(&symbol.name));
                }
                if gnu_hash_id.is_some()
                    && (symbol.section.is_some() || symbol.st_shndx != elf::SHN_UNDEF)
                {
                    gnu_hash = Some(elf::gnu_hash(&symbol.name));
                }
            }
            out_dynsyms.push(DynamicSymbolOut {
                id: symbol.id,
                name,
                hash,
                gnu_hash,
            });
        }
        let num_local_dynamic = out_dynsyms
            .iter()
            .take_while(|sym| self.dynamic_symbols.get(sym.id).st_bind() == elf::STB_LOCAL)
            .count();
        // We must sort for GNU hash before allocating symbol indices.
        let mut gnu_hash_symbol_count = 0;
        if gnu_hash_id.is_some() {
            if self.gnu_hash_bucket_count == 0 {
                return Err(Error::new(".gnu.hash bucket count is zero"));
            }
            // TODO: recalculate bucket_count?
            out_dynsyms[num_local_dynamic..].sort_by_key(|sym| match sym.gnu_hash {
                None => (0, 0),
                Some(hash) => (1, hash % self.gnu_hash_bucket_count),
            });
            gnu_hash_symbol_count = out_dynsyms
                .iter()
                .skip(num_local_dynamic)
                .skip_while(|sym| sym.gnu_hash.is_none())
                .count() as u32;
        }
        let mut out_dynsyms_index = vec![None; self.dynamic_symbols.len()];
        if dynsym_id.is_some() {
            writer.reserve_null_dynamic_symbol_index();
        }
        for out_dynsym in &mut out_dynsyms {
            out_dynsyms_index[out_dynsym.id.0] = Some(writer.reserve_dynamic_symbol_index());
        }

        // Hash parameters.
        let hash_index_base = 1; // Null symbol.
        let hash_chain_count = hash_index_base + out_dynsyms.len() as u32;

        // GNU hash parameters.
        let gnu_hash_index_base = if gnu_hash_symbol_count == 0 {
            0
        } else {
            out_dynsyms.len() as u32 - gnu_hash_symbol_count
        };
        let gnu_hash_symbol_base = gnu_hash_index_base + 1; // Null symbol.

        // Assign symbol indices.
        let mut out_syms = Vec::with_capacity(self.symbols.len());
        // Local symbols must come before global.
        let local_symbols = self
            .symbols
            .into_iter()
            .filter(|symbol| symbol.st_bind() == elf::STB_LOCAL);
        let global_symbols = self
            .symbols
            .into_iter()
            .filter(|symbol| symbol.st_bind() != elf::STB_LOCAL);
        for symbol in local_symbols.chain(global_symbols) {
            let name = if symbol.name.is_empty() {
                None
            } else {
                Some(writer.add_string(&symbol.name))
            };

            out_syms.push(SymbolOut {
                id: symbol.id,
                name,
            });
        }
        let num_local = out_syms
            .iter()
            .take_while(|sym| self.symbols.get(sym.id).st_bind() == elf::STB_LOCAL)
            .count();
        let mut out_syms_index = vec![None; self.symbols.len()];
        if symtab_id.is_some() {
            writer.reserve_null_symbol_index();
        }
        for out_sym in out_syms.iter_mut() {
            out_syms_index[out_sym.id.0] = Some(writer.reserve_symbol_index(None));
        }

        // Count the versions and add version strings.
        let mut verdef_count = 0;
        let mut verdaux_count = 0;
        let mut verdef_shared_base = false;
        let mut verneed_count = 0;
        let mut vernaux_count = 0;
        let mut out_version_files = vec![VersionFileOut::default(); self.version_files.len()];
        if let Some(version_base) = &self.version_base {
            verdef_count += 1;
            verdaux_count += 1;
            writer.add_dynamic_string(version_base);
        }
        for version in &self.versions {
            match &version.data {
                VersionData::Def(def) => {
                    if def.is_shared(verdef_count, self.version_base.as_ref()) {
                        verdef_shared_base = true;
                    } else {
                        verdaux_count += def.names.len();
                        for name in &def.names {
                            writer.add_dynamic_string(name);
                        }
                    }
                    verdef_count += 1;
                }
                VersionData::Need(need) => {
                    vernaux_count += 1;
                    writer.add_dynamic_string(&need.name);
                    out_version_files[need.file.0].versions.push(version.id);
                }
            }
        }
        for file in &self.version_files {
            verneed_count += 1;
            writer.add_dynamic_string(&file.name);
        }

        // Build the attributes sections.
        for out_section in &mut out_sections {
            let SectionData::Attributes(attributes) = &self.sections.get(out_section.id).data
            else {
                continue;
            };
            if attributes.subsections.is_empty() {
                continue;
            }
            let mut writer = writer.attributes_writer();
            for subsection in &attributes.subsections {
                writer.start_subsection(&subsection.vendor);
                for subsubsection in &subsection.subsubsections {
                    writer.start_subsubsection(subsubsection.tag.tag());
                    match &subsubsection.tag {
                        AttributeTag::File => {}
                        AttributeTag::Section(sections) => {
                            for id in sections {
                                if let Some(index) = out_sections_index[id.0] {
                                    writer.write_subsubsection_index(index.0);
                                }
                            }
                            writer.write_subsubsection_index(0);
                        }
                        AttributeTag::Symbol(symbols) => {
                            for id in symbols {
                                if let Some(index) = out_syms_index[id.0] {
                                    writer.write_subsubsection_index(index.0);
                                }
                            }
                            writer.write_subsubsection_index(0);
                        }
                    }
                    writer.write_subsubsection_attributes(&subsubsection.data);
                    writer.end_subsubsection();
                }
                writer.end_subsection();
            }
            out_section.attributes = writer.data();
        }

        // TODO: support section headers in strtab
        if shstrtab_id.is_none() && !out_sections.is_empty() {
            return Err(Error::new(".shstrtab section is needed but not present"));
        }
        if symtab_id.is_none() && !out_syms.is_empty() {
            return Err(Error::new(".symtab section is needed but not present"));
        }
        if symtab_shndx_id.is_none() && writer.symtab_shndx_needed() {
            return Err(Error::new(
                ".symtab.shndx section is needed but not present",
            ));
        } else if symtab_shndx_id.is_some() {
            writer.require_symtab_shndx();
        }
        if strtab_id.is_none() && writer.strtab_needed() {
            return Err(Error::new(".strtab section is needed but not present"));
        } else if strtab_id.is_some() {
            writer.require_strtab();
        }
        if dynsym_id.is_none() && !out_dynsyms.is_empty() {
            return Err(Error::new(".dynsym section is needed but not present"));
        }
        if dynstr_id.is_none() && writer.dynstr_needed() {
            return Err(Error::new(".dynstr section is needed but not present"));
        } else if dynstr_id.is_some() {
            writer.require_dynstr();
        }
        if gnu_verdef_id.is_none() && verdef_count > 0 {
            return Err(Error::new(
                ".gnu.version_d section is needed but not present",
            ));
        }
        if gnu_verneed_id.is_none() && verneed_count > 0 {
            return Err(Error::new(
                ".gnu.version_r section is needed but not present",
            ));
        }

        // Start reserving file ranges.
        writer.reserve_file_header();

        let mut dynsym_addr = None;
        let mut dynstr_addr = None;
        let mut hash_addr = None;
        let mut gnu_hash_addr = None;
        let mut versym_addr = None;
        let mut verdef_addr = None;
        let mut verneed_addr = None;

        if !self.segments.is_empty() {
            // TODO: support program headers in other locations.
            if self.header.e_phoff != writer.reserved_len() as u64 {
                return Err(Error(format!(
                    "Unsupported e_phoff value 0x{:x}",
                    self.header.e_phoff
                )));
            }
            writer.reserve_program_headers(self.segments.count() as u32);
        }

        let mut alloc_sections = Vec::new();
        if !self.segments.is_empty() {
            // Reserve alloc sections at original offsets.
            alloc_sections = out_sections
                .iter()
                .enumerate()
                .filter_map(|(index, out_section)| {
                    let section = self.sections.get(out_section.id);
                    if section.is_alloc() {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect();
            // The data for alloc sections may need to be written in a different order
            // from their section headers.
            alloc_sections.sort_by_key(|index| {
                let section = &self.sections.get(out_sections[*index].id);
                // Empty sections need to come before other sections at the same offset.
                (section.sh_offset, section.sh_size)
            });
            for index in &alloc_sections {
                let out_section = &mut out_sections[*index];
                let section = &self.sections.get(out_section.id);

                if section.sh_type == elf::SHT_NOBITS {
                    // sh_offset is meaningless for SHT_NOBITS, so preserve the input
                    // value without checking it.
                    out_section.offset = section.sh_offset as usize;
                    continue;
                }

                if section.sh_offset < writer.reserved_len() as u64 {
                    return Err(Error(format!(
                        "Unsupported sh_offset value 0x{:x} for section '{}', expected at least 0x{:x}",
                        section.sh_offset,
                        section.name,
                        writer.reserved_len(),
                    )));
                }
                // The input sh_offset needs to be preserved so that offsets in program
                // headers are correct.
                writer.reserve_until(section.sh_offset as usize);
                out_section.offset = match &section.data {
                    SectionData::Data(data) => {
                        writer.reserve(data.len(), section.sh_addralign as usize)
                    }
                    SectionData::DynamicRelocation(relocations) => writer
                        .reserve_relocations(relocations.len(), section.sh_type == elf::SHT_RELA),
                    SectionData::Note(data) => {
                        writer.reserve(data.len(), section.sh_addralign as usize)
                    }
                    SectionData::Dynamic(dynamics) => writer.reserve_dynamics(1 + dynamics.len()),
                    SectionData::DynamicSymbol => {
                        dynsym_addr = Some(section.sh_addr);
                        writer.reserve_dynsym()
                    }
                    SectionData::DynamicString => {
                        dynstr_addr = Some(section.sh_addr);
                        writer.reserve_dynstr()
                    }
                    SectionData::Hash => {
                        hash_addr = Some(section.sh_addr);
                        writer.reserve_hash(self.hash_bucket_count, hash_chain_count)
                    }
                    SectionData::GnuHash => {
                        gnu_hash_addr = Some(section.sh_addr);
                        writer.reserve_gnu_hash(
                            self.gnu_hash_bloom_count,
                            self.gnu_hash_bucket_count,
                            gnu_hash_symbol_count,
                        )
                    }
                    SectionData::GnuVersym => {
                        versym_addr = Some(section.sh_addr);
                        writer.reserve_gnu_versym()
                    }
                    SectionData::GnuVerdef => {
                        verdef_addr = Some(section.sh_addr);
                        writer.reserve_gnu_verdef(verdef_count, verdaux_count)
                    }
                    SectionData::GnuVerneed => {
                        verneed_addr = Some(section.sh_addr);
                        writer.reserve_gnu_verneed(verneed_count, vernaux_count)
                    }
                    _ => {
                        return Err(Error(format!(
                            "Unsupported alloc section type {:x} for section '{}'",
                            section.sh_type, section.name,
                        )));
                    }
                };
                if out_section.offset as u64 != section.sh_offset {
                    return Err(Error(format!(
                        "Unaligned sh_offset value 0x{:x} for section '{}', expected 0x{:x}",
                        section.sh_offset, section.name, out_section.offset,
                    )));
                }
            }
        }

        // Reserve non-alloc sections at any offset.
        for out_section in &mut out_sections {
            let section = self.sections.get(out_section.id);
            if !self.segments.is_empty() && section.is_alloc() {
                continue;
            }
            out_section.offset = match &section.data {
                SectionData::Data(data) => {
                    writer.reserve(data.len(), section.sh_addralign as usize)
                }
                SectionData::UninitializedData(_) => writer.reserved_len(),
                SectionData::Note(data) => {
                    writer.reserve(data.len(), section.sh_addralign as usize)
                }
                SectionData::Attributes(_) => {
                    writer.reserve(out_section.attributes.len(), section.sh_addralign as usize)
                }
                // These are handled elsewhere.
                SectionData::Relocation(_)
                | SectionData::SectionString
                | SectionData::Symbol
                | SectionData::SymbolSectionIndex
                | SectionData::String => {
                    continue;
                }
                _ => {
                    return Err(Error(format!(
                        "Unsupported non-alloc section type {:x}",
                        section.sh_type
                    )));
                }
            };
        }

        writer.reserve_symtab();
        writer.reserve_symtab_shndx();
        writer.reserve_strtab();

        // Reserve non-alloc relocations.
        for out_section in &mut out_sections {
            let section = self.sections.get(out_section.id);
            if !self.segments.is_empty() && section.is_alloc() {
                continue;
            }
            let SectionData::Relocation(relocations) = &section.data else {
                continue;
            };
            out_section.offset =
                writer.reserve_relocations(relocations.len(), section.sh_type == elf::SHT_RELA);
        }

        writer.reserve_shstrtab();
        writer.reserve_section_headers();

        // Start writing.
        writer.write_file_header(&write::elf::FileHeader {
            os_abi: self.header.os_abi,
            abi_version: self.header.abi_version,
            e_type: self.header.e_type,
            e_machine: self.header.e_machine,
            e_entry: self.header.e_entry,
            e_flags: self.header.e_flags,
        })?;

        if !self.segments.is_empty() {
            writer.write_align_program_headers();
            for segment in &self.segments {
                writer.write_program_header(&write::elf::ProgramHeader {
                    p_type: segment.p_type,
                    p_flags: segment.p_flags,
                    p_offset: segment.p_offset,
                    p_vaddr: segment.p_vaddr,
                    p_paddr: segment.p_paddr,
                    p_filesz: segment.p_filesz,
                    p_memsz: segment.p_memsz,
                    p_align: segment.p_align,
                });
            }
        }

        // Write alloc sections.
        if !self.segments.is_empty() {
            for index in &alloc_sections {
                let out_section = &mut out_sections[*index];
                let section = self.sections.get(out_section.id);

                if section.sh_type == elf::SHT_NOBITS {
                    continue;
                }

                writer.pad_until(out_section.offset);
                match &section.data {
                    SectionData::Data(data) => {
                        writer.write(data);
                    }
                    SectionData::DynamicRelocation(relocations) => {
                        for rel in relocations {
                            let r_sym = if let Some(symbol) = rel.symbol {
                                out_dynsyms_index[symbol.0].unwrap().0
                            } else {
                                0
                            };
                            writer.write_relocation(
                                section.sh_type == elf::SHT_RELA,
                                &write::elf::Rel {
                                    r_offset: rel.r_offset,
                                    r_sym,
                                    r_type: rel.r_type,
                                    r_addend: rel.r_addend,
                                },
                            );
                        }
                    }
                    SectionData::Note(data) => {
                        writer.write(data);
                    }
                    SectionData::Dynamic(dynamics) => {
                        for d in dynamics {
                            match *d {
                                Dynamic::Auto { tag } => {
                                    // TODO: support more values
                                    let val = match tag {
                                        elf::DT_SYMTAB => dynsym_addr.ok_or(Error::new(
                                            "Missing .dynsym section for DT_SYMTAB",
                                        ))?,
                                        elf::DT_STRTAB => dynstr_addr.ok_or(Error::new(
                                            "Missing .dynstr section for DT_STRTAB",
                                        ))?,
                                        elf::DT_STRSZ => writer.dynstr_len() as u64,
                                        elf::DT_HASH => hash_addr.ok_or(Error::new(
                                            "Missing .hash section for DT_HASH",
                                        ))?,
                                        elf::DT_GNU_HASH => gnu_hash_addr.ok_or(Error::new(
                                            "Missing .gnu.hash section for DT_GNU_HASH",
                                        ))?,
                                        elf::DT_VERSYM => versym_addr.ok_or(Error::new(
                                            "Missing .gnu.version section for DT_VERSYM",
                                        ))?,
                                        elf::DT_VERDEF => verdef_addr.ok_or(Error::new(
                                            "Missing .gnu.version_d section for DT_VERDEF",
                                        ))?,
                                        elf::DT_VERDEFNUM => verdef_count as u64,
                                        elf::DT_VERNEED => verneed_addr.ok_or(Error::new(
                                            "Missing .gnu.version_r section for DT_VERNEED",
                                        ))?,
                                        elf::DT_VERNEEDNUM => verneed_count as u64,
                                        _ => {
                                            return Err(Error(format!(
                                                "Cannot generate value for dynamic tag 0x{:x}",
                                                tag
                                            )))
                                        }
                                    };
                                    writer.write_dynamic(tag, val);
                                }
                                Dynamic::Integer { tag, val } => {
                                    writer.write_dynamic(tag, val);
                                }
                                Dynamic::String { tag, ref val } => {
                                    let val = writer.get_dynamic_string(val);
                                    writer.write_dynamic_string(tag, val);
                                }
                            }
                        }
                        writer.write_dynamic(elf::DT_NULL, 0);
                    }
                    SectionData::DynamicSymbol => {
                        writer.write_null_dynamic_symbol();
                        for out_dynsym in &out_dynsyms {
                            let symbol = self.dynamic_symbols.get(out_dynsym.id);
                            let section =
                                symbol.section.map(|id| out_sections_index[id.0].unwrap());
                            writer.write_dynamic_symbol(&write::elf::Sym {
                                name: out_dynsym.name,
                                section,
                                st_info: symbol.st_info,
                                st_other: symbol.st_other,
                                st_shndx: symbol.st_shndx,
                                st_value: symbol.st_value,
                                st_size: symbol.st_size,
                            });
                        }
                    }
                    SectionData::DynamicString => {
                        writer.write_dynstr();
                    }
                    SectionData::Hash => {
                        if self.hash_bucket_count == 0 {
                            return Err(Error::new(".hash bucket count is zero"));
                        }
                        writer.write_hash(self.hash_bucket_count, hash_chain_count, |index| {
                            out_dynsyms
                                .get(index.checked_sub(hash_index_base)? as usize)?
                                .hash
                        });
                    }
                    SectionData::GnuHash => {
                        if self.gnu_hash_bucket_count == 0 {
                            return Err(Error::new(".gnu.hash bucket count is zero"));
                        }
                        writer.write_gnu_hash(
                            gnu_hash_symbol_base,
                            self.gnu_hash_bloom_shift,
                            self.gnu_hash_bloom_count,
                            self.gnu_hash_bucket_count,
                            gnu_hash_symbol_count,
                            |index| {
                                out_dynsyms[(gnu_hash_index_base + index) as usize]
                                    .gnu_hash
                                    .unwrap()
                            },
                        );
                    }
                    SectionData::GnuVersym => {
                        writer.write_null_gnu_versym();
                        for out_dynsym in &out_dynsyms {
                            let symbol = self.dynamic_symbols.get(out_dynsym.id);
                            let mut index = symbol.version.0 as u16;
                            if symbol.version_hidden {
                                index |= elf::VERSYM_HIDDEN;
                            }
                            writer.write_gnu_versym(index);
                        }
                    }
                    SectionData::GnuVerdef => {
                        writer.write_align_gnu_verdef();
                        if let Some(version_base) = &self.version_base {
                            let verdef = write::elf::Verdef {
                                version: elf::VER_DEF_CURRENT,
                                flags: elf::VER_FLG_BASE,
                                index: 1,
                                aux_count: 1,
                                name: writer.get_dynamic_string(version_base),
                            };
                            if verdef_shared_base {
                                writer.write_gnu_verdef_shared(&verdef);
                            } else {
                                writer.write_gnu_verdef(&verdef);
                            }
                        }
                        for version in &self.versions {
                            if let VersionData::Def(def) = &version.data {
                                let mut names = def.names.iter();
                                let name = names.next().ok_or_else(|| {
                                    Error(format!("Missing SHT_GNU_VERDEF name {}", version.id.0))
                                })?;
                                writer.write_gnu_verdef(&write::elf::Verdef {
                                    version: elf::VER_DEF_CURRENT,
                                    flags: def.flags,
                                    index: version.id.0 as u16,
                                    aux_count: def.names.len() as u16,
                                    name: writer.get_dynamic_string(name),
                                });
                                for name in names {
                                    writer.write_gnu_verdaux(writer.get_dynamic_string(name));
                                }
                            }
                        }
                    }
                    SectionData::GnuVerneed => {
                        writer.write_align_gnu_verneed();
                        for file in &self.version_files {
                            let out_file = &out_version_files[file.id.0];
                            if out_file.versions.is_empty() {
                                continue;
                            }
                            writer.write_gnu_verneed(&write::elf::Verneed {
                                version: elf::VER_NEED_CURRENT,
                                aux_count: out_file.versions.len() as u16,
                                file: writer.get_dynamic_string(&file.name),
                            });
                            for id in &out_file.versions {
                                let version = self.versions.get(*id);
                                // This will always match.
                                if let VersionData::Need(need) = &version.data {
                                    debug_assert_eq!(*id, version.id);
                                    writer.write_gnu_vernaux(&write::elf::Vernaux {
                                        flags: need.flags,
                                        index: version.id.0 as u16,
                                        name: writer.get_dynamic_string(&need.name),
                                    });
                                }
                            }
                        }
                    }
                    _ => {
                        return Err(Error(format!(
                            "Unsupported alloc section type {:x}",
                            section.sh_type
                        )));
                    }
                }
            }
        }

        // Write non-alloc sections.
        for out_section in &mut out_sections {
            let section = self.sections.get(out_section.id);
            if !self.segments.is_empty() && section.is_alloc() {
                continue;
            }
            match &section.data {
                SectionData::Data(data) => {
                    writer.write_align(section.sh_addralign as usize);
                    debug_assert_eq!(out_section.offset, writer.len());
                    writer.write(data);
                }
                SectionData::UninitializedData(_) => {
                    // Nothing to do.
                }
                SectionData::Note(data) => {
                    writer.write_align(section.sh_addralign as usize);
                    debug_assert_eq!(out_section.offset, writer.len());
                    writer.write(data);
                }
                SectionData::Attributes(_) => {
                    writer.write_align(section.sh_addralign as usize);
                    debug_assert_eq!(out_section.offset, writer.len());
                    writer.write(&out_section.attributes);
                }
                // These are handled elsewhere.
                SectionData::Relocation(_)
                | SectionData::SectionString
                | SectionData::Symbol
                | SectionData::SymbolSectionIndex
                | SectionData::String => {}
                _ => {
                    return Err(Error(format!(
                        "Unsupported non-alloc section type {:x}",
                        section.sh_type
                    )));
                }
            }
        }

        writer.write_null_symbol();
        for out_sym in &out_syms {
            let symbol = self.symbols.get(out_sym.id);
            let section = symbol.section.map(|id| out_sections_index[id.0].unwrap());
            writer.write_symbol(&write::elf::Sym {
                name: out_sym.name,
                section,
                st_info: symbol.st_info,
                st_other: symbol.st_other,
                st_shndx: symbol.st_shndx,
                st_value: symbol.st_value,
                st_size: symbol.st_size,
            });
        }
        writer.write_symtab_shndx();
        writer.write_strtab();

        // Write non-alloc relocations.
        for section in &self.sections {
            if !self.segments.is_empty() && section.is_alloc() {
                continue;
            }
            let SectionData::Relocation(relocations) = &section.data else {
                continue;
            };
            writer.write_align_relocation();
            for rel in relocations {
                let r_sym = if let Some(id) = rel.symbol {
                    out_syms_index[id.0].unwrap().0
                } else {
                    0
                };
                writer.write_relocation(
                    section.sh_type == elf::SHT_RELA,
                    &write::elf::Rel {
                        r_offset: rel.r_offset,
                        r_sym,
                        r_type: rel.r_type,
                        r_addend: rel.r_addend,
                    },
                );
            }
        }

        writer.write_shstrtab();

        writer.write_null_section_header();
        for out_section in &out_sections {
            let section = self.sections.get(out_section.id);
            match &section.data {
                SectionData::Data(_)
                | SectionData::UninitializedData(_)
                | SectionData::Relocation(_)
                | SectionData::DynamicRelocation(_)
                | SectionData::Note(_)
                | SectionData::Dynamic(_)
                | SectionData::Attributes(_) => {
                    let sh_size = match &section.data {
                        SectionData::Data(data) => data.len() as u64,
                        SectionData::UninitializedData(len) => *len,
                        SectionData::Relocation(relocations) => {
                            (relocations.len()
                                * self.class().rel_size(section.sh_type == elf::SHT_RELA))
                                as u64
                        }
                        SectionData::DynamicRelocation(relocations) => {
                            (relocations.len()
                                * self.class().rel_size(section.sh_type == elf::SHT_RELA))
                                as u64
                        }
                        SectionData::Note(data) => data.len() as u64,
                        SectionData::Dynamic(dynamics) => {
                            ((1 + dynamics.len()) * self.class().dyn_size()) as u64
                        }
                        SectionData::Attributes(_) => out_section.attributes.len() as u64,
                        _ => {
                            return Err(Error(format!(
                                "Unimplemented size for section type {:x}",
                                section.sh_type
                            )))
                        }
                    };
                    let sh_link = if let Some(id) = section.sh_link_section {
                        if let Some(index) = out_sections_index[id.0] {
                            index.0
                        } else {
                            return Err(Error(format!(
                                "Invalid sh_link from section '{}' to deleted section '{}'",
                                section.name,
                                self.sections.get(id).name,
                            )));
                        }
                    } else {
                        0
                    };
                    let sh_info = if let Some(id) = section.sh_info_section {
                        if let Some(index) = out_sections_index[id.0] {
                            index.0
                        } else {
                            return Err(Error(format!(
                                "Invalid sh_info link from section '{}' to deleted section '{}'",
                                section.name,
                                self.sections.get(id).name,
                            )));
                        }
                    } else {
                        section.sh_info
                    };
                    writer.write_section_header(&write::elf::SectionHeader {
                        name: out_section.name,
                        sh_type: section.sh_type,
                        sh_flags: section.sh_flags,
                        sh_addr: section.sh_addr,
                        sh_offset: out_section.offset as u64,
                        sh_size,
                        sh_link,
                        sh_info,
                        sh_addralign: section.sh_addralign,
                        sh_entsize: section.sh_entsize,
                    });
                }
                SectionData::SectionString => {
                    writer.write_shstrtab_section_header();
                }
                SectionData::Symbol => {
                    writer.write_symtab_section_header(1 + num_local as u32);
                }
                SectionData::SymbolSectionIndex => {
                    writer.write_symtab_shndx_section_header();
                }
                SectionData::String => {
                    writer.write_strtab_section_header();
                }
                SectionData::DynamicString => {
                    writer.write_dynstr_section_header(section.sh_addr);
                }
                SectionData::DynamicSymbol => {
                    writer
                        .write_dynsym_section_header(section.sh_addr, 1 + num_local_dynamic as u32);
                }
                SectionData::Hash => {
                    writer.write_hash_section_header(section.sh_addr);
                }
                SectionData::GnuHash => {
                    writer.write_gnu_hash_section_header(section.sh_addr);
                }
                SectionData::GnuVersym => {
                    writer.write_gnu_versym_section_header(section.sh_addr);
                }
                SectionData::GnuVerdef => {
                    writer.write_gnu_verdef_section_header(section.sh_addr);
                }
                SectionData::GnuVerneed => {
                    writer.write_gnu_verneed_section_header(section.sh_addr);
                }
            }
        }
        debug_assert_eq!(writer.reserved_len(), writer.len());
        Ok(())
    }

    /// Delete segments, symbols, relocations, and dynamics that refer
    /// to deleted items.
    ///
    /// This calls `delete_orphan_segments`, `delete_orphan_symbols`,
    /// `delete_orphan_relocations`, and `delete_orphan_dynamics`.
    pub fn delete_orphans(&mut self) {
        self.delete_orphan_segments();
        self.delete_orphan_symbols();
        self.delete_orphan_relocations();
        self.delete_orphan_dynamics();
    }

    /// Set the delete flag for segments that only refer to deleted sections.
    pub fn delete_orphan_segments(&mut self) {
        let sections = &self.sections;
        for segment in &mut self.segments {
            // We only delete segments that have become empty due to section deletions.
            if segment.sections.is_empty() {
                continue;
            }
            segment.sections.retain(|id| !sections.get(*id).delete);
            segment.delete = segment.sections.is_empty();
        }
    }

    /// Set the delete flag for symbols that refer to deleted sections.
    pub fn delete_orphan_symbols(&mut self) {
        for symbol in &mut self.symbols {
            if let Some(section) = symbol.section {
                if self.sections.get_mut(section).delete {
                    symbol.delete = true;
                }
            }
        }
        for symbol in &mut self.dynamic_symbols {
            if let Some(section) = symbol.section {
                if self.sections.get_mut(section).delete {
                    symbol.delete = true;
                }
            }
        }
    }

    /// Delete relocations that refer to deleted symbols.
    pub fn delete_orphan_relocations(&mut self) {
        let symbols = &self.symbols;
        let dynamic_symbols = &self.dynamic_symbols;
        for section in &mut self.sections {
            match &mut section.data {
                SectionData::Relocation(relocations) => {
                    relocations.retain(|relocation| match relocation.symbol {
                        None => true,
                        Some(id) => !symbols.get(id).delete,
                    });
                }
                SectionData::DynamicRelocation(relocations) => {
                    relocations.retain(|relocation| match relocation.symbol {
                        None => true,
                        Some(id) => !dynamic_symbols.get(id).delete,
                    });
                }
                _ => {}
            }
        }
    }

    /// Delete dynamic entries that refer to deleted sections.
    pub fn delete_orphan_dynamics(&mut self) {
        let mut have_dynsym = false;
        let mut have_dynstr = false;
        let mut have_hash = false;
        let mut have_gnu_hash = false;
        let mut have_versym = false;
        let mut have_verdef = false;
        let mut have_verneed = false;
        for section in &self.sections {
            match &section.data {
                SectionData::DynamicSymbol => have_dynsym = true,
                SectionData::DynamicString => have_dynstr = true,
                SectionData::Hash => have_hash = true,
                SectionData::GnuHash => have_gnu_hash = true,
                SectionData::GnuVersym => have_versym = true,
                SectionData::GnuVerdef => have_verdef = true,
                SectionData::GnuVerneed => have_verneed = true,
                _ => {}
            }
        }
        for section in &mut self.sections {
            if let SectionData::Dynamic(dynamics) = &mut section.data {
                dynamics.retain(|dynamic| match dynamic {
                    Dynamic::Auto {
                        tag: elf::DT_SYMTAB,
                    } => have_dynsym,
                    Dynamic::Auto {
                        tag: elf::DT_STRTAB,
                    }
                    | Dynamic::Auto { tag: elf::DT_STRSZ } => have_dynstr,
                    Dynamic::Auto { tag: elf::DT_HASH } => have_hash,
                    Dynamic::Auto {
                        tag: elf::DT_GNU_HASH,
                    } => have_gnu_hash,
                    Dynamic::Auto {
                        tag: elf::DT_VERSYM,
                    } => have_versym,
                    Dynamic::Auto {
                        tag: elf::DT_VERNEED,
                    }
                    | Dynamic::Auto {
                        tag: elf::DT_VERNEEDNUM,
                    } => have_verneed,
                    Dynamic::Auto {
                        tag: elf::DT_VERDEF,
                    }
                    | Dynamic::Auto {
                        tag: elf::DT_VERDEFNUM,
                    } => have_verdef,
                    _ => true,
                });
            }
        }
    }

    /// Delete unused GNU version entries.
    pub fn delete_unused_versions(&mut self) {
        let mut version_used = vec![false; self.versions.len() + VERSION_ID_BASE];
        for symbol in &self.dynamic_symbols {
            version_used[symbol.version.0] = true;
        }
        let mut version_file_used = vec![false; self.version_files.len()];
        for version in &mut self.versions {
            if let VersionData::Need(need) = &version.data {
                // This is a dummy version that is required if DT_RELR is used.
                if need.name.as_slice() == b"GLIBC_ABI_DT_RELR" {
                    version_used[version.id.0] = true;
                }
            }
            if !version_used[version.id.0] {
                version.delete = true;
                continue;
            }
            if let VersionData::Need(need) = &version.data {
                version_file_used[need.file.0] = true;
            }
        }
        for file in &mut self.version_files {
            if !version_file_used[file.id.0] {
                file.delete = true;
            }
        }
    }

    /// Return the ELF file class that will be written.
    ///
    /// This can be useful for calculating sizes.
    pub fn class(&self) -> write::elf::Class {
        write::elf::Class { is_64: self.is_64 }
    }

    /// Calculate the size of the file header.
    pub fn file_header_size(&self) -> usize {
        self.class().file_header_size()
    }

    /// Calculate the size of the program headers.
    pub fn program_headers_size(&self) -> usize {
        self.segments.count() * self.class().program_header_size()
    }

    /// Calculate the size of the dynamic symbol table.
    ///
    /// To get an accurate result, you may need to first call
    /// [`Self::delete_orphan_symbols`].
    pub fn dynamic_symbol_size(&self) -> usize {
        (1 + self.dynamic_symbols.count()) * self.class().sym_size()
    }

    /// Calculate the size of the dynamic string table.
    ///
    /// This adds all of the currently used dynamic strings to a string table,
    /// calculates the size of the string table, and discards the string table.
    ///
    /// To get an accurate result, you may need to first call
    /// [`Self::delete_orphan_symbols`] and [`Self::delete_unused_versions`].
    pub fn dynamic_string_size(&self) -> usize {
        let mut dynstr = write::string::StringTable::default();
        for section in &self.sections {
            if let SectionData::Dynamic(dynamics) = &section.data {
                for dynamic in dynamics {
                    if let Dynamic::String { val, .. } = dynamic {
                        dynstr.add(val);
                    }
                }
            }
        }
        for symbol in &self.dynamic_symbols {
            dynstr.add(&symbol.name);
        }
        if let Some(version_base) = &self.version_base {
            dynstr.add(version_base);
        }
        for version in &self.versions {
            match &version.data {
                VersionData::Def(def) => {
                    for name in &def.names {
                        dynstr.add(name);
                    }
                }
                VersionData::Need(need) => {
                    dynstr.add(&need.name);
                }
            }
        }
        for file in &self.version_files {
            dynstr.add(&file.name);
        }
        dynstr.size(1)
    }

    /// Calculate the size of the hash table.
    ///
    /// To get an accurate result, you may need to first call
    /// [`Self::delete_orphan_symbols`].
    pub fn hash_size(&self) -> usize {
        let chain_count = 1 + self.dynamic_symbols.count();
        self.class()
            .hash_size(self.hash_bucket_count, chain_count as u32)
    }

    /// Calculate the size of the GNU hash table.
    ///
    /// To get an accurate result, you may need to first call
    /// [`Self::delete_orphan_symbols`].
    pub fn gnu_hash_size(&self) -> usize {
        let symbol_count = self.dynamic_symbols.count_defined();
        self.class().gnu_hash_size(
            self.gnu_hash_bloom_count,
            self.gnu_hash_bucket_count,
            symbol_count as u32,
        )
    }

    /// Calculate the size of the GNU symbol version section.
    ///
    /// To get an accurate result, you may need to first call
    /// [`Self::delete_orphan_symbols`] and [`Self::delete_unused_versions`].
    pub fn gnu_versym_size(&self) -> usize {
        let symbol_count = 1 + self.dynamic_symbols.count();
        self.class().gnu_versym_size(symbol_count)
    }

    /// Calculate the size of the GNU version definition section.
    ///
    /// To get an accurate result, you may need to first call
    /// [`Self::delete_orphan_symbols`] and [`Self::delete_unused_versions`].
    pub fn gnu_verdef_size(&self) -> usize {
        let mut verdef_count = 0;
        let mut verdaux_count = 0;
        if self.version_base.is_some() {
            verdef_count += 1;
            verdaux_count += 1;
        }
        for version in &self.versions {
            if let VersionData::Def(def) = &version.data {
                if !def.is_shared(verdef_count, self.version_base.as_ref()) {
                    verdaux_count += def.names.len();
                }
                verdef_count += 1;
            }
        }
        self.class().gnu_verdef_size(verdef_count, verdaux_count)
    }

    /// Calculate the size of the GNU version dependency section.
    ///
    /// To get an accurate result, you may need to first call
    /// [`Self::delete_orphan_symbols`] and [`Self::delete_unused_versions`].
    pub fn gnu_verneed_size(&self) -> usize {
        let verneed_count = self.version_files.count();
        let mut vernaux_count = 0;
        for version in &self.versions {
            if let VersionData::Need(_) = &version.data {
                vernaux_count += 1;
            }
        }
        self.class().gnu_verneed_size(verneed_count, vernaux_count)
    }

    /// Calculate the memory size of a section.
    ///
    /// Returns 0 for sections that are deleted or aren't allocated.
    ///
    /// To get an accurate result, you may need to first call
    /// [`Self::delete_orphan_symbols`] and [`Self::delete_unused_versions`].
    pub fn section_size(&self, section: &Section<'_>) -> usize {
        if section.delete || !section.is_alloc() {
            return 0;
        }
        match &section.data {
            SectionData::Data(data) => data.len(),
            SectionData::UninitializedData(len) => *len as usize,
            SectionData::Relocation(relocations) => {
                relocations.len() * self.class().rel_size(section.sh_type == elf::SHT_RELA)
            }
            SectionData::DynamicRelocation(relocations) => {
                relocations.len() * self.class().rel_size(section.sh_type == elf::SHT_RELA)
            }
            SectionData::Note(data) => data.len(),
            SectionData::Dynamic(dynamics) => (1 + dynamics.len()) * self.class().dyn_size(),
            SectionData::DynamicString => self.dynamic_string_size(),
            SectionData::DynamicSymbol => self.dynamic_symbol_size(),
            SectionData::Hash => self.hash_size(),
            SectionData::GnuHash => self.gnu_hash_size(),
            SectionData::GnuVersym => self.gnu_versym_size(),
            SectionData::GnuVerdef => self.gnu_verdef_size(),
            SectionData::GnuVerneed => self.gnu_verneed_size(),
            // None of these should be allocated.
            SectionData::SectionString
            | SectionData::Symbol
            | SectionData::SymbolSectionIndex
            | SectionData::String
            | SectionData::Attributes(_) => 0,
        }
    }

    /// Set the `sh_size` field for every allocated section.
    ///
    /// This is useful to call prior to doing memory layout.
    ///
    /// To get an accurate result, you may need to first call
    /// [`Self::delete_orphan_symbols`] and [`Self::delete_unused_versions`].
    pub fn set_section_sizes(&mut self) {
        for id in (0..self.sections.len()).map(SectionId) {
            let section = self.sections.get(id);
            if section.delete || !section.is_alloc() {
                continue;
            }
            self.sections.get_mut(id).sh_size = self.section_size(section) as u64;
        }
    }

    /// Find the section containing the dynamic table.
    ///
    /// This uses the `PT_DYNAMIC` program header to find the dynamic section.
    pub fn dynamic_section(&self) -> Option<SectionId> {
        let segment = self
            .segments
            .iter()
            .find(|segment| segment.p_type == elf::PT_DYNAMIC)?;
        // TODO: handle multiple sections in the segment?
        segment.sections.iter().copied().next()
    }

    /// Find the dynamic table entries.
    ///
    /// This uses the `PT_DYNAMIC` program header to find the dynamic section,
    pub fn dynamic_data(&self) -> Option<&[Dynamic<'data>]> {
        let section = self.dynamic_section()?;
        match &self.sections.get(section).data {
            SectionData::Dynamic(dynamics) => Some(dynamics),
            _ => None,
        }
    }

    /// Find the dynamic table entries.
    ///
    /// This uses the `PT_DYNAMIC` program header to find the dynamic section,
    pub fn dynamic_data_mut(&mut self) -> Option<&mut Vec<Dynamic<'data>>> {
        let section = self.dynamic_section()?;
        match &mut self.sections.get_mut(section).data {
            SectionData::Dynamic(dynamics) => Some(dynamics),
            _ => None,
        }
    }

    /// Find the section containing the interpreter path.
    ///
    /// This uses the `PT_INTERP` program header to find the interp section.
    pub fn interp_section(&self) -> Option<SectionId> {
        let segment = self
            .segments
            .iter()
            .find(|segment| segment.p_type == elf::PT_INTERP)?;
        // TODO: handle multiple sections in the segment?
        segment.sections.iter().copied().next()
    }

    /// Find the interpreter path.
    ///
    /// This uses the `PT_INTERP` program header to find the interp section.
    pub fn interp_data(&self) -> Option<&[u8]> {
        let section = self.interp_section()?;
        match &self.sections.get(section).data {
            SectionData::Data(data) => Some(data),
            _ => None,
        }
    }

    /// Find the interpreter path.
    ///
    /// This uses the `PT_INTERP` program header to find the interp section.
    pub fn interp_data_mut(&mut self) -> Option<&mut Bytes<'data>> {
        let section = self.interp_section()?;
        match &mut self.sections.get_mut(section).data {
            SectionData::Data(data) => Some(data),
            _ => None,
        }
    }
}

/// ELF file header.
///
/// This corresponds to fields in [`elf::FileHeader32`] or [`elf::FileHeader64`].
/// This only contains the ELF file header fields that can be modified.
/// The other fields are automatically calculated.
#[derive(Debug, Default)]
pub struct Header {
    /// The OS ABI field in the file header.
    ///
    /// One of the `ELFOSABI*` constants.
    pub os_abi: u8,
    /// The ABI version field in the file header.
    ///
    /// The meaning of this field depends on the `os_abi` value.
    pub abi_version: u8,
    /// The object file type in the file header.
    ///
    /// One of the `ET_*` constants.
    pub e_type: u16,
    /// The architecture in the file header.
    ///
    /// One of the `EM_*` constants.
    pub e_machine: u16,
    /// Entry point virtual address in the file header.
    pub e_entry: u64,
    /// The processor-specific flags in the file header.
    ///
    /// A combination of the `EF_*` constants.
    pub e_flags: u32,
    /// The file offset of the program header table.
    ///
    /// Writing will fail if the program header table cannot be placed at this offset.
    pub e_phoff: u64,
}

/// An ID for referring to a segment in [`Segments`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SegmentId(usize);

impl fmt::Debug for SegmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SegmentId({})", self.0)
    }
}

impl Id for SegmentId {
    fn index(&self) -> usize {
        self.0
    }
}

impl IdPrivate for SegmentId {
    fn new(id: usize) -> Self {
        SegmentId(id)
    }
}

/// A segment in [`Segments`].
///
/// This corresponds to [`elf::ProgramHeader32`] or [`elf::ProgramHeader64`].
#[derive(Debug)]
pub struct Segment<'data> {
    id: SegmentId,
    /// Ignore this segment when writing the ELF file.
    pub delete: bool,
    /// The `p_type` field in the ELF program header.
    ///
    /// One of the `PT_*` constants.
    pub p_type: u32,
    /// The `p_flags` field in the ELF program header.
    ///
    /// A combination of the `PF_*` constants.
    pub p_flags: u32,
    /// The `p_offset` field in the ELF program header.
    ///
    /// This is the file offset of the data in the segment. This should
    /// correspond to the file offset of the sections that are placed in
    /// this segment. Currently there is no support for section data
    /// that is not contained in sections.
    pub p_offset: u64,
    /// The `p_vaddr` field in the ELF program header.
    pub p_vaddr: u64,
    /// The `p_paddr` field in the ELF program header.
    pub p_paddr: u64,
    /// The `p_filesz` field in the ELF program header.
    pub p_filesz: u64,
    /// The `p_memsz` field in the ELF program header.
    pub p_memsz: u64,
    /// The `p_align` field in the ELF program header.
    pub p_align: u64,
    /// The sections contained in this segment.
    pub sections: Vec<SectionId>,
    // Might need to add reference to data if no sections.
    marker: PhantomData<&'data ()>,
}

impl<'data> Item for Segment<'data> {
    type Id = SegmentId;

    fn is_deleted(&self) -> bool {
        self.delete
    }
}

impl<'data> Segment<'data> {
    /// The ID used for referring to this segment.
    pub fn id(&self) -> SegmentId {
        self.id
    }

    /// Returns true if the segment type is `PT_LOAD`.
    pub fn is_load(&self) -> bool {
        self.p_type == elf::PT_LOAD
    }

    /// Returns true if the segment contains the given file offset.
    pub fn contains_offset(&self, offset: u64) -> bool {
        offset >= self.p_offset && offset - self.p_offset < self.p_filesz
    }

    /// Return the address corresponding to the given file offset.
    ///
    /// This will return a meaningless value if `contains_offset` is false.
    pub fn address_from_offset(&self, offset: u64) -> u64 {
        self.p_vaddr
            .wrapping_add(offset.wrapping_sub(self.p_offset))
    }

    /// Returns true if the segment contains the given address.
    pub fn contains_address(&self, address: u64) -> bool {
        address >= self.p_vaddr && address - self.p_vaddr < self.p_memsz
    }

    /// Remove all sections from the segment, and set its size to zero.
    pub fn remove_sections(&mut self) {
        self.p_filesz = 0;
        self.p_memsz = 0;
        self.sections.clear();
    }

    /// Add a section to the segment.
    ///
    /// If this is a [`elf::PT_LOAD`] segment, then the file offset and address of the
    /// section is changed to be at the end of the segment.
    ///
    /// The segment's file and address ranges are extended to include the section.
    /// This uses the `sh_size` field of the section, not the size of the section data.
    ///
    /// The section's id is added to the segment's list of sections.
    pub fn append_section(&mut self, section: &mut Section<'_>) {
        debug_assert_eq!(self.p_filesz, self.p_memsz);
        if self.p_type == elf::PT_LOAD {
            let align = section.sh_addralign;
            let offset = (self.p_offset + self.p_filesz + (align - 1)) & !(align - 1);
            let addr = (self.p_paddr + self.p_memsz + (align - 1)) & !(align - 1);
            section.sh_offset = offset;
            section.sh_addr = addr;
        }
        self.append_section_range(section);
        self.sections.push(section.id);
    }

    /// Extend this segment's file and address ranges to include the given section.
    ///
    /// If the segment's `p_memsz` is zero, then this signifies that the segment
    /// has no file or address range yet. In this case, the segment's file and address
    /// ranges are set equal to the section. Otherwise, the segment's file and address
    /// ranges are extended to include the section.
    ///
    /// This uses the `sh_size` field of the section, not the size of the section data.
    pub fn append_section_range(&mut self, section: &Section<'_>) {
        let section_filesize = if section.sh_type == elf::SHT_NOBITS {
            0
        } else {
            section.sh_size
        };
        if self.p_memsz == 0 {
            self.p_offset = section.sh_offset;
            self.p_filesz = section_filesize;
            self.p_vaddr = section.sh_addr;
            self.p_paddr = section.sh_addr;
            self.p_memsz = section.sh_size;
        } else {
            if self.p_offset > section.sh_offset {
                self.p_offset = section.sh_offset;
            }
            let filesz = section.sh_offset + section_filesize - self.p_offset;
            if self.p_filesz < filesz {
                self.p_filesz = filesz;
            }
            if self.p_vaddr > section.sh_addr {
                self.p_vaddr = section.sh_addr;
                self.p_paddr = section.sh_addr;
            }
            let memsz = section.sh_addr + section.sh_size - self.p_vaddr;
            if self.p_memsz < memsz {
                self.p_memsz = memsz;
            }
        }
    }

    /// Recalculate the file and address ranges of the segment.
    ///
    /// Resets the segment's file and address ranges to zero, and then
    /// calls `append_section_range` for each section in the segment.
    pub fn recalculate_ranges(&mut self, sections: &Sections<'data>) {
        self.p_offset = 0;
        self.p_filesz = 0;
        self.p_vaddr = 0;
        self.p_paddr = 0;
        self.p_memsz = 0;
        let ids = core::mem::take(&mut self.sections);
        for id in &ids {
            let section = sections.get(*id);
            self.append_section_range(section);
        }
        self.sections = ids;
    }
}

/// A segment table.
pub type Segments<'data> = Table<Segment<'data>>;

impl<'data> Segments<'data> {
    /// Add a new segment to the table.
    pub fn add(&mut self) -> &mut Segment<'data> {
        let id = self.next_id();
        self.push(Segment {
            id,
            delete: false,
            p_type: 0,
            p_flags: 0,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0,
            sections: Vec::new(),
            marker: PhantomData,
        });
        self.get_mut(id)
    }

    /// Find a `PT_LOAD` segment containing the given offset.
    pub fn find_load_segment_from_offset(&self, offset: u64) -> Option<&Segment<'data>> {
        self.iter()
            .find(|segment| segment.is_load() && segment.contains_offset(offset))
    }

    /// Add a new `PT_LOAD` segment to the table.
    ///
    /// The file offset and address will be derived from the current maximum for any segment.
    /// The address will be chosen so that `p_paddr % align == p_offset % align`.
    /// You may wish to use [`Builder::load_align`] for the alignment.
    pub fn add_load_segment(&mut self, flags: u32, align: u64) -> &mut Segment<'data> {
        let mut max_offset = 0;
        let mut max_addr = 0;
        for segment in &*self {
            let offset = segment.p_offset + segment.p_filesz;
            if max_offset < offset {
                max_offset = offset;
            }
            let addr = segment.p_vaddr + segment.p_memsz;
            if max_addr < addr {
                max_addr = addr;
            }
        }
        // No alignment is required for the segment file offset because sections
        // will add their alignment to the file offset when they are added.
        let offset = max_offset;
        // The address must be chosen so that addr % align == offset % align.
        let addr = ((max_addr + (align - 1)) & !(align - 1)) + (offset & (align - 1));

        let segment = self.add();
        segment.p_type = elf::PT_LOAD;
        segment.p_flags = flags;
        segment.p_offset = offset;
        segment.p_vaddr = addr;
        segment.p_paddr = addr;
        segment.p_align = align;
        segment
    }

    /// Add a copy of a segment to the table.
    ///
    /// This will copy the segment type, flags and alignment.
    ///
    /// Additionally, if the segment type is `PT_LOAD`, then the file offset and address
    /// will be set as in `add_load_segment`.
    pub fn copy(&mut self, id: SegmentId) -> &mut Segment<'data> {
        let segment = self.get(id);
        let p_type = segment.p_type;
        let p_flags = segment.p_flags;
        let p_align = segment.p_align;
        if p_type == elf::PT_LOAD {
            self.add_load_segment(p_flags, p_align)
        } else {
            let segment = self.add();
            segment.p_type = p_type;
            segment.p_flags = p_flags;
            segment.p_align = p_align;
            segment
        }
    }
}

/// An ID for referring to a section in [`Sections`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SectionId(usize);

impl fmt::Debug for SectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SectionId({})", self.0)
    }
}

impl Id for SectionId {
    fn index(&self) -> usize {
        self.0
    }
}

impl IdPrivate for SectionId {
    fn new(id: usize) -> Self {
        SectionId(id)
    }
}

/// A section in [`Sections`].
///
/// This corresponds to [`elf::SectionHeader32`] or [`elf::SectionHeader64`].
#[derive(Debug)]
pub struct Section<'data> {
    id: SectionId,
    /// Ignore this section when writing the ELF file.
    pub delete: bool,
    /// The name of the section.
    ///
    /// This is automatically added to the section header string table,
    /// and the resulting string table offset is used to set the `sh_name`
    /// field in the ELF section header.
    pub name: ByteString<'data>,
    /// The `sh_type` field in the ELF section header.
    ///
    /// One of the `SHT_*` constants.
    pub sh_type: u32,
    /// The `sh_flags` field in the ELF section header.
    ///
    /// A combination of the `SHF_*` constants.
    pub sh_flags: u64,
    /// The `sh_addr` field in the ELF section header.
    pub sh_addr: u64,
    /// The `sh_offset` field in the ELF section header.
    ///
    /// This is the file offset of the data in the section.
    /// Writing will fail if the data cannot be placed at this offset.
    ///
    /// This is only used for sections that have `SHF_ALLOC` set.
    /// For other sections, the section data is written at the next available
    /// offset.
    pub sh_offset: u64,
    /// The `sh_size` field in the ELF section header.
    ///
    /// This size is not used when writing. The size of the `data` field is
    /// used instead.
    pub sh_size: u64,
    /// The ID of the section linked to by the `sh_link` field in the ELF section header.
    pub sh_link_section: Option<SectionId>,
    /// The `sh_info` field in the ELF section header.
    ///
    /// Only used if `sh_info_section` is `None`.
    pub sh_info: u32,
    /// The ID of the section linked to by the `sh_info` field in the ELF section header.
    pub sh_info_section: Option<SectionId>,
    /// The `sh_addralign` field in the ELF section header.
    pub sh_addralign: u64,
    /// The `sh_entsize` field in the ELF section header.
    pub sh_entsize: u64,
    /// The section data.
    pub data: SectionData<'data>,
}

impl<'data> Item for Section<'data> {
    type Id = SectionId;

    fn is_deleted(&self) -> bool {
        self.delete
    }
}

impl<'data> Section<'data> {
    /// The ID used for referring to this section.
    pub fn id(&self) -> SectionId {
        self.id
    }

    /// Returns true if the section flags include `SHF_ALLOC`.
    pub fn is_alloc(&self) -> bool {
        self.sh_flags & u64::from(elf::SHF_ALLOC) != 0
    }

    /// Return the segment permission flags that are equivalent to the section flags.
    pub fn p_flags(&self) -> u32 {
        let mut p_flags = elf::PF_R;
        if self.sh_flags & u64::from(elf::SHF_WRITE) != 0 {
            p_flags |= elf::PF_W;
        }
        if self.sh_flags & u64::from(elf::SHF_EXECINSTR) != 0 {
            p_flags |= elf::PF_X;
        }
        p_flags
    }
}

/// The data for a [`Section`].
#[derive(Debug, Clone)]
pub enum SectionData<'data> {
    /// The section contains the given raw data bytes.
    Data(Bytes<'data>),
    /// The section contains uninitialised data bytes of the given length.
    UninitializedData(u64),
    /// The section contains relocations.
    Relocation(Vec<Relocation>),
    /// The section contains dynamic relocations.
    DynamicRelocation(Vec<DynamicRelocation>),
    /// The section contains notes.
    // TODO: parse notes
    Note(Bytes<'data>),
    /// The section contains dynamic entries.
    Dynamic(Vec<Dynamic<'data>>),
    /// The section contains attributes.
    ///
    /// This may be GNU attributes or other vendor-specific attributes.
    Attributes(AttributesSection<'data>),
    /// The section contains the strings for the section headers.
    SectionString,
    /// The section contains the symbol table.
    Symbol,
    /// The section contains the extended section index for the symbol table.
    SymbolSectionIndex,
    /// The section contains the strings for symbol table.
    String,
    /// The section contains the dynamic symbol table.
    DynamicSymbol,
    /// The section contains the dynamic string table.
    DynamicString,
    /// The section contains the hash table.
    Hash,
    /// The section contains the GNU hash table.
    GnuHash,
    /// The section contains the GNU symbol versions.
    GnuVersym,
    /// The section contains the GNU version definitions.
    GnuVerdef,
    /// The section contains the GNU version dependencies.
    GnuVerneed,
}

/// A section table.
pub type Sections<'data> = Table<Section<'data>>;

impl<'data> Sections<'data> {
    /// Add a new section to the table.
    pub fn add(&mut self) -> &mut Section<'data> {
        let id = self.next_id();
        self.push(Section {
            id,
            delete: false,
            name: ByteString::default(),
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link_section: None,
            sh_info: 0,
            sh_info_section: None,
            sh_addralign: 0,
            sh_entsize: 0,
            data: SectionData::Data(Bytes::default()),
        })
    }

    /// Add a copy of a section to the table.
    ///
    /// This will set the file offset of the copy to zero.
    /// [`Segment::append_section`] can be used to assign a valid file offset and a new address.
    pub fn copy(&mut self, id: SectionId) -> &mut Section<'data> {
        let section = self.get(id);
        let id = self.next_id();
        let name = section.name.clone();
        let sh_type = section.sh_type;
        let sh_flags = section.sh_flags;
        let sh_addr = section.sh_addr;
        let sh_size = section.sh_size;
        let sh_link_section = section.sh_link_section;
        let sh_info = section.sh_info;
        let sh_info_section = section.sh_info_section;
        let sh_addralign = section.sh_addralign;
        let sh_entsize = section.sh_entsize;
        let data = section.data.clone();
        self.push(Section {
            id,
            delete: false,
            name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset: 0,
            sh_size,
            sh_link_section,
            sh_info,
            sh_info_section,
            sh_addralign,
            sh_entsize,
            data,
        })
    }
}

/// An ID for referring to a symbol in [`Symbols`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SymbolId<const DYNAMIC: bool = false>(usize);

impl<const DYNAMIC: bool> fmt::Debug for SymbolId<DYNAMIC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const DYNAMIC: bool> Id for SymbolId<DYNAMIC> {
    fn index(&self) -> usize {
        self.0
    }
}

impl<const DYNAMIC: bool> IdPrivate for SymbolId<DYNAMIC> {
    fn new(id: usize) -> Self {
        SymbolId(id)
    }
}

/// A symbol in [`Symbols`].
///
/// This corresponds to [`elf::Sym32`] or [`elf::Sym64`].
#[derive(Debug)]
pub struct Symbol<'data, const DYNAMIC: bool = false> {
    id: SymbolId<DYNAMIC>,
    /// Ignore this symbol when writing the ELF file.
    pub delete: bool,
    /// The name of the symbol.
    pub name: ByteString<'data>,
    /// The section referenced by the symbol.
    ///
    /// Used to set the `st_shndx` field in the ELF symbol.
    pub section: Option<SectionId>,
    /// The `st_info` field in the ELF symbol.
    pub st_info: u8,
    /// The `st_other` field in the ELF symbol.
    pub st_other: u8,
    /// The `st_shndx` field in the ELF symbol.
    ///
    /// Only used if `Self::section` is `None`.
    pub st_shndx: u16,
    /// The `st_value` field in the ELF symbol.
    pub st_value: u64,
    /// The `st_size` field in the ELF symbol.
    pub st_size: u64,
    /// GNU version for dynamic symbols.
    pub version: VersionId,
    /// Set the [`elf::VERSYM_HIDDEN`] flag for this symbol.
    pub version_hidden: bool,
}

impl<'data, const DYNAMIC: bool> Item for Symbol<'data, DYNAMIC> {
    type Id = SymbolId<DYNAMIC>;

    fn is_deleted(&self) -> bool {
        self.delete
    }
}

impl<'data, const DYNAMIC: bool> Symbol<'data, DYNAMIC> {
    /// The ID used for referring to this symbol.
    pub fn id(&self) -> SymbolId<DYNAMIC> {
        self.id
    }

    /// Get the `st_bind` component of the `st_info` field.
    #[inline]
    pub fn st_bind(&self) -> u8 {
        self.st_info >> 4
    }

    /// Get the `st_type` component of the `st_info` field.
    #[inline]
    pub fn st_type(&self) -> u8 {
        self.st_info & 0xf
    }

    /// Set the `st_info` field given the `st_bind` and `st_type` components.
    #[inline]
    pub fn set_st_info(&mut self, st_bind: u8, st_type: u8) {
        self.st_info = (st_bind << 4) + (st_type & 0xf);
    }
}

/// A symbol table.
pub type Symbols<'data, const DYNAMIC: bool = false> = Table<Symbol<'data, DYNAMIC>>;

impl<'data, const DYNAMIC: bool> Symbols<'data, DYNAMIC> {
    /// Number of defined symbols.
    pub fn count_defined(&self) -> usize {
        self.into_iter()
            .filter(|symbol| symbol.st_shndx != elf::SHN_UNDEF)
            .count()
    }

    /// Add a new symbol to the table.
    pub fn add(&mut self) -> &mut Symbol<'data, DYNAMIC> {
        let id = self.next_id();
        self.push(Symbol {
            id,
            delete: false,
            name: ByteString::default(),
            section: None,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
            st_value: 0,
            st_size: 0,
            version: VersionId::local(),
            version_hidden: false,
        })
    }
}

/// A relocation stored in a [`Section`].
///
/// This corresponds to [`elf::Rel32`], [`elf::Rela32`], [`elf::Rel64`] or [`elf::Rela64`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Relocation<const DYNAMIC: bool = false> {
    /// The `r_offset` field in the ELF relocation.
    pub r_offset: u64,
    /// The symbol referenced by the ELF relocation.
    pub symbol: Option<SymbolId<DYNAMIC>>,
    /// The `r_type` field in the ELF relocation.
    pub r_type: u32,
    /// The `r_addend` field in the ELF relocation.
    ///
    /// Only used if the section type is `SHT_RELA`.
    pub r_addend: i64,
}

/// A dynamic symbol ID.
pub type DynamicSymbolId = SymbolId<true>;

/// A dynamic symbol.
pub type DynamicSymbol<'data> = Symbol<'data, true>;

/// A dynamic symbol table.
pub type DynamicSymbols<'data> = Symbols<'data, true>;

/// A dynamic relocation.
pub type DynamicRelocation = Relocation<true>;

/// An entry in the dynamic section.
///
/// This corresponds to [`elf::Dyn32`] or [`elf::Dyn64`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dynamic<'data> {
    /// The value is an automatically generated integer.
    ///
    /// Writing will fail if the value cannot be automatically generated.
    Auto {
        /// The `d_tag` field in the dynamic entry.
        ///
        /// One of the `DT_*` values.
        tag: u32,
    },
    /// The value is an integer.
    Integer {
        /// The `d_tag` field in the dynamic entry.
        ///
        /// One of the `DT_*` values.
        tag: u32,
        /// The `d_val` field in the dynamic entry.
        val: u64,
    },
    /// The value is a string.
    String {
        /// The `d_tag` field in the dynamic entry.
        ///
        /// One of the `DT_*` values.
        tag: u32,
        /// The string value.
        ///
        /// This will be stored in the dynamic string section.
        val: ByteString<'data>,
    },
}

impl<'data> Dynamic<'data> {
    /// The `d_tag` field in the dynamic entry.
    ///
    /// One of the `DT_*` values.
    pub fn tag(&self) -> u32 {
        match self {
            Dynamic::Auto { tag } => *tag,
            Dynamic::Integer { tag, .. } => *tag,
            Dynamic::String { tag, .. } => *tag,
        }
    }
}

/// An ID for referring to a filename in [`VersionFiles`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VersionFileId(usize);

impl fmt::Debug for VersionFileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VersionFileId({})", self.0)
    }
}

impl Id for VersionFileId {
    fn index(&self) -> usize {
        self.0
    }
}

impl IdPrivate for VersionFileId {
    fn new(id: usize) -> Self {
        VersionFileId(id)
    }
}

/// A filename used for GNU versioning.
///
/// Stored in [`VersionFiles`].
#[derive(Debug)]
pub struct VersionFile<'data> {
    id: VersionFileId,
    /// Ignore this file when writing the ELF file.
    pub delete: bool,
    /// The filename.
    pub name: ByteString<'data>,
}

impl<'data> Item for VersionFile<'data> {
    type Id = VersionFileId;

    fn is_deleted(&self) -> bool {
        self.delete
    }
}

impl<'data> VersionFile<'data> {
    /// The ID used for referring to this filename.
    pub fn id(&self) -> VersionFileId {
        self.id
    }
}

/// A table of filenames used for GNU versioning.
pub type VersionFiles<'data> = Table<VersionFile<'data>>;

impl<'data> VersionFiles<'data> {
    /// Add a new filename to the table.
    pub fn add(&mut self, name: ByteString<'data>) -> VersionFileId {
        let id = self.next_id();
        self.push(VersionFile {
            id,
            name,
            delete: false,
        });
        id
    }
}

const VERSION_ID_BASE: usize = 2;

/// An ID for referring to a version in [`Versions`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VersionId(usize);

impl fmt::Debug for VersionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VersionId({})", self.0)
    }
}

impl Id for VersionId {
    fn index(&self) -> usize {
        self.0 - VERSION_ID_BASE
    }
}

impl IdPrivate for VersionId {
    fn new(id: usize) -> Self {
        VersionId(VERSION_ID_BASE + id)
    }
}

impl VersionId {
    /// Return `True` if this is a special version that does not exist in the version table.
    pub fn is_special(&self) -> bool {
        self.0 < VERSION_ID_BASE
    }

    /// Return the ID for a version index of [`elf::VER_NDX_LOCAL`].
    pub fn local() -> Self {
        VersionId(elf::VER_NDX_LOCAL as usize)
    }

    /// Return the ID for a version index of [`elf::VER_NDX_GLOBAL`].
    pub fn global() -> Self {
        VersionId(elf::VER_NDX_GLOBAL as usize)
    }
}

/// A version for a symbol.
#[derive(Debug)]
pub struct Version<'data> {
    id: VersionId,
    /// The data for this version.
    pub data: VersionData<'data>,
    /// Ignore this version when writing the ELF file.
    pub delete: bool,
}

impl<'data> Item for Version<'data> {
    type Id = VersionId;

    fn is_deleted(&self) -> bool {
        self.delete
    }
}

impl<'data> Version<'data> {
    /// The ID used for referring to this version.
    pub fn id(&self) -> VersionId {
        self.id
    }
}

/// The data for a version for a symbol.
#[derive(Debug)]
pub enum VersionData<'data> {
    /// The version for a defined symbol.
    Def(VersionDef<'data>),
    /// The version for an undefined symbol.
    Need(VersionNeed<'data>),
}

/// A GNU version definition.
#[derive(Debug)]
pub struct VersionDef<'data> {
    /// The names for the version.
    ///
    /// This usually has two elements. The first element is the name of this
    /// version, and the second element is the name of the previous version
    /// in the tree of versions.
    pub names: Vec<ByteString<'data>>,
    /// The version flags.
    ///
    /// A combination of the `VER_FLG_*` constants.
    pub flags: u16,
}

impl<'data> VersionDef<'data> {
    /// Optimise for the common case where the first version is the same as the base version.
    fn is_shared(&self, index: usize, base: Option<&ByteString<'_>>) -> bool {
        index == 1 && self.names.len() == 1 && self.names.first() == base
    }
}

/// A GNU version dependency.
#[derive(Debug)]
pub struct VersionNeed<'data> {
    /// The filename of the library providing this version.
    pub file: VersionFileId,
    /// The name of the version.
    pub name: ByteString<'data>,
    /// The version flags.
    ///
    /// A combination of the `VER_FLG_*` constants.
    pub flags: u16,
}

/// A table of versions that are referenced by symbols.
pub type Versions<'data> = Table<Version<'data>>;

impl<'data> Versions<'data> {
    /// Add a version.
    pub fn add(&mut self, data: VersionData<'data>) -> VersionId {
        let id = self.next_id();
        self.push(Version {
            id,
            data,
            delete: false,
        });
        id
    }
}

/// The contents of an attributes section.
#[derive(Debug, Default, Clone)]
pub struct AttributesSection<'data> {
    /// The subsections.
    pub subsections: Vec<AttributesSubsection<'data>>,
}

impl<'data> AttributesSection<'data> {
    /// Create a new attributes section.
    pub fn new() -> Self {
        Self::default()
    }
}

/// A subsection of an attributes section.
#[derive(Debug, Clone)]
pub struct AttributesSubsection<'data> {
    /// The vendor namespace for these attributes.
    pub vendor: ByteString<'data>,
    /// The sub-subsections.
    pub subsubsections: Vec<AttributesSubsubsection<'data>>,
}

impl<'data> AttributesSubsection<'data> {
    /// Create a new subsection.
    pub fn new(vendor: ByteString<'data>) -> Self {
        AttributesSubsection {
            vendor,
            subsubsections: Vec::new(),
        }
    }
}

/// A sub-subsection in an attributes section.
#[derive(Debug, Clone)]
pub struct AttributesSubsubsection<'data> {
    /// The sub-subsection tag.
    pub tag: AttributeTag,
    /// The data containing the attributes.
    pub data: Bytes<'data>,
}

/// The tag for a sub-subsection in an attributes section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeTag {
    /// The attributes apply to the whole file.
    ///
    /// Correspeonds to [`elf::Tag_File`].
    File,
    /// The attributes apply to the given sections.
    ///
    /// Correspeonds to [`elf::Tag_Section`].
    Section(Vec<SectionId>),
    /// The attributes apply to the given symbols.
    ///
    /// Correspeonds to [`elf::Tag_Symbol`].
    Symbol(Vec<SymbolId>),
}

impl AttributeTag {
    /// Return the corresponding `elf::Tag_*` value for this tag.
    pub fn tag(&self) -> u8 {
        match self {
            AttributeTag::File => elf::Tag_File,
            AttributeTag::Section(_) => elf::Tag_Section,
            AttributeTag::Symbol(_) => elf::Tag_Symbol,
        }
    }
}
