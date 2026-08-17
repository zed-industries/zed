use crate::iter::plumbing::*;
use crate::iter::*;
use std::fmt;
use std::marker::PhantomData;

trait ChunkBySlice<T>: AsRef<[T]> + Default + Send {
    fn split(self, index: usize) -> (Self, Self);
    fn chunk_by(self, pred: &impl Fn(&T, &T) -> bool) -> impl Iterator<Item = Self>;

    fn find(&self, pred: &impl Fn(&T, &T) -> bool, start: usize, end: usize) -> Option<usize> {
        self.as_ref()[start..end]
            .windows(2)
            .position(move |w| !pred(&w[0], &w[1]))
            .map(|i| i + 1)
    }

    fn rfind(&self, pred: &impl Fn(&T, &T) -> bool, end: usize) -> Option<usize> {
        self.as_ref()[..end]
            .windows(2)
            .rposition(move |w| !pred(&w[0], &w[1]))
            .map(|i| i + 1)
    }
}

impl<T: Sync> ChunkBySlice<T> for &[T] {
    fn split(self, index: usize) -> (Self, Self) {
        self.split_at(index)
    }

    fn chunk_by(self, pred: &impl Fn(&T, &T) -> bool) -> impl Iterator<Item = Self> {
        self.chunk_by(pred)
    }
}

impl<T: Send> ChunkBySlice<T> for &mut [T] {
    fn split(self, index: usize) -> (Self, Self) {
        self.split_at_mut(index)
    }

    fn chunk_by(self, pred: &impl Fn(&T, &T) -> bool) -> impl Iterator<Item = Self> {
        self.chunk_by_mut(pred)
    }
}

struct ChunkByProducer<'p, T, Slice, Pred> {
    slice: Slice,
    pred: &'p Pred,
    tail: usize,
    marker: PhantomData<fn(&T)>,
}

// Note: this implementation is very similar to `SplitProducer`.
impl<T, Slice, Pred> UnindexedProducer for ChunkByProducer<'_, T, Slice, Pred>
where
    Slice: ChunkBySlice<T>,
    Pred: Fn(&T, &T) -> bool + Send + Sync,
{
    type Item = Slice;

    fn split(self) -> (Self, Option<Self>) {
        if self.tail < 2 {
            return (Self { tail: 0, ..self }, None);
        }

        // Look forward for the separator, and failing that look backward.
        let mid = self.tail / 2;
        let index = match self.slice.find(self.pred, mid, self.tail) {
            Some(i) => Some(mid + i),
            None => self.slice.rfind(self.pred, mid + 1),
        };

        if let Some(index) = index {
            let (left, right) = self.slice.split(index);

            let (left_tail, right_tail) = if index <= mid {
                // If we scanned backwards to find the separator, everything in
                // the right side is exhausted, with no separators left to find.
                (index, 0)
            } else {
                (mid + 1, self.tail - index)
            };

            // Create the left split before the separator.
            let left = Self {
                slice: left,
                tail: left_tail,
                ..self
            };

            // Create the right split following the separator.
            let right = Self {
                slice: right,
                tail: right_tail,
                ..self
            };

            (left, Some(right))
        } else {
            // The search is exhausted, no more separators...
            (Self { tail: 0, ..self }, None)
        }
    }

    fn fold_with<F>(self, mut folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        let Self {
            slice, pred, tail, ..
        } = self;

        let (slice, tail) = if tail == slice.as_ref().len() {
            // No tail section, so just let `consume_iter` do it all.
            (Some(slice), None)
        } else if let Some(index) = slice.rfind(pred, tail) {
            // We found the last separator to complete the tail, so
            // end with that slice after `consume_iter` finds the rest.
            let (left, right) = slice.split(index);
            (Some(left), Some(right))
        } else {
            // We know there are no separators at all, so it's all "tail".
            (None, Some(slice))
        };

        if let Some(slice) = slice {
            folder = folder.consume_iter(slice.chunk_by(pred));
        }

        if let Some(tail) = tail {
            folder = folder.consume(tail);
        }

        folder
    }
}

/// Parallel iterator over slice in (non-overlapping) chunks separated by a predicate.
///
/// This struct is created by the [`par_chunk_by`] method on `&[T]`.
///
/// [`par_chunk_by`]: super::ParallelSlice::par_chunk_by()
pub struct ChunkBy<'data, T, P> {
    pred: P,
    slice: &'data [T],
}

impl<T, P: Clone> Clone for ChunkBy<'_, T, P> {
    fn clone(&self) -> Self {
        ChunkBy {
            pred: self.pred.clone(),
            slice: self.slice,
        }
    }
}

impl<T: fmt::Debug, P> fmt::Debug for ChunkBy<'_, T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkBy")
            .field("slice", &self.slice)
            .finish()
    }
}

impl<'data, T, P> ChunkBy<'data, T, P> {
    pub(super) fn new(slice: &'data [T], pred: P) -> Self {
        Self { pred, slice }
    }
}

impl<'data, T, P> ParallelIterator for ChunkBy<'data, T, P>
where
    T: Sync,
    P: Fn(&T, &T) -> bool + Send + Sync,
{
    type Item = &'data [T];

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(
            ChunkByProducer {
                tail: self.slice.len(),
                slice: self.slice,
                pred: &self.pred,
                marker: PhantomData,
            },
            consumer,
        )
    }
}

/// Parallel iterator over slice in (non-overlapping) mutable chunks
/// separated by a predicate.
///
/// This struct is created by the [`par_chunk_by_mut`] method on `&mut [T]`.
///
/// [`par_chunk_by_mut`]: super::ParallelSliceMut::par_chunk_by_mut()
pub struct ChunkByMut<'data, T, P> {
    pred: P,
    slice: &'data mut [T],
}

impl<T: fmt::Debug, P> fmt::Debug for ChunkByMut<'_, T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkByMut")
            .field("slice", &self.slice)
            .finish()
    }
}

impl<'data, T, P> ChunkByMut<'data, T, P> {
    pub(super) fn new(slice: &'data mut [T], pred: P) -> Self {
        Self { pred, slice }
    }
}

impl<'data, T, P> ParallelIterator for ChunkByMut<'data, T, P>
where
    T: Send,
    P: Fn(&T, &T) -> bool + Send + Sync,
{
    type Item = &'data mut [T];

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(
            ChunkByProducer {
                tail: self.slice.len(),
                slice: self.slice,
                pred: &self.pred,
                marker: PhantomData,
            },
            consumer,
        )
    }
}
