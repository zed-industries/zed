use super::plumbing::*;
use super::*;

use std::fmt::{self, Debug};

/// `Positions` takes a predicate `predicate` and filters out elements that match,
/// yielding their indices.
///
/// This struct is created by the [`positions()`] method on [`IndexedParallelIterator`]
///
/// [`positions()`]: IndexedParallelIterator::positions()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Positions<I, P> {
    base: I,
    predicate: P,
}

impl<I: Debug, P> Debug for Positions<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Positions")
            .field("base", &self.base)
            .finish()
    }
}

impl<I, P> Positions<I, P> {
    /// Create a new `Positions` iterator.
    pub(super) fn new(base: I, predicate: P) -> Self {
        Positions { base, predicate }
    }
}

impl<I, P> ParallelIterator for Positions<I, P>
where
    I: IndexedParallelIterator,
    P: Fn(I::Item) -> bool + Sync + Send,
{
    type Item = usize;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer1 = PositionsConsumer::new(consumer, &self.predicate, 0);
        self.base.drive(consumer1)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct PositionsConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    offset: usize,
}

impl<'p, C, P> PositionsConsumer<'p, C, P> {
    fn new(base: C, predicate: &'p P, offset: usize) -> Self {
        PositionsConsumer {
            base,
            predicate,
            offset,
        }
    }
}

impl<'p, T, C, P> Consumer<T> for PositionsConsumer<'p, C, P>
where
    C: Consumer<usize>,
    P: Fn(T) -> bool + Sync,
{
    type Folder = PositionsFolder<'p, C::Folder, P>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, C::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            PositionsConsumer::new(left, self.predicate, self.offset),
            PositionsConsumer::new(right, self.predicate, self.offset + index),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        PositionsFolder {
            base: self.base.into_folder(),
            predicate: self.predicate,
            offset: self.offset,
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

struct PositionsFolder<'p, F, P> {
    base: F,
    predicate: &'p P,
    offset: usize,
}

impl<F, P, T> Folder<T> for PositionsFolder<'_, F, P>
where
    F: Folder<usize>,
    P: Fn(T) -> bool,
{
    type Result = F::Result;

    fn consume(mut self, item: T) -> Self {
        let index = self.offset;
        self.offset += 1;
        if (self.predicate)(item) {
            self.base = self.base.consume(index);
        }
        self
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
