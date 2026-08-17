use super::plumbing::*;
use super::ParallelIterator;

pub(super) fn reduce<PI, R, ID, T>(pi: PI, identity: ID, reduce_op: R) -> T
where
    PI: ParallelIterator<Item = T>,
    R: Fn(T, T) -> T + Sync,
    ID: Fn() -> T + Sync,
    T: Send,
{
    let consumer = ReduceConsumer {
        identity: &identity,
        reduce_op: &reduce_op,
    };
    pi.drive_unindexed(consumer)
}

struct ReduceConsumer<'r, R, ID> {
    identity: &'r ID,
    reduce_op: &'r R,
}

impl<'r, R, ID> Copy for ReduceConsumer<'r, R, ID> {}

impl<'r, R, ID> Clone for ReduceConsumer<'r, R, ID> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'r, R, ID, T> Consumer<T> for ReduceConsumer<'r, R, ID>
where
    R: Fn(T, T) -> T + Sync,
    ID: Fn() -> T + Sync,
    T: Send,
{
    type Folder = ReduceFolder<'r, R, T>;
    type Reducer = Self;
    type Result = T;

    fn split_at(self, _index: usize) -> (Self, Self, Self) {
        (self, self, self)
    }

    fn into_folder(self) -> Self::Folder {
        ReduceFolder {
            reduce_op: self.reduce_op,
            item: (self.identity)(),
        }
    }

    fn full(&self) -> bool {
        false
    }
}

impl<'r, R, ID, T> UnindexedConsumer<T> for ReduceConsumer<'r, R, ID>
where
    R: Fn(T, T) -> T + Sync,
    ID: Fn() -> T + Sync,
    T: Send,
{
    fn split_off_left(&self) -> Self {
        *self
    }

    fn to_reducer(&self) -> Self::Reducer {
        *self
    }
}

impl<'r, R, ID, T> Reducer<T> for ReduceConsumer<'r, R, ID>
where
    R: Fn(T, T) -> T + Sync,
{
    fn reduce(self, left: T, right: T) -> T {
        (self.reduce_op)(left, right)
    }
}

struct ReduceFolder<'r, R, T> {
    reduce_op: &'r R,
    item: T,
}

impl<'r, R, T> Folder<T> for ReduceFolder<'r, R, T>
where
    R: Fn(T, T) -> T,
{
    type Result = T;

    fn consume(self, item: T) -> Self {
        ReduceFolder {
            reduce_op: self.reduce_op,
            item: (self.reduce_op)(self.item, item),
        }
    }

    fn consume_iter<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        ReduceFolder {
            reduce_op: self.reduce_op,
            item: iter.into_iter().fold(self.item, self.reduce_op),
        }
    }

    fn complete(self) -> T {
        self.item
    }

    fn full(&self) -> bool {
        false
    }
}
