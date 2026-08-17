use crate::prelude::*;
use crate::runtime::vm::const_expr::{ConstEvalContext, ConstExprEvaluator};
use crate::runtime::vm::imports::Imports;
use crate::runtime::vm::instance::{Instance, InstanceHandle};
use crate::runtime::vm::memory::Memory;
use crate::runtime::vm::mpk::ProtectionKey;
use crate::runtime::vm::table::Table;
use crate::runtime::vm::{CompiledModuleId, ModuleRuntimeInfo, VMFuncRef, VMGcRef, VMStore};
use crate::store::{AutoAssertNoGc, InstanceId, StoreOpaque};
use crate::vm::VMGlobalDefinition;
use core::ptr::NonNull;
use core::{mem, ptr};
use wasmtime_environ::{
    DefinedMemoryIndex, DefinedTableIndex, HostPtr, InitMemory, MemoryInitialization,
    MemoryInitializer, Module, PrimaryMap, SizeOverflow, TableInitialValue, Trap, Tunables,
    VMOffsets, WasmHeapTopType,
};

#[cfg(feature = "gc")]
use crate::runtime::vm::{GcHeap, GcRuntime};

#[cfg(feature = "component-model")]
use wasmtime_environ::{
    StaticModuleIndex,
    component::{Component, VMComponentOffsets},
};

mod on_demand;
pub use self::on_demand::OnDemandInstanceAllocator;

#[cfg(feature = "pooling-allocator")]
mod pooling;
#[cfg(feature = "pooling-allocator")]
pub use self::pooling::{
    InstanceLimits, PoolConcurrencyLimitError, PoolingInstanceAllocator,
    PoolingInstanceAllocatorConfig,
};

/// Represents a request for a new runtime instance.
pub struct InstanceAllocationRequest<'a> {
    /// The instance id that this will be assigned within the store once the
    /// allocation has finished.
    pub id: InstanceId,

    /// The info related to the compiled version of this module,
    /// needed for instantiation: function metadata, JIT code
    /// addresses, precomputed images for lazy memory and table
    /// initialization, and the like. This Arc is cloned and held for
    /// the lifetime of the instance.
    pub runtime_info: &'a ModuleRuntimeInfo,

    /// The imports to use for the instantiation.
    pub imports: Imports<'a>,

    /// A pointer to the "store" for this instance to be allocated. The store
    /// correlates with the `Store` in wasmtime itself, and lots of contextual
    /// information about the execution of wasm can be learned through the
    /// store.
    ///
    /// Note that this is a raw pointer and has a static lifetime, both of which
    /// are a bit of a lie. This is done purely so a store can learn about
    /// itself when it gets called as a host function, and additionally so this
    /// runtime can access internals as necessary (such as the
    /// VMExternRefActivationsTable or the resource limiter methods).
    ///
    /// Note that this ends up being a self-pointer to the instance when stored.
    /// The reason is that the instance itself is then stored within the store.
    /// We use a number of `PhantomPinned` declarations to indicate this to the
    /// compiler. More info on this in `wasmtime/src/store.rs`
    pub store: StorePtr,

    /// Indicates '--wmemcheck' flag.
    #[cfg(feature = "wmemcheck")]
    pub wmemcheck: bool,

    /// Request that the instance's memories be protected by a specific
    /// protection key.
    #[cfg_attr(
        not(feature = "pooling-allocator"),
        expect(
            dead_code,
            reason = "easier to keep this field than remove it, not perf-critical to remove"
        )
    )]
    pub pkey: Option<ProtectionKey>,

    /// Tunable configuration options the engine is using.
    pub tunables: &'a Tunables,
}

/// A pointer to a Store. This Option<*mut dyn Store> is wrapped in a struct
/// so that the function to create a &mut dyn Store is a method on a member of
/// InstanceAllocationRequest, rather than on a &mut InstanceAllocationRequest
/// itself, because several use-sites require a split mut borrow on the
/// InstanceAllocationRequest.
pub struct StorePtr(Option<NonNull<dyn VMStore>>);

// We can't make `VMStore: Send + Sync` because that requires making all of
// Wastime's internals generic over the `Store`'s `T`. So instead, we take care
// in the whole VM layer to only use the `VMStore` in ways that are `Send`- and
// `Sync`-safe and we have to have these unsafe impls.
unsafe impl Send for StorePtr {}
unsafe impl Sync for StorePtr {}

impl StorePtr {
    /// A pointer to no Store.
    pub fn empty() -> Self {
        Self(None)
    }

    /// A pointer to a Store.
    pub fn new(ptr: NonNull<dyn VMStore>) -> Self {
        Self(Some(ptr))
    }

    /// The raw contents of this struct
    pub fn as_raw(&self) -> Option<NonNull<dyn VMStore>> {
        self.0
    }

    /// Use the StorePtr as a mut ref to the Store.
    ///
    /// Safety: must not be used outside the original lifetime of the borrow.
    pub(crate) unsafe fn get(&mut self) -> Option<&mut dyn VMStore> {
        let ptr = unsafe { self.0?.as_mut() };
        Some(ptr)
    }
}

/// The index of a memory allocation within an `InstanceAllocator`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MemoryAllocationIndex(u32);

impl Default for MemoryAllocationIndex {
    fn default() -> Self {
        // A default `MemoryAllocationIndex` that can be used with
        // `InstanceAllocator`s that don't actually need indices.
        MemoryAllocationIndex(u32::MAX)
    }
}

impl MemoryAllocationIndex {
    /// Get the underlying index of this `MemoryAllocationIndex`.
    #[cfg(feature = "pooling-allocator")]
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

/// The index of a table allocation within an `InstanceAllocator`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct TableAllocationIndex(u32);

impl Default for TableAllocationIndex {
    fn default() -> Self {
        // A default `TableAllocationIndex` that can be used with
        // `InstanceAllocator`s that don't actually need indices.
        TableAllocationIndex(u32::MAX)
    }
}

impl TableAllocationIndex {
    /// Get the underlying index of this `TableAllocationIndex`.
    #[cfg(feature = "pooling-allocator")]
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

/// The index of a table allocation within an `InstanceAllocator`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct GcHeapAllocationIndex(u32);

impl Default for GcHeapAllocationIndex {
    fn default() -> Self {
        // A default `GcHeapAllocationIndex` that can be used with
        // `InstanceAllocator`s that don't actually need indices.
        GcHeapAllocationIndex(u32::MAX)
    }
}

impl GcHeapAllocationIndex {
    /// Get the underlying index of this `GcHeapAllocationIndex`.
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

/// Trait that represents the hooks needed to implement an instance allocator.
///
/// Implement this trait when implementing new instance allocators, but don't
/// use this trait when you need an instance allocator. Instead use the
/// `InstanceAllocator` trait for that, which has additional helper methods and
/// a blanket implementation for all types that implement this trait.
///
/// # Safety
///
/// This trait is unsafe as it requires knowledge of Wasmtime's runtime
/// internals to implement correctly.
pub unsafe trait InstanceAllocatorImpl {
    /// Validate whether a component (including all of its contained core
    /// modules) is allocatable by this instance allocator.
    #[cfg(feature = "component-model")]
    fn validate_component_impl<'a>(
        &self,
        component: &Component,
        offsets: &VMComponentOffsets<HostPtr>,
        get_module: &'a dyn Fn(StaticModuleIndex) -> &'a Module,
    ) -> Result<()>;

    /// Validate whether a module is allocatable by this instance allocator.
    fn validate_module_impl(&self, module: &Module, offsets: &VMOffsets<HostPtr>) -> Result<()>;

    /// Validate whether a memory is allocatable by this instance allocator.
    #[cfg(feature = "gc")]
    fn validate_memory_impl(&self, memory: &wasmtime_environ::Memory) -> Result<()>;

    /// Increment the count of concurrent component instances that are currently
    /// allocated, if applicable.
    ///
    /// Not all instance allocators will have limits for the maximum number of
    /// concurrent component instances that can be live at the same time, and
    /// these allocators may implement this method with a no-op.
    //
    // Note: It would be nice to have an associated type that on construction
    // does the increment and on drop does the decrement but there are two
    // problems with this:
    //
    // 1. This trait's implementations are always used as trait objects, and
    //    associated types are not object safe.
    //
    // 2. We would want a parameterized `Drop` implementation so that we could
    //    pass in the `InstanceAllocatorImpl` on drop, but this doesn't exist in
    //    Rust. Therefore, we would be forced to add reference counting and
    //    stuff like that to keep a handle on the instance allocator from this
    //    theoretical type. That's a bummer.
    #[cfg(feature = "component-model")]
    fn increment_component_instance_count(&self) -> Result<()>;

    /// The dual of `increment_component_instance_count`.
    #[cfg(feature = "component-model")]
    fn decrement_component_instance_count(&self);

    /// Increment the count of concurrent core module instances that are
    /// currently allocated, if applicable.
    ///
    /// Not all instance allocators will have limits for the maximum number of
    /// concurrent core module instances that can be live at the same time, and
    /// these allocators may implement this method with a no-op.
    fn increment_core_instance_count(&self) -> Result<()>;

    /// The dual of `increment_core_instance_count`.
    fn decrement_core_instance_count(&self);

    /// Allocate a memory for an instance.
    fn allocate_memory(
        &self,
        request: &mut InstanceAllocationRequest,
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        memory_index: Option<DefinedMemoryIndex>,
    ) -> Result<(MemoryAllocationIndex, Memory)>;

    /// Deallocate an instance's previously allocated memory.
    ///
    /// # Unsafety
    ///
    /// The memory must have previously been allocated by
    /// `Self::allocate_memory`, be at the given index, and must currently be
    /// allocated. It must never be used again.
    unsafe fn deallocate_memory(
        &self,
        memory_index: Option<DefinedMemoryIndex>,
        allocation_index: MemoryAllocationIndex,
        memory: Memory,
    );

    /// Allocate a table for an instance.
    fn allocate_table(
        &self,
        req: &mut InstanceAllocationRequest,
        table: &wasmtime_environ::Table,
        tunables: &Tunables,
        table_index: DefinedTableIndex,
    ) -> Result<(TableAllocationIndex, Table)>;

    /// Deallocate an instance's previously allocated table.
    ///
    /// # Unsafety
    ///
    /// The table must have previously been allocated by `Self::allocate_table`,
    /// be at the given index, and must currently be allocated. It must never be
    /// used again.
    unsafe fn deallocate_table(
        &self,
        table_index: DefinedTableIndex,
        allocation_index: TableAllocationIndex,
        table: Table,
    );

    /// Allocates a fiber stack for calling async functions on.
    #[cfg(feature = "async")]
    fn allocate_fiber_stack(&self) -> Result<wasmtime_fiber::FiberStack>;

    /// Deallocates a fiber stack that was previously allocated with
    /// `allocate_fiber_stack`.
    ///
    /// # Safety
    ///
    /// The provided stack is required to have been allocated with
    /// `allocate_fiber_stack`.
    #[cfg(feature = "async")]
    unsafe fn deallocate_fiber_stack(&self, stack: wasmtime_fiber::FiberStack);

    /// Allocate a GC heap for allocating Wasm GC objects within.
    #[cfg(feature = "gc")]
    fn allocate_gc_heap(
        &self,
        engine: &crate::Engine,
        gc_runtime: &dyn GcRuntime,
        memory_alloc_index: MemoryAllocationIndex,
        memory: Memory,
    ) -> Result<(GcHeapAllocationIndex, Box<dyn GcHeap>)>;

    /// Deallocate a GC heap that was previously allocated with
    /// `allocate_gc_heap`.
    #[cfg(feature = "gc")]
    #[must_use = "it is the caller's responsibility to deallocate the GC heap's underlying memory \
                  storage after the GC heap is deallocated"]
    fn deallocate_gc_heap(
        &self,
        allocation_index: GcHeapAllocationIndex,
        gc_heap: Box<dyn GcHeap>,
    ) -> (MemoryAllocationIndex, Memory);

    /// Purges all lingering resources related to `module` from within this
    /// allocator.
    ///
    /// Primarily present for the pooling allocator to remove mappings of
    /// this module from slots in linear memory.
    fn purge_module(&self, module: CompiledModuleId);

    /// Use the next available protection key.
    ///
    /// The pooling allocator can use memory protection keys (MPK) for
    /// compressing the guard regions protecting against OOB. Each
    /// pool-allocated store needs its own key.
    fn next_available_pkey(&self) -> Option<ProtectionKey>;

    /// Restrict access to memory regions protected by `pkey`.
    ///
    /// This is useful for the pooling allocator, which can use memory
    /// protection keys (MPK). Note: this may still allow access to other
    /// protection keys, such as the default kernel key; see implementations of
    /// this.
    fn restrict_to_pkey(&self, pkey: ProtectionKey);

    /// Allow access to memory regions protected by any protection key.
    fn allow_all_pkeys(&self);
}

/// A thing that can allocate instances.
///
/// Don't implement this trait directly, instead implement
/// `InstanceAllocatorImpl` and you'll get this trait for free via a blanket
/// impl.
pub trait InstanceAllocator: InstanceAllocatorImpl {
    /// Validate whether a component (including all of its contained core
    /// modules) is allocatable with this instance allocator.
    #[cfg(feature = "component-model")]
    fn validate_component<'a>(
        &self,
        component: &Component,
        offsets: &VMComponentOffsets<HostPtr>,
        get_module: &'a dyn Fn(StaticModuleIndex) -> &'a Module,
    ) -> Result<()> {
        InstanceAllocatorImpl::validate_component_impl(self, component, offsets, get_module)
    }

    /// Validate whether a core module is allocatable with this instance
    /// allocator.
    fn validate_module(&self, module: &Module, offsets: &VMOffsets<HostPtr>) -> Result<()> {
        InstanceAllocatorImpl::validate_module_impl(self, module, offsets)
    }

    /// Validate whether a memory is allocatable with this instance allocator.
    #[cfg(feature = "gc")]
    fn validate_memory(&self, memory: &wasmtime_environ::Memory) -> Result<()> {
        InstanceAllocatorImpl::validate_memory_impl(self, memory)
    }

    /// Allocates a fresh `InstanceHandle` for the `req` given.
    ///
    /// This will allocate memories and tables internally from this allocator
    /// and weave that altogether into a final and complete `InstanceHandle`
    /// ready to be registered with a store.
    ///
    /// Note that the returned instance must still have `.initialize(..)` called
    /// on it to complete the instantiation process.
    ///
    /// # Safety
    ///
    /// The `request` provided must be valid, e.g. the imports within are
    /// correctly sized/typed for the instance being created.
    unsafe fn allocate_module(
        &self,
        mut request: InstanceAllocationRequest,
    ) -> Result<InstanceHandle> {
        let module = request.runtime_info.env_module();

        #[cfg(debug_assertions)]
        InstanceAllocatorImpl::validate_module_impl(self, module, request.runtime_info.offsets())
            .expect("module should have already been validated before allocation");

        self.increment_core_instance_count()?;

        let num_defined_memories = module.num_defined_memories();
        let mut memories = PrimaryMap::with_capacity(num_defined_memories);

        let num_defined_tables = module.num_defined_tables();
        let mut tables = PrimaryMap::with_capacity(num_defined_tables);

        match (|| {
            self.allocate_memories(&mut request, &mut memories)?;
            self.allocate_tables(&mut request, &mut tables)?;
            Ok(())
        })() {
            // SAFETY: memories/tables were just allocated from the store within
            // `request` and this function's own contract requires that the
            // imports are valid.
            Ok(_) => unsafe { Ok(Instance::new(request, memories, tables, &module.memories)) },
            Err(e) => {
                // SAFETY: these were previously allocated by this allocator
                unsafe {
                    self.deallocate_memories(&mut memories);
                    self.deallocate_tables(&mut tables);
                }
                self.decrement_core_instance_count();
                Err(e)
            }
        }
    }

    /// Deallocates the provided instance.
    ///
    /// This will null-out the pointer within `handle` and otherwise reclaim
    /// resources such as tables, memories, and the instance memory itself.
    ///
    /// # Unsafety
    ///
    /// The instance must have previously been allocated by `Self::allocate`.
    unsafe fn deallocate_module(&self, handle: &mut InstanceHandle) {
        // SAFETY: the contract of `deallocate_*` is itself a contract of this
        // function, that the memories/tables were previously allocated from
        // here.
        unsafe {
            self.deallocate_memories(handle.get_mut().memories_mut());
            self.deallocate_tables(handle.get_mut().tables_mut());
        }

        self.decrement_core_instance_count();
    }

    /// Allocate the memories for the given instance allocation request, pushing
    /// them into `memories`.
    fn allocate_memories(
        &self,
        request: &mut InstanceAllocationRequest,
        memories: &mut PrimaryMap<DefinedMemoryIndex, (MemoryAllocationIndex, Memory)>,
    ) -> Result<()> {
        let module = request.runtime_info.env_module();

        #[cfg(debug_assertions)]
        InstanceAllocatorImpl::validate_module_impl(self, module, request.runtime_info.offsets())
            .expect("module should have already been validated before allocation");

        for (memory_index, ty) in module.memories.iter().skip(module.num_imported_memories) {
            let memory_index = module
                .defined_memory_index(memory_index)
                .expect("should be a defined memory since we skipped imported ones");

            let memory = self.allocate_memory(request, ty, request.tunables, Some(memory_index))?;
            memories.push(memory);
        }

        Ok(())
    }

    /// Deallocate all the memories in the given primary map.
    ///
    /// # Unsafety
    ///
    /// The memories must have previously been allocated by
    /// `Self::allocate_memories`.
    unsafe fn deallocate_memories(
        &self,
        memories: &mut PrimaryMap<DefinedMemoryIndex, (MemoryAllocationIndex, Memory)>,
    ) {
        for (memory_index, (allocation_index, memory)) in mem::take(memories) {
            // Because deallocating memory is infallible, we don't need to worry
            // about leaking subsequent memories if the first memory failed to
            // deallocate. If deallocating memory ever becomes fallible, we will
            // need to be careful here!
            //
            // SAFETY: the unsafe contract here is the same as the unsafe
            // contract of this function, that the memories were previously
            // allocated by this allocator.
            unsafe {
                self.deallocate_memory(Some(memory_index), allocation_index, memory);
            }
        }
    }

    /// Allocate tables for the given instance allocation request, pushing them
    /// into `tables`.
    fn allocate_tables(
        &self,
        request: &mut InstanceAllocationRequest,
        tables: &mut PrimaryMap<DefinedTableIndex, (TableAllocationIndex, Table)>,
    ) -> Result<()> {
        let module = request.runtime_info.env_module();

        #[cfg(debug_assertions)]
        InstanceAllocatorImpl::validate_module_impl(self, module, request.runtime_info.offsets())
            .expect("module should have already been validated before allocation");

        for (index, table) in module.tables.iter().skip(module.num_imported_tables) {
            let def_index = module
                .defined_table_index(index)
                .expect("should be a defined table since we skipped imported ones");

            let table = self.allocate_table(request, table, request.tunables, def_index)?;
            tables.push(table);
        }

        Ok(())
    }

    /// Deallocate all the tables in the given primary map.
    ///
    /// # Unsafety
    ///
    /// The tables must have previously been allocated by
    /// `Self::allocate_tables`.
    unsafe fn deallocate_tables(
        &self,
        tables: &mut PrimaryMap<DefinedTableIndex, (TableAllocationIndex, Table)>,
    ) {
        for (table_index, (allocation_index, table)) in mem::take(tables) {
            // SAFETY: the tables here were allocated from this allocator per
            // the contract on this function itself.
            unsafe {
                self.deallocate_table(table_index, allocation_index, table);
            }
        }
    }
}

// Every `InstanceAllocatorImpl` is an `InstanceAllocator` when used
// correctly. Also, no one is allowed to override this trait's methods, they
// must use the defaults. This blanket impl provides both of those things.
impl<T: InstanceAllocatorImpl> InstanceAllocator for T {}

fn check_table_init_bounds(
    store: &mut StoreOpaque,
    instance: InstanceId,
    module: &Module,
) -> Result<()> {
    let mut const_evaluator = ConstExprEvaluator::default();

    for segment in module.table_initialization.segments.iter() {
        let mut context = ConstEvalContext::new(instance);
        let start = unsafe {
            const_evaluator
                .eval(store, &mut context, &segment.offset)
                .expect("const expression should be valid")
        };
        let start = usize::try_from(start.get_u32()).unwrap();
        let end = start.checked_add(usize::try_from(segment.elements.len()).unwrap());

        let table = store.instance_mut(instance).get_table(segment.table_index);
        match end {
            Some(end) if end <= table.size() => {
                // Initializer is in bounds
            }
            _ => {
                bail!("table out of bounds: elements segment does not fit")
            }
        }
    }

    Ok(())
}

fn initialize_tables(
    store: &mut StoreOpaque,
    context: &mut ConstEvalContext,
    const_evaluator: &mut ConstExprEvaluator,
    module: &Module,
) -> Result<()> {
    for (table, init) in module.table_initialization.initial_values.iter() {
        match init {
            // Tables are always initially null-initialized at this time
            TableInitialValue::Null { precomputed: _ } => {}

            TableInitialValue::Expr(expr) => {
                let raw = unsafe {
                    const_evaluator
                        .eval(store, context, expr)
                        .expect("const expression should be valid")
                };
                let idx = module.table_index(table);
                match module.tables[idx].ref_type.heap_type.top() {
                    WasmHeapTopType::Extern => {
                        store.gc_store_mut()?;
                        let (gc_store, instance) =
                            store.optional_gc_store_and_instance_mut(context.instance);
                        let gc_store = gc_store.unwrap();
                        let table = instance.get_defined_table(table);
                        let gc_ref = VMGcRef::from_raw_u32(raw.get_externref());
                        let items = (0..table.size())
                            .map(|_| gc_ref.as_ref().map(|r| gc_store.clone_gc_ref(r)));
                        table.init_gc_refs(0, items)?;
                    }

                    WasmHeapTopType::Any => {
                        store.gc_store_mut()?;
                        let (gc_store, instance) =
                            store.optional_gc_store_and_instance_mut(context.instance);
                        let gc_store = gc_store.unwrap();
                        let table = instance.get_defined_table(table);
                        let gc_ref = VMGcRef::from_raw_u32(raw.get_anyref());
                        let items = (0..table.size())
                            .map(|_| gc_ref.as_ref().map(|r| gc_store.clone_gc_ref(r)));
                        table.init_gc_refs(0, items)?;
                    }

                    WasmHeapTopType::Exn => {
                        store.gc_store_mut()?;
                        let (gc_store, instance) =
                            store.optional_gc_store_and_instance_mut(context.instance);
                        let gc_store = gc_store.unwrap();
                        let table = instance.get_defined_table(table);
                        let gc_ref = VMGcRef::from_raw_u32(raw.get_exnref());
                        let items = (0..table.size())
                            .map(|_| gc_ref.as_ref().map(|r| gc_store.clone_gc_ref(r)));
                        table.init_gc_refs(0, items)?;
                    }

                    WasmHeapTopType::Func => {
                        let table = store
                            .instance_mut(context.instance)
                            .get_defined_table(table);
                        let funcref = NonNull::new(raw.get_funcref().cast::<VMFuncRef>());
                        let items = (0..table.size()).map(|_| funcref);
                        table.init_func(0, items)?;
                    }

                    WasmHeapTopType::Cont => todo!(), // FIXME: #10248 stack switching support.
                }
            }
        }
    }

    // Note: if the module's table initializer state is in
    // FuncTable mode, we will lazily initialize tables based on
    // any statically-precomputed image of FuncIndexes, but there
    // may still be "leftover segments" that could not be
    // incorporated. So we have a unified handler here that
    // iterates over all segments (Segments mode) or leftover
    // segments (FuncTable mode) to initialize.
    for segment in module.table_initialization.segments.iter() {
        let start = unsafe {
            const_evaluator
                .eval(store, context, &segment.offset)
                .expect("const expression should be valid")
        };
        Instance::table_init_segment(
            store,
            context.instance,
            const_evaluator,
            segment.table_index,
            &segment.elements,
            start.get_u64(),
            0,
            segment.elements.len(),
        )?;
    }

    Ok(())
}

fn get_memory_init_start(
    store: &mut StoreOpaque,
    init: &MemoryInitializer,
    instance: InstanceId,
) -> Result<u64> {
    let mut context = ConstEvalContext::new(instance);
    let mut const_evaluator = ConstExprEvaluator::default();
    unsafe { const_evaluator.eval(store, &mut context, &init.offset) }.map(|v| {
        match store.instance(instance).env_module().memories[init.memory_index].idx_type {
            wasmtime_environ::IndexType::I32 => v.get_u32().into(),
            wasmtime_environ::IndexType::I64 => v.get_u64(),
        }
    })
}

fn check_memory_init_bounds(
    store: &mut StoreOpaque,
    instance: InstanceId,
    initializers: &[MemoryInitializer],
) -> Result<()> {
    for init in initializers {
        let memory = store.instance_mut(instance).get_memory(init.memory_index);
        let start = get_memory_init_start(store, init, instance)?;
        let end = usize::try_from(start)
            .ok()
            .and_then(|start| start.checked_add(init.data.len()));

        match end {
            Some(end) if end <= memory.current_length() => {
                // Initializer is in bounds
            }
            _ => {
                bail!("memory out of bounds: data segment does not fit")
            }
        }
    }

    Ok(())
}

fn initialize_memories(
    store: &mut StoreOpaque,
    context: &mut ConstEvalContext,
    const_evaluator: &mut ConstExprEvaluator,
    module: &Module,
) -> Result<()> {
    // Delegates to the `init_memory` method which is sort of a duplicate of
    // `instance.memory_init_segment` but is used at compile-time in other
    // contexts so is shared here to have only one method of memory
    // initialization.
    //
    // This call to `init_memory` notably implements all the bells and whistles
    // so errors only happen if an out-of-bounds segment is found, in which case
    // a trap is returned.

    struct InitMemoryAtInstantiation<'a> {
        module: &'a Module,
        store: &'a mut StoreOpaque,
        context: &'a mut ConstEvalContext,
        const_evaluator: &'a mut ConstExprEvaluator,
    }

    impl InitMemory for InitMemoryAtInstantiation<'_> {
        fn memory_size_in_bytes(
            &mut self,
            memory: wasmtime_environ::MemoryIndex,
        ) -> Result<u64, SizeOverflow> {
            let len = self
                .store
                .instance(self.context.instance)
                .get_memory(memory)
                .current_length();
            let len = u64::try_from(len).unwrap();
            Ok(len)
        }

        fn eval_offset(
            &mut self,
            memory: wasmtime_environ::MemoryIndex,
            expr: &wasmtime_environ::ConstExpr,
        ) -> Option<u64> {
            let val = unsafe { self.const_evaluator.eval(self.store, self.context, expr) }
                .expect("const expression should be valid");
            Some(
                match self
                    .store
                    .instance(self.context.instance)
                    .env_module()
                    .memories[memory]
                    .idx_type
                {
                    wasmtime_environ::IndexType::I32 => val.get_u32().into(),
                    wasmtime_environ::IndexType::I64 => val.get_u64(),
                },
            )
        }

        fn write(
            &mut self,
            memory_index: wasmtime_environ::MemoryIndex,
            init: &wasmtime_environ::StaticMemoryInitializer,
        ) -> bool {
            // If this initializer applies to a defined memory but that memory
            // doesn't need initialization, due to something like copy-on-write
            // pre-initializing it via mmap magic, then this initializer can be
            // skipped entirely.
            let instance = self.store.instance_mut(self.context.instance);
            if let Some(memory_index) = self.module.defined_memory_index(memory_index) {
                if !instance.memories[memory_index].1.needs_init() {
                    return true;
                }
            }
            let memory = instance.get_memory(memory_index);

            unsafe {
                let src = instance.wasm_data(init.data.clone());
                let offset = usize::try_from(init.offset).unwrap();
                let dst = memory.base.as_ptr().add(offset);

                assert!(offset + src.len() <= memory.current_length());

                // FIXME audit whether this is safe in the presence of shared
                // memory
                // (https://github.com/bytecodealliance/wasmtime/issues/4203).
                ptr::copy_nonoverlapping(src.as_ptr(), dst, src.len())
            }
            true
        }
    }

    let ok = module
        .memory_initialization
        .init_memory(&mut InitMemoryAtInstantiation {
            module,
            store,
            context,
            const_evaluator,
        });
    if !ok {
        return Err(Trap::MemoryOutOfBounds.into());
    }

    Ok(())
}

fn check_init_bounds(store: &mut StoreOpaque, instance: InstanceId, module: &Module) -> Result<()> {
    check_table_init_bounds(store, instance, module)?;

    match &module.memory_initialization {
        MemoryInitialization::Segmented(initializers) => {
            check_memory_init_bounds(store, instance, initializers)?;
        }
        // Statically validated already to have everything in-bounds.
        MemoryInitialization::Static { .. } => {}
    }

    Ok(())
}

fn initialize_globals(
    store: &mut StoreOpaque,
    context: &mut ConstEvalContext,
    const_evaluator: &mut ConstExprEvaluator,
    module: &Module,
) -> Result<()> {
    assert!(core::ptr::eq(
        &**store.instance(context.instance).env_module(),
        module
    ));

    let mut store = AutoAssertNoGc::new(store);

    for (index, init) in module.global_initializers.iter() {
        let raw = unsafe {
            const_evaluator
                .eval(&mut store, context, init)
                .expect("should be a valid const expr")
        };

        let instance = store.instance_mut(context.instance);
        let to = instance.global_ptr(index);
        let wasm_ty = module.globals[module.global_index(index)].wasm_ty;

        #[cfg(feature = "wmemcheck")]
        if index.as_u32() == 0 && wasm_ty == wasmtime_environ::WasmValType::I32 {
            if let Some(wmemcheck) = instance.wmemcheck_state_mut() {
                let size = usize::try_from(raw.get_i32()).unwrap();
                wmemcheck.set_stack_size(size);
            }
        }

        // This write is safe because we know we have the correct module for
        // this instance and its vmctx due to the assert above.
        unsafe {
            to.write(VMGlobalDefinition::from_val_raw(&mut store, wasm_ty, raw)?);
        };
    }
    Ok(())
}

pub fn initialize_instance(
    store: &mut StoreOpaque,
    instance: InstanceId,
    module: &Module,
    is_bulk_memory: bool,
) -> Result<()> {
    // If bulk memory is not enabled, bounds check the data and element segments before
    // making any changes. With bulk memory enabled, initializers are processed
    // in-order and side effects are observed up to the point of an out-of-bounds
    // initializer, so the early checking is not desired.
    if !is_bulk_memory {
        check_init_bounds(store, instance, module)?;
    }

    let mut context = ConstEvalContext::new(instance);
    let mut const_evaluator = ConstExprEvaluator::default();

    initialize_globals(store, &mut context, &mut const_evaluator, module)?;
    initialize_tables(store, &mut context, &mut const_evaluator, module)?;
    initialize_memories(store, &mut context, &mut const_evaluator, &module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocator_traits_are_object_safe() {
        fn _instance_allocator(_: &dyn InstanceAllocatorImpl) {}
        fn _instance_allocator_ext(_: &dyn InstanceAllocator) {}
    }
}
