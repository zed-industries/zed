use crate::{
    ser::Serializer,
    vec::{ArchivedVec, VecResolver},
    Archive, Serialize,
};
use core::{
    borrow::Borrow,
    cmp,
    ops::{Deref, Index, IndexMut},
    pin::Pin,
    slice::SliceIndex,
};

/// An archived [`Vec`].
///
/// This uses a [`RelPtr`](crate::rel_ptr::RelPtr) to a `[T]` under the hood. Unlike
/// [`ArchivedString`](crate::string::ArchivedString), it does not have an inline representation.
#[derive(Hash, Eq, Debug)]
#[repr(transparent)]
pub struct RawArchivedVec<T> {
    inner: ArchivedVec<T>,
}

impl<T> RawArchivedVec<T> {
    /// Returns a pointer to the first element of the archived vec.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    /// Returns the number of elements in the archived vec.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether the archived vec is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Gets the elements of the archived vec as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    /// Gets the elements of the archived vec as a pinned mutable slice.
    #[inline]
    pub fn pin_mut_slice(self: Pin<&mut Self>) -> Pin<&mut [T]> {
        unsafe { self.map_unchecked_mut(|s| &mut s.inner).pin_mut_slice() }
    }

    // This method can go away once pinned slices have indexing support
    // https://github.com/rust-lang/rust/pull/78370

    /// Gets the element at the given index ot this archived vec as a pinned mutable reference.
    #[inline]
    pub fn index_pin<I>(self: Pin<&mut Self>, index: I) -> Pin<&mut <[T] as Index<I>>::Output>
    where
        [T]: IndexMut<I>,
    {
        unsafe { self.map_unchecked_mut(|s| &mut s.inner).index_pin(index) }
    }

    /// Resolves an archived `Vec` from a given slice.
    ///
    /// # Safety
    ///
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of serializing `value` with
    ///   [`serialize_copy_from_slice`](RawArchivedVec::serialize_copy_from_slice).
    #[inline]
    pub unsafe fn resolve_from_slice<U: Archive<Archived = T>>(
        slice: &[U],
        pos: usize,
        resolver: VecResolver,
        out: *mut Self,
    ) {
        ArchivedVec::resolve_from_slice(slice, pos, resolver, out.cast());
    }

    /// Serializes an archived `Vec` from a given slice by directly copying bytes.
    ///
    /// # Safety
    ///
    /// The type being serialized must be copy-safe. Copy-safe types must be trivially copyable
    /// (have the same archived and unarchived representations) and contain no padding bytes. In
    /// situations where copying uninitialized bytes the output is acceptable, this function may be
    /// used with types that contain padding bytes.
    ///
    /// Additionally, the type being serialized must not require any validation. All bit patterns
    /// must represent valid values.
    #[inline]
    pub unsafe fn serialize_copy_from_slice<U, S>(
        slice: &[U],
        serializer: &mut S,
    ) -> Result<VecResolver, S::Error>
    where
        U: Serialize<S, Archived = T>,
        S: Serializer + ?Sized,
    {
        ArchivedVec::serialize_copy_from_slice(slice, serializer)
    }
}

impl<T> AsRef<[T]> for RawArchivedVec<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.inner.as_ref()
    }
}

impl<T> Borrow<[T]> for RawArchivedVec<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.inner.borrow()
    }
}

impl<T> Deref for RawArchivedVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for RawArchivedVec<T> {
    type Output = <[T] as Index<I>>::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.inner.index(index)
    }
}

impl<T: PartialEq<U>, U> PartialEq<RawArchivedVec<U>> for RawArchivedVec<T> {
    #[inline]
    fn eq(&self, other: &RawArchivedVec<U>) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: PartialEq<U>, U, const N: usize> PartialEq<[U; N]> for RawArchivedVec<T> {
    #[inline]
    fn eq(&self, other: &[U; N]) -> bool {
        self.inner.eq(&other[..])
    }
}

impl<T: PartialEq<U>, U, const N: usize> PartialEq<RawArchivedVec<T>> for [U; N] {
    #[inline]
    fn eq(&self, other: &RawArchivedVec<T>) -> bool {
        self.eq(&other.inner)
    }
}

impl<T: PartialEq<U>, U> PartialEq<[U]> for RawArchivedVec<T> {
    #[inline]
    fn eq(&self, other: &[U]) -> bool {
        self.inner.eq(other)
    }
}

impl<T: PartialEq<U>, U> PartialEq<RawArchivedVec<U>> for [T] {
    #[inline]
    fn eq(&self, other: &RawArchivedVec<U>) -> bool {
        self.eq(&other.inner)
    }
}

impl<T: PartialOrd> PartialOrd<RawArchivedVec<T>> for RawArchivedVec<T> {
    #[inline]
    fn partial_cmp(&self, other: &RawArchivedVec<T>) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: Ord> Ord for RawArchivedVec<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T: PartialOrd> PartialOrd<[T]> for RawArchivedVec<T> {
    #[inline]
    fn partial_cmp(&self, other: &[T]) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(other)
    }
}

impl<T: PartialOrd> PartialOrd<RawArchivedVec<T>> for [T] {
    #[inline]
    fn partial_cmp(&self, other: &RawArchivedVec<T>) -> Option<cmp::Ordering> {
        self.partial_cmp(&other.inner)
    }
}

#[cfg(feature = "validation")]
const _: () = {
    use crate::validation::{owned::CheckOwnedPointerError, ArchiveContext};
    use bytecheck::{CheckBytes, Error};

    impl<T, C> CheckBytes<C> for RawArchivedVec<T>
    where
        T: CheckBytes<C>,
        C: ArchiveContext + ?Sized,
        C::Error: Error,
    {
        type Error = CheckOwnedPointerError<[T], C>;

        #[inline]
        unsafe fn check_bytes<'a>(
            value: *const Self,
            context: &mut C,
        ) -> Result<&'a Self, Self::Error> {
            ArchivedVec::<T>::check_bytes_with::<C, _>(value.cast(), context, |_, _| Ok(()))?;
            Ok(&*value)
        }
    }
};
