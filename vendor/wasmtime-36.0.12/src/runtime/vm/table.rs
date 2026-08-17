//! Memory management for tables.
//!
//! `Table` is to WebAssembly tables what `LinearMemory` is to WebAssembly linear memories.

use crate::prelude::*;
use crate::runtime::vm::stack_switching::VMContObj;
use crate::runtime::vm::vmcontext::{VMFuncRef, VMTableDefinition};
use crate::runtime::vm::{GcStore, SendSyncPtr, VMGcRef, VMStore, VmPtr};
use core::alloc::Layout;
use core::mem;
use core::ops::Range;
use core::ptr::{self, NonNull};
use core::slice;
use core::{cmp, usize};
use wasmtime_environ::{
    FUNCREF_INIT_BIT, FUNCREF_MASK, IndexType, Trap, Tunables, WasmHeapTopType, WasmRefType,
};

/// An element going into or coming out of a table.
///
/// Table elements are stored as pointers and are default-initialized with
/// `ptr::null_mut`.
pub enum TableElement {
    /// A `funcref`.
    FuncRef(Option<NonNull<VMFuncRef>>),

    /// A GC reference.
    GcRef(Option<VMGcRef>),

    /// An uninitialized funcref value. This should never be exposed
    /// beyond the `wasmtime` crate boundary; the upper-level code
    /// (which has access to the info needed for lazy initialization)
    /// will replace it when fetched.
    UninitFunc,

    /// A `contref`
    ContRef(Option<VMContObj>),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TableElementType {
    Func,
    GcRef,
    Cont,
}

impl TableElementType {
    fn matches(&self, val: &TableElement) -> bool {
        match (val, self) {
            (TableElement::FuncRef(_), TableElementType::Func) => true,
            (TableElement::GcRef(_), TableElementType::GcRef) => true,
            (TableElement::ContRef(_), TableElementType::Cont) => true,
            _ => false,
        }
    }

    /// Returns the size required to actually store an element of this particular type
    pub fn element_size(&self) -> usize {
        match self {
            TableElementType::Func => core::mem::size_of::<FuncTableElem>(),
            TableElementType::GcRef => core::mem::size_of::<Option<VMGcRef>>(),
            TableElementType::Cont => core::mem::size_of::<ContTableElem>(),
        }
    }
}

// The usage of `*mut VMFuncRef` is safe w.r.t. thread safety, this just relies
// on thread-safety of `VMGcRef` itself.
unsafe impl Send for TableElement where VMGcRef: Send {}
unsafe impl Sync for TableElement where VMGcRef: Sync {}

impl TableElement {
    /// Consumes a table element into a pointer/reference, as it
    /// exists outside the table itself. This strips off any tag bits
    /// or other information that only lives inside the table.
    ///
    /// Can only be done to an initialized table element; lazy init
    /// must occur first. (In other words, lazy values do not survive
    /// beyond the table, as every table read path initializes them.)
    ///
    /// # Safety
    ///
    /// The same warnings as for `into_table_values()` apply.
    pub(crate) unsafe fn into_func_ref_asserting_initialized(self) -> Option<NonNull<VMFuncRef>> {
        match self {
            Self::FuncRef(e) => e,
            Self::UninitFunc => panic!("Uninitialized table element value outside of table slot"),
            Self::GcRef(_) => panic!("GC reference is not a function reference"),
            Self::ContRef(_) => panic!("Continuation reference is not a function reference"),
        }
    }

    /// Indicates whether this value is the "uninitialized element"
    /// value.
    pub(crate) fn is_uninit(&self) -> bool {
        match self {
            Self::UninitFunc => true,
            _ => false,
        }
    }
}

impl From<Option<NonNull<VMFuncRef>>> for TableElement {
    fn from(f: Option<NonNull<VMFuncRef>>) -> TableElement {
        TableElement::FuncRef(f)
    }
}

impl From<Option<VMGcRef>> for TableElement {
    fn from(x: Option<VMGcRef>) -> TableElement {
        TableElement::GcRef(x)
    }
}

impl From<VMGcRef> for TableElement {
    fn from(x: VMGcRef) -> TableElement {
        TableElement::GcRef(Some(x))
    }
}

impl From<Option<VMContObj>> for TableElement {
    fn from(c: Option<VMContObj>) -> TableElement {
        TableElement::ContRef(c)
    }
}

impl From<VMContObj> for TableElement {
    fn from(c: VMContObj) -> TableElement {
        TableElement::ContRef(Some(c))
    }
}

/// At-rest representation of a function in a funcref table.
///
/// Note that whether or not these pointers are tagged is a property of `Engine`
/// configuration. Also note that this specifically uses `VmPtr<T>` to handle
/// provenance here when loading/storing values to a table.
///
/// The possible values here are:
///
/// * `None` for untagged tables - a null function element
/// * `Some(_)` for untagged tables - a non-null function element
/// * `None` for tagged tables - an uninitialized element
/// * `Some(1)` for tagged tables - a null function element
/// * `Some(addr | 1)` for tagged tables - a non-null function element
#[derive(Copy, Clone)]
#[repr(transparent)]
struct MaybeTaggedFuncRef(Option<VmPtr<VMFuncRef>>);

impl MaybeTaggedFuncRef {
    const UNINIT: MaybeTaggedFuncRef = MaybeTaggedFuncRef(None);

    /// Converts the given `ptr`, a valid funcref pointer, into a tagged pointer
    /// by adding in the `FUNCREF_INIT_BIT`.
    fn from(ptr: Option<NonNull<VMFuncRef>>, lazy_init: bool) -> Self {
        let maybe_tagged = if lazy_init {
            Some(match ptr {
                Some(ptr) => ptr.map_addr(|a| a | FUNCREF_INIT_BIT),
                None => NonNull::new(core::ptr::without_provenance_mut(FUNCREF_INIT_BIT)).unwrap(),
            })
        } else {
            ptr
        };
        MaybeTaggedFuncRef(maybe_tagged.map(Into::into))
    }

    /// Converts a tagged pointer into a `TableElement`, returning `UninitFunc`
    /// for null (not a tagged value) or `FuncRef` for otherwise tagged values.
    fn into_table_element(self, lazy_init: bool) -> TableElement {
        let ptr = self.0;
        if lazy_init && ptr.is_none() {
            TableElement::UninitFunc
        } else {
            // Masking off the tag bit is harmless whether the table uses lazy
            // init or not.
            let unmasked =
                ptr.and_then(|ptr| NonNull::new(ptr.as_ptr().map_addr(|a| a & FUNCREF_MASK)));
            TableElement::FuncRef(unmasked)
        }
    }
}

pub type FuncTableElem = Option<SendSyncPtr<VMFuncRef>>;
pub type ContTableElem = Option<VMContObj>;

/// The maximum of the sizes of any of the table element types
#[cfg(feature = "pooling-allocator")]
pub const NOMINAL_MAX_TABLE_ELEM_SIZE: usize = {
    // ContTableElem intentionally excluded for "nominal" calculation.
    let sizes = [
        core::mem::size_of::<FuncTableElem>(),
        core::mem::size_of::<Option<VMGcRef>>(),
    ];

    // This is equivalent to `|data| {data.iter().reduce(std::cmp::max).unwrap()}`,
    // but as a `const` function, so we can use it to define a constant.
    const fn slice_max(data: &[usize]) -> usize {
        match data {
            [] => 0,
            [head, tail @ ..] => {
                let tail_max = slice_max(tail);
                if *head >= tail_max { *head } else { tail_max }
            }
        }
    }

    slice_max(&sizes)
};

pub enum StaticTable {
    Func(StaticFuncTable),
    GcRef(StaticGcRefTable),
    Cont(StaticContTable),
}

impl From<StaticFuncTable> for StaticTable {
    fn from(value: StaticFuncTable) -> Self {
        Self::Func(value)
    }
}

impl From<StaticGcRefTable> for StaticTable {
    fn from(value: StaticGcRefTable) -> Self {
        Self::GcRef(value)
    }
}

impl From<StaticContTable> for StaticTable {
    fn from(value: StaticContTable) -> Self {
        Self::Cont(value)
    }
}

pub struct StaticFuncTable {
    /// Where data for this table is stored. The length of this list is the
    /// maximum size of the table.
    data: SendSyncPtr<[FuncTableElem]>,
    /// The current size of the table.
    size: usize,
    /// Whether elements of this table are initialized lazily.
    lazy_init: bool,
}

pub struct StaticGcRefTable {
    /// Where data for this table is stored. The length of this list is the
    /// maximum size of the table.
    data: SendSyncPtr<[Option<VMGcRef>]>,
    /// The current size of the table.
    size: usize,
}

pub struct StaticContTable {
    /// Where data for this table is stored. The length of this list is the
    /// maximum size of the table.
    data: SendSyncPtr<[ContTableElem]>,
    /// The current size of the table.
    size: usize,
}

pub enum DynamicTable {
    Func(DynamicFuncTable),
    GcRef(DynamicGcRefTable),
    Cont(DynamicContTable),
}

impl From<DynamicFuncTable> for DynamicTable {
    fn from(value: DynamicFuncTable) -> Self {
        Self::Func(value)
    }
}

impl From<DynamicGcRefTable> for DynamicTable {
    fn from(value: DynamicGcRefTable) -> Self {
        Self::GcRef(value)
    }
}

impl From<DynamicContTable> for DynamicTable {
    fn from(value: DynamicContTable) -> Self {
        Self::Cont(value)
    }
}

pub struct DynamicFuncTable {
    /// Dynamically managed storage space for this table. The length of this
    /// vector is the current size of the table.
    elements: Vec<FuncTableElem>,
    /// Maximum size that `elements` can grow to.
    maximum: Option<usize>,
    /// Whether elements of this table are initialized lazily.
    lazy_init: bool,
}

pub struct DynamicGcRefTable {
    /// Dynamically managed storage space for this table. The length of this
    /// vector is the current size of the table.
    elements: Vec<Option<VMGcRef>>,
    /// Maximum size that `elements` can grow to.
    maximum: Option<usize>,
}

pub struct DynamicContTable {
    /// Dynamically managed storage space for this table. The length of this
    /// vector is the current size of the table.
    elements: Vec<ContTableElem>,
    /// Maximum size that `elements` can grow to.
    maximum: Option<usize>,
}

/// Represents an instance's table.
pub enum Table {
    /// A "static" table where storage space is managed externally, currently
    /// used with the pooling allocator.
    Static(StaticTable),
    /// A "dynamic" table where table storage space is dynamically allocated via
    /// `malloc` (aka Rust's `Vec`).
    Dynamic(DynamicTable),
}

impl From<StaticTable> for Table {
    fn from(value: StaticTable) -> Self {
        Self::Static(value)
    }
}

impl From<StaticFuncTable> for Table {
    fn from(value: StaticFuncTable) -> Self {
        let t: StaticTable = value.into();
        t.into()
    }
}

impl From<StaticGcRefTable> for Table {
    fn from(value: StaticGcRefTable) -> Self {
        let t: StaticTable = value.into();
        t.into()
    }
}

impl From<StaticContTable> for Table {
    fn from(value: StaticContTable) -> Self {
        let t: StaticTable = value.into();
        t.into()
    }
}

impl From<DynamicTable> for Table {
    fn from(value: DynamicTable) -> Self {
        Self::Dynamic(value)
    }
}

impl From<DynamicFuncTable> for Table {
    fn from(value: DynamicFuncTable) -> Self {
        let t: DynamicTable = value.into();
        t.into()
    }
}

impl From<DynamicGcRefTable> for Table {
    fn from(value: DynamicGcRefTable) -> Self {
        let t: DynamicTable = value.into();
        t.into()
    }
}

impl From<DynamicContTable> for Table {
    fn from(value: DynamicContTable) -> Self {
        let t: DynamicTable = value.into();
        t.into()
    }
}

pub(crate) fn wasm_to_table_type(ty: WasmRefType) -> TableElementType {
    match ty.heap_type.top() {
        WasmHeapTopType::Func => TableElementType::Func,
        WasmHeapTopType::Any | WasmHeapTopType::Extern => TableElementType::GcRef,
        WasmHeapTopType::Cont => TableElementType::Cont,
        WasmHeapTopType::Exn => TableElementType::GcRef,
    }
}

/// Allocate dynamic table elements of the given length.
///
/// Relies on the fact that our tables' elements are initialized to `None`,
/// which is represented by zero, to allocate pre-zeroed memory from the global
/// allocator and avoid manual zero-initialization.
///
/// # Safety
///
/// Should only ever be called with a `T` that is a table element type and where
/// `Option<T>`'s `None` variant is represented with zero.
unsafe fn alloc_dynamic_table_elements<T>(len: usize) -> Result<Vec<Option<T>>> {
    debug_assert!(
        unsafe {
            core::mem::MaybeUninit::<Option<T>>::zeroed()
                .assume_init()
                .is_none()
        },
        "null table elements are represented with zeroed memory"
    );

    if len == 0 {
        return Ok(vec![]);
    }

    let align = mem::align_of::<Option<T>>();

    let size = mem::size_of::<Option<T>>();
    let size = size.next_multiple_of(align);
    let size = size
        .checked_mul(len)
        .ok_or_else(|| format_err!("overflow calculating table allocation size"))?;

    let layout = Layout::from_size_align(size, align)?;

    let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
    ensure!(!ptr.is_null(), "failed to allocate memory for table");

    let elems = unsafe { Vec::<Option<T>>::from_raw_parts(ptr.cast(), len, len) };
    debug_assert!(elems.iter().all(|e| e.is_none()));

    Ok(elems)
}

impl Table {
    /// Create a new dynamic (movable) table instance for the specified table plan.
    pub fn new_dynamic(
        ty: &wasmtime_environ::Table,
        tunables: &Tunables,
        store: &mut dyn VMStore,
    ) -> Result<Self> {
        let (minimum, maximum) = Self::limit_new(ty, store)?;
        match wasm_to_table_type(ty.ref_type) {
            TableElementType::Func => Ok(Self::from(DynamicFuncTable {
                elements: unsafe { alloc_dynamic_table_elements(minimum)? },
                maximum,
                lazy_init: tunables.table_lazy_init,
            })),
            TableElementType::GcRef => Ok(Self::from(DynamicGcRefTable {
                elements: unsafe { alloc_dynamic_table_elements(minimum)? },
                maximum,
            })),
            TableElementType::Cont => Ok(Self::from(DynamicContTable {
                elements: vec![None; minimum],
                maximum,
            })),
        }
    }

    /// Create a new static (immovable) table instance for the specified table plan.
    pub unsafe fn new_static(
        ty: &wasmtime_environ::Table,
        tunables: &Tunables,
        data: SendSyncPtr<[u8]>,
        store: &mut dyn VMStore,
    ) -> Result<Self> {
        let (minimum, maximum) = Self::limit_new(ty, store)?;
        let size = minimum;
        let max = maximum.unwrap_or(usize::MAX);

        match wasm_to_table_type(ty.ref_type) {
            TableElementType::Func => {
                let len = {
                    let (before, data, after) = unsafe {
                        let data = data.as_non_null().as_ref();
                        data.align_to::<FuncTableElem>()
                    };
                    assert!(before.is_empty());
                    assert!(after.is_empty());
                    data.len()
                };
                ensure!(
                    usize::try_from(ty.limits.min).unwrap() <= len,
                    "initial table size of {} exceeds the pooling allocator's \
                     configured maximum table size of {len} elements",
                    ty.limits.min,
                );
                let data = SendSyncPtr::new(NonNull::slice_from_raw_parts(
                    data.as_non_null().cast::<FuncTableElem>(),
                    cmp::min(len, max),
                ));
                Ok(Self::from(StaticFuncTable {
                    data,
                    size,
                    lazy_init: tunables.table_lazy_init,
                }))
            }
            TableElementType::GcRef => {
                let len = {
                    let (before, data, after) = unsafe {
                        let data = data.as_non_null().as_ref();
                        data.align_to::<Option<VMGcRef>>()
                    };
                    assert!(before.is_empty());
                    assert!(after.is_empty());
                    data.len()
                };
                ensure!(
                    usize::try_from(ty.limits.min).unwrap() <= len,
                    "initial table size of {} exceeds the pooling allocator's \
                     configured maximum table size of {len} elements",
                    ty.limits.min,
                );
                let data = SendSyncPtr::new(NonNull::slice_from_raw_parts(
                    data.as_non_null().cast::<Option<VMGcRef>>(),
                    cmp::min(len, max),
                ));
                Ok(Self::from(StaticGcRefTable { data, size }))
            }
            TableElementType::Cont => {
                let len = {
                    let (before, data, after) = unsafe {
                        let data = data.as_non_null().as_ref();
                        data.align_to::<ContTableElem>()
                    };
                    assert!(before.is_empty());
                    assert!(after.is_empty());
                    data.len()
                };
                ensure!(
                    usize::try_from(ty.limits.min).unwrap() <= len,
                    "initial table size of {} exceeds the pooling allocator's \
                     configured maximum table size of {len} elements",
                    ty.limits.min,
                );
                let data = SendSyncPtr::new(NonNull::slice_from_raw_parts(
                    data.as_non_null().cast::<ContTableElem>(),
                    cmp::min(len, max),
                ));
                Ok(Self::from(StaticContTable { data, size }))
            }
        }
    }

    // Calls the `store`'s limiter to optionally prevent the table from being created.
    //
    // Returns the minimum and maximum size of the table if the table can be created.
    fn limit_new(
        ty: &wasmtime_environ::Table,
        store: &mut dyn VMStore,
    ) -> Result<(usize, Option<usize>)> {
        // No matter how the table limits are specified
        // The table size is limited by the host's pointer size
        let absolute_max = usize::MAX;

        // If the minimum overflows the host's pointer size, then we can't satisfy this request.
        // We defer the error to later so the `store` can be informed.
        let minimum = usize::try_from(ty.limits.min).ok();

        // The maximum size of the table is limited by:
        // * the host's pointer size.
        // * the table's maximum size if defined.
        // * if the table is 64-bit.
        let maximum = match (ty.limits.max, ty.idx_type) {
            (Some(max), _) => usize::try_from(max).ok(),
            (None, IndexType::I64) => usize::try_from(u64::MAX).ok(),
            (None, IndexType::I32) => usize::try_from(u32::MAX).ok(),
        };

        // Inform the store's limiter what's about to happen.
        if !store.table_growing(0, minimum.unwrap_or(absolute_max), maximum)? {
            bail!(
                "table minimum size of {} elements exceeds table limits",
                ty.limits.min
            );
        }

        // At this point we need to actually handle overflows, so bail out with
        // an error if we made it this far.
        let minimum = minimum.ok_or_else(|| {
            format_err!(
                "table minimum size of {} elements exceeds table limits",
                ty.limits.min
            )
        })?;
        Ok((minimum, maximum))
    }

    /// Returns the type of the elements in this table.
    pub fn element_type(&self) -> TableElementType {
        match self {
            Table::Static(StaticTable::Func(_)) | Table::Dynamic(DynamicTable::Func(_)) => {
                TableElementType::Func
            }
            Table::Static(StaticTable::GcRef(_)) | Table::Dynamic(DynamicTable::GcRef(_)) => {
                TableElementType::GcRef
            }
            Table::Static(StaticTable::Cont(_)) | Table::Dynamic(DynamicTable::Cont(_)) => {
                TableElementType::Cont
            }
        }
    }

    /// Returns whether or not the underlying storage of the table is "static".
    #[cfg(feature = "pooling-allocator")]
    pub(crate) fn is_static(&self) -> bool {
        matches!(self, Table::Static(_))
    }

    /// Returns the number of allocated elements.
    pub fn size(&self) -> usize {
        match self {
            Table::Static(StaticTable::Func(StaticFuncTable { size, .. })) => *size,
            Table::Static(StaticTable::GcRef(StaticGcRefTable { size, .. })) => *size,
            Table::Static(StaticTable::Cont(StaticContTable { size, .. })) => *size,
            Table::Dynamic(DynamicTable::Func(DynamicFuncTable { elements, .. })) => elements.len(),
            Table::Dynamic(DynamicTable::GcRef(DynamicGcRefTable { elements, .. })) => {
                elements.len()
            }
            Table::Dynamic(DynamicTable::Cont(DynamicContTable { elements, .. })) => elements.len(),
        }
    }

    /// Returns the maximum number of elements at runtime.
    ///
    /// Returns `None` if the table is unbounded.
    ///
    /// The runtime maximum may not be equal to the maximum from the table's Wasm type
    /// when it is being constrained by an instance allocator.
    pub fn maximum(&self) -> Option<usize> {
        match self {
            Table::Static(StaticTable::Cont(StaticContTable { data, .. })) => Some(data.len()),
            Table::Static(StaticTable::Func(StaticFuncTable { data, .. })) => Some(data.len()),
            Table::Static(StaticTable::GcRef(StaticGcRefTable { data, .. })) => Some(data.len()),
            Table::Dynamic(DynamicTable::Func(DynamicFuncTable { maximum, .. })) => *maximum,
            Table::Dynamic(DynamicTable::GcRef(DynamicGcRefTable { maximum, .. })) => *maximum,
            Table::Dynamic(DynamicTable::Cont(DynamicContTable { maximum, .. })) => *maximum,
        }
    }

    /// Initializes the contents of this table to the specified function
    ///
    /// # Panics
    ///
    /// Panics if the table is not a function table.
    pub fn init_func(
        &mut self,
        dst: u64,
        items: impl ExactSizeIterator<Item = Option<NonNull<VMFuncRef>>>,
    ) -> Result<(), Trap> {
        let dst = usize::try_from(dst).map_err(|_| Trap::TableOutOfBounds)?;

        let (funcrefs, lazy_init) = self.funcrefs_mut();
        let elements = funcrefs
            .get_mut(dst..)
            .and_then(|s| s.get_mut(..items.len()))
            .ok_or(Trap::TableOutOfBounds)?;

        for (item, slot) in items.zip(elements) {
            *slot = MaybeTaggedFuncRef::from(item, lazy_init);
        }
        Ok(())
    }

    /// Fill `table[dst..]` with values from `items`
    ///
    /// Returns a trap error on out-of-bounds accesses.
    pub fn init_gc_refs(
        &mut self,
        dst: u64,
        items: impl ExactSizeIterator<Item = Option<VMGcRef>>,
    ) -> Result<(), Trap> {
        let dst = usize::try_from(dst).map_err(|_| Trap::TableOutOfBounds)?;

        let elements = self
            .gc_refs_mut()
            .get_mut(dst..)
            .and_then(|s| s.get_mut(..items.len()))
            .ok_or(Trap::TableOutOfBounds)?;

        for (item, slot) in items.zip(elements) {
            *slot = item;
        }
        Ok(())
    }

    /// Fill `table[dst..dst + len]` with `val`.
    ///
    /// Returns a trap error on out-of-bounds accesses.
    ///
    /// # Panics
    ///
    /// Panics if `val` does not have a type that matches this table, or if
    /// `gc_store.is_none()` and this is a table of GC references.
    pub fn fill(
        &mut self,
        mut gc_store: Option<&mut GcStore>,
        dst: u64,
        val: TableElement,
        len: u64,
    ) -> Result<(), Trap> {
        let start = usize::try_from(dst).map_err(|_| Trap::TableOutOfBounds)?;
        let len = usize::try_from(len).map_err(|_| Trap::TableOutOfBounds)?;
        let end = start
            .checked_add(len)
            .ok_or_else(|| Trap::TableOutOfBounds)?;

        if end > self.size() {
            return Err(Trap::TableOutOfBounds);
        }

        match val {
            TableElement::FuncRef(f) => {
                let (funcrefs, lazy_init) = self.funcrefs_mut();
                funcrefs[start..end].fill(MaybeTaggedFuncRef::from(f, lazy_init));
            }
            TableElement::GcRef(r) => {
                // Clone the init GC reference into each table slot.
                for slot in &mut self.gc_refs_mut()[start..end] {
                    match gc_store.as_deref_mut() {
                        Some(s) => s.write_gc_ref(slot, r.as_ref()),
                        None => {
                            debug_assert!(slot.as_ref().is_none_or(|x| x.is_i31()));
                            debug_assert!(r.as_ref().is_none_or(|r| r.is_i31()));
                            *slot = r.as_ref().map(|r| r.copy_i31());
                        }
                    }
                }

                // Drop the init GC reference, since we aren't holding onto this
                // reference anymore, only the clones in the table.
                if let Some(r) = r {
                    match gc_store {
                        Some(s) => s.drop_gc_ref(r),
                        None => debug_assert!(r.is_i31()),
                    }
                }
            }
            TableElement::UninitFunc => {
                let (funcrefs, _lazy_init) = self.funcrefs_mut();
                funcrefs[start..end].fill(MaybeTaggedFuncRef::UNINIT);
            }
            TableElement::ContRef(c) => {
                let contrefs = self.contrefs_mut();
                contrefs[start..end].fill(c);
            }
        }

        Ok(())
    }

    /// Grow table by the specified amount of elements.
    ///
    /// Returns the previous size of the table if growth is successful.
    ///
    /// Returns `None` if table can't be grown by the specified amount of
    /// elements, or if the `init_value` is the wrong kind of table element.
    ///
    /// # Panics
    ///
    /// Panics if `init_value` does not have a type that matches this table.
    ///
    /// # Unsafety
    ///
    /// Resizing the table can reallocate its internal elements buffer. This
    /// table's instance's `VMContext` has raw pointers to the elements buffer
    /// that are used by Wasm, and they need to be fixed up before we call into
    /// Wasm again. Failure to do so will result in use-after-free inside Wasm.
    ///
    /// Generally, prefer using `InstanceHandle::table_grow`, which encapsulates
    /// this unsafety.
    pub unsafe fn grow(
        &mut self,
        delta: u64,
        init_value: TableElement,
        store: &mut dyn VMStore,
    ) -> Result<Option<usize>, Error> {
        let old_size = self.size();

        // Don't try to resize the table if its size isn't changing, just return
        // success.
        if delta == 0 {
            return Ok(Some(old_size));
        }
        let delta = usize::try_from(delta).map_err(|_| Trap::TableOutOfBounds)?;

        let new_size = match old_size.checked_add(delta) {
            Some(s) => s,
            None => {
                store.table_grow_failed(format_err!("overflow calculating new table size"))?;
                return Ok(None);
            }
        };

        if !store.table_growing(old_size, new_size, self.maximum())? {
            return Ok(None);
        }

        // The WebAssembly spec requires failing a `table.grow` request if
        // it exceeds the declared limits of the table. We may have set lower
        // limits in the instance allocator as well.
        if let Some(max) = self.maximum() {
            if new_size > max {
                store.table_grow_failed(format_err!("Table maximum size exceeded"))?;
                return Ok(None);
            }
        }

        debug_assert!(self.type_matches(&init_value));

        // First resize the storage and then fill with the init value
        match self {
            Table::Static(StaticTable::Func(StaticFuncTable { data, size, .. })) => {
                unsafe {
                    debug_assert!(data.as_ref()[*size..new_size].iter().all(|x| x.is_none()));
                }
                *size = new_size;
            }
            Table::Static(StaticTable::GcRef(StaticGcRefTable { data, size })) => {
                unsafe {
                    debug_assert!(data.as_ref()[*size..new_size].iter().all(|x| x.is_none()));
                }
                *size = new_size;
            }
            Table::Static(StaticTable::Cont(StaticContTable { data, size })) => {
                unsafe {
                    debug_assert!(data.as_ref()[*size..new_size].iter().all(|x| x.is_none()));
                }
                *size = new_size;
            }

            // These calls to `resize` could move the base address of
            // `elements`. If this table's limits declare it to be fixed-size,
            // then during AOT compilation we may have promised Cranelift that
            // the table base address won't change, so it is allowed to optimize
            // loading the base address. However, in that case the above checks
            // that delta is non-zero and the new size doesn't exceed the
            // maximum mean we can't get here.
            Table::Dynamic(DynamicTable::Func(DynamicFuncTable { elements, .. })) => {
                vec_reserve_resize(elements, new_size)?;
                elements.resize(new_size, None);
            }
            Table::Dynamic(DynamicTable::GcRef(DynamicGcRefTable { elements, .. })) => {
                vec_reserve_resize(elements, new_size)?;
                elements.resize_with(new_size, || None);
            }
            Table::Dynamic(DynamicTable::Cont(DynamicContTable { elements, .. })) => {
                vec_reserve_resize(elements, new_size)?;
                elements.resize(new_size, None);
            }
        }

        self.fill(
            store.store_opaque_mut().optional_gc_store_mut(),
            u64::try_from(old_size).unwrap(),
            init_value,
            u64::try_from(delta).unwrap(),
        )
        .expect("table should not be out of bounds");

        Ok(Some(old_size))
    }

    /// Get reference to the specified element.
    ///
    /// Returns `None` if the index is out of bounds.
    ///
    /// Panics if this is a table of GC references and `gc_store` is `None`.
    pub fn get(&self, gc_store: Option<&mut GcStore>, index: u64) -> Option<TableElement> {
        let index = usize::try_from(index).ok()?;
        match self.element_type() {
            TableElementType::Func => {
                let (funcrefs, lazy_init) = self.funcrefs();
                funcrefs
                    .get(index)
                    .copied()
                    .map(|e| e.into_table_element(lazy_init))
            }
            TableElementType::GcRef => self.gc_refs().get(index).map(|r| {
                let r = r.as_ref().map(|r| match gc_store {
                    Some(s) => s.clone_gc_ref(r),
                    None => r.copy_i31(),
                });

                TableElement::GcRef(r)
            }),
            TableElementType::Cont => self
                .contrefs()
                .get(index)
                .copied()
                .map(|e| TableElement::ContRef(e)),
        }
    }

    /// Set reference to the specified element.
    ///
    /// # Errors
    ///
    /// Returns an error if `index` is out of bounds or if this table type does
    /// not match the element type.
    ///
    /// # Panics
    ///
    /// Panics if `elem` is not of the right type for this table.
    pub fn set(&mut self, index: u64, elem: TableElement) -> Result<(), ()> {
        let index: usize = index.try_into().map_err(|_| ())?;
        match elem {
            TableElement::FuncRef(f) => {
                let (funcrefs, lazy_init) = self.funcrefs_mut();
                *funcrefs.get_mut(index).ok_or(())? = MaybeTaggedFuncRef::from(f, lazy_init);
            }
            TableElement::UninitFunc => {
                let (funcrefs, _lazy_init) = self.funcrefs_mut();
                *funcrefs.get_mut(index).ok_or(())? = MaybeTaggedFuncRef::UNINIT;
            }
            TableElement::GcRef(e) => {
                *self.gc_refs_mut().get_mut(index).ok_or(())? = e;
            }
            TableElement::ContRef(c) => {
                *self.contrefs_mut().get_mut(index).ok_or(())? = c;
            }
        }
        Ok(())
    }

    /// Copy `len` elements from `self[src_index..][..len]` into
    /// `dst_table[dst_index..][..len]`.
    ///
    /// # Errors
    ///
    /// Returns an error if the range is out of bounds of either the source or
    /// destination tables.
    pub fn copy_to(
        &self,
        dst: &mut Table,
        gc_store: Option<&mut GcStore>,
        dst_index: u64,
        src_index: u64,
        len: u64,
    ) -> Result<(), Trap> {
        let (src_range, dst_range) = Table::validate_copy(self, dst, dst_index, src_index, len)?;
        Self::copy_elements(gc_store, dst, self, dst_range, src_range);
        Ok(())
    }

    /// Copy `len` elements from `self[src_index..][..len]` into
    /// `self[dst_index..][..len]`.
    ///
    /// # Errors
    ///
    /// Returns an error if the range is out of bounds of either the source or
    /// destination tables.
    pub fn copy_within(
        &mut self,
        gc_store: Option<&mut GcStore>,
        dst_index: u64,
        src_index: u64,
        len: u64,
    ) -> Result<(), Trap> {
        let (src_range, dst_range) = Table::validate_copy(self, self, dst_index, src_index, len)?;
        self.copy_elements_within(gc_store, dst_range, src_range);
        Ok(())
    }

    /// Copy `len` elements from `src_table[src_index..]` into `dst_table[dst_index..]`.
    ///
    /// # Errors
    ///
    /// Returns an error if the range is out of bounds of either the source or
    /// destination tables.
    fn validate_copy(
        src: &Table,
        dst: &Table,
        dst_index: u64,
        src_index: u64,
        len: u64,
    ) -> Result<(Range<usize>, Range<usize>), Trap> {
        // https://webassembly.github.io/bulk-memory-operations/core/exec/instructions.html#exec-table-copy

        let src_index = usize::try_from(src_index).map_err(|_| Trap::TableOutOfBounds)?;
        let dst_index = usize::try_from(dst_index).map_err(|_| Trap::TableOutOfBounds)?;
        let len = usize::try_from(len).map_err(|_| Trap::TableOutOfBounds)?;

        if src_index.checked_add(len).map_or(true, |n| n > src.size())
            || dst_index.checked_add(len).map_or(true, |m| m > dst.size())
        {
            return Err(Trap::TableOutOfBounds);
        }

        debug_assert!(
            dst.element_type() == src.element_type(),
            "table element type mismatch"
        );

        let src_range = src_index..src_index + len;
        let dst_range = dst_index..dst_index + len;

        Ok((src_range, dst_range))
    }

    /// Return a `VMTableDefinition` for exposing the table to compiled wasm code.
    pub fn vmtable(&mut self) -> VMTableDefinition {
        match self {
            Table::Static(StaticTable::Func(StaticFuncTable { data, size, .. })) => {
                VMTableDefinition {
                    base: data.cast().into(),
                    current_elements: *size,
                }
            }
            Table::Static(StaticTable::GcRef(StaticGcRefTable { data, size })) => {
                VMTableDefinition {
                    base: data.cast().into(),
                    current_elements: *size,
                }
            }
            Table::Static(StaticTable::Cont(StaticContTable { data, size })) => VMTableDefinition {
                base: data.cast().into(),
                current_elements: *size,
            },
            Table::Dynamic(DynamicTable::Func(DynamicFuncTable { elements, .. })) => {
                VMTableDefinition {
                    base: NonNull::new(elements.as_mut_ptr()).unwrap().cast().into(),
                    current_elements: elements.len(),
                }
            }
            Table::Dynamic(DynamicTable::GcRef(DynamicGcRefTable { elements, .. })) => {
                VMTableDefinition {
                    base: NonNull::new(elements.as_mut_ptr()).unwrap().cast().into(),
                    current_elements: elements.len(),
                }
            }
            Table::Dynamic(DynamicTable::Cont(DynamicContTable { elements, .. })) => {
                VMTableDefinition {
                    base: NonNull::new(elements.as_mut_ptr()).unwrap().cast().into(),
                    current_elements: elements.len(),
                }
            }
        }
    }

    fn type_matches(&self, val: &TableElement) -> bool {
        self.element_type().matches(val)
    }

    fn funcrefs(&self) -> (&[MaybeTaggedFuncRef], bool) {
        assert_eq!(self.element_type(), TableElementType::Func);
        match self {
            Self::Dynamic(DynamicTable::Func(DynamicFuncTable {
                elements,
                lazy_init,
                ..
            })) => (
                unsafe { slice::from_raw_parts(elements.as_ptr().cast(), elements.len()) },
                *lazy_init,
            ),
            Self::Static(StaticTable::Func(StaticFuncTable {
                data,
                size,
                lazy_init,
            })) => (
                unsafe { slice::from_raw_parts(data.as_ptr().cast(), *size) },
                *lazy_init,
            ),
            _ => unreachable!(),
        }
    }

    fn funcrefs_mut(&mut self) -> (&mut [MaybeTaggedFuncRef], bool) {
        assert_eq!(self.element_type(), TableElementType::Func);
        match self {
            Self::Dynamic(DynamicTable::Func(DynamicFuncTable {
                elements,
                lazy_init,
                ..
            })) => (
                unsafe { slice::from_raw_parts_mut(elements.as_mut_ptr().cast(), elements.len()) },
                *lazy_init,
            ),
            Self::Static(StaticTable::Func(StaticFuncTable {
                data,
                size,
                lazy_init,
            })) => (
                unsafe { slice::from_raw_parts_mut(data.as_ptr().cast(), *size) },
                *lazy_init,
            ),
            _ => unreachable!(),
        }
    }

    fn gc_refs(&self) -> &[Option<VMGcRef>] {
        assert_eq!(self.element_type(), TableElementType::GcRef);
        match self {
            Self::Dynamic(DynamicTable::GcRef(DynamicGcRefTable { elements, .. })) => elements,
            Self::Static(StaticTable::GcRef(StaticGcRefTable { data, size })) => unsafe {
                &data.as_non_null().as_ref()[..*size]
            },
            _ => unreachable!(),
        }
    }

    fn contrefs(&self) -> &[Option<VMContObj>] {
        assert_eq!(self.element_type(), TableElementType::Cont);
        match self {
            Self::Dynamic(DynamicTable::Cont(DynamicContTable { elements, .. })) => unsafe {
                slice::from_raw_parts(elements.as_ptr().cast(), elements.len())
            },
            Self::Static(StaticTable::Cont(StaticContTable { data, size })) => unsafe {
                slice::from_raw_parts(data.as_ptr().cast(), *size)
            },
            _ => unreachable!(),
        }
    }

    fn contrefs_mut(&mut self) -> &mut [Option<VMContObj>] {
        assert_eq!(self.element_type(), TableElementType::Cont);
        match self {
            Self::Dynamic(DynamicTable::Cont(DynamicContTable { elements, .. })) => unsafe {
                slice::from_raw_parts_mut(elements.as_mut_ptr().cast(), elements.len())
            },
            Self::Static(StaticTable::Cont(StaticContTable { data, size })) => unsafe {
                slice::from_raw_parts_mut(data.as_ptr().cast(), *size)
            },
            _ => unreachable!(),
        }
    }

    /// Get this table's GC references as a slice.
    ///
    /// Panics if this is not a table of GC references.
    pub fn gc_refs_mut(&mut self) -> &mut [Option<VMGcRef>] {
        assert_eq!(self.element_type(), TableElementType::GcRef);
        match self {
            Self::Dynamic(DynamicTable::GcRef(DynamicGcRefTable { elements, .. })) => elements,
            Self::Static(StaticTable::GcRef(StaticGcRefTable { data, size })) => unsafe {
                &mut data.as_non_null().as_mut()[..*size]
            },
            _ => unreachable!(),
        }
    }

    fn copy_elements(
        gc_store: Option<&mut GcStore>,
        dst_table: &mut Self,
        src_table: &Self,
        dst_range: Range<usize>,
        src_range: Range<usize>,
    ) {
        // This can only be used when copying between different tables
        debug_assert!(!ptr::eq(dst_table, src_table));

        let ty = dst_table.element_type();

        match ty {
            TableElementType::Func => {
                // `funcref` are `Copy`, so just do a mempcy
                let (dst_funcrefs, _lazy_init) = dst_table.funcrefs_mut();
                let (src_funcrefs, _lazy_init) = src_table.funcrefs();
                dst_funcrefs[dst_range].copy_from_slice(&src_funcrefs[src_range]);
            }
            TableElementType::GcRef => {
                assert_eq!(
                    dst_range.end - dst_range.start,
                    src_range.end - src_range.start
                );
                assert!(dst_range.end <= dst_table.gc_refs().len());
                assert!(src_range.end <= src_table.gc_refs().len());
                let gc_store = gc_store.unwrap();
                for (dst, src) in dst_range.zip(src_range) {
                    gc_store.write_gc_ref(
                        &mut dst_table.gc_refs_mut()[dst],
                        src_table.gc_refs()[src].as_ref(),
                    );
                }
            }
            TableElementType::Cont => {
                // `contref` are `Copy`, so just do a mempcy
                dst_table.contrefs_mut()[dst_range]
                    .copy_from_slice(&src_table.contrefs()[src_range]);
            }
        }
    }

    fn copy_elements_within(
        &mut self,
        gc_store: Option<&mut GcStore>,
        dst_range: Range<usize>,
        src_range: Range<usize>,
    ) {
        assert_eq!(
            dst_range.end - dst_range.start,
            src_range.end - src_range.start
        );

        // This is a no-op.
        if src_range.start == dst_range.start {
            return;
        }

        let ty = self.element_type();
        match ty {
            TableElementType::Func => {
                // `funcref` are `Copy`, so just do a memmove
                let (funcrefs, _lazy_init) = self.funcrefs_mut();
                funcrefs.copy_within(src_range, dst_range.start);
            }
            TableElementType::GcRef => {
                let gc_store = gc_store.unwrap();

                // We need to clone each `externref` while handling overlapping
                // ranges
                let elements = self.gc_refs_mut();
                if dst_range.start < src_range.start {
                    for (d, s) in dst_range.zip(src_range) {
                        let (ds, ss) = elements.split_at_mut(s);
                        let dst = &mut ds[d];
                        let src = ss[0].as_ref();
                        gc_store.write_gc_ref(dst, src);
                    }
                } else {
                    for (s, d) in src_range.rev().zip(dst_range.rev()) {
                        let (ss, ds) = elements.split_at_mut(d);
                        let dst = &mut ds[0];
                        let src = ss[s].as_ref();
                        gc_store.write_gc_ref(dst, src);
                    }
                }
            }
            TableElementType::Cont => {
                // `contref` are `Copy`, so just do a memmove
                self.contrefs_mut().copy_within(src_range, dst_range.start);
            }
        }
    }
}

// The default table representation is an empty funcref table that cannot grow.
impl Default for Table {
    fn default() -> Self {
        Self::from(StaticFuncTable {
            data: SendSyncPtr::new(NonNull::from(&mut [])),
            size: 0,
            lazy_init: false,
        })
    }
}

fn vec_reserve_resize<T>(vec: &mut Vec<T>, new_size: usize) -> Result<()> {
    if vec.len() < new_size {
        vec.try_reserve(new_size - vec.len())?;
    }
    Ok(())
}
