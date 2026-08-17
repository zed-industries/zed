use std::borrow::Cow;

use crate::{encoding_size, CustomSection, Encode, Section, SectionId};

/// An encoder for the custom `name` section.
///
/// # Example
///
/// ```
/// use wasm_encoder::{Module, NameSection, NameMap};
///
/// let mut names = NameSection::new();
/// names.module("the module name");
///
/// let mut function_names = NameMap::new();
/// function_names.append(0, "name of function 0");
/// function_names.append(1, "a better function");
/// function_names.append(3, "the best function");
/// names.functions(&function_names);
///
/// let mut module = Module::new();
/// module.section(&names);
///
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct NameSection {
    bytes: Vec<u8>,
}

enum Subsection {
    // Currently specified in the wasm spec's appendix
    Module = 0,
    Function = 1,
    Local = 2,

    // specified as part of the extended name section proposal
    //
    // https://github.com/WebAssembly/extended-name-section/blob/main/proposals/extended-name-section/Overview.md
    Label = 3,
    Type = 4,
    Table = 5,
    Memory = 6,
    Global = 7,
    Element = 8,
    Data = 9,

    // https://github.com/WebAssembly/gc/issues/193
    Field = 10,

    // https://github.com/WebAssembly/exception-handling/pull/213
    Tag = 11,
}

impl NameSection {
    /// Creates a new blank `name` custom section.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a module name subsection to this section.
    ///
    /// This will indicate that the name of the entire module should be the
    /// `name` specified. Note that this should be encoded first before other
    /// subsections.
    pub fn module(&mut self, name: &str) {
        let len = encoding_size(u32::try_from(name.len()).unwrap());
        self.subsection_header(Subsection::Module, len + name.len());
        name.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of all functions in this wasm module.
    ///
    /// Function names are declared in the `names` map provided where the index
    /// in the map corresponds to the wasm index of the function. This section
    /// should come after the module name subsection (if present) and before the
    /// locals subsection (if present).
    pub fn functions(&mut self, names: &NameMap) {
        self.subsection_header(Subsection::Function, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of locals within functions in this
    /// wasm module.
    ///
    /// This section should come after the function name subsection (if present)
    /// and before the labels subsection (if present).
    pub fn locals(&mut self, names: &IndirectNameMap) {
        self.subsection_header(Subsection::Local, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of labels within functions in this
    /// wasm module.
    ///
    /// This section should come after the local name subsection (if present)
    /// and before the type subsection (if present).
    pub fn labels(&mut self, names: &IndirectNameMap) {
        self.subsection_header(Subsection::Label, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of all types in this wasm module.
    ///
    /// This section should come after the label name subsection (if present)
    /// and before the table subsection (if present).
    pub fn types(&mut self, names: &NameMap) {
        self.subsection_header(Subsection::Type, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of all tables in this wasm module.
    ///
    /// This section should come after the type name subsection (if present)
    /// and before the memory subsection (if present).
    pub fn tables(&mut self, names: &NameMap) {
        self.subsection_header(Subsection::Table, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of all memories in this wasm module.
    ///
    /// This section should come after the table name subsection (if present)
    /// and before the global subsection (if present).
    pub fn memories(&mut self, names: &NameMap) {
        self.subsection_header(Subsection::Memory, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of all globals in this wasm module.
    ///
    /// This section should come after the memory name subsection (if present)
    /// and before the element subsection (if present).
    pub fn globals(&mut self, names: &NameMap) {
        self.subsection_header(Subsection::Global, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of all elements in this wasm module.
    ///
    /// This section should come after the global name subsection (if present)
    /// and before the data subsection (if present).
    pub fn elements(&mut self, names: &NameMap) {
        self.subsection_header(Subsection::Element, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of all data in this wasm module.
    ///
    /// This section should come after the element name subsection (if present)
    /// and before the field subsection (if present).
    pub fn data(&mut self, names: &NameMap) {
        self.subsection_header(Subsection::Data, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of all tags in this wasm module.
    ///
    /// This section should come after the data name subsection (if present).
    pub fn tag(&mut self, names: &NameMap) {
        self.subsection_header(Subsection::Tag, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of fields within types in this
    /// wasm module.
    ///
    /// This section should come after the data name subsection (if present)
    /// and before the tag subsection (if present).
    pub fn fields(&mut self, names: &IndirectNameMap) {
        self.subsection_header(Subsection::Field, names.size());
        names.encode(&mut self.bytes);
    }

    /// Appends a subsection for the names of all tags in this wasm module.
    ///
    /// This section should come after the field name subsection (if present).
    pub fn tags(&mut self, names: &NameMap) {
        self.subsection_header(Subsection::Tag, names.size());
        names.encode(&mut self.bytes);
    }

    fn subsection_header(&mut self, id: Subsection, len: usize) {
        self.bytes.push(id as u8);
        len.encode(&mut self.bytes);
    }

    /// View the encoded section as a CustomSection.
    pub fn as_custom<'a>(&'a self) -> CustomSection<'a> {
        CustomSection {
            name: "name".into(),
            data: Cow::Borrowed(&self.bytes),
        }
    }
}

impl Encode for NameSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.as_custom().encode(sink);
    }
}

impl Section for NameSection {
    fn id(&self) -> u8 {
        SectionId::Custom.into()
    }
}

/// A map used to name items in a wasm module, organized by naming each
/// individual index.
///
/// This is used in conjunction with [`NameSection::functions`] and simlar
/// methods.
#[derive(Clone, Debug, Default)]
pub struct NameMap {
    bytes: Vec<u8>,
    count: u32,
}

impl NameMap {
    /// Creates a new empty `NameMap`.
    pub fn new() -> NameMap {
        NameMap {
            bytes: vec![],
            count: 0,
        }
    }

    /// Adds a an entry where the item at `idx` has the `name` specified.
    ///
    /// Note that indices should be appended in ascending order of the index
    /// value. Each index may only be named once, but not all indices must be
    /// named (e.g. `0 foo; 1 bar; 7 qux` is valid but `0 foo; 0 bar` is not).
    /// Names do not have to be unique (e.g. `0 foo; 1 foo; 2 foo` is valid).
    pub fn append(&mut self, idx: u32, name: &str) {
        idx.encode(&mut self.bytes);
        name.encode(&mut self.bytes);
        self.count += 1;
    }

    pub(crate) fn size(&self) -> usize {
        encoding_size(self.count) + self.bytes.len()
    }

    /// Returns whether no names have been added to this map.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl Encode for NameMap {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.count.encode(sink);
        sink.extend(&self.bytes);
    }
}

/// A map used to describe names with two levels of indirection, as opposed to a
/// [`NameMap`] which has one level of indirection.
///
/// This naming map is used with [`NameSection::locals`], for example.
#[derive(Clone, Debug, Default)]
pub struct IndirectNameMap {
    bytes: Vec<u8>,
    count: u32,
}

impl IndirectNameMap {
    /// Creates a new empty name map.
    pub fn new() -> IndirectNameMap {
        IndirectNameMap {
            bytes: vec![],
            count: 0,
        }
    }

    /// Adds a new entry where the item at `idx` has sub-items named within
    /// `names` as specified.
    ///
    /// For example if this is describing local names then `idx` is a function
    /// index where the indexes within `names` are local indices.
    pub fn append(&mut self, idx: u32, names: &NameMap) {
        idx.encode(&mut self.bytes);
        names.encode(&mut self.bytes);
        self.count += 1;
    }

    fn size(&self) -> usize {
        encoding_size(self.count) + self.bytes.len()
    }
}

impl Encode for IndirectNameMap {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.count.encode(sink);
        sink.extend(&self.bytes);
    }
}
