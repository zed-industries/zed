use crate::{
    ComponentExportKind, ComponentSection, ComponentSectionId, ComponentValType, Encode,
    encode_section,
};
use alloc::vec::Vec;

/// Represents the possible type bounds for type references.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TypeBounds {
    /// The type is bounded by equality to the type index specified.
    Eq(u32),
    /// This type is a fresh resource type,
    SubResource,
}

impl Encode for TypeBounds {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Self::Eq(i) => {
                sink.push(0x00);
                i.encode(sink);
            }
            Self::SubResource => sink.push(0x01),
        }
    }
}

/// Represents a reference to a type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ComponentTypeRef {
    /// The reference is to a core module type.
    ///
    /// The index is expected to be core type index to a core module type.
    Module(u32),
    /// The reference is to a function type.
    ///
    /// The index is expected to be a type index to a function type.
    Func(u32),
    /// The reference is to a value type.
    Value(ComponentValType),
    /// The reference is to a bounded type.
    Type(TypeBounds),
    /// The reference is to an instance type.
    ///
    /// The index is expected to be a type index to an instance type.
    Instance(u32),
    /// The reference is to a component type.
    ///
    /// The index is expected to be a type index to a component type.
    Component(u32),
}

impl ComponentTypeRef {
    /// Gets the export kind of the reference.
    pub fn kind(&self) -> ComponentExportKind {
        match self {
            Self::Module(_) => ComponentExportKind::Module,
            Self::Func(_) => ComponentExportKind::Func,
            Self::Value(_) => ComponentExportKind::Value,
            Self::Type(..) => ComponentExportKind::Type,
            Self::Instance(_) => ComponentExportKind::Instance,
            Self::Component(_) => ComponentExportKind::Component,
        }
    }
}

impl Encode for ComponentTypeRef {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.kind().encode(sink);

        match self {
            Self::Module(idx) | Self::Func(idx) | Self::Instance(idx) | Self::Component(idx) => {
                idx.encode(sink);
            }
            Self::Value(ty) => ty.encode(sink),
            Self::Type(bounds) => bounds.encode(sink),
        }
    }
}

/// An encoder for the import section of WebAssembly components.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Component, ComponentTypeSection, PrimitiveValType, ComponentImportSection, ComponentTypeRef};
///
/// let mut types = ComponentTypeSection::new();
///
/// // Define a function type of `[string, string] -> string`.
/// types
///   .function()
///   .params(
///     [
///       ("a", PrimitiveValType::String),
///       ("b", PrimitiveValType::String)
///     ]
///   )
///   .result(Some(PrimitiveValType::String.into()));
///
/// // This imports a function named `f` with the type defined above
/// let mut imports = ComponentImportSection::new();
/// imports.import("f", ComponentTypeRef::Func(0));
///
/// let mut component = Component::new();
/// component.section(&types);
/// component.section(&imports);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct ComponentImportSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl ComponentImportSection {
    /// Create a new component import section encoder.
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

    /// Define an import in the component import section.
    pub fn import(&mut self, name: &str, ty: ComponentTypeRef) -> &mut Self {
        encode_component_import_name(&mut self.bytes, name);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }
}

impl Encode for ComponentImportSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl ComponentSection for ComponentImportSection {
    fn id(&self) -> u8 {
        ComponentSectionId::Import.into()
    }
}

/// Prior to WebAssembly/component-model#263 import and export names were
/// discriminated with a leading byte indicating what kind of import they are.
/// After that PR though names are always prefixed with a 0x00 byte.
///
/// On 2023-10-28 in bytecodealliance/wasm-tools#1262 was landed to start
/// transitioning to "always lead with 0x00". That updated the validator/parser
/// to accept either 0x00 or 0x01 but the encoder wasn't updated at the time.
///
/// On 2024-09-03 in bytecodealliance/wasm-tools#TODO this encoder was updated
/// to always emit 0x00 as a leading byte.
///
/// This function corresponds with the `importname'` production in the
/// specification.
pub(crate) fn encode_component_import_name(bytes: &mut Vec<u8>, name: &str) {
    bytes.push(0x00);
    name.encode(bytes);
}
