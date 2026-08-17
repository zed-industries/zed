use super::{COMPONENT_SORT, CORE_MODULE_SORT, CORE_SORT, CORE_TYPE_SORT, TYPE_SORT};
use crate::{
    encode_section, ComponentExportKind, ComponentSection, ComponentSectionId, Encode, ExportKind,
};

/// Represents the kinds of outer aliasable items in a component.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentOuterAliasKind {
    /// The alias is to a core module.
    CoreModule,
    /// The alias is to a core type.
    CoreType,
    /// The alias is to a type.
    Type,
    /// The alias is to a component.
    Component,
}

impl Encode for ComponentOuterAliasKind {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Self::CoreModule => {
                sink.push(CORE_SORT);
                sink.push(CORE_MODULE_SORT);
            }
            Self::CoreType => {
                sink.push(CORE_SORT);
                sink.push(CORE_TYPE_SORT);
            }
            Self::Type => sink.push(TYPE_SORT),
            Self::Component => sink.push(COMPONENT_SORT),
        }
    }
}

/// An encoder for the alias section of WebAssembly component.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Component, Alias, ComponentAliasSection, ComponentExportKind, ComponentOuterAliasKind};
///
/// let mut aliases = ComponentAliasSection::new();
/// aliases.alias(Alias::InstanceExport { instance: 0, kind: ComponentExportKind::Func, name: "f" });
/// aliases.alias(Alias::Outer { count: 0, kind: ComponentOuterAliasKind::Type, index: 1 });
///
/// let mut component = Component::new();
/// component.section(&aliases);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct ComponentAliasSection {
    bytes: Vec<u8>,
    num_added: u32,
}

/// Different forms of aliases that can be inserted into a
/// [`ComponentAliasSection`].
#[derive(Copy, Clone, Debug)]
pub enum Alias<'a> {
    /// An alias of a component instance export.
    InstanceExport {
        /// The index of the component instance that's being aliased from.
        instance: u32,
        /// The kind of item that's being extracted from the component
        /// instance.
        kind: ComponentExportKind,
        /// The name of the export that's being aliased.
        name: &'a str,
    },
    /// Same as `InstanceExport`, but for core instances.
    #[allow(missing_docs)]
    CoreInstanceExport {
        instance: u32,
        kind: ExportKind,
        name: &'a str,
    },
    /// Aliasing an item from an outer component.
    Outer {
        /// The kind of item being aliased, either a type or a component.
        kind: ComponentOuterAliasKind,
        /// Number of levels "up" to go to lookup the index within. Level 0 is
        /// the current scope and level 1 is the enclosing scope, and so on.
        count: u32,
        /// The index of the item to alias within the scope referenced by
        /// `count`.
        index: u32,
    },
}

impl ComponentAliasSection {
    /// Create a new alias section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of aliases in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define an alias to a component instance's export.
    pub fn alias(&mut self, alias: Alias<'_>) -> &mut Self {
        alias.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }
}

impl Encode for ComponentAliasSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl ComponentSection for ComponentAliasSection {
    fn id(&self) -> u8 {
        ComponentSectionId::Alias.into()
    }
}

impl Encode for Alias<'_> {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Alias::InstanceExport {
                instance,
                kind,
                name,
            } => {
                kind.encode(sink);
                sink.push(0x00);
                instance.encode(sink);
                name.encode(sink);
            }
            Alias::CoreInstanceExport {
                instance,
                kind,
                name,
            } => {
                sink.push(CORE_SORT);
                kind.encode(sink);
                sink.push(0x01);
                instance.encode(sink);
                name.encode(sink);
            }
            Alias::Outer { kind, count, index } => {
                kind.encode(sink);
                sink.push(0x02);
                count.encode(sink);
                index.encode(sink);
            }
        }
    }
}
