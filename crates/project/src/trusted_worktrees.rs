//! TODO kb also block language servers and MCP servers from starting
use std::path::{Path, PathBuf};

use collections::HashSet;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, Subscription, Task, Window,
};
use util::ResultExt as _;

const TRUSTED_WORKSPACES_KEY: &str = "trusted_workspaces";
const TRUSTED_WORKSPACES_SEPARATOR: &str = "<|>";

pub fn init(cx: &mut App) {
    cx.spawn(async move |cx| {
        let trusted_worktrees = TrustedWorktrees::new().await;
        cx.update(|cx| {
            let trusted_worktees_storage = TrustedWorktreesStorage {
                trusted: cx.new(|_| trusted_worktrees),
                restricted: HashSet::default(),
            };
            cx.set_global(trusted_worktees_storage);
        })
        .log_err();
    })
    .detach();
}

/// A collection of worktree absolute paths that are considered trusted.
/// This can be used when checking for this criteria before enabling certain features.
#[derive(Clone)]
pub struct TrustedWorktreesStorage {
    trusted: Entity<TrustedWorktrees>,
    // TODO kb need to swtich over WorktreeId-based API from this level?
    // But worktrees' parent directories may be trusted and not being worktrees.
    restricted: HashSet<PathBuf>,
}

#[derive(Debug)]
pub enum TrustedWorktreesEvent {
    Trusted(HashSet<PathBuf>),
    Restricted(HashSet<PathBuf>),
}

/// A collection of absolute paths for trusted worktrees.
/// Such worktrees' local settings will be processed and applied.
///
/// Emits an event each time the worktree path checked and found not trusted,
/// or a certain worktree path had been trusted.
struct TrustedWorktrees {
    trusted_worktrees: HashSet<PathBuf>,
    serialization_task: Task<()>,
}

impl EventEmitter<TrustedWorktreesEvent> for TrustedWorktrees {}

impl TrustedWorktrees {
    async fn new() -> Self {
        Self {
            // TODO kb
            // * crate a new db table, FK onto remote_hosts DB table data
            // * store abs paths there still, but without odd separators
            trusted_worktrees: KEY_VALUE_STORE
                .read_kvp(TRUSTED_WORKSPACES_KEY)
                .ok()
                .flatten()
                .map(|workspaces| {
                    workspaces
                        .split(TRUSTED_WORKSPACES_SEPARATOR)
                        .map(|workspace_path| PathBuf::from(workspace_path))
                        .collect()
                })
                .unwrap_or_default(),
            serialization_task: Task::ready(()),
        }
    }

    fn trust(&mut self, paths: HashSet<PathBuf>, cx: &mut Context<'_, Self>) {
        let current_worktree_count = self.trusted_worktrees.len();
        self.trusted_worktrees
            .extend(paths.clone().into_iter().inspect(|path| {
                debug_assert!(
                    path.is_absolute(),
                    "Cannot trust non-absolute path {path:?}"
                )
            }));
        if current_worktree_count != self.trusted_worktrees.len() {
            let new_worktree_roots =
                self.trusted_worktrees
                    .iter()
                    .fold(String::new(), |mut acc, path| {
                        if !acc.is_empty() {
                            acc.push_str(TRUSTED_WORKSPACES_SEPARATOR);
                        }
                        acc.push_str(&path.to_string_lossy());
                        acc
                    });
            self.serialization_task = cx.background_spawn(async move {
                KEY_VALUE_STORE
                    .write_kvp(TRUSTED_WORKSPACES_KEY.to_string(), new_worktree_roots)
                    .await
                    .log_err();
            });
            cx.emit(TrustedWorktreesEvent::Trusted(paths));
        }
    }

    fn clear(&mut self, cx: &App) {
        self.trusted_worktrees.clear();
        self.serialization_task = cx.background_spawn(async move {
            KEY_VALUE_STORE
                .delete_kvp(TRUSTED_WORKSPACES_KEY.to_string())
                .await
                .log_err();
        });
    }
}

impl Global for TrustedWorktreesStorage {}

impl TrustedWorktreesStorage {
    pub fn subscribe<T: 'static>(
        &self,
        cx: &mut Context<T>,
        mut on_event: impl FnMut(&mut T, &TrustedWorktreesEvent, &mut Context<T>) + 'static,
    ) -> Subscription {
        cx.subscribe(&self.trusted, move |t, _, e, cx| on_event(t, e, cx))
    }

    pub fn subscribe_in<T: 'static>(
        &self,
        window: &mut Window,
        cx: &mut Context<T>,
        mut on_event: impl FnMut(&mut T, &TrustedWorktreesEvent, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        cx.subscribe_in(&self.trusted, window, move |t, _, e, window, cx| {
            on_event(t, e, window, cx)
        })
    }

    /// Adds worktree absolute paths to the trusted list.
    /// This will emit [`TrustedWorktreesEvent::Trusted`] event.
    pub fn trust(&mut self, trusted_paths: HashSet<PathBuf>, cx: &mut App) {
        // TODO kb unit test all this logic
        for trusted_path in &trusted_paths {
            self.restricted
                .retain(|restricted_path| !restricted_path.starts_with(trusted_path));
        }
        self.trusted.update(cx, |trusted_worktrees, cx| {
            trusted_worktrees.trust(trusted_paths, cx)
        });
    }

    /// Checks whether a certain worktree absolute path is trusted.
    /// If not, emits [`TrustedWorktreesEvent::Restricted`] event.
    pub fn can_trust_path(&mut self, abs_path: &Path, cx: &mut App) -> bool {
        debug_assert!(
            abs_path.is_absolute(),
            "Cannot check if trusting non-absolute path {abs_path:?}"
        );

        self.trusted.update(cx, |trusted_worktrees, cx| {
            let trusted_worktree_roots = &trusted_worktrees.trusted_worktrees;
            let mut can_trust = !self.restricted.contains(abs_path);
            if can_trust {
                can_trust = if trusted_worktree_roots.len() > 100 {
                    let mut path = Some(abs_path);
                    while let Some(path_to_check) = path {
                        if trusted_worktree_roots.contains(path_to_check) {
                            return true;
                        }
                        path = path_to_check.parent();
                    }
                    false
                } else {
                    trusted_worktree_roots
                        .iter()
                        .any(|trusted_root| abs_path.starts_with(&trusted_root))
                };
            }

            if !can_trust {
                if self.restricted.insert(abs_path.to_owned()) {
                    cx.emit(TrustedWorktreesEvent::Restricted(HashSet::from_iter([
                        abs_path.to_owned(),
                    ])));
                }
            }

            can_trust
        })
    }

    pub fn restricted_worktrees(&self) -> &HashSet<PathBuf> {
        &self.restricted
    }

    pub fn trust_all(&mut self, cx: &mut App) {
        let restricted = std::mem::take(&mut self.restricted);
        self.trust(restricted, cx);
    }

    pub fn clear_trusted_paths(&self, cx: &mut App) {
        self.trusted.update(cx, |trusted_worktrees, cx| {
            trusted_worktrees.clear(cx);
        });
    }

    pub fn take_task(&self, cx: &mut App) -> Task<()> {
        self.trusted.update(cx, |trusted_worktrees, _| {
            std::mem::replace(&mut trusted_worktrees.serialization_task, Task::ready(()))
        })
    }
}
