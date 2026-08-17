use std::fmt::{self, Debug};

use super::chunks::ChunkProducer;
use super::plumbing::*;
use super::*;

/// `FoldChunks` is an iterator that groups elements of an underlying iterator and applies a
/// function over them, producing a single value for each group.
///
/// This struct is created by the [`fold_chunks()`] method on [`IndexedParallelIterator`]
///
/// [`fold_chunks()`]: IndexedParallelIterator::fold_chunks()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FoldChunks<I, ID, F> {
    base: I,
    chunk_size: usize,
    fold_op: F,
    identity: ID,
}

impl<I: Debug, ID, F> Debug for FoldChunks<I, ID, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fold")
            .field("base", &self.base)
            .field("chunk_size", &self.chunk_size)
            .finish()
    }
}

impl<I, ID, F> FoldChunks<I, ID, F> {
    /// Creates a new `FoldChunks` iterator
    pub(super) fn new(base: I, chunk_size: usize, identity: ID, fold_op: F) -> Self {
        FoldChunks {
            base,
            chunk_size,
            identity,
            fold_op,
        }
    }
}

impl<I, ID, U, F> ParallelIterator for FoldChunks<I, ID, F>
where
    I: IndexedParallelIterator,
    ID: Fn() -> U + Send + Sync,
    F: Fn(U, I::Item) -> U + Send + Sync,
    U: Send,
{
    type Item = U;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<U>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<I, ID, U, F> IndexedParallelIterator for FoldChunks<I, ID, F>
where
    I: IndexedParallelIterator,
    ID: Fn() -> U + Send + Sync,
    F: Fn(U, I::Item) -> U + Send + Sync,
    U: Send,
{
    fn len(&self) -> usize {
        self.base.len().div_ceil(self.chunk_size)
    }

    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        let len = self.base.len();
        return self.base.with_producer(Callback {
            chunk_size: self.chunk_size,
            len,
            identity: self.identity,
            fold_op: self.fold_op,
            callback,
        });

        struct Callback<CB, ID, F> {
            chunk_size: usize,
            len: usize,
            identity: ID,
            fold_op: F,
            callback: CB,
        }

        impl<T, CB, ID, U, F> ProducerCallback<T> for Callback<CB, ID, F>
        where
            CB: ProducerCallback<U>,
            ID: Fn() -> U + Send + Sync,
            F: Fn(U, T) -> U + Send + Sync,
        {
            type Output = CB::Output;

            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = T>,
            {
                let identity = &self.identity;
                let fold_op = &self.fold_op;
                let fold_iter = move |iter: P::IntoIter| iter.fold(identity(), fold_op);
                let producer = ChunkProducer::new(self.chunk_size, self.len, base, fold_iter);
                self.callback.callback(producer)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Add;

    #[test]
    fn check_fold_chunks() {
        let words = "bishbashbosh!"
            .chars()
            .collect::<Vec<_>>()
            .into_par_iter()
            .fold_chunks(4, String::new, |mut s, c| {
                s.push(c);
                s
            })
            .collect::<Vec<_>>();

        assert_eq!(words, vec!["bish", "bash", "bosh", "!"]);
    }

    // 'closure' values for tests below
    fn id() -> i32 {
        0
    }
    fn sum<T, U>(x: T, y: U) -> T
    where
        T: Add<U, Output = T>,
    {
        x + y
    }

    #[test]
    #[should_panic(expected = "chunk_size must not be zero")]
    fn check_fold_chunks_zero_size() {
        let _: Vec<i32> = vec![1, 2, 3]
            .into_par_iter()
            .fold_chunks(0, id, sum)
            .collect();
    }

    #[test]
    fn check_fold_chunks_even_size() {
        assert_eq!(
            vec![1 + 2 + 3, 4 + 5 + 6, 7 + 8 + 9],
            (1..10)
                .into_par_iter()
                .fold_chunks(3, id, sum)
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn check_fold_chunks_empty() {
        let v: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(
            expected,
            v.into_par_iter()
                .fold_chunks(2, id, sum)
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn check_fold_chunks_len() {
        assert_eq!(4, (0..8).into_par_iter().fold_chunks(2, id, sum).len());
        assert_eq!(3, (0..9).into_par_iter().fold_chunks(3, id, sum).len());
        assert_eq!(3, (0..8).into_par_iter().fold_chunks(3, id, sum).len());
        assert_eq!(1, [1].par_iter().fold_chunks(3, id, sum).len());
        assert_eq!(0, (0..0).into_par_iter().fold_chunks(3, id, sum).len());
    }

    #[test]
    fn check_fold_chunks_uneven() {
        let cases: Vec<(Vec<u32>, usize, Vec<u32>)> = vec![
            ((0..5).collect(), 3, vec![1 + 2, 3 + 4]),
            (vec![1], 5, vec![1]),
            ((0..4).collect(), 3, vec![1 + 2, 3]),
        ];

        for (i, (v, n, expected)) in cases.into_iter().enumerate() {
            let mut res: Vec<u32> = vec![];
            v.par_iter()
                .fold_chunks(n, || 0, sum)
                .collect_into_vec(&mut res);
            assert_eq!(expected, res, "Case {i} failed");

            res.truncate(0);
            v.into_par_iter()
                .fold_chunks(n, || 0, sum)
                .rev()
                .collect_into_vec(&mut res);
            assert_eq!(
                expected.into_iter().rev().collect::<Vec<u32>>(),
                res,
                "Case {i} reversed failed"
            );
        }
    }
}
