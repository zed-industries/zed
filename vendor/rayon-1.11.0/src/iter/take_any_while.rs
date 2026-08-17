use super::plumbing::*;
use super::*;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};

/// `TakeAnyWhile` is an iterator that iterates over elements from anywhere in `I`
/// until the callback returns `false`.
/// This struct is created by the [`take_any_while()`] method on [`ParallelIterator`]
///
/// [`take_any_while()`]: ParallelIterator::take_any_while()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct TakeAnyWhile<I, P> {
    base: I,
    predicate: P,
}

impl<I: fmt::Debug, P> fmt::Debug for TakeAnyWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TakeAnyWhile")
            .field("base", &self.base)
            .finish()
    }
}

impl<I, P> TakeAnyWhile<I, P> {
    /// Creates a new `TakeAnyWhile` iterator.
    pub(super) fn new(base: I, predicate: P) -> Self {
        TakeAnyWhile { base, predicate }
    }
}

impl<I, P> ParallelIterator for TakeAnyWhile<I, P>
where
    I: ParallelIterator,
    P: Fn(&I::Item) -> bool + Sync + Send,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer1 = TakeAnyWhileConsumer {
            base: consumer,
            predicate: &self.predicate,
            taking: &AtomicBool::new(true),
        };
        self.base.drive_unindexed(consumer1)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct TakeAnyWhileConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    taking: &'p AtomicBool,
}

impl<'p, T, C, P> Consumer<T> for TakeAnyWhileConsumer<'p, C, P>
where
    C: Consumer<T>,
    P: Fn(&T) -> bool + Sync,
{
    type Folder = TakeAnyWhileFolder<'p, C::Folder, P>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            TakeAnyWhileConsumer { base: left, ..self },
            TakeAnyWhileConsumer {
                base: right,
                ..self
            },
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        TakeAnyWhileFolder {
            base: self.base.into_folder(),
            predicate: self.predicate,
            taking: self.taking,
        }
    }

    fn full(&self) -> bool {
        !self.taking.load(Ordering::Relaxed) || self.base.full()
    }
}

impl<'p, T, C, P> UnindexedConsumer<T> for TakeAnyWhileConsumer<'p, C, P>
where
    C: UnindexedConsumer<T>,
    P: Fn(&T) -> bool + Sync,
{
    fn split_off_left(&self) -> Self {
        TakeAnyWhileConsumer {
            base: self.base.split_off_left(),
            ..*self
        }
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct TakeAnyWhileFolder<'p, C, P> {
    base: C,
    predicate: &'p P,
    taking: &'p AtomicBool,
}

fn take<T>(item: &T, taking: &AtomicBool, predicate: &impl Fn(&T) -> bool) -> bool {
    if !taking.load(Ordering::Relaxed) {
        return false;
    }
    if predicate(item) {
        return true;
    }
    taking.store(false, Ordering::Relaxed);
    false
}

impl<'p, T, C, P> Folder<T> for TakeAnyWhileFolder<'p, C, P>
where
    C: Folder<T>,
    P: Fn(&T) -> bool + 'p,
{
    type Result = C::Result;

    fn consume(mut self, item: T) -> Self {
        if take(&item, self.taking, self.predicate) {
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
                .take_while(move |x| take(x, self.taking, self.predicate)),
        );
        self
    }

    fn complete(self) -> C::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        !self.taking.load(Ordering::Relaxed) || self.base.full()
    }
}
