use crate::User;
use std::sync::Arc;

#[derive(Clone)]
pub struct Call {
    pub from: Vec<Arc<User>>,
    pub room_id: u64,
}
