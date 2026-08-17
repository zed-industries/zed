use super::plumbing::*;
use super::*;
use std::iter;

/// `Zip` is an iterator that zips up `a` and `b` into a single iterator
/// of pairs. This struct is created by the [`zip()`] method on
/// [`IndexedParallelIterator`]
///
/// [`zip()`]: IndexedParallelIterator::zip()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Zip<A, B> {
    a: A,
    b: B,
}

impl<A, B> Zip<A, B> {
    /// Creates a new `Zip` iterator.
    pub(super) fn new(a: A, b: B) -> Self {
        Zip { a, b }
    }
}

impl<A, B> ParallelIterator for Zip<A, B>
where
    A: IndexedParallelIterator,
    B: IndexedParallelIterator,
{
    type Item = (A::Item, B::Item);

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

impl<A, B> IndexedParallelIterator for Zip<A, B>
where
    A: IndexedParallelIterator,
    B: IndexedParallelIterator,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        Ord::min(self.a.len(), self.b.len())
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        return self.a.with_producer(CallbackA {
            callback,
            b: self.b,
        });

        struct CallbackA<CB, B> {
            callback: CB,
            b: B,
        }

        impl<CB, ITEM, B> ProducerCallback<ITEM> for CallbackA<CB, B>
        where
            B: IndexedParallelIterator,
            CB: ProducerCallback<(ITEM, B::Item)>,
        {
            type Output = CB::Output;

            fn callback<A>(self, a_producer: A) -> Self::Output
            where
                A: Producer<Item = ITEM>,
            {
                self.b.with_producer(CallbackB {
                    a_producer,
                    callback: self.callback,
                })
            }
        }

        struct CallbackB<CB, A> {
            a_producer: A,
            callback: CB,
        }

        impl<CB, A, ITEM> ProducerCallback<ITEM> for CallbackB<CB, A>
        where
            A: Producer,
            CB: ProducerCallback<(A::Item, ITEM)>,
        {
            type Output = CB::Output;

            fn callback<B>(self, b_producer: B) -> Self::Output
            where
                B: Producer<Item = ITEM>,
            {
                self.callback.callback(ZipProducer {
                    a: self.a_producer,
                    b: b_producer,
                })
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////

struct ZipProducer<A: Producer, B: Producer> {
    a: A,
    b: B,
}

impl<A: Producer, B: Producer> Producer for ZipProducer<A, B> {
    type Item = (A::Item, B::Item);
    type IntoIter = iter::Zip<A::IntoIter, B::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.a.into_iter().zip(self.b.into_iter())
    }

    fn min_len(&self) -> usize {
        Ord::max(self.a.min_len(), self.b.min_len())
    }

    fn max_len(&self) -> usize {
        Ord::min(self.a.max_len(), self.b.max_len())
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (a_left, a_right) = self.a.split_at(index);
        let (b_left, b_right) = self.b.split_at(index);
        (
            ZipProducer {
                a: a_left,
                b: b_left,
            },
            ZipProducer {
                a: a_right,
                b: b_right,
            },
        )
    }
}
