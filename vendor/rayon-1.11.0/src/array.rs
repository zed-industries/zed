//! Parallel iterator types for [arrays] (`[T; N]`)
//!
//! You will rarely need to interact with this module directly unless you need
//! to name one of the iterator types.
//!
//! [arrays]: primitive@array

use crate::iter::plumbing::*;
use crate::iter::*;
use crate::slice::{Iter, IterMut};
use crate::vec::DrainProducer;
use std::mem::ManuallyDrop;

impl<'data, T: Sync + 'data, const N: usize> IntoParallelIterator for &'data [T; N] {
    type Item = &'data T;
    type Iter = Iter<'data, T>;

    fn into_par_iter(self) -> Self::Iter {
        <&[T]>::into_par_iter(self)
    }
}

impl<'data, T: Send + 'data, const N: usize> IntoParallelIterator for &'data mut [T; N] {
    type Item = &'data mut T;
    type Iter = IterMut<'data, T>;

    fn into_par_iter(self) -> Self::Iter {
        <&mut [T]>::into_par_iter(self)
    }
}

impl<T: Send, const N: usize> IntoParallelIterator for [T; N] {
    type Item = T;
    type Iter = IntoIter<T, N>;

    fn into_par_iter(self) -> Self::Iter {
        IntoIter { array: self }
    }
}

/// Parallel iterator that moves out of an array.
#[derive(Debug, Clone)]
pub struct IntoIter<T, const N: usize> {
    array: [T; N],
}

impl<T: Send, const N: usize> ParallelIterator for IntoIter<T, N> {
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(N)
    }
}

impl<T: Send, const N: usize> IndexedParallelIterator for IntoIter<T, N> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        N
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        unsafe {
            // Drain every item, and then the local array can just fall out of scope.
            let mut array = ManuallyDrop::new(self.array);
            let producer = DrainProducer::new(array.as_mut_slice());
            callback.callback(producer)
        }
    }
}
