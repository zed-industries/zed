mod dense_id;

use dense_id::DenseId;
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::{ops::Range, path::Path, sync::Arc};
use sum_tree::{Bias, SumTree, TreeMap};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RepoId(Uuid);

impl RepoId {
    fn new() -> Self {
        RepoId(Uuid::new_v4())
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct ReplicaId(u32);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct OperationCount(u64);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct OperationId {
    replica_id: ReplicaId,
    operation_count: OperationCount,
}

impl OperationId {
    fn tick(&mut self) -> OperationId {
        self.operation_count.0 += 1;
        *self
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct DocumentId(OperationId);

#[derive(Clone, Default)]
pub struct Db {
    snapshot: Arc<Mutex<DbSnapshot>>,
}

impl Db {
    pub fn create_repo(&self) -> Repo {
        let id = RepoId::new();
        let snapshot = RepoSnapshot::default();
        let repo = Repo {
            id,
            db: self.clone(),
        };
        self.snapshot.lock().repos.insert(id, snapshot);
        repo
    }

    /// Panics if the repo does not exist
    fn update_repo<F, T>(&self, id: RepoId, f: F) -> T
    where
        F: FnOnce(&mut RepoSnapshot) -> T,
    {
        self.snapshot
            .lock()
            .repos
            .update(&id, f)
            .expect("repo must exist")
    }
}

#[derive(Clone, Default)]
struct DbSnapshot {
    repos: TreeMap<RepoId, RepoSnapshot>,
}

pub struct Repo {
    id: RepoId,
    db: Db,
}

impl Repo {
    fn create_document(&self) -> Document {
        self.db.update_repo(self.id, |repo| {
            let operation_id = repo.next_operation_id.tick();
            let document_id = DocumentId(operation_id);

            let mut cursor = repo.document_fragments.cursor::<DocumentId>();
            let mut new_document_fragments = cursor.slice(&document_id, Bias::Right, &());
            new_document_fragments.push(
                DocumentFragment {
                    document_id,
                    location: DenseId::min(),
                    insertion_id: operation_id,
                    insertion_subrange: 0..0,
                    visible: true,
                    tombstones: Default::default(),
                    undo_count: 0,
                },
                &(),
            );
            new_document_fragments.append(cursor.suffix(&()), &());
            drop(cursor);

            repo.document_fragments = new_document_fragments;
            repo.document_metadata.insert(
                document_id,
                DocumentMetadata {
                    path: None,
                    last_change: operation_id,
                },
            );

            Document { id: document_id }
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct RepoSnapshot {
    head: OperationId,
    next_operation_id: OperationId,
    document_metadata: TreeMap<DocumentId, DocumentMetadata>,
    document_fragments: SumTree<DocumentFragment>,
    operations: SumTree<Operation>,
}

#[derive(Clone, Debug)]
struct DocumentMetadata {
    path: Option<Arc<Path>>,
    last_change: OperationId,
}

#[derive(Clone, Debug)]
struct DocumentFragment {
    document_id: DocumentId,
    location: DenseId,
    insertion_id: OperationId,
    insertion_subrange: Range<u64>,
    visible: bool,
    tombstones: SmallVec<[Tombstone; 2]>,
    undo_count: u16,
}

impl DocumentFragment {
    fn len(&self) -> u64 {
        self.insertion_subrange.end - self.insertion_subrange.start
    }
}

impl sum_tree::Item for DocumentFragment {
    type Summary = DocumentFragmentSummary;

    fn summary(&self) -> DocumentFragmentSummary {
        DocumentFragmentSummary {
            visible_len: if self.visible { self.len() } else { 0 },
            hidden_len: if self.visible { 0 } else { self.len() },
            max_document_id: self.document_id,
            max_location: self.location.clone(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct DocumentFragmentSummary {
    visible_len: u64,
    hidden_len: u64,
    max_document_id: DocumentId,
    max_location: DenseId,
}

impl sum_tree::Summary for DocumentFragmentSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.visible_len += summary.visible_len;
        self.hidden_len += summary.hidden_len;

        debug_assert!(summary.max_document_id > self.max_document_id);
        self.max_document_id = summary.max_document_id;

        debug_assert!(summary.max_location > self.max_location);
        self.max_location = summary.max_location.clone();
    }
}

impl<'a> sum_tree::Dimension<'a, DocumentFragmentSummary> for DocumentId {
    fn add_summary(&mut self, summary: &'a DocumentFragmentSummary, _: &()) {
        *self = summary.max_document_id
    }
}

#[derive(Clone, Debug)]
struct Tombstone {
    id: OperationId,
    undo_count: u16,
}

struct Document {
    id: DocumentId,
}

#[derive(Clone, Debug)]
struct Operation {
    id: OperationId,
    parent_ids: SmallVec<[OperationId; 2]>,
    kind: OperationKind,
}

#[derive(Clone, Debug)]
enum OperationKind {
    CreateDocument,
}

impl sum_tree::Item for Operation {
    type Summary = OperationId;

    fn summary(&self) -> Self::Summary {
        self.id
    }
}

impl sum_tree::KeyedItem for Operation {
    type Key = OperationId;

    fn key(&self) -> Self::Key {
        self.id
    }
}

impl sum_tree::Summary for OperationId {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        debug_assert!(summary > self);
        *self = *summary;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo() {
        let mut db = Db::default();
        let repo = db.create_repo();
        let doc = repo.create_document();
    }
}
