use super::invoke_wasm_and_catch_traps;
use crate::prelude::*;
use crate::runtime::vm::VMFuncRef;
use crate::store::{AutoAssertNoGc, StoreOpaque};
use crate::{
    AsContext, AsContextMut, Engine, Func, FuncType, HeapType, NoFunc, RefType, StoreContextMut,
    ValRaw, ValType,
};
use core::ffi::c_void;
use core::marker;
use core::mem::{self, MaybeUninit};
use core::ptr::{self, NonNull};
use wasmtime_environ::VMSharedTypeIndex;

/// A statically typed WebAssembly function.
///
/// Values of this type represent statically type-checked WebAssembly functions.
/// The function within a [`TypedFunc`] is statically known to have `Params` as its
/// parameters and `Results` as its results.
///
/// This structure is created via [`Func::typed`] or [`TypedFunc::new_unchecked`].
/// For more documentation about this see those methods.
pub struct TypedFunc<Params, Results> {
    _a: marker::PhantomData<fn(Params) -> Results>,
    ty: FuncType,
    func: Func,
}

impl<Params, Results> Clone for TypedFunc<Params, Results> {
    fn clone(&self) -> TypedFunc<Params, Results> {
        Self {
            _a: marker::PhantomData,
            ty: self.ty.clone(),
            func: self.func,
        }
    }
}

impl<Params, Results> TypedFunc<Params, Results>
where
    Params: WasmParams,
    Results: WasmResults,
{
    /// An unchecked version of [`Func::typed`] which does not perform a
    /// typecheck and simply assumes that the type declared here matches the
    /// type of this function.
    ///
    /// The semantics of this function are the same as [`Func::typed`] except
    /// that no error is returned because no typechecking is done.
    ///
    /// # Unsafety
    ///
    /// This function only safe to call if `typed` would otherwise return `Ok`
    /// for the same `Params` and `Results` specified. If `typed` would return
    /// an error then the returned `TypedFunc` is memory unsafe to invoke.
    pub unsafe fn new_unchecked(store: impl AsContext, func: Func) -> TypedFunc<Params, Results> {
        let store = store.as_context().0;
        unsafe { Self::_new_unchecked(store, func) }
    }

    pub(crate) unsafe fn _new_unchecked(
        store: &StoreOpaque,
        func: Func,
    ) -> TypedFunc<Params, Results> {
        let ty = func.load_ty(store);
        TypedFunc {
            _a: marker::PhantomData,
            ty,
            func,
        }
    }

    /// Returns the underlying [`Func`] that this is wrapping, losing the static
    /// type information in the process.
    pub fn func(&self) -> &Func {
        &self.func
    }

    /// Invokes this WebAssembly function with the specified parameters.
    ///
    /// Returns either the results of the call, or a [`Trap`] if one happened.
    ///
    /// For more information, see the [`Func::typed`] and [`Func::call`]
    /// documentation.
    ///
    /// # Errors
    ///
    /// For more information on errors see the documentation on [`Func::call`].
    ///
    /// # Panics
    ///
    /// This function will panic if it is called when the underlying [`Func`] is
    /// connected to an asynchronous store.
    ///
    /// [`Trap`]: crate::Trap
    #[inline]
    pub fn call(&self, mut store: impl AsContextMut, params: Params) -> Result<Results> {
        let mut store = store.as_context_mut();
        assert!(
            !store.0.async_support(),
            "must use `call_async` with async stores"
        );

        let func = self.func.vm_func_ref(store.0);
        unsafe { Self::call_raw(&mut store, &self.ty, func, params) }
    }

    /// Invokes this WebAssembly function with the specified parameters.
    ///
    /// Returns either the results of the call, or a [`Trap`] if one happened.
    ///
    /// For more information, see the [`Func::typed`] and [`Func::call_async`]
    /// documentation.
    ///
    /// # Errors
    ///
    /// For more information on errors see the documentation on [`Func::call`].
    ///
    /// # Panics
    ///
    /// This function will panic if it is called when the underlying [`Func`] is
    /// connected to a synchronous store.
    ///
    /// [`Trap`]: crate::Trap
    #[cfg(feature = "async")]
    pub async fn call_async(
        &self,
        mut store: impl AsContextMut<Data: Send>,
        params: Params,
    ) -> Result<Results>
    where
        Params: Sync,
        Results: Sync,
    {
        let mut store = store.as_context_mut();
        assert!(
            store.0.async_support(),
            "must use `call` with non-async stores"
        );

        store
            .on_fiber(|store| {
                let func = self.func.vm_func_ref(store.0);
                unsafe { Self::call_raw(store, &self.ty, func, params) }
            })
            .await?
    }

    /// Do a raw call of a typed function.
    ///
    /// # Safety
    ///
    /// `func` must be of the given type, and it additionally must be a valid
    /// store-owned pointer within the `store` provided.
    pub(crate) unsafe fn call_raw<T>(
        store: &mut StoreContextMut<'_, T>,
        ty: &FuncType,
        func: ptr::NonNull<VMFuncRef>,
        params: Params,
    ) -> Result<Results> {
        // double-check that params/results match for this function's type in
        // debug mode.
        //
        // SAFETY: this function requires that `ptr` is a valid function
        // pointer.
        unsafe {
            if cfg!(debug_assertions) {
                Self::debug_typecheck(store.0, func.as_ref().type_index);
            }
        }

        // Validate that all runtime values flowing into this store indeed
        // belong within this store, otherwise it would be unsafe for store
        // values to cross each other.

        union Storage<T: Copy, U: Copy> {
            params: MaybeUninit<T>,
            results: U,
        }

        let mut storage = Storage::<Params::ValRawStorage, Results::ValRawStorage> {
            params: MaybeUninit::uninit(),
        };

        {
            let mut store = AutoAssertNoGc::new(store.0);
            // SAFETY: it's safe to use a union field here as the field itself
            // is `MaybeUninit<_>` meaning nothing is accidentally considered
            // initialized.
            let dst: &mut MaybeUninit<_> = unsafe { &mut storage.params };
            params.store(&mut store, ty, dst)?;
        }

        // Try to capture only a single variable (a tuple) in the closure below.
        // This means the size of the closure is one pointer and is much more
        // efficient to move in memory. This closure is actually invoked on the
        // other side of a C++ shim, so it can never be inlined enough to make
        // the memory go away, so the size matters here for performance.
        let mut captures = (func, storage);

        let result = invoke_wasm_and_catch_traps(store, |caller, vm| {
            let (func_ref, storage) = &mut captures;
            let storage_len = mem::size_of_val::<Storage<_, _>>(storage) / mem::size_of::<ValRaw>();
            let storage: *mut Storage<_, _> = storage;
            let storage = storage.cast::<ValRaw>();
            let storage = core::ptr::slice_from_raw_parts_mut(storage, storage_len);
            let storage = NonNull::new(storage).unwrap();

            // SAFETY: this function's own contract is that `func_ref` is safe
            // to call and additionally that the params/results are correctly
            // ascribed for this function call to be safe.
            unsafe { VMFuncRef::array_call(*func_ref, vm, caller, storage) }
        });

        let (_, storage) = captures;
        result?;

        let mut store = AutoAssertNoGc::new(store.0);
        // SAFETY: this function is itself unsafe to ensure that the result type
        // ascription is correct for `Results` and matches the actual function.
        // Additionally given the correct type ascription all of the `results`
        // accessed here should be validly initialized.
        unsafe { Ok(Results::load(&mut store, &storage.results)) }
    }

    /// Purely a debug-mode assertion, not actually used in release builds.
    fn debug_typecheck(store: &StoreOpaque, func: VMSharedTypeIndex) {
        let ty = FuncType::from_shared_type_index(store.engine(), func);
        Params::typecheck(store.engine(), ty.params(), TypeCheckPosition::Param)
            .expect("params should match");
        Results::typecheck(store.engine(), ty.results(), TypeCheckPosition::Result)
            .expect("results should match");
    }
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum TypeCheckPosition {
    Param,
    Result,
}

/// A trait implemented for types which can be arguments and results for
/// closures passed to [`Func::wrap`] as well as parameters to [`Func::typed`].
///
/// This trait should not be implemented by user types. This trait may change at
/// any time internally. The types which implement this trait, however, are
/// stable over time.
///
/// For more information see [`Func::wrap`] and [`Func::typed`]
pub unsafe trait WasmTy: Send {
    // Do a "static" (aka at time of `func.typed::<P, R>()`) ahead-of-time type
    // check for this type at the given position. You probably don't need to
    // override this trait method.
    #[doc(hidden)]
    #[inline]
    fn typecheck(engine: &Engine, actual: ValType, position: TypeCheckPosition) -> Result<()> {
        let expected = Self::valtype();
        debug_assert!(expected.comes_from_same_engine(engine));
        debug_assert!(actual.comes_from_same_engine(engine));
        match position {
            // The caller is expecting to receive a `T` and the callee is
            // actually returning a `U`, so ensure that `U <: T`.
            TypeCheckPosition::Result => actual.ensure_matches(engine, &expected),
            // The caller is expecting to pass a `T` and the callee is expecting
            // to receive a `U`, so ensure that `T <: U`.
            TypeCheckPosition::Param => match (expected.as_ref(), actual.as_ref()) {
                // ... except that this technically-correct check would overly
                // restrict the usefulness of our typed function APIs for the
                // specific case of concrete reference types. Let's work through
                // an example.
                //
                // Consider functions that take a `(ref param $some_func_type)`
                // parameter:
                //
                // * We cannot have a static `wasmtime::SomeFuncTypeRef` type
                //   that implements `WasmTy` specifically for `(ref null
                //   $some_func_type)` because Wasm modules, and their types,
                //   are loaded dynamically at runtime.
                //
                // * Therefore the embedder's only option for `T <: (ref null
                //   $some_func_type)` is `T = (ref null nofunc)` aka
                //   `Option<wasmtime::NoFunc>`.
                //
                // * But that static type means they can *only* pass in the null
                //   function reference as an argument to the typed function.
                //   This is way too restrictive! For ergonomics, we want them
                //   to be able to pass in a `wasmtime::Func` whose type is
                //   `$some_func_type`!
                //
                // To lift this constraint and enable better ergonomics for
                // embedders, we allow `top(T) <: top(U)` -- i.e. they are part
                // of the same type hierarchy and a dynamic cast could possibly
                // succeed -- for the specific case of concrete heap type
                // parameters, and fall back to dynamic type checks on the
                // arguments passed to each invocation, as necessary.
                (Some(expected_ref), Some(actual_ref)) if actual_ref.heap_type().is_concrete() => {
                    expected_ref
                        .heap_type()
                        .top()
                        .ensure_matches(engine, &actual_ref.heap_type().top())
                }
                _ => expected.ensure_matches(engine, &actual),
            },
        }
    }

    // The value type that this Type represents.
    #[doc(hidden)]
    fn valtype() -> ValType;

    #[doc(hidden)]
    fn may_gc() -> bool {
        match Self::valtype() {
            ValType::Ref(_) => true,
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => false,
        }
    }

    // Dynamic checks that this value is being used with the correct store
    // context.
    #[doc(hidden)]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool;

    // Dynamic checks that `self <: actual` for concrete type arguments. See the
    // comment above in `WasmTy::typecheck`.
    //
    // Only ever called for concrete reference type arguments, so any type which
    // is not in a type hierarchy with concrete reference types can implement
    // this with `unreachable!()`.
    #[doc(hidden)]
    fn dynamic_concrete_type_check(
        &self,
        store: &StoreOpaque,
        nullable: bool,
        actual: &HeapType,
    ) -> Result<()>;

    // Is this a GC-managed reference that actually points to a GC object? That
    // is, `self` is *not* an `i31`, null reference, or uninhabited type.
    //
    // Note that it is okay if this returns false positives (i.e. `true` for
    // `Rooted<AnyRef>` without actually looking up the rooted `anyref` in the
    // store and reflecting on it to determine whether it is actually an
    // `i31`). However, it is not okay if this returns false negatives.
    #[doc(hidden)]
    #[inline]
    fn is_vmgcref_and_points_to_object(&self) -> bool {
        Self::valtype().is_vmgcref_type_and_points_to_object()
    }

    // Store `self` into `ptr`.
    //
    // NB: We _must not_ trigger a GC when passing refs from host code into Wasm
    // (e.g. returned from a host function or passed as arguments to a Wasm
    // function). After insertion into the activations table, the reference is
    // no longer rooted. If multiple references are being sent from the host
    // into Wasm and we allowed GCs during insertion, then the following events
    // could happen:
    //
    // * Reference A is inserted into the activations table. This does not
    //   trigger a GC, but does fill the table to capacity.
    //
    // * The caller's reference to A is removed. Now the only reference to A is
    //   from the activations table.
    //
    // * Reference B is inserted into the activations table. Because the table
    //   is at capacity, a GC is triggered.
    //
    // * A is reclaimed because the only reference keeping it alive was the
    //   activation table's reference (it isn't inside any Wasm frames on the
    //   stack yet, so stack scanning and stack maps don't increment its
    //   reference count).
    //
    // * We transfer control to Wasm, giving it A and B. Wasm uses A. That's a
    //   use-after-free bug.
    //
    // In conclusion, to prevent uses-after-free bugs, we cannot GC while
    // converting types into their raw ABI forms.
    #[doc(hidden)]
    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()>;

    // Load a version of `Self` from the `ptr` provided.
    //
    // # Safety
    //
    // This function is unsafe as it's up to the caller to ensure that `ptr` is
    // valid for this given type.
    #[doc(hidden)]
    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self;
}

macro_rules! integers {
    ($($primitive:ident/$get_primitive:ident => $ty:ident)*) => ($(
        unsafe impl WasmTy for $primitive {
            #[inline]
            fn valtype() -> ValType {
                ValType::$ty
            }
            #[inline]
            fn compatible_with_store(&self, _: &StoreOpaque) -> bool {
                true
            }
            #[inline]
            fn dynamic_concrete_type_check(&self, _: &StoreOpaque, _: bool, _: &HeapType) -> Result<()> {
                unreachable!()
            }
            #[inline]
            fn store(self, _store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
                ptr.write(ValRaw::$primitive(self));
                Ok(())
            }
            #[inline]
            unsafe fn load(_store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
                ptr.$get_primitive()
            }
        }
    )*)
}

integers! {
    i32/get_i32 => I32
    i64/get_i64 => I64
    u32/get_u32 => I32
    u64/get_u64 => I64
}

macro_rules! floats {
    ($($float:ident/$int:ident/$get_float:ident => $ty:ident)*) => ($(
        unsafe impl WasmTy for $float {
            #[inline]
            fn valtype() -> ValType {
                ValType::$ty
            }
            #[inline]
            fn compatible_with_store(&self, _: &StoreOpaque) -> bool {
                true
            }
            #[inline]
            fn dynamic_concrete_type_check(&self, _: &StoreOpaque, _: bool, _: &HeapType) -> Result<()> {
                unreachable!()
            }
            #[inline]
            fn store(self, _store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
                ptr.write(ValRaw::$float(self.to_bits()));
                Ok(())
            }
            #[inline]
            unsafe fn load(_store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
                $float::from_bits(ptr.$get_float())
            }
        }
    )*)
}

floats! {
    f32/u32/get_f32 => F32
    f64/u64/get_f64 => F64
}

unsafe impl WasmTy for NoFunc {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::NoFunc))
    }

    #[inline]
    fn compatible_with_store(&self, _store: &StoreOpaque) -> bool {
        match self._inner {}
    }

    #[inline]
    fn dynamic_concrete_type_check(&self, _: &StoreOpaque, _: bool, _: &HeapType) -> Result<()> {
        match self._inner {}
    }

    #[inline]
    fn is_vmgcref_and_points_to_object(&self) -> bool {
        match self._inner {}
    }

    #[inline]
    fn store(self, _store: &mut AutoAssertNoGc<'_>, _ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        match self._inner {}
    }

    #[inline]
    unsafe fn load(_store: &mut AutoAssertNoGc<'_>, _ptr: &ValRaw) -> Self {
        unreachable!("NoFunc is uninhabited")
    }
}

unsafe impl WasmTy for Option<NoFunc> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(true, HeapType::NoFunc))
    }

    #[inline]
    fn compatible_with_store(&self, _store: &StoreOpaque) -> bool {
        true
    }

    #[inline]
    fn dynamic_concrete_type_check(
        &self,
        _: &StoreOpaque,
        nullable: bool,
        ty: &HeapType,
    ) -> Result<()> {
        if nullable {
            // `(ref null nofunc) <: (ref null $f)` for all function types `$f`.
            Ok(())
        } else {
            bail!("argument type mismatch: expected non-nullable (ref {ty}), found null reference")
        }
    }

    #[inline]
    fn store(self, _store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        ptr.write(ValRaw::funcref(ptr::null_mut()));
        Ok(())
    }

    #[inline]
    unsafe fn load(_store: &mut AutoAssertNoGc<'_>, _ptr: &ValRaw) -> Self {
        None
    }
}

unsafe impl WasmTy for Func {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Func))
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.store == store.id()
    }

    #[inline]
    fn dynamic_concrete_type_check(
        &self,
        store: &StoreOpaque,
        _nullable: bool,
        expected: &HeapType,
    ) -> Result<()> {
        let expected = expected.unwrap_concrete_func();
        self.ensure_matches_ty(store, expected)
            .context("argument type mismatch for reference to concrete type")
    }

    #[inline]
    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        let abi = self.vm_func_ref(store);
        ptr.write(ValRaw::funcref(abi.cast::<c_void>().as_ptr()));
        Ok(())
    }

    #[inline]
    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        let p = NonNull::new(ptr.get_funcref()).unwrap().cast();

        // SAFETY: it's an unsafe contract of `load` that it's only provided
        // valid wasm values owned by `store`.
        unsafe { Func::from_vm_func_ref(store.id(), p) }
    }
}

unsafe impl WasmTy for Option<Func> {
    #[inline]
    fn valtype() -> ValType {
        ValType::FUNCREF
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        if let Some(f) = self {
            f.compatible_with_store(store)
        } else {
            true
        }
    }

    fn dynamic_concrete_type_check(
        &self,
        store: &StoreOpaque,
        nullable: bool,
        expected: &HeapType,
    ) -> Result<()> {
        if let Some(f) = self {
            let expected = expected.unwrap_concrete_func();
            f.ensure_matches_ty(store, expected)
                .context("argument type mismatch for reference to concrete type")
        } else if nullable {
            Ok(())
        } else {
            bail!(
                "argument type mismatch: expected non-nullable (ref {expected}), found null reference"
            )
        }
    }

    #[inline]
    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        let raw = if let Some(f) = self {
            f.vm_func_ref(store).as_ptr()
        } else {
            ptr::null_mut()
        };
        ptr.write(ValRaw::funcref(raw.cast::<c_void>()));
        Ok(())
    }

    #[inline]
    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        let ptr = NonNull::new(ptr.get_funcref())?.cast();

        // SAFETY: it's an unsafe contract of `load` that it's only provided
        // valid wasm values owned by `store`.
        unsafe { Some(Func::from_vm_func_ref(store.id(), ptr)) }
    }
}

/// A trait used for [`Func::typed`] and with [`TypedFunc`] to represent the set of
/// parameters for wasm functions.
///
/// This is implemented for bare types that can be passed to wasm as well as
/// tuples of those types.
pub unsafe trait WasmParams: Send {
    #[doc(hidden)]
    type ValRawStorage: Copy;

    #[doc(hidden)]
    fn typecheck(
        engine: &Engine,
        params: impl ExactSizeIterator<Item = crate::ValType>,
        position: TypeCheckPosition,
    ) -> Result<()>;

    #[doc(hidden)]
    fn vmgcref_pointing_to_object_count(&self) -> usize;

    #[doc(hidden)]
    fn store(
        self,
        store: &mut AutoAssertNoGc<'_>,
        func_ty: &FuncType,
        dst: &mut MaybeUninit<Self::ValRawStorage>,
    ) -> Result<()>;
}

// Forward an impl from `T` to `(T,)` for convenience if there's only one
// parameter.
unsafe impl<T> WasmParams for T
where
    T: WasmTy,
{
    type ValRawStorage = <(T,) as WasmParams>::ValRawStorage;

    fn typecheck(
        engine: &Engine,
        params: impl ExactSizeIterator<Item = crate::ValType>,
        position: TypeCheckPosition,
    ) -> Result<()> {
        <(T,) as WasmParams>::typecheck(engine, params, position)
    }

    #[inline]
    fn vmgcref_pointing_to_object_count(&self) -> usize {
        T::is_vmgcref_and_points_to_object(self) as usize
    }

    #[inline]
    fn store(
        self,
        store: &mut AutoAssertNoGc<'_>,
        func_ty: &FuncType,
        dst: &mut MaybeUninit<Self::ValRawStorage>,
    ) -> Result<()> {
        <(T,) as WasmParams>::store((self,), store, func_ty, dst)
    }
}

macro_rules! impl_wasm_params {
    ($n:tt $($t:ident)*) => {
        #[allow(non_snake_case, reason = "macro-generated code")]
        unsafe impl<$($t: WasmTy,)*> WasmParams for ($($t,)*) {
            type ValRawStorage = [ValRaw; $n];

            fn typecheck(
                _engine: &Engine,
                mut params: impl ExactSizeIterator<Item = crate::ValType>,
                _position: TypeCheckPosition,
            ) -> Result<()> {
                let mut _n = 0;

                $(
                    match params.next() {
                        Some(t) => {
                            _n += 1;
                            $t::typecheck(_engine, t, _position)?
                        },
                        None => bail!("expected {} types, found {}", $n, params.len() + _n),
                    }
                )*

                match params.next() {
                    None => Ok(()),
                    Some(_) => {
                        _n += 1;
                        bail!("expected {} types, found {}", $n, params.len() + _n)
                    },
                }
            }

            #[inline]
            fn vmgcref_pointing_to_object_count(&self) -> usize {
                let ($($t,)*) = self;
                0 $(
                    + $t.is_vmgcref_and_points_to_object() as usize
                )*
            }


            #[inline]
            fn store(
                self,
                _store: &mut AutoAssertNoGc<'_>,
                _func_ty: &FuncType,
                _ptr: &mut MaybeUninit<Self::ValRawStorage>,
            ) -> Result<()> {
                let ($($t,)*) = self;

                let mut _i = 0;
                $(
                    if !$t.compatible_with_store(_store) {
                        bail!("attempt to pass cross-`Store` value to Wasm as function argument");
                    }

                    if $t::valtype().is_ref() {
                        let param_ty = _func_ty.param(_i).unwrap();
                        let ref_ty = param_ty.unwrap_ref();
                        let heap_ty = ref_ty.heap_type();
                        if heap_ty.is_concrete() {
                            $t.dynamic_concrete_type_check(_store, ref_ty.is_nullable(), heap_ty)?;
                        }
                    }

                    let dst = map_maybe_uninit!(_ptr[_i]);
                    $t.store(_store, dst)?;

                    _i += 1;
                )*
                Ok(())
            }
        }
    };
}

for_each_function_signature!(impl_wasm_params);

/// A trait used for [`Func::typed`] and with [`TypedFunc`] to represent the set of
/// results for wasm functions.
pub unsafe trait WasmResults: WasmParams {
    #[doc(hidden)]
    unsafe fn load(store: &mut AutoAssertNoGc<'_>, abi: &Self::ValRawStorage) -> Self;
}

// Forwards from a bare type `T` to the 1-tuple type `(T,)`
unsafe impl<T: WasmTy> WasmResults for T {
    unsafe fn load(store: &mut AutoAssertNoGc<'_>, abi: &Self::ValRawStorage) -> Self {
        // SAFETY: the one-element tuple and single-type impls behave the same
        // way.
        unsafe { <(T,) as WasmResults>::load(store, abi).0 }
    }
}

macro_rules! impl_wasm_results {
    ($n:tt $($t:ident)*) => {
        #[allow(non_snake_case, reason = "macro-generated code")]
        unsafe impl<$($t: WasmTy,)*> WasmResults for ($($t,)*) {
            unsafe fn load(_store: &mut AutoAssertNoGc<'_>, abi: &Self::ValRawStorage) -> Self {
                let [$($t,)*] = abi;

                (
                    // SAFETY: this is forwarding the unsafe contract of the outer
                    // function to the inner functions here.
                    $(unsafe { $t::load(_store, $t) },)*
                )
            }
        }
    };
}

for_each_function_signature!(impl_wasm_results);
