use std::cmp::{self, Ordering};

use crate::{BranchId, LamportTime, OperationId, OrderedMap};
use smallvec::SmallVec;

#[derive(Clone, Default)]
pub struct Version {
    final_operations: SmallVec<[OperationId; 2]>,
    operations: OrderedMap<BranchId, LamportTime>,
}

impl Version {
    /// Build a new version that is >= both of the given versions.
    pub fn join(a: &Self, b: &Self) -> Self {
        // Take the union of the operations maps
        let operations = a.operations.union(&b.operations, cmp::max);

        todo!()
    }

    /// Build a new version that is <= both of the given versions.
    pub fn meet(a: &Self, b: &Self) -> Self {
        todo!()
    }

    /// Add an operation to the set of operations represented by this version.
    pub fn insert(&mut self, operation: OperationId) {
        if self
            .operations
            .get(&operation.branch)
            .map_or(true, |time| *time < operation.time)
        {
            self.operations.insert(operation.branch, operation.time)
        }
    }

    pub fn contains(&self, operation: &OperationId) -> bool {
        self.operations
            .get(&operation.branch)
            .map_or(false, |time| *time >= operation.time)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.final_operations == other.final_operations
    }
}

impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.final_operations == other.final_operations {
            Some(Ordering::Equal)
        } else if self
            .final_operations
            .iter()
            .all(|op_id| other.contains(op_id))
        {
            Some(Ordering::Less)
        } else if other
            .final_operations
            .iter()
            .all(|op_id| self.contains(op_id))
        {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}
