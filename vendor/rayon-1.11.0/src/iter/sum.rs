use super::plumbing::*;
use super::ParallelIterator;

use std::iter::{self, Sum};
use std::marker::PhantomData;

pub(super) fn sum<PI, S>(pi: PI) -> S
where
    PI: ParallelIterator,
    S: Send + Sum<PI::Item> + Sum,
{
    pi.drive_unindexed(SumConsumer::new())
}

fn add<T: Sum>(left: T, right: T) -> T {
    [left, right].into_iter().sum()
}

struct SumConsumer<S: Send> {
    _marker: PhantomData<*const S>,
}

unsafe impl<S: Send> Send for SumConsumer<S> {}

impl<S: Send> SumConsumer<S> {
    fn new() -> SumConsumer<S> {
        SumConsumer {
            _marker: PhantomData,
        }
    }
}

impl<S, T> Consumer<T> for SumConsumer<S>
where
    S: Send + Sum<T> + Sum,
{
    type Folder = SumFolder<S>;
    type Reducer = Self;
    type Result = S;

    fn split_at(self, _index: usize) -> (Self, Self, Self) {
        (SumConsumer::new(), SumConsumer::new(), SumConsumer::new())
    }

    fn into_folder(self) -> Self::Folder {
        SumFolder {
            sum: iter::empty::<T>().sum(),
        }
    }

    fn full(&self) -> bool {
        false
    }
}

impl<S, T> UnindexedConsumer<T> for SumConsumer<S>
where
    S: Send + Sum<T> + Sum,
{
    fn split_off_left(&self) -> Self {
        SumConsumer::new()
    }

    fn to_reducer(&self) -> Self::Reducer {
        SumConsumer::new()
    }
}

impl<S> Reducer<S> for SumConsumer<S>
where
    S: Send + Sum,
{
    fn reduce(self, left: S, right: S) -> S {
        add(left, right)
    }
}

struct SumFolder<S> {
    sum: S,
}

impl<S, T> Folder<T> for SumFolder<S>
where
    S: Sum<T> + Sum,
{
    type Result = S;

    fn consume(self, item: T) -> Self {
        SumFolder {
            sum: add(self.sum, iter::once(item).sum()),
        }
    }

    fn consume_iter<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        SumFolder {
            sum: add(self.sum, iter.into_iter().sum()),
        }
    }

    fn complete(self) -> S {
        self.sum
    }

    fn full(&self) -> bool {
        false
    }
}
