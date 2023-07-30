use crate::{
    btree::{self, KvStore},
    messages::Operation,
    OperationCount, OperationId, ReplicaId, RevisionId,
};
use anyhow::{anyhow, Result};
use collections::{BTreeSet, HashMap, HashSet, VecDeque};

struct History {
    operations: btree::Map<OperationId, Operation>,
    next_operation_id: OperationId,
}

impl History {
    fn new(replica_id: ReplicaId) -> Self {
        Self {
            operations: Default::default(),
            next_operation_id: OperationId::new(replica_id),
        }
    }

    fn next_operation_id(&mut self) -> OperationId {
        let next_operation_id = self.next_operation_id.tick();
        self.next_operation_id = next_operation_id;
        next_operation_id
    }

    async fn insert(&mut self, operation: Operation, kv: &dyn KvStore) -> Result<()> {
        self.next_operation_id.observe(operation.id());
        self.operations.store(operation.id(), operation, kv).await?;
        Ok(())
    }

    async fn operation(&mut self, id: OperationId, kv: &dyn KvStore) -> Result<Option<&Operation>> {
        self.operations.load(&id, kv).await
    }

    async fn traverse(&mut self, revision_id: &RevisionId, kv: &dyn KvStore) -> Result<Traversal> {
        let mut frontier = VecDeque::new();
        let mut traversed = BTreeSet::new();
        for operation_id in revision_id.iter() {
            traversed.insert((operation_id.operation_count, operation_id.replica_id));
            frontier.push_back(Frontier {
                source: *operation_id,
                revision: self
                    .operation(*operation_id, kv)
                    .await?
                    .ok_or_else(|| anyhow!("operation {:?} not found", operation_id))?
                    .parent()
                    .clone(),
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

struct Traversal<'a> {
    history: &'a mut History,
    frontier: VecDeque<Frontier>,
    traversed: BTreeSet<(OperationCount, ReplicaId)>,
    ancestors: HashMap<RevisionId, HashSet<OperationId>>,
    reachable_len: usize,
}

impl Traversal<'_> {
    async fn next(&mut self, kv: &dyn KvStore) -> Result<Option<TraversalResult>> {
        while let Some(frontier) = self.frontier.pop_front() {
            let reachable_from = self.ancestors.entry(frontier.revision.clone()).or_default();
            reachable_from.insert(frontier.source);
            if reachable_from.len() == self.reachable_len {
                let missing_operations_start = if let Some(max_op) = frontier
                    .revision
                    .iter()
                    .max_by_key(|op_id| op_id.operation_count)
                {
                    OperationCount(max_op.operation_count.0 + 1)
                } else {
                    OperationCount(0)
                };
                let operations = self
                    .traversed
                    .range(&(missing_operations_start, ReplicaId::default())..)
                    .map(|(operation_count, replica_id)| OperationId {
                        replica_id: *replica_id,
                        operation_count: *operation_count,
                    })
                    .collect();

                self.reachable_len = frontier.revision.len();
                self.frontier.clear();
                self.ancestors.clear();
                self.traversed.clear();
                for operation_id in frontier.revision.iter() {
                    self.traversed
                        .insert((operation_id.operation_count, operation_id.replica_id));
                    self.frontier.push_back(Frontier {
                        source: *operation_id,
                        revision: self
                            .history
                            .operation(*operation_id, kv)
                            .await?
                            .expect("operation must exist")
                            .parent()
                            .clone(),
                    });
                }

                return Ok(Some(TraversalResult {
                    revision: frontier.revision,
                    operations,
                }));
            } else {
                for operation_id in frontier.revision.iter() {
                    self.traversed
                        .insert((operation_id.operation_count, operation_id.replica_id));
                    self.frontier.push_back(Frontier {
                        source: frontier.source,
                        revision: self
                            .history
                            .operation(*operation_id, kv)
                            .await?
                            .expect("operation must exist")
                            .parent()
                            .clone(),
                    });
                }
            }
        }

        Ok(None)
    }
}

struct Frontier {
    source: OperationId,
    revision: RevisionId,
}

#[derive(Eq, PartialEq, Debug)]
struct TraversalResult {
    revision: RevisionId,
    operations: BTreeSet<OperationId>,
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

        assert_eq!(
            traversal(&[op4.id()], &mut history, &kv).await,
            &[
                TraversalResult {
                    revision: RevisionId::from([op2.id(), op3.id()].as_slice()),
                    operations: BTreeSet::from_iter([op4.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([op1.id()].as_slice()),
                    operations: BTreeSet::from_iter([op2.id(), op3.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([].as_slice()),
                    operations: BTreeSet::from_iter([op1.id()]),
                }
            ]
        );
        assert_eq!(
            traversal(&[op6.id()], &mut history, &kv).await,
            &[
                TraversalResult {
                    revision: RevisionId::from([op4.id()].as_slice()),
                    operations: BTreeSet::from_iter([op6.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([op2.id(), op3.id()].as_slice()),
                    operations: BTreeSet::from_iter([op4.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([op1.id()].as_slice()),
                    operations: BTreeSet::from_iter([op2.id(), op3.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([].as_slice()),
                    operations: BTreeSet::from_iter([op1.id()]),
                }
            ]
        );
        assert_eq!(
            traversal(&[op5.id(), op6.id()], &mut history, &kv).await,
            &[
                TraversalResult {
                    revision: RevisionId::from([op4.id()].as_slice()),
                    operations: BTreeSet::from_iter([op5.id(), op6.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([op2.id(), op3.id()].as_slice()),
                    operations: BTreeSet::from_iter([op4.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([op1.id()].as_slice()),
                    operations: BTreeSet::from_iter([op2.id(), op3.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([].as_slice()),
                    operations: BTreeSet::from_iter([op1.id()]),
                }
            ]
        );
        assert_eq!(
            traversal(&[op4.id(), op5.id()], &mut history, &kv).await,
            &[
                TraversalResult {
                    revision: RevisionId::from([op2.id(), op3.id()].as_slice()),
                    operations: BTreeSet::from_iter([op4.id(), op5.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([op1.id()].as_slice()),
                    operations: BTreeSet::from_iter([op2.id(), op3.id()]),
                },
                TraversalResult {
                    revision: RevisionId::from([].as_slice()),
                    operations: BTreeSet::from_iter([op1.id()]),
                }
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
    ) -> Vec<TraversalResult> {
        let mut traversal = history.traverse(&revision_id.into(), kv).await.unwrap();
        let mut results = Vec::new();
        while let Some(result) = traversal.next(kv).await.unwrap() {
            results.push(result);
        }
        results
    }
}
