use std::rc::Rc;
use std::time::{Duration, Instant};

use acp_thread::{AgentConnection, LoadError};
use agent_servers::AcpConnection;
use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::Result;
use collections::HashMap;
use futures::{FutureExt, future::Shared};
use gpui::{
    App, AppContext, Context, Entity, EventEmitter, Global, SharedString, Subscription, Task,
};

use project::{AgentServerStore, AgentServersUpdated, Project};
use watch::Receiver;

use crate::Agent;

const DEFAULT_UNUSED_CONNECTION_GRACE: Duration = Duration::from_secs(30);

const REAP_INTERVAL: Duration = Duration::from_secs(10);

pub struct UnusedConnectionGrace(pub Duration);
impl Global for UnusedConnectionGrace {}

impl UnusedConnectionGrace {
    pub fn global(cx: &App) -> Duration {
        cx.try_global::<Self>()
            .map_or(DEFAULT_UNUSED_CONNECTION_GRACE, |grace| grace.0)
    }
}

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
    pub connection: Rc<AcpConnection>,
}

/// Dropping a lease is deliberately inert — it holds a token instead of calling
/// back into the store — so one that outlives the store, or the entry it was
/// taken against, cannot corrupt the count.
#[derive(Clone)]
pub struct AgentConnectionLease {
    key: Agent,
    _token: Rc<()>,
}

impl AgentConnectionLease {
    pub fn key(&self) -> &Agent {
        &self.key
    }
}

struct CachedConnection {
    entry: Entity<AgentConnectionEntry>,
    /// Strong count of 1 means the store is the only holder.
    token: Rc<()>,
    unused_since: Option<Instant>,
    stale: bool,
}

impl CachedConnection {
    fn lease_count(&self) -> usize {
        Rc::strong_count(&self.token) - 1
    }

    fn lease(&self, key: Agent) -> AgentConnectionLease {
        AgentConnectionLease {
            key,
            _token: self.token.clone(),
        }
    }
}

pub struct AgentConnectionStore {
    project: Entity<Project>,
    entries: HashMap<Agent, CachedConnection>,
    _reaper: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl AgentConnectionStore {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let agent_server_store = project.read(cx).agent_server_store().clone();
        let subscription = cx.subscribe(&agent_server_store, Self::handle_agent_servers_updated);
        let reaper = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(REAP_INTERVAL).await;
                if this
                    .update(cx, |this, cx| {
                        this.reap_unused_connections(Instant::now(), cx)
                    })
                    .is_err()
                {
                    return;
                }
            }
        });
        Self {
            project,
            entries: HashMap::default(),
            _reaper: reaper,
            _subscriptions: vec![subscription],
        }
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn entry(&self, key: &Agent) -> Option<&Entity<AgentConnectionEntry>> {
        self.entries.get(key).map(|cached| &cached.entry)
    }

    pub fn connection_status(&self, key: &Agent, cx: &App) -> AgentConnectionStatus {
        self.entries
            .get(key)
            .map(|cached| cached.entry.read(cx).status())
            .unwrap_or(AgentConnectionStatus::Disconnected)
    }

    pub fn agent_version(&self, key: &Agent, cx: &App) -> Option<SharedString> {
        match self.entries.get(key)?.entry.read(cx) {
            AgentConnectionEntry::Connected(state) => state.connection.agent_version(),
            AgentConnectionEntry::Connecting { .. } | AgentConnectionEntry::Error { .. } => None,
        }
    }

    pub fn active_acp_connections(&self, cx: &App) -> Vec<ActiveAcpConnection> {
        self.entries
            .values()
            .filter_map(|cached| match cached.entry.read(cx) {
                AgentConnectionEntry::Connected(state) => state
                    .connection
                    .clone()
                    .downcast::<AcpConnection>()
                    .map(|connection| ActiveAcpConnection {
                        agent_id: state.connection.agent_id(),
                        connection,
                    }),
                AgentConnectionEntry::Connecting { .. } | AgentConnectionEntry::Error { .. } => {
                    None
                }
            })
            .collect()
    }

    pub fn lease_count(&self, key: &Agent) -> usize {
        self.entries
            .get(key)
            .map_or(0, |cached| cached.lease_count())
    }

    pub fn restart_connection(
        &mut self,
        key: Agent,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> Entity<AgentConnectionEntry> {
        if let Some(cached) = self.entries.get(&key) {
            if matches!(
                cached.entry.read(cx),
                AgentConnectionEntry::Connecting { .. }
            ) {
                return cached.entry.clone();
            }
        }

        self.connect(key, server, cx).0
    }

    /// Hold the returned lease for as long as the connection is needed,
    /// including across a task that is only awaiting an RPC.
    pub fn request_connection(
        &mut self,
        key: Agent,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> (Entity<AgentConnectionEntry>, AgentConnectionLease) {
        if let Some(cached) = self.entries.get_mut(&key) {
            let reusable = !cached.stale
                && !matches!(cached.entry.read(cx), AgentConnectionEntry::Error { .. });
            if reusable {
                cached.unused_since = None;
                return (cached.entry.clone(), cached.lease(key));
            }
        }

        self.connect(key, server, cx)
    }

    /// The replaced entry's token is carried over: its leases belong to holders
    /// still using this agent, and a fresh token would look unused.
    fn connect(
        &mut self,
        key: Agent,
        server: Rc<dyn AgentServer>,
        cx: &mut Context<Self>,
    ) -> (Entity<AgentConnectionEntry>, AgentConnectionLease) {
        let (mut new_version_rx, mut loading_status_rx, connect_task) =
            self.start_connection(server, cx);
        let connect_task = connect_task.shared();

        let entry = cx.new(|_cx| AgentConnectionEntry::Connecting {
            connect_task: connect_task.clone(),
        });

        let token = self
            .entries
            .remove(&key)
            .map_or_else(|| Rc::new(()), |cached| cached.token);
        let cached = CachedConnection {
            entry: entry.clone(),
            token,
            unused_since: None,
            stale: false,
        };
        let lease = cached.lease(key.clone());
        self.entries.insert(key.clone(), cached);
        cx.notify();

        cx.spawn({
            let key = key.clone();
            let entry = entry.downgrade();
            async move |this, cx| match connect_task.await {
                Ok(connected_state) => {
                    this.update(cx, move |this, cx| {
                        if !this.is_current_entry(&key, &entry) {
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
                        if !this.is_current_entry(&key, &entry) {
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
                        if !this.is_current_entry(&key, &entry) {
                            return;
                        }

                        entry
                            .update(cx, move |_entry, cx| {
                                cx.emit(AgentConnectionEntryEvent::NewVersionAvailable(
                                    version.into(),
                                ));
                            })
                            .ok();
                        if let Some(cached) = this.entries.get_mut(&key) {
                            cached.stale = true;
                        }
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
                        if !this.is_current_entry(&key, &entry) {
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

        (entry, lease)
    }

    fn is_current_entry(
        &self,
        key: &Agent,
        entry: &gpui::WeakEntity<AgentConnectionEntry>,
    ) -> bool {
        self.entries
            .get(key)
            .zip(entry.upgrade())
            .is_some_and(|(cached, entry)| cached.entry == entry)
    }

    /// Unused is observed on one pass and reaped on a later one, so a
    /// connection outlives its last holder by the grace period plus up to one
    /// sweep: a lease has no drop hook, and respawning costs more than waiting.
    pub fn reap_unused_connections(&mut self, now: Instant, cx: &mut Context<Self>) {
        let grace = UnusedConnectionGrace::global(cx);
        let mut reaped = false;

        self.entries.retain(|key, cached| {
            // Runs in-process, so there is no process to reclaim.
            if key.is_native() {
                return true;
            }
            if cached.lease_count() > 0 {
                cached.unused_since = None;
                return true;
            }
            // Leave it until it settles, so a connection requested a moment
            // ago is not reaped from under the caller awaiting it.
            if matches!(
                cached.entry.read(cx),
                AgentConnectionEntry::Connecting { .. }
            ) {
                return true;
            }

            let unused_since = *cached.unused_since.get_or_insert(now);
            if now.saturating_duration_since(unused_since) < grace {
                return true;
            }

            log::debug!("reaping unused agent connection for {key:?}");
            reaped = true;
            false
        });

        if reaped {
            cx.notify();
        }
    }

    fn handle_agent_servers_updated(
        &mut self,
        store: Entity<AgentServerStore>,
        _: &AgentServersUpdated,
        cx: &mut Context<Self>,
    ) {
        let store = store.read(cx);
        self.retain_configured_agents(|key| match key {
            Agent::NativeAgent => true,
            Agent::Custom { id } => store.external_agents.contains_key(id),
            #[cfg(any(test, feature = "test-support"))]
            Agent::Stub => true,
        });
        cx.notify();
    }

    /// An agent that has been unconfigured keeps its entry until its last lease
    /// is released, so that holders stay counted and the connection is reaped
    /// once rather than orphaned here and replaced by a fresh, uncounted one.
    /// Marking it stale keeps it from being handed to any new caller.
    pub(crate) fn retain_configured_agents(&mut self, is_configured: impl Fn(&Agent) -> bool) {
        self.entries.retain(|key, cached| {
            if is_configured(key) {
                return true;
            }
            if cached.lease_count() > 0 {
                cached.stale = true;
                return true;
            }
            false
        });
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

        let connect_task = server.connect(delegate, self.project.clone(), cx);
        let connect_task = cx.spawn(async move |_this, _cx| match connect_task.await {
            Ok(connection) => Ok(AgentConnectedState { connection }),
            Err(err) => match err.downcast::<LoadError>() {
                Ok(load_error) => Err(load_error),
                Err(err) => Err(LoadError::Other(SharedString::from(err.to_string()))),
            },
        });
        (new_version_rx, loading_status_rx, connect_task)
    }
}
