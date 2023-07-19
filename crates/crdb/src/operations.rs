use crate::{AnchorRange, OperationId, RevisionId};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateBranch {
    pub id: OperationId,
    pub parent: RevisionId,
    pub name: Arc<str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDocument {
    pub id: OperationId,
    pub parent: RevisionId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edit {
    pub id: OperationId,
    pub parent: RevisionId,
    pub edits: SmallVec<[(AnchorRange, Arc<str>); 2]>,
}
