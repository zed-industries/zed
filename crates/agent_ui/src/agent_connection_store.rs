use std::rc::Rc;

use acp_thread::AgentConnection;
use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::Result;
use collections::HashMap;
use gpui::{Context, Entity, SharedString, Subscription, Task};
use project::{AgentServerStore, AgentServersUpdated, Project};
use util::ResultExt as _;

use crate::ExternalAgent;
use project::ExternalAgentServerName;

enum ConnectionEntry {
    Connecting {
        /// Shared future for the in-flight connection attempt.
        /// Multiple requesters await clones of this receiver.
        result_rx: watch::Receiver<Option<Result<Rc<dyn AgentConnection>, SharedString>>>,
        status_rx: watch::Receiver<SharedString>,
        new_version_rx: watch::Receiver<Option<String>>,
    },
    Ready {
        connection: Rc<dyn AgentConnection>,
        status_rx: watch::Receiver<SharedString>,
        new_version_rx: watch::Receiver<Option<String>>,
    },
}

pub struct ConnectionRequestHandle {
    pub result: Task<Result<Rc<dyn AgentConnection>>>,
    pub status_rx: watch::Receiver<SharedString>,
    pub new_version_rx: watch::Receiver<Option<String>>,
}

pub struct AgentConnectionStore {
    project: Entity<Project>,
    entries: HashMap<ExternalAgent, ConnectionEntry>,
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
    ) -> ConnectionRequestHandle {
        if let Some(entry) = self.entries.get(&key) {
            match entry {
                ConnectionEntry::Ready {
                    connection,
                    status_rx,
                    new_version_rx,
                } => {
                    return ConnectionRequestHandle {
                        result: Task::ready(Ok(connection.clone())),
                        status_rx: status_rx.clone(),
                        new_version_rx: new_version_rx.clone(),
                    };
                }
                ConnectionEntry::Connecting {
                    result_rx,
                    status_rx,
                    new_version_rx,
                } => {
                    let mut result_rx = result_rx.clone();
                    let result = cx.spawn(async move |_this, _cx| {
                        Self::await_connection_result(&mut result_rx).await
                    });
                    return ConnectionRequestHandle {
                        result,
                        status_rx: status_rx.clone(),
                        new_version_rx: new_version_rx.clone(),
                    };
                }
            }
        }

        self.start_connection(key, server, cx)
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
        &mut self,
        key: ExternalAgent,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> ConnectionRequestHandle {
        let (status_tx, status_rx) = watch::channel::<SharedString>("Loading…".into());
        let (new_version_tx, new_version_rx) = watch::channel::<Option<String>>(None);
        let (mut result_tx, result_rx) =
            watch::channel::<Option<Result<Rc<dyn AgentConnection>, SharedString>>>(None);

        let agent_server_store = self.project.read(cx).agent_server_store().clone();
        let delegate = AgentServerDelegate::new(
            agent_server_store,
            self.project.clone(),
            Some(status_tx),
            Some(new_version_tx),
        );

        let connect_task = server.connect(delegate, cx);
        let entry_key = key.clone();
        cx.spawn(async move |this, cx| match connect_task.await {
            Ok(connection) => {
                result_tx.send(Some(Ok(connection.clone()))).ok();
                this.update(cx, |this, cx| {
                    if let Some(ConnectionEntry::Connecting {
                        status_rx,
                        new_version_rx,
                        ..
                    }) = this.entries.remove(&entry_key)
                    {
                        this.entries.insert(
                            entry_key,
                            ConnectionEntry::Ready {
                                connection,
                                status_rx,
                                new_version_rx,
                            },
                        );
                    }
                    cx.notify();
                })
                .log_err();
            }
            Err(err) => {
                result_tx.send(Some(Err(err.to_string().into()))).ok();
                this.update(cx, |this, cx| {
                    this.entries.remove(&entry_key);
                    cx.notify();
                })
                .log_err();
            }
        })
        .detach();

        let mut first_result_rx = result_rx.clone();
        let result = cx.spawn(async move |_this, _cx| {
            Self::await_connection_result(&mut first_result_rx).await
        });

        let handle_status_rx = status_rx.clone();
        let handle_new_version_rx = new_version_rx.clone();

        self.entries.insert(
            key,
            ConnectionEntry::Connecting {
                result_rx,
                status_rx,
                new_version_rx,
            },
        );

        ConnectionRequestHandle {
            result,
            status_rx: handle_status_rx,
            new_version_rx: handle_new_version_rx,
        }
    }

    async fn await_connection_result(
        result_rx: &mut watch::Receiver<Option<Result<Rc<dyn AgentConnection>, SharedString>>>,
    ) -> Result<Rc<dyn AgentConnection>> {
        loop {
            match result_rx.recv().await {
                Ok(Some(Ok(connection))) => return Ok(connection),
                Ok(Some(Err(message))) => return Err(anyhow::anyhow!("{}", message)),
                Ok(None) => continue,
                Err(_) => return Err(anyhow::anyhow!("connection attempt was cancelled")),
            }
        }
    }
}
