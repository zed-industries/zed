use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use collections::{HashMap, HashSet};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, SharedString, Task, WeakEntity,
};
use remote::RemoteConnectionOptions;
use rpc::{AnyProtoClient, proto};
use settings::{Settings as _, WorktreeId};
use util::{ResultExt as _, debug_panic};

use crate::{
    persistence::PROJECT_DB, project_settings::ProjectSettings, worktree_store::WorktreeStore,
};

pub fn init_global(
    worktree_store: Entity<WorktreeStore>,
    remote_host: Option<impl Into<RemoteHostLocation> + 'static>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    upstream_client: Option<(AnyProtoClient, u64)>,
    cx: &mut App,
) {
    match TrustedWorktrees::try_get_global(cx) {
        Some(trusted_worktrees) => {
            trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                trusted_worktrees.add_worktree_store(worktree_store, remote_host, cx);
            });
        }
        None => {
            let trusted_worktrees = cx.new(|cx| {
                TrustedWorktreesStorage::new(
                    worktree_store.clone(),
                    remote_host,
                    downstream_client,
                    upstream_client,
                    cx,
                )
            });
            cx.set_global(TrustedWorktrees(trusted_worktrees))
        }
    }
}

pub fn wait_for_worktree_trust(
    remote_host: Option<impl Into<RemoteHostLocation>>,
    cx: &mut App,
) -> Option<Task<()>> {
    let trusted_worktrees = TrustedWorktrees::try_get_global(cx)?;
    let remote_host = remote_host.map(|host| host.into());

    let remote_host = if trusted_worktrees.update(cx, |trusted_worktrees, cx| {
        trusted_worktrees.can_trust_global(remote_host.clone(), cx)
    }) {
        None
    } else {
        Some(remote_host)
    }?;

    Some(cx.spawn(async move |cx| {
        log::info!("Waiting for global startup to be trusted before starting context servers");
        let (tx, restricted_worktrees_task) = smol::channel::bounded::<()>(1);
        let Ok(_subscription) = cx.update(|cx| {
            cx.subscribe(&trusted_worktrees, move |_, e, _| {
                if let TrustedWorktreesEvent::Trusted(trusted_paths) = e {
                    if trusted_paths.contains(&PathTrust::Global(remote_host.clone())) {
                        tx.send_blocking(()).ok();
                    }
                }
            })
        }) else {
            return;
        };

        restricted_worktrees_task.recv().await.ok();
    }))
}

pub struct TrustedWorktrees(Entity<TrustedWorktreesStorage>);

impl Global for TrustedWorktrees {}

impl TrustedWorktrees {
    pub fn try_get_global(cx: &App) -> Option<Entity<TrustedWorktreesStorage>> {
        cx.try_global::<Self>().map(|this| this.0.clone())
    }
}

/// A collection of worktrees that are considered trusted and not trusted.
/// This can be used when checking for this criteria before enabling certain features.
///
/// Emits an event each time the worktree was checked and found not trusted,
/// or a certain worktree had been trusted.
pub struct TrustedWorktreesStorage {
    downstream_client: Option<(AnyProtoClient, u64)>,
    upstream_client: Option<(AnyProtoClient, u64)>,
    worktree_stores: HashMap<WeakEntity<WorktreeStore>, Option<RemoteHostLocation>>,
    trusted_paths: HashSet<PathTrust>,
    serialization_task: Task<()>,
    restricted: HashSet<WorktreeId>,
    remote_host: Option<RemoteHostLocation>,
    restricted_globals: HashSet<Option<RemoteHostLocation>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RemoteHostLocation {
    pub user_name: Option<SharedString>,
    pub host_name: SharedString,
}

impl From<RemoteConnectionOptions> for RemoteHostLocation {
    fn from(options: RemoteConnectionOptions) -> Self {
        let (user_name, host_name) = match options {
            RemoteConnectionOptions::Ssh(ssh) => (
                ssh.username.map(SharedString::new),
                SharedString::new(ssh.host),
            ),
            RemoteConnectionOptions::Wsl(wsl) => (
                wsl.user.map(SharedString::new),
                SharedString::new(wsl.distro_name),
            ),
        };
        RemoteHostLocation {
            user_name,
            host_name,
        }
    }
}

/// A unit of trust consideration: either a familiar worktree, or a path that may
/// influence other worktrees' trust.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PathTrust {
    /// General, no worktrees or files open case.
    Global(Option<RemoteHostLocation>),
    /// A worktree that is familiar to this workspace.
    Worktree(WorktreeId),
    /// A path that may be another worktree yet not loaded into workspace,
    /// or a parent path coming out of the security modal.
    AbsPath(PathBuf, Option<RemoteHostLocation>),
}

impl PathTrust {
    fn to_proto(&self, remote_host: Option<&RemoteHostLocation>) -> Option<proto::PathTrust> {
        match self {
            Self::Global(host) => {
                if host.as_ref() == remote_host {
                    Some(proto::PathTrust {
                        content: Some(proto::path_trust::Content::Global(0)),
                    })
                } else {
                    None
                }
            }
            Self::Worktree(worktree_id) => Some(proto::PathTrust {
                content: Some(proto::path_trust::Content::WorktreeId(
                    worktree_id.to_proto(),
                )),
            }),
            Self::AbsPath(path_buf, host) => {
                if host.as_ref() == remote_host {
                    Some(proto::PathTrust {
                        content: Some(proto::path_trust::Content::AbsPath(
                            path_buf.to_string_lossy().to_string(),
                        )),
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn from_proto(
        proto: proto::PathTrust,
        remote_host: Option<&RemoteHostLocation>,
    ) -> Option<Self> {
        Some(match proto.content? {
            proto::path_trust::Content::WorktreeId(id) => {
                Self::Worktree(WorktreeId::from_proto(id))
            }
            proto::path_trust::Content::AbsPath(path) => {
                Self::AbsPath(PathBuf::from(path), remote_host.cloned())
            }
            proto::path_trust::Content::Global(_) => Self::Global(remote_host.cloned()),
        })
    }
}

#[derive(Debug)]
pub enum TrustedWorktreesEvent {
    Trusted(HashSet<PathTrust>),
    Restricted(HashSet<PathTrust>),
}

impl EventEmitter<TrustedWorktreesEvent> for TrustedWorktreesStorage {}

impl TrustedWorktreesStorage {
    fn new(
        worktree_store: Entity<WorktreeStore>,
        remote_host: Option<impl Into<RemoteHostLocation>>,
        downstream_client: Option<(AnyProtoClient, u64)>,
        upstream_client: Option<(AnyProtoClient, u64)>,
        cx: &App,
    ) -> Self {
        let remote_host = remote_host.map(|remote_host| remote_host.into());
        let trusted_paths = if downstream_client.is_none() {
            match PROJECT_DB.fetch_trusted_worktrees(
                worktree_store.clone(),
                remote_host.clone(),
                cx,
            ) {
                Ok(trusted_paths) => trusted_paths,
                Err(e) => {
                    log::error!("Failed to do initial trusted worktrees fetch: {e:#}");
                    HashSet::default()
                }
            }
        } else {
            HashSet::default()
        };
        let restricted_globals = if trusted_paths.contains(&PathTrust::Global(remote_host.clone()))
        {
            HashSet::default()
        } else {
            HashSet::from_iter([remote_host.clone()])
        };

        if let Some((upstream_client, upstream_project_id)) = &upstream_client {
            let trusted_paths = trusted_paths
                .iter()
                .filter_map(|trusted_path| trusted_path.to_proto(remote_host.as_ref()))
                .collect::<Vec<_>>();
            if !trusted_paths.is_empty() {
                upstream_client
                    .send(proto::TrustWorktrees {
                        project_id: *upstream_project_id,
                        trusted_paths,
                    })
                    .ok();
            }

            if restricted_globals.contains(&remote_host) {
                upstream_client
                    .send(proto::RestrictWorktrees {
                        project_id: *upstream_project_id,
                        restrict_global: true,
                        worktree_ids: Vec::new(),
                    })
                    .ok();
            }
        }

        Self {
            trusted_paths,
            restricted_globals,
            downstream_client,
            upstream_client,
            remote_host: remote_host.clone(),
            restricted: HashSet::default(),
            serialization_task: Task::ready(()),
            worktree_stores: HashMap::from_iter([(worktree_store.downgrade(), remote_host)]),
        }
    }

    pub fn has_restricted_worktrees(&self) -> bool {
        !self.restricted.is_empty() || !self.restricted_globals.is_empty()
    }

    /// Adds worktree absolute paths to the trusted list.
    /// This will emit [`TrustedWorktreesEvent::Trusted`] event.
    pub fn trust(&mut self, mut trusted_paths: HashSet<PathTrust>, cx: &mut Context<Self>) {
        // TODO kb unit test all this logic

        for trusted_path in &trusted_paths {
            match trusted_path {
                PathTrust::Worktree(worktree_id) => {
                    self.restricted.remove(worktree_id);
                    self.trusted_paths.insert(PathTrust::Worktree(*worktree_id));
                }
                PathTrust::AbsPath(path, host) => {
                    debug_assert!(
                        path.is_absolute(),
                        "Cannot trust non-absolute path {path:?}"
                    );

                    let mut worktree_found = false;
                    self.worktree_stores
                        .retain(
                            |worktree_store, remote_host| match worktree_store.upgrade() {
                                Some(worktree_store) => {
                                    if remote_host == host {
                                        if let Some(worktree_id) = find_worktree_in_store(
                                            worktree_store.read(cx),
                                            &path,
                                            cx,
                                        ) {
                                            self.restricted.remove(&worktree_id);
                                            self.trusted_paths
                                                .insert(PathTrust::Worktree(worktree_id));
                                            worktree_found = true;
                                        }
                                    }

                                    true
                                }
                                None => false,
                            },
                        );

                    if !worktree_found {
                        let previous_restricted = std::mem::take(&mut self.restricted);
                        self.restricted = previous_restricted
                            .into_iter()
                            .filter(|restricted_worktree| {
                                let Some((restricted_worktree_path, _, restricted_host)) =
                                    self.find_worktree_data(*restricted_worktree, cx)
                                else {
                                    return false;
                                };
                                if &restricted_host != host {
                                    return true;
                                }
                                !restricted_worktree_path.starts_with(path)
                            })
                            .collect();
                        self.trusted_paths
                            .retain(|trusted_path| match trusted_path {
                                PathTrust::Global(_) | PathTrust::Worktree(_) => true,
                                PathTrust::AbsPath(trusted_abs_path, trusted_host) => {
                                    if trusted_host != host {
                                        return true;
                                    }
                                    !trusted_abs_path.starts_with(path)
                                }
                            });
                        self.trusted_paths
                            .insert(PathTrust::AbsPath(path.clone(), host.clone()));
                    }
                }
                PathTrust::Global(host) => {
                    self.restricted_globals.remove(host);
                    self.trusted_paths.insert(PathTrust::Global(host.clone()));
                }
            }
        }

        let mut new_trusted_globals = HashSet::default();
        let new_trusted_worktrees =
            self.trusted_paths
                .clone()
                .into_iter()
                .fold(HashMap::default(), |mut acc, path| {
                    if let Some((abs_path, remote_host)) = match path {
                        PathTrust::Worktree(worktree_id) => self
                            .find_worktree_data(worktree_id, cx)
                            .and_then(|(abs_path, is_file, remote_host)| {
                                Some(if is_file {
                                    (abs_path.parent()?.to_path_buf(), remote_host)
                                } else {
                                    (abs_path.to_path_buf(), remote_host)
                                })
                            }),
                        PathTrust::AbsPath(abs_path, remote_host) => Some((abs_path, remote_host)),
                        PathTrust::Global(host) => {
                            new_trusted_globals.insert(host);
                            None
                        }
                    } {
                        new_trusted_globals.insert(remote_host.clone());
                        acc.insert(abs_path, remote_host);
                    }
                    acc
                });

        // Trusting a local worktree means trusting the global cases around it too.
        trusted_paths.extend(
            new_trusted_globals
                .clone()
                .into_iter()
                .map(PathTrust::Global),
        );
        cx.emit(TrustedWorktreesEvent::Trusted(trusted_paths.clone()));

        if self.downstream_client.is_none() {
            self.serialization_task = cx.background_spawn(async move {
                PROJECT_DB
                    .save_trusted_worktrees(new_trusted_worktrees, new_trusted_globals)
                    .await
                    .log_err();
            });
            if let Some((upstream_client, upstream_project_id)) = &self.upstream_client {
                let trusted_paths = trusted_paths
                    .iter()
                    .filter_map(|trusted_path| trusted_path.to_proto(self.remote_host.as_ref()))
                    .collect::<Vec<_>>();
                if !trusted_paths.is_empty() {
                    upstream_client
                        .send(proto::TrustWorktrees {
                            project_id: *upstream_project_id,
                            trusted_paths,
                        })
                        .ok();
                }
            }
        }
    }

    pub fn restrict(&mut self, restricted_paths: HashSet<PathTrust>, cx: &mut Context<Self>) {
        for restricted_path in restricted_paths {
            match restricted_path {
                PathTrust::Global(remote_host_location) => {
                    self.restricted_globals.insert(remote_host_location.clone());
                    cx.emit(TrustedWorktreesEvent::Restricted(HashSet::from_iter([
                        PathTrust::Global(remote_host_location),
                    ])));
                }
                PathTrust::Worktree(worktree_id) => {
                    self.restricted.insert(worktree_id);
                    cx.emit(TrustedWorktreesEvent::Restricted(HashSet::from_iter([
                        PathTrust::Worktree(worktree_id),
                    ])));
                }
                PathTrust::AbsPath(..) => debug_panic!("Unexpected: cannot restrict an abs path"),
            }
        }
    }

    pub fn clear_trusted_paths(&mut self, cx: &App) -> Task<()> {
        if self.downstream_client.is_none() {
            self.trusted_paths.clear();
            let (tx, rx) = smol::channel::bounded(1);

            self.serialization_task = cx.background_spawn(async move {
                PROJECT_DB.clear_trusted_worktrees().await.log_err();
                tx.send(()).await.ok();
            });

            cx.background_spawn(async move {
                rx.recv().await.ok();
            })
        } else {
            Task::ready(())
        }
    }

    /// Checks whether a certain worktree is trusted.
    /// If not, emits [`TrustedWorktreesEvent::Restricted`] event.
    pub fn can_trust(&mut self, worktree_id: WorktreeId, cx: &mut Context<Self>) -> bool {
        if ProjectSettings::get_global(cx).session.trust_all_worktrees {
            return true;
        }
        if self.restricted.contains(&worktree_id) {
            return false;
        }
        if self
            .trusted_paths
            .contains(&PathTrust::Worktree(worktree_id))
        {
            return true;
        }

        if let Some((worktree_path, _, remote_host)) = self.find_worktree_data(worktree_id, cx) {
            for trusted_path in &self.trusted_paths {
                let PathTrust::AbsPath(trusted_path, trusted_remote_host) = trusted_path else {
                    continue;
                };
                if &remote_host != trusted_remote_host {
                    continue;
                }
                if worktree_path.starts_with(trusted_path) {
                    return true;
                }
            }
        }

        self.restricted.insert(worktree_id);
        cx.emit(TrustedWorktreesEvent::Restricted(HashSet::from_iter([
            PathTrust::Worktree(worktree_id),
        ])));
        if let Some((downstream_client, downstream_project_id)) = &self.downstream_client {
            downstream_client
                .send(proto::RestrictWorktrees {
                    project_id: *downstream_project_id,
                    restrict_global: false,
                    worktree_ids: vec![worktree_id.to_proto()],
                })
                .ok();
        }
        if let Some((upstream_client, upstream_project_id)) = &self.upstream_client {
            upstream_client
                .send(proto::RestrictWorktrees {
                    project_id: *upstream_project_id,
                    restrict_global: false,
                    worktree_ids: vec![worktree_id.to_proto()],
                })
                .ok();
        }
        false
    }

    pub fn can_trust_global(
        &mut self,
        remote_host: Option<impl Into<RemoteHostLocation>>,
        cx: &mut Context<Self>,
    ) -> bool {
        if ProjectSettings::get_global(cx).session.trust_all_worktrees {
            return true;
        }
        let remote_host = remote_host.map(|remote_host| remote_host.into());
        if self.restricted_globals.contains(&remote_host) {
            return false;
        }
        if self
            .trusted_paths
            .contains(&PathTrust::Global(remote_host.clone()))
        {
            return true;
        }

        self.restricted_globals.insert(remote_host.clone());
        cx.emit(TrustedWorktreesEvent::Restricted(HashSet::from_iter([
            PathTrust::Global(remote_host.clone()),
        ])));

        if remote_host == self.remote_host {
            if let Some((downstream_client, downstream_project_id)) = &self.downstream_client {
                downstream_client
                    .send(proto::RestrictWorktrees {
                        project_id: *downstream_project_id,
                        restrict_global: true,
                        worktree_ids: Vec::new(),
                    })
                    .ok();
            }
            if let Some((upstream_client, upstream_project_id)) = &self.upstream_client {
                upstream_client
                    .send(proto::RestrictWorktrees {
                        project_id: *upstream_project_id,
                        restrict_global: true,
                        worktree_ids: Vec::new(),
                    })
                    .ok();
            }
        }
        false
    }

    pub fn restricted_paths(
        &self,
        worktree_store: &WorktreeStore,
        remote_host: Option<impl Into<RemoteHostLocation>>,
        cx: &App,
    ) -> HashSet<Option<(WorktreeId, Arc<Path>)>> {
        let mut restricted_paths = self
            .restricted
            .iter()
            .filter_map(|&restricted_worktree_id| {
                let worktree = worktree_store.worktree_for_id(restricted_worktree_id, cx)?;
                Some((restricted_worktree_id, worktree.read(cx).abs_path()))
            })
            .map(Some)
            .collect::<HashSet<_>>();
        if self
            .restricted_globals
            .contains(&remote_host.map(|remote_host| remote_host.into()))
        {
            restricted_paths.insert(None);
        }
        restricted_paths
    }

    pub fn trust_all(&mut self, cx: &mut Context<Self>) {
        let restricted = std::mem::take(&mut self.restricted)
            .into_iter()
            .map(PathTrust::Worktree)
            .chain(
                std::mem::take(&mut self.restricted_globals)
                    .into_iter()
                    .map(PathTrust::Global),
            )
            .collect();
        self.trust(restricted, cx);
    }

    fn find_worktree_data(
        &mut self,
        worktree_id: WorktreeId,
        cx: &mut Context<Self>,
    ) -> Option<(Arc<Path>, bool, Option<RemoteHostLocation>)> {
        let mut worktree_data = None;
        self.worktree_stores.retain(
            |worktree_store, remote_host| match worktree_store.upgrade() {
                Some(worktree_store) => {
                    if worktree_data.is_none() {
                        if let Some(worktree) =
                            worktree_store.read(cx).worktree_for_id(worktree_id, cx)
                        {
                            worktree_data = Some((
                                worktree.read(cx).abs_path(),
                                worktree.read(cx).is_single_file(),
                                remote_host.clone(),
                            ));
                        }
                    }
                    true
                }
                None => false,
            },
        );
        worktree_data
    }

    fn add_worktree_store(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        remote_host: Option<impl Into<RemoteHostLocation>>,
        cx: &mut Context<Self>,
    ) {
        let remote_host = remote_host.map(|host_data| host_data.into());
        self.worktree_stores
            .insert(worktree_store.downgrade(), remote_host.clone());
        self.trusted_paths = self
            .trusted_paths
            .drain()
            .map(|path_trust| match path_trust {
                worktree @ PathTrust::Worktree(_) => worktree,
                PathTrust::AbsPath(abs_path, trusted_remote_host) => {
                    if trusted_remote_host != remote_host {
                        PathTrust::AbsPath(abs_path, trusted_remote_host)
                    } else {
                        find_worktree_in_store(worktree_store.read(cx), &abs_path, cx)
                            .map(PathTrust::Worktree)
                            .unwrap_or_else(|| PathTrust::AbsPath(abs_path, trusted_remote_host))
                    }
                }
                PathTrust::Global(host) => PathTrust::Global(host),
            })
            .collect();
        if !self
            .trusted_paths
            .contains(&PathTrust::Global(remote_host.clone()))
        {
            self.restricted_globals.insert(remote_host.clone());
        }
        cx.emit(TrustedWorktreesEvent::Restricted(HashSet::from_iter([
            PathTrust::Global(remote_host),
        ])));
    }
}

pub fn find_worktree_in_store(
    worktree_store: &WorktreeStore,
    abs_path: &Path,
    cx: &App,
) -> Option<WorktreeId> {
    let (worktree, path_in_worktree) = worktree_store.find_worktree(&abs_path, cx)?;
    if path_in_worktree.is_empty() {
        Some(worktree.read(cx).id())
    } else {
        None
    }
}
