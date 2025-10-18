use clock::Lamport;
use std::{fmt::Debug, ops::Add};
use sum_tree::{ContextLessSummary, Dimension, Edit, Item, KeyedItem, SumTree};

pub trait Operation: Clone + Debug {
    fn lamport_timestamp(&self) -> clock::Lamport;
}

#[derive(Clone, Debug)]
struct OperationItem<T>(T);

#[derive(Clone, Debug)]
pub struct OperationQueue<T: Operation>(SumTree<OperationItem<T>>);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OperationKey(clock::Lamport);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OperationSummary {
    pub key: OperationKey,
    pub len: usize,
}

impl OperationKey {
    pub fn new(timestamp: clock::Lamport) -> Self {
        Self(timestamp)
    }
}

impl<T: Operation> Default for OperationQueue<T> {
    fn default() -> Self {
        OperationQueue::new()
    }
}

impl<T: Operation> OperationQueue<T> {
    pub fn new() -> Self {
        OperationQueue(SumTree::default())
    }

    pub fn len(&self) -> usize {
        self.0.summary().len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, mut ops: Vec<T>) {
        ops.sort_by_key(|op| op.lamport_timestamp());
        ops.dedup_by_key(|op| op.lamport_timestamp());
        self.0.edit(
            ops.into_iter()
                .map(|op| Edit::Insert(OperationItem(op)))
                .collect(),
            (),
        );
    }

    pub fn drain(&mut self) -> Self {
        let clone = self.clone();
        self.0 = SumTree::default();
        clone
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|i| &i.0)
    }
}

impl ContextLessSummary for OperationSummary {
    fn zero() -> Self {
        OperationSummary {
            key: OperationKey::new(Lamport::MIN),
            len: 0,
        }
    }

    fn add_summary(&mut self, other: &Self) {
        assert!(self.key < other.key);
        self.key = other.key;
        self.len += other.len;
    }
}

impl Add<&Self> for OperationSummary {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        assert!(self.key < other.key);
        OperationSummary {
            key: other.key,
            len: self.len + other.len,
        }
    }
}

impl Dimension<'_, OperationSummary> for OperationKey {
    fn zero(_cx: ()) -> Self {
        OperationKey::new(Lamport::MIN)
    }

    fn add_summary(&mut self, summary: &OperationSummary, _: ()) {
        assert!(*self <= summary.key);
        *self = summary.key;
    }
}

impl<T: Operation> Item for OperationItem<T> {
    type Summary = OperationSummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
        OperationSummary {
            key: OperationKey::new(self.0.lamport_timestamp()),
            len: 1,
        }
    }
}

impl<T: Operation> KeyedItem for OperationItem<T> {
    type Key = OperationKey;

    fn key(&self) -> Self::Key {
        OperationKey::new(self.0.lamport_timestamp())
    }
}

#[cfg(test)]
mod tests {
    use clock::ReplicaId;

    use super::*;

    #[test]
    fn test_len() {
        let mut clock = clock::Lamport::new(ReplicaId::LOCAL);

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
    struct TestOperation(clock::Lamport);

    impl Operation for TestOperation {
        fn lamport_timestamp(&self) -> clock::Lamport {
            self.0
        }
    }
}
