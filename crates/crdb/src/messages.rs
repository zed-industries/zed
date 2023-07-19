use crate::{
    operations::{CreateBranch, CreateDocument, Edit},
    OperationId, RepoId, Request, RevisionId, RoomCredentials,
};
use serde::{Deserialize, Serialize};
use std::{any::Any, sync::Arc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RequestEnvelope {
    PublishRepo(PublishRepo),
}

impl RequestEnvelope {
    pub fn unwrap(self) -> Box<dyn Any> {
        Box::new(match self {
            RequestEnvelope::PublishRepo(request) => request,
        })
    }
}

impl From<Operation> for MessageEnvelope {
    fn from(value: Operation) -> Self {
        Self::Operation(value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublishRepo {
    pub id: RepoId,
    pub name: Arc<str>,
}

impl Request for PublishRepo {
    type Response = PublishRepoResponse;
}

impl Into<RequestEnvelope> for PublishRepo {
    fn into(self) -> RequestEnvelope {
        RequestEnvelope::PublishRepo(self)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublishRepoResponse {
    pub credentials: RoomCredentials,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageEnvelope {
    Operation(Operation),
}

impl MessageEnvelope {
    pub fn unwrap(self) -> Box<dyn Any> {
        Box::new(match self {
            MessageEnvelope::Operation(message) => message,
        })
    }
}

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
