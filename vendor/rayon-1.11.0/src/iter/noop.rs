use super::plumbing::*;

pub(super) struct NoopConsumer;

impl<T> Consumer<T> for NoopConsumer {
    type Folder = NoopConsumer;
    type Reducer = NoopReducer;
    type Result = ();

    fn split_at(self, _index: usize) -> (Self, Self, NoopReducer) {
        (NoopConsumer, NoopConsumer, NoopReducer)
    }

    fn into_folder(self) -> Self {
        self
    }

    fn full(&self) -> bool {
        false
    }
}

impl<T> Folder<T> for NoopConsumer {
    type Result = ();

    fn consume(self, _item: T) -> Self {
        self
    }

    fn consume_iter<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().for_each(drop);
        self
    }

    fn complete(self) {}

    fn full(&self) -> bool {
        false
    }
}

impl<T> UnindexedConsumer<T> for NoopConsumer {
    fn split_off_left(&self) -> Self {
        NoopConsumer
    }

    fn to_reducer(&self) -> NoopReducer {
        NoopReducer
    }
}

pub(super) struct NoopReducer;

impl Reducer<()> for NoopReducer {
    fn reduce(self, _left: (), _right: ()) {}
}
