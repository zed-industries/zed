use super::plumbing::*;
use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// `TakeAny` is an iterator that iterates over `n` elements from anywhere in `I`.
/// This struct is created by the [`take_any()`] method on [`ParallelIterator`]
///
/// [`take_any()`]: ParallelIterator::take_any()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct TakeAny<I> {
    base: I,
    count: usize,
}

impl<I> TakeAny<I> {
    /// Creates a new `TakeAny` iterator.
    pub(super) fn new(base: I, count: usize) -> Self {
        TakeAny { base, count }
    }
}

impl<I> ParallelIterator for TakeAny<I>
where
    I: ParallelIterator,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer1 = TakeAnyConsumer {
            base: consumer,
            count: &AtomicUsize::new(self.count),
        };
        self.base.drive_unindexed(consumer1)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct TakeAnyConsumer<'f, C> {
    base: C,
    count: &'f AtomicUsize,
}

impl<'f, T, C> Consumer<T> for TakeAnyConsumer<'f, C>
where
    C: Consumer<T>,
    T: Send,
{
    type Folder = TakeAnyFolder<'f, C::Folder>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            TakeAnyConsumer { base: left, ..self },
            TakeAnyConsumer {
                base: right,
                ..self
            },
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        TakeAnyFolder {
            base: self.base.into_folder(),
            count: self.count,
        }
    }

    fn full(&self) -> bool {
        self.count.load(Ordering::Relaxed) == 0 || self.base.full()
    }
}

impl<'f, T, C> UnindexedConsumer<T> for TakeAnyConsumer<'f, C>
where
    C: UnindexedConsumer<T>,
    T: Send,
{
    fn split_off_left(&self) -> Self {
        TakeAnyConsumer {
            base: self.base.split_off_left(),
            ..*self
        }
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct TakeAnyFolder<'f, C> {
    base: C,
    count: &'f AtomicUsize,
}

fn checked_decrement(u: &AtomicUsize) -> bool {
    u.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |u| u.checked_sub(1))
        .is_ok()
}

impl<'f, T, C> Folder<T> for TakeAnyFolder<'f, C>
where
    C: Folder<T>,
{
    type Result = C::Result;

    fn consume(mut self, item: T) -> Self {
        if checked_decrement(self.count) {
            self.base = self.base.consume(item);
        }
        self
    }

    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        self.base = self.base.consume_iter(
            iter.into_iter()
                .take_while(move |_| checked_decrement(self.count)),
        );
        self
    }

    fn complete(self) -> C::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.count.load(Ordering::Relaxed) == 0 || self.base.full()
    }
}
