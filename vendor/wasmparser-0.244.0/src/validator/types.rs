//! Types relating to type information provided by validation.

use super::core::Module;
#[cfg(feature = "component-model")]
use crate::validator::component::ComponentState;
#[cfg(feature = "component-model")]
use crate::validator::component_types::{ComponentTypeAlloc, ComponentTypeList};
use crate::{AbstractHeapType, collections::map::Entry};
use crate::{CompositeInnerType, prelude::*};
use crate::{
    Export, ExternalKind, GlobalType, Import, Matches, MemoryType, PackedIndex, RecGroup, RefType,
    Result, SubType, TableType, TypeRef, UnpackedIndex, ValType, WithRecGroup,
};
use crate::{FuncType, HeapType, ValidatorId};
use alloc::sync::Arc;
use core::ops::{Deref, DerefMut, Index, Range};
use core::{hash::Hash, mem};

/// A trait shared by all type identifiers.
///
/// Any id that can be used to get a type from a `Types`.
//
// Or, internally, from a `TypeList`.
pub trait TypeIdentifier: core::fmt::Debug + Copy + Eq + Sized + 'static {
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
/// This is the data that can be retrieved by indexing with the associated
/// [`TypeIdentifier`].
pub trait TypeData: core::fmt::Debug {
    /// The identifier for this type data.
    type Id: TypeIdentifier<Data = Self>;

    /// Is this type a core sub type (or rec group of sub types)?
    const IS_CORE_SUB_TYPE: bool;

    /// Get the info for this type.
    #[doc(hidden)]
    fn type_info(&self, types: &TypeList) -> TypeInfo;
}

macro_rules! define_type_id {
    ($name:ident, $data:ty, $($list:ident).*, $type_str:expr) => {
        #[doc = "Represents a unique identifier for a "]
        #[doc = $type_str]
        #[doc = " type known to a [`crate::Validator`]."]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
                &types.$($list).*
            }

            fn list_mut(types: &mut TypeList) -> &mut SnapshotList<Self::Data> {
                &mut types.$($list).*
            }

            fn index(&self) -> usize {
                usize::try_from(self.index).unwrap()
            }
        }


        // The size of type IDs was seen to have a large-ish impact in #844, so
        // this assert ensures that it stays relatively small.
        const _: () = {
            assert!(core::mem::size_of::<$name>() <= 4);
        };
    };
}
#[cfg(feature = "component-model")]
pub(crate) use define_type_id;

/// Represents a unique identifier for a core type type known to a
/// [`crate::Validator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct CoreTypeId {
    index: u32,
}

#[test]
fn assert_core_type_id_small() {
    assert!(core::mem::size_of::<CoreTypeId>() <= 4);
}

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
    const IS_CORE_SUB_TYPE: bool = true;
    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        // TODO(#1036): calculate actual size for func, array, struct.
        let size = 1 + match &self.composite_type.inner {
            CompositeInnerType::Func(ty) => 1 + (ty.params().len() + ty.results().len()) as u32,
            CompositeInnerType::Array(_) => 2,
            CompositeInnerType::Struct(ty) => 1 + 2 * ty.fields.len() as u32,
            CompositeInnerType::Cont(_) => 1,
        };
        TypeInfo::core(size)
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
    const IS_CORE_SUB_TYPE: bool = true;
    fn type_info(&self, _types: &TypeList) -> TypeInfo {
        let size = self.end.index() - self.start.index();
        TypeInfo::core(u32::try_from(size).unwrap())
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[cfg(feature = "component-model")]
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
    #[cfg(feature = "component-model")]
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

    #[cfg(feature = "component-model")]
    pub(crate) fn contains_borrow(&self) -> bool {
        (self.0 >> 31) != 0
    }
}

/// The entity type for imports and exports of a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// The entity is a function with exact type.
    FuncExact(CoreTypeId),
}

impl EntityType {
    #[cfg(feature = "component-model")]
    pub(crate) fn desc(&self) -> &'static str {
        match self {
            Self::Func(_) => "func",
            Self::Table(_) => "table",
            Self::Memory(_) => "memory",
            Self::Global(_) => "global",
            Self::Tag(_) => "tag",
            Self::FuncExact(_) => "func_exact",
        }
    }

    pub(crate) fn info(&self, types: &TypeList) -> TypeInfo {
        match self {
            Self::Func(id) | Self::Tag(id) | Self::FuncExact(id) => types[*id].type_info(types),
            Self::Table(_) | Self::Memory(_) | Self::Global(_) => TypeInfo::new(),
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub(super) enum TypesKind {
    Module(Arc<Module>),
    #[cfg(feature = "component-model")]
    Component(ComponentState),
}

/// Represents the types known to a [`crate::Validator`] once validation has completed.
///
/// The type information is returned via the [`crate::Validator::end`] method.
pub struct Types {
    id: ValidatorId,
    pub(super) list: TypeList,
    pub(super) kind: TypesKind,
}

#[derive(Clone, Copy)]
pub(super) enum TypesRefKind<'a> {
    Module(&'a Module),
    #[cfg(feature = "component-model")]
    Component(&'a ComponentState),
}

/// Represents the types known to a [`crate::Validator`] during validation.
///
/// Retrieved via the [`crate::Validator::types`] method.
#[derive(Clone, Copy)]
pub struct TypesRef<'a> {
    id: ValidatorId,
    pub(super) list: &'a TypeList,
    pub(super) kind: TypesRefKind<'a>,
}

impl<'a> TypesRef<'a> {
    pub(crate) fn from_module(id: ValidatorId, types: &'a TypeList, module: &'a Module) -> Self {
        Self {
            id,
            list: types,
            kind: TypesRefKind::Module(module),
        }
    }

    #[cfg(feature = "component-model")]
    pub(crate) fn from_component(
        id: ValidatorId,
        types: &'a TypeList,
        component: &'a ComponentState,
    ) -> Self {
        Self {
            id,
            list: types,
            kind: TypesRefKind::Component(component),
        }
    }

    /// Get the id of the validator that these types are associated with.
    #[inline]
    pub fn id(&self) -> ValidatorId {
        self.id
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

    /// Get the id of the rec group that the given type id was defined within.
    pub fn rec_group_id_of(&self, id: CoreTypeId) -> RecGroupId {
        self.list.rec_group_id_of(id)
    }

    /// Get the types within a rec group.
    pub fn rec_group_elements(&self, id: RecGroupId) -> impl ExactSizeIterator<Item = CoreTypeId> {
        let range = &self.list.rec_group_elements[id];
        (range.start.index..range.end.index).map(|index| CoreTypeId { index })
    }

    /// Get the super type of the given type id, if any.
    pub fn supertype_of(&self, id: CoreTypeId) -> Option<CoreTypeId> {
        self.list.supertype_of(id)
    }

    /// Gets a core WebAssembly type id from a type index.
    ///
    /// Note that this is not to be confused with
    /// [`TypesRef::component_type_at`] which gets a component type from its
    /// index, nor [`TypesRef::core_type_at_in_component`] which is for
    /// learning about core types in components.
    ///
    /// # Panics
    ///
    /// This will panic if the `index` provided is out of bounds.
    pub fn core_type_at_in_module(&self, index: u32) -> CoreTypeId {
        match &self.kind {
            TypesRefKind::Module(module) => module.types[index as usize].into(),
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(_) => panic!("use `core_type_at_in_component` instead"),
        }
    }

    /// Returns the number of core types defined so far.
    ///
    /// Note that this is only for core modules, for components you should use
    /// [`TypesRef::core_type_count_in_component`] instead.
    pub fn core_type_count_in_module(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.types.len() as u32,
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(_) => 0,
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
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(component) => &component.core_tables,
        };
        tables[index as usize]
    }

    /// Returns the number of tables defined so far.
    pub fn table_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.tables.len() as u32,
            #[cfg(feature = "component-model")]
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
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(component) => &component.core_memories,
        };

        memories[index as usize]
    }

    /// Returns the number of memories defined so far.
    pub fn memory_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.memories.len() as u32,
            #[cfg(feature = "component-model")]
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
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(component) => &component.core_globals,
        };

        globals[index as usize]
    }

    /// Returns the number of globals defined so far.
    pub fn global_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.globals.len() as u32,
            #[cfg(feature = "component-model")]
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
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(component) => &component.core_tags,
        };
        tags[index as usize]
    }

    /// Returns the number of tags defined so far.
    pub fn tag_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.tags.len() as u32,
            #[cfg(feature = "component-model")]
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
            #[cfg(feature = "component-model")]
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
            #[cfg(feature = "component-model")]
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
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(_) => {
                panic!("no elements on a component")
            }
        }
    }

    /// Returns the number of elements defined so far.
    pub fn element_count(&self) -> u32 {
        match &self.kind {
            TypesRefKind::Module(module) => module.element_types.len() as u32,
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(_) => 0,
        }
    }

    /// Gets the entity type for the given import.
    pub fn entity_type_from_import(&self, import: &Import) -> Option<EntityType> {
        match &self.kind {
            TypesRefKind::Module(module) => Some(match import.ty {
                TypeRef::Func(idx) => EntityType::Func(*module.types.get(idx as usize)?),
                TypeRef::FuncExact(idx) => EntityType::FuncExact(*module.types.get(idx as usize)?),
                TypeRef::Table(ty) => EntityType::Table(ty),
                TypeRef::Memory(ty) => EntityType::Memory(ty),
                TypeRef::Global(ty) => EntityType::Global(ty),
                TypeRef::Tag(ty) => EntityType::Tag(*module.types.get(ty.func_type_idx as usize)?),
            }),
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(_) => None,
        }
    }

    /// Gets the entity type from the given export.
    pub fn entity_type_from_export(&self, export: &Export) -> Option<EntityType> {
        match &self.kind {
            TypesRefKind::Module(module) => Some(match export.kind {
                ExternalKind::Func | ExternalKind::FuncExact => EntityType::Func(
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
                ExternalKind::Tag => EntityType::Tag(*module.tags.get(export.index as usize)?),
            }),
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(_) => None,
        }
    }

    /// Returns an iterator over the core wasm imports found.
    ///
    /// Returns `None` if this type information is for a component.
    pub fn core_imports(
        &self,
    ) -> Option<impl Iterator<Item = (&'a str, &'a str, EntityType)> + 'a> {
        match &self.kind {
            TypesRefKind::Module(module) => Some(
                module
                    .imports
                    .iter()
                    .flat_map(|((m, n), t)| t.iter().map(move |t| (m.as_str(), n.as_str(), *t))),
            ),
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(_) => None,
        }
    }

    /// Returns an iterator over the core wasm exports found.
    ///
    /// Returns `None` if this type information is for a component.
    pub fn core_exports(&self) -> Option<impl Iterator<Item = (&'a str, EntityType)> + 'a> {
        match &self.kind {
            TypesRefKind::Module(module) => {
                Some(module.exports.iter().map(|(n, t)| (n.as_str(), *t)))
            }
            #[cfg(feature = "component-model")]
            TypesRefKind::Component(_) => None,
        }
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
    pub(crate) fn from_module(id: ValidatorId, types: TypeList, module: Arc<Module>) -> Self {
        Self {
            id,
            list: types,
            kind: TypesKind::Module(module),
        }
    }

    #[cfg(feature = "component-model")]
    pub(crate) fn from_component(
        id: ValidatorId,
        types: TypeList,
        component: ComponentState,
    ) -> Self {
        Self {
            id,
            list: types,
            kind: TypesKind::Component(component),
        }
    }

    /// Return a [`TypesRef`] through which types can be inspected.
    pub fn as_ref(&self) -> TypesRef<'_> {
        TypesRef {
            id: self.id,
            list: &self.list,
            kind: match &self.kind {
                TypesKind::Module(module) => TypesRefKind::Module(module),
                #[cfg(feature = "component-model")]
                TypesKind::Component(component) => TypesRefKind::Component(component),
            },
        }
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
#[derive(Debug)]
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

#[derive(Debug)]
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
    #[cfg(feature = "component-model")]
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
        match self.get(index) {
            Some(x) => x,
            None => panic!(
                "out-of-bounds indexing into `SnapshotList`: index is {index}, but length is {}",
                self.len()
            ),
        }
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
#[derive(Default, Debug)]
// Only public because it shows up in a public trait's `doc(hidden)` method.
#[doc(hidden)]
pub struct TypeList {
    // Core Wasm types.
    //
    // A primary map from `CoreTypeId` to `SubType`.
    pub(super) core_types: SnapshotList<SubType>,
    // The id of each core Wasm type's rec group.
    //
    // A secondary map from `CoreTypeId` to `RecGroupId`.
    pub(super) core_type_to_rec_group: SnapshotList<RecGroupId>,
    // The supertype of each core type.
    //
    // A secondary map from `CoreTypeId` to `Option<CoreTypeId>`.
    pub(super) core_type_to_supertype: SnapshotList<Option<CoreTypeId>>,
    // The subtyping depth of each core type. We use `u8::MAX` as a sentinel for
    // an uninitialized entry.
    //
    // A secondary map from `CoreTypeId` to `u8`.
    pub(super) core_type_to_depth: Option<IndexMap<CoreTypeId, u8>>,
    // A primary map from `RecGroupId` to the range of the rec group's elements
    // within `core_types`.
    pub(super) rec_group_elements: SnapshotList<Range<CoreTypeId>>,
    // A hash map from rec group elements to their canonical `RecGroupId`.
    //
    // This is `None` when a list is "committed" meaning that no more insertions
    // can happen.
    pub(super) canonical_rec_groups: Option<Map<RecGroup, RecGroupId>>,

    #[cfg(feature = "component-model")]
    pub(super) component: ComponentTypeList,
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
    ///
    /// If the `needs_type_canonicalization` flag is provided then the type will
    /// be intern'd here and its indices will be canonicalized to `CoreTypeId`
    /// from the previous `RecGroup`-based indices.
    ///
    /// If the `needs_type_canonicalization` flag is `false` then it must be
    /// required that `RecGroup` doesn't have any rec-group-relative references
    /// and it will additionally not be intern'd.
    pub fn intern_canonical_rec_group(
        &mut self,
        needs_type_canonicalization: bool,
        mut rec_group: RecGroup,
    ) -> (bool, RecGroupId) {
        let rec_group_id = self.rec_group_elements.len();
        let rec_group_id = u32::try_from(rec_group_id).unwrap();
        let rec_group_id = RecGroupId::from_index(rec_group_id);

        if needs_type_canonicalization {
            let canonical_rec_groups = self
                .canonical_rec_groups
                .as_mut()
                .expect("cannot intern into a committed list");
            let entry = match canonical_rec_groups.entry(rec_group) {
                Entry::Occupied(e) => return (false, *e.get()),
                Entry::Vacant(e) => e,
            };
            rec_group = entry.key().clone();
            entry.insert(rec_group_id);
        }

        let start = self.core_types.len();
        let start = u32::try_from(start).unwrap();
        let start = CoreTypeId::from_index(start);

        for mut ty in rec_group.into_types() {
            debug_assert_eq!(self.core_types.len(), self.core_type_to_supertype.len());
            debug_assert_eq!(self.core_types.len(), self.core_type_to_rec_group.len());

            self.core_type_to_supertype
                .push(ty.supertype_idx.and_then(|idx| match idx.unpack() {
                    UnpackedIndex::RecGroup(offset) => {
                        Some(CoreTypeId::from_index(start.index + offset))
                    }
                    UnpackedIndex::Id(id) => Some(id),
                    // Only invalid wasm has this, at this point, so defer the
                    // error to later.
                    UnpackedIndex::Module(_) => None,
                }));
            ty.remap_indices(&mut |index| {
                // Note that `UnpackedIndex::Id` is unmodified and
                // `UnpackedIndex::Module` means that this is invalid wasm which
                // will get an error returned later.
                if let UnpackedIndex::RecGroup(offset) = index.unpack() {
                    *index = UnpackedIndex::Id(CoreTypeId::from_index(start.index + offset))
                        .pack()
                        .unwrap();
                }
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

        return (true, rec_group_id);
    }

    /// Helper for interning a sub type as a rec group; see
    /// [`Self::intern_canonical_rec_group`].
    pub fn intern_sub_type(&mut self, sub_ty: SubType, offset: usize) -> CoreTypeId {
        let (_is_new, group_id) =
            self.intern_canonical_rec_group(true, RecGroup::implicit(offset, sub_ty));
        self[group_id].start
    }

    /// Helper for interning a function type as a rec group; see
    /// [`Self::intern_sub_type`].
    pub fn intern_func_type(&mut self, ty: FuncType, offset: usize) -> CoreTypeId {
        self.intern_sub_type(SubType::func(ty, false), offset)
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

    /// Get the subtyping depth of the given type. A type without any supertype
    /// has depth 0.
    pub fn get_subtyping_depth(&self, id: CoreTypeId) -> u8 {
        let depth = self
            .core_type_to_depth
            .as_ref()
            .expect("cannot get subtype depth from a committed list")[&id];
        debug_assert!(usize::from(depth) <= crate::limits::MAX_WASM_SUBTYPING_DEPTH);
        depth
    }

    /// Set the subtyping depth of the given type. This may only be done once
    /// per type.
    pub fn set_subtyping_depth(&mut self, id: CoreTypeId, depth: u8) {
        debug_assert!(usize::from(depth) <= crate::limits::MAX_WASM_SUBTYPING_DEPTH);
        let map = self
            .core_type_to_depth
            .as_mut()
            .expect("cannot set a subtype depth in a committed list");
        debug_assert!(!map.contains_key(&id));
        map.insert(id, depth);
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

        use AbstractHeapType::*;
        use CompositeInnerType as CT;
        use HeapType as HT;
        match (a.heap_type(), b.heap_type()) {
            (a, b) if a == b => true,

            (
                HT::Abstract {
                    shared: a_shared,
                    ty: a_ty,
                },
                HT::Abstract {
                    shared: b_shared,
                    ty: b_ty,
                },
            ) => a_shared == b_shared && a_ty.is_subtype_of(b_ty),

            (HT::Concrete(a), HT::Abstract { shared, ty })
            | (HT::Exact(a), HT::Abstract { shared, ty }) => {
                let a_ty = &subtype(a_group, a).composite_type;
                if a_ty.shared != shared {
                    return false;
                }
                match ty {
                    Any | Eq => matches!(a_ty.inner, CT::Array(_) | CT::Struct(_)),
                    Struct => matches!(a_ty.inner, CT::Struct(_)),
                    Array => matches!(a_ty.inner, CT::Array(_)),
                    Func => matches!(a_ty.inner, CT::Func(_)),
                    Cont => matches!(a_ty.inner, CT::Cont(_)),
                    // Nothing else matches. (Avoid full wildcard matches so
                    // that adding/modifying variants is easier in the future.)
                    Extern | Exn | I31 | None | NoFunc | NoExtern | NoExn | NoCont => false,
                }
            }

            (HT::Abstract { shared, ty }, HT::Concrete(b))
            | (HT::Abstract { shared, ty }, HT::Exact(b)) => {
                let b_ty = &subtype(b_group, b).composite_type;
                if shared != b_ty.shared {
                    return false;
                }
                match ty {
                    None => matches!(b_ty.inner, CT::Array(_) | CT::Struct(_)),
                    NoFunc => matches!(b_ty.inner, CT::Func(_)),
                    NoCont => matches!(b_ty.inner, CT::Cont(_)),
                    // Nothing else matches. (Avoid full wildcard matches so
                    // that adding/modifying variants is easier in the future.)
                    Cont | Func | Extern | Exn | Any | Eq | Array | I31 | Struct | NoExtern
                    | NoExn => false,
                }
            }

            (HT::Concrete(a), HT::Concrete(b)) | (HT::Exact(a), HT::Concrete(b)) => {
                self.id_is_subtype(core_type_id(a_group, a), core_type_id(b_group, b))
            }

            (HT::Exact(a), HT::Exact(b)) => core_type_id(a_group, a) == core_type_id(b_group, b),

            (HT::Concrete(_), HT::Exact(_)) => false,
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

    /// Is `ty` shared?
    pub fn valtype_is_shared(&self, ty: ValType) -> bool {
        match ty {
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => true,
            ValType::Ref(rt) => self.reftype_is_shared(rt),
        }
    }

    /// Is the reference type `ty` shared?
    ///
    /// This is complicated by concrete heap types whose shared-ness must be
    /// checked by looking at the type they point to.
    pub fn reftype_is_shared(&self, ty: RefType) -> bool {
        match ty.heap_type() {
            HeapType::Abstract { shared, .. } => shared,
            HeapType::Concrete(index) | HeapType::Exact(index) => {
                self[index.as_core_type_id().unwrap()].composite_type.shared
            }
        }
    }

    /// Get the top type of the given heap type.
    ///
    /// Concrete types must have had their indices canonicalized to core type
    /// ids, otherwise this method will panic.
    pub fn top_type(&self, heap_type: &HeapType) -> HeapType {
        use AbstractHeapType::*;
        match *heap_type {
            HeapType::Concrete(idx) | HeapType::Exact(idx) => {
                let ty = &self[idx.as_core_type_id().unwrap()].composite_type;
                let shared = ty.shared;
                match ty.inner {
                    CompositeInnerType::Func(_) => HeapType::Abstract { shared, ty: Func },
                    CompositeInnerType::Array(_) | CompositeInnerType::Struct(_) => {
                        HeapType::Abstract { shared, ty: Any }
                    }
                    CompositeInnerType::Cont(_) => HeapType::Abstract { shared, ty: Cont },
                }
            }
            HeapType::Abstract { shared, ty } => {
                let ty = match ty {
                    Func | NoFunc => Func,
                    Extern | NoExtern => Extern,
                    Any | Eq | Struct | Array | I31 | None => Any,
                    Exn | NoExn => Exn,
                    Cont | NoCont => Cont,
                };
                HeapType::Abstract { shared, ty }
            }
        }
    }

    pub fn commit(&mut self) -> TypeList {
        TypeList {
            core_types: self.core_types.commit(),
            core_type_to_rec_group: self.core_type_to_rec_group.commit(),
            core_type_to_supertype: self.core_type_to_supertype.commit(),
            core_type_to_depth: None,
            rec_group_elements: self.rec_group_elements.commit(),
            canonical_rec_groups: None,
            #[cfg(feature = "component-model")]
            component: self.component.commit(),
        }
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
    #[cfg(feature = "component-model")]
    pub(super) component_alloc: ComponentTypeAlloc,
}

impl Default for TypeAlloc {
    fn default() -> TypeAlloc {
        let mut ret = TypeAlloc {
            list: TypeList::default(),
            #[cfg(feature = "component-model")]
            component_alloc: ComponentTypeAlloc::default(),
        };
        ret.list.core_type_to_depth = Some(Default::default());
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
