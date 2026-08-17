use super::plumbing::*;
use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// `SkipAny` is an iterator that skips over `n` elements from anywhere in `I`.
/// This struct is created by the [`skip_any()`] method on [`ParallelIterator`]
///
/// [`skip_any()`]: ParallelIterator::skip_any()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct SkipAny<I> {
    base: I,
    count: usize,
}

impl<I> SkipAny<I> {
    /// Creates a new `SkipAny` iterator.
    pub(super) fn new(base: I, count: usize) -> Self {
        SkipAny { base, count }
    }
}

impl<I> ParallelIterator for SkipAny<I>
where
    I: ParallelIterator,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer1 = SkipAnyConsumer {
            base: consumer,
            count: &AtomicUsize::new(self.count),
        };
        self.base.drive_unindexed(consumer1)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct SkipAnyConsumer<'f, C> {
    base: C,
    count: &'f AtomicUsize,
}

impl<'f, T, C> Consumer<T> for SkipAnyConsumer<'f, C>
where
    C: Consumer<T>,
    T: Send,
{
    type Folder = SkipAnyFolder<'f, C::Folder>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            SkipAnyConsumer { base: left, ..self },
            SkipAnyConsumer {
                base: right,
                ..self
            },
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        SkipAnyFolder {
            base: self.base.into_folder(),
            count: self.count,
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<'f, T, C> UnindexedConsumer<T> for SkipAnyConsumer<'f, C>
where
    C: UnindexedConsumer<T>,
    T: Send,
{
    fn split_off_left(&self) -> Self {
        SkipAnyConsumer {
            base: self.base.split_off_left(),
            ..*self
        }
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct SkipAnyFolder<'f, C> {
    base: C,
    count: &'f AtomicUsize,
}

fn checked_decrement(u: &AtomicUsize) -> bool {
    u.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |u| u.checked_sub(1))
        .is_ok()
}

impl<'f, T, C> Folder<T> for SkipAnyFolder<'f, C>
where
    C: Folder<T>,
{
    type Result = C::Result;

    fn consume(mut self, item: T) -> Self {
        if !checked_decrement(self.count) {
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
                .skip_while(move |_| checked_decrement(self.count)),
        );
        self
    }

    fn complete(self) -> C::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}
