//! Validation implementation for ArchivedHashIndex.

use crate::{collections::ArchivedHashIndex, validation::ArchiveContext, Archived, RelPtr};
use bytecheck::{CheckBytes, Error, SliceCheckError};
use core::{
    alloc::{Layout, LayoutError},
    convert::Infallible,
    fmt, ptr,
};

/// Errors that can occur while checking an archived hash index.
#[derive(Debug)]
pub enum HashIndexError<C> {
    /// An error occurred while checking the layouts of displacements or entries
    LayoutError(LayoutError),
    /// A displacement value was invalid
    InvalidDisplacement {
        /// The index of the entry with an invalid displacement
        index: usize,
        /// The value of the entry at the invalid location
        value: u32,
    },
    /// A bounds error occurred
    ContextError(C),
}

impl<C> From<LayoutError> for HashIndexError<C> {
    #[inline]
    fn from(e: LayoutError) -> Self {
        Self::LayoutError(e)
    }
}

impl<C> From<Infallible> for HashIndexError<C> {
    #[inline]
    fn from(_: Infallible) -> Self {
        unsafe { core::hint::unreachable_unchecked() }
    }
}

impl<C> From<SliceCheckError<Infallible>> for HashIndexError<C> {
    #[inline]
    fn from(_: SliceCheckError<Infallible>) -> Self {
        unsafe { core::hint::unreachable_unchecked() }
    }
}

impl<C: fmt::Display> fmt::Display for HashIndexError<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashIndexError::LayoutError(e) => write!(f, "layout error: {}", e),
            HashIndexError::InvalidDisplacement { index, value } => write!(
                f,
                "invalid displacement: value {} at index {}",
                value, index,
            ),
            HashIndexError::ContextError(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl<C: Error + 'static> Error for HashIndexError<C> {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                HashIndexError::LayoutError(e) => Some(e as &dyn Error),
                HashIndexError::InvalidDisplacement { .. } => None,
                HashIndexError::ContextError(e) => Some(e as &dyn Error),
            }
        }
    }
};

impl<C: ArchiveContext + ?Sized> CheckBytes<C> for ArchivedHashIndex
where
    C::Error: Error,
{
    type Error = HashIndexError<C::Error>;

    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        let len = from_archived!(*Archived::<usize>::check_bytes(
            ptr::addr_of!((*value).len),
            context,
        )?) as usize;
        Layout::array::<Archived<u32>>(len)?;

        let displace_rel_ptr =
            RelPtr::manual_check_bytes(ptr::addr_of!((*value).displace), context)?;
        let displace_ptr = context
            .check_subtree_ptr::<[Archived<u32>]>(
                displace_rel_ptr.base(),
                displace_rel_ptr.offset(),
                len,
            )
            .map_err(HashIndexError::ContextError)?;

        let range = context
            .push_prefix_subtree(displace_ptr)
            .map_err(HashIndexError::ContextError)?;
        let displace = <[Archived<u32>]>::check_bytes(displace_ptr, context)?;
        context
            .pop_prefix_range(range)
            .map_err(HashIndexError::ContextError)?;

        for (i, &d) in displace.iter().enumerate() {
            let d = from_archived!(d);
            if d as usize >= len && d < 0x80_00_00_00 {
                return Err(HashIndexError::InvalidDisplacement { index: i, value: d });
            }
        }

        Ok(&*value)
    }
}
