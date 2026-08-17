use super::plumbing::*;
use super::*;

/// `MinLen` is an iterator that imposes a minimum length on iterator splits.
/// This struct is created by the [`with_min_len()`] method on [`IndexedParallelIterator`]
///
/// [`with_min_len()`]: IndexedParallelIterator::with_min_len()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct MinLen<I> {
    base: I,
    min: usize,
}

impl<I> MinLen<I> {
    /// Creates a new `MinLen` iterator.
    pub(super) fn new(base: I, min: usize) -> Self {
        MinLen { base, min }
    }
}

impl<I> ParallelIterator for MinLen<I>
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

impl<I> IndexedParallelIterator for MinLen<I>
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
        return self.base.with_producer(Callback {
            callback,
            min: self.min,
        });

        struct Callback<CB> {
            callback: CB,
            min: usize,
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
                let producer = MinLenProducer {
                    base,
                    min: self.min,
                };
                self.callback.callback(producer)
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////
// `MinLenProducer` implementation

struct MinLenProducer<P> {
    base: P,
    min: usize,
}

impl<P> Producer for MinLenProducer<P>
where
    P: Producer,
{
    type Item = P::Item;
    type IntoIter = P::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }

    fn min_len(&self) -> usize {
        Ord::max(self.min, self.base.min_len())
    }

    fn max_len(&self) -> usize {
        self.base.max_len()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.base.split_at(index);
        (
            MinLenProducer {
                base: left,
                min: self.min,
            },
            MinLenProducer {
                base: right,
                min: self.min,
            },
        )
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        self.base.fold_with(folder)
    }
}

/// `MaxLen` is an iterator that imposes a maximum length on iterator splits.
/// This struct is created by the [`with_max_len()`] method on [`IndexedParallelIterator`]
///
/// [`with_max_len()`]: IndexedParallelIterator::with_max_len()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct MaxLen<I> {
    base: I,
    max: usize,
}

impl<I> MaxLen<I> {
    /// Creates a new `MaxLen` iterator.
    pub(super) fn new(base: I, max: usize) -> Self {
        MaxLen { base, max }
    }
}

impl<I> ParallelIterator for MaxLen<I>
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

impl<I> IndexedParallelIterator for MaxLen<I>
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
        return self.base.with_producer(Callback {
            callback,
            max: self.max,
        });

        struct Callback<CB> {
            callback: CB,
            max: usize,
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
                let producer = MaxLenProducer {
                    base,
                    max: self.max,
                };
                self.callback.callback(producer)
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////
// `MaxLenProducer` implementation

struct MaxLenProducer<P> {
    base: P,
    max: usize,
}

impl<P> Producer for MaxLenProducer<P>
where
    P: Producer,
{
    type Item = P::Item;
    type IntoIter = P::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }

    fn min_len(&self) -> usize {
        self.base.min_len()
    }

    fn max_len(&self) -> usize {
        Ord::min(self.max, self.base.max_len())
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.base.split_at(index);
        (
            MaxLenProducer {
                base: left,
                max: self.max,
            },
            MaxLenProducer {
                base: right,
                max: self.max,
            },
        )
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        self.base.fold_with(folder)
    }
}
