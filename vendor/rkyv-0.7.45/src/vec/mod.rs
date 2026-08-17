//! An archived version of `Vec`.

mod raw;

use crate::{
    ser::{ScratchSpace, Serializer},
    Archive, Archived, RelPtr, Serialize, SerializeUnsized,
};
use core::{
    borrow::Borrow,
    cmp, fmt, hash,
    ops::{Deref, Index, IndexMut},
    pin::Pin,
    slice::SliceIndex,
};

pub use self::raw::*;

/// An archived [`Vec`].
///
/// This uses a [`RelPtr`] to a `[T]` under the hood. Unlike
/// [`ArchivedString`](crate::string::ArchivedString), it does not have an inline representation.
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedVec<T> {
    ptr: RelPtr<T>,
    len: Archived<usize>,
}

impl<T> ArchivedVec<T> {
    /// Returns a pointer to the first element of the archived vec.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    /// Returns the number of elements in the archived vec.
    #[inline]
    pub fn len(&self) -> usize {
        from_archived!(self.len) as usize
    }

    /// Returns whether the archived vec is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the elements of the archived vec as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    /// Gets the elements of the archived vec as a pinned mutable slice.
    #[inline]
    pub fn pin_mut_slice(self: Pin<&mut Self>) -> Pin<&mut [T]> {
        unsafe {
            self.map_unchecked_mut(|s| core::slice::from_raw_parts_mut(s.ptr.as_mut_ptr(), s.len()))
        }
    }

    // This method can go away once pinned slices have indexing support
    // https://github.com/rust-lang/rust/pull/78370

    /// Gets the element at the given index ot this archived vec as a pinned mutable reference.
    #[inline]
    pub fn index_pin<I>(self: Pin<&mut Self>, index: I) -> Pin<&mut <[T] as Index<I>>::Output>
    where
        [T]: IndexMut<I>,
    {
        unsafe { self.pin_mut_slice().map_unchecked_mut(|s| &mut s[index]) }
    }

    /// Resolves an archived `Vec` from a given slice.
    ///
    /// # Safety
    ///
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of serializing `value`
    #[inline]
    pub unsafe fn resolve_from_slice<U: Archive<Archived = T>>(
        slice: &[U],
        pos: usize,
        resolver: VecResolver,
        out: *mut Self,
    ) {
        Self::resolve_from_len(slice.len(), pos, resolver, out);
    }

    /// Resolves an archived `Vec` from a given length.
    ///
    /// # Safety
    ///
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must bet he result of serializing `value`
    #[inline]
    pub unsafe fn resolve_from_len(len: usize, pos: usize, resolver: VecResolver, out: *mut Self) {
        let (fp, fo) = out_field!(out.ptr);
        RelPtr::emplace(pos + fp, resolver.pos, fo);
        let (fp, fo) = out_field!(out.len);
        usize::resolve(&len, pos + fp, (), fo);
    }

    /// Serializes an archived `Vec` from a given slice.
    #[inline]
    pub fn serialize_from_slice<U: Serialize<S, Archived = T>, S: Serializer + ?Sized>(
        slice: &[U],
        serializer: &mut S,
    ) -> Result<VecResolver, S::Error>
    where
        // This bound is necessary only in no-alloc, no-std situations
        // SerializeUnsized is only implemented for U: Serialize<Resolver = ()> in that case
        [U]: SerializeUnsized<S>,
    {
        Ok(VecResolver {
            pos: slice.serialize_unsized(serializer)?,
        })
    }

    /// Serializes an archived `Vec` from a given slice by directly copying bytes.
    ///
    /// # Safety
    ///
    /// The type being serialized must be copy-safe. Copy-safe types must be trivially copyable
    /// (have the same archived and unarchived representations) and contain no padding bytes. In
    /// situations where copying uninitialized bytes the output is acceptable, this function may be
    /// used with types that contain padding bytes.
    #[inline]
    pub unsafe fn serialize_copy_from_slice<U, S>(
        slice: &[U],
        serializer: &mut S,
    ) -> Result<VecResolver, S::Error>
    where
        U: Serialize<S, Archived = T>,
        S: Serializer + ?Sized,
    {
        use ::core::{mem::size_of, slice::from_raw_parts};

        let pos = serializer.align_for::<T>()?;

        let bytes = from_raw_parts(slice.as_ptr().cast::<u8>(), size_of::<T>() * slice.len());
        serializer.write(bytes)?;

        Ok(VecResolver { pos })
    }

    /// Serializes an archived `Vec` from a given iterator.
    ///
    /// This method is unable to perform copy optimizations; prefer
    /// [`serialize_from_slice`](ArchivedVec::serialize_from_slice) when possible.
    #[inline]
    pub fn serialize_from_iter<U, B, I, S>(
        iter: I,
        serializer: &mut S,
    ) -> Result<VecResolver, S::Error>
    where
        U: Serialize<S, Archived = T>,
        B: Borrow<U>,
        I: ExactSizeIterator<Item = B>,
        S: ScratchSpace + Serializer + ?Sized,
    {
        use crate::ScratchVec;

        unsafe {
            let mut resolvers = ScratchVec::new(serializer, iter.len())?;

            for value in iter {
                let resolver = value.borrow().serialize(serializer)?;
                resolvers.push((value, resolver));
            }
            let pos = serializer.align_for::<T>()?;
            for (value, resolver) in resolvers.drain(..) {
                serializer.resolve_aligned(value.borrow(), resolver)?;
            }

            resolvers.free(serializer)?;

            Ok(VecResolver { pos })
        }
    }
}

impl<T> AsRef<[T]> for ArchivedVec<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> Borrow<[T]> for ArchivedVec<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: fmt::Debug> fmt::Debug for ArchivedVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}

impl<T> Deref for ArchivedVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: Eq> Eq for ArchivedVec<T> {}

impl<T: hash::Hash> hash::Hash for ArchivedVec<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for ArchivedVec<T> {
    type Output = <[T] as Index<I>>::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl<T: Ord> Ord for ArchivedVec<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: PartialEq<U>, U> PartialEq<ArchivedVec<U>> for ArchivedVec<T> {
    #[inline]
    fn eq(&self, other: &ArchivedVec<U>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T: PartialEq<U>, U, const N: usize> PartialEq<[U; N]> for ArchivedVec<T> {
    #[inline]
    fn eq(&self, other: &[U; N]) -> bool {
        self.as_slice().eq(&other[..])
    }
}

impl<T: PartialEq<U>, U, const N: usize> PartialEq<ArchivedVec<T>> for [U; N] {
    #[inline]
    fn eq(&self, other: &ArchivedVec<T>) -> bool {
        other.eq(self)
    }
}

impl<T: PartialEq<U>, U> PartialEq<[U]> for ArchivedVec<T> {
    #[inline]
    fn eq(&self, other: &[U]) -> bool {
        self.as_slice().eq(other)
    }
}

impl<T: PartialEq<U>, U> PartialEq<ArchivedVec<U>> for [T] {
    #[inline]
    fn eq(&self, other: &ArchivedVec<U>) -> bool {
        self.eq(other.as_slice())
    }
}

impl<T: PartialOrd> PartialOrd<ArchivedVec<T>> for ArchivedVec<T> {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedVec<T>) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: PartialOrd> PartialOrd<[T]> for ArchivedVec<T> {
    #[inline]
    fn partial_cmp(&self, other: &[T]) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other)
    }
}

impl<T: PartialOrd> PartialOrd<ArchivedVec<T>> for [T] {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedVec<T>) -> Option<cmp::Ordering> {
        self.partial_cmp(other.as_slice())
    }
}

/// The resolver for [`ArchivedVec`].
pub struct VecResolver {
    pos: usize,
}

#[cfg(feature = "validation")]
const _: () = {
    use crate::validation::{
        owned::{CheckOwnedPointerError, OwnedPointerError},
        ArchiveContext,
    };
    use bytecheck::{CheckBytes, Error};

    impl<T> ArchivedVec<T> {
        /// Checks the bytes of the `ArchivedVec` with the given element checking function.
        ///
        /// # Safety
        ///
        /// `check_elements` must ensure that the pointer given to it contains only valid data.
        pub unsafe fn check_bytes_with<'a, C, F>(
            value: *const Self,
            context: &mut C,
            check_elements: F,
        ) -> Result<&'a Self, CheckOwnedPointerError<[T], C>>
        where
            T: CheckBytes<C>,
            C: ArchiveContext + ?Sized,
            F: FnOnce(*const [T], &mut C) -> Result<(), <[T] as CheckBytes<C>>::Error>,
        {
            let rel_ptr = RelPtr::<[T]>::manual_check_bytes(value.cast(), context)
                .map_err(OwnedPointerError::PointerCheckBytesError)?;
            let ptr = context
                .check_subtree_rel_ptr(rel_ptr)
                .map_err(OwnedPointerError::ContextError)?;

            let range = context
                .push_prefix_subtree(ptr)
                .map_err(OwnedPointerError::ContextError)?;
            check_elements(ptr, context).map_err(OwnedPointerError::ValueCheckBytesError)?;
            context
                .pop_prefix_range(range)
                .map_err(OwnedPointerError::ContextError)?;

            Ok(&*value)
        }
    }

    impl<T, C> CheckBytes<C> for ArchivedVec<T>
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
            Self::check_bytes_with::<C, _>(value, context, |v, c| {
                <[T]>::check_bytes(v, c).map(|_| ())
            })
        }
    }
};
