//! Per-thread draft prompt persistence.
//!
//! Drafts are persisted in the thread metadata store with `session_id: None`,
//! but their unsent prompt text is kept separately here so we don't have to
//! plumb draft-prompt storage through the native agent's thread database.

use agent_client_protocol::schema as acp;
use anyhow::Context as _;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Task};
use util::ResultExt as _;

use crate::thread_metadata_store::ThreadId;

const NAMESPACE: &str = "agent_draft_prompts";

pub fn read(thread_id: ThreadId, cx: &App) -> Option<Vec<acp::ContentBlock>> {
    let kvp = KeyValueStore::global(cx);
    let raw = kvp
        .scoped(NAMESPACE)
        .read(&thread_id_key(thread_id))
        .log_err()
        .flatten()?;
    serde_json::from_str(&raw).log_err()
}

pub fn write(
    thread_id: ThreadId,
    prompt: &[acp::ContentBlock],
    cx: &App,
) -> Task<anyhow::Result<()>> {
    let kvp = KeyValueStore::global(cx);
    let key = thread_id_key(thread_id);
    let payload = match serde_json::to_string(prompt).context("serializing draft prompt") {
        Ok(payload) => payload,
        Err(err) => return Task::ready(Err(err)),
    };
    cx.background_spawn(async move { kvp.scoped(NAMESPACE).write(key, payload).await })
}

pub fn delete(thread_id: ThreadId, cx: &App) -> Task<anyhow::Result<()>> {
    let kvp = KeyValueStore::global(cx);
    let key = thread_id_key(thread_id);
    cx.background_spawn(async move { kvp.scoped(NAMESPACE).delete(key).await })
}

fn thread_id_key(thread_id: ThreadId) -> String {
    thread_id.to_key_string()
}
