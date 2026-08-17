use crate::linker::{Definition, DefinitionType};
use crate::prelude::*;
use crate::runtime::vm::{
    self, Imports, ModuleRuntimeInfo, VMFuncRef, VMFunctionImport, VMGlobalImport, VMMemoryImport,
    VMTableImport, VMTagImport,
};
use crate::store::{AllocateInstanceKind, InstanceId, StoreInstanceId, StoreOpaque};
use crate::types::matching;
use crate::{
    AsContextMut, Engine, Export, Extern, Func, Global, Memory, Module, ModuleExport, SharedMemory,
    StoreContext, StoreContextMut, Table, Tag, TypedFunc,
};
use alloc::sync::Arc;
use core::ptr::NonNull;
use wasmparser::WasmFeatures;
use wasmtime_environ::{
    EntityIndex, EntityType, FuncIndex, GlobalIndex, MemoryIndex, PrimaryMap, TableIndex, TagIndex,
    TypeTrace,
};

/// An instantiated WebAssembly module.
///
/// This type represents the instantiation of a [`Module`]. Once instantiated
/// you can access the [`exports`](Instance::exports) which are of type
/// [`Extern`] and provide the ability to call functions, set globals, read
/// memory, etc. When interacting with any wasm code you'll want to make an
/// [`Instance`] to call any code or execute anything.
///
/// Instances are owned by a [`Store`](crate::Store) which is passed in at
/// creation time. It's recommended to create instances with
/// [`Linker::instantiate`](crate::Linker::instantiate) or similar
/// [`Linker`](crate::Linker) methods, but a more low-level constructor is also
/// available as [`Instance::new`].
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Instance {
    id: StoreInstanceId,
}

// Double-check that the C representation in `instance.h` matches our in-Rust
// representation here in terms of size/alignment/etc.
const _: () = {
    #[repr(C)]
    struct C(u64, usize);
    assert!(core::mem::size_of::<C>() == core::mem::size_of::<Instance>());
    assert!(core::mem::align_of::<C>() == core::mem::align_of::<Instance>());
    assert!(core::mem::offset_of!(Instance, id) == 0);
};

impl Instance {
    /// Creates a new [`Instance`] from the previously compiled [`Module`] and
    /// list of `imports` specified.
    ///
    /// This method instantiates the `module` provided with the `imports`,
    /// following the procedure in the [core specification][inst] to
    /// instantiate. Instantiation can fail for a number of reasons (many
    /// specified below), but if successful the `start` function will be
    /// automatically run (if specified in the `module`) and then the
    /// [`Instance`] will be returned.
    ///
    /// Per the WebAssembly spec, instantiation includes running the module's
    /// start function, if it has one (not to be confused with the `_start`
    /// function, which is not run).
    ///
    /// Note that this is a low-level function that just performs an
    /// instantiation. See the [`Linker`](crate::Linker) struct for an API which
    /// provides a convenient way to link imports and provides automatic Command
    /// and Reactor behavior.
    ///
    /// ## Providing Imports
    ///
    /// The entries in the list of `imports` are intended to correspond 1:1
    /// with the list of imports returned by [`Module::imports`]. Before
    /// calling [`Instance::new`] you'll want to inspect the return value of
    /// [`Module::imports`] and, for each import type, create an [`Extern`]
    /// which corresponds to that type.  These [`Extern`] values are all then
    /// collected into a list and passed to this function.
    ///
    /// Note that this function is intentionally relatively low level. For an
    /// easier time passing imports by doing name-based resolution it's
    /// recommended to instead use the [`Linker`](crate::Linker) type.
    ///
    /// ## Errors
    ///
    /// This function can fail for a number of reasons, including, but not
    /// limited to:
    ///
    /// * The number of `imports` provided doesn't match the number of imports
    ///   returned by the `module`'s [`Module::imports`] method.
    /// * The type of any [`Extern`] doesn't match the corresponding
    ///   [`ExternType`] entry that it maps to.
    /// * The `start` function in the instance, if present, traps.
    /// * Module/instance resource limits are exceeded.
    ///
    /// When instantiation fails it's recommended to inspect the return value to
    /// see why it failed, or bubble it upwards. If you'd like to specifically
    /// check for trap errors, you can use `error.downcast::<Trap>()`. For more
    /// about error handling see the [`Trap`] documentation.
    ///
    /// [`Trap`]: crate::Trap
    ///
    /// # Panics
    ///
    /// This function will panic if called with a store associated with a
    /// [`asynchronous config`](crate::Config::async_support). This function
    /// will also panic if any [`Extern`] supplied is not owned by `store`.
    ///
    /// [inst]: https://webassembly.github.io/spec/core/exec/modules.html#exec-instantiation
    /// [`ExternType`]: crate::ExternType
    pub fn new(
        mut store: impl AsContextMut,
        module: &Module,
        imports: &[Extern],
    ) -> Result<Instance> {
        let mut store = store.as_context_mut();
        let imports = Instance::typecheck_externs(store.0, module, imports)?;
        // Note that the unsafety here should be satisfied by the call to
        // `typecheck_externs` above which satisfies the condition that all
        // the imports are valid for this module.
        unsafe { Instance::new_started(&mut store, module, imports.as_ref()) }
    }

    /// Same as [`Instance::new`], except for usage in [asynchronous stores].
    ///
    /// For more details about this function see the documentation on
    /// [`Instance::new`]. The only difference between these two methods is that
    /// this one will asynchronously invoke the wasm start function in case it
    /// calls any imported function which is an asynchronous host function (e.g.
    /// created with [`Func::new_async`](crate::Func::new_async).
    ///
    /// # Panics
    ///
    /// This function will panic if called with a store associated with a
    /// [`synchronous config`](crate::Config::new). This is only compatible with
    /// stores associated with an [`asynchronous
    /// config`](crate::Config::async_support).
    ///
    /// This function will also panic, like [`Instance::new`], if any [`Extern`]
    /// specified does not belong to `store`.
    #[cfg(feature = "async")]
    pub async fn new_async(
        mut store: impl AsContextMut<Data: Send>,
        module: &Module,
        imports: &[Extern],
    ) -> Result<Instance> {
        let mut store = store.as_context_mut();
        let imports = Instance::typecheck_externs(store.0, module, imports)?;
        // See `new` for notes on this unsafety
        unsafe { Instance::new_started_async(&mut store, module, imports.as_ref()).await }
    }

    fn typecheck_externs(
        store: &mut StoreOpaque,
        module: &Module,
        imports: &[Extern],
    ) -> Result<OwnedImports> {
        for import in imports {
            if !import.comes_from_same_store(store) {
                bail!("cross-`Store` instantiation is not currently supported");
            }
        }

        typecheck(module, imports, |cx, ty, item| {
            let item = DefinitionType::from(store, item);
            cx.definition(ty, &item)
        })?;

        // When pushing functions into `OwnedImports` it's required that their
        // `wasm_call` fields are all filled out. This `module` is guaranteed
        // to have any trampolines necessary for functions so register the
        // module with the store and then attempt to fill out any outstanding
        // holes.
        //
        // Note that under normal operation this shouldn't do much as the list
        // of funcs-with-holes should generally be empty. As a result the
        // process of filling this out is not super optimized at this point.
        store.modules_mut().register_module(module);
        let (funcrefs, modules) = store.func_refs_and_modules();
        funcrefs.fill(modules);

        let mut owned_imports = OwnedImports::new(module);
        for import in imports {
            owned_imports.push(import, store);
        }
        Ok(owned_imports)
    }

    /// Internal function to create an instance and run the start function.
    ///
    /// This function's unsafety is the same as `Instance::new_raw`.
    pub(crate) unsafe fn new_started<T>(
        store: &mut StoreContextMut<'_, T>,
        module: &Module,
        imports: Imports<'_>,
    ) -> Result<Instance> {
        assert!(
            !store.0.async_support(),
            "must use async instantiation when async support is enabled",
        );

        // SAFETY: the safety contract of `new_started_impl` is the same as this
        // function.
        unsafe { Self::new_started_impl(store, module, imports) }
    }

    /// Internal function to create an instance and run the start function.
    ///
    /// ONLY CALL THIS IF YOU HAVE ALREADY CHECKED FOR ASYNCNESS AND HANDLED
    /// THE FIBER NONSENSE
    pub(crate) unsafe fn new_started_impl<T>(
        store: &mut StoreContextMut<'_, T>,
        module: &Module,
        imports: Imports<'_>,
    ) -> Result<Instance> {
        // SAFETY: the safety contract of `new_raw` is the same as this
        // function.
        let (instance, start) = unsafe { Instance::new_raw(store.0, module, imports)? };
        if let Some(start) = start {
            instance.start_raw(store, start)?;
        }
        Ok(instance)
    }

    /// Internal function to create an instance and run the start function.
    ///
    /// This function's unsafety is the same as `Instance::new_raw`.
    #[cfg(feature = "async")]
    async unsafe fn new_started_async<T>(
        store: &mut StoreContextMut<'_, T>,
        module: &Module,
        imports: Imports<'_>,
    ) -> Result<Instance>
    where
        T: Send + 'static,
    {
        assert!(
            store.0.async_support(),
            "must use sync instantiation when async support is disabled",
        );

        store
            .on_fiber(|store| {
                // SAFETY: the unsafe contract of `new_started_impl` is the same
                // as this function.
                unsafe { Self::new_started_impl(store, module, imports) }
            })
            .await?
    }

    /// Internal function to create an instance which doesn't have its `start`
    /// function run yet.
    ///
    /// This is not intended to be exposed from Wasmtime, it's intended to
    /// refactor out common code from `new_started` and `new_started_async`.
    ///
    /// Note that this step needs to be run on a fiber in async mode even
    /// though it doesn't do any blocking work because an async resource
    /// limiter may need to yield.
    ///
    /// # Unsafety
    ///
    /// This method is unsafe because it does not type-check the `imports`
    /// provided. The `imports` provided must be suitable for the module
    /// provided as well.
    unsafe fn new_raw(
        store: &mut StoreOpaque,
        module: &Module,
        imports: Imports<'_>,
    ) -> Result<(Instance, Option<FuncIndex>)> {
        if !Engine::same(store.engine(), module.engine()) {
            bail!("cross-`Engine` instantiation is not currently supported");
        }
        store.bump_resource_counts(module)?;

        // Allocate the GC heap, if necessary.
        if module.env_module().needs_gc_heap {
            let _ = store.gc_store_mut()?;
        }

        let compiled_module = module.compiled_module();

        // Register the module just before instantiation to ensure we keep the module
        // properly referenced while in use by the store.
        let module_id = store.modules_mut().register_module(module);

        // The first thing we do is issue an instance allocation request
        // to the instance allocator. This, on success, will give us an
        // instance handle.
        //
        // SAFETY: this module, by construction, was already validated within
        // the store.
        let id = unsafe {
            store.allocate_instance(
                AllocateInstanceKind::Module(module_id),
                &ModuleRuntimeInfo::Module(module.clone()),
                imports,
            )?
        };

        // Additionally, before we start doing fallible instantiation, we
        // do one more step which is to insert an `InstanceData`
        // corresponding to this instance. This `InstanceData` can be used
        // via `Caller::get_export` if our instance's state "leaks" into
        // other instances, even if we don't return successfully from this
        // function.
        //
        // We don't actually load all exports from the instance at this
        // time, instead preferring to lazily load them as they're demanded.
        // For module/instance exports, though, those aren't actually
        // stored in the instance handle so we need to immediately handle
        // those here.
        let instance = Instance::from_wasmtime(id, store);

        // Now that we've recorded all information we need to about this
        // instance within a `Store` we can start performing fallible
        // initialization. Note that we still defer the `start` function to
        // later since that may need to run asynchronously.
        //
        // If this returns an error (or if the start function traps) then
        // any other initialization which may have succeeded which placed
        // items from this instance into other instances should be ok when
        // those items are loaded and run we'll have all the metadata to
        // look at them.
        let bulk_memory = store
            .engine()
            .features()
            .contains(WasmFeatures::BULK_MEMORY);

        vm::initialize_instance(store, id, compiled_module.module(), bulk_memory)?;

        Ok((instance, compiled_module.module().start_func))
    }

    pub(crate) fn from_wasmtime(id: InstanceId, store: &mut StoreOpaque) -> Instance {
        Instance {
            id: StoreInstanceId::new(store.id(), id),
        }
    }

    fn start_raw<T>(&self, store: &mut StoreContextMut<'_, T>, start: FuncIndex) -> Result<()> {
        // If a start function is present, invoke it. Make sure we use all the
        // trap-handling configuration in `store` as well.
        let store_id = store.0.id();
        let mut instance = self.id.get_mut(store.0);
        // SAFETY: the `store_id` is the id of the store that owns this
        // instance and any function stored within the instance.
        let f = unsafe { instance.as_mut().get_exported_func(store_id, start) };
        let caller_vmctx = instance.vmctx();
        unsafe {
            let funcref = f.vm_func_ref(store.0);
            super::func::invoke_wasm_and_catch_traps(store, |_default_caller, vm| {
                VMFuncRef::array_call(funcref, vm, caller_vmctx, NonNull::from(&mut []))
            })?;
        }
        Ok(())
    }

    /// Get this instance's module.
    pub fn module<'a, T: 'static>(&self, store: impl Into<StoreContext<'a, T>>) -> &'a Module {
        self._module(store.into().0)
    }

    fn _module<'a>(&self, store: &'a StoreOpaque) -> &'a Module {
        store.module_for_instance(self.id).unwrap()
    }

    /// Returns the list of exported items from this [`Instance`].
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn exports<'a, T: 'static>(
        &'a self,
        store: impl Into<StoreContextMut<'a, T>>,
    ) -> impl ExactSizeIterator<Item = Export<'a>> + 'a {
        self._exports(store.into().0)
    }

    fn _exports<'a>(
        &'a self,
        store: &'a mut StoreOpaque,
    ) -> impl ExactSizeIterator<Item = Export<'a>> + 'a {
        let module = store[self.id].env_module().clone();
        let mut items = Vec::new();
        for (_name, entity) in module.exports.iter() {
            items.push(self._get_export(store, *entity));
        }
        store[self.id]
            .env_module()
            .exports
            .iter()
            .zip(items)
            .map(|((name, _), item)| Export::new(name, item))
    }

    /// Looks up an exported [`Extern`] value by name.
    ///
    /// This method will search the module for an export named `name` and return
    /// the value, if found.
    ///
    /// Returns `None` if there was no export named `name`.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    ///
    /// # Why does `get_export` take a mutable context?
    ///
    /// This method requires a mutable context because an instance's exports are
    /// lazily populated, and we cache them as they are accessed. This makes
    /// instantiating a module faster, but also means this method requires a
    /// mutable context.
    pub fn get_export(&self, mut store: impl AsContextMut, name: &str) -> Option<Extern> {
        let store = store.as_context_mut().0;
        let entity = *store[self.id].env_module().exports.get(name)?;
        Some(self._get_export(store, entity))
    }

    /// Looks up an exported [`Extern`] value by a [`ModuleExport`] value.
    ///
    /// This is similar to [`Instance::get_export`] but uses a [`ModuleExport`] value to avoid
    /// string lookups where possible. [`ModuleExport`]s can be obtained by calling
    /// [`Module::get_export_index`] on the [`Module`] that this instance was instantiated with.
    ///
    /// This method will search the module for an export with a matching entity index and return
    /// the value, if found.
    ///
    /// Returns `None` if there was no export with a matching entity index.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_module_export(
        &self,
        mut store: impl AsContextMut,
        export: &ModuleExport,
    ) -> Option<Extern> {
        let store = store.as_context_mut().0;

        // Verify the `ModuleExport` matches the module used in this instance.
        if self._module(store).id() != export.module {
            return None;
        }

        Some(self._get_export(store, export.entity))
    }

    fn _get_export(&self, store: &mut StoreOpaque, entity: EntityIndex) -> Extern {
        let id = store.id();
        // SAFETY: the store `id` owns this instance and all exports contained
        // within.
        let export = unsafe { self.id.get_mut(store).get_export_by_index_mut(id, entity) };
        unsafe { Extern::from_wasmtime_export(export, store) }
    }

    /// Looks up an exported [`Func`] value by name.
    ///
    /// Returns `None` if there was no export named `name`, or if there was but
    /// it wasn't a function.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_func(&self, store: impl AsContextMut, name: &str) -> Option<Func> {
        self.get_export(store, name)?.into_func()
    }

    /// Looks up an exported [`Func`] value by name and with its type.
    ///
    /// This function is a convenience wrapper over [`Instance::get_func`] and
    /// [`Func::typed`]. For more information see the linked documentation.
    ///
    /// Returns an error if `name` isn't a function export or if the export's
    /// type did not match `Params` or `Results`
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_typed_func<Params, Results>(
        &self,
        mut store: impl AsContextMut,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>>
    where
        Params: crate::WasmParams,
        Results: crate::WasmResults,
    {
        let f = self
            .get_export(store.as_context_mut(), name)
            .and_then(|f| f.into_func())
            .ok_or_else(|| anyhow!("failed to find function export `{}`", name))?;
        Ok(f.typed::<Params, Results>(store)
            .with_context(|| format!("failed to convert function `{name}` to given type"))?)
    }

    /// Looks up an exported [`Table`] value by name.
    ///
    /// Returns `None` if there was no export named `name`, or if there was but
    /// it wasn't a table.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_table(&self, store: impl AsContextMut, name: &str) -> Option<Table> {
        self.get_export(store, name)?.into_table()
    }

    /// Looks up an exported [`Memory`] value by name.
    ///
    /// Returns `None` if there was no export named `name`, or if there was but
    /// it wasn't a memory.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_memory(&self, store: impl AsContextMut, name: &str) -> Option<Memory> {
        self.get_export(store, name)?.into_memory()
    }

    /// Looks up an exported [`SharedMemory`] value by name.
    ///
    /// Returns `None` if there was no export named `name`, or if there was but
    /// it wasn't a shared memory.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_shared_memory(
        &self,
        mut store: impl AsContextMut,
        name: &str,
    ) -> Option<SharedMemory> {
        let mut store = store.as_context_mut();
        self.get_export(&mut store, name)?.into_shared_memory()
    }

    /// Looks up an exported [`Global`] value by name.
    ///
    /// Returns `None` if there was no export named `name`, or if there was but
    /// it wasn't a global.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_global(&self, store: impl AsContextMut, name: &str) -> Option<Global> {
        self.get_export(store, name)?.into_global()
    }

    /// Looks up a tag [`Tag`] by name.
    ///
    /// Returns `None` if there was no export named `name`, or if there was but
    /// it wasn't a tag.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_tag(&self, store: impl AsContextMut, name: &str) -> Option<Tag> {
        self.get_export(store, name)?.into_tag()
    }

    #[allow(
        dead_code,
        reason = "c-api crate does not yet support exnrefs and causes this method to be dead."
    )]
    pub(crate) fn id(&self) -> InstanceId {
        self.id.instance()
    }

    /// Get all globals within this instance.
    ///
    /// Returns both import and defined globals.
    ///
    /// Returns both exported and non-exported globals.
    ///
    /// Gives access to the full globals space.
    #[cfg(feature = "coredump")]
    pub(crate) fn all_globals<'a>(
        &'a self,
        store: &'a mut StoreOpaque,
    ) -> impl ExactSizeIterator<Item = (GlobalIndex, Global)> + 'a {
        let store_id = store.id();
        store[self.id].all_globals(store_id)
    }

    /// Get all memories within this instance.
    ///
    /// Returns both import and defined memories.
    ///
    /// Returns both exported and non-exported memories.
    ///
    /// Gives access to the full memories space.
    #[cfg(feature = "coredump")]
    pub(crate) fn all_memories<'a>(
        &'a self,
        store: &'a StoreOpaque,
    ) -> impl ExactSizeIterator<Item = (MemoryIndex, Memory)> + 'a {
        let store_id = store.id();
        store[self.id].all_memories(store_id)
    }
}

pub(crate) struct OwnedImports {
    functions: PrimaryMap<FuncIndex, VMFunctionImport>,
    tables: PrimaryMap<TableIndex, VMTableImport>,
    memories: PrimaryMap<MemoryIndex, VMMemoryImport>,
    globals: PrimaryMap<GlobalIndex, VMGlobalImport>,
    tags: PrimaryMap<TagIndex, VMTagImport>,
}

impl OwnedImports {
    fn new(module: &Module) -> OwnedImports {
        let mut ret = OwnedImports::empty();
        ret.reserve(module);
        return ret;
    }

    pub(crate) fn empty() -> OwnedImports {
        OwnedImports {
            functions: PrimaryMap::new(),
            tables: PrimaryMap::new(),
            memories: PrimaryMap::new(),
            globals: PrimaryMap::new(),
            tags: PrimaryMap::new(),
        }
    }

    pub(crate) fn reserve(&mut self, module: &Module) {
        let raw = module.compiled_module().module();
        self.functions.reserve(raw.num_imported_funcs);
        self.tables.reserve(raw.num_imported_tables);
        self.memories.reserve(raw.num_imported_memories);
        self.globals.reserve(raw.num_imported_globals);
        self.tags.reserve(raw.num_imported_tags);
    }

    #[cfg(feature = "component-model")]
    pub(crate) fn clear(&mut self) {
        self.functions.clear();
        self.tables.clear();
        self.memories.clear();
        self.globals.clear();
        self.tags.clear();
    }

    fn push(&mut self, item: &Extern, store: &mut StoreOpaque) {
        match item {
            Extern::Func(i) => {
                self.functions.push(i.vmimport(store));
            }
            Extern::Global(i) => {
                self.globals.push(i.vmimport(store));
            }
            Extern::Table(i) => {
                self.tables.push(i.vmimport(store));
            }
            Extern::Memory(i) => {
                self.memories.push(i.vmimport(store));
            }
            Extern::SharedMemory(i) => {
                self.memories.push(i.vmimport(store));
            }
            Extern::Tag(i) => {
                self.tags.push(i.vmimport(store));
            }
        }
    }

    /// Note that this is unsafe as the validity of `item` is not verified and
    /// it contains a bunch of raw pointers.
    #[cfg(feature = "component-model")]
    pub(crate) fn push_export(&mut self, store: &StoreOpaque, item: &crate::runtime::vm::Export) {
        match item {
            crate::runtime::vm::Export::Function(f) => {
                // SAFETY: the funcref associated with a `Func` is valid to use
                // under the `store` that owns the function.
                let f = unsafe { f.vm_func_ref(store).as_ref() };
                self.functions.push(VMFunctionImport {
                    wasm_call: f.wasm_call.unwrap(),
                    array_call: f.array_call,
                    vmctx: f.vmctx,
                });
            }
            crate::runtime::vm::Export::Global(g) => {
                self.globals.push(g.vmimport(store));
            }
            crate::runtime::vm::Export::Table(t) => {
                self.tables.push(t.vmimport(store));
            }
            crate::runtime::vm::Export::Memory { memory, .. } => {
                self.memories.push(memory.vmimport(store));
            }
            crate::runtime::vm::Export::Tag(t) => {
                self.tags.push(t.vmimport(store));
            }
        }
    }

    pub(crate) fn as_ref(&self) -> Imports<'_> {
        Imports {
            tables: self.tables.values().as_slice(),
            globals: self.globals.values().as_slice(),
            memories: self.memories.values().as_slice(),
            functions: self.functions.values().as_slice(),
            tags: self.tags.values().as_slice(),
        }
    }
}

/// An instance, pre-instantiation, that is ready to be instantiated.
///
/// This structure represents an instance *just before* it was instantiated,
/// after all type-checking and imports have been resolved. The only thing left
/// to do for this instance is to actually run the process of instantiation.
///
/// Note that an `InstancePre` may not be tied to any particular [`Store`] if
/// none of the imports it closed over are tied to any particular [`Store`].
///
/// This structure is created through the [`Linker::instantiate_pre`] method,
/// which also has some more information and examples.
///
/// [`Store`]: crate::Store
/// [`Linker::instantiate_pre`]: crate::Linker::instantiate_pre
pub struct InstancePre<T> {
    module: Module,

    /// The items which this `InstancePre` use to instantiate the `module`
    /// provided, passed to `Instance::new_started` after inserting them into a
    /// `Store`.
    ///
    /// Note that this is stored as an `Arc<[T]>` to quickly move a strong
    /// reference to everything internally into a `Store<T>` without having to
    /// clone each individual item.
    items: Arc<[Definition]>,

    /// A count of `Definition::HostFunc` entries in `items` above to
    /// preallocate space in a `Store` up front for all entries to be inserted.
    host_funcs: usize,

    /// The `VMFuncRef`s for the functions in `items` that do not
    /// have a `wasm_call` trampoline. We pre-allocate and pre-patch these
    /// `VMFuncRef`s so that we don't have to do it at
    /// instantiation time.
    ///
    /// This is an `Arc<[T]>` for the same reason as `items`.
    func_refs: Arc<[VMFuncRef]>,

    _marker: core::marker::PhantomData<fn() -> T>,
}

/// InstancePre's clone does not require T: Clone
impl<T> Clone for InstancePre<T> {
    fn clone(&self) -> Self {
        Self {
            module: self.module.clone(),
            items: self.items.clone(),
            host_funcs: self.host_funcs,
            func_refs: self.func_refs.clone(),
            _marker: self._marker,
        }
    }
}

impl<T: 'static> InstancePre<T> {
    /// Creates a new `InstancePre` which type-checks the `items` provided and
    /// on success is ready to instantiate a new instance.
    ///
    /// # Unsafety
    ///
    /// This method is unsafe as the `T` of the `InstancePre<T>` is not
    /// guaranteed to be the same as the `T` within the `Store`, the caller must
    /// verify that.
    pub(crate) unsafe fn new(module: &Module, items: Vec<Definition>) -> Result<InstancePre<T>> {
        typecheck(module, &items, |cx, ty, item| cx.definition(ty, &item.ty()))?;

        let mut func_refs = vec![];
        let mut host_funcs = 0;
        for item in &items {
            match item {
                Definition::Extern(_, _) => {}
                Definition::HostFunc(f) => {
                    host_funcs += 1;
                    if f.func_ref().wasm_call.is_none() {
                        // `f` needs its `VMFuncRef::wasm_call` patched with a
                        // Wasm-to-native trampoline.
                        debug_assert!(matches!(f.host_ctx(), crate::HostContext::Array(_)));
                        func_refs.push(VMFuncRef {
                            wasm_call: module
                                .wasm_to_array_trampoline(f.sig_index())
                                .map(|f| f.into()),
                            ..*f.func_ref()
                        });
                    }
                }
            }
        }

        Ok(InstancePre {
            module: module.clone(),
            items: items.into(),
            host_funcs,
            func_refs: func_refs.into(),
            _marker: core::marker::PhantomData,
        })
    }

    /// Returns a reference to the module that this [`InstancePre`] will be
    /// instantiating.
    pub fn module(&self) -> &Module {
        &self.module
    }

    /// Instantiates this instance, creating a new instance within the provided
    /// `store`.
    ///
    /// This function will run the actual process of instantiation to
    /// completion. This will use all of the previously-closed-over items as
    /// imports to instantiate the module that this was originally created with.
    ///
    /// For more information about instantiation see [`Instance::new`].
    ///
    /// # Panics
    ///
    /// Panics if any import closed over by this [`InstancePre`] isn't owned by
    /// `store`, or if `store` has async support enabled. Additionally this
    /// function will panic if the `store` provided comes from a different
    /// [`Engine`] than the [`InstancePre`] originally came from.
    pub fn instantiate(&self, mut store: impl AsContextMut<Data = T>) -> Result<Instance> {
        let mut store = store.as_context_mut();
        let imports = pre_instantiate_raw(
            &mut store.0,
            &self.module,
            &self.items,
            self.host_funcs,
            &self.func_refs,
        )?;

        // This unsafety should be handled by the type-checking performed by the
        // constructor of `InstancePre` to assert that all the imports we're passing
        // in match the module we're instantiating.
        unsafe { Instance::new_started(&mut store, &self.module, imports.as_ref()) }
    }

    /// Creates a new instance, running the start function asynchronously
    /// instead of inline.
    ///
    /// For more information about asynchronous instantiation see the
    /// documentation on [`Instance::new_async`].
    ///
    /// # Panics
    ///
    /// Panics if any import closed over by this [`InstancePre`] isn't owned by
    /// `store`, or if `store` does not have async support enabled.
    #[cfg(feature = "async")]
    pub async fn instantiate_async(
        &self,
        mut store: impl AsContextMut<Data = T>,
    ) -> Result<Instance>
    where
        T: Send,
    {
        let mut store = store.as_context_mut();
        let imports = pre_instantiate_raw(
            &mut store.0,
            &self.module,
            &self.items,
            self.host_funcs,
            &self.func_refs,
        )?;

        // This unsafety should be handled by the type-checking performed by the
        // constructor of `InstancePre` to assert that all the imports we're passing
        // in match the module we're instantiating.
        unsafe { Instance::new_started_async(&mut store, &self.module, imports.as_ref()).await }
    }
}

/// Helper function shared between
/// `InstancePre::{instantiate,instantiate_async}`
///
/// This is an out-of-line function to avoid the generic on `InstancePre` and
/// get this compiled into the `wasmtime` crate to avoid having it monomorphized
/// elsewhere.
fn pre_instantiate_raw(
    store: &mut StoreOpaque,
    module: &Module,
    items: &Arc<[Definition]>,
    host_funcs: usize,
    func_refs: &Arc<[VMFuncRef]>,
) -> Result<OwnedImports> {
    // Register this module and use it to fill out any funcref wasm_call holes
    // we can. For more comments on this see `typecheck_externs`.
    store.modules_mut().register_module(module);
    let (funcrefs, modules) = store.func_refs_and_modules();
    funcrefs.fill(modules);

    if host_funcs > 0 {
        // Any linker-defined function of the `Definition::HostFunc` variant
        // will insert a function into the store automatically as part of
        // instantiation, so reserve space here to make insertion more efficient
        // as it won't have to realloc during the instantiation.
        funcrefs.reserve_storage(host_funcs);

        // The usage of `to_extern_store_rooted` requires that the items are
        // rooted via another means, which happens here by cloning the list of
        // items into the store once. This avoids cloning each individual item
        // below.
        funcrefs.push_instance_pre_definitions(items.clone());
        funcrefs.push_instance_pre_func_refs(func_refs.clone());
    }

    let mut func_refs = func_refs.iter().map(|f| NonNull::from(f));
    let mut imports = OwnedImports::new(module);
    for import in items.iter() {
        if !import.comes_from_same_store(store) {
            bail!("cross-`Store` instantiation is not currently supported");
        }
        // This unsafety should be encapsulated in the constructor of
        // `InstancePre` where the `T` of the original item should match the
        // `T` of the store. Additionally the rooting necessary has happened
        // above.
        let item = match import {
            Definition::Extern(e, _) => e.clone(),
            Definition::HostFunc(func) => unsafe {
                func.to_func_store_rooted(
                    store,
                    if func.func_ref().wasm_call.is_none() {
                        Some(func_refs.next().unwrap())
                    } else {
                        None
                    },
                )
                .into()
            },
        };
        imports.push(&item, store);
    }

    Ok(imports)
}

fn typecheck<I>(
    module: &Module,
    import_args: &[I],
    check: impl Fn(&matching::MatchCx<'_>, &EntityType, &I) -> Result<()>,
) -> Result<()> {
    let env_module = module.compiled_module().module();
    let expected_len = env_module.imports().count();
    let actual_len = import_args.len();
    if expected_len != actual_len {
        bail!("expected {expected_len} imports, found {actual_len}");
    }
    let cx = matching::MatchCx::new(module.engine());
    for ((name, field, expected_ty), actual) in env_module.imports().zip(import_args) {
        debug_assert!(expected_ty.is_canonicalized_for_runtime_usage());
        check(&cx, &expected_ty, actual)
            .with_context(|| format!("incompatible import type for `{name}::{field}`"))?;
    }
    Ok(())
}
