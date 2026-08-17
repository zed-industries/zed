//! Working with GC `array` objects.

use crate::runtime::vm::VMGcRef;
use crate::store::StoreId;
use crate::vm::{VMArrayRef, VMGcHeader};
use crate::{AnyRef, FieldType};
use crate::{
    ArrayType, AsContext, AsContextMut, EqRef, GcHeapOutOfMemory, GcRefImpl, GcRootIndex, HeapType,
    ManuallyRooted, RefType, Rooted, Val, ValRaw, ValType, WasmTy,
    prelude::*,
    store::{AutoAssertNoGc, StoreContextMut, StoreOpaque},
};
use core::mem::{self, MaybeUninit};
use wasmtime_environ::{GcArrayLayout, GcLayout, VMGcKind, VMSharedTypeIndex};

/// An allocator for a particular Wasm GC array type.
///
/// Every `ArrayRefPre` is associated with a particular [`Store`][crate::Store]
/// and a particular [`ArrayType`][crate::ArrayType].
///
/// Reusing an allocator across many allocations amortizes some per-type runtime
/// overheads inside Wasmtime. An `ArrayRefPre` is to `ArrayRef`s as an
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
/// // Define an array type.
/// let array_ty = ArrayType::new(
///    store.engine(),
///    FieldType::new(Mutability::Var, ValType::I32.into()),
/// );
///
/// // Create an allocator for the array type.
/// let allocator = ArrayRefPre::new(&mut store, array_ty);
///
/// {
///     let mut scope = RootScope::new(&mut store);
///
///     // Allocate a bunch of instances of our array type using the same
///     // allocator! This is faster than creating a new allocator for each
///     // instance we want to allocate.
///     for i in 0..10 {
///         let len = 42;
///         let elem = Val::I32(36);
///         ArrayRef::new(&mut scope, &allocator, &elem, len)?;
///     }
/// }
/// # Ok(())
/// # }
/// # let _ = foo();
/// ```
pub struct ArrayRefPre {
    store_id: StoreId,
    ty: ArrayType,
}

impl ArrayRefPre {
    /// Create a new `ArrayRefPre` that is associated with the given store
    /// and type.
    pub fn new(mut store: impl AsContextMut, ty: ArrayType) -> Self {
        Self::_new(store.as_context_mut().0, ty)
    }

    pub(crate) fn _new(store: &mut StoreOpaque, ty: ArrayType) -> Self {
        store.insert_gc_host_alloc_type(ty.registered_type().clone());
        let store_id = store.id();
        ArrayRefPre { store_id, ty }
    }

    pub(crate) fn layout(&self) -> &GcArrayLayout {
        self.ty
            .registered_type()
            .layout()
            .expect("array types have a layout")
            .unwrap_array()
    }

    pub(crate) fn type_index(&self) -> VMSharedTypeIndex {
        self.ty.registered_type().index()
    }
}

/// A reference to a GC-managed `array` instance.
///
/// WebAssembly `array`s are a sequence of elements of some homogeneous
/// type. The elements length is determined at allocation time — two instances
/// of the same array type may have different lengths — but, once allocated, an
/// array's length can never be resized. An array's elements are mutable or
/// constant, depending on the array's type. This determines whether any array
/// element can be assigned a new value or not. Each element is either an
/// unpacked [`Val`][crate::Val] or a packed 8-/16-bit integer. Array elements
/// are dynamically accessed via indexing; out-of-bounds accesses result in
/// traps.
///
/// Like all WebAssembly references, these are opaque and unforgeable to Wasm:
/// they cannot be faked and Wasm cannot, for example, cast the integer
/// `0x12345678` into a reference, pretend it is a valid `arrayref`, and trick
/// the host into dereferencing it and segfaulting or worse.
///
/// Note that you can also use `Rooted<ArrayRef>` and `ManuallyRooted<ArrayRef>`
/// as a type parameter with [`Func::typed`][crate::Func::typed]- and
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
/// // Define the type for an array of `i32`s.
/// let array_ty = ArrayType::new(
///    store.engine(),
///    FieldType::new(Mutability::Var, ValType::I32.into()),
/// );
///
/// // Create an allocator for the array type.
/// let allocator = ArrayRefPre::new(&mut store, array_ty);
///
/// {
///     let mut scope = RootScope::new(&mut store);
///
///     // Allocate an instance of the array type.
///     let len = 36;
///     let elem = Val::I32(42);
///     let my_array = match ArrayRef::new(&mut scope, &allocator, &elem, len) {
///         Ok(s) => s,
///         Err(e) => match e.downcast::<GcHeapOutOfMemory<()>>() {
///             // If the heap is out of memory, then do a GC to free up some
///             // space and try again.
///             Ok(oom) => {
///                 // Do a GC! Note: in an async context, you'd want to do
///                 // `scope.as_context_mut().gc_async().await`.
///                 scope.as_context_mut().gc(Some(&oom));
///
///                 // Try again. If the GC heap is still out of memory, then we
///                 // weren't able to free up resources for this allocation, so
///                 // propagate the error.
///                 ArrayRef::new(&mut scope, &allocator, &elem, len)?
///             }
///             // Propagate any other kind of error.
///             Err(e) => return Err(e),
///         }
///     };
///
///     // That instance's elements should have the initial value.
///     for i in 0..len {
///         let val = my_array.get(&mut scope, i)?.unwrap_i32();
///         assert_eq!(val, 42);
///     }
///
///     // We can set an element to a new value because the type was defined with
///     // mutable elements (as opposed to const).
///     my_array.set(&mut scope, 3, Val::I32(1234))?;
///     let new_val = my_array.get(&mut scope, 3)?.unwrap_i32();
///     assert_eq!(new_val, 1234);
/// }
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
#[derive(Debug)]
#[repr(transparent)]
pub struct ArrayRef {
    pub(super) inner: GcRootIndex,
}

unsafe impl GcRefImpl for ArrayRef {
    fn transmute_ref(index: &GcRootIndex) -> &Self {
        // Safety: `ArrayRef` is a newtype of a `GcRootIndex`.
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

impl Rooted<ArrayRef> {
    /// Upcast this `arrayref` into an `anyref`.
    #[inline]
    pub fn to_anyref(self) -> Rooted<AnyRef> {
        self.unchecked_cast()
    }

    /// Upcast this `arrayref` into an `eqref`.
    #[inline]
    pub fn to_eqref(self) -> Rooted<EqRef> {
        self.unchecked_cast()
    }
}

impl ManuallyRooted<ArrayRef> {
    /// Upcast this `arrayref` into an `anyref`.
    #[inline]
    pub fn to_anyref(self) -> ManuallyRooted<AnyRef> {
        self.unchecked_cast()
    }

    /// Upcast this `arrayref` into an `eqref`.
    #[inline]
    pub fn to_eqref(self) -> ManuallyRooted<EqRef> {
        self.unchecked_cast()
    }
}

/// An iterator for elements in `ArrayRef::new[_async].
///
/// NB: We can't use `iter::repeat(elem).take(len)` because that doesn't
/// implement `ExactSizeIterator`.
#[derive(Clone)]
struct RepeatN<'a>(&'a Val, u32);

impl<'a> Iterator for RepeatN<'a> {
    type Item = &'a Val;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == 0 {
            None
        } else {
            self.1 -= 1;
            Some(self.0)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for RepeatN<'_> {
    fn len(&self) -> usize {
        usize::try_from(self.1).unwrap()
    }
}

impl ArrayRef {
    /// Allocate a new `array` of the given length, with every element
    /// initialized to `elem`.
    ///
    /// For example, `ArrayRef::new(ctx, pre, &Val::I64(9), 3)` allocates the
    /// array `[9, 9, 9]`.
    ///
    /// This is similar to the `array.new` instruction.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for allocating this
    /// new array, then this method will automatically trigger a synchronous
    /// collection in an attempt to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If the given `elem` value's type does not match the `allocator`'s array
    /// type's element type, an error is returned.
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<()>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// # Panics
    ///
    /// Panics if the `store` is configured for async; use
    /// [`ArrayRef::new_async`][crate::ArrayRef::new_async] to perform
    /// asynchronous allocation instead.
    ///
    /// Panics if either the allocator or the `elem` value is not associated
    /// with the given store.
    pub fn new(
        mut store: impl AsContextMut,
        allocator: &ArrayRefPre,
        elem: &Val,
        len: u32,
    ) -> Result<Rooted<ArrayRef>> {
        Self::_new(store.as_context_mut().0, allocator, elem, len)
    }

    pub(crate) fn _new(
        store: &mut StoreOpaque,
        allocator: &ArrayRefPre,
        elem: &Val,
        len: u32,
    ) -> Result<Rooted<ArrayRef>> {
        store.retry_after_gc((), |store, ()| {
            Self::new_from_iter(store, allocator, RepeatN(elem, len))
        })
    }

    /// Asynchronously allocate a new `array` of the given length, with every
    /// element initialized to `elem`.
    ///
    /// For example, `ArrayRef::new(ctx, pre, &Val::I64(9), 3)` allocates the
    /// array `[9, 9, 9]`.
    ///
    /// This is similar to the `array.new` instruction.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for allocating this
    /// new array, then this method will automatically trigger a asynchronous
    /// collection in an attempt to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If the given `elem` value's type does not match the `allocator`'s array
    /// type's element type, an error is returned.
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<()>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// # Panics
    ///
    /// Panics if your engine is not configured for async; use
    /// [`ArrayRef::new_async`][crate::ArrayRef::new_async] to perform
    /// synchronous allocation instead.
    ///
    /// Panics if either the allocator or the `elem` value is not associated
    /// with the given store.
    #[cfg(feature = "async")]
    pub async fn new_async(
        mut store: impl AsContextMut,
        allocator: &ArrayRefPre,
        elem: &Val,
        len: u32,
    ) -> Result<Rooted<ArrayRef>> {
        Self::_new_async(store.as_context_mut().0, allocator, elem, len).await
    }

    #[cfg(feature = "async")]
    pub(crate) async fn _new_async(
        store: &mut StoreOpaque,
        allocator: &ArrayRefPre,
        elem: &Val,
        len: u32,
    ) -> Result<Rooted<ArrayRef>> {
        store
            .retry_after_gc_async((), |store, ()| {
                Self::new_from_iter(store, allocator, RepeatN(elem, len))
            })
            .await
    }

    /// Like `ArrayRef::new` but when async is configured must only ever be
    /// called from on a fiber stack.
    pub(crate) unsafe fn new_maybe_async(
        store: &mut StoreOpaque,
        allocator: &ArrayRefPre,
        elem: &Val,
        len: u32,
    ) -> Result<Rooted<ArrayRef>> {
        // Type check the initial element value against the element type.
        elem.ensure_matches_ty(store, allocator.ty.element_type().unpack())
            .context("element type mismatch")?;

        unsafe {
            store.retry_after_gc_maybe_async((), |store, ()| {
                Self::new_from_iter(store, allocator, RepeatN(elem, len))
            })
        }
    }

    /// Allocate a new array of the given elements.
    ///
    /// Does not attempt a GC on OOM; leaves that to callers.
    fn new_from_iter<'a>(
        store: &mut StoreOpaque,
        allocator: &ArrayRefPre,
        elems: impl Clone + ExactSizeIterator<Item = &'a Val>,
    ) -> Result<Rooted<ArrayRef>> {
        assert_eq!(
            store.id(),
            allocator.store_id,
            "attempted to use a `ArrayRefPre` with the wrong store"
        );

        // Type check the elements against the element type.
        for elem in elems.clone() {
            elem.ensure_matches_ty(store, allocator.ty.element_type().unpack())
                .context("element type mismatch")?;
        }

        let len = u32::try_from(elems.len()).unwrap();

        // Allocate the array and write each field value into the appropriate
        // offset.
        let arrayref = store
            .gc_store_mut()?
            .alloc_uninit_array(allocator.type_index(), len, allocator.layout())
            .context("unrecoverable error when allocating new `arrayref`")?
            .map_err(|n| GcHeapOutOfMemory::new((), n))?;

        // From this point on, if we get any errors, then the array is not
        // fully initialized, so we need to eagerly deallocate it before the
        // next GC where the collector might try to interpret one of the
        // uninitialized fields as a GC reference.
        let mut store = AutoAssertNoGc::new(store);
        match (|| {
            let elem_ty = allocator.ty.element_type();
            for (i, elem) in elems.enumerate() {
                let i = u32::try_from(i).unwrap();
                debug_assert!(i < len);
                arrayref.initialize_elem(&mut store, allocator.layout(), &elem_ty, i, *elem)?;
            }
            Ok(())
        })() {
            Ok(()) => Ok(Rooted::new(&mut store, arrayref.into())),
            Err(e) => {
                store.gc_store_mut()?.dealloc_uninit_array(arrayref);
                Err(e)
            }
        }
    }

    /// Synchronously allocate a new `array` containing the given elements.
    ///
    /// For example, `ArrayRef::new_fixed(ctx, pre, &[Val::I64(4), Val::I64(5),
    /// Val::I64(6)])` allocates the array `[4, 5, 6]`.
    ///
    /// This is similar to the `array.new_fixed` instruction.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for allocating this
    /// new array, then this method will automatically trigger a synchronous
    /// collection in an attempt to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If any of the `elems` values' type does not match the `allocator`'s
    /// array type's element type, an error is returned.
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<()>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// # Panics
    ///
    /// Panics if the `store` is configured for async; use
    /// [`ArrayRef::new_fixed_async`][crate::ArrayRef::new_fixed_async] to
    /// perform asynchronous allocation instead.
    ///
    /// Panics if the allocator or any of the `elems` values are not associated
    /// with the given store.
    pub fn new_fixed(
        mut store: impl AsContextMut,
        allocator: &ArrayRefPre,
        elems: &[Val],
    ) -> Result<Rooted<ArrayRef>> {
        Self::_new_fixed(store.as_context_mut().0, allocator, elems)
    }

    pub(crate) fn _new_fixed(
        store: &mut StoreOpaque,
        allocator: &ArrayRefPre,
        elems: &[Val],
    ) -> Result<Rooted<ArrayRef>> {
        store.retry_after_gc((), |store, ()| {
            Self::new_from_iter(store, allocator, elems.iter())
        })
    }

    /// Asynchronously allocate a new `array` containing the given elements.
    ///
    /// For example, `ArrayRef::new_fixed_async(ctx, pre, &[Val::I64(4),
    /// Val::I64(5), Val::I64(6)])` allocates the array `[4, 5, 6]`.
    ///
    /// This is similar to the `array.new_fixed` instruction.
    ///
    /// If your engine is not configured for async, use
    /// [`ArrayRef::new_fixed`][crate::ArrayRef::new_fixed] to perform
    /// synchronous allocation.
    ///
    /// # Automatic Garbage Collection
    ///
    /// If the GC heap is at capacity, and there isn't room for allocating this
    /// new array, then this method will automatically trigger a synchronous
    /// collection in an attempt to free up space in the GC heap.
    ///
    /// # Errors
    ///
    /// If any of the `elems` values' type does not match the `allocator`'s
    /// array type's element type, an error is returned.
    ///
    /// If the allocation cannot be satisfied because the GC heap is currently
    /// out of memory, then a [`GcHeapOutOfMemory<()>`][crate::GcHeapOutOfMemory]
    /// error is returned. The allocation might succeed on a second attempt if
    /// you drop some rooted GC references and try again.
    ///
    /// # Panics
    ///
    /// Panics if the `store` is not configured for async; use
    /// [`ArrayRef::new_fixed`][crate::ArrayRef::new_fixed] to perform
    /// synchronous allocation instead.
    ///
    /// Panics if the allocator or any of the `elems` values are not associated
    /// with the given store.
    #[cfg(feature = "async")]
    pub async fn new_fixed_async(
        mut store: impl AsContextMut,
        allocator: &ArrayRefPre,
        elems: &[Val],
    ) -> Result<Rooted<ArrayRef>> {
        Self::_new_fixed_async(store.as_context_mut().0, allocator, elems).await
    }

    #[cfg(feature = "async")]
    pub(crate) async fn _new_fixed_async(
        store: &mut StoreOpaque,
        allocator: &ArrayRefPre,
        elems: &[Val],
    ) -> Result<Rooted<ArrayRef>> {
        store
            .retry_after_gc_async((), |store, ()| {
                Self::new_from_iter(store, allocator, elems.iter())
            })
            .await
    }

    /// Like `ArrayRef::new_fixed[_async]` but it is the caller's responsibility
    /// to ensure that when async is enabled, this is only called from on a
    /// fiber stack.
    pub(crate) unsafe fn new_fixed_maybe_async(
        store: &mut StoreOpaque,
        allocator: &ArrayRefPre,
        elems: &[Val],
    ) -> Result<Rooted<ArrayRef>> {
        unsafe {
            store.retry_after_gc_maybe_async((), |store, ()| {
                Self::new_from_iter(store, allocator, elems.iter())
            })
        }
    }

    #[inline]
    pub(crate) fn comes_from_same_store(&self, store: &StoreOpaque) -> bool {
        self.inner.comes_from_same_store(store)
    }

    /// Get this `arrayref`'s type.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn ty(&self, store: impl AsContext) -> Result<ArrayType> {
        self._ty(store.as_context().0)
    }

    pub(crate) fn _ty(&self, store: &StoreOpaque) -> Result<ArrayType> {
        assert!(self.comes_from_same_store(store));
        let index = self.type_index(store)?;
        Ok(ArrayType::from_shared_type_index(store.engine(), index))
    }

    /// Does this `arrayref` match the given type?
    ///
    /// That is, is this array's type a subtype of the given type?
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store or if the
    /// type is not associated with the store's engine.
    pub fn matches_ty(&self, store: impl AsContext, ty: &ArrayType) -> Result<bool> {
        self._matches_ty(store.as_context().0, ty)
    }

    pub(crate) fn _matches_ty(&self, store: &StoreOpaque, ty: &ArrayType) -> Result<bool> {
        assert!(self.comes_from_same_store(store));
        Ok(self._ty(store)?.matches(ty))
    }

    pub(crate) fn ensure_matches_ty(&self, store: &StoreOpaque, ty: &ArrayType) -> Result<()> {
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

    /// Get the length of this array.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn len(&self, store: impl AsContext) -> Result<u32> {
        self._len(store.as_context().0)
    }

    pub(crate) fn _len(&self, store: &StoreOpaque) -> Result<u32> {
        assert!(self.comes_from_same_store(store));
        let gc_ref = self.inner.try_gc_ref(store)?;
        debug_assert!({
            let header = store.gc_store()?.header(gc_ref);
            header.kind().matches(VMGcKind::ArrayRef)
        });
        let arrayref = gc_ref.as_arrayref_unchecked();
        Ok(arrayref.len(store))
    }

    /// Get the values of this array's elements.
    ///
    /// Note that `i8` and `i16` element values are zero-extended into
    /// `Val::I32(_)`s.
    ///
    /// # Errors
    ///
    /// Return an error if this reference has been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if this reference is associated with a different store.
    pub fn elems<'a, T: 'static>(
        &'a self,
        store: impl Into<StoreContextMut<'a, T>>,
    ) -> Result<impl ExactSizeIterator<Item = Val> + 'a> {
        self._elems(store.into().0)
    }

    pub(crate) fn _elems<'a>(
        &'a self,
        store: &'a mut StoreOpaque,
    ) -> Result<impl ExactSizeIterator<Item = Val> + 'a> {
        assert!(self.comes_from_same_store(store));
        let store = AutoAssertNoGc::new(store);

        let gc_ref = self.inner.try_gc_ref(&store)?;
        let header = store.gc_store()?.header(gc_ref);
        debug_assert!(header.kind().matches(VMGcKind::ArrayRef));

        let len = self._len(&store)?;

        return Ok(Elems {
            arrayref: self,
            store,
            index: 0,
            len,
        });

        struct Elems<'a, 'b> {
            arrayref: &'a ArrayRef,
            store: AutoAssertNoGc<'b>,
            index: u32,
            len: u32,
        }

        impl Iterator for Elems<'_, '_> {
            type Item = Val;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let i = self.index;
                debug_assert!(i <= self.len);
                if i >= self.len {
                    return None;
                }
                self.index += 1;
                Some(self.arrayref._get(&mut self.store, i).unwrap())
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len - self.index;
                let len = usize::try_from(len).unwrap();
                (len, Some(len))
            }
        }

        impl ExactSizeIterator for Elems<'_, '_> {
            #[inline]
            fn len(&self) -> usize {
                let len = self.len - self.index;
                usize::try_from(len).unwrap()
            }
        }
    }

    fn header<'a>(&self, store: &'a AutoAssertNoGc<'_>) -> Result<&'a VMGcHeader> {
        assert!(self.comes_from_same_store(&store));
        let gc_ref = self.inner.try_gc_ref(store)?;
        Ok(store.gc_store()?.header(gc_ref))
    }

    fn arrayref<'a>(&self, store: &'a AutoAssertNoGc<'_>) -> Result<&'a VMArrayRef> {
        assert!(self.comes_from_same_store(&store));
        let gc_ref = self.inner.try_gc_ref(store)?;
        debug_assert!(self.header(store)?.kind().matches(VMGcKind::ArrayRef));
        Ok(gc_ref.as_arrayref_unchecked())
    }

    pub(crate) fn layout(&self, store: &AutoAssertNoGc<'_>) -> Result<GcArrayLayout> {
        assert!(self.comes_from_same_store(&store));
        let type_index = self.type_index(store)?;
        let layout = store
            .engine()
            .signatures()
            .layout(type_index)
            .expect("array types should have GC layouts");
        match layout {
            GcLayout::Array(a) => Ok(a),
            GcLayout::Struct(_) => unreachable!(),
            GcLayout::Exception(_) => unreachable!(),
        }
    }

    fn field_ty(&self, store: &StoreOpaque) -> Result<FieldType> {
        let ty = self._ty(store)?;
        Ok(ty.field_type())
    }

    /// Get this array's `index`th element.
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
    pub fn get(&self, mut store: impl AsContextMut, index: u32) -> Result<Val> {
        let mut store = AutoAssertNoGc::new(store.as_context_mut().0);
        self._get(&mut store, index)
    }

    pub(crate) fn _get(&self, store: &mut AutoAssertNoGc<'_>, index: u32) -> Result<Val> {
        assert!(
            self.comes_from_same_store(store),
            "attempted to use an array with the wrong store",
        );
        let arrayref = self.arrayref(store)?.unchecked_copy();
        let field_ty = self.field_ty(store)?;
        let layout = self.layout(store)?;
        let len = arrayref.len(store);
        ensure!(
            index < len,
            "index out of bounds: the length is {len} but the index is {index}"
        );
        Ok(arrayref.read_elem(store, &layout, field_ty.element_type(), index))
    }

    /// Set this array's `index`th element.
    ///
    /// # Errors
    ///
    /// Returns an error in the following scenarios:
    ///
    /// * When given a value of the wrong type, such as trying to write an `f32`
    ///   value into an array of `i64` elements.
    ///
    /// * When the array elements are not mutable.
    ///
    /// * When `index` is not within the range `0..self.len(ctx)`.
    ///
    /// * When `value` is a GC reference that has since been unrooted.
    ///
    /// # Panics
    ///
    /// Panics if either this reference or the given `value` is associated with
    /// a different store.
    pub fn set(&self, mut store: impl AsContextMut, index: u32, value: Val) -> Result<()> {
        self._set(store.as_context_mut().0, index, value)
    }

    pub(crate) fn _set(&self, store: &mut StoreOpaque, index: u32, value: Val) -> Result<()> {
        assert!(
            self.comes_from_same_store(store),
            "attempted to use an array with the wrong store",
        );
        assert!(
            value.comes_from_same_store(store),
            "attempted to use a value with the wrong store",
        );

        let mut store = AutoAssertNoGc::new(store);

        let field_ty = self.field_ty(&store)?;
        ensure!(
            field_ty.mutability().is_var(),
            "cannot set element {index}: array elements are not mutable"
        );

        value
            .ensure_matches_ty(&store, &field_ty.element_type().unpack())
            .with_context(|| format!("cannot set element {index}: type mismatch"))?;

        let layout = self.layout(&store)?;
        let arrayref = self.arrayref(&store)?.unchecked_copy();

        let len = arrayref.len(&store);
        ensure!(
            index < len,
            "index out of bounds: the length is {len} but the index is {index}"
        );

        arrayref.write_elem(&mut store, &layout, field_ty.element_type(), index, value)
    }

    pub(crate) fn type_index(&self, store: &StoreOpaque) -> Result<VMSharedTypeIndex> {
        let gc_ref = self.inner.try_gc_ref(store)?;
        let header = store.gc_store()?.header(gc_ref);
        debug_assert!(header.kind().matches(VMGcKind::ArrayRef));
        Ok(header.ty().expect("arrayrefs should have concrete types"))
    }

    /// Create a new `Rooted<ArrayRef>` from the given GC reference.
    ///
    /// `gc_ref` should point to a valid `arrayref` and should belong to the
    /// store's GC heap. Failure to uphold these invariants is memory safe but
    /// will lead to general incorrectness such as panics or wrong results.
    pub(crate) fn from_cloned_gc_ref(
        store: &mut AutoAssertNoGc<'_>,
        gc_ref: VMGcRef,
    ) -> Rooted<Self> {
        debug_assert!(gc_ref.is_arrayref(&*store.unwrap_gc_store().gc_heap));
        Rooted::new(store, gc_ref)
    }
}

unsafe impl WasmTy for Rooted<ArrayRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Array))
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
            HeapType::Any | HeapType::Eq | HeapType::Array => Ok(()),
            HeapType::ConcreteArray(ty) => self.ensure_matches_ty(store, ty),

            HeapType::Extern
            | HeapType::NoExtern
            | HeapType::Func
            | HeapType::ConcreteFunc(_)
            | HeapType::NoFunc
            | HeapType::I31
            | HeapType::Struct
            | HeapType::ConcreteStruct(_)
            | HeapType::Cont
            | HeapType::NoCont
            | HeapType::ConcreteCont(_)
            | HeapType::Exn
            | HeapType::NoExn
            | HeapType::ConcreteExn(_)
            | HeapType::None => bail!(
                "type mismatch: expected `(ref {ty})`, got `(ref {})`",
                self._ty(store)?,
            ),
        }
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        self.wasm_ty_store(store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        Self::wasm_ty_load(store, ptr.get_anyref(), ArrayRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<Rooted<ArrayRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::ARRAYREF
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
            Some(s) => Rooted::<ArrayRef>::dynamic_concrete_type_check(s, store, nullable, ty),
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
        <Rooted<ArrayRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <Rooted<ArrayRef>>::wasm_ty_option_load(
            store,
            ptr.get_anyref(),
            ArrayRef::from_cloned_gc_ref,
        )
    }
}

unsafe impl WasmTy for ManuallyRooted<ArrayRef> {
    #[inline]
    fn valtype() -> ValType {
        ValType::Ref(RefType::new(false, HeapType::Array))
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
            HeapType::Any | HeapType::Eq | HeapType::Array => Ok(()),
            HeapType::ConcreteArray(ty) => self.ensure_matches_ty(store, ty),

            HeapType::Extern
            | HeapType::NoExtern
            | HeapType::Func
            | HeapType::ConcreteFunc(_)
            | HeapType::NoFunc
            | HeapType::I31
            | HeapType::Struct
            | HeapType::ConcreteStruct(_)
            | HeapType::Cont
            | HeapType::NoCont
            | HeapType::ConcreteCont(_)
            | HeapType::Exn
            | HeapType::NoExn
            | HeapType::ConcreteExn(_)
            | HeapType::None => bail!(
                "type mismatch: expected `(ref {ty})`, got `(ref {})`",
                self._ty(store)?,
            ),
        }
    }

    fn store(self, store: &mut AutoAssertNoGc<'_>, ptr: &mut MaybeUninit<ValRaw>) -> Result<()> {
        self.wasm_ty_store(store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        Self::wasm_ty_load(store, ptr.get_anyref(), ArrayRef::from_cloned_gc_ref)
    }
}

unsafe impl WasmTy for Option<ManuallyRooted<ArrayRef>> {
    #[inline]
    fn valtype() -> ValType {
        ValType::ARRAYREF
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
                ManuallyRooted::<ArrayRef>::dynamic_concrete_type_check(s, store, nullable, ty)
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
        <ManuallyRooted<ArrayRef>>::wasm_ty_option_store(self, store, ptr, ValRaw::anyref)
    }

    unsafe fn load(store: &mut AutoAssertNoGc<'_>, ptr: &ValRaw) -> Self {
        <ManuallyRooted<ArrayRef>>::wasm_ty_option_load(
            store,
            ptr.get_anyref(),
            ArrayRef::from_cloned_gc_ref,
        )
    }
}
