use agent_client_protocol::schema::v1 as acp;
use anyhow::Context as _;
use chrono::{DateTime, Utc};
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Task};
use project::AgentId;
use serde::{Deserialize, Serialize};
use util::ResultExt as _;

const NAMESPACE: &str = "acp_session_client_state";
const VERSION: u32 = 1;

#[derive(Clone, Serialize)]
struct StateKey {
    agent_id: AgentId,
    session_id: acp::SessionId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AcpSessionClientState {
    version: u32,
    draft_prompt: Option<Vec<acp::ContentBlock>>,
    updated_at: DateTime<Utc>,
}

pub fn read_draft_prompt(
    agent_id: &AgentId,
    session_id: &acp::SessionId,
    cx: &App,
) -> Option<Vec<acp::ContentBlock>> {
    read(agent_id, session_id, cx).and_then(|state| state.draft_prompt)
}

fn read(
    agent_id: &AgentId,
    session_id: &acp::SessionId,
    cx: &App,
) -> Option<AcpSessionClientState> {
    let key = key(agent_id, session_id).log_err()?;
    let kvp = KeyValueStore::global(cx);
    let raw = kvp.scoped(NAMESPACE).read(&key).log_err().flatten()?;
    serde_json::from_str(&raw).log_err()
}

pub fn write_draft_prompt(
    agent_id: AgentId,
    session_id: acp::SessionId,
    draft_prompt: Vec<acp::ContentBlock>,
    cx: &App,
) -> Task<anyhow::Result<()>> {
    let kvp = KeyValueStore::global(cx);
    let key = match key(&agent_id, &session_id) {
        Ok(key) => key,
        Err(err) => return Task::ready(Err(err)),
    };
    let payload = match serde_json::to_string(&AcpSessionClientState {
        version: VERSION,
        draft_prompt: Some(draft_prompt),
        updated_at: Utc::now(),
    })
    .context("serializing ACP session client state")
    {
        Ok(payload) => payload,
        Err(err) => return Task::ready(Err(err)),
    };
    cx.background_spawn(async move { kvp.scoped(NAMESPACE).write(key, payload).await })
}

pub fn delete(agent_id: AgentId, session_id: acp::SessionId, cx: &App) -> Task<anyhow::Result<()>> {
    let kvp = KeyValueStore::global(cx);
    let key = match key(&agent_id, &session_id) {
        Ok(key) => key,
        Err(err) => return Task::ready(Err(err)),
    };
    cx.background_spawn(async move { kvp.scoped(NAMESPACE).delete(key).await })
}

fn key(agent_id: &AgentId, session_id: &acp::SessionId) -> anyhow::Result<String> {
    serde_json::to_string(&StateKey {
        agent_id: agent_id.clone(),
        session_id: session_id.clone(),
    })
    .context("serializing ACP session client state key")
}
