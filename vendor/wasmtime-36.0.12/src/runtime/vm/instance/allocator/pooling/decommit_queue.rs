//! A queue for batching decommits together.
//!
//! We don't immediately decommit a Wasm table/memory/stack/etc... eagerly, but
//! instead batch them up to be decommitted together. This module implements
//! that queuing and batching.
//!
//! Even when batching is "disabled" we still use this queue. Batching is
//! disabled by specifying a batch size of one, in which case, this queue will
//! immediately get flushed every time we push onto it.

use super::PoolingInstanceAllocator;
use crate::vm::{MemoryAllocationIndex, MemoryImageSlot, Table, TableAllocationIndex};
use smallvec::SmallVec;

#[cfg(feature = "async")]
use wasmtime_fiber::FiberStack;

#[cfg(unix)]
#[expect(non_camel_case_types, reason = "matching libc naming")]
type iovec = libc::iovec;

#[cfg(not(unix))]
#[expect(non_camel_case_types, reason = "matching libc naming")]
struct iovec {
    iov_base: *mut libc::c_void,
    iov_len: libc::size_t,
}

#[repr(transparent)]
struct IoVec(iovec);

unsafe impl Send for IoVec {}
unsafe impl Sync for IoVec {}

impl std::fmt::Debug for IoVec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IoVec")
            .field("base", &self.0.iov_base)
            .field("len", &self.0.iov_len)
            .finish()
    }
}

#[cfg(feature = "async")]
struct SendSyncStack(FiberStack);
#[cfg(feature = "async")]
unsafe impl Send for SendSyncStack {}
#[cfg(feature = "async")]
unsafe impl Sync for SendSyncStack {}

#[derive(Default)]
pub struct DecommitQueue {
    raw: SmallVec<[IoVec; 2]>,
    memories: SmallVec<[(MemoryAllocationIndex, MemoryImageSlot); 1]>,
    tables: SmallVec<[(TableAllocationIndex, Table); 1]>,
    #[cfg(feature = "async")]
    stacks: SmallVec<[SendSyncStack; 1]>,
    //
    // TODO: GC heaps are not well-integrated with the pooling allocator
    // yet. Once we better integrate them, we should start (optionally) zeroing
    // them, and batching that up here.
    //
    // #[cfg(feature = "gc")]
    // pub gc_heaps: SmallVec<[(GcHeapAllocationIndex, Box<dyn GcHeap>); 1]>,
}

impl std::fmt::Debug for DecommitQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecommitQueue")
            .field("raw", &self.raw)
            .finish_non_exhaustive()
    }
}

impl DecommitQueue {
    /// Append another queue to this queue.
    pub fn append(
        &mut self,
        Self {
            raw,
            memories,
            tables,
            #[cfg(feature = "async")]
            stacks,
        }: &mut Self,
    ) {
        self.raw.append(raw);
        self.memories.append(memories);
        self.tables.append(tables);
        #[cfg(feature = "async")]
        self.stacks.append(stacks);
    }

    /// How many raw memory regions are enqueued for decommit?
    pub fn raw_len(&self) -> usize {
        self.raw.len()
    }

    /// Enqueue a region of memory for decommit.
    ///
    /// It is the caller's responsibility to push the associated data via
    /// `self.push_{memory,table,stack}` as appropriate.
    ///
    /// # Safety
    ///
    /// The enqueued memory regions must be safe to decommit when `flush` is
    /// called (no other references, not in use, won't be otherwise unmapped,
    /// etc...).
    pub unsafe fn push_raw(&mut self, ptr: *mut u8, len: usize) {
        self.raw.push(IoVec(iovec {
            iov_base: ptr.cast(),
            iov_len: len,
        }));
    }

    /// Push a memory into the queue.
    ///
    /// # Safety
    ///
    /// This memory should not be in use, and its decommit regions must have
    /// already been enqueued via `self.enqueue_raw`.
    pub unsafe fn push_memory(
        &mut self,
        allocation_index: MemoryAllocationIndex,
        image: MemoryImageSlot,
    ) {
        self.memories.push((allocation_index, image));
    }

    /// Push a table into the queue.
    ///
    /// # Safety
    ///
    /// This table should not be in use, and its decommit regions must have
    /// already been enqueued via `self.enqueue_raw`.
    pub unsafe fn push_table(&mut self, allocation_index: TableAllocationIndex, table: Table) {
        self.tables.push((allocation_index, table));
    }

    /// Push a stack into the queue.
    ///
    /// # Safety
    ///
    /// This stack should not be in use, and its decommit regions must have
    /// already been enqueued via `self.enqueue_raw`.
    #[cfg(feature = "async")]
    pub unsafe fn push_stack(&mut self, stack: FiberStack) {
        self.stacks.push(SendSyncStack(stack));
    }

    fn decommit_all_raw(&mut self) {
        for iovec in self.raw.drain(..) {
            unsafe {
                crate::vm::sys::vm::decommit_pages(iovec.0.iov_base.cast(), iovec.0.iov_len)
                    .unwrap_or_else(|e| {
                        panic!(
                            "failed to decommit ptr={:#p}, len={:#x}: {e}",
                            iovec.0.iov_base, iovec.0.iov_len
                        )
                    });
            }
        }
    }

    /// Flush this queue, decommitting all enqueued regions in batch.
    ///
    /// Returns `true` if we did any decommits and returned their entities to
    /// the associated free lists; `false` if the queue was empty.
    pub fn flush(mut self, pool: &PoolingInstanceAllocator) -> bool {
        // First, do the raw decommit syscall(s).
        self.decommit_all_raw();

        // Second, restore the various entities to their associated pools' free
        // lists. This is safe, and they are ready for reuse, now that their
        // memory regions have been decommitted.
        let mut deallocated_any = false;
        for (allocation_index, image) in self.memories {
            deallocated_any = true;
            unsafe {
                pool.memories.deallocate(allocation_index, image);
            }
        }
        for (allocation_index, table) in self.tables {
            deallocated_any = true;
            unsafe {
                pool.tables.deallocate(allocation_index, table);
            }
        }
        #[cfg(feature = "async")]
        for stack in self.stacks {
            deallocated_any = true;
            unsafe {
                pool.stacks.deallocate(stack.0);
            }
        }

        deallocated_any
    }
}
