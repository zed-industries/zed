#[cfg(feature = "gc")]
mod enabled;
#[cfg(feature = "gc")]
pub use enabled::*;

#[cfg(not(feature = "gc"))]
mod disabled;
#[cfg(not(feature = "gc"))]
pub use disabled::*;

mod func_ref;
mod gc_ref;
mod gc_runtime;
mod host_data;
mod i31;

pub use func_ref::*;
pub use gc_ref::*;
pub use gc_runtime::*;
pub use host_data::*;
pub use i31::*;

use crate::prelude::*;
use crate::runtime::vm::{GcHeapAllocationIndex, VMMemoryDefinition};
use core::any::Any;
use core::mem::MaybeUninit;
use core::{alloc::Layout, num::NonZeroU32};
use wasmtime_environ::{
    GcArrayLayout, GcExceptionLayout, GcStructLayout, VMGcKind, VMSharedTypeIndex,
};

/// GC-related data that is one-to-one with a `wasmtime::Store`.
///
/// Contains everything we need to do collections, invoke barriers, etc...
///
/// In general, exposes a very similar interface to `GcHeap`, but fills in some
/// of the context arguments for callers (such as the `ExternRefHostDataTable`)
/// since they are all stored together inside `GcStore`.
pub struct GcStore {
    /// This GC heap's allocation index (primarily used for integrating with the
    /// pooling allocator).
    pub allocation_index: GcHeapAllocationIndex,

    /// The actual GC heap.
    pub gc_heap: Box<dyn GcHeap>,

    /// The `externref` host data table for this GC heap.
    pub host_data_table: ExternRefHostDataTable,

    /// The function-references table for this GC heap.
    pub func_ref_table: FuncRefTable,
}

impl GcStore {
    /// Create a new `GcStore`.
    pub fn new(allocation_index: GcHeapAllocationIndex, gc_heap: Box<dyn GcHeap>) -> Self {
        let host_data_table = ExternRefHostDataTable::default();
        let func_ref_table = FuncRefTable::default();
        Self {
            allocation_index,
            gc_heap,
            host_data_table,
            func_ref_table,
        }
    }

    /// Get the `VMMemoryDefinition` for this GC heap.
    pub fn vmmemory_definition(&self) -> VMMemoryDefinition {
        self.gc_heap.vmmemory()
    }

    /// Perform garbage collection within this heap.
    pub fn gc(&mut self, roots: GcRootsIter<'_>) {
        let mut collection = self.gc_heap.gc(roots, &mut self.host_data_table);
        collection.collect();
    }

    /// Asynchronously perform garbage collection within this heap.
    #[cfg(feature = "async")]
    pub async fn gc_async(&mut self, roots: GcRootsIter<'_>) {
        let collection = self.gc_heap.gc(roots, &mut self.host_data_table);
        collect_async(collection).await;
    }

    /// Get the kind of the given GC reference.
    pub fn kind(&self, gc_ref: &VMGcRef) -> VMGcKind {
        debug_assert!(!gc_ref.is_i31());
        self.header(gc_ref).kind()
    }

    /// Get the header of the given GC reference.
    pub fn header(&self, gc_ref: &VMGcRef) -> &VMGcHeader {
        debug_assert!(!gc_ref.is_i31());
        self.gc_heap.header(gc_ref)
    }

    /// Clone a GC reference, calling GC write barriers as necessary.
    pub fn clone_gc_ref(&mut self, gc_ref: &VMGcRef) -> VMGcRef {
        if gc_ref.is_i31() {
            gc_ref.unchecked_copy()
        } else {
            self.gc_heap.clone_gc_ref(gc_ref)
        }
    }

    /// Write the `source` GC reference into the uninitialized `destination`
    /// slot, performing write barriers as necessary.
    pub fn init_gc_ref(
        &mut self,
        destination: &mut MaybeUninit<Option<VMGcRef>>,
        source: Option<&VMGcRef>,
    ) {
        // Initialize the destination to `None`, at which point the regular GC
        // write barrier is safe to reuse.
        let destination = destination.write(None);
        self.write_gc_ref(destination, source);
    }

    /// Write the `source` GC reference into the `destination` slot, performing
    /// write barriers as necessary.
    pub fn write_gc_ref(&mut self, destination: &mut Option<VMGcRef>, source: Option<&VMGcRef>) {
        // If neither the source nor destination actually point to a GC object
        // (that is, they are both either null or `i31ref`s) then we can skip
        // the GC barrier.
        if destination.as_ref().map_or(true, |d| d.is_i31())
            && source.as_ref().map_or(true, |s| s.is_i31())
        {
            *destination = source.map(|s| s.unchecked_copy());
            return;
        }

        self.gc_heap
            .write_gc_ref(&mut self.host_data_table, destination, source);
    }

    /// Drop the given GC reference, performing drop barriers as necessary.
    pub fn drop_gc_ref(&mut self, gc_ref: VMGcRef) {
        if !gc_ref.is_i31() {
            self.gc_heap.drop_gc_ref(&mut self.host_data_table, gc_ref);
        }
    }

    /// Hook to call whenever a GC reference is about to be exposed to Wasm.
    ///
    /// Returns the raw representation of this GC ref, ready to be passed to
    /// Wasm.
    #[must_use]
    pub fn expose_gc_ref_to_wasm(&mut self, gc_ref: VMGcRef) -> NonZeroU32 {
        let raw = gc_ref.as_raw_non_zero_u32();
        if !gc_ref.is_i31() {
            log::trace!("exposing GC ref to Wasm: {gc_ref:p}");
            self.gc_heap.expose_gc_ref_to_wasm(gc_ref);
        }
        raw
    }

    /// Allocate a new `externref`.
    ///
    /// Returns:
    ///
    /// * `Ok(Ok(_))`: Successfully allocated the `externref`.
    ///
    /// * `Ok(Err((value, n)))`: Failed to allocate the `externref`, but doing a GC
    ///   and then trying again may succeed. Returns the given `value` as the
    ///   error payload, along with the size of the failed allocation.
    ///
    /// * `Err(_)`: Unrecoverable allocation failure.
    pub fn alloc_externref(
        &mut self,
        value: Box<dyn Any + Send + Sync>,
    ) -> Result<Result<VMExternRef, (Box<dyn Any + Send + Sync>, u64)>> {
        let host_data_id = self.host_data_table.alloc(value);
        match self.gc_heap.alloc_externref(host_data_id)? {
            Ok(x) => Ok(Ok(x)),
            Err(n) => Ok(Err((self.host_data_table.dealloc(host_data_id), n))),
        }
    }

    /// Get a shared borrow of the given `externref`'s host data.
    ///
    /// Passing invalid `VMExternRef`s (eg garbage values or `externref`s
    /// associated with a different heap is memory safe but will lead to general
    /// incorrectness such as panics and wrong results.
    pub fn externref_host_data(&self, externref: &VMExternRef) -> &(dyn Any + Send + Sync) {
        let host_data_id = self.gc_heap.externref_host_data(externref);
        self.host_data_table.get(host_data_id)
    }

    /// Get a mutable borrow of the given `externref`'s host data.
    ///
    /// Passing invalid `VMExternRef`s (eg garbage values or `externref`s
    /// associated with a different heap is memory safe but will lead to general
    /// incorrectness such as panics and wrong results.
    pub fn externref_host_data_mut(
        &mut self,
        externref: &VMExternRef,
    ) -> &mut (dyn Any + Send + Sync) {
        let host_data_id = self.gc_heap.externref_host_data(externref);
        self.host_data_table.get_mut(host_data_id)
    }

    /// Allocate a raw object with the given header and layout.
    pub fn alloc_raw(
        &mut self,
        header: VMGcHeader,
        layout: Layout,
    ) -> Result<Result<VMGcRef, u64>> {
        self.gc_heap.alloc_raw(header, layout)
    }

    /// Allocate an uninitialized struct with the given type index and layout.
    ///
    /// This does NOT check that the index is currently allocated in the types
    /// registry or that the layout matches the index's type. Failure to uphold
    /// those invariants is memory safe, but will lead to general incorrectness
    /// such as panics and wrong results.
    pub fn alloc_uninit_struct(
        &mut self,
        ty: VMSharedTypeIndex,
        layout: &GcStructLayout,
    ) -> Result<Result<VMStructRef, u64>> {
        self.gc_heap.alloc_uninit_struct(ty, layout)
    }

    /// Deallocate an uninitialized struct.
    pub fn dealloc_uninit_struct(&mut self, structref: VMStructRef) {
        self.gc_heap.dealloc_uninit_struct(structref);
    }

    /// Get the data for the given object reference.
    ///
    /// Panics when the structref and its size is out of the GC heap bounds.
    pub fn gc_object_data(&mut self, gc_ref: &VMGcRef) -> &mut VMGcObjectData {
        self.gc_heap.gc_object_data_mut(gc_ref)
    }

    /// Get the object datas for the given pair of object references.
    ///
    /// Panics if `a` and `b` are the same reference or either is out of bounds.
    pub fn gc_object_data_pair(
        &mut self,
        a: &VMGcRef,
        b: &VMGcRef,
    ) -> (&mut VMGcObjectData, &mut VMGcObjectData) {
        assert_ne!(a, b);
        self.gc_heap.gc_object_data_pair(a, b)
    }

    /// Allocate an uninitialized array with the given type index.
    ///
    /// This does NOT check that the index is currently allocated in the types
    /// registry or that the layout matches the index's type. Failure to uphold
    /// those invariants is memory safe, but will lead to general incorrectness
    /// such as panics and wrong results.
    pub fn alloc_uninit_array(
        &mut self,
        ty: VMSharedTypeIndex,
        len: u32,
        layout: &GcArrayLayout,
    ) -> Result<Result<VMArrayRef, u64>> {
        self.gc_heap.alloc_uninit_array(ty, len, layout)
    }

    /// Deallocate an uninitialized array.
    pub fn dealloc_uninit_array(&mut self, arrayref: VMArrayRef) {
        self.gc_heap.dealloc_uninit_array(arrayref);
    }

    /// Get the length of the given array.
    pub fn array_len(&self, arrayref: &VMArrayRef) -> u32 {
        self.gc_heap.array_len(arrayref)
    }

    /// Allocate an uninitialized exception object with the given type
    /// index.
    ///
    /// This does NOT check that the index is currently allocated in the types
    /// registry or that the layout matches the index's type. Failure to uphold
    /// those invariants is memory safe, but will lead to general incorrectness
    /// such as panics and wrong results.
    pub fn alloc_uninit_exn(
        &mut self,
        ty: VMSharedTypeIndex,
        layout: &GcExceptionLayout,
    ) -> Result<Result<VMExnRef, u64>> {
        self.gc_heap.alloc_uninit_exn(ty, layout)
    }

    /// Deallocate an uninitialized exception object.
    pub fn dealloc_uninit_exn(&mut self, exnref: VMExnRef) {
        self.gc_heap.dealloc_uninit_exn(exnref);
    }
}
