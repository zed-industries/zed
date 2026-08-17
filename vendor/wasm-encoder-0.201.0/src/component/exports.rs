use super::{
    COMPONENT_SORT, CORE_MODULE_SORT, CORE_SORT, FUNCTION_SORT, INSTANCE_SORT, TYPE_SORT,
    VALUE_SORT,
};
use crate::{encode_section, ComponentSection, ComponentSectionId, ComponentTypeRef, Encode};

/// Represents the kind of an export from a WebAssembly component.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentExportKind {
    /// The export is a core module.
    Module,
    /// The export is a function.
    Func,
    /// The export is a value.
    Value,
    /// The export is a type.
    Type,
    /// The export is an instance.
    Instance,
    /// The export is a component.
    Component,
}

impl Encode for ComponentExportKind {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Self::Module => {
                sink.push(CORE_SORT);
                sink.push(CORE_MODULE_SORT);
            }
            Self::Func => {
                sink.push(FUNCTION_SORT);
            }
            Self::Value => {
                sink.push(VALUE_SORT);
            }
            Self::Type => {
                sink.push(TYPE_SORT);
            }
            Self::Instance => {
                sink.push(INSTANCE_SORT);
            }
            Self::Component => {
                sink.push(COMPONENT_SORT);
            }
        }
    }
}

/// An encoder for the export section of WebAssembly component.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Component, ComponentExportSection, ComponentExportKind};
///
/// // This exports a function named "foo"
/// let mut exports = ComponentExportSection::new();
/// exports.export("foo", ComponentExportKind::Func, 0, None);
///
/// let mut component = Component::new();
/// component.section(&exports);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct ComponentExportSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl ComponentExportSection {
    /// Create a new component export section encoder.
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
    pub fn export(
        &mut self,
        name: &str,
        kind: ComponentExportKind,
        index: u32,
        ty: Option<ComponentTypeRef>,
    ) -> &mut Self {
        crate::component::imports::push_extern_name_byte(&mut self.bytes, name);
        name.encode(&mut self.bytes);
        kind.encode(&mut self.bytes);
        index.encode(&mut self.bytes);
        match ty {
            Some(ty) => {
                self.bytes.push(0x01);
                ty.encode(&mut self.bytes);
            }
            None => {
                self.bytes.push(0x00);
            }
        }
        self.num_added += 1;
        self
    }
}

impl Encode for ComponentExportSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl ComponentSection for ComponentExportSection {
    fn id(&self) -> u8 {
        ComponentSectionId::Export.into()
    }
}
