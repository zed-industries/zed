use std::rc::Rc;

use acp_thread::{AgentConnection, LoadError};
use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::Result;
use collections::HashMap;
use futures::{FutureExt, future::Shared};
use gpui::{AppContext, Context, Entity, SharedString, Subscription, Task};
use project::{AgentServerStore, AgentServersUpdated, Project};
use watch::Receiver;

use crate::ExternalAgent;
use project::ExternalAgentServerName;

pub enum ConnectionEntry {
    Connecting {
        connect_task: Shared<Task<Result<Rc<dyn AgentConnection>, LoadError>>>,
    },
    Connected {
        connection: Rc<dyn AgentConnection>,
        new_version: Option<SharedString>,
    },
    Error {
        error: LoadError,
    },
}

impl ConnectionEntry {
    pub fn wait_for_connection(&self) -> Shared<Task<Result<Rc<dyn AgentConnection>, LoadError>>> {
        match self {
            ConnectionEntry::Connecting { connect_task } => connect_task.clone(),
            ConnectionEntry::Connected { connection, .. } => {
                Task::ready(Ok(connection.clone())).shared()
            }
            ConnectionEntry::Error { error } => Task::ready(Err(error.clone())).shared(),
        }
    }
}

pub struct AgentConnectionStore {
    project: Entity<Project>,
    entries: HashMap<ExternalAgent, Entity<ConnectionEntry>>,
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

    pub fn request_connection(
        &mut self,
        key: ExternalAgent,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> Entity<ConnectionEntry> {
        self.entries.get(&key).cloned().unwrap_or_else(|| {
            let (mut new_version_rx, connect_task) = self.start_connection(server.clone(), cx);
            let connect_task = connect_task.shared();

            let entry = cx.new(|_cx| ConnectionEntry::Connecting {
                connect_task: connect_task.clone(),
            });

            self.entries.insert(key, entry.clone());

            cx.spawn({
                let entry = entry.clone();
                async move |_this, cx| match connect_task.await {
                    Ok(connection) => {
                        entry.update(cx, |entry, cx| {
                            if let ConnectionEntry::Connecting { .. } = entry {
                                *entry = ConnectionEntry::Connected {
                                    connection,
                                    new_version: None,
                                };
                                cx.notify();
                            }
                        });
                    }
                    Err(error) => {
                        entry.update(cx, |entry, cx| {
                            if let ConnectionEntry::Connecting { .. } = entry {
                                *entry = ConnectionEntry::Error { error };
                                cx.notify();
                            }
                        });
                    }
                }
            })
            .detach();

            cx.spawn({
                let entry = entry.clone();
                async move |_this, cx| {
                    while let Ok(version) = new_version_rx.recv().await {
                        entry.update(cx, |entry, cx| {
                            if let ConnectionEntry::Connected { new_version, .. } = entry {
                                *new_version = version.map(|v| v.into());
                                cx.notify();
                            }
                        });
                    }
                }
            })
            .detach();

            entry
        })
    }

    pub fn invalidate(&mut self, key: &ExternalAgent) {
        self.entries.remove(key);
    }

    fn handle_agent_servers_updated(
        &mut self,
        store: Entity<AgentServerStore>,
        _: &AgentServersUpdated,
        cx: &mut Context<Self>,
    ) {
        let store = store.read(cx);
        self.entries.retain(|key, _| match key {
            ExternalAgent::NativeAgent => true,
            ExternalAgent::Custom { name } => store
                .external_agents
                .contains_key(&ExternalAgentServerName(name.clone())),
        });
        cx.notify();
    }

    fn start_connection(
        &self,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> (
        Receiver<Option<String>>,
        Task<Result<Rc<dyn AgentConnection>, LoadError>>,
    ) {
        let (new_version_tx, new_version_rx) = watch::channel::<Option<String>>(None);

        let agent_server_store = self.project.read(cx).agent_server_store().clone();
        let delegate = AgentServerDelegate::new(
            agent_server_store,
            self.project.clone(),
            Some(new_version_tx),
        );

        let connect_task = server.connect(delegate, cx);
        let connect_task = cx.spawn(async move |_this, _cx| match connect_task.await {
            Ok(connection) => Ok(connection),
            Err(err) => match err.downcast::<LoadError>() {
                Ok(load_error) => Err(load_error),
                Err(err) => Err(LoadError::Other(SharedString::from(err.to_string()))),
            },
        });
        (new_version_rx, connect_task)
    }
}
