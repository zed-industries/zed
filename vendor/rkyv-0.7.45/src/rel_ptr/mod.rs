//! Relative pointer implementations and options.

#[cfg(feature = "validation")]
mod validation;

use crate::{ArchivePointee, ArchiveUnsized, Archived};
use core::{
    convert::TryFrom,
    fmt,
    marker::{PhantomData, PhantomPinned},
    ptr,
};

/// An error where the distance between two positions cannot be represented by the offset type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetError {
    /// The offset overflowed the range of `isize`
    IsizeOverflow,
    /// The offset is too far for the offset type of the relative pointer
    ExceedsStorageRange,
}

impl fmt::Display for OffsetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OffsetError::IsizeOverflow => write!(f, "the offset overflowed the range of `isize`"),
            OffsetError::ExceedsStorageRange => write!(
                f,
                "the offset is too far for the offset type of the relative pointer"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OffsetError {}

/// Calculates the offset between two positions as an `isize`.
///
/// This function exists solely to get the distance between two `usizes` as an `isize` with a full
/// range of values.
///
/// # Examples
///
/// ```
/// use rkyv::rel_ptr::{signed_offset, OffsetError};
///
/// assert_eq!(signed_offset(0, 1), Ok(1));
/// assert_eq!(signed_offset(1, 0), Ok(-1));
/// assert_eq!(signed_offset(0, isize::MAX as usize), Ok(isize::MAX));
/// assert_eq!(signed_offset(isize::MAX as usize, 0), Ok(-isize::MAX));
/// assert_eq!(signed_offset(0, isize::MAX as usize + 1), Err(OffsetError::IsizeOverflow));
/// assert_eq!(signed_offset(isize::MAX as usize + 1, 0), Ok(isize::MIN));
/// assert_eq!(signed_offset(0, isize::MAX as usize + 2), Err(OffsetError::IsizeOverflow));
/// assert_eq!(signed_offset(isize::MAX as usize + 2, 0), Err(OffsetError::IsizeOverflow));
/// ```
#[inline]
pub fn signed_offset(from: usize, to: usize) -> Result<isize, OffsetError> {
    let (result, overflow) = to.overflowing_sub(from);
    if (!overflow && result <= (isize::MAX as usize))
        || (overflow && result >= (isize::MIN as usize))
    {
        Ok(result as isize)
    } else {
        Err(OffsetError::IsizeOverflow)
    }
}

/// A offset that can be used with [`RawRelPtr`].
pub trait Offset: Copy {
    /// Creates a new offset between a `from` position and a `to` position.
    fn between(from: usize, to: usize) -> Result<Self, OffsetError>;

    /// Gets the offset as an `isize`.
    fn to_isize(&self) -> isize;
}

macro_rules! impl_offset {
    ($ty:ty) => {
        impl Offset for $ty {
            #[inline]
            fn between(from: usize, to: usize) -> Result<Self, OffsetError> {
                // pointer::add and pointer::offset require that the computed offsets cannot
                // overflow an isize, which is why we're using signed_offset instead of checked_sub
                // for unsized types
                Self::try_from(signed_offset(from, to)?)
                    .map_err(|_| OffsetError::ExceedsStorageRange)
            }

            #[inline]
            fn to_isize(&self) -> isize {
                // We're guaranteed that our offset will not exceed the the capacity of an `isize`
                *self as isize
            }
        }
    };
    (@endian $ty:ty) => {
        impl Offset for Archived<$ty> {
            #[inline]
            fn between(from: usize, to: usize) -> Result<Self, OffsetError> {
                // pointer::add and pointer::offset require that the computed offsets cannot
                // overflow an isize, which is why we're using signed_offset instead of checked_sub
                // for unsized types
                <$ty>::try_from(signed_offset(from, to)?)
                    .map(|x| to_archived!(x))
                    .map_err(|_| OffsetError::ExceedsStorageRange)
            }

            #[inline]
            fn to_isize(&self) -> isize {
                // We're guaranteed that our offset will not exceed the the capacity of an `isize`
                from_archived!(*self) as isize
            }
        }
    };
}

impl_offset!(i8);
impl_offset!(@endian i16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_offset!(@endian i32);
#[cfg(target_pointer_width = "64")]
impl_offset!(@endian i64);
impl_offset!(u8);
impl_offset!(@endian u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_offset!(@endian u32);
#[cfg(target_pointer_width = "64")]
impl_offset!(@endian u64);

/// Errors that can occur while creating raw relative pointers.
#[derive(Debug)]
pub enum RelPtrError {
    /// The given `from` and `to` positions for the relative pointer failed to form a valid offset.
    ///
    /// This is probably because the distance between them could not be represented by the offset
    /// type.
    OffsetError,
}

/// An untyped pointer which resolves relative to its position in memory.
///
/// This is the most fundamental building block in rkyv. It allows the construction and use of
/// pointers that can be safely relocated as long as the source and target are moved together. This
/// is what allows memory to be moved from disk into memory and accessed without decoding.
///
/// Regular pointers are *absolute*, meaning that the pointee can be moved without invalidating the
/// pointer. However, the target cannot be moved or the pointer is invalidated.
///
/// Relative pointers are *relative*, meaning that the pointee can be moved with the target without
/// invalidating the pointer. However, if either the pointee or the target move independently, the
/// pointer will be invalidated.
#[repr(transparent)]
pub struct RawRelPtr<O> {
    offset: O,
    _phantom: PhantomPinned,
}

impl<O: Offset> RawRelPtr<O> {
    /// Attempts to create a new `RawRelPtr` in-place between the given `from` and `to` positions.
    ///
    /// # Safety
    ///
    /// - `out` must be located at position `from`
    /// - `to` must be a position within the archive
    #[inline]
    pub unsafe fn try_emplace(from: usize, to: usize, out: *mut Self) -> Result<(), OffsetError> {
        let offset = O::between(from, to)?;
        ptr::addr_of_mut!((*out).offset).write(offset);
        Ok(())
    }

    /// Creates a new `RawRelPtr` in-place between the given `from` and `to` positions.
    ///
    /// # Safety
    ///
    /// - `out` must be located at position `from`
    /// - `to` must be a position within the archive
    /// - The offset between `from` and `to` must fit in an `isize` and not exceed the offset
    ///   storage
    #[inline]
    pub unsafe fn emplace(from: usize, to: usize, out: *mut Self) {
        Self::try_emplace(from, to, out).unwrap();
    }

    /// Gets the base pointer for the relative pointer.
    #[inline]
    pub fn base(&self) -> *const u8 {
        (self as *const Self).cast::<u8>()
    }

    /// Gets the mutable base pointer for the relative pointer.
    #[inline]
    pub fn base_mut(&mut self) -> *mut u8 {
        (self as *mut Self).cast::<u8>()
    }

    /// Gets the offset of the relative pointer from its base.
    #[inline]
    pub fn offset(&self) -> isize {
        self.offset.to_isize()
    }

    /// Gets whether the offset of the relative pointer is 0.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.offset() == 0
    }

    /// Calculates the memory address being pointed to by this relative pointer.
    #[inline]
    pub fn as_ptr(&self) -> *const () {
        unsafe { self.base().offset(self.offset()).cast() }
    }

    /// Returns an unsafe mutable pointer to the memory address being pointed to
    /// by this relative pointer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut () {
        unsafe { self.base_mut().offset(self.offset()).cast() }
    }
}

impl<O: fmt::Debug> fmt::Debug for RawRelPtr<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawRelPtr")
            .field("offset", &self.offset)
            .finish()
    }
}

impl<O: Offset> fmt::Pointer for RawRelPtr<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

/// A raw relative pointer that uses an archived `i8` as the underlying offset.
pub type RawRelPtrI8 = RawRelPtr<Archived<i8>>;
/// A raw relative pointer that uses an archived `i16` as the underlying offset.
pub type RawRelPtrI16 = RawRelPtr<Archived<i16>>;
/// A raw relative pointer that uses an archived `i32` as the underlying offset.
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
pub type RawRelPtrI32 = RawRelPtr<Archived<i32>>;
/// A raw relative pointer that uses an archived `i64` as the underlying offset.
#[cfg(target_pointer_width = "64")]
pub type RawRelPtrI64 = RawRelPtr<Archived<i64>>;

/// A raw relative pointer that uses an archived `u8` as the underlying offset.
pub type RawRelPtrU8 = RawRelPtr<Archived<u8>>;
/// A raw relative pointer that uses an archived `u16` as the underlying offset.
pub type RawRelPtrU16 = RawRelPtr<Archived<u16>>;
/// A raw relative pointer that uses an archived `u32` as the underlying offset.
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
pub type RawRelPtrU32 = RawRelPtr<Archived<u32>>;
/// A raw relative pointer that uses an archived `u64` as the underlying offset.
#[cfg(target_pointer_width = "64")]
pub type RawRelPtrU64 = RawRelPtr<Archived<u64>>;

// TODO: implement for NonZero types

/// A pointer which resolves to relative to its position in memory.
///
/// This is a strongly-typed version of [`RawRelPtr`].
///
/// See [`Archive`](crate::Archive) for an example of creating one.
pub struct RelPtr<T: ArchivePointee + ?Sized, O> {
    raw_ptr: RawRelPtr<O>,
    metadata: T::ArchivedMetadata,
    _phantom: PhantomData<T>,
}

impl<T, O: Offset> RelPtr<T, O> {
    /// Attempts to create a relative pointer from one position to another.
    ///
    /// # Safety
    ///
    /// - `from` must be the position of `out` within the archive
    /// - `to` must be the position of some valid `T`
    #[inline]
    pub unsafe fn try_emplace(from: usize, to: usize, out: *mut Self) -> Result<(), OffsetError> {
        let (fp, fo) = out_field!(out.raw_ptr);
        // Skip metadata since sized T is guaranteed to be ()
        RawRelPtr::try_emplace(from + fp, to, fo)
    }

    /// Creates a relative pointer from one position to another.
    ///
    /// # Panics
    ///
    /// - The offset between `from` and `to` does not fit in an `isize`
    /// - The offset between `from` and `to` exceeds the offset storage
    ///
    /// # Safety
    ///
    /// - `from` must be the position of `out` within the archive
    /// - `to` must be the position of some valid `T`
    #[inline]
    pub unsafe fn emplace(from: usize, to: usize, out: *mut Self) {
        Self::try_emplace(from, to, out).unwrap();
    }
}

impl<T: ArchivePointee + ?Sized, O: Offset> RelPtr<T, O>
where
    T::ArchivedMetadata: Default,
{
    /// Attempts to create a null relative pointer with default metadata.
    ///
    /// # Safety
    ///
    /// `pos` must be the position of `out` within the archive.
    #[inline]
    pub unsafe fn try_emplace_null(pos: usize, out: *mut Self) -> Result<(), OffsetError> {
        let (fp, fo) = out_field!(out.raw_ptr);
        RawRelPtr::try_emplace(pos + fp, pos, fo)?;
        let (_, fo) = out_field!(out.metadata);
        fo.write(Default::default());
        Ok(())
    }

    /// Creates a null relative pointer with default metadata.
    ///
    /// # Panics
    ///
    /// - An offset of `0` does not fit in an `isize`
    /// - An offset of `0` exceeds the offset storage
    ///
    /// # Safety
    ///
    /// `pos` must be the position of `out` within the archive.
    #[inline]
    pub unsafe fn emplace_null(pos: usize, out: *mut Self) {
        Self::try_emplace_null(pos, out).unwrap()
    }
}

impl<T: ArchivePointee + ?Sized, O: Offset> RelPtr<T, O> {
    /// Attempts to create a relative pointer from one position to another.
    ///
    /// # Safety
    ///
    /// - `from` must be the position of `out` within the archive
    /// - `to` must be the position of some valid `T`
    /// - `value` must be the value being serialized
    /// - `metadata_resolver` must be the result of serializing the metadata of `value`
    #[inline]
    pub unsafe fn try_resolve_emplace<U: ArchiveUnsized<Archived = T> + ?Sized>(
        from: usize,
        to: usize,
        value: &U,
        metadata_resolver: U::MetadataResolver,
        out: *mut Self,
    ) -> Result<(), OffsetError> {
        let (fp, fo) = out_field!(out.raw_ptr);
        RawRelPtr::try_emplace(from + fp, to, fo)?;
        let (fp, fo) = out_field!(out.metadata);
        value.resolve_metadata(from + fp, metadata_resolver, fo);
        Ok(())
    }

    /// Creates a relative pointer from one position to another.
    ///
    /// # Panics
    ///
    /// - The offset between `from` and `to` does not fit in an `isize`
    /// - The offset between `from` and `to` exceeds the offset storage
    ///
    /// # Safety
    ///
    /// - `from` must be the position of `out` within the archive
    /// - `to` must be the position of some valid `T`
    /// - `value` must be the value being serialized
    /// - `metadata_resolver` must be the result of serializing the metadata of `value`
    #[inline]
    pub unsafe fn resolve_emplace<U: ArchiveUnsized<Archived = T> + ?Sized>(
        from: usize,
        to: usize,
        value: &U,
        metadata_resolver: U::MetadataResolver,
        out: *mut Self,
    ) {
        Self::try_resolve_emplace(from, to, value, metadata_resolver, out).unwrap();
    }

    /// Attempts to create a relative pointer from one position to another given
    /// raw pointer metadata.
    ///
    /// This does the same thing as [`RelPtr::try_resolve_emplace`] but you must supply
    /// the [`<T as ArchivePointee>::ArchivedMetadata`][ArchivePointee::ArchivedMetadata]
    /// yourself directly rather than through an implementation of [`ArchiveUnsized`] on some
    /// value.
    ///
    /// # Safety
    ///
    /// - `from` must be the position of `out` within the archive
    /// - `to` must be the position of some valid `T`
    /// - `value` must be the value being serialized
    /// - `archived_metadata` must produce valid metadata for the pointee of the resulting
    /// `RelPtr` (the thing being pointed at) when [`<T as ArchivePointee>::pointer_metadata(archived_metadata)`][ArchivePointee::pointer_metadata]
    /// is called.
    pub unsafe fn try_resolve_emplace_from_raw_parts(
        from: usize,
        to: usize,
        archived_metadata: <T as ArchivePointee>::ArchivedMetadata,
        out: *mut Self,
    ) -> Result<(), OffsetError> {
        let (fp, fo) = out_field!(out.raw_ptr);
        RawRelPtr::try_emplace(from + fp, to, fo)?;
        let (_fp, fo) = out_field!(out.metadata);
        *fo = archived_metadata;
        Ok(())
    }

    /// Creates a relative pointer from one position to another given
    /// raw pointer metadata.
    ///
    /// This does the same thing as [`RelPtr::resolve_emplace`] but you must supply
    /// the [`<T as ArchivePointee>::ArchivedMetadata`][ArchivePointee::ArchivedMetadata]
    /// yourself directly rather than through an implementation of [`ArchiveUnsized`] on some
    /// value.
    ///
    /// # Panics
    ///
    /// - The offset between `from` and `to` does not fit in an `isize`
    /// - The offset between `from` and `to` exceeds the offset storage
    ///
    /// # Safety
    ///
    /// - `from` must be the position of `out` within the archive
    /// - `to` must be the position of some valid `T`
    /// - `value` must be the value being serialized
    /// - `archived_metadata` must produce valid metadata for the pointee of the resulting
    /// `RelPtr` (the thing being pointed at) when [`<T as ArchivePointee>::pointer_metadata(archived_metadata)`][ArchivePointee::pointer_metadata]
    /// is called.
    pub unsafe fn resolve_emplace_from_raw_parts(
        from: usize,
        to: usize,
        archived_metadata: <T as ArchivePointee>::ArchivedMetadata,
        out: *mut Self,
    ) {
        Self::try_resolve_emplace_from_raw_parts(from, to, archived_metadata, out).unwrap();
    }

    /// Gets the base pointer for the relative pointer.
    #[inline]
    pub fn base(&self) -> *const u8 {
        self.raw_ptr.base()
    }

    /// Gets the mutable base pointer for the relative pointer.
    #[inline]
    pub fn base_mut(&mut self) -> *mut u8 {
        self.raw_ptr.base_mut()
    }

    /// Gets the offset of the relative pointer from its base.
    #[inline]
    pub fn offset(&self) -> isize {
        self.raw_ptr.offset()
    }

    /// Gets whether the offset of the relative pointer is 0.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.raw_ptr.is_null()
    }

    /// Gets the metadata of the relative pointer.
    #[inline]
    pub fn metadata(&self) -> &T::ArchivedMetadata {
        &self.metadata
    }

    /// Calculates the memory address being pointed to by this relative pointer.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        ptr_meta::from_raw_parts(self.raw_ptr.as_ptr(), T::pointer_metadata(&self.metadata))
    }

    /// Returns an unsafe mutable pointer to the memory address being pointed to by this relative
    /// pointer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        ptr_meta::from_raw_parts_mut(
            self.raw_ptr.as_mut_ptr(),
            T::pointer_metadata(&self.metadata),
        )
    }
}

impl<T: ArchivePointee + ?Sized, O: fmt::Debug> fmt::Debug for RelPtr<T, O>
where
    T::ArchivedMetadata: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RelPtr")
            .field("raw_ptr", &self.raw_ptr)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl<T: ArchivePointee + ?Sized, O: Offset> fmt::Pointer for RelPtr<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}
