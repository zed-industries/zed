//! This module contains the parallel iterator types for double-ended queues
//! (`VecDeque<T>`). You will rarely need to interact with it directly
//! unless you have need to name one of the iterator types.

use std::collections::VecDeque;
use std::ops::{Range, RangeBounds};

use crate::iter::plumbing::*;
use crate::iter::*;
use crate::math::simplify_range;

use crate::slice;
use crate::vec;

/// Parallel iterator over a double-ended queue
#[derive(Debug, Clone)]
pub struct IntoIter<T: Send> {
    inner: vec::IntoIter<T>,
}

impl<T: Send> IntoParallelIterator for VecDeque<T> {
    type Item = T;
    type Iter = IntoIter<T>;

    fn into_par_iter(self) -> Self::Iter {
        // NOTE: requires data movement if the deque doesn't start at offset 0.
        let inner = Vec::from(self).into_par_iter();
        IntoIter { inner }
    }
}

delegate_indexed_iterator! {
    IntoIter<T> => T,
    impl<T: Send>
}

/// Parallel iterator over an immutable reference to a double-ended queue
#[derive(Debug)]
pub struct Iter<'a, T> {
    inner: Chain<slice::Iter<'a, T>, slice::Iter<'a, T>>,
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T: Sync> IntoParallelIterator for &'a VecDeque<T> {
    type Item = &'a T;
    type Iter = Iter<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        let (a, b) = self.as_slices();
        Iter {
            inner: a.into_par_iter().chain(b),
        }
    }
}

delegate_indexed_iterator! {
    Iter<'a, T> => &'a T,
    impl<'a, T: Sync>
}

/// Parallel iterator over a mutable reference to a double-ended queue
#[derive(Debug)]
pub struct IterMut<'a, T> {
    inner: Chain<slice::IterMut<'a, T>, slice::IterMut<'a, T>>,
}

impl<'a, T: Send> IntoParallelIterator for &'a mut VecDeque<T> {
    type Item = &'a mut T;
    type Iter = IterMut<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        let (a, b) = self.as_mut_slices();
        IterMut {
            inner: a.into_par_iter().chain(b),
        }
    }
}

delegate_indexed_iterator! {
    IterMut<'a, T> => &'a mut T,
    impl<'a, T: Send>
}

/// Draining parallel iterator that moves a range out of a double-ended queue,
/// but keeps the total capacity.
#[derive(Debug)]
pub struct Drain<'a, T> {
    deque: &'a mut VecDeque<T>,
    range: Range<usize>,
    orig_len: usize,
}

impl<'a, T: Send> ParallelDrainRange<usize> for &'a mut VecDeque<T> {
    type Iter = Drain<'a, T>;
    type Item = T;

    fn par_drain<R: RangeBounds<usize>>(self, range: R) -> Self::Iter {
        Drain {
            orig_len: self.len(),
            range: simplify_range(range, self.len()),
            deque: self,
        }
    }
}

impl<T: Send> ParallelIterator for Drain<'_, T> {
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

impl<T: Send> IndexedParallelIterator for Drain<'_, T> {
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
        // NOTE: requires data movement if the deque doesn't start at offset 0.
        super::DrainGuard::new(self.deque)
            .par_drain(self.range.clone())
            .with_producer(callback)
    }
}

impl<T> Drop for Drain<'_, T> {
    fn drop(&mut self) {
        if self.deque.len() != self.orig_len - self.range.len() {
            // We must not have produced, so just call a normal drain to remove the items.
            assert_eq!(self.deque.len(), self.orig_len);
            self.deque.drain(self.range.clone());
        }
    }
}
