//! Validation implementation for ArchivedIndexMap.

use crate::{
    collections::{
        hash_index::validation::HashIndexError,
        index_map::ArchivedIndexMap,
        util::{validation::ArchivedEntryError, Entry},
        ArchivedHashIndex,
    },
    validation::ArchiveContext,
    Archived, RelPtr,
};
use bytecheck::{CheckBytes, Error, SliceCheckError};
use core::{
    alloc::{Layout, LayoutError},
    convert::Infallible,
    fmt,
    hash::Hash,
    ptr,
};

/// Errors that can occur while checking an archived index map.
#[derive(Debug)]
pub enum IndexMapError<K, V, C> {
    /// An error occurred while checking the hash index
    HashIndexError(HashIndexError<C>),
    /// An error occurred while checking the layouts of displacements or entries
    LayoutError(LayoutError),
    /// A pivot indexes outside of the entries array
    PivotOutOfBounds {
        /// The index of the pivot when iterating
        index: usize,
        /// The pivot value that was invalid
        pivot: usize,
    },
    /// An error occurred while checking the entries
    CheckEntryError(SliceCheckError<ArchivedEntryError<K, V>>),
    /// A key is not located at the correct position
    ///
    /// This can either be due to the key being invalid for the hash index, or the pivot for the key
    /// not pointing to it.
    InvalidKeyPosition {
        /// The index of the key when iterating
        index: usize,
    },
    /// A bounds error occurred
    ContextError(C),
}

impl<K: fmt::Display, V: fmt::Display, E: fmt::Display> fmt::Display for IndexMapError<K, V, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexMapError::HashIndexError(e) => write!(f, "hash index check error: {}", e),
            IndexMapError::LayoutError(e) => write!(f, "layout error: {}", e),
            IndexMapError::PivotOutOfBounds { index, pivot } => {
                write!(f, "pivot out of bounds: {} at index {}", pivot, index)
            }
            IndexMapError::CheckEntryError(e) => write!(f, "entry check error: {}", e),
            IndexMapError::InvalidKeyPosition { index } => {
                write!(f, "invalid key position: at index {}", index)
            }
            IndexMapError::ContextError(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl<K, V, C> Error for IndexMapError<K, V, C>
    where
        K: Error + 'static,
        V: Error + 'static,
        C: Error + 'static,
    {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                IndexMapError::HashIndexError(e) => Some(e as &dyn Error),
                IndexMapError::LayoutError(e) => Some(e as &dyn Error),
                IndexMapError::PivotOutOfBounds { .. } => None,
                IndexMapError::CheckEntryError(e) => Some(e as &dyn Error),
                IndexMapError::InvalidKeyPosition { .. } => None,
                IndexMapError::ContextError(e) => Some(e as &dyn Error),
            }
        }
    }
};

impl<K, V, C> From<Infallible> for IndexMapError<K, V, C> {
    fn from(_: Infallible) -> Self {
        unsafe { core::hint::unreachable_unchecked() }
    }
}

impl<K, V, C> From<SliceCheckError<Infallible>> for IndexMapError<K, V, C> {
    #[inline]
    fn from(_: SliceCheckError<Infallible>) -> Self {
        unsafe { core::hint::unreachable_unchecked() }
    }
}

impl<K, V, C> From<HashIndexError<C>> for IndexMapError<K, V, C> {
    #[inline]
    fn from(e: HashIndexError<C>) -> Self {
        Self::HashIndexError(e)
    }
}

impl<K, V, C> From<LayoutError> for IndexMapError<K, V, C> {
    #[inline]
    fn from(e: LayoutError) -> Self {
        Self::LayoutError(e)
    }
}

impl<K, V, C> From<SliceCheckError<ArchivedEntryError<K, V>>> for IndexMapError<K, V, C> {
    #[inline]
    fn from(e: SliceCheckError<ArchivedEntryError<K, V>>) -> Self {
        Self::CheckEntryError(e)
    }
}

impl<K, V, C> CheckBytes<C> for ArchivedIndexMap<K, V>
where
    K: CheckBytes<C> + Eq + Hash,
    V: CheckBytes<C>,
    C: ArchiveContext + ?Sized,
    C::Error: Error,
{
    type Error = IndexMapError<K::Error, V::Error, C::Error>;

    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        let index = ArchivedHashIndex::check_bytes(ptr::addr_of!((*value).index), context)?;

        // Entries
        Layout::array::<Entry<K, V>>(index.len())?;
        let entries_rel_ptr = RelPtr::manual_check_bytes(ptr::addr_of!((*value).entries), context)?;
        let entries_ptr = context
            .check_subtree_ptr::<[Entry<K, V>]>(
                entries_rel_ptr.base(),
                entries_rel_ptr.offset(),
                index.len(),
            )
            .map_err(IndexMapError::ContextError)?;

        let range = context
            .push_prefix_subtree(entries_ptr)
            .map_err(IndexMapError::ContextError)?;
        let entries = <[Entry<K, V>]>::check_bytes(entries_ptr, context)?;
        context
            .pop_prefix_range(range)
            .map_err(IndexMapError::ContextError)?;

        // Pivots
        Layout::array::<Archived<usize>>(index.len())?;
        let pivots_rel_ptr = RelPtr::manual_check_bytes(ptr::addr_of!((*value).pivots), context)?;
        let pivots_ptr = context
            .check_subtree_ptr::<[Archived<usize>]>(
                pivots_rel_ptr.base(),
                pivots_rel_ptr.offset(),
                index.len(),
            )
            .map_err(IndexMapError::ContextError)?;

        let range = context
            .push_prefix_subtree(pivots_ptr)
            .map_err(IndexMapError::ContextError)?;
        let pivots = <[Archived<usize>]>::check_bytes(pivots_ptr, context)?;
        context
            .pop_prefix_range(range)
            .map_err(IndexMapError::ContextError)?;

        for (i, pivot) in pivots.iter().enumerate() {
            let pivot = from_archived!(*pivot) as usize;
            if pivot >= index.len() {
                return Err(IndexMapError::PivotOutOfBounds { index: i, pivot });
            }
        }

        for (i, entry) in entries.iter().enumerate() {
            if let Some(pivot_index) = index.index(&entry.key) {
                let pivot = from_archived!(pivots[pivot_index]) as usize;
                if pivot != i {
                    return Err(IndexMapError::InvalidKeyPosition { index: i });
                }
            } else {
                return Err(IndexMapError::InvalidKeyPosition { index: i });
            }
        }

        Ok(&*value)
    }
}
