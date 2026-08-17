use super::{
    InstanceAllocationRequest, InstanceAllocatorImpl, MemoryAllocationIndex, TableAllocationIndex,
};
use crate::prelude::*;
use crate::runtime::vm::CompiledModuleId;
use crate::runtime::vm::instance::RuntimeMemoryCreator;
use crate::runtime::vm::memory::{DefaultMemoryCreator, Memory};
use crate::runtime::vm::mpk::ProtectionKey;
use crate::runtime::vm::table::Table;
use alloc::sync::Arc;
use wasmtime_environ::{
    DefinedMemoryIndex, DefinedTableIndex, HostPtr, Module, Tunables, VMOffsets,
};

#[cfg(feature = "gc")]
use crate::runtime::vm::{GcHeap, GcHeapAllocationIndex, GcRuntime};

#[cfg(feature = "async")]
use wasmtime_fiber::RuntimeFiberStackCreator;

#[cfg(feature = "component-model")]
use wasmtime_environ::{
    StaticModuleIndex,
    component::{Component, VMComponentOffsets},
};

/// Represents the on-demand instance allocator.
#[derive(Clone)]
pub struct OnDemandInstanceAllocator {
    mem_creator: Option<Arc<dyn RuntimeMemoryCreator>>,
    #[cfg(feature = "async")]
    stack_creator: Option<Arc<dyn RuntimeFiberStackCreator>>,
    #[cfg(feature = "async")]
    stack_size: usize,
    #[cfg(feature = "async")]
    stack_zeroing: bool,
}

impl OnDemandInstanceAllocator {
    /// Creates a new on-demand instance allocator.
    pub fn new(
        mem_creator: Option<Arc<dyn RuntimeMemoryCreator>>,
        stack_size: usize,
        stack_zeroing: bool,
    ) -> Self {
        let _ = (stack_size, stack_zeroing); // suppress warnings when async feature is disabled.
        Self {
            mem_creator,
            #[cfg(feature = "async")]
            stack_creator: None,
            #[cfg(feature = "async")]
            stack_size,
            #[cfg(feature = "async")]
            stack_zeroing,
        }
    }

    /// Set the stack creator.
    #[cfg(feature = "async")]
    pub fn set_stack_creator(&mut self, stack_creator: Arc<dyn RuntimeFiberStackCreator>) {
        self.stack_creator = Some(stack_creator);
    }
}

impl Default for OnDemandInstanceAllocator {
    fn default() -> Self {
        Self {
            mem_creator: None,
            #[cfg(feature = "async")]
            stack_creator: None,
            #[cfg(feature = "async")]
            stack_size: 0,
            #[cfg(feature = "async")]
            stack_zeroing: false,
        }
    }
}

unsafe impl InstanceAllocatorImpl for OnDemandInstanceAllocator {
    #[cfg(feature = "component-model")]
    fn validate_component_impl<'a>(
        &self,
        _component: &Component,
        _offsets: &VMComponentOffsets<HostPtr>,
        _get_module: &'a dyn Fn(StaticModuleIndex) -> &'a Module,
    ) -> Result<()> {
        Ok(())
    }

    fn validate_module_impl(&self, _module: &Module, _offsets: &VMOffsets<HostPtr>) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "gc")]
    fn validate_memory_impl(&self, _memory: &wasmtime_environ::Memory) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "component-model")]
    fn increment_component_instance_count(&self) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "component-model")]
    fn decrement_component_instance_count(&self) {}

    fn increment_core_instance_count(&self) -> Result<()> {
        Ok(())
    }

    fn decrement_core_instance_count(&self) {}

    fn allocate_memory(
        &self,
        request: &mut InstanceAllocationRequest,
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        memory_index: Option<DefinedMemoryIndex>,
    ) -> Result<(MemoryAllocationIndex, Memory)> {
        let creator = self
            .mem_creator
            .as_deref()
            .unwrap_or_else(|| &DefaultMemoryCreator);

        let image = if let Some(memory_index) = memory_index {
            request.runtime_info.memory_image(memory_index)?
        } else {
            None
        };

        let allocation_index = MemoryAllocationIndex::default();
        let memory = Memory::new_dynamic(
            ty,
            tunables,
            creator,
            unsafe {
                request
                    .store
                    .get()
                    .expect("if module has memory plans, store is not empty")
            },
            image,
        )?;
        Ok((allocation_index, memory))
    }

    unsafe fn deallocate_memory(
        &self,
        _memory_index: Option<DefinedMemoryIndex>,
        allocation_index: MemoryAllocationIndex,
        _memory: Memory,
    ) {
        debug_assert_eq!(allocation_index, MemoryAllocationIndex::default());
        // Normal destructors do all the necessary clean up.
    }

    fn allocate_table(
        &self,
        request: &mut InstanceAllocationRequest,
        ty: &wasmtime_environ::Table,
        tunables: &Tunables,
        _table_index: DefinedTableIndex,
    ) -> Result<(TableAllocationIndex, Table)> {
        let allocation_index = TableAllocationIndex::default();
        let table = Table::new_dynamic(ty, tunables, unsafe {
            request
                .store
                .get()
                .expect("if module has table plans, store is not empty")
        })?;
        Ok((allocation_index, table))
    }

    unsafe fn deallocate_table(
        &self,
        _table_index: DefinedTableIndex,
        allocation_index: TableAllocationIndex,
        _table: Table,
    ) {
        debug_assert_eq!(allocation_index, TableAllocationIndex::default());
        // Normal destructors do all the necessary clean up.
    }

    #[cfg(feature = "async")]
    fn allocate_fiber_stack(&self) -> Result<wasmtime_fiber::FiberStack> {
        if self.stack_size == 0 {
            anyhow::bail!("fiber stacks are not supported by the allocator")
        }
        let stack = match &self.stack_creator {
            Some(stack_creator) => {
                let stack = stack_creator.new_stack(self.stack_size, self.stack_zeroing)?;
                wasmtime_fiber::FiberStack::from_custom(stack)
            }
            None => wasmtime_fiber::FiberStack::new(self.stack_size, self.stack_zeroing),
        }?;
        Ok(stack)
    }

    #[cfg(feature = "async")]
    unsafe fn deallocate_fiber_stack(&self, stack: wasmtime_fiber::FiberStack) {
        // The on-demand allocator has no further bookkeeping for fiber stacks
        // beyond dropping them.
        let _ = stack;
    }

    fn purge_module(&self, _: CompiledModuleId) {}

    fn next_available_pkey(&self) -> Option<ProtectionKey> {
        // The on-demand allocator cannot use protection keys--it requires
        // back-to-back allocation of memory slots that this allocator cannot
        // guarantee.
        None
    }

    fn restrict_to_pkey(&self, _: ProtectionKey) {
        // The on-demand allocator cannot use protection keys; an on-demand
        // allocator will never hand out protection keys to the stores its
        // engine creates.
        unreachable!()
    }

    fn allow_all_pkeys(&self) {
        // The on-demand allocator cannot use protection keys; an on-demand
        // allocator will never hand out protection keys to the stores its
        // engine creates.
        unreachable!()
    }

    #[cfg(feature = "gc")]
    fn allocate_gc_heap(
        &self,
        engine: &crate::Engine,
        gc_runtime: &dyn GcRuntime,
        memory_alloc_index: MemoryAllocationIndex,
        memory: Memory,
    ) -> Result<(GcHeapAllocationIndex, Box<dyn GcHeap>)> {
        debug_assert_eq!(memory_alloc_index, MemoryAllocationIndex::default());
        let mut heap = gc_runtime.new_gc_heap(engine)?;
        heap.attach(memory);
        Ok((GcHeapAllocationIndex::default(), heap))
    }

    #[cfg(feature = "gc")]
    fn deallocate_gc_heap(
        &self,
        allocation_index: GcHeapAllocationIndex,
        mut gc_heap: Box<dyn crate::runtime::vm::GcHeap>,
    ) -> (MemoryAllocationIndex, Memory) {
        debug_assert_eq!(allocation_index, GcHeapAllocationIndex::default());
        (MemoryAllocationIndex::default(), gc_heap.detach())
    }
}
