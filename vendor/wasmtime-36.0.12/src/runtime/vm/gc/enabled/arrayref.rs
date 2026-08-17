use super::{truncate_i32_to_i8, truncate_i32_to_i16};
use crate::{
    AnyRef, ExnRef, ExternRef, Func, HeapType, RootedGcRefImpl, StorageType, Val, ValType,
    prelude::*,
    runtime::vm::{GcHeap, GcStore, VMGcRef},
    store::{AutoAssertNoGc, StoreOpaque},
    vm::{FuncRefTableId, SendSyncPtr},
};
use core::fmt;
use wasmtime_environ::{GcArrayLayout, VMGcKind};

/// A `VMGcRef` that we know points to a `array`.
///
/// Create a `VMArrayRef` via `VMGcRef::into_arrayref` and
/// `VMGcRef::as_arrayref`, or their untyped equivalents
/// `VMGcRef::into_arrayref_unchecked` and `VMGcRef::as_arrayref_unchecked`.
///
/// Note: This is not a `TypedGcRef<_>` because each collector can have a
/// different concrete representation of `arrayref` that they allocate inside
/// their heaps.
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VMArrayRef(VMGcRef);

impl fmt::Pointer for VMArrayRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

impl From<VMArrayRef> for VMGcRef {
    #[inline]
    fn from(x: VMArrayRef) -> Self {
        x.0
    }
}

impl VMGcRef {
    /// Is this `VMGcRef` pointing to a `array`?
    pub fn is_arrayref(&self, gc_heap: &(impl GcHeap + ?Sized)) -> bool {
        if self.is_i31() {
            return false;
        }

        let header = gc_heap.header(&self);
        header.kind().matches(VMGcKind::ArrayRef)
    }

    /// Create a new `VMArrayRef` from the given `gc_ref`.
    ///
    /// If this is not a GC reference to an `arrayref`, `Err(self)` is
    /// returned.
    pub fn into_arrayref(self, gc_heap: &(impl GcHeap + ?Sized)) -> Result<VMArrayRef, VMGcRef> {
        if self.is_arrayref(gc_heap) {
            Ok(self.into_arrayref_unchecked())
        } else {
            Err(self)
        }
    }

    /// Create a new `VMArrayRef` from `self` without actually checking that
    /// `self` is an `arrayref`.
    ///
    /// This method does not check that `self` is actually an `arrayref`, but
    /// it should be. Failure to uphold this invariant is memory safe but will
    /// result in general incorrectness down the line such as panics or wrong
    /// results.
    #[inline]
    pub fn into_arrayref_unchecked(self) -> VMArrayRef {
        debug_assert!(!self.is_i31());
        VMArrayRef(self)
    }

    /// Get this GC reference as an `arrayref` reference, if it actually is an
    /// `arrayref` reference.
    pub fn as_arrayref(&self, gc_heap: &(impl GcHeap + ?Sized)) -> Option<&VMArrayRef> {
        if self.is_arrayref(gc_heap) {
            Some(self.as_arrayref_unchecked())
        } else {
            None
        }
    }

    /// Get this GC reference as an `arrayref` reference without checking if it
    /// actually is an `arrayref` reference.
    ///
    /// Calling this method on a non-`arrayref` reference is memory safe, but
    /// will lead to general incorrectness like panics and wrong results.
    pub fn as_arrayref_unchecked(&self) -> &VMArrayRef {
        debug_assert!(!self.is_i31());
        let ptr = self as *const VMGcRef;
        let ret = unsafe { &*ptr.cast() };
        assert!(matches!(ret, VMArrayRef(VMGcRef { .. })));
        ret
    }
}

impl VMArrayRef {
    /// Get the underlying `VMGcRef`.
    pub fn as_gc_ref(&self) -> &VMGcRef {
        &self.0
    }

    /// Clone this `VMArrayRef`, running any GC barriers as necessary.
    pub fn clone(&self, gc_store: &mut GcStore) -> Self {
        Self(gc_store.clone_gc_ref(&self.0))
    }

    /// Explicitly drop this `arrayref`, running GC drop barriers as necessary.
    pub fn drop(self, gc_store: &mut GcStore) {
        gc_store.drop_gc_ref(self.0);
    }

    /// Copy this `VMArrayRef` without running the GC's clone barriers.
    ///
    /// Prefer calling `clone(&mut GcStore)` instead! This is mostly an internal
    /// escape hatch for collector implementations.
    ///
    /// Failure to run GC barriers when they would otherwise be necessary can
    /// lead to leaks, panics, and wrong results. It cannot lead to memory
    /// unsafety, however.
    pub fn unchecked_copy(&self) -> Self {
        Self(self.0.unchecked_copy())
    }

    /// Get the length of this array.
    pub fn len(&self, store: &StoreOpaque) -> u32 {
        store.unwrap_gc_store().array_len(self)
    }

    /// Read an element of the given `StorageType` into a `Val`.
    ///
    /// `i8` and `i16` fields are zero-extended into `Val::I32(_)`s.
    ///
    /// Does not check that this array's elements are actually of type
    /// `ty`. That is the caller's responsibility. Failure to do so is memory
    /// safe, but will lead to general incorrectness such as panics and wrong
    /// results.
    ///
    /// Panics on out-of-bounds accesses.
    pub fn read_elem(
        &self,
        store: &mut AutoAssertNoGc,
        layout: &GcArrayLayout,
        ty: &StorageType,
        index: u32,
    ) -> Val {
        let offset = layout.elem_offset(index);
        let data = store.unwrap_gc_store_mut().gc_object_data(self.as_gc_ref());
        match ty {
            StorageType::I8 => Val::I32(data.read_u8(offset).into()),
            StorageType::I16 => Val::I32(data.read_u16(offset).into()),
            StorageType::ValType(ValType::I32) => Val::I32(data.read_i32(offset)),
            StorageType::ValType(ValType::I64) => Val::I64(data.read_i64(offset)),
            StorageType::ValType(ValType::F32) => Val::F32(data.read_u32(offset)),
            StorageType::ValType(ValType::F64) => Val::F64(data.read_u64(offset)),
            StorageType::ValType(ValType::V128) => Val::V128(data.read_v128(offset)),
            StorageType::ValType(ValType::Ref(r)) => match r.heap_type().top() {
                HeapType::Extern => {
                    let raw = data.read_u32(offset);
                    Val::ExternRef(ExternRef::_from_raw(store, raw))
                }
                HeapType::Any => {
                    let raw = data.read_u32(offset);
                    Val::AnyRef(AnyRef::_from_raw(store, raw))
                }
                HeapType::Exn => {
                    let raw = data.read_u32(offset);
                    Val::ExnRef(ExnRef::_from_raw(store, raw))
                }
                HeapType::Func => {
                    let func_ref_id = data.read_u32(offset);
                    let func_ref_id = FuncRefTableId::from_raw(func_ref_id);
                    let func_ref = store
                        .unwrap_gc_store()
                        .func_ref_table
                        .get_untyped(func_ref_id);
                    Val::FuncRef(unsafe {
                        func_ref.map(|p| Func::from_vm_func_ref(store.id(), p.as_non_null()))
                    })
                }
                otherwise => unreachable!("not a top type: {otherwise:?}"),
            },
        }
    }

    /// Write the given value into this array at the given offset.
    ///
    /// Returns an error if `val` is a GC reference that has since been
    /// unrooted.
    ///
    /// Does not check that `val` matches `ty`, nor that the field is actually
    /// of type `ty`. Checking those things is the caller's responsibility.
    /// Failure to do so is memory safe, but will lead to general incorrectness
    /// such as panics and wrong results.
    ///
    /// Panics on out-of-bounds accesses.
    pub fn write_elem(
        &self,
        store: &mut AutoAssertNoGc,
        layout: &GcArrayLayout,
        ty: &StorageType,
        index: u32,
        val: Val,
    ) -> Result<()> {
        debug_assert!(val._matches_ty(&store, &ty.unpack())?);

        let offset = layout.elem_offset(index);
        let data = store.unwrap_gc_store_mut().gc_object_data(self.as_gc_ref());
        match val {
            Val::I32(i) if ty.is_i8() => data.write_i8(offset, truncate_i32_to_i8(i)),
            Val::I32(i) if ty.is_i16() => data.write_i16(offset, truncate_i32_to_i16(i)),
            Val::I32(i) => data.write_i32(offset, i),
            Val::I64(i) => data.write_i64(offset, i),
            Val::F32(f) => data.write_u32(offset, f),
            Val::F64(f) => data.write_u64(offset, f),
            Val::V128(v) => data.write_v128(offset, v),

            // For GC-managed references, we need to take care to run the
            // appropriate barriers, even when we are writing null references
            // into the array.
            //
            // POD-read the old value into a local copy, run the GC write
            // barrier on that local copy, and then POD-write the updated
            // value back into the array. This avoids transmuting the inner
            // data, which would probably be fine, but this approach is
            // Obviously Correct and should get us by for now. If LLVM isn't
            // able to elide some of these unnecessary copies, and this
            // method is ever hot enough, we can always come back and clean
            // it up in the future.
            Val::ExternRef(e) => {
                let raw = data.read_u32(offset);
                let mut gc_ref = VMGcRef::from_raw_u32(raw);
                let e = match e {
                    Some(e) => Some(e.try_gc_ref(store)?.unchecked_copy()),
                    None => None,
                };
                store.gc_store_mut()?.write_gc_ref(&mut gc_ref, e.as_ref());
                let data = store.gc_store_mut()?.gc_object_data(self.as_gc_ref());
                data.write_u32(offset, gc_ref.map_or(0, |r| r.as_raw_u32()));
            }
            Val::AnyRef(a) => {
                let raw = data.read_u32(offset);
                let mut gc_ref = VMGcRef::from_raw_u32(raw);
                let a = match a {
                    Some(a) => Some(a.try_gc_ref(store)?.unchecked_copy()),
                    None => None,
                };
                store.gc_store_mut()?.write_gc_ref(&mut gc_ref, a.as_ref());
                let data = store.gc_store_mut()?.gc_object_data(self.as_gc_ref());
                data.write_u32(offset, gc_ref.map_or(0, |r| r.as_raw_u32()));
            }
            Val::ExnRef(e) => {
                let raw = data.read_u32(offset);
                let mut gc_ref = VMGcRef::from_raw_u32(raw);
                let e = match e {
                    Some(e) => Some(e.try_gc_ref(store)?.unchecked_copy()),
                    None => None,
                };
                store.gc_store_mut()?.write_gc_ref(&mut gc_ref, e.as_ref());
                let data = store.gc_store_mut()?.gc_object_data(self.as_gc_ref());
                data.write_u32(offset, gc_ref.map_or(0, |r| r.as_raw_u32()));
            }

            Val::FuncRef(f) => {
                let func_ref = match f {
                    Some(f) => Some(SendSyncPtr::new(f.vm_func_ref(store))),
                    None => None,
                };
                let id = unsafe { store.gc_store_mut()?.func_ref_table.intern(func_ref) };
                store
                    .gc_store_mut()?
                    .gc_object_data(self.as_gc_ref())
                    .write_u32(offset, id.into_raw());
            }
        }
        Ok(())
    }

    /// Initialize an element in this arrayref that is currently uninitialized.
    ///
    /// The difference between this method and `write_elem` is that GC barriers
    /// are handled differently. When overwriting an initialized element (aka
    /// `write_elem`) we need to call the full write GC write barrier, which
    /// logically drops the old GC reference and clones the new GC
    /// reference. When we are initializing an element for the first time, there
    /// is no old GC reference that is being overwritten and which we need to
    /// drop, so we only need to clone the new GC reference.
    ///
    /// Calling this method on a arrayref that has already had the associated
    /// element initialized will result in GC bugs. These are memory safe but
    /// will lead to generally incorrect behavior such as panics, leaks, and
    /// incorrect results.
    ///
    /// Does not check that `val` matches `ty`, nor that the field is actually
    /// of type `ty`. Checking those things is the caller's responsibility.
    /// Failure to do so is memory safe, but will lead to general incorrectness
    /// such as panics and wrong results.
    ///
    /// Returns an error if `val` is a GC reference that has since been
    /// unrooted.
    ///
    /// Panics on out-of-bounds accesses.
    pub fn initialize_elem(
        &self,
        store: &mut AutoAssertNoGc,
        layout: &GcArrayLayout,
        ty: &StorageType,
        index: u32,
        val: Val,
    ) -> Result<()> {
        debug_assert!(val._matches_ty(&store, &ty.unpack())?);
        let offset = layout.elem_offset(index);
        match val {
            Val::I32(i) if ty.is_i8() => store
                .gc_store_mut()?
                .gc_object_data(self.as_gc_ref())
                .write_i8(offset, truncate_i32_to_i8(i)),
            Val::I32(i) if ty.is_i16() => store
                .gc_store_mut()?
                .gc_object_data(self.as_gc_ref())
                .write_i16(offset, truncate_i32_to_i16(i)),
            Val::I32(i) => store
                .gc_store_mut()?
                .gc_object_data(self.as_gc_ref())
                .write_i32(offset, i),
            Val::I64(i) => store
                .gc_store_mut()?
                .gc_object_data(self.as_gc_ref())
                .write_i64(offset, i),
            Val::F32(f) => store
                .gc_store_mut()?
                .gc_object_data(self.as_gc_ref())
                .write_u32(offset, f),
            Val::F64(f) => store
                .gc_store_mut()?
                .gc_object_data(self.as_gc_ref())
                .write_u64(offset, f),
            Val::V128(v) => store
                .gc_store_mut()?
                .gc_object_data(self.as_gc_ref())
                .write_v128(offset, v),

            // NB: We don't need to do a write barrier when initializing a
            // field, because there is nothing being overwritten. Therefore, we
            // just the clone barrier.
            Val::ExternRef(x) => {
                let x = match x {
                    None => 0,
                    Some(x) => x.try_clone_gc_ref(store)?.as_raw_u32(),
                };
                store
                    .gc_store_mut()?
                    .gc_object_data(self.as_gc_ref())
                    .write_u32(offset, x);
            }
            Val::AnyRef(x) => {
                let x = match x {
                    None => 0,
                    Some(x) => x.try_clone_gc_ref(store)?.as_raw_u32(),
                };
                store
                    .gc_store_mut()?
                    .gc_object_data(self.as_gc_ref())
                    .write_u32(offset, x);
            }
            Val::ExnRef(x) => {
                let x = match x {
                    None => 0,
                    Some(x) => x.try_clone_gc_ref(store)?.as_raw_u32(),
                };
                store
                    .gc_store_mut()?
                    .gc_object_data(self.as_gc_ref())
                    .write_u32(offset, x);
            }

            Val::FuncRef(f) => {
                let func_ref = match f {
                    Some(f) => Some(SendSyncPtr::new(f.vm_func_ref(store))),
                    None => None,
                };
                let id = unsafe { store.gc_store_mut()?.func_ref_table.intern(func_ref) };
                store
                    .gc_store_mut()?
                    .gc_object_data(self.as_gc_ref())
                    .write_u32(offset, id.into_raw());
            }
        }
        Ok(())
    }
}
