use crate::component::Instance;
use crate::component::func::{Func, LiftContext, LowerContext, Options};
use crate::component::matching::InstanceType;
use crate::component::storage::{storage_as_slice, storage_as_slice_mut};
use crate::prelude::*;
use crate::{AsContextMut, StoreContext, StoreContextMut, ValRaw};
use alloc::borrow::Cow;
use core::fmt;
use core::iter;
use core::marker;
use core::mem::{self, MaybeUninit};
use core::str;
use wasmtime_environ::component::{
    CanonicalAbiInfo, InterfaceType, MAX_FLAT_PARAMS, MAX_FLAT_RESULTS, StringEncoding, VariantInfo,
};

#[cfg(feature = "component-model-async")]
use crate::component::concurrent::{self, AsAccessor, PreparedCall};

/// A statically-typed version of [`Func`] which takes `Params` as input and
/// returns `Return`.
///
/// This is an efficient way to invoke a WebAssembly component where if the
/// inputs and output are statically known this can eschew the vast majority of
/// machinery and checks when calling WebAssembly. This is the most optimized
/// way to call a WebAssembly component.
///
/// Note that like [`Func`] this is a pointer within a [`Store`](crate::Store)
/// and usage will panic if used with the wrong store.
///
/// This type is primarily created with the [`Func::typed`] API.
///
/// See [`ComponentType`] for more information about supported types.
pub struct TypedFunc<Params, Return> {
    func: Func,

    // The definition of this field is somewhat subtle and may be surprising.
    // Naively one might expect something like
    //
    //      _marker: marker::PhantomData<fn(Params) -> Return>,
    //
    // Since this is a function pointer after all. The problem with this
    // definition though is that it imposes the wrong variance on `Params` from
    // what we want. Abstractly a `fn(Params)` is able to store `Params` within
    // it meaning you can only give it `Params` that live longer than the
    // function pointer.
    //
    // With a component model function, however, we're always copying data from
    // the host into the guest, so we are never storing pointers to `Params`
    // into the guest outside the duration of a `call`, meaning we can actually
    // accept values in `TypedFunc::call` which live for a shorter duration
    // than the `Params` argument on the struct.
    //
    // This all means that we don't use a phantom function pointer, but instead
    // feign phantom storage here to get the variance desired.
    _marker: marker::PhantomData<(Params, Return)>,
}

impl<Params, Return> Copy for TypedFunc<Params, Return> {}

impl<Params, Return> Clone for TypedFunc<Params, Return> {
    fn clone(&self) -> TypedFunc<Params, Return> {
        *self
    }
}

impl<Params, Return> TypedFunc<Params, Return>
where
    Params: ComponentNamedList + Lower,
    Return: ComponentNamedList + Lift,
{
    /// Creates a new [`TypedFunc`] from the provided component [`Func`],
    /// unsafely asserting that the underlying function takes `Params` as
    /// input and returns `Return`.
    ///
    /// # Unsafety
    ///
    /// This is an unsafe function because it does not verify that the [`Func`]
    /// provided actually implements this signature. It's up to the caller to
    /// have performed some other sort of check to ensure that the signature is
    /// correct.
    pub unsafe fn new_unchecked(func: Func) -> TypedFunc<Params, Return> {
        TypedFunc {
            _marker: marker::PhantomData,
            func,
        }
    }

    /// Returns the underlying un-typed [`Func`] that this [`TypedFunc`]
    /// references.
    pub fn func(&self) -> &Func {
        &self.func
    }

    /// Calls the underlying WebAssembly component function using the provided
    /// `params` as input.
    ///
    /// This method is used to enter into a component. Execution happens within
    /// the `store` provided. The `params` are copied into WebAssembly memory
    /// as appropriate and a core wasm function is invoked.
    ///
    /// # Post-return
    ///
    /// In the component model each function can have a "post return" specified
    /// which allows cleaning up the arguments returned to the host. For example
    /// if WebAssembly returns a string to the host then it might be a uniquely
    /// allocated string which, after the host finishes processing it, needs to
    /// be deallocated in the wasm instance's own linear memory to prevent
    /// memory leaks in wasm itself. The `post-return` canonical abi option is
    /// used to configured this.
    ///
    /// To accommodate this feature of the component model after invoking a
    /// function via [`TypedFunc::call`] you must next invoke
    /// [`TypedFunc::post_return`]. Note that the return value of the function
    /// should be processed between these two function calls. The return value
    /// continues to be usable from an embedder's perspective after
    /// `post_return` is called, but after `post_return` is invoked it may no
    /// longer retain the same value that the wasm module originally returned.
    ///
    /// Also note that [`TypedFunc::post_return`] must be invoked irrespective
    /// of whether the canonical ABI option `post-return` was configured or not.
    /// This means that embedders must unconditionally call
    /// [`TypedFunc::post_return`] when a function returns. If this function
    /// call returns an error, however, then [`TypedFunc::post_return`] is not
    /// required.
    ///
    /// # Errors
    ///
    /// This function can return an error for a number of reasons:
    ///
    /// * If the wasm itself traps during execution.
    /// * If the wasm traps while copying arguments into memory.
    /// * If the wasm provides bad allocation pointers when copying arguments
    ///   into memory.
    /// * If the wasm returns a value which violates the canonical ABI.
    /// * If this function's instances cannot be entered, for example if the
    ///   instance is currently calling a host function.
    /// * If a previous function call occurred and the corresponding
    ///   `post_return` hasn't been invoked yet.
    ///
    /// In general there are many ways that things could go wrong when copying
    /// types in and out of a wasm module with the canonical ABI, and certain
    /// error conditions are specific to certain types. For example a
    /// WebAssembly module can't return an invalid `char`. When allocating space
    /// for this host to copy a string into the returned pointer must be
    /// in-bounds in memory.
    ///
    /// If an error happens then the error should contain detailed enough
    /// information to understand which part of the canonical ABI went wrong
    /// and what to inspect.
    ///
    /// # Panics
    ///
    /// Panics if this is called on a function in an asynchronous store. This
    /// only works with functions defined within a synchronous store. Also
    /// panics if `store` does not own this function.
    pub fn call(&self, store: impl AsContextMut, params: Params) -> Result<Return> {
        assert!(
            !store.as_context().async_support(),
            "must use `call_async` when async support is enabled on the config"
        );
        self.call_impl(store, params)
    }

    /// Exactly like [`Self::call`], except for use on asynchronous stores.
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
        params: Params,
    ) -> Result<Return>
    where
        Return: 'static,
    {
        let mut store = store.as_context_mut();
        assert!(
            store.0.async_support(),
            "cannot use `call_async` when async support is not enabled on the config"
        );
        #[cfg(feature = "component-model-async")]
        {
            use crate::component::concurrent::TaskId;
            use crate::runtime::vm::SendSyncPtr;
            use core::ptr::NonNull;

            let ptr = SendSyncPtr::from(NonNull::from(&params).cast::<u8>());
            let prepared =
                self.prepare_call(store.as_context_mut(), false, false, move |cx, ty, dst| {
                    // SAFETY: The goal here is to get `Params`, a non-`'static`
                    // value, to live long enough to the lowering of the
                    // parameters. We're guaranteed that `Params` lives in the
                    // future of the outer function (we're in an `async fn`) so it'll
                    // stay alive as long as the future itself. That is distinct,
                    // for example, from the signature of `call_concurrent` below.
                    //
                    // Here a pointer to `Params` is smuggled to this location
                    // through a `SendSyncPtr<u8>` to thwart the `'static` check
                    // of rustc and the signature of `prepare_call`.
                    //
                    // Note the use of `RemoveOnDrop` in the code that follows
                    // this closure, which ensures that the task will be removed
                    // from the concurrent state to which it belongs when the
                    // containing `Future` is dropped, thereby ensuring that
                    // this closure will never be called if it hasn't already,
                    // meaning it will never see a dangling pointer.
                    let params = unsafe { ptr.cast::<Params>().as_ref() };
                    Self::lower_args(cx, ty, dst, params)
                })?;

            struct RemoveOnDrop<'a, T: 'static> {
                store: StoreContextMut<'a, T>,
                task: TaskId,
            }

            impl<'a, T> Drop for RemoveOnDrop<'a, T> {
                fn drop(&mut self) {
                    self.task.remove(self.store.as_context_mut()).unwrap();
                }
            }

            let mut wrapper = RemoveOnDrop {
                store,
                task: prepared.task_id(),
            };

            let result = concurrent::queue_call(wrapper.store.as_context_mut(), prepared)?;
            self.func
                .instance
                .run_concurrent(wrapper.store.as_context_mut(), async |_| result.await)
                .await?
        }
        #[cfg(not(feature = "component-model-async"))]
        {
            store
                .on_fiber(|store| self.call_impl(store, params))
                .await?
        }
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
        params: Params,
    ) -> Result<Return>
    where
        Params: 'static,
        Return: 'static,
    {
        let accessor = accessor.as_accessor();
        let result = accessor.with(|mut store| {
            let mut store = store.as_context_mut();
            assert!(
                store.0.async_support(),
                "cannot use `call_concurrent` when async support is not enabled on the config"
            );

            let prepared =
                self.prepare_call(store.as_context_mut(), true, true, move |cx, ty, dst| {
                    Self::lower_args(cx, ty, dst, &params)
                })?;
            concurrent::queue_call(store, prepared)
        });

        result?.await
    }

    fn lower_args<T>(
        cx: &mut LowerContext<T>,
        ty: InterfaceType,
        dst: &mut [MaybeUninit<ValRaw>],
        params: &Params,
    ) -> Result<()> {
        use crate::component::storage::slice_to_storage_mut;

        if Params::flatten_count() <= MAX_FLAT_PARAMS {
            // SAFETY: the safety of `slice_to_storage_mut` relies on
            // `Params::Lower` being represented by a sequence of
            // `ValRaw`, and that's a guarantee upheld by the `Lower`
            // trait itself.
            let dst: &mut MaybeUninit<Params::Lower> = unsafe { slice_to_storage_mut(dst) };
            Self::lower_stack_args(cx, &params, ty, dst)
        } else {
            Self::lower_heap_args(cx, &params, ty, &mut dst[0])
        }
    }

    /// Calls `concurrent::prepare_call` with monomorphized functions for
    /// lowering the parameters and lifting the result according to the number
    /// of core Wasm parameters and results in the signature of the function to
    /// be called.
    #[cfg(feature = "component-model-async")]
    fn prepare_call<T>(
        self,
        store: StoreContextMut<'_, T>,
        remove_task_automatically: bool,
        call_post_return_automatically: bool,
        lower: impl FnOnce(
            &mut LowerContext<T>,
            InterfaceType,
            &mut [MaybeUninit<ValRaw>],
        ) -> Result<()>
        + Send
        + Sync
        + 'static,
    ) -> Result<PreparedCall<Return>>
    where
        Return: 'static,
    {
        use crate::component::storage::slice_to_storage;

        let param_count = if Params::flatten_count() <= MAX_FLAT_PARAMS {
            Params::flatten_count()
        } else {
            1
        };
        let max_results = if self.func.abi_async(store.0) {
            MAX_FLAT_PARAMS
        } else {
            MAX_FLAT_RESULTS
        };
        concurrent::prepare_call(
            store,
            self.func,
            param_count,
            remove_task_automatically,
            call_post_return_automatically,
            move |func, store, params_out| {
                func.with_lower_context(store, call_post_return_automatically, |cx, ty| {
                    lower(cx, ty, params_out)
                })
            },
            move |func, store, results| {
                let result = if Return::flatten_count() <= max_results {
                    func.with_lift_context(store, |cx, ty| {
                        // SAFETY: Per the safety requiments documented for the
                        // `ComponentType` trait, `Return::Lower` must be
                        // compatible at the binary level with a `[ValRaw; N]`,
                        // where `N` is `mem::size_of::<Return::Lower>() /
                        // mem::size_of::<ValRaw>()`.  And since this function
                        // is only used when `Return::flatten_count() <=
                        // MAX_FLAT_RESULTS` and `MAX_FLAT_RESULTS == 1`, `N`
                        // can only either be 0 or 1.
                        //
                        // See `ComponentInstance::exit_call` for where we use
                        // the result count passed from
                        // `wasmtime_environ::fact::trampoline`-generated code
                        // to ensure the slice has the correct length, and also
                        // `concurrent::start_call` for where we conservatively
                        // use a slice length of 1 unconditionally.  Also note
                        // that, as of this writing `slice_to_storage`
                        // double-checks the slice length is sufficient.
                        let results: &Return::Lower = unsafe { slice_to_storage(results) };
                        Self::lift_stack_result(cx, ty, results)
                    })?
                } else {
                    func.with_lift_context(store, |cx, ty| {
                        Self::lift_heap_result(cx, ty, &results[0])
                    })?
                };
                Ok(Box::new(result))
            },
        )
    }

    fn call_impl(&self, mut store: impl AsContextMut, params: Params) -> Result<Return> {
        let store = store.as_context_mut();

        if self.func.abi_async(store.0) {
            bail!("must enable the `component-model-async` feature to call async-lifted exports")
        }

        // Note that this is in theory simpler than it might read at this time.
        // Here we're doing a runtime dispatch on the `flatten_count` for the
        // params/results to see whether they're inbounds. This creates 4 cases
        // to handle. In reality this is a highly optimizable branch where LLVM
        // will easily figure out that only one branch here is taken.
        //
        // Otherwise this current construction is done to ensure that the stack
        // space reserved for the params/results is always of the appropriate
        // size (as the params/results needed differ depending on the "flatten"
        // count)
        //
        // SAFETY: the safety of these invocations of `call_raw` depends on the
        // correctness of the ascription of the `LowerParams` and `LowerReturn`
        // types on the `call_raw` function. That's upheld here through the
        // safety requirements of `Lift` and `Lower` on `Params` and `Return` in
        // combination with checking the various possible branches here and
        // dispatching to appropriately typed functions.
        unsafe {
            // This type is used as `LowerParams` for `call_raw` which is either
            // `Params::Lower` or `ValRaw` representing it's either on the stack
            // or it's on the heap. This allocates 1 extra `ValRaw` on the stack
            // if `Params` is empty and `Return` is also empty, but that's a
            // reasonable enough price to pay for now given the current code
            // organization.
            #[derive(Copy, Clone)]
            union Union<T: Copy, U: Copy> {
                _a: T,
                _b: U,
            }

            if Return::flatten_count() <= MAX_FLAT_RESULTS {
                self.func.call_raw(
                    store,
                    |cx, ty, dst: &mut MaybeUninit<Union<Params::Lower, ValRaw>>| {
                        let dst = storage_as_slice_mut(dst);
                        Self::lower_args(cx, ty, dst, &params)
                    },
                    Self::lift_stack_result,
                )
            } else {
                self.func.call_raw(
                    store,
                    |cx, ty, dst: &mut MaybeUninit<Union<Params::Lower, ValRaw>>| {
                        let dst = storage_as_slice_mut(dst);
                        Self::lower_args(cx, ty, dst, &params)
                    },
                    Self::lift_heap_result,
                )
            }
        }
    }

    /// Lower parameters directly onto the stack specified by the `dst`
    /// location.
    ///
    /// This is only valid to call when the "flatten count" is small enough, or
    /// when the canonical ABI says arguments go through the stack rather than
    /// the heap.
    fn lower_stack_args<T>(
        cx: &mut LowerContext<'_, T>,
        params: &Params,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Params::Lower>,
    ) -> Result<()> {
        assert!(Params::flatten_count() <= MAX_FLAT_PARAMS);
        params.linear_lower_to_flat(cx, ty, dst)?;
        Ok(())
    }

    /// Lower parameters onto a heap-allocated location.
    ///
    /// This is used when the stack space to be used for the arguments is above
    /// the `MAX_FLAT_PARAMS` threshold. Here the wasm's `realloc` function is
    /// invoked to allocate space and then parameters are stored at that heap
    /// pointer location.
    fn lower_heap_args<T>(
        cx: &mut LowerContext<'_, T>,
        params: &Params,
        ty: InterfaceType,
        dst: &mut MaybeUninit<ValRaw>,
    ) -> Result<()> {
        // Memory must exist via validation if the arguments are stored on the
        // heap, so we can create a `MemoryMut` at this point. Afterwards
        // `realloc` is used to allocate space for all the arguments and then
        // they're all stored in linear memory.
        //
        // Note that `realloc` will bake in a check that the returned pointer is
        // in-bounds.
        let ptr = cx.realloc(0, 0, Params::ALIGN32, Params::SIZE32)?;
        params.linear_lower_to_memory(cx, ty, ptr)?;

        // Note that the pointer here is stored as a 64-bit integer. This allows
        // this to work with either 32 or 64-bit memories. For a 32-bit memory
        // it'll just ignore the upper 32 zero bits, and for 64-bit memories
        // this'll have the full 64-bits. Note that for 32-bit memories the call
        // to `realloc` above guarantees that the `ptr` is in-bounds meaning
        // that we will know that the zero-extended upper bits of `ptr` are
        // guaranteed to be zero.
        //
        // This comment about 64-bit integers is also referred to below with
        // "WRITEPTR64".
        dst.write(ValRaw::i64(ptr as i64));

        Ok(())
    }

    /// Lift the result of a function directly from the stack result.
    ///
    /// This is only used when the result fits in the maximum number of stack
    /// slots.
    fn lift_stack_result(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        dst: &Return::Lower,
    ) -> Result<Return> {
        Return::linear_lift_from_flat(cx, ty, dst)
    }

    /// Lift the result of a function where the result is stored indirectly on
    /// the heap.
    fn lift_heap_result(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        dst: &ValRaw,
    ) -> Result<Return> {
        assert!(Return::flatten_count() > MAX_FLAT_RESULTS);
        // FIXME(#4311): needs to read an i64 for memory64
        let ptr = usize::try_from(dst.get_u32())?;
        if ptr % usize::try_from(Return::ALIGN32)? != 0 {
            bail!("return pointer not aligned");
        }

        let bytes = cx
            .memory()
            .get(ptr..)
            .and_then(|b| b.get(..Return::SIZE32))
            .ok_or_else(|| anyhow::anyhow!("pointer out of bounds of memory"))?;
        Return::linear_lift_from_memory(cx, ty, bytes)
    }

    /// See [`Func::post_return`]
    pub fn post_return(&self, store: impl AsContextMut) -> Result<()> {
        self.func.post_return(store)
    }

    /// See [`Func::post_return_async`]
    #[cfg(feature = "async")]
    pub async fn post_return_async<T: Send>(
        &self,
        store: impl AsContextMut<Data = T>,
    ) -> Result<()> {
        self.func.post_return_async(store).await
    }
}

/// A trait representing a static list of named types that can be passed to or
/// returned from a [`TypedFunc`].
///
/// This trait is implemented for a number of tuple types and is not expected
/// to be implemented externally. The contents of this trait are hidden as it's
/// intended to be an implementation detail of Wasmtime. The contents of this
/// trait are not covered by Wasmtime's stability guarantees.
///
/// For more information about this trait see [`Func::typed`] and
/// [`TypedFunc`].
//
// Note that this is an `unsafe` trait, and the unsafety means that
// implementations of this trait must be correct or otherwise [`TypedFunc`]
// would not be memory safe. The main reason this is `unsafe` is the
// `typecheck` function which must operate correctly relative to the `AsTuple`
// interpretation of the implementor.
pub unsafe trait ComponentNamedList: ComponentType {}

/// A trait representing types which can be passed to and read from components
/// with the canonical ABI.
///
/// This trait is implemented for Rust types which can be communicated to
/// components. The [`Func::typed`] and [`TypedFunc`] Rust items are the main
/// consumers of this trait.
///
/// Supported Rust types include:
///
/// | Component Model Type              | Rust Type                            |
/// |-----------------------------------|--------------------------------------|
/// | `{s,u}{8,16,32,64}`               | `{i,u}{8,16,32,64}`                  |
/// | `f{32,64}`                        | `f{32,64}`                           |
/// | `bool`                            | `bool`                               |
/// | `char`                            | `char`                               |
/// | `tuple<A, B>`                     | `(A, B)`                             |
/// | `option<T>`                       | `Option<T>`                          |
/// | `result`                          | `Result<(), ()>`                     |
/// | `result<T>`                       | `Result<T, ()>`                      |
/// | `result<_, E>`                    | `Result<(), E>`                      |
/// | `result<T, E>`                    | `Result<T, E>`                       |
/// | `string`                          | `String`, `&str`, or [`WasmStr`]     |
/// | `list<T>`                         | `Vec<T>`, `&[T]`, or [`WasmList`]    |
/// | `own<T>`, `borrow<T>`             | [`Resource<T>`] or [`ResourceAny`]   |
/// | `record`                          | [`#[derive(ComponentType)]`][d-cm]   |
/// | `variant`                         | [`#[derive(ComponentType)]`][d-cm]   |
/// | `enum`                            | [`#[derive(ComponentType)]`][d-cm]   |
/// | `flags`                           | [`flags!`][f-m]                      |
///
/// [`Resource<T>`]: crate::component::Resource
/// [`ResourceAny`]: crate::component::ResourceAny
/// [d-cm]: macro@crate::component::ComponentType
/// [f-m]: crate::component::flags
///
/// Rust standard library pointers such as `&T`, `Box<T>`, and `Arc<T>`
/// additionally represent whatever type `T` represents in the component model.
/// Note that types such as `record`, `variant`, `enum`, and `flags` are
/// generated by the embedder at compile time. These macros derive
/// implementation of this trait for custom types to map to custom types in the
/// component model. Note that for `record`, `variant`, `enum`, and `flags`
/// those types are often generated by the
/// [`bindgen!`](crate::component::bindgen) macro from WIT definitions.
///
/// Types that implement [`ComponentType`] are used for `Params` and `Return`
/// in [`TypedFunc`] and [`Func::typed`].
///
/// The contents of this trait are hidden as it's intended to be an
/// implementation detail of Wasmtime. The contents of this trait are not
/// covered by Wasmtime's stability guarantees.
///
/// # Safety
///
/// Note that this is an `unsafe` trait as `TypedFunc`'s safety heavily relies on
/// the correctness of the implementations of this trait. Some ways in which this
/// trait must be correct to be safe are:
///
/// * The `Lower` associated type must be a `ValRaw` sequence. It doesn't have to
///   literally be `[ValRaw; N]` but when laid out in memory it must be adjacent
///   `ValRaw` values and have a multiple of the size of `ValRaw` and the same
///   alignment.
///
/// * The `lower` function must initialize the bits within `Lower` that are going
///   to be read by the trampoline that's used to enter core wasm. A trampoline
///   is passed `*mut Lower` and will read the canonical abi arguments in
///   sequence, so all of the bits must be correctly initialized.
///
/// * The `size` and `align` functions must be correct for this value stored in
///   the canonical ABI. The `Cursor<T>` iteration of these bytes rely on this
///   for correctness as they otherwise eschew bounds-checking.
///
/// There are likely some other correctness issues which aren't documented as
/// well, this isn't currently an exhaustive list. It suffices to say, though,
/// that correctness bugs in this trait implementation are highly likely to
/// lead to security bugs, which again leads to the `unsafe` in the trait.
///
/// Note that this trait specifically is not sealed because `bindgen!`-generated
/// types must be able to implement this trait using a `#[derive]` macro. For
/// users it's recommended to not implement this trait manually given the
/// non-exhaustive list of safety requirements that must be upheld. This trait
/// is implemented at your own risk if you do so.
///
/// # Send and Sync
///
/// While on the topic of safety it's worth discussing the `Send` and `Sync`
/// bounds here as well. These bounds might naively seem like they shouldn't be
/// required for all component types as they're host-level types not guest-level
/// types persisted anywhere. Various subtleties lead to these bounds, however:
///
/// * Fibers require that all stack-local variables are `Send` and `Sync` for
///   fibers themselves to be send/sync. Unfortunately we have no help from the
///   compiler on this one so it's up to Wasmtime's discipline to maintain this.
///   One instance of this is that return values are placed on the stack as
///   they're lowered into guest memory. This lowering operation can involve
///   malloc and context switches, so return values must be Send/Sync.
///
/// * In the implementation of component model async it's not uncommon for types
///   to be "buffered" in the store temporarily. For example parameters might
///   reside in a store temporarily while wasm has backpressure turned on.
///
/// Overall it's generally easiest to require `Send` and `Sync` for all
/// component types. There additionally aren't known use case for non-`Send` or
/// non-`Sync` types at this time.
pub unsafe trait ComponentType: Send + Sync {
    /// Representation of the "lowered" form of this component value.
    ///
    /// Lowerings lower into core wasm values which are represented by `ValRaw`.
    /// This `Lower` type must be a list of `ValRaw` as either a literal array
    /// or a struct where every field is a `ValRaw`. This must be `Copy` (as
    /// `ValRaw` is `Copy`) and support all byte patterns. This being correct is
    /// one reason why the trait is unsafe.
    #[doc(hidden)]
    type Lower: Copy;

    /// The information about this type's canonical ABI (size/align/etc).
    #[doc(hidden)]
    const ABI: CanonicalAbiInfo;

    #[doc(hidden)]
    const SIZE32: usize = Self::ABI.size32 as usize;
    #[doc(hidden)]
    const ALIGN32: u32 = Self::ABI.align32;

    #[doc(hidden)]
    const IS_RUST_UNIT_TYPE: bool = false;

    /// Whether this type might require a call to the guest's realloc function
    /// to allocate linear memory when lowering (e.g. a non-empty `string`).
    ///
    /// If this is `false`, Wasmtime may optimize lowering by using
    /// `LowerContext::new_without_realloc` and lowering values outside of any
    /// fiber.  That will panic if the lowering process ends up needing realloc
    /// after all, so `true` is a conservative default.
    #[doc(hidden)]
    const MAY_REQUIRE_REALLOC: bool = true;

    /// Returns the number of core wasm abi values will be used to represent
    /// this type in its lowered form.
    ///
    /// This divides the size of `Self::Lower` by the size of `ValRaw`.
    #[doc(hidden)]
    fn flatten_count() -> usize {
        assert!(mem::size_of::<Self::Lower>() % mem::size_of::<ValRaw>() == 0);
        assert!(mem::align_of::<Self::Lower>() == mem::align_of::<ValRaw>());
        mem::size_of::<Self::Lower>() / mem::size_of::<ValRaw>()
    }

    /// Performs a type-check to see whether this component value type matches
    /// the interface type `ty` provided.
    #[doc(hidden)]
    fn typecheck(ty: &InterfaceType, types: &InstanceType<'_>) -> Result<()>;
}

#[doc(hidden)]
pub unsafe trait ComponentVariant: ComponentType {
    const CASES: &'static [Option<CanonicalAbiInfo>];
    const INFO: VariantInfo = VariantInfo::new_static(Self::CASES);
    const PAYLOAD_OFFSET32: usize = Self::INFO.payload_offset32 as usize;
}

/// Host types which can be passed to WebAssembly components.
///
/// This trait is implemented for all types that can be passed to components
/// either as parameters of component exports or returns of component imports.
/// This trait represents the ability to convert from the native host
/// representation to the canonical ABI.
///
/// Built-in types to Rust such as `Option<T>` implement this trait as
/// appropriate. For a mapping of component model to Rust types see
/// [`ComponentType`].
///
/// For user-defined types, for example `record` types mapped to Rust `struct`s,
/// this crate additionally has
/// [`#[derive(Lower)]`](macro@crate::component::Lower).
///
/// Note that like [`ComponentType`] the definition of this trait is intended to
/// be an internal implementation detail of Wasmtime at this time. It's
/// recommended to use the `#[derive(Lower)]` implementation instead.
pub unsafe trait Lower: ComponentType {
    /// Performs the "lower" function in the linear memory version of the
    /// canonical ABI.
    ///
    /// This method will lower the current value into a component. The `lower`
    /// function performs a "flat" lowering into the `dst` specified which is
    /// allowed to be uninitialized entering this method but is guaranteed to be
    /// fully initialized if the method returns `Ok(())`.
    ///
    /// The `cx` context provided is the context within which this lowering is
    /// happening. This contains information such as canonical options specified
    /// (e.g. string encodings, memories, etc), the store itself, along with
    /// type information.
    ///
    /// The `ty` parameter is the destination type that is being lowered into.
    /// For example this is the component's "view" of the type that is being
    /// lowered. This is guaranteed to have passed a `typecheck` earlier.
    ///
    /// This will only be called if `typecheck` passes for `Op::Lower`.
    #[doc(hidden)]
    fn linear_lower_to_flat<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()>;

    /// Performs the "store" operation in the linear memory version of the
    /// canonical ABI.
    ///
    /// This function will store `self` into the linear memory described by
    /// `cx` at the `offset` provided.
    ///
    /// It is expected that `offset` is a valid offset in memory for
    /// `Self::SIZE32` bytes. At this time that's not an unsafe contract as it's
    /// always re-checked on all stores, but this is something that will need to
    /// be improved in the future to remove extra bounds checks. For now this
    /// function will panic if there's a bug and `offset` isn't valid within
    /// memory.
    ///
    /// The `ty` type information passed here is the same as the type
    /// information passed to `lower` above, and is the component's own view of
    /// what the resulting type should be.
    ///
    /// This will only be called if `typecheck` passes for `Op::Lower`.
    #[doc(hidden)]
    fn linear_lower_to_memory<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()>;

    /// Provided method to lower a list of `Self` into memory.
    ///
    /// Requires that `offset` has already been checked for alignment and
    /// validity in terms of being in-bounds, otherwise this may panic.
    ///
    /// This is primarily here to get overridden for implementations of integers
    /// which can avoid some extra fluff and use a pattern that's more easily
    /// optimizable by LLVM.
    #[doc(hidden)]
    fn linear_store_list_to_memory<T>(
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        mut offset: usize,
        items: &[Self],
    ) -> Result<()>
    where
        Self: Sized,
    {
        for item in items {
            item.linear_lower_to_memory(cx, ty, offset)?;
            offset += Self::SIZE32;
        }
        Ok(())
    }
}

/// Host types which can be created from the canonical ABI.
///
/// This is the mirror of the [`Lower`] trait where it represents the capability
/// of acquiring items from WebAssembly and passing them to the host.
///
/// Built-in types to Rust such as `Option<T>` implement this trait as
/// appropriate. For a mapping of component model to Rust types see
/// [`ComponentType`].
///
/// For user-defined types, for example `record` types mapped to Rust `struct`s,
/// this crate additionally has
/// [`#[derive(Lift)]`](macro@crate::component::Lift).
///
/// Note that like [`ComponentType`] the definition of this trait is intended to
/// be an internal implementation detail of Wasmtime at this time. It's
/// recommended to use the `#[derive(Lift)]` implementation instead.
pub unsafe trait Lift: Sized + ComponentType {
    /// Performs the "lift" operation in the linear memory version of the
    /// canonical ABI.
    ///
    /// This function performs a "flat" lift operation from the `src` specified
    /// which is a sequence of core wasm values. The lifting operation will
    /// validate core wasm values and produce a `Self` on success.
    ///
    /// The `cx` provided contains contextual information such as the store
    /// that's being loaded from, canonical options, and type information.
    ///
    /// The `ty` parameter is the origin component's specification for what the
    /// type that is being lifted is. For example this is the record type or the
    /// resource type that is being lifted.
    ///
    /// Note that this has a default implementation but if `typecheck` passes
    /// for `Op::Lift` this needs to be overridden.
    #[doc(hidden)]
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self>;

    /// Performs the "load" operation in the linear memory version of the
    /// canonical ABI.
    ///
    /// This will read the `bytes` provided, which are a sub-slice into the
    /// linear memory described by `cx`. The `bytes` array provided is
    /// guaranteed to be `Self::SIZE32` bytes large. All of memory is then also
    /// available through `cx` for bounds-checks and such as necessary for
    /// strings/lists.
    ///
    /// The `ty` argument is the type that's being loaded, as described by the
    /// original component.
    ///
    /// Note that this has a default implementation but if `typecheck` passes
    /// for `Op::Lift` this needs to be overridden.
    #[doc(hidden)]
    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self>;

    /// Converts `list` into a `Vec<T>`, used in `Lift for Vec<T>`.
    #[doc(hidden)]
    fn linear_lift_list_from_memory(
        cx: &mut LiftContext<'_>,
        list: &WasmList<Self>,
    ) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        let mut dst = Vec::with_capacity(list.len);
        Self::linear_lift_into_from_memory(cx, list, &mut dst)?;
        Ok(dst)
    }

    /// Load no more than `max_count` items from `list` into `dst`.
    ///
    /// This is primarily here to get overridden for implementations of integers
    /// which can avoid some extra fluff and use a pattern that's more easily
    /// optimizable by LLVM.
    #[doc(hidden)]
    fn linear_lift_into_from_memory(
        cx: &mut LiftContext<'_>,
        list: &WasmList<Self>,
        dst: &mut impl Extend<Self>,
    ) -> Result<()>
    where
        Self: Sized,
    {
        for i in 0..list.len {
            dst.extend(Some(list.get_from_store(cx, i).unwrap()?));
        }
        Ok(())
    }
}

// Macro to help generate "forwarding implementations" of `ComponentType` to
// another type, used for wrappers in Rust like `&T`, `Box<T>`, etc. Note that
// these wrappers only implement lowering because lifting native Rust types
// cannot be done.
macro_rules! forward_type_impls {
    ($(($($generics:tt)*) $a:ty => $b:ty,)*) => ($(
        unsafe impl <$($generics)*> ComponentType for $a {
            type Lower = <$b as ComponentType>::Lower;

            const ABI: CanonicalAbiInfo = <$b as ComponentType>::ABI;

            #[inline]
            fn typecheck(ty: &InterfaceType, types: &InstanceType<'_>) -> Result<()> {
                <$b as ComponentType>::typecheck(ty, types)
            }
        }
    )*)
}

forward_type_impls! {
    (T: ComponentType + ?Sized) &'_ T => T,
    (T: ComponentType + ?Sized) Box<T> => T,
    (T: ComponentType + ?Sized) alloc::sync::Arc<T> => T,
    () String => str,
    (T: ComponentType) Vec<T> => [T],
}

macro_rules! forward_lowers {
    ($(($($generics:tt)*) $a:ty => $b:ty,)*) => ($(
        unsafe impl <$($generics)*> Lower for $a {
            fn linear_lower_to_flat<U>(
                &self,
                cx: &mut LowerContext<'_, U>,
                ty: InterfaceType,
                dst: &mut MaybeUninit<Self::Lower>,
            ) -> Result<()> {
                <$b as Lower>::linear_lower_to_flat(self, cx, ty, dst)
            }

            fn linear_lower_to_memory<U>(
                &self,
                cx: &mut LowerContext<'_, U>,
                ty: InterfaceType,
                offset: usize,
            ) -> Result<()> {
                <$b as Lower>::linear_lower_to_memory(self, cx, ty, offset)
            }
        }
    )*)
}

forward_lowers! {
    (T: Lower + ?Sized) &'_ T => T,
    (T: Lower + ?Sized) Box<T> => T,
    (T: Lower + ?Sized) alloc::sync::Arc<T> => T,
    () String => str,
    (T: Lower) Vec<T> => [T],
}

macro_rules! forward_string_lifts {
    ($($a:ty,)*) => ($(
        unsafe impl Lift for $a {
            #[inline]
            fn linear_lift_from_flat(cx: &mut LiftContext<'_>, ty: InterfaceType, src: &Self::Lower) -> Result<Self> {
                Ok(<WasmStr as Lift>::linear_lift_from_flat(cx, ty, src)?.to_str_from_memory(cx.memory())?.into())
            }

            #[inline]
            fn linear_lift_from_memory(cx: &mut LiftContext<'_>, ty: InterfaceType, bytes: &[u8]) -> Result<Self> {
                Ok(<WasmStr as Lift>::linear_lift_from_memory(cx, ty, bytes)?.to_str_from_memory(cx.memory())?.into())
            }
        }
    )*)
}

forward_string_lifts! {
    Box<str>,
    alloc::sync::Arc<str>,
    String,
}

macro_rules! forward_list_lifts {
    ($($a:ty,)*) => ($(
        unsafe impl <T: Lift> Lift for $a {
            fn linear_lift_from_flat(cx: &mut LiftContext<'_>, ty: InterfaceType, src: &Self::Lower) -> Result<Self> {
                let list = <WasmList::<T> as Lift>::linear_lift_from_flat(cx, ty, src)?;
                Ok(T::linear_lift_list_from_memory(cx, &list)?.into())
            }

            fn linear_lift_from_memory(cx: &mut LiftContext<'_>, ty: InterfaceType, bytes: &[u8]) -> Result<Self> {
                let list = <WasmList::<T> as Lift>::linear_lift_from_memory(cx, ty, bytes)?;
                Ok(T::linear_lift_list_from_memory(cx, &list)?.into())
            }
        }
    )*)
}

forward_list_lifts! {
    Box<[T]>,
    alloc::sync::Arc<[T]>,
    Vec<T>,
}

// Macro to help generate `ComponentType` implementations for primitive types
// such as integers, char, bool, etc.
macro_rules! integers {
    ($($primitive:ident = $ty:ident in $field:ident/$get:ident with abi:$abi:ident,)*) => ($(
        unsafe impl ComponentType for $primitive {
            type Lower = ValRaw;

            const ABI: CanonicalAbiInfo = CanonicalAbiInfo::$abi;

            const MAY_REQUIRE_REALLOC: bool = false;

            fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
                match ty {
                    InterfaceType::$ty => Ok(()),
                    other => bail!("expected `{}` found `{}`", desc(&InterfaceType::$ty), desc(other))
                }
            }
        }

        unsafe impl Lower for $primitive {
            #[inline]
            #[allow(trivial_numeric_casts, reason = "macro-generated code")]
            fn linear_lower_to_flat<T>(
                &self,
                _cx: &mut LowerContext<'_, T>,
                ty: InterfaceType,
                dst: &mut MaybeUninit<Self::Lower>,
            ) -> Result<()> {
                debug_assert!(matches!(ty, InterfaceType::$ty));
                dst.write(ValRaw::$field(*self as $field));
                Ok(())
            }

            #[inline]
            fn linear_lower_to_memory<T>(
                &self,
                cx: &mut LowerContext<'_, T>,
                ty: InterfaceType,
                offset: usize,
            ) -> Result<()> {
                debug_assert!(matches!(ty, InterfaceType::$ty));
                debug_assert!(offset % Self::SIZE32 == 0);
                *cx.get(offset) = self.to_le_bytes();
                Ok(())
            }

            fn linear_store_list_to_memory<T>(
                cx: &mut LowerContext<'_, T>,
                ty: InterfaceType,
                offset: usize,
                items: &[Self],
            ) -> Result<()> {
                debug_assert!(matches!(ty, InterfaceType::$ty));

                // Double-check that the CM alignment is at least the host's
                // alignment for this type which should be true for all
                // platforms.
                assert!((Self::ALIGN32 as usize) >= mem::align_of::<Self>());

                // Slice `cx`'s memory to the window that we'll be modifying.
                // This should all have already been verified in terms of
                // alignment and sizing meaning that these assertions here are
                // not truly necessary but are instead double-checks.
                //
                // Note that we're casting a `[u8]` slice to `[Self]` with
                // `align_to_mut` which is not safe in general but is safe in
                // our specific case as all `u8` patterns are valid `Self`
                // patterns since `Self` is an integral type.
                let dst = &mut cx.as_slice_mut()[offset..][..items.len() * Self::SIZE32];
                let (before, middle, end) = unsafe { dst.align_to_mut::<Self>() };
                assert!(before.is_empty() && end.is_empty());
                assert_eq!(middle.len(), items.len());

                // And with all that out of the way perform the copying loop.
                // This is not a `copy_from_slice` because endianness needs to
                // be handled here, but LLVM should pretty easily transform this
                // into a memcpy on little-endian platforms.
                for (dst, src) in middle.iter_mut().zip(items) {
                    *dst = src.to_le();
                }
                Ok(())
            }
        }

        unsafe impl Lift for $primitive {
            #[inline]
            #[allow(
                trivial_numeric_casts,
                clippy::cast_possible_truncation,
                reason = "macro-generated code"
            )]
            fn linear_lift_from_flat(_cx: &mut LiftContext<'_>, ty: InterfaceType, src: &Self::Lower) -> Result<Self> {
                debug_assert!(matches!(ty, InterfaceType::$ty));
                Ok(src.$get() as $primitive)
            }

            #[inline]
            fn linear_lift_from_memory(_cx: &mut LiftContext<'_>, ty: InterfaceType, bytes: &[u8]) -> Result<Self> {
                debug_assert!(matches!(ty, InterfaceType::$ty));
                debug_assert!((bytes.as_ptr() as usize) % Self::SIZE32 == 0);
                Ok($primitive::from_le_bytes(bytes.try_into().unwrap()))
            }

            fn linear_lift_into_from_memory(
                cx: &mut LiftContext<'_>,
                list: &WasmList<Self>,
                dst: &mut impl Extend<Self>,
            ) -> Result<()>
            where
                Self: Sized,
            {
                dst.extend(list._as_le_slice(cx.memory())
                           .iter()
                           .map(|i| Self::from_le(*i)));
                Ok(())
            }
        }
    )*)
}

integers! {
    i8 = S8 in i32/get_i32 with abi:SCALAR1,
    u8 = U8 in u32/get_u32 with abi:SCALAR1,
    i16 = S16 in i32/get_i32 with abi:SCALAR2,
    u16 = U16 in u32/get_u32 with abi:SCALAR2,
    i32 = S32 in i32/get_i32 with abi:SCALAR4,
    u32 = U32 in u32/get_u32 with abi:SCALAR4,
    i64 = S64 in i64/get_i64 with abi:SCALAR8,
    u64 = U64 in u64/get_u64 with abi:SCALAR8,
}

macro_rules! floats {
    ($($float:ident/$get_float:ident = $ty:ident with abi:$abi:ident)*) => ($(const _: () = {
        unsafe impl ComponentType for $float {
            type Lower = ValRaw;

            const ABI: CanonicalAbiInfo = CanonicalAbiInfo::$abi;

            fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
                match ty {
                    InterfaceType::$ty => Ok(()),
                    other => bail!("expected `{}` found `{}`", desc(&InterfaceType::$ty), desc(other))
                }
            }
        }

        unsafe impl Lower for $float {
            #[inline]
            fn linear_lower_to_flat<T>(
                &self,
                _cx: &mut LowerContext<'_, T>,
                ty: InterfaceType,
                dst: &mut MaybeUninit<Self::Lower>,
            ) -> Result<()> {
                debug_assert!(matches!(ty, InterfaceType::$ty));
                dst.write(ValRaw::$float(self.to_bits()));
                Ok(())
            }

            #[inline]
            fn linear_lower_to_memory<T>(
                &self,
                cx: &mut LowerContext<'_, T>,
                ty: InterfaceType,
                offset: usize,
            ) -> Result<()> {
                debug_assert!(matches!(ty, InterfaceType::$ty));
                debug_assert!(offset % Self::SIZE32 == 0);
                let ptr = cx.get(offset);
                *ptr = self.to_bits().to_le_bytes();
                Ok(())
            }

            fn linear_store_list_to_memory<T>(
                cx: &mut LowerContext<'_, T>,
                ty: InterfaceType,
                offset: usize,
                items: &[Self],
            ) -> Result<()> {
                debug_assert!(matches!(ty, InterfaceType::$ty));

                // Double-check that the CM alignment is at least the host's
                // alignment for this type which should be true for all
                // platforms.
                assert!((Self::ALIGN32 as usize) >= mem::align_of::<Self>());

                // Slice `cx`'s memory to the window that we'll be modifying.
                // This should all have already been verified in terms of
                // alignment and sizing meaning that these assertions here are
                // not truly necessary but are instead double-checks.
                let dst = &mut cx.as_slice_mut()[offset..][..items.len() * Self::SIZE32];
                assert!(dst.as_ptr().cast::<Self>().is_aligned());

                // And with all that out of the way perform the copying loop.
                // This is not a `copy_from_slice` because endianness needs to
                // be handled here, but LLVM should pretty easily transform this
                // into a memcpy on little-endian platforms.
                // TODO use `as_chunks` when https://github.com/rust-lang/rust/issues/74985
                // is stabilized
                for (dst, src) in iter::zip(dst.chunks_exact_mut(Self::SIZE32), items) {
                    let dst: &mut [u8; Self::SIZE32] = dst.try_into().unwrap();
                    *dst = src.to_le_bytes();
                }
                Ok(())
            }
        }

        unsafe impl Lift for $float {
            #[inline]
            fn linear_lift_from_flat(_cx: &mut LiftContext<'_>, ty: InterfaceType, src: &Self::Lower) -> Result<Self> {
                debug_assert!(matches!(ty, InterfaceType::$ty));
                Ok($float::from_bits(src.$get_float()))
            }

            #[inline]
            fn linear_lift_from_memory(_cx: &mut LiftContext<'_>, ty: InterfaceType, bytes: &[u8]) -> Result<Self> {
                debug_assert!(matches!(ty, InterfaceType::$ty));
                debug_assert!((bytes.as_ptr() as usize) % Self::SIZE32 == 0);
                Ok($float::from_le_bytes(bytes.try_into().unwrap()))
            }

            fn linear_lift_list_from_memory(cx: &mut LiftContext<'_>, list: &WasmList<Self>) -> Result<Vec<Self>> where Self: Sized {
                // See comments in `WasmList::get` for the panicking indexing
                let byte_size = list.len * mem::size_of::<Self>();
                let bytes = &cx.memory()[list.ptr..][..byte_size];

                // The canonical ABI requires that everything is aligned to its
                // own size, so this should be an aligned array.
                assert!(bytes.as_ptr().cast::<Self>().is_aligned());

                // Copy the resulting slice to a new Vec, handling endianness
                // in the process
                // TODO use `as_chunks` when https://github.com/rust-lang/rust/issues/74985
                // is stabilized
                Ok(
                    bytes
                        .chunks_exact(Self::SIZE32)
                        .map(|i| $float::from_le_bytes(i.try_into().unwrap()))
                        .collect()
                )
            }
        }
    };)*)
}

floats! {
    f32/get_f32 = Float32 with abi:SCALAR4
    f64/get_f64 = Float64 with abi:SCALAR8
}

unsafe impl ComponentType for bool {
    type Lower = ValRaw;

    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::SCALAR1;

    fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::Bool => Ok(()),
            other => bail!("expected `bool` found `{}`", desc(other)),
        }
    }
}

unsafe impl Lower for bool {
    fn linear_lower_to_flat<T>(
        &self,
        _cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()> {
        debug_assert!(matches!(ty, InterfaceType::Bool));
        dst.write(ValRaw::i32(*self as i32));
        Ok(())
    }

    fn linear_lower_to_memory<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        debug_assert!(matches!(ty, InterfaceType::Bool));
        debug_assert!(offset % Self::SIZE32 == 0);
        cx.get::<1>(offset)[0] = *self as u8;
        Ok(())
    }
}

unsafe impl Lift for bool {
    #[inline]
    fn linear_lift_from_flat(
        _cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        debug_assert!(matches!(ty, InterfaceType::Bool));
        match src.get_i32() {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[inline]
    fn linear_lift_from_memory(
        _cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        debug_assert!(matches!(ty, InterfaceType::Bool));
        match bytes[0] {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
}

unsafe impl ComponentType for char {
    type Lower = ValRaw;

    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::SCALAR4;

    fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::Char => Ok(()),
            other => bail!("expected `char` found `{}`", desc(other)),
        }
    }
}

unsafe impl Lower for char {
    #[inline]
    fn linear_lower_to_flat<T>(
        &self,
        _cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()> {
        debug_assert!(matches!(ty, InterfaceType::Char));
        dst.write(ValRaw::u32(u32::from(*self)));
        Ok(())
    }

    #[inline]
    fn linear_lower_to_memory<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        debug_assert!(matches!(ty, InterfaceType::Char));
        debug_assert!(offset % Self::SIZE32 == 0);
        *cx.get::<4>(offset) = u32::from(*self).to_le_bytes();
        Ok(())
    }
}

unsafe impl Lift for char {
    #[inline]
    fn linear_lift_from_flat(
        _cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        debug_assert!(matches!(ty, InterfaceType::Char));
        Ok(char::try_from(src.get_u32())?)
    }

    #[inline]
    fn linear_lift_from_memory(
        _cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        debug_assert!(matches!(ty, InterfaceType::Char));
        debug_assert!((bytes.as_ptr() as usize) % Self::SIZE32 == 0);
        let bits = u32::from_le_bytes(bytes.try_into().unwrap());
        Ok(char::try_from(bits)?)
    }
}

// FIXME(#4311): these probably need different constants for memory64
const UTF16_TAG: usize = 1 << 31;
const MAX_STRING_BYTE_LENGTH: usize = (1 << 31) - 1;

// Note that this is similar to `ComponentType for WasmStr` except it can only
// be used for lowering, not lifting.
unsafe impl ComponentType for str {
    type Lower = [ValRaw; 2];

    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::POINTER_PAIR;

    fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::String => Ok(()),
            other => bail!("expected `string` found `{}`", desc(other)),
        }
    }
}

unsafe impl Lower for str {
    fn linear_lower_to_flat<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<[ValRaw; 2]>,
    ) -> Result<()> {
        debug_assert!(matches!(ty, InterfaceType::String));
        let (ptr, len) = lower_string(cx, self)?;
        // See "WRITEPTR64" above for why this is always storing a 64-bit
        // integer.
        map_maybe_uninit!(dst[0]).write(ValRaw::i64(ptr as i64));
        map_maybe_uninit!(dst[1]).write(ValRaw::i64(len as i64));
        Ok(())
    }

    fn linear_lower_to_memory<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        debug_assert!(matches!(ty, InterfaceType::String));
        debug_assert!(offset % (Self::ALIGN32 as usize) == 0);
        let (ptr, len) = lower_string(cx, self)?;
        // FIXME(#4311): needs memory64 handling
        *cx.get(offset + 0) = u32::try_from(ptr).unwrap().to_le_bytes();
        *cx.get(offset + 4) = u32::try_from(len).unwrap().to_le_bytes();
        Ok(())
    }
}

fn lower_string<T>(cx: &mut LowerContext<'_, T>, string: &str) -> Result<(usize, usize)> {
    // Note that in general the wasm module can't assume anything about what the
    // host strings are encoded as. Additionally hosts are allowed to have
    // differently-encoded strings at runtime. Finally when copying a string
    // into wasm it's somewhat strict in the sense that the various patterns of
    // allocation and such are already dictated for us.
    //
    // In general what this means is that when copying a string from the host
    // into the destination we need to follow one of the cases of copying into
    // WebAssembly. It doesn't particularly matter which case as long as it ends
    // up in the right encoding. For example a destination encoding of
    // latin1+utf16 has a number of ways to get copied into and we do something
    // here that isn't the default "utf8 to latin1+utf16" since we have access
    // to simd-accelerated helpers in the `encoding_rs` crate. This is ok though
    // because we can fake that the host string was already stored in latin1
    // format and follow that copy pattern instead.
    match cx.options.string_encoding() {
        // This corresponds to `store_string_copy` in the canonical ABI where
        // the host's representation is utf-8 and the wasm module wants utf-8 so
        // a copy is all that's needed (and the `realloc` can be precise for the
        // initial memory allocation).
        StringEncoding::Utf8 => {
            if string.len() > MAX_STRING_BYTE_LENGTH {
                bail!(
                    "string length of {} too large to copy into wasm",
                    string.len()
                );
            }
            let ptr = cx.realloc(0, 0, 1, string.len())?;
            cx.as_slice_mut()[ptr..][..string.len()].copy_from_slice(string.as_bytes());
            Ok((ptr, string.len()))
        }

        // This corresponds to `store_utf8_to_utf16` in the canonical ABI. Here
        // an over-large allocation is performed and then shrunk afterwards if
        // necessary.
        StringEncoding::Utf16 => {
            let size = string.len() * 2;
            if size > MAX_STRING_BYTE_LENGTH {
                bail!(
                    "string length of {} too large to copy into wasm",
                    string.len()
                );
            }
            let mut ptr = cx.realloc(0, 0, 2, size)?;
            let mut copied = 0;
            let bytes = &mut cx.as_slice_mut()[ptr..][..size];
            for (u, bytes) in string.encode_utf16().zip(bytes.chunks_mut(2)) {
                let u_bytes = u.to_le_bytes();
                bytes[0] = u_bytes[0];
                bytes[1] = u_bytes[1];
                copied += 1;
            }
            if (copied * 2) < size {
                ptr = cx.realloc(ptr, size, 2, copied * 2)?;
            }
            Ok((ptr, copied))
        }

        StringEncoding::CompactUtf16 => {
            // This corresponds to `store_string_to_latin1_or_utf16`
            let bytes = string.as_bytes();
            let mut iter = string.char_indices();
            let mut ptr = cx.realloc(0, 0, 2, bytes.len())?;
            let mut dst = &mut cx.as_slice_mut()[ptr..][..bytes.len()];
            let mut result = 0;
            while let Some((i, ch)) = iter.next() {
                // Test if this `char` fits into the latin1 encoding.
                if let Ok(byte) = u8::try_from(u32::from(ch)) {
                    dst[result] = byte;
                    result += 1;
                    continue;
                }

                // .. if utf16 is forced to be used then the allocation is
                // bumped up to the maximum size.
                let worst_case = bytes
                    .len()
                    .checked_mul(2)
                    .ok_or_else(|| anyhow!("byte length overflow"))?;
                if worst_case > MAX_STRING_BYTE_LENGTH {
                    bail!("byte length too large");
                }
                ptr = cx.realloc(ptr, bytes.len(), 2, worst_case)?;
                dst = &mut cx.as_slice_mut()[ptr..][..worst_case];

                // Previously encoded latin1 bytes are inflated to their 16-bit
                // size for utf16
                for i in (0..result).rev() {
                    dst[2 * i] = dst[i];
                    dst[2 * i + 1] = 0;
                }

                // and then the remainder of the string is encoded.
                for (u, bytes) in string[i..]
                    .encode_utf16()
                    .zip(dst[2 * result..].chunks_mut(2))
                {
                    let u_bytes = u.to_le_bytes();
                    bytes[0] = u_bytes[0];
                    bytes[1] = u_bytes[1];
                    result += 1;
                }
                if worst_case > 2 * result {
                    ptr = cx.realloc(ptr, worst_case, 2, 2 * result)?;
                }
                return Ok((ptr, result | UTF16_TAG));
            }
            if result < bytes.len() {
                ptr = cx.realloc(ptr, bytes.len(), 2, result)?;
            }
            Ok((ptr, result))
        }
    }
}

/// Representation of a string located in linear memory in a WebAssembly
/// instance.
///
/// This type can be used in place of `String` and `str` for string-taking APIs
/// in some situations. The purpose of this type is to represent a range of
/// validated bytes within a component but does not actually copy the bytes. The
/// primary method, [`WasmStr::to_str`], attempts to return a reference to the
/// string directly located in the component's memory, avoiding a copy into the
/// host if possible.
///
/// The downside of this type, however, is that accessing a string requires a
/// [`Store`](crate::Store) pointer (via [`StoreContext`]). Bindings generated
/// by [`bindgen!`](crate::component::bindgen), for example, do not have access
/// to [`StoreContext`] and thus can't use this type.
///
/// This is intended for more advanced use cases such as defining functions
/// directly in a [`Linker`](crate::component::Linker). It's expected that in
/// the future [`bindgen!`](crate::component::bindgen) will also have a way to
/// use this type.
///
/// This type is used with [`TypedFunc`], for example, when WebAssembly returns
/// a string. This type cannot be used to give a string to WebAssembly, instead
/// `&str` should be used for that (since it's coming from the host).
///
/// Note that this type represents an in-bounds string in linear memory, but it
/// does not represent a valid string (e.g. valid utf-8). Validation happens
/// when [`WasmStr::to_str`] is called.
///
/// Also note that this type does not implement [`Lower`], it only implements
/// [`Lift`].
pub struct WasmStr {
    ptr: usize,
    len: usize,
    options: Options,
}

impl WasmStr {
    pub(crate) fn new(ptr: usize, len: usize, cx: &mut LiftContext<'_>) -> Result<WasmStr> {
        let byte_len = match cx.options.string_encoding() {
            StringEncoding::Utf8 => Some(len),
            StringEncoding::Utf16 => len.checked_mul(2),
            StringEncoding::CompactUtf16 => {
                if len & UTF16_TAG == 0 {
                    Some(len)
                } else {
                    (len ^ UTF16_TAG).checked_mul(2)
                }
            }
        };
        match byte_len.and_then(|len| ptr.checked_add(len)) {
            Some(n) if n <= cx.memory().len() => cx.consume_fuel(n - ptr)?,
            _ => bail!("string pointer/length out of bounds of memory"),
        }
        Ok(WasmStr {
            ptr,
            len,
            options: *cx.options,
        })
    }

    /// Returns the underlying string that this cursor points to.
    ///
    /// Note that this will internally decode the string from the wasm's
    /// encoding to utf-8 and additionally perform validation.
    ///
    /// The `store` provided must be the store where this string lives to
    /// access the correct memory.
    ///
    /// # Errors
    ///
    /// Returns an error if the string wasn't encoded correctly (e.g. invalid
    /// utf-8).
    ///
    /// # Panics
    ///
    /// Panics if this string is not owned by `store`.
    //
    // TODO: should add accessors for specifically utf-8 and utf-16 that perhaps
    // in an opt-in basis don't do validation. Additionally there should be some
    // method that returns `[u16]` after validating to avoid the utf16-to-utf8
    // transcode.
    pub fn to_str<'a, T: 'static>(
        &self,
        store: impl Into<StoreContext<'a, T>>,
    ) -> Result<Cow<'a, str>> {
        let store = store.into().0;
        let memory = self.options.memory(store);
        self.to_str_from_memory(memory)
    }

    pub(crate) fn to_str_from_memory<'a>(&self, memory: &'a [u8]) -> Result<Cow<'a, str>> {
        match self.options.string_encoding() {
            StringEncoding::Utf8 => self.decode_utf8(memory),
            StringEncoding::Utf16 => self.decode_utf16(memory, self.len),
            StringEncoding::CompactUtf16 => {
                if self.len & UTF16_TAG == 0 {
                    self.decode_latin1(memory)
                } else {
                    self.decode_utf16(memory, self.len ^ UTF16_TAG)
                }
            }
        }
    }

    fn decode_utf8<'a>(&self, memory: &'a [u8]) -> Result<Cow<'a, str>> {
        // Note that bounds-checking already happen in construction of `WasmStr`
        // so this is never expected to panic. This could theoretically be
        // unchecked indexing if we're feeling wild enough.
        Ok(str::from_utf8(&memory[self.ptr..][..self.len])?.into())
    }

    fn decode_utf16<'a>(&self, memory: &'a [u8], len: usize) -> Result<Cow<'a, str>> {
        // See notes in `decode_utf8` for why this is panicking indexing.
        let memory = &memory[self.ptr..][..len * 2];
        Ok(core::char::decode_utf16(
            memory
                .chunks(2)
                .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap())),
        )
        .collect::<Result<String, _>>()?
        .into())
    }

    fn decode_latin1<'a>(&self, memory: &'a [u8]) -> Result<Cow<'a, str>> {
        // See notes in `decode_utf8` for why this is panicking indexing.
        Ok(encoding_rs::mem::decode_latin1(
            &memory[self.ptr..][..self.len],
        ))
    }
}

// Note that this is similar to `ComponentType for str` except it can only be
// used for lifting, not lowering.
unsafe impl ComponentType for WasmStr {
    type Lower = <str as ComponentType>::Lower;

    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::POINTER_PAIR;

    fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::String => Ok(()),
            other => bail!("expected `string` found `{}`", desc(other)),
        }
    }
}

unsafe impl Lift for WasmStr {
    #[inline]
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        debug_assert!(matches!(ty, InterfaceType::String));
        // FIXME(#4311): needs memory64 treatment
        let ptr = src[0].get_u32();
        let len = src[1].get_u32();
        let (ptr, len) = (usize::try_from(ptr)?, usize::try_from(len)?);
        WasmStr::new(ptr, len, cx)
    }

    #[inline]
    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        debug_assert!(matches!(ty, InterfaceType::String));
        debug_assert!((bytes.as_ptr() as usize) % (Self::ALIGN32 as usize) == 0);
        // FIXME(#4311): needs memory64 treatment
        let ptr = u32::from_le_bytes(bytes[..4].try_into().unwrap());
        let len = u32::from_le_bytes(bytes[4..].try_into().unwrap());
        let (ptr, len) = (usize::try_from(ptr)?, usize::try_from(len)?);
        WasmStr::new(ptr, len, cx)
    }
}

unsafe impl<T> ComponentType for [T]
where
    T: ComponentType,
{
    type Lower = [ValRaw; 2];

    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::POINTER_PAIR;

    fn typecheck(ty: &InterfaceType, types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::List(t) => T::typecheck(&types.types[*t].element, types),
            other => bail!("expected `list` found `{}`", desc(other)),
        }
    }
}

unsafe impl<T> Lower for [T]
where
    T: Lower,
{
    fn linear_lower_to_flat<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<[ValRaw; 2]>,
    ) -> Result<()> {
        let elem = match ty {
            InterfaceType::List(i) => cx.types[i].element,
            _ => bad_type_info(),
        };
        let (ptr, len) = lower_list(cx, elem, self)?;
        // See "WRITEPTR64" above for why this is always storing a 64-bit
        // integer.
        map_maybe_uninit!(dst[0]).write(ValRaw::i64(ptr as i64));
        map_maybe_uninit!(dst[1]).write(ValRaw::i64(len as i64));
        Ok(())
    }

    fn linear_lower_to_memory<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        let elem = match ty {
            InterfaceType::List(i) => cx.types[i].element,
            _ => bad_type_info(),
        };
        debug_assert!(offset % (Self::ALIGN32 as usize) == 0);
        let (ptr, len) = lower_list(cx, elem, self)?;
        *cx.get(offset + 0) = u32::try_from(ptr).unwrap().to_le_bytes();
        *cx.get(offset + 4) = u32::try_from(len).unwrap().to_le_bytes();
        Ok(())
    }
}

// FIXME: this is not a memcpy for `T` where `T` is something like `u8`.
//
// Some attempts to fix this have proved not fruitful. In isolation an attempt
// was made where:
//
// * `MemoryMut` stored a `*mut [u8]` as its "last view" of memory to avoid
//   reloading the base pointer constantly. This view is reset on `realloc`.
// * The bounds-checks in `MemoryMut::get` were removed (replaced with unsafe
//   indexing)
//
// Even then though this didn't correctly vectorized for `Vec<u8>`. It's not
// entirely clear why but it appeared that it's related to reloading the base
// pointer to memory (I guess from `MemoryMut` itself?). Overall I'm not really
// clear on what's happening there, but this is surely going to be a performance
// bottleneck in the future.
fn lower_list<T, U>(
    cx: &mut LowerContext<'_, U>,
    ty: InterfaceType,
    list: &[T],
) -> Result<(usize, usize)>
where
    T: Lower,
{
    let elem_size = T::SIZE32;
    let size = list
        .len()
        .checked_mul(elem_size)
        .ok_or_else(|| anyhow!("size overflow copying a list"))?;
    let ptr = cx.realloc(0, 0, T::ALIGN32, size)?;
    T::linear_store_list_to_memory(cx, ty, ptr, list)?;
    Ok((ptr, list.len()))
}

/// Representation of a list of values that are owned by a WebAssembly instance.
///
/// For some more commentary about the rationale for this type see the
/// documentation of [`WasmStr`]. In summary this type can avoid a copy when
/// passing data to the host in some situations but is additionally more
/// cumbersome to use by requiring a [`Store`](crate::Store) to be provided.
///
/// This type is used whenever a `(list T)` is returned from a [`TypedFunc`],
/// for example. This type represents a list of values that are stored in linear
/// memory which are waiting to be read.
///
/// Note that this type represents only a valid range of bytes for the list
/// itself, it does not represent validity of the elements themselves and that's
/// performed when they're iterated.
///
/// Note that this type does not implement the [`Lower`] trait, only [`Lift`].
pub struct WasmList<T> {
    ptr: usize,
    len: usize,
    options: Options,
    elem: InterfaceType,
    instance: Instance,
    _marker: marker::PhantomData<T>,
}

impl<T: Lift> WasmList<T> {
    pub(crate) fn new(
        ptr: usize,
        len: usize,
        cx: &mut LiftContext<'_>,
        elem: InterfaceType,
    ) -> Result<WasmList<T>> {
        match len
            .checked_mul(T::SIZE32)
            .and_then(|len| ptr.checked_add(len))
        {
            Some(n) if n <= cx.memory().len() => cx.consume_fuel(n - ptr)?,
            _ => bail!("list pointer/length out of bounds of memory"),
        }
        if ptr % usize::try_from(T::ALIGN32)? != 0 {
            bail!("list pointer is not aligned")
        }
        Ok(WasmList {
            ptr,
            len,
            options: *cx.options,
            elem,
            instance: cx.instance_handle(),
            _marker: marker::PhantomData,
        })
    }

    /// Returns the item length of this vector
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Gets the `n`th element of this list.
    ///
    /// Returns `None` if `index` is out of bounds. Returns `Some(Err(..))` if
    /// the value couldn't be decoded (it was invalid). Returns `Some(Ok(..))`
    /// if the value is valid.
    ///
    /// # Panics
    ///
    /// This function will panic if the string did not originally come from the
    /// `store` specified.
    //
    // TODO: given that interface values are intended to be consumed in one go
    // should we even expose a random access iteration API? In theory all
    // consumers should be validating through the iterator.
    pub fn get(&self, mut store: impl AsContextMut, index: usize) -> Option<Result<T>> {
        let store = store.as_context_mut().0;
        self.options.store_id().assert_belongs_to(store.id());
        let mut cx = LiftContext::new(store, &self.options, self.instance);
        self.get_from_store(&mut cx, index)
    }

    fn get_from_store(&self, cx: &mut LiftContext<'_>, index: usize) -> Option<Result<T>> {
        if index >= self.len {
            return None;
        }
        // Note that this is using panicking indexing and this is expected to
        // never fail. The bounds-checking here happened during the construction
        // of the `WasmList` itself which means these should always be in-bounds
        // (and wasm memory can only grow). This could theoretically be
        // unchecked indexing if we're confident enough and it's actually a perf
        // issue one day.
        let bytes = &cx.memory()[self.ptr + index * T::SIZE32..][..T::SIZE32];
        Some(T::linear_lift_from_memory(cx, self.elem, bytes))
    }

    /// Returns an iterator over the elements of this list.
    ///
    /// Each item of the list may fail to decode and is represented through the
    /// `Result` value of the iterator.
    pub fn iter<'a, U: 'static>(
        &'a self,
        store: impl Into<StoreContextMut<'a, U>>,
    ) -> impl ExactSizeIterator<Item = Result<T>> + 'a {
        let store = store.into().0;
        self.options.store_id().assert_belongs_to(store.id());
        let mut cx = LiftContext::new(store, &self.options, self.instance);
        (0..self.len).map(move |i| self.get_from_store(&mut cx, i).unwrap())
    }
}

macro_rules! raw_wasm_list_accessors {
    ($($i:ident)*) => ($(
        impl WasmList<$i> {
            /// Get access to the raw underlying memory for this list.
            ///
            /// This method will return a direct slice into the original wasm
            /// module's linear memory where the data for this slice is stored.
            /// This allows the embedder to have efficient access to the
            /// underlying memory if needed and avoid copies and such if
            /// desired.
            ///
            /// Note that multi-byte integers are stored in little-endian format
            /// so portable processing of this slice must be aware of the host's
            /// byte-endianness. The `from_le` constructors in the Rust standard
            /// library should be suitable for converting from little-endian.
            ///
            /// # Panics
            ///
            /// Panics if the `store` provided is not the one from which this
            /// slice originated.
            pub fn as_le_slice<'a, T: 'static>(&self, store: impl Into<StoreContext<'a, T>>) -> &'a [$i] {
                let memory = self.options.memory(store.into().0);
                self._as_le_slice(memory)
            }

            fn _as_le_slice<'a>(&self, all_of_memory: &'a [u8]) -> &'a [$i] {
                // See comments in `WasmList::get` for the panicking indexing
                let byte_size = self.len * mem::size_of::<$i>();
                let bytes = &all_of_memory[self.ptr..][..byte_size];

                // The canonical ABI requires that everything is aligned to its
                // own size, so this should be an aligned array. Furthermore the
                // alignment of primitive integers for hosts should be smaller
                // than or equal to the size of the primitive itself, meaning
                // that a wasm canonical-abi-aligned list is also aligned for
                // the host. That should mean that the head/tail slices here are
                // empty.
                //
                // Also note that the `unsafe` here is needed since the type
                // we're aligning to isn't guaranteed to be valid, but in our
                // case it's just integers and bytes so this should be safe.
                unsafe {
                    let (head, body, tail) = bytes.align_to::<$i>();
                    assert!(head.is_empty() && tail.is_empty());
                    body
                }
            }
        }
    )*)
}

raw_wasm_list_accessors! {
    i8 i16 i32 i64
    u8 u16 u32 u64
}

// Note that this is similar to `ComponentType for str` except it can only be
// used for lifting, not lowering.
unsafe impl<T: ComponentType> ComponentType for WasmList<T> {
    type Lower = <[T] as ComponentType>::Lower;

    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::POINTER_PAIR;

    fn typecheck(ty: &InterfaceType, types: &InstanceType<'_>) -> Result<()> {
        <[T] as ComponentType>::typecheck(ty, types)
    }
}

unsafe impl<T: Lift> Lift for WasmList<T> {
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        let elem = match ty {
            InterfaceType::List(i) => cx.types[i].element,
            _ => bad_type_info(),
        };
        // FIXME(#4311): needs memory64 treatment
        let ptr = src[0].get_u32();
        let len = src[1].get_u32();
        let (ptr, len) = (usize::try_from(ptr)?, usize::try_from(len)?);
        WasmList::new(ptr, len, cx, elem)
    }

    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        let elem = match ty {
            InterfaceType::List(i) => cx.types[i].element,
            _ => bad_type_info(),
        };
        debug_assert!((bytes.as_ptr() as usize) % (Self::ALIGN32 as usize) == 0);
        // FIXME(#4311): needs memory64 treatment
        let ptr = u32::from_le_bytes(bytes[..4].try_into().unwrap());
        let len = u32::from_le_bytes(bytes[4..].try_into().unwrap());
        let (ptr, len) = (usize::try_from(ptr)?, usize::try_from(len)?);
        WasmList::new(ptr, len, cx, elem)
    }
}

/// Verify that the given wasm type is a tuple with the expected fields in the right order.
fn typecheck_tuple(
    ty: &InterfaceType,
    types: &InstanceType<'_>,
    expected: &[fn(&InterfaceType, &InstanceType<'_>) -> Result<()>],
) -> Result<()> {
    match ty {
        InterfaceType::Tuple(t) => {
            let tuple = &types.types[*t];
            if tuple.types.len() != expected.len() {
                bail!(
                    "expected {}-tuple, found {}-tuple",
                    expected.len(),
                    tuple.types.len()
                );
            }
            for (ty, check) in tuple.types.iter().zip(expected) {
                check(ty, types)?;
            }
            Ok(())
        }
        other => bail!("expected `tuple` found `{}`", desc(other)),
    }
}

/// Verify that the given wasm type is a record with the expected fields in the right order and with the right
/// names.
pub fn typecheck_record(
    ty: &InterfaceType,
    types: &InstanceType<'_>,
    expected: &[(&str, fn(&InterfaceType, &InstanceType<'_>) -> Result<()>)],
) -> Result<()> {
    match ty {
        InterfaceType::Record(index) => {
            let fields = &types.types[*index].fields;

            if fields.len() != expected.len() {
                bail!(
                    "expected record of {} fields, found {} fields",
                    expected.len(),
                    fields.len()
                );
            }

            for (field, &(name, check)) in fields.iter().zip(expected) {
                check(&field.ty, types)
                    .with_context(|| format!("type mismatch for field {name}"))?;

                if field.name != name {
                    bail!("expected record field named {}, found {}", name, field.name);
                }
            }

            Ok(())
        }
        other => bail!("expected `record` found `{}`", desc(other)),
    }
}

/// Verify that the given wasm type is a variant with the expected cases in the right order and with the right
/// names.
pub fn typecheck_variant(
    ty: &InterfaceType,
    types: &InstanceType<'_>,
    expected: &[(
        &str,
        Option<fn(&InterfaceType, &InstanceType<'_>) -> Result<()>>,
    )],
) -> Result<()> {
    match ty {
        InterfaceType::Variant(index) => {
            let cases = &types.types[*index].cases;

            if cases.len() != expected.len() {
                bail!(
                    "expected variant of {} cases, found {} cases",
                    expected.len(),
                    cases.len()
                );
            }

            for ((case_name, case_ty), &(name, check)) in cases.iter().zip(expected) {
                if *case_name != name {
                    bail!("expected variant case named {name}, found {case_name}");
                }

                match (check, case_ty) {
                    (Some(check), Some(ty)) => check(ty, types)
                        .with_context(|| format!("type mismatch for case {name}"))?,
                    (None, None) => {}
                    (Some(_), None) => {
                        bail!("case `{name}` has no type but one was expected")
                    }
                    (None, Some(_)) => {
                        bail!("case `{name}` has a type but none was expected")
                    }
                }
            }

            Ok(())
        }
        other => bail!("expected `variant` found `{}`", desc(other)),
    }
}

/// Verify that the given wasm type is a enum with the expected cases in the right order and with the right
/// names.
pub fn typecheck_enum(
    ty: &InterfaceType,
    types: &InstanceType<'_>,
    expected: &[&str],
) -> Result<()> {
    match ty {
        InterfaceType::Enum(index) => {
            let names = &types.types[*index].names;

            if names.len() != expected.len() {
                bail!(
                    "expected enum of {} names, found {} names",
                    expected.len(),
                    names.len()
                );
            }

            for (name, expected) in names.iter().zip(expected) {
                if name != expected {
                    bail!("expected enum case named {}, found {}", expected, name);
                }
            }

            Ok(())
        }
        other => bail!("expected `enum` found `{}`", desc(other)),
    }
}

/// Verify that the given wasm type is a flags type with the expected flags in the right order and with the right
/// names.
pub fn typecheck_flags(
    ty: &InterfaceType,
    types: &InstanceType<'_>,
    expected: &[&str],
) -> Result<()> {
    match ty {
        InterfaceType::Flags(index) => {
            let names = &types.types[*index].names;

            if names.len() != expected.len() {
                bail!(
                    "expected flags type with {} names, found {} names",
                    expected.len(),
                    names.len()
                );
            }

            for (name, expected) in names.iter().zip(expected) {
                if name != expected {
                    bail!("expected flag named {}, found {}", expected, name);
                }
            }

            Ok(())
        }
        other => bail!("expected `flags` found `{}`", desc(other)),
    }
}

/// Format the specified bitflags using the specified names for debugging
pub fn format_flags(bits: &[u32], names: &[&str], f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("(")?;
    let mut wrote = false;
    for (index, name) in names.iter().enumerate() {
        if ((bits[index / 32] >> (index % 32)) & 1) != 0 {
            if wrote {
                f.write_str("|")?;
            } else {
                wrote = true;
            }

            f.write_str(name)?;
        }
    }
    f.write_str(")")
}

unsafe impl<T> ComponentType for Option<T>
where
    T: ComponentType,
{
    type Lower = TupleLower<<u32 as ComponentType>::Lower, T::Lower>;

    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::variant_static(&[None, Some(T::ABI)]);

    fn typecheck(ty: &InterfaceType, types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::Option(t) => T::typecheck(&types.types[*t].ty, types),
            other => bail!("expected `option` found `{}`", desc(other)),
        }
    }
}

unsafe impl<T> ComponentVariant for Option<T>
where
    T: ComponentType,
{
    const CASES: &'static [Option<CanonicalAbiInfo>] = &[None, Some(T::ABI)];
}

unsafe impl<T> Lower for Option<T>
where
    T: Lower,
{
    fn linear_lower_to_flat<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()> {
        let payload = match ty {
            InterfaceType::Option(ty) => cx.types[ty].ty,
            _ => bad_type_info(),
        };
        match self {
            None => {
                map_maybe_uninit!(dst.A1).write(ValRaw::i32(0));
                // Note that this is unsafe as we're writing an arbitrary
                // bit-pattern to an arbitrary type, but part of the unsafe
                // contract of the `ComponentType` trait is that we can assign
                // any bit-pattern. By writing all zeros here we're ensuring
                // that the core wasm arguments this translates to will all be
                // zeros (as the canonical ABI requires).
                unsafe {
                    map_maybe_uninit!(dst.A2).as_mut_ptr().write_bytes(0u8, 1);
                }
            }
            Some(val) => {
                map_maybe_uninit!(dst.A1).write(ValRaw::i32(1));
                val.linear_lower_to_flat(cx, payload, map_maybe_uninit!(dst.A2))?;
            }
        }
        Ok(())
    }

    fn linear_lower_to_memory<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        debug_assert!(offset % (Self::ALIGN32 as usize) == 0);
        let payload = match ty {
            InterfaceType::Option(ty) => cx.types[ty].ty,
            _ => bad_type_info(),
        };
        match self {
            None => {
                cx.get::<1>(offset)[0] = 0;
            }
            Some(val) => {
                cx.get::<1>(offset)[0] = 1;
                val.linear_lower_to_memory(
                    cx,
                    payload,
                    offset + (Self::INFO.payload_offset32 as usize),
                )?;
            }
        }
        Ok(())
    }
}

unsafe impl<T> Lift for Option<T>
where
    T: Lift,
{
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        let payload = match ty {
            InterfaceType::Option(ty) => cx.types[ty].ty,
            _ => bad_type_info(),
        };
        Ok(match src.A1.get_i32() {
            0 => None,
            1 => Some(T::linear_lift_from_flat(cx, payload, &src.A2)?),
            _ => bail!("invalid option discriminant"),
        })
    }

    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        debug_assert!((bytes.as_ptr() as usize) % (Self::ALIGN32 as usize) == 0);
        let payload_ty = match ty {
            InterfaceType::Option(ty) => cx.types[ty].ty,
            _ => bad_type_info(),
        };
        let discrim = bytes[0];
        let payload = &bytes[Self::INFO.payload_offset32 as usize..];
        match discrim {
            0 => Ok(None),
            1 => Ok(Some(T::linear_lift_from_memory(cx, payload_ty, payload)?)),
            _ => bail!("invalid option discriminant"),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ResultLower<T: Copy, E: Copy> {
    tag: ValRaw,
    payload: ResultLowerPayload<T, E>,
}

#[derive(Clone, Copy)]
#[repr(C)]
union ResultLowerPayload<T: Copy, E: Copy> {
    ok: T,
    err: E,
}

unsafe impl<T, E> ComponentType for Result<T, E>
where
    T: ComponentType,
    E: ComponentType,
{
    type Lower = ResultLower<T::Lower, E::Lower>;

    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::variant_static(&[Some(T::ABI), Some(E::ABI)]);

    fn typecheck(ty: &InterfaceType, types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::Result(r) => {
                let result = &types.types[*r];
                match &result.ok {
                    Some(ty) => T::typecheck(ty, types)?,
                    None if T::IS_RUST_UNIT_TYPE => {}
                    None => bail!("expected no `ok` type"),
                }
                match &result.err {
                    Some(ty) => E::typecheck(ty, types)?,
                    None if E::IS_RUST_UNIT_TYPE => {}
                    None => bail!("expected no `err` type"),
                }
                Ok(())
            }
            other => bail!("expected `result` found `{}`", desc(other)),
        }
    }
}

/// Lowers the payload of a variant into the storage for the entire payload,
/// handling writing zeros at the end of the representation if this payload is
/// smaller than the entire flat representation.
///
/// * `payload` - the flat storage space for the entire payload of the variant
/// * `typed_payload` - projection from the payload storage space to the
///   individual storage space for this variant.
/// * `lower` - lowering operation used to initialize the `typed_payload` return
///   value.
///
/// For more information on this se the comments in the `Lower for Result`
/// implementation below.
pub unsafe fn lower_payload<P, T>(
    payload: &mut MaybeUninit<P>,
    typed_payload: impl FnOnce(&mut MaybeUninit<P>) -> &mut MaybeUninit<T>,
    lower: impl FnOnce(&mut MaybeUninit<T>) -> Result<()>,
) -> Result<()> {
    let typed = typed_payload(payload);
    lower(typed)?;

    let typed_len = unsafe { storage_as_slice(typed).len() };
    let payload = unsafe { storage_as_slice_mut(payload) };
    for slot in payload[typed_len..].iter_mut() {
        slot.write(ValRaw::u64(0));
    }
    Ok(())
}

unsafe impl<T, E> ComponentVariant for Result<T, E>
where
    T: ComponentType,
    E: ComponentType,
{
    const CASES: &'static [Option<CanonicalAbiInfo>] = &[Some(T::ABI), Some(E::ABI)];
}

unsafe impl<T, E> Lower for Result<T, E>
where
    T: Lower,
    E: Lower,
{
    fn linear_lower_to_flat<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()> {
        let (ok, err) = match ty {
            InterfaceType::Result(ty) => {
                let ty = &cx.types[ty];
                (ty.ok, ty.err)
            }
            _ => bad_type_info(),
        };

        // This implementation of `Lower::lower`, if you're reading these from
        // the top of this file, is the first location that the "join" logic of
        // the component model's canonical ABI encountered. The rough problem is
        // that let's say we have a component model type of the form:
        //
        //      (result u64 (error (tuple f32 u16)))
        //
        // The flat representation of this is actually pretty tricky. Currently
        // it is:
        //
        //      i32 i64 i32
        //
        // The first `i32` is the discriminant for the `result`, and the payload
        // is represented by `i64 i32`. The "ok" variant will only use the `i64`
        // and the "err" variant will use both `i64` and `i32`.
        //
        // In the "ok" variant the first issue is encountered. The size of one
        // variant may not match the size of the other variants. All variants
        // start at the "front" but when lowering a type we need to be sure to
        // initialize the later variants (lest we leak random host memory into
        // the guest module). Due to how the `Lower` type is represented as a
        // `union` of all the variants what ends up happening here is that
        // internally within the `lower_payload` after the typed payload is
        // lowered the remaining bits of the payload that weren't initialized
        // are all set to zero. This will guarantee that we'll write to all the
        // slots for each variant.
        //
        // The "err" variant encounters the second issue, however, which is that
        // the flat representation for each type may differ between payloads. In
        // the "ok" arm an `i64` is written, but the `lower` implementation for
        // the "err" arm will write an `f32` and then an `i32`. For this
        // implementation of `lower` to be valid the `f32` needs to get inflated
        // to an `i64` with zero-padding in the upper bits. What may be
        // surprising, however, is that none of this is handled in this file.
        // This implementation looks like it's blindly deferring to `E::lower`
        // and hoping it does the right thing.
        //
        // In reality, however, the correctness of variant lowering relies on
        // two subtle details of the `ValRaw` implementation in Wasmtime:
        //
        // 1. First the `ValRaw` value always contains little-endian values.
        //    This means that if a `u32` is written, a `u64` is read, and then
        //    the `u64` has its upper bits truncated the original value will
        //    always be retained. This is primarily here for big-endian
        //    platforms where if it weren't little endian then the opposite
        //    would occur and the wrong value would be read.
        //
        // 2. Second, and perhaps even more subtly, the `ValRaw` constructors
        //    for 32-bit types actually always initialize 64-bits of the
        //    `ValRaw`. In the component model flat ABI only 32 and 64-bit types
        //    are used so 64-bits is big enough to contain everything. This
        //    means that when a `ValRaw` is written into the destination it will
        //    always, whether it's needed or not, be "ready" to get extended up
        //    to 64-bits.
        //
        // Put together these two subtle guarantees means that all `Lower`
        // implementations can be written "naturally" as one might naively
        // expect. Variants will, on each arm, zero out remaining fields and all
        // writes to the flat representation will automatically be 64-bit writes
        // meaning that if the value is read as a 64-bit value, which isn't
        // known at the time of the write, it'll still be correct.
        match self {
            Ok(e) => {
                map_maybe_uninit!(dst.tag).write(ValRaw::i32(0));
                unsafe {
                    lower_payload(
                        map_maybe_uninit!(dst.payload),
                        |payload| map_maybe_uninit!(payload.ok),
                        |dst| match ok {
                            Some(ok) => e.linear_lower_to_flat(cx, ok, dst),
                            None => Ok(()),
                        },
                    )
                }
            }
            Err(e) => {
                map_maybe_uninit!(dst.tag).write(ValRaw::i32(1));
                unsafe {
                    lower_payload(
                        map_maybe_uninit!(dst.payload),
                        |payload| map_maybe_uninit!(payload.err),
                        |dst| match err {
                            Some(err) => e.linear_lower_to_flat(cx, err, dst),
                            None => Ok(()),
                        },
                    )
                }
            }
        }
    }

    fn linear_lower_to_memory<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        let (ok, err) = match ty {
            InterfaceType::Result(ty) => {
                let ty = &cx.types[ty];
                (ty.ok, ty.err)
            }
            _ => bad_type_info(),
        };
        debug_assert!(offset % (Self::ALIGN32 as usize) == 0);
        let payload_offset = Self::INFO.payload_offset32 as usize;
        match self {
            Ok(e) => {
                cx.get::<1>(offset)[0] = 0;
                if let Some(ok) = ok {
                    e.linear_lower_to_memory(cx, ok, offset + payload_offset)?;
                }
            }
            Err(e) => {
                cx.get::<1>(offset)[0] = 1;
                if let Some(err) = err {
                    e.linear_lower_to_memory(cx, err, offset + payload_offset)?;
                }
            }
        }
        Ok(())
    }
}

unsafe impl<T, E> Lift for Result<T, E>
where
    T: Lift,
    E: Lift,
{
    #[inline]
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        let (ok, err) = match ty {
            InterfaceType::Result(ty) => {
                let ty = &cx.types[ty];
                (ty.ok, ty.err)
            }
            _ => bad_type_info(),
        };
        // Note that this implementation specifically isn't trying to actually
        // reinterpret or alter the bits of `lower` depending on which variant
        // we're lifting. This ends up all working out because the value is
        // stored in little-endian format.
        //
        // When stored in little-endian format the `{T,E}::Lower`, when each
        // individual `ValRaw` is read, means that if an i64 value, extended
        // from an i32 value, was stored then when the i32 value is read it'll
        // automatically ignore the upper bits.
        //
        // This "trick" allows us to seamlessly pass through the `Self::Lower`
        // representation into the lifting/lowering without trying to handle
        // "join"ed types as per the canonical ABI. It just so happens that i64
        // bits will naturally be reinterpreted as f64. Additionally if the
        // joined type is i64 but only the lower bits are read that's ok and we
        // don't need to validate the upper bits.
        //
        // This is largely enabled by WebAssembly/component-model#35 where no
        // validation needs to be performed for ignored bits and bytes here.
        Ok(match src.tag.get_i32() {
            0 => Ok(unsafe { lift_option(cx, ok, &src.payload.ok)? }),
            1 => Err(unsafe { lift_option(cx, err, &src.payload.err)? }),
            _ => bail!("invalid expected discriminant"),
        })
    }

    #[inline]
    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        debug_assert!((bytes.as_ptr() as usize) % (Self::ALIGN32 as usize) == 0);
        let discrim = bytes[0];
        let payload = &bytes[Self::INFO.payload_offset32 as usize..];
        let (ok, err) = match ty {
            InterfaceType::Result(ty) => {
                let ty = &cx.types[ty];
                (ty.ok, ty.err)
            }
            _ => bad_type_info(),
        };
        match discrim {
            0 => Ok(Ok(load_option(cx, ok, &payload[..T::SIZE32])?)),
            1 => Ok(Err(load_option(cx, err, &payload[..E::SIZE32])?)),
            _ => bail!("invalid expected discriminant"),
        }
    }
}

fn lift_option<T>(cx: &mut LiftContext<'_>, ty: Option<InterfaceType>, src: &T::Lower) -> Result<T>
where
    T: Lift,
{
    match ty {
        Some(ty) => T::linear_lift_from_flat(cx, ty, src),
        None => Ok(empty_lift()),
    }
}

fn load_option<T>(cx: &mut LiftContext<'_>, ty: Option<InterfaceType>, bytes: &[u8]) -> Result<T>
where
    T: Lift,
{
    match ty {
        Some(ty) => T::linear_lift_from_memory(cx, ty, bytes),
        None => Ok(empty_lift()),
    }
}

fn empty_lift<T>() -> T
where
    T: Lift,
{
    assert!(T::IS_RUST_UNIT_TYPE);
    assert_eq!(mem::size_of::<T>(), 0);
    unsafe { MaybeUninit::uninit().assume_init() }
}

/// Helper structure to define `Lower` for tuples below.
///
/// Uses default type parameters to have fields be zero-sized and not present
/// in memory for smaller tuple values.
#[expect(non_snake_case, reason = "more amenable to macro-generated code")]
#[doc(hidden)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TupleLower<
    T1 = (),
    T2 = (),
    T3 = (),
    T4 = (),
    T5 = (),
    T6 = (),
    T7 = (),
    T8 = (),
    T9 = (),
    T10 = (),
    T11 = (),
    T12 = (),
    T13 = (),
    T14 = (),
    T15 = (),
    T16 = (),
    T17 = (),
> {
    // NB: these names match the names in `for_each_function_signature!`
    A1: T1,
    A2: T2,
    A3: T3,
    A4: T4,
    A5: T5,
    A6: T6,
    A7: T7,
    A8: T8,
    A9: T9,
    A10: T10,
    A11: T11,
    A12: T12,
    A13: T13,
    A14: T14,
    A15: T15,
    A16: T16,
    A17: T17,
    _align_tuple_lower0_correctly: [ValRaw; 0],
}

macro_rules! impl_component_ty_for_tuples {
    ($n:tt $($t:ident)*) => {
        #[allow(non_snake_case, reason = "macro-generated code")]
        unsafe impl<$($t,)*> ComponentType for ($($t,)*)
            where $($t: ComponentType),*
        {
            type Lower = TupleLower<$($t::Lower),*>;

            const ABI: CanonicalAbiInfo = CanonicalAbiInfo::record_static(&[
                $($t::ABI),*
            ]);

            const IS_RUST_UNIT_TYPE: bool = {
                let mut _is_unit = true;
                $(
                    let _anything_to_bind_the_macro_variable = $t::IS_RUST_UNIT_TYPE;
                    _is_unit = false;
                )*
                _is_unit
            };

            fn typecheck(
                ty: &InterfaceType,
                types: &InstanceType<'_>,
            ) -> Result<()> {
                typecheck_tuple(ty, types, &[$($t::typecheck),*])
            }
        }

        #[allow(non_snake_case, reason = "macro-generated code")]
        unsafe impl<$($t,)*> Lower for ($($t,)*)
            where $($t: Lower),*
        {
            fn linear_lower_to_flat<U>(
                &self,
                cx: &mut LowerContext<'_, U>,
                ty: InterfaceType,
                _dst: &mut MaybeUninit<Self::Lower>,
            ) -> Result<()> {
                let types = match ty {
                    InterfaceType::Tuple(t) => &cx.types[t].types,
                    _ => bad_type_info(),
                };
                let ($($t,)*) = self;
                let mut _types = types.iter();
                $(
                    let ty = *_types.next().unwrap_or_else(bad_type_info);
                    $t.linear_lower_to_flat(cx, ty, map_maybe_uninit!(_dst.$t))?;
                )*
                Ok(())
            }

            fn linear_lower_to_memory<U>(
                &self,
                cx: &mut LowerContext<'_, U>,
                ty: InterfaceType,
                mut _offset: usize,
            ) -> Result<()> {
                debug_assert!(_offset % (Self::ALIGN32 as usize) == 0);
                let types = match ty {
                    InterfaceType::Tuple(t) => &cx.types[t].types,
                    _ => bad_type_info(),
                };
                let ($($t,)*) = self;
                let mut _types = types.iter();
                $(
                    let ty = *_types.next().unwrap_or_else(bad_type_info);
                    $t.linear_lower_to_memory(cx, ty, $t::ABI.next_field32_size(&mut _offset))?;
                )*
                Ok(())
            }
        }

        #[allow(non_snake_case, reason = "macro-generated code")]
        unsafe impl<$($t,)*> Lift for ($($t,)*)
            where $($t: Lift),*
        {
            #[inline]
            fn linear_lift_from_flat(cx: &mut LiftContext<'_>, ty: InterfaceType, _src: &Self::Lower) -> Result<Self> {
                let types = match ty {
                    InterfaceType::Tuple(t) => &cx.types[t].types,
                    _ => bad_type_info(),
                };
                let mut _types = types.iter();
                Ok(($(
                    $t::linear_lift_from_flat(
                        cx,
                        *_types.next().unwrap_or_else(bad_type_info),
                        &_src.$t,
                    )?,
                )*))
            }

            #[inline]
            fn linear_lift_from_memory(cx: &mut LiftContext<'_>, ty: InterfaceType, bytes: &[u8]) -> Result<Self> {
                debug_assert!((bytes.as_ptr() as usize) % (Self::ALIGN32 as usize) == 0);
                let types = match ty {
                    InterfaceType::Tuple(t) => &cx.types[t].types,
                    _ => bad_type_info(),
                };
                let mut _types = types.iter();
                let mut _offset = 0;
                $(
                    let ty = *_types.next().unwrap_or_else(bad_type_info);
                    let $t = $t::linear_lift_from_memory(cx, ty, &bytes[$t::ABI.next_field32_size(&mut _offset)..][..$t::SIZE32])?;
                )*
                Ok(($($t,)*))
            }
        }

        #[allow(non_snake_case, reason = "macro-generated code")]
        unsafe impl<$($t,)*> ComponentNamedList for ($($t,)*)
            where $($t: ComponentType),*
        {}
    };
}

for_each_function_signature!(impl_component_ty_for_tuples);

pub fn desc(ty: &InterfaceType) -> &'static str {
    match ty {
        InterfaceType::U8 => "u8",
        InterfaceType::S8 => "s8",
        InterfaceType::U16 => "u16",
        InterfaceType::S16 => "s16",
        InterfaceType::U32 => "u32",
        InterfaceType::S32 => "s32",
        InterfaceType::U64 => "u64",
        InterfaceType::S64 => "s64",
        InterfaceType::Float32 => "f32",
        InterfaceType::Float64 => "f64",
        InterfaceType::Bool => "bool",
        InterfaceType::Char => "char",
        InterfaceType::String => "string",
        InterfaceType::List(_) => "list",
        InterfaceType::Tuple(_) => "tuple",
        InterfaceType::Option(_) => "option",
        InterfaceType::Result(_) => "result",

        InterfaceType::Record(_) => "record",
        InterfaceType::Variant(_) => "variant",
        InterfaceType::Flags(_) => "flags",
        InterfaceType::Enum(_) => "enum",
        InterfaceType::Own(_) => "owned resource",
        InterfaceType::Borrow(_) => "borrowed resource",
        InterfaceType::Future(_) => "future",
        InterfaceType::Stream(_) => "stream",
        InterfaceType::ErrorContext(_) => "error-context",
    }
}

#[cold]
#[doc(hidden)]
pub fn bad_type_info<T>() -> T {
    // NB: should consider something like `unreachable_unchecked` here if this
    // becomes a performance bottleneck at some point, but that also comes with
    // a tradeoff of propagating a lot of unsafety, so it may not be worth it.
    panic!("bad type information detected");
}
