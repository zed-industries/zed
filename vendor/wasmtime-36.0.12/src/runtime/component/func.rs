use crate::component::instance::Instance;
use crate::component::matching::InstanceType;
use crate::component::storage::storage_as_slice;
use crate::component::types::Type;
use crate::component::values::Val;
use crate::prelude::*;
use crate::runtime::vm::component::{ComponentInstance, InstanceFlags, ResourceTables};
use crate::runtime::vm::{Export, VMFuncRef};
use crate::store::StoreOpaque;
use crate::{AsContext, AsContextMut, StoreContextMut, ValRaw};
use core::mem::{self, MaybeUninit};
use core::ptr::NonNull;
use wasmtime_environ::component::{
    CanonicalOptions, ExportIndex, InterfaceType, MAX_FLAT_PARAMS, MAX_FLAT_RESULTS, TypeFuncIndex,
    TypeTuple,
};

#[cfg(feature = "component-model-async")]
use crate::component::concurrent::{self, AsAccessor, PreparedCall};

mod host;
mod options;
mod typed;
pub use self::host::*;
pub use self::options::*;
pub use self::typed::*;

/// A WebAssembly component function which can be called.
///
/// This type is the dual of [`wasmtime::Func`](crate::Func) for component
/// functions. An instance of [`Func`] represents a component function from a
/// component [`Instance`](crate::component::Instance). Like with
/// [`wasmtime::Func`](crate::Func) it's possible to call functions either
/// synchronously or asynchronously and either typed or untyped.
#[derive(Copy, Clone, Debug)]
#[repr(C)] // here for the C API.
pub struct Func {
    instance: Instance,
    index: ExportIndex,
}

// Double-check that the C representation in `component/instance.h` matches our
// in-Rust representation here in terms of size/alignment/etc.
const _: () = {
    #[repr(C)]
    struct T(u64, u32);
    #[repr(C)]
    struct C(T, u32);
    assert!(core::mem::size_of::<C>() == core::mem::size_of::<Func>());
    assert!(core::mem::align_of::<C>() == core::mem::align_of::<Func>());
    assert!(core::mem::offset_of!(Func, instance) == 0);
};

impl Func {
    pub(crate) fn from_lifted_func(instance: Instance, index: ExportIndex) -> Func {
        Func { instance, index }
    }

    /// Attempt to cast this [`Func`] to a statically typed [`TypedFunc`] with
    /// the provided `Params` and `Return`.
    ///
    /// This function will perform a type-check at runtime that the [`Func`]
    /// takes `Params` as parameters and returns `Return`. If the type-check
    /// passes then a [`TypedFunc`] will be returned which can be used to
    /// invoke the function in an efficient, statically-typed, and ergonomic
    /// manner.
    ///
    /// The `Params` type parameter here is a tuple of the parameters to the
    /// function. A function which takes no arguments should use `()`, a
    /// function with one argument should use `(T,)`, etc. Note that all
    /// `Params` must also implement the [`Lower`] trait since they're going
    /// into wasm.
    ///
    /// The `Return` type parameter is the return value of this function. A
    /// return value of `()` means that there's no return (similar to a Rust
    /// unit return) and otherwise a type `T` can be specified. Note that the
    /// `Return` must also implement the [`Lift`] trait since it's coming from
    /// wasm.
    ///
    /// Types specified here must implement the [`ComponentType`] trait. This
    /// trait is implemented for built-in types to Rust such as integer
    /// primitives, floats, `Option<T>`, `Result<T, E>`, strings, `Vec<T>`, and
    /// more. As parameters you'll be passing native Rust types.
    ///
    /// See the documentation for [`ComponentType`] for more information about
    /// supported types.
    ///
    /// # Errors
    ///
    /// If the function does not actually take `Params` as its parameters or
    /// return `Return` then an error will be returned.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is not owned by the `store`
    /// specified.
    ///
    /// # Examples
    ///
    /// Calling a function which takes no parameters and has no return value:
    ///
    /// ```
    /// # use wasmtime::component::Func;
    /// # use wasmtime::Store;
    /// # fn foo(func: &Func, store: &mut Store<()>) -> anyhow::Result<()> {
    /// let typed = func.typed::<(), ()>(&store)?;
    /// typed.call(store, ())?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Calling a function which takes one string parameter and returns a
    /// string:
    ///
    /// ```
    /// # use wasmtime::component::Func;
    /// # use wasmtime::Store;
    /// # fn foo(func: &Func, mut store: Store<()>) -> anyhow::Result<()> {
    /// let typed = func.typed::<(&str,), (String,)>(&store)?;
    /// let ret = typed.call(&mut store, ("Hello, ",))?.0;
    /// println!("returned string was: {}", ret);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Calling a function which takes multiple parameters and returns a boolean:
    ///
    /// ```
    /// # use wasmtime::component::Func;
    /// # use wasmtime::Store;
    /// # fn foo(func: &Func, mut store: Store<()>) -> anyhow::Result<()> {
    /// let typed = func.typed::<(u32, Option<&str>, &[u8]), (bool,)>(&store)?;
    /// let ok: bool = typed.call(&mut store, (1, Some("hello"), b"bytes!"))?.0;
    /// println!("return value was: {ok}");
    /// # Ok(())
    /// # }
    /// ```
    pub fn typed<Params, Return>(&self, store: impl AsContext) -> Result<TypedFunc<Params, Return>>
    where
        Params: ComponentNamedList + Lower,
        Return: ComponentNamedList + Lift,
    {
        self._typed(store.as_context().0, None)
    }

    pub(crate) fn _typed<Params, Return>(
        &self,
        store: &StoreOpaque,
        instance: Option<&ComponentInstance>,
    ) -> Result<TypedFunc<Params, Return>>
    where
        Params: ComponentNamedList + Lower,
        Return: ComponentNamedList + Lift,
    {
        self.typecheck::<Params, Return>(store, instance)?;
        unsafe { Ok(TypedFunc::new_unchecked(*self)) }
    }

    fn typecheck<Params, Return>(
        &self,
        store: &StoreOpaque,
        instance: Option<&ComponentInstance>,
    ) -> Result<()>
    where
        Params: ComponentNamedList + Lower,
        Return: ComponentNamedList + Lift,
    {
        let cx = InstanceType::new(instance.unwrap_or_else(|| self.instance.id().get(store)));
        let ty = &cx.types[self.ty(store)];

        Params::typecheck(&InterfaceType::Tuple(ty.params), &cx)
            .context("type mismatch with parameters")?;
        Return::typecheck(&InterfaceType::Tuple(ty.results), &cx)
            .context("type mismatch with results")?;

        Ok(())
    }

    /// Get the parameter names and types for this function.
    pub fn params(&self, store: impl AsContext) -> Box<[(String, Type)]> {
        let store = store.as_context();
        let instance = self.instance.id().get(store.0);
        let types = instance.component().types();
        let func_ty = &types[self.ty(store.0)];
        types[func_ty.params]
            .types
            .iter()
            .zip(&func_ty.param_names)
            .map(|(ty, name)| (name.clone(), Type::from(ty, &InstanceType::new(instance))))
            .collect()
    }

    /// Get the result types for this function.
    pub fn results(&self, store: impl AsContext) -> Box<[Type]> {
        let store = store.as_context();
        let instance = self.instance.id().get(store.0);
        let types = instance.component().types();
        let ty = self.ty(store.0);
        types[types[ty].results]
            .types
            .iter()
            .map(|ty| Type::from(ty, &InstanceType::new(instance)))
            .collect()
    }

    fn ty(&self, store: &StoreOpaque) -> TypeFuncIndex {
        let instance = self.instance.id().get(store);
        let (ty, _, _) = instance.component().export_lifted_function(self.index);
        ty
    }

    /// Invokes this function with the `params` given and returns the result.
    ///
    /// The `params` provided must match the parameters that this function takes
    /// in terms of their types and the number of parameters. Results will be
    /// written to the `results` slice provided if the call completes
    /// successfully. The initial types of the values in `results` are ignored
    /// and values are overwritten to write the result. It's required that the
    /// size of `results` exactly matches the number of results that this
    /// function produces.
    ///
    /// Note that after a function is invoked the embedder needs to invoke
    /// [`Func::post_return`] to execute any final cleanup required by the
    /// guest. This function call is required to either call the function again
    /// or to call another function.
    ///
    /// For more detailed information see the documentation of
    /// [`TypedFunc::call`].
    ///
    /// # Errors
    ///
    /// Returns an error in situations including but not limited to:
    ///
    /// * `params` is not the right size or if the values have the wrong type
    /// * `results` is not the right size
    /// * A trap occurs while executing the function
    /// * The function calls a host function which returns an error
    ///
    /// See [`TypedFunc::call`] for more information in addition to
    /// [`wasmtime::Func::call`](crate::Func::call).
    ///
    /// # Panics
    ///
    /// Panics if this is called on a function in an asynchronous store. This
    /// only works with functions defined within a synchronous store. Also
    /// panics if `store` does not own this function.
    pub fn call(
        &self,
        mut store: impl AsContextMut,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<()> {
        let mut store = store.as_context_mut();
        assert!(
            !store.0.async_support(),
            "must use `call_async` when async support is enabled on the config"
        );
        self.call_impl(&mut store.as_context_mut(), params, results)
    }

    /// Exactly like [`Self::call`] except for use on async stores.
    ///
    /// Note that after this [`Func::post_return_async`] will be used instead of
    /// the synchronous version at [`Func::post_return`].
    ///
    /// # Panics
    ///
    /// Panics if this is called on a function in a synchronous store. This
    /// only works with functions defined within an asynchronous store. Also
    /// panics if `store` does not own this function.
    #[cfg(feature = "async")]
    pub async fn call_async(
        &self,
        mut store: impl AsContextMut<Data: Send>,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<()> {
        let mut store = store.as_context_mut();

        #[cfg(feature = "component-model-async")]
        {
            self.instance
                .run_concurrent(&mut store, async |store| {
                    self.call_concurrent_dynamic(store, params, results, false)
                        .await
                })
                .await?
        }
        #[cfg(not(feature = "component-model-async"))]
        {
            assert!(
                store.0.async_support(),
                "cannot use `call_async` without enabling async support in the config"
            );
            store
                .on_fiber(|store| self.call_impl(store, params, results))
                .await?
        }
    }

    fn check_params_results<T>(
        &self,
        store: StoreContextMut<T>,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<()> {
        let param_tys = self.params(&store);
        if param_tys.len() != params.len() {
            bail!(
                "expected {} argument(s), got {}",
                param_tys.len(),
                params.len(),
            );
        }

        let result_tys = self.results(&store);

        if result_tys.len() != results.len() {
            bail!(
                "expected {} result(s), got {}",
                result_tys.len(),
                results.len(),
            );
        }

        Ok(())
    }

    /// Start a concurrent call to this function.
    ///
    /// Unlike [`Self::call`] and [`Self::call_async`] (both of which require
    /// exclusive access to the store until the completion of the call), calls
    /// made using this method may run concurrently with other calls to the same
    /// instance.  In addition, the runtime will call the `post-return` function
    /// (if any) automatically when the guest task completes -- no need to
    /// explicitly call `Func::post_return` afterward.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this function.
    #[cfg(feature = "component-model-async")]
    pub async fn call_concurrent(
        self,
        accessor: impl AsAccessor<Data: Send>,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<()> {
        self.call_concurrent_dynamic(accessor.as_accessor(), params, results, true)
            .await
    }

    /// Internal helper function for `call_async` and `call_concurrent`.
    #[cfg(feature = "component-model-async")]
    async fn call_concurrent_dynamic(
        self,
        store: impl AsAccessor<Data: Send>,
        params: &[Val],
        results: &mut [Val],
        call_post_return_automatically: bool,
    ) -> Result<()> {
        let store = store.as_accessor();
        let result = store.with(|mut store| {
            assert!(
                store.as_context_mut().0.async_support(),
                "cannot use `call_concurrent` when async support is not enabled on the config"
            );
            self.check_params_results(store.as_context_mut(), params, results)?;
            let prepared = self.prepare_call_dynamic(
                store.as_context_mut(),
                params.to_vec(),
                call_post_return_automatically,
            )?;
            concurrent::queue_call(store.as_context_mut(), prepared)
        })?;

        let run_results = result.await?;
        assert_eq!(run_results.len(), results.len());
        for (result, slot) in run_results.into_iter().zip(results) {
            *slot = result;
        }
        Ok(())
    }

    /// Calls `concurrent::prepare_call` with monomorphized functions for
    /// lowering the parameters and lifting the result.
    #[cfg(feature = "component-model-async")]
    fn prepare_call_dynamic<'a, T: Send + 'static>(
        self,
        mut store: StoreContextMut<'a, T>,
        params: Vec<Val>,
        call_post_return_automatically: bool,
    ) -> Result<PreparedCall<Vec<Val>>> {
        let store = store.as_context_mut();

        concurrent::prepare_call(
            store,
            self,
            MAX_FLAT_PARAMS,
            true,
            call_post_return_automatically,
            move |func, store, params_out| {
                func.with_lower_context(store, call_post_return_automatically, |cx, ty| {
                    Self::lower_args(cx, &params, ty, params_out)
                })
            },
            move |func, store, results| {
                let max_flat = if func.abi_async(store) {
                    MAX_FLAT_PARAMS
                } else {
                    MAX_FLAT_RESULTS
                };
                let results = func.with_lift_context(store, |cx, ty| {
                    Self::lift_results(cx, ty, results, max_flat)?.collect::<Result<Vec<_>>>()
                })?;
                Ok(Box::new(results))
            },
        )
    }

    fn call_impl(
        &self,
        mut store: impl AsContextMut,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<()> {
        let mut store = store.as_context_mut();

        self.check_params_results(store.as_context_mut(), params, results)?;

        if self.abi_async(store.0) {
            unreachable!(
                "async-lifted exports should have failed validation \
                 when `component-model-async` feature disabled"
            );
        }

        // SAFETY: the chosen representations of type parameters to `call_raw`
        // here should be generally safe to work with:
        //
        // * parameters use `MaybeUninit<[MaybeUninit<ValRaw>; MAX_FLAT_PARAMS]>`
        //   which represents the maximal possible number of parameters that can
        //   be passed to lifted component functions. This is modeled with
        //   `MaybeUninit` to represent how it all starts as uninitialized and
        //   thus can't be safely read during lowering.
        //
        // * results are modeled as `[ValRaw; MAX_FLAT_RESULTS]` which
        //   represents the maximal size of values that can be returned. Note
        //   that if the function doesn't actually have a return value then the
        //   `ValRaw` inside the array will have undefined contents. That is
        //   safe in Rust, however, due to `ValRaw` being a `union`. The
        //   contents should dynamically not be read due to the type of the
        //   function used here matching the actual lift.
        unsafe {
            self.call_raw(
                store,
                |cx, ty, dst: &mut MaybeUninit<[MaybeUninit<ValRaw>; MAX_FLAT_PARAMS]>| {
                    // SAFETY: it's safe to assume that
                    // `MaybeUninit<array-of-maybe-uninit>` is initialized because
                    // each individual element is still considered uninitialized.
                    let dst: &mut [MaybeUninit<ValRaw>] = dst.assume_init_mut();
                    Self::lower_args(cx, params, ty, dst)
                },
                |cx, results_ty, src: &[ValRaw; MAX_FLAT_RESULTS]| {
                    let max_flat = MAX_FLAT_RESULTS;
                    for (result, slot) in
                        Self::lift_results(cx, results_ty, src, max_flat)?.zip(results)
                    {
                        *slot = result?;
                    }
                    Ok(())
                },
            )
        }
    }

    pub(crate) fn lifted_core_func(&self, store: &mut StoreOpaque) -> NonNull<VMFuncRef> {
        let def = {
            let instance = self.instance.id().get(store);
            let (_ty, def, _options) = instance.component().export_lifted_function(self.index);
            def.clone()
        };
        match self.instance.lookup_vmdef(store, &def) {
            Export::Function(f) => f.vm_func_ref(store),
            _ => unreachable!(),
        }
    }

    pub(crate) fn post_return_core_func(&self, store: &StoreOpaque) -> Option<NonNull<VMFuncRef>> {
        let instance = self.instance.id().get(store);
        let component = instance.component();
        let (_ty, _def, options) = component.export_lifted_function(self.index);
        let post_return = component.env_component().options[options].post_return;
        post_return.map(|i| instance.runtime_post_return(i))
    }

    pub(crate) fn abi_async(&self, store: &StoreOpaque) -> bool {
        let instance = self.instance.id().get(store);
        let component = instance.component();
        let (_ty, _def, options) = component.export_lifted_function(self.index);
        component.env_component().options[options].async_
    }

    pub(crate) fn abi_info<'a>(
        &self,
        store: &'a StoreOpaque,
    ) -> (Options, InstanceFlags, TypeFuncIndex, &'a CanonicalOptions) {
        let vminstance = self.instance.id().get(store);
        let component = vminstance.component();
        let (ty, _def, options_index) = component.export_lifted_function(self.index);
        let raw_options = &component.env_component().options[options_index];
        let options = Options::new_index(store, self.instance, options_index);
        (
            options,
            vminstance.instance_flags(raw_options.instance),
            ty,
            raw_options,
        )
    }

    /// Invokes the underlying wasm function, lowering arguments and lifting the
    /// result.
    ///
    /// The `lower` function and `lift` function provided here are what actually
    /// do the lowering and lifting. The `LowerParams` and `LowerReturn` types
    /// are what will be allocated on the stack for this function call. They
    /// should be appropriately sized for the lowering/lifting operation
    /// happening.
    ///
    /// # Safety
    ///
    /// The safety of this function relies on the correct definitions of the
    /// `LowerParams` and `LowerReturn` type. They must match the type of `self`
    /// for the params/results that are going to be produced. Additionally
    /// these types must be representable with a sequence of `ValRaw` values.
    unsafe fn call_raw<T, Return, LowerParams, LowerReturn>(
        &self,
        mut store: StoreContextMut<'_, T>,
        lower: impl FnOnce(
            &mut LowerContext<'_, T>,
            InterfaceType,
            &mut MaybeUninit<LowerParams>,
        ) -> Result<()>,
        lift: impl FnOnce(&mut LiftContext<'_>, InterfaceType, &LowerReturn) -> Result<Return>,
    ) -> Result<Return>
    where
        LowerParams: Copy,
        LowerReturn: Copy,
    {
        let export = self.lifted_core_func(store.0);

        #[repr(C)]
        union Union<Params: Copy, Return: Copy> {
            params: Params,
            ret: Return,
        }

        let space = &mut MaybeUninit::<Union<LowerParams, LowerReturn>>::uninit();

        // Double-check the size/alignment of `space`, just in case.
        //
        // Note that this alone is not enough to guarantee the validity of the
        // `unsafe` block below, but it's definitely required. In any case LLVM
        // should be able to trivially see through these assertions and remove
        // them in release mode.
        let val_size = mem::size_of::<ValRaw>();
        let val_align = mem::align_of::<ValRaw>();
        assert!(mem::size_of_val(space) % val_size == 0);
        assert!(mem::size_of_val(map_maybe_uninit!(space.params)) % val_size == 0);
        assert!(mem::size_of_val(map_maybe_uninit!(space.ret)) % val_size == 0);
        assert!(mem::align_of_val(space) == val_align);
        assert!(mem::align_of_val(map_maybe_uninit!(space.params)) == val_align);
        assert!(mem::align_of_val(map_maybe_uninit!(space.ret)) == val_align);

        self.with_lower_context(store.as_context_mut(), false, |cx, ty| {
            cx.enter_call();
            lower(cx, ty, map_maybe_uninit!(space.params))
        })?;

        // SAFETY: We are providing the guarantee that all the inputs are valid.
        // The various pointers passed in for the function are all valid since
        // they're coming from our store, and the `params_and_results` should
        // have the correct layout for the core wasm function we're calling.
        // Note that this latter point relies on the correctness of this module
        // and `ComponentType` implementations, hence `ComponentType` being an
        // `unsafe` trait.
        unsafe {
            crate::Func::call_unchecked_raw(
                &mut store,
                export,
                NonNull::new(core::ptr::slice_from_raw_parts_mut(
                    space.as_mut_ptr().cast(),
                    mem::size_of_val(space) / mem::size_of::<ValRaw>(),
                ))
                .unwrap(),
            )?;
        }

        // SAFETY: We're relying on the correctness of the structure of
        // `LowerReturn` and the type-checking performed to acquire the
        // `TypedFunc` to make this safe. It should be the case that
        // `LowerReturn` is the exact representation of the return value when
        // interpreted as `[ValRaw]`, and additionally they should have the
        // correct types for the function we just called (which filled in the
        // return values).
        let ret: &LowerReturn = unsafe { map_maybe_uninit!(space.ret).assume_init_ref() };

        // Lift the result into the host while managing post-return state
        // here as well.
        //
        // After a successful lift the return value of the function, which
        // is currently required to be 0 or 1 values according to the
        // canonical ABI, is saved within the `Store`'s `FuncData`. This'll
        // later get used in post-return.
        // flags.set_needs_post_return(true);
        let val = self.with_lift_context(store.0, |cx, ty| lift(cx, ty, ret))?;

        // SAFETY: it's a contract of this function that `LowerReturn` is an
        // appropriate representation of the result of this function.
        let ret_slice = unsafe { storage_as_slice(ret) };

        self.instance.id().get_mut(store.0).post_return_arg_set(
            self.index,
            match ret_slice.len() {
                0 => ValRaw::i32(0),
                1 => ret_slice[0],
                _ => unreachable!(),
            },
        );
        return Ok(val);
    }

    /// Invokes the `post-return` canonical ABI option, if specified, after a
    /// [`Func::call`] has finished.
    ///
    /// This function is a required method call after a [`Func::call`] completes
    /// successfully. After the embedder has finished processing the return
    /// value then this function must be invoked.
    ///
    /// # Errors
    ///
    /// This function will return an error in the case of a WebAssembly trap
    /// happening during the execution of the `post-return` function, if
    /// specified.
    ///
    /// # Panics
    ///
    /// This function will panic if it's not called under the correct
    /// conditions. This can only be called after a previous invocation of
    /// [`Func::call`] completes successfully, and this function can only
    /// be called for the same [`Func`] that was `call`'d.
    ///
    /// If this function is called when [`Func::call`] was not previously
    /// called, then it will panic. If a different [`Func`] for the same
    /// component instance was invoked then this function will also panic
    /// because the `post-return` needs to happen for the other function.
    ///
    /// Panics if this is called on a function in an asynchronous store.
    /// This only works with functions defined within a synchronous store.
    #[inline]
    pub fn post_return(&self, mut store: impl AsContextMut) -> Result<()> {
        let store = store.as_context_mut();
        assert!(
            !store.0.async_support(),
            "must use `post_return_async` when async support is enabled on the config"
        );
        self.post_return_impl(store)
    }

    /// Exactly like [`Self::post_return`] except for use on async stores.
    ///
    /// # Panics
    ///
    /// Panics if this is called on a function in a synchronous store. This
    /// only works with functions defined within an asynchronous store.
    #[cfg(feature = "async")]
    pub async fn post_return_async(&self, mut store: impl AsContextMut<Data: Send>) -> Result<()> {
        let mut store = store.as_context_mut();
        assert!(
            store.0.async_support(),
            "cannot use `post_return_async` without enabling async support in the config"
        );
        // Future optimization opportunity: conditionally use a fiber here since
        // some func's post_return will not need the async context (i.e. end up
        // calling async host functionality)
        store.on_fiber(|store| self.post_return_impl(store)).await?
    }

    fn post_return_impl(&self, mut store: impl AsContextMut) -> Result<()> {
        let mut store = store.as_context_mut();

        let index = self.index;
        let vminstance = self.instance.id().get(store.0);
        let component = vminstance.component();
        let (_ty, _def, options) = component.export_lifted_function(index);
        let post_return = self.post_return_core_func(store.0);
        let mut flags =
            vminstance.instance_flags(component.env_component().options[options].instance);
        let mut instance = self.instance.id().get_mut(store.0);
        let post_return_arg = instance.as_mut().post_return_arg_take(index);

        unsafe {
            // First assert that the instance is in a "needs post return" state.
            // This will ensure that the previous action on the instance was a
            // function call above. This flag is only set after a component
            // function returns so this also can't be called (as expected)
            // during a host import for example.
            //
            // Note, though, that this assert is not sufficient because it just
            // means some function on this instance needs its post-return
            // called. We need a precise post-return for a particular function
            // which is the second assert here (the `.expect`). That will assert
            // that this function itself needs to have its post-return called.
            //
            // The theory at least is that these two asserts ensure component
            // model semantics are upheld where the host properly calls
            // `post_return` on the right function despite the call being a
            // separate step in the API.
            assert!(
                flags.needs_post_return(),
                "post_return can only be called after a function has previously been called",
            );
            let post_return_arg = post_return_arg.expect("calling post_return on wrong function");

            // This is a sanity-check assert which shouldn't ever trip.
            assert!(!flags.may_enter());

            // Unset the "needs post return" flag now that post-return is being
            // processed. This will cause future invocations of this method to
            // panic, even if the function call below traps.
            flags.set_needs_post_return(false);

            // If the function actually had a `post-return` configured in its
            // canonical options that's executed here.
            //
            // Note that if this traps (returns an error) this function
            // intentionally leaves the instance in a "poisoned" state where it
            // can no longer be entered because `may_enter` is `false`.
            if let Some(func) = post_return {
                crate::Func::call_unchecked_raw(
                    &mut store,
                    func,
                    NonNull::new(core::ptr::slice_from_raw_parts(&post_return_arg, 1).cast_mut())
                        .unwrap(),
                )?;
            }

            // And finally if everything completed successfully then the "may
            // enter" flag is set to `true` again here which enables further use
            // of the component.
            flags.set_may_enter(true);

            let (calls, host_table, _, instance) = store
                .0
                .component_resource_state_with_instance(self.instance);
            ResourceTables {
                host_table: Some(host_table),
                calls,
                guest: Some(instance.guest_tables()),
            }
            .exit_call()?;
        }
        Ok(())
    }

    fn lower_args<T>(
        cx: &mut LowerContext<'_, T>,
        params: &[Val],
        params_ty: InterfaceType,
        dst: &mut [MaybeUninit<ValRaw>],
    ) -> Result<()> {
        let params_ty = match params_ty {
            InterfaceType::Tuple(i) => &cx.types[i],
            _ => unreachable!(),
        };
        if params_ty.abi.flat_count(MAX_FLAT_PARAMS).is_some() {
            let dst = &mut dst.iter_mut();

            params
                .iter()
                .zip(params_ty.types.iter())
                .try_for_each(|(param, ty)| param.lower(cx, *ty, dst))
        } else {
            Self::store_args(cx, &params_ty, params, dst)
        }
    }

    fn store_args<T>(
        cx: &mut LowerContext<'_, T>,
        params_ty: &TypeTuple,
        args: &[Val],
        dst: &mut [MaybeUninit<ValRaw>],
    ) -> Result<()> {
        let size = usize::try_from(params_ty.abi.size32).unwrap();
        let ptr = cx.realloc(0, 0, params_ty.abi.align32, size)?;
        let mut offset = ptr;
        for (ty, arg) in params_ty.types.iter().zip(args) {
            let abi = cx.types.canonical_abi(ty);
            arg.store(cx, *ty, abi.next_field32_size(&mut offset))?;
        }

        dst[0].write(ValRaw::i64(ptr as i64));

        Ok(())
    }

    fn lift_results<'a, 'b>(
        cx: &'a mut LiftContext<'b>,
        results_ty: InterfaceType,
        src: &'a [ValRaw],
        max_flat: usize,
    ) -> Result<Box<dyn Iterator<Item = Result<Val>> + 'a>> {
        let results_ty = match results_ty {
            InterfaceType::Tuple(i) => &cx.types[i],
            _ => unreachable!(),
        };
        if results_ty.abi.flat_count(max_flat).is_some() {
            let mut flat = src.iter();
            Ok(Box::new(
                results_ty
                    .types
                    .iter()
                    .map(move |ty| Val::lift(cx, *ty, &mut flat)),
            ))
        } else {
            let iter = Self::load_results(cx, results_ty, &mut src.iter())?;
            Ok(Box::new(iter))
        }
    }

    fn load_results<'a, 'b>(
        cx: &'a mut LiftContext<'b>,
        results_ty: &'a TypeTuple,
        src: &mut core::slice::Iter<'_, ValRaw>,
    ) -> Result<impl Iterator<Item = Result<Val>> + use<'a, 'b>> {
        // FIXME(#4311): needs to read an i64 for memory64
        let ptr = usize::try_from(src.next().unwrap().get_u32())?;
        if ptr % usize::try_from(results_ty.abi.align32)? != 0 {
            bail!("return pointer not aligned");
        }

        let bytes = cx
            .memory()
            .get(ptr..)
            .and_then(|b| b.get(..usize::try_from(results_ty.abi.size32).unwrap()))
            .ok_or_else(|| anyhow::anyhow!("pointer out of bounds of memory"))?;

        let mut offset = 0;
        Ok(results_ty.types.iter().map(move |ty| {
            let abi = cx.types.canonical_abi(ty);
            let offset = abi.next_field32_size(&mut offset);
            Val::load(cx, *ty, &bytes[offset..][..abi.size32 as usize])
        }))
    }

    #[cfg(feature = "component-model-async")]
    pub(crate) fn instance(self) -> Instance {
        self.instance
    }

    #[cfg(feature = "component-model-async")]
    pub(crate) fn index(self) -> ExportIndex {
        self.index
    }

    /// Creates a `LowerContext` using the configuration values of this lifted
    /// function.
    ///
    /// The `lower` closure provided should perform the actual lowering and
    /// return the result of the lowering operation which is then returned from
    /// this function as well.
    fn with_lower_context<T>(
        self,
        mut store: StoreContextMut<T>,
        may_enter: bool,
        lower: impl FnOnce(&mut LowerContext<T>, InterfaceType) -> Result<()>,
    ) -> Result<()> {
        let types = self.instance.id().get(store.0).component().types().clone();
        let (options, mut flags, ty, _) = self.abi_info(store.0);

        // Test the "may enter" flag which is a "lock" on this instance.
        // This is immediately set to `false` afterwards and note that
        // there's no on-cleanup setting this flag back to true. That's an
        // intentional design aspect where if anything goes wrong internally
        // from this point on the instance is considered "poisoned" and can
        // never be entered again. The only time this flag is set to `true`
        // again is after post-return logic has completed successfully.
        unsafe {
            if !flags.may_enter() {
                bail!(crate::Trap::CannotEnterComponent);
            }
            flags.set_may_enter(false);
        }

        // Perform the actual lowering, where while this is running the
        // component is forbidden from calling imports.
        unsafe {
            debug_assert!(flags.may_leave());
            flags.set_may_leave(false);
        }
        let mut cx = LowerContext::new(store.as_context_mut(), &options, &types, self.instance);
        let result = lower(&mut cx, InterfaceType::Tuple(types[ty].params));
        unsafe { flags.set_may_leave(true) };
        result?;

        // If this is an async function and `may_enter == true` then we're
        // allowed to reenter the component at this point, and otherwise flag a
        // post-return call being required as we're about to enter wasm and
        // afterwards need a post-return.
        unsafe {
            if may_enter && options.async_() {
                flags.set_may_enter(true);
            } else {
                flags.set_needs_post_return(true);
            }
        }

        Ok(())
    }

    /// Creates a `LiftContext` using the configuration values with this lifted
    /// function.
    ///
    /// The closure `lift` provided should actually perform the lift itself and
    /// the result of that closure is returned from this function call as well.
    fn with_lift_context<R>(
        self,
        store: &mut StoreOpaque,
        lift: impl FnOnce(&mut LiftContext, InterfaceType) -> Result<R>,
    ) -> Result<R> {
        let (options, _flags, ty, _) = self.abi_info(store);
        let mut cx = LiftContext::new(store, &options, self.instance);
        let ty = InterfaceType::Tuple(cx.types[ty].results);
        lift(&mut cx, ty)
    }
}
