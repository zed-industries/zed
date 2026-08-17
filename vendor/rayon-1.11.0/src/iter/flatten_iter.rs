use super::plumbing::*;
use super::*;

/// `FlattenIter` turns each element to a serial iterator, then flattens these iterators
/// together. This struct is created by the [`flatten_iter()`] method on [`ParallelIterator`].
///
/// [`flatten_iter()`]: ParallelIterator::flatten_iter()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct FlattenIter<I> {
    base: I,
}

impl<I> FlattenIter<I> {
    /// Creates a new `FlattenIter` iterator.
    pub(super) fn new(base: I) -> Self {
        FlattenIter { base }
    }
}

impl<I> ParallelIterator for FlattenIter<I>
where
    I: ParallelIterator<Item: IntoIterator<Item: Send>>,
{
    type Item = <I::Item as IntoIterator>::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer = FlattenIterConsumer::new(consumer);
        self.base.drive_unindexed(consumer)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct FlattenIterConsumer<C> {
    base: C,
}

impl<C> FlattenIterConsumer<C> {
    fn new(base: C) -> Self {
        FlattenIterConsumer { base }
    }
}

impl<T, C> Consumer<T> for FlattenIterConsumer<C>
where
    C: UnindexedConsumer<T::Item>,
    T: IntoIterator,
{
    type Folder = FlattenIterFolder<C::Folder>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, C::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            FlattenIterConsumer::new(left),
            FlattenIterConsumer::new(right),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        FlattenIterFolder {
            base: self.base.into_folder(),
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<T, C> UnindexedConsumer<T> for FlattenIterConsumer<C>
where
    C: UnindexedConsumer<T::Item>,
    T: IntoIterator,
{
    fn split_off_left(&self) -> Self {
        FlattenIterConsumer::new(self.base.split_off_left())
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct FlattenIterFolder<C> {
    base: C,
}

impl<T, C> Folder<T> for FlattenIterFolder<C>
where
    C: Folder<T::Item>,
    T: IntoIterator,
{
    type Result = C::Result;

    fn consume(self, item: T) -> Self {
        let base = self.base.consume_iter(item);
        FlattenIterFolder { base }
    }

    fn consume_iter<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter().flatten();
        let base = self.base.consume_iter(iter);
        FlattenIterFolder { base }
    }

    fn complete(self) -> Self::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}
