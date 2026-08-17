use super::plumbing::*;
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub(super) fn find<I, P>(pi: I, find_op: P) -> Option<I::Item>
where
    I: ParallelIterator,
    P: Fn(&I::Item) -> bool + Sync,
{
    let found = AtomicBool::new(false);
    let consumer = FindConsumer::new(&find_op, &found);
    pi.drive_unindexed(consumer)
}

struct FindConsumer<'p, P> {
    find_op: &'p P,
    found: &'p AtomicBool,
}

impl<'p, P> FindConsumer<'p, P> {
    fn new(find_op: &'p P, found: &'p AtomicBool) -> Self {
        FindConsumer { find_op, found }
    }
}

impl<'p, T, P: 'p> Consumer<T> for FindConsumer<'p, P>
where
    T: Send,
    P: Fn(&T) -> bool + Sync,
{
    type Folder = FindFolder<'p, T, P>;
    type Reducer = FindReducer;
    type Result = Option<T>;

    fn split_at(self, _index: usize) -> (Self, Self, Self::Reducer) {
        (self.split_off_left(), self, FindReducer)
    }

    fn into_folder(self) -> Self::Folder {
        FindFolder {
            find_op: self.find_op,
            found: self.found,
            item: None,
        }
    }

    fn full(&self) -> bool {
        self.found.load(Ordering::Relaxed)
    }
}

impl<'p, T, P: 'p> UnindexedConsumer<T> for FindConsumer<'p, P>
where
    T: Send,
    P: Fn(&T) -> bool + Sync,
{
    fn split_off_left(&self) -> Self {
        FindConsumer::new(self.find_op, self.found)
    }

    fn to_reducer(&self) -> Self::Reducer {
        FindReducer
    }
}

struct FindFolder<'p, T, P> {
    find_op: &'p P,
    found: &'p AtomicBool,
    item: Option<T>,
}

impl<'p, T, P> Folder<T> for FindFolder<'p, T, P>
where
    P: Fn(&T) -> bool + 'p,
{
    type Result = Option<T>;

    fn consume(mut self, item: T) -> Self {
        if (self.find_op)(&item) {
            self.found.store(true, Ordering::Relaxed);
            self.item = Some(item);
        }
        self
    }

    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        fn not_full<T>(found: &AtomicBool) -> impl Fn(&T) -> bool + '_ {
            move |_| !found.load(Ordering::Relaxed)
        }

        self.item = iter
            .into_iter()
            // stop iterating if another thread has found something
            .take_while(not_full(self.found))
            .find(self.find_op);
        if self.item.is_some() {
            self.found.store(true, Ordering::Relaxed)
        }
        self
    }

    fn complete(self) -> Self::Result {
        self.item
    }

    fn full(&self) -> bool {
        self.found.load(Ordering::Relaxed)
    }
}

struct FindReducer;

impl<T> Reducer<Option<T>> for FindReducer {
    fn reduce(self, left: Option<T>, right: Option<T>) -> Option<T> {
        left.or(right)
    }
}
