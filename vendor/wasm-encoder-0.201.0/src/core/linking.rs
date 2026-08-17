use std::borrow::Cow;

use crate::{encode_section, CustomSection, Encode, Section, SectionId};

const VERSION: u32 = 2;

/// An encoder for the [linking custom
/// section](https://github.com/WebAssembly/tool-conventions/blob/master/Linking.md#linking-metadata-section).
///
/// This section is a non-standard convention that is supported by the LLVM
/// toolchain. It, along with associated "reloc.*" custom sections, allows you
/// to treat a Wasm module as a low-level object file that can be linked with
/// other Wasm object files to produce a final, complete Wasm module.
///
/// The linking section must come before the reloc sections.
///
/// # Example
///
/// ```
/// use wasm_encoder::{LinkingSection, Module, SymbolTable};
///
/// // Create a new linking section.
/// let mut linking = LinkingSection::new();
///
/// // Define a symbol table.
/// let mut sym_tab = SymbolTable::new();
///
/// // Define a function symbol in the symbol table.
/// let flags = SymbolTable::WASM_SYM_BINDING_LOCAL | SymbolTable::WASM_SYM_EXPORTED;
/// let func_index = 42;
/// let sym_name = "my_exported_func";
/// sym_tab.function(flags, func_index, Some(sym_name));
///
/// // Add the symbol table to our linking section.
/// linking.symbol_table(&sym_tab);
///
/// // Add the linking section to a new Wasm module and get the encoded bytes.
/// let mut module = Module::new();
/// module.section(&linking);
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Debug)]
pub struct LinkingSection {
    bytes: Vec<u8>,
}

impl LinkingSection {
    /// Construct a new encoder for the linking custom section.
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: `fn segment_info` for the `WASM_SEGMENT_INFO` linking subsection.

    // TODO: `fn init_funcs` for the `WASM_INIT_FUNCS` linking subsection.

    // TODO: `fn comdat_info` for the `WASM_COMDAT_INFO` linking subsection.

    /// Add a symbol table subsection.
    pub fn symbol_table(&mut self, symbol_table: &SymbolTable) -> &mut Self {
        symbol_table.encode(&mut self.bytes);
        self
    }
}

impl Default for LinkingSection {
    fn default() -> Self {
        let mut bytes = Vec::new();
        VERSION.encode(&mut bytes);
        Self { bytes }
    }
}

impl Encode for LinkingSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        CustomSection {
            name: "linking".into(),
            data: Cow::Borrowed(&self.bytes),
        }
        .encode(sink);
    }
}

impl Section for LinkingSection {
    fn id(&self) -> u8 {
        SectionId::Custom.into()
    }
}

#[allow(unused)]
const WASM_SEGMENT_INFO: u8 = 5;
#[allow(unused)]
const WASM_INIT_FUNCS: u8 = 6;
#[allow(unused)]
const WASM_COMDAT_INFO: u8 = 7;
const WASM_SYMBOL_TABLE: u8 = 8;

/// A subsection of the [linking custom section][crate::LinkingSection] that
/// provides extra information about the symbols present in this Wasm object
/// file.
#[derive(Clone, Debug, Default)]
pub struct SymbolTable {
    bytes: Vec<u8>,
    num_added: u32,
}

const SYMTAB_FUNCTION: u32 = 0;
const SYMTAB_DATA: u32 = 1;
const SYMTAB_GLOBAL: u32 = 2;
#[allow(unused)]
const SYMTAB_SECTION: u32 = 3;
#[allow(unused)]
const SYMTAB_TAG: u32 = 4;
const SYMTAB_TABLE: u32 = 5;

impl SymbolTable {
    /// Construct a new symbol table subsection encoder.
    pub fn new() -> Self {
        SymbolTable {
            bytes: vec![],
            num_added: 0,
        }
    }

    /// Define a function symbol in this symbol table.
    ///
    /// The `name` must be omitted if `index` references an imported table and
    /// the `WASM_SYM_EXPLICIT_NAME` flag is not set.
    pub fn function(&mut self, flags: u32, index: u32, name: Option<&str>) -> &mut Self {
        SYMTAB_FUNCTION.encode(&mut self.bytes);
        flags.encode(&mut self.bytes);
        index.encode(&mut self.bytes);
        if let Some(name) = name {
            name.encode(&mut self.bytes);
        }
        self.num_added += 1;
        self
    }

    /// Define a global symbol in this symbol table.
    ///
    /// The `name` must be omitted if `index` references an imported table and
    /// the `WASM_SYM_EXPLICIT_NAME` flag is not set.
    pub fn global(&mut self, flags: u32, index: u32, name: Option<&str>) -> &mut Self {
        SYMTAB_GLOBAL.encode(&mut self.bytes);
        flags.encode(&mut self.bytes);
        index.encode(&mut self.bytes);
        if let Some(name) = name {
            name.encode(&mut self.bytes);
        }
        self.num_added += 1;
        self
    }

    // TODO: tags

    /// Define a table symbol in this symbol table.
    ///
    /// The `name` must be omitted if `index` references an imported table and
    /// the `WASM_SYM_EXPLICIT_NAME` flag is not set.
    pub fn table(&mut self, flags: u32, index: u32, name: Option<&str>) -> &mut Self {
        SYMTAB_TABLE.encode(&mut self.bytes);
        flags.encode(&mut self.bytes);
        index.encode(&mut self.bytes);
        if let Some(name) = name {
            name.encode(&mut self.bytes);
        }
        self.num_added += 1;
        self
    }

    /// Add a data symbol to this symbol table.
    pub fn data(
        &mut self,
        flags: u32,
        name: &str,
        definition: Option<DataSymbolDefinition>,
    ) -> &mut Self {
        SYMTAB_DATA.encode(&mut self.bytes);
        flags.encode(&mut self.bytes);
        name.encode(&mut self.bytes);
        if let Some(def) = definition {
            def.index.encode(&mut self.bytes);
            def.offset.encode(&mut self.bytes);
            def.size.encode(&mut self.bytes);
        }
        self.num_added += 1;
        self
    }

    // TODO: sections

    /// This is a weak symbol.
    ///
    /// This flag is mutually exclusive with `WASM_SYM_BINDING_LOCAL`.
    ///
    /// When linking multiple modules defining the same symbol, all weak
    /// definitions are discarded if any strong definitions exist; then if
    /// multiple weak definitions exist all but one (unspecified) are discarded;
    /// and finally it is an error if more than one definition remains.
    pub const WASM_SYM_BINDING_WEAK: u32 = 0x1;

    /// This is a local symbol.
    ///
    /// This flag is mutually exclusive with `WASM_SYM_BINDING_WEAK`.
    ///
    /// Local symbols are not to be exported, or linked to other
    /// modules/sections. The names of all non-local symbols must be unique, but
    /// the names of local symbols are not considered for uniqueness. A local
    /// function or global symbol cannot reference an import.
    pub const WASM_SYM_BINDING_LOCAL: u32 = 0x02;

    /// This is a hidden symbol.
    ///
    /// Hidden symbols are not to be exported when performing the final link,
    /// but may be linked to other modules.
    pub const WASM_SYM_VISIBILITY_HIDDEN: u32 = 0x04;

    /// This symbol is not defined.
    ///
    /// For non-data symbols, this must match whether the symbol is an import or
    /// is defined; for data symbols, determines whether a segment is specified.
    pub const WASM_SYM_UNDEFINED: u32 = 0x10;

    /// This symbol is intended to be exported from the wasm module to the host
    /// environment.
    ///
    /// This differs from the visibility flags in that it effects the static
    /// linker.
    pub const WASM_SYM_EXPORTED: u32 = 0x20;

    /// This symbol uses an explicit symbol name, rather than reusing the name
    /// from a wasm import.
    ///
    /// This allows it to remap imports from foreign WebAssembly modules into
    /// local symbols with different names.
    pub const WASM_SYM_EXPLICIT_NAME: u32 = 0x40;

    /// This symbol is intended to be included in the linker output, regardless
    /// of whether it is used by the program.
    pub const WASM_SYM_NO_STRIP: u32 = 0x80;
}

impl Encode for SymbolTable {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.push(WASM_SYMBOL_TABLE);
        encode_section(sink, self.num_added, &self.bytes);
    }
}

/// The definition of a data symbol within a symbol table.
#[derive(Clone, Debug)]
pub struct DataSymbolDefinition {
    /// The index of the data segment that this symbol is in.
    pub index: u32,
    /// The offset of this symbol within its segment.
    pub offset: u32,
    /// The byte size (which can be zero) of this data symbol.
    ///
    /// Note that `offset + size` must be less than or equal to the segment's
    /// size.
    pub size: u32,
}
