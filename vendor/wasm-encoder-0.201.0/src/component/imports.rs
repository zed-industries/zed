use crate::{
    encode_section, ComponentExportKind, ComponentSection, ComponentSectionId, ComponentValType,
    Encode,
};

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
///   .result(PrimitiveValType::String);
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
        push_extern_name_byte(&mut self.bytes, name);
        name.encode(&mut self.bytes);
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
/// This function is a compatibility shim for the time being while this change
/// is being rolled out. That PR is technically a spec-breaking change relative
/// to prior but we want old tooling to continue to work with new modules. To
/// handle that names that look like IDs, `a:b/c`, get an 0x01 prefix instead of
/// the spec-defined 0x00 prefix. That makes components produced by current
/// versions of this crate compatible with older parsers.
///
/// Note that wasmparser has a similar case where it parses either 0x01 or 0x00.
/// That means that the selection of a byte here doesn't actually matter for
/// wasmparser's own validation. Eventually this will go away and an 0x00 byte
/// will always be emitted to align with the spec.
pub(crate) fn push_extern_name_byte(bytes: &mut Vec<u8>, name: &str) {
    if name.contains(':') {
        bytes.push(0x01);
    } else {
        bytes.push(0x00);
    }
}
