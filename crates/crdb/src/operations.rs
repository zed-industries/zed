use crate::{AnchorRange, OperationId, RepoSnapshot, RevisionId};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateBranch {
    pub id: OperationId,
    pub parent: RevisionId,
    pub name: Arc<str>,
}

impl CreateBranch {
    pub fn apply(self, repo: &mut RepoSnapshot) -> Result<()> {
        todo!()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDocument {
    pub id: OperationId,
    pub parent: RevisionId,
}

impl CreateDocument {
    pub fn apply(self, repo: &mut RepoSnapshot) -> Result<()> {
        todo!()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edit {
    pub id: OperationId,
    pub parent: RevisionId,
    pub edits: SmallVec<[(AnchorRange, Arc<str>); 2]>,
}

impl Edit {
    pub fn apply(self, repo: &mut RepoSnapshot) -> Result<()> {
        todo!()
    }
}
