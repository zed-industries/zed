extern crate alloc;

use alloc::alloc::{alloc, dealloc, handle_alloc_error, Layout, realloc};
use core::cmp;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use crate::mem::{dangling, Mem, MemBuilder, MemBuilderSizeable, MemRawParts, MemResizable};

/// Heap allocated memory.
#[derive(Default, Clone, Copy)]
pub struct Heap;
impl MemBuilder for Heap {
    /// Implements [`MemResizable`], [`MemRawParts`].
    type Mem = HeapMem;

    #[inline]
    fn build(&mut self, element_layout: Layout) -> HeapMem {
        HeapMem {
            mem: dangling(&element_layout),
            size: 0,
            element_layout
        }
    }
}
impl MemBuilderSizeable for Heap{
    #[inline]
    fn build_with_size(&mut self, element_layout: Layout, capacity: usize) -> Self::Mem
    {
        let mut mem = self.build(element_layout);
        mem.resize(capacity);
        mem
    }
}

pub struct HeapMem {
    mem: NonNull<u8>,
    size: usize,        // in elements
    element_layout: Layout, // size is aligned
}

impl Mem for HeapMem {
    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.mem.as_ptr()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.mem.as_ptr()
    }

    #[inline]
    fn element_layout(&self) -> Layout {
        self.element_layout
    }

    #[inline]
    fn size(&self) -> usize {
        self.size
    }

    fn expand(&mut self, additional: usize){
        let requested_size = self.size() + additional;
        let new_size = cmp::max(self.size() * 2, requested_size);
        self.resize(new_size);
    }
}

impl MemResizable for HeapMem {
    fn resize(&mut self, new_size: usize) {
        if self.size == new_size{
            return;
        }

        if self.element_layout.size() != 0 {
            unsafe{
                // Non checked mul, because this memory size already allocated.
                let mem_layout = Layout::from_size_align_unchecked(
                    self.element_layout.size() * self.size,
                    self.element_layout.align()
                );

                self.mem =
                    if new_size == 0 {
                        dealloc(self.mem.as_ptr(), mem_layout);
                        dangling(&self.element_layout)
                    } else {
                        // mul carefully, to prevent overflow.
                        let new_mem_size = self.element_layout.size()
                            .checked_mul(new_size).unwrap();
                        let new_mem_layout = Layout::from_size_align_unchecked(
                            new_mem_size, self.element_layout.align()
                        );

                        if self.size == 0 {
                            // allocate
                            NonNull::new(alloc(new_mem_layout))
                        } else {
                            // reallocate
                            NonNull::new(realloc(
                                self.mem.as_ptr(), mem_layout,new_mem_size
                            ))
                        }
                        .unwrap_or_else(|| handle_alloc_error(new_mem_layout))
                    }
            }
        }
        self.size = new_size;
    }
}

impl MemRawParts for HeapMem{
    type Handle = NonNull<u8>;

    #[inline]
    fn into_raw_parts(self) -> (Self::Handle, Layout, usize) {
        let this = ManuallyDrop::new(self);
        (this.mem, this.element_layout, this.size)
    }

    #[inline]
    unsafe fn from_raw_parts(handle: Self::Handle, element_layout: Layout, size: usize) -> Self {
        Self{
            mem: handle,
            size,
            element_layout
        }
    }
}

impl Drop for HeapMem {
    fn drop(&mut self) {
        self.resize(0);
    }
}

unsafe impl Send for HeapMem{}
unsafe impl Sync for HeapMem{}