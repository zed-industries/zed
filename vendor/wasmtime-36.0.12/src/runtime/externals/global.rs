use crate::prelude::*;
use crate::runtime::vm::{self, VMGlobalDefinition, VMGlobalKind, VMOpaqueContext};
use crate::{
    AnyRef, AsContext, AsContextMut, ExternRef, Func, GlobalType, HeapType, Mutability, Ref,
    RootedGcRefImpl, Val, ValType,
    store::{AutoAssertNoGc, InstanceId, StoreId, StoreInstanceId, StoreOpaque},
    trampoline::generate_global_export,
};
use core::ptr;
use core::ptr::NonNull;
use wasmtime_environ::DefinedGlobalIndex;

/// A WebAssembly `global` value which can be read and written to.
///
/// A `global` in WebAssembly is sort of like a global variable within an
/// [`Instance`](crate::Instance). The `global.get` and `global.set`
/// instructions will modify and read global values in a wasm module. Globals
/// can either be imported or exported from wasm modules.
///
/// A [`Global`] "belongs" to the store that it was originally created within
/// (either via [`Global::new`] or via instantiating a
/// [`Module`](crate::Module)). Operations on a [`Global`] only work with the
/// store it belongs to, and if another store is passed in by accident then
/// methods will panic.
#[derive(Copy, Clone, Debug)]
#[repr(C)] // here for the C API
pub struct Global {
    /// The store that this global belongs to.
    store: StoreId,
    /// Either `InstanceId` or `ComponentInstanceId` internals depending on
    /// `kind` below.
    instance: u32,
    /// Which method of definition was used when creating this global.
    kind: VMGlobalKind,
}

// Double-check that the C representation in `extern.h` matches our in-Rust
// representation here in terms of size/alignment/etc.
const _: () = {
    #[repr(C)]
    struct C(u64, u32, u32, u32);
    assert!(core::mem::size_of::<C>() == core::mem::size_of::<Global>());
    assert!(core::mem::align_of::<C>() == core::mem::align_of::<Global>());
    assert!(core::mem::offset_of!(Global, store) == 0);
};

impl Global {
    /// Creates a new WebAssembly `global` value with the provide type `ty` and
    /// initial value `val`.
    ///
    /// The `store` argument will be the owner of the [`Global`] returned. Using
    /// the returned [`Global`] other items in the store may access this global.
    /// For example this could be provided as an argument to
    /// [`Instance::new`](crate::Instance::new) or
    /// [`Linker::define`](crate::Linker::define).
    ///
    /// # Errors
    ///
    /// Returns an error if the `ty` provided does not match the type of the
    /// value `val`, or if `val` comes from a different store than `store`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = Engine::default();
    /// let mut store = Store::new(&engine, ());
    ///
    /// let ty = GlobalType::new(ValType::I32, Mutability::Const);
    /// let i32_const = Global::new(&mut store, ty, 1i32.into())?;
    /// let ty = GlobalType::new(ValType::F64, Mutability::Var);
    /// let f64_mut = Global::new(&mut store, ty, 2.0f64.into())?;
    ///
    /// let module = Module::new(
    ///     &engine,
    ///     "(module
    ///         (global (import \"\" \"i32-const\") i32)
    ///         (global (import \"\" \"f64-mut\") (mut f64))
    ///     )"
    /// )?;
    ///
    /// let mut linker = Linker::new(&engine);
    /// linker.define(&store, "", "i32-const", i32_const)?;
    /// linker.define(&store, "", "f64-mut", f64_mut)?;
    ///
    /// let instance = linker.instantiate(&mut store, &module)?;
    /// // ...
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(mut store: impl AsContextMut, ty: GlobalType, val: Val) -> Result<Global> {
        Global::_new(store.as_context_mut().0, ty, val)
    }

    fn _new(store: &mut StoreOpaque, ty: GlobalType, val: Val) -> Result<Global> {
        val.ensure_matches_ty(store, ty.content()).context(
            "type mismatch: initial value provided does not match the type of this global",
        )?;
        generate_global_export(store, ty, val)
    }

    pub(crate) fn new_host(store: &StoreOpaque, index: DefinedGlobalIndex) -> Global {
        Global {
            store: store.id(),
            instance: 0,
            kind: VMGlobalKind::Host(index),
        }
    }

    pub(crate) fn new_instance(
        store: &StoreOpaque,
        instance: InstanceId,
        index: DefinedGlobalIndex,
    ) -> Global {
        Global {
            store: store.id(),
            instance: instance.as_u32(),
            kind: VMGlobalKind::Instance(index),
        }
    }

    /// Returns the underlying type of this `global`.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this global.
    pub fn ty(&self, store: impl AsContext) -> GlobalType {
        self._ty(store.as_context().0)
    }

    pub(crate) fn _ty(&self, store: &StoreOpaque) -> GlobalType {
        GlobalType::from_wasmtime_global(store.engine(), self.wasmtime_ty(store))
    }

    /// Returns the current [`Val`] of this global.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this global.
    pub fn get(&self, mut store: impl AsContextMut) -> Val {
        unsafe {
            let store = store.as_context_mut();
            let definition = self.definition(store.0).as_ref();
            let mut store = AutoAssertNoGc::new(store.0);
            match self._ty(&store).content() {
                ValType::I32 => Val::from(*definition.as_i32()),
                ValType::I64 => Val::from(*definition.as_i64()),
                ValType::F32 => Val::F32(*definition.as_u32()),
                ValType::F64 => Val::F64(*definition.as_u64()),
                ValType::V128 => Val::V128(definition.get_u128().into()),
                ValType::Ref(ref_ty) => {
                    let reference: Ref = match ref_ty.heap_type() {
                        HeapType::Func | HeapType::ConcreteFunc(_) => {
                            Func::_from_raw(&mut store, definition.as_func_ref().cast()).into()
                        }

                        HeapType::NoFunc => Ref::Func(None),

                        HeapType::Extern => Ref::Extern(definition.as_gc_ref().map(|r| {
                            let r = store.unwrap_gc_store_mut().clone_gc_ref(r);
                            ExternRef::from_cloned_gc_ref(&mut store, r)
                        })),

                        HeapType::NoCont | HeapType::ConcreteCont(_) | HeapType::Cont => {
                            // TODO(#10248) Required to support stack switching in the embedder API.
                            unimplemented!()
                        }

                        HeapType::NoExtern => Ref::Extern(None),

                        HeapType::Any
                        | HeapType::Eq
                        | HeapType::I31
                        | HeapType::Struct
                        | HeapType::ConcreteStruct(_)
                        | HeapType::Array
                        | HeapType::ConcreteArray(_)
                        | HeapType::Exn
                        | HeapType::ConcreteExn(_) => definition
                            .as_gc_ref()
                            .map(|r| {
                                let r = store.unwrap_gc_store_mut().clone_gc_ref(r);
                                AnyRef::from_cloned_gc_ref(&mut store, r)
                            })
                            .into(),

                        HeapType::NoExn => Ref::Exn(None),

                        HeapType::None => Ref::Any(None),
                    };
                    debug_assert!(
                        ref_ty.is_nullable() || !reference.is_null(),
                        "if the type is non-nullable, we better have a non-null reference"
                    );
                    reference.into()
                }
            }
        }
    }

    /// Attempts to set the current value of this global to [`Val`].
    ///
    /// # Errors
    ///
    /// Returns an error if this global has a different type than `Val`, if
    /// it's not a mutable global, or if `val` comes from a different store than
    /// the one provided.
    ///
    /// # Panics
    ///
    /// Panics if `store` does not own this global.
    pub fn set(&self, mut store: impl AsContextMut, val: Val) -> Result<()> {
        let mut store = AutoAssertNoGc::new(store.as_context_mut().0);
        let global_ty = self._ty(&store);
        if global_ty.mutability() != Mutability::Var {
            bail!("immutable global cannot be set");
        }
        val.ensure_matches_ty(&store, global_ty.content())
            .context("type mismatch: attempt to set global to value of wrong type")?;
        unsafe {
            let definition = self.definition(&store).as_mut();
            match val {
                Val::I32(i) => *definition.as_i32_mut() = i,
                Val::I64(i) => *definition.as_i64_mut() = i,
                Val::F32(f) => *definition.as_u32_mut() = f,
                Val::F64(f) => *definition.as_u64_mut() = f,
                Val::V128(i) => definition.set_u128(i.into()),
                Val::FuncRef(f) => {
                    *definition.as_func_ref_mut() =
                        f.map_or(ptr::null_mut(), |f| f.vm_func_ref(&store).as_ptr().cast());
                }
                Val::ExternRef(e) => {
                    let new = match e {
                        None => None,
                        Some(e) => Some(e.try_gc_ref(&store)?.unchecked_copy()),
                    };
                    let new = new.as_ref();
                    definition.write_gc_ref(store.unwrap_gc_store_mut(), new);
                }
                Val::AnyRef(a) => {
                    let new = match a {
                        None => None,
                        Some(a) => Some(a.try_gc_ref(&store)?.unchecked_copy()),
                    };
                    let new = new.as_ref();
                    definition.write_gc_ref(store.unwrap_gc_store_mut(), new);
                }
                Val::ExnRef(e) => {
                    let new = match e {
                        None => None,
                        Some(e) => Some(e.try_gc_ref(&store)?.unchecked_copy()),
                    };
                    let new = new.as_ref();
                    definition.write_gc_ref(store.unwrap_gc_store_mut(), new);
                }
            }
        }
        Ok(())
    }

    #[cfg(feature = "gc")]
    pub(crate) fn trace_root(&self, store: &mut StoreOpaque, gc_roots_list: &mut vm::GcRootsList) {
        if let Some(ref_ty) = self._ty(store).content().as_ref() {
            if !ref_ty.is_vmgcref_type_and_points_to_object() {
                return;
            }

            if let Some(gc_ref) = unsafe { self.definition(store).as_ref().as_gc_ref() } {
                unsafe {
                    gc_roots_list.add_root(gc_ref.into(), "Wasm global");
                }
            }
        }
    }

    pub(crate) fn from_host(store: StoreId, index: DefinedGlobalIndex) -> Global {
        Global {
            store,
            instance: 0,
            kind: VMGlobalKind::Host(index),
        }
    }

    pub(crate) fn from_core(instance: StoreInstanceId, index: DefinedGlobalIndex) -> Global {
        Global {
            store: instance.store_id(),
            instance: instance.instance().as_u32(),
            kind: VMGlobalKind::Instance(index),
        }
    }

    #[cfg(feature = "component-model")]
    pub(crate) fn from_component_flags(
        instance: crate::component::store::StoreComponentInstanceId,
        index: wasmtime_environ::component::RuntimeComponentInstanceIndex,
    ) -> Global {
        Global {
            store: instance.store_id(),
            instance: instance.instance().as_u32(),
            kind: VMGlobalKind::ComponentFlags(index),
        }
    }

    pub(crate) fn wasmtime_ty<'a>(&self, store: &'a StoreOpaque) -> &'a wasmtime_environ::Global {
        self.store.assert_belongs_to(store.id());
        match self.kind {
            VMGlobalKind::Instance(index) => {
                let instance = InstanceId::from_u32(self.instance);
                let module = store.instance(instance).env_module();
                let index = module.global_index(index);
                &module.globals[index]
            }
            VMGlobalKind::Host(index) => unsafe { &store.host_globals()[index].get().as_ref().ty },
            #[cfg(feature = "component-model")]
            VMGlobalKind::ComponentFlags(_) => {
                const TY: wasmtime_environ::Global = wasmtime_environ::Global {
                    mutability: true,
                    wasm_ty: wasmtime_environ::WasmValType::I32,
                };
                &TY
            }
        }
    }

    pub(crate) fn vmimport(&self, store: &StoreOpaque) -> vm::VMGlobalImport {
        let vmctx = match self.kind {
            VMGlobalKind::Instance(_) => {
                let instance = InstanceId::from_u32(self.instance);
                Some(VMOpaqueContext::from_vmcontext(store.instance(instance).vmctx()).into())
            }
            VMGlobalKind::Host(_) => None,
            #[cfg(feature = "component-model")]
            VMGlobalKind::ComponentFlags(_) => {
                let instance = crate::component::ComponentInstanceId::from_u32(self.instance);
                Some(
                    VMOpaqueContext::from_vmcomponent(store.component_instance(instance).vmctx())
                        .into(),
                )
            }
        };
        vm::VMGlobalImport {
            from: self.definition(store).into(),
            vmctx,
            kind: self.kind,
        }
    }

    pub(crate) fn comes_from_same_store(&self, store: &StoreOpaque) -> bool {
        store.id() == self.store
    }

    /// Get a stable hash key for this global.
    ///
    /// Even if the same underlying global definition is added to the
    /// `StoreData` multiple times and becomes multiple `wasmtime::Global`s,
    /// this hash key will be consistent across all of these globals.
    #[cfg(feature = "coredump")]
    pub(crate) fn hash_key(&self, store: &StoreOpaque) -> impl core::hash::Hash + Eq + use<> {
        self.definition(store).as_ptr().addr()
    }

    fn definition(&self, store: &StoreOpaque) -> NonNull<VMGlobalDefinition> {
        self.store.assert_belongs_to(store.id());
        match self.kind {
            VMGlobalKind::Instance(index) => {
                let instance = InstanceId::from_u32(self.instance);
                store.instance(instance).global_ptr(index)
            }
            VMGlobalKind::Host(index) => unsafe {
                NonNull::from(&mut store.host_globals()[index].get().as_mut().global)
            },
            #[cfg(feature = "component-model")]
            VMGlobalKind::ComponentFlags(index) => {
                let instance = crate::component::ComponentInstanceId::from_u32(self.instance);
                store
                    .component_instance(instance)
                    .instance_flags(index)
                    .as_raw()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Instance, Module, Store};

    #[test]
    fn hash_key_is_stable_across_duplicate_store_data_entries() -> Result<()> {
        let mut store = Store::<()>::default();
        let module = Module::new(
            store.engine(),
            r#"
                (module
                    (global (export "g") (mut i32) (i32.const 0))
                )
            "#,
        )?;
        let instance = Instance::new(&mut store, &module, &[])?;

        // Each time we `get_global`, we call `Global::from_wasmtime` which adds
        // a new entry to `StoreData`, so `g1` and `g2` will have different
        // indices into `StoreData`.
        let g1 = instance.get_global(&mut store, "g").unwrap();
        let g2 = instance.get_global(&mut store, "g").unwrap();

        // That said, they really point to the same global.
        assert_eq!(g1.get(&mut store).unwrap_i32(), 0);
        assert_eq!(g2.get(&mut store).unwrap_i32(), 0);
        g1.set(&mut store, Val::I32(42))?;
        assert_eq!(g1.get(&mut store).unwrap_i32(), 42);
        assert_eq!(g2.get(&mut store).unwrap_i32(), 42);

        // And therefore their hash keys are the same.
        assert!(g1.hash_key(&store.as_context().0) == g2.hash_key(&store.as_context().0));

        // But the hash keys are different from different globals.
        let instance2 = Instance::new(&mut store, &module, &[])?;
        let g3 = instance2.get_global(&mut store, "g").unwrap();
        assert!(g1.hash_key(&store.as_context().0) != g3.hash_key(&store.as_context().0));

        Ok(())
    }
}
