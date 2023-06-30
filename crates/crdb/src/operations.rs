use crate::{AnchorRange, Message, OperationId, RevisionId};
use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Clone, Debug)]
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

impl Message for Operation {
    fn to_bytes(&self) -> Vec<u8> {
        serde_
    }
}

#[derive(Clone, Debug)]
pub struct CreateBranch {
    pub id: OperationId,
    pub parent: RevisionId,
    pub name: Arc<str>,
}

#[derive(Clone, Debug)]
pub struct CreateDocument {
    pub id: OperationId,
    pub parent: SmallVec<[OperationId; 2]>,
}

#[derive(Clone, Debug)]
pub struct Edit {
    pub id: OperationId,
    pub parent: SmallVec<[OperationId; 2]>,
    pub edits: SmallVec<[(AnchorRange, Arc<str>); 2]>,
}
