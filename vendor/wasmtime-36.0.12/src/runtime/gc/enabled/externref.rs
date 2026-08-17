//! Implementation of `externref` in Wasmtime.

use super::{AnyRef, RootedGcRefImpl};
use crate::prelude::*;
use crate::runtime::vm::VMGcRef;
use crate::{
    AsContextMut, GcHeapOutOfMemory, GcRefImpl, GcRootIndex, HeapType, ManuallyRooted, RefType,
    Result, Rooted, StoreContext, StoreContextMut, ValRaw, ValType, WasmTy,
    store::{AutoAssertNoGc, StoreOpaque},
};
use core::any::Any;
use core::mem;
use core::mem::MaybeUninit;

/// An opaque, GC-managed reference to some host data that can be passed to
/// WebAssembly.
///
/// The `ExternRef` type represents WebAssembly `externref` values. Wasm can't
/// do anything with the `externref`s other than put them in tables, globals,
/// and locals or pass them to other functions (such as imported functions from
/// the host). Unlike `anyref`s, Wasm guests cannot directly allocate new
/// `externref`s; only the host can.
///
/// You can use `ExternRef` to give access to host objects and control the
/// operations that Wasm can perform on them via what functions you allow Wasm
/// to import.
///
/// Like all WebAssembly references, these are opaque and unforgeable to Wasm:
/// they cannot be faked and Wasm cannot, for example, cast the integer
/// `0x12345678` into a reference, pretend it is a valid `externref`, and trick
/// the host into dereferencing it and segfaulting or worse.
///
/// Note that you can also use `Rooted<ExternRef>` and
/// `ManuallyRooted<ExternRef>` as a type parameter with
/// [`Func::typed`][crate::Func::typed]- and
/// [`Func::wrap`][crate::Func::wrap]-style APIs.
///
/// # Example
///
/// ```
/// # use wasmtime::*;
/// # use std::borrow::Cow;
/// # fn _foo() -> Result<()> {
/// let engine = Engine::default();
/// let mut store = Store::new(&engine, ());
///
/// // Define some APIs for working with host strings from Wasm via `externref`.
/// let mut linker = Linker::new(&engine);
/// linker.func_wrap(
///     "host-string",
///     "new",
///     |caller: Caller<'_, ()>| -> Result<Rooted<ExternRef>> {
///         ExternRef::new(caller, Cow::from(""))
///     },
/// )?;
/// linker.func_wrap(
///     "host-string",
///     "concat",
///     |mut caller: Caller<'_, ()>, a: Rooted<ExternRef>, b: Rooted<ExternRef>| -> Result<Rooted<ExternRef>> {
///         let mut s = a
///             .data(&caller)?
///             .ok_or_else(|| Error::msg("externref has no host data"))?
///             .downcast_ref::<Cow<str>>()
///             .ok_or_else(|| Error::msg("externref was not a string"))?
///             .clone()
///             .into_owned();
///         let b = b
///             .data(&caller)?
///             .ok_or_else(|| Error::msg("externref has no host data"))?
///             .downcast_ref::<Cow<str>>()
///             .ok_or_else(|| Error::msg("externref was not a string"))?;
///         s.push_str(&b);
///         ExternRef::new(&mut caller, s)
///     },
/// )?;
///
/// // Here is a Wasm module that uses those APIs.
/// let module = Module::new(
///     &engine,
///     r#"
///         (module
///             (import "host-string" "concat" (func $concat (param externref externref)
///                                                          (result externref)))
///             (func (export "run") (param externref externref) (result externref)
///                 local.get 0
///                 local.get 1
///                 call $concat
///             )
///         )
///     "#,
/// )?;
///
/// // Create a couple `externref`s wrapping `Cow<str>`s.
/// let hello = ExternRef::new(&mut store, Cow::from("Hello, "))?;
/// let world = ExternRef::new(&mut store, Cow::from("World!"))?;
///
/// // Instantiate the module and pass the `externref`s into it.
/// let instance = linker.instantiate(&mut store, &module)?;
/// let result = instance
///     .get_typed_func::<(Rooted<ExternRef>, Rooted<ExternRef>), Rooted<ExternRef>>(&mut store, "run")?
///     .call(&mut store, (hello, world))?;
///
/// // The module should have concatenated the strings together!
/// assert_eq!(
///     result
///         .data(&store)?
///         .expect("externref should have host data")
///         .downcast_ref::<Cow<str>>()
///         .expect("host data should be a `Cow<str>`"),
///     "Hello, World!"
/// );
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ExternRef {
    pub(crate) inner: GcRootIndex,
}

unsafe impl GcRefImpl for ExternRef {
    fn transmute_ref(index: &GcRootIndex) -> &Self {
        // Safety: `ExternRef` is a newtype of a `GcRootIndex`.
        let me: &Self = unsafe { mem::transmute(index) };

        // Assert we really are just a newtype of a `GcRootIndex`.
        assert!(matches!(
            me,
            Self {
                inner: GcRootIndex { .. },
            }
        ));

        me
    }
}

impl ExternRef {
    /// Synchronously allocates a new `ExternRef` wrapping the given value.
    ///
    /// The resulting value is automatically unrooted when the given `context`'s
    /// scope is exited. If you need to hold the reference past the `context`'s
    /// scope, convert the result into a
    /// [`ManuallyRooted<T>`][crate::ManuallyRooted]. See the documentation for
    /// [`Rooted<T>`][crate::Rooted] and
    /// [`ManuallyRooted<T>`][crate::ManuallyRooted] for more details.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for allocating a new
    /// `externref`, this method will automatically trigger a synchronous
    /// collection in an attempt to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<T>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// The [`GcHeapOutOfMemory<T>`][crate::GcHeapOutOfMemory] error contains
    /// the host value that the `externref` would have wrapped. You can extract
    /// that value from the error and reuse it when attempting to allocate an
    /// `externref` again after dropping rooted GC references and then
    /// performing a collection or otherwise do with it whatever you see fit.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn _foo() -> Result<()> {
    /// let mut store = Store::<()>::default();
    ///
    /// {
    ///     let mut scope = RootScope::new(&mut store);
    ///
    ///     // Allocate an `externref` wrapping a `String`.
    ///     let externref = match ExternRef::new(&mut scope, "hello!".to_string()) {
    ///         // The allocation succeeded.
    ///         Ok(x) => x,
    ///         // The allocation failed.
    ///         Err(e) => match e.downcast::<GcHeapOutOfMemory<String>>() {
    ///             // The allocation failed because the GC heap does not have
    ///             // capacity for this allocation.
    ///             Ok(oom) => {
    ///                 // Take back ownership of our `String`.
    ///                 let s = oom.into_inner();
    ///                 // Drop some rooted GC refs from our system to potentially
    ///                 // free up space for Wasmtime to make this allocation.
    /// #               let drop_some_gc_refs = || {};
    ///                 drop_some_gc_refs();
    ///                 // Finally, try to allocate again, reusing the original
    ///                 // string.
    ///                 ExternRef::new(&mut scope, s)?
    ///             }
    ///             Err(e) => return Err(e),
    ///         },
    ///     };
    ///
    ///     // Use the `externref`, pass it to Wasm, etc...
    /// }
    ///
    /// // The `externref` is automatically unrooted when we exit the scope.
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the `context` is configured for async; use
    /// [`ExternRef::new_async`][crate::ExternRef::new_async] to perform
    /// asynchronous allocation instead.
    pub fn new<T>(mut context: impl AsContextMut, value: T) -> Result<Rooted<ExternRef>>
    where
        T: 'static + Any + Send + Sync,
    {
        let ctx = context.as_context_mut().0;
        Self::_new(ctx, value)
    }

    pub(crate) fn _new<T>(store: &mut StoreOpaque, value: T) -> Result<Rooted<ExternRef>>
    where
        T: 'static + Any + Send + Sync,
    {
        // Allocate the box once, regardless how many gc-and-retry attempts we
        // make.
        let value: Box<dyn Any + Send + Sync> = Box::new(value);

        let gc_ref = store
            .retry_after_gc(value, |store, value| {
                store
                    .gc_store_mut()?
                    .alloc_externref(value)
                    .context("unrecoverable error when allocating new `externref`")?
                    .map_err(|(x, n)| GcHeapOutOfMemory::new(x, n).into())
            })
            // Translate the `GcHeapOutOfMemory`'s inner value from the boxed
            // trait object into `T`.
            .map_err(
                |e| match e.downcast::<GcHeapOutOfMemory<Box<dyn Any + Send + Sync>>>() {
                    Ok(oom) => oom.map_inner(|x| *x.downcast::<T>().unwrap()).into(),
                    Err(e) => e,
                },
            )?;

        let mut ctx = AutoAssertNoGc::new(store);
        Ok(Self::from_cloned_gc_ref(&mut ctx, gc_ref.into()))
    }

    /// Asynchronously allocates a new `ExternRef` wrapping the given value.
    ///
    /// The resulting value is automatically unrooted when the given `context`'s
    /// scope is exited. If you need to hold the reference past the `context`'s
    /// scope, convert the result into a
    /// [`ManuallyRooted<T>`][crate::ManuallyRooted]. See the documentation for
    /// [`Rooted<T>`][crate::Rooted] and
    /// [`ManuallyRooted<T>`][crate::ManuallyRooted] for more details.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for allocating a new
    /// `externref`, this method will automatically trigger an asynchronous
    /// collection in an attempt to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<T>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// The [`GcHeapOutOfMemory<T>`][crate::GcHeapOutOfMemory] error contains
    /// the host value that the `externref` would have wrapped. You can extract
    /// that value from the error and reuse it when attempting to allocate an
    /// `externref` again after dropping rooted GC references and then
    /// performing a collection or otherwise do with it whatever you see fit.
    ///
    /// # Example
    ///
    /// ```
    /// use wasmtime::*;
    ///
    /// # async fn _foo() -> Result<()> {
    /// let mut store = Store::<()>::default();
    ///
    /// {
    ///     let mut scope = RootScope::new(&mut store);
    ///
    ///     // Create an `externref` wrapping a `String`.
    ///     let externref = match ExternRef::new_async(&mut scope, "hello!".to_string()).await {
    ///         // The allocation succeeded.
    ///         Ok(x) => x,
    ///         // The allocation failed.
    ///         Err(e) => match e.downcast::<GcHeapOutOfMemory<String>>() {
    ///             // The allocation failed because the GC heap does not have
    ///             // capacity for this allocation.
    ///             Ok(oom) => {
    ///                 // Take back ownership of our `String`.
    ///                 let s = oom.into_inner();
    ///                 // Drop some rooted GC refs from our system to potentially
    ///                 // free up space for Wasmtime to make this allocation.
    /// #               let drop_some_gc_refs = || {};
    ///                 drop_some_gc_refs();
    ///                 // Finally, try to allocate again, reusing the original
    ///                 // string.
    ///                 ExternRef::new_async(&mut scope, s).await?
    ///             }
    ///             Err(e) => return Err(e),
    ///         },
    ///     };
    ///
    ///     // Use the `externref`, pass it to Wasm, etc...
    /// }
    ///
    /// // The `externref` is automatically unrooted when we exit the scope.
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the `context` is not configured for async; use
    /// [`ExternRef::new`][crate::ExternRef::new] to perform synchronous
    /// allocation instead.
    #[cfg(feature = "async")]
    pub async fn new_async<T>(mut context: impl AsContextMut, value: T) -> Result<Rooted<ExternRef>>
    where
        T: 'static + Any + Send + Sync,
    {
        let ctx = context.as_context_mut().0;
        Self::_new_async(ctx, value).await
    }

    #[cfg(feature = "async")]
    pub(crate) async fn _new_async<T>(
        store: &mut StoreOpaque,
        value: T,
    ) -> Result<Rooted<ExternRef>>
    where
        T: 'static + Any + Send + Sync,
    {
        // Allocate the box once, regardless how many gc-and-retry attempts we
        // make.
        let value: Box<dyn Any + Send + Sync> = Box::new(value);

        let gc_ref = store
            .retry_after_gc_async(value, |store, value| {
                store
                    .gc_store_mut()?
                    .alloc_externref(value)
                    .context("unrecoverable error when allocating new `externref`")?
                    .map_err(|(x, n)| GcHeapOutOfMemory::new(x, n).into())
            })
            .await
            // Translate the `GcHeapOutOfMemory`'s inner value from the boxed
            // trait object into `T`.
            .map_err(
                |e| match e.downcast::<GcHeapOutOfMemory<Box<dyn Any + Send + Sync>>>() {
                    Ok(oom) => oom.map_inner(|x| *x.downcast::<T>().unwrap()).into(),
                    Err(e) => e,
                },
            )?;

        let mut ctx = AutoAssertNoGc::new(store);
        Ok(Self::from_cloned_gc_ref(&mut ctx, gc_ref.into()))
    }

    /// Convert an `anyref` into an `externref`.
    ///
    /// This is equivalent to the `extern.convert_any` instruction in Wasm.
    ///
    /// You can get the underlying `anyref` again via the
    /// [`AnyRef::convert_extern`] method or the `any.convert_extern` Wasm
    /// instruction.
    ///
    /// The resulting `ExternRef` will not have any host data associated with
    /// it, so [`ExternRef::data`] and [`ExternRef::data_mut`] will both return
    /// `None`.
    ///
    /// Returns an error if the `anyref` GC reference has been unrooted (eg if
    /// you attempt to use a `Rooted<AnyRef>` after exiting the scope it was
    /// rooted within). See the documentation for [`Rooted<T>`][crate::Rooted]
    /// for more details.
    ///
    /// # Example
    ///
    /// ```
    /// use wasmtime::*;
    /// # fn foo() -> Result<()> {
    /// let engine = Engine::default();
    /// let mut store = Store::new(&engine, ());
    ///
    /// // Create an `anyref`.
    /// let i31 = I31::wrapping_u32(0x1234);
    /// let anyref = AnyRef::from_i31(&mut store, i31);
    ///
    /// // Convert that `anyref` into an `externref`.
    /// let externref = ExternRef::convert_any(&mut store, anyref)?;
    ///
    /// // This `externref` doesn't have any associated host data.
    /// assert!(externref.data(&store)?.is_none());
    ///
    /// // We can convert it back to an `anyref` and get its underlying `i31`
    /// // data.
    /// let anyref = AnyRef::convert_extern(&mut store, externref)?;
    /// assert_eq!(anyref.unwrap_i31(&store)?.get_u32(), 0x1234);
    /// # Ok(()) }
    /// # foo().unwrap();
    pub fn convert_any(
        mut context: impl AsContextMut,
        anyref: Rooted<AnyRef>,
    ) -> Result<Rooted<ExternRef>> {
        let mut store = AutoAssertNoGc::new(context.as_context_mut().0);
        Self::_convert_any(&mut store, anyref)
    }

    pub(crate) fn _convert_any(
        store: &mut AutoAssertNoGc<'_>,
        anyref: Rooted<AnyRef>,
    ) -> Result<Rooted<ExternRef>> {
        let gc_ref = anyref.try_clone_gc_ref(store)?;
        Ok(Self::from_cloned_gc_ref(store, gc_ref))
    }

    /// Create a new `Rooted<ExternRef>` from the given GC reference.
    ///
    /// Does not invoke the `GcRuntime`'s clone hook; callers should ensure it
    /// has been called.
    ///
    /// `gc_ref` should be a GC reference pointing to an instance of `externref`
    /// that is in this store's GC heap. Failure to uphold this invariant is
    /// memory safe but will result in general incorrectness such as panics and
    /// wrong results.
    pub(crate) fn from_cloned_gc_ref(
        store: &mut AutoAssertNoGc<'_>,
        gc_ref: VMGcRef,
    ) -> Rooted<Self> {
        assert!(
            gc_ref.is_extern_ref(&*store.unwrap_gc_store().gc_heap)
                || gc_ref.is_any_ref(&*store.unwrap_gc_store().gc_heap),
            "GC reference {gc_ref:#p} should be an externref or anyref"
        );
        Rooted::new(store, gc_ref)
    }

    /// Get a shared borrow of the underlying data for this `ExternRef`.
    ///
    /// Returns `None` if this is an `externref` wrapper of an `anyref` created
    /// by the `extern.convert_any` instruction or the
    /// [`ExternRef::convert_any`] method.
    ///
    /// Returns an error if this `externref` GC reference has been unrooted (eg
    /// if you attempt to use a `Rooted<ExternRef>` after exiting the scope it
    /// was rooted within). See the documentation for
    /// [`Rooted<T>`][crate::Rooted] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn _foo() -> Result<()> {
    /// let mut store = Store::<()>::default();
    ///
    /// let externref = ExternRef::new(&mut store, "hello")?;
    ///
    /// // Access the `externref`'s host data.
    /// let data = externref.data(&store)?.ok_or_else(|| Error::msg("no host data"))?;
    /// // Dowcast it to a `&str`.
    /// let data = data.downcast_ref::<&str>().ok_or_else(|| Error::msg("not a str"))?;
    /// // We should have got the data we created the `externref` with!
    /// assert_eq!(*data, "hello");
    /// # Ok(())
    /// # }
    /// ```
    pub fn data<'a, T>(
        &self,
        store: impl Into<StoreContext<'a, T>>,
    ) -> Result<Option<&'a (dyn Any + Send + Sync)>>
    where
        T: 'static,
    {
        let store = store.into().0;
        let gc_ref = self.inner.try_gc_ref(&store)?;
        let gc_store = store.gc_store()?;
        if let Some(externref) = gc_ref.as_externref(&*gc_store.gc_heap) {
            Ok(Some(gc_store.externref_host_data(externref)))
        } else {
            Ok(None)
        }
    }

    /// Get an exclusive borrow of the underlying data for this `ExternRef`.
    ///
    /// Returns `None` if this is an `externref` wrapper of an `anyref` created
    /// by the `extern.convert_any` instruction or the
    /// [`ExternRef::convert_any`] constructor.
    ///
    /// Returns an error if this `externref` GC reference has been unrooted (eg
    /// if you attempt to use a `Rooted<ExternRef>` after exiting the scope it
    /// was rooted within). See the documentation for
    /// [`Rooted<T>`][crate::Rooted] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn _foo() -> Result<()> {
    /// let mut store = Store::<()>::default();
    ///
    /// let externref = ExternRef::new::<usize>(&mut store, 0)?;
    ///
    /// // Access the `externref`'s host data.
    /// let data = externref.data_mut(&mut store)?.ok_or_else(|| Error::msg("no host data"))?;
    /// // Dowcast it to a `usize`.
    /// let data = data.downcast_mut::<usize>().ok_or_else(|| Error::msg("not a usize"))?;
    /// // We initialized to zero.
    /// assert_eq!(*data, 0);
    /// // And we can mutate the value!
    /// *data += 10;
    /// # Ok(())
    /// # }
    /// ```
    pub fn data_mut<'a, T>(
        &self,
        store: impl Into<StoreContextMut<'a, T>>,
    ) -> Result<Option<&'a mut (dyn Any + Send + Sync)>>
    where
        T: 'static,
    {
        let store = store.into().0;
        // NB: need to do an unchecked copy to release the borrow on the store
        // so that we can get the store's GC store. But importantly we cannot
        // trigger a GC while we are working with `gc_ref` here.
        let gc_ref = self.inner.try_gc_ref(store)?.unchecked_copy();
        let gc_store = store.gc_store_mut()?;
        if let Some(externref) = gc_ref.as_externref(&*gc_store.gc_heap) {
            Ok(Some(gc_store.externref_host_data_mut(externref)))
        } else {
            Ok(None)
        }
    }

    /// Creates a new strongly-owned [`ExternRef`] from the raw value provided.
    ///
    /// This is intended to be used in conjunction with [`Func::new_unchecked`],
    /// [`Func::call_unchecked`], and [`ValRaw`] with its `externref` field.
    ///
    /// This function assumes that `raw` is an externref value which is
    /// currently rooted within the [`Store`].
    ///
    /// # Correctness
    ///
    /// This function is tricky to get right because `raw` not only must be a
    /// valid `externref` value produced prior by [`ExternRef::to_raw`] but it
    /// must also be correctly rooted within the store. When arguments are
    /// provided to a callback with [`Func::new_unchecked`], for example, or
    /// returned via [`Func::call_unchecked`], if a GC is performed within the
    /// store then floating `externref` values are not rooted and will be GC'd,
    /// meaning that this function will no longer be correct to call with the
    /// values cleaned up. This function must be invoked *before* possible GC
    /// operations can happen (such as calling Wasm).
    ///
    ///
    /// When in doubt try to not use this. Instead use the Rust APIs of
    /// [`TypedFunc`] and friends. Note though that this function is not
    /// `unsafe` as any value can be passed in. Incorrect values can result in
    /// runtime panics, however, so care must still be taken with this method.
    ///
    /// [`Func::call_unchecked`]: crate::Func::call_unchecked
    /// [`Func::new_unchecked`]: crate::Func::new_unchecked
    /// [`Store`]: crate::Store
    /// [`TypedFunc`]: crate::TypedFunc
    /// [`ValRaw`]: crate::ValRaw
    pub fn from_raw(mut store: impl AsContextMut, raw: u32) -> Option<Rooted<ExternRef>> {
        let mut store = AutoAssertNoGc::new(store.as_context_mut().0);
        Self::_from_raw(&mut store, raw)
    }

    // (Not actually memory unsafe since we have indexed GC heaps.)
    pub(crate) fn _from_raw(store: &mut AutoAssertNoGc, raw: u32) -> Option<Rooted<ExternRef>> {
        let gc_ref = VMGcRef::from_raw_u32(raw)?;
        let gc_ref = store.unwrap_gc_store_mut().clone_gc_ref(&gc_ref);
        Some(Self::from_cloned_gc_ref(store, gc_ref))
    }

    /// Converts this [`ExternRef`] to a raw value suitable to store within a
    /// [`ValRaw`].
    ///
    /// Returns an error if this `externref` has been unrooted.
    ///
    /// # Correctness
    ///
    /// Produces a raw value which is only valid to pass into a store if a GC
    /// doesn't happen between when the value is produce and when it's passed
    /// into the store.
    ///
    /// [`ValRaw`]: crate::ValRaw
    pub fn to_raw(&self, mut store: impl AsContextMut) -> Result<u32> {
        let mut store = AutoAssertNoGc::new(store.as_context_mut().0);
        self._to_raw(&mut store)
    }

    pub(crate) fn _to_raw(&self, store: &mut AutoAssertNoGc) -> Result<u32> {
        let gc_ref = self.inner.try_clone_gc_ref(store)?;
        let raw = store.unwrap_gc_store_mut().expose_gc_ref_to_wasm(gc_ref);
        Ok(raw.get())
    }
}

unsafe impl WasmTy for Rooted<ExternRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Extern))
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.comes_from_same_store(store)
    }

    #[inline]
    fn dynamic_concrete_type_check(&self, _: &StoreOpaque, _: bool, _: &HeapType) -> Result<()> {
        unreachable!()
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        self.wasm_ty_store(store, ptr, ValRaw::externref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        Self::wasm_ty_load(store, ptr.get_externref(), ExternRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<Rooted<ExternRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::EXTERNREF
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.map_or(true, |x| x.comes_from_same_store(store))
    }

    #[inline]
    fn dynamic_concrete_type_check(&self, _: &StoreOpaque, _: bool, _: &HeapType) -> Result<()> {
        unreachable!()
    }

    #[inline]
    fn is_vmgcref_and_points_to_object(&self) -> bool {
        self.is_some()
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        <Rooted<ExternRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::externref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <Rooted<ExternRef>>::wasm_ty_option_load(
            store,
            ptr.get_externref(),
            ExternRef::from_cloned_gc_ref,
        )
    }
}

unsafe impl WasmTy for ManuallyRooted<ExternRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Extern))
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.comes_from_same_store(store)
    }

    #[inline]
    fn dynamic_concrete_type_check(&self, _: &StoreOpaque, _: bool, _: &HeapType) -> Result<()> {
        unreachable!()
    }

    #[inline]
    fn is_vmgcref_and_points_to_object(&self) -> bool {
        true
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        self.wasm_ty_store(store, ptr, ValRaw::externref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        Self::wasm_ty_load(store, ptr.get_externref(), ExternRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<ManuallyRooted<ExternRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::EXTERNREF
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.as_ref()
            .map_or(true, |x| x.comes_from_same_store(store))
    }

    #[inline]
    fn dynamic_concrete_type_check(&self, _: &StoreOpaque, _: bool, _: &HeapType) -> Result<()> {
        unreachable!()
    }

    #[inline]
    fn is_vmgcref_and_points_to_object(&self) -> bool {
        self.is_some()
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        <ManuallyRooted<ExternRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::externref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <ManuallyRooted<ExternRef>>::wasm_ty_option_load(
            store,
            ptr.get_externref(),
            ExternRef::from_cloned_gc_ref,
        )
    }
}
