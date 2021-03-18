use crate::{
    sum_tree::{Cursor, Dimension, Edit, Item, KeyedItem, SumTree},
    time,
};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

pub trait Operation: Clone + Debug + Eq {
    fn timestamp(&self) -> time::Lamport;
}

#[derive(Clone, Debug)]
pub struct OperationQueue<T: Operation>(SumTree<T>);

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct OperationKey(time::Lamport);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OperationSummary {
    key: OperationKey,
    len: usize,
}

impl<T: Operation> OperationQueue<T> {
    pub fn new() -> Self {
        OperationQueue(SumTree::new())
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.summary().len
    }

    pub fn insert(&mut self, mut ops: Vec<T>) {
        ops.sort_by_key(|op| op.timestamp());
        ops.dedup_by_key(|op| op.timestamp());
        let mut edits = ops
            .into_iter()
            .map(|op| Edit::Insert(op))
            .collect::<Vec<_>>();
        self.0.edit(&mut edits);
    }

    pub fn drain(&mut self) -> Self {
        let clone = self.clone();
        self.0 = SumTree::new();
        clone
    }

    pub fn cursor(&self) -> Cursor<T, (), ()> {
        self.0.cursor()
    }
}

impl<T: Operation> Item for T {
    type Summary = OperationSummary;

    fn summary(&self) -> Self::Summary {
        OperationSummary {
            key: OperationKey(self.timestamp()),
            len: 1,
        }
    }
}

impl<T: Operation> KeyedItem for T {
    type Key = OperationKey;

    fn key(&self) -> Self::Key {
        OperationKey(self.timestamp())
    }
}

impl<'a> AddAssign<&'a Self> for OperationSummary {
    fn add_assign(&mut self, other: &Self) {
        assert!(self.key < other.key);
        self.key = other.key;
        self.len += other.len;
    }
}

impl<'a> Add<&'a Self> for OperationSummary {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        assert!(self.key < other.key);
        OperationSummary {
            key: other.key,
            len: self.len + other.len,
        }
    }
}

impl<'a> Dimension<'a, OperationSummary> for OperationKey {
    fn add_summary(&mut self, summary: &OperationSummary) {
        assert!(*self <= summary.key);
        *self = summary.key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let mut clock = time::Lamport::new(0);

        let mut queue = OperationQueue::new();
        assert_eq!(queue.len(), 0);

        queue.insert(vec![
            TestOperation(clock.tick()),
            TestOperation(clock.tick()),
        ]);
        assert_eq!(queue.len(), 2);

        queue.insert(vec![TestOperation(clock.tick())]);
        assert_eq!(queue.len(), 3);

        drop(queue.drain());
        assert_eq!(queue.len(), 0);

        queue.insert(vec![TestOperation(clock.tick())]);
        assert_eq!(queue.len(), 1);
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestOperation(time::Lamport);

    impl Operation for TestOperation {
        fn timestamp(&self) -> time::Lamport {
            self.0
        }
    }
}
