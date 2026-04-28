//! Global store of nREPL connections, keyed by workspace.
//!
//! One workspace == zero or one active TCP connection to an nREPL server,
//! plus a single nREPL `session` cloned on top of it. Per the design doc
//! ("Session model"), every editor in the workspace eventually shares that
//! one session so `*1`/`*2`/`*3` and `*ns*` continuity behave the way
//! Clojure devs expect.
//!
//! State for a single connection lives in [`NreplConnection`], which is
//! itself an entity so views can observe it directly. The store mostly
//! manages the workspace -> connection map and the command-palette filter.

use std::{future::Future, net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use command_palette_hooks::CommandPaletteFilter;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EntityId, Global, SharedString, Subscription,
    Task, WeakEntity,
};
use project::Fs;
use settings::{Settings, SettingsStore};
use util::ResultExt as _;
use workspace::Workspace;

use crate::client::NreplClient;
use crate::discovery::{DiscoveredPort, discover_port};
use crate::nrepl_settings::NreplSettings;

const NAMESPACE: &str = "nrepl";

struct GlobalNreplStore(Entity<NreplStore>);
impl Global for GlobalNreplStore {}

pub struct NreplStore {
    fs: Arc<dyn Fs>,
    enabled: bool,
    connections: HashMap<EntityId, Entity<NreplConnection>>,
    _subscriptions: Vec<Subscription>,
}

impl NreplStore {
    pub(crate) fn init(fs: Arc<dyn Fs>, cx: &mut App) {
        let store = cx.new(move |cx| Self::new(fs, cx));
        cx.set_global(GlobalNreplStore(store));
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalNreplStore>().0.clone()
    }

    pub fn new(fs: Arc<dyn Fs>, cx: &mut Context<Self>) -> Self {
        let subscriptions = vec![
            cx.observe_global::<SettingsStore>(|this, cx| {
                this.set_enabled(NreplSettings::enabled(cx), cx);
            }),
            cx.on_app_quit(Self::shutdown_all),
        ];

        let this = Self {
            fs,
            enabled: NreplSettings::enabled(cx),
            connections: HashMap::default(),
            _subscriptions: subscriptions,
        };
        this.update_command_palette_filter(cx);
        this
    }

    pub fn fs(&self) -> &Arc<dyn Fs> {
        &self.fs
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Iterate every live workspace connection, in arbitrary order.
    pub fn connections(&self) -> impl Iterator<Item = &Entity<NreplConnection>> {
        self.connections.values()
    }

    /// Returns the connection for `workspace_id`, if there is one.
    pub fn connection_for_workspace(
        &self,
        workspace_id: EntityId,
    ) -> Option<&Entity<NreplConnection>> {
        self.connections.get(&workspace_id)
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut Context<Self>) {
        if self.enabled == enabled {
            return;
        }
        self.enabled = enabled;
        self.update_command_palette_filter(cx);
        if !enabled {
            // Flip-off tears everything down. Re-enabling does *not* auto-
            // reconnect: the user explicitly invokes `nrepl::Connect`
            // again, same as the initial connect path.
            self.connections.clear();
        }
        cx.notify();
    }

    fn update_command_palette_filter(&self, cx: &mut Context<Self>) {
        let enabled = self.enabled;
        CommandPaletteFilter::update_global(cx, |filter, _| {
            if enabled {
                filter.show_namespace(NAMESPACE);
            } else {
                filter.hide_namespace(NAMESPACE);
            }
        });
    }

    /// Replaces any existing connection for `workspace` with a new one
    /// targeted at `target`. The returned entity transitions through the
    /// states in [`ConnectionState`] as resolution and the TCP handshake
    /// progress; observers should drive re-renders off `cx.notify()` on
    /// the connection.
    pub fn connect(
        &mut self,
        workspace: WeakEntity<Workspace>,
        target: ConnectTarget,
        cx: &mut Context<Self>,
    ) -> Entity<NreplConnection> {
        let workspace_id = workspace.entity_id();

        // Drop the prior connection (if any) so its IO tasks tear down
        // before we issue a fresh `clone` op on a new socket. The detached
        // best-effort `:op "close"` runs in the background.
        self.disconnect(workspace_id, cx);

        let fs = self.fs.clone();
        let settings = NreplSettings::get_global(cx).clone();

        let connection = cx.new(|cx| NreplConnection::new(workspace, target, fs, settings, cx));
        self.connections.insert(workspace_id, connection.clone());
        cx.notify();
        connection
    }

    /// Drops the connection for `workspace_id`. Returns `true` if there
    /// was one.
    ///
    /// We send `{:op "close"}` as a best-effort courtesy so the server
    /// discards the session's `*1`/`*2`/`*3` bindings immediately rather
    /// than waiting for TCP teardown to deliver EOF. The detached close
    /// task ends as soon as the server replies (or the socket goes away).
    pub fn disconnect(&mut self, workspace_id: EntityId, cx: &mut Context<Self>) -> bool {
        let Some(connection) = self.connections.remove(&workspace_id) else {
            return false;
        };
        cx.notify();

        let close = cx.spawn(async move |_, cx| {
            // Snapshot the client + session id on the foreground so the
            // `:op "close"` itself can run without holding the App.
            let client_session = cx.update(|cx| match &connection.read(cx).state {
                ConnectionState::Connected {
                    client, session, ..
                } => Some((client.clone(), session.clone())),
                _ => None,
            });
            if let Some((client, session)) = client_session {
                let _: Option<()> = client.close_session(&session).await.log_err();
            }
            // `connection` drops here, releasing the last `NreplClient`
            // clone (if any) and tearing down the IO tasks.
            drop(connection);
        });
        close.detach();
        true
    }

    fn shutdown_all(&mut self, cx: &mut Context<Self>) -> impl Future<Output = ()> + use<> {
        // App is exiting: skip the polite `:op "close"` round-trip. The
        // server will see the TCP connection close and clean up sessions
        // shortly, which is good enough at quit time.
        self.connections.clear();
        cx.notify();
        futures::future::ready(())
    }
}

/// Where a [`NreplStore::connect`] call should aim.
#[derive(Clone, Debug)]
pub enum ConnectTarget {
    /// Discover via `.nrepl-port` in any visible local worktree root.
    /// First match wins (alphabetical isn't guaranteed; user-visible
    /// order is what we walk).
    Auto,
    /// Connect to an explicit address. The address is used verbatim — no
    /// discovery is attempted, and a failure is reported as-is.
    Address(SocketAddr),
}

pub struct NreplConnection {
    workspace: WeakEntity<Workspace>,
    target: ConnectTarget,
    state: ConnectionState,
    // Holding the task here ensures the connect work is cancelled if the
    // entity is dropped (e.g. user runs Connect again before the previous
    // attempt finishes).
    _connect_task: Option<Task<()>>,
}

#[derive(Clone)]
pub enum ConnectionState {
    /// Resolving an address (e.g. reading `.nrepl-port`).
    Resolving,
    /// TCP connect + initial `clone` op in flight.
    Connecting { addr: SocketAddr },
    /// Connected and ready to evaluate against `session`.
    Connected {
        client: NreplClient,
        addr: SocketAddr,
        port_file: Option<PathBuf>,
        session: String,
    },
    /// Connect attempt failed. The error string is rendered verbatim in
    /// the sessions panel and toasts; keep it informative (full anyhow
    /// chain) but on a single logical line.
    Failed { error: SharedString },
}

impl NreplConnection {
    fn new(
        workspace: WeakEntity<Workspace>,
        target: ConnectTarget,
        fs: Arc<dyn Fs>,
        settings: NreplSettings,
        cx: &mut Context<Self>,
    ) -> Self {
        // Capture the executor on the foreground side so the spawn closure
        // doesn't have to round-trip through `cx.update` to ask for it.
        let executor = cx.background_executor().clone();

        let task = cx.spawn({
            let workspace = workspace.clone();
            let target = target.clone();
            async move |this, cx| {
                connect_inner(this, workspace, target, fs, settings, executor, cx).await;
            }
        });
        Self {
            workspace,
            target,
            state: ConnectionState::Resolving,
            _connect_task: Some(task),
        }
    }

    pub fn workspace(&self) -> &WeakEntity<Workspace> {
        &self.workspace
    }

    pub fn target(&self) -> &ConnectTarget {
        &self.target
    }

    pub fn state(&self) -> &ConnectionState {
        &self.state
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.state, ConnectionState::Connected { .. })
    }

    /// Returns the underlying [`NreplClient`] when the connection is
    /// established. The client is cheap to clone — clones share the
    /// connection and request multiplexer.
    pub fn client(&self) -> Option<&NreplClient> {
        match &self.state {
            ConnectionState::Connected { client, .. } => Some(client),
            _ => None,
        }
    }

    /// The workspace's default nREPL session id, when connected.
    pub fn session(&self) -> Option<&str> {
        match &self.state {
            ConnectionState::Connected { session, .. } => Some(session.as_str()),
            _ => None,
        }
    }

    /// Address we connected (or are connecting) to.
    pub fn address(&self) -> Option<SocketAddr> {
        match &self.state {
            ConnectionState::Connecting { addr } => Some(*addr),
            ConnectionState::Connected { addr, .. } => Some(*addr),
            _ => None,
        }
    }

    /// Path to the `.nrepl-port` file we discovered, if any.
    pub fn port_file(&self) -> Option<&PathBuf> {
        match &self.state {
            ConnectionState::Connected { port_file, .. } => port_file.as_ref(),
            _ => None,
        }
    }
}

async fn connect_inner(
    this: WeakEntity<NreplConnection>,
    workspace: WeakEntity<Workspace>,
    target: ConnectTarget,
    fs: Arc<dyn Fs>,
    settings: NreplSettings,
    executor: gpui::BackgroundExecutor,
    cx: &mut AsyncApp,
) {
    let result: Result<(NreplClient, SocketAddr, Option<PathBuf>, String)> = async {
        let (addr, port_file) = match target {
            ConnectTarget::Auto => {
                // Snapshot worktree roots on the foreground; everything
                // else (file IO, TCP) runs without holding the App.
                let roots = workspace.read_with(cx, |workspace, cx| {
                    workspace
                        .project()
                        .read(cx)
                        .visible_worktrees(cx)
                        .filter_map(|w| {
                            let w = w.read(cx);
                            // Skip remote worktrees — v1 is localhost
                            // only, and `fs.load` against a remote
                            // worktree path would be the wrong fs anyway.
                            w.is_local().then(|| w.abs_path().to_path_buf())
                        })
                        .collect::<Vec<_>>()
                })?;

                let port: DiscoveredPort = discover_port(&fs, roots, &settings.port_file)
                    .await?
                    .ok_or_else(|| {
                        anyhow!(
                            "no `{}` found in any worktree; start your nREPL server first \
                             (e.g. `clj -M:nrepl`, `lein repl`, `bb nrepl-server`, shadow-cljs)",
                            settings.port_file
                        )
                    })?;
                (
                    port.socket_addr(&settings.default_host),
                    Some(port.port_file),
                )
            }
            ConnectTarget::Address(addr) => (addr, None),
        };

        this.update(cx, |this, cx| {
            this.state = ConnectionState::Connecting { addr };
            cx.notify();
        })?;

        let client = NreplClient::connect(addr, &executor)
            .await
            .with_context(|| format!("connecting to nREPL at {addr}"))?;
        let session = client
            .clone_session()
            .await
            .context("cloning initial nREPL session")?;

        Ok((client, addr, port_file, session))
    }
    .await;

    this.update(cx, |this, cx| {
        this.state = match result {
            Ok((client, addr, port_file, session)) => {
                log::info!("nrepl: connected to {addr} (session {session})");
                ConnectionState::Connected {
                    client,
                    addr,
                    port_file,
                    session,
                }
            }
            Err(err) => {
                log::warn!("nrepl: connect failed: {err:#}");
                ConnectionState::Failed {
                    error: format!("{err:#}").into(),
                }
            }
        };
        cx.notify();
    })
    .ok();
}
