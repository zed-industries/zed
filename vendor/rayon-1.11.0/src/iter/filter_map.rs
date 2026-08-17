use super::plumbing::*;
use super::*;

use std::fmt::{self, Debug};

/// `FilterMap` creates an iterator that uses `filter_op` to both filter and map elements.
/// This struct is created by the [`filter_map()`] method on [`ParallelIterator`].
///
/// [`filter_map()`]: ParallelIterator::filter_map()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FilterMap<I, P> {
    base: I,
    filter_op: P,
}

impl<I: Debug, P> Debug for FilterMap<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterMap")
            .field("base", &self.base)
            .finish()
    }
}

impl<I, P> FilterMap<I, P> {
    /// Creates a new `FilterMap` iterator.
    pub(super) fn new(base: I, filter_op: P) -> Self {
        FilterMap { base, filter_op }
    }
}

impl<I, P, R> ParallelIterator for FilterMap<I, P>
where
    I: ParallelIterator,
    P: Fn(I::Item) -> Option<R> + Sync + Send,
    R: Send,
{
    type Item = R;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer = FilterMapConsumer::new(consumer, &self.filter_op);
        self.base.drive_unindexed(consumer)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct FilterMapConsumer<'p, C, P> {
    base: C,
    filter_op: &'p P,
}

impl<'p, C, P: 'p> FilterMapConsumer<'p, C, P> {
    fn new(base: C, filter_op: &'p P) -> Self {
        FilterMapConsumer { base, filter_op }
    }
}

impl<'p, T, U, C, P> Consumer<T> for FilterMapConsumer<'p, C, P>
where
    C: Consumer<U>,
    P: Fn(T) -> Option<U> + Sync + 'p,
{
    type Folder = FilterMapFolder<'p, C::Folder, P>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            FilterMapConsumer::new(left, self.filter_op),
            FilterMapConsumer::new(right, self.filter_op),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        let base = self.base.into_folder();
        FilterMapFolder {
            base,
            filter_op: self.filter_op,
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<'p, T, U, C, P> UnindexedConsumer<T> for FilterMapConsumer<'p, C, P>
where
    C: UnindexedConsumer<U>,
    P: Fn(T) -> Option<U> + Sync + 'p,
{
    fn split_off_left(&self) -> Self {
        FilterMapConsumer::new(self.base.split_off_left(), self.filter_op)
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct FilterMapFolder<'p, C, P> {
    base: C,
    filter_op: &'p P,
}

impl<'p, T, U, C, P> Folder<T> for FilterMapFolder<'p, C, P>
where
    C: Folder<U>,
    P: Fn(T) -> Option<U> + Sync + 'p,
{
    type Result = C::Result;

    fn consume(self, item: T) -> Self {
        let filter_op = self.filter_op;
        if let Some(mapped_item) = filter_op(item) {
            let base = self.base.consume(mapped_item);
            FilterMapFolder { base, filter_op }
        } else {
            self
        }
    }

    // This cannot easily specialize `consume_iter` to be better than
    // the default, because that requires checking `self.base.full()`
    // during a call to `self.base.consume_iter()`. (#632)

    fn complete(self) -> C::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}
