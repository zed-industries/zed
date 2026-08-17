//! Serialization traits, serializers, and adapters.

pub mod serializers;

use crate::{Archive, ArchiveUnsized, Fallible, RelPtr, Serialize, SerializeUnsized};
use core::{alloc::Layout, mem, ptr::NonNull, slice};

/// A byte sink that knows where it is.
///
/// A type that is [`io::Write`](std::io::Write) can be wrapped in a
/// [`WriteSerializer`](serializers::WriteSerializer) to equip it with `Serializer`.
///
/// It's important that the memory for archived objects is properly aligned before attempting to
/// read objects out of it; use an [`AlignedVec`](crate::AlignedVec) or the
/// [`AlignedBytes`](crate::AlignedBytes) wrappers if they are appropriate.
pub trait Serializer: Fallible {
    /// Returns the current position of the serializer.
    fn pos(&self) -> usize;

    /// Attempts to write the given bytes to the serializer.
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;

    /// Advances the given number of bytes as padding.
    #[inline]
    fn pad(&mut self, padding: usize) -> Result<(), Self::Error> {
        const MAX_ZEROES: usize = 32;
        const ZEROES: [u8; MAX_ZEROES] = [0; MAX_ZEROES];
        debug_assert!(padding < MAX_ZEROES);

        self.write(&ZEROES[0..padding])
    }

    /// Aligns the position of the serializer to the given alignment.
    #[inline]
    fn align(&mut self, align: usize) -> Result<usize, Self::Error> {
        let mask = align - 1;
        debug_assert_eq!(align & mask, 0);

        self.pad((align - (self.pos() & mask)) & mask)?;
        Ok(self.pos())
    }

    /// Aligns the position of the serializer to be suitable to write the given type.
    #[inline]
    fn align_for<T>(&mut self) -> Result<usize, Self::Error> {
        self.align(mem::align_of::<T>())
    }

    /// Resolves the given value with its resolver and writes the archived type.
    ///
    /// Returns the position of the written archived type.
    ///
    /// # Safety
    ///
    /// - `resolver` must be the result of serializing `value`
    /// - The serializer must be aligned for a `T::Archived`
    unsafe fn resolve_aligned<T: Archive + ?Sized>(
        &mut self,
        value: &T,
        resolver: T::Resolver,
    ) -> Result<usize, Self::Error> {
        let pos = self.pos();
        debug_assert_eq!(pos & (mem::align_of::<T::Archived>() - 1), 0);

        let mut resolved = mem::MaybeUninit::<T::Archived>::uninit();
        resolved.as_mut_ptr().write_bytes(0, 1);
        value.resolve(pos, resolver, resolved.as_mut_ptr());

        let data = resolved.as_ptr().cast::<u8>();
        let len = mem::size_of::<T::Archived>();
        self.write(slice::from_raw_parts(data, len))?;
        Ok(pos)
    }

    /// Archives the given object and returns the position it was archived at.
    #[inline]
    fn serialize_value<T: Serialize<Self>>(&mut self, value: &T) -> Result<usize, Self::Error> {
        let resolver = value.serialize(self)?;
        self.align_for::<T::Archived>()?;
        unsafe { self.resolve_aligned(value, resolver) }
    }

    /// Resolves the given reference with its resolver and writes the archived reference.
    ///
    /// Returns the position of the written archived `RelPtr`.
    ///
    /// # Safety
    ///
    /// - `metadata_resolver` must be the result of serializing the metadata of `value`
    /// - `to` must be the position of the serialized `value` within the archive
    /// - The serializer must be aligned for a `RelPtr<T::Archived>`
    unsafe fn resolve_unsized_aligned<T: ArchiveUnsized + ?Sized>(
        &mut self,
        value: &T,
        to: usize,
        metadata_resolver: T::MetadataResolver,
    ) -> Result<usize, Self::Error> {
        let from = self.pos();
        debug_assert_eq!(from & (mem::align_of::<RelPtr<T::Archived>>() - 1), 0);

        let mut resolved = mem::MaybeUninit::<RelPtr<T::Archived>>::uninit();
        resolved.as_mut_ptr().write_bytes(0, 1);
        value.resolve_unsized(from, to, metadata_resolver, resolved.as_mut_ptr());

        let data = resolved.as_ptr().cast::<u8>();
        let len = mem::size_of::<RelPtr<T::Archived>>();
        self.write(slice::from_raw_parts(data, len))?;
        Ok(from)
    }

    /// Archives a reference to the given object and returns the position it was archived at.
    #[inline]
    fn serialize_unsized_value<T: SerializeUnsized<Self> + ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<usize, Self::Error> {
        let to = value.serialize_unsized(self)?;
        let metadata_resolver = value.serialize_metadata(self)?;
        self.align_for::<RelPtr<T::Archived>>()?;
        unsafe { self.resolve_unsized_aligned(value, to, metadata_resolver) }
    }
}

// Someday this can probably be replaced with alloc::Allocator

/// A serializer that can allocate scratch space.
pub trait ScratchSpace: Fallible {
    /// Allocates scratch space of the requested size.
    ///
    /// # Safety
    ///
    /// `layout` must have non-zero size.
    unsafe fn push_scratch(&mut self, layout: Layout) -> Result<NonNull<[u8]>, Self::Error>;

    /// Deallocates previously allocated scratch space.
    ///
    /// # Safety
    ///
    /// - `ptr` must be the scratch memory last allocated with `push_scratch`.
    /// - `layout` must be the same layout that was used to allocate that block of memory.
    unsafe fn pop_scratch(&mut self, ptr: NonNull<u8>, layout: Layout) -> Result<(), Self::Error>;
}

/// A registry that tracks serialized shared memory.
///
/// This trait is required to serialize shared pointers.
pub trait SharedSerializeRegistry: Fallible {
    /// Gets the position of a previously-added shared pointer.
    ///
    /// Returns `None` if the pointer has not yet been added.
    fn get_shared_ptr(&self, value: *const u8) -> Option<usize>;

    /// Gets the position of a previously-added shared value.
    ///
    /// Returns `None` if the value has not yet been added.
    #[inline]
    fn get_shared<T: ?Sized>(&self, value: &T) -> Option<usize> {
        self.get_shared_ptr(value as *const T as *const u8)
    }

    /// Adds the position of a shared pointer to the registry.
    fn add_shared_ptr(&mut self, value: *const u8, pos: usize) -> Result<(), Self::Error>;

    /// Adds the position of a shared value to the registry.
    #[inline]
    fn add_shared<T: ?Sized>(&mut self, value: &T, pos: usize) -> Result<(), Self::Error> {
        self.add_shared_ptr(value as *const T as *const u8, pos)
    }

    /// Archives the given shared value and returns its position. If the value has already been
    /// added then it returns the position of the previously added value.
    #[inline]
    fn serialize_shared<T: SerializeUnsized<Self> + ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<usize, Self::Error>
    where
        Self: Serializer,
    {
        if let Some(pos) = self.get_shared(value) {
            Ok(pos)
        } else {
            let pos = value.serialize_unsized(self)?;
            self.add_shared(value, pos)?;
            Ok(pos)
        }
    }
}
