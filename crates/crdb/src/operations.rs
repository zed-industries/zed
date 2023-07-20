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
    pub fn apply(self, repo: &mut RepoSnapshot) -> Result<()> {
        let branch_id = self.branch_id;
        let branch = repo
            .branches
            .get(&self.branch_id)
            .ok_or_else(|| anyhow!("branch {:?} not found", self.branch_id))?;

        let mut revision = repo
            .revisions
            .get(&branch.head)
            .ok_or_else(|| {
                anyhow!(
                    "revision {:?} not found in branch {:?}",
                    branch.head,
                    self.branch_id
                )
            })?
            .clone();
        let new_head = if self.parent == branch.head {
            smallvec![self.id]
        } else {
            let mut head = branch.head.clone();
            head.push(self.id);
            head
        };

        revision.apply_create_document(self);
        repo.branches
            .update(&branch_id, |branch| branch.head = new_head.clone());
        repo.revisions.insert(new_head, revision);

        Ok(())
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
    pub fn apply(self, repo: &mut RepoSnapshot) -> Result<()> {
        Err(anyhow!("not implemented"))
    }
}
