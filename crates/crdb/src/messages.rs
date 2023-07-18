use crate::{Message, RepoId, Request, RoomCredentials};
use std::sync::Arc;

pub struct PublishRepo {
    pub id: RepoId,
    pub name: Arc<str>,
}

impl Message for PublishRepo {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Request for PublishRepo {
    type Response = PublishRepoResponse;
}

pub struct PublishRepoResponse {
    pub credentials: RoomCredentials,
}

impl Message for PublishRepoResponse {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
