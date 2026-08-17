//! Working with GC `eqref`s.

use crate::{
    AnyRef, ArrayRef, ArrayType, AsContext, GcRefImpl, GcRootIndex, HeapType, I31, ManuallyRooted,
    RefType, Rooted, StructRef, StructType, ValRaw, ValType, WasmTy,
    prelude::*,
    runtime::vm::VMGcRef,
    store::{AutoAssertNoGc, StoreOpaque},
};
use core::mem::{self, MaybeUninit};
use wasmtime_environ::VMGcKind;

/// A reference to a GC-managed object that can be tested for equality.
///
/// The WebAssembly reference types that can be tested for equality, and
/// therefore are `eqref`s, include `structref`s, `arrayref`s, and
/// `i31ref`s. `funcref`s, `exnref`s, and `externref`s cannot be tested for
/// equality by Wasm, and are not `eqref`s.
///
/// Use the [`Rooted::ref_eq`][Rooted::ref_eq] method to actually test two
/// references for equality.
///
/// Like all WebAssembly references, these are opaque to and unforgeable by
/// Wasm: they cannot be faked and Wasm cannot, for example, cast the integer
/// `0x12345678` into a reference, pretend it is a valid `eqref`, and trick the
/// host into dereferencing it and segfaulting or worse.
///
/// Note that you can also use `Rooted<EqRef>` and `ManuallyRooted<EqRef>` as a
/// type parameter with [`Func::typed`][crate::Func::typed]- and
/// [`Func::wrap`][crate::Func::wrap]-style APIs.
///
/// # Example
///
/// ```
/// use wasmtime::*;
///
/// # fn foo() -> Result<()> {
/// let mut config = Config::new();
/// config.wasm_function_references(true);
/// config.wasm_gc(true);
///
/// let engine = Engine::new(&config)?;
/// let mut store = Store::new(&engine, ());
///
/// // Define a module that exports a function that returns a new `eqref` each
/// // time it is invoked.
/// let module = Module::new(&engine, r#"
///     (module
///         (global $g (mut i32) (i32.const 0))
///         (func (export "new-eqref") (result (ref eq))
///             ;; Increment $g.
///             global.get $g
///             i32.const 1
///             i32.add
///             global.set $g
///
///             ;; Create an `i31ref`, which is a kind of `eqref`, from $g.
///             global.get $g
///             ref.i31
///         )
///     )
/// "#)?;
///
/// // Instantiate the module.
/// let instance = Instance::new(&mut store, &module, &[])?;
///
/// // Get the exported function.
/// let new_eqref = instance.get_typed_func::<(), Rooted<EqRef>>(&mut store, "new-eqref")?;
///
/// {
///     let mut scope = RootScope::new(&mut store);
///
///     // Call the function to get an `eqref`.
///     let x = new_eqref.call(&mut scope, ())?;
///
///     // `x` is equal to itself!
///     assert!(Rooted::ref_eq(&scope, &x, &x)?);
///
///     // Call the function again to get a new, different `eqref`.
///     let y = new_eqref.call(&mut scope, ())?;
///
///     // `x` is not equal to `y`!
///     assert!(!Rooted::ref_eq(&scope, &x, &y)?);
/// }
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
#[derive(Debug)]
#[repr(transparent)]
pub struct EqRef {
    pub(super) inner: GcRootIndex,
}

impl From<Rooted<StructRef>> for Rooted<EqRef> {
    #[inline]
    fn from(s: Rooted<StructRef>) -> Self {
        s.to_eqref()
    }
}

impl From<ManuallyRooted<StructRef>> for ManuallyRooted<EqRef> {
    #[inline]
    fn from(s: ManuallyRooted<StructRef>) -> Self {
        s.to_eqref()
    }
}

impl From<Rooted<ArrayRef>> for Rooted<EqRef> {
    #[inline]
    fn from(s: Rooted<ArrayRef>) -> Self {
        s.to_eqref()
    }
}

impl From<ManuallyRooted<ArrayRef>> for ManuallyRooted<EqRef> {
    #[inline]
    fn from(s: ManuallyRooted<ArrayRef>) -> Self {
        s.to_eqref()
    }
}

unsafe impl GcRefImpl for EqRef {
    fn transmute_ref(index: &GcRootIndex) -> &Self {
        // Safety: `EqRef` is a newtype of a `GcRootIndex`.
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

impl Rooted<EqRef> {
    /// Upcast this `eqref` into an `anyref`.
    #[inline]
    pub fn to_anyref(self) -> Rooted<AnyRef> {
        self.unchecked_cast()
    }
}

impl ManuallyRooted<EqRef> {
    /// Upcast this `eqref` into an `anyref`.
    #[inline]
    pub fn to_anyref(self) -> ManuallyRooted<AnyRef> {
        self.unchecked_cast()
    }
}

impl EqRef {
    /// Create a new `Rooted<AnyRef>` from the given GC reference.
    ///
    /// `gc_ref` should point to a valid `anyref` and should belong to the
    /// store's GC heap. Failure to uphold these invariants is memory safe but
    /// will lead to general incorrectness such as panics or wrong results.
    pub(crate) fn from_cloned_gc_ref(
        store: &mut AutoAssertNoGc<'_>,
        gc_ref: VMGcRef,
    ) -> Rooted<Self> {
        debug_assert!(
            gc_ref.is_i31()
                || store
                    .unwrap_gc_store()
                    .header(&gc_ref)
                    .kind()
                    .matches(VMGcKind::EqRef)
        );
        Rooted::new(store, gc_ref)
    }

    #[inline]
    pub(crate) fn comes_from_same_store(&self, store: &StoreOpaque) -> bool {
        self.inner.comes_from_same_store(store)
    }

    /// Get the type of this reference.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn ty(&self, store: impl AsContext) -> Result<HeapType> {
        self._ty(store.as_context().0)
    }

    pub(crate) fn _ty(&self, store: &StoreOpaque) -> Result<HeapType> {
        let gc_ref = self.inner.try_gc_ref(store)?;
        if gc_ref.is_i31() {
            return Ok(HeapType::I31);
        }

        let header = store.gc_store()?.header(gc_ref);

        if header.kind().matches(VMGcKind::StructRef) {
            return Ok(HeapType::ConcreteStruct(
                StructType::from_shared_type_index(store.engine(), header.ty().unwrap()),
            ));
        }

        if header.kind().matches(VMGcKind::ArrayRef) {
            return Ok(HeapType::ConcreteArray(ArrayType::from_shared_type_index(
                store.engine(),
                header.ty().unwrap(),
            )));
        }

        unreachable!("no other kinds of `eqref`s")
    }

    /// Does this `eqref` match the given type?
    ///
    /// That is, is this object's type a subtype of the given type?
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn matches_ty(&self, store: impl AsContext, ty: &HeapType) -> Result<bool> {
        self._matches_ty(store.as_context().0, ty)
    }

    pub(crate) fn _matches_ty(&self, store: &StoreOpaque, ty: &HeapType) -> Result<bool> {
        assert!(self.comes_from_same_store(store));
        Ok(self._ty(store)?.matches(ty))
    }

    pub(crate) fn ensure_matches_ty(&self, store: &StoreOpaque, ty: &HeapType) -> Result<()> {
        if !self.comes_from_same_store(store) {
            bail!("function used with wrong store");
        }
        if self._matches_ty(store, ty)? {
            Ok(())
        } else {
            let actual_ty = self._ty(store)?;
            bail!("type mismatch: expected `(ref {ty})`, found `(ref {actual_ty})`")
        }
    }

    /// Is this `eqref` an `i31`?
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn is_i31(&self, store: impl AsContext) -> Result<bool> {
        self._is_i31(store.as_context().0)
    }

    pub(crate) fn _is_i31(&self, store: &StoreOpaque) -> Result<bool> {
        assert!(self.comes_from_same_store(store));
        let gc_ref = self.inner.try_gc_ref(store)?;
        Ok(gc_ref.is_i31())
    }

    /// Downcast this `eqref` to an `i31`.
    ///
    /// If this `eqref` is an `i31`, then `Some(_)` is returned.
    ///
    /// If this `eqref` is not an `i31`, then `None` is returned.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn as_i31(&self, store: impl AsContext) -> Result<Option<I31>> {
        self._as_i31(store.as_context().0)
    }

    pub(crate) fn _as_i31(&self, store: &StoreOpaque) -> Result<Option<I31>> {
        assert!(self.comes_from_same_store(store));
        let gc_ref = self.inner.try_gc_ref(store)?;
        Ok(gc_ref.as_i31().map(Into::into))
    }

    /// Downcast this `eqref` to an `i31`, panicking if this `eqref` is not an
    /// `i31`.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store, or if
    /// this `eqref` is not an `i31`.
    pub fn unwrap_i31(&self, store: impl AsContext) -> Result<I31> {
        Ok(self.as_i31(store)?.expect("EqRef::unwrap_i31 on non-i31"))
    }

    /// Is this `eqref` a `structref`?
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn is_struct(&self, store: impl AsContext) -> Result<bool> {
        self._is_struct(store.as_context().0)
    }

    pub(crate) fn _is_struct(&self, store: &StoreOpaque) -> Result<bool> {
        let gc_ref = self.inner.try_gc_ref(store)?;
        Ok(!gc_ref.is_i31() && store.gc_store()?.kind(gc_ref).matches(VMGcKind::StructRef))
    }

    /// Downcast this `eqref` to a `structref`.
    ///
    /// If this `eqref` is a `structref`, then `Some(_)` is returned.
    ///
    /// If this `eqref` is not a `structref`, then `None` is returned.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn as_struct(&self, store: impl AsContext) -> Result<Option<Rooted<StructRef>>> {
        self._as_struct(store.as_context().0)
    }

    pub(crate) fn _as_struct(&self, store: &StoreOpaque) -> Result<Option<Rooted<StructRef>>> {
        if self._is_struct(store)? {
            Ok(Some(Rooted::from_gc_root_index(self.inner)))
        } else {
            Ok(None)
        }
    }

    /// Downcast this `eqref` to a `structref`, panicking if this `eqref` is
    /// not a `structref`.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store, or if
    /// this `eqref` is not a `struct`.
    pub fn unwrap_struct(&self, store: impl AsContext) -> Result<Rooted<StructRef>> {
        self._unwrap_struct(store.as_context().0)
    }

    pub(crate) fn _unwrap_struct(&self, store: &StoreOpaque) -> Result<Rooted<StructRef>> {
        Ok(self
            ._as_struct(store)?
            .expect("EqRef::unwrap_struct on non-structref"))
    }

    /// Is this `eqref` an `arrayref`?
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn is_array(&self, store: impl AsContext) -> Result<bool> {
        self._is_array(store.as_context().0)
    }

    pub(crate) fn _is_array(&self, store: &StoreOpaque) -> Result<bool> {
        let gc_ref = self.inner.try_gc_ref(store)?;
        Ok(!gc_ref.is_i31() && store.gc_store()?.kind(gc_ref).matches(VMGcKind::ArrayRef))
    }

    /// Downcast this `eqref` to an `arrayref`.
    ///
    /// If this `eqref` is an `arrayref`, then `Some(_)` is returned.
    ///
    /// If this `eqref` is not an `arrayref`, then `None` is returned.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn as_array(&self, store: impl AsContext) -> Result<Option<Rooted<ArrayRef>>> {
        self._as_array(store.as_context().0)
    }

    pub(crate) fn _as_array(&self, store: &StoreOpaque) -> Result<Option<Rooted<ArrayRef>>> {
        if self._is_array(store)? {
            Ok(Some(Rooted::from_gc_root_index(self.inner)))
        } else {
            Ok(None)
        }
    }

    /// Downcast this `eqref` to an `arrayref`, panicking if this `eqref` is
    /// not an `arrayref`.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store, or if
    /// this `eqref` is not an `array`.
    pub fn unwrap_array(&self, store: impl AsContext) -> Result<Rooted<ArrayRef>> {
        self._unwrap_array(store.as_context().0)
    }

    pub(crate) fn _unwrap_array(&self, store: &StoreOpaque) -> Result<Rooted<ArrayRef>> {
        Ok(self
            ._as_array(store)?
            .expect("EqRef::unwrap_array on non-arrayref"))
    }
}

unsafe impl WasmTy for Rooted<EqRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Eq))
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.comes_from_same_store(store)
    }

    #[inline]
    fn dynamic_concrete_type_check(
        &self,
        store: &StoreOpaque,
        _nullable: bool,
        ty: &HeapType,
    ) -> Result<()> {
        self.ensure_matches_ty(store, ty)
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        self.wasm_ty_store(store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        Self::wasm_ty_load(store, ptr.get_anyref(), EqRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<Rooted<EqRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::EQREF
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.map_or(true, |x| x.comes_from_same_store(store))
    }

    #[inline]
    fn dynamic_concrete_type_check(
        &self,
        store: &StoreOpaque,
        nullable: bool,
        ty: &HeapType,
    ) -> Result<()> {
        match self {
            Some(s) => Rooted::<EqRef>::dynamic_concrete_type_check(s, store, nullable, ty),
            None => {
                ensure!(
                    nullable,
                    "expected a non-null reference, but found a null reference"
                );
                Ok(())
            }
        }
    }

    #[inline]
    fn is_vmgcref_and_points_to_object(&self) -> bool {
        self.is_some()
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        <Rooted<EqRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <Rooted<EqRef>>::wasm_ty_option_load(store, ptr.get_anyref(), EqRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for ManuallyRooted<EqRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Eq))
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.comes_from_same_store(store)
    }

    #[inline]
    fn dynamic_concrete_type_check(
        &self,
        store: &StoreOpaque,
        _: bool,
        ty: &HeapType,
    ) -> Result<()> {
        self.ensure_matches_ty(store, ty)
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        self.wasm_ty_store(store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        Self::wasm_ty_load(store, ptr.get_anyref(), EqRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<ManuallyRooted<EqRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::EQREF
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.as_ref()
            .map_or(true, |x| x.comes_from_same_store(store))
    }

    #[inline]
    fn dynamic_concrete_type_check(
        &self,
        store: &StoreOpaque,
        nullable: bool,
        ty: &HeapType,
    ) -> Result<()> {
        match self {
            Some(s) => ManuallyRooted::<EqRef>::dynamic_concrete_type_check(s, store, nullable, ty),
            None => {
                ensure!(
                    nullable,
                    "expected a non-null reference, but found a null reference"
                );
                Ok(())
            }
        }
    }

    #[inline]
    fn is_vmgcref_and_points_to_object(&self) -> bool {
        self.is_some()
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        <ManuallyRooted<EqRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <ManuallyRooted<EqRef>>::wasm_ty_option_load(
            store,
            ptr.get_anyref(),
            EqRef::from_cloned_gc_ref,
        )
    }
}
