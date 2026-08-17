use super::{
    CORE_FUNCTION_SORT, CORE_GLOBAL_SORT, CORE_MEMORY_SORT, CORE_TABLE_SORT, CORE_TAG_SORT,
};
use crate::{encode_section, Encode, Section, SectionId};
use alloc::vec::Vec;

/// Represents the kind of an export from a WebAssembly module.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ExportKind {
    /// The export is a function.
    Func = CORE_FUNCTION_SORT,
    /// The export is a table.
    Table = CORE_TABLE_SORT,
    /// The export is a memory.
    Memory = CORE_MEMORY_SORT,
    /// The export is a global.
    Global = CORE_GLOBAL_SORT,
    /// The export is a tag.
    Tag = CORE_TAG_SORT,
}

impl Encode for ExportKind {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.push(*self as u8);
    }
}

/// An encoder for the export section of WebAssembly module.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Module, ExportSection, ExportKind};
///
/// let mut exports = ExportSection::new();
/// exports.export("foo", ExportKind::Func, 0);
///
/// let mut module = Module::new();
/// module.section(&exports);
///
/// let bytes = module.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct ExportSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl ExportSection {
    /// Create a new export section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of exports in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define an export in the export section.
    pub fn export(&mut self, name: &str, kind: ExportKind, index: u32) -> &mut Self {
        name.encode(&mut self.bytes);
        kind.encode(&mut self.bytes);
        index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }
}

impl Encode for ExportSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl Section for ExportSection {
    fn id(&self) -> u8 {
        SectionId::Export.into()
    }
}
