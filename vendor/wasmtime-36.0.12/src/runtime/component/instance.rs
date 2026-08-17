use crate::component::func::HostFunc;
use crate::component::matching::InstanceType;
use crate::component::store::{ComponentInstanceId, StoreComponentInstanceId};
use crate::component::{
    Component, ComponentExportIndex, ComponentNamedList, Func, Lift, Lower, ResourceType,
    TypedFunc, types::ComponentItem,
};
use crate::instance::OwnedImports;
use crate::linker::DefinitionType;
use crate::prelude::*;
use crate::runtime::vm::component::{
    CallContexts, ComponentInstance, ResourceTables, TypedResource, TypedResourceIndex,
};
use crate::runtime::vm::{self, VMFuncRef};
use crate::store::StoreOpaque;
use crate::{AsContext, AsContextMut, Engine, Module, StoreContextMut};
use alloc::sync::Arc;
use core::marker;
use core::pin::Pin;
use core::ptr::NonNull;
use wasmtime_environ::{EngineOrModuleTypeIndex, component::*};
use wasmtime_environ::{EntityIndex, EntityType, PrimaryMap};

/// An instantiated component.
///
/// This type represents an instantiated [`Component`](super::Component).
/// Instances have exports which can be accessed through functions such as
/// [`Instance::get_func`] or [`Instance::get_export`]. Instances are owned by a
/// [`Store`](crate::Store) and all methods require a handle to the store.
///
/// Component instances are created through
/// [`Linker::instantiate`](super::Linker::instantiate) and its family of
/// methods.
///
/// This type is similar to the core wasm version
/// [`wasmtime::Instance`](crate::Instance) except that it represents an
/// instantiated component instead of an instantiated module.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Instance {
    id: StoreComponentInstanceId,
}

// Double-check that the C representation in `component/instance.h` matches our
// in-Rust representation here in terms of size/alignment/etc.
const _: () = {
    #[repr(C)]
    struct C(u64, u32);
    assert!(core::mem::size_of::<C>() == core::mem::size_of::<Instance>());
    assert!(core::mem::align_of::<C>() == core::mem::align_of::<Instance>());
    assert!(core::mem::offset_of!(Instance, id) == 0);
};

impl Instance {
    /// Creates a raw `Instance` from the internal identifiers within the store.
    pub(crate) fn from_wasmtime(store: &StoreOpaque, id: ComponentInstanceId) -> Instance {
        Instance {
            id: StoreComponentInstanceId::new(store.id(), id),
        }
    }

    /// Looks up an exported function by name within this [`Instance`].
    ///
    /// The `store` argument provided must be the store that this instance
    /// lives within and the `name` argument is the lookup key by which to find
    /// the exported function. If the function is found then `Some` is returned
    /// and otherwise `None` is returned.
    ///
    /// The `name` here can be a string such as `&str` or it can be a
    /// [`ComponentExportIndex`] which is loaded prior from a [`Component`].
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    ///
    /// # Examples
    ///
    /// Looking up a function which is exported from the root of a component:
    ///
    /// ```
    /// use wasmtime::{Engine, Store};
    /// use wasmtime::component::{Component, Linker};
    ///
    /// # fn main() -> wasmtime::Result<()> {
    /// let engine = Engine::default();
    /// let component = Component::new(
    ///     &engine,
    ///     r#"
    ///         (component
    ///             (core module $m
    ///                 (func (export "f"))
    ///             )
    ///             (core instance $i (instantiate $m))
    ///             (func (export "f")
    ///                 (canon lift (core func $i "f")))
    ///         )
    ///     "#,
    /// )?;
    ///
    /// // Look up the function by name
    /// let mut store = Store::new(&engine, ());
    /// let instance = Linker::new(&engine).instantiate(&mut store, &component)?;
    /// let func = instance.get_func(&mut store, "f").unwrap();
    ///
    /// // The function can also be looked up by an index via a precomputed index.
    /// let export = component.get_export_index(None, "f").unwrap();
    /// let func = instance.get_func(&mut store, &export).unwrap();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Looking up a function which is exported from a nested instance:
    ///
    /// ```
    /// use wasmtime::{Engine, Store};
    /// use wasmtime::component::{Component, Linker};
    ///
    /// # fn main() -> wasmtime::Result<()> {
    /// let engine = Engine::default();
    /// let component = Component::new(
    ///     &engine,
    ///     r#"
    ///         (component
    ///             (core module $m
    ///                 (func (export "f"))
    ///             )
    ///             (core instance $i (instantiate $m))
    ///             (func $f
    ///                 (canon lift (core func $i "f")))
    ///
    ///             (instance $i
    ///                 (export "f" (func $f)))
    ///             (export "i" (instance $i))
    ///         )
    ///     "#,
    /// )?;
    ///
    /// // First look up the exported instance, then use that to lookup the
    /// // exported function.
    /// let instance_index = component.get_export_index(None, "i").unwrap();
    /// let func_index = component.get_export_index(Some(&instance_index), "f").unwrap();
    ///
    /// // Then use `func_index` at runtime.
    /// let mut store = Store::new(&engine, ());
    /// let instance = Linker::new(&engine).instantiate(&mut store, &component)?;
    /// let func = instance.get_func(&mut store, &func_index).unwrap();
    ///
    /// // Alternatively the `instance` can be used directly in conjunction with
    /// // the `get_export_index` method.
    /// let instance_index = instance.get_export_index(&mut store, None, "i").unwrap();
    /// let func_index = instance.get_export_index(&mut store, Some(&instance_index), "f").unwrap();
    /// let func = instance.get_func(&mut store, &func_index).unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_func(
        &self,
        mut store: impl AsContextMut,
        name: impl InstanceExportLookup,
    ) -> Option<Func> {
        let store = store.as_context_mut().0;
        let instance = self.id.get(store);
        let component = instance.component();

        // Validate that `name` exists within `self.`
        let index = name.lookup(component)?;

        // Validate that `index` is indeed a lifted function.
        match &component.env_component().export_items[index] {
            Export::LiftedFunction { .. } => {}
            _ => return None,
        }

        // And package up the indices!
        Some(Func::from_lifted_func(*self, index))
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
        name: impl InstanceExportLookup,
    ) -> Result<TypedFunc<Params, Results>>
    where
        Params: ComponentNamedList + Lower,
        Results: ComponentNamedList + Lift,
    {
        let f = self
            .get_func(store.as_context_mut(), name)
            .ok_or_else(|| anyhow!("failed to find function export"))?;
        Ok(f.typed::<Params, Results>(store)
            .with_context(|| format!("failed to convert function to given type"))?)
    }

    /// Looks up an exported module by name within this [`Instance`].
    ///
    /// The `store` argument provided must be the store that this instance
    /// lives within and the `name` argument is the lookup key by which to find
    /// the exported module. If the module is found then `Some` is returned
    /// and otherwise `None` is returned.
    ///
    /// The `name` here can be a string such as `&str` or it can be a
    /// [`ComponentExportIndex`] which is loaded prior from a [`Component`].
    ///
    /// For some examples see [`Instance::get_func`] for loading values from a
    /// component.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_module(
        &self,
        mut store: impl AsContextMut,
        name: impl InstanceExportLookup,
    ) -> Option<Module> {
        let store = store.as_context_mut().0;
        let (instance, export) = self.lookup_export(store, name)?;
        match export {
            Export::ModuleStatic { index, .. } => {
                Some(instance.component().static_module(*index).clone())
            }
            Export::ModuleImport { import, .. } => match instance.runtime_import(*import) {
                RuntimeImport::Module(m) => Some(m.clone()),
                _ => unreachable!(),
            },
            _ => None,
        }
    }

    /// Looks up an exported resource type by name within this [`Instance`].
    ///
    /// The `store` argument provided must be the store that this instance
    /// lives within and the `name` argument is the lookup key by which to find
    /// the exported resource. If the resource is found then `Some` is returned
    /// and otherwise `None` is returned.
    ///
    /// The `name` here can be a string such as `&str` or it can be a
    /// [`ComponentExportIndex`] which is loaded prior from a [`Component`].
    ///
    /// For some examples see [`Instance::get_func`] for loading values from a
    /// component.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_resource(
        &self,
        mut store: impl AsContextMut,
        name: impl InstanceExportLookup,
    ) -> Option<ResourceType> {
        let store = store.as_context_mut().0;
        let (instance, export) = self.lookup_export(store, name)?;
        match export {
            Export::Type(TypeDef::Resource(id)) => {
                Some(InstanceType::new(instance).resource_type(*id))
            }
            Export::Type(_)
            | Export::LiftedFunction { .. }
            | Export::ModuleStatic { .. }
            | Export::ModuleImport { .. }
            | Export::Instance { .. } => None,
        }
    }

    /// A methods similar to [`Component::get_export`] except for this
    /// instance.
    ///
    /// This method will lookup the `name` provided within the `instance`
    /// provided and return a [`ComponentItem`] describing the export,
    /// and [`ComponentExportIndex`] which can be passed other `get_*`
    /// functions like [`Instance::get_func`].
    ///
    /// The [`ComponentItem`] is more expensive to compute than the
    /// [`ComponentExportIndex`]. If you are not consuming the
    /// [`ComponentItem`], use [`Instance::get_export_index`] instead.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_export(
        &self,
        mut store: impl AsContextMut,
        instance: Option<&ComponentExportIndex>,
        name: &str,
    ) -> Option<(ComponentItem, ComponentExportIndex)> {
        self._get_export(store.as_context_mut().0, instance, name)
    }

    fn _get_export(
        &self,
        store: &StoreOpaque,
        instance: Option<&ComponentExportIndex>,
        name: &str,
    ) -> Option<(ComponentItem, ComponentExportIndex)> {
        let data = self.id().get(store);
        let component = data.component();
        let index = component.lookup_export_index(instance, name)?;
        let item = ComponentItem::from_export(
            &store.engine(),
            &component.env_component().export_items[index],
            &InstanceType::new(data),
        );
        Some((
            item,
            ComponentExportIndex {
                id: data.component().id(),
                index,
            },
        ))
    }

    /// A methods similar to [`Component::get_export_index`] except for this
    /// instance.
    ///
    /// This method will lookup the `name` provided within the `instance`
    /// provided and return a [`ComponentExportIndex`] which can be passed
    /// other `get_*` functions like [`Instance::get_func`].
    ///
    /// If you need the [`ComponentItem`] corresponding to this export, use
    /// the [`Instance::get_export`] instead.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this instance.
    pub fn get_export_index(
        &self,
        mut store: impl AsContextMut,
        instance: Option<&ComponentExportIndex>,
        name: &str,
    ) -> Option<ComponentExportIndex> {
        let data = self.id().get(store.as_context_mut().0);
        let index = data.component().lookup_export_index(instance, name)?;
        Some(ComponentExportIndex {
            id: data.component().id(),
            index,
        })
    }

    fn lookup_export<'a>(
        &self,
        store: &'a StoreOpaque,
        name: impl InstanceExportLookup,
    ) -> Option<(&'a ComponentInstance, &'a Export)> {
        let data = self.id().get(store);
        let index = name.lookup(data.component())?;
        Some((data, &data.component().env_component().export_items[index]))
    }

    /// Returns the [`InstancePre`] that was used to create this instance.
    pub fn instance_pre<T>(&self, store: impl AsContext<Data = T>) -> InstancePre<T> {
        // This indexing operation asserts the Store owns the Instance.
        // Therefore, the InstancePre<T> must match the Store<T>.
        let data = self.id().get(store.as_context().0);

        // SAFETY: calling this method safely here relies on matching the `T`
        // in `InstancePre<T>` to the store itself, which is happening in the
        // type signature just above by ensuring the store's data is `T` which
        // matches the return value.
        unsafe { data.instance_pre() }
    }

    pub(crate) fn id(&self) -> StoreComponentInstanceId {
        self.id
    }

    /// Implementation of the `resource.new` intrinsic for `i32`
    /// representations.
    pub(crate) fn resource_new32(
        self,
        store: &mut StoreOpaque,
        ty: TypeResourceTableIndex,
        rep: u32,
    ) -> Result<u32> {
        let (calls, _, _, instance) = store.component_resource_state_with_instance(self);
        resource_tables(calls, instance).resource_new(TypedResource::Component { ty, rep })
    }

    /// Implementation of the `resource.rep` intrinsic for `i32`
    /// representations.
    pub(crate) fn resource_rep32(
        self,
        store: &mut StoreOpaque,
        ty: TypeResourceTableIndex,
        index: u32,
    ) -> Result<u32> {
        let (calls, _, _, instance) = store.component_resource_state_with_instance(self);
        resource_tables(calls, instance).resource_rep(TypedResourceIndex::Component { ty, index })
    }

    /// Implementation of the `resource.drop` intrinsic.
    pub(crate) fn resource_drop(
        self,
        store: &mut StoreOpaque,
        ty: TypeResourceTableIndex,
        index: u32,
    ) -> Result<Option<u32>> {
        let (calls, _, _, instance) = store.component_resource_state_with_instance(self);
        resource_tables(calls, instance).resource_drop(TypedResourceIndex::Component { ty, index })
    }

    pub(crate) fn resource_transfer_own(
        self,
        store: &mut StoreOpaque,
        index: u32,
        src: TypeResourceTableIndex,
        dst: TypeResourceTableIndex,
    ) -> Result<u32> {
        let (calls, _, _, instance) = store.component_resource_state_with_instance(self);
        let mut tables = resource_tables(calls, instance);
        let rep = tables.resource_lift_own(TypedResourceIndex::Component { ty: src, index })?;
        tables.resource_lower_own(TypedResource::Component { ty: dst, rep })
    }

    pub(crate) fn resource_transfer_borrow(
        self,
        store: &mut StoreOpaque,
        index: u32,
        src: TypeResourceTableIndex,
        dst: TypeResourceTableIndex,
    ) -> Result<u32> {
        let dst_owns_resource = self.id().get(store).resource_owned_by_own_instance(dst);
        let (calls, _, _, instance) = store.component_resource_state_with_instance(self);
        let mut tables = resource_tables(calls, instance);
        let rep = tables.resource_lift_borrow(TypedResourceIndex::Component { ty: src, index })?;
        // Implement `lower_borrow`'s special case here where if a borrow's
        // resource type is owned by `dst` then the destination receives the
        // representation directly rather than a handle to the representation.
        //
        // This can perhaps become a different libcall in the future to avoid
        // this check at runtime since we know at compile time whether the
        // destination type owns the resource, but that's left as a future
        // refactoring if truly necessary.
        if dst_owns_resource {
            return Ok(rep);
        }
        tables.resource_lower_borrow(TypedResource::Component { ty: dst, rep })
    }

    pub(crate) fn resource_enter_call(self, store: &mut StoreOpaque) {
        let (calls, _, _, instance) = store.component_resource_state_with_instance(self);
        resource_tables(calls, instance).enter_call()
    }

    pub(crate) fn resource_exit_call(self, store: &mut StoreOpaque) -> Result<()> {
        let (calls, _, _, instance) = store.component_resource_state_with_instance(self);
        resource_tables(calls, instance).exit_call()
    }

    pub(crate) fn lookup_vmdef(&self, store: &mut StoreOpaque, def: &CoreDef) -> vm::Export {
        lookup_vmdef(store, self.id.instance(), def)
    }
}

/// Translates a `CoreDef`, a definition of a core wasm item, to an
/// [`Export`] which is the runtime core wasm definition.
pub(crate) fn lookup_vmdef(
    store: &mut StoreOpaque,
    id: ComponentInstanceId,
    def: &CoreDef,
) -> vm::Export {
    match def {
        CoreDef::Export(e) => lookup_vmexport(store, id, e),
        CoreDef::Trampoline(idx) => {
            let funcref = store
                .store_data_mut()
                .component_instance_mut(id)
                .trampoline_func_ref(*idx);
            // SAFETY: the `funcref` is owned by `store` and is valid within
            // that store, so it's safe to create a `Func`.
            vm::Export::Function(unsafe { crate::Func::from_vm_func_ref(store.id(), funcref) })
        }
        CoreDef::InstanceFlags(idx) => {
            let id = StoreComponentInstanceId::new(store.id(), id);
            vm::Export::Global(crate::Global::from_component_flags(id, *idx))
        }
    }
}

/// Translates a `CoreExport<T>`, an export of some core instance within
/// this component, to the actual runtime definition of that item.
pub(crate) fn lookup_vmexport<T>(
    store: &mut StoreOpaque,
    id: ComponentInstanceId,
    item: &CoreExport<T>,
) -> vm::Export
where
    T: Copy + Into<EntityIndex>,
{
    let store_id = store.id();
    let id = store
        .store_data_mut()
        .component_instance_mut(id)
        .instance(item.instance);
    let instance = store.instance_mut(id);
    let idx = match &item.item {
        ExportItem::Index(idx) => (*idx).into(),

        // FIXME: ideally at runtime we don't actually do any name lookups
        // here. This will only happen when the host supplies an imported
        // module so while the structure can't be known at compile time we
        // do know at `InstancePre` time, for example, what all the host
        // imports are. In theory we should be able to, as part of
        // `InstancePre` construction, perform all name=>index mappings
        // during that phase so the actual instantiation of an `InstancePre`
        // skips all string lookups. This should probably only be
        // investigated if this becomes a performance issue though.
        ExportItem::Name(name) => instance.env_module().exports[name],
    };
    // SAFETY: the `store_id` owns this instance and all exports contained
    // within.
    unsafe { instance.get_export_by_index_mut(store_id, idx) }
}

fn resource_tables<'a>(
    calls: &'a mut CallContexts,
    instance: Pin<&'a mut ComponentInstance>,
) -> ResourceTables<'a> {
    ResourceTables {
        host_table: None,
        calls,
        guest: Some(instance.guest_tables()),
    }
}

/// Trait used to lookup the export of a component instance.
///
/// This trait is used as an implementation detail of [`Instance::get_func`]
/// and related `get_*` methods. Notable implementors of this trait are:
///
/// * `str`
/// * `String`
/// * [`ComponentExportIndex`]
///
/// Note that this is intended to be a `wasmtime`-sealed trait so it shouldn't
/// need to be implemented externally.
pub trait InstanceExportLookup {
    #[doc(hidden)]
    fn lookup(&self, component: &Component) -> Option<ExportIndex>;
}

impl<T> InstanceExportLookup for &T
where
    T: InstanceExportLookup + ?Sized,
{
    fn lookup(&self, component: &Component) -> Option<ExportIndex> {
        T::lookup(self, component)
    }
}

impl InstanceExportLookup for str {
    fn lookup(&self, component: &Component) -> Option<ExportIndex> {
        component
            .env_component()
            .exports
            .get(self, &NameMapNoIntern)
            .copied()
    }
}

impl InstanceExportLookup for String {
    fn lookup(&self, component: &Component) -> Option<ExportIndex> {
        str::lookup(self, component)
    }
}

struct Instantiator<'a> {
    component: &'a Component,
    id: ComponentInstanceId,
    core_imports: OwnedImports,
    imports: &'a PrimaryMap<RuntimeImportIndex, RuntimeImport>,
}

pub(crate) enum RuntimeImport {
    Func(Arc<HostFunc>),
    Module(Module),
    Resource {
        ty: ResourceType,

        // A strong reference to the host function that represents the
        // destructor for this resource. At this time all resources here are
        // host-defined resources. Note that this is itself never read because
        // the funcref below points to it.
        //
        // Also note that the `Arc` here is used to support the same host
        // function being used across multiple instances simultaneously. Or
        // otherwise this makes `InstancePre::instantiate` possible to create
        // separate instances all sharing the same host function.
        _dtor: Arc<crate::func::HostFunc>,

        // A raw function which is filled out (including `wasm_call`) which
        // points to the internals of the `_dtor` field. This is read and
        // possibly executed by wasm.
        dtor_funcref: VMFuncRef,
    },
}

pub type ImportedResources = PrimaryMap<ResourceIndex, ResourceType>;

impl<'a> Instantiator<'a> {
    fn new(
        component: &'a Component,
        store: &mut StoreOpaque,
        imports: &'a Arc<PrimaryMap<RuntimeImportIndex, RuntimeImport>>,
    ) -> Instantiator<'a> {
        let env_component = component.env_component();
        store.modules_mut().register_component(component);
        let imported_resources: ImportedResources =
            PrimaryMap::with_capacity(env_component.imported_resources.len());

        let instance = ComponentInstance::new(
            store.store_data().components.next_component_instance_id(),
            component,
            Arc::new(imported_resources),
            imports,
            store.traitobj(),
        );
        let id = store.store_data_mut().push_component_instance(instance);

        Instantiator {
            component,
            imports,
            core_imports: OwnedImports::empty(),
            id,
        }
    }

    fn run<T>(&mut self, store: &mut StoreContextMut<'_, T>) -> Result<()> {
        let env_component = self.component.env_component();

        // Before all initializers are processed configure all destructors for
        // host-defined resources. No initializer will correspond to these and
        // it's required to happen before they're needed, so execute this first.
        for (idx, import) in env_component.imported_resources.iter() {
            let (ty, func_ref) = match &self.imports[*import] {
                RuntimeImport::Resource {
                    ty, dtor_funcref, ..
                } => (*ty, NonNull::from(dtor_funcref)),
                _ => unreachable!(),
            };
            let i = self.instance_resource_types_mut(store.0).push(ty);
            assert_eq!(i, idx);
            self.instance_mut(store.0)
                .set_resource_destructor(idx, Some(func_ref));
        }

        // Next configure all `VMFuncRef`s for trampolines that this component
        // will require. These functions won't actually get used until their
        // associated state has been initialized through the global initializers
        // below, but the funcrefs can all be configured here.
        for (idx, sig) in env_component.trampolines.iter() {
            let ptrs = self.component.trampoline_ptrs(idx);
            let signature = match self.component.signatures().shared_type(*sig) {
                Some(s) => s,
                None => panic!("found unregistered signature: {sig:?}"),
            };

            self.instance_mut(store.0).set_trampoline(
                idx,
                ptrs.wasm_call,
                ptrs.array_call,
                signature,
            );
        }

        for initializer in env_component.initializers.iter() {
            match initializer {
                GlobalInitializer::InstantiateModule(m) => {
                    let module;
                    let imports = match m {
                        // Since upvars are statically know we know that the
                        // `args` list is already in the right order.
                        InstantiateModule::Static(idx, args) => {
                            module = self.component.static_module(*idx);
                            self.build_imports(store.0, module, args.iter())
                        }

                        // With imports, unlike upvars, we need to do runtime
                        // lookups with strings to determine the order of the
                        // imports since it's whatever the actual module
                        // requires.
                        //
                        // FIXME: see the note in `ExportItem::Name` handling
                        // above for how we ideally shouldn't do string lookup
                        // here.
                        InstantiateModule::Import(idx, args) => {
                            module = match &self.imports[*idx] {
                                RuntimeImport::Module(m) => m,
                                _ => unreachable!(),
                            };
                            let args = module
                                .imports()
                                .map(|import| &args[import.module()][import.name()]);
                            self.build_imports(store.0, module, args)
                        }
                    };

                    // Note that the unsafety here should be ok because the
                    // validity of the component means that type-checks have
                    // already been performed. This means that the unsafety due
                    // to imports having the wrong type should not happen here.
                    //
                    // Also note we are calling new_started_impl because we have
                    // already checked for asyncness and are running on a fiber
                    // if required.

                    let i = unsafe {
                        crate::Instance::new_started_impl(store, module, imports.as_ref())?
                    };
                    self.instance_mut(store.0).push_instance_id(i.id());
                }

                GlobalInitializer::LowerImport { import, index } => {
                    let func = match &self.imports[*import] {
                        RuntimeImport::Func(func) => func,
                        _ => unreachable!(),
                    };
                    self.instance_mut(store.0)
                        .set_lowering(*index, func.lowering());
                }

                GlobalInitializer::ExtractTable(table) => self.extract_table(store.0, table),

                GlobalInitializer::ExtractMemory(mem) => self.extract_memory(store.0, mem),

                GlobalInitializer::ExtractRealloc(realloc) => {
                    self.extract_realloc(store.0, realloc)
                }

                GlobalInitializer::ExtractCallback(callback) => {
                    self.extract_callback(store.0, callback)
                }

                GlobalInitializer::ExtractPostReturn(post_return) => {
                    self.extract_post_return(store.0, post_return)
                }

                GlobalInitializer::Resource(r) => self.resource(store.0, r),
            }
        }
        Ok(())
    }

    fn resource(&mut self, store: &mut StoreOpaque, resource: &Resource) {
        let dtor = resource
            .dtor
            .as_ref()
            .map(|dtor| lookup_vmdef(store, self.id, dtor));
        let dtor = dtor.map(|export| match export {
            crate::runtime::vm::Export::Function(f) => f.vm_func_ref(store),
            _ => unreachable!(),
        });
        let index = self
            .component
            .env_component()
            .resource_index(resource.index);
        let instance = self.instance(store);
        let ty = ResourceType::guest(store.id(), instance, resource.index);
        self.instance_mut(store)
            .set_resource_destructor(index, dtor);
        let i = self.instance_resource_types_mut(store).push(ty);
        debug_assert_eq!(i, index);
    }

    fn extract_memory(&mut self, store: &mut StoreOpaque, memory: &ExtractMemory) {
        let mem = match lookup_vmexport(store, self.id, &memory.export) {
            crate::runtime::vm::Export::Memory { memory, .. } => memory,
            _ => unreachable!(),
        };
        let import = mem.vmimport(store);
        self.instance_mut(store)
            .set_runtime_memory(memory.index, import.from.as_non_null());
    }

    fn extract_realloc(&mut self, store: &mut StoreOpaque, realloc: &ExtractRealloc) {
        let func_ref = match lookup_vmdef(store, self.id, &realloc.def) {
            crate::runtime::vm::Export::Function(f) => f.vm_func_ref(store),
            _ => unreachable!(),
        };
        self.instance_mut(store)
            .set_runtime_realloc(realloc.index, func_ref);
    }

    fn extract_callback(&mut self, store: &mut StoreOpaque, callback: &ExtractCallback) {
        let func_ref = match lookup_vmdef(store, self.id, &callback.def) {
            crate::runtime::vm::Export::Function(f) => f.vm_func_ref(store),
            _ => unreachable!(),
        };
        self.instance_mut(store)
            .set_runtime_callback(callback.index, func_ref);
    }

    fn extract_post_return(&mut self, store: &mut StoreOpaque, post_return: &ExtractPostReturn) {
        let func_ref = match lookup_vmdef(store, self.id, &post_return.def) {
            crate::runtime::vm::Export::Function(f) => f.vm_func_ref(store),
            _ => unreachable!(),
        };
        self.instance_mut(store)
            .set_runtime_post_return(post_return.index, func_ref);
    }

    fn extract_table(&mut self, store: &mut StoreOpaque, table: &ExtractTable) {
        let export = match lookup_vmexport(store, self.id, &table.export) {
            crate::runtime::vm::Export::Table(t) => t,
            _ => unreachable!(),
        };
        let import = export.vmimport(store);
        self.instance_mut(store)
            .set_runtime_table(table.index, import);
    }

    fn build_imports<'b>(
        &mut self,
        store: &mut StoreOpaque,
        module: &Module,
        args: impl Iterator<Item = &'b CoreDef>,
    ) -> &OwnedImports {
        self.core_imports.clear();
        self.core_imports.reserve(module);
        let mut imports = module.compiled_module().module().imports();

        for arg in args {
            // The general idea of Wasmtime is that at runtime type-checks for
            // core wasm instantiations internally within a component are
            // unnecessary and superfluous. Naturally though mistakes may be
            // made, so double-check this property of wasmtime in debug mode.

            if cfg!(debug_assertions) {
                let (imp_module, imp_name, expected) = imports.next().unwrap();
                self.assert_type_matches(store, module, arg, imp_module, imp_name, expected);
            }

            // The unsafety here should be ok since the `export` is loaded
            // directly from an instance which should only give us valid export
            // items.
            let export = lookup_vmdef(store, self.id, arg);
            self.core_imports.push_export(store, &export);
        }
        debug_assert!(imports.next().is_none());

        &self.core_imports
    }

    fn assert_type_matches(
        &self,
        store: &mut StoreOpaque,
        module: &Module,
        arg: &CoreDef,
        imp_module: &str,
        imp_name: &str,
        expected: EntityType,
    ) {
        let export = lookup_vmdef(store, self.id, arg);

        // If this value is a core wasm function then the type check is inlined
        // here. This can otherwise fail `Extern::from_wasmtime_export` because
        // there's no guarantee that there exists a trampoline for `f` so this
        // can't fall through to the case below
        if let crate::runtime::vm::Export::Function(f) = &export {
            let expected = match expected.unwrap_func() {
                EngineOrModuleTypeIndex::Engine(e) => Some(e),
                EngineOrModuleTypeIndex::Module(m) => module.signatures().shared_type(m),
                EngineOrModuleTypeIndex::RecGroup(_) => unreachable!(),
            };
            let actual = unsafe { f.vm_func_ref(store).as_ref().type_index };
            assert_eq!(
                expected,
                Some(actual),
                "type mismatch for import {imp_module:?} {imp_name:?}!!!\n\n\
                 expected {:#?}\n\n\
                 found {:#?}",
                expected.and_then(|e| store.engine().signatures().borrow(e)),
                store.engine().signatures().borrow(actual)
            );
            return;
        }

        let val = unsafe { crate::Extern::from_wasmtime_export(export, store) };
        let ty = DefinitionType::from(store, &val);
        crate::types::matching::MatchCx::new(module.engine())
            .definition(&expected, &ty)
            .expect("unexpected typecheck failure");
    }

    /// Convenience helper to return the `&ComponentInstance` that's being
    /// instantiated.
    fn instance<'b>(&self, store: &'b StoreOpaque) -> &'b ComponentInstance {
        store.store_data().component_instance(self.id)
    }

    /// Same as [`Self::instance`], but for mutability.
    fn instance_mut<'b>(&self, store: &'b mut StoreOpaque) -> Pin<&'b mut ComponentInstance> {
        store.store_data_mut().component_instance_mut(self.id)
    }

    // NB: This method is only intended to be called during the instantiation
    // process because the `Arc::get_mut` here is fallible and won't generally
    // succeed once the instance has been handed to the embedder. Before that
    // though it should be guaranteed that the single owning reference currently
    // lives within the `ComponentInstance` that's being built.
    fn instance_resource_types_mut<'b>(
        &self,
        store: &'b mut StoreOpaque,
    ) -> &'b mut ImportedResources {
        Arc::get_mut(self.instance_mut(store).resource_types_mut()).unwrap()
    }
}

/// A "pre-instantiated" [`Instance`] which has all of its arguments already
/// supplied and is ready to instantiate.
///
/// This structure represents an efficient form of instantiation where import
/// type-checking and import lookup has all been resolved by the time that this
/// type is created. This type is primarily created through the
/// [`Linker::instantiate_pre`](crate::component::Linker::instantiate_pre)
/// method.
pub struct InstancePre<T: 'static> {
    component: Component,
    imports: Arc<PrimaryMap<RuntimeImportIndex, RuntimeImport>>,
    resource_types: Arc<PrimaryMap<ResourceIndex, ResourceType>>,
    _marker: marker::PhantomData<fn() -> T>,
}

// `InstancePre`'s clone does not require `T: Clone`
impl<T: 'static> Clone for InstancePre<T> {
    fn clone(&self) -> Self {
        Self {
            component: self.component.clone(),
            imports: self.imports.clone(),
            resource_types: self.resource_types.clone(),
            _marker: self._marker,
        }
    }
}

impl<T: 'static> InstancePre<T> {
    /// This function is `unsafe` since there's no guarantee that the
    /// `RuntimeImport` items provided are guaranteed to work with the `T` of
    /// the store.
    ///
    /// Additionally there is no static guarantee that the `imports` provided
    /// satisfy the imports of the `component` provided.
    pub(crate) unsafe fn new_unchecked(
        component: Component,
        imports: Arc<PrimaryMap<RuntimeImportIndex, RuntimeImport>>,
        resource_types: Arc<PrimaryMap<ResourceIndex, ResourceType>>,
    ) -> InstancePre<T> {
        InstancePre {
            component,
            imports,
            resource_types,
            _marker: marker::PhantomData,
        }
    }

    /// Returns the underlying component that will be instantiated.
    pub fn component(&self) -> &Component {
        &self.component
    }

    #[doc(hidden)]
    /// Returns the type at which the underlying component will be
    /// instantiated. This contains the instantiated type information which
    /// was determined by the Linker.
    pub fn instance_type(&self) -> InstanceType<'_> {
        InstanceType {
            types: &self.component.types(),
            resources: &self.resource_types,
        }
    }

    /// Returns the underlying engine.
    pub fn engine(&self) -> &Engine {
        self.component.engine()
    }

    /// Performs the instantiation process into the store specified.
    //
    // TODO: needs more docs
    pub fn instantiate(&self, store: impl AsContextMut<Data = T>) -> Result<Instance> {
        assert!(
            !store.as_context().async_support(),
            "must use async instantiation when async support is enabled"
        );
        self.instantiate_impl(store)
    }
    /// Performs the instantiation process into the store specified.
    ///
    /// Exactly like [`Self::instantiate`] except for use on async stores.
    //
    // TODO: needs more docs
    #[cfg(feature = "async")]
    pub async fn instantiate_async(
        &self,
        mut store: impl AsContextMut<Data = T>,
    ) -> Result<Instance>
    where
        T: Send,
    {
        let mut store = store.as_context_mut();
        assert!(
            store.0.async_support(),
            "must use sync instantiation when async support is disabled"
        );
        store.on_fiber(|store| self.instantiate_impl(store)).await?
    }

    fn instantiate_impl(&self, mut store: impl AsContextMut<Data = T>) -> Result<Instance> {
        let mut store = store.as_context_mut();
        store
            .engine()
            .allocator()
            .increment_component_instance_count()?;
        let mut instantiator = Instantiator::new(&self.component, store.0, &self.imports);
        instantiator.run(&mut store).map_err(|e| {
            store
                .engine()
                .allocator()
                .decrement_component_instance_count();
            e
        })?;
        let instance = Instance::from_wasmtime(store.0, instantiator.id);
        store.0.push_component_instance(instance);
        Ok(instance)
    }
}
