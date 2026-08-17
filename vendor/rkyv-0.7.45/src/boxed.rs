//! An archived version of `Box`.

use crate::{
    ser::Serializer, ArchivePointee, ArchiveUnsized, Fallible, MetadataResolver, RelPtr, Serialize,
    SerializeUnsized,
};
use core::{borrow::Borrow, cmp, fmt, hash, ops::Deref, pin::Pin};

/// An archived [`Box`].
///
/// This is a thin wrapper around a [`RelPtr`] to the archived type.
#[repr(transparent)]
pub struct ArchivedBox<T: ArchivePointee + ?Sized>(RelPtr<T>);

impl<T: ArchivePointee + ?Sized> ArchivedBox<T> {
    /// Returns a reference to the value of this archived box.
    #[inline]
    pub fn get(&self) -> &T {
        unsafe { &*self.0.as_ptr() }
    }

    /// Returns a pinned mutable reference to the value of this archived box
    #[inline]
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T> {
        unsafe { self.map_unchecked_mut(|s| &mut *s.0.as_mut_ptr()) }
    }

    /// Resolves an archived box from the given value and parameters.
    ///
    /// # Safety
    ///
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of serializing `value`
    #[inline]
    pub unsafe fn resolve_from_ref<U: ArchiveUnsized<Archived = T> + ?Sized>(
        value: &U,
        pos: usize,
        resolver: BoxResolver<U::MetadataResolver>,
        out: *mut Self,
    ) {
        let (fp, fo) = out_field!(out.0);
        value.resolve_unsized(pos + fp, resolver.pos, resolver.metadata_resolver, fo);
    }

    /// Serializes an archived box from the given value and serializer.
    #[inline]
    pub fn serialize_from_ref<U, S>(
        value: &U,
        serializer: &mut S,
    ) -> Result<BoxResolver<U::MetadataResolver>, S::Error>
    where
        U: SerializeUnsized<S, Archived = T> + ?Sized,
        S: Fallible + ?Sized,
    {
        Ok(BoxResolver {
            pos: value.serialize_unsized(serializer)?,
            metadata_resolver: value.serialize_metadata(serializer)?,
        })
    }

    /// Resolves an archived box from a [`BoxResolver`] which contains
    /// the raw [`<T as ArchivePointee>::ArchivedMetadata`] directly.
    ///
    /// # Safety
    ///
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be obtained by following the safety documentation of
    /// [`BoxResolver::from_raw_parts`].
    ///
    /// [`<T as ArchivePointee>::ArchivedMetadata`]: ArchivePointee::ArchivedMetadata
    pub unsafe fn resolve_from_raw_parts(
        pos: usize,
        resolver: BoxResolver<<T as ArchivePointee>::ArchivedMetadata>,
        out: *mut Self,
    ) {
        let (fp, fo) = out_field!(out.0);
        RelPtr::resolve_emplace_from_raw_parts(
            pos + fp,
            resolver.pos,
            resolver.metadata_resolver,
            fo,
        );
    }

    #[doc(hidden)]
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl<T> ArchivedBox<[T]> {
    /// Serializes an archived `Box` from a given slice by directly copying bytes.
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
    ) -> Result<BoxResolver<MetadataResolver<[U]>>, S::Error>
    where
        U: Serialize<S, Archived = T>,
        S: Serializer + ?Sized,
    {
        use ::core::{mem::size_of, slice::from_raw_parts};

        let pos = serializer.align_for::<T>()?;

        let bytes = from_raw_parts(slice.as_ptr().cast::<u8>(), size_of::<T>() * slice.len());
        serializer.write(bytes)?;

        Ok(BoxResolver {
            pos,
            metadata_resolver: (),
        })
    }
}

impl<T: ArchivePointee + ?Sized> ArchivedBox<T>
where
    T::ArchivedMetadata: Default,
{
    #[doc(hidden)]
    #[inline]
    pub unsafe fn emplace_null(pos: usize, out: *mut Self) {
        let (fp, fo) = out_field!(out.0);
        RelPtr::emplace_null(pos + fp, fo);
    }
}

impl<T: ArchivePointee + ?Sized> AsRef<T> for ArchivedBox<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T: ArchivePointee + ?Sized> Borrow<T> for ArchivedBox<T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T: ArchivePointee + ?Sized> fmt::Debug for ArchivedBox<T>
where
    T::ArchivedMetadata: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ArchivedBox").field(&self.0).finish()
    }
}

impl<T: ArchivePointee + ?Sized> Deref for ArchivedBox<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T: ArchivePointee + fmt::Display + ?Sized> fmt::Display for ArchivedBox<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl<T: ArchivePointee + Eq + ?Sized> Eq for ArchivedBox<T> {}

impl<T: ArchivePointee + hash::Hash + ?Sized> hash::Hash for ArchivedBox<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

impl<T: ArchivePointee + Ord + ?Sized> Ord for ArchivedBox<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<T: ArchivePointee + PartialEq<U> + ?Sized, U: ArchivePointee + ?Sized>
    PartialEq<ArchivedBox<U>> for ArchivedBox<T>
{
    #[inline]
    fn eq(&self, other: &ArchivedBox<U>) -> bool {
        self.get().eq(other.get())
    }
}

impl<T: ArchivePointee + PartialOrd + ?Sized> PartialOrd for ArchivedBox<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.get().partial_cmp(other.get())
    }
}

impl<T: ArchivePointee + ?Sized> fmt::Pointer for ArchivedBox<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = self.get() as *const T;
        fmt::Pointer::fmt(&ptr, f)
    }
}

/// The resolver for `Box`.
pub struct BoxResolver<M> {
    pos: usize,
    metadata_resolver: M,
}

impl<M> BoxResolver<M> {
    /// Create a a new [`BoxResolver<M>`] from raw parts. Note that `M` here is ***not*** the same
    /// `T` which should be serialized/contained in the resulting [`ArchivedBox<T>`], and is rather
    /// a type that can be used to resolve any needed [`ArchivePointee::ArchivedMetadata`]
    /// for the serialized pointed-to value.
    ///
    /// In most cases, you won't need to create a [`BoxResolver`] yourself and can instead obtain it through
    /// [`ArchivedBox::serialize_from_ref`] or [`ArchivedBox::serialize_copy_from_slice`].
    ///
    /// # Safety
    ///
    /// Technically no unsafety can happen directly from calling this function, however, passing this as a resolver to
    /// [`ArchivedBox`]'s resolving functions absolutely can. In general this should be treated as a semi-private type, as
    /// constructing a valid resolver is quite fraught. Please make sure you understand what the implications are before doing it.
    ///
    /// - `pos`: You must ensure that you serialized and resolved (i.e. [`Serializer::serialize_value`])
    /// a `T` which will be pointed to by the final [`ArchivedBox<T>`] that this resolver will help resolve
    /// at the given `pos` within the archive.
    ///
    /// - `metadata_resolver`: You must also ensure that the given `metadata_resolver` can be used to successfully produce
    /// valid [`<T as ArchivePointee>::ArchivedMetadata`] for that serialized `T`. This means it must either be:
    ///     - The necessary [`<T as ArchivePointee>::ArchivedMetadata`] itself, in which case you may use the created
    /// `BoxResolver<<T as ArchivePointee>::ArchivedMetadata>` as a resolver in [`ArchivedBox::resolve_from_raw_parts`]
    ///     - An [`ArchiveUnsized::MetadataResolver`] obtained from some `value: &U` where `U: ArchiveUnsized<Archived = T>`, in which case you
    /// must pass that same `value: &U` into [`ArchivedBox::resolve_from_ref`] along with this [`BoxResolver`].
    ///
    /// [`<T as ArchivePointee>::ArchivedMetadata`]: ArchivePointee::ArchivedMetadata
    pub unsafe fn from_raw_parts(pos: usize, metadata_resolver: M) -> Self {
        Self {
            pos,
            metadata_resolver,
        }
    }
}

#[cfg(feature = "validation")]
const _: () = {
    use crate::validation::{
        owned::{CheckOwnedPointerError, OwnedPointerError},
        ArchiveContext, LayoutRaw,
    };
    use bytecheck::{CheckBytes, Error};
    use ptr_meta::Pointee;

    impl<T, C> CheckBytes<C> for ArchivedBox<T>
    where
        T: ArchivePointee + CheckBytes<C> + LayoutRaw + Pointee + ?Sized,
        C: ArchiveContext + ?Sized,
        T::ArchivedMetadata: CheckBytes<C>,
        C::Error: Error,
    {
        type Error = CheckOwnedPointerError<T, C>;

        #[inline]
        unsafe fn check_bytes<'a>(
            value: *const Self,
            context: &mut C,
        ) -> Result<&'a Self, Self::Error> {
            let rel_ptr = RelPtr::<T>::manual_check_bytes(value.cast(), context)
                .map_err(OwnedPointerError::PointerCheckBytesError)?;
            let ptr = context
                .check_subtree_rel_ptr(rel_ptr)
                .map_err(OwnedPointerError::ContextError)?;

            let range = context
                .push_prefix_subtree(ptr)
                .map_err(OwnedPointerError::ContextError)?;
            T::check_bytes(ptr, context).map_err(OwnedPointerError::ValueCheckBytesError)?;
            context
                .pop_prefix_range(range)
                .map_err(OwnedPointerError::ContextError)?;

            Ok(&*value)
        }
    }
};
