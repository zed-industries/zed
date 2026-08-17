use crate::component::{MAX_FLAT_PARAMS, MAX_FLAT_RESULTS};
use crate::{EntityType, ModuleInternedTypeIndex, ModuleTypes, PrimaryMap};
use crate::{TypeTrace, prelude::*};
use core::hash::{Hash, Hasher};
use core::ops::Index;
use serde_derive::{Deserialize, Serialize};
use wasmparser::component_types::ComponentAnyTypeId;
use wasmtime_component_util::{DiscriminantSize, FlagsSize};

pub use crate::StaticModuleIndex;

macro_rules! indices {
    ($(
        $(#[$a:meta])*
        pub struct $name:ident(u32);
    )*) => ($(
        $(#[$a])*
        #[derive(
            Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug,
            Serialize, Deserialize,
        )]
        #[repr(transparent)]
        pub struct $name(u32);
        cranelift_entity::entity_impl!($name);
    )*);
}

indices! {
    // ========================================================================
    // These indices are used during compile time only when we're translating a
    // component at this time. The actual indices are not persisted beyond the
    // compile phase to when we're actually working with the component at
    // runtime.

    /// Index within a component's component type index space.
    pub struct ComponentTypeIndex(u32);

    /// Index within a component's module index space.
    pub struct ModuleIndex(u32);

    /// Index within a component's component index space.
    pub struct ComponentIndex(u32);

    /// Index within a component's module instance index space.
    pub struct ModuleInstanceIndex(u32);

    /// Index within a component's component instance index space.
    pub struct ComponentInstanceIndex(u32);

    /// Index within a component's component function index space.
    pub struct ComponentFuncIndex(u32);

    // ========================================================================
    // These indices are used to lookup type information within a `TypeTables`
    // structure. These represent generally deduplicated type information across
    // an entire component and are a form of RTTI in a sense.

    /// Index pointing to a component's type (exports/imports with
    /// component-model types)
    pub struct TypeComponentIndex(u32);

    /// Index pointing to a component instance's type (exports with
    /// component-model types, no imports)
    pub struct TypeComponentInstanceIndex(u32);

    /// Index pointing to a core wasm module's type (exports/imports with
    /// core wasm types)
    pub struct TypeModuleIndex(u32);

    /// Index pointing to a component model function type with arguments/result
    /// as interface types.
    pub struct TypeFuncIndex(u32);

    /// Index pointing to a record type in the component model (aka a struct).
    pub struct TypeRecordIndex(u32);
    /// Index pointing to a variant type in the component model (aka an enum).
    pub struct TypeVariantIndex(u32);
    /// Index pointing to a tuple type in the component model.
    pub struct TypeTupleIndex(u32);
    /// Index pointing to a flags type in the component model.
    pub struct TypeFlagsIndex(u32);
    /// Index pointing to an enum type in the component model.
    pub struct TypeEnumIndex(u32);
    /// Index pointing to an option type in the component model (aka a
    /// `Option<T, E>`)
    pub struct TypeOptionIndex(u32);
    /// Index pointing to an result type in the component model (aka a
    /// `Result<T, E>`)
    pub struct TypeResultIndex(u32);
    /// Index pointing to a list type in the component model.
    pub struct TypeListIndex(u32);
    /// Index pointing to a future type in the component model.
    pub struct TypeFutureIndex(u32);

    /// Index pointing to a future table within a component.
    ///
    /// This is analogous to `TypeResourceTableIndex` in that it tracks
    /// ownership of futures within each (sub)component instance.
    pub struct TypeFutureTableIndex(u32);

    /// Index pointing to a stream type in the component model.
    pub struct TypeStreamIndex(u32);

    /// Index pointing to a stream table within a component.
    ///
    /// This is analogous to `TypeResourceTableIndex` in that it tracks
    /// ownership of stream within each (sub)component instance.
    pub struct TypeStreamTableIndex(u32);

    /// Index pointing to a error context table within a component.
    ///
    /// This is analogous to `TypeResourceTableIndex` in that it tracks
    /// ownership of error contexts within each (sub)component instance.
    pub struct TypeComponentLocalErrorContextTableIndex(u32);

    /// Index pointing to a (component) globally tracked error context table entry
    ///
    /// Unlike [`TypeComponentLocalErrorContextTableIndex`], this index refers to
    /// the global state table for error contexts at the level of the entire component,
    /// not just a subcomponent.
    pub struct TypeComponentGlobalErrorContextTableIndex(u32);

    /// Index pointing to a resource table within a component.
    ///
    /// This is a Wasmtime-specific type index which isn't part of the component
    /// model per-se (or at least not the binary format). This index represents
    /// a pointer to a table of runtime information tracking state for resources
    /// within a component. Tables are generated per-resource-per-component
    /// meaning that if the exact same resource is imported into 4 subcomponents
    /// then that's 5 tables: one for the defining component and one for each
    /// subcomponent.
    ///
    /// All resource-related intrinsics operate on table-local indices which
    /// indicate which table the intrinsic is modifying. Each resource table has
    /// an origin resource type (defined by `ResourceIndex`) along with a
    /// component instance that it's recorded for.
    pub struct TypeResourceTableIndex(u32);

    /// Index pointing to a resource within a component.
    ///
    /// This index space covers all unique resource type definitions. For
    /// example all unique imports come first and then all locally-defined
    /// resources come next. Note that this does not count the number of runtime
    /// tables required to track resources (that's `TypeResourceTableIndex`
    /// instead). Instead this is a count of the number of unique
    /// `(type (resource (rep ..)))` declarations within a component, plus
    /// imports.
    ///
    /// This is then used for correlating various information such as
    /// destructors, origin information, etc.
    pub struct ResourceIndex(u32);

    /// Index pointing to a local resource defined within a component.
    ///
    /// This is similar to `FooIndex` and `DefinedFooIndex` for core wasm and
    /// the idea here is that this is guaranteed to be a wasm-defined resource
    /// which is connected to a component instance for example.
    pub struct DefinedResourceIndex(u32);

    // ========================================================================
    // Index types used to identify modules and components during compilation.

    /// Index into a "closed over variables" list for components used to
    /// implement outer aliases. For more information on this see the
    /// documentation for the `LexicalScope` structure.
    pub struct ModuleUpvarIndex(u32);

    /// Same as `ModuleUpvarIndex` but for components.
    pub struct ComponentUpvarIndex(u32);

    /// Same as `StaticModuleIndex` but for components.
    pub struct StaticComponentIndex(u32);

    // ========================================================================
    // These indices are actually used at runtime when managing a component at
    // this time.

    /// Index that represents a core wasm instance created at runtime.
    ///
    /// This is used to keep track of when instances are created and is able to
    /// refer back to previously created instances for exports and such.
    pub struct RuntimeInstanceIndex(u32);

    /// Same as `RuntimeInstanceIndex` but tracks component instances instead.
    pub struct RuntimeComponentInstanceIndex(u32);

    /// Used to index imports into a `Component`
    ///
    /// This does not correspond to anything in the binary format for the
    /// component model.
    pub struct ImportIndex(u32);

    /// Index that represents a leaf item imported into a component where a
    /// "leaf" means "not an instance".
    ///
    /// This does not correspond to anything in the binary format for the
    /// component model.
    pub struct RuntimeImportIndex(u32);

    /// Index that represents a lowered host function and is used to represent
    /// host function lowerings with options and such.
    ///
    /// This does not correspond to anything in the binary format for the
    /// component model.
    pub struct LoweredIndex(u32);

    /// Index representing a linear memory extracted from a wasm instance
    /// which is stored in a `VMComponentContext`. This is used to deduplicate
    /// references to the same linear memory where it's only stored once in a
    /// `VMComponentContext`.
    ///
    /// This does not correspond to anything in the binary format for the
    /// component model.
    pub struct RuntimeMemoryIndex(u32);

    /// Same as `RuntimeMemoryIndex` except for the `realloc` function.
    pub struct RuntimeReallocIndex(u32);

    /// Same as `RuntimeMemoryIndex` except for the `callback` function.
    pub struct RuntimeCallbackIndex(u32);

    /// Same as `RuntimeMemoryIndex` except for the `post-return` function.
    pub struct RuntimePostReturnIndex(u32);

    /// Index representing a table extracted from a wasm instance which is
    /// stored in a `VMComponentContext`. This is used to deduplicate references
    /// to the same table when it's only stored once in a `VMComponentContext`.
    ///
    /// This does not correspond to anything in the binary format for the
    /// component model.
    pub struct RuntimeTableIndex(u32);

    /// Index for all trampolines that are compiled in Cranelift for a
    /// component.
    ///
    /// This is used to point to various bits of metadata within a compiled
    /// component and is stored in the final compilation artifact. This does not
    /// have a direct correspondence to any wasm definition.
    pub struct TrampolineIndex(u32);

    /// An index into `Component::export_items` at the end of compilation.
    pub struct ExportIndex(u32);

    /// An index into `Component::options` at the end of compilation.
    pub struct OptionsIndex(u32);
}

// Reexport for convenience some core-wasm indices which are also used in the
// component model, typically for when aliasing exports of core wasm modules.
pub use crate::{FuncIndex, GlobalIndex, MemoryIndex, TableIndex};

/// Equivalent of `EntityIndex` but for the component model instead of core
/// wasm.
#[derive(Debug, Clone, Copy)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum ComponentItem {
    Func(ComponentFuncIndex),
    Module(ModuleIndex),
    Component(ComponentIndex),
    ComponentInstance(ComponentInstanceIndex),
    Type(ComponentAnyTypeId),
}

/// Runtime information about the type information contained within a component.
///
/// One of these is created per top-level component which describes all of the
/// types contained within the top-level component itself. Each sub-component
/// will have a pointer to this value as well.
#[derive(Default, Serialize, Deserialize)]
pub struct ComponentTypes {
    pub(super) modules: PrimaryMap<TypeModuleIndex, TypeModule>,
    pub(super) components: PrimaryMap<TypeComponentIndex, TypeComponent>,
    pub(super) component_instances: PrimaryMap<TypeComponentInstanceIndex, TypeComponentInstance>,
    pub(super) functions: PrimaryMap<TypeFuncIndex, TypeFunc>,
    pub(super) lists: PrimaryMap<TypeListIndex, TypeList>,
    pub(super) records: PrimaryMap<TypeRecordIndex, TypeRecord>,
    pub(super) variants: PrimaryMap<TypeVariantIndex, TypeVariant>,
    pub(super) tuples: PrimaryMap<TypeTupleIndex, TypeTuple>,
    pub(super) enums: PrimaryMap<TypeEnumIndex, TypeEnum>,
    pub(super) flags: PrimaryMap<TypeFlagsIndex, TypeFlags>,
    pub(super) options: PrimaryMap<TypeOptionIndex, TypeOption>,
    pub(super) results: PrimaryMap<TypeResultIndex, TypeResult>,
    pub(super) resource_tables: PrimaryMap<TypeResourceTableIndex, TypeResourceTable>,
    pub(super) module_types: Option<ModuleTypes>,
    pub(super) futures: PrimaryMap<TypeFutureIndex, TypeFuture>,
    pub(super) future_tables: PrimaryMap<TypeFutureTableIndex, TypeFutureTable>,
    pub(super) streams: PrimaryMap<TypeStreamIndex, TypeStream>,
    pub(super) stream_tables: PrimaryMap<TypeStreamTableIndex, TypeStreamTable>,
    pub(super) error_context_tables:
        PrimaryMap<TypeComponentLocalErrorContextTableIndex, TypeErrorContextTable>,
}

impl TypeTrace for ComponentTypes {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(crate::EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for (_, m) in &self.modules {
            m.trace(func)?;
        }
        if let Some(m) = self.module_types.as_ref() {
            m.trace(func)?;
        }
        Ok(())
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut crate::EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for (_, m) in &mut self.modules {
            m.trace_mut(func)?;
        }
        if let Some(m) = self.module_types.as_mut() {
            m.trace_mut(func)?;
        }
        Ok(())
    }
}

impl ComponentTypes {
    /// Returns the core wasm module types known within this component.
    pub fn module_types(&self) -> &ModuleTypes {
        self.module_types.as_ref().unwrap()
    }

    /// Returns the core wasm module types known within this component.
    pub fn module_types_mut(&mut self) -> &mut ModuleTypes {
        self.module_types.as_mut().unwrap()
    }

    /// Returns the canonical ABI information about the specified type.
    pub fn canonical_abi(&self, ty: &InterfaceType) -> &CanonicalAbiInfo {
        match ty {
            InterfaceType::U8 | InterfaceType::S8 | InterfaceType::Bool => {
                &CanonicalAbiInfo::SCALAR1
            }

            InterfaceType::U16 | InterfaceType::S16 => &CanonicalAbiInfo::SCALAR2,

            InterfaceType::U32
            | InterfaceType::S32
            | InterfaceType::Float32
            | InterfaceType::Char
            | InterfaceType::Own(_)
            | InterfaceType::Borrow(_)
            | InterfaceType::Future(_)
            | InterfaceType::Stream(_)
            | InterfaceType::ErrorContext(_) => &CanonicalAbiInfo::SCALAR4,

            InterfaceType::U64 | InterfaceType::S64 | InterfaceType::Float64 => {
                &CanonicalAbiInfo::SCALAR8
            }

            InterfaceType::String | InterfaceType::List(_) => &CanonicalAbiInfo::POINTER_PAIR,

            InterfaceType::Record(i) => &self[*i].abi,
            InterfaceType::Variant(i) => &self[*i].abi,
            InterfaceType::Tuple(i) => &self[*i].abi,
            InterfaceType::Flags(i) => &self[*i].abi,
            InterfaceType::Enum(i) => &self[*i].abi,
            InterfaceType::Option(i) => &self[*i].abi,
            InterfaceType::Result(i) => &self[*i].abi,
        }
    }

    /// Adds a new `table` to the list of resource tables for this component.
    pub fn push_resource_table(&mut self, table: TypeResourceTable) -> TypeResourceTableIndex {
        self.resource_tables.push(table)
    }
}

macro_rules! impl_index {
    ($(impl Index<$ty:ident> for ComponentTypes { $output:ident => $field:ident })*) => ($(
        impl core::ops::Index<$ty> for ComponentTypes {
            type Output = $output;
            #[inline]
            fn index(&self, idx: $ty) -> &$output {
                &self.$field[idx]
            }
        }

        #[cfg(feature = "compile")]
        impl core::ops::Index<$ty> for super::ComponentTypesBuilder {
            type Output = $output;
            #[inline]
            fn index(&self, idx: $ty) -> &$output {
                &self.component_types()[idx]
            }
        }
    )*)
}

impl_index! {
    impl Index<TypeModuleIndex> for ComponentTypes { TypeModule => modules }
    impl Index<TypeComponentIndex> for ComponentTypes { TypeComponent => components }
    impl Index<TypeComponentInstanceIndex> for ComponentTypes { TypeComponentInstance => component_instances }
    impl Index<TypeFuncIndex> for ComponentTypes { TypeFunc => functions }
    impl Index<TypeRecordIndex> for ComponentTypes { TypeRecord => records }
    impl Index<TypeVariantIndex> for ComponentTypes { TypeVariant => variants }
    impl Index<TypeTupleIndex> for ComponentTypes { TypeTuple => tuples }
    impl Index<TypeEnumIndex> for ComponentTypes { TypeEnum => enums }
    impl Index<TypeFlagsIndex> for ComponentTypes { TypeFlags => flags }
    impl Index<TypeOptionIndex> for ComponentTypes { TypeOption => options }
    impl Index<TypeResultIndex> for ComponentTypes { TypeResult => results }
    impl Index<TypeListIndex> for ComponentTypes { TypeList => lists }
    impl Index<TypeResourceTableIndex> for ComponentTypes { TypeResourceTable => resource_tables }
    impl Index<TypeFutureIndex> for ComponentTypes { TypeFuture => futures }
    impl Index<TypeStreamIndex> for ComponentTypes { TypeStream => streams }
    impl Index<TypeFutureTableIndex> for ComponentTypes { TypeFutureTable => future_tables }
    impl Index<TypeStreamTableIndex> for ComponentTypes { TypeStreamTable => stream_tables }
    impl Index<TypeComponentLocalErrorContextTableIndex> for ComponentTypes { TypeErrorContextTable => error_context_tables }
}

// Additionally forward anything that can index `ModuleTypes` to `ModuleTypes`
// (aka `SignatureIndex`)
impl<T> Index<T> for ComponentTypes
where
    ModuleTypes: Index<T>,
{
    type Output = <ModuleTypes as Index<T>>::Output;
    fn index(&self, idx: T) -> &Self::Output {
        self.module_types.as_ref().unwrap().index(idx)
    }
}

/// Types of imports and exports in the component model.
///
/// These types are what's available for import and export in components. Note
/// that all indirect indices contained here are intended to be looked up
/// through a sibling `ComponentTypes` structure.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TypeDef {
    /// A component and its type.
    Component(TypeComponentIndex),
    /// An instance of a component.
    ComponentInstance(TypeComponentInstanceIndex),
    /// A component function, not to be confused with a core wasm function.
    ComponentFunc(TypeFuncIndex),
    /// An type in an interface.
    Interface(InterfaceType),
    /// A core wasm module and its type.
    Module(TypeModuleIndex),
    /// A core wasm function using only core wasm types.
    CoreFunc(ModuleInternedTypeIndex),
    /// A resource type which operates on the specified resource table.
    ///
    /// Note that different resource tables may point to the same underlying
    /// actual resource type, but that's a private detail.
    Resource(TypeResourceTableIndex),
}

impl TypeDef {
    /// A human readable description of what kind of type definition this is.
    pub fn desc(&self) -> &str {
        match self {
            TypeDef::Component(_) => "component",
            TypeDef::ComponentInstance(_) => "instance",
            TypeDef::ComponentFunc(_) => "function",
            TypeDef::Interface(_) => "type",
            TypeDef::Module(_) => "core module",
            TypeDef::CoreFunc(_) => "core function",
            TypeDef::Resource(_) => "resource",
        }
    }
}

// NB: Note that maps below are stored as an `IndexMap` now but the order
// typically does not matter. As a minor implementation detail we want the
// serialization of this type to always be deterministic and using `IndexMap`
// gets us that over using a `HashMap` for example.

/// The type of a module in the component model.
///
/// Note that this is not to be confused with `TypeComponent` below. This is
/// intended only for core wasm modules, not for components.
#[derive(Serialize, Deserialize, Default)]
pub struct TypeModule {
    /// The values that this module imports.
    ///
    /// Note that the value of this map is a core wasm `EntityType`, not a
    /// component model `TypeRef`. Additionally note that this reflects the
    /// two-level namespace of core WebAssembly, but unlike core wasm all import
    /// names are required to be unique to describe a module in the component
    /// model.
    pub imports: IndexMap<(String, String), EntityType>,

    /// The values that this module exports.
    ///
    /// Note that the value of this map is the core wasm `EntityType` to
    /// represent that core wasm items are being exported.
    pub exports: IndexMap<String, EntityType>,
}

impl TypeTrace for TypeModule {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(crate::EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for ty in self.imports.values() {
            ty.trace(func)?;
        }
        for ty in self.exports.values() {
            ty.trace(func)?;
        }
        Ok(())
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut crate::EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for ty in self.imports.values_mut() {
            ty.trace_mut(func)?;
        }
        for ty in self.exports.values_mut() {
            ty.trace_mut(func)?;
        }
        Ok(())
    }
}

/// The type of a component in the component model.
#[derive(Serialize, Deserialize, Default)]
pub struct TypeComponent {
    /// The named values that this component imports.
    pub imports: IndexMap<String, TypeDef>,
    /// The named values that this component exports.
    pub exports: IndexMap<String, TypeDef>,
}

/// The type of a component instance in the component model, or an instantiated
/// component.
///
/// Component instances only have exports of types in the component model.
#[derive(Serialize, Deserialize, Default)]
pub struct TypeComponentInstance {
    /// The list of exports that this component has along with their types.
    pub exports: IndexMap<String, TypeDef>,
}

/// A component function type in the component model.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeFunc {
    /// Names of parameters.
    pub param_names: Vec<String>,
    /// Parameters to the function represented as a tuple.
    pub params: TypeTupleIndex,
    /// Results of the function represented as a tuple.
    pub results: TypeTupleIndex,
}

/// All possible interface types that values can have.
///
/// This list represents an exhaustive listing of interface types and the
/// shapes that they can take. Note that this enum is considered an "index" of
/// forms where for non-primitive types a `ComponentTypes` structure is used to
/// lookup further information based on the index found here.
#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq, Debug)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum InterfaceType {
    Bool,
    S8,
    U8,
    S16,
    U16,
    S32,
    U32,
    S64,
    U64,
    Float32,
    Float64,
    Char,
    String,
    Record(TypeRecordIndex),
    Variant(TypeVariantIndex),
    List(TypeListIndex),
    Tuple(TypeTupleIndex),
    Flags(TypeFlagsIndex),
    Enum(TypeEnumIndex),
    Option(TypeOptionIndex),
    Result(TypeResultIndex),
    Own(TypeResourceTableIndex),
    Borrow(TypeResourceTableIndex),
    Future(TypeFutureTableIndex),
    Stream(TypeStreamTableIndex),
    ErrorContext(TypeComponentLocalErrorContextTableIndex),
}

/// Bye information about a type in the canonical ABI, with metadata for both
/// memory32 and memory64-based types.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct CanonicalAbiInfo {
    /// The byte-size of this type in a 32-bit memory.
    pub size32: u32,
    /// The byte-alignment of this type in a 32-bit memory.
    pub align32: u32,
    /// The byte-size of this type in a 64-bit memory.
    pub size64: u32,
    /// The byte-alignment of this type in a 64-bit memory.
    pub align64: u32,
    /// The number of types it takes to represents this type in the "flat"
    /// representation of the canonical abi where everything is passed as
    /// immediate arguments or results.
    ///
    /// If this is `None` then this type is not representable in the flat ABI
    /// because it is too large.
    pub flat_count: Option<u8>,
}

impl Default for CanonicalAbiInfo {
    fn default() -> CanonicalAbiInfo {
        CanonicalAbiInfo {
            size32: 0,
            align32: 1,
            size64: 0,
            align64: 1,
            flat_count: Some(0),
        }
    }
}

const fn align_to(a: u32, b: u32) -> u32 {
    assert!(b.is_power_of_two());
    (a + (b - 1)) & !(b - 1)
}

const fn max(a: u32, b: u32) -> u32 {
    if a > b { a } else { b }
}

impl CanonicalAbiInfo {
    /// ABI information for zero-sized types.
    pub const ZERO: CanonicalAbiInfo = CanonicalAbiInfo {
        size32: 0,
        align32: 1,
        size64: 0,
        align64: 1,
        flat_count: Some(0),
    };

    /// ABI information for one-byte scalars.
    pub const SCALAR1: CanonicalAbiInfo = CanonicalAbiInfo::scalar(1);
    /// ABI information for two-byte scalars.
    pub const SCALAR2: CanonicalAbiInfo = CanonicalAbiInfo::scalar(2);
    /// ABI information for four-byte scalars.
    pub const SCALAR4: CanonicalAbiInfo = CanonicalAbiInfo::scalar(4);
    /// ABI information for eight-byte scalars.
    pub const SCALAR8: CanonicalAbiInfo = CanonicalAbiInfo::scalar(8);

    const fn scalar(size: u32) -> CanonicalAbiInfo {
        CanonicalAbiInfo {
            size32: size,
            align32: size,
            size64: size,
            align64: size,
            flat_count: Some(1),
        }
    }

    /// ABI information for lists/strings which are "pointer pairs"
    pub const POINTER_PAIR: CanonicalAbiInfo = CanonicalAbiInfo {
        size32: 8,
        align32: 4,
        size64: 16,
        align64: 8,
        flat_count: Some(2),
    };

    /// Returns the abi for a record represented by the specified fields.
    pub fn record<'a>(fields: impl Iterator<Item = &'a CanonicalAbiInfo>) -> CanonicalAbiInfo {
        // NB: this is basically a duplicate copy of
        // `CanonicalAbiInfo::record_static` and the two should be kept in sync.

        let mut ret = CanonicalAbiInfo::default();
        for field in fields {
            ret.size32 = align_to(ret.size32, field.align32) + field.size32;
            ret.align32 = ret.align32.max(field.align32);
            ret.size64 = align_to(ret.size64, field.align64) + field.size64;
            ret.align64 = ret.align64.max(field.align64);
            ret.flat_count = add_flat(ret.flat_count, field.flat_count);
        }
        ret.size32 = align_to(ret.size32, ret.align32);
        ret.size64 = align_to(ret.size64, ret.align64);
        return ret;
    }

    /// Same as `CanonicalAbiInfo::record` but in a `const`-friendly context.
    pub const fn record_static(fields: &[CanonicalAbiInfo]) -> CanonicalAbiInfo {
        // NB: this is basically a duplicate copy of `CanonicalAbiInfo::record`
        // and the two should be kept in sync.

        let mut ret = CanonicalAbiInfo::ZERO;
        let mut i = 0;
        while i < fields.len() {
            let field = &fields[i];
            ret.size32 = align_to(ret.size32, field.align32) + field.size32;
            ret.align32 = max(ret.align32, field.align32);
            ret.size64 = align_to(ret.size64, field.align64) + field.size64;
            ret.align64 = max(ret.align64, field.align64);
            ret.flat_count = add_flat(ret.flat_count, field.flat_count);
            i += 1;
        }
        ret.size32 = align_to(ret.size32, ret.align32);
        ret.size64 = align_to(ret.size64, ret.align64);
        return ret;
    }

    /// Returns the delta from the current value of `offset` to align properly
    /// and read the next record field of type `abi` for 32-bit memories.
    pub fn next_field32(&self, offset: &mut u32) -> u32 {
        *offset = align_to(*offset, self.align32) + self.size32;
        *offset - self.size32
    }

    /// Same as `next_field32`, but bumps a usize pointer
    pub fn next_field32_size(&self, offset: &mut usize) -> usize {
        let cur = u32::try_from(*offset).unwrap();
        let cur = align_to(cur, self.align32) + self.size32;
        *offset = usize::try_from(cur).unwrap();
        usize::try_from(cur - self.size32).unwrap()
    }

    /// Returns the delta from the current value of `offset` to align properly
    /// and read the next record field of type `abi` for 64-bit memories.
    pub fn next_field64(&self, offset: &mut u32) -> u32 {
        *offset = align_to(*offset, self.align64) + self.size64;
        *offset - self.size64
    }

    /// Same as `next_field64`, but bumps a usize pointer
    pub fn next_field64_size(&self, offset: &mut usize) -> usize {
        let cur = u32::try_from(*offset).unwrap();
        let cur = align_to(cur, self.align64) + self.size64;
        *offset = usize::try_from(cur).unwrap();
        usize::try_from(cur - self.size64).unwrap()
    }

    /// Returns ABI information for a structure which contains `count` flags.
    pub const fn flags(count: usize) -> CanonicalAbiInfo {
        let (size, align, flat_count) = match FlagsSize::from_count(count) {
            FlagsSize::Size0 => (0, 1, 0),
            FlagsSize::Size1 => (1, 1, 1),
            FlagsSize::Size2 => (2, 2, 1),
            FlagsSize::Size4Plus(n) => ((n as u32) * 4, 4, n),
        };
        CanonicalAbiInfo {
            size32: size,
            align32: align,
            size64: size,
            align64: align,
            flat_count: Some(flat_count),
        }
    }

    fn variant<'a, I>(cases: I) -> CanonicalAbiInfo
    where
        I: IntoIterator<Item = Option<&'a CanonicalAbiInfo>>,
        I::IntoIter: ExactSizeIterator,
    {
        // NB: this is basically a duplicate definition of
        // `CanonicalAbiInfo::variant_static`, these should be kept in sync.

        let cases = cases.into_iter();
        let discrim_size = u32::from(DiscriminantSize::from_count(cases.len()).unwrap());
        let mut max_size32 = 0;
        let mut max_align32 = discrim_size;
        let mut max_size64 = 0;
        let mut max_align64 = discrim_size;
        let mut max_case_count = Some(0);
        for case in cases {
            if let Some(case) = case {
                max_size32 = max_size32.max(case.size32);
                max_align32 = max_align32.max(case.align32);
                max_size64 = max_size64.max(case.size64);
                max_align64 = max_align64.max(case.align64);
                max_case_count = max_flat(max_case_count, case.flat_count);
            }
        }
        CanonicalAbiInfo {
            size32: align_to(
                align_to(discrim_size, max_align32) + max_size32,
                max_align32,
            ),
            align32: max_align32,
            size64: align_to(
                align_to(discrim_size, max_align64) + max_size64,
                max_align64,
            ),
            align64: max_align64,
            flat_count: add_flat(max_case_count, Some(1)),
        }
    }

    /// Same as `CanonicalAbiInfo::variant` but `const`-safe
    pub const fn variant_static(cases: &[Option<CanonicalAbiInfo>]) -> CanonicalAbiInfo {
        // NB: this is basically a duplicate definition of
        // `CanonicalAbiInfo::variant`, these should be kept in sync.

        let discrim_size = match DiscriminantSize::from_count(cases.len()) {
            Some(size) => size.byte_size(),
            None => unreachable!(),
        };
        let mut max_size32 = 0;
        let mut max_align32 = discrim_size;
        let mut max_size64 = 0;
        let mut max_align64 = discrim_size;
        let mut max_case_count = Some(0);
        let mut i = 0;
        while i < cases.len() {
            let case = &cases[i];
            if let Some(case) = case {
                max_size32 = max(max_size32, case.size32);
                max_align32 = max(max_align32, case.align32);
                max_size64 = max(max_size64, case.size64);
                max_align64 = max(max_align64, case.align64);
                max_case_count = max_flat(max_case_count, case.flat_count);
            }
            i += 1;
        }
        CanonicalAbiInfo {
            size32: align_to(
                align_to(discrim_size, max_align32) + max_size32,
                max_align32,
            ),
            align32: max_align32,
            size64: align_to(
                align_to(discrim_size, max_align64) + max_size64,
                max_align64,
            ),
            align64: max_align64,
            flat_count: add_flat(max_case_count, Some(1)),
        }
    }

    /// Calculates ABI information for an enum with `cases` cases.
    pub const fn enum_(cases: usize) -> CanonicalAbiInfo {
        // NB: this is basically a duplicate definition of
        // `CanonicalAbiInfo::variant`, these should be kept in sync.

        let discrim_size = match DiscriminantSize::from_count(cases) {
            Some(size) => size.byte_size(),
            None => unreachable!(),
        };
        CanonicalAbiInfo {
            size32: discrim_size,
            align32: discrim_size,
            size64: discrim_size,
            align64: discrim_size,
            flat_count: Some(1),
        }
    }

    /// Returns the flat count of this ABI information so long as the count
    /// doesn't exceed the `max` specified.
    pub fn flat_count(&self, max: usize) -> Option<usize> {
        let flat = usize::from(self.flat_count?);
        if flat > max { None } else { Some(flat) }
    }
}

/// ABI information about the representation of a variant.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct VariantInfo {
    /// The size of the discriminant used.
    #[serde(with = "serde_discrim_size")]
    pub size: DiscriminantSize,
    /// The offset of the payload from the start of the variant in 32-bit
    /// memories.
    pub payload_offset32: u32,
    /// The offset of the payload from the start of the variant in 64-bit
    /// memories.
    pub payload_offset64: u32,
}

impl VariantInfo {
    /// Returns the abi information for a variant represented by the specified
    /// cases.
    pub fn new<'a, I>(cases: I) -> (VariantInfo, CanonicalAbiInfo)
    where
        I: IntoIterator<Item = Option<&'a CanonicalAbiInfo>>,
        I::IntoIter: ExactSizeIterator,
    {
        let cases = cases.into_iter();
        let size = DiscriminantSize::from_count(cases.len()).unwrap();
        let abi = CanonicalAbiInfo::variant(cases);
        (
            VariantInfo {
                size,
                payload_offset32: align_to(u32::from(size), abi.align32),
                payload_offset64: align_to(u32::from(size), abi.align64),
            },
            abi,
        )
    }
    /// TODO
    pub const fn new_static(cases: &[Option<CanonicalAbiInfo>]) -> VariantInfo {
        let size = match DiscriminantSize::from_count(cases.len()) {
            Some(size) => size,
            None => unreachable!(),
        };
        let abi = CanonicalAbiInfo::variant_static(cases);
        VariantInfo {
            size,
            payload_offset32: align_to(size.byte_size(), abi.align32),
            payload_offset64: align_to(size.byte_size(), abi.align64),
        }
    }
}

mod serde_discrim_size {
    use super::DiscriminantSize;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    pub fn serialize<S>(disc: &DiscriminantSize, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        u32::from(*disc).serialize(ser)
    }

    pub fn deserialize<'de, D>(deser: D) -> Result<DiscriminantSize, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u32::deserialize(deser)? {
            1 => Ok(DiscriminantSize::Size1),
            2 => Ok(DiscriminantSize::Size2),
            4 => Ok(DiscriminantSize::Size4),
            _ => Err(D::Error::custom("invalid discriminant size")),
        }
    }
}

/// Shape of a "record" type in interface types.
///
/// This is equivalent to a `struct` in Rust.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeRecord {
    /// The fields that are contained within this struct type.
    pub fields: Box<[RecordField]>,
    /// Byte information about this type in the canonical ABI.
    pub abi: CanonicalAbiInfo,
}

/// One field within a record.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct RecordField {
    /// The name of the field, unique amongst all fields in a record.
    pub name: String,
    /// The type that this field contains.
    pub ty: InterfaceType,
}

/// Shape of a "variant" type in interface types.
///
/// Variants are close to Rust `enum` declarations where a value is one of many
/// cases and each case has a unique name and an optional payload associated
/// with it.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct TypeVariant {
    /// The list of cases that this variant can take.
    pub cases: IndexMap<String, Option<InterfaceType>>,
    /// Byte information about this type in the canonical ABI.
    pub abi: CanonicalAbiInfo,
    /// Byte information about this variant type.
    pub info: VariantInfo,
}

impl Hash for TypeVariant {
    fn hash<H: Hasher>(&self, h: &mut H) {
        let TypeVariant { cases, abi, info } = self;
        cases.len().hash(h);
        for pair in cases {
            pair.hash(h);
        }
        abi.hash(h);
        info.hash(h);
    }
}

/// Shape of a "tuple" type in interface types.
///
/// This is largely the same as a tuple in Rust, basically a record with
/// unnamed fields.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeTuple {
    /// The types that are contained within this tuple.
    pub types: Box<[InterfaceType]>,
    /// Byte information about this type in the canonical ABI.
    pub abi: CanonicalAbiInfo,
}

/// Shape of a "flags" type in interface types.
///
/// This can be thought of as a record-of-bools, although the representation is
/// more efficient as bitflags.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct TypeFlags {
    /// The names of all flags, all of which are unique.
    pub names: IndexSet<String>,
    /// Byte information about this type in the canonical ABI.
    pub abi: CanonicalAbiInfo,
}

impl Hash for TypeFlags {
    fn hash<H: Hasher>(&self, h: &mut H) {
        let TypeFlags { names, abi } = self;
        names.len().hash(h);
        for name in names {
            name.hash(h);
        }
        abi.hash(h);
    }
}

/// Shape of an "enum" type in interface types, not to be confused with a Rust
/// `enum` type.
///
/// In interface types enums are simply a bag of names, and can be seen as a
/// variant where all payloads are `Unit`.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct TypeEnum {
    /// The names of this enum, all of which are unique.
    pub names: IndexSet<String>,
    /// Byte information about this type in the canonical ABI.
    pub abi: CanonicalAbiInfo,
    /// Byte information about this variant type.
    pub info: VariantInfo,
}

impl Hash for TypeEnum {
    fn hash<H: Hasher>(&self, h: &mut H) {
        let TypeEnum { names, abi, info } = self;
        names.len().hash(h);
        for name in names {
            name.hash(h);
        }
        abi.hash(h);
        info.hash(h);
    }
}

/// Shape of an "option" interface type.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeOption {
    /// The `T` in `Result<T, E>`
    pub ty: InterfaceType,
    /// Byte information about this type in the canonical ABI.
    pub abi: CanonicalAbiInfo,
    /// Byte information about this variant type.
    pub info: VariantInfo,
}

/// Shape of a "result" interface type.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeResult {
    /// The `T` in `Result<T, E>`
    pub ok: Option<InterfaceType>,
    /// The `E` in `Result<T, E>`
    pub err: Option<InterfaceType>,
    /// Byte information about this type in the canonical ABI.
    pub abi: CanonicalAbiInfo,
    /// Byte information about this variant type.
    pub info: VariantInfo,
}

/// Shape of a "future" interface type.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeFuture {
    /// The `T` in `future<T>`
    pub payload: Option<InterfaceType>,
}

/// Metadata about a future table added to a component.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeFutureTable {
    /// The specific future type this table is used for.
    pub ty: TypeFutureIndex,
    /// The specific component instance this table is used for.
    pub instance: RuntimeComponentInstanceIndex,
}

/// Shape of a "stream" interface type.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeStream {
    /// The `T` in `stream<T>`
    pub payload: Option<InterfaceType>,
}

/// Metadata about a stream table added to a component.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeStreamTable {
    /// The specific stream type this table is used for.
    pub ty: TypeStreamIndex,
    /// The specific component instance this table is used for.
    pub instance: RuntimeComponentInstanceIndex,
}

/// Metadata about a error context table added to a component.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeErrorContextTable {
    /// The specific component instance this table is used for.
    pub instance: RuntimeComponentInstanceIndex,
}

/// Metadata about a resource table added to a component.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeResourceTable {
    /// The original resource that this table contains.
    ///
    /// This is used when destroying resources within this table since this
    /// original definition will know how to execute destructors.
    pub ty: ResourceIndex,

    /// The component instance that contains this resource table.
    pub instance: RuntimeComponentInstanceIndex,
}

/// Shape of a "list" interface type.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeList {
    /// The element type of the list.
    pub element: InterfaceType,
}

/// Maximum number of flat types, for either params or results.
pub const MAX_FLAT_TYPES: usize = if MAX_FLAT_PARAMS > MAX_FLAT_RESULTS {
    MAX_FLAT_PARAMS
} else {
    MAX_FLAT_RESULTS
};

const fn add_flat(a: Option<u8>, b: Option<u8>) -> Option<u8> {
    const MAX: u8 = MAX_FLAT_TYPES as u8;
    let sum = match (a, b) {
        (Some(a), Some(b)) => match a.checked_add(b) {
            Some(c) => c,
            None => return None,
        },
        _ => return None,
    };
    if sum > MAX { None } else { Some(sum) }
}

const fn max_flat(a: Option<u8>, b: Option<u8>) -> Option<u8> {
    match (a, b) {
        (Some(a), Some(b)) => {
            if a > b {
                Some(a)
            } else {
                Some(b)
            }
        }
        _ => None,
    }
}

/// Flat representation of a type in just core wasm types.
pub struct FlatTypes<'a> {
    /// The flat representation of this type in 32-bit memories.
    pub memory32: &'a [FlatType],
    /// The flat representation of this type in 64-bit memories.
    pub memory64: &'a [FlatType],
}

impl FlatTypes<'_> {
    /// Returns the number of flat types used to represent this type.
    ///
    /// Note that this length is the same regardless to the size of memory.
    pub fn len(&self) -> usize {
        assert_eq!(self.memory32.len(), self.memory64.len());
        self.memory32.len()
    }
}

// Note that this is intentionally duplicated here to keep the size to 1 byte
// regardless to changes in the core wasm type system since this will only
// ever use integers/floats for the foreseeable future.
#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Eq, Copy, Clone)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum FlatType {
    I32,
    I64,
    F32,
    F64,
}
