use collections::{HashMap, HashSet};
use futures::StreamExt as _;
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, Subscription, Task, WeakEntity,
};
pub use http_client::download_gate::{
    BinaryDownload, COPILOT, DownloadGate, NODE_RUNTIME, PRETTIER,
};
use rpc::{AnyProtoClient, proto};
use settings::{RegisterSetting, Settings, SettingsStore};

use crate::{Project, trusted_worktrees::RemoteHostLocation, worktree_store::WorktreeStore};

pub type DbBinaryDownloads = HashMap<Option<RemoteHostLocation>, HashSet<BinaryDownload>>;

pub fn init(db_downloads: DbBinaryDownloads, cx: &mut App) -> DownloadGate {
    if let Some(store) = BinaryDownloads::try_get_global(cx) {
        return store.read(cx).gate.clone();
    }
    let store = cx.new(|cx| BinaryDownloadsStore::new(db_downloads, cx));
    let gate = store.read(cx).gate.clone();
    cx.set_global(BinaryDownloads(store));
    gate
}

pub fn track_remote_binary_downloads(
    worktree_store: &Entity<WorktreeStore>,
    host: RemoteHostLocation,
    upstream_client: AnyProtoClient,
    cx: &mut App,
) {
    if let Some(store) = BinaryDownloads::try_get_global(cx) {
        store.update(cx, |store, _| {
            store.add_remote_project(worktree_store.downgrade(), host, upstream_client);
        });
    }
}

pub fn resync_remote_binary_downloads(worktree_store: &Entity<WorktreeStore>, cx: &mut App) {
    if let Some(store) = BinaryDownloads::try_get_global(cx) {
        store
            .read(cx)
            .send_allowed_upstream(&worktree_store.downgrade());
    }
}

pub fn download_gate(cx: &App) -> DownloadGate {
    BinaryDownloads::try_get_global(cx)
        .map(|store| store.read(cx).gate.clone())
        .unwrap_or_else(DownloadGate::deny_all)
}

pub fn pending_downloads(project: &Project, cx: &App) -> Vec<BinaryDownload> {
    BinaryDownloads::try_get_global(cx)
        .map(|store| store.read(cx).pending_downloads(project))
        .unwrap_or_default()
}

pub fn allow_for_project(project: &Entity<Project>, download: BinaryDownload, cx: &mut App) {
    if let Some(store) = BinaryDownloads::try_get_global(cx) {
        store.update(cx, |store, cx| store.allow_download(project, download, cx));
    }
}

#[derive(Clone, Copy, Debug, RegisterSetting)]
pub struct BinaryDownloadsSettings {
    /// Whether Zed may download tools without asking first.
    ///
    /// Default: false
    pub allow_binary_downloads: bool,
}

impl Settings for BinaryDownloadsSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            allow_binary_downloads: content.allow_binary_downloads.unwrap(),
        }
    }
}

pub struct BinaryDownloads(Entity<BinaryDownloadsStore>);

impl Global for BinaryDownloads {}

impl BinaryDownloads {
    pub fn try_get_global(cx: &App) -> Option<Entity<BinaryDownloadsStore>> {
        cx.try_global::<Self>().map(|global| global.0.clone())
    }
}

#[derive(Debug)]
pub enum BinaryDownloadsEvent {
    Allowed(BinaryDownload),
    Cleared,
}

impl EventEmitter<BinaryDownloadsEvent> for BinaryDownloadsStore {}

pub struct BinaryDownloadsStore {
    gate: DownloadGate,
    allowed: DbBinaryDownloads,
    remote_projects: Vec<RemoteProject>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    serialization: Task<()>,
    _settings_subscription: Subscription,
    _gate_changes: Task<()>,
}

impl BinaryDownloadsStore {
    pub fn pending_downloads(&self, project: &Project) -> Vec<BinaryDownload> {
        let mut pending = self.gate.pending();
        if let Some(remote_project) = self.remote_project(&project.worktree_store().downgrade()) {
            pending.extend(remote_project.pending.iter().cloned());
        }
        pending.sort();
        pending.dedup();
        pending
    }

    pub fn allow_download(
        &mut self,
        project: &Entity<Project>,
        download: BinaryDownload,
        cx: &mut Context<Self>,
    ) {
        self.decide(project, download, true, cx);
    }

    pub fn deny_download(
        &mut self,
        project: &Entity<Project>,
        download: BinaryDownload,
        cx: &mut Context<Self>,
    ) {
        self.decide(project, download, false, cx);
    }

    pub fn clear_allowed_downloads(&mut self, cx: &mut Context<Self>) {
        self.allowed.clear();
        self.gate.clear_allowed();
        cx.emit(BinaryDownloadsEvent::Cleared);
        cx.notify();
    }

    pub fn set_remote_pending(
        &mut self,
        worktree_store: &WeakEntity<WorktreeStore>,
        pending: Vec<BinaryDownload>,
        cx: &mut Context<Self>,
    ) {
        if let Some(remote_project) = self
            .remote_projects
            .iter_mut()
            .find(|remote_project| &remote_project.worktree_store == worktree_store)
        {
            remote_project.pending = pending;
            cx.notify();
        }
    }

    pub fn allow_from_downstream(&mut self, downloads: Vec<BinaryDownload>) {
        for download in downloads {
            self.gate.allow(download);
        }
        self.send_pending_downstream();
    }

    pub fn deny_from_downstream(&mut self, downloads: Vec<BinaryDownload>) {
        for download in downloads {
            self.gate.deny(download);
        }
        self.send_pending_downstream();
    }

    pub fn set_downstream_client(&mut self, client: AnyProtoClient, project_id: u64) {
        self.downstream_client = Some((client, project_id));
        self.send_pending_downstream();
    }

    pub fn schedule_serialization<S>(&mut self, cx: &mut Context<Self>, serialize: S)
    where
        S: FnOnce(DbBinaryDownloads, &App) -> Task<()>,
    {
        self.serialization = serialize(self.allowed.clone(), cx);
    }

    fn new(allowed: DbBinaryDownloads, cx: &mut Context<Self>) -> Self {
        let (gate, mut changes) = DownloadGate::new(
            BinaryDownloadsSettings::get_global(cx).allow_binary_downloads,
            allowed.get(&None).into_iter().flatten().cloned(),
        );
        let settings_subscription = cx.observe_global::<SettingsStore>(|store, cx| {
            store
                .gate
                .set_allow_all(BinaryDownloadsSettings::get_global(cx).allow_binary_downloads);
        });
        let gate_changes = cx.spawn(async move |store, cx| {
            while changes.next().await.is_some() {
                let updated = store.update(cx, |store, cx| {
                    let pending = store.gate.pending();
                    if !pending.is_empty() {
                        log::info!(
                            "Waiting for approval to download {}",
                            pending
                                .iter()
                                .map(|download| download.tool.as_ref())
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    }
                    store.send_pending_downstream();
                    cx.notify();
                });
                if updated.is_err() {
                    break;
                }
            }
        });
        Self {
            gate,
            allowed,
            remote_projects: Vec::new(),
            downstream_client: None,
            serialization: Task::ready(()),
            _settings_subscription: settings_subscription,
            _gate_changes: gate_changes,
        }
    }

    fn decide(
        &mut self,
        project: &Entity<Project>,
        download: BinaryDownload,
        allow: bool,
        cx: &mut Context<Self>,
    ) {
        let worktree_store = project.read(cx).worktree_store().downgrade();
        let pending_locally = self.gate.pending().contains(&download);
        let remote_project = self
            .remote_projects
            .iter_mut()
            .find(|remote_project| remote_project.worktree_store == worktree_store);
        let decide_locally = remote_project.is_none() || pending_locally;
        if let Some(remote_project) = remote_project
            && (remote_project.pending.contains(&download) || !pending_locally)
        {
            remote_project
                .pending
                .retain(|pending_download| pending_download != &download);
            let tools = vec![download.tool.to_string()];
            let project_id = proto::REMOTE_SERVER_PROJECT_ID;
            if allow {
                remote_project
                    .upstream_client
                    .send(proto::AllowBinaryDownloads { project_id, tools })
                    .ok();
                self.allowed
                    .entry(Some(remote_project.host.clone()))
                    .or_default()
                    .insert(download.clone());
            } else {
                remote_project
                    .upstream_client
                    .send(proto::DenyBinaryDownloads { project_id, tools })
                    .ok();
            }
        }
        if decide_locally {
            if allow {
                self.gate.allow(download.clone());
                self.allowed
                    .entry(None)
                    .or_default()
                    .insert(download.clone());
            } else {
                self.gate.deny(download.clone());
            }
        }
        if allow {
            cx.emit(BinaryDownloadsEvent::Allowed(download));
        }
        cx.notify();
    }

    fn remote_project(&self, worktree_store: &WeakEntity<WorktreeStore>) -> Option<&RemoteProject> {
        self.remote_projects
            .iter()
            .find(|remote_project| &remote_project.worktree_store == worktree_store)
    }

    fn add_remote_project(
        &mut self,
        worktree_store: WeakEntity<WorktreeStore>,
        host: RemoteHostLocation,
        upstream_client: AnyProtoClient,
    ) {
        self.remote_projects.retain(|remote_project| {
            remote_project.worktree_store.is_upgradable()
                && remote_project.worktree_store != worktree_store
        });
        self.remote_projects.push(RemoteProject {
            worktree_store: worktree_store.clone(),
            host,
            upstream_client,
            pending: Vec::new(),
        });
        self.send_allowed_upstream(&worktree_store);
    }

    fn send_allowed_upstream(&self, worktree_store: &WeakEntity<WorktreeStore>) {
        let Some(remote_project) = self.remote_project(worktree_store) else {
            return;
        };
        remote_project
            .upstream_client
            .send(proto::AllowBinaryDownloads {
                project_id: proto::REMOTE_SERVER_PROJECT_ID,
                tools: self
                    .allowed
                    .get(&Some(remote_project.host.clone()))
                    .into_iter()
                    .flatten()
                    .map(|download| download.tool.to_string())
                    .collect(),
            })
            .ok();
    }

    fn send_pending_downstream(&self) {
        if let Some((client, project_id)) = &self.downstream_client {
            client
                .send(proto::UpdatePendingBinaryDownloads {
                    project_id: *project_id,
                    tools: self
                        .gate
                        .pending()
                        .iter()
                        .map(|download| download.tool.to_string())
                        .collect(),
                })
                .ok();
        }
    }
}

struct RemoteProject {
    worktree_store: WeakEntity<WorktreeStore>,
    host: RemoteHostLocation,
    upstream_client: AnyProtoClient,
    pending: Vec<BinaryDownload>,
}
