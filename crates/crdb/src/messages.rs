use crate::{RepoId, Request};
use std::sync::Arc;

pub struct PublishRepo {
    pub id: RepoId,
    pub name: Arc<str>,
}

impl Request for PublishRepo {
    type Response = ();
}
