//! This module contains the parallel iterator types for heaps
//! (`BinaryHeap<T>`). You will rarely need to interact with it directly
//! unless you have need to name one of the iterator types.

use std::collections::BinaryHeap;

use crate::iter::plumbing::*;
use crate::iter::*;

use crate::slice;
use crate::vec;

/// Parallel iterator over a binary heap
#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}

impl<T: Send> IntoParallelIterator for BinaryHeap<T> {
    type Item = T;
    type Iter = IntoIter<T>;

    fn into_par_iter(self) -> Self::Iter {
        IntoIter {
            inner: Vec::from(self).into_par_iter(),
        }
    }
}

delegate_indexed_iterator! {
    IntoIter<T> => T,
    impl<T: Send>
}

/// Parallel iterator over an immutable reference to a binary heap
#[derive(Debug)]
pub struct Iter<'a, T> {
    inner: slice::Iter<'a, T>,
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T: Sync> IntoParallelIterator for &'a BinaryHeap<T> {
    type Item = &'a T;
    type Iter = Iter<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        Iter {
            inner: self.as_slice().into_par_iter(),
        }
    }
}

delegate_indexed_iterator! {
    Iter<'a, T> => &'a T,
    impl<'a, T: Sync + 'a>
}

// `BinaryHeap` doesn't have a mutable `Iterator`

/// Draining parallel iterator that moves out of a binary heap,
/// but keeps the total capacity.
#[derive(Debug)]
pub struct Drain<'a, T> {
    heap: &'a mut BinaryHeap<T>,
}

// NB: The only reason we require `T: Ord` is for `DrainGuard` to reconstruct
// the heap `From<Vec<T>>` afterward, even though that will actually be empty.
impl<'a, T: Ord + Send> ParallelDrainFull for &'a mut BinaryHeap<T> {
    type Iter = Drain<'a, T>;
    type Item = T;

    fn par_drain(self) -> Self::Iter {
        Drain { heap: self }
    }
}

impl<T: Ord + Send> ParallelIterator for Drain<'_, T> {
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

impl<T: Ord + Send> IndexedParallelIterator for Drain<'_, T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.heap.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        super::DrainGuard::new(self.heap)
            .par_drain(..)
            .with_producer(callback)
    }
}

impl<T> Drop for Drain<'_, T> {
    fn drop(&mut self) {
        if !self.heap.is_empty() {
            // We must not have produced, so just call a normal drain to remove the items.
            self.heap.drain();
        }
    }
}
