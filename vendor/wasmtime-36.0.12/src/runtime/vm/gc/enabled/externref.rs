use crate::runtime::vm::{GcHeap, GcStore, VMGcRef};
use core::fmt;
use wasmtime_environ::VMGcKind;

/// A `VMGcRef` that we know points to an `externref`.
///
/// Create a `VMExternRef` via `VMGcRef::into_externref` and
/// `VMGcRef::as_externref`, or their untyped equivalents
/// `VMGcRef::into_externref_unchecked` and `VMGcRef::as_externref_unchecked`.
///
/// Note: This is not a `TypedGcRef<_>` because each collector can have a
/// different concrete representation of `externref` that they allocate inside
/// their heaps.
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VMExternRef(VMGcRef);

impl fmt::Pointer for VMExternRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

impl From<VMExternRef> for VMGcRef {
    #[inline]
    fn from(x: VMExternRef) -> Self {
        x.0
    }
}

impl VMGcRef {
    /// Create a new `VMExternRef` from the given `gc_ref`.
    ///
    /// If this is not GC reference to an `externref`, `Err(self)` is returned.
    pub fn into_externref(self, gc_heap: &impl GcHeap) -> Result<VMExternRef, VMGcRef> {
        if self.is_i31() {
            return Err(self);
        }
        if gc_heap.header(&self).kind() == VMGcKind::ExternRef {
            Ok(VMExternRef(self))
        } else {
            Err(self)
        }
    }

    /// Create a new `VMExternRef` from `self` without actually checking that
    /// `self` is an `externref`.
    ///
    /// This method does not check that `self` is actually an `externref`, but
    /// it should be. Failure to uphold this invariant is memory safe but will
    /// result in general incorrectness down the line such as panics or wrong
    /// results.
    #[inline]
    pub fn into_externref_unchecked(self) -> VMExternRef {
        debug_assert!(!self.is_i31());
        VMExternRef(self)
    }

    /// Get this GC reference as an `externref` reference, if it actually is an
    /// `externref` reference.
    pub fn as_externref(&self, gc_heap: &(impl GcHeap + ?Sized)) -> Option<&VMExternRef> {
        if self.is_i31() {
            return None;
        }
        if gc_heap.header(&self).kind() == VMGcKind::ExternRef {
            let ptr = self as *const VMGcRef;
            let ret = unsafe { &*ptr.cast() };
            assert!(matches!(ret, VMExternRef(VMGcRef { .. })));
            Some(ret)
        } else {
            None
        }
    }
}

impl VMExternRef {
    /// Get the underlying `VMGcRef`.
    pub fn as_gc_ref(&self) -> &VMGcRef {
        &self.0
    }

    /// Clone this `VMExternRef`, running any GC barriers as necessary.
    pub fn clone(&self, gc_store: &mut GcStore) -> Self {
        Self(gc_store.clone_gc_ref(&self.0))
    }

    /// Explicitly drop this `externref`, running GC drop barriers as necessary.
    pub fn drop(self, gc_store: &mut GcStore) {
        gc_store.drop_gc_ref(self.0);
    }

    /// Copy this `VMExternRef` without running the GC's clone barriers.
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
}
