use super::plumbing::*;
use super::*;

use std::iter;

/// `Copied` is an iterator that copies the elements of an underlying iterator.
///
/// This struct is created by the [`copied()`] method on [`ParallelIterator`]
///
/// [`copied()`]: ParallelIterator::copied()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Copied<I> {
    base: I,
}

impl<I> Copied<I> {
    /// Creates a new `Copied` iterator.
    pub(super) fn new(base: I) -> Self {
        Copied { base }
    }
}

impl<'a, T, I> ParallelIterator for Copied<I>
where
    I: ParallelIterator<Item = &'a T>,
    T: 'a + Copy + Send + Sync,
{
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer1 = CopiedConsumer::new(consumer);
        self.base.drive_unindexed(consumer1)
    }

    fn opt_len(&self) -> Option<usize> {
        self.base.opt_len()
    }
}

impl<'a, T, I> IndexedParallelIterator for Copied<I>
where
    I: IndexedParallelIterator<Item = &'a T>,
    T: 'a + Copy + Send + Sync,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        let consumer1 = CopiedConsumer::new(consumer);
        self.base.drive(consumer1)
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

        impl<'a, T, CB> ProducerCallback<&'a T> for Callback<CB>
        where
            CB: ProducerCallback<T>,
            T: 'a + Copy + Send,
        {
            type Output = CB::Output;

            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = &'a T>,
            {
                let producer = CopiedProducer { base };
                self.callback.callback(producer)
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////

struct CopiedProducer<P> {
    base: P,
}

impl<'a, T, P> Producer for CopiedProducer<P>
where
    P: Producer<Item = &'a T>,
    T: 'a + Copy,
{
    type Item = T;
    type IntoIter = iter::Copied<P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter().copied()
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
            CopiedProducer { base: left },
            CopiedProducer { base: right },
        )
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        self.base.fold_with(CopiedFolder { base: folder }).base
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct CopiedConsumer<C> {
    base: C,
}

impl<C> CopiedConsumer<C> {
    fn new(base: C) -> Self {
        CopiedConsumer { base }
    }
}

impl<'a, T, C> Consumer<&'a T> for CopiedConsumer<C>
where
    C: Consumer<T>,
    T: 'a + Copy,
{
    type Folder = CopiedFolder<C::Folder>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            CopiedConsumer::new(left),
            CopiedConsumer::new(right),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        CopiedFolder {
            base: self.base.into_folder(),
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<'a, T, C> UnindexedConsumer<&'a T> for CopiedConsumer<C>
where
    C: UnindexedConsumer<T>,
    T: 'a + Copy,
{
    fn split_off_left(&self) -> Self {
        CopiedConsumer::new(self.base.split_off_left())
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct CopiedFolder<F> {
    base: F,
}

impl<'a, T, F> Folder<&'a T> for CopiedFolder<F>
where
    F: Folder<T>,
    T: 'a + Copy,
{
    type Result = F::Result;

    fn consume(self, &item: &'a T) -> Self {
        CopiedFolder {
            base: self.base.consume(item),
        }
    }

    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.base = self.base.consume_iter(iter.into_iter().copied());
        self
    }

    fn complete(self) -> F::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}
