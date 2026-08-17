//! Support for reading Wasm files.
//!
//! [`WasmFile`] implements the [`Object`] trait for Wasm files.
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::Range;
use core::{slice, str};
use wasmparser as wp;

use crate::read::{
    self, Architecture, ComdatKind, CompressedData, CompressedFileRange, Error, Export, FileFlags,
    Import, NoDynamicRelocationIterator, Object, ObjectComdat, ObjectKind, ObjectSection,
    ObjectSegment, ObjectSymbol, ObjectSymbolTable, ReadError, ReadRef, Relocation, RelocationMap,
    Result, SectionFlags, SectionIndex, SectionKind, SegmentFlags, SymbolFlags, SymbolIndex,
    SymbolKind, SymbolScope, SymbolSection,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
enum SectionId {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12,
    Tag = 13,
}
// Update this constant when adding new section id:
const MAX_SECTION_ID: usize = SectionId::Tag as usize;

/// A WebAssembly object file.
#[derive(Debug)]
pub struct WasmFile<'data, R = &'data [u8]> {
    data: &'data [u8],
    has_memory64: bool,
    // All sections, including custom sections.
    sections: Vec<SectionHeader<'data>>,
    // Indices into `sections` of sections with a non-zero id.
    id_sections: Box<[Option<usize>; MAX_SECTION_ID + 1]>,
    // Whether the file has DWARF information.
    has_debug_symbols: bool,
    // Symbols collected from imports, exports, code and name sections.
    symbols: Vec<WasmSymbolInternal<'data>>,
    // Address of the function body for the entry point.
    entry: u64,
    marker: PhantomData<R>,
}

#[derive(Debug)]
struct SectionHeader<'data> {
    id: SectionId,
    range: Range<usize>,
    name: &'data str,
}

#[derive(Clone)]
enum LocalFunctionKind {
    Unknown,
    Exported,
}

impl<T> ReadError<T> for wasmparser::Result<T> {
    fn read_error(self, error: &'static str) -> Result<T> {
        self.map_err(|_| Error(error))
    }
}

impl<'data, R: ReadRef<'data>> WasmFile<'data, R> {
    /// Parse the raw wasm data.
    pub fn parse(data: R) -> Result<Self> {
        let len = data.len().read_error("Unknown Wasm file size")?;
        let data = data.read_bytes_at(0, len).read_error("Wasm read failed")?;
        let parser = wp::Parser::new(0).parse_all(data);

        let mut file = WasmFile {
            data,
            has_memory64: false,
            sections: Vec::new(),
            id_sections: Default::default(),
            has_debug_symbols: false,
            symbols: Vec::new(),
            entry: 0,
            marker: PhantomData,
        };

        let mut main_file_symbol = Some(WasmSymbolInternal {
            name: "",
            address: 0,
            size: 0,
            kind: SymbolKind::File,
            section: SymbolSection::None,
            scope: SymbolScope::Compilation,
            weak: false,
        });

        let mut local_func_kinds = Vec::new();
        let mut entry_func_id = None;
        let mut code_range_start = 0;
        let mut code_ranges = Vec::new();
        let mut imports = None;
        let mut exports = None;
        let mut names = None;
        let mut symbols = None;
        // One-to-one mapping of globals to their value (if the global is a constant integer).
        let mut global_values = Vec::new();

        for payload in parser {
            let payload = payload.read_error("Invalid Wasm section header")?;

            match payload {
                wp::Payload::Version { encoding, .. } => {
                    if encoding != wp::Encoding::Module {
                        return Err(Error("Unsupported Wasm encoding"));
                    }
                }
                wp::Payload::TypeSection(section) => {
                    file.add_section(SectionId::Type, section.range(), "");
                }
                wp::Payload::ImportSection(section) => {
                    file.add_section(SectionId::Import, section.range(), "");
                    imports = Some(section);
                }
                wp::Payload::FunctionSection(section) => {
                    file.add_section(SectionId::Function, section.range(), "");
                    local_func_kinds =
                        vec![LocalFunctionKind::Unknown; section.into_iter().count()];
                }
                wp::Payload::TableSection(section) => {
                    file.add_section(SectionId::Table, section.range(), "");
                }
                wp::Payload::MemorySection(section) => {
                    file.add_section(SectionId::Memory, section.range(), "");
                    for memory in section {
                        let memory = memory.read_error("Couldn't read a memory item")?;
                        file.has_memory64 |= memory.memory64;
                    }
                }
                wp::Payload::GlobalSection(section) => {
                    file.add_section(SectionId::Global, section.range(), "");
                    for global in section {
                        let global = global.read_error("Couldn't read a global item")?;
                        let mut address = None;
                        if !global.ty.mutable {
                            // There should be exactly one instruction.
                            let init = global.init_expr.get_operators_reader().read();
                            address = match init.read_error("Couldn't read a global init expr")? {
                                wp::Operator::I32Const { value } => Some(value as u64),
                                wp::Operator::I64Const { value } => Some(value as u64),
                                _ => None,
                            };
                        }
                        global_values.push(address);
                    }
                }
                wp::Payload::ExportSection(section) => {
                    file.add_section(SectionId::Export, section.range(), "");
                    exports = Some(section);
                }
                wp::Payload::StartSection { func, range, .. } => {
                    file.add_section(SectionId::Start, range, "");
                    entry_func_id = Some(func);
                }
                wp::Payload::ElementSection(section) => {
                    file.add_section(SectionId::Element, section.range(), "");
                }
                wp::Payload::CodeSectionStart { range, .. } => {
                    code_range_start = range.start;
                    file.add_section(SectionId::Code, range, "");
                }
                wp::Payload::CodeSectionEntry(body) => {
                    let range = body.range();
                    let address = range.start as u64 - code_range_start as u64;
                    let size = (range.end - range.start) as u64;
                    code_ranges.push((address, size));
                }
                wp::Payload::DataSection(section) => {
                    file.add_section(SectionId::Data, section.range(), "");
                }
                wp::Payload::DataCountSection { range, .. } => {
                    file.add_section(SectionId::DataCount, range, "");
                }
                wp::Payload::TagSection(section) => {
                    file.add_section(SectionId::Tag, section.range(), "");
                }
                wp::Payload::CustomSection(section) => {
                    let name = section.name();
                    let size = section.data().len();
                    let mut range = section.range();
                    range.start = range.end - size;
                    file.add_section(SectionId::Custom, range, name);
                    if name == "name" {
                        let reader = wp::BinaryReader::new(section.data(), section.data_offset());
                        names = Some(wp::NameSectionReader::new(reader));
                    } else if name == "linking" {
                        // https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md
                        let reader = wp::BinaryReader::new(section.data(), section.data_offset());
                        let linking = wp::LinkingSectionReader::new(reader)
                            .read_error("Invalid Wasm linking section")?;
                        for subsection in linking {
                            let subsection =
                                subsection.read_error("Invalid Wasm linking subsection")?;
                            if let wp::Linking::SymbolTable(s) = subsection {
                                symbols = Some(s);
                            }
                        }
                    } else if name.starts_with(".debug_") {
                        file.has_debug_symbols = true;
                    }
                }
                _ => {}
            }
        }

        if let Some(entry_func_id) = entry_func_id {
            if let Some(range) = code_ranges.get(entry_func_id as usize) {
                file.entry = range.0;
            }
        }

        let mut import_func_names = Vec::new();
        let mut import_global_names = Vec::new();
        if let Some(imports) = imports {
            let mut last_module_name = None;

            for import in imports {
                let import = import.read_error("Couldn't read an import item")?;
                let kind = match import.ty {
                    wp::TypeRef::Func(_) => {
                        import_func_names.push(import.name);
                        SymbolKind::Text
                    }
                    wp::TypeRef::Memory(memory) => {
                        file.has_memory64 |= memory.memory64;
                        SymbolKind::Data
                    }
                    wp::TypeRef::Global(_) => {
                        import_global_names.push(import.name);
                        SymbolKind::Data
                    }
                    wp::TypeRef::Table(_) => SymbolKind::Data,
                    wp::TypeRef::Tag(_) => SymbolKind::Unknown,
                };

                if symbols.is_some() {
                    // We have a symbol table, so we don't need to add symbols for imports.
                    // TODO: never add symbols for imports. Return them via Object::imports instead.
                    continue;
                }

                let module_name = import.module;
                if last_module_name != Some(module_name) {
                    file.symbols.push(WasmSymbolInternal {
                        name: module_name,
                        address: 0,
                        size: 0,
                        kind: SymbolKind::File,
                        section: SymbolSection::None,
                        scope: SymbolScope::Dynamic,
                        weak: false,
                    });
                    last_module_name = Some(module_name);
                }

                file.symbols.push(WasmSymbolInternal {
                    name: import.name,
                    address: 0,
                    size: 0,
                    kind,
                    section: SymbolSection::Undefined,
                    scope: SymbolScope::Dynamic,
                    weak: false,
                });
            }
        }

        if let Some(symbols) = symbols {
            // We have a symbol table, so we don't need to add symbols for locals or exports.
            // These sections shouldn't be present at the same time as a symbol table anyway.
            // TODO: never add symbols for exports. Return them via Object::exports instead.
            exports = None;
            names = None;

            for symbol in symbols {
                let symbol = symbol.read_error("Invalid Wasm linking symbol")?;
                let flags = match symbol {
                    wp::SymbolInfo::Func { flags, .. } => flags,
                    wp::SymbolInfo::Data { flags, .. } => flags,
                    wp::SymbolInfo::Global { flags, .. } => flags,
                    wp::SymbolInfo::Section { flags, .. } => flags,
                    wp::SymbolInfo::Event { flags, .. } => flags,
                    wp::SymbolInfo::Table { flags, .. } => flags,
                };
                let kind = if flags.contains(wp::SymbolFlags::TLS) {
                    SymbolKind::Tls
                } else {
                    match symbol {
                        wp::SymbolInfo::Func { .. } => SymbolKind::Text,
                        wp::SymbolInfo::Data { .. } => SymbolKind::Data,
                        wp::SymbolInfo::Global { .. } => SymbolKind::Data,
                        wp::SymbolInfo::Section { .. } => SymbolKind::Section,
                        wp::SymbolInfo::Event { .. } => SymbolKind::Unknown,
                        wp::SymbolInfo::Table { .. } => SymbolKind::Data,
                    }
                };
                let section = if flags.contains(wp::SymbolFlags::UNDEFINED) {
                    SymbolSection::Undefined
                } else if flags.contains(wp::SymbolFlags::ABSOLUTE) {
                    SymbolSection::Absolute
                } else {
                    match symbol {
                        wp::SymbolInfo::Func { .. } => {
                            SymbolSection::Section(SectionIndex(SectionId::Code as usize))
                        }
                        _ => {
                            // TODO: anything that is defined should have a known section.
                            // Additionally, address and size should be within this section.
                            SymbolSection::Unknown
                        }
                    }
                };
                let scope = if flags.contains(wp::SymbolFlags::BINDING_LOCAL) {
                    SymbolScope::Compilation
                } else if flags.contains(wp::SymbolFlags::VISIBILITY_HIDDEN) {
                    SymbolScope::Linkage
                } else {
                    SymbolScope::Dynamic
                };
                let weak = flags.contains(wp::SymbolFlags::BINDING_WEAK);

                let mut address = 0;
                let mut size = 0;
                let name = match symbol {
                    wp::SymbolInfo::Func {
                        index, mut name, ..
                    } => {
                        if let Some(local_index) = index.checked_sub(import_func_names.len() as u32)
                        {
                            if let Some(range) = code_ranges.get(local_index as usize).copied() {
                                address = range.0;
                                size = range.1;
                            }
                        } else {
                            if !flags.contains(wp::SymbolFlags::EXPLICIT_NAME) {
                                name = Some(import_func_names[index as usize]);
                            }
                        }
                        name
                    }
                    wp::SymbolInfo::Data { name, symbol, .. } => {
                        if let Some(symbol) = symbol {
                            // TODO: this is an offset within a data segment.
                            // This may need to be changed to be an offset within the data section.
                            address = symbol.offset.into();
                            size = symbol.size.into();
                        }
                        Some(name)
                    }
                    wp::SymbolInfo::Section { .. } => {
                        // TODO: find the section name
                        None
                    }
                    wp::SymbolInfo::Global { name, index, .. } => {
                        if !flags.contains(wp::SymbolFlags::EXPLICIT_NAME) {
                            import_global_names.get(index as usize).copied()
                        } else {
                            name
                        }
                    }
                    wp::SymbolInfo::Event { name, .. } | wp::SymbolInfo::Table { name, .. } => name,
                };

                file.symbols.push(WasmSymbolInternal {
                    name: name.unwrap_or(""),
                    address,
                    size,
                    kind,
                    section,
                    scope,
                    weak,
                });
            }
        }

        if let Some(exports) = exports {
            if let Some(main_file_symbol) = main_file_symbol.take() {
                file.symbols.push(main_file_symbol);
            }

            for export in exports {
                let export = export.read_error("Couldn't read an export item")?;

                let (kind, section_idx) = match export.kind {
                    wp::ExternalKind::Func => {
                        if let Some(local_func_id) =
                            export.index.checked_sub(import_func_names.len() as u32)
                        {
                            let local_func_kind = local_func_kinds
                                .get_mut(local_func_id as usize)
                                .read_error("Invalid Wasm export index")?;
                            *local_func_kind = LocalFunctionKind::Exported;
                        }
                        (SymbolKind::Text, SectionId::Code)
                    }
                    wp::ExternalKind::Table
                    | wp::ExternalKind::Memory
                    | wp::ExternalKind::Global => (SymbolKind::Data, SectionId::Data),
                    // TODO
                    wp::ExternalKind::Tag => continue,
                };

                // Try to guess the symbol address. Rust and C export a global containing
                // the address in linear memory of the symbol.
                let mut address = 0;
                let mut size = 0;
                if export.kind == wp::ExternalKind::Global {
                    if let Some(&Some(x)) = global_values.get(export.index as usize) {
                        address = x;
                    }
                }
                if export.kind == wp::ExternalKind::Func {
                    if let Some(local_func_id) =
                        export.index.checked_sub(import_func_names.len() as u32)
                    {
                        if let Some(range) = code_ranges.get(local_func_id as usize) {
                            address = range.0;
                            size = range.1
                        }
                    }
                }

                file.symbols.push(WasmSymbolInternal {
                    name: export.name,
                    address,
                    size,
                    kind,
                    section: SymbolSection::Section(SectionIndex(section_idx as usize)),
                    scope: SymbolScope::Dynamic,
                    weak: false,
                });
            }
        }
        if let Some(names) = names {
            if let Some(main_file_symbol) = main_file_symbol.take() {
                file.symbols.push(main_file_symbol);
            }
            for name in names {
                let name = name.read_error("Invalid wasm name section")?;
                let wp::Name::Function(name_map) = name else {
                    continue;
                };
                for naming in name_map {
                    let naming = naming.read_error("Couldn't read a function name")?;
                    let Some(local_index) =
                        naming.index.checked_sub(import_func_names.len() as u32)
                    else {
                        continue;
                    };
                    let Some(LocalFunctionKind::Unknown) =
                        local_func_kinds.get(local_index as usize)
                    else {
                        continue;
                    };
                    let Some((address, size)) = code_ranges.get(local_index as usize).copied()
                    else {
                        continue;
                    };
                    file.symbols.push(WasmSymbolInternal {
                        name: naming.name,
                        address,
                        size,
                        kind: SymbolKind::Text,
                        section: SymbolSection::Section(SectionIndex(SectionId::Code as usize)),
                        scope: SymbolScope::Compilation,
                        weak: false,
                    });
                }
            }
        }

        Ok(file)
    }

    fn add_section(&mut self, id: SectionId, range: Range<usize>, name: &'data str) {
        let section = SectionHeader { id, range, name };
        self.id_sections[id as usize] = Some(self.sections.len());
        self.sections.push(section);
    }
}

impl<'data, R> read::private::Sealed for WasmFile<'data, R> {}

impl<'data, R: ReadRef<'data>> Object<'data> for WasmFile<'data, R> {
    type Segment<'file>
        = WasmSegment<'data, 'file, R>
    where
        Self: 'file,
        'data: 'file;
    type SegmentIterator<'file>
        = WasmSegmentIterator<'data, 'file, R>
    where
        Self: 'file,
        'data: 'file;
    type Section<'file>
        = WasmSection<'data, 'file, R>
    where
        Self: 'file,
        'data: 'file;
    type SectionIterator<'file>
        = WasmSectionIterator<'data, 'file, R>
    where
        Self: 'file,
        'data: 'file;
    type Comdat<'file>
        = WasmComdat<'data, 'file, R>
    where
        Self: 'file,
        'data: 'file;
    type ComdatIterator<'file>
        = WasmComdatIterator<'data, 'file, R>
    where
        Self: 'file,
        'data: 'file;
    type Symbol<'file>
        = WasmSymbol<'data, 'file>
    where
        Self: 'file,
        'data: 'file;
    type SymbolIterator<'file>
        = WasmSymbolIterator<'data, 'file>
    where
        Self: 'file,
        'data: 'file;
    type SymbolTable<'file>
        = WasmSymbolTable<'data, 'file>
    where
        Self: 'file,
        'data: 'file;
    type DynamicRelocationIterator<'file>
        = NoDynamicRelocationIterator
    where
        Self: 'file,
        'data: 'file;

    #[inline]
    fn architecture(&self) -> Architecture {
        if self.has_memory64 {
            Architecture::Wasm64
        } else {
            Architecture::Wasm32
        }
    }

    #[inline]
    fn is_little_endian(&self) -> bool {
        true
    }

    #[inline]
    fn is_64(&self) -> bool {
        self.has_memory64
    }

    fn kind(&self) -> ObjectKind {
        // TODO: check for `linking` custom section
        ObjectKind::Unknown
    }

    fn segments(&self) -> Self::SegmentIterator<'_> {
        WasmSegmentIterator { file: self }
    }

    fn section_by_name_bytes<'file>(
        &'file self,
        section_name: &[u8],
    ) -> Option<WasmSection<'data, 'file, R>> {
        self.sections()
            .find(|section| section.name_bytes() == Ok(section_name))
    }

    fn section_by_index(&self, index: SectionIndex) -> Result<WasmSection<'data, '_, R>> {
        // TODO: Missing sections should return an empty section.
        let id_section = self
            .id_sections
            .get(index.0)
            .and_then(|x| *x)
            .read_error("Invalid Wasm section index")?;
        let section = self.sections.get(id_section).unwrap();
        Ok(WasmSection {
            file: self,
            section,
        })
    }

    fn sections(&self) -> Self::SectionIterator<'_> {
        WasmSectionIterator {
            file: self,
            sections: self.sections.iter(),
        }
    }

    fn comdats(&self) -> Self::ComdatIterator<'_> {
        WasmComdatIterator { file: self }
    }

    #[inline]
    fn symbol_by_index(&self, index: SymbolIndex) -> Result<WasmSymbol<'data, '_>> {
        let symbol = self
            .symbols
            .get(index.0)
            .read_error("Invalid Wasm symbol index")?;
        Ok(WasmSymbol { index, symbol })
    }

    fn symbols(&self) -> Self::SymbolIterator<'_> {
        WasmSymbolIterator {
            symbols: self.symbols.iter().enumerate(),
        }
    }

    fn symbol_table(&self) -> Option<WasmSymbolTable<'data, '_>> {
        Some(WasmSymbolTable {
            symbols: &self.symbols,
        })
    }

    fn dynamic_symbols(&self) -> Self::SymbolIterator<'_> {
        WasmSymbolIterator {
            symbols: [].iter().enumerate(),
        }
    }

    #[inline]
    fn dynamic_symbol_table(&self) -> Option<WasmSymbolTable<'data, '_>> {
        None
    }

    #[inline]
    fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator> {
        None
    }

    fn imports(&self) -> Result<Vec<Import<'data>>> {
        // TODO: return entries in the import section
        Ok(Vec::new())
    }

    fn exports(&self) -> Result<Vec<Export<'data>>> {
        // TODO: return entries in the export section
        Ok(Vec::new())
    }

    fn has_debug_symbols(&self) -> bool {
        self.has_debug_symbols
    }

    fn relative_address_base(&self) -> u64 {
        0
    }

    #[inline]
    fn entry(&self) -> u64 {
        self.entry
    }

    #[inline]
    fn flags(&self) -> FileFlags {
        FileFlags::None
    }
}

/// An iterator for the segments in a [`WasmFile`].
///
/// This is a stub that doesn't implement any functionality.
#[derive(Debug)]
pub struct WasmSegmentIterator<'data, 'file, R = &'data [u8]> {
    #[allow(unused)]
    file: &'file WasmFile<'data, R>,
}

impl<'data, 'file, R> Iterator for WasmSegmentIterator<'data, 'file, R> {
    type Item = WasmSegment<'data, 'file, R>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// A segment in a [`WasmFile`].
///
/// This is a stub that doesn't implement any functionality.
#[derive(Debug)]
pub struct WasmSegment<'data, 'file, R = &'data [u8]> {
    #[allow(unused)]
    file: &'file WasmFile<'data, R>,
}

impl<'data, 'file, R> read::private::Sealed for WasmSegment<'data, 'file, R> {}

impl<'data, 'file, R> ObjectSegment<'data> for WasmSegment<'data, 'file, R> {
    #[inline]
    fn address(&self) -> u64 {
        unreachable!()
    }

    #[inline]
    fn size(&self) -> u64 {
        unreachable!()
    }

    #[inline]
    fn align(&self) -> u64 {
        unreachable!()
    }

    #[inline]
    fn file_range(&self) -> (u64, u64) {
        unreachable!()
    }

    fn data(&self) -> Result<&'data [u8]> {
        unreachable!()
    }

    fn data_range(&self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>> {
        unreachable!()
    }

    #[inline]
    fn name_bytes(&self) -> Result<Option<&[u8]>> {
        unreachable!()
    }

    #[inline]
    fn name(&self) -> Result<Option<&str>> {
        unreachable!()
    }

    #[inline]
    fn flags(&self) -> SegmentFlags {
        unreachable!()
    }
}

/// An iterator for the sections in a [`WasmFile`].
#[derive(Debug)]
pub struct WasmSectionIterator<'data, 'file, R = &'data [u8]> {
    file: &'file WasmFile<'data, R>,
    sections: slice::Iter<'file, SectionHeader<'data>>,
}

impl<'data, 'file, R> Iterator for WasmSectionIterator<'data, 'file, R> {
    type Item = WasmSection<'data, 'file, R>;

    fn next(&mut self) -> Option<Self::Item> {
        let section = self.sections.next()?;
        Some(WasmSection {
            file: self.file,
            section,
        })
    }
}

/// A section in a [`WasmFile`].
///
/// Most functionality is provided by the [`ObjectSection`] trait implementation.
#[derive(Debug)]
pub struct WasmSection<'data, 'file, R = &'data [u8]> {
    file: &'file WasmFile<'data, R>,
    section: &'file SectionHeader<'data>,
}

impl<'data, 'file, R> read::private::Sealed for WasmSection<'data, 'file, R> {}

impl<'data, 'file, R: ReadRef<'data>> ObjectSection<'data> for WasmSection<'data, 'file, R> {
    type RelocationIterator = WasmRelocationIterator<'data, 'file, R>;

    #[inline]
    fn index(&self) -> SectionIndex {
        // Note that we treat all custom sections as index 0.
        // This is ok because they are never looked up by index.
        SectionIndex(self.section.id as usize)
    }

    #[inline]
    fn address(&self) -> u64 {
        0
    }

    #[inline]
    fn size(&self) -> u64 {
        let range = &self.section.range;
        (range.end - range.start) as u64
    }

    #[inline]
    fn align(&self) -> u64 {
        1
    }

    #[inline]
    fn file_range(&self) -> Option<(u64, u64)> {
        let range = &self.section.range;
        Some((range.start as _, range.end as _))
    }

    #[inline]
    fn data(&self) -> Result<&'data [u8]> {
        let range = &self.section.range;
        self.file
            .data
            .read_bytes_at(range.start as u64, range.end as u64 - range.start as u64)
            .read_error("Invalid Wasm section size or offset")
    }

    fn data_range(&self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>> {
        unimplemented!()
    }

    #[inline]
    fn compressed_file_range(&self) -> Result<CompressedFileRange> {
        Ok(CompressedFileRange::none(self.file_range()))
    }

    #[inline]
    fn compressed_data(&self) -> Result<CompressedData<'data>> {
        self.data().map(CompressedData::none)
    }

    #[inline]
    fn name_bytes(&self) -> Result<&'data [u8]> {
        self.name().map(str::as_bytes)
    }

    #[inline]
    fn name(&self) -> Result<&'data str> {
        Ok(match self.section.id {
            SectionId::Custom => self.section.name,
            SectionId::Type => "<type>",
            SectionId::Import => "<import>",
            SectionId::Function => "<function>",
            SectionId::Table => "<table>",
            SectionId::Memory => "<memory>",
            SectionId::Global => "<global>",
            SectionId::Export => "<export>",
            SectionId::Start => "<start>",
            SectionId::Element => "<element>",
            SectionId::Code => "<code>",
            SectionId::Data => "<data>",
            SectionId::DataCount => "<data_count>",
            SectionId::Tag => "<tag>",
        })
    }

    #[inline]
    fn segment_name_bytes(&self) -> Result<Option<&[u8]>> {
        Ok(None)
    }

    #[inline]
    fn segment_name(&self) -> Result<Option<&str>> {
        Ok(None)
    }

    #[inline]
    fn kind(&self) -> SectionKind {
        match self.section.id {
            SectionId::Custom => match self.section.name {
                "reloc." | "linking" => SectionKind::Linker,
                _ => SectionKind::Other,
            },
            SectionId::Type => SectionKind::Metadata,
            SectionId::Import => SectionKind::Linker,
            SectionId::Function => SectionKind::Metadata,
            SectionId::Table => SectionKind::UninitializedData,
            SectionId::Memory => SectionKind::UninitializedData,
            SectionId::Global => SectionKind::Data,
            SectionId::Export => SectionKind::Linker,
            SectionId::Start => SectionKind::Linker,
            SectionId::Element => SectionKind::Data,
            SectionId::Code => SectionKind::Text,
            SectionId::Data => SectionKind::Data,
            SectionId::DataCount => SectionKind::UninitializedData,
            SectionId::Tag => SectionKind::Data,
        }
    }

    #[inline]
    fn relocations(&self) -> WasmRelocationIterator<'data, 'file, R> {
        WasmRelocationIterator(PhantomData)
    }

    fn relocation_map(&self) -> read::Result<RelocationMap> {
        RelocationMap::new(self.file, self)
    }

    #[inline]
    fn flags(&self) -> SectionFlags {
        SectionFlags::None
    }
}

/// An iterator for the COMDAT section groups in a [`WasmFile`].
///
/// This is a stub that doesn't implement any functionality.
#[derive(Debug)]
pub struct WasmComdatIterator<'data, 'file, R = &'data [u8]> {
    #[allow(unused)]
    file: &'file WasmFile<'data, R>,
}

impl<'data, 'file, R> Iterator for WasmComdatIterator<'data, 'file, R> {
    type Item = WasmComdat<'data, 'file, R>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// A COMDAT section group in a [`WasmFile`].
///
/// This is a stub that doesn't implement any functionality.
#[derive(Debug)]
pub struct WasmComdat<'data, 'file, R = &'data [u8]> {
    #[allow(unused)]
    file: &'file WasmFile<'data, R>,
}

impl<'data, 'file, R> read::private::Sealed for WasmComdat<'data, 'file, R> {}

impl<'data, 'file, R> ObjectComdat<'data> for WasmComdat<'data, 'file, R> {
    type SectionIterator = WasmComdatSectionIterator<'data, 'file, R>;

    #[inline]
    fn kind(&self) -> ComdatKind {
        unreachable!();
    }

    #[inline]
    fn symbol(&self) -> SymbolIndex {
        unreachable!();
    }

    #[inline]
    fn name_bytes(&self) -> Result<&'data [u8]> {
        unreachable!();
    }

    #[inline]
    fn name(&self) -> Result<&'data str> {
        unreachable!();
    }

    #[inline]
    fn sections(&self) -> Self::SectionIterator {
        unreachable!();
    }
}

/// An iterator for the sections in a COMDAT section group in a [`WasmFile`].
///
/// This is a stub that doesn't implement any functionality.
#[derive(Debug)]
pub struct WasmComdatSectionIterator<'data, 'file, R = &'data [u8]> {
    #[allow(unused)]
    file: &'file WasmFile<'data, R>,
}

impl<'data, 'file, R> Iterator for WasmComdatSectionIterator<'data, 'file, R> {
    type Item = SectionIndex;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// A symbol table in a [`WasmFile`].
#[derive(Debug)]
pub struct WasmSymbolTable<'data, 'file> {
    symbols: &'file [WasmSymbolInternal<'data>],
}

impl<'data, 'file> read::private::Sealed for WasmSymbolTable<'data, 'file> {}

impl<'data, 'file> ObjectSymbolTable<'data> for WasmSymbolTable<'data, 'file> {
    type Symbol = WasmSymbol<'data, 'file>;
    type SymbolIterator = WasmSymbolIterator<'data, 'file>;

    fn symbols(&self) -> Self::SymbolIterator {
        WasmSymbolIterator {
            symbols: self.symbols.iter().enumerate(),
        }
    }

    fn symbol_by_index(&self, index: SymbolIndex) -> Result<Self::Symbol> {
        let symbol = self
            .symbols
            .get(index.0)
            .read_error("Invalid Wasm symbol index")?;
        Ok(WasmSymbol { index, symbol })
    }
}

/// An iterator for the symbols in a [`WasmFile`].
#[derive(Debug)]
pub struct WasmSymbolIterator<'data, 'file> {
    symbols: core::iter::Enumerate<slice::Iter<'file, WasmSymbolInternal<'data>>>,
}

impl<'data, 'file> Iterator for WasmSymbolIterator<'data, 'file> {
    type Item = WasmSymbol<'data, 'file>;

    fn next(&mut self) -> Option<Self::Item> {
        let (index, symbol) = self.symbols.next()?;
        Some(WasmSymbol {
            index: SymbolIndex(index),
            symbol,
        })
    }
}

/// A symbol in a [`WasmFile`].
///
/// Most functionality is provided by the [`ObjectSymbol`] trait implementation.
#[derive(Clone, Copy, Debug)]
pub struct WasmSymbol<'data, 'file> {
    index: SymbolIndex,
    symbol: &'file WasmSymbolInternal<'data>,
}

#[derive(Clone, Debug)]
struct WasmSymbolInternal<'data> {
    name: &'data str,
    address: u64,
    size: u64,
    kind: SymbolKind,
    section: SymbolSection,
    scope: SymbolScope,
    weak: bool,
}

impl<'data, 'file> read::private::Sealed for WasmSymbol<'data, 'file> {}

impl<'data, 'file> ObjectSymbol<'data> for WasmSymbol<'data, 'file> {
    #[inline]
    fn index(&self) -> SymbolIndex {
        self.index
    }

    #[inline]
    fn name_bytes(&self) -> read::Result<&'data [u8]> {
        Ok(self.symbol.name.as_bytes())
    }

    #[inline]
    fn name(&self) -> read::Result<&'data str> {
        Ok(self.symbol.name)
    }

    #[inline]
    fn address(&self) -> u64 {
        self.symbol.address
    }

    #[inline]
    fn size(&self) -> u64 {
        self.symbol.size
    }

    #[inline]
    fn kind(&self) -> SymbolKind {
        self.symbol.kind
    }

    #[inline]
    fn section(&self) -> SymbolSection {
        self.symbol.section
    }

    #[inline]
    fn is_undefined(&self) -> bool {
        self.symbol.section == SymbolSection::Undefined
    }

    #[inline]
    fn is_definition(&self) -> bool {
        (self.symbol.kind == SymbolKind::Text || self.symbol.kind == SymbolKind::Data)
            && self.symbol.section != SymbolSection::Undefined
    }

    #[inline]
    fn is_common(&self) -> bool {
        self.symbol.section == SymbolSection::Common
    }

    #[inline]
    fn is_weak(&self) -> bool {
        self.symbol.weak
    }

    #[inline]
    fn scope(&self) -> SymbolScope {
        self.symbol.scope
    }

    #[inline]
    fn is_global(&self) -> bool {
        self.symbol.scope != SymbolScope::Compilation
    }

    #[inline]
    fn is_local(&self) -> bool {
        self.symbol.scope == SymbolScope::Compilation
    }

    #[inline]
    fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex> {
        SymbolFlags::None
    }
}

/// An iterator for the relocations for a [`WasmSection`].
///
/// This is a stub that doesn't implement any functionality.
#[derive(Debug)]
pub struct WasmRelocationIterator<'data, 'file, R = &'data [u8]>(
    PhantomData<(&'data (), &'file (), R)>,
);

impl<'data, 'file, R> Iterator for WasmRelocationIterator<'data, 'file, R> {
    type Item = (u64, Relocation);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
