use crate::iter::plumbing::*;
use crate::iter::*;

/// Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.
#[derive(Debug)]
pub struct RChunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}

impl<'data, T> RChunks<'data, T> {
    pub(super) fn new(chunk_size: usize, slice: &'data [T]) -> Self {
        Self { chunk_size, slice }
    }
}

impl<T> Clone for RChunks<'_, T> {
    fn clone(&self) -> Self {
        RChunks { ..*self }
    }
}

impl<'data, T: Sync> ParallelIterator for RChunks<'data, T> {
    type Item = &'data [T];

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

impl<T: Sync> IndexedParallelIterator for RChunks<'_, T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.slice.len().div_ceil(self.chunk_size)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(RChunksProducer {
            chunk_size: self.chunk_size,
            slice: self.slice,
        })
    }
}

struct RChunksProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}

impl<'data, T: 'data + Sync> Producer for RChunksProducer<'data, T> {
    type Item = &'data [T];
    type IntoIter = ::std::slice::RChunks<'data, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.rchunks(self.chunk_size)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = self.slice.len().saturating_sub(index * self.chunk_size);
        let (left, right) = self.slice.split_at(elem_index);
        (
            RChunksProducer {
                chunk_size: self.chunk_size,
                slice: right,
            },
            RChunksProducer {
                chunk_size: self.chunk_size,
                slice: left,
            },
        )
    }
}

/// Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.
#[derive(Debug)]
pub struct RChunksExact<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
    rem: &'data [T],
}

impl<'data, T> RChunksExact<'data, T> {
    pub(super) fn new(chunk_size: usize, slice: &'data [T]) -> Self {
        let rem_len = slice.len() % chunk_size;
        let (rem, slice) = slice.split_at(rem_len);
        Self {
            chunk_size,
            slice,
            rem,
        }
    }

    /// Return the remainder of the original slice that is not going to be
    /// returned by the iterator. The returned slice has at most `chunk_size-1`
    /// elements.
    pub fn remainder(&self) -> &'data [T] {
        self.rem
    }
}

impl<T> Clone for RChunksExact<'_, T> {
    fn clone(&self) -> Self {
        RChunksExact { ..*self }
    }
}

impl<'data, T: Sync> ParallelIterator for RChunksExact<'data, T> {
    type Item = &'data [T];

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

impl<T: Sync> IndexedParallelIterator for RChunksExact<'_, T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.slice.len() / self.chunk_size
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(RChunksExactProducer {
            chunk_size: self.chunk_size,
            slice: self.slice,
        })
    }
}

struct RChunksExactProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}

impl<'data, T: 'data + Sync> Producer for RChunksExactProducer<'data, T> {
    type Item = &'data [T];
    type IntoIter = ::std::slice::RChunksExact<'data, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.rchunks_exact(self.chunk_size)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = self.slice.len() - index * self.chunk_size;
        let (left, right) = self.slice.split_at(elem_index);
        (
            RChunksExactProducer {
                chunk_size: self.chunk_size,
                slice: right,
            },
            RChunksExactProducer {
                chunk_size: self.chunk_size,
                slice: left,
            },
        )
    }
}

/// Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.
#[derive(Debug)]
pub struct RChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}

impl<'data, T> RChunksMut<'data, T> {
    pub(super) fn new(chunk_size: usize, slice: &'data mut [T]) -> Self {
        Self { chunk_size, slice }
    }
}

impl<'data, T: Send> ParallelIterator for RChunksMut<'data, T> {
    type Item = &'data mut [T];

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

impl<T: Send> IndexedParallelIterator for RChunksMut<'_, T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.slice.len().div_ceil(self.chunk_size)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(RChunksMutProducer {
            chunk_size: self.chunk_size,
            slice: self.slice,
        })
    }
}

struct RChunksMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}

impl<'data, T: 'data + Send> Producer for RChunksMutProducer<'data, T> {
    type Item = &'data mut [T];
    type IntoIter = ::std::slice::RChunksMut<'data, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.rchunks_mut(self.chunk_size)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = self.slice.len().saturating_sub(index * self.chunk_size);
        let (left, right) = self.slice.split_at_mut(elem_index);
        (
            RChunksMutProducer {
                chunk_size: self.chunk_size,
                slice: right,
            },
            RChunksMutProducer {
                chunk_size: self.chunk_size,
                slice: left,
            },
        )
    }
}

/// Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.
#[derive(Debug)]
pub struct RChunksExactMut<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
    rem: &'data mut [T],
}

impl<'data, T: Send> RChunksExactMut<'data, T> {
    pub(super) fn new(chunk_size: usize, slice: &'data mut [T]) -> Self {
        let rem_len = slice.len() % chunk_size;
        let (rem, slice) = slice.split_at_mut(rem_len);
        Self {
            chunk_size,
            slice,
            rem,
        }
    }

    /// Return the remainder of the original slice that is not going to be
    /// returned by the iterator. The returned slice has at most `chunk_size-1`
    /// elements.
    ///
    /// Note that this has to consume `self` to return the original lifetime of
    /// the data, which prevents this from actually being used as a parallel
    /// iterator since that also consumes. This method is provided for parity
    /// with `std::iter::RChunksExactMut`, but consider calling `remainder()` or
    /// `take_remainder()` as alternatives.
    pub fn into_remainder(self) -> &'data mut [T] {
        self.rem
    }

    /// Return the remainder of the original slice that is not going to be
    /// returned by the iterator. The returned slice has at most `chunk_size-1`
    /// elements.
    ///
    /// Consider `take_remainder()` if you need access to the data with its
    /// original lifetime, rather than borrowing through `&mut self` here.
    pub fn remainder(&mut self) -> &mut [T] {
        self.rem
    }

    /// Return the remainder of the original slice that is not going to be
    /// returned by the iterator. The returned slice has at most `chunk_size-1`
    /// elements. Subsequent calls will return an empty slice.
    pub fn take_remainder(&mut self) -> &'data mut [T] {
        std::mem::take(&mut self.rem)
    }
}

impl<'data, T: Send + 'data> ParallelIterator for RChunksExactMut<'data, T> {
    type Item = &'data mut [T];

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

impl<'data, T: Send + 'data> IndexedParallelIterator for RChunksExactMut<'data, T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.slice.len() / self.chunk_size
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(RChunksExactMutProducer {
            chunk_size: self.chunk_size,
            slice: self.slice,
        })
    }
}

struct RChunksExactMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}

impl<'data, T: 'data + Send> Producer for RChunksExactMutProducer<'data, T> {
    type Item = &'data mut [T];
    type IntoIter = ::std::slice::RChunksExactMut<'data, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.rchunks_exact_mut(self.chunk_size)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = self.slice.len() - index * self.chunk_size;
        let (left, right) = self.slice.split_at_mut(elem_index);
        (
            RChunksExactMutProducer {
                chunk_size: self.chunk_size,
                slice: right,
            },
            RChunksExactMutProducer {
                chunk_size: self.chunk_size,
                slice: left,
            },
        )
    }
}
