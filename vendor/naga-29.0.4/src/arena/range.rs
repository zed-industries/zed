//! Well-typed ranges of [`Arena`]s.
//!
//! This module defines the [`Range`] type, representing a contiguous range of
//! entries in an [`Arena`].
//!
//! [`Arena`]: super::Arena

use core::{fmt, marker::PhantomData, ops};

use super::{
    handle::{Handle, Index},
    Arena,
};

/// A strongly typed range of handles.
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(transparent)
)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(test, derive(PartialEq))]
pub struct Range<T> {
    pub(super) inner: ops::Range<u32>,
    #[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(skip))]
    marker: PhantomData<T>,
}

impl<T> Range<T> {
    pub(crate) const fn erase_type(self) -> Range<()> {
        let Self { inner, marker: _ } = self;
        Range {
            inner,
            marker: PhantomData,
        }
    }
}

// NOTE: Keep this diagnostic in sync with that of [`BadHandle`].
#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
#[error("Handle range {range:?} of {kind} is either not present, or inaccessible yet")]
pub struct BadRangeError {
    // This error is used for many `Handle` types, but there's no point in making this generic, so
    // we just flatten them all to `Handle<()>` here.
    kind: &'static str,
    range: Range<()>,
}

impl BadRangeError {
    pub fn new<T>(range: Range<T>) -> Self {
        Self {
            kind: core::any::type_name::<T>(),
            range: range.erase_type(),
        }
    }
}

impl<T> Clone for Range<T> {
    fn clone(&self) -> Self {
        Range {
            inner: self.inner.clone(),
            marker: self.marker,
        }
    }
}

impl<T> fmt::Debug for Range<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "[{}..{}]", self.inner.start, self.inner.end)
    }
}

impl<T> Iterator for Range<T> {
    type Item = Handle<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.start < self.inner.end {
            let next = self.inner.start;
            self.inner.start += 1;
            Some(Handle::new(Index::new(next).unwrap()))
        } else {
            None
        }
    }
}

impl<T> Range<T> {
    /// Return a range enclosing handles `first` through `last`, inclusive.
    pub fn new_from_bounds(first: Handle<T>, last: Handle<T>) -> Self {
        Self {
            inner: (first.index() as u32)..(last.index() as u32 + 1),
            marker: Default::default(),
        }
    }

    /// Return a range covering all handles with indices from `0` to `size`.
    pub(super) fn full_range_from_size(size: usize) -> Self {
        Self {
            inner: 0..size as u32,
            marker: Default::default(),
        }
    }

    /// return the first and last handles included in `self`.
    ///
    /// If `self` is an empty range, there are no handles included, so
    /// return `None`.
    pub const fn first_and_last(&self) -> Option<(Handle<T>, Handle<T>)> {
        if self.inner.start < self.inner.end {
            Some((
                // `Range::new_from_bounds` expects a start- and end-inclusive
                // range, but `self.inner` is an end-exclusive range.
                Handle::new(Index::new(self.inner.start).unwrap()),
                Handle::new(Index::new(self.inner.end - 1).unwrap()),
            ))
        } else {
            None
        }
    }

    /// Return the index range covered by `self`.
    pub fn index_range(&self) -> ops::Range<u32> {
        self.inner.clone()
    }

    /// Construct a `Range` that covers the indices in `inner`.
    pub fn from_index_range(inner: ops::Range<u32>, arena: &Arena<T>) -> Self {
        // Since `inner` is a `Range<u32>`, we only need to check that
        // the start and end are well-ordered, and that the end fits
        // within `arena`.
        assert!(inner.start <= inner.end);
        assert!(inner.end as usize <= arena.len());
        Self {
            inner,
            marker: Default::default(),
        }
    }
}
