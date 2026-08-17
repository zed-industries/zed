use super::plumbing::*;
use super::*;
use std::iter;

/// `StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step.
/// This struct is created by the [`step_by()`] method on [`IndexedParallelIterator`]
///
/// [`step_by()`]: IndexedParallelIterator::step_by()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct StepBy<I> {
    base: I,
    step: usize,
}

impl<I> StepBy<I> {
    /// Creates a new `StepBy` iterator.
    pub(super) fn new(base: I, step: usize) -> Self {
        StepBy { base, step }
    }
}

impl<I> ParallelIterator for StepBy<I>
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

impl<I> IndexedParallelIterator for StepBy<I>
where
    I: IndexedParallelIterator,
{
    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.base.len().div_ceil(self.step)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        let len = self.base.len();
        return self.base.with_producer(Callback {
            callback,
            step: self.step,
            len,
        });

        struct Callback<CB> {
            callback: CB,
            step: usize,
            len: usize,
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
                let producer = StepByProducer {
                    base,
                    step: self.step,
                    len: self.len,
                };
                self.callback.callback(producer)
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////
// Producer implementation

struct StepByProducer<P> {
    base: P,
    step: usize,
    len: usize,
}

impl<P> Producer for StepByProducer<P>
where
    P: Producer,
{
    type Item = P::Item;
    type IntoIter = iter::StepBy<P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter().step_by(self.step)
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let elem_index = Ord::min(index * self.step, self.len);

        let (left, right) = self.base.split_at(elem_index);
        (
            StepByProducer {
                base: left,
                step: self.step,
                len: elem_index,
            },
            StepByProducer {
                base: right,
                step: self.step,
                len: self.len - elem_index,
            },
        )
    }

    fn min_len(&self) -> usize {
        self.base.min_len().div_ceil(self.step)
    }

    fn max_len(&self) -> usize {
        self.base.max_len() / self.step
    }
}
