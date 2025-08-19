use acp_thread::{AcpThreadMetadata, AgentConnection, AgentServerName};
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use assistant_context::SavedContextMetadata;
use chrono::{DateTime, Utc};
use collections::HashMap;
use gpui::{Entity, Global, SharedString, Task, prelude::*};
use project::Project;
use serde::{Deserialize, Serialize};
use ui::App;

use std::{path::Path, rc::Rc, sync::Arc, time::Duration};

use crate::NativeAgentServer;

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

#[derive(Default)]
pub struct AgentHistory {
    entries: HashMap<acp::SessionId, AcpThreadMetadata>,
    loaded: bool,
}

pub struct HistoryStore {
    agents: HashMap<AgentServerName, AgentHistory>, // todo!() text threads
}
// note, we have to share the history store between all windows
// because we only get updates from one connection at a time.
struct GlobalHistoryStore(Entity<HistoryStore>);
impl Global for GlobalHistoryStore {}

impl HistoryStore {
    pub fn get_or_init(project: &Entity<Project>, cx: &mut App) -> Entity<Self> {
        if cx.has_global::<GlobalHistoryStore>() {
            return cx.global::<GlobalHistoryStore>().0.clone();
        }
        let history_store = cx.new(|cx| HistoryStore::new(cx));
        cx.set_global(GlobalHistoryStore(history_store.clone()));
        let root_dir = project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path())
            .unwrap_or_else(|| paths::home_dir().as_path().into());

        let agent = NativeAgentServer::new(project.read(cx).fs().clone());
        let connect = agent.connect(&root_dir, project, cx);
        cx.spawn({
            let history_store = history_store.clone();
            async move |cx| {
                let connection = connect.await?.history().unwrap();
                history_store
                    .update(cx, |history_store, cx| {
                        history_store.load_history(agent.name(), connection.as_ref(), cx)
                    })?
                    .await
            }
        })
        .detach_and_log_err(cx);
        history_store
    }

    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            agents: HashMap::default(),
        }
    }

    pub fn update_history(&mut self, entry: AcpThreadMetadata, cx: &mut Context<Self>) {
        let agent = self
            .agents
            .entry(entry.agent.clone())
            .or_insert(Default::default());

        agent.entries.insert(entry.id.clone(), entry);
        cx.notify()
    }

    pub fn load_history(
        &mut self,
        agent_name: AgentServerName,
        connection: &dyn acp_thread::AgentHistory,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        let threads = connection.list_threads(cx);
        cx.spawn(async move |this, cx| {
            let threads = threads.await?;

            this.update(cx, |this, cx| {
                this.agents.insert(
                    agent_name,
                    AgentHistory {
                        loaded: true,
                        entries: threads.into_iter().map(|t| (t.id.clone(), t)).collect(),
                    },
                );
                cx.notify()
            })
        })
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
                .flat_map(|history| history.entries.values().cloned()) // todo!("surface the loading state?")
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
