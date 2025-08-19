use acp_thread::{AcpThreadMetadata, AgentConnection, AgentServerName};
use agent_client_protocol as acp;
use assistant_context::SavedContextMetadata;
use chrono::{DateTime, Utc};
use collections::HashMap;
use gpui::{SharedString, Task, prelude::*};
use serde::{Deserialize, Serialize};

use std::{path::Path, sync::Arc, time::Duration};

const MAX_RECENTLY_OPENED_ENTRIES: usize = 6;
const NAVIGATION_HISTORY_PATH: &str = "agent-navigation-history.json";
const SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE: Duration = Duration::from_millis(50);

// todo!(put this in the UI)
#[derive(Clone, Debug)]
pub enum HistoryEntry {
    AcpThread(AcpThreadMetadata),
    TextThread(SavedContextMetadata),
}

impl HistoryEntry {
    pub fn updated_at(&self) -> DateTime<Utc> {
        match self {
            HistoryEntry::AcpThread(thread) => thread.updated_at,
            HistoryEntry::TextThread(context) => context.mtime.to_utc(),
        }
    }

    pub fn id(&self) -> HistoryEntryId {
        match self {
            HistoryEntry::AcpThread(thread) => {
                HistoryEntryId::Thread(thread.agent.clone(), thread.id.clone())
            }
            HistoryEntry::TextThread(context) => HistoryEntryId::Context(context.path.clone()),
        }
    }

    pub fn title(&self) -> &SharedString {
        match self {
            HistoryEntry::AcpThread(thread) => &thread.title,
            HistoryEntry::TextThread(context) => &context.title,
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
    entries: watch::Receiver<Option<Vec<AcpThreadMetadata>>>,
    _task: Task<()>,
}

pub struct HistoryStore {
    agents: HashMap<AgentServerName, AgentHistory>, // todo!() text threads
}

impl HistoryStore {
    pub fn new(_cx: &mut Context<Self>) -> Self {
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
        let history = AgentHistory {
            entries: history.clone(),
            _task: cx.spawn(async move |this, cx| {
                dbg!("loaded", history.borrow().as_ref().map(|b| b.len()));
                while history.changed().await.is_ok() {
                    this.update(cx, |_, cx| cx.notify()).ok();
                }
            }),
        };
        self.agents.insert(agent_name.clone(), history);
    }

    pub fn entries(&mut self, _cx: &mut Context<Self>) -> Vec<HistoryEntry> {
        let mut history_entries = Vec::new();

        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return history_entries;
        }

        history_entries.extend(
            self.agents
                .values_mut()
                .flat_map(|history| history.entries.borrow().clone().unwrap_or_default()) // todo!("surface the loading state?")
                .map(HistoryEntry::AcpThread),
        );
        // todo!() include the text threads in here.

        history_entries.sort_unstable_by_key(|entry| std::cmp::Reverse(entry.updated_at()));
        history_entries
    }

    pub fn recent_entries(&mut self, limit: usize, cx: &mut Context<Self>) -> Vec<HistoryEntry> {
        self.entries(cx).into_iter().take(limit).collect()
    }
}
