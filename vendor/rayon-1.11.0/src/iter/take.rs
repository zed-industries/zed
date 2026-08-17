use super::plumbing::*;
use super::*;

/// `Take` is an iterator that iterates over the first `n` elements.
/// This struct is created by the [`take()`] method on [`IndexedParallelIterator`]
///
/// [`take()`]: IndexedParallelIterator::take()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Take<I> {
    base: I,
    n: usize,
}

impl<I> Take<I>
where
    I: IndexedParallelIterator,
{
    /// Creates a new `Take` iterator.
    pub(super) fn new(base: I, n: usize) -> Self {
        let n = Ord::min(base.len(), n);
        Take { base, n }
    }
}

impl<I> ParallelIterator for Take<I>
where
    I: IndexedParallelIterator,
{
    type Item = I::Item;

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

impl<I> IndexedParallelIterator for Take<I>
where
    I: IndexedParallelIterator,
{
    fn len(&self) -> usize {
        self.n
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        return self.base.with_producer(Callback {
            callback,
            n: self.n,
        });

        struct Callback<CB> {
            callback: CB,
            n: usize,
        }

        impl<T, CB> ProducerCallback<T> for Callback<CB>
        where
            CB: ProducerCallback<T>,
        {
            type Output = CB::Output;
            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = T>,
            {
                let (producer, _) = base.split_at(self.n);
                self.callback.callback(producer)
            }
        }
    }
}
