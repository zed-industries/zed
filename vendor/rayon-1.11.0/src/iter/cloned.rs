use super::plumbing::*;
use super::*;

use std::iter;

/// `Cloned` is an iterator that clones the elements of an underlying iterator.
///
/// This struct is created by the [`cloned()`] method on [`ParallelIterator`]
///
/// [`cloned()`]: ParallelIterator::cloned()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Cloned<I> {
    base: I,
}

impl<I> Cloned<I> {
    /// Creates a new `Cloned` iterator.
    pub(super) fn new(base: I) -> Self {
        Cloned { base }
    }
}

impl<'a, T, I> ParallelIterator for Cloned<I>
where
    I: ParallelIterator<Item = &'a T>,
    T: 'a + Clone + Send + Sync,
{
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer1 = ClonedConsumer::new(consumer);
        self.base.drive_unindexed(consumer1)
    }

    fn opt_len(&self) -> Option<usize> {
        self.base.opt_len()
    }
}

impl<'a, T, I> IndexedParallelIterator for Cloned<I>
where
    I: IndexedParallelIterator<Item = &'a T>,
    T: 'a + Clone + Send + Sync,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        let consumer1 = ClonedConsumer::new(consumer);
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
            T: 'a + Clone + Send,
        {
            type Output = CB::Output;

            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = &'a T>,
            {
                let producer = ClonedProducer { base };
                self.callback.callback(producer)
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////

struct ClonedProducer<P> {
    base: P,
}

impl<'a, T, P> Producer for ClonedProducer<P>
where
    P: Producer<Item = &'a T>,
    T: 'a + Clone,
{
    type Item = T;
    type IntoIter = iter::Cloned<P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter().cloned()
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
            ClonedProducer { base: left },
            ClonedProducer { base: right },
        )
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        self.base.fold_with(ClonedFolder { base: folder }).base
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct ClonedConsumer<C> {
    base: C,
}

impl<C> ClonedConsumer<C> {
    fn new(base: C) -> Self {
        ClonedConsumer { base }
    }
}

impl<'a, T, C> Consumer<&'a T> for ClonedConsumer<C>
where
    C: Consumer<T>,
    T: 'a + Clone,
{
    type Folder = ClonedFolder<C::Folder>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            ClonedConsumer::new(left),
            ClonedConsumer::new(right),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        ClonedFolder {
            base: self.base.into_folder(),
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<'a, T, C> UnindexedConsumer<&'a T> for ClonedConsumer<C>
where
    C: UnindexedConsumer<T>,
    T: 'a + Clone,
{
    fn split_off_left(&self) -> Self {
        ClonedConsumer::new(self.base.split_off_left())
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct ClonedFolder<F> {
    base: F,
}

impl<'a, T, F> Folder<&'a T> for ClonedFolder<F>
where
    F: Folder<T>,
    T: 'a + Clone,
{
    type Result = F::Result;

    fn consume(self, item: &'a T) -> Self {
        ClonedFolder {
            base: self.base.consume(item.clone()),
        }
    }

    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.base = self.base.consume_iter(iter.into_iter().cloned());
        self
    }

    fn complete(self) -> F::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}
