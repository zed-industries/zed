//! Types relating to type information provided by validation.

use super::{
    component::{ComponentState, ExternKind},
    core::Module,
};
use crate::{validator::names::KebabString, HeapType};
use crate::{
    BinaryReaderError, CompositeType, Export, ExternalKind, FuncType, GlobalType, Import, Matches,
    MemoryType, PackedIndex, PrimitiveValType, RecGroup, RefType, Result, SubType, TableType,
    TypeRef, UnpackedIndex, ValType, WithRecGroup,
};
use indexmap::{IndexMap, IndexSet};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::ops::{Index, Range};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    mem,
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// The maximum number of parameters in the canonical ABI that can be passed by value.
///
/// Functions that exceed this limit will instead pass parameters indirectly from
/// linear memory via a single pointer parameter.
const MAX_FLAT_FUNC_PARAMS: usize = 16;
/// The maximum number of results in the canonical ABI that can be returned by a function.
///
/// Functions that exceed this limit have their results written to linear memory via an
/// additional pointer parameter (imports) or return a single pointer value (exports).
const MAX_FLAT_FUNC_RESULTS: usize = 1;

/// The maximum lowered types, including a possible type for a return pointer parameter.
const MAX_LOWERED_TYPES: usize = MAX_FLAT_FUNC_PARAMS + 1;

/// A simple alloc-free list of types used for calculating lowered function signatures.
pub(crate) struct LoweredTypes {
    types: [ValType; MAX_LOWERED_TYPES],
    len: usize,
    max: usize,
}

impl LoweredTypes {
    fn new(max: usize) -> Self {
        assert!(max <= MAX_LOWERED_TYPES);
        Self {
            types: [ValType::I32; MAX_LOWERED_TYPES],
            len: 0,
            max,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn maxed(&self) -> bool {
        self.len == self.max
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut ValType> {
        if index < self.len {
            Some(&mut self.types[index])
        } else {
            None
        }
    }

    fn push(&mut self, ty: ValType) -> bool {
        if self.maxed() {
            return false;
        }

        self.types[self.len] = ty;
        self.len += 1;
        true
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    pub fn as_slice(&self) -> &[ValType] {
        &self.types[..self.len]
    }

    pub fn iter(&self) -> impl Iterator<Item = ValType> + '_ {
        self.as_slice().iter().copied()
    }
}

/// Represents information about a component function type lowering.
pub(crate) struct LoweringInfo {
    pub(crate) params: LoweredTypes,
    pub(crate) results: LoweredTypes,
    pub(crate) requires_memory: bool,
    pub(crate) requires_realloc: bool,
}

impl LoweringInfo {
    pub(crate) fn into_func_type(self) -> FuncType {
        FuncType::new(
            self.params.as_slice().iter().copied(),
            self.results.as_slice().iter().copied(),
        )
    }
}

impl Default for LoweringInfo {
    fn default() -> Self {
        Self {
            params: LoweredTypes::new(MAX_FLAT_FUNC_PARAMS),
            results: LoweredTypes::new(MAX_FLAT_FUNC_RESULTS),
            requires_memory: false,
            requires_realloc: false,
        }
    }
}

fn push_primitive_wasm_types(ty: &PrimitiveValType, lowered_types: &mut LoweredTypes) -> bool {
    match ty {
        PrimitiveValType::Bool
        | PrimitiveValType::S8
        | PrimitiveValType::U8
        | PrimitiveValType::S16
        | PrimitiveValType::U16
        | PrimitiveValType::S32
        | PrimitiveValType::U32
        | PrimitiveValType::Char => lowered_types.push(ValType::I32),
        PrimitiveValType::S64 | PrimitiveValType::U64 => lowered_types.push(ValType::I64),
        PrimitiveValType::Float32 => lowered_types.push(ValType::F32),
        PrimitiveValType::Float64 => lowered_types.push(ValType::F64),
        PrimitiveValType::String => {
            lowered_types.push(ValType::I32) && lowered_types.push(ValType::I32)
        }
    }
}

/// A trait shared by all type identifiers.
///
/// Any id that can be used to get a type from a `Types`.
//
// Or, internally, from a `TypeList`.
pub trait TypeIdentifier: std::fmt::Debug + Copy + Eq + Sized + 'static {
    /// The data pointed to by this type of id.
    type Data: TypeData<Id = Self>;

    /// Create a type id from an index.
    #[doc(hidden)]
    fn from_index(index: u32) -> Self;

    /// Get a shared reference to the list where this id's type data is stored
    /// within.
    #[doc(hidden)]
    fn list(types: &TypeList) -> &SnapshotList<Self::Data>;

    /// Get an exclusive reference to the list where this id's type data is
    /// stored within.
    #[doc(hidden)]
    fn list_mut(types: &mut TypeList) -> &mut SnapshotList<Self::Data>;

    /// The raw index of this id.
    #[doc(hidden)]
    fn index(&self) -> usize;
}

/// A trait shared by all types within a `Types`.
///
/// This is the data that can be retreived by indexing with the associated
/// [`TypeIdentifier`].
pub trait TypeData: std::fmt::Debug {
    /// The identifier for this type data.
    type Id: TypeIdentifier<Data = Self>;

    /// Get the info for this type.
    #[doc(hidden)]
    fn type_info(&self, types: &TypeList) -> TypeInfo;
}

/// A type that can be aliased in the component model.
pub trait Aliasable {
    #[doc(hidden)]
    fn alias_id(&self) -> u32;

    #[doc(hidden)]
    fn set_alias_id(&mut self, alias_id: u32);
}

/// A fresh alias id that means the entity is not an alias of anything.
///
/// Note that the `TypeList::alias_counter` starts at zero, so we can't use that
/// as this sentinel. The implementation limits are such that we can't ever
/// generate `u32::MAX` aliases, so we don't need to worryabout running into
/// this value in practice either.
const NO_ALIAS: u32 = u32::MAX;

macro_rules! define_type_id {
    ($name:ident, $data:ty, $list:ident, $type_str:expr) => {
        #[doc = "Represents a unique identifier for a "]
        #[doc = $type_str]
        #[doc = " type known to a [`crate::Validator`]."]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        #[repr(C)] // Use fixed field layout to ensure minimal size.
        pub struct $name {
            /// The index into the associated list of types.
            index: u32,
        }

        impl TypeIdentifier for $name {
            type Data = $data;

            fn from_index(index: u32) -> Self {
                $name { index }
            }

            fn list(types: &TypeList) -> &SnapshotList<Self::Data> {
                &types.$list
            }

            fn list_mut(types: &mut TypeList) -> &mut SnapshotList<Self::Data> {
                &mut types.$list
            }

            fn index(&self) -> usize {
                usize::try_from(self.index).unwrap()
            }
        }

        impl Aliasable for $name {
            fn alias_id(&self) -> u32 {
                NO_ALIAS
            }

            fn set_alias_id(&mut self, _: u32) {}
        }

        // The size of type IDs was seen to have a large-ish impact in #844, so
        // this assert ensures that it stays relatively small.
        const _: () = {
            assert!(std::mem::size_of::<$name>() <= 4);
        };
    };
}

/// A core WebAssembly type, in the core WebAssembly types index space.
pub enum CoreType {
    /// A sub type.
    Sub(SubType),

    /// A module type.
    ///
    /// Does not actually appear in core Wasm at the moment. Only used for the
    /// core types index space within components.
    Module(ModuleType),
}

/// Represents a unique identifier for a core type type known to a
/// [`crate::Validator`]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CoreTypeId {
    index: u32,
}

const _: () = {
    assert!(std::mem::size_of::<CoreTypeId>() <= 4);
};

impl TypeIdentifier for CoreTypeId {
    type Data = SubType;

    fn from_index(index: u32) -> Self {
        CoreTypeId { index }
    }

    fn list(types: &TypeList) -> &SnapshotList<Self::Data> {
        &types.core_types
    }

    fn list_mut(types: &mut TypeList) -> &mut SnapshotList<Self::Data> {
        &mut types.core_types
    }

    fn index(&self) -> usize {
        usize::try_from(self.index).unwrap()
    }
}

impl TypeData for SubType {
    type Id = CoreTypeId;

    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        // TODO(#1036): calculate actual size for func, array, struct.
        let size = 1 + match &self.composite_type {
            CompositeType::Func(ty) => 1 + (ty.params().len() + ty.results().len()) as u32,
            CompositeType::Array(_) => 2,
            CompositeType::Struct(ty) => 1 + 2 * ty.fields.len() as u32,
        };
        TypeInfo::core(size)
    }
}

impl CoreType {
    /// Get the underlying `SubType` or panic.
    pub fn unwrap_sub(&self) -> &SubType {
        match self {
            CoreType::Sub(s) => s,
            CoreType::Module(_) => panic!("`unwrap_sub` on module type"),
        }
    }

    /// Get the underlying `FuncType` within this `SubType` or panic.
    pub fn unwrap_func(&self) -> &FuncType {
        match &self.unwrap_sub().composite_type {
            CompositeType::Func(f) => f,
            CompositeType::Array(_) | CompositeType::Struct(_) => {
                panic!("`unwrap_func` on non-func composite type")
            }
        }
    }

    /// Get the underlying `ModuleType` or panic.
    pub fn unwrap_module(&self) -> &ModuleType {
        match self {
            CoreType::Module(m) => m,
            CoreType::Sub(_) => panic!("`unwrap_module` on a subtype"),
        }
    }
}

macro_rules! define_wrapper_id {
    (
        $(#[$outer_attrs:meta])*
        pub enum $name:ident {
            $(
                #[unwrap = $unwrap:ident]
                $(#[$inner_attrs:meta])*
                $variant:ident ( $inner:ty ) ,
            )*
        }
    ) => {
        $(#[$outer_attrs])*
        pub enum $name {
            $(
                $(#[$inner_attrs])*
                $variant ( $inner ) ,
            )*
        }

        $(
            impl From<$inner> for $name {
                #[inline]
                fn from(x: $inner) -> Self {
                    Self::$variant(x)
                }
            }

            impl TryFrom<$name> for $inner {
                type Error = ();

                #[inline]
                fn try_from(x: $name) -> Result<Self, Self::Error> {
                    match x {
                        $name::$variant(x) => Ok(x),
                        _ => Err(())
                    }
                }
            }
        )*

        impl $name {
            $(
                #[doc = "Unwrap a `"]
                #[doc = stringify!($inner)]
                #[doc = "` or panic."]
                #[inline]
                pub fn $unwrap(self) -> $inner {
                    <$inner>::try_from(self).unwrap()
                }
            )*
        }
    };
}

macro_rules! define_transitive_conversions {
    (
        $(
            $outer:ty,
            $middle:ty,
            $inner:ty,
            $unwrap:ident;
        )*
    ) => {
        $(
            impl From<$inner> for $outer {
                #[inline]
                fn from(x: $inner) -> Self {
                    <$middle>::from(x).into()
                }
            }

            impl TryFrom<$outer> for $inner {
                type Error = ();

                #[inline]
                fn try_from(x: $outer) -> Result<Self, Self::Error> {
                    let middle = <$middle>::try_from(x)?;
                    <$inner>::try_from(middle)
                }
            }

            impl $outer {
                #[doc = "Unwrap a `"]
                #[doc = stringify!($inner)]
                #[doc = "` or panic."]
                #[inline]
                pub fn $unwrap(self) -> $inner {
                    <$inner>::try_from(self).unwrap()
                }
            }
        )*
    };
}

define_wrapper_id! {
    /// An identifier pointing to any kind of type, component or core.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum AnyTypeId {
        #[unwrap = unwrap_component_core_type]
        /// A core type.
        Core(ComponentCoreTypeId),

        #[unwrap = unwrap_component_any_type]
        /// A component type.
        Component(ComponentAnyTypeId),
    }
}

define_transitive_conversions! {
    AnyTypeId, ComponentCoreTypeId, CoreTypeId, unwrap_core_type;
    AnyTypeId, ComponentCoreTypeId, ComponentCoreModuleTypeId, unwrap_component_core_module_type;
    AnyTypeId, ComponentAnyTypeId, AliasableResourceId, unwrap_aliasable_resource;
    AnyTypeId, ComponentAnyTypeId, ComponentDefinedTypeId, unwrap_component_defined_type;
    AnyTypeId, ComponentAnyTypeId, ComponentFuncTypeId, unwrap_component_func_type;
    AnyTypeId, ComponentAnyTypeId, ComponentInstanceTypeId, unwrap_component_instance_type;
    AnyTypeId, ComponentAnyTypeId, ComponentTypeId, unwrap_component_type;
}

impl AnyTypeId {
    /// Peel off one layer of aliasing from this type and return the aliased
    /// inner type, or `None` if this type is not aliasing anything.
    pub fn peel_alias(&self, types: &Types) -> Option<Self> {
        match *self {
            Self::Core(id) => id.peel_alias(types).map(Self::Core),
            Self::Component(id) => types.peel_alias(id).map(Self::Component),
        }
    }
}

define_wrapper_id! {
    /// An identifier for a core type or a core module's type.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum ComponentCoreTypeId {
        #[unwrap = unwrap_sub]
        /// A core type.
        Sub(CoreTypeId),

        #[unwrap = unwrap_module]
        /// A core module's type.
        Module(ComponentCoreModuleTypeId),
    }
}

impl ComponentCoreTypeId {
    /// Peel off one layer of aliasing from this type and return the aliased
    /// inner type, or `None` if this type is not aliasing anything.
    pub fn peel_alias(&self, types: &Types) -> Option<Self> {
        match *self {
            Self::Sub(_) => None,
            Self::Module(id) => types.peel_alias(id).map(Self::Module),
        }
    }
}

/// An aliasable resource identifier.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AliasableResourceId {
    id: ResourceId,
    alias_id: u32,
}

impl Aliasable for AliasableResourceId {
    fn alias_id(&self) -> u32 {
        self.alias_id
    }

    fn set_alias_id(&mut self, alias_id: u32) {
        self.alias_id = alias_id;
    }
}

impl AliasableResourceId {
    /// Create a new instance with the specified resource ID and `self`'s alias
    /// ID.
    pub fn with_resource_id(&self, id: ResourceId) -> Self {
        Self {
            id,
            alias_id: self.alias_id,
        }
    }

    /// Get the underlying resource.
    pub fn resource(&self) -> ResourceId {
        self.id
    }

    pub(crate) fn resource_mut(&mut self) -> &mut ResourceId {
        &mut self.id
    }
}

define_wrapper_id! {
    /// An identifier for any kind of component type.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum ComponentAnyTypeId {
        #[unwrap = unwrap_resource]
        /// The type is a resource with the specified id.
        Resource(AliasableResourceId),

        #[unwrap = unwrap_defined]
        /// The type is a defined type with the specified id.
        Defined(ComponentDefinedTypeId),

        #[unwrap = unwrap_func]
        /// The type is a function type with the specified id.
        Func(ComponentFuncTypeId),

        #[unwrap = unwrap_instance]
        /// The type is an instance type with the specified id.
        Instance(ComponentInstanceTypeId),

        #[unwrap = unwrap_component]
        /// The type is a component type with the specified id.
        Component(ComponentTypeId),
    }
}

impl Aliasable for ComponentAnyTypeId {
    fn alias_id(&self) -> u32 {
        match self {
            ComponentAnyTypeId::Resource(x) => x.alias_id(),
            ComponentAnyTypeId::Defined(x) => x.alias_id(),
            ComponentAnyTypeId::Func(x) => x.alias_id(),
            ComponentAnyTypeId::Instance(x) => x.alias_id(),
            ComponentAnyTypeId::Component(x) => x.alias_id(),
        }
    }

    fn set_alias_id(&mut self, alias_id: u32) {
        match self {
            ComponentAnyTypeId::Resource(x) => x.set_alias_id(alias_id),
            ComponentAnyTypeId::Defined(x) => x.set_alias_id(alias_id),
            ComponentAnyTypeId::Func(x) => x.set_alias_id(alias_id),
            ComponentAnyTypeId::Instance(x) => x.set_alias_id(alias_id),
            ComponentAnyTypeId::Component(x) => x.set_alias_id(alias_id),
        }
    }
}

impl ComponentAnyTypeId {
    pub(crate) fn info(&self, types: &TypeList) -> TypeInfo {
        match *self {
            Self::Resource(_) => TypeInfo::new(),
            Self::Defined(id) => types[id].type_info(types),
            Self::Func(id) => types[id].type_info(types),
            Self::Instance(id) => types[id].type_info(types),
            Self::Component(id) => types[id].type_info(types),
        }
    }

    pub(crate) fn desc(&self) -> &'static str {
        match self {
            Self::Resource(_) => "resource",
            Self::Defined(_) => "defined type",
            Self::Func(_) => "func",
            Self::Instance(_) => "instance",
            Self::Component(_) => "component",
        }
    }
}

define_type_id!(
    RecGroupId,
    Range<CoreTypeId>,
    rec_group_elements,
    "recursion group"
);

impl TypeData for Range<CoreTypeId> {
    type Id = RecGroupId;

    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        let size = self.end.index() - self.start.index();
        TypeInfo::core(u32::try_from(size).unwrap())
    }
}

define_type_id!(ComponentTypeId, ComponentType, components, "component");

define_type_id!(
    ComponentValueTypeId,
    ComponentValType,
    component_values,
    "component value"
);

define_type_id!(
    ComponentInstanceTypeId,
    ComponentInstanceType,
    component_instances,
    "component instance"
);

define_type_id!(
    ComponentFuncTypeId,
    ComponentFuncType,
    component_funcs,
    "component function"
);

define_type_id!(
    ComponentCoreInstanceTypeId,
    InstanceType,
    core_instances,
    "component's core instance"
);

define_type_id!(
    ComponentCoreModuleTypeId,
    ModuleType,
    core_modules,
    "component's core module"
);

/// Represents a unique identifier for a component type type known to a
/// [`crate::Validator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ComponentDefinedTypeId {
    index: u32,
    alias_id: u32,
}

const _: () = {
    assert!(std::mem::size_of::<ComponentDefinedTypeId>() <= 8);
};

impl TypeIdentifier for ComponentDefinedTypeId {
    type Data = ComponentDefinedType;

    fn from_index(index: u32) -> Self {
        ComponentDefinedTypeId {
            index,
            alias_id: NO_ALIAS,
        }
    }

    fn list(types: &TypeList) -> &SnapshotList<Self::Data> {
        &types.component_defined_types
    }

    fn list_mut(types: &mut TypeList) -> &mut SnapshotList<Self::Data> {
        &mut types.component_defined_types
    }

    fn index(&self) -> usize {
        usize::try_from(self.index).unwrap()
    }
}

impl Aliasable for ComponentDefinedTypeId {
    fn alias_id(&self) -> u32 {
        self.alias_id
    }

    fn set_alias_id(&mut self, alias_id: u32) {
        self.alias_id = alias_id;
    }
}

/// Metadata about a type and its transitive structure.
///
/// Currently contains two properties:
///
/// * The "size" of a type - a proxy to the recursive size of a type if
///   everything in the type were unique (e.g. no shared references). Not an
///   approximation of runtime size, but instead of type-complexity size if
///   someone were to visit each element of the type individually. For example
///   `u32` has size 1 and `(list u32)` has size 2 (roughly). Used to prevent
///   massive trees of types.
///
/// * Whether or not a type contains a "borrow" transitively inside of it. For
///   example `(borrow $t)` and `(list (borrow $t))` both contain borrows, but
///   `(list u32)` does not. Used to validate that component function results do
///   not contain borrows.
///
/// Currently this is represented as a compact 32-bit integer to ensure that
/// `TypeId`, which this is stored in, remains relatively small. The maximum
/// type size allowed in wasmparser is 1M at this time which is 20 bits of
/// information, and then one more bit is used for whether or not a borrow is
/// used. Currently this uses the low 24 bits for the type size and the MSB for
/// the borrow bit.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// Only public because it shows up in a public trait's `doc(hidden)` method.
#[doc(hidden)]
pub struct TypeInfo(u32);

impl TypeInfo {
    /// Creates a new blank set of type information.
    ///
    /// Defaults to size 1 to ensure that this consumes space in the final type
    /// structure.
    pub(crate) fn new() -> TypeInfo {
        TypeInfo::_new(1, false)
    }

    /// Creates a new blank set of information about a leaf "borrow" type which
    /// has size 1.
    pub(crate) fn borrow() -> TypeInfo {
        TypeInfo::_new(1, true)
    }

    /// Creates type information corresponding to a core type of the `size`
    /// specified, meaning no borrows are contained within.
    pub(crate) fn core(size: u32) -> TypeInfo {
        TypeInfo::_new(size, false)
    }

    fn _new(size: u32, contains_borrow: bool) -> TypeInfo {
        assert!(size < (1 << 24));
        TypeInfo(size | ((contains_borrow as u32) << 31))
    }

    /// Combines another set of type information into this one, for example if
    /// this is a record which has `other` as a field.
    ///
    /// Updates the size of `self` and whether or not this type contains a
    /// borrow based on whether `other` contains a borrow.
    ///
    /// Returns an error if the type size would exceed this crate's static limit
    /// of a type size.
    pub(crate) fn combine(&mut self, other: TypeInfo, offset: usize) -> Result<()> {
        *self = TypeInfo::_new(
            super::combine_type_sizes(self.size(), other.size(), offset)?,
            self.contains_borrow() || other.contains_borrow(),
        );
        Ok(())
    }

    pub(crate) fn size(&self) -> u32 {
        self.0 & 0xffffff
    }

    pub(crate) fn contains_borrow(&self) -> bool {
        (self.0 >> 31) != 0
    }
}

/// A component value type.
#[derive(Debug, Clone, Copy)]
pub enum ComponentValType {
    /// The value type is one of the primitive types.
    Primitive(PrimitiveValType),
    /// The type is represented with the given type identifier.
    Type(ComponentDefinedTypeId),
}

impl TypeData for ComponentValType {
    type Id = ComponentValueTypeId;

    fn type_info(&self, types: &TypeList) -> TypeInfo {
        match self {
            ComponentValType::Primitive(_) => TypeInfo::new(),
            ComponentValType::Type(id) => types[*id].type_info(types),
        }
    }
}

impl ComponentValType {
    pub(crate) fn contains_ptr(&self, types: &TypeList) -> bool {
        match self {
            ComponentValType::Primitive(ty) => ty.contains_ptr(),
            ComponentValType::Type(ty) => types[*ty].contains_ptr(types),
        }
    }

    fn push_wasm_types(&self, types: &TypeList, lowered_types: &mut LoweredTypes) -> bool {
        match self {
            Self::Primitive(ty) => push_primitive_wasm_types(ty, lowered_types),
            Self::Type(id) => types[*id].push_wasm_types(types, lowered_types),
        }
    }

    pub(crate) fn info(&self, types: &TypeList) -> TypeInfo {
        match self {
            Self::Primitive(_) => TypeInfo::new(),
            Self::Type(id) => types[*id].type_info(types),
        }
    }
}

/// The entity type for imports and exports of a module.
#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    /// The entity is a function.
    Func(CoreTypeId),
    /// The entity is a table.
    Table(TableType),
    /// The entity is a memory.
    Memory(MemoryType),
    /// The entity is a global.
    Global(GlobalType),
    /// The entity is a tag.
    Tag(CoreTypeId),
}

impl EntityType {
    pub(crate) fn desc(&self) -> &'static str {
        match self {
            Self::Func(_) => "func",
            Self::Table(_) => "table",
            Self::Memory(_) => "memory",
            Self::Global(_) => "global",
            Self::Tag(_) => "tag",
        }
    }

    pub(crate) fn info(&self, types: &TypeList) -> TypeInfo {
        match self {
            Self::Func(id) | Self::Tag(id) => types[*id].type_info(types),
            Self::Table(_) | Self::Memory(_) | Self::Global(_) => TypeInfo::new(),
        }
    }
}

trait ModuleImportKey {
    fn module(&self) -> &str;
    fn name(&self) -> &str;
}

impl<'a> Borrow<dyn ModuleImportKey + 'a> for (String, String) {
    fn borrow(&self) -> &(dyn ModuleImportKey + 'a) {
        self
    }
}

impl Hash for (dyn ModuleImportKey + '_) {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.module().hash(state);
        self.name().hash(state);
    }
}

impl PartialEq for (dyn ModuleImportKey + '_) {
    fn eq(&self, other: &Self) -> bool {
        self.module() == other.module() && self.name() == other.name()
    }
}

impl Eq for (dyn ModuleImportKey + '_) {}

impl ModuleImportKey for (String, String) {
    fn module(&self) -> &str {
        &self.0
    }

    fn name(&self) -> &str {
        &self.1
    }
}

impl ModuleImportKey for (&str, &str) {
    fn module(&self) -> &str {
        self.0
    }

    fn name(&self) -> &str {
        self.1
    }
}

/// Represents a core module type.
#[derive(Debug, Clone)]
pub struct ModuleType {
    /// Metadata about this module type
    pub(crate) info: TypeInfo,
    /// The imports of the module type.
    pub imports: IndexMap<(String, String), EntityType>,
    /// The exports of the module type.
    pub exports: IndexMap<String, EntityType>,
}

impl TypeData for ModuleType {
    type Id = ComponentCoreModuleTypeId;

    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        self.info
    }
}

impl ModuleType {
    /// Looks up an import by its module and name.
    ///
    /// Returns `None` if the import was not found.
    pub fn lookup_import(&self, module: &str, name: &str) -> Option<&EntityType> {
        self.imports.get(&(module, name) as &dyn ModuleImportKey)
    }
}

/// Represents the kind of module instance type.
#[derive(Debug, Clone)]
pub enum CoreInstanceTypeKind {
    /// The instance type is the result of instantiating a module type.
    Instantiated(ComponentCoreModuleTypeId),

    /// The instance type is the result of instantiating from exported items.
    Exports(IndexMap<String, EntityType>),
}

/// Represents a module instance type.
#[derive(Debug, Clone)]
pub struct InstanceType {
    /// Metadata about this instance type
    pub(crate) info: TypeInfo,
    /// The kind of module instance type.
    pub kind: CoreInstanceTypeKind,
}

impl TypeData for InstanceType {
    type Id = ComponentCoreInstanceTypeId;

    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        self.info
    }
}

impl InstanceType {
    /// Gets the exports of the instance type.
    pub fn exports<'a>(&'a self, types: TypesRef<'a>) -> &'a IndexMap<String, EntityType> {
        self.internal_exports(types.list)
    }

    pub(crate) fn internal_exports<'a>(
        &'a self,
        types: &'a TypeList,
    ) -> &'a IndexMap<String, EntityType> {
        match &self.kind {
            CoreInstanceTypeKind::Instantiated(id) => &types[*id].exports,
            CoreInstanceTypeKind::Exports(exports) => exports,
        }
    }
}

/// The entity type for imports and exports of a component.
#[derive(Debug, Clone, Copy)]
pub enum ComponentEntityType {
    /// The entity is a core module.
    Module(ComponentCoreModuleTypeId),
    /// The entity is a function.
    Func(ComponentFuncTypeId),
    /// The entity is a value.
    Value(ComponentValType),
    /// The entity is a type.
    Type {
        /// This is the identifier of the type that was referenced when this
        /// entity was created.
        referenced: ComponentAnyTypeId,
        /// This is the identifier of the type that was created when this type
        /// was imported or exported from the component.
        ///
        /// Note that the underlying type information for the `referenced`
        /// field and for this `created` field is the same, but these two types
        /// will hash to different values.
        created: ComponentAnyTypeId,
    },
    /// The entity is a component instance.
    Instance(ComponentInstanceTypeId),
    /// The entity is a component.
    Component(ComponentTypeId),
}

impl ComponentEntityType {
    /// Determines if component entity type `a` is a subtype of `b`.
    pub fn is_subtype_of(a: &Self, at: TypesRef, b: &Self, bt: TypesRef) -> bool {
        SubtypeCx::new(at.list, bt.list)
            .component_entity_type(a, b, 0)
            .is_ok()
    }

    pub(crate) fn desc(&self) -> &'static str {
        match self {
            Self::Module(_) => "module",
            Self::Func(_) => "func",
            Self::Value(_) => "value",
            Self::Type { .. } => "type",
            Self::Instance(_) => "instance",
            Self::Component(_) => "component",
        }
    }

    pub(crate) fn info(&self, types: &TypeList) -> TypeInfo {
        match self {
            Self::Module(ty) => types[*ty].type_info(types),
            Self::Func(ty) => types[*ty].type_info(types),
            Self::Type { referenced: ty, .. } => ty.info(types),
            Self::Instance(ty) => types[*ty].type_info(types),
            Self::Component(ty) => types[*ty].type_info(types),
            Self::Value(ty) => ty.info(types),
        }
    }
}

/// Represents a type of a component.
#[derive(Debug, Clone)]
pub struct ComponentType {
    /// Metadata about this component type
    pub(crate) info: TypeInfo,

    /// The imports of the component type.
    ///
    /// Each import has its own kebab-name and an optional URL listed. Note that
    /// the set of import names is disjoint with the set of export names.
    pub imports: IndexMap<String, ComponentEntityType>,

    /// The exports of the component type.
    ///
    /// Each export has its own kebab-name and an optional URL listed. Note that
    /// the set of export names is disjoint with the set of import names.
    pub exports: IndexMap<String, ComponentEntityType>,

    /// Universally quantified resources required to be provided when
    /// instantiating this component type.
    ///
    /// Each resource in this map is explicitly imported somewhere in the
    /// `imports` map. The "path" to where it's imported is specified by the
    /// `Vec<usize>` payload here. For more information about the indexes see
    /// the documentation on `ComponentState::imported_resources`.
    ///
    /// This should technically be inferrable from the structure of `imports`,
    /// but it's stored as an auxiliary set for subtype checking and
    /// instantiation.
    ///
    /// Note that this is not a set of all resources referred to by the
    /// `imports`. Instead it's only those created, relative to the internals of
    /// this component, by the imports.
    pub imported_resources: Vec<(ResourceId, Vec<usize>)>,

    /// The dual of the `imported_resources`, or the set of defined
    /// resources -- those created through the instantiation process which are
    /// unique to this component.
    ///
    /// This set is similar to the `imported_resources` set but it's those
    /// contained within the `exports`. Instantiating this component will
    /// create fresh new versions of all of these resources. The path here is
    /// within the `exports` array.
    pub defined_resources: Vec<(ResourceId, Vec<usize>)>,

    /// The set of all resources which are explicitly exported by this
    /// component, and where they're exported.
    ///
    /// This mapping is stored separately from `defined_resources` to ensure
    /// that it contains all exported resources, not just those which are
    /// defined. That means that this can cover reexports of imported
    /// resources, exports of local resources, or exports of closed-over
    /// resources for example.
    pub explicit_resources: IndexMap<ResourceId, Vec<usize>>,
}

impl TypeData for ComponentType {
    type Id = ComponentTypeId;

    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        self.info
    }
}

/// Represents a type of a component instance.
#[derive(Debug, Clone)]
pub struct ComponentInstanceType {
    /// Metadata about this instance type
    pub(crate) info: TypeInfo,

    /// The list of exports, keyed by name, that this instance has.
    ///
    /// An optional URL and type of each export is provided as well.
    pub exports: IndexMap<String, ComponentEntityType>,

    /// The list of "defined resources" or those which are closed over in
    /// this instance type.
    ///
    /// This list is populated, for example, when the type of an instance is
    /// declared and it contains its own resource type exports defined
    /// internally. For example:
    ///
    /// ```wasm
    /// (component
    ///     (type (instance
    ///         (export "x" (type sub resource)) ;; one `defined_resources` entry
    ///     ))
    /// )
    /// ```
    ///
    /// This list is also a bit of an oddity, however, because the type of a
    /// concrete instance will always have this as empty. For example:
    ///
    /// ```wasm
    /// (component
    ///     (type $t (instance (export "x" (type sub resource))))
    ///
    ///     ;; the type of this instance has no defined resources
    ///     (import "i" (instance (type $t)))
    /// )
    /// ```
    ///
    /// This list ends up only being populated for instance types declared in a
    /// module which aren't yet "attached" to anything. Once something is
    /// instantiated, imported, exported, or otherwise refers to a concrete
    /// instance then this list is always empty. For concrete instances
    /// defined resources are tracked in the component state or component type.
    pub defined_resources: Vec<ResourceId>,

    /// The list of all resources that are explicitly exported from this
    /// instance type along with the path they're exported at.
    pub explicit_resources: IndexMap<ResourceId, Vec<usize>>,
}

impl TypeData for ComponentInstanceType {
    type Id = ComponentInstanceTypeId;

    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        self.info
    }
}

/// Represents a type of a component function.
#[derive(Debug, Clone)]
pub struct ComponentFuncType {
    /// Metadata about this function type.
    pub(crate) info: TypeInfo,
    /// The function parameters.
    pub params: Box<[(KebabString, ComponentValType)]>,
    /// The function's results.
    pub results: Box<[(Option<KebabString>, ComponentValType)]>,
}

impl TypeData for ComponentFuncType {
    type Id = ComponentFuncTypeId;

    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        self.info
    }
}

impl ComponentFuncType {
    /// Lowers the component function type to core parameter and result types for the
    /// canonical ABI.
    pub(crate) fn lower(&self, types: &TypeList, is_lower: bool) -> LoweringInfo {
        let mut info = LoweringInfo::default();

        for (_, ty) in self.params.iter() {
            // Check to see if `ty` has a pointer somewhere in it, needed for
            // any type that transitively contains either a string or a list.
            // In this situation lowered functions must specify `memory`, and
            // lifted functions must specify `realloc` as well. Lifted functions
            // gain their memory requirement through the final clause of this
            // function.
            if is_lower {
                if !info.requires_memory {
                    info.requires_memory = ty.contains_ptr(types);
                }
            } else {
                if !info.requires_realloc {
                    info.requires_realloc = ty.contains_ptr(types);
                }
            }

            if !ty.push_wasm_types(types, &mut info.params) {
                // Too many parameters to pass directly
                // Function will have a single pointer parameter to pass the arguments
                // via linear memory
                info.params.clear();
                assert!(info.params.push(ValType::I32));
                info.requires_memory = true;

                // We need realloc as well when lifting a function
                if !is_lower {
                    info.requires_realloc = true;
                }
                break;
            }
        }

        for (_, ty) in self.results.iter() {
            // Results of lowered functions that contains pointers must be
            // allocated by the callee meaning that realloc is required.
            // Results of lifted function are allocated by the guest which
            // means that no realloc option is necessary.
            if is_lower && !info.requires_realloc {
                info.requires_realloc = ty.contains_ptr(types);
            }

            if !ty.push_wasm_types(types, &mut info.results) {
                // Too many results to return directly, either a retptr parameter will be used (import)
                // or a single pointer will be returned (export)
                info.results.clear();
                if is_lower {
                    info.params.max = MAX_LOWERED_TYPES;
                    assert!(info.params.push(ValType::I32));
                } else {
                    assert!(info.results.push(ValType::I32));
                }
                info.requires_memory = true;
                break;
            }
        }

        // Memory is always required when realloc is required
        info.requires_memory |= info.requires_realloc;

        info
    }
}

/// Represents a variant case.
#[derive(Debug, Clone)]
pub struct VariantCase {
    /// The variant case type.
    pub ty: Option<ComponentValType>,
    /// The name of the variant case refined by this one.
    pub refines: Option<KebabString>,
}

/// Represents a record type.
#[derive(Debug, Clone)]
pub struct RecordType {
    /// Metadata about this record type.
    pub(crate) info: TypeInfo,
    /// The map of record fields.
    pub fields: IndexMap<KebabString, ComponentValType>,
}

/// Represents a variant type.
#[derive(Debug, Clone)]
pub struct VariantType {
    /// Metadata about this variant type.
    pub(crate) info: TypeInfo,
    /// The map of variant cases.
    pub cases: IndexMap<KebabString, VariantCase>,
}

/// Represents a tuple type.
#[derive(Debug, Clone)]
pub struct TupleType {
    /// Metadata about this tuple type.
    pub(crate) info: TypeInfo,
    /// The types of the tuple.
    pub types: Box<[ComponentValType]>,
}

/// Represents a component defined type.
#[derive(Debug, Clone)]
pub enum ComponentDefinedType {
    /// The type is a primitive value type.
    Primitive(PrimitiveValType),
    /// The type is a record.
    Record(RecordType),
    /// The type is a variant.
    Variant(VariantType),
    /// The type is a list.
    List(ComponentValType),
    /// The type is a tuple.
    Tuple(TupleType),
    /// The type is a set of flags.
    Flags(IndexSet<KebabString>),
    /// The type is an enumeration.
    Enum(IndexSet<KebabString>),
    /// The type is an `option`.
    Option(ComponentValType),
    /// The type is a `result`.
    Result {
        /// The `ok` type.
        ok: Option<ComponentValType>,
        /// The `error` type.
        err: Option<ComponentValType>,
    },
    /// The type is an owned handle to the specified resource.
    Own(AliasableResourceId),
    /// The type is a borrowed handle to the specified resource.
    Borrow(AliasableResourceId),
}

impl TypeData for ComponentDefinedType {
    type Id = ComponentDefinedTypeId;

    fn type_info(&self, types: &TypeList) -> TypeInfo {
        match self {
            Self::Primitive(_) | Self::Flags(_) | Self::Enum(_) | Self::Own(_) => TypeInfo::new(),
            Self::Borrow(_) => TypeInfo::borrow(),
            Self::Record(r) => r.info,
            Self::Variant(v) => v.info,
            Self::Tuple(t) => t.info,
            Self::List(ty) | Self::Option(ty) => ty.info(types),
            Self::Result { ok, err } => {
                let default = TypeInfo::new();
                let mut info = ok.map(|ty| ty.type_info(types)).unwrap_or(default);
                info.combine(err.map(|ty| ty.type_info(types)).unwrap_or(default), 0)
                    .unwrap();
                info
            }
        }
    }
}

impl ComponentDefinedType {
    pub(crate) fn contains_ptr(&self, types: &TypeList) -> bool {
        match self {
            Self::Primitive(ty) => ty.contains_ptr(),
            Self::Record(r) => r.fields.values().any(|ty| ty.contains_ptr(types)),
            Self::Variant(v) => v
                .cases
                .values()
                .any(|case| case.ty.map(|ty| ty.contains_ptr(types)).unwrap_or(false)),
            Self::List(_) => true,
            Self::Tuple(t) => t.types.iter().any(|ty| ty.contains_ptr(types)),
            Self::Flags(_) | Self::Enum(_) | Self::Own(_) | Self::Borrow(_) => false,
            Self::Option(ty) => ty.contains_ptr(types),
            Self::Result { ok, err } => {
                ok.map(|ty| ty.contains_ptr(types)).unwrap_or(false)
                    || err.map(|ty| ty.contains_ptr(types)).unwrap_or(false)
            }
        }
    }

    fn push_wasm_types(&self, types: &TypeList, lowered_types: &mut LoweredTypes) -> bool {
        match self {
            Self::Primitive(ty) => push_primitive_wasm_types(ty, lowered_types),
            Self::Record(r) => r
                .fields
                .iter()
                .all(|(_, ty)| ty.push_wasm_types(types, lowered_types)),
            Self::Variant(v) => Self::push_variant_wasm_types(
                v.cases.iter().filter_map(|(_, case)| case.ty.as_ref()),
                types,
                lowered_types,
            ),
            Self::List(_) => lowered_types.push(ValType::I32) && lowered_types.push(ValType::I32),
            Self::Tuple(t) => t
                .types
                .iter()
                .all(|ty| ty.push_wasm_types(types, lowered_types)),
            Self::Flags(names) => {
                (0..(names.len() + 31) / 32).all(|_| lowered_types.push(ValType::I32))
            }
            Self::Enum(_) | Self::Own(_) | Self::Borrow(_) => lowered_types.push(ValType::I32),
            Self::Option(ty) => {
                Self::push_variant_wasm_types([ty].into_iter(), types, lowered_types)
            }
            Self::Result { ok, err } => {
                Self::push_variant_wasm_types(ok.iter().chain(err.iter()), types, lowered_types)
            }
        }
    }

    fn push_variant_wasm_types<'a>(
        cases: impl Iterator<Item = &'a ComponentValType>,
        types: &TypeList,
        lowered_types: &mut LoweredTypes,
    ) -> bool {
        // Push the discriminant
        if !lowered_types.push(ValType::I32) {
            return false;
        }

        let start = lowered_types.len();

        for ty in cases {
            let mut temp = LoweredTypes::new(lowered_types.max);

            if !ty.push_wasm_types(types, &mut temp) {
                return false;
            }

            for (i, ty) in temp.iter().enumerate() {
                match lowered_types.get_mut(start + i) {
                    Some(prev) => *prev = Self::join_types(*prev, ty),
                    None => {
                        if !lowered_types.push(ty) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn join_types(a: ValType, b: ValType) -> ValType {
        use ValType::*;

        match (a, b) {
            (I32, I32) | (I64, I64) | (F32, F32) | (F64, F64) => a,
            (I32, F32) | (F32, I32) => I32,
            (_, I64 | F64) | (I64 | F64, _) => I64,
            _ => panic!("unexpected wasm type for canonical ABI"),
        }
    }

    fn desc(&self) -> &'static str {
        match self {
            ComponentDefinedType::Record(_) => "record",
            ComponentDefinedType::Primitive(_) => "primitive",
            ComponentDefinedType::Variant(_) => "variant",
            ComponentDefinedType::Tuple(_) => "tuple",
            ComponentDefinedType::Enum(_) => "enum",
            ComponentDefinedType::Flags(_) => "flags",
            ComponentDefinedType::Option(_) => "option",
            ComponentDefinedType::List(_) => "list",
            ComponentDefinedType::Result { .. } => "result",
            ComponentDefinedType::Own(_) => "own",
            ComponentDefinedType::Borrow(_) => "borrow",
        }
    }
}

/// An opaque identifier intended to be used to distinguish whether two
/// resource types are equivalent or not.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Copy)]
#[repr(packed(4))] // try to not waste 4 bytes in padding
pub struct ResourceId {
    // This is a globally unique identifier which is assigned once per
    // `TypeAlloc`. This ensures that resource identifiers from different
    // instances of `Types`, for example, are considered unique.
    //
    // Technically 64-bits should be enough for all resource ids ever, but
    // they're allocated so often it's predicted that an atomic increment
    // per resource id is probably too expensive. To amortize that cost each
    // top-level wasm component gets a single globally unique identifier, and
    // then within a component contextually unique identifiers are handed out.
    globally_unique_id: usize,

    // A contextually unique id within the globally unique id above. This is
    // allocated within a `TypeAlloc` with its own counter, and allocations of
    // this are cheap as nothing atomic is required.
    //
    // The 32-bit storage here should ideally be enough for any component
    // containing resources. If memory usage becomes an issue (this struct is
    // 12 bytes instead of 8 or 4) then this could get folded into the globally
    // unique id with everything using an atomic increment perhaps.
    contextually_unique_id: u32,
}

#[allow(clippy::large_enum_variant)]
enum TypesKind {
    Module(Arc<Module>),
    Component(ComponentState),
}

/// Represents the types known to a [`crate::Validator`] once validation has completed.
///
/// The type information is returned via the [`crate::Validator::end`] method.
pub struct Types {
    list: TypeList,
    kind: TypesKind,
}

#[derive(Clone, Copy)]
enum TypesRefKind<'a> {
    Module(&'a Module),
    Component(&'a ComponentState),
}

/// Represents the types known to a [`crate::Validator`] during validation.
///
/// Retrieved via the [`crate::Validator::types`] method.
#[derive(Clone, Copy)]
pub struct TypesRef<'a> {
    list: &'a TypeList,
    kind: TypesRefKind<'a>,
}

impl<'a> TypesRef<'a> {
    pub(crate) fn from_module(types: &'a TypeList, module: &'a Module) -> Self {
        Self {
            list: types,
            kind: TypesRefKind::Module(module),
        }
    }

    pub(crate) fn from_component(types: &'a TypeList, component: &'a ComponentState) -> Self {
        Self {
            list: types,
            kind: TypesRefKind::Component(component),
        }
    }

    /// Gets a type based on its type id.
    ///
    /// Returns `None` if the type id is unknown.
    pub fn get<T>(&self, id: T) -> Option<&'a T::Data>
    where
        T: TypeIdentifier,
    {
        self.list.get(id)
    }

    /// Gets a core WebAssembly type id from a type index.
    ///
    /// Note that this is in contrast to [`TypesRef::component_type_at`] which
    /// gets a component type from its index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn core_type_at(&self, index: u32) -> ComponentCoreTypeId {
        match &self.kind {
            TypesRefKind::Module(module) => ComponentCoreTypeId::Sub(module.types[index as usize]),
            TypesRefKind::Component(component) => component.core_types[index as usize],
        }
    }

    /// Gets a type id from a type index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid type index or if this type information
    /// represents a core module.
    pub fn component_any_type_at(&self, index: u32) -> ComponentAnyTypeId {
        match &self.kind {
            TypesRefKind::Module(_) => panic!("not a component"),
            TypesRefKind::Component(component) => component.types[index as usize],
        }
    }

    /// Gets a component type id from a type index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid component type index or if this type
    /// information represents a core module.
    pub fn component_type_at(&self, index: u32) -> ComponentTypeId {
        match self.component_any_type_at(index) {
            ComponentAnyTypeId::Component(id) => id,
            _ => panic!("not a component type"),
        }
    }

    /// Gets a type id from a type index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid function index or if this type
    /// information represents a core module.
    pub fn component_defined_type_at(&self, index: u32) -> ComponentDefinedTypeId {
        match self.component_any_type_at(index) {
            ComponentAnyTypeId::Defined(id) => id,
            _ => panic!("not a defined type"),
        }
    }

    /// Returns the number of core types defined so far.
    pub fn core_type_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.types.len() as u32,
            TypesRefKind::Component(component) => component.core_types.len() as u32,
        }
    }

    /// Returns the number of component types defined so far.
    pub fn component_type_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(_module) => 0,
            TypesRefKind::Component(component) => component.types.len() as u32,
        }
    }

    /// Gets the type of a table at the given table index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn table_at(&self, index: u32) -> TableType {
        let tables = match &self.kind {
            TypesRefKind::Module(module) => &module.tables,
            TypesRefKind::Component(component) => &component.core_tables,
        };
        tables[index as usize]
    }

    /// Returns the number of tables defined so far.
    pub fn table_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.tables.len() as u32,
            TypesRefKind::Component(component) => component.core_tables.len() as u32,
        }
    }

    /// Gets the type of a memory at the given memory index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn memory_at(&self, index: u32) -> MemoryType {
        let memories = match &self.kind {
            TypesRefKind::Module(module) => &module.memories,
            TypesRefKind::Component(component) => &component.core_memories,
        };

        memories[index as usize]
    }

    /// Returns the number of memories defined so far.
    pub fn memory_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.memories.len() as u32,
            TypesRefKind::Component(component) => component.core_memories.len() as u32,
        }
    }

    /// Gets the type of a global at the given global index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn global_at(&self, index: u32) -> GlobalType {
        let globals = match &self.kind {
            TypesRefKind::Module(module) => &module.globals,
            TypesRefKind::Component(component) => &component.core_globals,
        };

        globals[index as usize]
    }

    /// Returns the number of globals defined so far.
    pub fn global_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.globals.len() as u32,
            TypesRefKind::Component(component) => component.core_globals.len() as u32,
        }
    }

    /// Gets the type of a tag at the given tag index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn tag_at(&self, index: u32) -> CoreTypeId {
        let tags = match &self.kind {
            TypesRefKind::Module(module) => &module.tags,
            TypesRefKind::Component(component) => &component.core_tags,
        };
        tags[index as usize]
    }

    /// Returns the number of tags defined so far.
    pub fn tag_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.tags.len() as u32,
            TypesRefKind::Component(component) => component.core_tags.len() as u32,
        }
    }

    /// Gets the type of a core function at the given function index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn core_function_at(&self, index: u32) -> CoreTypeId {
        match &self.kind {
            TypesRefKind::Module(module) => module.types[module.functions[index as usize] as usize],
            TypesRefKind::Component(component) => component.core_funcs[index as usize],
        }
    }

    /// Gets the count of core functions defined so far.
    ///
    /// Note that this includes imported functions, defined functions, and for
    /// components lowered/aliased functions.
    pub fn function_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.functions.len() as u32,
            TypesRefKind::Component(component) => component.core_funcs.len() as u32,
        }
    }

    /// Gets the type of an element segment at the given element segment index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn element_at(&self, index: u32) -> RefType {
        match &self.kind {
            TypesRefKind::Module(module) => module.element_types[index as usize],
            TypesRefKind::Component(_) => {
                panic!("no elements on a component")
            }
        }
    }

    /// Returns the number of elements defined so far.
    pub fn element_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.element_types.len() as u32,
            TypesRefKind::Component(_) => 0,
        }
    }

    /// Gets the type of a component function at the given function index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn component_function_at(&self, index: u32) -> ComponentFuncTypeId {
        match &self.kind {
            TypesRefKind::Module(_) => panic!("not a component"),
            TypesRefKind::Component(component) => component.funcs[index as usize],
        }
    }

    /// Returns the number of component functions defined so far.
    pub fn component_function_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(_module) => 0,
            TypesRefKind::Component(component) => component.funcs.len() as u32,
        }
    }

    /// Gets the type of a module at the given module index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn module_at(&self, index: u32) -> ComponentCoreModuleTypeId {
        match &self.kind {
            TypesRefKind::Module(_) => panic!("not a component"),
            TypesRefKind::Component(component) => component.core_modules[index as usize],
        }
    }

    /// Returns the number of core wasm modules defined so far.
    pub fn module_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(_module) => 0,
            TypesRefKind::Component(component) => component.core_modules.len() as u32,
        }
    }

    /// Gets the type of a module instance at the given module instance index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn core_instance_at(&self, index: u32) -> ComponentCoreInstanceTypeId {
        match &self.kind {
            TypesRefKind::Module(_) => panic!("not a component"),
            TypesRefKind::Component(component) => component.core_instances[index as usize],
        }
    }

    /// Returns the number of core wasm instances defined so far.
    pub fn core_instance_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(_module) => 0,
            TypesRefKind::Component(component) => component.core_instances.len() as u32,
        }
    }

    /// Gets the type of a component at the given component index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn component_at(&self, index: u32) -> ComponentTypeId {
        match &self.kind {
            TypesRefKind::Module(_) => panic!("not a component"),
            TypesRefKind::Component(component) => component.components[index as usize],
        }
    }

    /// Returns the number of components defined so far.
    pub fn component_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(_module) => 0,
            TypesRefKind::Component(component) => component.components.len() as u32,
        }
    }

    /// Gets the type of an component instance at the given component instance index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn component_instance_at(&self, index: u32) -> ComponentInstanceTypeId {
        match &self.kind {
            TypesRefKind::Module(_) => panic!("not a component"),
            TypesRefKind::Component(component) => component.instances[index as usize],
        }
    }

    /// Returns the number of component instances defined so far.
    pub fn component_instance_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(_module) => 0,
            TypesRefKind::Component(component) => component.instances.len() as u32,
        }
    }

    /// Gets the type of a value at the given value index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn value_at(&self, index: u32) -> ComponentValType {
        match &self.kind {
            TypesRefKind::Module(_) => panic!("not a component"),
            TypesRefKind::Component(component) => component.values[index as usize].0,
        }
    }

    /// Gets the entity type for the given import.
    pub fn entity_type_from_import(&self, import: &Import) -> Option<EntityType> {
        match &self.kind {
            TypesRefKind::Module(module) => Some(match import.ty {
                TypeRef::Func(idx) => EntityType::Func(*module.types.get(idx as usize)?),
                TypeRef::Table(ty) => EntityType::Table(ty),
                TypeRef::Memory(ty) => EntityType::Memory(ty),
                TypeRef::Global(ty) => EntityType::Global(ty),
                TypeRef::Tag(ty) => EntityType::Tag(*module.types.get(ty.func_type_idx as usize)?),
            }),
            TypesRefKind::Component(_) => None,
        }
    }

    /// Gets the entity type from the given export.
    pub fn entity_type_from_export(&self, export: &Export) -> Option<EntityType> {
        match &self.kind {
            TypesRefKind::Module(module) => Some(match export.kind {
                ExternalKind::Func => EntityType::Func(
                    module.types[*module.functions.get(export.index as usize)? as usize],
                ),
                ExternalKind::Table => {
                    EntityType::Table(*module.tables.get(export.index as usize)?)
                }
                ExternalKind::Memory => {
                    EntityType::Memory(*module.memories.get(export.index as usize)?)
                }
                ExternalKind::Global => {
                    EntityType::Global(*module.globals.get(export.index as usize)?)
                }
                ExternalKind::Tag => EntityType::Tag(
                    module.types[*module.functions.get(export.index as usize)? as usize],
                ),
            }),
            TypesRefKind::Component(_) => None,
        }
    }

    /// Gets the component entity type for the given component import.
    pub fn component_entity_type_of_import(&self, name: &str) -> Option<ComponentEntityType> {
        match &self.kind {
            TypesRefKind::Module(_) => None,
            TypesRefKind::Component(component) => Some(*component.imports.get(name)?),
        }
    }

    /// Gets the component entity type for the given component export.
    pub fn component_entity_type_of_export(&self, name: &str) -> Option<ComponentEntityType> {
        match &self.kind {
            TypesRefKind::Module(_) => None,
            TypesRefKind::Component(component) => Some(*component.exports.get(name)?),
        }
    }

    /// Attempts to lookup the type id that `ty` is an alias of.
    ///
    /// Returns `None` if `ty` wasn't listed as aliasing a prior type.
    pub fn peel_alias<T>(&self, ty: T) -> Option<T>
    where
        T: Aliasable,
    {
        self.list.peel_alias(ty)
    }
}

impl<T> Index<T> for TypesRef<'_>
where
    T: TypeIdentifier,
{
    type Output = T::Data;

    fn index(&self, index: T) -> &Self::Output {
        &self.list[index]
    }
}

impl Types {
    pub(crate) fn from_module(types: TypeList, module: Arc<Module>) -> Self {
        Self {
            list: types,
            kind: TypesKind::Module(module),
        }
    }

    pub(crate) fn from_component(types: TypeList, component: ComponentState) -> Self {
        Self {
            list: types,
            kind: TypesKind::Component(component),
        }
    }

    /// Gets a reference to this validation type information.
    pub fn as_ref(&self) -> TypesRef {
        TypesRef {
            list: &self.list,
            kind: match &self.kind {
                TypesKind::Module(module) => TypesRefKind::Module(module),
                TypesKind::Component(component) => TypesRefKind::Component(component),
            },
        }
    }

    /// Gets a type based on its type id.
    ///
    /// Returns `None` if the type id is unknown.
    pub fn get<T>(&self, id: T) -> Option<&T::Data>
    where
        T: TypeIdentifier,
    {
        self.as_ref().get(id)
    }

    /// Gets a core WebAssembly type at the given type index.
    ///
    /// Note that this is in contrast to [`TypesRef::component_type_at`] which
    /// gets a component type from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid function index.
    pub fn core_type_at(&self, index: u32) -> ComponentCoreTypeId {
        self.as_ref().core_type_at(index)
    }

    /// Gets a component WebAssembly type at the given type index.
    ///
    /// Note that this is in contrast to [`TypesRef::core_type_at`] which gets a
    /// core type from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid type index.
    pub fn component_any_type_at(&self, index: u32) -> ComponentAnyTypeId {
        self.as_ref().component_any_type_at(index)
    }

    /// Gets a component type at the given type index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid component type index.
    pub fn component_type_at(&self, index: u32) -> ComponentTypeId {
        self.as_ref().component_type_at(index)
    }

    /// Gets a component type from the given component type index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid defined type index or if this type
    /// information represents a core module.
    pub fn component_defined_type_at(&self, index: u32) -> ComponentDefinedTypeId {
        self.as_ref().component_defined_type_at(index)
    }

    /// Gets the count of core types.
    pub fn type_count(&self) -> usize {
        match &self.kind {
            TypesKind::Module(module) => module.types.len(),
            TypesKind::Component(component) => component.core_types.len(),
        }
    }

    /// Gets the type of a table at the given table index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid function index.
    pub fn table_at(&self, index: u32) -> TableType {
        self.as_ref().table_at(index)
    }

    /// Gets the count of imported and defined tables.
    pub fn table_count(&self) -> usize {
        match &self.kind {
            TypesKind::Module(module) => module.tables.len(),
            TypesKind::Component(component) => component.core_tables.len(),
        }
    }

    /// Gets the type of a memory at the given memory index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid function index.
    pub fn memory_at(&self, index: u32) -> MemoryType {
        self.as_ref().memory_at(index)
    }

    /// Gets the count of imported and defined memories.
    pub fn memory_count(&self) -> u32 {
        self.as_ref().memory_count()
    }

    /// Gets the type of a global at the given global index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid function index.
    pub fn global_at(&self, index: u32) -> GlobalType {
        self.as_ref().global_at(index)
    }

    /// Gets the count of imported and defined globals.
    pub fn global_count(&self) -> u32 {
        self.as_ref().global_count()
    }

    /// Gets the type of a tag at the given tag index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid function index.
    pub fn tag_at(&self, index: u32) -> CoreTypeId {
        self.as_ref().tag_at(index)
    }

    /// Gets the count of imported and defined tags.
    pub fn tag_count(&self) -> u32 {
        self.as_ref().tag_count()
    }

    /// Gets the type of a core function at the given function index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not a valid function index.
    pub fn core_function_at(&self, index: u32) -> CoreTypeId {
        self.as_ref().core_function_at(index)
    }

    /// Gets the count of core functions defined so far.
    ///
    /// Note that this includes imported functions, defined functions, and for
    /// components lowered/aliased functions.
    pub fn core_function_count(&self) -> u32 {
        self.as_ref().function_count()
    }

    /// Gets the type of an element segment at the given element segment index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn element_at(&self, index: u32) -> RefType {
        self.as_ref().element_at(index)
    }

    /// Gets the count of element segments.
    pub fn element_count(&self) -> u32 {
        self.as_ref().element_count()
    }

    /// Gets the type of a component function at the given function index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn component_function_at(&self, index: u32) -> ComponentFuncTypeId {
        self.as_ref().component_function_at(index)
    }

    /// Gets the count of imported, exported, or aliased component functions.
    pub fn component_function_count(&self) -> u32 {
        self.as_ref().component_function_count()
    }

    /// Gets the type of a module at the given module index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn module_at(&self, index: u32) -> ComponentCoreModuleTypeId {
        self.as_ref().module_at(index)
    }

    /// Gets the count of imported, exported, or aliased modules.
    pub fn module_count(&self) -> usize {
        match &self.kind {
            TypesKind::Module(_) => 0,
            TypesKind::Component(component) => component.core_modules.len(),
        }
    }

    /// Gets the type of a module instance at the given module instance index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn core_instance_at(&self, index: u32) -> ComponentCoreInstanceTypeId {
        self.as_ref().core_instance_at(index)
    }

    /// Gets the count of imported, exported, or aliased core module instances.
    pub fn core_instance_count(&self) -> usize {
        match &self.kind {
            TypesKind::Module(_) => 0,
            TypesKind::Component(component) => component.core_instances.len(),
        }
    }

    /// Gets the type of a component at the given component index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn component_at(&self, index: u32) -> ComponentTypeId {
        self.as_ref().component_at(index)
    }

    /// Gets the count of imported, exported, or aliased components.
    pub fn component_count(&self) -> usize {
        match &self.kind {
            TypesKind::Module(_) => 0,
            TypesKind::Component(component) => component.components.len(),
        }
    }

    /// Gets the type of an component instance at the given component instance index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn component_instance_at(&self, index: u32) -> ComponentInstanceTypeId {
        self.as_ref().component_instance_at(index)
    }

    /// Gets the count of imported, exported, or aliased component instances.
    pub fn component_instance_count(&self) -> usize {
        match &self.kind {
            TypesKind::Module(_) => 0,
            TypesKind::Component(component) => component.instances.len(),
        }
    }

    /// Gets the type of a value at the given value index.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds or if this type
    /// information represents a core module.
    pub fn value_at(&self, index: u32) -> ComponentValType {
        self.as_ref().value_at(index)
    }

    /// Gets the count of imported, exported, or aliased values.
    pub fn value_count(&self) -> usize {
        match &self.kind {
            TypesKind::Module(_) => 0,
            TypesKind::Component(component) => component.values.len(),
        }
    }

    /// Gets the entity type from the given import.
    pub fn entity_type_from_import(&self, import: &Import) -> Option<EntityType> {
        self.as_ref().entity_type_from_import(import)
    }

    /// Gets the entity type from the given export.
    pub fn entity_type_from_export(&self, export: &Export) -> Option<EntityType> {
        self.as_ref().entity_type_from_export(export)
    }

    /// Gets the component entity type for the given component import name.
    pub fn component_entity_type_of_import(&self, name: &str) -> Option<ComponentEntityType> {
        self.as_ref().component_entity_type_of_import(name)
    }

    /// Gets the component entity type for the given component export name.
    pub fn component_entity_type_of_export(&self, name: &str) -> Option<ComponentEntityType> {
        self.as_ref().component_entity_type_of_export(name)
    }

    /// Attempts to lookup the type id that `ty` is an alias of.
    ///
    /// Returns `None` if `ty` wasn't listed as aliasing a prior type.
    pub fn peel_alias<T>(&self, ty: T) -> Option<T>
    where
        T: Aliasable,
    {
        self.list.peel_alias(ty)
    }
}

impl<T> Index<T> for Types
where
    T: TypeIdentifier,
{
    type Output = T::Data;

    fn index(&self, id: T) -> &Self::Output {
        &self.list[id]
    }
}

/// This is a type which mirrors a subset of the `Vec<T>` API, but is intended
/// to be able to be cheaply snapshotted and cloned.
///
/// When each module's code sections start we "commit" the current list of types
/// in the global list of types. This means that the temporary `cur` vec here is
/// pushed onto `snapshots` and wrapped up in an `Arc`. At that point we clone
/// this entire list (which is then O(modules), not O(types in all modules)) and
/// pass out as a context to each function validator.
///
/// Otherwise, though, this type behaves as if it were a large `Vec<T>`, but
/// it's represented by lists of contiguous chunks.
//
// Only public because it shows up in a public trait's `doc(hidden)` method.
#[doc(hidden)]
pub struct SnapshotList<T> {
    // All previous snapshots, the "head" of the list that this type represents.
    // The first entry in this pair is the starting index for all elements
    // contained in the list, and the second element is the list itself. Note
    // the `Arc` wrapper around sub-lists, which makes cloning time for this
    // `SnapshotList` O(snapshots) rather than O(snapshots_total), which for
    // us in this context means the number of modules, not types.
    //
    // Note that this list is sorted least-to-greatest in order of the index for
    // binary searching.
    snapshots: Vec<Arc<Snapshot<T>>>,

    // This is the total length of all lists in the `snapshots` array.
    snapshots_total: usize,

    // The current list of types for the current snapshot that are being built.
    cur: Vec<T>,
}

struct Snapshot<T> {
    prior_types: usize,
    items: Vec<T>,
}

impl<T> SnapshotList<T> {
    /// Same as `<&[T]>::get`
    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        // Check to see if this index falls on our local list
        if index >= self.snapshots_total {
            return self.cur.get(index - self.snapshots_total);
        }
        // ... and failing that we do a binary search to figure out which bucket
        // it's in. Note the `i-1` in the `Err` case because if we don't find an
        // exact match the type is located in the previous bucket.
        let i = match self
            .snapshots
            .binary_search_by_key(&index, |snapshot| snapshot.prior_types)
        {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        let snapshot = &self.snapshots[i];
        Some(&snapshot.items[index - snapshot.prior_types])
    }

    /// Same as `Vec::push`
    pub(crate) fn push(&mut self, val: T) {
        self.cur.push(val);
    }

    /// Same as `<[T]>::len`
    pub(crate) fn len(&self) -> usize {
        self.cur.len() + self.snapshots_total
    }

    /// Same as `Vec::truncate` but can only truncate uncommitted elements.
    pub(crate) fn truncate(&mut self, len: usize) {
        assert!(len >= self.snapshots_total);
        self.cur.truncate(len - self.snapshots_total);
    }

    /// Commits previously pushed types into this snapshot vector, and returns a
    /// clone of this list.
    ///
    /// The returned `SnapshotList` can be used to access all the same types as
    /// this list itself. This list also is not changed (from an external
    /// perspective) and can continue to access all the same types.
    pub(crate) fn commit(&mut self) -> SnapshotList<T> {
        // If the current chunk has new elements, commit them in to an
        // `Arc`-wrapped vector in the snapshots list. Note the `shrink_to_fit`
        // ahead of time to hopefully keep memory usage lower than it would
        // otherwise be.
        let len = self.cur.len();
        if len > 0 {
            self.cur.shrink_to_fit();
            self.snapshots.push(Arc::new(Snapshot {
                prior_types: self.snapshots_total,
                items: mem::take(&mut self.cur),
            }));
            self.snapshots_total += len;
        }
        SnapshotList {
            snapshots: self.snapshots.clone(),
            snapshots_total: self.snapshots_total,
            cur: Vec::new(),
        }
    }
}

impl<T> Index<usize> for SnapshotList<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T, U> Index<U> for SnapshotList<T>
where
    U: TypeIdentifier<Data = T>,
{
    type Output = T;

    #[inline]
    fn index(&self, id: U) -> &T {
        self.get(id.index()).unwrap()
    }
}

impl<T> Default for SnapshotList<T> {
    fn default() -> SnapshotList<T> {
        SnapshotList {
            snapshots: Vec::new(),
            snapshots_total: 0,
            cur: Vec::new(),
        }
    }
}

/// A snapshot list of types.
///
/// Note that the snapshot lists below do not correspond with index spaces. Many
/// different kinds of types are in the same index space (e.g. all of the
/// component model's {component, instance, defined, func} types are in the same
/// index space). However, we store each of them in their own type-specific
/// snapshot list and give each of them their own identifier type.
#[derive(Default)]
// Only public because it shows up in a public trait's `doc(hidden)` method.
#[doc(hidden)]
pub struct TypeList {
    // Keeps track of which `alias_id` is an alias of which other `alias_id`.
    alias_mappings: HashMap<u32, u32>,
    // Counter for generating new `alias_id`s.
    alias_counter: u32,
    // Snapshots of previously committed `TypeList`s' aliases.
    alias_snapshots: Vec<TypeListAliasSnapshot>,

    // Core Wasm types.
    //
    // A primary map from `CoreTypeId` to `SubType`.
    core_types: SnapshotList<SubType>,
    // The id of each core Wasm type's rec group.
    //
    // A secondary map from `CoreTypeId` to `RecGroupId`.
    core_type_to_rec_group: SnapshotList<RecGroupId>,
    // The supertype of each core type.
    //
    // A secondary map from `coreTypeId` to `Option<CoreTypeId>`.
    core_type_to_supertype: SnapshotList<Option<CoreTypeId>>,
    // A primary map from `RecGroupId` to the range of the rec group's elements
    // within `core_types`.
    rec_group_elements: SnapshotList<Range<CoreTypeId>>,
    // A hash map from rec group elements to their canonical `RecGroupId`.
    //
    // This is `None` when a list is "committed" meaning that no more insertions
    // can happen.
    canonical_rec_groups: Option<HashMap<RecGroup, RecGroupId>>,

    // Component model types.
    components: SnapshotList<ComponentType>,
    component_defined_types: SnapshotList<ComponentDefinedType>,
    component_values: SnapshotList<ComponentValType>,
    component_instances: SnapshotList<ComponentInstanceType>,
    component_funcs: SnapshotList<ComponentFuncType>,
    core_modules: SnapshotList<ModuleType>,
    core_instances: SnapshotList<InstanceType>,
}

#[derive(Clone, Debug)]
struct TypeListAliasSnapshot {
    // The `alias_counter` at the time that this snapshot was taken.
    alias_counter: u32,

    // The alias mappings in this snapshot.
    alias_mappings: HashMap<u32, u32>,
}

struct TypeListCheckpoint {
    core_types: usize,
    components: usize,
    component_defined_types: usize,
    component_values: usize,
    component_instances: usize,
    component_funcs: usize,
    core_modules: usize,
    core_instances: usize,
    core_type_to_rec_group: usize,
    core_type_to_supertype: usize,
    rec_group_elements: usize,
    canonical_rec_groups: usize,
}

impl TypeList {
    pub fn get<T>(&self, id: T) -> Option<&T::Data>
    where
        T: TypeIdentifier,
    {
        T::list(self).get(id.index())
    }

    pub fn push<T>(&mut self, ty: T) -> T::Id
    where
        T: TypeData,
    {
        let index = u32::try_from(T::Id::list(self).len()).unwrap();
        let id = T::Id::from_index(index);
        T::Id::list_mut(self).push(ty);
        id
    }

    /// Intern the given recursion group (that has already been canonicalized)
    /// and return its associated id and whether this was a new recursion group
    /// or not.
    pub fn intern_canonical_rec_group(&mut self, rec_group: RecGroup) -> (bool, RecGroupId) {
        let canonical_rec_groups = self
            .canonical_rec_groups
            .as_mut()
            .expect("cannot intern into a committed list");
        let entry = match canonical_rec_groups.entry(rec_group) {
            Entry::Occupied(e) => return (false, *e.get()),
            Entry::Vacant(e) => e,
        };

        let rec_group_id = self.rec_group_elements.len();
        let rec_group_id = u32::try_from(rec_group_id).unwrap();
        let rec_group_id = RecGroupId::from_index(rec_group_id);

        let start = self.core_types.len();
        let start = u32::try_from(start).unwrap();
        let start = CoreTypeId::from_index(start);

        for ty in entry.key().types() {
            debug_assert_eq!(self.core_types.len(), self.core_type_to_supertype.len());
            debug_assert_eq!(self.core_types.len(), self.core_type_to_rec_group.len());

            self.core_type_to_supertype
                .push(ty.supertype_idx.map(|idx| match idx.unpack() {
                    UnpackedIndex::RecGroup(offset) => CoreTypeId::from_index(start.index + offset),
                    UnpackedIndex::Id(id) => id,
                    UnpackedIndex::Module(_) => unreachable!("in canonical form"),
                }));
            let mut ty = ty.clone();
            ty.remap_indices(&mut |index| {
                match index.unpack() {
                    UnpackedIndex::Id(_) => {}
                    UnpackedIndex::Module(_) => unreachable!(),
                    UnpackedIndex::RecGroup(offset) => {
                        *index = UnpackedIndex::Id(CoreTypeId::from_index(start.index + offset))
                            .pack()
                            .unwrap();
                    }
                };
                Ok(())
            })
            .expect("cannot fail");
            self.core_types.push(ty);
            self.core_type_to_rec_group.push(rec_group_id);
        }

        let end = self.core_types.len();
        let end = u32::try_from(end).unwrap();
        let end = CoreTypeId::from_index(end);

        let range = start..end;

        self.rec_group_elements.push(range.clone());

        entry.insert(rec_group_id);
        return (true, rec_group_id);
    }

    /// Get the `CoreTypeId` for a local index into a rec group.
    pub fn rec_group_local_id(
        &self,
        rec_group: RecGroupId,
        index: u32,
        offset: usize,
    ) -> Result<CoreTypeId> {
        let elems = &self[rec_group];
        let len = elems.end.index() - elems.start.index();
        let len = u32::try_from(len).unwrap();
        if index < len {
            let id = u32::try_from(elems.start.index()).unwrap() + index;
            let id = CoreTypeId::from_index(id);
            Ok(id)
        } else {
            bail!(
                offset,
                "unknown type {index}: type index out of rec group bounds"
            )
        }
    }

    /// Get the id of the rec group that the given type id was defined within.
    pub fn rec_group_id_of(&self, id: CoreTypeId) -> RecGroupId {
        self.core_type_to_rec_group[id.index()]
    }

    /// Get the super type of the given type id, if any.
    pub fn supertype_of(&self, id: CoreTypeId) -> Option<CoreTypeId> {
        self.core_type_to_supertype[id.index()]
    }

    /// Get the `CoreTypeId` for a canonicalized `PackedIndex`.
    ///
    /// Panics when given a non-canonicalized `PackedIndex`.
    pub fn at_canonicalized_packed_index(
        &self,
        rec_group: RecGroupId,
        index: PackedIndex,
        offset: usize,
    ) -> Result<CoreTypeId> {
        self.at_canonicalized_unpacked_index(rec_group, index.unpack(), offset)
    }

    /// Get the `CoreTypeId` for a canonicalized `UnpackedIndex`.
    ///
    /// Panics when given a non-canonicalized `PackedIndex`.
    pub fn at_canonicalized_unpacked_index(
        &self,
        rec_group: RecGroupId,
        index: UnpackedIndex,
        offset: usize,
    ) -> Result<CoreTypeId> {
        match index {
            UnpackedIndex::Module(_) => panic!("not canonicalized"),
            UnpackedIndex::Id(id) => Ok(id),
            UnpackedIndex::RecGroup(idx) => self.rec_group_local_id(rec_group, idx, offset),
        }
    }

    /// Does `a` structurally match `b`?
    pub fn matches(&self, a: CoreTypeId, b: CoreTypeId) -> bool {
        let a = WithRecGroup::new(self, a);
        let a = WithRecGroup::map(a, |a| &self[a]);

        let b = WithRecGroup::new(self, b);
        let b = WithRecGroup::map(b, |b| &self[b]);

        Matches::matches(self, a, b)
    }

    /// Is `a == b` or was `a` declared (potentially transitively) to be a
    /// subtype of `b`?
    pub fn id_is_subtype(&self, mut a: CoreTypeId, b: CoreTypeId) -> bool {
        loop {
            if a == b {
                return true;
            }

            // TODO: maintain supertype vectors and implement this check in O(1)
            // instead of O(n) time.
            a = match self.supertype_of(a) {
                Some(a) => a,
                None => return false,
            };
        }
    }

    /// Like `id_is_subtype` but for `RefType`s.
    ///
    /// Both `a` and `b` must be canonicalized already.
    pub fn reftype_is_subtype(&self, a: RefType, b: RefType) -> bool {
        // NB: Don't need `RecGroupId`s since we are calling from outside of the
        // rec group, and so any `PackedIndex`es we encounter have already been
        // canonicalized to `CoreTypeId`s directly.
        self.reftype_is_subtype_impl(a, None, b, None)
    }

    /// Implementation of `RefType` and `HeapType` subtyping.
    ///
    /// Panics if we need rec groups but aren't given them. Rec groups only need
    /// to be passed in when checking subtyping of `RefType`s that we encounter
    /// while validating a rec group itself.
    pub(crate) fn reftype_is_subtype_impl(
        &self,
        a: RefType,
        a_group: Option<RecGroupId>,
        b: RefType,
        b_group: Option<RecGroupId>,
    ) -> bool {
        if a == b && a_group == b_group {
            return true;
        }

        if a.is_nullable() && !b.is_nullable() {
            return false;
        }

        let core_type_id = |group: Option<RecGroupId>, index: UnpackedIndex| -> CoreTypeId {
            if let Some(id) = index.as_core_type_id() {
                id
            } else {
                self.at_canonicalized_unpacked_index(group.unwrap(), index, usize::MAX)
                    .expect("type references are checked during canonicalization")
            }
        };

        let subtype = |group, index| -> &SubType {
            let id = core_type_id(group, index);
            &self[id]
        };

        use HeapType as HT;
        match (a.heap_type(), b.heap_type()) {
            (a, b) if a == b => true,

            (HT::Eq | HT::I31 | HT::Struct | HT::Array | HT::None, HT::Any) => true,
            (HT::I31 | HT::Struct | HT::Array | HT::None, HT::Eq) => true,
            (HT::NoExtern, HT::Extern) => true,
            (HT::NoFunc, HT::Func) => true,
            (HT::None, HT::I31 | HT::Array | HT::Struct) => true,

            (HT::Concrete(a), HT::Eq | HT::Any) => matches!(
                subtype(a_group, a).composite_type,
                CompositeType::Array(_) | CompositeType::Struct(_)
            ),

            (HT::Concrete(a), HT::Struct) => {
                matches!(subtype(a_group, a).composite_type, CompositeType::Struct(_))
            }

            (HT::Concrete(a), HT::Array) => {
                matches!(subtype(a_group, a).composite_type, CompositeType::Array(_))
            }

            (HT::Concrete(a), HT::Func) => {
                matches!(subtype(a_group, a).composite_type, CompositeType::Func(_))
            }

            (HT::Concrete(a), HT::Concrete(b)) => {
                self.id_is_subtype(core_type_id(a_group, a), core_type_id(b_group, b))
            }

            (HT::None, HT::Concrete(b)) => matches!(
                subtype(b_group, b).composite_type,
                CompositeType::Array(_) | CompositeType::Struct(_)
            ),

            (HT::NoFunc, HT::Concrete(b)) => {
                matches!(subtype(b_group, b).composite_type, CompositeType::Func(_))
            }

            // Nothing else matches. (Avoid full wildcard matches so that
            // adding/modifying variants is easier in the future.)
            (HT::Concrete(_), _)
            | (HT::Func, _)
            | (HT::Extern, _)
            | (HT::Any, _)
            | (HT::None, _)
            | (HT::NoExtern, _)
            | (HT::NoFunc, _)
            | (HT::Eq, _)
            | (HT::Struct, _)
            | (HT::Array, _)
            | (HT::I31, _) => false,

            // TODO: this probably isn't right, this is probably related to some
            // gc type.
            (HT::Exn, _) => false,
        }
    }

    /// Like `id_is_subtype` but for `RefType`s.
    ///
    /// Both `a` and `b` must be canonicalized already.
    pub fn valtype_is_subtype(&self, a: ValType, b: ValType) -> bool {
        match (a, b) {
            (a, b) if a == b => true,
            (ValType::Ref(a), ValType::Ref(b)) => self.reftype_is_subtype(a, b),
            (ValType::Ref(_), _)
            | (ValType::I32, _)
            | (ValType::I64, _)
            | (ValType::F32, _)
            | (ValType::F64, _)
            | (ValType::V128, _) => false,
        }
    }

    /// Get the top type of the given heap type.
    ///
    /// Concrete types must have had their indices canonicalized to core type
    /// ids, otherwise this method will panic.
    pub fn top_type(&self, heap_type: &HeapType) -> HeapType {
        match *heap_type {
            HeapType::Concrete(idx) => match self[idx.as_core_type_id().unwrap()].composite_type {
                CompositeType::Func(_) => HeapType::Func,
                CompositeType::Array(_) | CompositeType::Struct(_) => HeapType::Any,
            },
            HeapType::Func | HeapType::NoFunc => HeapType::Func,
            HeapType::Extern | HeapType::NoExtern => HeapType::Extern,
            HeapType::Any
            | HeapType::Eq
            | HeapType::Struct
            | HeapType::Array
            | HeapType::I31
            | HeapType::None => HeapType::Any,
            HeapType::Exn => HeapType::Exn,
        }
    }

    fn checkpoint(&self) -> TypeListCheckpoint {
        let TypeList {
            alias_mappings: _,
            alias_counter: _,
            alias_snapshots: _,
            core_types,
            components,
            component_defined_types,
            component_values,
            component_instances,
            component_funcs,
            core_modules,
            core_instances,
            core_type_to_rec_group,
            core_type_to_supertype,
            rec_group_elements,
            canonical_rec_groups,
        } = self;

        TypeListCheckpoint {
            core_types: core_types.len(),
            components: components.len(),
            component_defined_types: component_defined_types.len(),
            component_values: component_values.len(),
            component_instances: component_instances.len(),
            component_funcs: component_funcs.len(),
            core_modules: core_modules.len(),
            core_instances: core_instances.len(),
            core_type_to_rec_group: core_type_to_rec_group.len(),
            core_type_to_supertype: core_type_to_supertype.len(),
            rec_group_elements: rec_group_elements.len(),
            canonical_rec_groups: canonical_rec_groups.as_ref().map(|m| m.len()).unwrap_or(0),
        }
    }

    fn reset_to_checkpoint(&mut self, checkpoint: TypeListCheckpoint) {
        let TypeList {
            alias_mappings: _,
            alias_counter: _,
            alias_snapshots: _,
            core_types,
            components,
            component_defined_types,
            component_values,
            component_instances,
            component_funcs,
            core_modules,
            core_instances,
            core_type_to_rec_group,
            core_type_to_supertype,
            rec_group_elements,
            canonical_rec_groups,
        } = self;

        core_types.truncate(checkpoint.core_types);
        components.truncate(checkpoint.components);
        component_defined_types.truncate(checkpoint.component_defined_types);
        component_values.truncate(checkpoint.component_values);
        component_instances.truncate(checkpoint.component_instances);
        component_funcs.truncate(checkpoint.component_funcs);
        core_modules.truncate(checkpoint.core_modules);
        core_instances.truncate(checkpoint.core_instances);
        core_type_to_rec_group.truncate(checkpoint.core_type_to_rec_group);
        core_type_to_supertype.truncate(checkpoint.core_type_to_supertype);
        rec_group_elements.truncate(checkpoint.rec_group_elements);

        if let Some(canonical_rec_groups) = canonical_rec_groups {
            assert_eq!(
                canonical_rec_groups.len(),
                checkpoint.canonical_rec_groups,
                "checkpointing does not support resetting `canonical_rec_groups` (it would require a \
                 proper immutable and persistent hash map) so adding new groups is disallowed"
            );
        }
    }

    pub fn commit(&mut self) -> TypeList {
        // Note that the `alias_counter` is bumped here to ensure that the
        // previous value of the unique counter is never used for an actual type
        // so it's suitable for lookup via a binary search.
        let alias_counter = self.alias_counter;
        self.alias_counter += 1;

        self.alias_snapshots.push(TypeListAliasSnapshot {
            alias_counter,
            alias_mappings: mem::take(&mut self.alias_mappings),
        });

        TypeList {
            alias_mappings: HashMap::new(),
            alias_counter: self.alias_counter,
            alias_snapshots: self.alias_snapshots.clone(),
            core_types: self.core_types.commit(),
            components: self.components.commit(),
            component_defined_types: self.component_defined_types.commit(),
            component_values: self.component_values.commit(),
            component_instances: self.component_instances.commit(),
            component_funcs: self.component_funcs.commit(),
            core_modules: self.core_modules.commit(),
            core_instances: self.core_instances.commit(),
            core_type_to_rec_group: self.core_type_to_rec_group.commit(),
            core_type_to_supertype: self.core_type_to_supertype.commit(),
            rec_group_elements: self.rec_group_elements.commit(),
            canonical_rec_groups: None,
        }
    }

    /// See `SnapshotList::with_unique`.
    pub fn with_unique<T>(&mut self, mut ty: T) -> T
    where
        T: Aliasable,
    {
        self.alias_mappings
            .insert(self.alias_counter, ty.alias_id());
        ty.set_alias_id(self.alias_counter);
        self.alias_counter += 1;
        ty
    }

    /// Attempts to lookup the type id that `ty` is an alias of.
    ///
    /// Returns `None` if `ty` wasn't listed as aliasing a prior type.
    pub fn peel_alias<T>(&self, mut ty: T) -> Option<T>
    where
        T: Aliasable,
    {
        let alias_id = ty.alias_id();

        // The unique counter in each snapshot is the unique counter at the
        // time of the snapshot so it's guaranteed to never be used, meaning
        // that `Ok` should never show up here. With an `Err` it's where the
        // index would be placed meaning that the index in question is the
        // smallest value over the unique id's value, meaning that slot has the
        // mapping we're interested in.
        let i = match self
            .alias_snapshots
            .binary_search_by_key(&alias_id, |snapshot| snapshot.alias_counter)
        {
            Ok(_) => unreachable!(),
            Err(i) => i,
        };

        // If the `i` index is beyond the snapshot array then lookup in the
        // current mappings instead since it may refer to a type not snapshot
        // yet.
        ty.set_alias_id(match self.alias_snapshots.get(i) {
            Some(snapshot) => *snapshot.alias_mappings.get(&alias_id)?,
            None => *self.alias_mappings.get(&alias_id)?,
        });
        Some(ty)
    }
}

impl<T> Index<T> for TypeList
where
    T: TypeIdentifier,
{
    type Output = T::Data;

    fn index(&self, id: T) -> &Self::Output {
        let arena = T::list(self);
        &arena[id.index()]
    }
}

/// Thin wrapper around `TypeList` which provides an allocator of unique ids for
/// types contained within this list.
pub(crate) struct TypeAlloc {
    list: TypeList,

    // This is assigned at creation of a `TypeAlloc` and then never changed.
    // It's used in one entry for all `ResourceId`s contained within.
    globally_unique_id: usize,

    // This is a counter that's incremeneted each time `alloc_resource_id` is
    // called.
    next_resource_id: u32,
}

impl Default for TypeAlloc {
    fn default() -> TypeAlloc {
        static NEXT_GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);
        let mut ret = TypeAlloc {
            list: TypeList::default(),
            globally_unique_id: {
                let id = NEXT_GLOBAL_ID.fetch_add(1, Ordering::Relaxed);
                if id > usize::MAX - 10_000 {
                    NEXT_GLOBAL_ID.store(usize::MAX - 10_000, Ordering::Relaxed);
                    panic!("overflow on the global id counter");
                }
                id
            },
            next_resource_id: 0,
        };
        ret.list.canonical_rec_groups = Some(Default::default());
        ret
    }
}

impl Deref for TypeAlloc {
    type Target = TypeList;
    fn deref(&self) -> &TypeList {
        &self.list
    }
}

impl DerefMut for TypeAlloc {
    fn deref_mut(&mut self) -> &mut TypeList {
        &mut self.list
    }
}

impl TypeAlloc {
    /// Pushes a new type into this list, returning an identifier which can be
    /// used to later retrieve it.
    ///
    /// The returned identifier is unique within this `TypeAlloc` and won't be
    /// hash-equivalent to anything else.
    pub fn push_ty<T>(&mut self, ty: T) -> T::Id
    where
        T: TypeData,
    {
        self.list.push(ty)
    }

    /// Allocates a new unique resource identifier.
    ///
    /// Note that uniqueness is only a property within this `TypeAlloc`.
    pub fn alloc_resource_id(&mut self) -> AliasableResourceId {
        let contextually_unique_id = self.next_resource_id;
        self.next_resource_id = self.next_resource_id.checked_add(1).unwrap();
        AliasableResourceId {
            id: ResourceId {
                globally_unique_id: self.globally_unique_id,
                contextually_unique_id,
            },
            alias_id: NO_ALIAS,
        }
    }

    /// Adds the set of "free variables" of the `id` provided to the `set`
    /// provided.
    ///
    /// Free variables are defined as resources. Any resource, perhaps
    /// transitively, referred to but not defined by `id` is added to the `set`
    /// and returned.
    pub fn free_variables_any_type_id(
        &self,
        id: ComponentAnyTypeId,
        set: &mut IndexSet<ResourceId>,
    ) {
        match id {
            ComponentAnyTypeId::Resource(r) => {
                set.insert(r.resource());
            }
            ComponentAnyTypeId::Defined(id) => {
                self.free_variables_component_defined_type_id(id, set)
            }
            ComponentAnyTypeId::Func(id) => self.free_variables_component_func_type_id(id, set),
            ComponentAnyTypeId::Instance(id) => {
                self.free_variables_component_instance_type_id(id, set)
            }
            ComponentAnyTypeId::Component(id) => self.free_variables_component_type_id(id, set),
        }
    }

    pub fn free_variables_component_defined_type_id(
        &self,
        id: ComponentDefinedTypeId,
        set: &mut IndexSet<ResourceId>,
    ) {
        match &self[id] {
            ComponentDefinedType::Primitive(_)
            | ComponentDefinedType::Flags(_)
            | ComponentDefinedType::Enum(_) => {}
            ComponentDefinedType::Record(r) => {
                for ty in r.fields.values() {
                    self.free_variables_valtype(ty, set);
                }
            }
            ComponentDefinedType::Tuple(r) => {
                for ty in r.types.iter() {
                    self.free_variables_valtype(ty, set);
                }
            }
            ComponentDefinedType::Variant(r) => {
                for ty in r.cases.values() {
                    if let Some(ty) = &ty.ty {
                        self.free_variables_valtype(ty, set);
                    }
                }
            }
            ComponentDefinedType::List(ty) | ComponentDefinedType::Option(ty) => {
                self.free_variables_valtype(ty, set);
            }
            ComponentDefinedType::Result { ok, err } => {
                if let Some(ok) = ok {
                    self.free_variables_valtype(ok, set);
                }
                if let Some(err) = err {
                    self.free_variables_valtype(err, set);
                }
            }
            ComponentDefinedType::Own(id) | ComponentDefinedType::Borrow(id) => {
                set.insert(id.resource());
            }
        }
    }

    pub fn free_variables_component_type_id(
        &self,
        id: ComponentTypeId,
        set: &mut IndexSet<ResourceId>,
    ) {
        let i = &self[id];
        // Recurse on the imports/exports of components, but remove the
        // imported and defined resources within the component itself.
        //
        // Technically this needs to add all the free variables of the
        // exports, remove the defined resources, then add the free
        // variables of imports, then remove the imported resources. Given
        // prior validation of component types, however, the defined
        // and imported resources are disjoint and imports can't refer to
        // defined resources, so doing this all in one go should be
        // equivalent.
        for ty in i.imports.values().chain(i.exports.values()) {
            self.free_variables_component_entity(ty, set);
        }
        for (id, _path) in i.imported_resources.iter().chain(&i.defined_resources) {
            set.remove(id);
        }
    }

    pub fn free_variables_component_instance_type_id(
        &self,
        id: ComponentInstanceTypeId,
        set: &mut IndexSet<ResourceId>,
    ) {
        let i = &self[id];
        // Like components, add in all the free variables of referenced
        // types but then remove those defined by this component instance
        // itself.
        for ty in i.exports.values() {
            self.free_variables_component_entity(ty, set);
        }
        for id in i.defined_resources.iter() {
            set.remove(id);
        }
    }

    pub fn free_variables_component_func_type_id(
        &self,
        id: ComponentFuncTypeId,
        set: &mut IndexSet<ResourceId>,
    ) {
        let i = &self[id];
        for ty in i
            .params
            .iter()
            .map(|(_, ty)| ty)
            .chain(i.results.iter().map(|(_, ty)| ty))
        {
            self.free_variables_valtype(ty, set);
        }
    }

    /// Same as `free_variables_type_id`, but for `ComponentEntityType`.
    pub fn free_variables_component_entity(
        &self,
        ty: &ComponentEntityType,
        set: &mut IndexSet<ResourceId>,
    ) {
        match ty {
            ComponentEntityType::Module(_) => {}
            ComponentEntityType::Func(id) => self.free_variables_component_func_type_id(*id, set),
            ComponentEntityType::Instance(id) => {
                self.free_variables_component_instance_type_id(*id, set)
            }
            ComponentEntityType::Component(id) => self.free_variables_component_type_id(*id, set),
            ComponentEntityType::Type { created, .. } => {
                self.free_variables_any_type_id(*created, set);
            }
            ComponentEntityType::Value(ty) => self.free_variables_valtype(ty, set),
        }
    }

    /// Same as `free_variables_type_id`, but for `ComponentValType`.
    fn free_variables_valtype(&self, ty: &ComponentValType, set: &mut IndexSet<ResourceId>) {
        match ty {
            ComponentValType::Primitive(_) => {}
            ComponentValType::Type(id) => self.free_variables_component_defined_type_id(*id, set),
        }
    }

    /// Returns whether the type `id` is "named" where named types are presented
    /// via the provided `set`.
    ///
    /// This requires that `id` is a `Defined` type.
    pub(crate) fn type_named_type_id(
        &self,
        id: ComponentDefinedTypeId,
        set: &HashSet<ComponentAnyTypeId>,
    ) -> bool {
        let ty = &self[id];
        match ty {
            // Primitives are always considered named
            ComponentDefinedType::Primitive(_) => true,

            // These structures are never allowed to be anonymous, so they
            // themselves must be named.
            ComponentDefinedType::Flags(_)
            | ComponentDefinedType::Enum(_)
            | ComponentDefinedType::Record(_)
            | ComponentDefinedType::Variant(_) => set.contains(&id.into()),

            // All types below here are allowed to be anonymous, but their
            // own components must be appropriately named.
            ComponentDefinedType::Tuple(r) => {
                r.types.iter().all(|t| self.type_named_valtype(t, set))
            }
            ComponentDefinedType::Result { ok, err } => {
                ok.as_ref()
                    .map(|t| self.type_named_valtype(t, set))
                    .unwrap_or(true)
                    && err
                        .as_ref()
                        .map(|t| self.type_named_valtype(t, set))
                        .unwrap_or(true)
            }
            ComponentDefinedType::List(ty) | ComponentDefinedType::Option(ty) => {
                self.type_named_valtype(ty, set)
            }

            // own/borrow themselves don't have to be named, but the resource
            // they refer to must be named.
            ComponentDefinedType::Own(id) | ComponentDefinedType::Borrow(id) => {
                set.contains(&(*id).into())
            }
        }
    }

    pub(crate) fn type_named_valtype(
        &self,
        ty: &ComponentValType,
        set: &HashSet<ComponentAnyTypeId>,
    ) -> bool {
        match ty {
            ComponentValType::Primitive(_) => true,
            ComponentValType::Type(id) => self.type_named_type_id(*id, set),
        }
    }
}

/// A helper trait to provide the functionality necessary to resources within a
/// type.
///
/// This currently exists to abstract over `TypeAlloc` and `SubtypeArena` which
/// both need to perform remapping operations.
pub trait Remap
where
    Self: Index<ComponentTypeId, Output = ComponentType>,
    Self: Index<ComponentDefinedTypeId, Output = ComponentDefinedType>,
    Self: Index<ComponentInstanceTypeId, Output = ComponentInstanceType>,
    Self: Index<ComponentFuncTypeId, Output = ComponentFuncType>,
{
    /// Pushes a new anonymous type within this object, returning an identifier
    /// which can be used to refer to it.
    fn push_ty<T>(&mut self, ty: T) -> T::Id
    where
        T: TypeData;

    /// Apply `map` to the keys of `tmp`, setting `*any_changed = true` if any
    /// keys were remapped.
    fn map_map(
        tmp: &mut IndexMap<ResourceId, Vec<usize>>,
        any_changed: &mut bool,
        map: &Remapping,
    ) {
        for (id, path) in mem::take(tmp) {
            let id = match map.resources.get(&id) {
                Some(id) => {
                    *any_changed = true;
                    *id
                }
                None => id,
            };
            tmp.insert(id, path);
        }
    }

    /// If `any_changed` is true, push `ty`, update `map` to point `id` to the
    /// new type ID, set `id` equal to the new type ID, and return `true`.
    /// Otherwise, update `map` to point `id` to itself and return `false`.
    fn insert_if_any_changed<T>(
        &mut self,
        map: &mut Remapping,
        any_changed: bool,
        id: &mut T::Id,
        ty: T,
    ) -> bool
    where
        T: TypeData,
        T::Id: Into<ComponentAnyTypeId>,
    {
        let new = if any_changed { self.push_ty(ty) } else { *id };
        map.types.insert((*id).into(), new.into());
        let changed = *id != new;
        *id = new;
        changed
    }

    /// Recursively search for any resource types reachable from `id`, updating
    /// it and `map` if any are found and remapped, returning `true` iff at last
    /// one is remapped.
    fn remap_component_any_type_id(
        &mut self,
        id: &mut ComponentAnyTypeId,
        map: &mut Remapping,
    ) -> bool {
        match id {
            ComponentAnyTypeId::Resource(id) => self.remap_resource_id(id, map),
            ComponentAnyTypeId::Defined(id) => self.remap_component_defined_type_id(id, map),
            ComponentAnyTypeId::Func(id) => self.remap_component_func_type_id(id, map),
            ComponentAnyTypeId::Instance(id) => self.remap_component_instance_type_id(id, map),
            ComponentAnyTypeId::Component(id) => self.remap_component_type_id(id, map),
        }
    }

    /// If `map` indicates `id` should be remapped, update it and return `true`.
    /// Otherwise, do nothing and return `false`.
    fn remap_resource_id(&mut self, id: &mut AliasableResourceId, map: &Remapping) -> bool {
        if let Some(changed) = map.remap_id(id) {
            return changed;
        }

        match map.resources.get(&id.resource()) {
            None => false,
            Some(new_id) => {
                *id.resource_mut() = *new_id;
                true
            }
        }
    }

    /// Recursively search for any resource types reachable from `id`, updating
    /// it and `map` if any are found and remapped, returning `true` iff at last
    /// one is remapped.
    fn remap_component_type_id(&mut self, id: &mut ComponentTypeId, map: &mut Remapping) -> bool {
        if let Some(changed) = map.remap_id(id) {
            return changed;
        }

        let mut any_changed = false;
        let mut ty = self[*id].clone();
        for ty in ty.imports.values_mut().chain(ty.exports.values_mut()) {
            any_changed |= self.remap_component_entity(ty, map);
        }
        for (id, _) in ty
            .imported_resources
            .iter_mut()
            .chain(&mut ty.defined_resources)
        {
            if let Some(new) = map.resources.get(id) {
                *id = *new;
                any_changed = true;
            }
        }
        Self::map_map(&mut ty.explicit_resources, &mut any_changed, map);
        self.insert_if_any_changed(map, any_changed, id, ty)
    }

    /// Recursively search for any resource types reachable from `id`, updating
    /// it and `map` if any are found and remapped, returning `true` iff at last
    /// one is remapped.
    fn remap_component_defined_type_id(
        &mut self,
        id: &mut ComponentDefinedTypeId,
        map: &mut Remapping,
    ) -> bool {
        if let Some(changed) = map.remap_id(id) {
            return changed;
        }

        let mut any_changed = false;
        let mut tmp = self[*id].clone();
        match &mut tmp {
            ComponentDefinedType::Primitive(_)
            | ComponentDefinedType::Flags(_)
            | ComponentDefinedType::Enum(_) => {}
            ComponentDefinedType::Record(r) => {
                for ty in r.fields.values_mut() {
                    any_changed |= self.remap_valtype(ty, map);
                }
            }
            ComponentDefinedType::Tuple(r) => {
                for ty in r.types.iter_mut() {
                    any_changed |= self.remap_valtype(ty, map);
                }
            }
            ComponentDefinedType::Variant(r) => {
                for ty in r.cases.values_mut() {
                    if let Some(ty) = &mut ty.ty {
                        any_changed |= self.remap_valtype(ty, map);
                    }
                }
            }
            ComponentDefinedType::List(ty) | ComponentDefinedType::Option(ty) => {
                any_changed |= self.remap_valtype(ty, map);
            }
            ComponentDefinedType::Result { ok, err } => {
                if let Some(ok) = ok {
                    any_changed |= self.remap_valtype(ok, map);
                }
                if let Some(err) = err {
                    any_changed |= self.remap_valtype(err, map);
                }
            }
            ComponentDefinedType::Own(id) | ComponentDefinedType::Borrow(id) => {
                any_changed |= self.remap_resource_id(id, map);
            }
        }
        self.insert_if_any_changed(map, any_changed, id, tmp)
    }

    /// Recursively search for any resource types reachable from `id`, updating
    /// it and `map` if any are found and remapped, returning `true` iff at last
    /// one is remapped.
    fn remap_component_instance_type_id(
        &mut self,
        id: &mut ComponentInstanceTypeId,
        map: &mut Remapping,
    ) -> bool {
        if let Some(changed) = map.remap_id(id) {
            return changed;
        }

        let mut any_changed = false;
        let mut tmp = self[*id].clone();
        for ty in tmp.exports.values_mut() {
            any_changed |= self.remap_component_entity(ty, map);
        }
        for id in tmp.defined_resources.iter_mut() {
            if let Some(new) = map.resources.get(id) {
                *id = *new;
                any_changed = true;
            }
        }
        Self::map_map(&mut tmp.explicit_resources, &mut any_changed, map);
        self.insert_if_any_changed(map, any_changed, id, tmp)
    }

    /// Recursively search for any resource types reachable from `id`, updating
    /// it and `map` if any are found and remapped, returning `true` iff at last
    /// one is remapped.
    fn remap_component_func_type_id(
        &mut self,
        id: &mut ComponentFuncTypeId,
        map: &mut Remapping,
    ) -> bool {
        if let Some(changed) = map.remap_id(id) {
            return changed;
        }

        let mut any_changed = false;
        let mut tmp = self[*id].clone();
        for ty in tmp
            .params
            .iter_mut()
            .map(|(_, ty)| ty)
            .chain(tmp.results.iter_mut().map(|(_, ty)| ty))
        {
            any_changed |= self.remap_valtype(ty, map);
        }
        self.insert_if_any_changed(map, any_changed, id, tmp)
    }

    /// Same as `remap_type_id`, but works with `ComponentEntityType`.
    fn remap_component_entity(
        &mut self,
        ty: &mut ComponentEntityType,
        map: &mut Remapping,
    ) -> bool {
        match ty {
            ComponentEntityType::Module(_) => {
                // Can't reference resources.
                false
            }
            ComponentEntityType::Func(id) => self.remap_component_func_type_id(id, map),
            ComponentEntityType::Instance(id) => self.remap_component_instance_type_id(id, map),
            ComponentEntityType::Component(id) => self.remap_component_type_id(id, map),
            ComponentEntityType::Type {
                referenced,
                created,
            } => {
                let mut changed = self.remap_component_any_type_id(referenced, map);
                if *referenced == *created {
                    *created = *referenced;
                } else {
                    changed |= self.remap_component_any_type_id(created, map);
                }
                changed
            }
            ComponentEntityType::Value(ty) => self.remap_valtype(ty, map),
        }
    }

    /// Same as `remap_type_id`, but works with `ComponentValType`.
    fn remap_valtype(&mut self, ty: &mut ComponentValType, map: &mut Remapping) -> bool {
        match ty {
            ComponentValType::Primitive(_) => false,
            ComponentValType::Type(id) => self.remap_component_defined_type_id(id, map),
        }
    }
}

/// Utility for mapping equivalent `ResourceId`s to each other and (when paired with the `Remap` trait)
/// non-destructively edit type lists to reflect those mappings.
#[derive(Debug, Default)]
pub struct Remapping {
    /// A mapping from old resource ID to new resource ID.
    pub(crate) resources: HashMap<ResourceId, ResourceId>,

    /// A mapping filled in during the remapping process which records how a
    /// type was remapped, if applicable. This avoids remapping multiple
    /// references to the same type and instead only processing it once.
    types: HashMap<ComponentAnyTypeId, ComponentAnyTypeId>,
}

impl Remap for TypeAlloc {
    fn push_ty<T>(&mut self, ty: T) -> T::Id
    where
        T: TypeData,
    {
        <TypeList>::push(self, ty)
    }
}

impl<T> Index<T> for TypeAlloc
where
    T: TypeIdentifier,
{
    type Output = T::Data;

    #[inline]
    fn index(&self, id: T) -> &T::Data {
        &self.list[id]
    }
}

impl Remapping {
    /// Add a mapping from the specified old resource ID to the new resource ID
    pub fn add(&mut self, old: ResourceId, new: ResourceId) {
        self.resources.insert(old, new);
    }

    /// Clear the type cache while leaving the resource mappings intact.
    pub fn reset_type_cache(&mut self) {
        self.types.clear()
    }

    fn remap_id<T>(&self, id: &mut T) -> Option<bool>
    where
        T: Copy + Into<ComponentAnyTypeId> + TryFrom<ComponentAnyTypeId>,
        T::Error: std::fmt::Debug,
    {
        let old: ComponentAnyTypeId = (*id).into();
        let new = self.types.get(&old)?;
        if *new == old {
            Some(false)
        } else {
            *id = T::try_from(*new).expect("should never remap across different kinds");
            Some(true)
        }
    }
}

/// Helper structure used to perform subtyping computations.
///
/// This type is used whenever a subtype needs to be tested in one direction or
/// the other. The methods of this type are the various entry points for
/// subtyping.
///
/// Internally this contains arenas for two lists of types. The `a` arena is
/// intended to be used for lookup of the first argument to all of the methods
/// below, and the `b` arena is used for lookup of the second argument.
///
/// Arenas here are used specifically for component-based subtyping queries. In
/// these situations new types must be created based on substitution mappings,
/// but the types all have temporary lifetimes. Everything in these arenas is
/// thrown away once the subtyping computation has finished.
///
/// Note that this subtyping context also explicitly supports being created
/// from to different lists `a` and `b` originally, for testing subtyping
/// between two different components for example.
pub struct SubtypeCx<'a> {
    /// Lookup arena for first type argument
    pub a: SubtypeArena<'a>,
    /// Lookup arena for second type argument
    pub b: SubtypeArena<'a>,
}

impl<'a> SubtypeCx<'a> {
    /// Create a new instance with the specified type lists
    pub fn new_with_refs(a: TypesRef<'a>, b: TypesRef<'a>) -> SubtypeCx<'a> {
        Self::new(a.list, b.list)
    }

    pub(crate) fn new(a: &'a TypeList, b: &'a TypeList) -> SubtypeCx<'a> {
        SubtypeCx {
            a: SubtypeArena::new(a),
            b: SubtypeArena::new(b),
        }
    }

    /// Swap the type lists
    pub fn swap(&mut self) {
        mem::swap(&mut self.a, &mut self.b);
    }

    /// Executes the closure `f`, resetting the internal arenas to their
    /// original size after the closure finishes.
    ///
    /// This enables `f` to modify the internal arenas while relying on all
    /// changes being discarded after the closure finishes.
    fn with_checkpoint<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        let a = self.a.list.checkpoint();
        let b = self.b.list.checkpoint();
        let result = f(self);
        self.a.list.reset_to_checkpoint(a);
        self.b.list.reset_to_checkpoint(b);
        result
    }

    /// Tests whether `a` is a subtype of `b`.
    ///
    /// Errors are reported at the `offset` specified.
    pub fn component_entity_type(
        &mut self,
        a: &ComponentEntityType,
        b: &ComponentEntityType,
        offset: usize,
    ) -> Result<()> {
        use ComponentEntityType::*;

        match (a, b) {
            (Module(a), Module(b)) => self.module_type(*a, *b, offset),
            (Module(_), b) => bail!(offset, "expected {}, found module", b.desc()),

            (Func(a), Func(b)) => self.component_func_type(*a, *b, offset),
            (Func(_), b) => bail!(offset, "expected {}, found func", b.desc()),

            (Value(a), Value(b)) => self.component_val_type(a, b, offset),
            (Value(_), b) => bail!(offset, "expected {}, found value", b.desc()),

            (Type { referenced: a, .. }, Type { referenced: b, .. }) => {
                self.component_any_type_id(*a, *b, offset)
            }
            (Type { .. }, b) => bail!(offset, "expected {}, found type", b.desc()),

            (Instance(a), Instance(b)) => self.component_instance_type(*a, *b, offset),
            (Instance(_), b) => bail!(offset, "expected {}, found instance", b.desc()),

            (Component(a), Component(b)) => self.component_type(*a, *b, offset),
            (Component(_), b) => bail!(offset, "expected {}, found component", b.desc()),
        }
    }

    /// Tests whether `a` is a subtype of `b`.
    ///
    /// Errors are reported at the `offset` specified.
    pub fn component_type(
        &mut self,
        a: ComponentTypeId,
        b: ComponentTypeId,
        offset: usize,
    ) -> Result<()> {
        // Components are ... tricky. They follow the same basic
        // structure as core wasm modules, but they also have extra
        // logic to handle resource types. Resources are effectively
        // abstract types so this is sort of where an ML module system
        // in the component model becomes a reality.
        //
        // This also leverages the `open_instance_type` method below
        // heavily which internally has its own quite large suite of
        // logic. More-or-less what's happening here is:
        //
        // 1. Pretend that the imports of B are given as values to the
        //    imports of A. If A didn't import anything, for example,
        //    that's great and the subtyping definitely passes there.
        //    This operation produces a mapping of all the resources of
        //    A's imports to resources in B's imports.
        //
        // 2. This mapping is applied to all of A's exports. This means
        //    that all exports of A referring to A's imported resources
        //    now instead refer to B's. Note, though that A's exports
        //    still refer to its own defined resources.
        //
        // 3. The same `open_instance_type` method used during the
        //    first step is used again, but this time on the exports
        //    in the reverse direction. This performs a similar
        //    operation, though, by creating a mapping from B's
        //    defined resources to A's defined resources. The map
        //    itself is discarded as it's not needed.
        //
        // The order that everything passed here is intentional, but
        // also subtle. I personally think of it as
        // `open_instance_type` takes a list of things to satisfy a
        // signature and produces a mapping of resources in the
        // signature to those provided in the list of things. The
        // order of operations then goes:
        //
        // * Someone thinks they have a component of type B, but they
        //   actually have a component of type A (e.g. due to this
        //   subtype check passing).
        // * This person provides the imports of B and that must be
        //   sufficient to satisfy the imports of A. This is the first
        //   `open_instance_type` check.
        // * Now though the resources provided by B are substituted
        //   into A's exports since that's what was provided.
        // * A's exports are then handed back to the original person,
        //   and these exports must satisfy the signature required by B
        //   since that's what they're expecting.
        // * This is the second `open_instance_type` which, to get
        //   resource types to line up, will map from A's defined
        //   resources to B's defined resources.
        //
        // If all that passes then the resources should all line up
        // perfectly. Any misalignment is reported as a subtyping
        // error.
        let b_imports = self.b[b]
            .imports
            .iter()
            .map(|(name, ty)| (name.clone(), ty.clone()))
            .collect();
        self.swap();
        let mut import_mapping =
            self.open_instance_type(&b_imports, a, ExternKind::Import, offset)?;
        self.swap();
        self.with_checkpoint(|this| {
            let mut a_exports = this.a[a]
                .exports
                .iter()
                .map(|(name, ty)| (name.clone(), ty.clone()))
                .collect::<IndexMap<_, _>>();
            for ty in a_exports.values_mut() {
                this.a.remap_component_entity(ty, &mut import_mapping);
            }
            this.open_instance_type(&a_exports, b, ExternKind::Export, offset)?;
            Ok(())
        })
    }

    /// Tests whether `a` is a subtype of `b`.
    ///
    /// Errors are reported at the `offset` specified.
    pub fn component_instance_type(
        &mut self,
        a_id: ComponentInstanceTypeId,
        b_id: ComponentInstanceTypeId,
        offset: usize,
    ) -> Result<()> {
        // For instance type subtyping, all exports in the other
        // instance type must be present in this instance type's
        // exports (i.e. it can export *more* than what this instance
        // type needs).
        let a = &self.a[a_id];
        let b = &self.b[b_id];

        let mut exports = Vec::with_capacity(b.exports.len());
        for (k, b) in b.exports.iter() {
            match a.exports.get(k) {
                Some(a) => exports.push((*a, *b)),
                None => bail!(offset, "missing expected export `{k}`"),
            }
        }
        for (i, (a, b)) in exports.iter().enumerate() {
            let err = match self.component_entity_type(a, b, offset) {
                Ok(()) => continue,
                Err(e) => e,
            };
            // On failure attach the name of this export as context to
            // the error message to leave a breadcrumb trail.
            let (name, _) = self.b[b_id].exports.get_index(i).unwrap();
            return Err(err.with_context(|| format!("type mismatch in instance export `{name}`")));
        }
        Ok(())
    }

    /// Tests whether `a` is a subtype of `b`.
    ///
    /// Errors are reported at the `offset` specified.
    pub fn component_func_type(
        &mut self,
        a: ComponentFuncTypeId,
        b: ComponentFuncTypeId,
        offset: usize,
    ) -> Result<()> {
        let a = &self.a[a];
        let b = &self.b[b];

        // Note that this intentionally diverges from the upstream
        // specification in terms of subtyping. This is a full
        // type-equality check which ensures that the structure of `a`
        // exactly matches the structure of `b`. The rationale for this
        // is:
        //
        // * Primarily in Wasmtime subtyping based on function types is
        //   not implemented. This includes both subtyping a host
        //   import and additionally handling subtyping as functions
        //   cross component boundaries. The host import subtyping (or
        //   component export subtyping) is not clear how to handle at
        //   all at this time. The subtyping of functions between
        //   components can more easily be handled by extending the
        //   `fact` compiler, but that hasn't been done yet.
        //
        // * The upstream specification is currently pretty
        //   intentionally vague precisely what subtyping is allowed.
        //   Implementing a strict check here is intended to be a
        //   conservative starting point for the component model which
        //   can be extended in the future if necessary.
        //
        // * The interaction with subtyping on bindings generation, for
        //   example, is a tricky problem that doesn't have a clear
        //   answer at this time.  Effectively this is more rationale
        //   for being conservative in the first pass of the component
        //   model.
        //
        // So, in conclusion, the test here (and other places that
        // reference this comment) is for exact type equality with no
        // differences.
        if a.params.len() != b.params.len() {
            bail!(
                offset,
                "expected {} parameters, found {}",
                b.params.len(),
                a.params.len(),
            );
        }
        if a.results.len() != b.results.len() {
            bail!(
                offset,
                "expected {} results, found {}",
                b.results.len(),
                a.results.len(),
            );
        }
        for ((an, a), (bn, b)) in a.params.iter().zip(b.params.iter()) {
            if an != bn {
                bail!(offset, "expected parameter named `{bn}`, found `{an}`");
            }
            self.component_val_type(a, b, offset)
                .with_context(|| format!("type mismatch in function parameter `{an}`"))?;
        }
        for ((an, a), (bn, b)) in a.results.iter().zip(b.results.iter()) {
            if an != bn {
                bail!(offset, "mismatched result names");
            }
            self.component_val_type(a, b, offset)
                .with_context(|| "type mismatch with result type")?;
        }
        Ok(())
    }

    /// Tests whether `a` is a subtype of `b`.
    ///
    /// Errors are reported at the `offset` specified.
    pub fn module_type(
        &mut self,
        a: ComponentCoreModuleTypeId,
        b: ComponentCoreModuleTypeId,
        offset: usize,
    ) -> Result<()> {
        // For module type subtyping, all exports in the other module
        // type must be present in this module type's exports (i.e. it
        // can export *more* than what this module type needs).
        // However, for imports, the check is reversed (i.e. it is okay
        // to import *less* than what this module type needs).
        self.swap();
        let a_imports = &self.b[a].imports;
        let b_imports = &self.a[b].imports;
        for (k, a) in a_imports {
            match b_imports.get(k) {
                Some(b) => self
                    .entity_type(b, a, offset)
                    .with_context(|| format!("type mismatch in import `{}::{}`", k.0, k.1))?,
                None => bail!(offset, "missing expected import `{}::{}`", k.0, k.1),
            }
        }
        self.swap();
        let a = &self.a[a];
        let b = &self.b[b];
        for (k, b) in b.exports.iter() {
            match a.exports.get(k) {
                Some(a) => self
                    .entity_type(a, b, offset)
                    .with_context(|| format!("type mismatch in export `{k}`"))?,
                None => bail!(offset, "missing expected export `{k}`"),
            }
        }
        Ok(())
    }

    /// Tests whether `a` is a subtype of `b`.
    ///
    /// Errors are reported at the `offset` specified.
    pub fn component_any_type_id(
        &mut self,
        a: ComponentAnyTypeId,
        b: ComponentAnyTypeId,
        offset: usize,
    ) -> Result<()> {
        match (a, b) {
            (ComponentAnyTypeId::Resource(a), ComponentAnyTypeId::Resource(b)) => {
                if a.resource() == b.resource() {
                    Ok(())
                } else {
                    bail!(
                        offset,
                        "resource types are not the same ({:?} vs. {:?})",
                        a.resource(),
                        b.resource()
                    )
                }
            }
            (ComponentAnyTypeId::Resource(_), b) => {
                bail!(offset, "expected {}, found resource", b.desc())
            }
            (ComponentAnyTypeId::Defined(a), ComponentAnyTypeId::Defined(b)) => {
                self.component_defined_type(a, b, offset)
            }
            (ComponentAnyTypeId::Defined(_), b) => {
                bail!(offset, "expected {}, found defined type", b.desc())
            }

            (ComponentAnyTypeId::Func(a), ComponentAnyTypeId::Func(b)) => {
                self.component_func_type(a, b, offset)
            }
            (ComponentAnyTypeId::Func(_), b) => {
                bail!(offset, "expected {}, found func type", b.desc())
            }

            (ComponentAnyTypeId::Instance(a), ComponentAnyTypeId::Instance(b)) => {
                self.component_instance_type(a, b, offset)
            }
            (ComponentAnyTypeId::Instance(_), b) => {
                bail!(offset, "expected {}, found instance type", b.desc())
            }

            (ComponentAnyTypeId::Component(a), ComponentAnyTypeId::Component(b)) => {
                self.component_type(a, b, offset)
            }
            (ComponentAnyTypeId::Component(_), b) => {
                bail!(offset, "expected {}, found component type", b.desc())
            }
        }
    }

    /// The building block for subtyping checks when components are
    /// instantiated and when components are tested if they're subtypes of each
    /// other.
    ///
    /// This method takes a number of arguments:
    ///
    /// * `a` - this is a list of typed items which can be thought of as
    ///   concrete values to test against `b`.
    /// * `b` - this `TypeId` must point to `Type::Component`.
    /// * `kind` - indicates whether the `imports` or `exports` of `b` are
    ///   being tested against for the values in `a`.
    /// * `offset` - the binary offset at which to report errors if one happens.
    ///
    /// This will attempt to determine if the items in `a` satisfy the
    /// signature required by the `kind` items of `b`. For example component
    /// instantiation will have `a` as the list of arguments provided to
    /// instantiation, `b` is the component being instantiated, and `kind` is
    /// `ExternKind::Import`.
    ///
    /// This function, if successful, will return a mapping of the resources in
    /// `b` to the resources in `a` provided. This mapping is guaranteed to
    /// contain all the resources for `b` (all imported resources for
    /// `ExternKind::Import` or all defined resources for `ExternKind::Export`).
    pub fn open_instance_type(
        &mut self,
        a: &IndexMap<String, ComponentEntityType>,
        b: ComponentTypeId,
        kind: ExternKind,
        offset: usize,
    ) -> Result<Remapping> {
        // First, determine the mapping from resources in `b` to those supplied
        // by arguments in `a`.
        //
        // This loop will iterate over all the appropriate resources in `b`
        // and find the corresponding resource in `args`. The exact lists
        // in use here depend on the `kind` provided. This necessarily requires
        // a sequence of string lookups to find the corresponding items in each
        // list.
        //
        // The path to each resource in `resources` is precomputed as a list of
        // indexes. The first index is into `b`'s list of `entities`, and gives
        // the name that `b` assigns to the resource.  Each subsequent index,
        // if present, means that this resource was present through a layer of
        // an instance type, and the index is into the instance type's exports.
        // More information about this can be found on
        // `ComponentState::imported_resources`.
        //
        // This loop will follow the list of indices for each resource and, at
        // the same time, walk through the arguments supplied to instantiating
        // the `component_type`. This means that within `component_type`
        // index-based lookups are performed while in `args` name-based
        // lookups are performed.
        //
        // Note that here it's possible that `args` doesn't actually supply the
        // correct type of import for each item since argument checking has
        // not proceeded yet. These type errors, however, aren't handled by
        // this loop and are deferred below to the main subtyping check. That
        // means that `mapping` won't necessarily have a mapping for all
        // imported resources into `component_type`, but that should be ok.
        let component_type = &self.b[b];
        let entities = match kind {
            ExternKind::Import => &component_type.imports,
            ExternKind::Export => &component_type.exports,
        };
        let resources = match kind {
            ExternKind::Import => &component_type.imported_resources,
            ExternKind::Export => &component_type.defined_resources,
        };
        let mut mapping = Remapping::default();
        'outer: for (resource, path) in resources.iter() {
            // Lookup the first path item in `imports` and the corresponding
            // entry in `args` by name.
            let (name, ty) = entities.get_index(path[0]).unwrap();
            let mut ty = *ty;
            let mut arg = a.get(name);

            // Lookup all the subsequent `path` entries, if any, by index in
            // `ty` and by name in `arg`. Type errors in `arg` are skipped over
            // entirely.
            for i in path.iter().skip(1).copied() {
                let id = match ty {
                    ComponentEntityType::Instance(id) => id,
                    _ => unreachable!(),
                };
                let (name, next_ty) = self.b[id].exports.get_index(i).unwrap();
                ty = *next_ty;
                arg = match arg {
                    Some(ComponentEntityType::Instance(id)) => self.a[*id].exports.get(name),
                    _ => continue 'outer,
                };
            }

            // Double-check that `ty`, the leaf type of `component_type`, is
            // indeed the expected resource.
            if cfg!(debug_assertions) {
                let id = match ty {
                    ComponentEntityType::Type { created, .. } => match created {
                        ComponentAnyTypeId::Resource(id) => id.resource(),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
                assert_eq!(id, *resource);
            }

            // The leaf of `arg` should be a type which is a resource. If not
            // it's skipped and this'll wind up generating an error later on in
            // subtype checking below.
            if let Some(ComponentEntityType::Type { created, .. }) = arg {
                if let ComponentAnyTypeId::Resource(r) = created {
                    mapping.resources.insert(*resource, r.resource());
                }
            }
        }

        // Now that a mapping from the resources in `b` to the resources in `a`
        // has been determined it's possible to perform the actual subtype
        // check.
        //
        // This subtype check notably needs to ensure that all resource types
        // line up. To achieve this the `mapping` previously calculated is used
        // to perform a substitution on each component entity type.
        //
        // The first loop here performs a name lookup to create a list of
        // values from `a` to expected items in `b`. Once the list is created
        // the substitution check is performed on each element.
        let mut to_typecheck = Vec::new();
        for (name, expected) in entities.iter() {
            match a.get(name) {
                Some(arg) => to_typecheck.push((arg.clone(), expected.clone())),
                None => bail!(offset, "missing {} named `{name}`", kind.desc()),
            }
        }
        let mut type_map = HashMap::default();
        for (i, (actual, expected)) in to_typecheck.into_iter().enumerate() {
            let result = self.with_checkpoint(|this| {
                let mut expected = expected;
                this.b.remap_component_entity(&mut expected, &mut mapping);
                mapping.types.clear();
                this.component_entity_type(&actual, &expected, offset)
            });
            let err = match result {
                Ok(()) => {
                    // On a successful type-check record a mapping of
                    // type-to-type in `type_map` for any type imports that were
                    // satisfied. This is then used afterwards when performing
                    // type substitution to remap all component-local types to
                    // those that were provided in the imports.
                    self.register_type_renamings(actual, expected, &mut type_map);
                    continue;
                }
                Err(e) => e,
            };

            // If an error happens then attach the name of the entity to the
            // error message using the `i` iteration counter.
            let component_type = &self.b[b];
            let entities = match kind {
                ExternKind::Import => &component_type.imports,
                ExternKind::Export => &component_type.exports,
            };
            let (name, _) = entities.get_index(i).unwrap();
            return Err(err.with_context(|| format!("type mismatch for {} `{name}`", kind.desc())));
        }
        mapping.types = type_map;
        Ok(mapping)
    }

    pub(crate) fn entity_type(&self, a: &EntityType, b: &EntityType, offset: usize) -> Result<()> {
        macro_rules! limits_match {
            ($a:expr, $b:expr) => {{
                let a = $a;
                let b = $b;
                a.initial >= b.initial
                    && match b.maximum {
                        Some(b_max) => match a.maximum {
                            Some(a_max) => a_max <= b_max,
                            None => false,
                        },
                        None => true,
                    }
            }};
        }

        match (a, b) {
            (EntityType::Func(a), EntityType::Func(b)) => {
                self.core_func_type(self.a[*a].unwrap_func(), self.b[*b].unwrap_func(), offset)
            }
            (EntityType::Func(_), b) => bail!(offset, "expected {}, found func", b.desc()),
            (EntityType::Table(a), EntityType::Table(b)) => {
                if a.element_type != b.element_type {
                    bail!(
                        offset,
                        "expected table element type {}, found {}",
                        b.element_type,
                        a.element_type,
                    )
                }
                if limits_match!(a, b) {
                    Ok(())
                } else {
                    bail!(offset, "mismatch in table limits")
                }
            }
            (EntityType::Table(_), b) => bail!(offset, "expected {}, found table", b.desc()),
            (EntityType::Memory(a), EntityType::Memory(b)) => {
                if a.shared != b.shared {
                    bail!(offset, "mismatch in the shared flag for memories")
                }
                if a.memory64 != b.memory64 {
                    bail!(offset, "mismatch in index type used for memories")
                }
                if limits_match!(a, b) {
                    Ok(())
                } else {
                    bail!(offset, "mismatch in memory limits")
                }
            }
            (EntityType::Memory(_), b) => bail!(offset, "expected {}, found memory", b.desc()),
            (EntityType::Global(a), EntityType::Global(b)) => {
                if a.mutable != b.mutable {
                    bail!(offset, "global types differ in mutability")
                }
                if a.content_type == b.content_type {
                    Ok(())
                } else {
                    bail!(
                        offset,
                        "expected global type {}, found {}",
                        b.content_type,
                        a.content_type,
                    )
                }
            }
            (EntityType::Global(_), b) => bail!(offset, "expected {}, found global", b.desc()),
            (EntityType::Tag(a), EntityType::Tag(b)) => {
                self.core_func_type(self.a[*a].unwrap_func(), self.b[*b].unwrap_func(), offset)
            }
            (EntityType::Tag(_), b) => bail!(offset, "expected {}, found tag", b.desc()),
        }
    }

    fn core_func_type(&self, a: &FuncType, b: &FuncType, offset: usize) -> Result<()> {
        if a == b {
            Ok(())
        } else {
            bail!(
                offset,
                "expected: {}\n\
                 found:    {}",
                b.desc(),
                a.desc(),
            )
        }
    }

    pub(crate) fn component_val_type(
        &self,
        a: &ComponentValType,
        b: &ComponentValType,
        offset: usize,
    ) -> Result<()> {
        match (a, b) {
            (ComponentValType::Primitive(a), ComponentValType::Primitive(b)) => {
                self.primitive_val_type(*a, *b, offset)
            }
            (ComponentValType::Type(a), ComponentValType::Type(b)) => {
                self.component_defined_type(*a, *b, offset)
            }
            (ComponentValType::Primitive(a), ComponentValType::Type(b)) => match &self.b[*b] {
                ComponentDefinedType::Primitive(b) => self.primitive_val_type(*a, *b, offset),
                b => bail!(offset, "expected {}, found {a}", b.desc()),
            },
            (ComponentValType::Type(a), ComponentValType::Primitive(b)) => match &self.a[*a] {
                ComponentDefinedType::Primitive(a) => self.primitive_val_type(*a, *b, offset),
                a => bail!(offset, "expected {b}, found {}", a.desc()),
            },
        }
    }

    fn component_defined_type(
        &self,
        a: ComponentDefinedTypeId,
        b: ComponentDefinedTypeId,
        offset: usize,
    ) -> Result<()> {
        use ComponentDefinedType::*;

        // Note that the implementation of subtyping here diverges from the
        // upstream specification intentionally, see the documentation on
        // function subtyping for more information.
        match (&self.a[a], &self.b[b]) {
            (Primitive(a), Primitive(b)) => self.primitive_val_type(*a, *b, offset),
            (Primitive(a), b) => bail!(offset, "expected {}, found {a}", b.desc()),
            (Record(a), Record(b)) => {
                if a.fields.len() != b.fields.len() {
                    bail!(
                        offset,
                        "expected {} fields, found {}",
                        b.fields.len(),
                        a.fields.len(),
                    );
                }

                for ((aname, a), (bname, b)) in a.fields.iter().zip(b.fields.iter()) {
                    if aname != bname {
                        bail!(offset, "expected field name `{bname}`, found `{aname}`");
                    }
                    self.component_val_type(a, b, offset)
                        .with_context(|| format!("type mismatch in record field `{aname}`"))?;
                }
                Ok(())
            }
            (Record(_), b) => bail!(offset, "expected {}, found record", b.desc()),
            (Variant(a), Variant(b)) => {
                if a.cases.len() != b.cases.len() {
                    bail!(
                        offset,
                        "expected {} cases, found {}",
                        b.cases.len(),
                        a.cases.len(),
                    );
                }
                for ((aname, a), (bname, b)) in a.cases.iter().zip(b.cases.iter()) {
                    if aname != bname {
                        bail!(offset, "expected case named `{bname}`, found `{aname}`");
                    }
                    match (&a.ty, &b.ty) {
                        (Some(a), Some(b)) => self
                            .component_val_type(a, b, offset)
                            .with_context(|| format!("type mismatch in variant case `{aname}`"))?,
                        (None, None) => {}
                        (None, Some(_)) => {
                            bail!(offset, "expected case `{aname}` to have a type, found none")
                        }
                        (Some(_), None) => bail!(offset, "expected case `{aname}` to have no type"),
                    }
                }
                Ok(())
            }
            (Variant(_), b) => bail!(offset, "expected {}, found variant", b.desc()),
            (List(a), List(b)) | (Option(a), Option(b)) => self.component_val_type(a, b, offset),
            (List(_), b) => bail!(offset, "expected {}, found list", b.desc()),
            (Option(_), b) => bail!(offset, "expected {}, found option", b.desc()),
            (Tuple(a), Tuple(b)) => {
                if a.types.len() != b.types.len() {
                    bail!(
                        offset,
                        "expected {} types, found {}",
                        b.types.len(),
                        a.types.len(),
                    );
                }
                for (i, (a, b)) in a.types.iter().zip(b.types.iter()).enumerate() {
                    self.component_val_type(a, b, offset)
                        .with_context(|| format!("type mismatch in tuple field {i}"))?;
                }
                Ok(())
            }
            (Tuple(_), b) => bail!(offset, "expected {}, found tuple", b.desc()),
            (at @ Flags(a), Flags(b)) | (at @ Enum(a), Enum(b)) => {
                let desc = match at {
                    Flags(_) => "flags",
                    _ => "enum",
                };
                if a.len() == b.len() && a.iter().eq(b.iter()) {
                    Ok(())
                } else {
                    bail!(offset, "mismatch in {desc} elements")
                }
            }
            (Flags(_), b) => bail!(offset, "expected {}, found flags", b.desc()),
            (Enum(_), b) => bail!(offset, "expected {}, found enum", b.desc()),
            (Result { ok: ao, err: ae }, Result { ok: bo, err: be }) => {
                match (ao, bo) {
                    (None, None) => {}
                    (Some(a), Some(b)) => self
                        .component_val_type(a, b, offset)
                        .with_context(|| "type mismatch in ok variant")?,
                    (None, Some(_)) => bail!(offset, "expected ok type, but found none"),
                    (Some(_), None) => bail!(offset, "expected ok type to not be present"),
                }
                match (ae, be) {
                    (None, None) => {}
                    (Some(a), Some(b)) => self
                        .component_val_type(a, b, offset)
                        .with_context(|| "type mismatch in err variant")?,
                    (None, Some(_)) => bail!(offset, "expected err type, but found none"),
                    (Some(_), None) => bail!(offset, "expected err type to not be present"),
                }
                Ok(())
            }
            (Result { .. }, b) => bail!(offset, "expected {}, found result", b.desc()),
            (Own(a), Own(b)) | (Borrow(a), Borrow(b)) => {
                if a.resource() == b.resource() {
                    Ok(())
                } else {
                    bail!(offset, "resource types are not the same")
                }
            }
            (Own(_), b) => bail!(offset, "expected {}, found own", b.desc()),
            (Borrow(_), b) => bail!(offset, "expected {}, found borrow", b.desc()),
        }
    }

    fn primitive_val_type(
        &self,
        a: PrimitiveValType,
        b: PrimitiveValType,
        offset: usize,
    ) -> Result<()> {
        // Note that this intentionally diverges from the upstream specification
        // at this time and only considers exact equality for subtyping
        // relationships.
        //
        // More information can be found in the subtyping implementation for
        // component functions.
        if a == b {
            Ok(())
        } else {
            bail!(offset, "expected primitive `{b}` found primitive `{a}`")
        }
    }

    fn register_type_renamings(
        &self,
        actual: ComponentEntityType,
        expected: ComponentEntityType,
        type_map: &mut HashMap<ComponentAnyTypeId, ComponentAnyTypeId>,
    ) {
        match (expected, actual) {
            (
                ComponentEntityType::Type {
                    created: expected, ..
                },
                ComponentEntityType::Type {
                    created: actual, ..
                },
            ) => {
                let prev = type_map.insert(expected, actual);
                assert!(prev.is_none());
            }
            (ComponentEntityType::Instance(expected), ComponentEntityType::Instance(actual)) => {
                let actual = &self.a[actual];
                for (name, expected) in self.b[expected].exports.iter() {
                    let actual = actual.exports[name];
                    self.register_type_renamings(actual, *expected, type_map);
                }
            }
            _ => {}
        }
    }
}

/// A helper typed used purely during subtyping as part of `SubtypeCx`.
///
/// This takes a `types` list as input which is the "base" of the ids that can
/// be indexed through this arena. All future types pushed into this, if any,
/// are stored in `self.list`.
///
/// This is intended to have arena-like behavior where everything pushed onto
/// `self.list` is thrown away after a subtyping computation is performed. All
/// new types pushed into this arena are purely temporary.
pub struct SubtypeArena<'a> {
    types: &'a TypeList,
    list: TypeList,
}

impl<'a> SubtypeArena<'a> {
    fn new(types: &'a TypeList) -> SubtypeArena<'a> {
        SubtypeArena {
            types,
            list: TypeList::default(),
        }
    }
}

impl<T> Index<T> for SubtypeArena<'_>
where
    T: TypeIdentifier,
{
    type Output = T::Data;

    fn index(&self, id: T) -> &T::Data {
        let index = id.index();
        if index < T::list(self.types).len() {
            &self.types[id]
        } else {
            let temp_index = index - T::list(self.types).len();
            let temp_index = u32::try_from(temp_index).unwrap();
            let temp_id = T::from_index(temp_index);
            &self.list[temp_id]
        }
    }
}

impl Remap for SubtypeArena<'_> {
    fn push_ty<T>(&mut self, ty: T) -> T::Id
    where
        T: TypeData,
    {
        let index = T::Id::list(&self.list).len() + T::Id::list(self.types).len();
        let index = u32::try_from(index).unwrap();
        self.list.push(ty);
        T::Id::from_index(index)
    }
}

/// Helper trait for adding contextual information to an error, modeled after
/// `anyhow::Context`.
pub(crate) trait Context {
    fn with_context<S>(self, context: impl FnOnce() -> S) -> Self
    where
        S: Into<String>;
}

impl<T> Context for Result<T> {
    fn with_context<S>(self, context: impl FnOnce() -> S) -> Self
    where
        S: Into<String>,
    {
        match self {
            Ok(val) => Ok(val),
            Err(e) => Err(e.with_context(context)),
        }
    }
}

impl Context for BinaryReaderError {
    fn with_context<S>(mut self, context: impl FnOnce() -> S) -> Self
    where
        S: Into<String>,
    {
        self.add_context(context().into());
        self
    }
}
