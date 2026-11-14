use std::rc::{Rc, Weak};

use acp_thread::{AgentConnection, LoadError};
use agent_servers::{AcpConnection, AcpDebugMessage, AcpV2Connection};
use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::Result;
use collections::HashMap;
use futures::{FutureExt, future::Shared};
use gpui::{App, AppContext, Context, Entity, EventEmitter, SharedString, Subscription, Task};

use project::{AgentServerStore, AgentServersUpdated, Project};
use watch::Receiver;

use crate::Agent;

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
    LoadingStatusChanged(Option<SharedString>),
}

impl EventEmitter<AgentConnectionEntryEvent> for AgentConnectionEntry {}

#[derive(Clone)]
pub struct ActiveAcpConnection {
    pub agent_id: project::AgentId,
    pub connection: Rc<dyn AgentConnection>,
}

impl ActiveAcpConnection {
    pub fn subscribe_debug_messages(
        &self,
    ) -> Option<(
        Vec<AcpDebugMessage>,
        async_channel::Receiver<AcpDebugMessage>,
    )> {
        if let Some(connection) = self.connection.clone().downcast::<AcpConnection>() {
            Some(connection.subscribe_debug_messages())
        } else {
            self.connection
                .clone()
                .downcast::<AcpV2Connection>()
                .map(|connection| connection.subscribe_debug_messages())
        }
    }
}

pub struct AgentConnectionStore {
    project: Entity<Project>,
    entries: HashMap<Agent, Entity<AgentConnectionEntry>>,
    owned_connections: Vec<Weak<dyn AgentConnection>>,
    shutdown: Option<watch::Sender<()>>,
    _subscriptions: Vec<Subscription>,
}

impl AgentConnectionStore {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let agent_server_store = project.read(cx).agent_server_store().clone();
        let subscription = cx.subscribe(&agent_server_store, Self::handle_agent_servers_updated);
        let (shutdown, _) = watch::channel(());
        cx.on_release(|this, _| this.shutdown_connections())
            .detach();
        let quit = cx.on_app_quit(|this, _| {
            this.shutdown_connections();
            async {}
        });
        Self {
            project,
            entries: HashMap::default(),
            owned_connections: Vec::new(),
            shutdown: Some(shutdown),
            _subscriptions: vec![subscription, quit],
        }
    }

    fn shutdown_connections(&mut self) {
        drop(self.shutdown.take());
        // Cache replacement must not hide a connection retained by an older view or receipt.
        for connection in self
            .owned_connections
            .drain(..)
            .filter_map(|owner| owner.upgrade())
        {
            connection.shutdown();
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

    pub fn agent_version(&self, key: &Agent, cx: &App) -> Option<SharedString> {
        match self.entries.get(key)?.read(cx) {
            AgentConnectionEntry::Connected(state) => state.connection.agent_version(),
            AgentConnectionEntry::Connecting { .. } | AgentConnectionEntry::Error { .. } => None,
        }
    }

    pub fn active_acp_connections(&self, cx: &App) -> Vec<ActiveAcpConnection> {
        self.entries
            .values()
            .filter_map(|entry| match entry.read(cx) {
                AgentConnectionEntry::Connected(state) => {
                    let connection = state.connection.clone();
                    let concrete = connection.clone().into_any();
                    (concrete.is::<AcpConnection>() || concrete.is::<AcpV2Connection>()).then(
                        || ActiveAcpConnection {
                            agent_id: connection.agent_id(),
                            connection,
                        },
                    )
                }
                AgentConnectionEntry::Connecting { .. } | AgentConnectionEntry::Error { .. } => {
                    None
                }
            })
            .collect()
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

        let (mut new_version_rx, mut loading_status_rx, connect_task) =
            self.start_connection(server, cx);
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
                        cx.notify();
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
            let key = key.clone();
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

        cx.spawn({
            let entry = entry.downgrade();
            async move |this, cx| {
                while let Ok(status) = loading_status_rx.recv().await {
                    let status = status.map(SharedString::from);
                    let key = key.clone();
                    let entry = entry.clone();
                    this.update(cx, move |this, cx| {
                        if this.entries.get(&key) != entry.upgrade().as_ref() {
                            return;
                        }

                        entry
                            .update(cx, move |_entry, cx| {
                                cx.emit(AgentConnectionEntryEvent::LoadingStatusChanged(status));
                            })
                            .ok();
                        cx.notify();
                    })
                    .ok();
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
            #[cfg(any(test, feature = "test-support"))]
            Agent::Stub => true,
        });
        cx.notify();
    }

    fn start_connection(
        &self,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> (
        Receiver<Option<String>>,
        Receiver<Option<String>>,
        Task<Result<AgentConnectedState, LoadError>>,
    ) {
        let (new_version_tx, new_version_rx) = watch::channel::<Option<String>>(None);
        let (loading_status_tx, loading_status_rx) = watch::channel::<Option<String>>(None);

        let agent_server_store = self.project.read(cx).agent_server_store().clone();
        let delegate = AgentServerDelegate::new(
            agent_server_store,
            Some(new_version_tx),
            Some(loading_status_tx),
        );

        let shutdown = self.shutdown.as_ref().map(watch::Sender::receiver);
        let connect_task =
            shutdown.map(|shutdown| (server.connect(delegate, self.project.clone(), cx), shutdown));
        let connect_task = cx.spawn(async move |this, cx| {
            let owner_released = || LoadError::Other("Agent connection owner was released".into());
            let Some((connect_task, mut shutdown)) = connect_task else {
                return Err(owner_released());
            };
            let stopped = shutdown.changed();
            futures::pin_mut!(stopped);
            let result = match futures::future::select(connect_task, stopped).await {
                futures::future::Either::Left((result, _)) => result,
                futures::future::Either::Right(_) => return Err(owner_released()),
            };
            let connection = result.map_err(|error| match error.downcast::<LoadError>() {
                Ok(error) => error,
                Err(error) => LoadError::Other(error.to_string().into()),
            })?;
            let registered = this
                .update(cx, |this, _| {
                    if this.shutdown.is_none() {
                        return false;
                    }
                    this.owned_connections
                        .retain(|connection| connection.strong_count() > 0);
                    this.owned_connections.push(Rc::downgrade(&connection));
                    true
                })
                .is_ok_and(|registered| registered);
            if !registered {
                connection.shutdown();
                return Err(owner_released());
            }
            Ok(AgentConnectedState { connection })
        });
        (new_version_rx, loading_status_rx, connect_task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation_view::tests::{StubAgentServer, init_test};
    use acp_thread::StubAgentConnection;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use std::{
        any::Any,
        path::Path,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
    };

    #[gpui::test]
    async fn test_connection_owner_shutdown_includes_replaced_connections(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", serde_json::json!({})).await;
        let project = Project::test(fs, [Path::new("/project")], cx).await;
        for quitting in [false, true] {
            let store = cx.new(|cx| AgentConnectionStore::new(project.clone(), cx));
            let original = StubAgentConnection::new();
            let replacement = StubAgentConnection::new();
            let entry = store.update(cx, |store, cx| {
                store.request_connection(
                    Agent::Stub,
                    Rc::new(StubAgentServer::new(original.clone())),
                    cx,
                )
            });
            let observer = entry.read_with(cx, |entry, _| entry.wait_for_connection());
            let connected = observer.await.expect("connected");
            cx.run_until_parked();
            let other_view = connected.clone();
            drop(connected);
            assert_eq!(
                original.shutdown_calls(),
                0,
                "one observer is not the owner"
            );
            let replacement_entry = store.update(cx, |store, cx| {
                store.restart_connection(
                    Agent::Stub,
                    Rc::new(StubAgentServer::new(replacement.clone())),
                    cx,
                )
            });
            let connected = replacement_entry
                .read_with(cx, |entry, _| entry.wait_for_connection())
                .await
                .expect("replacement connected");
            cx.run_until_parked();
            assert_eq!(
                original.shutdown_calls(),
                0,
                "restart preserves older views"
            );
            if quitting {
                cx.update(|cx| cx.shutdown());
                assert_eq!(original.shutdown_calls(), 1);
                assert_eq!(replacement.shutdown_calls(), 1);
                let late = store.update(cx, |store, cx| {
                    store.restart_connection(
                        Agent::Stub,
                        Rc::new(StubAgentServer::new(StubAgentConnection::new())),
                        cx,
                    )
                });
                assert!(
                    late.read_with(cx, |entry, _| entry.wait_for_connection())
                        .await
                        .is_err(),
                    "an ended owner cannot start another connection"
                );
            }
            drop(store);
            cx.update(|_| ());
            cx.run_until_parked();
            assert_eq!(original.shutdown_calls(), 1);
            assert_eq!(replacement.shutdown_calls(), 1);
            assert!(original.take_close_session_calls().is_empty());
            assert!(replacement.take_close_session_calls().is_empty());
            drop(other_view);
            drop(connected);
        }
    }

    struct PendingAgentServer {
        started: Arc<AtomicBool>,
        dropped: Arc<AtomicBool>,
    }

    impl AgentServer for PendingAgentServer {
        fn logo(&self) -> ui::IconName {
            ui::IconName::ZedAgent
        }

        fn agent_id(&self) -> project::AgentId {
            "pending".into()
        }

        fn connect(
            &self,
            _delegate: AgentServerDelegate,
            _project: Entity<Project>,
            cx: &mut App,
        ) -> Task<Result<Rc<dyn AgentConnection>>> {
            let started = self.started.clone();
            let dropped = self.dropped.clone();
            cx.foreground_executor().spawn(async move {
                let _guard = util::defer(move || dropped.store(true, Ordering::SeqCst));
                started.store(true, Ordering::SeqCst);
                futures::future::pending().await
            })
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[gpui::test]
    async fn test_connection_owner_shutdown_cancels_pending_startup(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", serde_json::json!({})).await;
        let project = Project::test(fs, [Path::new("/project")], cx).await;
        for quitting in [false, true] {
            let store = cx.new(|cx| AgentConnectionStore::new(project.clone(), cx));
            let started = Arc::new(AtomicBool::new(false));
            let dropped = Arc::new(AtomicBool::new(false));
            let entry = store.update(cx, |store, cx| {
                store.request_connection(
                    Agent::Stub,
                    Rc::new(PendingAgentServer {
                        started: started.clone(),
                        dropped: dropped.clone(),
                    }),
                    cx,
                )
            });
            let pending = entry.read_with(cx, |entry, _| entry.wait_for_connection());
            cx.run_until_parked();
            assert!(started.load(Ordering::SeqCst));
            if quitting {
                cx.update(|cx| cx.shutdown());
            } else {
                drop(store);
                cx.update(|_| ());
            }
            assert!(pending.await.is_err());
            cx.run_until_parked();
            assert!(dropped.load(Ordering::SeqCst));
        }
        let store = cx.new(|cx| AgentConnectionStore::new(project.clone(), cx));
        let connection = StubAgentConnection::new();
        let entry = store.update(cx, |store, cx| {
            store.request_connection(
                Agent::Stub,
                Rc::new(StubAgentServer::new(connection.clone())),
                cx,
            )
        });
        let pending = entry.read_with(cx, |entry, _| entry.wait_for_connection());
        drop(store);
        cx.update(|_| ());
        assert!(
            pending.await.is_err(),
            "late success cannot outlive its owner"
        );
        assert_eq!(connection.shutdown_calls(), 1);
    }
}
