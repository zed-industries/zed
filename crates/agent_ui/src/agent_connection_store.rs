use std::rc::Rc;

use acp_thread::{AgentConnection, LoadError};
use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::Result;
use collections::HashMap;
use futures::{FutureExt, future::Shared};
use gpui::{AppContext, Context, Entity, EventEmitter, SharedString, Subscription, Task};
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
    pub history: Entity<ThreadHistory>,
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
            AgentConnectionEntry::Connected(state) => Some(&state.history),
            _ => None,
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

    pub fn entry(&self, key: &Agent) -> Option<&Entity<AgentConnectionEntry>> {
        self.entries.get(key)
    }

    pub fn request_connection(
        &mut self,
        key: Agent,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> Entity<AgentConnectionEntry> {
        self.entries.get(&key).cloned().unwrap_or_else(|| {
            let (mut new_version_rx, connect_task) = self.start_connection(server.clone(), cx);
            let connect_task = connect_task.shared();

            let entry = cx.new(|_cx| AgentConnectionEntry::Connecting {
                connect_task: connect_task.clone(),
            });

            self.entries.insert(key.clone(), entry.clone());

            cx.spawn({
                let key = key.clone();
                let entry = entry.clone();
                async move |this, cx| match connect_task.await {
                    Ok(connected_state) => {
                        entry.update(cx, |entry, cx| {
                            if let AgentConnectionEntry::Connecting { .. } = entry {
                                *entry = AgentConnectionEntry::Connected(connected_state);
                                cx.notify();
                            }
                        });
                    }
                    Err(error) => {
                        entry.update(cx, |entry, cx| {
                            if let AgentConnectionEntry::Connecting { .. } = entry {
                                *entry = AgentConnectionEntry::Error { error };
                                cx.notify();
                            }
                        });
                        this.update(cx, |this, _cx| this.entries.remove(&key)).ok();
                    }
                }
            })
            .detach();

            cx.spawn({
                let entry = entry.clone();
                async move |this, cx| {
                    while let Ok(version) = new_version_rx.recv().await {
                        if let Some(version) = version {
                            entry.update(cx, |_entry, cx| {
                                cx.emit(AgentConnectionEntryEvent::NewVersionAvailable(
                                    version.clone().into(),
                                ));
                            });
                            this.update(cx, |this, _cx| this.entries.remove(&key)).ok();
                        }
                    }
                }
            })
            .detach();

            entry
        })
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

        let connect_task = server.connect(delegate, cx);
        let connect_task = cx.spawn(async move |_this, cx| match connect_task.await {
            Ok(connection) => cx.update(|cx| {
                let history = cx.new(|cx| ThreadHistory::new(connection.session_list(cx), cx));
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
