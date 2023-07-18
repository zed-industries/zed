use crate::{RepoId, Request, RoomCredentials};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Serialize, Deserialize)]
pub struct PublishRepo {
    pub id: RepoId,
    pub name: Arc<str>,
}

impl Request for PublishRepo {
    type Response = PublishRepoResponse;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublishRepoResponse {
    pub credentials: RoomCredentials,
}
