use crate::{
    dense_id::DenseId, AnchorRange, BranchSnapshot, DocumentFragment, DocumentMetadata,
    OperationId, RepoSnapshot, Revision, RevisionId,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::sync::Arc;
use sum_tree::Bias;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateBranch {
    pub id: OperationId,
    pub parent: RevisionId,
    pub name: Arc<str>,
}

impl CreateBranch {
    pub fn apply(self, repo: &mut RepoSnapshot) -> Result<()> {
        let revision = repo
            .revisions
            .get(&self.parent)
            .ok_or_else(|| anyhow!("parent revision not found"))?
            .clone();
        repo.branches.insert(
            self.id,
            BranchSnapshot {
                name: self.name,
                head: smallvec![self.id],
            },
        );
        repo.revisions.insert(smallvec![self.id], revision);
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDocument {
    pub id: OperationId,
    pub branch_id: OperationId,
    pub parent: RevisionId,
}

impl CreateDocument {
    pub fn apply(self, revision: &mut Revision) {
        let mut cursor = revision.document_fragments.cursor::<OperationId>();
        let mut new_document_fragments = cursor.slice(&self.id, Bias::Right, &());
        new_document_fragments.push(
            DocumentFragment {
                document_id: self.id,
                location: DenseId::min(),
                insertion_id: self.id,
                insertion_subrange: 0..0,
                tombstones: Default::default(),
                undo_count: 0,
            },
            &(),
        );
        new_document_fragments.append(cursor.suffix(&()), &());
        drop(cursor);

        revision.document_fragments = new_document_fragments;
        revision.document_metadata.insert(
            self.id,
            DocumentMetadata {
                path: None,
                last_change: self.id,
            },
        );
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edit {
    pub id: OperationId,
    pub branch_id: OperationId,
    pub parent: RevisionId,
    pub edits: SmallVec<[(AnchorRange, Arc<str>); 2]>,
}

impl Edit {
    pub fn apply(self, parent_revision: &Revision, head_revision: &mut Revision) -> Result<()> {
        Err(anyhow!("not implemented"))
    }
}
