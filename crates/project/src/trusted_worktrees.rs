//! A module, responsible for managing the trust logic in Zed.
//!
//! It deals with multiple hosts, distinguished by [`RemoteHostLocation`].
//! Each [`crate::Project`] and `HeadlessProject` should call [`init_global`], if wants to establish the trust mechanism.
//! This will set up a [`gpui::Global`] with [`TrustedWorktrees`] entity that will persist, restore and allow querying for worktree trust.
//! It's also possible to subscribe on [`TrustedWorktreesEvent`] events of this entity to track trust changes dynamically.
//!
//! The implementation can synchronize trust information with the remote hosts: currently, WSL and SSH.
//! Docker and Collab remotes do not employ trust mechanism, as manage that themselves.
//!
//! Unless `trust_all_worktrees` auto trust is enabled, does not trust anything that was not persisted before.
//! When dealing with "restricted" and other related concepts in the API, it means all explicitly restricted, after any of the [`TrustedWorktreesStore::can_trust`] and [`TrustedWorktreesStore::can_trust_global`] calls.
//!
//!
//!
//!
//! Path rust hierarchy.
//!
//! Zed has multiple layers of trust, based on the requests and [`PathTrust`] enum variants.
//! From the least to the most trusted level:
//!
//! * "single file worktree"
//!
//! After opening an empty Zed it's possible to open just a file, same as after opening a directory in Zed it's possible to open a file outside of this directory.
//! Usual scenario for both cases is opening Zed's settings.json file via `zed: open settings file` command: that starts a language server for a new file open, which originates from a newly created, single file worktree.
//!
//! Spawning a language server is potentially dangerous, and Zed needs to restrict that by default.
//! Each single file worktree requires a separate trust permission, unless a more global level is trusted.
//!
//! * "workspace"
//!
//! Even an empty Zed instance with no files or directories open is potentially dangerous: opening an Assistant Panel and creating new external agent thread might require installing and running MCP servers.
//!
//! Disabling the entire panel is possible with ai-related settings.
//! Yet when it's enabled, it's still reasonably safe to use remote AI agents and control their permissions in the Assistant Panel.
//!
//! Unlike that, MCP servers are similar to language servers and may require fetching, installing and running packages or binaries.
//! Given that those servers are not tied to any particular worktree, this level of trust is required to operate any MCP server.
//!
//! Workspace level of trust assumes all single file worktrees are trusted too, for the same host: if we allow global MCP server-related functionality, we can already allow spawning language servers for single file worktrees as well.
//!
//! * "directory worktree"
//!
//! If a directory is open in Zed, it's a full worktree which may spawn multiple language servers associated with it.
//! Each such worktree requires a separate trust permission, so each separate directory worktree has to be trusted separately, unless a more global level is trusted.
//!
//! When a directory worktree is trusted and language servers are allowed to be downloaded and started, hence we also allow workspace level of trust (hence, "single file worktree" level of trust also).
//!
//! * "path override"
//!
//! To ease trusting multiple directory worktrees at once, it's possible to trust a parent directory of a certain directory worktree opened in Zed.
//! Trusting a directory means trusting all its subdirectories as well, including all current and potential directory worktrees.
//!
//! If we trust multiple projects to install and spawn various language server processes, we can also allow workspace trust requests for MCP servers installation and spawning.

use collections::{HashMap, HashSet};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, SharedString, Task, WeakEntity,
};
use remote::RemoteConnectionOptions;
use rpc::{AnyProtoClient, proto};
use settings::{Settings as _, WorktreeId};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::debug_panic;

use crate::{project_settings::ProjectSettings, worktree_store::WorktreeStore};

pub fn init(
    db_trusted_paths: TrustedPaths,
    downstream_client: Option<(AnyProtoClient, u64)>,
    upstream_client: Option<(AnyProtoClient, u64)>,
    cx: &mut App,
) {
    if TrustedWorktrees::try_get_global(cx).is_none() {
        let trusted_worktrees = cx.new(|_| {
            TrustedWorktreesStore::new(
                db_trusted_paths,
                None,
                None,
                downstream_client,
                upstream_client,
            )
        });
        cx.set_global(TrustedWorktrees(trusted_worktrees))
    }
}

/// An initialization call to set up trust global for a particular project (remote or local).
pub fn track_worktree_trust(
    worktree_store: Entity<WorktreeStore>,
    remote_host: Option<RemoteHostLocation>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    upstream_client: Option<(AnyProtoClient, u64)>,
    cx: &mut App,
) {
    match TrustedWorktrees::try_get_global(cx) {
        Some(trusted_worktrees) => {
            trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                let sync_upstream = trusted_worktrees.upstream_client.as_ref().map(|(_, id)| id)
                    != upstream_client.as_ref().map(|(_, id)| id);
                trusted_worktrees.downstream_client = downstream_client;
                trusted_worktrees.upstream_client = upstream_client;
                trusted_worktrees.add_worktree_store(worktree_store, remote_host, cx);

                if sync_upstream {
                    if let Some((upstream_client, upstream_project_id)) =
                        &trusted_worktrees.upstream_client
                    {
                        let trusted_paths = trusted_worktrees
                            .trusted_paths
                            .iter()
                            .flat_map(|(_, paths)| {
                                paths.iter().map(|trusted_path| trusted_path.to_proto())
                            })
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
            });
        }
        None => log::debug!("No TrustedWorktrees initialized, not tracking worktree trust"),
    }
}

/// Waits until at least [`PathTrust::Workspace`] level of trust is granted for the host the [`TrustedWorktrees`] was initialized with.
pub fn wait_for_default_workspace_trust(
    what_waits: &'static str,
    cx: &mut App,
) -> Option<Task<()>> {
    let trusted_worktrees = TrustedWorktrees::try_get_global(cx)?;
    wait_for_workspace_trust(
        trusted_worktrees.read(cx).remote_host.clone(),
        what_waits,
        cx,
    )
}

/// Waits until at least [`PathTrust::Workspace`] level of trust is granted for a particular host.
pub fn wait_for_workspace_trust(
    remote_host: Option<impl Into<RemoteHostLocation>>,
    what_waits: &'static str,
    cx: &mut App,
) -> Option<Task<()>> {
    let trusted_worktrees = TrustedWorktrees::try_get_global(cx)?;
    let remote_host = remote_host.map(|host| host.into());

    let remote_host = if trusted_worktrees.update(cx, |trusted_worktrees, cx| {
        trusted_worktrees.can_trust_workspace(remote_host.clone(), cx)
    }) {
        None
    } else {
        Some(remote_host)
    }?;

    Some(cx.spawn(async move |cx| {
        log::info!("Waiting for workspace to be trusted before starting {what_waits}");
        let (tx, restricted_worktrees_task) = smol::channel::bounded::<()>(1);
        let Ok(_subscription) = cx.update(|cx| {
            cx.subscribe(&trusted_worktrees, move |_, e, _| {
                if let TrustedWorktreesEvent::Trusted(trusted_host, trusted_paths) = e {
                    if trusted_host == &remote_host && trusted_paths.contains(&PathTrust::Workspace)
                    {
                        log::info!("Workspace is trusted for {what_waits}");
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

/// A collection of worktree trust metadata, can be accessed globally (if initialized) and subscribed to.
pub struct TrustedWorktrees(Entity<TrustedWorktreesStore>);

impl Global for TrustedWorktrees {}

impl TrustedWorktrees {
    pub fn try_get_global(cx: &App) -> Option<Entity<TrustedWorktreesStore>> {
        cx.try_global::<Self>().map(|this| this.0.clone())
    }
}

/// A collection of worktrees that are considered trusted and not trusted.
/// This can be used when checking for this criteria before enabling certain features.
///
/// Emits an event each time the worktree was checked and found not trusted,
/// or a certain worktree had been trusted.
pub struct TrustedWorktreesStore {
    downstream_client: Option<(AnyProtoClient, u64)>,
    upstream_client: Option<(AnyProtoClient, u64)>,
    worktree_stores: HashMap<WeakEntity<WorktreeStore>, Option<RemoteHostLocation>>,
    trusted_paths: TrustedPaths,
    restricted: HashSet<WorktreeId>,
    remote_host: Option<RemoteHostLocation>,
    restricted_workspaces: HashSet<Option<RemoteHostLocation>>,
}

/// An identifier of a host to split the trust questions by.
/// Each trusted data change and event is done for a particular host.
/// A host may contain more than one worktree or even project open concurrently.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RemoteHostLocation {
    pub user_name: Option<SharedString>,
    pub host_identifier: SharedString,
}

impl From<RemoteConnectionOptions> for RemoteHostLocation {
    fn from(options: RemoteConnectionOptions) -> Self {
        let (user_name, host_name) = match options {
            RemoteConnectionOptions::Ssh(ssh) => (
                ssh.username.map(SharedString::new),
                SharedString::new(ssh.host.to_string()),
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
            host_identifier: host_name,
        }
    }
}

/// A unit of trust consideration inside a particular host:
/// either a familiar worktree, or a path that may influence other worktrees' trust.
/// See module-level documentation on the trust model.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PathTrust {
    /// General, no worktrees or files open case.
    /// E.g. MCP servers can be spawned from a blank Zed instance, but will do `npm i` and other potentially malicious actions.
    Workspace,
    /// A worktree that is familiar to this workspace.
    /// Either a single file or a directory worktree.
    Worktree(WorktreeId),
    /// A path that may be another worktree yet not loaded into any workspace (hence, without any `WorktreeId`),
    /// or a parent path coming out of the security modal.
    AbsPath(PathBuf),
}

impl PathTrust {
    fn to_proto(&self) -> proto::PathTrust {
        match self {
            Self::Workspace => proto::PathTrust {
                content: Some(proto::path_trust::Content::Workspace(0)),
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
            proto::path_trust::Content::Workspace(_) => Self::Workspace,
        })
    }
}

/// A change of trust on a certain host.
#[derive(Debug)]
pub enum TrustedWorktreesEvent {
    Trusted(Option<RemoteHostLocation>, HashSet<PathTrust>),
    Restricted(Option<RemoteHostLocation>, HashSet<PathTrust>),
}

impl EventEmitter<TrustedWorktreesEvent> for TrustedWorktreesStore {}

pub type TrustedPaths = HashMap<Option<RemoteHostLocation>, HashSet<PathTrust>>;

impl TrustedWorktreesStore {
    fn new(
        trusted_paths: TrustedPaths,
        worktree_store: Option<Entity<WorktreeStore>>,
        remote_host: Option<RemoteHostLocation>,
        downstream_client: Option<(AnyProtoClient, u64)>,
        upstream_client: Option<(AnyProtoClient, u64)>,
    ) -> Self {
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
        }

        let worktree_stores = match worktree_store {
            Some(worktree_store) => {
                HashMap::from_iter([(worktree_store.downgrade(), remote_host.clone())])
            }
            None => HashMap::default(),
        };

        Self {
            trusted_paths,
            downstream_client,
            upstream_client,
            remote_host,
            restricted_workspaces: HashSet::default(),
            restricted: HashSet::default(),
            worktree_stores,
        }
    }

    /// Whether a particular worktree store has associated worktrees that are restricted, or an associated host is restricted.
    pub fn has_restricted_worktrees(
        &self,
        worktree_store: &Entity<WorktreeStore>,
        cx: &App,
    ) -> bool {
        let Some(remote_host) = self.worktree_stores.get(&worktree_store.downgrade()) else {
            return false;
        };
        self.restricted_workspaces.contains(remote_host)
            || self.restricted.iter().any(|restricted_worktree| {
                worktree_store
                    .read(cx)
                    .worktree_for_id(*restricted_worktree, cx)
                    .is_some()
            })
    }

    /// Adds certain entities on this host to the trusted list.
    /// This will emit [`TrustedWorktreesEvent::Trusted`] event for all passed entries
    /// and the ones that got auto trusted based on trust hierarchy (see module-level docs).
    pub fn trust(
        &mut self,
        mut trusted_paths: HashSet<PathTrust>,
        remote_host: Option<RemoteHostLocation>,
        cx: &mut Context<Self>,
    ) {
        let mut new_workspace_trusted = false;
        let mut new_trusted_single_file_worktrees = HashSet::default();
        let mut new_trusted_other_worktrees = HashSet::default();
        let mut new_trusted_abs_paths = HashSet::default();
        for trusted_path in trusted_paths.iter().chain(
            self.trusted_paths
                .remove(&remote_host)
                .iter()
                .flat_map(|current_trusted| current_trusted.iter()),
        ) {
            match trusted_path {
                PathTrust::Workspace => new_workspace_trusted = true,
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
                                new_workspace_trusted = true;
                            }
                        }
                    }
                }
                PathTrust::AbsPath(path) => {
                    new_workspace_trusted = true;
                    debug_assert!(
                        path.is_absolute(),
                        "Cannot trust non-absolute path {path:?}"
                    );
                    new_trusted_abs_paths.insert(path.clone());
                }
            }
        }

        if new_workspace_trusted {
            new_trusted_single_file_worktrees.clear();
            self.restricted_workspaces.remove(&remote_host);
            trusted_paths.insert(PathTrust::Workspace);
        }
        new_trusted_other_worktrees.retain(|(worktree_abs_path, _)| {
            new_trusted_abs_paths
                .iter()
                .all(|new_trusted_path| !worktree_abs_path.starts_with(new_trusted_path))
        });
        if !new_trusted_other_worktrees.is_empty() {
            new_trusted_single_file_worktrees.clear();
        }
        self.restricted = std::mem::take(&mut self.restricted)
            .into_iter()
            .filter(|restricted_worktree| {
                let Some((restricted_worktree_path, is_file, restricted_host)) =
                    self.find_worktree_data(*restricted_worktree, cx)
                else {
                    return false;
                };
                if restricted_host != remote_host {
                    return true;
                }
                let retain = (!is_file
                    || (!new_workspace_trusted && new_trusted_other_worktrees.is_empty()))
                    && new_trusted_abs_paths.iter().all(|new_trusted_path| {
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
            if trusted_paths.is_empty() && new_workspace_trusted {
                trusted_paths.insert(PathTrust::Workspace);
            }
        }

        cx.emit(TrustedWorktreesEvent::Trusted(
            remote_host,
            trusted_paths.clone(),
        ));

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

    /// Restricts certain entities on this host.
    /// This will emit [`TrustedWorktreesEvent::Restricted`] event for all passed entries.
    pub fn restrict(
        &mut self,
        restricted_paths: HashSet<PathTrust>,
        remote_host: Option<RemoteHostLocation>,
        cx: &mut Context<Self>,
    ) {
        for restricted_path in restricted_paths {
            match restricted_path {
                PathTrust::Workspace => {
                    self.restricted_workspaces.insert(remote_host.clone());
                    cx.emit(TrustedWorktreesEvent::Restricted(
                        remote_host.clone(),
                        HashSet::from_iter([PathTrust::Workspace]),
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

    /// Erases all trust information.
    /// Requires Zed's restart to take proper effect.
    pub fn clear_trusted_paths(&mut self) {
        self.trusted_paths.clear();
    }

    /// Checks whether a certain worktree is trusted (or on a larger trust level).
    /// If not, emits [`TrustedWorktreesEvent::Restricted`] event if for the first time and not trusted, or no corresponding worktree store was found.
    ///
    /// No events or data adjustment happens when `trust_all_worktrees` auto trust is enabled.
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

        // See module documentation for details on trust level.
        if is_file && self.trusted_paths.contains_key(&remote_host) {
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
                    restrict_workspace: false,
                    worktree_ids: vec![worktree_id.to_proto()],
                })
                .ok();
        }
        if let Some((upstream_client, upstream_project_id)) = &self.upstream_client {
            upstream_client
                .send(proto::RestrictWorktrees {
                    project_id: *upstream_project_id,
                    restrict_workspace: false,
                    worktree_ids: vec![worktree_id.to_proto()],
                })
                .ok();
        }
        false
    }

    /// Checks whether a certain worktree is trusted globally (or on a larger trust level).
    /// If not, emits [`TrustedWorktreesEvent::Restricted`] event if checked for the first time and not trusted.
    ///
    /// No events or data adjustment happens when `trust_all_worktrees` auto trust is enabled.
    pub fn can_trust_workspace(
        &mut self,
        remote_host: Option<RemoteHostLocation>,
        cx: &mut Context<Self>,
    ) -> bool {
        if ProjectSettings::get_global(cx).session.trust_all_worktrees {
            return true;
        }
        if self.restricted_workspaces.contains(&remote_host) {
            return false;
        }
        if self.trusted_paths.contains_key(&remote_host) {
            return true;
        }

        self.restricted_workspaces.insert(remote_host.clone());
        cx.emit(TrustedWorktreesEvent::Restricted(
            remote_host.clone(),
            HashSet::from_iter([PathTrust::Workspace]),
        ));

        if remote_host == self.remote_host {
            if let Some((downstream_client, downstream_project_id)) = &self.downstream_client {
                downstream_client
                    .send(proto::RestrictWorktrees {
                        project_id: *downstream_project_id,
                        restrict_workspace: true,
                        worktree_ids: Vec::new(),
                    })
                    .ok();
            }
            if let Some((upstream_client, upstream_project_id)) = &self.upstream_client {
                upstream_client
                    .send(proto::RestrictWorktrees {
                        project_id: *upstream_project_id,
                        restrict_workspace: true,
                        worktree_ids: Vec::new(),
                    })
                    .ok();
            }
        }
        false
    }

    /// Lists all explicitly restricted worktrees (via [`TrustedWorktreesStore::can_trust`] and [`TrustedWorktreesStore::can_trust_workspace`] method calls) for a particular worktree store on a particular host.
    pub fn restricted_worktrees(
        &self,
        worktree_store: &WorktreeStore,
        remote_host: Option<RemoteHostLocation>,
        cx: &App,
    ) -> HashSet<Option<(WorktreeId, Arc<Path>)>> {
        let mut single_file_paths = HashSet::default();
        let other_paths = self
            .restricted
            .iter()
            .filter_map(|&restricted_worktree_id| {
                let worktree = worktree_store.worktree_for_id(restricted_worktree_id, cx)?;
                let worktree = worktree.read(cx);
                let abs_path = worktree.abs_path();
                if worktree.is_single_file() {
                    single_file_paths.insert(Some((restricted_worktree_id, abs_path)));
                    None
                } else {
                    Some((restricted_worktree_id, abs_path))
                }
            })
            .map(Some)
            .collect::<HashSet<_>>();

        if !other_paths.is_empty() {
            return other_paths;
        } else if self.restricted_workspaces.contains(&remote_host) {
            return HashSet::from_iter([None]);
        } else {
            single_file_paths
        }
    }

    /// Switches the "trust nothing" mode to "automatically trust everything".
    /// This does not influence already persisted data, but stops adding new worktrees there.
    pub fn auto_trust_all(&mut self, cx: &mut Context<Self>) {
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
            if self.restricted_workspaces.remove(&remote_host) {
                worktrees.insert(PathTrust::Workspace);
            }
            self.trust(worktrees, remote_host, cx);
        }

        for remote_host in std::mem::take(&mut self.restricted_workspaces) {
            self.trust(HashSet::from_iter([PathTrust::Workspace]), remote_host, cx);
        }
    }

    /// Returns a normalized representation of the trusted paths to store in the DB.
    pub fn trusted_paths_for_serialization(
        &mut self,
        cx: &mut Context<Self>,
    ) -> (
        HashSet<Option<RemoteHostLocation>>,
        HashMap<Option<RemoteHostLocation>, HashSet<PathBuf>>,
    ) {
        let mut new_trusted_workspaces = HashSet::default();
        let new_trusted_worktrees = self
            .trusted_paths
            .clone()
            .into_iter()
            .map(|(host, paths)| {
                let abs_paths = paths
                    .into_iter()
                    .flat_map(|path| match path {
                        PathTrust::Worktree(worktree_id) => self
                            .find_worktree_data(worktree_id, cx)
                            .map(|(abs_path, ..)| abs_path.to_path_buf()),
                        PathTrust::AbsPath(abs_path) => Some(abs_path),
                        PathTrust::Workspace => {
                            new_trusted_workspaces.insert(host.clone());
                            None
                        }
                    })
                    .collect();
                (host, abs_paths)
            })
            .collect();
        (new_trusted_workspaces, new_trusted_worktrees)
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, path::PathBuf, rc::Rc};

    use collections::HashSet;
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    use crate::{FakeFs, Project};

    use super::*;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            if cx.try_global::<SettingsStore>().is_none() {
                let settings_store = SettingsStore::test(cx);
                cx.set_global(settings_store);
            }
            if cx.try_global::<TrustedWorktrees>().is_some() {
                cx.remove_global::<TrustedWorktrees>();
            }
        });
    }

    fn init_trust_global(
        worktree_store: Entity<WorktreeStore>,
        cx: &mut TestAppContext,
    ) -> Entity<TrustedWorktreesStore> {
        cx.update(|cx| {
            init(HashMap::default(), None, None, cx);
            track_worktree_trust(worktree_store, None, None, None, cx);
            TrustedWorktrees::try_get_global(cx).expect("global should be set")
        })
    }

    #[gpui::test]
    async fn test_single_worktree_trust(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "main.rs": "fn main() {}" }))
            .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_id = worktree_store.read_with(cx, |store, cx| {
            store.worktrees().next().unwrap().read(cx).id()
        });

        let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

        let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
        cx.update({
            let events = events.clone();
            |cx| {
                cx.subscribe(&trusted_worktrees, move |_, event, _| {
                    events.borrow_mut().push(match event {
                        TrustedWorktreesEvent::Trusted(host, paths) => {
                            TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                        }
                        TrustedWorktreesEvent::Restricted(host, paths) => {
                            TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                        }
                    });
                })
            }
        })
        .detach();

        let can_trust = trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(!can_trust, "worktree should be restricted by default");

        {
            let events = events.borrow();
            assert_eq!(events.len(), 1);
            match &events[0] {
                TrustedWorktreesEvent::Restricted(host, paths) => {
                    assert!(host.is_none());
                    assert!(paths.contains(&PathTrust::Worktree(worktree_id)));
                }
                _ => panic!("expected Restricted event"),
            }
        }

        let has_restricted = trusted_worktrees.read_with(cx, |store, cx| {
            store.has_restricted_worktrees(&worktree_store, cx)
        });
        assert!(has_restricted, "should have restricted worktrees");

        let restricted = worktree_store.read_with(cx, |ws, cx| {
            trusted_worktrees
                .read(cx)
                .restricted_worktrees(ws, None, cx)
        });
        assert!(
            restricted
                .iter()
                .any(|r| r.as_ref().map(|(id, _)| *id) == Some(worktree_id))
        );

        events.borrow_mut().clear();

        let can_trust_again =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(!can_trust_again, "worktree should still be restricted");
        assert!(
            events.borrow().is_empty(),
            "no duplicate Restricted event on repeated can_trust"
        );

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
                None,
                cx,
            );
        });

        {
            let events = events.borrow();
            assert_eq!(events.len(), 1);
            match &events[0] {
                TrustedWorktreesEvent::Trusted(host, paths) => {
                    assert!(host.is_none());
                    assert!(paths.contains(&PathTrust::Worktree(worktree_id)));
                }
                _ => panic!("expected Trusted event"),
            }
        }

        let can_trust_after =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(can_trust_after, "worktree should be trusted after trust()");

        let has_restricted_after = trusted_worktrees.read_with(cx, |store, cx| {
            store.has_restricted_worktrees(&worktree_store, cx)
        });
        assert!(
            !has_restricted_after,
            "should have no restricted worktrees after trust"
        );

        let restricted_after = worktree_store.read_with(cx, |ws, cx| {
            trusted_worktrees
                .read(cx)
                .restricted_worktrees(ws, None, cx)
        });
        assert!(
            restricted_after.is_empty(),
            "restricted set should be empty"
        );
    }

    #[gpui::test]
    async fn test_workspace_trust_no_worktrees(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({})).await;

        let project = Project::test(fs, Vec::<&Path>::new(), cx).await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
        cx.update({
            let events = events.clone();
            |cx| {
                cx.subscribe(&trusted_worktrees, move |_, event, _| {
                    events.borrow_mut().push(match event {
                        TrustedWorktreesEvent::Trusted(host, paths) => {
                            TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                        }
                        TrustedWorktreesEvent::Restricted(host, paths) => {
                            TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                        }
                    });
                })
            }
        })
        .detach();

        let can_trust_workspace =
            trusted_worktrees.update(cx, |store, cx| store.can_trust_workspace(None, cx));
        assert!(
            !can_trust_workspace,
            "workspace should be restricted by default"
        );

        {
            let events = events.borrow();
            assert_eq!(events.len(), 1);
            match &events[0] {
                TrustedWorktreesEvent::Restricted(host, paths) => {
                    assert!(host.is_none());
                    assert!(paths.contains(&PathTrust::Workspace));
                }
                _ => panic!("expected Restricted event"),
            }
        }

        events.borrow_mut().clear();

        let can_trust_workspace_again =
            trusted_worktrees.update(cx, |store, cx| store.can_trust_workspace(None, cx));
        assert!(
            !can_trust_workspace_again,
            "workspace should still be restricted"
        );
        assert!(
            events.borrow().is_empty(),
            "no duplicate Restricted event on repeated can_trust_workspace"
        );

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(HashSet::from_iter([PathTrust::Workspace]), None, cx);
        });

        {
            let events = events.borrow();
            assert_eq!(events.len(), 1);
            match &events[0] {
                TrustedWorktreesEvent::Trusted(host, paths) => {
                    assert!(host.is_none());
                    assert!(paths.contains(&PathTrust::Workspace));
                }
                _ => panic!("expected Trusted event"),
            }
        }

        let can_trust_workspace_after =
            trusted_worktrees.update(cx, |store, cx| store.can_trust_workspace(None, cx));
        assert!(
            can_trust_workspace_after,
            "workspace should be trusted after trust()"
        );
    }

    #[gpui::test]
    async fn test_single_file_worktree_trust(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "foo.rs": "fn foo() {}" }))
            .await;

        let project = Project::test(fs, [path!("/root/foo.rs").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_id = worktree_store.read_with(cx, |store, cx| {
            let worktree = store.worktrees().next().unwrap();
            let worktree = worktree.read(cx);
            assert!(worktree.is_single_file(), "expected single-file worktree");
            worktree.id()
        });

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
        cx.update({
            let events = events.clone();
            |cx| {
                cx.subscribe(&trusted_worktrees, move |_, event, _| {
                    events.borrow_mut().push(match event {
                        TrustedWorktreesEvent::Trusted(host, paths) => {
                            TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                        }
                        TrustedWorktreesEvent::Restricted(host, paths) => {
                            TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                        }
                    });
                })
            }
        })
        .detach();

        let can_trust = trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(
            !can_trust,
            "single-file worktree should be restricted by default"
        );

        {
            let events = events.borrow();
            assert_eq!(events.len(), 1);
            match &events[0] {
                TrustedWorktreesEvent::Restricted(host, paths) => {
                    assert!(host.is_none());
                    assert!(paths.contains(&PathTrust::Worktree(worktree_id)));
                }
                _ => panic!("expected Restricted event"),
            }
        }

        events.borrow_mut().clear();

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
                None,
                cx,
            );
        });

        {
            let events = events.borrow();
            assert_eq!(events.len(), 1);
            match &events[0] {
                TrustedWorktreesEvent::Trusted(host, paths) => {
                    assert!(host.is_none());
                    assert!(paths.contains(&PathTrust::Worktree(worktree_id)));
                }
                _ => panic!("expected Trusted event"),
            }
        }

        let can_trust_after =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(
            can_trust_after,
            "single-file worktree should be trusted after trust()"
        );
    }

    #[gpui::test]
    async fn test_workspace_trust_unlocks_single_file_worktree(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "foo.rs": "fn foo() {}" }))
            .await;

        let project = Project::test(fs, [path!("/root/foo.rs").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_id = worktree_store.read_with(cx, |store, cx| {
            let worktree = store.worktrees().next().unwrap();
            let worktree = worktree.read(cx);
            assert!(worktree.is_single_file(), "expected single-file worktree");
            worktree.id()
        });

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        let can_trust_workspace =
            trusted_worktrees.update(cx, |store, cx| store.can_trust_workspace(None, cx));
        assert!(
            !can_trust_workspace,
            "workspace should be restricted by default"
        );

        let can_trust_file =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(
            !can_trust_file,
            "single-file worktree should be restricted by default"
        );

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(HashSet::from_iter([PathTrust::Workspace]), None, cx);
        });

        let can_trust_workspace_after =
            trusted_worktrees.update(cx, |store, cx| store.can_trust_workspace(None, cx));
        assert!(
            can_trust_workspace_after,
            "workspace should be trusted after trust(Workspace)"
        );

        let can_trust_file_after =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(
            can_trust_file_after,
            "single-file worktree should be trusted after workspace trust"
        );
    }

    #[gpui::test]
    async fn test_multiple_single_file_worktrees_trust_one(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "a.rs": "fn a() {}",
                "b.rs": "fn b() {}",
                "c.rs": "fn c() {}"
            }),
        )
        .await;

        let project = Project::test(
            fs,
            [
                path!("/root/a.rs").as_ref(),
                path!("/root/b.rs").as_ref(),
                path!("/root/c.rs").as_ref(),
            ],
            cx,
        )
        .await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
            store
                .worktrees()
                .map(|worktree| {
                    let worktree = worktree.read(cx);
                    assert!(worktree.is_single_file());
                    worktree.id()
                })
                .collect()
        });
        assert_eq!(worktree_ids.len(), 3);

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        for &worktree_id in &worktree_ids {
            let can_trust =
                trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
            assert!(
                !can_trust,
                "worktree {worktree_id:?} should be restricted initially"
            );
        }

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(worktree_ids[1])]),
                None,
                cx,
            );
        });

        let can_trust_0 =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_ids[0], cx));
        let can_trust_1 =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_ids[1], cx));
        let can_trust_2 =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_ids[2], cx));

        assert!(!can_trust_0, "worktree 0 should still be restricted");
        assert!(can_trust_1, "worktree 1 should be trusted");
        assert!(!can_trust_2, "worktree 2 should still be restricted");
    }

    #[gpui::test]
    async fn test_two_directory_worktrees_separate_trust(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/projects"),
            json!({
                "project_a": { "main.rs": "fn main() {}" },
                "project_b": { "lib.rs": "pub fn lib() {}" }
            }),
        )
        .await;

        let project = Project::test(
            fs,
            [
                path!("/projects/project_a").as_ref(),
                path!("/projects/project_b").as_ref(),
            ],
            cx,
        )
        .await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
            store
                .worktrees()
                .map(|worktree| {
                    let worktree = worktree.read(cx);
                    assert!(!worktree.is_single_file());
                    worktree.id()
                })
                .collect()
        });
        assert_eq!(worktree_ids.len(), 2);

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        let can_trust_a =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_ids[0], cx));
        let can_trust_b =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_ids[1], cx));
        assert!(!can_trust_a, "project_a should be restricted initially");
        assert!(!can_trust_b, "project_b should be restricted initially");

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(worktree_ids[0])]),
                None,
                cx,
            );
        });

        let can_trust_a =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_ids[0], cx));
        let can_trust_b =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_ids[1], cx));
        assert!(can_trust_a, "project_a should be trusted after trust()");
        assert!(!can_trust_b, "project_b should still be restricted");

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(worktree_ids[1])]),
                None,
                cx,
            );
        });

        let can_trust_a =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_ids[0], cx));
        let can_trust_b =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_ids[1], cx));
        assert!(can_trust_a, "project_a should remain trusted");
        assert!(can_trust_b, "project_b should now be trusted");
    }

    #[gpui::test]
    async fn test_directory_worktree_trust_enables_workspace(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "main.rs": "fn main() {}" }))
            .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_id = worktree_store.read_with(cx, |store, cx| {
            let worktree = store.worktrees().next().unwrap();
            assert!(!worktree.read(cx).is_single_file());
            worktree.read(cx).id()
        });

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        let can_trust_workspace =
            trusted_worktrees.update(cx, |store, cx| store.can_trust_workspace(None, cx));
        assert!(
            !can_trust_workspace,
            "workspace should be restricted initially"
        );

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
                None,
                cx,
            );
        });

        let can_trust_workspace_after =
            trusted_worktrees.update(cx, |store, cx| store.can_trust_workspace(None, cx));
        assert!(
            can_trust_workspace_after,
            "workspace should be trusted after trusting directory worktree"
        );
    }

    #[gpui::test]
    async fn test_directory_worktree_trust_enables_single_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "project": { "main.rs": "fn main() {}" },
                "standalone.rs": "fn standalone() {}"
            }),
        )
        .await;

        let project = Project::test(
            fs,
            [path!("/project").as_ref(), path!("/standalone.rs").as_ref()],
            cx,
        )
        .await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let (dir_worktree_id, file_worktree_id) = worktree_store.read_with(cx, |store, cx| {
            let worktrees: Vec<_> = store.worktrees().collect();
            assert_eq!(worktrees.len(), 2);
            let (dir_worktree, file_worktree) = if worktrees[0].read(cx).is_single_file() {
                (&worktrees[1], &worktrees[0])
            } else {
                (&worktrees[0], &worktrees[1])
            };
            assert!(!dir_worktree.read(cx).is_single_file());
            assert!(file_worktree.read(cx).is_single_file());
            (dir_worktree.read(cx).id(), file_worktree.read(cx).id())
        });

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        let can_trust_file =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(file_worktree_id, cx));
        assert!(
            !can_trust_file,
            "single-file worktree should be restricted initially"
        );

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(dir_worktree_id)]),
                None,
                cx,
            );
        });

        let can_trust_dir =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(dir_worktree_id, cx));
        let can_trust_file_after =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(file_worktree_id, cx));
        assert!(can_trust_dir, "directory worktree should be trusted");
        assert!(
            can_trust_file_after,
            "single-file worktree should be trusted after directory worktree trust"
        );
    }

    #[gpui::test]
    async fn test_abs_path_trust_covers_multiple_worktrees(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/workspace"),
            json!({
                "project_a": { "main.rs": "fn main() {}" },
                "project_b": { "lib.rs": "pub fn lib() {}" }
            }),
        )
        .await;

        let project = Project::test(
            fs,
            [
                path!("/workspace/project_a").as_ref(),
                path!("/workspace/project_b").as_ref(),
            ],
            cx,
        )
        .await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
            store
                .worktrees()
                .map(|worktree| worktree.read(cx).id())
                .collect()
        });
        assert_eq!(worktree_ids.len(), 2);

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        for &worktree_id in &worktree_ids {
            let can_trust =
                trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
            assert!(!can_trust, "worktree should be restricted initially");
        }

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::AbsPath(PathBuf::from(path!("/workspace")))]),
                None,
                cx,
            );
        });

        for &worktree_id in &worktree_ids {
            let can_trust =
                trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
            assert!(
                can_trust,
                "worktree should be trusted after parent path trust"
            );
        }
    }

    #[gpui::test]
    async fn test_auto_trust_all(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "project_a": { "main.rs": "fn main() {}" },
                "project_b": { "lib.rs": "pub fn lib() {}" },
                "single.rs": "fn single() {}"
            }),
        )
        .await;

        let project = Project::test(
            fs,
            [
                path!("/project_a").as_ref(),
                path!("/project_b").as_ref(),
                path!("/single.rs").as_ref(),
            ],
            cx,
        )
        .await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
            store
                .worktrees()
                .map(|worktree| worktree.read(cx).id())
                .collect()
        });
        assert_eq!(worktree_ids.len(), 3);

        let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

        let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
        cx.update({
            let events = events.clone();
            |cx| {
                cx.subscribe(&trusted_worktrees, move |_, event, _| {
                    events.borrow_mut().push(match event {
                        TrustedWorktreesEvent::Trusted(host, paths) => {
                            TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                        }
                        TrustedWorktreesEvent::Restricted(host, paths) => {
                            TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                        }
                    });
                })
            }
        })
        .detach();

        for &worktree_id in &worktree_ids {
            let can_trust =
                trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
            assert!(!can_trust, "worktree should be restricted initially");
        }
        let can_trust_workspace =
            trusted_worktrees.update(cx, |store, cx| store.can_trust_workspace(None, cx));
        assert!(
            !can_trust_workspace,
            "workspace should be restricted initially"
        );

        let has_restricted = trusted_worktrees.read_with(cx, |store, cx| {
            store.has_restricted_worktrees(&worktree_store, cx)
        });
        assert!(has_restricted, "should have restricted worktrees");

        events.borrow_mut().clear();

        trusted_worktrees.update(cx, |store, cx| {
            store.auto_trust_all(cx);
        });

        for &worktree_id in &worktree_ids {
            let can_trust =
                trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
            assert!(
                can_trust,
                "worktree {worktree_id:?} should be trusted after auto_trust_all"
            );
        }

        let can_trust_workspace =
            trusted_worktrees.update(cx, |store, cx| store.can_trust_workspace(None, cx));
        assert!(
            can_trust_workspace,
            "workspace should be trusted after auto_trust_all"
        );

        let has_restricted_after = trusted_worktrees.read_with(cx, |store, cx| {
            store.has_restricted_worktrees(&worktree_store, cx)
        });
        assert!(
            !has_restricted_after,
            "should have no restricted worktrees after auto_trust_all"
        );

        let trusted_event_count = events
            .borrow()
            .iter()
            .filter(|e| matches!(e, TrustedWorktreesEvent::Trusted(..)))
            .count();
        assert!(
            trusted_event_count > 0,
            "should have emitted Trusted events"
        );
    }

    #[gpui::test]
    async fn test_wait_for_global_trust_already_trusted(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "main.rs": "fn main() {}" }))
            .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(HashSet::from_iter([PathTrust::Workspace]), None, cx);
        });

        let task = cx.update(|cx| wait_for_workspace_trust(None::<RemoteHostLocation>, "test", cx));
        assert!(task.is_none(), "should return None when already trusted");
    }

    #[gpui::test]
    async fn test_wait_for_workspace_trust_resolves_on_trust(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "main.rs": "fn main() {}" }))
            .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        let task = cx.update(|cx| wait_for_workspace_trust(None::<RemoteHostLocation>, "test", cx));
        assert!(
            task.is_some(),
            "should return Some(Task) when not yet trusted"
        );

        let task = task.unwrap();

        cx.executor().run_until_parked();

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(HashSet::from_iter([PathTrust::Workspace]), None, cx);
        });

        cx.executor().run_until_parked();
        task.await;
    }

    #[gpui::test]
    async fn test_wait_for_default_workspace_trust_resolves_on_directory_worktree_trust(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "main.rs": "fn main() {}" }))
            .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_id = worktree_store.read_with(cx, |store, cx| {
            let worktree = store.worktrees().next().unwrap();
            assert!(!worktree.read(cx).is_single_file());
            worktree.read(cx).id()
        });

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        let task = cx.update(|cx| wait_for_default_workspace_trust("test", cx));
        assert!(
            task.is_some(),
            "should return Some(Task) when not yet trusted"
        );

        let task = task.unwrap();

        cx.executor().run_until_parked();

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
                None,
                cx,
            );
        });

        cx.executor().run_until_parked();
        task.await;
    }

    #[gpui::test]
    async fn test_trust_restrict_trust_cycle(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "main.rs": "fn main() {}" }))
            .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_id = worktree_store.read_with(cx, |store, cx| {
            store.worktrees().next().unwrap().read(cx).id()
        });

        let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

        let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
        cx.update({
            let events = events.clone();
            |cx| {
                cx.subscribe(&trusted_worktrees, move |_, event, _| {
                    events.borrow_mut().push(match event {
                        TrustedWorktreesEvent::Trusted(host, paths) => {
                            TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                        }
                        TrustedWorktreesEvent::Restricted(host, paths) => {
                            TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                        }
                    });
                })
            }
        })
        .detach();

        let can_trust = trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(!can_trust, "should be restricted initially");
        assert_eq!(events.borrow().len(), 1);
        events.borrow_mut().clear();

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
                None,
                cx,
            );
        });
        let can_trust = trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(can_trust, "should be trusted after trust()");
        assert_eq!(events.borrow().len(), 1);
        assert!(matches!(
            &events.borrow()[0],
            TrustedWorktreesEvent::Trusted(..)
        ));
        events.borrow_mut().clear();

        trusted_worktrees.update(cx, |store, cx| {
            store.restrict(
                HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
                None,
                cx,
            );
        });
        let can_trust = trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(!can_trust, "should be restricted after restrict()");
        assert_eq!(events.borrow().len(), 1);
        assert!(matches!(
            &events.borrow()[0],
            TrustedWorktreesEvent::Restricted(..)
        ));

        let has_restricted = trusted_worktrees.read_with(cx, |store, cx| {
            store.has_restricted_worktrees(&worktree_store, cx)
        });
        assert!(has_restricted);
        events.borrow_mut().clear();

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
                None,
                cx,
            );
        });
        let can_trust = trusted_worktrees.update(cx, |store, cx| store.can_trust(worktree_id, cx));
        assert!(can_trust, "should be trusted again after second trust()");
        assert_eq!(events.borrow().len(), 1);
        assert!(matches!(
            &events.borrow()[0],
            TrustedWorktreesEvent::Trusted(..)
        ));

        let has_restricted = trusted_worktrees.read_with(cx, |store, cx| {
            store.has_restricted_worktrees(&worktree_store, cx)
        });
        assert!(!has_restricted);
    }

    #[gpui::test]
    async fn test_multi_host_trust_isolation(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "local_project": { "main.rs": "fn main() {}" },
                "remote_project": { "lib.rs": "pub fn lib() {}" }
            }),
        )
        .await;

        let project = Project::test(
            fs,
            [
                path!("/local_project").as_ref(),
                path!("/remote_project").as_ref(),
            ],
            cx,
        )
        .await;
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
            store
                .worktrees()
                .map(|worktree| worktree.read(cx).id())
                .collect()
        });
        assert_eq!(worktree_ids.len(), 2);
        let local_worktree = worktree_ids[0];
        let _remote_worktree = worktree_ids[1];

        let trusted_worktrees = init_trust_global(worktree_store, cx);

        let host_a: Option<RemoteHostLocation> = None;
        let host_b = Some(RemoteHostLocation {
            user_name: Some("user".into()),
            host_identifier: "remote-host".into(),
        });

        let can_trust_local =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(local_worktree, cx));
        assert!(!can_trust_local, "local worktree restricted on host_a");

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Workspace]),
                host_b.clone(),
                cx,
            );
        });

        let can_trust_workspace_a = trusted_worktrees.update(cx, |store, cx| {
            store.can_trust_workspace(host_a.clone(), cx)
        });
        assert!(
            !can_trust_workspace_a,
            "host_a workspace should still be restricted"
        );

        let can_trust_workspace_b = trusted_worktrees.update(cx, |store, cx| {
            store.can_trust_workspace(host_b.clone(), cx)
        });
        assert!(can_trust_workspace_b, "host_b workspace should be trusted");

        trusted_worktrees.update(cx, |store, cx| {
            store.trust(
                HashSet::from_iter([PathTrust::Worktree(local_worktree)]),
                host_a.clone(),
                cx,
            );
        });

        let can_trust_local_after =
            trusted_worktrees.update(cx, |store, cx| store.can_trust(local_worktree, cx));
        assert!(
            can_trust_local_after,
            "local worktree should be trusted on host_a"
        );

        let can_trust_workspace_a_after = trusted_worktrees.update(cx, |store, cx| {
            store.can_trust_workspace(host_a.clone(), cx)
        });
        assert!(
            can_trust_workspace_a_after,
            "host_a workspace should be trusted after directory trust"
        );
    }
}
