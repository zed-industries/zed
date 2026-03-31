use std::rc::Rc;

use acp_thread::{AgentConnection, LoadError};
use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::Result;
use collections::HashMap;
use futures::{FutureExt, future::Shared};
use gpui::{App, AppContext, Context, Entity, EventEmitter, SharedString, Subscription, Task};

use project::{AgentServerStore, AgentServersUpdated, Project};
use watch::Receiver;

use crate::{Agent, ThreadHistory};

pub enum AgentConnectionEntry {
    Connecting {
        connect_task: Shared<Task<Result<AgentConnectedState, LoadError>>>,
    },
    Connected(AgentConnectedState),
    Error {
        error: LoadError,
    },
}

#[derive(Clone)]
pub struct AgentConnectedState {
    pub connection: Rc<dyn AgentConnection>,
    pub history: Option<Entity<ThreadHistory>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

impl AgentConnectionEntry {
    pub fn wait_for_connection(&self) -> Shared<Task<Result<AgentConnectedState, LoadError>>> {
        match self {
            AgentConnectionEntry::Connecting { connect_task } => connect_task.clone(),
            AgentConnectionEntry::Connected(state) => Task::ready(Ok(state.clone())).shared(),
            AgentConnectionEntry::Error { error } => Task::ready(Err(error.clone())).shared(),
        }
    }

    pub fn history(&self) -> Option<&Entity<ThreadHistory>> {
        match self {
            AgentConnectionEntry::Connected(state) => state.history.as_ref(),
            _ => None,
        }
    }

    pub fn status(&self) -> AgentConnectionStatus {
        match self {
            AgentConnectionEntry::Connecting { .. } => AgentConnectionStatus::Connecting,
            AgentConnectionEntry::Connected(_) => AgentConnectionStatus::Connected,
            AgentConnectionEntry::Error { .. } => AgentConnectionStatus::Disconnected,
        }
    }
}

pub enum AgentConnectionEntryEvent {
    NewVersionAvailable(SharedString),
}

impl EventEmitter<AgentConnectionEntryEvent> for AgentConnectionEntry {}

pub struct AgentConnectionStore {
    project: Entity<Project>,
    entries: HashMap<Agent, Entity<AgentConnectionEntry>>,
    _subscriptions: Vec<Subscription>,
}

impl AgentConnectionStore {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let agent_server_store = project.read(cx).agent_server_store().clone();
        let subscription = cx.subscribe(&agent_server_store, Self::handle_agent_servers_updated);
        Self {
            project,
            entries: HashMap::default(),
            _subscriptions: vec![subscription],
        }
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn entry(&self, key: &Agent) -> Option<&Entity<AgentConnectionEntry>> {
        self.entries.get(key)
    }

    pub fn connection_status(&self, key: &Agent, cx: &App) -> AgentConnectionStatus {
        self.entries
            .get(key)
            .map(|entry| entry.read(cx).status())
            .unwrap_or(AgentConnectionStatus::Disconnected)
    }

    pub fn restart_connection(
        &mut self,
        key: Agent,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> Entity<AgentConnectionEntry> {
        if let Some(entry) = self.entries.get(&key) {
            if matches!(entry.read(cx), AgentConnectionEntry::Connecting { .. }) {
                return entry.clone();
            }
        }

        self.entries.remove(&key);
        self.request_connection(key, server, cx)
    }

    pub fn request_connection(
        &mut self,
        key: Agent,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> Entity<AgentConnectionEntry> {
        if let Some(entry) = self.entries.get(&key) {
            return entry.clone();
        }

        let (mut new_version_rx, connect_task) = self.start_connection(server, cx);
        let connect_task = connect_task.shared();

        let entry = cx.new(|_cx| AgentConnectionEntry::Connecting {
            connect_task: connect_task.clone(),
        });

        self.entries.insert(key.clone(), entry.clone());
        cx.notify();

        cx.spawn({
            let key = key.clone();
            let entry = entry.downgrade();
            async move |this, cx| match connect_task.await {
                Ok(connected_state) => {
                    this.update(cx, move |this, cx| {
                        if this.entries.get(&key) != entry.upgrade().as_ref() {
                            return;
                        }

                        entry
                            .update(cx, move |entry, cx| {
                                if let AgentConnectionEntry::Connecting { .. } = entry {
                                    *entry = AgentConnectionEntry::Connected(connected_state);
                                    cx.notify();
                                }
                            })
                            .ok();
                    })
                    .ok();
                }
                Err(error) => {
                    this.update(cx, move |this, cx| {
                        if this.entries.get(&key) != entry.upgrade().as_ref() {
                            return;
                        }

                        entry
                            .update(cx, move |entry, cx| {
                                if let AgentConnectionEntry::Connecting { .. } = entry {
                                    *entry = AgentConnectionEntry::Error { error };
                                    cx.notify();
                                }
                            })
                            .ok();
                        this.entries.remove(&key);
                        cx.notify();
                    })
                    .ok();
                }
            }
        })
        .detach();

        cx.spawn({
            let entry = entry.downgrade();
            async move |this, cx| {
                while let Ok(version) = new_version_rx.recv().await {
                    let Some(version) = version else {
                        continue;
                    };

                    this.update(cx, move |this, cx| {
                        if this.entries.get(&key) != entry.upgrade().as_ref() {
                            return;
                        }

                        entry
                            .update(cx, move |_entry, cx| {
                                cx.emit(AgentConnectionEntryEvent::NewVersionAvailable(
                                    version.into(),
                                ));
                            })
                            .ok();
                        this.entries.remove(&key);
                        cx.notify();
                    })
                    .ok();
                    break;
                }
            }
        })
        .detach();

        entry
    }

    fn handle_agent_servers_updated(
        &mut self,
        store: Entity<AgentServerStore>,
        _: &AgentServersUpdated,
        cx: &mut Context<Self>,
    ) {
        let store = store.read(cx);
        self.entries.retain(|key, _| match key {
            Agent::NativeAgent => true,
            Agent::Custom { id } => store.external_agents.contains_key(id),
        });
        cx.notify();
    }

    fn start_connection(
        &self,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> (
        Receiver<Option<String>>,
        Task<Result<AgentConnectedState, LoadError>>,
    ) {
        let (new_version_tx, new_version_rx) = watch::channel::<Option<String>>(None);

        let agent_server_store = self.project.read(cx).agent_server_store().clone();
        let delegate = AgentServerDelegate::new(agent_server_store, Some(new_version_tx));

        let connect_task = server.connect(delegate, self.project.clone(), cx);
        let connect_task = cx.spawn(async move |_this, cx| match connect_task.await {
            Ok(connection) => cx.update(|cx| {
                let history = connection
                    .session_list(cx)
                    .map(|session_list| cx.new(|cx| ThreadHistory::new(session_list, cx)));
                Ok(AgentConnectedState {
                    connection,
                    history,
                })
            }),
            Err(err) => match err.downcast::<LoadError>() {
                Ok(load_error) => Err(load_error),
                Err(err) => Err(LoadError::Other(SharedString::from(err.to_string()))),
            },
        });
        (new_version_rx, connect_task)
    }
}
