// Copyright © 2023-2025 Andrea Corbellini and contributors
// SPDX-License-Identifier: BSD-3-Clause

use crate::CircularBuffer;
use core::fmt;
use core::iter::FusedIterator;
use core::ops::Bound;
use core::ops::RangeBounds;

/// An owning [iterator](core::iter::Iterator) over the elements of a [`CircularBuffer`].
///
/// This yields the elements of a `CircularBuffer` from front to back.
///
/// This struct is created when iterating over a `CircularBuffer`. See the documentation for
/// [`IntoIterator`] for more details.
#[derive(Clone)]
pub struct IntoIter<const N: usize, T> {
    inner: CircularBuffer<N, T>,
}

impl<const N: usize, T> IntoIter<N, T> {
    pub(crate) const fn new(inner: CircularBuffer<N, T>) -> Self {
        Self { inner }
    }
}

impl<const N: usize, T> Iterator for IntoIter<N, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.len();
        (len, Some(len))
    }
}

impl<const N: usize, T> ExactSizeIterator for IntoIter<N, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<const N: usize, T> FusedIterator for IntoIter<N, T> {}

impl<const N: usize, T> DoubleEndedIterator for IntoIter<N, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.pop_back()
    }
}

impl<const N: usize, T> fmt::Debug for IntoIter<N, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

pub(crate) fn translate_range_bounds<const N: usize, T, R>(
    buf: &CircularBuffer<N, T>,
    range: R,
) -> (usize, usize)
where
    R: RangeBounds<usize>,
{
    let start = match range.start_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => x
            .checked_add(1)
            .expect("range start index exceeds maximum usize"),
        Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        Bound::Included(x) => x
            .checked_add(1)
            .expect("range end index exceeds maximum usize"),
        Bound::Excluded(x) => *x,
        Bound::Unbounded => buf.len(),
    };

    assert!(
        end <= buf.len(),
        "range end index {} out of range for buffer of length {}",
        end,
        buf.len()
    );
    assert!(
        start <= end,
        "range starts at index {start} but ends at index {end}"
    );

    (start, end)
}

/// An [iterator](core::iter::Iterator) over the elements of a `CircularBuffer`.
///
/// This struct is created by [`CircularBuffer::iter()`] and [`CircularBuffer::range()`]. See
/// their documentation for more details.
pub struct Iter<'a, T> {
    pub(crate) right: &'a [T],
    pub(crate) left: &'a [T],
}

impl<'a, T> Iter<'a, T> {
    pub(crate) const fn empty() -> Self {
        Self {
            right: &[],
            left: &[],
        }
    }

    pub(crate) fn new<const N: usize>(buf: &'a CircularBuffer<N, T>) -> Self {
        let (right, left) = buf.as_slices();
        Self { right, left }
    }

    pub(crate) fn over_range<const N: usize, R>(buf: &'a CircularBuffer<N, T>, range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let (start, end) = translate_range_bounds(buf, range);
        if start >= end {
            Self::empty()
        } else {
            let len = buf.len();
            let mut it = Self::new(buf);
            it.advance_front_by(start);
            it.advance_back_by(len - end);
            it
        }
    }

    fn advance_front_by(&mut self, count: usize) {
        if self.right.len() > count {
            let _ = self.right.split_off(..count);
        } else {
            let take_left = count - self.right.len();
            debug_assert!(
                take_left <= self.left.len(),
                "attempted to advance past the back of the buffer"
            );
            let _ = self.left.split_off(..take_left);
            self.right = &[];
        }
    }

    fn advance_back_by(&mut self, count: usize) {
        if self.left.len() > count {
            let take_left = self.left.len() - count;
            let _ = self.left.split_off(take_left..);
        } else {
            let take_right = self.right.len() - (count - self.left.len());
            debug_assert!(
                take_right <= self.right.len(),
                "attempted to advance past the front of the buffer"
            );
            let _ = self.right.split_off(take_right..);
            self.left = &[];
        }
    }
}

impl<T> Default for Iter<'_, T> {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.right.split_off_first() {
            Some(item)
        } else if let Some(item) = self.left.split_off_first() {
            Some(item)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.right.len() + self.left.len()
    }
}

impl<T> FusedIterator for Iter<'_, T> {}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.left.split_off_last() {
            Some(item)
        } else if let Some(item) = self.right.split_off_last() {
            Some(item)
        } else {
            None
        }
    }
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Self {
            right: self.right,
            left: self.left,
        }
    }
}

impl<T> fmt::Debug for Iter<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A mutable [iterator](core::iter::Iterator) over the elements of a `CircularBuffer`.
///
/// This struct is created by [`CircularBuffer::iter_mut()`] and [`CircularBuffer::range_mut()`].
/// See their documentation for more details.
pub struct IterMut<'a, T> {
    right: &'a mut [T],
    left: &'a mut [T],
}

impl<'a, T> IterMut<'a, T> {
    pub(crate) fn empty() -> Self {
        Self {
            right: &mut [],
            left: &mut [],
        }
    }

    pub(crate) fn new<const N: usize>(buf: &'a mut CircularBuffer<N, T>) -> Self {
        let (right, left) = buf.as_mut_slices();
        Self { right, left }
    }

    pub(crate) fn over_range<const N: usize, R>(buf: &'a mut CircularBuffer<N, T>, range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let (start, end) = translate_range_bounds(buf, range);
        if start >= end {
            Self::empty()
        } else {
            let len = buf.len();
            let mut it = Self::new(buf);
            it.advance_front_by(start);
            it.advance_back_by(len - end);
            it
        }
    }

    fn advance_front_by(&mut self, count: usize) {
        if self.right.len() > count {
            let _ = self.right.split_off_mut(..count);
        } else {
            let take_left = count - self.right.len();
            debug_assert!(
                take_left <= self.left.len(),
                "attempted to advance past the back of the buffer"
            );
            let _ = self.left.split_off_mut(..take_left);
            self.right = &mut [];
        }
    }

    fn advance_back_by(&mut self, count: usize) {
        if self.left.len() > count {
            let take_left = self.left.len() - count;
            let _ = self.left.split_off_mut(take_left..);
        } else {
            let take_right = self.right.len() - (count - self.left.len());
            debug_assert!(
                take_right <= self.right.len(),
                "attempted to advance past the front of the buffer"
            );
            let _ = self.right.split_off_mut(take_right..);
            self.left = &mut [];
        }
    }
}

impl<T> Default for IterMut<'_, T> {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.right.split_off_first_mut() {
            Some(item)
        } else if let Some(item) = self.left.split_off_first_mut() {
            Some(item)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.right.len() + self.left.len()
    }
}

impl<T> FusedIterator for IterMut<'_, T> {}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.left.split_off_last_mut() {
            Some(item)
        } else if let Some(item) = self.right.split_off_last_mut() {
            Some(item)
        } else {
            None
        }
    }
}

impl<T> fmt::Debug for IterMut<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let it = Iter {
            right: self.right,
            left: self.left,
        };
        it.fmt(f)
    }
}
