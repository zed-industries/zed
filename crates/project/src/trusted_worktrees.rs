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
//! Zed does not consider invisible, `worktree.is_visible() == false` worktrees in Zed, as those are programmatically created inside Zed for internal needs, e.g. a tmp dir for `keymap_editor.rs` needs.
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
//! * "directory worktree"
//!
//! If a directory is open in Zed, it's a full worktree which may spawn multiple language servers associated with it.
//! Each such worktree requires a separate trust permission, so each separate directory worktree has to be trusted separately, unless a more global level is trusted.
//!
//! When a directory worktree is trusted and language servers are allowed to be downloaded and started, hence, "single file worktree" level of trust also.
//!
//! * "path override"
//!
//! To ease trusting multiple directory worktrees at once, it's possible to trust a parent directory of a certain directory worktree opened in Zed.
//! Trusting a directory means trusting all its subdirectories as well, including all current and potential directory worktrees.

use client::ProjectId;
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

pub fn init(db_trusted_paths: DbTrustedPaths, cx: &mut App) {
    if TrustedWorktrees::try_get_global(cx).is_none() {
        let trusted_worktrees = cx.new(|_| TrustedWorktreesStore::new(db_trusted_paths));
        cx.set_global(TrustedWorktrees(trusted_worktrees))
    }
}

/// An initialization call to set up trust global for a particular project (remote or local).
pub fn track_worktree_trust(
    worktree_store: Entity<WorktreeStore>,
    remote_host: Option<RemoteHostLocation>,
    downstream_client: Option<(AnyProtoClient, ProjectId)>,
    upstream_client: Option<(AnyProtoClient, ProjectId)>,
    cx: &mut App,
) {
    match TrustedWorktrees::try_get_global(cx) {
        Some(trusted_worktrees) => {
            trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                trusted_worktrees.add_worktree_store(
                    worktree_store.clone(),
                    remote_host,
                    downstream_client,
                    upstream_client.clone(),
                    cx,
                );

                if let Some((upstream_client, upstream_project_id)) = upstream_client {
                    let trusted_paths = trusted_worktrees
                        .trusted_paths
                        .get(&worktree_store.downgrade())
                        .into_iter()
                        .flatten()
                        .map(|trusted_path| trusted_path.to_proto())
                        .collect::<Vec<_>>();
                    if !trusted_paths.is_empty() {
                        upstream_client
                            .send(proto::TrustWorktrees {
                                project_id: upstream_project_id.0,
                                trusted_paths,
                            })
                            .ok();
                    }
                }
            });
        }
        None => log::debug!("No TrustedWorktrees initialized, not tracking worktree trust"),
    }
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
#[derive(Debug)]
pub struct TrustedWorktreesStore {
    worktree_stores: HashMap<WeakEntity<WorktreeStore>, StoreData>,
    db_trusted_paths: DbTrustedPaths,
    trusted_paths: TrustedPaths,
    restricted: HashMap<WeakEntity<WorktreeStore>, HashSet<WorktreeId>>,
    worktree_trust_serialization: Task<()>,
}

#[derive(Debug, Default)]
struct StoreData {
    upstream_client: Option<(AnyProtoClient, ProjectId)>,
    downstream_client: Option<(AnyProtoClient, ProjectId)>,
    host: Option<RemoteHostLocation>,
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
            #[cfg(feature = "test-support")]
            RemoteConnectionOptions::Mock(mock) => {
                (None, SharedString::new(format!("mock-{}", mock.id)))
            }
        };
        Self {
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
        })
    }
}

/// A change of trust on a certain host.
#[derive(Debug)]
pub enum TrustedWorktreesEvent {
    Trusted(WeakEntity<WorktreeStore>, HashSet<PathTrust>),
    Restricted(WeakEntity<WorktreeStore>, HashSet<PathTrust>),
}

impl EventEmitter<TrustedWorktreesEvent> for TrustedWorktreesStore {}

type TrustedPaths = HashMap<WeakEntity<WorktreeStore>, HashSet<PathTrust>>;
pub type DbTrustedPaths = HashMap<Option<RemoteHostLocation>, HashSet<PathBuf>>;

impl TrustedWorktreesStore {
    fn new(db_trusted_paths: DbTrustedPaths) -> Self {
        Self {
            db_trusted_paths,
            trusted_paths: HashMap::default(),
            worktree_stores: HashMap::default(),
            restricted: HashMap::default(),
            worktree_trust_serialization: Task::ready(()),
        }
    }

    /// Whether a particular worktree store has associated worktrees that are restricted, or an associated host is restricted.
    pub fn has_restricted_worktrees(
        &self,
        worktree_store: &Entity<WorktreeStore>,
        cx: &App,
    ) -> bool {
        self.restricted
            .get(&worktree_store.downgrade())
            .is_some_and(|restricted_worktrees| {
                restricted_worktrees.iter().any(|restricted_worktree| {
                    worktree_store
                        .read(cx)
                        .worktree_for_id(*restricted_worktree, cx)
                        .is_some()
                })
            })
    }

    #[cfg(feature = "test-support")]
    pub fn restricted_worktrees_for_store(
        &self,
        worktree_store: &Entity<WorktreeStore>,
    ) -> HashSet<WorktreeId> {
        self.restricted
            .get(&worktree_store.downgrade())
            .unwrap()
            .clone()
    }

    /// Adds certain entities on this host to the trusted list.
    /// This will emit [`TrustedWorktreesEvent::Trusted`] event for all passed entries
    /// and the ones that got auto trusted based on trust hierarchy (see module-level docs).
    pub fn trust(
        &mut self,
        worktree_store: &Entity<WorktreeStore>,
        mut trusted_paths: HashSet<PathTrust>,
        cx: &mut Context<Self>,
    ) {
        let weak_worktree_store = worktree_store.downgrade();
        let mut new_trusted_single_file_worktrees = HashSet::default();
        let mut new_trusted_other_worktrees = HashSet::default();
        let mut new_trusted_abs_paths = HashSet::default();
        for trusted_path in trusted_paths.iter().chain(
            self.trusted_paths
                .remove(&weak_worktree_store)
                .iter()
                .flat_map(|current_trusted| current_trusted.iter()),
        ) {
            match trusted_path {
                PathTrust::Worktree(worktree_id) => {
                    if let Some(restricted_worktrees) =
                        self.restricted.get_mut(&weak_worktree_store)
                    {
                        restricted_worktrees.remove(worktree_id);
                        if restricted_worktrees.is_empty() {
                            self.restricted.remove(&weak_worktree_store);
                        }
                    };

                    if let Some(worktree) =
                        worktree_store.read(cx).worktree_for_id(*worktree_id, cx)
                    {
                        if worktree.read(cx).is_single_file() {
                            new_trusted_single_file_worktrees.insert(*worktree_id);
                        } else {
                            new_trusted_other_worktrees
                                .insert((worktree.read(cx).abs_path(), *worktree_id));
                        }
                    }
                }
                PathTrust::AbsPath(abs_path) => {
                    debug_assert!(
                        util::paths::is_absolute(
                            &abs_path.to_string_lossy(),
                            worktree_store.read(cx).path_style()
                        ),
                        "Cannot trust non-absolute path {abs_path:?} on path style {style:?}",
                        style = worktree_store.read(cx).path_style()
                    );
                    if let Some((worktree_id, is_file)) =
                        find_worktree_in_store(worktree_store.read(cx), abs_path, cx)
                    {
                        if is_file {
                            new_trusted_single_file_worktrees.insert(worktree_id);
                        } else {
                            new_trusted_other_worktrees
                                .insert((Arc::from(abs_path.as_path()), worktree_id));
                        }
                    }
                    new_trusted_abs_paths.insert(abs_path.clone());
                }
            }
        }

        new_trusted_other_worktrees.retain(|(worktree_abs_path, _)| {
            new_trusted_abs_paths
                .iter()
                .all(|new_trusted_path| !worktree_abs_path.starts_with(new_trusted_path))
        });
        if !new_trusted_other_worktrees.is_empty() {
            new_trusted_single_file_worktrees.clear();
        }

        if let Some(restricted_worktrees) = self.restricted.remove(&weak_worktree_store) {
            let new_restricted_worktrees = restricted_worktrees
                .into_iter()
                .filter(|restricted_worktree| {
                    let Some(worktree) = worktree_store
                        .read(cx)
                        .worktree_for_id(*restricted_worktree, cx)
                    else {
                        return false;
                    };
                    let is_file = worktree.read(cx).is_single_file();

                    // When trusting an abs path on the host, we transitively trust all single file worktrees on this host too.
                    if is_file && !new_trusted_abs_paths.is_empty() {
                        trusted_paths.insert(PathTrust::Worktree(*restricted_worktree));
                        return false;
                    }

                    let restricted_worktree_path = worktree.read(cx).abs_path();
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
            self.restricted
                .insert(weak_worktree_store.clone(), new_restricted_worktrees);
        }

        {
            let trusted_paths = self
                .trusted_paths
                .entry(weak_worktree_store.clone())
                .or_default();
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
        }

        if let Some(store_data) = self.worktree_stores.get(&weak_worktree_store) {
            if let Some((upstream_client, upstream_project_id)) = &store_data.upstream_client {
                let trusted_paths = trusted_paths
                    .iter()
                    .map(|trusted_path| trusted_path.to_proto())
                    .collect::<Vec<_>>();
                if !trusted_paths.is_empty() {
                    upstream_client
                        .send(proto::TrustWorktrees {
                            project_id: upstream_project_id.0,
                            trusted_paths,
                        })
                        .ok();
                }
            }
        }
        cx.emit(TrustedWorktreesEvent::Trusted(
            weak_worktree_store,
            trusted_paths,
        ));
    }

    /// Restricts certain entities on this host.
    /// This will emit [`TrustedWorktreesEvent::Restricted`] event for all passed entries.
    pub fn restrict(
        &mut self,
        worktree_store: WeakEntity<WorktreeStore>,
        restricted_paths: HashSet<PathTrust>,
        cx: &mut Context<Self>,
    ) {
        let mut restricted = HashSet::default();
        for restricted_path in restricted_paths {
            match restricted_path {
                PathTrust::Worktree(worktree_id) => {
                    self.restricted
                        .entry(worktree_store.clone())
                        .or_default()
                        .insert(worktree_id);
                    restricted.insert(PathTrust::Worktree(worktree_id));
                }
                PathTrust::AbsPath(..) => debug_panic!("Unexpected: cannot restrict an abs path"),
            }
        }

        cx.emit(TrustedWorktreesEvent::Restricted(
            worktree_store,
            restricted,
        ));
    }

    /// Erases all trust information.
    /// Requires Zed's restart to take proper effect.
    pub fn clear_trusted_paths(&mut self) {
        self.trusted_paths.clear();
        self.db_trusted_paths.clear();
    }

    /// Checks whether a certain worktree is trusted (or on a larger trust level).
    /// If not, emits [`TrustedWorktreesEvent::Restricted`] event if for the first time and not trusted, or no corresponding worktree store was found.
    ///
    /// No events or data adjustment happens when `trust_all_worktrees` auto trust is enabled.
    pub fn can_trust(
        &mut self,
        worktree_store: &Entity<WorktreeStore>,
        worktree_id: WorktreeId,
        cx: &mut Context<Self>,
    ) -> bool {
        if ProjectSettings::get_global(cx).session.trust_all_worktrees {
            return true;
        }

        let weak_worktree_store = worktree_store.downgrade();
        let Some(worktree) = worktree_store.read(cx).worktree_for_id(worktree_id, cx) else {
            return false;
        };
        let worktree_path = worktree.read(cx).abs_path();
        // Zed opened an "internal" directory: e.g. a tmp dir for `keymap_editor.rs` needs.
        if !worktree.read(cx).is_visible() {
            log::debug!("Skipping worktree trust checks for not visible {worktree_path:?}");
            return true;
        }

        let is_file = worktree.read(cx).is_single_file();
        if self
            .restricted
            .get(&weak_worktree_store)
            .is_some_and(|restricted_worktrees| restricted_worktrees.contains(&worktree_id))
        {
            return false;
        }

        if self
            .trusted_paths
            .get(&weak_worktree_store)
            .is_some_and(|trusted_paths| trusted_paths.contains(&PathTrust::Worktree(worktree_id)))
        {
            return true;
        }

        // * Single files are auto-approved when something else (not a single file) was approved on this host already.
        // * If parent path is trusted already, this worktree is stusted also.
        //
        // See module documentation for details on trust level.
        if let Some(trusted_paths) = self.trusted_paths.get(&weak_worktree_store) {
            let auto_trusted = worktree_store.read_with(cx, |worktree_store, cx| {
                trusted_paths.iter().any(|trusted_path| match trusted_path {
                    PathTrust::Worktree(worktree_id) => worktree_store
                        .worktree_for_id(*worktree_id, cx)
                        .is_some_and(|worktree| {
                            let worktree = worktree.read(cx);
                            worktree_path.starts_with(&worktree.abs_path())
                                || (is_file && !worktree.is_single_file())
                        }),
                    PathTrust::AbsPath(trusted_path) => {
                        is_file || worktree_path.starts_with(trusted_path)
                    }
                })
            });
            if auto_trusted {
                return true;
            }
        }

        self.restricted
            .entry(weak_worktree_store.clone())
            .or_default()
            .insert(worktree_id);
        log::info!("Worktree {worktree_path:?} is not trusted");
        if let Some(store_data) = self.worktree_stores.get(&weak_worktree_store) {
            if let Some((downstream_client, downstream_project_id)) = &store_data.downstream_client
            {
                downstream_client
                    .send(proto::RestrictWorktrees {
                        project_id: downstream_project_id.0,
                        worktree_ids: vec![worktree_id.to_proto()],
                    })
                    .ok();
            }
            if let Some((upstream_client, upstream_project_id)) = &store_data.upstream_client {
                upstream_client
                    .send(proto::RestrictWorktrees {
                        project_id: upstream_project_id.0,
                        worktree_ids: vec![worktree_id.to_proto()],
                    })
                    .ok();
            }
        }
        cx.emit(TrustedWorktreesEvent::Restricted(
            weak_worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
        ));
        false
    }

    /// Lists all explicitly restricted worktrees (via [`TrustedWorktreesStore::can_trust`] method calls) for a particular worktree store on a particular host.
    pub fn restricted_worktrees(
        &self,
        worktree_store: &Entity<WorktreeStore>,
        cx: &App,
    ) -> HashSet<(WorktreeId, Arc<Path>)> {
        let mut single_file_paths = HashSet::default();

        let other_paths = self
            .restricted
            .get(&worktree_store.downgrade())
            .into_iter()
            .flatten()
            .filter_map(|&restricted_worktree_id| {
                let worktree = worktree_store
                    .read(cx)
                    .worktree_for_id(restricted_worktree_id, cx)?;
                let worktree = worktree.read(cx);
                let abs_path = worktree.abs_path();
                if worktree.is_single_file() {
                    single_file_paths.insert((restricted_worktree_id, abs_path));
                    None
                } else {
                    Some((restricted_worktree_id, abs_path))
                }
            })
            .collect::<HashSet<_>>();

        if !other_paths.is_empty() {
            return other_paths;
        } else {
            single_file_paths
        }
    }

    /// Switches the "trust nothing" mode to "automatically trust everything".
    /// This does not influence already persisted data, but stops adding new worktrees there.
    pub fn auto_trust_all(&mut self, cx: &mut Context<Self>) {
        for (worktree_store, worktrees) in std::mem::take(&mut self.restricted).into_iter().fold(
            HashMap::default(),
            |mut acc, (remote_host, worktrees)| {
                acc.entry(remote_host)
                    .or_insert_with(HashSet::default)
                    .extend(worktrees.into_iter().map(PathTrust::Worktree));
                acc
            },
        ) {
            if let Some(worktree_store) = worktree_store.upgrade() {
                self.trust(&worktree_store, worktrees, cx);
            }
        }
    }

    pub fn schedule_serialization<S>(&mut self, cx: &mut Context<Self>, serialize: S)
    where
        S: FnOnce(HashMap<Option<RemoteHostLocation>, HashSet<PathBuf>>, &App) -> Task<()>
            + 'static,
    {
        self.worktree_trust_serialization = serialize(self.trusted_paths_for_serialization(cx), cx);
    }

    fn trusted_paths_for_serialization(
        &mut self,
        cx: &mut Context<Self>,
    ) -> HashMap<Option<RemoteHostLocation>, HashSet<PathBuf>> {
        let new_trusted_paths = self
            .trusted_paths
            .iter()
            .filter_map(|(worktree_store, paths)| {
                let host = self.worktree_stores.get(&worktree_store)?.host.clone();
                let abs_paths = paths
                    .iter()
                    .flat_map(|path| match path {
                        PathTrust::Worktree(worktree_id) => worktree_store
                            .upgrade()
                            .and_then(|worktree_store| {
                                worktree_store.read(cx).worktree_for_id(*worktree_id, cx)
                            })
                            .map(|worktree| worktree.read(cx).abs_path().to_path_buf()),
                        PathTrust::AbsPath(abs_path) => Some(abs_path.clone()),
                    })
                    .collect::<HashSet<_>>();
                Some((host, abs_paths))
            })
            .chain(self.db_trusted_paths.drain())
            .fold(HashMap::default(), |mut acc, (host, paths)| {
                acc.entry(host)
                    .or_insert_with(HashSet::default)
                    .extend(paths);
                acc
            });

        self.db_trusted_paths = new_trusted_paths.clone();
        new_trusted_paths
    }

    fn add_worktree_store(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        remote_host: Option<RemoteHostLocation>,
        downstream_client: Option<(AnyProtoClient, ProjectId)>,
        upstream_client: Option<(AnyProtoClient, ProjectId)>,
        cx: &mut Context<Self>,
    ) {
        self.worktree_stores
            .retain(|worktree_store, _| worktree_store.is_upgradable());
        let weak_worktree_store = worktree_store.downgrade();
        self.worktree_stores.insert(
            weak_worktree_store.clone(),
            StoreData {
                host: remote_host.clone(),
                downstream_client,
                upstream_client,
            },
        );

        let mut new_trusted_paths = HashSet::default();
        if let Some(db_trusted_paths) = self.db_trusted_paths.get(&remote_host) {
            new_trusted_paths.extend(db_trusted_paths.clone().into_iter().map(PathTrust::AbsPath));
        }
        if let Some(trusted_paths) = self.trusted_paths.remove(&weak_worktree_store) {
            new_trusted_paths.extend(trusted_paths);
        }
        if !new_trusted_paths.is_empty() {
            self.trusted_paths.insert(
                weak_worktree_store,
                new_trusted_paths
                    .into_iter()
                    .map(|path_trust| match path_trust {
                        PathTrust::AbsPath(abs_path) => {
                            find_worktree_in_store(worktree_store.read(cx), &abs_path, cx)
                                .map(|(worktree_id, _)| PathTrust::Worktree(worktree_id))
                                .unwrap_or_else(|| PathTrust::AbsPath(abs_path))
                        }
                        other => other,
                    })
                    .collect(),
            );
        }
    }
}

fn find_worktree_in_store(
    worktree_store: &WorktreeStore,
    abs_path: &Path,
    cx: &App,
) -> Option<(WorktreeId, bool)> {
    let (worktree, path_in_worktree) = worktree_store.find_worktree(&abs_path, cx)?;
    if path_in_worktree.is_empty() {
        Some((worktree.read(cx).id(), worktree.read(cx).is_single_file()))
    } else {
        None
    }
}
