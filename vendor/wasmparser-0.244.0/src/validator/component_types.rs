//! Types relating to type information provided by validation.

use super::component::ExternKind;
use super::{CanonicalOptions, Concurrency};
use crate::validator::StringEncoding;
use crate::validator::names::KebabString;
use crate::validator::types::{
    CoreTypeId, EntityType, SnapshotList, TypeAlloc, TypeData, TypeIdentifier, TypeInfo, TypeList,
    Types, TypesKind, TypesRef, TypesRefKind,
};
use crate::{AbstractHeapType, CompositeInnerType, HeapType, RefType, StorageType, prelude::*};
use crate::{
    BinaryReaderError, FuncType, MemoryType, PrimitiveValType, Result, TableType, ValType,
};
use core::fmt;
use core::ops::Index;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    mem,
};

/// The maximum number of parameters in the canonical ABI that can be passed by value.
///
/// Functions that exceed this limit will instead pass parameters indirectly from
/// linear memory via a single pointer parameter.
const MAX_FLAT_FUNC_PARAMS: usize = 16;
/// The maximum number of parameters in the canonical ABI that can be passed by
/// value in async function imports/exports.
const MAX_FLAT_ASYNC_PARAMS: usize = 4;
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

    #[track_caller]
    fn assert_push(&mut self, ty: ValType) {
        assert!(self.try_push(ty));
    }

    #[must_use = "value is not actually pushed when maxed"]
    fn try_push(&mut self, ty: ValType) -> bool {
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

impl fmt::Debug for LoweredTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

/// Represents a component function type's in-progress lowering into a core
/// type.
#[derive(Debug)]
struct LoweredSignature {
    params: LoweredTypes,
    results: LoweredTypes,
}

impl LoweredSignature {
    pub(crate) fn into_func_type(self) -> FuncType {
        FuncType::new(
            self.params.as_slice().iter().copied(),
            self.results.as_slice().iter().copied(),
        )
    }
}

impl Default for LoweredSignature {
    fn default() -> Self {
        Self {
            params: LoweredTypes::new(MAX_FLAT_FUNC_PARAMS),
            results: LoweredTypes::new(MAX_FLAT_FUNC_RESULTS),
        }
    }
}

impl PrimitiveValType {
    pub(crate) fn lower_gc(
        &self,
        types: &TypeList,
        _abi: Abi,
        options: &CanonicalOptions,
        offset: usize,
        core: ArgOrField,
    ) -> Result<()> {
        match (self, core) {
            (
                PrimitiveValType::Bool,
                ArgOrField::Field(StorageType::I8) | ArgOrField::Arg(ValType::I32),
            ) => Ok(()),
            (PrimitiveValType::Bool, ArgOrField::Arg(_)) => bail!(
                offset,
                "expected to lower component `bool` type to core `i32` type, found `{core}`"
            ),
            (PrimitiveValType::Bool, ArgOrField::Field(_)) => bail!(
                offset,
                "expected to lower component `bool` type to core `i8` type, found `{core}`"
            ),

            (
                PrimitiveValType::S8,
                ArgOrField::Field(StorageType::I8) | ArgOrField::Arg(ValType::I32),
            ) => Ok(()),
            (PrimitiveValType::S8, ArgOrField::Arg(_)) => bail!(
                offset,
                "expected to lower component `s8` type to core `i32` type, found `{core}`"
            ),
            (PrimitiveValType::S8, ArgOrField::Field(_)) => bail!(
                offset,
                "expected to lower component `s8` type to core `i8` type, found `{core}`"
            ),

            (
                PrimitiveValType::U8,
                ArgOrField::Field(StorageType::I8) | ArgOrField::Arg(ValType::I32),
            ) => Ok(()),
            (PrimitiveValType::U8, ArgOrField::Arg(_)) => bail!(
                offset,
                "expected to lower component `u8` type to core `i32` type, found `{core}`"
            ),
            (PrimitiveValType::U8, ArgOrField::Field(_)) => bail!(
                offset,
                "expected to lower component `u8` type to core `i8` type, found `{core}`"
            ),

            (
                PrimitiveValType::S16,
                ArgOrField::Field(StorageType::I16) | ArgOrField::Arg(ValType::I32),
            ) => Ok(()),
            (PrimitiveValType::S16, ArgOrField::Arg(_)) => bail!(
                offset,
                "expected to lower component `s16` type to core `i32` type, found `{core}`"
            ),
            (PrimitiveValType::S16, ArgOrField::Field(_)) => bail!(
                offset,
                "expected to lower component `s16` type to core `i16` type, found `{core}`"
            ),

            (
                PrimitiveValType::U16,
                ArgOrField::Field(StorageType::I16) | ArgOrField::Arg(ValType::I32),
            ) => Ok(()),
            (PrimitiveValType::U16, ArgOrField::Arg(_)) => bail!(
                offset,
                "expected to lower component `u16` type to core `i32` type, found `{core}`"
            ),
            (PrimitiveValType::U16, ArgOrField::Field(_)) => bail!(
                offset,
                "expected to lower component `u16` type to core `i16` type, found `{core}`"
            ),

            (PrimitiveValType::S32, _) if core.as_val_type() == Some(ValType::I32) => Ok(()),
            (PrimitiveValType::S32, _) => bail!(
                offset,
                "expected to lower component `s32` type to core `i32` type, found `{core}`"
            ),

            (PrimitiveValType::U32, _) if core.as_val_type() == Some(ValType::I32) => Ok(()),
            (PrimitiveValType::U32, _) => bail!(
                offset,
                "expected to lower component `u32` type to core `i32` type, found `{core}`"
            ),

            (PrimitiveValType::S64, _) if core.as_val_type() == Some(ValType::I64) => Ok(()),
            (PrimitiveValType::S64, _) => bail!(
                offset,
                "expected to lower component `s64` type to core `i64` type, found `{core}`"
            ),

            (PrimitiveValType::U64, _) if core.as_val_type() == Some(ValType::I64) => Ok(()),
            (PrimitiveValType::U64, _) => bail!(
                offset,
                "expected to lower component `u64` type to core `i64` type, found `{core}`"
            ),

            (PrimitiveValType::F32, _) if core.as_val_type() == Some(ValType::F32) => Ok(()),
            (PrimitiveValType::F32, _) => bail!(
                offset,
                "expected to lower component `f32` type to core `f32` type, found `{core}`"
            ),

            (PrimitiveValType::F64, _) if core.as_val_type() == Some(ValType::F64) => Ok(()),
            (PrimitiveValType::F64, _) => bail!(
                offset,
                "expected to lower component `f64` type to core `f64` type, found `{core}`"
            ),

            (PrimitiveValType::Char, _) if core.as_val_type() == Some(ValType::I32) => Ok(()),
            (PrimitiveValType::Char, _) => bail!(
                offset,
                "expected to lower component `char` type to core `i32` type, found `{core}`"
            ),

            (PrimitiveValType::String, _) => {
                let type_mismatch_err = || {
                    let expected = match options.string_encoding {
                        StringEncoding::Utf8 | StringEncoding::CompactUtf16 => {
                            "(ref null? (array (mut? i8)))"
                        }
                        StringEncoding::Utf16 => "(ref null? (array (mut? i16)))",
                    };
                    bail!(
                        offset,
                        "expected to lower component `string` type to core `{expected}` \
                         type, found `{core}`"
                    )
                };

                match core.as_concrete_ref() {
                    Some(id) => match types[id].composite_type.inner {
                        CompositeInnerType::Array(ty) => {
                            match (options.string_encoding, ty.0.element_type) {
                                (
                                    StringEncoding::Utf8 | StringEncoding::CompactUtf16,
                                    StorageType::I8,
                                )
                                | (StringEncoding::Utf16, StorageType::I16) => Ok(()),
                                _ => type_mismatch_err(),
                            }
                        }
                        _ => type_mismatch_err(),
                    },
                    _ => type_mismatch_err(),
                }
            }

            (PrimitiveValType::ErrorContext, _) => {
                if let Some(r) = core.as_ref_type() {
                    if let HeapType::Abstract {
                        shared: _,
                        ty: AbstractHeapType::Extern,
                    } = r.heap_type()
                    {
                        return Ok(());
                    }
                }
                bail!(
                    offset,
                    "expected to lower component `error-context` type into core `(ref null? extern)` type, but \
                     found `{core}`",
                )
            }
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
        | PrimitiveValType::Char
        | PrimitiveValType::ErrorContext => lowered_types.try_push(ValType::I32),
        PrimitiveValType::S64 | PrimitiveValType::U64 => lowered_types.try_push(ValType::I64),
        PrimitiveValType::F32 => lowered_types.try_push(ValType::F32),
        PrimitiveValType::F64 => lowered_types.try_push(ValType::F64),
        PrimitiveValType::String => {
            lowered_types.try_push(ValType::I32) && lowered_types.try_push(ValType::I32)
        }
    }
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
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

macro_rules! define_type_id {
    ($name:ident $($rest:tt)*) => {
        super::types::define_type_id!($name $($rest)*);

        impl Aliasable for $name {
            fn alias_id(&self) -> u32 {
                NO_ALIAS
            }

            fn set_alias_id(&mut self, _: u32) {}
        }
    }
}

define_type_id!(
    ComponentTypeId,
    ComponentType,
    component.components,
    "component"
);

define_type_id!(
    ComponentValueTypeId,
    ComponentValType,
    component.component_values,
    "component value"
);

define_type_id!(
    ComponentInstanceTypeId,
    ComponentInstanceType,
    component.component_instances,
    "component instance"
);

define_type_id!(
    ComponentFuncTypeId,
    ComponentFuncType,
    component.component_funcs,
    "component function"
);

define_type_id!(
    ComponentCoreInstanceTypeId,
    InstanceType,
    component.core_instances,
    "component's core instance"
);

define_type_id!(
    ComponentCoreModuleTypeId,
    ModuleType,
    component.core_modules,
    "component's core module"
);

/// Represents a unique identifier for a component type type known to a
/// [`crate::Validator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ComponentDefinedTypeId {
    index: u32,
    alias_id: u32,
}

#[test]
fn assert_defined_type_small() {
    assert!(core::mem::size_of::<ComponentDefinedTypeId>() <= 8);
}

impl TypeIdentifier for ComponentDefinedTypeId {
    type Data = ComponentDefinedType;

    fn from_index(index: u32) -> Self {
        ComponentDefinedTypeId {
            index,
            alias_id: NO_ALIAS,
        }
    }

    fn list(types: &TypeList) -> &SnapshotList<Self::Data> {
        &types.component.component_defined_types
    }

    fn list_mut(types: &mut TypeList) -> &mut SnapshotList<Self::Data> {
        &mut types.component.component_defined_types
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
    const IS_CORE_SUB_TYPE: bool = false;
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

    fn lower_gc(
        &self,
        types: &TypeList,
        abi: Abi,
        options: &CanonicalOptions,
        offset: usize,
        core: ArgOrField,
    ) -> Result<()> {
        match self {
            ComponentValType::Primitive(ty) => ty.lower_gc(types, abi, options, offset, core),
            ComponentValType::Type(ty) => types[*ty].lower_gc(types, abi, options, offset, core),
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

impl Hash for dyn ModuleImportKey + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.module().hash(state);
        self.name().hash(state);
    }
}

impl PartialEq for dyn ModuleImportKey + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.module() == other.module() && self.name() == other.name()
    }
}

impl Eq for dyn ModuleImportKey + '_ {}

impl Ord for dyn ModuleImportKey + '_ {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match self.module().cmp(other.module()) {
            core::cmp::Ordering::Equal => (),
            order => return order,
        };
        self.name().cmp(other.name())
    }
}

impl PartialOrd for dyn ModuleImportKey + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

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
    const IS_CORE_SUB_TYPE: bool = false;
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
    const IS_CORE_SUB_TYPE: bool = false;
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
    ///
    /// # Panics
    ///
    /// Panics if the two given `TypesRef`s are not associated with the same
    /// `Validator`.
    pub fn is_subtype_of(a: &Self, at: TypesRef<'_>, b: &Self, bt: TypesRef<'_>) -> bool {
        assert_eq!(at.id(), bt.id());
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
    /// This should technically be inferable from the structure of `imports`,
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
    const IS_CORE_SUB_TYPE: bool = false;
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
    const IS_CORE_SUB_TYPE: bool = false;
    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        self.info
    }
}

/// Represents a type of a component function.
#[derive(Debug, Clone)]
pub struct ComponentFuncType {
    /// Metadata about this function type.
    pub(crate) info: TypeInfo,
    /// Whether or not this is an async function.
    pub async_: bool,
    /// The function parameters.
    pub params: Box<[(KebabString, ComponentValType)]>,
    /// The function's result.
    pub result: Option<ComponentValType>,
}

impl TypeData for ComponentFuncType {
    type Id = ComponentFuncTypeId;
    const IS_CORE_SUB_TYPE: bool = false;
    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        self.info
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Abi {
    Lift,
    Lower,
}

impl Abi {
    fn invert(&self) -> Self {
        match self {
            Abi::Lift => Abi::Lower,
            Abi::Lower => Abi::Lift,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ArgOrField {
    /// Lifting to, or lowering from, an argument value.
    Arg(ValType),
    /// Lifting to, or lowering from, a struct field or array element.
    Field(StorageType),
}

impl From<ValType> for ArgOrField {
    fn from(v: ValType) -> Self {
        Self::Arg(v)
    }
}

impl From<StorageType> for ArgOrField {
    fn from(v: StorageType) -> Self {
        Self::Field(v)
    }
}

impl core::fmt::Display for ArgOrField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgOrField::Arg(ty) => core::fmt::Display::fmt(ty, f),
            ArgOrField::Field(ty) => core::fmt::Display::fmt(ty, f),
        }
    }
}

impl ArgOrField {
    pub(crate) fn as_val_type(self) -> Option<ValType> {
        match self {
            ArgOrField::Arg(ty) | ArgOrField::Field(StorageType::Val(ty)) => Some(ty),
            _ => None,
        }
    }

    pub(crate) fn as_ref_type(self) -> Option<RefType> {
        self.as_val_type()?.as_reference_type()
    }

    pub(crate) fn as_concrete_ref(self) -> Option<CoreTypeId> {
        match self.as_ref_type()?.heap_type() {
            HeapType::Abstract { .. } => None,
            HeapType::Concrete(idx) | HeapType::Exact(idx) => {
                let id = idx
                    .as_core_type_id()
                    .expect("validation only sees core type ids");
                Some(id)
            }
        }
    }
}

pub(crate) enum LoweredFuncType {
    New(FuncType),
    Existing(CoreTypeId),
}

impl LoweredFuncType {
    pub(crate) fn intern(self, types: &mut TypeAlloc, offset: usize) -> CoreTypeId {
        match self {
            LoweredFuncType::New(ty) => types.intern_func_type(ty, offset),
            LoweredFuncType::Existing(id) => id,
        }
    }
}

impl ComponentFuncType {
    /// Lowers the component function type to core parameter and result types for the
    /// canonical ABI.
    pub(crate) fn lower(
        &self,
        types: &TypeList,
        options: &CanonicalOptions,
        abi: Abi,
        offset: usize,
    ) -> Result<LoweredFuncType> {
        let mut sig = LoweredSignature::default();

        if options.gc {
            return self.lower_gc(types, abi, options, offset);
        }

        if abi == Abi::Lower && options.concurrency.is_async() {
            sig.params.max = MAX_FLAT_ASYNC_PARAMS;
        }

        for (_, ty) in self.params.iter() {
            // Check to see if `ty` has a pointer somewhere in it, needed for
            // any type that transitively contains either a string or a list.
            // In this situation lowered functions must specify `memory`, and
            // lifted functions must specify `realloc` as well. Lifted functions
            // gain their memory requirement through the final clause of this
            // function.
            match abi {
                Abi::Lower => {
                    options.require_memory_if(offset, || ty.contains_ptr(types))?;
                }
                Abi::Lift => {
                    options.require_realloc_if(offset, || ty.contains_ptr(types))?;
                }
            }

            if !ty.push_wasm_types(types, &mut sig.params) {
                // Too many parameters to pass directly
                // Function will have a single pointer parameter to pass the arguments
                // via linear memory
                sig.params.clear();
                assert!(sig.params.try_push(ValType::I32));
                options.require_memory(offset)?;

                // We need realloc as well when lifting a function
                if let Abi::Lift = abi {
                    options.require_realloc(offset)?;
                }
                break;
            }
        }

        match (abi, options.concurrency) {
            (Abi::Lower | Abi::Lift, Concurrency::Sync) => {
                if let Some(ty) = &self.result {
                    // Results of lowered functions that contains pointers must be
                    // allocated by the callee meaning that realloc is required.
                    // Results of lifted function are allocated by the guest which
                    // means that no realloc option is necessary.
                    options.require_realloc_if(offset, || {
                        abi == Abi::Lower && ty.contains_ptr(types)
                    })?;

                    if !ty.push_wasm_types(types, &mut sig.results) {
                        // Too many results to return directly, either a retptr
                        // parameter will be used (import) or a single pointer
                        // will be returned (export).
                        sig.results.clear();
                        options.require_memory(offset)?;
                        match abi {
                            Abi::Lower => {
                                sig.params.max = MAX_LOWERED_TYPES;
                                assert!(sig.params.try_push(ValType::I32));
                            }
                            Abi::Lift => {
                                assert!(sig.results.try_push(ValType::I32));
                            }
                        }
                    }
                }
            }
            (Abi::Lower, Concurrency::Async { callback: _ }) => {
                if self.result.is_some() {
                    sig.params.max = MAX_LOWERED_TYPES;
                    sig.params.assert_push(ValType::I32);
                    options.require_memory(offset)?;
                }
                sig.results.assert_push(ValType::I32);
            }
            (Abi::Lift, Concurrency::Async { callback }) => {
                if let Some(ty) = &self.result {
                    // The result of an async lift will be returned via a call
                    // to `task.return` rather than the lifted function itself.
                    // Here we require a memory if either the return type
                    // contains a pointer or has a flattened form that exceeds
                    // `MAX_FLAT_FUNC_PARAMS`.
                    //
                    // Note that the return type itself has no effect on the
                    // expected core signature of the lifted function.

                    let overflow =
                        !ty.push_wasm_types(types, &mut LoweredTypes::new(MAX_FLAT_FUNC_PARAMS));

                    options.require_memory_if(offset, || overflow || ty.contains_ptr(types))?;
                }
                if callback.is_some() {
                    sig.results.assert_push(ValType::I32);
                }
            }
        }

        Ok(LoweredFuncType::New(sig.into_func_type()))
    }

    fn lower_gc(
        &self,
        types: &TypeList,
        abi: Abi,
        options: &CanonicalOptions,
        offset: usize,
    ) -> Result<LoweredFuncType> {
        let core_type_id = options.core_type.unwrap();
        let core_func_ty = types[core_type_id].unwrap_func();

        ensure!(
            core_func_ty.params().len() == self.params.len(),
            offset,
            "declared `core-type` has {} parameters, but component function has {} parameters",
            core_func_ty.params().len(),
            self.params.len(),
        );
        for (core, (_name, comp)) in core_func_ty.params().iter().zip(self.params.iter()) {
            comp.lower_gc(types, abi.invert(), options, offset, (*core).into())?;
        }

        ensure!(
            core_func_ty.results().len() == usize::from(self.result.is_some()),
            offset,
            "declared `core-type` has {} results, but component function has {} results",
            core_func_ty.results().len(),
            usize::from(self.result.is_some()),
        );
        if let Some(result) = self.result {
            result.lower_gc(
                types,
                abi,
                options,
                offset,
                core_func_ty.results()[0].into(),
            )?;
        }

        Ok(LoweredFuncType::Existing(core_type_id))
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

impl RecordType {
    fn lower_gc(
        &self,
        types: &TypeList,
        abi: Abi,
        options: &CanonicalOptions,
        offset: usize,
        core: ArgOrField,
    ) -> Result<()> {
        lower_gc_product_type(
            self.fields.values(),
            types,
            abi,
            options,
            offset,
            core,
            "record",
        )
    }
}

/// Represents a variant type.
#[derive(Debug, Clone)]
pub struct VariantType {
    /// Metadata about this variant type.
    pub(crate) info: TypeInfo,
    /// The map of variant cases.
    pub cases: IndexMap<KebabString, VariantCase>,
}

impl VariantType {
    fn lower_gc(
        &self,
        types: &TypeList,
        abi: Abi,
        options: &CanonicalOptions,
        offset: usize,
        core: ArgOrField,
    ) -> Result<()> {
        lower_gc_sum_type(types, abi, options, offset, core, "variant")
    }
}

/// Common helper for lowering sum types (variants, options, and results) to
/// core GC types.
fn lower_gc_sum_type(
    types: &TypeList,
    _abi: Abi,
    _options: &CanonicalOptions,
    offset: usize,
    core: ArgOrField,
    kind: &str,
) -> Result<()> {
    if let Some(id) = core.as_concrete_ref() {
        if let CompositeInnerType::Struct(ty) = &types[id].composite_type.inner {
            if ty.fields.is_empty() {
                return Ok(());
            }
        }
    }

    bail!(
        offset,
        "expected to lower component `{kind}` type to core `(ref null? (struct))`, \
         but found `{core}`",
    )
}

/// Represents a tuple type.
#[derive(Debug, Clone)]
pub struct TupleType {
    /// Metadata about this tuple type.
    pub(crate) info: TypeInfo,
    /// The types of the tuple.
    pub types: Box<[ComponentValType]>,
}

impl TupleType {
    fn lower_gc(
        &self,
        types: &TypeList,
        abi: Abi,
        options: &CanonicalOptions,
        offset: usize,
        core: ArgOrField,
    ) -> Result<()> {
        lower_gc_product_type(
            self.types.iter(),
            types,
            abi,
            options,
            offset,
            core,
            "tuple",
        )
    }
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
    /// The type is a map.
    Map(ComponentValType, ComponentValType),
    /// The type is a fixed size list.
    FixedSizeList(ComponentValType, u32),
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
    /// A future type with the specified payload type.
    Future(Option<ComponentValType>),
    /// A stream type with the specified payload type.
    Stream(Option<ComponentValType>),
}

impl TypeData for ComponentDefinedType {
    type Id = ComponentDefinedTypeId;
    const IS_CORE_SUB_TYPE: bool = false;
    fn type_info(&self, types: &TypeList) -> TypeInfo {
        match self {
            Self::Primitive(_)
            | Self::Flags(_)
            | Self::Enum(_)
            | Self::Own(_)
            | Self::Future(_)
            | Self::Stream(_) => TypeInfo::new(),
            Self::Borrow(_) => TypeInfo::borrow(),
            Self::Record(r) => r.info,
            Self::Variant(v) => v.info,
            Self::Tuple(t) => t.info,
            Self::List(ty) | Self::FixedSizeList(ty, _) | Self::Option(ty) => ty.info(types),
            Self::Map(k, v) => {
                let mut info = k.info(types);
                info.combine(v.info(types), 0).unwrap();
                info
            }
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
            Self::List(_) | Self::Map(_, _) => true,
            Self::Tuple(t) => t.types.iter().any(|ty| ty.contains_ptr(types)),
            Self::Flags(_)
            | Self::Enum(_)
            | Self::Own(_)
            | Self::Borrow(_)
            | Self::Future(_)
            | Self::Stream(_) => false,
            Self::Option(ty) | Self::FixedSizeList(ty, _) => ty.contains_ptr(types),
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
            Self::List(_) | Self::Map(_, _) => {
                lowered_types.try_push(ValType::I32) && lowered_types.try_push(ValType::I32)
            }
            Self::FixedSizeList(ty, length) => {
                (0..*length).all(|_n| ty.push_wasm_types(types, lowered_types))
            }
            Self::Tuple(t) => t
                .types
                .iter()
                .all(|ty| ty.push_wasm_types(types, lowered_types)),
            Self::Flags(names) => {
                (0..(names.len() + 31) / 32).all(|_| lowered_types.try_push(ValType::I32))
            }
            Self::Enum(_) | Self::Own(_) | Self::Borrow(_) | Self::Future(_) | Self::Stream(_) => {
                lowered_types.try_push(ValType::I32)
            }
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
        if !lowered_types.try_push(ValType::I32) {
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
                        if !lowered_types.try_push(ty) {
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
            ComponentDefinedType::Map(_, _) => "map",
            ComponentDefinedType::FixedSizeList(_, _) => "fixed size list",
            ComponentDefinedType::Result { .. } => "result",
            ComponentDefinedType::Own(_) => "own",
            ComponentDefinedType::Borrow(_) => "borrow",
            ComponentDefinedType::Future(_) => "future",
            ComponentDefinedType::Stream(_) => "stream",
        }
    }

    fn lower_gc(
        &self,
        types: &TypeList,
        abi: Abi,
        options: &CanonicalOptions,
        offset: usize,
        core: ArgOrField,
    ) -> Result<()> {
        match self {
            ComponentDefinedType::Primitive(ty) => ty.lower_gc(types, abi, options, offset, core),

            ComponentDefinedType::Record(ty) => ty.lower_gc(types, abi, options, offset, core),

            ComponentDefinedType::Variant(ty) => ty.lower_gc(types, abi, options, offset, core),

            ComponentDefinedType::List(ty) | ComponentDefinedType::FixedSizeList(ty, _) => {
                let id = match core.as_concrete_ref() {
                    Some(id) => id,
                    None => bail!(
                        offset,
                        "expected to lower component `list` type into `(ref null? (array ...))`, but \
                         found `{core}`",
                    ),
                };
                let array_ty = match types[id].composite_type.inner {
                    CompositeInnerType::Array(ty) => ty,
                    _ => bail!(
                        offset,
                        "expected to lower component `list` type into `(ref null? (array ...))`, but \
                         found `{core}`",
                    ),
                };
                ty.lower_gc(types, abi, options, offset, array_ty.0.element_type.into())
            }

            ComponentDefinedType::Map(_, _) => bail!(
                offset,
                "GC lowering for component `map` type is not yet implemented"
            ),

            ComponentDefinedType::Tuple(ty) => ty.lower_gc(types, abi, options, offset, core),

            ComponentDefinedType::Flags(flags) => {
                assert!(flags.len() <= 32, "required by validation");
                if core.as_val_type() == Some(ValType::I32) {
                    Ok(())
                } else {
                    bail!(
                        offset,
                        "expected to lower component `flags` type into core `i32` type, but \
                         found `{core}`",
                    )
                }
            }

            ComponentDefinedType::Enum(_) => {
                if core.as_val_type() == Some(ValType::I32) {
                    Ok(())
                } else {
                    bail!(
                        offset,
                        "expected to lower component `enum` type into core `i32` type, but \
                         found `{core}`",
                    )
                }
            }

            ComponentDefinedType::Option(_) => {
                lower_gc_sum_type(types, abi, options, offset, core, "option")
            }

            ComponentDefinedType::Result { .. } => {
                lower_gc_sum_type(types, abi, options, offset, core, "result")
            }

            ComponentDefinedType::Own(_)
            | ComponentDefinedType::Borrow(_)
            | ComponentDefinedType::Future(_)
            | ComponentDefinedType::Stream(_) => {
                if let Some(r) = core.as_ref_type() {
                    if let HeapType::Abstract {
                        shared: _,
                        ty: AbstractHeapType::Extern,
                    } = r.heap_type()
                    {
                        return Ok(());
                    }
                }
                bail!(
                    offset,
                    "expected to lower component `{}` type into core `(ref null? extern)` type, but \
                     found `{core}`",
                    self.desc()
                )
            }
        }
    }
}

/// Shared helper for lowering component record and tuple types to core GC
/// types.
fn lower_gc_product_type<'a, I>(
    fields: I,
    types: &TypeList,
    abi: Abi,
    options: &CanonicalOptions,
    offset: usize,
    core: ArgOrField,
    kind: &str,
) -> core::result::Result<(), BinaryReaderError>
where
    I: IntoIterator<Item = &'a ComponentValType>,
    I::IntoIter: ExactSizeIterator,
{
    let fields = fields.into_iter();
    let fields_len = fields.len();

    if let Some(id) = core.as_concrete_ref() {
        if let CompositeInnerType::Struct(ty) = &types[id].composite_type.inner {
            ensure!(
                ty.fields.len() == fields_len,
                offset,
                "core `struct` has {} fields, but component `{kind}` has {fields_len} fields",
                ty.fields.len(),
            );
            for (core, comp) in ty.fields.iter().zip(fields) {
                comp.lower_gc(types, abi, options, offset, core.element_type.into())?;
            }
            return Ok(());
        }
    }

    bail!(
        offset,
        "expected to lower component `{kind}` type to core `(ref null? (struct ...))`, \
         but found `{core}`",
    )
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

impl<'a> TypesRef<'a> {
    /// Gets a core WebAssembly type id from a type index.
    ///
    /// Note that this is not to be confused with
    /// [`TypesRef::component_type_at`] which gets a component type from its
    /// index, nor [`TypesRef::core_type_count_in_module`] which does not work
    /// for components.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn core_type_at_in_component(&self, index: u32) -> ComponentCoreTypeId {
        match &self.kind {
            TypesRefKind::Module(_) => panic!("use `component_type_at_in_module` instead"),
            TypesRefKind::Component(component) => component.core_types[index as usize],
        }
    }

    /// Returns the number of core types defined so far within a component.
    ///
    /// This should only be used for components. For modules see
    /// [`TypesRef::core_type_count_in_module`].
    pub fn core_type_count_in_component(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(_) => 0,
            TypesRefKind::Component(component) => component.core_types.len() as u32,
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

    /// Returns the number of component types defined so far.
    pub fn component_type_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(_module) => 0,
            TypesRefKind::Component(component) => component.types.len() as u32,
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

    /// Returns the number of component values defined so far.
    pub fn value_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(_module) => 0,
            TypesRefKind::Component(component) => component.values.len() as u32,
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

impl Types {
    /// Gets a component WebAssembly type at the given type index.
    ///
    /// Note that this is in contrast to [`TypesRef::core_type_at_in_component`]
    /// which gets a core type from its index.
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

/// A snapshot list of types.
#[derive(Debug, Default)]
pub(crate) struct ComponentTypeList {
    // Keeps track of which `alias_id` is an alias of which other `alias_id`.
    alias_mappings: Map<u32, u32>,
    // Counter for generating new `alias_id`s.
    alias_counter: u32,
    // Snapshots of previously committed `TypeList`s' aliases.
    alias_snapshots: Vec<TypeListAliasSnapshot>,

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
    alias_mappings: Map<u32, u32>,
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
    core_type_to_depth: usize,
    rec_group_elements: usize,
    canonical_rec_groups: usize,
}

impl TypeList {
    fn checkpoint(&self) -> TypeListCheckpoint {
        let TypeList {
            component:
                ComponentTypeList {
                    alias_mappings: _,
                    alias_counter: _,
                    alias_snapshots: _,
                    components,
                    component_defined_types,
                    component_values,
                    component_instances,
                    component_funcs,
                    core_modules,
                    core_instances,
                },
            core_types,
            core_type_to_rec_group,
            core_type_to_supertype,
            core_type_to_depth,
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
            core_type_to_depth: core_type_to_depth.as_ref().map(|m| m.len()).unwrap_or(0),
            rec_group_elements: rec_group_elements.len(),
            canonical_rec_groups: canonical_rec_groups.as_ref().map(|m| m.len()).unwrap_or(0),
        }
    }

    fn reset_to_checkpoint(&mut self, checkpoint: TypeListCheckpoint) {
        let TypeList {
            component:
                ComponentTypeList {
                    alias_mappings: _,
                    alias_counter: _,
                    alias_snapshots: _,
                    components,
                    component_defined_types,
                    component_values,
                    component_instances,
                    component_funcs,
                    core_modules,
                    core_instances,
                },
            core_types,
            core_type_to_rec_group,
            core_type_to_supertype,
            core_type_to_depth,
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

        if let Some(core_type_to_depth) = core_type_to_depth {
            assert_eq!(
                core_type_to_depth.len(),
                checkpoint.core_type_to_depth,
                "checkpointing does not support resetting `core_type_to_depth` (it would require a \
                 proper immutable and persistent hash map) so adding new groups is disallowed"
            );
        }
        if let Some(canonical_rec_groups) = canonical_rec_groups {
            assert_eq!(
                canonical_rec_groups.len(),
                checkpoint.canonical_rec_groups,
                "checkpointing does not support resetting `canonical_rec_groups` (it would require a \
                 proper immutable and persistent hash map) so adding new groups is disallowed"
            );
        }
    }

    /// See `SnapshotList::with_unique`.
    pub fn with_unique<T>(&mut self, mut ty: T) -> T
    where
        T: Aliasable,
    {
        self.component
            .alias_mappings
            .insert(self.component.alias_counter, ty.alias_id());
        ty.set_alias_id(self.component.alias_counter);
        self.component.alias_counter += 1;
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
            .component
            .alias_snapshots
            .binary_search_by_key(&alias_id, |snapshot| snapshot.alias_counter)
        {
            Ok(_) => unreachable!(),
            Err(i) => i,
        };

        // If the `i` index is beyond the snapshot array then lookup in the
        // current mappings instead since it may refer to a type not snapshot
        // yet.
        ty.set_alias_id(match self.component.alias_snapshots.get(i) {
            Some(snapshot) => *snapshot.alias_mappings.get(&alias_id)?,
            None => *self.component.alias_mappings.get(&alias_id)?,
        });
        Some(ty)
    }
}

impl ComponentTypeList {
    pub fn commit(&mut self) -> ComponentTypeList {
        // Note that the `alias_counter` is bumped here to ensure that the
        // previous value of the unique counter is never used for an actual type
        // so it's suitable for lookup via a binary search.
        let alias_counter = self.alias_counter;
        self.alias_counter += 1;

        self.alias_snapshots.push(TypeListAliasSnapshot {
            alias_counter,
            alias_mappings: mem::take(&mut self.alias_mappings),
        });

        ComponentTypeList {
            alias_mappings: Map::default(),
            alias_counter: self.alias_counter,
            alias_snapshots: self.alias_snapshots.clone(),
            components: self.components.commit(),
            component_defined_types: self.component_defined_types.commit(),
            component_values: self.component_values.commit(),
            component_instances: self.component_instances.commit(),
            component_funcs: self.component_funcs.commit(),
            core_modules: self.core_modules.commit(),
            core_instances: self.core_instances.commit(),
        }
    }
}

pub(crate) struct ComponentTypeAlloc {
    // This is assigned at creation of a `TypeAlloc` and then never changed.
    // It's used in one entry for all `ResourceId`s contained within.
    globally_unique_id: usize,

    // This is a counter that's incremeneted each time `alloc_resource_id` is
    // called.
    next_resource_id: u32,
}

impl Default for ComponentTypeAlloc {
    fn default() -> ComponentTypeAlloc {
        static NEXT_GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);
        ComponentTypeAlloc {
            globally_unique_id: {
                let id = NEXT_GLOBAL_ID.fetch_add(1, Ordering::Relaxed);
                if id > usize::MAX - 10_000 {
                    NEXT_GLOBAL_ID.store(usize::MAX - 10_000, Ordering::Relaxed);
                    panic!("overflow on the global id counter");
                }
                id
            },
            next_resource_id: 0,
        }
    }
}

impl TypeAlloc {
    /// Allocates a new unique resource identifier.
    ///
    /// Note that uniqueness is only a property within this `TypeAlloc`.
    pub fn alloc_resource_id(&mut self) -> AliasableResourceId {
        let contextually_unique_id = self.component_alloc.next_resource_id;
        self.component_alloc.next_resource_id = self
            .component_alloc
            .next_resource_id
            .checked_add(1)
            .unwrap();
        AliasableResourceId {
            id: ResourceId {
                globally_unique_id: self.component_alloc.globally_unique_id,
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
            ComponentDefinedType::List(ty)
            | ComponentDefinedType::FixedSizeList(ty, _)
            | ComponentDefinedType::Option(ty) => {
                self.free_variables_valtype(ty, set);
            }
            ComponentDefinedType::Map(k, v) => {
                self.free_variables_valtype(k, set);
                self.free_variables_valtype(v, set);
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
            ComponentDefinedType::Future(ty) => {
                if let Some(ty) = ty {
                    self.free_variables_valtype(ty, set);
                }
            }
            ComponentDefinedType::Stream(ty) => {
                if let Some(ty) = ty {
                    self.free_variables_valtype(ty, set);
                }
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
            set.swap_remove(id);
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
            set.swap_remove(id);
        }
    }

    pub fn free_variables_component_func_type_id(
        &self,
        id: ComponentFuncTypeId,
        set: &mut IndexSet<ResourceId>,
    ) {
        let i = &self[id];
        for ty in i.params.iter().map(|(_, ty)| ty).chain(&i.result) {
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
        set: &Set<ComponentAnyTypeId>,
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
            | ComponentDefinedType::Variant(_) => set.contains(&ComponentAnyTypeId::from(id)),

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
            ComponentDefinedType::List(ty)
            | ComponentDefinedType::FixedSizeList(ty, _)
            | ComponentDefinedType::Option(ty) => self.type_named_valtype(ty, set),
            ComponentDefinedType::Map(k, v) => {
                self.type_named_valtype(k, set) && self.type_named_valtype(v, set)
            }

            // own/borrow themselves don't have to be named, but the resource
            // they refer to must be named.
            ComponentDefinedType::Own(id) | ComponentDefinedType::Borrow(id) => {
                set.contains(&ComponentAnyTypeId::from(*id))
            }

            ComponentDefinedType::Future(ty) => ty
                .as_ref()
                .map(|ty| self.type_named_valtype(ty, set))
                .unwrap_or(true),

            ComponentDefinedType::Stream(ty) => ty
                .as_ref()
                .map(|ty| self.type_named_valtype(ty, set))
                .unwrap_or(true),
        }
    }

    pub(crate) fn type_named_valtype(
        &self,
        ty: &ComponentValType,
        set: &Set<ComponentAnyTypeId>,
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
    ///
    /// For internal use only!
    #[doc(hidden)]
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
            ComponentDefinedType::List(ty)
            | ComponentDefinedType::FixedSizeList(ty, _)
            | ComponentDefinedType::Option(ty) => {
                any_changed |= self.remap_valtype(ty, map);
            }
            ComponentDefinedType::Map(k, v) => {
                any_changed |= self.remap_valtype(k, map);
                any_changed |= self.remap_valtype(v, map);
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
            ComponentDefinedType::Future(ty) | ComponentDefinedType::Stream(ty) => {
                if let Some(ty) = ty {
                    any_changed |= self.remap_valtype(ty, map);
                }
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
            .chain(&mut tmp.result)
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
    pub(crate) resources: Map<ResourceId, ResourceId>,

    /// A mapping filled in during the remapping process which records how a
    /// type was remapped, if applicable. This avoids remapping multiple
    /// references to the same type and instead only processing it once.
    types: Map<ComponentAnyTypeId, ComponentAnyTypeId>,
}

impl Remap for TypeAlloc {
    fn push_ty<T>(&mut self, ty: T) -> T::Id
    where
        T: TypeData,
    {
        <TypeList>::push(self, ty)
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
        T::Error: core::fmt::Debug,
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

impl<'a> SubtypeCx<'a> {
    /// Create a new instance with the specified type lists
    ///
    /// # Panics
    ///
    /// Panics if the two given `TypesRef`s are not associated with the same
    /// `Validator`.
    pub fn new_with_refs(a: TypesRef<'a>, b: TypesRef<'a>) -> SubtypeCx<'a> {
        assert_eq!(a.id(), b.id());
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
            .map(|(name, ty)| (name.clone(), *ty))
            .collect();
        self.swap();
        let mut import_mapping =
            self.open_instance_type(&b_imports, a, ExternKind::Import, offset)?;
        self.swap();
        self.with_checkpoint(|this| {
            let mut a_exports = this.a[a]
                .exports
                .iter()
                .map(|(name, ty)| (name.clone(), *ty))
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

        if a.async_ != b.async_ {
            let a_desc = if a.async_ { "async" } else { "sync" };
            let b_desc = if b.async_ { "async" } else { "sync" };
            bail!(
                offset,
                "expected {a_desc} function, found {b_desc} function",
            );
        }

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
        for ((an, a), (bn, b)) in a.params.iter().zip(b.params.iter()) {
            if an != bn {
                bail!(offset, "expected parameter named `{bn}`, found `{an}`");
            }
            self.component_val_type(a, b, offset)
                .with_context(|| format!("type mismatch in function parameter `{an}`"))?;
        }

        match (&a.result, &b.result) {
            (Some(a), Some(b)) => self
                .component_val_type(a, b, offset)
                .with_context(|| "type mismatch with result type")?,
            (None, None) => {}

            (Some(_), None) => bail!(offset, "expected a result, found none"),
            (None, Some(_)) => bail!(offset, "expected no result, found one"),
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
                Some(arg) => to_typecheck.push((*arg, *expected)),
                None => bail!(offset, "missing {} named `{name}`", kind.desc()),
            }
        }
        let mut type_map = Map::default();
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
        match (a, b) {
            (EntityType::Func(a), EntityType::Func(b))
            | (EntityType::FuncExact(a), EntityType::Func(b)) => {
                self.core_func_type(*a, *b, offset)
            }
            (EntityType::Func(_), b) => bail!(offset, "expected {}, found func", b.desc()),
            (EntityType::FuncExact(a), EntityType::FuncExact(b)) => {
                self.core_func_type(*b, *a, offset)?;
                self.core_func_type(*a, *b, offset)
            }
            (EntityType::FuncExact(_), b) => {
                bail!(offset, "expected {}, found func_exact", b.desc())
            }
            (EntityType::Table(a), EntityType::Table(b)) => Self::table_type(a, b, offset),
            (EntityType::Table(_), b) => bail!(offset, "expected {}, found table", b.desc()),
            (EntityType::Memory(a), EntityType::Memory(b)) => Self::memory_type(a, b, offset),
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
            (EntityType::Tag(a), EntityType::Tag(b)) => self.core_func_type(*a, *b, offset),
            (EntityType::Tag(_), b) => bail!(offset, "expected {}, found tag", b.desc()),
        }
    }

    pub(crate) fn table_type(a: &TableType, b: &TableType, offset: usize) -> Result<()> {
        if a.element_type != b.element_type {
            bail!(
                offset,
                "expected table element type {}, found {}",
                b.element_type,
                a.element_type,
            )
        }
        if a.shared != b.shared {
            bail!(offset, "mismatch in the shared flag for tables")
        }
        if limits_match!(a, b) {
            Ok(())
        } else {
            bail!(offset, "mismatch in table limits")
        }
    }

    pub(crate) fn memory_type(a: &MemoryType, b: &MemoryType, offset: usize) -> Result<()> {
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

    fn core_func_type(&self, a: CoreTypeId, b: CoreTypeId, offset: usize) -> Result<()> {
        debug_assert!(self.a.get(a).is_some());
        debug_assert!(self.b.get(b).is_some());
        if self.a.id_is_subtype(a, b) {
            debug_assert!(self.a.get(b).is_some());
            debug_assert!(self.b.get(a).is_some());
            Ok(())
        } else {
            bail!(
                offset,
                "expected: {}\n\
                 found:    {}",
                self.b[b],
                self.a[a],
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
            (Map(ak, av), Map(bk, bv)) => {
                self.component_val_type(ak, bk, offset)
                    .with_context(|| "type mismatch in map key")?;
                self.component_val_type(av, bv, offset)
                    .with_context(|| "type mismatch in map value")
            }
            (Map(_, _), b) => bail!(offset, "expected {}, found map", b.desc()),
            (FixedSizeList(a, asize), FixedSizeList(b, bsize)) => {
                if asize != bsize {
                    bail!(offset, "expected fixed size {bsize}, found size {asize}")
                } else {
                    self.component_val_type(a, b, offset)
                }
            }
            (FixedSizeList(_, _), b) => bail!(offset, "expected {}, found list", b.desc()),
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
            (Future(a), Future(b)) => match (a, b) {
                (None, None) => Ok(()),
                (Some(a), Some(b)) => self
                    .component_val_type(a, b, offset)
                    .with_context(|| "type mismatch in future"),
                (None, Some(_)) => bail!(offset, "expected future type, but found none"),
                (Some(_), None) => bail!(offset, "expected future type to not be present"),
            },
            (Future(_), b) => bail!(offset, "expected {}, found future", b.desc()),
            (Stream(a), Stream(b)) => match (a, b) {
                (None, None) => Ok(()),
                (Some(a), Some(b)) => self
                    .component_val_type(a, b, offset)
                    .with_context(|| "type mismatch in stream"),
                (None, Some(_)) => bail!(offset, "expected stream type, but found none"),
                (Some(_), None) => bail!(offset, "expected stream type to not be present"),
            },
            (Stream(_), b) => bail!(offset, "expected {}, found stream", b.desc()),
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
        type_map: &mut Map<ComponentAnyTypeId, ComponentAnyTypeId>,
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

    fn get<T>(&self, id: T) -> Option<&T::Data>
    where
        T: TypeIdentifier,
    {
        let index = id.index();
        if index < T::list(self.types).len() {
            self.types.get(id)
        } else {
            let temp_index = index - T::list(self.types).len();
            let temp_index = u32::try_from(temp_index).unwrap();
            let temp_id = T::from_index(temp_index);
            self.list.get(temp_id)
        }
    }

    /// Is `a == b` or was `a` declared (potentially transitively) to be a
    /// subtype of `b`?
    fn id_is_subtype(&self, a: CoreTypeId, b: CoreTypeId) -> bool {
        self.get(a).is_some() && self.get(b).is_some() && {
            // NB: we can query `self.types.id_is_subtype` directly, and ignore
            // `self.list`, because `self.list` should never contain core types.
            debug_assert!(a.index() < CoreTypeId::list(self.types).len());
            debug_assert!(b.index() < CoreTypeId::list(self.types).len());
            self.types.id_is_subtype(a, b)
        }
    }
}

impl<T> Index<T> for SubtypeArena<'_>
where
    T: TypeIdentifier,
{
    type Output = T::Data;

    fn index(&self, id: T) -> &T::Data {
        self.get(id).unwrap()
    }
}

impl Remap for SubtypeArena<'_> {
    fn push_ty<T>(&mut self, ty: T) -> T::Id
    where
        T: TypeData,
    {
        assert!(
            !T::IS_CORE_SUB_TYPE,
            "cannot push core sub types into `SubtypeArena`s, that would break type canonicalization"
        );
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
