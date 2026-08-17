use std::fmt::{self, Debug};

use super::chunks::ChunkProducer;
use super::plumbing::*;
use super::*;

/// `FoldChunksWith` is an iterator that groups elements of an underlying iterator and applies a
/// function over them, producing a single value for each group.
///
/// This struct is created by the [`fold_chunks_with()`] method on [`IndexedParallelIterator`]
///
/// [`fold_chunks_with()`]: IndexedParallelIterator::fold_chunks()
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FoldChunksWith<I, U, F> {
    base: I,
    chunk_size: usize,
    item: U,
    fold_op: F,
}

impl<I: Debug, U: Debug, F> Debug for FoldChunksWith<I, U, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fold")
            .field("base", &self.base)
            .field("chunk_size", &self.chunk_size)
            .field("item", &self.item)
            .finish()
    }
}

impl<I, U, F> FoldChunksWith<I, U, F> {
    /// Creates a new `FoldChunksWith` iterator
    pub(super) fn new(base: I, chunk_size: usize, item: U, fold_op: F) -> Self {
        FoldChunksWith {
            base,
            chunk_size,
            item,
            fold_op,
        }
    }
}

impl<I, U, F> ParallelIterator for FoldChunksWith<I, U, F>
where
    I: IndexedParallelIterator,
    U: Send + Clone,
    F: Fn(U, I::Item) -> U + Send + Sync,
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

impl<I, U, F> IndexedParallelIterator for FoldChunksWith<I, U, F>
where
    I: IndexedParallelIterator,
    U: Send + Clone,
    F: Fn(U, I::Item) -> U + Send + Sync,
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
            item: self.item,
            fold_op: self.fold_op,
            callback,
        });

        struct Callback<CB, T, F> {
            chunk_size: usize,
            len: usize,
            item: T,
            fold_op: F,
            callback: CB,
        }

        impl<T, U, F, CB> ProducerCallback<T> for Callback<CB, U, F>
        where
            CB: ProducerCallback<U>,
            U: Send + Clone,
            F: Fn(U, T) -> U + Send + Sync,
        {
            type Output = CB::Output;

            fn callback<P>(self, base: P) -> CB::Output
            where
                P: Producer<Item = T>,
            {
                let item = self.item;
                let fold_op = &self.fold_op;
                let fold_iter = move |iter: P::IntoIter| iter.fold(item.clone(), fold_op);
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
    fn check_fold_chunks_with() {
        let words = "bishbashbosh!"
            .chars()
            .collect::<Vec<_>>()
            .into_par_iter()
            .fold_chunks_with(4, String::new(), |mut s, c| {
                s.push(c);
                s
            })
            .collect::<Vec<_>>();

        assert_eq!(words, vec!["bish", "bash", "bosh", "!"]);
    }

    // 'closure' value for tests below
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
            .fold_chunks_with(0, 0, sum)
            .collect();
    }

    #[test]
    fn check_fold_chunks_even_size() {
        assert_eq!(
            vec![1 + 2 + 3, 4 + 5 + 6, 7 + 8 + 9],
            (1..10)
                .into_par_iter()
                .fold_chunks_with(3, 0, sum)
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn check_fold_chunks_with_empty() {
        let v: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(
            expected,
            v.into_par_iter()
                .fold_chunks_with(2, 0, sum)
                .collect::<Vec<i32>>()
        );
    }

    #[test]
    fn check_fold_chunks_len() {
        assert_eq!(4, (0..8).into_par_iter().fold_chunks_with(2, 0, sum).len());
        assert_eq!(3, (0..9).into_par_iter().fold_chunks_with(3, 0, sum).len());
        assert_eq!(3, (0..8).into_par_iter().fold_chunks_with(3, 0, sum).len());
        assert_eq!(1, [1].par_iter().fold_chunks_with(3, 0, sum).len());
        assert_eq!(0, (0..0).into_par_iter().fold_chunks_with(3, 0, sum).len());
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
                .fold_chunks_with(n, 0, sum)
                .collect_into_vec(&mut res);
            assert_eq!(expected, res, "Case {i} failed");

            res.truncate(0);
            v.into_par_iter()
                .fold_chunks_with(n, 0, sum)
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
