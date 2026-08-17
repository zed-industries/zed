//! Validation implementations and helper types.

pub mod owned;
pub mod validators;

use crate::{Archive, ArchivePointee, CheckBytes, Fallible, RelPtr};
use core::{alloc::Layout, alloc::LayoutError, any::TypeId, fmt};
use ptr_meta::Pointee;
#[cfg(feature = "std")]
use std::error::Error;

// Replace this trait with core::mem::{align_of_val_raw, size_of_val_raw} when they get stabilized.

/// Gets the layout of a type from its pointee type and metadata.
pub trait LayoutRaw
where
    Self: Pointee,
{
    /// Gets the layout of the type.
    fn layout_raw(metadata: <Self as Pointee>::Metadata) -> Result<Layout, LayoutError>;
}

impl<T> LayoutRaw for T {
    #[inline]
    fn layout_raw(_: <Self as Pointee>::Metadata) -> Result<Layout, LayoutError> {
        Ok(Layout::new::<T>())
    }
}

impl<T> LayoutRaw for [T] {
    #[inline]
    fn layout_raw(metadata: <Self as Pointee>::Metadata) -> Result<Layout, LayoutError> {
        Layout::array::<T>(metadata)
    }
}

impl LayoutRaw for str {
    #[inline]
    fn layout_raw(metadata: <Self as Pointee>::Metadata) -> Result<Layout, LayoutError> {
        Layout::array::<u8>(metadata)
    }
}

#[cfg(feature = "std")]
impl LayoutRaw for ::std::ffi::CStr {
    #[inline]
    fn layout_raw(metadata: <Self as Pointee>::Metadata) -> Result<Layout, LayoutError> {
        Layout::array::<::std::os::raw::c_char>(metadata)
    }
}

/// A context that can validate nonlocal archive memory.
pub trait ArchiveContext: Fallible {
    /// A prefix range from an archive context.
    ///
    /// Ranges must be popped in the reverse order they are pushed.
    type PrefixRange: 'static;

    /// A suffix range from an archive context.
    ///
    /// Ranges must be popped in the reverse order they are pushed.
    type SuffixRange: 'static;

    /// Checks that a relative pointer points to an address within the archive.
    ///
    /// The returned pointer is not guaranteed to point to an object that is contained completely
    /// within the archive. Use [`bounds_check_layout`](ArchiveContext::bounds_check_layout) to
    /// verify that an object with some layout is located at the target address.
    ///
    /// # Safety
    ///
    /// - `base` must be inside the archive this validator was created for.
    unsafe fn bounds_check_ptr(
        &mut self,
        base: *const u8,
        offset: isize,
    ) -> Result<*const u8, Self::Error>;

    /// Checks that a given pointer can be dereferenced.
    ///
    /// The returned pointer is guaranteed to be located within the archive. This means that the
    /// returned pointer is safe to check, but may be vulnerable to memory overlap and recursion
    /// attacks unless the subtree range is properly restricted. Use `check_subtree_ptr` to perform
    /// the subtree range check as well.
    ///
    /// # Safety
    ///
    /// - `data_address` must be inside the archive this validator was created for.
    /// - `layout` must be the layout for the given pointer.
    unsafe fn bounds_check_layout(
        &mut self,
        data_address: *const u8,
        layout: &Layout,
    ) -> Result<(), Self::Error>;

    /// Checks that the given relative pointer can be dereferenced.
    ///
    /// The returned pointer is guaranteed to be located within the archive. This means that the
    /// returned pointer is safe to check, but may be vulnerable to memory overlap and recursion
    /// attacks unless the subtree range is properly restricted. Use `check_subtree_ptr` to perform
    /// the subtree range check as well.
    ///
    /// # Safety
    ///
    /// - `base` must be inside the archive this validator was created for.
    /// - `metadata` must be the metadata for the pointer defined by `base` and `offset`.
    #[inline]
    unsafe fn check_ptr<T: LayoutRaw + Pointee + ?Sized>(
        &mut self,
        base: *const u8,
        offset: isize,
        metadata: T::Metadata,
    ) -> Result<*const T, Self::Error> {
        let data_address = self.bounds_check_ptr(base, offset)?;
        let layout = T::layout_raw(metadata).map_err(Self::wrap_layout_error)?;
        let ptr = ptr_meta::from_raw_parts(data_address.cast(), metadata);
        self.bounds_check_layout(data_address, &layout)?;
        Ok(ptr)
    }

    /// Checks that the given `RelPtr` can be dereferenced.
    ///
    /// The returned pointer is guaranteed to be located within the archive. This means that the
    /// returned pointer is safe to check, but may be vulnerable to memory overlap and recursion
    /// attacks unless the subtree range is properly restricted. Use `check_subtree_ptr` to perform
    /// the subtree range check as well.
    ///
    /// # Safety
    ///
    /// - `rel_ptr` must be inside the archive this validator was created for.
    #[inline]
    unsafe fn check_rel_ptr<T: ArchivePointee + LayoutRaw + ?Sized>(
        &mut self,
        rel_ptr: &RelPtr<T>,
    ) -> Result<*const T, Self::Error> {
        let metadata = T::pointer_metadata(rel_ptr.metadata());
        self.check_ptr(rel_ptr.base(), rel_ptr.offset(), metadata)
    }

    /// Checks that the given data address and layout is located completely within the subtree
    /// range.
    ///
    /// # Safety
    ///
    /// - `data_address` must be inside the archive this validator was created for.
    unsafe fn bounds_check_subtree_ptr_layout(
        &mut self,
        data_address: *const u8,
        layout: &Layout,
    ) -> Result<(), Self::Error>;

    /// Checks that the given pointer is located completely within the subtree range.
    ///
    /// # Safety
    ///
    /// - `ptr` must be inside the archive this validator was created for.
    #[inline]
    unsafe fn bounds_check_subtree_ptr<T: LayoutRaw + ?Sized>(
        &mut self,
        ptr: *const T,
    ) -> Result<(), Self::Error> {
        let layout = T::layout_raw(ptr_meta::metadata(ptr)).map_err(Self::wrap_layout_error)?;
        self.bounds_check_subtree_ptr_layout(ptr.cast(), &layout)
    }

    /// Checks that the given relative pointer to a subtree can be dereferenced.
    ///
    /// # Safety
    ///
    /// - `base` must be inside the archive this validator was created for.
    /// - `metadata` must be the metadata for the pointer defined by `base` and `offset`.
    #[inline]
    unsafe fn check_subtree_ptr<T: LayoutRaw + Pointee + ?Sized>(
        &mut self,
        base: *const u8,
        offset: isize,
        metadata: T::Metadata,
    ) -> Result<*const T, Self::Error> {
        let ptr = self.check_ptr(base, offset, metadata)?;
        self.bounds_check_subtree_ptr(ptr)?;
        Ok(ptr)
    }

    /// Checks that the given `RelPtr` to a subtree can be dereferenced.
    ///
    /// # Safety
    ///
    /// - `rel_ptr` must be inside the archive this validator was created for.
    #[inline]
    unsafe fn check_subtree_rel_ptr<T: ArchivePointee + LayoutRaw + ?Sized>(
        &mut self,
        rel_ptr: &RelPtr<T>,
    ) -> Result<*const T, Self::Error> {
        let ptr = self.check_rel_ptr(rel_ptr)?;
        self.bounds_check_subtree_ptr(ptr)?;
        Ok(ptr)
    }

    /// Pushes a new subtree range onto the validator and starts validating it.
    ///
    /// After calling `push_subtree_claim_to`, the validator will have a subtree range starting at
    /// the original start and ending at `root`. After popping the returned range, the validator
    /// will have a subtree range starting at `end` and ending at the original end.
    ///
    /// # Safety
    ///
    /// `root` and `end` must be located inside the archive.
    unsafe fn push_prefix_subtree_range(
        &mut self,
        root: *const u8,
        end: *const u8,
    ) -> Result<Self::PrefixRange, Self::Error>;

    /// Pushes a new subtree range onto the validator and starts validating it.
    ///
    /// The claimed range spans from the end of `start` to the end of the current subobject range.
    ///
    /// # Safety
    ///
    /// `` must be located inside the archive.
    #[inline]
    unsafe fn push_prefix_subtree<T: LayoutRaw + ?Sized>(
        &mut self,
        root: *const T,
    ) -> Result<Self::PrefixRange, Self::Error> {
        let layout = T::layout_raw(ptr_meta::metadata(root)).map_err(Self::wrap_layout_error)?;
        self.push_prefix_subtree_range(root as *const u8, (root as *const u8).add(layout.size()))
    }

    /// Pops the given range, restoring the original state with the pushed range removed.
    ///
    /// If the range was not popped in reverse order, an error is returned.
    fn pop_prefix_range(&mut self, range: Self::PrefixRange) -> Result<(), Self::Error>;

    /// Pushes a new subtree range onto the validator and starts validating it.
    ///
    /// After calling `push_prefix_subtree_range`, the validator will have a subtree range starting
    /// at `start` and ending at `root`. After popping the returned range, the validator will have a
    /// subtree range starting at the original start and ending at `start`.
    ///
    /// # Safety
    ///
    /// `start` and `root` must be located inside the archive.
    unsafe fn push_suffix_subtree_range(
        &mut self,
        start: *const u8,
        root: *const u8,
    ) -> Result<Self::SuffixRange, Self::Error>;

    /// Finishes the given range, restoring the original state with the pushed range removed.
    ///
    /// If the range was not popped in reverse order, an error is returned.
    fn pop_suffix_range(&mut self, range: Self::SuffixRange) -> Result<(), Self::Error>;

    /// Wraps a layout error in an ArchiveContext error
    fn wrap_layout_error(error: LayoutError) -> Self::Error;

    /// Verifies that all outstanding claims have been returned.
    fn finish(&mut self) -> Result<(), Self::Error>;
}

/// A context that can validate shared archive memory.
///
/// Shared pointers require this kind of context to validate.
pub trait SharedContext: Fallible {
    /// Registers the given `ptr` as a shared pointer with the given type.
    ///
    /// Returns `true` if the pointer was newly-registered and `check_bytes` should be called.
    fn register_shared_ptr(&mut self, ptr: *const u8, type_id: TypeId)
        -> Result<bool, Self::Error>;
}

/// Errors that can occur when checking an archive.
#[derive(Debug)]
pub enum CheckArchiveError<T, C> {
    /// An error that occurred while validating an object
    CheckBytesError(T),
    /// A context error occurred
    ContextError(C),
}

impl<T: fmt::Display, C: fmt::Display> fmt::Display for CheckArchiveError<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckArchiveError::CheckBytesError(e) => write!(f, "check bytes error: {}", e),
            CheckArchiveError::ContextError(e) => write!(f, "context error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl<T: Error + 'static, C: Error + 'static> Error for CheckArchiveError<T, C> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CheckArchiveError::CheckBytesError(e) => Some(e as &dyn Error),
            CheckArchiveError::ContextError(e) => Some(e as &dyn Error),
        }
    }
}

/// The error type that can be produced by checking the given type with the given validator.
pub type CheckTypeError<T, C> =
    CheckArchiveError<<T as CheckBytes<C>>::Error, <C as Fallible>::Error>;

// TODO: change this to be the public-facing API (uses pos: isize instead of pos: usize)
#[inline]
fn internal_check_archived_value_with_context<'a, T, C>(
    buf: &'a [u8],
    pos: isize,
    context: &mut C,
) -> Result<&'a T::Archived, CheckTypeError<T::Archived, C>>
where
    T: Archive,
    T::Archived: CheckBytes<C> + Pointee<Metadata = ()>,
    C: ArchiveContext + ?Sized,
{
    unsafe {
        let ptr = context
            .check_subtree_ptr(buf.as_ptr(), pos, ())
            .map_err(CheckArchiveError::ContextError)?;

        let range = context
            .push_prefix_subtree(ptr)
            .map_err(CheckArchiveError::ContextError)?;
        let result =
            CheckBytes::check_bytes(ptr, context).map_err(CheckArchiveError::CheckBytesError)?;
        context
            .pop_prefix_range(range)
            .map_err(CheckArchiveError::ContextError)?;

        context.finish().map_err(CheckArchiveError::ContextError)?;
        Ok(result)
    }
}

/// Checks the given archive with an additional context.
///
/// See [`check_archived_value`](crate::validation::validators::check_archived_value) for more details.
#[inline]
pub fn check_archived_value_with_context<'a, T, C>(
    buf: &'a [u8],
    pos: usize,
    context: &mut C,
) -> Result<&'a T::Archived, CheckTypeError<T::Archived, C>>
where
    T: Archive,
    T::Archived: CheckBytes<C> + Pointee<Metadata = ()>,
    C: ArchiveContext + ?Sized,
{
    internal_check_archived_value_with_context::<T, C>(buf, pos as isize, context)
}

/// Checks the given archive with an additional context.
///
/// See [`check_archived_value`](crate::validation::validators::check_archived_value) for more details.
#[inline]
pub fn check_archived_root_with_context<'a, T, C>(
    buf: &'a [u8],
    context: &mut C,
) -> Result<&'a T::Archived, CheckTypeError<T::Archived, C>>
where
    T: Archive,
    T::Archived: CheckBytes<C> + Pointee<Metadata = ()>,
    C: ArchiveContext + ?Sized,
{
    internal_check_archived_value_with_context::<T, C>(
        buf,
        buf.len() as isize - core::mem::size_of::<T::Archived>() as isize,
        context,
    )
}
