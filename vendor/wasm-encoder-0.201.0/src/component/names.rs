use std::borrow::Cow;

use super::*;
use crate::{encoding_size, ExportKind, NameMap, SectionId};

/// Encoding for the `component-name` custom section which assigns
/// human-readable names to items within a component.
#[derive(Clone, Debug, Default)]
pub struct ComponentNameSection {
    bytes: Vec<u8>,
}

enum Subsection {
    Component,
    Decls,
}

impl ComponentNameSection {
    /// Creates a new blank `name` custom section.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a component name subsection to this section.
    ///
    /// This will indicate that the name of the entire component should be the
    /// `name` specified. Note that this should be encoded first before other
    /// subsections.
    pub fn component(&mut self, name: &str) {
        let len = encoding_size(u32::try_from(name.len()).unwrap());
        self.subsection_header(Subsection::Component, len + name.len());
        name.encode(&mut self.bytes);
    }

    /// Appends a decls name subsection to name core functions within the
    /// component.
    pub fn core_funcs(&mut self, names: &NameMap) {
        self.core_decls(ExportKind::Func as u8, names)
    }

    /// Appends a decls name subsection to name core tables within the
    /// component.
    pub fn core_tables(&mut self, names: &NameMap) {
        self.core_decls(ExportKind::Table as u8, names)
    }

    /// Appends a decls name subsection to name core memories within the
    /// component.
    pub fn core_memories(&mut self, names: &NameMap) {
        self.core_decls(ExportKind::Memory as u8, names)
    }

    /// Appends a decls name subsection to name core globals within the
    /// component.
    pub fn core_globals(&mut self, names: &NameMap) {
        self.core_decls(ExportKind::Global as u8, names)
    }

    /// Appends a decls name subsection to name core types within the
    /// component.
    pub fn core_types(&mut self, names: &NameMap) {
        self.core_decls(CORE_TYPE_SORT, names)
    }

    /// Appends a decls name subsection to name core modules within the
    /// component.
    pub fn core_modules(&mut self, names: &NameMap) {
        self.core_decls(CORE_MODULE_SORT, names)
    }

    /// Appends a decls name subsection to name core instances within the
    /// component.
    pub fn core_instances(&mut self, names: &NameMap) {
        self.core_decls(CORE_INSTANCE_SORT, names)
    }

    /// Appends a decls name subsection to name component functions within the
    /// component.
    pub fn funcs(&mut self, names: &NameMap) {
        self.component_decls(FUNCTION_SORT, names)
    }

    /// Appends a decls name subsection to name component values within the
    /// component.
    pub fn values(&mut self, names: &NameMap) {
        self.component_decls(VALUE_SORT, names)
    }

    /// Appends a decls name subsection to name component type within the
    /// component.
    pub fn types(&mut self, names: &NameMap) {
        self.component_decls(TYPE_SORT, names)
    }

    /// Appends a decls name subsection to name components within the
    /// component.
    pub fn components(&mut self, names: &NameMap) {
        self.component_decls(COMPONENT_SORT, names)
    }

    /// Appends a decls name subsection to name component instances within the
    /// component.
    pub fn instances(&mut self, names: &NameMap) {
        self.component_decls(INSTANCE_SORT, names)
    }

    fn component_decls(&mut self, kind: u8, names: &NameMap) {
        self.subsection_header(Subsection::Decls, 1 + names.size());
        self.bytes.push(kind);
        names.encode(&mut self.bytes);
    }

    fn core_decls(&mut self, kind: u8, names: &NameMap) {
        self.subsection_header(Subsection::Decls, 2 + names.size());
        self.bytes.push(CORE_SORT);
        self.bytes.push(kind);
        names.encode(&mut self.bytes);
    }

    fn subsection_header(&mut self, id: Subsection, len: usize) {
        self.bytes.push(id as u8);
        len.encode(&mut self.bytes);
    }

    /// Returns whether this section is empty, or nothing has been encoded.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// View the encoded section as a CustomSection.
    pub fn as_custom<'a>(&'a self) -> CustomSection<'a> {
        CustomSection {
            name: "component-name".into(),
            data: Cow::Borrowed(&self.bytes),
        }
    }
}

impl Encode for ComponentNameSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.as_custom().encode(sink);
    }
}

impl ComponentSection for ComponentNameSection {
    fn id(&self) -> u8 {
        SectionId::Custom.into()
    }
}
