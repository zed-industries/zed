//! Archived versions of `ops` types.

use core::{
    cmp, fmt,
    ops::{Bound, RangeBounds},
};

/// An archived [`Range`](::core::ops::Range).
#[derive(Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedRange<T> {
    /// The lower bound of the range (inclusive).
    pub start: T,
    /// The upper bound of the range (inclusive).
    pub end: T,
}

impl<T: fmt::Debug> fmt::Debug for ArchivedRange<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}

impl<T: PartialOrd<T>> ArchivedRange<T> {
    /// Returns `true` if `item` is contained in the range.
    #[inline]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: PartialOrd<T> + ?Sized,
    {
        <Self as RangeBounds<T>>::contains(self, item)
    }

    /// Returns `true` if the range contains no items.
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self.start.partial_cmp(&self.end) {
            None | Some(cmp::Ordering::Greater) | Some(cmp::Ordering::Equal) => true,
            Some(cmp::Ordering::Less) => false,
        }
    }
}

impl<T> RangeBounds<T> for ArchivedRange<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.start)
    }

    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Bound::Excluded(&self.end)
    }
}

// RangeInclusive

/// An archived [`RangeInclusive`](::core::ops::RangeInclusive).
#[derive(Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedRangeInclusive<T> {
    /// The lower bound of the range (inclusive).
    pub start: T,
    /// The upper bound of the range (inclusive).
    pub end: T,
}

impl<T: fmt::Debug> fmt::Debug for ArchivedRangeInclusive<T> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..=")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}

impl<T: PartialOrd<T>> ArchivedRangeInclusive<T> {
    /// Returns `true` if `item` is contained in the range.
    #[inline]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: PartialOrd<T> + ?Sized,
    {
        <Self as RangeBounds<T>>::contains(self, item)
    }

    /// Returns `true` if the range contains no items.
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self.start.partial_cmp(&self.end) {
            None | Some(cmp::Ordering::Greater) => true,
            Some(cmp::Ordering::Less) | Some(cmp::Ordering::Equal) => false,
        }
    }
}

impl<T> RangeBounds<T> for ArchivedRangeInclusive<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.start)
    }

    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(&self.end)
    }
}

/// An archived [`RangeFrom`](::core::ops::RangeFrom).
#[derive(Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedRangeFrom<T> {
    /// The lower bound of the range (inclusive).
    pub start: T,
}

impl<T: fmt::Debug> fmt::Debug for ArchivedRangeFrom<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        Ok(())
    }
}

impl<T: PartialOrd<T>> ArchivedRangeFrom<T> {
    /// Returns `true` if `item` is contained in the range.
    #[inline]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        <Self as RangeBounds<T>>::contains(self, item)
    }
}

impl<T> RangeBounds<T> for ArchivedRangeFrom<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.start)
    }

    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }
}

/// An archived [`RangeTo`](::core::ops::RangeTo).
#[derive(Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedRangeTo<T> {
    /// The upper bound of the range (exclusive).
    pub end: T,
}

impl<T: fmt::Debug> fmt::Debug for ArchivedRangeTo<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}

impl<T: PartialOrd<T>> ArchivedRangeTo<T> {
    /// Returns `true` if `item` is contained in the range.
    #[inline]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        <Self as RangeBounds<T>>::contains(self, item)
    }
}

impl<T> RangeBounds<T> for ArchivedRangeTo<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }

    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Bound::Excluded(&self.end)
    }
}

/// An archived [`RangeToInclusive`](::core::ops::RangeToInclusive).
#[derive(Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedRangeToInclusive<T> {
    /// The upper bound of the range (inclusive).
    pub end: T,
}

impl<T: fmt::Debug> fmt::Debug for ArchivedRangeToInclusive<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "..=")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}

impl<T: PartialOrd<T>> ArchivedRangeToInclusive<T> {
    /// Returns `true` if `item` is contained in the range.
    #[inline]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        <Self as RangeBounds<T>>::contains(self, item)
    }
}

impl<T> RangeBounds<T> for ArchivedRangeToInclusive<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }

    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(&self.end)
    }
}
