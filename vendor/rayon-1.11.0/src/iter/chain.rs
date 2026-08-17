use super::plumbing::*;
use super::*;
use rayon_core::join;
use std::iter;

/// `Chain` is an iterator that joins `b` after `a` in one continuous iterator.
/// This struct is created by the [`chain()`] method on [`ParallelIterator`]
///
/// [`chain()`]: ParallelIterator::chain()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Chain<A, B> {
    a: A,
    b: B,
}

impl<A, B> Chain<A, B> {
    /// Creates a new `Chain` iterator.
    pub(super) fn new(a: A, b: B) -> Self {
        Chain { a, b }
    }
}

impl<A, B> ParallelIterator for Chain<A, B>
where
    A: ParallelIterator,
    B: ParallelIterator<Item = A::Item>,
{
    type Item = A::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let Chain { a, b } = self;

        // If we returned a value from our own `opt_len`, then the collect consumer in particular
        // will balk at being treated like an actual `UnindexedConsumer`.  But when we do know the
        // length, we can use `Consumer::split_at` instead, and this is still harmless for other
        // truly-unindexed consumers too.
        let (left, right, reducer) = if let Some(len) = a.opt_len() {
            consumer.split_at(len)
        } else {
            let reducer = consumer.to_reducer();
            (consumer.split_off_left(), consumer, reducer)
        };

        let (a, b) = join(|| a.drive_unindexed(left), || b.drive_unindexed(right));
        reducer.reduce(a, b)
    }

    fn opt_len(&self) -> Option<usize> {
        self.a.opt_len()?.checked_add(self.b.opt_len()?)
    }
}

impl<A, B> IndexedParallelIterator for Chain<A, B>
where
    A: IndexedParallelIterator,
    B: IndexedParallelIterator<Item = A::Item>,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        let Chain { a, b } = self;
        let (left, right, reducer) = consumer.split_at(a.len());
        let (a, b) = join(|| a.drive(left), || b.drive(right));
        reducer.reduce(a, b)
    }

    fn len(&self) -> usize {
        self.a.len().checked_add(self.b.len()).expect("overflow")
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        let a_len = self.a.len();
        return self.a.with_producer(CallbackA {
            callback,
            a_len,
            b: self.b,
        });

        struct CallbackA<CB, B> {
            callback: CB,
            a_len: usize,
            b: B,
        }

        impl<CB, B> ProducerCallback<B::Item> for CallbackA<CB, B>
        where
            B: IndexedParallelIterator,
            CB: ProducerCallback<B::Item>,
        {
            type Output = CB::Output;

            fn callback<A>(self, a_producer: A) -> Self::Output
            where
                A: Producer<Item = B::Item>,
            {
                self.b.with_producer(CallbackB {
                    callback: self.callback,
                    a_len: self.a_len,
                    a_producer,
                })
            }
        }

        struct CallbackB<CB, A> {
            callback: CB,
            a_len: usize,
            a_producer: A,
        }

        impl<CB, A> ProducerCallback<A::Item> for CallbackB<CB, A>
        where
            A: Producer,
            CB: ProducerCallback<A::Item>,
        {
            type Output = CB::Output;

            fn callback<B>(self, b_producer: B) -> Self::Output
            where
                B: Producer<Item = A::Item>,
            {
                let producer = ChainProducer::new(self.a_len, self.a_producer, b_producer);
                self.callback.callback(producer)
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////

struct ChainProducer<A, B>
where
    A: Producer,
    B: Producer<Item = A::Item>,
{
    a_len: usize,
    a: A,
    b: B,
}

impl<A, B> ChainProducer<A, B>
where
    A: Producer,
    B: Producer<Item = A::Item>,
{
    fn new(a_len: usize, a: A, b: B) -> Self {
        ChainProducer { a_len, a, b }
    }
}

impl<A, B> Producer for ChainProducer<A, B>
where
    A: Producer,
    B: Producer<Item = A::Item>,
{
    type Item = A::Item;
    type IntoIter = ChainSeq<A::IntoIter, B::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        ChainSeq::new(self.a.into_iter(), self.b.into_iter())
    }

    fn min_len(&self) -> usize {
        Ord::max(self.a.min_len(), self.b.min_len())
    }

    fn max_len(&self) -> usize {
        Ord::min(self.a.max_len(), self.b.max_len())
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        if index <= self.a_len {
            let a_rem = self.a_len - index;
            let (a_left, a_right) = self.a.split_at(index);
            let (b_left, b_right) = self.b.split_at(0);
            (
                ChainProducer::new(index, a_left, b_left),
                ChainProducer::new(a_rem, a_right, b_right),
            )
        } else {
            let (a_left, a_right) = self.a.split_at(self.a_len);
            let (b_left, b_right) = self.b.split_at(index - self.a_len);
            (
                ChainProducer::new(self.a_len, a_left, b_left),
                ChainProducer::new(0, a_right, b_right),
            )
        }
    }

    fn fold_with<F>(self, mut folder: F) -> F
    where
        F: Folder<A::Item>,
    {
        folder = self.a.fold_with(folder);
        if folder.full() {
            folder
        } else {
            self.b.fold_with(folder)
        }
    }
}

// ////////////////////////////////////////////////////////////////////////

/// Wrapper for `Chain` to implement `ExactSizeIterator`
struct ChainSeq<A, B> {
    chain: iter::Chain<A, B>,
}

impl<A, B> ChainSeq<A, B> {
    fn new(a: A, b: B) -> ChainSeq<A, B>
    where
        A: ExactSizeIterator,
        B: ExactSizeIterator<Item = A::Item>,
    {
        ChainSeq { chain: a.chain(b) }
    }
}

impl<A, B> Iterator for ChainSeq<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.chain.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chain.size_hint()
    }
}

impl<A, B> ExactSizeIterator for ChainSeq<A, B>
where
    A: ExactSizeIterator,
    B: ExactSizeIterator<Item = A::Item>,
{
}

impl<A, B> DoubleEndedIterator for ChainSeq<A, B>
where
    A: DoubleEndedIterator,
    B: DoubleEndedIterator<Item = A::Item>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.chain.next_back()
    }
}
