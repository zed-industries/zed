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
//! From the least to the most trust level:
//!
//! * "single file worktree"
//!
//! After opening an empty Zed it's possible to open just a file, same as after opening a directory in Zed it's possible to open a file outside of this directory.
//! Usual scenario for both cases is opening Zed's settings.json file via `zed: open settings file` command: that starts a language server for a new file open, which originates from a newly created, single file worktree.
//!
//! Spawning a language server is potentially dangerous, and Zed needs to restrict that by default.
//! Each single file worktree requires a separate trust permission, unless a more global level is trusted.
//!
//! * "global"
//!
//! Even an empty Zed instance with no files or directories open is potentially dangerous: opening an Assistant Panel and creating new external agent thread might require installing and running MCP servers.
//!
//! Disabling the entire panel is possible with ai-related settings.
//! Yet when it's enabled, it's still reasonably safe to use remote AI agents and control their permissions in the Assistant Panel.
//!
//! Unlike that, MCP servers are similar to language servers and may require fetching, installing and running packages or binaries.
//! Given that those servers are not tied to any particular worktree, this level of trust is required to operate any MCP server.
//!
//! Global level of trust assumes all single file worktrees are trusted too, for the same host: if we allow global MCP server-related functionality, we can already allow spawning language servers for single file worktrees as well.
//!
//! * "directory worktree"
//!
//! If a directory is open in Zed, it's a full worktree which may spawn multiple language servers associated with it.
//! Each such worktree requires a separate trust permission, so each separate directory worktree has to be trusted separately, unless a more global level is trusted.
//!
//! When a directory worktree is trusted and language servers are allowed to be downloaded and started, hence we also allow "global" level of trust (hence, "single file worktree" level of trust also).
//!
//! * "path override"
//!
//! To ease trusting multiple directory worktrees at once, it's possible to trust a parent directory of a certain directory worktree opened in Zed.
//! Trusting a directory means trusting all its subdirectories as well, including all current and potential directory worktrees.
//!
//! If we trust multiple projects to install and spawn various language server processes, we can also allow global trust requests for MCP servers installation and spawning.

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

/// An initialization call to set up trust global for a particular project (remote or local).
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
                TrustedWorktreesStore::new(
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

/// Waits until at least [`PathTrust::Global`] level of trust is granted for a particular host.
pub fn wait_for_global_trust(
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
    trusted_paths: HashMap<Option<RemoteHostLocation>, HashSet<PathTrust>>,
    serialization_task: Task<()>,
    restricted: HashSet<WorktreeId>,
    remote_host: Option<RemoteHostLocation>,
    restricted_globals: HashSet<Option<RemoteHostLocation>>,
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
            host_identifier: host_name,
        }
    }
}

// TODO kb split Worktree into file and directory variants?

/// A unit of trust consideration inside a particular host:
/// either a familiar worktree, or a path that may influence other worktrees' trust.
/// See module-level documentation on the trust model.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PathTrust {
    /// General, no worktrees or files open case.
    /// E.g. MCP servers can be spawned from a blank Zed instance, but will do `npm i` and other potentially malicious actions.
    Global,
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

/// A change of trust on a certain host.
#[derive(Debug)]
pub enum TrustedWorktreesEvent {
    Trusted(Option<RemoteHostLocation>, HashSet<PathTrust>),
    Restricted(Option<RemoteHostLocation>, HashSet<PathTrust>),
}

impl EventEmitter<TrustedWorktreesEvent> for TrustedWorktreesStore {}

impl TrustedWorktreesStore {
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

        Self {
            trusted_paths,
            downstream_client,
            upstream_client,
            remote_host: remote_host.clone(),
            restricted_globals: HashSet::default(),
            restricted: HashSet::default(),
            serialization_task: Task::ready(()),
            worktree_stores: HashMap::from_iter([(worktree_store.downgrade(), remote_host)]),
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
        self.restricted_globals.contains(remote_host)
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
        // TODO kb unit test all this logic
        let mut new_global_trusted = false;
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
                PathTrust::Global => new_global_trusted = true,
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
            self.restricted_globals.remove(&remote_host);
        }
        new_trusted_other_worktrees.retain(|(worktree_abs_path, _)| {
            new_trusted_abs_paths
                .iter()
                .all(|new_trusted_path| !worktree_abs_path.starts_with(new_trusted_path))
        });
        if !new_trusted_other_worktrees.is_empty() {
            new_trusted_single_file_worktrees.clear();
        }
        let previous_restricted = std::mem::take(&mut self.restricted);
        self.restricted = previous_restricted
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
                let retain = (!is_file || new_trusted_other_worktrees.is_empty())
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
                            .map(|(abs_path, ..)| abs_path.to_path_buf()),
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
            // Do not persist auto trusted worktrees
            if !ProjectSettings::get_global(cx).session.trust_all_worktrees {
                self.serialization_task = cx.background_spawn(async move {
                    PROJECT_DB
                        .save_trusted_worktrees(new_trusted_worktrees, new_trusted_globals)
                        .await
                        .log_err();
                });
            }

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

    /// Erases all trust information.
    /// Requires Zed's restart to take proper effect.
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

    /// Checks whether a certain worktree is trusted globally (or on a larger trust level).
    /// If not, emits [`TrustedWorktreesEvent::Restricted`] event if checked for the first time and not trusted.
    ///
    /// No events or data adjustment happens when `trust_all_worktrees` auto trust is enabled.
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
        if self.trusted_paths.contains_key(&remote_host) {
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

    /// Lists all explicitly restricted worktrees (via [`TrustedWorktreesStore::can_trust`] and [`TrustedWorktreesStore::can_trust_global`] method calls) for a particular worktree store on a particular host.
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
        } else if self.restricted_globals.contains(&remote_host) {
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

pub(crate) fn find_worktree_in_store(
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
