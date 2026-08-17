//! Working with GC `struct` objects.

use crate::runtime::vm::VMGcRef;
use crate::store::StoreId;
use crate::vm::{VMGcHeader, VMStructRef};
use crate::{AnyRef, FieldType};
use crate::{
    AsContext, AsContextMut, EqRef, GcHeapOutOfMemory, GcRefImpl, GcRootIndex, HeapType,
    ManuallyRooted, RefType, Rooted, StructType, Val, ValRaw, ValType, WasmTy,
    prelude::*,
    store::{AutoAssertNoGc, StoreContextMut, StoreOpaque},
};
use core::mem::{self, MaybeUninit};
use wasmtime_environ::{GcLayout, GcStructLayout, VMGcKind, VMSharedTypeIndex};

/// An allocator for a particular Wasm GC struct type.
///
/// Every `StructRefPre` is associated with a particular
/// [`Store`][crate::Store] and a particular [StructType][crate::StructType].
///
/// Reusing an allocator across many allocations amortizes some per-type runtime
/// overheads inside Wasmtime. A `StructRefPre` is to `StructRef`s as an
/// `InstancePre` is to `Instance`s.
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
/// // Define a struct type.
/// let struct_ty = StructType::new(
///    store.engine(),
///    [FieldType::new(Mutability::Var, StorageType::I8)],
/// )?;
///
/// // Create an allocator for the struct type.
/// let allocator = StructRefPre::new(&mut store, struct_ty);
///
/// {
///     let mut scope = RootScope::new(&mut store);
///
///     // Allocate a bunch of instances of our struct type using the same
///     // allocator! This is faster than creating a new allocator for each
///     // instance we want to allocate.
///     for i in 0..10 {
///         StructRef::new(&mut scope, &allocator, &[Val::I32(i)])?;
///     }
/// }
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
pub struct StructRefPre {
    store_id: StoreId,
    ty: StructType,
}

impl StructRefPre {
    /// Create a new `StructRefPre` that is associated with the given store
    /// and type.
    pub fn new(mut store: impl AsContextMut, ty: StructType) -> Self {
        Self::_new(store.as_context_mut().0, ty)
    }

    pub(crate) fn _new(store: &mut StoreOpaque, ty: StructType) -> Self {
        store.insert_gc_host_alloc_type(ty.registered_type().clone());
        let store_id = store.id();

        StructRefPre { store_id, ty }
    }

    pub(crate) fn layout(&self) -> &GcStructLayout {
        self.ty
            .registered_type()
            .layout()
            .expect("struct types have a layout")
            .unwrap_struct()
    }

    pub(crate) fn type_index(&self) -> VMSharedTypeIndex {
        self.ty.registered_type().index()
    }
}

/// A reference to a GC-managed `struct` instance.
///
/// WebAssembly `struct`s are static, fixed-length, ordered sequences of
/// fields. Fields are named by index, not by identifier; in this way, they are
/// similar to Rust's tuples. Each field is mutable or constant and stores
/// unpacked [`Val`][crate::Val]s or packed 8-/16-bit integers.
///
/// Like all WebAssembly references, these are opaque and unforgeable to Wasm:
/// they cannot be faked and Wasm cannot, for example, cast the integer
/// `0x12345678` into a reference, pretend it is a valid `structref`, and trick
/// the host into dereferencing it and segfaulting or worse.
///
/// Note that you can also use `Rooted<StructRef>` and
/// `ManuallyRooted<StructRef>` as a type parameter with
/// [`Func::typed`][crate::Func::typed]- and
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
/// // Define a struct type.
/// let struct_ty = StructType::new(
///    store.engine(),
///    [FieldType::new(Mutability::Var, StorageType::I8)],
/// )?;
///
/// // Create an allocator for the struct type.
/// let allocator = StructRefPre::new(&mut store, struct_ty);
///
/// {
///     let mut scope = RootScope::new(&mut store);
///
///     // Allocate an instance of the struct type.
///     let my_struct = StructRef::new(&mut scope, &allocator, &[Val::I32(42)])?;
///
///     // That instance's field should have the expected value.
///     let val = my_struct.field(&mut scope, 0)?.unwrap_i32();
///     assert_eq!(val, 42);
///
///     // And we can update the field's value because it is a mutable field.
///     my_struct.set_field(&mut scope, 0, Val::I32(36))?;
///     let new_val = my_struct.field(&mut scope, 0)?.unwrap_i32();
///     assert_eq!(new_val, 36);
/// }
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
#[derive(Debug)]
#[repr(transparent)]
pub struct StructRef {
    pub(super) inner: GcRootIndex,
}

unsafe impl GcRefImpl for StructRef {
    fn transmute_ref(index: &GcRootIndex) -> &Self {
        // Safety: `StructRef` is a newtype of a `GcRootIndex`.
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

impl Rooted<StructRef> {
    /// Upcast this `structref` into an `anyref`.
    #[inline]
    pub fn to_anyref(self) -> Rooted<AnyRef> {
        self.unchecked_cast()
    }

    /// Upcast this `structref` into an `eqref`.
    #[inline]
    pub fn to_eqref(self) -> Rooted<EqRef> {
        self.unchecked_cast()
    }
}

impl ManuallyRooted<StructRef> {
    /// Upcast this `structref` into an `anyref`.
    #[inline]
    pub fn to_anyref(self) -> ManuallyRooted<AnyRef> {
        self.unchecked_cast()
    }

    /// Upcast this `structref` into an `eqref`.
    #[inline]
    pub fn to_eqref(self) -> ManuallyRooted<EqRef> {
        self.unchecked_cast()
    }
}

impl StructRef {
    /// Synchronously allocate a new `struct` and get a reference to it.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for allocating this
    /// new struct, then this method will automatically trigger a synchronous
    /// collection in an attempt to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If the given `fields` values' types do not match the field types of the
    /// `allocator`'s struct type, an error is returned.
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<()>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// # Panics
    ///
    /// Panics if your engine is configured for async; use
    /// [`StructRef::new_async`][crate::StructRef::new_async] to perform
    /// synchronous allocation instead.
    ///
    /// Panics if the allocator, or any of the field values, is not associated
    /// with the given store.
    pub fn new(
        mut store: impl AsContextMut,
        allocator: &StructRefPre,
        fields: &[Val],
    ) -> Result<Rooted<StructRef>> {
        Self::_new(store.as_context_mut().0, allocator, fields)
    }

    pub(crate) fn _new(
        store: &mut StoreOpaque,
        allocator: &StructRefPre,
        fields: &[Val],
    ) -> Result<Rooted<StructRef>> {
        assert!(
            !store.async_support(),
            "use `StructRef::new_async` with asynchronous stores"
        );
        Self::type_check_fields(store, allocator, fields)?;
        store.retry_after_gc((), |store, ()| {
            Self::new_unchecked(store, allocator, fields)
        })
    }

    /// Asynchronously allocate a new `struct` and get a reference to it.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for allocating this
    /// new struct, then this method will automatically trigger a synchronous
    /// collection in an attempt to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If the given `fields` values' types do not match the field types of the
    /// `allocator`'s struct type, an error is returned.
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<()>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// # Panics
    ///
    /// Panics if your engine is not configured for async; use
    /// [`StructRef::new`][crate::StructRef::new] to perform synchronous
    /// allocation instead.
    ///
    /// Panics if the allocator, or any of the field values, is not associated
    /// with the given store.
    #[cfg(feature = "async")]
    pub async fn new_async(
        mut store: impl AsContextMut,
        allocator: &StructRefPre,
        fields: &[Val],
    ) -> Result<Rooted<StructRef>> {
        Self::_new_async(store.as_context_mut().0, allocator, fields).await
    }

    #[cfg(feature = "async")]
    pub(crate) async fn _new_async(
        store: &mut StoreOpaque,
        allocator: &StructRefPre,
        fields: &[Val],
    ) -> Result<Rooted<StructRef>> {
        assert!(
            store.async_support(),
            "use `StructRef::new` with synchronous stores"
        );
        Self::type_check_fields(store, allocator, fields)?;
        store
            .retry_after_gc_async((), |store, ()| {
                Self::new_unchecked(store, allocator, fields)
            })
            .await
    }

    /// Like `Self::new` but caller's must ensure that if the store is
    /// configured for async, this is only ever called from on a fiber stack.
    pub(crate) unsafe fn new_maybe_async(
        store: &mut StoreOpaque,
        allocator: &StructRefPre,
        fields: &[Val],
    ) -> Result<Rooted<StructRef>> {
        Self::type_check_fields(store, allocator, fields)?;
        unsafe {
            store.retry_after_gc_maybe_async((), |store, ()| {
                Self::new_unchecked(store, allocator, fields)
            })
        }
    }

    /// Type check the field values before allocating a new struct.
    fn type_check_fields(
        store: &mut StoreOpaque,
        allocator: &StructRefPre,
        fields: &[Val],
    ) -> Result<(), Error> {
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
    /// new struct.
    ///
    /// Does not attempt GC+retry on OOM, that is the caller's responsibility.
    fn new_unchecked(
        store: &mut StoreOpaque,
        allocator: &StructRefPre,
        fields: &[Val],
    ) -> Result<Rooted<StructRef>> {
        assert_eq!(
            store.id(),
            allocator.store_id,
            "attempted to use a `StructRefPre` with the wrong store"
        );

        // Allocate the struct and write each field value into the appropriate
        // offset.
        let structref = store
            .gc_store_mut()?
            .alloc_uninit_struct(allocator.type_index(), &allocator.layout())
            .context("unrecoverable error when allocating new `structref`")?
            .map_err(|n| GcHeapOutOfMemory::new((), n))?;

        // From this point on, if we get any errors, then the struct is not
        // fully initialized, so we need to eagerly deallocate it before the
        // next GC where the collector might try to interpret one of the
        // uninitialized fields as a GC reference.
        let mut store = AutoAssertNoGc::new(store);
        match (|| {
            for (index, (ty, val)) in allocator.ty.fields().zip(fields).enumerate() {
                structref.initialize_field(
                    &mut store,
                    allocator.layout(),
                    ty.element_type(),
                    index,
                    *val,
                )?;
            }
            Ok(())
        })() {
            Ok(()) => Ok(Rooted::new(&mut store, structref.into())),
            Err(e) => {
                store.gc_store_mut()?.dealloc_uninit_struct(structref);
                Err(e)
            }
        }
    }

    #[inline]
    pub(crate) fn comes_from_same_store(&self, store: &StoreOpaque) -> bool {
        self.inner.comes_from_same_store(store)
    }

    /// Get this `structref`'s type.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn ty(&self, store: impl AsContext) -> Result<StructType> {
        self._ty(store.as_context().0)
    }

    pub(crate) fn _ty(&self, store: &StoreOpaque) -> Result<StructType> {
        assert!(self.comes_from_same_store(store));
        let index = self.type_index(store)?;
        Ok(StructType::from_shared_type_index(store.engine(), index))
    }

    /// Does this `structref` match the given type?
    ///
    /// That is, is this struct's type a subtype of the given type?
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store or if the
    /// type is not associated with the store's engine.
    pub fn matches_ty(&self, store: impl AsContext, ty: &StructType) -> Result<bool> {
        self._matches_ty(store.as_context().0, ty)
    }

    pub(crate) fn _matches_ty(&self, store: &StoreOpaque, ty: &StructType) -> Result<bool> {
        assert!(self.comes_from_same_store(store));
        Ok(self._ty(store)?.matches(ty))
    }

    pub(crate) fn ensure_matches_ty(&self, store: &StoreOpaque, ty: &StructType) -> Result<()> {
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

    /// Get the values of this struct's fields.
    ///
    /// Note that `i8` and `i16` field values are zero-extended into
    /// `Val::I32(_)`s.
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
        debug_assert!(header.kind().matches(VMGcKind::StructRef));

        let index = header.ty().expect("structrefs should have concrete types");
        let ty = StructType::from_shared_type_index(store.engine(), index);
        let len = ty.fields().len();

        return Ok(Fields {
            structref: self,
            store,
            index: 0,
            len,
        });

        struct Fields<'a, 'b> {
            structref: &'a StructRef,
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
                Some(self.structref._field(&mut self.store, i).unwrap())
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

    fn structref<'a>(&self, store: &'a AutoAssertNoGc<'_>) -> Result<&'a VMStructRef> {
        assert!(self.comes_from_same_store(&store));
        let gc_ref = self.inner.try_gc_ref(store)?;
        debug_assert!(self.header(store)?.kind().matches(VMGcKind::StructRef));
        Ok(gc_ref.as_structref_unchecked())
    }

    fn layout(&self, store: &AutoAssertNoGc<'_>) -> Result<GcStructLayout> {
        assert!(self.comes_from_same_store(&store));
        let type_index = self.type_index(store)?;
        let layout = store
            .engine()
            .signatures()
            .layout(type_index)
            .expect("struct types should have GC layouts");
        match layout {
            GcLayout::Struct(s) => Ok(s),
            GcLayout::Array(_) => unreachable!(),
            GcLayout::Exception(_) => unreachable!(),
        }
    }

    fn field_ty(&self, store: &StoreOpaque, field: usize) -> Result<FieldType> {
        let ty = self._ty(store)?;
        match ty.field(field) {
            Some(f) => Ok(f),
            None => {
                let len = ty.fields().len();
                bail!("cannot access field {field}: struct only has {len} fields")
            }
        }
    }

    /// Get this struct's `index`th field.
    ///
    /// Note that `i8` and `i16` field values are zero-extended into
    /// `Val::I32(_)`s.
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
        let structref = self.structref(store)?.unchecked_copy();
        let field_ty = self.field_ty(store, index)?;
        let layout = self.layout(store)?;
        Ok(structref.read_field(store, &layout, field_ty.element_type(), index))
    }

    /// Set this struct's `index`th field.
    ///
    /// # Errors
    ///
    /// Returns an error in the following scenarios:
    ///
    /// * When given a value of the wrong type, such as trying to set an `f32`
    ///   field to an `i64` value.
    ///
    /// * When the field is not mutable.
    ///
    /// * When this struct does not have an `index`th field, i.e. `index` is out
    ///   of bounds.
    ///
    /// * When `value` is a GC reference that has since been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn set_field(&self, mut store: impl AsContextMut, index: usize, value: Val) -> Result<()> {
        self._set_field(store.as_context_mut().0, index, value)
    }

    pub(crate) fn _set_field(
        &self,
        store: &mut StoreOpaque,
        index: usize,
        value: Val,
    ) -> Result<()> {
        assert!(self.comes_from_same_store(store));
        let mut store = AutoAssertNoGc::new(store);

        let field_ty = self.field_ty(&store, index)?;
        ensure!(
            field_ty.mutability().is_var(),
            "cannot set field {index}: field is not mutable"
        );

        value
            .ensure_matches_ty(&store, &field_ty.element_type().unpack())
            .with_context(|| format!("cannot set field {index}: type mismatch"))?;

        let layout = self.layout(&store)?;
        let structref = self.structref(&store)?.unchecked_copy();

        structref.write_field(&mut store, &layout, field_ty.element_type(), index, value)
    }

    pub(crate) fn type_index(&self, store: &StoreOpaque) -> Result<VMSharedTypeIndex> {
        let gc_ref = self.inner.try_gc_ref(store)?;
        let header = store.gc_store()?.header(gc_ref);
        debug_assert!(header.kind().matches(VMGcKind::StructRef));
        Ok(header.ty().expect("structrefs should have concrete types"))
    }

    /// Create a new `Rooted<StructRef>` from the given GC reference.
    ///
    /// `gc_ref` should point to a valid `structref` and should belong to the
    /// store's GC heap. Failure to uphold these invariants is memory safe but
    /// will lead to general incorrectness such as panics or wrong results.
    pub(crate) fn from_cloned_gc_ref(
        store: &mut AutoAssertNoGc<'_>,
        gc_ref: VMGcRef,
    ) -> Rooted<Self> {
        debug_assert!(gc_ref.is_structref(&*store.unwrap_gc_store().gc_heap));
        Rooted::new(store, gc_ref)
    }
}

unsafe impl WasmTy for Rooted<StructRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Struct))
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
        match ty {
            HeapType::Any | HeapType::Eq | HeapType::Struct => Ok(()),
            HeapType::ConcreteStruct(ty) => self.ensure_matches_ty(store, ty),

            HeapType::Extern
            | HeapType::NoExtern
            | HeapType::Func
            | HeapType::ConcreteFunc(_)
            | HeapType::NoFunc
            | HeapType::I31
            | HeapType::Array
            | HeapType::ConcreteArray(_)
            | HeapType::None
            | HeapType::NoCont
            | HeapType::Cont
            | HeapType::ConcreteCont(_)
            | HeapType::NoExn
            | HeapType::Exn
            | HeapType::ConcreteExn(_) => bail!(
                "type mismatch: expected `(ref {ty})`, got `(ref {})`",
                self._ty(store)?,
            ),
        }
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        self.wasm_ty_store(store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        Self::wasm_ty_load(store, ptr.get_anyref(), StructRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<Rooted<StructRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::STRUCTREF
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
            Some(s) => Rooted::<StructRef>::dynamic_concrete_type_check(s, store, nullable, ty),
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
        <Rooted<StructRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <Rooted<StructRef>>::wasm_ty_option_load(
            store,
            ptr.get_anyref(),
            StructRef::from_cloned_gc_ref,
        )
    }
}

unsafe impl WasmTy for ManuallyRooted<StructRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Struct))
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
        match ty {
            HeapType::Any | HeapType::Eq | HeapType::Struct => Ok(()),
            HeapType::ConcreteStruct(ty) => self.ensure_matches_ty(store, ty),

            HeapType::Extern
            | HeapType::NoExtern
            | HeapType::Func
            | HeapType::ConcreteFunc(_)
            | HeapType::NoFunc
            | HeapType::I31
            | HeapType::Array
            | HeapType::ConcreteArray(_)
            | HeapType::None
            | HeapType::NoCont
            | HeapType::Cont
            | HeapType::ConcreteCont(_)
            | HeapType::NoExn
            | HeapType::Exn
            | HeapType::ConcreteExn(_) => bail!(
                "type mismatch: expected `(ref {ty})`, got `(ref {})`",
                self._ty(store)?,
            ),
        }
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        self.wasm_ty_store(store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        Self::wasm_ty_load(store, ptr.get_anyref(), StructRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<ManuallyRooted<StructRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::STRUCTREF
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
            Some(s) => {
                ManuallyRooted::<StructRef>::dynamic_concrete_type_check(s, store, nullable, ty)
            }
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
        <ManuallyRooted<StructRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <ManuallyRooted<StructRef>>::wasm_ty_option_load(
            store,
            ptr.get_anyref(),
            StructRef::from_cloned_gc_ref,
        )
    }
}
