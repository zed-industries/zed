use crate::User;
use std::sync::Arc;

#[derive(Clone)]
pub struct Call {
    pub room_id: u64,
    pub caller: Arc<User>,
    pub participants: Vec<Arc<User>>,
}
