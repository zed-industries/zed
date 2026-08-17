use super::GcHeapAllocationIndex;
use super::index_allocator::{SimpleIndexAllocator, SlotId};
use crate::runtime::vm::{GcHeap, GcRuntime, PoolingInstanceAllocatorConfig, Result};
use crate::vm::{Memory, MemoryAllocationIndex};
use crate::{Engine, prelude::*};
use std::sync::Mutex;

enum HeapSlot {
    /// The is available for use, and we may or may not have lazily allocated
    /// its associated GC heap yet.
    Free(Option<Box<dyn GcHeap>>),

    /// The slot's heap is currently in use, and it is backed by this memory
    /// allocation index.
    InUse(MemoryAllocationIndex),
}

impl HeapSlot {
    fn alloc(&mut self, memory_alloc_index: MemoryAllocationIndex) -> Option<Box<dyn GcHeap>> {
        match self {
            HeapSlot::Free(gc_heap) => {
                let gc_heap = gc_heap.take();
                *self = HeapSlot::InUse(memory_alloc_index);
                gc_heap
            }
            HeapSlot::InUse(_) => panic!("already in use"),
        }
    }

    fn dealloc(&mut self, heap: Box<dyn GcHeap>) -> MemoryAllocationIndex {
        match *self {
            HeapSlot::Free(_) => panic!("already free"),
            HeapSlot::InUse(memory_alloc_index) => {
                *self = HeapSlot::Free(Some(heap));
                memory_alloc_index
            }
        }
    }
}

/// A pool of reusable GC heaps.
pub struct GcHeapPool {
    max_gc_heaps: usize,
    index_allocator: SimpleIndexAllocator,
    heaps: Mutex<Box<[HeapSlot]>>,
}

impl std::fmt::Debug for GcHeapPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GcHeapPool")
            .field("max_gc_heaps", &self.max_gc_heaps)
            .field("index_allocator", &self.index_allocator)
            .field("heaps", &"..")
            .finish()
    }
}

impl GcHeapPool {
    /// Create a new `GcHeapPool` with the given configuration.
    pub fn new(config: &PoolingInstanceAllocatorConfig) -> Result<Self> {
        let index_allocator = SimpleIndexAllocator::new(config.limits.total_gc_heaps);
        let max_gc_heaps = usize::try_from(config.limits.total_gc_heaps).unwrap();

        // Each individual GC heap in the pool is lazily allocated. See the
        // `allocate` method.
        let heaps = Mutex::new((0..max_gc_heaps).map(|_| HeapSlot::Free(None)).collect());

        Ok(Self {
            max_gc_heaps,
            index_allocator,
            heaps,
        })
    }

    /// Are there zero slots in use right now?
    pub fn is_empty(&self) -> bool {
        self.index_allocator.is_empty()
    }

    /// Allocate a single table for the given instance allocation request.
    pub fn allocate(
        &self,
        engine: &Engine,
        gc_runtime: &dyn GcRuntime,
        memory_alloc_index: MemoryAllocationIndex,
        memory: Memory,
    ) -> Result<(GcHeapAllocationIndex, Box<dyn GcHeap>)> {
        let allocation_index = self
            .index_allocator
            .alloc()
            .map(|slot| GcHeapAllocationIndex(slot.0))
            .ok_or_else(|| {
                anyhow!(
                    "maximum concurrent GC heap limit of {} reached",
                    self.max_gc_heaps
                )
            })?;
        debug_assert_ne!(allocation_index, GcHeapAllocationIndex::default());

        let mut heap = match {
            let mut heaps = self.heaps.lock().unwrap();
            heaps[allocation_index.index()].alloc(memory_alloc_index)
        } {
            // If we already have a heap at this slot, reuse it.
            Some(heap) => heap,
            // Otherwise, we haven't forced this slot's lazily allocated heap
            // yet. So do that now.
            None => gc_runtime.new_gc_heap(engine)?,
        };

        debug_assert!(!heap.is_attached());
        heap.attach(memory);

        Ok((allocation_index, heap))
    }

    /// Deallocate a previously-allocated GC heap.
    pub fn deallocate(
        &self,
        allocation_index: GcHeapAllocationIndex,
        mut heap: Box<dyn GcHeap>,
    ) -> (MemoryAllocationIndex, Memory) {
        debug_assert_ne!(allocation_index, GcHeapAllocationIndex::default());

        let memory = heap.detach();

        // NB: Replace the heap before freeing the index. If we did it in the
        // opposite order, a concurrent allocation request could reallocate the
        // index before we have replaced the heap.

        let memory_alloc_index = {
            let mut heaps = self.heaps.lock().unwrap();
            heaps[allocation_index.index()].dealloc(heap)
        };

        self.index_allocator.free(SlotId(allocation_index.0));

        (memory_alloc_index, memory)
    }
}
