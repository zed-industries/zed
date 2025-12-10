// TODO kb docs
/* FOR A SINGLE HOST LOCATION
 *
 * Single File Worktree
 * Global
 * Directory Worktree
 * AbsPath
 *
 */

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
    remote_host: Option<RemoteHostLocation>,
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
                if let TrustedWorktreesEvent::Trusted(trusted_host, trusted_paths) = e {
                    if trusted_host == &remote_host && trusted_paths.contains(&PathTrust::Global) {
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
    trusted_paths: HashMap<Option<RemoteHostLocation>, HashSet<PathTrust>>,
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
            RemoteConnectionOptions::Docker(docker_connection_options) => (
                Some(SharedString::new(docker_connection_options.name)),
                SharedString::new(docker_connection_options.container_id),
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
    Global,
    /// A worktree that is familiar to this workspace.
    Worktree(WorktreeId),
    /// A path that may be another worktree yet not loaded into workspace,
    /// or a parent path coming out of the security modal.
    AbsPath(PathBuf),
}

impl PathTrust {
    fn to_proto(&self) -> proto::PathTrust {
        match self {
            Self::Global => proto::PathTrust {
                content: Some(proto::path_trust::Content::Global(0)),
            },
            Self::Worktree(worktree_id) => proto::PathTrust {
                content: Some(proto::path_trust::Content::WorktreeId(
                    worktree_id.to_proto(),
                )),
            },
            Self::AbsPath(path_buf) => proto::PathTrust {
                content: Some(proto::path_trust::Content::AbsPath(
                    path_buf.to_string_lossy().to_string(),
                )),
            },
        }
    }

    pub fn from_proto(proto: proto::PathTrust) -> Option<Self> {
        Some(match proto.content? {
            proto::path_trust::Content::WorktreeId(id) => {
                Self::Worktree(WorktreeId::from_proto(id))
            }
            proto::path_trust::Content::AbsPath(path) => Self::AbsPath(PathBuf::from(path)),
            proto::path_trust::Content::Global(_) => Self::Global,
        })
    }
}

#[derive(Debug)]
pub enum TrustedWorktreesEvent {
    Trusted(Option<RemoteHostLocation>, HashSet<PathTrust>),
    Restricted(Option<RemoteHostLocation>, HashSet<PathTrust>),
}

impl EventEmitter<TrustedWorktreesEvent> for TrustedWorktreesStorage {}

impl TrustedWorktreesStorage {
    fn new(
        worktree_store: Entity<WorktreeStore>,
        remote_host: Option<RemoteHostLocation>,
        downstream_client: Option<(AnyProtoClient, u64)>,
        upstream_client: Option<(AnyProtoClient, u64)>,
        cx: &App,
    ) -> Self {
        let trusted_paths = if downstream_client.is_none() {
            match PROJECT_DB.fetch_trusted_worktrees(
                worktree_store.clone(),
                remote_host.clone(),
                cx,
            ) {
                Ok(trusted_paths) => trusted_paths,
                Err(e) => {
                    log::error!("Failed to do initial trusted worktrees fetch: {e:#}");
                    HashMap::default()
                }
            }
        } else {
            HashMap::default()
        };
        let restricted_globals = if trusted_paths
            .get(&remote_host)
            .is_some_and(|trusted_paths| trusted_paths.contains(&PathTrust::Global))
        {
            HashSet::default()
        } else {
            HashSet::from_iter([remote_host.clone()])
        };

        if let Some((upstream_client, upstream_project_id)) = &upstream_client {
            let trusted_paths = trusted_paths
                .iter()
                .flat_map(|(_, paths)| paths.iter().map(|trusted_path| trusted_path.to_proto()))
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

    pub fn has_restricted_worktrees(
        &self,
        worktree_store: &Entity<WorktreeStore>,
        cx: &App,
    ) -> bool {
        let Some(remote_host) = self.worktree_stores.get(&worktree_store.downgrade()) else {
            return false;
        };
        self.restricted_globals.contains(remote_host)
            || self.restricted.iter().any(|restricted_worktree| {
                worktree_store
                    .read(cx)
                    .worktree_for_id(*restricted_worktree, cx)
                    .is_some()
            })
    }

    /// Adds worktree absolute paths to the trusted list.
    /// This will emit [`TrustedWorktreesEvent::Trusted`] event.
    pub fn trust(
        &mut self,
        mut trusted_paths: HashSet<PathTrust>,
        remote_host: Option<RemoteHostLocation>,
        cx: &mut Context<Self>,
    ) {
        // TODO kb unit test all this logic
        let current_trusted = self.trusted_paths.remove(&remote_host);
        let mut new_global_trusted = false;
        let mut new_trusted_single_file_worktrees = HashSet::default();
        let mut new_trusted_other_worktrees = HashSet::default();
        let mut new_trusted_abs_paths = HashSet::default();
        for trusted_path in trusted_paths.iter().chain(
            current_trusted
                .iter()
                .flat_map(|current_trusted| current_trusted.iter()),
        ) {
            match trusted_path {
                PathTrust::Global => {
                    self.restricted_globals.remove(&remote_host);
                    new_global_trusted = true;
                }
                PathTrust::Worktree(worktree_id) => {
                    self.restricted.remove(worktree_id);
                    if let Some((abs_path, is_file, host)) =
                        self.find_worktree_data(*worktree_id, cx)
                    {
                        if host == remote_host {
                            if is_file {
                                new_trusted_single_file_worktrees.insert(*worktree_id);
                            } else {
                                new_trusted_other_worktrees.insert((abs_path, *worktree_id));
                                new_global_trusted = true;
                            }
                        }
                    }
                }
                PathTrust::AbsPath(path) => {
                    new_global_trusted = true;
                    debug_assert!(
                        path.is_absolute(),
                        "Cannot trust non-absolute path {path:?}"
                    );
                    new_trusted_abs_paths.insert(path.clone());
                }
            }
        }

        if new_global_trusted {
            new_trusted_single_file_worktrees.clear();
        }
        new_trusted_other_worktrees.retain(|(worktree_abs_path, _)| {
            new_trusted_abs_paths
                .iter()
                .all(|new_trusted_path| !worktree_abs_path.starts_with(new_trusted_path))
        });
        let previous_restricted = std::mem::take(&mut self.restricted);
        self.restricted = previous_restricted
            .into_iter()
            .filter(|restricted_worktree| {
                let Some((restricted_worktree_path, _, restricted_host)) =
                    self.find_worktree_data(*restricted_worktree, cx)
                else {
                    return true;
                };
                if restricted_host != remote_host {
                    return true;
                }
                let retain = new_trusted_abs_paths.iter().all(|new_trusted_path| {
                    !restricted_worktree_path.starts_with(new_trusted_path)
                });
                if !retain {
                    trusted_paths.insert(PathTrust::Worktree(*restricted_worktree));
                }
                retain
            })
            .collect();

        {
            let trusted_paths = self.trusted_paths.entry(remote_host.clone()).or_default();
            trusted_paths.extend(new_trusted_abs_paths.into_iter().map(PathTrust::AbsPath));
            trusted_paths.extend(
                new_trusted_other_worktrees
                    .into_iter()
                    .map(|(_, worktree_id)| PathTrust::Worktree(worktree_id)),
            );
            trusted_paths.extend(
                new_trusted_single_file_worktrees
                    .into_iter()
                    .map(PathTrust::Worktree),
            );
            if trusted_paths.is_empty() && new_global_trusted {
                trusted_paths.insert(PathTrust::Global);
            }
        }

        let mut new_trusted_globals = HashSet::default();
        let new_trusted_worktrees = self
            .trusted_paths
            .clone()
            .into_iter()
            .map(|(host, paths)| {
                let abs_paths = paths
                    .into_iter()
                    .flat_map(|path| match path {
                        PathTrust::Worktree(worktree_id) => self
                            // TODO kb how correct this method is?
                            // What if different windows find different set of worktrees?
                            .find_worktree_data(worktree_id, cx)
                            .and_then(|(abs_path, ..)| Some(abs_path.to_path_buf())),
                        PathTrust::AbsPath(abs_path) => Some(abs_path),
                        PathTrust::Global => {
                            new_trusted_globals.insert(host.clone());
                            None
                        }
                    })
                    .collect();
                (host, abs_paths)
            })
            .collect();

        cx.emit(TrustedWorktreesEvent::Trusted(
            remote_host,
            trusted_paths.clone(),
        ));

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
                    .map(|trusted_path| trusted_path.to_proto())
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

    pub fn restrict(
        &mut self,
        restricted_paths: HashSet<PathTrust>,
        remote_host: Option<RemoteHostLocation>,
        cx: &mut Context<Self>,
    ) {
        for restricted_path in restricted_paths {
            match restricted_path {
                PathTrust::Global => {
                    self.restricted_globals.insert(remote_host.clone());
                    cx.emit(TrustedWorktreesEvent::Restricted(
                        remote_host.clone(),
                        HashSet::from_iter([PathTrust::Global]),
                    ));
                }
                PathTrust::Worktree(worktree_id) => {
                    self.restricted.insert(worktree_id);
                    cx.emit(TrustedWorktreesEvent::Restricted(
                        remote_host.clone(),
                        HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
                    ));
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

        let Some((worktree_path, is_file, remote_host)) = self.find_worktree_data(worktree_id, cx)
        else {
            return false;
        };

        if self
            .trusted_paths
            .get(&remote_host)
            .is_some_and(|trusted_paths| trusted_paths.contains(&PathTrust::Worktree(worktree_id)))
        {
            return true;
        }
        // Single-file worktrees are, for example, files drag-and-dropped into Zed, Zed's settings,
        // task and bindings files, language servers' go to definition targets outside the current worktree, etc., etc.
        // Avoid flashing them with another warning, if global trust is enabled or any other trust was made on the same host.
        if is_file && self.already_trusted_host(remote_host.clone(), cx) {
            return true;
        }

        let parent_path_trusted =
            self.trusted_paths
                .get(&remote_host)
                .is_some_and(|trusted_paths| {
                    trusted_paths.iter().any(|trusted_path| {
                        let PathTrust::AbsPath(trusted_path) = trusted_path else {
                            return false;
                        };
                        worktree_path.starts_with(trusted_path)
                    })
                });
        if parent_path_trusted {
            return true;
        }

        self.restricted.insert(worktree_id);
        cx.emit(TrustedWorktreesEvent::Restricted(
            remote_host,
            HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
        ));
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
        remote_host: Option<RemoteHostLocation>,
        cx: &mut Context<Self>,
    ) -> bool {
        if ProjectSettings::get_global(cx).session.trust_all_worktrees {
            return true;
        }
        if self.restricted_globals.contains(&remote_host) {
            return false;
        }
        if self.already_trusted_host(remote_host.clone(), cx) {
            return true;
        }

        self.restricted_globals.insert(remote_host.clone());
        cx.emit(TrustedWorktreesEvent::Restricted(
            remote_host.clone(),
            HashSet::from_iter([PathTrust::Global]),
        ));

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
        remote_host: Option<RemoteHostLocation>,
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
        if self.restricted_globals.contains(&remote_host) {
            restricted_paths.insert(None);
        }
        restricted_paths
    }

    pub fn trust_all(&mut self, cx: &mut Context<Self>) {
        for (remote_host, mut worktrees) in std::mem::take(&mut self.restricted)
            .into_iter()
            .flat_map(|restricted_worktree| {
                let (_, _, host) = self.find_worktree_data(restricted_worktree, cx)?;
                Some((restricted_worktree, host))
            })
            .fold(HashMap::default(), |mut acc, (worktree_id, remote_host)| {
                acc.entry(remote_host)
                    .or_insert_with(HashSet::default)
                    .insert(PathTrust::Worktree(worktree_id));
                acc
            })
        {
            if self.restricted_globals.remove(&remote_host) {
                worktrees.insert(PathTrust::Global);
            }
            self.trust(worktrees, remote_host, cx);
        }

        for remote_host in std::mem::take(&mut self.restricted_globals) {
            self.trust(HashSet::from_iter([PathTrust::Global]), remote_host, cx);
        }
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
        remote_host: Option<RemoteHostLocation>,
        cx: &mut Context<Self>,
    ) {
        self.worktree_stores
            .insert(worktree_store.downgrade(), remote_host.clone());

        if let Some(trusted_paths) = self.trusted_paths.remove(&remote_host) {
            if !trusted_paths.contains(&PathTrust::Global) {
                self.restricted_globals.insert(remote_host.clone());
            }

            self.trusted_paths.insert(
                remote_host.clone(),
                trusted_paths
                    .into_iter()
                    .map(|path_trust| match path_trust {
                        PathTrust::AbsPath(abs_path) => {
                            find_worktree_in_store(worktree_store.read(cx), &abs_path, cx)
                                .map(PathTrust::Worktree)
                                .unwrap_or_else(|| PathTrust::AbsPath(abs_path))
                        }
                        other => other,
                    })
                    .collect(),
            );
        }

        cx.emit(TrustedWorktreesEvent::Restricted(
            remote_host,
            HashSet::from_iter([PathTrust::Global]),
        ));
    }

    fn already_trusted_host(
        &mut self,
        remote_host: Option<RemoteHostLocation>,
        cx: &mut Context<Self>,
    ) -> bool {
        let mut already_trusted = false;
        if let Some(trusted_paths) = self.trusted_paths.remove(&remote_host) {
            already_trusted = trusted_paths.iter().any(|trusted_path| match trusted_path {
                PathTrust::Worktree(worktree_id) => {
                    if let Some((_, false, trusted_host)) =
                        self.find_worktree_data(*worktree_id, cx)
                    {
                        trusted_host == remote_host
                    } else {
                        false
                    }
                }
                PathTrust::AbsPath(_) | PathTrust::Global => true,
            });
            self.trusted_paths.insert(remote_host, trusted_paths);
        }

        already_trusted
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
