use super::plumbing::*;
use super::*;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};

/// `SkipAnyWhile` is an iterator that skips over elements from anywhere in `I`
/// until the callback returns `false`.
/// This struct is created by the [`skip_any_while()`] method on [`ParallelIterator`]
///
/// [`skip_any_while()`]: ParallelIterator::skip_any_while()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct SkipAnyWhile<I, P> {
    base: I,
    predicate: P,
}

impl<I: fmt::Debug, P> fmt::Debug for SkipAnyWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SkipAnyWhile")
            .field("base", &self.base)
            .finish()
    }
}

impl<I, P> SkipAnyWhile<I, P> {
    /// Creates a new `SkipAnyWhile` iterator.
    pub(super) fn new(base: I, predicate: P) -> Self {
        SkipAnyWhile { base, predicate }
    }
}

impl<I, P> ParallelIterator for SkipAnyWhile<I, P>
where
    I: ParallelIterator,
    P: Fn(&I::Item) -> bool + Sync + Send,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer1 = SkipAnyWhileConsumer {
            base: consumer,
            predicate: &self.predicate,
            skipping: &AtomicBool::new(true),
        };
        self.base.drive_unindexed(consumer1)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Consumer implementation

struct SkipAnyWhileConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    skipping: &'p AtomicBool,
}

impl<'p, T, C, P> Consumer<T> for SkipAnyWhileConsumer<'p, C, P>
where
    C: Consumer<T>,
    P: Fn(&T) -> bool + Sync,
{
    type Folder = SkipAnyWhileFolder<'p, C::Folder, P>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            SkipAnyWhileConsumer { base: left, ..self },
            SkipAnyWhileConsumer {
                base: right,
                ..self
            },
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        SkipAnyWhileFolder {
            base: self.base.into_folder(),
            predicate: self.predicate,
            skipping: self.skipping,
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<'p, T, C, P> UnindexedConsumer<T> for SkipAnyWhileConsumer<'p, C, P>
where
    C: UnindexedConsumer<T>,
    P: Fn(&T) -> bool + Sync,
{
    fn split_off_left(&self) -> Self {
        SkipAnyWhileConsumer {
            base: self.base.split_off_left(),
            ..*self
        }
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

struct SkipAnyWhileFolder<'p, C, P> {
    base: C,
    predicate: &'p P,
    skipping: &'p AtomicBool,
}

fn skip<T>(item: &T, skipping: &AtomicBool, predicate: &impl Fn(&T) -> bool) -> bool {
    if !skipping.load(Ordering::Relaxed) {
        return false;
    }
    if predicate(item) {
        return true;
    }
    skipping.store(false, Ordering::Relaxed);
    false
}

impl<'p, T, C, P> Folder<T> for SkipAnyWhileFolder<'p, C, P>
where
    C: Folder<T>,
    P: Fn(&T) -> bool + 'p,
{
    type Result = C::Result;

    fn consume(mut self, item: T) -> Self {
        if !skip(&item, self.skipping, self.predicate) {
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
                .skip_while(move |x| skip(x, self.skipping, self.predicate)),
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
