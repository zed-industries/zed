use crate::{OrderedMap, OrderedSet};
use smallvec::SmallVec;
use std::{collections::hash_map::DefaultHasher, hash::Hasher, sync::Arc};

#[derive(Default, Eq, PartialEq)]
struct Version {
    parents: SmallVec<[Arc<Operation>; 2]>,
    operations: OrderedMap<BranchId, OperationCount>,
}

struct Operation {
    id: OperationId,
    version: Version,
}

#[derive(Clone, Default, Debug)]
struct OperationId {
    branch: BranchId,
    operation_count: OperationCount,
}

#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct BranchId {
    replica: ReplicaId,
    context: ContextId,
}

type ContextId = u32;
type ReplicaId = u32;
type OperationCount = u32;

impl Version {
    pub fn seed() -> Self {
        Self::default()
    }

    pub fn operation(&mut self, branch: BranchId) -> OperationId {
        let operation_count = if let Some(prev_count) = self.operations.get(&branch) {
            let operation_count = prev_count + 1;
            self.operations.insert(branch, operation_count);
            operation_count
        } else {
            self.operations.insert(branch, 1);
            1
        };

        OperationId {
            branch,
            operation_count,
        }
    }

    pub fn join(&mut self, other: &Self) {
        if self >= other {
        } else if other >= self {
            *self = other.clone();
        } else {
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

// impl Event {
//     pub fn root() -> Arc<Self> {
//         Arc::new(Self {
//             hash: 0,
//             roots: Default::default(),
//             events: OrderedSet::from_ordered_entries([0]),
//         })
//     }

//     pub fn new(left: &Arc<Event>, right: &Arc<Event>) -> Arc<Self> {
//         if left.events.contains(&right.hash) {
//             return left.clone();
//         } else if right.events.contains(&left.hash) {
//             return right.clone();
//         } else {
//             let mut roots = SmallVec::new();
//             roots.push(left.clone());
//             roots.push(right.clone());
//             let mut hasher = DefaultHasher::new();
//             hasher.write_u64(left.hash);
//             hasher.write_u64(right.hash);
//             let hash = hasher.finish();
//             let mut events = left.events.clone();
//             // TODO: make this more efficient.
//             for event in right.events.iter() {
//                 events.insert(*event);
//             }
//             events.insert(hash);
//             Arc::new(Self {
//                 hash,
//                 roots,
//                 events,
//             })
//         }
//     }

//     pub fn version(&self) -> Version {
//         Version {
//             roots: vec![self.clone()],
//         }
//     }
// }

// #[derive(Clone, Eq, PartialEq)]
// struct Version {
//     roots: Vec<Arc<Event>>,
// }

// impl Version {
//     pub fn greatest_lower_bound(left: &Self, right: &Self) -> Version {
//         let mut left = left.clone();
//         left.roots.retain(|left_root| {
//             right
//                 .roots
//                 .iter()
//                 .any(|right_root| !left_root.events.contains(&right_root.hash))
//         });

//         // if left
//         //     .roots
//         //     .iter()
//         //     .all(|root| right.events.contains(&root.hash))
//         // {
//         //     return vec![left.clone()];
//         // } else if right
//         //     .roots
//         //     .iter()
//         //     .all(|root| right.events.contains(&root.hash))
//         // {
//         //     return vec![right.clone()];
//         // } else {
//         //     vec![left.clone(), right.clone()]
//         // }
//     }
// }
