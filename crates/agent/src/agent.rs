pub mod context;
pub mod context_store;
pub mod thread_store;

use chrono::{DateTime, Utc};
pub use context::{AgentContext, ContextId, ContextLoadResult};
pub use context_store::ContextStore;
use fs::Fs;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub use thread_store::{
    DetailedSummaryState, SerializedThread, TextThreadStore, ThreadId, ThreadStore,
};

pub fn init(fs: Arc<dyn Fs>, cx: &mut gpui::App) {
    thread_store::init(fs, cx);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct MessageId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSnapshot {
    pub worktree_snapshots: Vec<WorktreeSnapshot>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorktreeSnapshot {
    pub worktree_path: String,
    pub git_state: Option<GitState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitState {
    pub remote_url: Option<String>,
    pub head_sha: Option<String>,
    pub current_branch: Option<String>,
    pub diff: Option<String>,
}
