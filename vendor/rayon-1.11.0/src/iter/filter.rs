use super::plumbing::*;
use super::*;

use std::fmt::{self, Debug};

/// `Filter` takes a predicate `filter_op` and filters out elements that match.
/// This struct is created by the [`filter()`] method on [`ParallelIterator`]
///
/// [`filter()`]: ParallelIterator::filter()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Filter<I, P> {
    base: I,
    filter_op: P,
}

impl<I: Debug, P> Debug for Filter<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Filter").field("base", &self.base).finish()
    }
}

impl<I, P> Filter<I, P> {
    /// Creates a new `Filter` iterator.
    pub(super) fn new(base: I, filter_op: P) -> Self {
        Filter { base, filter_op }
    }
}

impl<I, P> ParallelIterator for Filter<I, P>
where
    I: ParallelIterator,
    P: Fn(&I::Item) -> bool + Sync + Send,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer1 = FilterConsumer::new(consumer, &self.filter_op);
        self.base.drive_unindexed(consumer1)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct FilterConsumer<'p, C, P> {
    base: C,
    filter_op: &'p P,
}

impl<'p, C, P> FilterConsumer<'p, C, P> {
    fn new(base: C, filter_op: &'p P) -> Self {
        FilterConsumer { base, filter_op }
    }
}

impl<'p, T, C, P: 'p> Consumer<T> for FilterConsumer<'p, C, P>
where
    C: Consumer<T>,
    P: Fn(&T) -> bool + Sync,
{
    type Folder = FilterFolder<'p, C::Folder, P>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, C::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            FilterConsumer::new(left, self.filter_op),
            FilterConsumer::new(right, self.filter_op),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        FilterFolder {
            base: self.base.into_folder(),
            filter_op: self.filter_op,
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<'p, T, C, P: 'p> UnindexedConsumer<T> for FilterConsumer<'p, C, P>
where
    C: UnindexedConsumer<T>,
    P: Fn(&T) -> bool + Sync,
{
    fn split_off_left(&self) -> Self {
        FilterConsumer::new(self.base.split_off_left(), self.filter_op)
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct FilterFolder<'p, C, P> {
    base: C,
    filter_op: &'p P,
}

impl<'p, C, P, T> Folder<T> for FilterFolder<'p, C, P>
where
    C: Folder<T>,
    P: Fn(&T) -> bool + 'p,
{
    type Result = C::Result;

    fn consume(self, item: T) -> Self {
        let filter_op = self.filter_op;
        if filter_op(&item) {
            let base = self.base.consume(item);
            FilterFolder { base, filter_op }
        } else {
            self
        }
    }

    // This cannot easily specialize `consume_iter` to be better than
    // the default, because that requires checking `self.base.full()`
    // during a call to `self.base.consume_iter()`. (#632)

    fn complete(self) -> Self::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}
