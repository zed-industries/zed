use acp_thread::{AcpThreadMetadata, AgentConnection, AgentServerName};
use agent::{ThreadId, thread_store::ThreadStore};
use agent_client_protocol as acp;
use anyhow::{Context as _, Result};
use assistant_context::SavedContextMetadata;
use chrono::{DateTime, Utc};
use collections::HashMap;
use gpui::{App, AsyncApp, Entity, SharedString, Task, prelude::*};
use itertools::Itertools;
use paths::contexts_dir;
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::{collections::VecDeque, path::Path, sync::Arc, time::Duration};
use util::ResultExt as _;

const MAX_RECENTLY_OPENED_ENTRIES: usize = 6;
const NAVIGATION_HISTORY_PATH: &str = "agent-navigation-history.json";
const SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE: Duration = Duration::from_millis(50);

#[derive(Clone, Debug)]
pub enum HistoryEntry {
    Thread(AcpThreadMetadata),
    Context(SavedContextMetadata),
}

impl HistoryEntry {
    pub fn updated_at(&self) -> DateTime<Utc> {
        match self {
            HistoryEntry::Thread(thread) => thread.updated_at,
            HistoryEntry::Context(context) => context.mtime.to_utc(),
        }
    }

    pub fn id(&self) -> HistoryEntryId {
        match self {
            HistoryEntry::Thread(thread) => {
                HistoryEntryId::Thread(thread.agent.clone(), thread.id.clone())
            }
            HistoryEntry::Context(context) => HistoryEntryId::Context(context.path.clone()),
        }
    }

    pub fn title(&self) -> &SharedString {
        match self {
            HistoryEntry::Thread(thread) => &thread.title,
            HistoryEntry::Context(context) => &context.title,
        }
    }
}

/// Generic identifier for a history entry.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HistoryEntryId {
    Thread(AgentServerName, acp::SessionId),
    Context(Arc<Path>),
}

#[derive(Serialize, Deserialize)]
enum SerializedRecentOpen {
    Thread(String),
    ContextName(String),
    /// Old format which stores the full path
    Context(String),
}

pub struct AgentHistory {
    entries: HashMap<acp::SessionId, AcpThreadMetadata>,
    _task: Task<Result<()>>,
}

pub struct HistoryStore {
    agents: HashMap<AgentServerName, AgentHistory>,
}

impl HistoryStore {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            agents: HashMap::default(),
        }
    }

    pub fn register_agent(
        &mut self,
        agent_name: AgentServerName,
        connection: &dyn AgentConnection,
        cx: &mut Context<Self>,
    ) {
        let Some(mut history) = connection.list_threads(cx) else {
            return;
        };
        let task = cx.spawn(async move |this, cx| {
            while let Some(updated_history) = history.next().await {
                dbg!(&updated_history);
                this.update(cx, |this, cx| {
                    for entry in updated_history {
                        let agent = this
                            .agents
                            .get_mut(&entry.agent)
                            .context("agent not found")?;
                        agent.entries.insert(entry.id.clone(), entry);
                    }
                    cx.notify();
                    anyhow::Ok(())
                })??
            }
            Ok(())
        });
        self.agents.insert(
            agent_name,
            AgentHistory {
                entries: Default::default(),
                _task: task,
            },
        );
    }

    pub fn entries(&self, cx: &mut Context<Self>) -> Vec<HistoryEntry> {
        let mut history_entries = Vec::new();

        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return history_entries;
        }

        history_entries.extend(
            self.agents
                .values()
                .flat_map(|agent| agent.entries.values())
                .cloned()
                .map(HistoryEntry::Thread),
        );
        // todo!() include the text threads in here.

        history_entries.sort_unstable_by_key(|entry| std::cmp::Reverse(entry.updated_at()));
        history_entries
    }

    pub fn recent_entries(&self, limit: usize, cx: &mut Context<Self>) -> Vec<HistoryEntry> {
        self.entries(cx).into_iter().take(limit).collect()
    }
}
