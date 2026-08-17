use super::plumbing::*;
use super::*;
use std::iter;
use std::ops::Range;

/// `Enumerate` is an iterator that returns the current count along with the element.
/// This struct is created by the [`enumerate()`] method on [`IndexedParallelIterator`]
///
/// [`enumerate()`]: IndexedParallelIterator::enumerate()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Enumerate<I> {
    base: I,
}

impl<I> Enumerate<I> {
    /// Creates a new `Enumerate` iterator.
    pub(super) fn new(base: I) -> Self {
        Enumerate { base }
    }
}

impl<I> ParallelIterator for Enumerate<I>
where
    I: IndexedParallelIterator,
{
    type Item = (usize, I::Item);

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

impl<I> IndexedParallelIterator for Enumerate<I>
where
    I: IndexedParallelIterator,
{
    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.base.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        return self.base.with_producer(Callback { callback });

        struct Callback<CB> {
            callback: CB,
        }

        impl<I, CB> ProducerCallback<I> for Callback<CB>
        where
            CB: ProducerCallback<(usize, I)>,
        {
            type Output = CB::Output;
            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = I>,
            {
                let producer = EnumerateProducer { base, offset: 0 };
                self.callback.callback(producer)
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////
// Producer implementation

struct EnumerateProducer<P> {
    base: P,
    offset: usize,
}

impl<P> Producer for EnumerateProducer<P>
where
    P: Producer,
{
    type Item = (usize, P::Item);
    type IntoIter = iter::Zip<Range<usize>, P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        // Enumerate only works for IndexedParallelIterators. Since those
        // have a max length of usize::MAX, their max index is
        // usize::MAX - 1, so the range 0..usize::MAX includes all
        // possible indices.
        //
        // However, we should to use a precise end to the range, otherwise
        // reversing the iterator may have to walk back a long ways before
        // `Zip::next_back` can produce anything.
        let base = self.base.into_iter();
        let end = self.offset + base.len();
        (self.offset..end).zip(base)
    }

    fn min_len(&self) -> usize {
        self.base.min_len()
    }
    fn max_len(&self) -> usize {
        self.base.max_len()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.base.split_at(index);
        (
            EnumerateProducer {
                base: left,
                offset: self.offset,
            },
            EnumerateProducer {
                base: right,
                offset: self.offset + index,
            },
        )
    }
}
