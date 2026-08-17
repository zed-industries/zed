use super::noop::NoopConsumer;
use super::plumbing::*;
use super::*;

/// `Skip` is an iterator that skips over the first `n` elements.
/// This struct is created by the [`skip()`] method on [`IndexedParallelIterator`]
///
/// [`skip()`]: IndexedParallelIterator::skip()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Skip<I> {
    base: I,
    n: usize,
}

impl<I> Skip<I>
where
    I: IndexedParallelIterator,
{
    /// Creates a new `Skip` iterator.
    pub(super) fn new(base: I, n: usize) -> Self {
        let n = Ord::min(base.len(), n);
        Skip { base, n }
    }
}

impl<I> ParallelIterator for Skip<I>
where
    I: IndexedParallelIterator,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<I> IndexedParallelIterator for Skip<I>
where
    I: IndexedParallelIterator,
{
    fn len(&self) -> usize {
        self.base.len() - self.n
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        return self.base.with_producer(Callback {
            callback,
            n: self.n,
        });

        struct Callback<CB> {
            callback: CB,
            n: usize,
        }

        impl<T, CB> ProducerCallback<T> for Callback<CB>
        where
            CB: ProducerCallback<T>,
        {
            type Output = CB::Output;
            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = T>,
            {
                crate::in_place_scope(|scope| {
                    let Self { callback, n } = self;
                    let (before_skip, after_skip) = base.split_at(n);

                    // Run the skipped part separately for side effects.
                    // We'll still get any panics propagated back by the scope.
                    scope.spawn(move |_| bridge_producer_consumer(n, before_skip, NoopConsumer));

                    callback.callback(after_skip)
                })
            }
        }
    }
}
