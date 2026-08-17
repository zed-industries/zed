use crate::iter::plumbing::*;
use crate::iter::*;

/// Parallel iterator over immutable non-overlapping chunks of a slice
#[derive(Debug)]
pub struct Chunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}

impl<'data, T> Chunks<'data, T> {
    pub(super) fn new(chunk_size: usize, slice: &'data [T]) -> Self {
        Self { chunk_size, slice }
    }
}

impl<T> Clone for Chunks<'_, T> {
    fn clone(&self) -> Self {
        Chunks { ..*self }
    }
}

impl<'data, T: Sync> ParallelIterator for Chunks<'data, T> {
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

impl<T: Sync> IndexedParallelIterator for Chunks<'_, T> {
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
        callback.callback(ChunksProducer {
            chunk_size: self.chunk_size,
            slice: self.slice,
        })
    }
}

struct ChunksProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}

impl<'data, T: 'data + Sync> Producer for ChunksProducer<'data, T> {
    type Item = &'data [T];
    type IntoIter = ::std::slice::Chunks<'data, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.chunks(self.chunk_size)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = Ord::min(index * self.chunk_size, self.slice.len());
        let (left, right) = self.slice.split_at(elem_index);
        (
            ChunksProducer {
                chunk_size: self.chunk_size,
                slice: left,
            },
            ChunksProducer {
                chunk_size: self.chunk_size,
                slice: right,
            },
        )
    }
}

/// Parallel iterator over immutable non-overlapping chunks of a slice
#[derive(Debug)]
pub struct ChunksExact<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
    rem: &'data [T],
}

impl<'data, T> ChunksExact<'data, T> {
    pub(super) fn new(chunk_size: usize, slice: &'data [T]) -> Self {
        let rem_len = slice.len() % chunk_size;
        let len = slice.len() - rem_len;
        let (slice, rem) = slice.split_at(len);
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

impl<T> Clone for ChunksExact<'_, T> {
    fn clone(&self) -> Self {
        ChunksExact { ..*self }
    }
}

impl<'data, T: Sync> ParallelIterator for ChunksExact<'data, T> {
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

impl<T: Sync> IndexedParallelIterator for ChunksExact<'_, T> {
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
        callback.callback(ChunksExactProducer {
            chunk_size: self.chunk_size,
            slice: self.slice,
        })
    }
}

struct ChunksExactProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}

impl<'data, T: 'data + Sync> Producer for ChunksExactProducer<'data, T> {
    type Item = &'data [T];
    type IntoIter = ::std::slice::ChunksExact<'data, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.chunks_exact(self.chunk_size)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = index * self.chunk_size;
        let (left, right) = self.slice.split_at(elem_index);
        (
            ChunksExactProducer {
                chunk_size: self.chunk_size,
                slice: left,
            },
            ChunksExactProducer {
                chunk_size: self.chunk_size,
                slice: right,
            },
        )
    }
}

/// Parallel iterator over mutable non-overlapping chunks of a slice
#[derive(Debug)]
pub struct ChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}

impl<'data, T> ChunksMut<'data, T> {
    pub(super) fn new(chunk_size: usize, slice: &'data mut [T]) -> Self {
        Self { chunk_size, slice }
    }
}

impl<'data, T: Send> ParallelIterator for ChunksMut<'data, T> {
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

impl<T: Send> IndexedParallelIterator for ChunksMut<'_, T> {
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
        callback.callback(ChunksMutProducer {
            chunk_size: self.chunk_size,
            slice: self.slice,
        })
    }
}

struct ChunksMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}

impl<'data, T: 'data + Send> Producer for ChunksMutProducer<'data, T> {
    type Item = &'data mut [T];
    type IntoIter = ::std::slice::ChunksMut<'data, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.chunks_mut(self.chunk_size)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = Ord::min(index * self.chunk_size, self.slice.len());
        let (left, right) = self.slice.split_at_mut(elem_index);
        (
            ChunksMutProducer {
                chunk_size: self.chunk_size,
                slice: left,
            },
            ChunksMutProducer {
                chunk_size: self.chunk_size,
                slice: right,
            },
        )
    }
}

/// Parallel iterator over mutable non-overlapping chunks of a slice
#[derive(Debug)]
pub struct ChunksExactMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
    rem: &'data mut [T],
}

impl<'data, T> ChunksExactMut<'data, T> {
    pub(super) fn new(chunk_size: usize, slice: &'data mut [T]) -> Self {
        let rem_len = slice.len() % chunk_size;
        let len = slice.len() - rem_len;
        let (slice, rem) = slice.split_at_mut(len);
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
    /// with `std::iter::ChunksExactMut`, but consider calling `remainder()` or
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

impl<'data, T: Send> ParallelIterator for ChunksExactMut<'data, T> {
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

impl<T: Send> IndexedParallelIterator for ChunksExactMut<'_, T> {
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
        callback.callback(ChunksExactMutProducer {
            chunk_size: self.chunk_size,
            slice: self.slice,
        })
    }
}

struct ChunksExactMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}

impl<'data, T: 'data + Send> Producer for ChunksExactMutProducer<'data, T> {
    type Item = &'data mut [T];
    type IntoIter = ::std::slice::ChunksExactMut<'data, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.chunks_exact_mut(self.chunk_size)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = index * self.chunk_size;
        let (left, right) = self.slice.split_at_mut(elem_index);
        (
            ChunksExactMutProducer {
                chunk_size: self.chunk_size,
                slice: left,
            },
            ChunksExactMutProducer {
                chunk_size: self.chunk_size,
                slice: right,
            },
        )
    }
}
