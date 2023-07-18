use crate::{AnchorRange, OperationId, RevisionId};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Operation {
    CreateDocument(CreateDocument),
    Edit(Edit),
    CreateBranch(CreateBranch),
}

impl Operation {
    pub fn id(&self) -> OperationId {
        match self {
            Operation::CreateDocument(op) => op.id,
            Operation::Edit(op) => op.id,
            Operation::CreateBranch(op) => op.id,
        }
    }

    pub fn parent(&self) -> &RevisionId {
        match self {
            Operation::CreateDocument(op) => &op.parent,
            Operation::Edit(op) => &op.parent,
            Operation::CreateBranch(op) => &op.parent,
        }
    }
}

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
