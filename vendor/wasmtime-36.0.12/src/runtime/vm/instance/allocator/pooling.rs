//! Implements the pooling instance allocator.
//!
//! The pooling instance allocator maps memory in advance and allocates
//! instances, memories, tables, and stacks from a pool of available resources.
//! Using the pooling instance allocator can speed up module instantiation when
//! modules can be constrained based on configurable limits
//! ([`InstanceLimits`]). Each new instance is stored in a "slot"; as instances
//! are allocated and freed, these slots are either filled or emptied:
//!
//! ```text
//! ┌──────┬──────┬──────┬──────┬──────┐
//! │Slot 0│Slot 1│Slot 2│Slot 3│......│
//! └──────┴──────┴──────┴──────┴──────┘
//! ```
//!
//! Each slot has a "slot ID"--an index into the pool. Slot IDs are handed out
//! by the [`index_allocator`] module. Note that each kind of pool-allocated
//! item is stored in its own separate pool: [`memory_pool`], [`table_pool`],
//! [`stack_pool`]. See those modules for more details.

mod decommit_queue;
mod index_allocator;
mod memory_pool;
mod table_pool;

#[cfg(feature = "gc")]
mod gc_heap_pool;

#[cfg(all(feature = "async"))]
mod generic_stack_pool;
#[cfg(all(feature = "async", unix, not(miri)))]
mod unix_stack_pool;

#[cfg(all(feature = "async"))]
cfg_if::cfg_if! {
    if #[cfg(all(unix, not(miri), not(asan)))] {
        use unix_stack_pool as stack_pool;
    } else {
        use generic_stack_pool as stack_pool;
    }
}

use self::decommit_queue::DecommitQueue;
use self::memory_pool::MemoryPool;
use self::table_pool::TablePool;
use super::{
    InstanceAllocationRequest, InstanceAllocatorImpl, MemoryAllocationIndex, TableAllocationIndex,
};
use crate::MpkEnabled;
use crate::prelude::*;
use crate::runtime::vm::{
    CompiledModuleId, Memory, Table,
    instance::Instance,
    mpk::{self, ProtectionKey, ProtectionMask},
};
use std::borrow::Cow;
use std::fmt::Display;
use std::sync::{Mutex, MutexGuard};
use std::{
    mem,
    sync::atomic::{AtomicU64, Ordering},
};
use wasmtime_environ::{
    DefinedMemoryIndex, DefinedTableIndex, HostPtr, Module, Tunables, VMOffsets,
};

#[cfg(feature = "gc")]
use super::GcHeapAllocationIndex;
#[cfg(feature = "gc")]
use crate::runtime::vm::{GcHeap, GcRuntime};
#[cfg(feature = "gc")]
use gc_heap_pool::GcHeapPool;

#[cfg(feature = "async")]
use stack_pool::StackPool;

#[cfg(feature = "component-model")]
use wasmtime_environ::{
    StaticModuleIndex,
    component::{Component, VMComponentOffsets},
};

fn round_up_to_pow2(n: usize, to: usize) -> usize {
    debug_assert!(to > 0);
    debug_assert!(to.is_power_of_two());
    (n + to - 1) & !(to - 1)
}

/// Instance-related limit configuration for pooling.
///
/// More docs on this can be found at `wasmtime::PoolingAllocationConfig`.
#[derive(Debug, Copy, Clone)]
pub struct InstanceLimits {
    /// The maximum number of component instances that may be allocated
    /// concurrently.
    pub total_component_instances: u32,

    /// The maximum size of a component's `VMComponentContext`, not including
    /// any of its inner core modules' `VMContext` sizes.
    pub component_instance_size: usize,

    /// The maximum number of core module instances that may be allocated
    /// concurrently.
    pub total_core_instances: u32,

    /// The maximum number of core module instances that a single component may
    /// transitively contain.
    pub max_core_instances_per_component: u32,

    /// The maximum number of Wasm linear memories that a component may
    /// transitively contain.
    pub max_memories_per_component: u32,

    /// The maximum number of tables that a component may transitively contain.
    pub max_tables_per_component: u32,

    /// The total number of linear memories in the pool, across all instances.
    pub total_memories: u32,

    /// The total number of tables in the pool, across all instances.
    pub total_tables: u32,

    /// The total number of async stacks in the pool, across all instances.
    #[cfg(feature = "async")]
    pub total_stacks: u32,

    /// Maximum size of a core instance's `VMContext`.
    pub core_instance_size: usize,

    /// Maximum number of tables per instance.
    pub max_tables_per_module: u32,

    /// Maximum number of word-size elements per table.
    ///
    /// Note that tables for element types such as continuations
    /// that use more than one word of storage may store fewer
    /// elements.
    pub table_elements: usize,

    /// Maximum number of linear memories per instance.
    pub max_memories_per_module: u32,

    /// Maximum byte size of a linear memory, must be smaller than
    /// `memory_reservation` in `Tunables`.
    pub max_memory_size: usize,

    /// The total number of GC heaps in the pool, across all instances.
    #[cfg(feature = "gc")]
    pub total_gc_heaps: u32,
}

impl Default for InstanceLimits {
    fn default() -> Self {
        let total = if cfg!(target_pointer_width = "32") {
            100
        } else {
            1000
        };
        // See doc comments for `wasmtime::PoolingAllocationConfig` for these
        // default values
        Self {
            total_component_instances: total,
            component_instance_size: 1 << 20, // 1 MiB
            total_core_instances: total,
            max_core_instances_per_component: u32::MAX,
            max_memories_per_component: u32::MAX,
            max_tables_per_component: u32::MAX,
            total_memories: total,
            total_tables: total,
            #[cfg(feature = "async")]
            total_stacks: total,
            core_instance_size: 1 << 20, // 1 MiB
            max_tables_per_module: 1,
            // NB: in #8504 it was seen that a C# module in debug module can
            // have 10k+ elements.
            table_elements: 20_000,
            max_memories_per_module: 1,
            #[cfg(target_pointer_width = "64")]
            max_memory_size: 1 << 32, // 4G,
            #[cfg(target_pointer_width = "32")]
            max_memory_size: 10 << 20, // 10 MiB
            #[cfg(feature = "gc")]
            total_gc_heaps: total,
        }
    }
}

/// Configuration options for the pooling instance allocator supplied at
/// construction.
#[derive(Copy, Clone, Debug)]
pub struct PoolingInstanceAllocatorConfig {
    /// See `PoolingAllocatorConfig::max_unused_warm_slots` in `wasmtime`
    pub max_unused_warm_slots: u32,
    /// The target number of decommits to do per batch. This is not precise, as
    /// we can queue up decommits at times when we aren't prepared to
    /// immediately flush them, and so we may go over this target size
    /// occasionally.
    pub decommit_batch_size: usize,
    /// The size, in bytes, of async stacks to allocate (not including the guard
    /// page).
    pub stack_size: usize,
    /// The limits to apply to instances allocated within this allocator.
    pub limits: InstanceLimits,
    /// Whether or not async stacks are zeroed after use.
    pub async_stack_zeroing: bool,
    /// If async stack zeroing is enabled and the host platform is Linux this is
    /// how much memory to zero out with `memset`.
    ///
    /// The rest of memory will be zeroed out with `madvise`.
    #[cfg(feature = "async")]
    pub async_stack_keep_resident: usize,
    /// How much linear memory, in bytes, to keep resident after resetting for
    /// use with the next instance. This much memory will be `memset` to zero
    /// when a linear memory is deallocated.
    ///
    /// Memory exceeding this amount in the wasm linear memory will be released
    /// with `madvise` back to the kernel.
    ///
    /// Only applicable on Linux.
    pub linear_memory_keep_resident: usize,
    /// Same as `linear_memory_keep_resident` but for tables.
    pub table_keep_resident: usize,
    /// Whether to enable memory protection keys.
    pub memory_protection_keys: MpkEnabled,
    /// How many memory protection keys to allocate.
    pub max_memory_protection_keys: usize,
}

impl Default for PoolingInstanceAllocatorConfig {
    fn default() -> PoolingInstanceAllocatorConfig {
        PoolingInstanceAllocatorConfig {
            max_unused_warm_slots: 100,
            decommit_batch_size: 1,
            stack_size: 2 << 20,
            limits: InstanceLimits::default(),
            async_stack_zeroing: false,
            #[cfg(feature = "async")]
            async_stack_keep_resident: 0,
            linear_memory_keep_resident: 0,
            table_keep_resident: 0,
            memory_protection_keys: MpkEnabled::Disable,
            max_memory_protection_keys: 16,
        }
    }
}

/// An error returned when the pooling allocator cannot allocate a table,
/// memory, etc... because the maximum number of concurrent allocations for that
/// entity has been reached.
#[derive(Debug)]
pub struct PoolConcurrencyLimitError {
    limit: usize,
    kind: Cow<'static, str>,
}

impl core::error::Error for PoolConcurrencyLimitError {}

impl Display for PoolConcurrencyLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let limit = self.limit;
        let kind = &self.kind;
        write!(f, "maximum concurrent limit of {limit} for {kind} reached")
    }
}

impl PoolConcurrencyLimitError {
    fn new(limit: usize, kind: impl Into<Cow<'static, str>>) -> Self {
        Self {
            limit,
            kind: kind.into(),
        }
    }
}

/// Implements the pooling instance allocator.
///
/// This allocator internally maintains pools of instances, memories, tables,
/// and stacks.
///
/// Note: the resource pools are manually dropped so that the fault handler
/// terminates correctly.
#[derive(Debug)]
pub struct PoolingInstanceAllocator {
    decommit_batch_size: usize,
    limits: InstanceLimits,

    // The number of live core module and component instances at any given
    // time. Note that this can temporarily go over the configured limit. This
    // doesn't mean we have actually overshot, but that we attempted to allocate
    // a new instance and incremented the counter, we've seen (or are about to
    // see) that the counter is beyond the configured threshold, and are going
    // to decrement the counter and return an error but haven't done so yet. See
    // the increment trait methods for more details.
    live_core_instances: AtomicU64,
    live_component_instances: AtomicU64,

    decommit_queue: Mutex<DecommitQueue>,
    memories: MemoryPool,
    tables: TablePool,

    #[cfg(feature = "gc")]
    gc_heaps: GcHeapPool,

    #[cfg(feature = "async")]
    stacks: StackPool,
}

impl Drop for PoolingInstanceAllocator {
    fn drop(&mut self) {
        if !cfg!(debug_assertions) {
            return;
        }

        // NB: when cfg(not(debug_assertions)) it is okay that we don't flush
        // the queue, as the sub-pools will unmap those ranges anyways, so
        // there's no point in decommitting them. But we do need to flush the
        // queue when debug assertions are enabled to make sure that all
        // entities get returned to their associated sub-pools and we can
        // differentiate between a leaking slot and an enqueued-for-decommit
        // slot.
        let queue = self.decommit_queue.lock().unwrap();
        self.flush_decommit_queue(queue);

        debug_assert_eq!(self.live_component_instances.load(Ordering::Acquire), 0);
        debug_assert_eq!(self.live_core_instances.load(Ordering::Acquire), 0);

        debug_assert!(self.memories.is_empty());
        debug_assert!(self.tables.is_empty());

        #[cfg(feature = "gc")]
        debug_assert!(self.gc_heaps.is_empty());

        #[cfg(feature = "async")]
        debug_assert!(self.stacks.is_empty());
    }
}

impl PoolingInstanceAllocator {
    /// Creates a new pooling instance allocator with the given strategy and limits.
    pub fn new(config: &PoolingInstanceAllocatorConfig, tunables: &Tunables) -> Result<Self> {
        Ok(Self {
            decommit_batch_size: config.decommit_batch_size,
            limits: config.limits,
            live_component_instances: AtomicU64::new(0),
            live_core_instances: AtomicU64::new(0),
            decommit_queue: Mutex::new(DecommitQueue::default()),
            memories: MemoryPool::new(config, tunables)?,
            tables: TablePool::new(config)?,
            #[cfg(feature = "gc")]
            gc_heaps: GcHeapPool::new(config)?,
            #[cfg(feature = "async")]
            stacks: StackPool::new(config)?,
        })
    }

    fn core_instance_size(&self) -> usize {
        round_up_to_pow2(self.limits.core_instance_size, mem::align_of::<Instance>())
    }

    fn validate_table_plans(&self, module: &Module) -> Result<()> {
        self.tables.validate(module)
    }

    fn validate_memory_plans(&self, module: &Module) -> Result<()> {
        self.memories.validate_memories(module)
    }

    fn validate_core_instance_size(&self, offsets: &VMOffsets<HostPtr>) -> Result<()> {
        let layout = Instance::alloc_layout(offsets);
        if layout.size() <= self.core_instance_size() {
            return Ok(());
        }

        // If this `module` exceeds the allocation size allotted to it then an
        // error will be reported here. The error of "required N bytes but
        // cannot allocate that" is pretty opaque, however, because it's not
        // clear what the breakdown of the N bytes are and what to optimize
        // next. To help provide a better error message here some fancy-ish
        // logic is done here to report the breakdown of the byte request into
        // the largest portions and where it's coming from.
        let mut message = format!(
            "instance allocation for this module \
             requires {} bytes which exceeds the configured maximum \
             of {} bytes; breakdown of allocation requirement:\n\n",
            layout.size(),
            self.core_instance_size(),
        );

        let mut remaining = layout.size();
        let mut push = |name: &str, bytes: usize| {
            assert!(remaining >= bytes);
            remaining -= bytes;

            // If the `name` region is more than 5% of the allocation request
            // then report it here, otherwise ignore it. We have less than 20
            // fields so we're guaranteed that something should be reported, and
            // otherwise it's not particularly interesting to learn about 5
            // different fields that are all 8 or 0 bytes. Only try to report
            // the "major" sources of bytes here.
            if bytes > layout.size() / 20 {
                message.push_str(&format!(
                    " * {:.02}% - {} bytes - {}\n",
                    ((bytes as f32) / (layout.size() as f32)) * 100.0,
                    bytes,
                    name,
                ));
            }
        };

        // The `Instance` itself requires some size allocated to it.
        push("instance state management", mem::size_of::<Instance>());

        // Afterwards the `VMContext`'s regions are why we're requesting bytes,
        // so ask it for descriptions on each region's byte size.
        for (desc, size) in offsets.region_sizes() {
            push(desc, size as usize);
        }

        // double-check we accounted for all the bytes
        assert_eq!(remaining, 0);

        bail!("{}", message)
    }

    #[cfg(feature = "component-model")]
    fn validate_component_instance_size(
        &self,
        offsets: &VMComponentOffsets<HostPtr>,
    ) -> Result<()> {
        if usize::try_from(offsets.size_of_vmctx()).unwrap() <= self.limits.component_instance_size
        {
            return Ok(());
        }

        // TODO: Add context with detailed accounting of what makes up all the
        // `VMComponentContext`'s space like we do for module instances.
        bail!(
            "instance allocation for this component requires {} bytes of `VMComponentContext` \
             space which exceeds the configured maximum of {} bytes",
            offsets.size_of_vmctx(),
            self.limits.component_instance_size
        )
    }

    fn flush_decommit_queue(&self, mut locked_queue: MutexGuard<'_, DecommitQueue>) -> bool {
        // Take the queue out of the mutex and drop the lock, to minimize
        // contention.
        let queue = mem::take(&mut *locked_queue);
        drop(locked_queue);
        queue.flush(self)
    }

    /// Execute `f` and if it returns `Err(PoolConcurrencyLimitError)`, then try
    /// flushing the decommit queue. If flushing the queue freed up slots, then
    /// try running `f` again.
    fn with_flush_and_retry<T>(&self, mut f: impl FnMut() -> Result<T>) -> Result<T> {
        f().or_else(|e| {
            if e.is::<PoolConcurrencyLimitError>() {
                let queue = self.decommit_queue.lock().unwrap();
                if self.flush_decommit_queue(queue) {
                    return f();
                }
            }

            Err(e)
        })
    }

    fn merge_or_flush(&self, mut local_queue: DecommitQueue) {
        match local_queue.raw_len() {
            // If we didn't enqueue any regions for decommit, then we must have
            // either memset the whole entity or eagerly remapped it to zero
            // because we don't have linux's `madvise(DONTNEED)` semantics. In
            // either case, the entity slot is ready for reuse immediately.
            0 => {
                local_queue.flush(self);
            }

            // We enqueued at least our batch size of regions for decommit, so
            // flush the local queue immediately. Don't bother inspecting (or
            // locking!) the shared queue.
            n if n >= self.decommit_batch_size => {
                local_queue.flush(self);
            }

            // If we enqueued some regions for decommit, but did not reach our
            // batch size, so we don't want to flush it yet, then merge the
            // local queue into the shared queue.
            n => {
                debug_assert!(n < self.decommit_batch_size);
                let mut shared_queue = self.decommit_queue.lock().unwrap();
                shared_queue.append(&mut local_queue);
                // And if the shared queue now has at least as many regions
                // enqueued for decommit as our batch size, then we can flush
                // it.
                if shared_queue.raw_len() >= self.decommit_batch_size {
                    self.flush_decommit_queue(shared_queue);
                }
            }
        }
    }
}

unsafe impl InstanceAllocatorImpl for PoolingInstanceAllocator {
    #[cfg(feature = "component-model")]
    fn validate_component_impl<'a>(
        &self,
        component: &Component,
        offsets: &VMComponentOffsets<HostPtr>,
        get_module: &'a dyn Fn(StaticModuleIndex) -> &'a Module,
    ) -> Result<()> {
        self.validate_component_instance_size(offsets)
            .context("component instance size does not fit in pooling allocator requirements")?;

        let mut num_core_instances = 0;
        let mut num_memories = 0;
        let mut num_tables = 0;
        for init in &component.initializers {
            use wasmtime_environ::component::GlobalInitializer::*;
            use wasmtime_environ::component::InstantiateModule;
            match init {
                InstantiateModule(InstantiateModule::Import(_, _)) => {
                    num_core_instances += 1;
                    // Can't statically account for the total vmctx size, number
                    // of memories, and number of tables in this component.
                }
                InstantiateModule(InstantiateModule::Static(static_module_index, _)) => {
                    let module = get_module(*static_module_index);
                    let offsets = VMOffsets::new(HostPtr, &module);
                    self.validate_module_impl(module, &offsets)?;
                    num_core_instances += 1;
                    num_memories += module.num_defined_memories();
                    num_tables += module.num_defined_tables();
                }
                LowerImport { .. }
                | ExtractMemory(_)
                | ExtractTable(_)
                | ExtractRealloc(_)
                | ExtractCallback(_)
                | ExtractPostReturn(_)
                | Resource(_) => {}
            }
        }

        if num_core_instances
            > usize::try_from(self.limits.max_core_instances_per_component).unwrap()
        {
            bail!(
                "The component transitively contains {num_core_instances} core module instances, \
                 which exceeds the configured maximum of {} in the pooling allocator",
                self.limits.max_core_instances_per_component
            );
        }

        if num_memories > usize::try_from(self.limits.max_memories_per_component).unwrap() {
            bail!(
                "The component transitively contains {num_memories} Wasm linear memories, which \
                 exceeds the configured maximum of {} in the pooling allocator",
                self.limits.max_memories_per_component
            );
        }

        if num_tables > usize::try_from(self.limits.max_tables_per_component).unwrap() {
            bail!(
                "The component transitively contains {num_tables} tables, which exceeds the \
                 configured maximum of {} in the pooling allocator",
                self.limits.max_tables_per_component
            );
        }

        Ok(())
    }

    fn validate_module_impl(&self, module: &Module, offsets: &VMOffsets<HostPtr>) -> Result<()> {
        self.validate_memory_plans(module)
            .context("module memory does not fit in pooling allocator requirements")?;
        self.validate_table_plans(module)
            .context("module table does not fit in pooling allocator requirements")?;
        self.validate_core_instance_size(offsets)
            .context("module instance size does not fit in pooling allocator requirements")?;
        Ok(())
    }

    #[cfg(feature = "gc")]
    fn validate_memory_impl(&self, memory: &wasmtime_environ::Memory) -> Result<()> {
        self.memories.validate_memory(memory)
    }

    #[cfg(feature = "component-model")]
    fn increment_component_instance_count(&self) -> Result<()> {
        let old_count = self.live_component_instances.fetch_add(1, Ordering::AcqRel);
        if old_count >= u64::from(self.limits.total_component_instances) {
            self.decrement_component_instance_count();
            return Err(PoolConcurrencyLimitError::new(
                usize::try_from(self.limits.total_component_instances).unwrap(),
                "component instances",
            )
            .into());
        }
        Ok(())
    }

    #[cfg(feature = "component-model")]
    fn decrement_component_instance_count(&self) {
        self.live_component_instances.fetch_sub(1, Ordering::AcqRel);
    }

    fn increment_core_instance_count(&self) -> Result<()> {
        let old_count = self.live_core_instances.fetch_add(1, Ordering::AcqRel);
        if old_count >= u64::from(self.limits.total_core_instances) {
            self.decrement_core_instance_count();
            return Err(PoolConcurrencyLimitError::new(
                usize::try_from(self.limits.total_core_instances).unwrap(),
                "core instances",
            )
            .into());
        }
        Ok(())
    }

    fn decrement_core_instance_count(&self) {
        self.live_core_instances.fetch_sub(1, Ordering::AcqRel);
    }

    fn allocate_memory(
        &self,
        request: &mut InstanceAllocationRequest,
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        memory_index: Option<DefinedMemoryIndex>,
    ) -> Result<(MemoryAllocationIndex, Memory)> {
        self.with_flush_and_retry(|| self.memories.allocate(request, ty, tunables, memory_index))
    }

    unsafe fn deallocate_memory(
        &self,
        _memory_index: Option<DefinedMemoryIndex>,
        allocation_index: MemoryAllocationIndex,
        memory: Memory,
    ) {
        // Reset the image slot. If there is any error clearing the
        // image, just drop it here, and let the drop handler for the
        // slot unmap in a way that retains the address space
        // reservation.
        let mut image = memory.unwrap_static_image();
        let mut queue = DecommitQueue::default();
        image
            .clear_and_remain_ready(self.memories.keep_resident, |ptr, len| {
                // SAFETY: the memory in `image` won't be used until this
                // decommit queue is flushed, and by definition the memory is
                // not in use when calling this function.
                unsafe {
                    queue.push_raw(ptr, len);
                }
            })
            .expect("failed to reset memory image");

        // SAFETY: this image is not in use and its memory regions were enqueued
        // with `push_raw` above.
        unsafe {
            queue.push_memory(allocation_index, image);
        }
        self.merge_or_flush(queue);
    }

    fn allocate_table(
        &self,
        request: &mut InstanceAllocationRequest,
        ty: &wasmtime_environ::Table,
        tunables: &Tunables,
        _table_index: DefinedTableIndex,
    ) -> Result<(super::TableAllocationIndex, Table)> {
        self.with_flush_and_retry(|| self.tables.allocate(request, ty, tunables))
    }

    unsafe fn deallocate_table(
        &self,
        _table_index: DefinedTableIndex,
        allocation_index: TableAllocationIndex,
        mut table: Table,
    ) {
        let mut queue = DecommitQueue::default();
        // SAFETY: This table is no longer in use by the allocator when this
        // method is called and additionally all image ranges are pushed with
        // the understanding that the memory won't get used until the whole
        // queue is flushed.
        unsafe {
            self.tables
                .reset_table_pages_to_zero(allocation_index, &mut table, |ptr, len| {
                    queue.push_raw(ptr, len);
                });
        }

        // SAFETY: the table has had all its memory regions enqueued above.
        unsafe {
            queue.push_table(allocation_index, table);
        }
        self.merge_or_flush(queue);
    }

    #[cfg(feature = "async")]
    fn allocate_fiber_stack(&self) -> Result<wasmtime_fiber::FiberStack> {
        self.with_flush_and_retry(|| self.stacks.allocate())
    }

    #[cfg(feature = "async")]
    unsafe fn deallocate_fiber_stack(&self, mut stack: wasmtime_fiber::FiberStack) {
        let mut queue = DecommitQueue::default();
        // SAFETY: the stack is no longer in use by definition when this
        // function is called and memory ranges pushed here are otherwise no
        // longer in use.
        unsafe {
            self.stacks
                .zero_stack(&mut stack, |ptr, len| queue.push_raw(ptr, len));
        }
        // SAFETY: this stack's memory regions were enqueued above.
        unsafe {
            queue.push_stack(stack);
        }
        self.merge_or_flush(queue);
    }

    fn purge_module(&self, module: CompiledModuleId) {
        self.memories.purge_module(module);
    }

    fn next_available_pkey(&self) -> Option<ProtectionKey> {
        self.memories.next_available_pkey()
    }

    fn restrict_to_pkey(&self, pkey: ProtectionKey) {
        mpk::allow(ProtectionMask::zero().or(pkey));
    }

    fn allow_all_pkeys(&self) {
        mpk::allow(ProtectionMask::all());
    }

    #[cfg(feature = "gc")]
    fn allocate_gc_heap(
        &self,
        engine: &crate::Engine,
        gc_runtime: &dyn GcRuntime,
        memory_alloc_index: MemoryAllocationIndex,
        memory: Memory,
    ) -> Result<(GcHeapAllocationIndex, Box<dyn GcHeap>)> {
        self.gc_heaps
            .allocate(engine, gc_runtime, memory_alloc_index, memory)
    }

    #[cfg(feature = "gc")]
    fn deallocate_gc_heap(
        &self,
        allocation_index: GcHeapAllocationIndex,
        gc_heap: Box<dyn GcHeap>,
    ) -> (MemoryAllocationIndex, Memory) {
        self.gc_heaps.deallocate(allocation_index, gc_heap)
    }
}

#[cfg(test)]
#[cfg(target_pointer_width = "64")]
mod test {
    use super::*;

    #[test]
    fn test_pooling_allocator_with_memory_pages_exceeded() {
        let config = PoolingInstanceAllocatorConfig {
            limits: InstanceLimits {
                total_memories: 1,
                max_memory_size: 0x100010000,
                ..Default::default()
            },
            ..PoolingInstanceAllocatorConfig::default()
        };
        assert_eq!(
            PoolingInstanceAllocator::new(
                &config,
                &Tunables {
                    memory_reservation: 0x10000,
                    ..Tunables::default_host()
                },
            )
            .map_err(|e| e.to_string())
            .expect_err("expected a failure constructing instance allocator"),
            "maximum memory size of 0x100010000 bytes exceeds the configured \
             memory reservation of 0x10000 bytes"
        );
    }

    #[cfg(all(
        unix,
        target_pointer_width = "64",
        feature = "async",
        not(miri),
        not(asan)
    ))]
    #[test]
    fn test_stack_zeroed() -> Result<()> {
        let config = PoolingInstanceAllocatorConfig {
            max_unused_warm_slots: 0,
            limits: InstanceLimits {
                total_stacks: 1,
                total_memories: 0,
                total_tables: 0,
                ..Default::default()
            },
            stack_size: 128,
            async_stack_zeroing: true,
            ..PoolingInstanceAllocatorConfig::default()
        };
        let allocator = PoolingInstanceAllocator::new(&config, &Tunables::default_host())?;

        unsafe {
            for _ in 0..255 {
                let stack = allocator.allocate_fiber_stack()?;

                // The stack pointer is at the top, so decrement it first
                let addr = stack.top().unwrap().sub(1);

                assert_eq!(*addr, 0);
                *addr = 1;

                allocator.deallocate_fiber_stack(stack);
            }
        }

        Ok(())
    }

    #[cfg(all(
        unix,
        target_pointer_width = "64",
        feature = "async",
        not(miri),
        not(asan)
    ))]
    #[test]
    fn test_stack_unzeroed() -> Result<()> {
        let config = PoolingInstanceAllocatorConfig {
            max_unused_warm_slots: 0,
            limits: InstanceLimits {
                total_stacks: 1,
                total_memories: 0,
                total_tables: 0,
                ..Default::default()
            },
            stack_size: 128,
            async_stack_zeroing: false,
            ..PoolingInstanceAllocatorConfig::default()
        };
        let allocator = PoolingInstanceAllocator::new(&config, &Tunables::default_host())?;

        unsafe {
            for i in 0..255 {
                let stack = allocator.allocate_fiber_stack()?;

                // The stack pointer is at the top, so decrement it first
                let addr = stack.top().unwrap().sub(1);

                assert_eq!(*addr, i);
                *addr = i + 1;

                allocator.deallocate_fiber_stack(stack);
            }
        }

        Ok(())
    }
}
