use crate::User;
use std::sync::Arc;

#[derive(Clone)]
pub struct IncomingCall {
    pub room_id: u64,
    pub caller: Arc<User>,
    pub participants: Vec<Arc<User>>,
    pub initial_project_id: Option<u64>,
}
