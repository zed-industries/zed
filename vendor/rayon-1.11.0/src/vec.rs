//! Parallel iterator types for [vectors] (`Vec<T>`)
//!
//! You will rarely need to interact with this module directly unless you need
//! to name one of the iterator types.
//!
//! [vectors]: mod@std::vec

use crate::iter::plumbing::*;
use crate::iter::*;
use crate::math::simplify_range;
use crate::slice::{Iter, IterMut};
use std::iter;
use std::mem;
use std::ops::{Range, RangeBounds};
use std::ptr;
use std::slice;

impl<'data, T: Sync + 'data> IntoParallelIterator for &'data Vec<T> {
    type Item = &'data T;
    type Iter = Iter<'data, T>;

    fn into_par_iter(self) -> Self::Iter {
        <&[T]>::into_par_iter(self)
    }
}

impl<'data, T: Send + 'data> IntoParallelIterator for &'data mut Vec<T> {
    type Item = &'data mut T;
    type Iter = IterMut<'data, T>;

    fn into_par_iter(self) -> Self::Iter {
        <&mut [T]>::into_par_iter(self)
    }
}

/// Parallel iterator that moves out of a vector.
#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    vec: Vec<T>,
}

impl<T: Send> IntoParallelIterator for Vec<T> {
    type Item = T;
    type Iter = IntoIter<T>;

    fn into_par_iter(self) -> Self::Iter {
        IntoIter { vec: self }
    }
}

impl<T: Send> IntoParallelIterator for Box<[T]> {
    type Item = T;
    type Iter = IntoIter<T>;

    fn into_par_iter(self) -> Self::Iter {
        IntoIter { vec: self.into() }
    }
}

impl<T: Send> ParallelIterator for IntoIter<T> {
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<T: Send> IndexedParallelIterator for IntoIter<T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn with_producer<CB>(mut self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        // Drain every item, and then the vector only needs to free its buffer.
        self.vec.par_drain(..).with_producer(callback)
    }
}

impl<'data, T: Send> ParallelDrainRange<usize> for &'data mut Vec<T> {
    type Iter = Drain<'data, T>;
    type Item = T;

    fn par_drain<R: RangeBounds<usize>>(self, range: R) -> Self::Iter {
        Drain {
            orig_len: self.len(),
            range: simplify_range(range, self.len()),
            vec: self,
        }
    }
}

/// Draining parallel iterator that moves a range out of a vector, but keeps the total capacity.
#[derive(Debug)]
pub struct Drain<'data, T: Send> {
    vec: &'data mut Vec<T>,
    range: Range<usize>,
    orig_len: usize,
}

impl<'data, T: Send> ParallelIterator for Drain<'data, T> {
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'data, T: Send> IndexedParallelIterator for Drain<'data, T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.range.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        unsafe {
            // Make the vector forget about the drained items, and temporarily the tail too.
            self.vec.set_len(self.range.start);

            // Create the producer as the exclusive "owner" of the slice.
            let producer = DrainProducer::from_vec(self.vec, self.range.len());

            // The producer will move or drop each item from the drained range.
            callback.callback(producer)
        }
    }
}

impl<'data, T: Send> Drop for Drain<'data, T> {
    fn drop(&mut self) {
        let Range { start, end } = self.range;
        if self.vec.len() == self.orig_len {
            // We must not have produced, so just call a normal drain to remove the items.
            self.vec.drain(start..end);
        } else if start == end {
            // Empty range, so just restore the length to its original state
            unsafe {
                self.vec.set_len(self.orig_len);
            }
        } else if end < self.orig_len {
            // The producer was responsible for consuming the drained items.
            // Move the tail items to their new place, then set the length to include them.
            unsafe {
                let ptr = self.vec.as_mut_ptr().add(start);
                let tail_ptr = self.vec.as_ptr().add(end);
                let tail_len = self.orig_len - end;
                ptr::copy(tail_ptr, ptr, tail_len);
                self.vec.set_len(start + tail_len);
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////

pub(crate) struct DrainProducer<'data, T: Send> {
    slice: &'data mut [T],
}

impl<T: Send> DrainProducer<'_, T> {
    /// Creates a draining producer, which *moves* items from the slice.
    ///
    /// Unsafe because `!Copy` data must not be read after the borrow is released.
    pub(crate) unsafe fn new(slice: &mut [T]) -> DrainProducer<'_, T> {
        DrainProducer { slice }
    }

    /// Creates a draining producer, which *moves* items from the tail of the vector.
    ///
    /// Unsafe because we're moving from beyond `vec.len()`, so the caller must ensure
    /// that data is initialized and not read after the borrow is released.
    unsafe fn from_vec(vec: &mut Vec<T>, len: usize) -> DrainProducer<'_, T> {
        let start = vec.len();
        assert!(vec.capacity() - start >= len);

        // The pointer is derived from `Vec` directly, not through a `Deref`,
        // so it has provenance over the whole allocation.
        let ptr = vec.as_mut_ptr().add(start);
        DrainProducer::new(slice::from_raw_parts_mut(ptr, len))
    }
}

impl<'data, T: 'data + Send> Producer for DrainProducer<'data, T> {
    type Item = T;
    type IntoIter = SliceDrain<'data, T>;

    fn into_iter(mut self) -> Self::IntoIter {
        // replace the slice so we don't drop it twice
        let slice = mem::take(&mut self.slice);
        SliceDrain {
            iter: slice.iter_mut(),
        }
    }

    fn split_at(mut self, index: usize) -> (Self, Self) {
        // replace the slice so we don't drop it twice
        let slice = mem::take(&mut self.slice);
        let (left, right) = slice.split_at_mut(index);
        unsafe { (DrainProducer::new(left), DrainProducer::new(right)) }
    }
}

impl<'data, T: 'data + Send> Drop for DrainProducer<'data, T> {
    fn drop(&mut self) {
        // extract the slice so we can use `Drop for [T]`
        let slice_ptr: *mut [T] = mem::take::<&'data mut [T]>(&mut self.slice);
        unsafe { ptr::drop_in_place::<[T]>(slice_ptr) };
    }
}

// ////////////////////////////////////////////////////////////////////////

// like std::vec::Drain, without updating a source Vec
pub(crate) struct SliceDrain<'data, T> {
    iter: slice::IterMut<'data, T>,
}

impl<'data, T: 'data> Iterator for SliceDrain<'data, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // Coerce the pointer early, so we don't keep the
        // reference that's about to be invalidated.
        let ptr: *const T = self.iter.next()?;
        Some(unsafe { ptr::read(ptr) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.len()
    }
}

impl<'data, T: 'data> DoubleEndedIterator for SliceDrain<'data, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Coerce the pointer early, so we don't keep the
        // reference that's about to be invalidated.
        let ptr: *const T = self.iter.next_back()?;
        Some(unsafe { ptr::read(ptr) })
    }
}

impl<'data, T: 'data> ExactSizeIterator for SliceDrain<'data, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'data, T: 'data> iter::FusedIterator for SliceDrain<'data, T> {}

impl<'data, T: 'data> Drop for SliceDrain<'data, T> {
    fn drop(&mut self) {
        // extract the iterator so we can use `Drop for [T]`
        let slice_ptr: *mut [T] = mem::replace(&mut self.iter, [].iter_mut()).into_slice();
        unsafe { ptr::drop_in_place::<[T]>(slice_ptr) };
    }
}
