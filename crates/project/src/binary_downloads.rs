//! Tracks the effective `allow_binary_downloads` setting per worktree and emits
//! events when it flips, so that subsystems that manage tool binaries
//! (language servers, prettier, debug adapters, MCP servers, copilot, npm
//! packages) can react and restart their bookkeeping without each subsystem
//! polling the [`settings::SettingsStore`] on every change.
//!
//! Modelled after [`crate::trusted_worktrees`]: there's a single global
//! [`BinaryDownloads`] entity, and each [`crate::Project`] (or `HeadlessProject`)
//! registers its [`WorktreeStore`] via [`track_binary_downloads`].

use anyhow::{Context as _, Result};
use client::ProjectId;
use collections::{HashMap, HashSet};
use futures::{
    StreamExt as _,
    channel::{mpsc, oneshot},
};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, SharedString, Subscription,
    WeakEntity,
};
use postage::{sink::Sink as _, stream::Stream as _, watch};
use rpc::{AnyProtoClient, proto};
use settings::{Settings as _, SettingsLocation, SettingsStore, WorktreeId};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use util::rel_path::RelPath;

use crate::{
    Project,
    project_settings::ProjectSettings,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

pub fn init(cx: &mut App) {
    if BinaryDownloads::try_get_global(cx).is_none() {
        let entity = cx.new(BinaryDownloadsStore::new);
        cx.set_global(BinaryDownloads(entity));
    }
}

pub fn track_binary_downloads(worktree_store: Entity<WorktreeStore>, cx: &mut App) {
    init(cx);
    let Some(entity) = BinaryDownloads::try_get_global(cx) else {
        return;
    };
    entity.update(cx, |store, cx| {
        store.add_worktree_store(worktree_store, cx);
    });
}

pub fn track_remote_binary_downloads(
    worktree_store: Entity<WorktreeStore>,
    upstream_client: (AnyProtoClient, ProjectId),
    cx: &mut App,
) {
    init(cx);
    let Some(entity) = BinaryDownloads::try_get_global(cx) else {
        return;
    };
    entity.update(cx, |store, cx| {
        store.add_upstream_client(worktree_store, upstream_client, cx);
    });
}

pub struct BinaryDownloads(pub Entity<BinaryDownloadsStore>);
impl Global for BinaryDownloads {}
impl BinaryDownloads {
    pub fn try_get_global(cx: &App) -> Option<Entity<BinaryDownloadsStore>> {
        cx.try_global::<Self>().map(|this| this.0.clone())
    }
}

/// Identifies a single one-off install prompt: a tool needed by a worktree
/// while downloads are disabled. `worktree_id` is `None` for tools that aren't
/// worktree-scoped.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ToolInstall {
    pub worktree_id: Option<WorktreeId>,
    pub tool: SharedString,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PendingToolInstall {
    pub install: ToolInstall,
    pub origin: ToolInstallOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ToolInstallOrigin {
    Local,
    Remote(WeakEntity<WorktreeStore>),
}

#[derive(Debug)]
pub enum BinaryDownloadsEvent {
    /// `allow_binary_downloads` flipped from `false` to `true` for the listed
    /// worktrees of the given store.
    Allowed(WeakEntity<WorktreeStore>, HashSet<WorktreeId>),
    /// `allow_binary_downloads` flipped from `true` to `false` for the listed
    /// worktrees of the given store.
    Disallowed(WeakEntity<WorktreeStore>, HashSet<WorktreeId>),
    InstallRequested(ToolInstall),
    InstallResolved(ToolInstall),
}

impl EventEmitter<BinaryDownloadsEvent> for BinaryDownloadsStore {}

pub struct BinaryDownloadsStore {
    approved_tools: HashSet<ToolInstall>,
    removed_worktrees: HashSet<WorktreeId>,
    generation: Arc<AtomicU64>,
    waiters: HashMap<ToolInstall, Waiter>,

    upstream_clients: HashMap<WeakEntity<WorktreeStore>, (AnyProtoClient, ProjectId)>,
    remote_pending: HashMap<WeakEntity<WorktreeStore>, Vec<ToolInstall>>,
    _worktree_subscriptions: HashMap<WeakEntity<WorktreeStore>, [Subscription; 2]>,
    _settings_subscription: Subscription,
}

struct Waiter {
    sender: watch::Sender<bool>,
    pending_install: bool,
}

impl BinaryDownloadsStore {
    fn new(cx: &mut Context<Self>) -> Self {
        let settings_subscription = cx.observe_global::<SettingsStore>(Self::on_settings_changed);
        Self {
            approved_tools: HashSet::default(),
            removed_worktrees: HashSet::default(),
            generation: Arc::new(AtomicU64::new(0)),
            waiters: HashMap::default(),

            upstream_clients: HashMap::default(),
            remote_pending: HashMap::default(),
            _worktree_subscriptions: HashMap::default(),
            _settings_subscription: settings_subscription,
        }
    }

    /// Returns a watch channel that yields `true` once `allow_binary_downloads`
    /// becomes `true` for the given scope. Pass `Some(worktree_id)` for a
    /// worktree-scoped wait or `None` to wait on the global default (for
    /// callers that have no worktree in hand, such as buffers backed by no
    /// file). Returns `None` when downloads are already allowed, so callers
    /// can early-out without spinning up a wait.
    ///
    /// This mirrors the worktree-trust wait so that subsystems that need to
    /// download a binary can `await` for approval instead of failing fast and
    /// being restarted later.
    pub fn wait_until_tool_allowed(
        &mut self,
        worktree_id: Option<WorktreeId>,
        tool: impl Into<SharedString>,
        cx: &App,
    ) -> Option<watch::Receiver<bool>> {
        if worktree_id.is_some_and(|id| self.removed_worktrees.contains(&id)) {
            return Some(watch::channel::<bool>().1);
        }
        let key = ToolInstall {
            worktree_id,
            tool: tool.into(),
        };
        if self.tool_download_allowed(key.worktree_id, key.tool.clone(), cx) {
            if let Some(mut waiter) = self.waiters.remove(&key) {
                waiter.sender.blocking_send(true).ok();
            }
            return None;
        }
        let waiter = self.waiters.entry(key).or_insert_with(|| Waiter {
            sender: watch::channel::<bool>().0,
            pending_install: false,
        });
        Some(waiter.sender.subscribe())
    }

    /// Returns the effective `allow_binary_downloads` value, optionally scoped
    /// to a particular worktree. Equivalent to reading [`ProjectSettings`]
    /// directly, but kept here so callers can route all binary-download trust
    /// queries through one place.
    pub fn allow_binary_downloads(worktree_id: Option<WorktreeId>, cx: &App) -> bool {
        let location = worktree_id.map(|worktree_id| SettingsLocation {
            worktree_id,
            path: RelPath::empty(),
        });
        ProjectSettings::get(location, cx).allow_binary_downloads
    }

    /// Returns `None` when the download may proceed immediately (downloads are
    /// already allowed, or this tool was previously approved). Otherwise
    /// returns a receiver that yields `true` once the user approves or the
    /// effective setting flips on.
    pub fn request_tool_install(
        &mut self,
        worktree_id: Option<WorktreeId>,
        tool: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> Option<watch::Receiver<bool>> {
        if worktree_id.is_some_and(|id| self.removed_worktrees.contains(&id)) {
            return Some(watch::channel::<bool>().1);
        }
        let key = ToolInstall {
            worktree_id,
            tool: tool.into(),
        };
        if self.tool_download_allowed(key.worktree_id, key.tool.clone(), cx) {
            if let Some(mut waiter) = self.waiters.remove(&key) {
                waiter.sender.blocking_send(true).ok();
            }
            return None;
        }

        let waiter = self.waiters.entry(key.clone()).or_insert_with(|| Waiter {
            sender: watch::channel::<bool>().0,
            pending_install: false,
        });
        let receiver = waiter.sender.subscribe();
        if !waiter.pending_install {
            waiter.pending_install = true;
            cx.emit(BinaryDownloadsEvent::InstallRequested(key));
            cx.notify();
        }
        Some(receiver)
    }

    pub fn approve_tool_install(
        &mut self,
        worktree_id: Option<WorktreeId>,
        tool: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) {
        let key = ToolInstall {
            worktree_id,
            tool: tool.into(),
        };
        self.invalidate_pending_responses();
        self.approved_tools.insert(key.clone());
        if let Some(mut waiter) = self.waiters.remove(&key) {
            waiter.sender.blocking_send(true).ok();
        }

        cx.emit(BinaryDownloadsEvent::InstallResolved(key));
        cx.notify();
    }

    pub fn approve_remote_tool_install(
        &self,
        worktree_store: &WeakEntity<WorktreeStore>,
        install: &ToolInstall,
    ) -> Result<()> {
        anyhow::ensure!(worktree_store.is_upgradable(), "Remote project is closed");
        anyhow::ensure!(
            self.remote_pending
                .get(worktree_store)
                .is_some_and(|pending| pending.contains(install)),
            "Remote install request is no longer pending"
        );
        let (client, project_id) = self
            .upstream_clients
            .get(worktree_store)
            .context("Remote project is not connected")?;
        client.send(proto::ApproveToolInstall {
            project_id: project_id.0,
            worktree_id: install.worktree_id.map(|id| id.to_proto()),
            tool: install.tool.to_string(),
        })
    }

    /// Read-only check: true when downloads are enabled for the scope or the
    /// user already approved this exact [`ToolInstall`]. Never prompts or
    /// registers waiters.
    pub fn tool_download_allowed(
        &self,
        worktree_id: Option<WorktreeId>,
        tool: impl Into<SharedString>,
        cx: &App,
    ) -> bool {
        if worktree_id.is_some_and(|id| self.removed_worktrees.contains(&id)) {
            return false;
        }
        if Self::allow_binary_downloads(worktree_id, cx) {
            return true;
        }
        let key = ToolInstall {
            worktree_id,
            tool: tool.into(),
        };
        self.approved_tools.contains(&key)
    }

    /// Tools that requested a one-off install while downloads are disabled and
    /// still await approval. Silent waiters are excluded.
    pub fn pending_tool_installs(&self) -> Vec<ToolInstall> {
        self.waiters
            .iter()
            .filter(|(_, waiter)| waiter.pending_install)
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn pending_tool_installs_for_project(
        &self,
        project: &Project,
        cx: &App,
    ) -> Vec<PendingToolInstall> {
        let worktree_store = project.worktree_store();
        let worktree_ids = worktree_store
            .read(cx)
            .worktrees()
            .map(|worktree| worktree.read(cx).id())
            .collect::<HashSet<_>>();
        let mut pending = self
            .pending_tool_installs()
            .into_iter()
            .filter(|install| {
                install.worktree_id.is_none_or(|worktree_id| {
                    !project.is_via_collab() && worktree_ids.contains(&worktree_id)
                })
            })
            .map(|install| PendingToolInstall {
                install,
                origin: ToolInstallOrigin::Local,
            })
            .collect::<Vec<_>>();
        if !project.is_via_collab() {
            let origin = worktree_store.downgrade();
            if let Some(installs) = self.remote_pending.get(&origin) {
                pending.extend(installs.iter().cloned().map(|install| PendingToolInstall {
                    install,
                    origin: ToolInstallOrigin::Remote(origin.clone()),
                }));
            }
        }
        pending.sort_by(|a, b| {
            a.install
                .tool
                .cmp(&b.install.tool)
                .then_with(|| {
                    (a.origin != ToolInstallOrigin::Local)
                        .cmp(&(b.origin != ToolInstallOrigin::Local))
                })
                .then_with(|| a.install.worktree_id.cmp(&b.install.worktree_id))
        });
        pending
    }

    pub fn set_remote_pending_installs(
        &mut self,
        worktree_store: WeakEntity<WorktreeStore>,
        pending: Vec<ToolInstall>,
        cx: &mut Context<Self>,
    ) {
        if worktree_store.is_upgradable() {
            self.remote_pending.insert(worktree_store, pending);
            cx.notify();
        }
    }

    fn add_worktree_store(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) {
        self._worktree_subscriptions
            .retain(|ws, _| ws.is_upgradable());

        let weak = worktree_store.downgrade();
        let subscription = cx.subscribe(&worktree_store, Self::on_worktree_store_event);
        let release_subscription = cx.observe_release(&worktree_store, {
            let weak = weak.clone();
            move |store, worktrees, cx| {
                for worktree in worktrees.worktrees() {
                    store.remove_worktree(worktree.read(cx).id());
                }
                store.upstream_clients.remove(&weak);
                store.remote_pending.remove(&weak);
                store._worktree_subscriptions.remove(&weak);
                cx.notify();
            }
        });
        self._worktree_subscriptions
            .insert(weak, [subscription, release_subscription]);
    }

    fn add_upstream_client(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        upstream_client: (AnyProtoClient, ProjectId),
        cx: &mut Context<Self>,
    ) {
        self.add_worktree_store(worktree_store.clone(), cx);
        self.upstream_clients
            .insert(worktree_store.downgrade(), upstream_client);
    }

    fn invalidate_pending_responses(&self) {
        self.generation
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |value| {
                value.checked_add(1)
            })
            .ok();
    }

    fn on_settings_changed(&mut self, cx: &mut Context<Self>) {
        self.invalidate_pending_responses();
        let unblocked_keys = self
            .waiters
            .keys()
            .filter(|key| self.tool_download_allowed(key.worktree_id, key.tool.clone(), cx))
            .cloned()
            .collect::<Vec<_>>();
        for key in unblocked_keys {
            if let Some(mut waiter) = self.waiters.remove(&key) {
                waiter.sender.blocking_send(true).ok();
                if waiter.pending_install {
                    cx.emit(BinaryDownloadsEvent::InstallResolved(key));
                }
            }
        }
        cx.notify();
    }

    pub fn node_downloads_allowed(&self, cx: &App) -> bool {
        Self::allow_binary_downloads(None, cx)
    }

    fn remove_worktree(&mut self, worktree_id: WorktreeId) {
        self.removed_worktrees.insert(worktree_id);
        self.invalidate_pending_responses();
        self.waiters
            .retain(|key, _| key.worktree_id != Some(worktree_id));
        self.approved_tools
            .retain(|key| key.worktree_id != Some(worktree_id));
    }

    fn on_worktree_store_event(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(_) => self.invalidate_pending_responses(),
            WorktreeStoreEvent::WorktreeRemoved(_, worktree_id)
            | WorktreeStoreEvent::WorktreeReleased(_, worktree_id) => {
                let origin = worktree_store.downgrade();
                if let Some(pending) = self.remote_pending.get_mut(&origin) {
                    pending.retain(|key| key.worktree_id != Some(*worktree_id));
                }
                self.remove_worktree(*worktree_id);
                cx.notify();
            }
            _ => {}
        }
    }
}

/// Looks up the global store and calls
/// [`BinaryDownloadsStore::request_tool_install`], returning `None` when the
/// download may proceed immediately.
pub fn request_tool_install(
    worktree_id: Option<WorktreeId>,
    tool: impl Into<SharedString>,
    cx: &mut App,
) -> Option<watch::Receiver<bool>> {
    let Some(store) = BinaryDownloads::try_get_global(cx) else {
        return Some(watch::channel::<bool>().1);
    };
    store.update(cx, |store, cx| {
        store.request_tool_install(worktree_id, tool, cx)
    })
}

pub fn tool_download_allowed(
    worktree_id: Option<WorktreeId>,
    tool: impl Into<SharedString>,
    cx: &App,
) -> bool {
    match BinaryDownloads::try_get_global(cx) {
        Some(store) => store.read(cx).tool_download_allowed(worktree_id, tool, cx),
        None => false,
    }
}

/// A `Send + Sync` handle for gating downloads from contexts that cannot hold
/// an [`gpui::AsyncApp`] (e.g. a `Send` `DapDelegate` or `node_runtime`). Call
/// [`DownloadGate::permit`] at the point a download happens. Backed by a
/// foreground task that runs [`request_tool_install`].
#[derive(Clone)]
pub struct DownloadGate {
    requests: mpsc::UnboundedSender<GateRequest>,
    generation: Arc<AtomicU64>,
}

enum GateResponse {
    Proceed,
    WaitForSetting(watch::Receiver<bool>),
    Blocked,
}

#[derive(Clone, Copy)]
enum GateMode {
    Query,
    Request,
    Silent,
}

struct GateRequest {
    tool: SharedString,
    mode: GateMode,
    respond: oneshot::Sender<(u64, GateResponse)>,
}

impl DownloadGate {
    /// Returns `None` when no binary-downloads store is installed.
    pub fn new(worktree_id: Option<WorktreeId>, cx: &mut App) -> Option<Self> {
        let store = BinaryDownloads::try_get_global(cx)?;
        Self::with_handler(
            move |tool, mode, cx| {
                store.update(cx, |store, cx| match mode {
                    GateMode::Query => {
                        if store.tool_download_allowed(worktree_id, tool, cx) {
                            GateResponse::Proceed
                        } else {
                            GateResponse::Blocked
                        }
                    }
                    GateMode::Silent => {
                        match store.wait_until_tool_allowed(worktree_id, tool, cx) {
                            Some(receiver) => GateResponse::WaitForSetting(receiver),
                            None => GateResponse::Proceed,
                        }
                    }
                    GateMode::Request => match store.request_tool_install(worktree_id, tool, cx) {
                        Some(_) => GateResponse::Blocked,
                        None => GateResponse::Proceed,
                    },
                })
            },
            cx,
        )
    }

    pub async fn is_allowed(&self, tool: &str) -> bool {
        matches!(
            self.request_permit(tool, GateMode::Query).await,
            GateResponse::Proceed
        )
    }

    pub async fn permit(&self, tool: &str) -> bool {
        !matches!(
            self.request_permit(tool, GateMode::Request).await,
            GateResponse::Blocked
        )
    }

    /// Never prompts: resolves immediately when the tool is already approved,
    /// else once `allow_binary_downloads` flips on. For background refreshes of
    /// tools with a working local copy.
    pub async fn permit_silent(&self, tool: &str) -> bool {
        match self.request_permit(tool, GateMode::Silent).await {
            GateResponse::Proceed => true,
            GateResponse::WaitForSetting(receiver) => {
                await_downloads_allowed(Some(receiver), tool).await && self.is_allowed(tool).await
            }
            GateResponse::Blocked => false,
        }
    }

    fn with_handler(
        handler: impl Fn(SharedString, GateMode, &mut App) -> GateResponse + 'static,
        cx: &mut App,
    ) -> Option<Self> {
        let store = BinaryDownloads::try_get_global(cx)?;
        let generation = store.read(cx).generation.clone();
        let (requests_tx, mut requests_rx) = mpsc::unbounded::<GateRequest>();
        cx.spawn(async move |cx| {
            while let Some(request) = requests_rx.next().await {
                let response = cx.update(|cx| {
                    let response = handler(request.tool, request.mode, cx);
                    (store.read(cx).generation.load(Ordering::SeqCst), response)
                });
                request.respond.send(response).ok();
            }
        })
        .detach();
        Some(Self {
            requests: requests_tx,
            generation,
        })
    }

    async fn request_permit(&self, tool: &str, mode: GateMode) -> GateResponse {
        loop {
            let (respond_tx, respond_rx) = oneshot::channel();
            let request = GateRequest {
                tool: tool.to_string().into(),
                mode,
                respond: respond_tx,
            };
            if self.requests.unbounded_send(request).is_err() {
                return GateResponse::Blocked;
            }
            let Ok((generation, response)) = respond_rx.await else {
                return GateResponse::Blocked;
            };
            if generation == u64::MAX {
                return GateResponse::Blocked;
            }
            if generation == self.generation.load(Ordering::SeqCst) {
                return response;
            }
        }
    }
}

pub fn npm_install_gate(cx: &mut App) -> Option<node_runtime::NpmInstallGate> {
    let gate = DownloadGate::new(None, cx)?;
    Some(std::sync::Arc::new(move |package| {
        let gate = gate.clone();
        futures::FutureExt::boxed(async move { gate.is_allowed(&package).await })
    }))
}

pub fn scoped_node_runtime(
    runtime: &node_runtime::NodeRuntime,
    worktree_id: Option<WorktreeId>,
    tool: impl Into<SharedString>,
    cx: &mut App,
) -> node_runtime::NodeRuntime {
    let tool = tool.into();
    let gate = DownloadGate::new(worktree_id, cx).map(|gate| {
        std::sync::Arc::new(move |_package: String| {
            let gate = gate.clone();
            let tool = tool.clone();
            futures::FutureExt::boxed(async move { gate.permit(&tool).await })
        }) as node_runtime::NpmInstallGate
    });
    runtime.with_install_gate(gate)
}

pub fn node_runtime_with_permission(
    runtime: &node_runtime::NodeRuntime,
    permission: impl Fn(&mut App) -> bool + 'static,
    cx: &mut App,
) -> node_runtime::NodeRuntime {
    let gate = DownloadGate::with_handler(
        move |_, _, cx| {
            if permission(cx) {
                GateResponse::Proceed
            } else {
                GateResponse::Blocked
            }
        },
        cx,
    );
    runtime.with_install_gate(gate.map(|gate| {
        Arc::new(move |package: String| {
            let gate = gate.clone();
            futures::FutureExt::boxed(async move { gate.permit(&package).await })
        }) as node_runtime::NpmInstallGate
    }))
}

pub fn node_downloads_allowed(cx: &App) -> bool {
    match BinaryDownloads::try_get_global(cx) {
        Some(store) => store.read(cx).node_downloads_allowed(cx),
        None => false,
    }
}

pub async fn await_downloads_allowed(
    wait: Option<watch::Receiver<bool>>,
    description: &str,
) -> bool {
    let Some(mut wait) = wait else {
        return true;
    };
    if *wait.borrow() {
        return true;
    }
    log::info!("Waiting for binary downloads approval before installing {description}");
    loop {
        match wait.recv().await {
            Some(true) => break,
            Some(false) => {}
            None => {
                log::info!("Binary downloads wait for {description} cancelled");
                return false;
            }
        }
    }
    log::info!("Binary downloads allowed, installing {description}");
    true
}
