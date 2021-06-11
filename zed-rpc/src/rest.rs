use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWorktreeResponse {
    pub worktree_id: u64,
    pub rpc_address: String,
}
