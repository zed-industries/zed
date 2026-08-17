use super::noop::*;
use super::plumbing::*;
use super::ParallelIterator;

pub(super) fn for_each<I, F, T>(pi: I, op: &F)
where
    I: ParallelIterator<Item = T>,
    F: Fn(T) + Sync,
    T: Send,
{
    let consumer = ForEachConsumer { op };
    pi.drive_unindexed(consumer)
}

struct ForEachConsumer<'f, F> {
    op: &'f F,
}

impl<'f, F, T> Consumer<T> for ForEachConsumer<'f, F>
where
    F: Fn(T) + Sync,
{
    type Folder = ForEachConsumer<'f, F>;
    type Reducer = NoopReducer;
    type Result = ();

    fn split_at(self, _index: usize) -> (Self, Self, NoopReducer) {
        (self.split_off_left(), self, NoopReducer)
    }

    fn into_folder(self) -> Self {
        self
    }

    fn full(&self) -> bool {
        false
    }
}

impl<'f, F, T> Folder<T> for ForEachConsumer<'f, F>
where
    F: Fn(T) + Sync,
{
    type Result = ();

    fn consume(self, item: T) -> Self {
        (self.op)(item);
        self
    }

    fn consume_iter<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().for_each(self.op);
        self
    }

    fn complete(self) {}

    fn full(&self) -> bool {
        false
    }
}

impl<'f, F, T> UnindexedConsumer<T> for ForEachConsumer<'f, F>
where
    F: Fn(T) + Sync,
{
    fn split_off_left(&self) -> Self {
        ForEachConsumer { op: self.op }
    }

    fn to_reducer(&self) -> NoopReducer {
        NoopReducer
    }
}
