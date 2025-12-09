use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use collections::{HashMap, HashSet};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, SharedString, Task, WeakEntity,
};
use remote::RemoteConnectionOptions;
use settings::{Settings as _, WorktreeId};
use util::ResultExt as _;

use crate::{
    persistence::PROJECT_DB, project_settings::ProjectSettings, worktree_store::WorktreeStore,
};

pub fn init_global(
    worktree_store: Entity<WorktreeStore>,
    remote_host: Option<impl Into<RemoteHostLocation> + 'static>,
    cx: &mut App,
) {
    match TrustedWorktrees::try_get_global(cx) {
        Some(trusted_worktrees) => {
            trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                trusted_worktrees.add_worktree_store(worktree_store, remote_host, cx);
            });
        }
        None => {
            cx.spawn(async move |cx| {
                let Ok(trusted_worktrees) = cx.update(|cx| TrustedWorktrees::try_get_global(cx))
                else {
                    return;
                };
                match trusted_worktrees {
                    Some(trusted_worktrees) => {
                        trusted_worktrees
                            .update(cx, |trusted_worktrees, cx| {
                                trusted_worktrees.add_worktree_store(
                                    worktree_store,
                                    remote_host,
                                    cx,
                                );
                            })
                            .log_err();
                    }
                    None => {
                        let Ok(trusted_worktrees) = cx.update(|cx| {
                            TrustedWorktreesStorage::new(worktree_store.clone(), remote_host, cx)
                        }) else {
                            return;
                        };
                        let trusted_worktrees = trusted_worktrees.await;
                        let Ok(trusted_worktrees) = cx.new(|_| trusted_worktrees) else {
                            return;
                        };
                        cx.update(|cx| cx.set_global(TrustedWorktrees(trusted_worktrees)))
                            .ok();
                    }
                }
            })
            .detach();
        }
    }
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
    worktree_stores: HashMap<WeakEntity<WorktreeStore>, Option<RemoteHostLocation>>,
    trusted_paths: HashSet<PathTrust>,
    serialization_task: Task<()>,
    restricted: HashSet<WorktreeId>,
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
        cx: &App,
    ) -> Task<Self> {
        let remote_host = remote_host.map(|remote_host| remote_host.into());
        cx.spawn(async move |cx| {
            let trusted_paths = match cx.update(|cx| {
                PROJECT_DB.fetch_trusted_worktrees(worktree_store.clone(), remote_host.clone(), cx)
            }) {
                Ok(trusted_paths) => match trusted_paths.await {
                    Ok(trusted_paths) => trusted_paths,
                    Err(e) => {
                        log::error!("Failed to do initial trusted worktrees fetch: {e:#}");
                        HashSet::default()
                    }
                },
                Err(_window_closed) => HashSet::default(),
            };
            Self {
                trusted_paths,
                restricted: HashSet::default(),
                restricted_globals: HashSet::from_iter([remote_host.clone()]),
                serialization_task: Task::ready(()),
                worktree_stores: HashMap::from_iter([(worktree_store.downgrade(), remote_host)]),
            }
        })
    }

    pub fn has_restricted_worktrees(&self) -> bool {
        !self.restricted.is_empty() || !self.restricted_globals.is_empty()
    }

    pub fn has_global_trust(&self, remote_host: Option<impl Into<RemoteHostLocation>>) -> bool {
        let remote_host = remote_host.map(|remote_host| remote_host.into());
        self.trusted_paths.contains(&PathTrust::Global(remote_host))
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
                                let Some((restricted_worktree_path, restricted_host)) =
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
                            .map(|(abs_path, remote_host)| (abs_path.to_path_buf(), remote_host)),
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

        let closure_new_trusted_globals = new_trusted_globals.clone();
        self.serialization_task = cx.background_spawn(async move {
            PROJECT_DB
                .save_trusted_worktrees(new_trusted_worktrees, closure_new_trusted_globals)
                .await
                .log_err();
        });

        // Trusting a local worktree means trusting the global cases around it too.
        trusted_paths.extend(new_trusted_globals.into_iter().map(PathTrust::Global));
        cx.emit(TrustedWorktreesEvent::Trusted(trusted_paths));
    }

    pub fn clear_trusted_paths(&mut self, cx: &App) -> Task<()> {
        self.trusted_paths.clear();
        let (tx, rx) = smol::channel::bounded(1);
        self.serialization_task = cx.background_spawn(async move {
            PROJECT_DB.clear_worktrees().await.log_err();
            tx.send(()).await.ok();
        });
        cx.background_spawn(async move {
            rx.recv().await.ok();
        })
    }

    /// Checks whether a certain worktree is trusted.
    /// If not, emits [`TrustedWorktreesEvent::Restricted`] event.
    pub fn can_trust(&mut self, worktree: WorktreeId, cx: &mut Context<Self>) -> bool {
        if ProjectSettings::get_global(cx).session.trust_all_worktrees {
            return true;
        }
        if self.restricted.contains(&worktree) {
            return false;
        }
        if self.trusted_paths.contains(&PathTrust::Worktree(worktree)) {
            return true;
        }

        if let Some((worktree_path, remote_host)) = self.find_worktree_data(worktree, cx) {
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

        self.restricted.insert(worktree);
        cx.emit(TrustedWorktreesEvent::Restricted(HashSet::from_iter([
            PathTrust::Worktree(worktree),
        ])));
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
            PathTrust::Global(remote_host),
        ])));
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
    ) -> Option<(Arc<Path>, Option<RemoteHostLocation>)> {
        let mut worktree_data = None;
        self.worktree_stores.retain(
            |worktree_store, remote_host| match worktree_store.upgrade() {
                Some(worktree_store) => {
                    if worktree_data.is_none() {
                        if let Some(worktree) =
                            worktree_store.read(cx).worktree_for_id(worktree_id, cx)
                        {
                            worktree_data =
                                Some((worktree.read(cx).abs_path(), remote_host.clone()));
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
        self.restricted_globals.insert(remote_host.clone());
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
