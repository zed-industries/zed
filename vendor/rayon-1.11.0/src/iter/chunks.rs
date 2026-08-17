use super::plumbing::*;
use super::*;

/// `Chunks` is an iterator that groups elements of an underlying iterator.
///
/// This struct is created by the [`chunks()`] method on [`IndexedParallelIterator`]
///
/// [`chunks()`]: IndexedParallelIterator::chunks()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Chunks<I> {
    size: usize,
    i: I,
}

impl<I> Chunks<I> {
    /// Creates a new `Chunks` iterator
    pub(super) fn new(i: I, size: usize) -> Self {
        Chunks { i, size }
    }
}

impl<I> ParallelIterator for Chunks<I>
where
    I: IndexedParallelIterator,
{
    type Item = Vec<I::Item>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Vec<I::Item>>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<I> IndexedParallelIterator for Chunks<I>
where
    I: IndexedParallelIterator,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.i.len().div_ceil(self.size)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        let len = self.i.len();
        return self.i.with_producer(Callback {
            size: self.size,
            len,
            callback,
        });

        struct Callback<CB> {
            size: usize,
            len: usize,
            callback: CB,
        }

        impl<T, CB> ProducerCallback<T> for Callback<CB>
        where
            CB: ProducerCallback<Vec<T>>,
        {
            type Output = CB::Output;

            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = T>,
            {
                let producer = ChunkProducer::new(self.size, self.len, base, Vec::from_iter);
                self.callback.callback(producer)
            }
        }
    }
}

pub(super) struct ChunkProducer<P, F> {
    chunk_size: usize,
    len: usize,
    base: P,
    map: F,
}

impl<P, F> ChunkProducer<P, F> {
    pub(super) fn new(chunk_size: usize, len: usize, base: P, map: F) -> Self {
        Self {
            chunk_size,
            len,
            base,
            map,
        }
    }
}

impl<P, F, T> Producer for ChunkProducer<P, F>
where
    P: Producer,
    F: Fn(P::IntoIter) -> T + Send + Clone,
{
    type Item = T;
    type IntoIter = std::iter::Map<ChunkSeq<P>, F>;

    fn into_iter(self) -> Self::IntoIter {
        let chunks = ChunkSeq {
            chunk_size: self.chunk_size,
            len: self.len,
            inner: if self.len > 0 { Some(self.base) } else { None },
        };
        chunks.map(self.map)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = Ord::min(index * self.chunk_size, self.len);
        let (left, right) = self.base.split_at(elem_index);
        (
            ChunkProducer {
                chunk_size: self.chunk_size,
                len: elem_index,
                base: left,
                map: self.map.clone(),
            },
            ChunkProducer {
                chunk_size: self.chunk_size,
                len: self.len - elem_index,
                base: right,
                map: self.map,
            },
        )
    }

    fn min_len(&self) -> usize {
        self.base.min_len().div_ceil(self.chunk_size)
    }

    fn max_len(&self) -> usize {
        self.base.max_len() / self.chunk_size
    }
}

pub(super) struct ChunkSeq<P> {
    chunk_size: usize,
    len: usize,
    inner: Option<P>,
}

impl<P> Iterator for ChunkSeq<P>
where
    P: Producer,
{
    type Item = P::IntoIter;

    fn next(&mut self) -> Option<Self::Item> {
        let producer = self.inner.take()?;
        if self.len > self.chunk_size {
            let (left, right) = producer.split_at(self.chunk_size);
            self.inner = Some(right);
            self.len -= self.chunk_size;
            Some(left.into_iter())
        } else {
            debug_assert!(self.len > 0);
            self.len = 0;
            Some(producer.into_iter())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<P> ExactSizeIterator for ChunkSeq<P>
where
    P: Producer,
{
    #[inline]
    fn len(&self) -> usize {
        self.len.div_ceil(self.chunk_size)
    }
}

impl<P> DoubleEndedIterator for ChunkSeq<P>
where
    P: Producer,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let producer = self.inner.take()?;
        if self.len > self.chunk_size {
            let mut size = self.len % self.chunk_size;
            if size == 0 {
                size = self.chunk_size;
            }
            let (left, right) = producer.split_at(self.len - size);
            self.inner = Some(left);
            self.len -= size;
            Some(right.into_iter())
        } else {
            debug_assert!(self.len > 0);
            self.len = 0;
            Some(producer.into_iter())
        }
    }
}
