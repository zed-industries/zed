use crate::{
    operations::{CreateBranch, CreateDocument, Edit},
    OperationCount, OperationId, ReplicaId, RepoId, Request, RevisionId, RoomCredentials,
};
use collections::BTreeMap;
use lazy_static::__Deref;
use serde::{Deserialize, Serialize};
use std::{any::Any, sync::Arc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RequestEnvelope {
    PublishRepo(PublishRepo),
    CloneRepo(CloneRepo),
    SyncRepo(SyncRepo),
    PublishOperations(PublishOperations),
}

impl RequestEnvelope {
    pub fn unwrap(self) -> Box<dyn Any> {
        match self {
            RequestEnvelope::PublishRepo(request) => Box::new(request),
            RequestEnvelope::CloneRepo(request) => Box::new(request),
            RequestEnvelope::SyncRepo(request) => Box::new(request),
            RequestEnvelope::PublishOperations(request) => Box::new(request),
        }
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
pub struct CloneRepo {
    pub name: Arc<str>,
}

impl Request for CloneRepo {
    type Response = CloneRepoResponse;
}

impl Into<RequestEnvelope> for CloneRepo {
    fn into(self) -> RequestEnvelope {
        RequestEnvelope::CloneRepo(self)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CloneRepoResponse {
    pub repo_id: RepoId,
    pub replica_id: ReplicaId,
    pub credentials: RoomCredentials,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncRepo {
    pub id: RepoId,
    pub max_operation_ids: BTreeMap<ReplicaId, OperationCount>,
}

impl Request for SyncRepo {
    type Response = SyncRepoResponse;
}

impl Into<RequestEnvelope> for SyncRepo {
    fn into(self) -> RequestEnvelope {
        RequestEnvelope::SyncRepo(self)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SyncRepoResponse {
    pub operations: Vec<Operation>,
    pub max_operation_ids: BTreeMap<ReplicaId, OperationCount>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublishOperations {
    pub repo_id: RepoId,
    pub operations: Vec<Operation>,
}

impl Request for PublishOperations {
    type Response = ();
}

impl Into<RequestEnvelope> for PublishOperations {
    fn into(self) -> RequestEnvelope {
        RequestEnvelope::PublishOperations(self)
    }
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
