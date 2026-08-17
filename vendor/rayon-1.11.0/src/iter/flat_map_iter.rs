use super::plumbing::*;
use super::*;

use std::fmt::{self, Debug};

/// `FlatMapIter` maps each element to a serial iterator, then flattens these iterators together.
/// This struct is created by the [`flat_map_iter()`] method on [`ParallelIterator`]
///
/// [`flat_map_iter()`]: ParallelIterator::flat_map_iter()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FlatMapIter<I, F> {
    base: I,
    map_op: F,
}

impl<I: Debug, F> Debug for FlatMapIter<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlatMapIter")
            .field("base", &self.base)
            .finish()
    }
}

impl<I, F> FlatMapIter<I, F> {
    /// Creates a new `FlatMapIter` iterator.
    pub(super) fn new(base: I, map_op: F) -> Self {
        FlatMapIter { base, map_op }
    }
}

impl<I, F, SI> ParallelIterator for FlatMapIter<I, F>
where
    I: ParallelIterator,
    F: Fn(I::Item) -> SI + Sync + Send,
    SI: IntoIterator<Item: Send>,
{
    type Item = SI::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer = FlatMapIterConsumer::new(consumer, &self.map_op);
        self.base.drive_unindexed(consumer)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct FlatMapIterConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}

impl<'f, C, F> FlatMapIterConsumer<'f, C, F> {
    fn new(base: C, map_op: &'f F) -> Self {
        FlatMapIterConsumer { base, map_op }
    }
}

impl<'f, T, U, C, F> Consumer<T> for FlatMapIterConsumer<'f, C, F>
where
    C: UnindexedConsumer<U::Item>,
    F: Fn(T) -> U + Sync,
    U: IntoIterator,
{
    type Folder = FlatMapIterFolder<'f, C::Folder, F>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, C::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            FlatMapIterConsumer::new(left, self.map_op),
            FlatMapIterConsumer::new(right, self.map_op),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        FlatMapIterFolder {
            base: self.base.into_folder(),
            map_op: self.map_op,
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<'f, T, U, C, F> UnindexedConsumer<T> for FlatMapIterConsumer<'f, C, F>
where
    C: UnindexedConsumer<U::Item>,
    F: Fn(T) -> U + Sync,
    U: IntoIterator,
{
    fn split_off_left(&self) -> Self {
        FlatMapIterConsumer::new(self.base.split_off_left(), self.map_op)
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct FlatMapIterFolder<'f, C, F> {
    base: C,
    map_op: &'f F,
}

impl<'f, T, U, C, F> Folder<T> for FlatMapIterFolder<'f, C, F>
where
    C: Folder<U::Item>,
    F: Fn(T) -> U,
    U: IntoIterator,
{
    type Result = C::Result;

    fn consume(self, item: T) -> Self {
        let map_op = self.map_op;
        let base = self.base.consume_iter(map_op(item));
        FlatMapIterFolder { base, map_op }
    }

    fn consume_iter<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let map_op = self.map_op;
        let iter = iter.into_iter().flat_map(map_op);
        let base = self.base.consume_iter(iter);
        FlatMapIterFolder { base, map_op }
    }

    fn complete(self) -> Self::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}
