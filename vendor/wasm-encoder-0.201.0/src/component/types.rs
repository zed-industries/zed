use super::CORE_TYPE_SORT;
use crate::{
    encode_section, Alias, ComponentExportKind, ComponentOuterAliasKind, ComponentSection,
    ComponentSectionId, ComponentTypeRef, Encode, EntityType, ValType,
};

/// Represents the type of a core module.
#[derive(Debug, Clone, Default)]
pub struct ModuleType {
    bytes: Vec<u8>,
    num_added: u32,
    types_added: u32,
}

impl ModuleType {
    /// Creates a new core module type.
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines an import in this module type.
    pub fn import(&mut self, module: &str, name: &str, ty: EntityType) -> &mut Self {
        crate::component::imports::push_extern_name_byte(&mut self.bytes, name);
        module.encode(&mut self.bytes);
        name.encode(&mut self.bytes);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Define a type in this module type.
    ///
    /// The returned encoder must be used before adding another definition.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn ty(&mut self) -> CoreTypeEncoder {
        self.bytes.push(0x01);
        self.num_added += 1;
        self.types_added += 1;
        CoreTypeEncoder(&mut self.bytes)
    }

    /// Defines an outer core type alias in this module type.
    pub fn alias_outer_core_type(&mut self, count: u32, index: u32) -> &mut Self {
        self.bytes.push(0x02);
        self.bytes.push(CORE_TYPE_SORT);
        self.bytes.push(0x01); // outer
        count.encode(&mut self.bytes);
        index.encode(&mut self.bytes);
        self.num_added += 1;
        self.types_added += 1;
        self
    }

    /// Defines an export in this module type.
    pub fn export(&mut self, name: &str, ty: EntityType) -> &mut Self {
        self.bytes.push(0x03);
        name.encode(&mut self.bytes);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Gets the number of types that have been added to this module type.
    pub fn type_count(&self) -> u32 {
        self.types_added
    }
}

impl Encode for ModuleType {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.push(0x50);
        self.num_added.encode(sink);
        sink.extend(&self.bytes);
    }
}

/// Used to encode core types.
#[derive(Debug)]
pub struct CoreTypeEncoder<'a>(pub(crate) &'a mut Vec<u8>);

impl<'a> CoreTypeEncoder<'a> {
    /// Define a function type.
    pub fn function<P, R>(self, params: P, results: R)
    where
        P: IntoIterator<Item = ValType>,
        P::IntoIter: ExactSizeIterator,
        R: IntoIterator<Item = ValType>,
        R::IntoIter: ExactSizeIterator,
    {
        let params = params.into_iter();
        let results = results.into_iter();

        self.0.push(0x60);
        params.len().encode(self.0);
        params.for_each(|p| p.encode(self.0));
        results.len().encode(self.0);
        results.for_each(|p| p.encode(self.0));
    }

    /// Define a module type.
    pub fn module(self, ty: &ModuleType) {
        ty.encode(self.0);
    }
}

/// An encoder for the core type section of WebAssembly components.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Component, CoreTypeSection, ModuleType};
///
/// let mut types = CoreTypeSection::new();
///
/// types.module(&ModuleType::new());
///
/// let mut component = Component::new();
/// component.section(&types);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct CoreTypeSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl CoreTypeSection {
    /// Create a new core type section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of types in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Encode a type into this section.
    ///
    /// The returned encoder must be finished before adding another type.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn ty(&mut self) -> CoreTypeEncoder<'_> {
        self.num_added += 1;
        CoreTypeEncoder(&mut self.bytes)
    }

    /// Define a function type in this type section.
    pub fn function<P, R>(&mut self, params: P, results: R) -> &mut Self
    where
        P: IntoIterator<Item = ValType>,
        P::IntoIter: ExactSizeIterator,
        R: IntoIterator<Item = ValType>,
        R::IntoIter: ExactSizeIterator,
    {
        self.ty().function(params, results);
        self
    }

    /// Define a module type in this type section.
    ///
    /// Currently this is only used for core type sections in components.
    pub fn module(&mut self, ty: &ModuleType) -> &mut Self {
        self.ty().module(ty);
        self
    }
}

impl Encode for CoreTypeSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl ComponentSection for CoreTypeSection {
    fn id(&self) -> u8 {
        ComponentSectionId::CoreType.into()
    }
}

/// Represents a component type.
#[derive(Debug, Clone, Default)]
pub struct ComponentType {
    bytes: Vec<u8>,
    num_added: u32,
    core_types_added: u32,
    types_added: u32,
    instances_added: u32,
}

impl ComponentType {
    /// Creates a new component type.
    pub fn new() -> Self {
        Self::default()
    }

    /// Define a core type in this component type.
    ///
    /// The returned encoder must be used before adding another definition.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn core_type(&mut self) -> CoreTypeEncoder {
        self.bytes.push(0x00);
        self.num_added += 1;
        self.core_types_added += 1;
        CoreTypeEncoder(&mut self.bytes)
    }

    /// Define a type in this component type.
    ///
    /// The returned encoder must be used before adding another definition.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn ty(&mut self) -> ComponentTypeEncoder {
        self.bytes.push(0x01);
        self.num_added += 1;
        self.types_added += 1;
        ComponentTypeEncoder(&mut self.bytes)
    }

    /// Defines an alias for an exported item of a prior instance or an
    /// outer type.
    pub fn alias(&mut self, alias: Alias<'_>) -> &mut Self {
        self.bytes.push(0x02);
        alias.encode(&mut self.bytes);
        self.num_added += 1;
        match &alias {
            Alias::InstanceExport {
                kind: ComponentExportKind::Type,
                ..
            }
            | Alias::Outer {
                kind: ComponentOuterAliasKind::Type,
                ..
            } => self.types_added += 1,
            Alias::Outer {
                kind: ComponentOuterAliasKind::CoreType,
                ..
            } => self.core_types_added += 1,
            Alias::InstanceExport {
                kind: ComponentExportKind::Instance,
                ..
            } => self.instances_added += 1,
            _ => {}
        }
        self
    }

    /// Defines an import in this component type.
    pub fn import(&mut self, name: &str, ty: ComponentTypeRef) -> &mut Self {
        self.bytes.push(0x03);
        crate::component::imports::push_extern_name_byte(&mut self.bytes, name);
        name.encode(&mut self.bytes);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        match ty {
            ComponentTypeRef::Type(..) => self.types_added += 1,
            ComponentTypeRef::Instance(..) => self.instances_added += 1,
            _ => {}
        }
        self
    }

    /// Defines an export in this component type.
    pub fn export(&mut self, name: &str, ty: ComponentTypeRef) -> &mut Self {
        self.bytes.push(0x04);
        crate::component::imports::push_extern_name_byte(&mut self.bytes, name);
        name.encode(&mut self.bytes);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        match ty {
            ComponentTypeRef::Type(..) => self.types_added += 1,
            ComponentTypeRef::Instance(..) => self.instances_added += 1,
            _ => {}
        }
        self
    }

    /// Gets the number of core types that have been added to this component type.
    pub fn core_type_count(&self) -> u32 {
        self.core_types_added
    }

    /// Gets the number of types that have been added or aliased in this component type.
    pub fn type_count(&self) -> u32 {
        self.types_added
    }

    /// Gets the number of instances that have been defined in this component
    /// type through imports, exports, or aliases.
    pub fn instance_count(&self) -> u32 {
        self.instances_added
    }
}

impl Encode for ComponentType {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.push(0x41);
        self.num_added.encode(sink);
        sink.extend(&self.bytes);
    }
}

/// Represents an instance type.
#[derive(Debug, Clone, Default)]
pub struct InstanceType(ComponentType);

impl InstanceType {
    /// Creates a new instance type.
    pub fn new() -> Self {
        Self::default()
    }

    /// Define a core type in this instance type.
    ///
    /// The returned encoder must be used before adding another definition.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn core_type(&mut self) -> CoreTypeEncoder {
        self.0.core_type()
    }

    /// Define a type in this instance type.
    ///
    /// The returned encoder must be used before adding another definition.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn ty(&mut self) -> ComponentTypeEncoder {
        self.0.ty()
    }

    /// Defines an outer core type alias in this component type.
    pub fn alias(&mut self, alias: Alias<'_>) -> &mut Self {
        self.0.alias(alias);
        self
    }

    /// Defines an export in this instance type.
    pub fn export(&mut self, name: &str, ty: ComponentTypeRef) -> &mut Self {
        self.0.export(name, ty);
        self
    }

    /// Gets the number of core types that have been added to this instance type.
    pub fn core_type_count(&self) -> u32 {
        self.0.core_types_added
    }

    /// Gets the number of types that have been added or aliased in this instance type.
    pub fn type_count(&self) -> u32 {
        self.0.types_added
    }

    /// Gets the number of instances that have been imported or exported or
    /// aliased in this instance type.
    pub fn instance_count(&self) -> u32 {
        self.0.instances_added
    }

    /// Returns whether or not this instance type is empty.
    pub fn is_empty(&self) -> bool {
        self.0.num_added == 0
    }

    /// Returns the number of entries added to this instance types.
    pub fn len(&self) -> u32 {
        self.0.num_added
    }
}

impl Encode for InstanceType {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.push(0x42);
        self.0.num_added.encode(sink);
        sink.extend(&self.0.bytes);
    }
}

/// Used to encode component function types.
#[derive(Debug)]
pub struct ComponentFuncTypeEncoder<'a>(&'a mut Vec<u8>);

impl<'a> ComponentFuncTypeEncoder<'a> {
    fn new(sink: &'a mut Vec<u8>) -> Self {
        sink.push(0x40);
        Self(sink)
    }

    /// Defines named parameters.
    ///
    /// Parameters must be defined before defining results.
    pub fn params<'b, P, T>(&mut self, params: P) -> &mut Self
    where
        P: IntoIterator<Item = (&'b str, T)>,
        P::IntoIter: ExactSizeIterator,
        T: Into<ComponentValType>,
    {
        let params = params.into_iter();
        params.len().encode(self.0);
        for (name, ty) in params {
            name.encode(self.0);
            ty.into().encode(self.0);
        }
        self
    }

    /// Defines a single unnamed result.
    ///
    /// This method cannot be used with `results`.
    pub fn result(&mut self, ty: impl Into<ComponentValType>) -> &mut Self {
        self.0.push(0x00);
        ty.into().encode(self.0);
        self
    }

    /// Defines named results.
    ///
    /// This method cannot be used with `result`.
    pub fn results<'b, R, T>(&mut self, results: R) -> &mut Self
    where
        R: IntoIterator<Item = (&'b str, T)>,
        R::IntoIter: ExactSizeIterator,
        T: Into<ComponentValType>,
    {
        self.0.push(0x01);
        let results = results.into_iter();
        results.len().encode(self.0);
        for (name, ty) in results {
            name.encode(self.0);
            ty.into().encode(self.0);
        }
        self
    }
}

/// Used to encode component and instance types.
#[derive(Debug)]
pub struct ComponentTypeEncoder<'a>(&'a mut Vec<u8>);

impl<'a> ComponentTypeEncoder<'a> {
    /// Define a component type.
    pub fn component(self, ty: &ComponentType) {
        ty.encode(self.0);
    }

    /// Define an instance type.
    pub fn instance(self, ty: &InstanceType) {
        ty.encode(self.0);
    }

    /// Define a function type.
    pub fn function(self) -> ComponentFuncTypeEncoder<'a> {
        ComponentFuncTypeEncoder::new(self.0)
    }

    /// Define a defined component type.
    ///
    /// The returned encoder must be used before adding another type.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn defined_type(self) -> ComponentDefinedTypeEncoder<'a> {
        ComponentDefinedTypeEncoder(self.0)
    }

    /// Define a resource type.
    pub fn resource(self, rep: ValType, dtor: Option<u32>) {
        self.0.push(0x3f);
        rep.encode(self.0);
        match dtor {
            Some(i) => {
                self.0.push(0x01);
                i.encode(self.0);
            }
            None => self.0.push(0x00),
        }
    }
}

/// Represents a primitive component value type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveValType {
    /// The type is a boolean.
    Bool,
    /// The type is a signed 8-bit integer.
    S8,
    /// The type is an unsigned 8-bit integer.
    U8,
    /// The type is a signed 16-bit integer.
    S16,
    /// The type is an unsigned 16-bit integer.
    U16,
    /// The type is a signed 32-bit integer.
    S32,
    /// The type is an unsigned 32-bit integer.
    U32,
    /// The type is a signed 64-bit integer.
    S64,
    /// The type is an unsigned 64-bit integer.
    U64,
    /// The type is a 32-bit floating point number.
    Float32,
    /// The type is a 64-bit floating point number.
    Float64,
    /// The type is a Unicode character.
    Char,
    /// The type is a string.
    String,
}

impl Encode for PrimitiveValType {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.push(match self {
            Self::Bool => 0x7f,
            Self::S8 => 0x7e,
            Self::U8 => 0x7d,
            Self::S16 => 0x7c,
            Self::U16 => 0x7b,
            Self::S32 => 0x7a,
            Self::U32 => 0x79,
            Self::S64 => 0x78,
            Self::U64 => 0x77,
            Self::Float32 => 0x76,
            Self::Float64 => 0x75,
            Self::Char => 0x74,
            Self::String => 0x73,
        });
    }
}

#[cfg(feature = "wasmparser")]
impl From<wasmparser::PrimitiveValType> for PrimitiveValType {
    fn from(ty: wasmparser::PrimitiveValType) -> Self {
        match ty {
            wasmparser::PrimitiveValType::Bool => PrimitiveValType::Bool,
            wasmparser::PrimitiveValType::S8 => PrimitiveValType::S8,
            wasmparser::PrimitiveValType::U8 => PrimitiveValType::U8,
            wasmparser::PrimitiveValType::S16 => PrimitiveValType::S16,
            wasmparser::PrimitiveValType::U16 => PrimitiveValType::U16,
            wasmparser::PrimitiveValType::S32 => PrimitiveValType::S32,
            wasmparser::PrimitiveValType::U32 => PrimitiveValType::U32,
            wasmparser::PrimitiveValType::S64 => PrimitiveValType::S64,
            wasmparser::PrimitiveValType::U64 => PrimitiveValType::U64,
            wasmparser::PrimitiveValType::Float32 => PrimitiveValType::Float32,
            wasmparser::PrimitiveValType::Float64 => PrimitiveValType::Float64,
            wasmparser::PrimitiveValType::Char => PrimitiveValType::Char,
            wasmparser::PrimitiveValType::String => PrimitiveValType::String,
        }
    }
}

/// Represents a component value type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentValType {
    /// The value is a primitive type.
    Primitive(PrimitiveValType),
    /// The value is to a defined value type.
    ///
    /// The type index must be to a value type.
    Type(u32),
}

impl Encode for ComponentValType {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Self::Primitive(ty) => ty.encode(sink),
            Self::Type(index) => (*index as i64).encode(sink),
        }
    }
}

impl From<PrimitiveValType> for ComponentValType {
    fn from(ty: PrimitiveValType) -> Self {
        Self::Primitive(ty)
    }
}

/// Used for encoding component defined types.
#[derive(Debug)]
pub struct ComponentDefinedTypeEncoder<'a>(&'a mut Vec<u8>);

impl ComponentDefinedTypeEncoder<'_> {
    /// Define a primitive value type.
    pub fn primitive(self, ty: PrimitiveValType) {
        ty.encode(self.0);
    }

    /// Define a record type.
    pub fn record<'a, F, T>(self, fields: F)
    where
        F: IntoIterator<Item = (&'a str, T)>,
        F::IntoIter: ExactSizeIterator,
        T: Into<ComponentValType>,
    {
        let fields = fields.into_iter();
        self.0.push(0x72);
        fields.len().encode(self.0);
        for (name, ty) in fields {
            name.encode(self.0);
            ty.into().encode(self.0);
        }
    }

    /// Define a variant type.
    pub fn variant<'a, C>(self, cases: C)
    where
        C: IntoIterator<Item = (&'a str, Option<ComponentValType>, Option<u32>)>,
        C::IntoIter: ExactSizeIterator,
    {
        let cases = cases.into_iter();
        self.0.push(0x71);
        cases.len().encode(self.0);
        for (name, ty, refines) in cases {
            name.encode(self.0);
            ty.encode(self.0);
            refines.encode(self.0);
        }
    }

    /// Define a list type.
    pub fn list(self, ty: impl Into<ComponentValType>) {
        self.0.push(0x70);
        ty.into().encode(self.0);
    }

    /// Define a tuple type.
    pub fn tuple<I, T>(self, types: I)
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
        T: Into<ComponentValType>,
    {
        let types = types.into_iter();
        self.0.push(0x6F);
        types.len().encode(self.0);
        for ty in types {
            ty.into().encode(self.0);
        }
    }

    /// Define a flags type.
    pub fn flags<'a, I>(self, names: I)
    where
        I: IntoIterator<Item = &'a str>,
        I::IntoIter: ExactSizeIterator,
    {
        let names = names.into_iter();
        self.0.push(0x6E);
        names.len().encode(self.0);
        for name in names {
            name.encode(self.0);
        }
    }

    /// Define an enum type.
    pub fn enum_type<'a, I>(self, tags: I)
    where
        I: IntoIterator<Item = &'a str>,
        I::IntoIter: ExactSizeIterator,
    {
        let tags = tags.into_iter();
        self.0.push(0x6D);
        tags.len().encode(self.0);
        for tag in tags {
            tag.encode(self.0);
        }
    }

    /// Define an option type.
    pub fn option(self, ty: impl Into<ComponentValType>) {
        self.0.push(0x6B);
        ty.into().encode(self.0);
    }

    /// Define a result type.
    pub fn result(self, ok: Option<ComponentValType>, err: Option<ComponentValType>) {
        self.0.push(0x6A);
        ok.encode(self.0);
        err.encode(self.0);
    }

    /// Define a `own` handle type
    pub fn own(self, idx: u32) {
        self.0.push(0x69);
        idx.encode(self.0);
    }

    /// Define a `borrow` handle type
    pub fn borrow(self, idx: u32) {
        self.0.push(0x68);
        idx.encode(self.0);
    }
}

/// An encoder for the type section of WebAssembly components.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Component, ComponentTypeSection, PrimitiveValType};
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
/// let mut component = Component::new();
/// component.section(&types);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct ComponentTypeSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl ComponentTypeSection {
    /// Create a new component type section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of types in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Encode a type into this section.
    ///
    /// The returned encoder must be finished before adding another type.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn ty(&mut self) -> ComponentTypeEncoder<'_> {
        self.num_added += 1;
        ComponentTypeEncoder(&mut self.bytes)
    }

    /// Define a component type in this type section.
    pub fn component(&mut self, ty: &ComponentType) -> &mut Self {
        self.ty().component(ty);
        self
    }

    /// Define an instance type in this type section.
    pub fn instance(&mut self, ty: &InstanceType) -> &mut Self {
        self.ty().instance(ty);
        self
    }

    /// Define a function type in this type section.
    pub fn function(&mut self) -> ComponentFuncTypeEncoder<'_> {
        self.ty().function()
    }

    /// Add a component defined type to this type section.
    ///
    /// The returned encoder must be used before adding another type.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn defined_type(&mut self) -> ComponentDefinedTypeEncoder<'_> {
        self.ty().defined_type()
    }

    /// Defines a new resource type.
    pub fn resource(&mut self, rep: ValType, dtor: Option<u32>) -> &mut Self {
        self.ty().resource(rep, dtor);
        self
    }
}

impl Encode for ComponentTypeSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl ComponentSection for ComponentTypeSection {
    fn id(&self) -> u8 {
        ComponentSectionId::Type.into()
    }
}
