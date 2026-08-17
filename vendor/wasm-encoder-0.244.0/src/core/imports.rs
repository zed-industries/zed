use crate::{
    CORE_FUNCTION_EXACT_SORT, CORE_FUNCTION_SORT, CORE_GLOBAL_SORT, CORE_MEMORY_SORT,
    CORE_TABLE_SORT, CORE_TAG_SORT, Encode, GlobalType, MemoryType, Section, SectionId, TableType,
    TagType, encode_section,
};
use alloc::borrow::Cow;
use alloc::vec::Vec;

/// The type of an entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    /// A function type.
    ///
    /// The value is an index into the types section.
    Function(u32),
    /// A table type.
    Table(TableType),
    /// A memory type.
    Memory(MemoryType),
    /// A global type.
    Global(GlobalType),
    /// A tag type.
    ///
    /// This variant is used with the exception handling proposal.
    Tag(TagType),
    /// A function exact type.
    ///
    /// The value is an index into the types section.
    FunctionExact(u32),
}

impl Encode for EntityType {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Self::Function(f) => {
                sink.push(CORE_FUNCTION_SORT);
                f.encode(sink);
            }
            Self::FunctionExact(f) => {
                sink.push(CORE_FUNCTION_EXACT_SORT);
                f.encode(sink);
            }
            Self::Table(t) => {
                sink.push(CORE_TABLE_SORT);
                t.encode(sink);
            }
            Self::Memory(t) => {
                sink.push(CORE_MEMORY_SORT);
                t.encode(sink);
            }
            Self::Global(t) => {
                sink.push(CORE_GLOBAL_SORT);
                t.encode(sink);
            }
            Self::Tag(t) => {
                sink.push(CORE_TAG_SORT);
                t.encode(sink);
            }
        }
    }
}

impl From<TableType> for EntityType {
    fn from(t: TableType) -> Self {
        Self::Table(t)
    }
}

impl From<MemoryType> for EntityType {
    fn from(t: MemoryType) -> Self {
        Self::Memory(t)
    }
}

impl From<GlobalType> for EntityType {
    fn from(t: GlobalType) -> Self {
        Self::Global(t)
    }
}

impl From<TagType> for EntityType {
    fn from(t: TagType) -> Self {
        Self::Tag(t)
    }
}

/// An import item to be used with [`Imports::Single`].
#[derive(Clone, Debug)]
pub struct Import<'a> {
    /// The import's module name.
    pub module: &'a str,
    /// The import's item name.
    pub name: &'a str,
    /// The import's time.
    pub ty: EntityType,
}

/// An import item to be used with [`Imports::Compact1`].
#[derive(Clone, Debug)]
pub struct ImportCompact<'a> {
    /// The import's item name.
    pub name: &'a str,
    /// The import's type.
    pub ty: EntityType,
}

/// A single entry in the import section of a WebAssembly module, possibly containing multiple imports.
#[derive(Clone, Debug)]
pub enum Imports<'a> {
    /// A single import item.
    Single(Import<'a>),
    /// A group of imports with a common module name.
    Compact1 {
        /// The common module name.
        module: &'a str,
        /// The individual import items (name/type).
        items: Cow<'a, [ImportCompact<'a>]>,
    },
    /// A group of imports with a common module name and type.
    Compact2 {
        /// The common module name.
        module: &'a str,
        /// The common import type.
        ty: EntityType,
        /// The individual import item names.
        names: Cow<'a, [&'a str]>,
    },
}

/// An encoder for the import section of WebAssembly modules.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{MemoryType, Module, ImportSection};
///
/// let mut imports = ImportSection::new();
/// imports.import(
///     "env",
///     "memory",
///     MemoryType {
///         minimum: 1,
///         maximum: None,
///         memory64: false,
///         shared: false,
///         page_size_log2: None,
///     }
/// );
///
/// let mut module = Module::new();
/// module.section(&imports);
///
/// let bytes = module.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct ImportSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl ImportSection {
    /// Create a new import section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of imports in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define imports in the import section.
    pub fn imports<'a>(&mut self, imports: Imports<'a>) -> &mut Self {
        match imports {
            Imports::Single(import) => {
                import.module.encode(&mut self.bytes);
                import.name.encode(&mut self.bytes);
                import.ty.encode(&mut self.bytes);
            }
            Imports::Compact1 { module, items } => {
                module.encode(&mut self.bytes);
                self.bytes.push(0x00); // empty name
                self.bytes.push(0x7F);
                items.len().encode(&mut self.bytes);
                for item in items.iter() {
                    item.name.encode(&mut self.bytes);
                    item.ty.encode(&mut self.bytes);
                }
            }
            Imports::Compact2 { module, ty, names } => {
                module.encode(&mut self.bytes);
                self.bytes.push(0x00); // empty name
                self.bytes.push(0x7E);
                ty.encode(&mut self.bytes);
                names.len().encode(&mut self.bytes);
                for item in names.iter() {
                    item.encode(&mut self.bytes);
                }
            }
        }
        self.num_added += 1;
        self
    }

    /// Define an import in the import section.
    pub fn import(&mut self, module: &str, name: &str, ty: impl Into<EntityType>) -> &mut Self {
        self.imports(Imports::Single(Import {
            module,
            name,
            ty: ty.into(),
        }))
    }
}

impl Encode for ImportSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl Section for ImportSection {
    fn id(&self) -> u8 {
        SectionId::Import.into()
    }
}
