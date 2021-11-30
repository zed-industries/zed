use super::Operation;
use std::{fmt::Debug, ops::Add};
use sum_tree::{Cursor, Dimension, Edit, Item, KeyedItem, SumTree, Summary};

#[derive(Clone, Debug)]
pub struct OperationQueue(SumTree<Operation>);

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct OperationKey(clock::Lamport);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OperationSummary {
    pub key: OperationKey,
    pub len: usize,
}

impl OperationKey {
    pub fn new(timestamp: clock::Lamport) -> Self {
        Self(timestamp)
    }
}

impl OperationQueue {
    pub fn new() -> Self {
        OperationQueue(SumTree::new())
    }

    pub fn len(&self) -> usize {
        self.0.summary().len
    }

    pub fn insert(&mut self, mut ops: Vec<Operation>) {
        ops.sort_by_key(|op| op.lamport_timestamp());
        ops.dedup_by_key(|op| op.lamport_timestamp());
        self.0
            .edit(ops.into_iter().map(Edit::Insert).collect(), &());
    }

    pub fn drain(&mut self) -> Self {
        let clone = self.clone();
        self.0 = SumTree::new();
        clone
    }

    pub fn cursor(&self) -> Cursor<Operation, ()> {
        self.0.cursor()
    }
}

impl Summary for OperationSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &()) {
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
    fn add_summary(&mut self, summary: &OperationSummary, _: &()) {
        assert!(*self <= summary.key);
        *self = summary.key;
    }
}

impl Item for Operation {
    type Summary = OperationSummary;

    fn summary(&self) -> Self::Summary {
        OperationSummary {
            key: OperationKey::new(self.lamport_timestamp()),
            len: 1,
        }
    }
}

impl KeyedItem for Operation {
    type Key = OperationKey;

    fn key(&self) -> Self::Key {
        OperationKey::new(self.lamport_timestamp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let mut clock = clock::Lamport::new(0);

        let mut queue = OperationQueue::new();
        assert_eq!(queue.len(), 0);

        queue.insert(vec![
            Operation::Test(clock.tick()),
            Operation::Test(clock.tick()),
        ]);
        assert_eq!(queue.len(), 2);

        queue.insert(vec![Operation::Test(clock.tick())]);
        assert_eq!(queue.len(), 3);

        drop(queue.drain());
        assert_eq!(queue.len(), 0);

        queue.insert(vec![Operation::Test(clock.tick())]);
        assert_eq!(queue.len(), 1);
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestOperation(clock::Lamport);
}
