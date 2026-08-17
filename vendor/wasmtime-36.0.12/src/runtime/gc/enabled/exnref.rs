//! Implementation of `exnref` in Wasmtime.

use crate::runtime::vm::VMGcRef;
use crate::store::StoreId;
use crate::vm::{VMExnRef, VMGcHeader};
use crate::{
    AsContext, AsContextMut, GcRefImpl, GcRootIndex, HeapType, ManuallyRooted, RefType, Result,
    Rooted, Val, ValRaw, ValType, WasmTy,
    store::{AutoAssertNoGc, StoreOpaque},
};
use crate::{ExnType, FieldType, GcHeapOutOfMemory, StoreContextMut, Tag, prelude::*};
use core::mem;
use core::mem::MaybeUninit;
use wasmtime_environ::{GcExceptionLayout, GcLayout, VMGcKind, VMSharedTypeIndex};

/// An allocator for a particular Wasm GC exception type.
///
/// Every `ExnRefPre` is associated with a particular
/// [`Store`][crate::Store] and a particular
/// [ExnType][crate::ExnType].
///
/// Reusing an allocator across many allocations amortizes some
/// per-type runtime overheads inside Wasmtime. An `ExnRefPre` is to
/// `ExnRef`s as an `InstancePre` is to `Instance`s.
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
/// // Define a exn type.
/// let exn_ty = ExnType::new(
///    store.engine(),
///    [ValType::I32],
/// )?;
///
/// // Create an allocator for the exn type.
/// let allocator = ExnRefPre::new(&mut store, exn_ty.clone());
///
/// // Create a tag instance to associate with our exception objects.
/// let tag = Tag::new(&mut store, &exn_ty.tag_type()).unwrap();
///
/// {
///     let mut scope = RootScope::new(&mut store);
///
///     // Allocate a bunch of instances of our exception type using the same
///     // allocator! This is faster than creating a new allocator for each
///     // instance we want to allocate.
///     for i in 0..10 {
///         ExnRef::new(&mut scope, &allocator, &tag, &[Val::I32(i)])?;
///     }
/// }
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
pub struct ExnRefPre {
    store_id: StoreId,
    ty: ExnType,
}

impl ExnRefPre {
    /// Create a new `ExnRefPre` that is associated with the given store
    /// and type.
    pub fn new(mut store: impl AsContextMut, ty: ExnType) -> Self {
        Self::_new(store.as_context_mut().0, ty)
    }

    pub(crate) fn _new(store: &mut StoreOpaque, ty: ExnType) -> Self {
        store.insert_gc_host_alloc_type(ty.registered_type().clone());
        let store_id = store.id();

        ExnRefPre { store_id, ty }
    }

    pub(crate) fn layout(&self) -> &GcExceptionLayout {
        self.ty
            .registered_type()
            .layout()
            .expect("exn types have a layout")
            .unwrap_exception()
    }

    pub(crate) fn type_index(&self) -> VMSharedTypeIndex {
        self.ty.registered_type().index()
    }
}

/// An `exnref` GC reference.
///
/// The `ExnRef` type represents WebAssembly `exnref` values. These
/// are references to exception objects created either by catching a
/// thrown exception in WebAssembly with a `catch_ref` clause of a
/// `try_table`, or by allocating via the host API.
///
/// Note that you can also use `Rooted<ExnRef>` and `ManuallyRooted<ExnRef>` as
/// a type parameter with [`Func::typed`][crate::Func::typed]- and
/// [`Func::wrap`][crate::Func::wrap]-style APIs.
#[derive(Debug)]
#[repr(transparent)]
pub struct ExnRef {
    pub(super) inner: GcRootIndex,
}

unsafe impl GcRefImpl for ExnRef {
    fn transmute_ref(index: &GcRootIndex) -> &Self {
        // Safety: `ExnRef` is a newtype of a `GcRootIndex`.
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

impl ExnRef {
    /// Creates a new strongly-owned [`ExnRef`] from the raw value provided.
    ///
    /// This is intended to be used in conjunction with [`Func::new_unchecked`],
    /// [`Func::call_unchecked`], and [`ValRaw`] with its `anyref` field.
    ///
    /// This function assumes that `raw` is an `exnref` value which is currently
    /// rooted within the [`Store`].
    ///
    /// # Correctness
    ///
    /// This function is tricky to get right because `raw` not only must be a
    /// valid `exnref` value produced prior by [`ExnRef::to_raw`] but it must
    /// also be correctly rooted within the store. When arguments are provided
    /// to a callback with [`Func::new_unchecked`], for example, or returned via
    /// [`Func::call_unchecked`], if a GC is performed within the store then
    /// floating `exnref` values are not rooted and will be GC'd, meaning that
    /// this function will no longer be correct to call with the values cleaned
    /// up. This function must be invoked *before* possible GC operations can
    /// happen (such as calling Wasm).
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
    pub fn from_raw(mut store: impl AsContextMut, raw: u32) -> Option<Rooted<Self>> {
        let mut store = AutoAssertNoGc::new(store.as_context_mut().0);
        Self::_from_raw(&mut store, raw)
    }

    // (Not actually memory unsafe since we have indexed GC heaps.)
    pub(crate) fn _from_raw(store: &mut AutoAssertNoGc, raw: u32) -> Option<Rooted<Self>> {
        let gc_ref = VMGcRef::from_raw_u32(raw)?;
        let gc_ref = store.unwrap_gc_store_mut().clone_gc_ref(&gc_ref);
        Some(Self::from_cloned_gc_ref(store, gc_ref))
    }

    /// Synchronously allocate a new exception object and get a
    /// reference to it.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for
    /// allocating this new exception object, then this method will
    /// automatically trigger a synchronous collection in an attempt
    /// to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If the given `fields` values' types do not match the field
    /// types of the `allocator`'s exception type, an error is
    /// returned.
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<()>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// # Panics
    ///
    /// Panics if your engine is configured for async; use
    /// [`ExnRef::new_async`][crate::ExnRef::new_async] to perform
    /// synchronous allocation instead.
    ///
    /// Panics if the allocator, or any of the field values, is not associated
    /// with the given store.
    pub fn new(
        mut store: impl AsContextMut,
        allocator: &ExnRefPre,
        tag: &Tag,
        fields: &[Val],
    ) -> Result<Rooted<ExnRef>> {
        Self::_new(store.as_context_mut().0, allocator, tag, fields)
    }

    pub(crate) fn _new(
        store: &mut StoreOpaque,
        allocator: &ExnRefPre,
        tag: &Tag,
        fields: &[Val],
    ) -> Result<Rooted<ExnRef>> {
        assert!(
            !store.async_support(),
            "use `ExnRef::new_async` with asynchronous stores"
        );
        Self::type_check_tag_and_fields(store, allocator, tag, fields)?;
        store.retry_after_gc((), |store, ()| {
            Self::new_unchecked(store, allocator, tag, fields)
        })
    }

    /// Asynchronously allocate a new exception object and get a
    /// reference to it.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for allocating this
    /// new exn, then this method will automatically trigger a synchronous
    /// collection in an attempt to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If the given `fields` values' types do not match the field
    /// types of the `allocator`'s exception type, an error is
    /// returned.
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<()>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// # Panics
    ///
    /// Panics if your engine is not configured for async; use
    /// [`ExnRef::new`][crate::ExnRef::new] to perform synchronous
    /// allocation instead.
    ///
    /// Panics if the allocator, or any of the field values, is not associated
    /// with the given store.
    #[cfg(feature = "async")]
    pub async fn new_async(
        mut store: impl AsContextMut,
        allocator: &ExnRefPre,
        tag: &Tag,
        fields: &[Val],
    ) -> Result<Rooted<ExnRef>> {
        Self::_new_async(store.as_context_mut().0, allocator, tag, fields).await
    }

    #[cfg(feature = "async")]
    pub(crate) async fn _new_async(
        store: &mut StoreOpaque,
        allocator: &ExnRefPre,
        tag: &Tag,
        fields: &[Val],
    ) -> Result<Rooted<ExnRef>> {
        assert!(
            store.async_support(),
            "use `ExnRef::new` with synchronous stores"
        );
        Self::type_check_tag_and_fields(store, allocator, tag, fields)?;
        store
            .retry_after_gc_async((), |store, ()| {
                Self::new_unchecked(store, allocator, tag, fields)
            })
            .await
    }

    /// Type check the tag instance and field values before allocating
    /// a new exception object.
    fn type_check_tag_and_fields(
        store: &mut StoreOpaque,
        allocator: &ExnRefPre,
        tag: &Tag,
        fields: &[Val],
    ) -> Result<(), Error> {
        assert!(
            tag.comes_from_same_store(store),
            "tag comes from the wrong store"
        );
        ensure!(
            tag.wasmtime_ty(store).signature.unwrap_engine_type_index()
                == allocator.ty.tag_type().ty().type_index(),
            "incorrect signature for tag when creating exception object"
        );
        let expected_len = allocator.ty.fields().len();
        let actual_len = fields.len();
        ensure!(
            actual_len == expected_len,
            "expected {expected_len} fields, got {actual_len}"
        );
        for (ty, val) in allocator.ty.fields().zip(fields) {
            assert!(
                val.comes_from_same_store(store),
                "field value comes from the wrong store",
            );
            let ty = ty.element_type().unpack();
            val.ensure_matches_ty(store, ty)
                .context("field type mismatch")?;
        }
        Ok(())
    }

    /// Given that the field values have already been type checked, allocate a
    /// new exn.
    ///
    /// Does not attempt GC+retry on OOM, that is the caller's responsibility.
    fn new_unchecked(
        store: &mut StoreOpaque,
        allocator: &ExnRefPre,
        tag: &Tag,
        fields: &[Val],
    ) -> Result<Rooted<ExnRef>> {
        assert_eq!(
            store.id(),
            allocator.store_id,
            "attempted to use a `ExnRefPre` with the wrong store"
        );

        // Allocate the exn and write each field value into the appropriate
        // offset.
        let exnref = store
            .gc_store_mut()?
            .alloc_uninit_exn(allocator.type_index(), &allocator.layout())
            .context("unrecoverable error when allocating new `exnref`")?
            .map_err(|n| GcHeapOutOfMemory::new((), n))?;

        // From this point on, if we get any errors, then the exn is not
        // fully initialized, so we need to eagerly deallocate it before the
        // next GC where the collector might try to interpret one of the
        // uninitialized fields as a GC reference.
        let mut store = AutoAssertNoGc::new(store);
        match (|| {
            let (instance, index) = tag.to_raw_indices();
            exnref.initialize_tag(&mut store, allocator.layout(), instance, index)?;
            for (index, (ty, val)) in allocator.ty.fields().zip(fields).enumerate() {
                exnref.initialize_field(
                    &mut store,
                    allocator.layout(),
                    ty.element_type(),
                    index,
                    *val,
                )?;
            }
            Ok(())
        })() {
            Ok(()) => Ok(Rooted::new(&mut store, exnref.into())),
            Err(e) => {
                store.gc_store_mut()?.dealloc_uninit_exn(exnref);
                Err(e)
            }
        }
    }

    pub(crate) fn type_index(&self, store: &StoreOpaque) -> Result<VMSharedTypeIndex> {
        let gc_ref = self.inner.try_gc_ref(store)?;
        let header = store.gc_store()?.header(gc_ref);
        debug_assert!(header.kind().matches(VMGcKind::ExnRef));
        Ok(header.ty().expect("exnrefs should have concrete types"))
    }

    /// Create a new `Rooted<ExnRef>` from the given GC reference.
    ///
    /// `gc_ref` should point to a valid `exnref` and should belong to
    /// the store's GC heap. Failure to uphold these invariants is
    /// memory safe but will lead to general incorrectness such as
    /// panics or wrong results.
    pub(crate) fn from_cloned_gc_ref(
        store: &mut AutoAssertNoGc<'_>,
        gc_ref: VMGcRef,
    ) -> Rooted<Self> {
        debug_assert!(
            store
                .unwrap_gc_store()
                .header(&gc_ref)
                .kind()
                .matches(VMGcKind::ExnRef)
        );
        Rooted::new(store, gc_ref)
    }

    #[inline]
    pub(crate) fn comes_from_same_store(&self, store: &StoreOpaque) -> bool {
        self.inner.comes_from_same_store(store)
    }

    /// Converts this [`ExnRef`] to a raw value suitable to store within a
    /// [`ValRaw`].
    ///
    /// Returns an error if this `exnref` has been unrooted.
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

    pub(crate) fn _to_raw(&self, store: &mut AutoAssertNoGc<'_>) -> Result<u32> {
        let gc_ref = self.inner.try_clone_gc_ref(store)?;
        let raw = if gc_ref.is_i31() {
            gc_ref.as_raw_non_zero_u32()
        } else {
            store.gc_store_mut()?.expose_gc_ref_to_wasm(gc_ref)
        };
        Ok(raw.get())
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
    pub fn ty(&self, store: impl AsContext) -> Result<ExnType> {
        self._ty(store.as_context().0)
    }

    pub(crate) fn _ty(&self, store: &StoreOpaque) -> Result<ExnType> {
        assert!(self.comes_from_same_store(store));
        let index = self.type_index(store)?;
        Ok(ExnType::from_shared_type_index(store.engine(), index))
    }

    /// Does this `exnref` match the given type?
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
        Ok(HeapType::from(self._ty(store)?).matches(ty))
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

    /// Get the values of this exception object's fields.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn fields<'a, T: 'static>(
        &'a self,
        store: impl Into<StoreContextMut<'a, T>>,
    ) -> Result<impl ExactSizeIterator<Item = Val> + 'a> {
        self._fields(store.into().0)
    }

    pub(crate) fn _fields<'a>(
        &'a self,
        store: &'a mut StoreOpaque,
    ) -> Result<impl ExactSizeIterator<Item = Val> + 'a> {
        assert!(self.comes_from_same_store(store));
        let store = AutoAssertNoGc::new(store);

        let gc_ref = self.inner.try_gc_ref(&store)?;
        let header = store.gc_store()?.header(gc_ref);
        debug_assert!(header.kind().matches(VMGcKind::ExnRef));

        let index = header.ty().expect("exnrefs should have concrete types");
        let ty = ExnType::from_shared_type_index(store.engine(), index);
        let len = ty.fields().len();

        return Ok(Fields {
            exnref: self,
            store,
            index: 0,
            len,
        });

        struct Fields<'a, 'b> {
            exnref: &'a ExnRef,
            store: AutoAssertNoGc<'b>,
            index: usize,
            len: usize,
        }

        impl Iterator for Fields<'_, '_> {
            type Item = Val;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let i = self.index;
                debug_assert!(i <= self.len);
                if i >= self.len {
                    return None;
                }
                self.index += 1;
                Some(self.exnref._field(&mut self.store, i).unwrap())
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len - self.index;
                (len, Some(len))
            }
        }

        impl ExactSizeIterator for Fields<'_, '_> {
            #[inline]
            fn len(&self) -> usize {
                self.len - self.index
            }
        }
    }

    fn header<'a>(&self, store: &'a AutoAssertNoGc<'_>) -> Result<&'a VMGcHeader> {
        assert!(self.comes_from_same_store(&store));
        let gc_ref = self.inner.try_gc_ref(store)?;
        Ok(store.gc_store()?.header(gc_ref))
    }

    fn exnref<'a>(&self, store: &'a AutoAssertNoGc<'_>) -> Result<&'a VMExnRef> {
        assert!(self.comes_from_same_store(&store));
        let gc_ref = self.inner.try_gc_ref(store)?;
        debug_assert!(self.header(store)?.kind().matches(VMGcKind::ExnRef));
        Ok(gc_ref.as_exnref_unchecked())
    }

    fn layout(&self, store: &AutoAssertNoGc<'_>) -> Result<GcExceptionLayout> {
        assert!(self.comes_from_same_store(&store));
        let type_index = self.type_index(store)?;
        let layout = store
            .engine()
            .signatures()
            .layout(type_index)
            .expect("exn types should have GC layouts");
        match layout {
            GcLayout::Struct(_) => unreachable!(),
            GcLayout::Array(_) => unreachable!(),
            GcLayout::Exception(e) => Ok(e),
        }
    }

    fn field_ty(&self, store: &StoreOpaque, field: usize) -> Result<FieldType> {
        let ty = self._ty(store)?;
        match ty.field(field) {
            Some(f) => Ok(f),
            None => {
                let len = ty.fields().len();
                bail!("cannot access field {field}: exn only has {len} fields")
            }
        }
    }

    /// Get this exception object's `index`th field.
    ///
    /// # Errors
    ///
    /// Returns an `Err(_)` if the index is out of bounds or this reference has
    /// been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn field(&self, mut store: impl AsContextMut, index: usize) -> Result<Val> {
        let mut store = AutoAssertNoGc::new(store.as_context_mut().0);
        self._field(&mut store, index)
    }

    pub(crate) fn _field(&self, store: &mut AutoAssertNoGc<'_>, index: usize) -> Result<Val> {
        assert!(self.comes_from_same_store(store));
        let exnref = self.exnref(store)?.unchecked_copy();
        let field_ty = self.field_ty(store, index)?;
        let layout = self.layout(store)?;
        Ok(exnref.read_field(store, &layout, field_ty.element_type(), index))
    }

    /// Get this exception object's associated tag.
    ///
    /// # Errors
    ///
    /// Returns an `Err(_)` if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn tag(&self, mut store: impl AsContextMut) -> Result<Tag> {
        let mut store = AutoAssertNoGc::new(store.as_context_mut().0);
        assert!(self.comes_from_same_store(&store));
        let exnref = self.exnref(&store)?.unchecked_copy();
        let layout = self.layout(&store)?;
        let (instance, index) = exnref.tag(&mut store, &layout)?;
        Ok(Tag::from_raw_indices(&*store, instance, index))
    }
}

unsafe impl WasmTy for Rooted<ExnRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Exn))
    }

    #[inline]
    fn compatible_with_store(&self, store: &StoreOpaque) -> bool {
        self.comes_from_same_store(store)
    }

    #[inline]
    fn dynamic_concrete_type_check(
        &self,
        _store: &StoreOpaque,
        _nullable: bool,
        _ty: &HeapType,
    ) -> Result<()> {
        // Wasm can't specify a concrete exn type, so there are no
        // dynamic checks.
        Ok(())
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        self.wasm_ty_store(store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        Self::wasm_ty_load(store, ptr.get_anyref(), ExnRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<Rooted<ExnRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::EXNREF
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
            Some(a) => a.ensure_matches_ty(store, ty),
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
        <Rooted<ExnRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <Rooted<ExnRef>>::wasm_ty_option_load(store, ptr.get_anyref(), ExnRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for ManuallyRooted<ExnRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Exn))
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
        Self::wasm_ty_load(store, ptr.get_anyref(), ExnRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<ManuallyRooted<ExnRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::EXNREF
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
            Some(a) => a.ensure_matches_ty(store, ty),
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
        <ManuallyRooted<ExnRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <ManuallyRooted<ExnRef>>::wasm_ty_option_load(
            store,
            ptr.get_anyref(),
            ExnRef::from_cloned_gc_ref,
        )
    }
}
