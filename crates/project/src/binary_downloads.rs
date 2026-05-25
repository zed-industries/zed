//! Tracks the effective `allow_binary_downloads` setting per worktree and emits
//! events when it flips, so that subsystems that manage tool binaries
//! (language servers, prettier, debug adapters, MCP servers, copilot, npm
//! packages) can react and restart their bookkeeping without each subsystem
//! polling the [`settings::SettingsStore`] on every change.
//!
//! Modelled after [`crate::trusted_worktrees`]: there's a single global
//! [`BinaryDownloads`] entity, and each [`crate::Project`] (or `HeadlessProject`)
//! registers its [`WorktreeStore`] via [`track_binary_downloads`].

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
use util::{ResultExt as _, rel_path::RelPath};

use crate::{
    project_settings::ProjectSettings,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

pub fn init(cx: &mut App) {
    if BinaryDownloads::try_get_global(cx).is_none() {
        let entity = cx.new(BinaryDownloadsStore::new);
        cx.set_global(BinaryDownloads(entity));
    }
}

/// Registers `worktree_store` so that subsequent setting changes that flip
/// `allow_binary_downloads` for any of its worktrees emit a
/// [`BinaryDownloadsEvent`]. Safe to call on hosts where [`init`] was not
/// invoked: it just becomes a no-op.
pub fn track_binary_downloads(worktree_store: Entity<WorktreeStore>, cx: &mut App) {
    let Some(entity) = BinaryDownloads::try_get_global(cx) else {
        return;
    };
    entity.update(cx, |store, cx| {
        store.add_worktree_store(worktree_store, cx);
    });
}

/// Registers a remote project's worktree store so pending installs pushed from
/// the headless host are listed locally and approvals are forwarded upstream;
/// a no-op without [`init`].
pub fn track_remote_binary_downloads(
    worktree_store: Entity<WorktreeStore>,
    upstream_client: (AnyProtoClient, ProjectId),
    cx: &mut App,
) {
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
    waiters: HashMap<ToolInstall, Waiter>,
    worktree_stores: Vec<WeakEntity<WorktreeStore>>,
    upstream_clients: HashMap<WeakEntity<WorktreeStore>, (AnyProtoClient, ProjectId)>,
    remote_pending: HashMap<WeakEntity<WorktreeStore>, Vec<ToolInstall>>,
    _worktree_subscriptions: HashMap<WeakEntity<WorktreeStore>, Subscription>,
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
            waiters: HashMap::default(),
            worktree_stores: Vec::new(),
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
        self.approved_tools.insert(key.clone());
        if let Some(mut waiter) = self.waiters.remove(&key) {
            waiter.sender.blocking_send(true).ok();
        }
        for (worktree_store, pending) in self.remote_pending.iter_mut() {
            if let Some(position) = pending.iter().position(|install| install == &key) {
                pending.remove(position);
                if let Some((client, project_id)) = self.upstream_clients.get(worktree_store) {
                    client
                        .send(proto::ApproveToolInstall {
                            project_id: project_id.0,
                            worktree_id: key.worktree_id.map(|id| id.to_proto()),
                            tool: key.tool.to_string(),
                        })
                        .log_err();
                }
            }
        }
        cx.emit(BinaryDownloadsEvent::InstallResolved(key));
        cx.notify();
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
        let mut installs = self
            .waiters
            .iter()
            .filter(|(_, waiter)| waiter.pending_install)
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>();
        for install in self.remote_pending.values().flatten() {
            if !installs.contains(install) {
                installs.push(install.clone());
            }
        }
        installs
    }

    /// Replaces the pending installs mirrored for the given remote project's
    /// store, emitting events for the diff.
    pub fn set_remote_pending_installs(
        &mut self,
        worktree_store: WeakEntity<WorktreeStore>,
        pending: Vec<ToolInstall>,
        cx: &mut Context<Self>,
    ) {
        let previous = self
            .remote_pending
            .insert(worktree_store, pending.clone())
            .unwrap_or_default();
        for install in &previous {
            if !pending.contains(install) {
                cx.emit(BinaryDownloadsEvent::InstallResolved(install.clone()));
            }
        }
        for install in pending {
            if !previous.contains(&install) {
                cx.emit(BinaryDownloadsEvent::InstallRequested(install));
            }
        }
        cx.notify();
    }

    fn add_worktree_store(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) {
        self.worktree_stores.retain(|ws| ws.is_upgradable());
        self._worktree_subscriptions
            .retain(|ws, _| ws.is_upgradable());

        let weak = worktree_store.downgrade();
        let subscription = cx.subscribe(&worktree_store, Self::on_worktree_store_event);
        self.worktree_stores.push(weak.clone());
        self._worktree_subscriptions.insert(weak, subscription);
    }

    fn add_upstream_client(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        upstream_client: (AnyProtoClient, ProjectId),
        cx: &mut Context<Self>,
    ) {
        self.upstream_clients.retain(|ws, _| ws.is_upgradable());
        self.remote_pending.retain(|ws, _| ws.is_upgradable());
        self._worktree_subscriptions
            .retain(|ws, _| ws.is_upgradable());

        let weak = worktree_store.downgrade();
        let subscription = cx.subscribe(&worktree_store, Self::on_worktree_store_event);
        self.upstream_clients.insert(weak.clone(), upstream_client);
        self._worktree_subscriptions.insert(weak, subscription);
    }

    fn on_settings_changed(&mut self, cx: &mut Context<Self>) {
        let unblocked_keys = self
            .waiters
            .keys()
            .filter(|key| Self::allow_binary_downloads(key.worktree_id, cx))
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

    /// Second-line backstop for npm installs; first-line consent lives at the
    /// tool call sites. Passes once any consent exists, so approved tools are
    /// not double-prompted under their package names.
    pub fn npm_install_backstop_permitted(
        &self,
        worktree_id: Option<WorktreeId>,
        tool: impl Into<SharedString>,
        cx: &App,
    ) -> bool {
        self.tool_download_allowed(worktree_id, tool, cx)
            || self.node_downloads_allowed(cx)
            || !self.approved_tools.is_empty()
    }

    /// True when downloads are enabled in any scope. One-off tool approvals
    /// deliberately do not unlock the managed Node.js download.
    pub fn node_downloads_allowed(&self, cx: &App) -> bool {
        Self::allow_binary_downloads(None, cx)
            || self.worktree_stores.iter().any(|worktree_store| {
                worktree_store.upgrade().is_some_and(|worktree_store| {
                    worktree_store
                        .read(cx)
                        .visible_worktrees(cx)
                        .any(|worktree| {
                            Self::allow_binary_downloads(Some(worktree.read(cx).id()), cx)
                        })
                })
            })
    }

    fn on_worktree_store_event(
        &mut self,
        _: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeRemoved(_, worktree_id)
            | WorktreeStoreEvent::WorktreeReleased(_, worktree_id) => {
                self.waiters
                    .retain(|key, _| key.worktree_id != Some(*worktree_id));
                for pending in self.remote_pending.values_mut() {
                    pending.retain(|key| key.worktree_id != Some(*worktree_id));
                }
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
    BinaryDownloads::try_get_global(cx).and_then(|store| {
        store.update(cx, |store, cx| {
            store.request_tool_install(worktree_id, tool, cx)
        })
    })
}

pub fn tool_download_allowed(
    worktree_id: Option<WorktreeId>,
    tool: impl Into<SharedString>,
    cx: &App,
) -> bool {
    match BinaryDownloads::try_get_global(cx) {
        Some(store) => store.read(cx).tool_download_allowed(worktree_id, tool, cx),
        None => true,
    }
}

/// A `Send + Sync` handle for gating downloads from contexts that cannot hold
/// an [`gpui::AsyncApp`] (e.g. a `Send` `DapDelegate` or `node_runtime`). Call
/// [`DownloadGate::permit`] at the point a download happens. Backed by a
/// foreground task that runs [`request_tool_install`].
#[derive(Clone)]
pub struct DownloadGate {
    requests: mpsc::UnboundedSender<GateRequest>,
}

enum GateResponse {
    Proceed,
    WaitForSetting(watch::Receiver<bool>),
    Blocked,
}

enum GateMode {
    Request,
    Silent,
    Backstop,
}

struct GateRequest {
    tool: SharedString,
    mode: GateMode,
    respond: oneshot::Sender<GateResponse>,
}

impl DownloadGate {
    /// Returns `None` when no binary-downloads store is installed.
    pub fn new(worktree_id: Option<WorktreeId>, cx: &mut App) -> Option<Self> {
        let store = BinaryDownloads::try_get_global(cx)?;
        let (requests_tx, mut requests_rx) = mpsc::unbounded::<GateRequest>();
        cx.spawn(async move |cx| {
            while let Some(request) = requests_rx.next().await {
                let response = cx.update(|cx| {
                    store.update(cx, |store, cx| match request.mode {
                        GateMode::Silent => {
                            match store.wait_until_tool_allowed(worktree_id, request.tool, cx) {
                                Some(receiver) => GateResponse::WaitForSetting(receiver),
                                None => GateResponse::Proceed,
                            }
                        }
                        GateMode::Request => {
                            match store.request_tool_install(worktree_id, request.tool, cx) {
                                Some(_) => GateResponse::Blocked,
                                None => GateResponse::Proceed,
                            }
                        }
                        GateMode::Backstop => {
                            if store.npm_install_backstop_permitted(worktree_id, request.tool, cx) {
                                GateResponse::Proceed
                            } else {
                                GateResponse::Blocked
                            }
                        }
                    })
                });
                request.respond.send(response).ok();
            }
        })
        .detach();
        Some(Self {
            requests: requests_tx,
        })
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
                await_downloads_allowed(Some(receiver), tool).await
            }
            GateResponse::Blocked => false,
        }
    }

    pub async fn permit_backstop(&self, tool: &str) -> bool {
        !matches!(
            self.request_permit(tool, GateMode::Backstop).await,
            GateResponse::Blocked
        )
    }

    async fn request_permit(&self, tool: &str, mode: GateMode) -> GateResponse {
        let (respond_tx, respond_rx) = oneshot::channel();
        let request = GateRequest {
            tool: tool.to_string().into(),
            mode,
            respond: respond_tx,
        };
        if self.requests.unbounded_send(request).is_err() {
            return GateResponse::Blocked;
        }
        respond_rx.await.unwrap_or(GateResponse::Blocked)
    }
}

pub fn npm_install_gate(cx: &mut App) -> Option<node_runtime::NpmInstallGate> {
    let gate = DownloadGate::new(None, cx)?;
    Some(std::sync::Arc::new(move |package| {
        let gate = gate.clone();
        futures::FutureExt::boxed(async move { gate.permit_backstop(&package).await })
    }))
}

pub fn node_downloads_allowed(cx: &App) -> bool {
    match BinaryDownloads::try_get_global(cx) {
        Some(store) => store.read(cx).node_downloads_allowed(cx),
        None => ProjectSettings::get_global(cx).allow_binary_downloads,
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
