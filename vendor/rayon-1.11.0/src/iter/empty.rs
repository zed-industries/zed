use crate::iter::plumbing::*;
use crate::iter::*;

use std::fmt;
use std::marker::PhantomData;

/// Creates a parallel iterator that produces nothing.
///
/// This admits no parallelism on its own, but it could be used for code that
/// deals with generic parallel iterators.
///
/// # Examples
///
/// ```
/// use rayon::prelude::*;
/// use rayon::iter::empty;
///
/// let pi = (0..1234).into_par_iter()
///     .chain(empty())
///     .chain(1234..10_000);
///
/// assert_eq!(pi.count(), 10_000);
/// ```
pub fn empty<T: Send>() -> Empty<T> {
    Empty {
        marker: PhantomData,
    }
}

/// Iterator adaptor for [the `empty()` function].
///
/// [the `empty()` function]: empty()
pub struct Empty<T> {
    marker: PhantomData<T>,
}

impl<T> Clone for Empty<T> {
    fn clone(&self) -> Self {
        Empty {
            marker: PhantomData,
        }
    }
}

impl<T: Send> fmt::Debug for Empty<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Empty")
    }
}

impl<T: Send> ParallelIterator for Empty<T> {
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.drive(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(0)
    }
}

impl<T: Send> IndexedParallelIterator for Empty<T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        consumer.into_folder().complete()
    }

    fn len(&self) -> usize {
        0
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(EmptyProducer(PhantomData))
    }
}

/// Private empty producer
struct EmptyProducer<T: Send>(PhantomData<T>);

impl<T: Send> Producer for EmptyProducer<T> {
    type Item = T;
    type IntoIter = std::iter::Empty<T>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        debug_assert_eq!(index, 0);
        (self, EmptyProducer(PhantomData))
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        folder
    }
}
