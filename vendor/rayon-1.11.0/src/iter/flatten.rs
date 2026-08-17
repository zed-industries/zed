use super::plumbing::*;
use super::*;

/// `Flatten` turns each element to a parallel iterator, then flattens these iterators
/// together. This struct is created by the [`flatten()`] method on [`ParallelIterator`].
///
/// [`flatten()`]: ParallelIterator::flatten()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Flatten<I> {
    base: I,
}

impl<I> Flatten<I> {
    /// Creates a new `Flatten` iterator.
    pub(super) fn new(base: I) -> Self {
        Flatten { base }
    }
}

impl<I> ParallelIterator for Flatten<I>
where
    I: ParallelIterator<Item: IntoParallelIterator>,
{
    type Item = <I::Item as IntoParallelIterator>::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer = FlattenConsumer::new(consumer);
        self.base.drive_unindexed(consumer)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct FlattenConsumer<C> {
    base: C,
}

impl<C> FlattenConsumer<C> {
    fn new(base: C) -> Self {
        FlattenConsumer { base }
    }
}

impl<T, C> Consumer<T> for FlattenConsumer<C>
where
    C: UnindexedConsumer<T::Item>,
    T: IntoParallelIterator,
{
    type Folder = FlattenFolder<C, C::Result>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, C::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            FlattenConsumer::new(left),
            FlattenConsumer::new(right),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        FlattenFolder {
            base: self.base,
            previous: None,
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<T, C> UnindexedConsumer<T> for FlattenConsumer<C>
where
    C: UnindexedConsumer<T::Item>,
    T: IntoParallelIterator,
{
    fn split_off_left(&self) -> Self {
        FlattenConsumer::new(self.base.split_off_left())
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct FlattenFolder<C, R> {
    base: C,
    previous: Option<R>,
}

impl<T, C> Folder<T> for FlattenFolder<C, C::Result>
where
    C: UnindexedConsumer<T::Item>,
    T: IntoParallelIterator,
{
    type Result = C::Result;

    fn consume(self, item: T) -> Self {
        let par_iter = item.into_par_iter();
        let consumer = self.base.split_off_left();
        let result = par_iter.drive_unindexed(consumer);

        let previous = match self.previous {
            None => Some(result),
            Some(previous) => {
                let reducer = self.base.to_reducer();
                Some(reducer.reduce(previous, result))
            }
        };

        FlattenFolder {
            base: self.base,
            previous,
        }
    }

    fn complete(self) -> Self::Result {
        match self.previous {
            Some(previous) => previous,
            None => self.base.into_folder().complete(),
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}
