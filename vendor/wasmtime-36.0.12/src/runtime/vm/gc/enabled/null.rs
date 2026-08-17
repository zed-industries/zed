//! The null collector.
//!
//! The null collector bump allocates objects until it runs out of space, at
//! which point it returns an out-of-memory error. It never collects garbage.
//! It does not require any GC barriers.

use super::*;
use crate::{
    Engine,
    prelude::*,
    vm::{
        ExternRefHostDataId, ExternRefHostDataTable, GarbageCollection, GcHeap, GcHeapObject,
        GcProgress, GcRootsIter, GcRuntime, SendSyncUnsafeCell, TypedGcRef, VMGcHeader, VMGcRef,
        VMMemoryDefinition,
    },
};
use core::ptr::NonNull;
use core::{alloc::Layout, any::Any, num::NonZeroU32};
use wasmtime_environ::{
    GcArrayLayout, GcExceptionLayout, GcStructLayout, GcTypeLayouts, VMGcKind, VMSharedTypeIndex,
    null::NullTypeLayouts,
};

/// The null collector.
#[derive(Default)]
pub struct NullCollector {
    layouts: NullTypeLayouts,
}

unsafe impl GcRuntime for NullCollector {
    fn layouts(&self) -> &dyn GcTypeLayouts {
        &self.layouts
    }

    fn new_gc_heap(&self, _: &Engine) -> Result<Box<dyn GcHeap>> {
        let heap = NullHeap::new()?;
        Ok(Box::new(heap) as _)
    }
}

/// A GC heap for the null collector.
#[repr(C)]
struct NullHeap {
    /// Bump-allocation finger indexing within `1..self.heap.len()`.
    ///
    /// NB: this is an `UnsafeCell` because it is written to by compiled Wasm
    /// code.
    next: SendSyncUnsafeCell<NonZeroU32>,

    /// The number of active no-gc scopes at the current moment.
    no_gc_count: usize,

    /// The actual storage for the GC heap.
    memory: Option<crate::vm::Memory>,
}

/// The common header for all arrays in the null collector.
#[repr(C)]
struct VMNullArrayHeader {
    header: VMGcHeader,
    length: u32,
}

unsafe impl GcHeapObject for VMNullArrayHeader {
    #[inline]
    fn is(header: &VMGcHeader) -> bool {
        header.kind() == VMGcKind::ArrayRef
    }
}

impl VMNullArrayHeader {
    fn typed_ref<'a>(
        gc_heap: &NullHeap,
        array: &'a VMArrayRef,
    ) -> &'a TypedGcRef<VMNullArrayHeader> {
        let gc_ref = array.as_gc_ref();
        debug_assert!(gc_ref.is_typed::<VMNullArrayHeader>(gc_heap));
        gc_ref.as_typed_unchecked()
    }
}

/// The representation of an `externref` in the null collector.
#[repr(C)]
struct VMNullExternRef {
    header: VMGcHeader,
    host_data: ExternRefHostDataId,
}

unsafe impl GcHeapObject for VMNullExternRef {
    #[inline]
    fn is(header: &VMGcHeader) -> bool {
        header.kind() == VMGcKind::ExternRef
    }
}

impl VMNullExternRef {
    /// Convert a generic `externref` to a typed reference to our concrete
    /// `externref` type.
    fn typed_ref<'a>(
        gc_heap: &NullHeap,
        externref: &'a VMExternRef,
    ) -> &'a TypedGcRef<VMNullExternRef> {
        let gc_ref = externref.as_gc_ref();
        debug_assert!(gc_ref.is_typed::<VMNullExternRef>(gc_heap));
        gc_ref.as_typed_unchecked()
    }
}

impl NullHeap {
    /// Construct a new, default heap for the null collector.
    fn new() -> Result<Self> {
        Ok(Self {
            no_gc_count: 0,
            next: SendSyncUnsafeCell::new(NonZeroU32::new(u32::MAX).unwrap()),
            memory: None,
        })
    }

    /// Attempt to bump-allocate an object with the given layout and
    /// header.
    ///
    /// Returns `Ok(Ok(r))` on success, `Ok(Err(bytes_needed))` when we don't
    /// have enough heap space but growing the GC heap could make it
    /// allocatable, and `Err(_)` when we don't have enough space and growing
    /// the GC heap won't help.
    fn alloc(&mut self, mut header: VMGcHeader, layout: Layout) -> Result<Result<VMGcRef, u64>> {
        debug_assert!(layout.size() >= core::mem::size_of::<VMGcHeader>());
        debug_assert!(layout.align() >= core::mem::align_of::<VMGcHeader>());

        // Make sure that the requested allocation's size fits in the GC
        // header's unused bits.
        let size = match u32::try_from(layout.size()).ok().and_then(|size| {
            if VMGcKind::value_fits_in_unused_bits(size) {
                Some(size)
            } else {
                None
            }
        }) {
            Some(size) => size,
            None => return Err(crate::Trap::AllocationTooLarge.into()),
        };

        let next = *self.next.get_mut();

        // Increment the bump pointer to the layout's requested alignment.
        let aligned = match u32::try_from(layout.align())
            .ok()
            .and_then(|align| next.get().checked_next_multiple_of(align))
        {
            Some(aligned) => aligned,
            None => return Err(crate::Trap::AllocationTooLarge.into()),
        };

        // Check whether the allocation fits in the heap space we have left.
        let end_of_object = match aligned.checked_add(size) {
            Some(end) => end,
            None => return Err(crate::Trap::AllocationTooLarge.into()),
        };
        let len = self.memory.as_ref().unwrap().byte_size();
        let len = u32::try_from(len).unwrap_or(u32::MAX);
        if end_of_object > len {
            return Ok(Err(u64::try_from(layout.size()).unwrap()));
        }

        // Update the bump pointer, write the header, and return the GC ref.
        *self.next.get_mut() = NonZeroU32::new(end_of_object).unwrap();

        let aligned = NonZeroU32::new(aligned).unwrap();
        let gc_ref = VMGcRef::from_heap_index(aligned).unwrap();

        debug_assert_eq!(header.reserved_u26(), 0);
        header.set_reserved_u26(size);
        *self.header_mut(&gc_ref) = header;

        Ok(Ok(gc_ref))
    }
}

unsafe impl GcHeap for NullHeap {
    fn is_attached(&self) -> bool {
        self.memory.is_some()
    }

    fn attach(&mut self, memory: crate::vm::Memory) {
        assert!(!self.is_attached());
        self.memory = Some(memory);
        self.next = SendSyncUnsafeCell::new(NonZeroU32::new(1).unwrap());
    }

    fn detach(&mut self) -> crate::vm::Memory {
        assert!(self.is_attached());

        let NullHeap {
            next,
            no_gc_count,
            memory,
        } = self;

        *next.get_mut() = NonZeroU32::new(1).unwrap();
        *no_gc_count = 0;

        self.next = SendSyncUnsafeCell::new(NonZeroU32::new(u32::MAX).unwrap());
        memory.take().unwrap()
    }

    fn as_any(&self) -> &dyn Any {
        self as _
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as _
    }

    fn enter_no_gc_scope(&mut self) {
        self.no_gc_count += 1;
    }

    fn exit_no_gc_scope(&mut self) {
        self.no_gc_count -= 1;
    }

    unsafe fn take_memory(&mut self) -> crate::vm::Memory {
        debug_assert!(self.is_attached());
        self.memory.take().unwrap()
    }

    unsafe fn replace_memory(&mut self, memory: crate::vm::Memory, _delta_bytes_grown: u64) {
        debug_assert!(self.memory.is_none());
        self.memory = Some(memory);
    }

    fn vmmemory(&self) -> VMMemoryDefinition {
        debug_assert!(self.is_attached());
        self.memory.as_ref().unwrap().vmmemory()
    }

    fn clone_gc_ref(&mut self, gc_ref: &VMGcRef) -> VMGcRef {
        gc_ref.unchecked_copy()
    }

    fn write_gc_ref(
        &mut self,
        _host_data_table: &mut ExternRefHostDataTable,
        destination: &mut Option<VMGcRef>,
        source: Option<&VMGcRef>,
    ) {
        *destination = source.map(|s| s.unchecked_copy());
    }

    fn expose_gc_ref_to_wasm(&mut self, _gc_ref: VMGcRef) {
        // Don't need to do anything special here.
    }

    fn alloc_externref(
        &mut self,
        host_data: ExternRefHostDataId,
    ) -> Result<Result<VMExternRef, u64>> {
        let gc_ref = match self.alloc(VMGcHeader::externref(), Layout::new::<VMNullExternRef>())? {
            Ok(r) => r,
            Err(bytes_needed) => return Ok(Err(bytes_needed)),
        };
        self.index_mut::<VMNullExternRef>(gc_ref.as_typed_unchecked())
            .host_data = host_data;
        Ok(Ok(gc_ref.into_externref_unchecked()))
    }

    fn externref_host_data(&self, externref: &VMExternRef) -> ExternRefHostDataId {
        let typed_ref = VMNullExternRef::typed_ref(self, externref);
        self.index(typed_ref).host_data
    }

    fn object_size(&self, gc_ref: &VMGcRef) -> usize {
        let size = self.header(gc_ref).reserved_u26();
        usize::try_from(size).unwrap()
    }

    fn header(&self, gc_ref: &VMGcRef) -> &VMGcHeader {
        self.index(gc_ref.as_typed_unchecked())
    }

    fn header_mut(&mut self, gc_ref: &VMGcRef) -> &mut VMGcHeader {
        self.index_mut(gc_ref.as_typed_unchecked())
    }

    fn alloc_raw(&mut self, header: VMGcHeader, layout: Layout) -> Result<Result<VMGcRef, u64>> {
        self.alloc(header, layout)
    }

    fn alloc_uninit_struct(
        &mut self,
        ty: VMSharedTypeIndex,
        layout: &GcStructLayout,
    ) -> Result<Result<VMStructRef, u64>> {
        self.alloc(
            VMGcHeader::from_kind_and_index(VMGcKind::StructRef, ty),
            layout.layout(),
        )
        .map(|r| r.map(|r| r.into_structref_unchecked()))
    }

    fn dealloc_uninit_struct(&mut self, _struct_ref: VMStructRef) {}

    fn alloc_uninit_array(
        &mut self,
        ty: VMSharedTypeIndex,
        length: u32,
        layout: &GcArrayLayout,
    ) -> Result<Result<VMArrayRef, u64>> {
        self.alloc(
            VMGcHeader::from_kind_and_index(VMGcKind::ArrayRef, ty),
            layout.layout(length),
        )
        .map(|r| {
            r.map(|r| {
                self.index_mut::<VMNullArrayHeader>(r.as_typed_unchecked())
                    .length = length;
                r.into_arrayref_unchecked()
            })
        })
    }

    fn dealloc_uninit_array(&mut self, _array_ref: VMArrayRef) {}

    fn array_len(&self, arrayref: &VMArrayRef) -> u32 {
        let arrayref = VMNullArrayHeader::typed_ref(self, arrayref);
        self.index(arrayref).length
    }

    fn alloc_uninit_exn(
        &mut self,
        ty: VMSharedTypeIndex,
        layout: &GcExceptionLayout,
    ) -> Result<Result<VMExnRef, u64>> {
        self.alloc(
            VMGcHeader::from_kind_and_index(VMGcKind::ExnRef, ty),
            layout.layout(),
        )
        .map(|r| r.map(|r| r.into_exnref_unchecked()))
    }

    fn dealloc_uninit_exn(&mut self, _exnref: VMExnRef) {}

    fn gc<'a>(
        &'a mut self,
        _roots: GcRootsIter<'a>,
        _host_data_table: &'a mut ExternRefHostDataTable,
    ) -> Box<dyn GarbageCollection<'a> + 'a> {
        assert_eq!(self.no_gc_count, 0, "Cannot GC inside a no-GC scope!");
        Box::new(NullCollection {})
    }

    unsafe fn vmctx_gc_heap_data(&self) -> NonNull<u8> {
        let ptr_to_next: *mut NonZeroU32 = unsafe { self.next.get() };
        NonNull::new(ptr_to_next).unwrap().cast()
    }
}

struct NullCollection {}

impl<'a> GarbageCollection<'a> for NullCollection {
    fn collect_increment(&mut self) -> GcProgress {
        GcProgress::Complete
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_gc_null_header_size_align() {
        assert_eq!(
            (wasmtime_environ::null::HEADER_SIZE as usize),
            core::mem::size_of::<VMGcHeader>()
        );
        assert_eq!(
            (wasmtime_environ::null::HEADER_ALIGN as usize),
            core::mem::align_of::<VMGcHeader>()
        );
    }

    #[test]
    fn vm_null_array_header_length_offset() {
        assert_eq!(
            wasmtime_environ::null::ARRAY_LENGTH_OFFSET,
            u32::try_from(core::mem::offset_of!(VMNullArrayHeader, length)).unwrap(),
        );
    }
}
