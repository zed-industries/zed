use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWorktreeResponse {
    pub worktree_id: i32,
    pub rpc_address: String,
}
