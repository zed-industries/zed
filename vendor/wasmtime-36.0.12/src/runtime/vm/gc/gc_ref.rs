use crate::prelude::*;
use crate::runtime::vm::{GcHeap, GcStore, I31};
use core::fmt;
use core::marker;
use core::num::NonZeroU32;
use wasmtime_environ::packed_option::ReservedValue;
use wasmtime_environ::{VMGcKind, VMSharedTypeIndex};

/// The common header for all objects allocated in a GC heap.
///
/// This header is shared across all collectors, although particular collectors
/// may always add their own trailing fields to this header for all of their own
/// GC objects.
///
/// This is a bit-packed structure that logically has the following fields:
///
/// ```ignore
/// struct VMGcHeader {
///     // Highest 5 bits.
///     kind: VMGcKind,
///
///     // 27 bits available for the `GcRuntime` to make use of however it sees fit.
///     reserved: u27,
///
///     // The `VMSharedTypeIndex` for this GC object, if it isn't an
///     // `externref` (or an `externref` re-wrapped as an `anyref`). `None` is
///     // represented with `VMSharedTypeIndex::reserved_value()`.
///     ty: Option<VMSharedTypeIndex>,
/// }
/// ```
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct VMGcHeader {
    /// The object's `VMGcKind` and 27 bits of space reserved for however the GC
    /// sees fit to use it.
    kind: u32,

    /// The object's type index.
    ty: VMSharedTypeIndex,
}

unsafe impl GcHeapObject for VMGcHeader {
    #[inline]
    fn is(_: &VMGcHeader) -> bool {
        true
    }
}

const _: () = {
    use core::mem::offset_of;
    use wasmtime_environ::*;
    assert!((VM_GC_HEADER_SIZE as usize) == core::mem::size_of::<VMGcHeader>());
    assert!((VM_GC_HEADER_ALIGN as usize) == core::mem::align_of::<VMGcHeader>());
    assert!((VM_GC_HEADER_KIND_OFFSET as usize) == offset_of!(VMGcHeader, kind));
    assert!((VM_GC_HEADER_TYPE_INDEX_OFFSET as usize) == offset_of!(VMGcHeader, ty));
};

impl VMGcHeader {
    /// Create the header for an `externref`.
    pub fn externref() -> Self {
        Self::from_kind_and_index(VMGcKind::ExternRef, VMSharedTypeIndex::reserved_value())
    }

    /// Create the header for the given kind and type index.
    pub fn from_kind_and_index(kind: VMGcKind, ty: VMSharedTypeIndex) -> Self {
        let kind = kind.as_u32();
        Self { kind, ty }
    }

    /// Get the kind of GC object that this is.
    pub fn kind(&self) -> VMGcKind {
        VMGcKind::from_high_bits_of_u32(self.kind)
    }

    /// Get the reserved 26 bits in this header.
    ///
    /// These are bits are reserved for `GcRuntime` implementations to make use
    /// of however they see fit.
    pub fn reserved_u26(&self) -> u32 {
        self.kind & VMGcKind::UNUSED_MASK
    }

    /// Set the 26-bit reserved value.
    ///
    /// # Panics
    ///
    /// Panics if the given `value` has any of the upper 6 bits set.
    pub fn set_reserved_u26(&mut self, value: u32) {
        assert!(
            VMGcKind::value_fits_in_unused_bits(value),
            "VMGcHeader::set_reserved_u26 with value using more than 26 bits: \
             {value:#034b} ({value}, {value:#010x})"
        );
        self.kind &= VMGcKind::MASK;
        self.kind |= value;
    }

    /// Set the 26-bit reserved value.
    ///
    /// # Safety
    ///
    /// The given `value` must only use the lower 26 bits; its upper 6 bits must
    /// be unset.
    pub unsafe fn unchecked_set_reserved_u26(&mut self, value: u32) {
        debug_assert_eq!(value & VMGcKind::MASK, 0);
        self.kind &= VMGcKind::MASK;
        self.kind |= value;
    }

    /// Get this object's specific concrete type.
    pub fn ty(&self) -> Option<VMSharedTypeIndex> {
        if self.ty.is_reserved_value() {
            None
        } else {
            Some(self.ty)
        }
    }
}

/// A raw, unrooted GC reference.
///
/// A `VMGcRef` is either:
///
/// * A reference to some kind of object on the GC heap, but we don't know
///   exactly which kind without further reflection. Furthermore, this is not
///   actually a pointer, but a compact index into a Wasm GC heap.
///
/// * An `i31ref`: it doesn't actually reference an object in the GC heap, but
///   is instead an inline, unboxed 31-bit integer.
///
/// ## `VMGcRef` and GC Barriers
///
/// Depending on the garbage collector in use, cloning, writing, and dropping a
/// `VMGcRef` may require invoking GC barriers (little snippets of code provided
/// by the collector to ensure it is correctly tracking all GC references).
///
/// Therefore, to encourage correct usage of GC barriers, this type does *NOT*
/// implement `Clone` or `Copy`. Use `GcStore::clone_gc_ref`,
/// `GcStore::write_gc_ref`, and `GcStore::drop_gc_ref` to clone, write, and
/// drop `VMGcRef`s respectively.
///
/// As an escape hatch, if you really need to copy a `VMGcRef` without invoking
/// GC barriers and you understand why that will not lead to GC bugs in this
/// particular case, you can use the `unchecked_copy` method.
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VMGcRef(NonZeroU32);

impl<T> From<TypedGcRef<T>> for VMGcRef {
    #[inline]
    fn from(value: TypedGcRef<T>) -> Self {
        value.gc_ref
    }
}

impl fmt::LowerHex for VMGcRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for VMGcRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Pointer for VMGcRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#x}")
    }
}

impl VMGcRef {
    /// If this bit is set on a GC reference, then the GC reference is actually an
    /// unboxed `i31`.
    ///
    /// Must be kept in sync with `wasmtime_cranelift::I31_REF_DISCRIMINANT`.
    pub const I31_REF_DISCRIMINANT: u32 = 1;

    /// Create a new `VMGcRef` from the given raw u32 value.
    ///
    /// Does not discriminate between indices into a GC heap and `i31ref`s.
    ///
    /// Returns `None` for zero values.
    ///
    /// The given index should point to a valid GC-managed object within this
    /// reference's associated heap. Failure to uphold this will be memory safe,
    /// but will lead to general failures such as panics or incorrect results.
    pub fn from_raw_u32(raw: u32) -> Option<Self> {
        Some(Self::from_raw_non_zero_u32(NonZeroU32::new(raw)?))
    }

    /// Create a new `VMGcRef` from the given index into a GC heap.
    ///
    /// The given index should point to a valid GC-managed object within this
    /// reference's associated heap. Failure to uphold this will be memory safe,
    /// but will lead to general failures such as panics or incorrect results.
    ///
    /// Returns `None` when the index is not 2-byte aligned and therefore
    /// conflicts with the `i31ref` discriminant.
    pub fn from_heap_index(index: NonZeroU32) -> Option<Self> {
        if (index.get() & Self::I31_REF_DISCRIMINANT) == 0 {
            Some(Self::from_raw_non_zero_u32(index))
        } else {
            None
        }
    }

    /// Create a new `VMGcRef` from the given raw value.
    ///
    /// Does not discriminate between indices into a GC heap and `i31ref`s.
    pub fn from_raw_non_zero_u32(raw: NonZeroU32) -> Self {
        VMGcRef(raw)
    }

    /// Construct a new `VMGcRef` from an unboxed 31-bit integer.
    #[inline]
    pub fn from_i31(val: I31) -> Self {
        let val = (val.get_u32() << 1) | Self::I31_REF_DISCRIMINANT;
        debug_assert_ne!(val, 0);
        let non_zero = unsafe { NonZeroU32::new_unchecked(val) };
        VMGcRef::from_raw_non_zero_u32(non_zero)
    }

    /// Copy this `VMGcRef` without running the GC's clone barriers.
    ///
    /// Prefer calling `clone(&mut GcStore)` instead! This is mostly an internal
    /// escape hatch for collector implementations.
    ///
    /// Failure to run GC barriers when they would otherwise be necessary can
    /// lead to leaks, panics, and wrong results. It cannot lead to memory
    /// unsafety, however.
    pub fn unchecked_copy(&self) -> Self {
        VMGcRef(self.0)
    }

    /// Copy this `i31` GC reference, which never requires any GC barriers.
    ///
    /// Panics if this is not an `i31`.
    pub fn copy_i31(&self) -> Self {
        assert!(self.is_i31());
        self.unchecked_copy()
    }

    /// Get this GC reference as a u32 index into its GC heap.
    ///
    /// Returns `None` for `i31ref`s.
    pub fn as_heap_index(&self) -> Option<NonZeroU32> {
        if self.is_i31() { None } else { Some(self.0) }
    }

    /// Get this GC reference as a raw, non-zero u32 value, regardless whether
    /// it is actually a reference to a GC object or is an `i31ref`.
    pub fn as_raw_non_zero_u32(&self) -> NonZeroU32 {
        self.0
    }

    /// Get this GC reference as a raw u32 value, regardless whether it is
    /// actually a reference to a GC object or is an `i31ref`.
    pub fn as_raw_u32(&self) -> u32 {
        self.0.get()
    }

    /// Creates a typed GC reference from `self`, checking that `self` actually
    /// is a `T`.
    ///
    /// If this is not a GC reference to a `T`, then `Err(self)` is returned.
    pub fn into_typed<T>(self, gc_heap: &impl GcHeap) -> Result<TypedGcRef<T>, Self>
    where
        T: GcHeapObject,
    {
        if self.is_i31() {
            return Err(self);
        }
        if T::is(gc_heap.header(&self)) {
            Ok(TypedGcRef {
                gc_ref: self,
                _phantom: marker::PhantomData,
            })
        } else {
            Err(self)
        }
    }

    /// Creates a typed GC reference without actually checking that `self` is a
    /// `T`.
    ///
    /// `self` should point to a `T` object. Failure to uphold this invariant is
    /// memory safe, but will lead to general incorrectness such as panics or
    /// wrong results.
    pub fn into_typed_unchecked<T>(self) -> TypedGcRef<T>
    where
        T: GcHeapObject,
    {
        debug_assert!(!self.is_i31());
        TypedGcRef {
            gc_ref: self,
            _phantom: marker::PhantomData,
        }
    }

    /// Is this GC reference pointing to a `T`?
    pub fn is_typed<T>(&self, gc_heap: &impl GcHeap) -> bool
    where
        T: GcHeapObject,
    {
        if self.is_i31() {
            return false;
        }
        T::is(gc_heap.header(&self))
    }

    /// Borrow `self` as a typed GC reference, checking that `self` actually is
    /// a `T`.
    #[inline]
    pub fn as_typed<T>(&self, gc_heap: &impl GcHeap) -> Option<&TypedGcRef<T>>
    where
        T: GcHeapObject,
    {
        if self.is_i31() {
            return None;
        }
        if T::is(gc_heap.header(&self)) {
            let ptr = self as *const VMGcRef;
            let ret = unsafe { &*ptr.cast() };
            assert!(matches!(
                ret,
                TypedGcRef {
                    gc_ref: VMGcRef(_),
                    _phantom
                }
            ));
            Some(ret)
        } else {
            None
        }
    }

    /// Creates a typed GC reference without actually checking that `self` is a
    /// `T`.
    ///
    /// `self` should point to a `T` object. Failure to uphold this invariant is
    /// memory safe, but will lead to general incorrectness such as panics or
    /// wrong results.
    pub fn as_typed_unchecked<T>(&self) -> &TypedGcRef<T>
    where
        T: GcHeapObject,
    {
        debug_assert!(!self.is_i31());
        let ptr = self as *const VMGcRef;
        let ret = unsafe { &*ptr.cast() };
        assert!(matches!(
            ret,
            TypedGcRef {
                gc_ref: VMGcRef(_),
                _phantom
            }
        ));
        ret
    }

    /// Get a reference to the GC header that this GC reference is pointing to.
    ///
    /// Returns `None` when this is an `i31ref` and doesn't actually point to a
    /// GC header.
    pub fn gc_header<'a>(&self, gc_heap: &'a (impl GcHeap + ?Sized)) -> Option<&'a VMGcHeader> {
        if self.is_i31() {
            None
        } else {
            Some(gc_heap.header(self))
        }
    }

    /// Is this `VMGcRef` actually an unboxed 31-bit integer, and not actually a
    /// GC reference?
    #[inline]
    pub fn is_i31(&self) -> bool {
        let val = self.0.get();
        (val & Self::I31_REF_DISCRIMINANT) != 0
    }

    /// Get the underlying `i31`, if any.
    #[inline]
    pub fn as_i31(&self) -> Option<I31> {
        if self.is_i31() {
            let val = self.0.get();
            Some(I31::wrapping_u32(val >> 1))
        } else {
            None
        }
    }

    /// Get the underlying `i31`, panicking if this is not an `i31`.
    #[inline]
    pub fn unwrap_i31(&self) -> I31 {
        self.as_i31().unwrap()
    }

    /// Is this `VMGcRef` a `VMExternRef`?
    #[inline]
    pub fn is_extern_ref(&self, gc_heap: &(impl GcHeap + ?Sized)) -> bool {
        self.gc_header(gc_heap)
            .map_or(false, |h| h.kind().matches(VMGcKind::ExternRef))
    }

    /// Is this `VMGcRef` an `anyref`?
    #[inline]
    pub fn is_any_ref(&self, gc_heap: &(impl GcHeap + ?Sized)) -> bool {
        self.is_i31()
            || self
                .gc_header(gc_heap)
                .map_or(false, |h| h.kind().matches(VMGcKind::AnyRef))
    }
}

/// A trait implemented by all objects allocated inside a GC heap.
///
/// # Safety
///
/// All implementations must:
///
/// * Be `repr(C)` or `repr(transparent)`
///
/// * Begin with a `VMGcHeader` as their first field
///
/// * Not have `Drop` implementations (aka, `std::mem::needs_drop::<Self>()`
///   should return `false`).
///
/// * Be memory safe to transmute to from an arbitrary byte sequence (that is,
///   it is okay if some bit patterns are invalid with regards to correctness,
///   so long as these invalid bit patterns cannot lead to memory unsafety).
pub unsafe trait GcHeapObject: Send + Sync {
    /// Check whether the GC object with the given header is an instance of
    /// `Self`.
    fn is(header: &VMGcHeader) -> bool;
}

/// A GC reference to a heap object of concrete type `T`.
///
/// Create typed GC refs via `VMGcRef::into_typed` and `VMGcRef::as_typed`, as
/// well as via their unchecked equivalents `VMGcRef::into_typed_unchecked` and
/// `VMGcRef::as_typed_unchecked`.
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TypedGcRef<T> {
    gc_ref: VMGcRef,
    _phantom: marker::PhantomData<*mut T>,
}

impl<T> TypedGcRef<T>
where
    T: GcHeapObject,
{
    /// Clone this `VMGcRef`, running any GC barriers as necessary.
    pub fn clone(&self, gc_store: &mut GcStore) -> Self {
        Self {
            gc_ref: gc_store.clone_gc_ref(&self.gc_ref),
            _phantom: marker::PhantomData,
        }
    }

    /// Explicitly drop this GC reference, running any GC barriers as necessary.
    pub fn drop(self, gc_store: &mut GcStore) {
        gc_store.drop_gc_ref(self.gc_ref);
    }

    /// Copy this GC reference without running the GC's clone barriers.
    ///
    /// Prefer calling `clone(&mut GcStore)` instead! This is mostly an internal
    /// escape hatch for collector implementations.
    ///
    /// Failure to run GC barriers when they would otherwise be necessary can
    /// lead to leaks, panics, and wrong results. It cannot lead to memory
    /// unsafety, however.
    pub fn unchecked_copy(&self) -> Self {
        Self {
            gc_ref: self.gc_ref.unchecked_copy(),
            _phantom: marker::PhantomData,
        }
    }
}

impl<T> TypedGcRef<T> {
    /// Get the untyped version of this GC reference.
    pub fn as_untyped(&self) -> &VMGcRef {
        &self.gc_ref
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_bits() {
        let kind = VMGcKind::StructRef;
        let ty = VMSharedTypeIndex::new(1234);
        let mut header = VMGcHeader::from_kind_and_index(kind, ty);

        assert_eq!(header.reserved_u26(), 0);
        assert_eq!(header.kind(), kind);
        assert_eq!(header.ty(), Some(ty));

        header.set_reserved_u26(36);
        assert_eq!(header.reserved_u26(), 36);
        assert_eq!(header.kind(), kind);
        assert_eq!(header.ty(), Some(ty));

        let max = (1 << 26) - 1;
        header.set_reserved_u26(max);
        assert_eq!(header.reserved_u26(), max);
        assert_eq!(header.kind(), kind);
        assert_eq!(header.ty(), Some(ty));

        header.set_reserved_u26(0);
        assert_eq!(header.reserved_u26(), 0);
        assert_eq!(header.kind(), kind);
        assert_eq!(header.ty(), Some(ty));

        let result = std::panic::catch_unwind(move || header.set_reserved_u26(max + 1));
        assert!(result.is_err());
    }
}
