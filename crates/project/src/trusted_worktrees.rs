//! TODO kb check for other vulnerabilities
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use collections::{HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, SharedString, Task, WeakEntity,
};
use remote::RemoteConnectionOptions;
use settings::{Settings as _, WorktreeId};
use util::ResultExt as _;

use crate::{project_settings::ProjectSettings, worktree_store::WorktreeStore};

const TRUSTED_WORKSPACES_KEY: &str = "trusted_workspaces";
const TRUSTED_WORKSPACES_SEPARATOR: &str = "<|>";

pub fn init_global(
    worktree_store: Entity<WorktreeStore>,
    remote_host: Option<impl Into<RemoteHostData> + 'static>,
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
    worktree_stores: HashMap<WeakEntity<WorktreeStore>, Option<RemoteHostData>>,
    trusted_paths: HashSet<PathTrust>,
    serialization_task: Task<()>,
    restricted: HashSet<WorktreeId>,
}

pub struct RemoteHostData {
    user_name: Option<SharedString>,
    host_name: SharedString,
}

impl From<RemoteConnectionOptions> for RemoteHostData {
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
        RemoteHostData {
            user_name,
            host_name,
        }
    }
}

/// A unit of trust consideration: either a familiar worktree, or a path that may
/// influence other worktrees' trust.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PathTrust {
    /// A worktree that is familiar to this workspace.
    Worktree(WorktreeId),
    /// A path that may be another worktree yet not loaded into workspace,
    /// or a parent path coming out of the security modal.
    AbsPath(PathBuf),
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
        remote_host: Option<impl Into<RemoteHostData>>,
        cx: &App,
    ) -> Task<Self> {
        let remote_host = remote_host.map(|remote_host| remote_host.into());
        cx.spawn(async move |cx| {
            let trusted_paths = cx
                .background_spawn(async move {
                    KEY_VALUE_STORE
                        // TODO kb
                        // * crate a new db table, FK onto remote_hosts DB table data
                        // * store abs paths there still, but without odd separators
                        .read_kvp(TRUSTED_WORKSPACES_KEY)
                        .log_err()
                        .flatten()
                })
                .await;
            Self {
                trusted_paths: trusted_paths
                    .map(|workspaces| {
                        workspaces
                            .split(TRUSTED_WORKSPACES_SEPARATOR)
                            .map(|workspace_path| PathBuf::from(workspace_path))
                            .filter_map(|abs_path| {
                                worktree_store
                                    .read_with(cx, |worktree_store, cx| {
                                        find_worktree_in_store(worktree_store, &abs_path, cx)
                                            .map(PathTrust::Worktree)
                                            .unwrap_or_else(|| PathTrust::AbsPath(abs_path))
                                    })
                                    .ok()
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                restricted: HashSet::default(),
                serialization_task: Task::ready(()),
                worktree_stores: HashMap::from_iter([(worktree_store.downgrade(), remote_host)]),
            }
        })
    }

    /// Adds worktree absolute paths to the trusted list.
    /// This will emit [`TrustedWorktreesEvent::Trusted`] event.
    pub fn trust(&mut self, trusted_paths: HashSet<PathTrust>, cx: &mut Context<'_, Self>) {
        // TODO kb unit test all this logic
        for trusted_path in &trusted_paths {
            match trusted_path {
                PathTrust::Worktree(worktree_id) => {
                    self.restricted.remove(worktree_id);
                    self.trusted_paths.insert(PathTrust::Worktree(*worktree_id));
                }
                PathTrust::AbsPath(path) => {
                    debug_assert!(
                        path.is_absolute(),
                        "Cannot trust non-absolute path {path:?}"
                    );

                    let mut worktree_found = false;
                    self.worktree_stores.retain(|worktree_store, _| {
                        match worktree_store.upgrade() {
                            Some(worktree_store) => {
                                if let Some(worktree_id) =
                                    find_worktree_in_store(worktree_store.read(cx), &path, cx)
                                {
                                    self.restricted.remove(&worktree_id);
                                    self.trusted_paths.insert(PathTrust::Worktree(worktree_id));
                                    worktree_found = true;
                                }
                                true
                            }
                            None => false,
                        }
                    });

                    if !worktree_found {
                        let previous_restricted = std::mem::take(&mut self.restricted);
                        self.restricted = previous_restricted
                            .into_iter()
                            .filter(|restricted_worktree| {
                                let Some(restricted_worktree_path) =
                                    self.find_worktree_path(*restricted_worktree, cx)
                                else {
                                    return false;
                                };
                                !restricted_worktree_path.starts_with(path)
                            })
                            .collect();
                        self.trusted_paths
                            .retain(|trusted_path| match trusted_path {
                                PathTrust::Worktree(_) => true,
                                PathTrust::AbsPath(trusted_abs_path) => {
                                    !trusted_abs_path.starts_with(path)
                                }
                            });
                        self.trusted_paths.insert(PathTrust::AbsPath(path.clone()));
                    }
                }
            }
        }

        let new_worktree_roots =
            self.trusted_paths
                .clone()
                .into_iter()
                .fold(String::new(), |mut acc, path| {
                    if let Some(abs_path) = match path {
                        PathTrust::Worktree(worktree_id) => self
                            .find_worktree_path(worktree_id, cx)
                            .map(|abs_path| abs_path.to_path_buf()),
                        PathTrust::AbsPath(abs_path) => Some(abs_path),
                    } {
                        if !acc.is_empty() {
                            acc.push_str(TRUSTED_WORKSPACES_SEPARATOR);
                        }
                        acc.push_str(&abs_path.to_string_lossy())
                    }

                    acc
                });
        self.serialization_task = cx.background_spawn(async move {
            KEY_VALUE_STORE
                .write_kvp(TRUSTED_WORKSPACES_KEY.to_string(), new_worktree_roots)
                .await
                .log_err();
        });
        cx.emit(TrustedWorktreesEvent::Trusted(trusted_paths));
    }

    pub fn clear_trusted_paths(&mut self, cx: &App) -> Task<()> {
        self.trusted_paths.clear();
        let (tx, rx) = smol::channel::bounded(1);
        self.serialization_task = cx.background_spawn(async move {
            KEY_VALUE_STORE
                .delete_kvp(TRUSTED_WORKSPACES_KEY.to_string())
                .await
                .log_err();
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

        if let Some(worktree_path) = self.find_worktree_path(worktree, cx) {
            for trusted_path in &self.trusted_paths {
                let PathTrust::AbsPath(trusted_path) = trusted_path else {
                    continue;
                };
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

    pub fn restricted_worktree_abs_paths(
        &self,
        worktree_store: &WorktreeStore,
        cx: &App,
    ) -> HashMap<WorktreeId, Arc<Path>> {
        self.restricted
            .iter()
            .filter_map(|&restricted_worktree_id| {
                let worktree = worktree_store.worktree_for_id(restricted_worktree_id, cx)?;
                Some((restricted_worktree_id, worktree.read(cx).abs_path()))
            })
            .collect()
    }

    pub fn trust_all(&mut self, cx: &mut Context<Self>) {
        let restricted = std::mem::take(&mut self.restricted)
            .into_iter()
            .map(PathTrust::Worktree)
            .collect();
        self.trust(restricted, cx);
    }

    fn find_worktree_path(
        &mut self,
        worktree_id: WorktreeId,
        cx: &mut Context<Self>,
    ) -> Option<Arc<Path>> {
        let mut worktree_path = None;
        self.worktree_stores
            .retain(|worktree_store, _| match worktree_store.upgrade() {
                Some(worktree_store) => {
                    if worktree_path.is_none() {
                        if let Some(worktree) =
                            worktree_store.read(cx).worktree_for_id(worktree_id, cx)
                        {
                            worktree_path = Some(worktree.read(cx).abs_path());
                        }
                    }
                    true
                }
                None => false,
            });
        worktree_path
    }

    fn add_worktree_store(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        remote_host: Option<impl Into<RemoteHostData>>,
        cx: &mut Context<Self>,
    ) {
        self.worktree_stores.insert(
            worktree_store.downgrade(),
            remote_host.map(|host_data| host_data.into()),
        );
        self.trusted_paths = self
            .trusted_paths
            .drain()
            .map(|path_trust| match path_trust {
                worktree @ PathTrust::Worktree(_) => worktree,
                PathTrust::AbsPath(abs_path) => {
                    find_worktree_in_store(worktree_store.read(cx), &abs_path, cx)
                        .map(PathTrust::Worktree)
                        .unwrap_or_else(|| PathTrust::AbsPath(abs_path))
                }
            })
            .collect();
    }
}

fn find_worktree_in_store(
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
