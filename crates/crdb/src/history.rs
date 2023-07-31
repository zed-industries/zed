use std::{cmp::Ordering, iter, mem, ops::RangeBounds};

use crate::{
    btree::{self, Bias, KvStore, SavedId},
    messages::Operation,
    OperationCount, OperationId, ReplicaId, RevisionId,
};
use anyhow::{anyhow, Result};
use collections::{BTreeSet, Bound, HashMap, HashSet, VecDeque};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Serialize, Deserialize)]
pub struct SavedHistory {
    operations: SavedId,
    next_operation_id: OperationId,
    max_operation_ids: SavedId,
    deferred_operations: SavedId,
}

#[derive(Clone, Debug)]
pub struct History {
    operations: btree::Map<OperationId, Operation>,
    next_operation_id: OperationId,
    max_operation_ids: btree::Map<ReplicaId, OperationCount>,
    deferred_operations: btree::Sequence<DeferredOperation>,
}

impl History {
    pub fn new(replica_id: ReplicaId) -> Self {
        Self {
            operations: Default::default(),
            next_operation_id: OperationId::new(replica_id),
            max_operation_ids: Default::default(),
            deferred_operations: Default::default(),
        }
    }

    pub fn ptr_eq(&self, other: &Self) -> bool {
        btree::Map::ptr_eq(&self.operations, &other.operations)
            && btree::Map::ptr_eq(&self.max_operation_ids, &other.max_operation_ids)
            && btree::Sequence::ptr_eq(&self.deferred_operations, &other.deferred_operations)
            && self.next_operation_id == other.next_operation_id
    }

    pub async fn load(saved_history: SavedHistory, kv: &dyn KvStore) -> Result<Self> {
        Ok(Self {
            operations: btree::Map::load_root(saved_history.operations, kv).await?,
            next_operation_id: saved_history.next_operation_id,
            max_operation_ids: btree::Map::load_all(saved_history.max_operation_ids, kv).await?,
            deferred_operations: btree::Sequence::load_root(saved_history.deferred_operations, kv)
                .await?,
        })
    }

    pub async fn save(&self, kv: &dyn KvStore) -> Result<SavedHistory> {
        Ok(SavedHistory {
            operations: self.operations.save(kv).await?,
            next_operation_id: self.next_operation_id,
            max_operation_ids: self.max_operation_ids.save(kv).await?,
            deferred_operations: self.deferred_operations.save(kv).await?,
        })
    }

    pub fn next_operation_id(&mut self) -> OperationId {
        self.next_operation_id.tick()
    }

    pub fn max_operation_ids(&self) -> &btree::Map<ReplicaId, OperationCount> {
        &self.max_operation_ids
    }

    pub async fn insert(
        &mut self,
        operation: Operation,
        kv: &dyn KvStore,
    ) -> Result<SmallVec<[Operation; 1]>> {
        let op_id = operation.id();
        self.next_operation_id.observe(op_id);
        if self
            .max_operation_ids
            .load(&op_id.replica_id, kv)
            .await?
            .copied()
            < Some(op_id.operation_count)
        {
            self.max_operation_ids
                .insert(op_id.replica_id, op_id.operation_count);
        }
        self.operations.store(op_id, operation, kv).await?;

        self.deferred_operations
            .load(kv, &(), |probe| {
                let key_range = (
                    Bound::Excluded(*probe.start),
                    Bound::Included(*probe.summary),
                );
                key_range.contains(&op_id)
            })
            .await?;
        let mut cursor = self.deferred_operations.cursor::<OperationId>();
        let mut remaining = cursor.slice(&op_id, Bias::Left, &());
        let mut flushed = SmallVec::new();
        flushed.extend(
            cursor
                .slice(&op_id, Bias::Right, &())
                .iter()
                .map(|deferred| deferred.operation.clone()),
        );
        remaining.append(cursor.suffix(&()), &());
        drop(cursor);
        self.deferred_operations = remaining;
        Ok(flushed)
    }

    pub fn insert_local(&mut self, operation: Operation) {
        let id = operation.id();
        self.next_operation_id.observe(operation.id());
        self.max_operation_ids
            .insert(id.replica_id, id.operation_count);
        self.operations.insert(id, operation);
    }

    pub async fn defer(&mut self, operation: Operation, kv: &dyn KvStore) -> Result<()> {
        for parent in operation.parent().iter() {
            self.deferred_operations
                .load(kv, &(), |probe| {
                    let key_range = (
                        Bound::Excluded(*probe.start),
                        Bound::Included(*probe.summary),
                    );
                    key_range.contains(&operation.id())
                })
                .await?;
            self.deferred_operations.insert_or_replace(
                DeferredOperation {
                    parent: *parent,
                    operation: operation.clone(),
                },
                &(),
            );
        }
        Ok(())
    }

    pub async fn can_apply(&mut self, operation: &Operation, kv: &dyn KvStore) -> Result<bool> {
        for parent in operation.parent().iter() {
            if self.operations.load(parent, kv).await?.is_none() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub async fn has_applied(&mut self, operation: &Operation, kv: &dyn KvStore) -> Result<bool> {
        Ok(self.operations.load(&operation.id(), kv).await?.is_some())
    }

    pub async fn operation(
        &mut self,
        id: OperationId,
        kv: &dyn KvStore,
    ) -> Result<Option<&Operation>> {
        self.operations.load(&id, kv).await
    }

    pub async fn operations_since(
        &mut self,
        version: &btree::Map<ReplicaId, OperationCount>,
        kv: &dyn KvStore,
    ) -> Result<Vec<Operation>> {
        let mut new_operations = Vec::new();
        for (replica_id, end_op_count) in self.max_operation_ids.iter() {
            let start_op = OperationId {
                replica_id: *replica_id,
                operation_count: version
                    .get(&replica_id)
                    .map(|count| OperationCount(count.0 + 1))
                    .unwrap_or_default(),
            };
            let end_op = OperationId {
                replica_id: *replica_id,
                operation_count: *end_op_count,
            };

            new_operations.extend(
                self.operations
                    .load_from(&start_op, kv)
                    .await?
                    .take_while(|(op_id, _)| **op_id <= end_op)
                    .map(|(_, op)| op.clone()),
            );
        }
        Ok(new_operations)
    }

    pub async fn traverse(
        &mut self,
        revision_id: &RevisionId,
        kv: &dyn KvStore,
    ) -> Result<Traversal> {
        let mut frontier = VecDeque::new();
        let mut traversed = HashMap::default();
        for operation_id in revision_id.iter() {
            let parent_revision = self
                .operation(*operation_id, kv)
                .await?
                .ok_or_else(|| anyhow!("operation {:?} not found", operation_id))?
                .parent()
                .clone();
            traversed
                .entry(parent_revision.clone())
                .or_insert(HashSet::default())
                .insert((revision_id.clone(), *operation_id));
            frontier.push_back(Frontier {
                source: *operation_id,
                revision: parent_revision,
            });
        }

        Ok(Traversal {
            history: self,
            frontier,
            traversed,
            ancestors: Default::default(),
            reachable_len: revision_id.len(),
        })
    }
}

struct Frontier {
    source: OperationId,
    revision: RevisionId,
}

pub struct Traversal<'a> {
    pub history: &'a mut History,
    frontier: VecDeque<Frontier>,
    traversed: HashMap<RevisionId, HashSet<(RevisionId, OperationId)>>,
    ancestors: HashMap<RevisionId, HashSet<OperationId>>,
    reachable_len: usize,
}

impl Traversal<'_> {
    pub async fn next(&mut self, kv: &dyn KvStore) -> Result<Option<TraversalPath>> {
        while let Some(frontier) = self.frontier.pop_front() {
            let reachable_from = self.ancestors.entry(frontier.revision.clone()).or_default();
            reachable_from.insert(frontier.source);

            if reachable_from.len() == self.reachable_len {
                self.reachable_len = frontier.revision.len();
                self.frontier.clear();
                self.ancestors.clear();
                let traversed = mem::take(&mut self.traversed);
                for operation_id in frontier.revision.iter() {
                    let parent_revision = self
                        .history
                        .operation(*operation_id, kv)
                        .await?
                        .expect("operation must exist")
                        .parent()
                        .clone();
                    self.traversed
                        .entry(parent_revision.clone())
                        .or_default()
                        .insert((frontier.revision.clone(), *operation_id));
                    self.frontier.push_back(Frontier {
                        source: *operation_id,
                        revision: parent_revision,
                    });
                }

                return Ok(Some(TraversalPath {
                    revision_id: frontier.revision,
                    traversed,
                }));
            } else {
                for operation_id in frontier.revision.iter() {
                    let parent_revision = self
                        .history
                        .operation(*operation_id, kv)
                        .await?
                        .expect("operation must exist")
                        .parent()
                        .clone();
                    self.traversed
                        .entry(parent_revision.clone())
                        .or_default()
                        .insert((frontier.revision.clone(), *operation_id));

                    self.frontier.push_back(Frontier {
                        source: frontier.source,
                        revision: parent_revision,
                    });
                }
            }
        }

        Ok(None)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct TraversalPath {
    pub revision_id: RevisionId,
    pub traversed: HashMap<RevisionId, HashSet<(RevisionId, OperationId)>>,
}

impl TraversalPath {
    pub fn replay(mut self) -> impl Iterator<Item = TraversalPathOperation> {
        let mut stack = VecDeque::new();
        if let Some(children) = self.traversed.remove(&self.revision_id) {
            for (child_revision_id, operation_id) in children {
                stack.push_back(TraversalPathOperation {
                    parent_revision_id: self.revision_id.clone(),
                    target_revision_id: child_revision_id,
                    operation_id,
                });
            }
        }

        iter::from_fn(move || {
            while let Some(entry) = stack.pop_front() {
                if let Some(children) = self.traversed.remove(&entry.target_revision_id) {
                    for (child_revision, operation_id) in children {
                        stack.push_back(TraversalPathOperation {
                            parent_revision_id: entry.target_revision_id.clone(),
                            target_revision_id: child_revision,
                            operation_id,
                        });
                    }
                }

                return Some(entry);
            }

            None
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraversalPathOperation {
    pub parent_revision_id: RevisionId,
    pub target_revision_id: RevisionId,
    pub operation_id: OperationId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DeferredOperation {
    parent: OperationId,
    operation: Operation,
}

impl PartialEq for DeferredOperation {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.operation.id() == other.operation.id()
    }
}

impl Eq for DeferredOperation {}

impl PartialOrd for DeferredOperation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DeferredOperation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.parent
            .cmp(&other.parent)
            .then_with(|| self.operation.id().cmp(&other.operation.id()))
    }
}

impl btree::Item for DeferredOperation {
    type Summary = OperationId;

    fn summary(&self) -> Self::Summary {
        self.parent
    }
}

impl btree::KeyedItem for DeferredOperation {
    type Key = (OperationId, OperationId);

    fn key(&self) -> Self::Key {
        (self.parent, self.operation.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btree::tests::InMemoryKv;

    #[gpui::test]
    async fn test_traversal() {
        let kv = InMemoryKv::default();
        let mut history = History::new(ReplicaId(0));
        let op1 = insert_operation(&[], &mut history, &kv).await;
        let op2 = insert_operation(&[op1.id()], &mut history, &kv).await;
        let op3 = insert_operation(&[op1.id()], &mut history, &kv).await;
        let op4 = insert_operation(&[op2.id(), op3.id()], &mut history, &kv).await;
        let op5 = insert_operation(&[op4.id()], &mut history, &kv).await;
        let op6 = insert_operation(&[op4.id()], &mut history, &kv).await;
        let op7 = insert_operation(&[op2.id()], &mut history, &kv).await;
        let op8 = insert_operation(&[op5.id()], &mut history, &kv).await;
        let op9 = insert_operation(&[op5.id()], &mut history, &kv).await;
        let op10 = insert_operation(&[op8.id()], &mut history, &kv).await;
        let op11 = insert_operation(&[op9.id(), op10.id()], &mut history, &kv).await;

        assert_eq!(
            traversal(&[op4.id()], &mut history, &kv).await,
            &[
                (
                    RevisionId::from([op2.id(), op3.id()].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                        target_revision_id: RevisionId::from([op4.id()].as_slice()),
                        operation_id: op4.id(),
                    }]
                ),
                (
                    RevisionId::from([op1.id()].as_slice()),
                    vec![
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op2.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op3.id(),
                        }
                    ]
                ),
                (
                    RevisionId::from([].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([].as_slice()),
                        target_revision_id: RevisionId::from([op1.id()].as_slice()),
                        operation_id: op1.id(),
                    }]
                ),
            ]
        );
        assert_eq!(
            traversal(&[op6.id()], &mut history, &kv).await,
            &[
                (
                    RevisionId::from([op4.id()].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([op4.id()].as_slice()),
                        target_revision_id: RevisionId::from([op6.id()].as_slice()),
                        operation_id: op6.id(),
                    }]
                ),
                (
                    RevisionId::from([op2.id(), op3.id()].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                        target_revision_id: RevisionId::from([op4.id()].as_slice()),
                        operation_id: op4.id(),
                    }]
                ),
                (
                    RevisionId::from([op1.id()].as_slice()),
                    vec![
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op2.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op3.id(),
                        }
                    ]
                ),
                (
                    RevisionId::from([].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([].as_slice()),
                        target_revision_id: RevisionId::from([op1.id()].as_slice()),
                        operation_id: op1.id(),
                    }]
                ),
            ]
        );
        assert_eq!(
            traversal(&[op5.id(), op6.id()], &mut history, &kv).await,
            &[
                (
                    RevisionId::from([op4.id()].as_slice()),
                    vec![
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op4.id()].as_slice()),
                            target_revision_id: RevisionId::from([op5.id(), op6.id()].as_slice()),
                            operation_id: op6.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op4.id()].as_slice()),
                            target_revision_id: RevisionId::from([op5.id(), op6.id()].as_slice()),
                            operation_id: op5.id(),
                        }
                    ]
                ),
                (
                    RevisionId::from([op2.id(), op3.id()].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                        target_revision_id: RevisionId::from([op4.id()].as_slice()),
                        operation_id: op4.id(),
                    }]
                ),
                (
                    RevisionId::from([op1.id()].as_slice()),
                    vec![
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op2.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op3.id(),
                        }
                    ]
                ),
                (
                    RevisionId::from([].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([].as_slice()),
                        target_revision_id: RevisionId::from([op1.id()].as_slice()),
                        operation_id: op1.id(),
                    }]
                ),
            ]
        );
        assert_eq!(
            traversal(&[op4.id(), op7.id()], &mut history, &kv).await,
            &[
                (
                    RevisionId::from([op1.id()].as_slice()),
                    vec![
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op2.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id()].as_slice()),
                            operation_id: op2.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op3.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            target_revision_id: RevisionId::from([op4.id(), op7.id()].as_slice()),
                            operation_id: op4.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op2.id()].as_slice()),
                            target_revision_id: RevisionId::from([op4.id(), op7.id()].as_slice()),
                            operation_id: op7.id(),
                        }
                    ]
                ),
                (
                    RevisionId::from([].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([].as_slice()),
                        target_revision_id: RevisionId::from([op1.id()].as_slice()),
                        operation_id: op1.id(),
                    }]
                ),
            ]
        );
        assert_eq!(
            traversal(&[op11.id()], &mut history, &kv).await,
            &[
                (
                    RevisionId::from([op9.id(), op10.id()].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([op9.id(), op10.id()].as_slice()),
                        target_revision_id: RevisionId::from([op11.id()].as_slice()),
                        operation_id: op11.id(),
                    }]
                ),
                (
                    RevisionId::from([op5.id()].as_slice()),
                    vec![
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op5.id()].as_slice()),
                            target_revision_id: RevisionId::from([op9.id(), op10.id()].as_slice()),
                            operation_id: op9.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op5.id()].as_slice()),
                            target_revision_id: RevisionId::from([op8.id()].as_slice()),
                            operation_id: op8.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op8.id()].as_slice()),
                            target_revision_id: RevisionId::from([op9.id(), op10.id()].as_slice()),
                            operation_id: op10.id(),
                        }
                    ]
                ),
                (
                    RevisionId::from([op4.id()].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([op4.id()].as_slice()),
                        target_revision_id: RevisionId::from([op5.id()].as_slice()),
                        operation_id: op5.id(),
                    }]
                ),
                (
                    RevisionId::from([op2.id(), op3.id()].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                        target_revision_id: RevisionId::from([op4.id()].as_slice()),
                        operation_id: op4.id(),
                    }]
                ),
                (
                    RevisionId::from([op1.id()].as_slice()),
                    vec![
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op2.id(),
                        },
                        TraversalPathOperation {
                            parent_revision_id: RevisionId::from([op1.id()].as_slice()),
                            target_revision_id: RevisionId::from([op2.id(), op3.id()].as_slice()),
                            operation_id: op3.id(),
                        }
                    ]
                ),
                (
                    RevisionId::from([].as_slice()),
                    vec![TraversalPathOperation {
                        parent_revision_id: RevisionId::from([].as_slice()),
                        target_revision_id: RevisionId::from([op1.id()].as_slice()),
                        operation_id: op1.id(),
                    }]
                ),
            ]
        );
    }

    async fn insert_operation(
        parent: &[OperationId],
        history: &mut History,
        kv: &dyn KvStore,
    ) -> Operation {
        let operation = Operation::CreateBranch(crate::operations::CreateBranch {
            id: history.next_operation_id(),
            parent: parent.into(),
            name: "1".into(),
        });
        history.insert(operation.clone(), kv).await.unwrap();
        operation
    }

    async fn traversal(
        revision_id: &[OperationId],
        history: &mut History,
        kv: &dyn KvStore,
    ) -> Vec<(RevisionId, Vec<TraversalPathOperation>)> {
        let mut traversal = history.traverse(&revision_id.into(), kv).await.unwrap();
        let mut results = Vec::new();
        while let Some(result) = traversal.next(kv).await.unwrap() {
            results.push((result.revision_id.clone(), result.replay().collect()));
        }
        results
    }
}
