use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use collections::HashSet;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, Subscription, Task, WindowId,
};
use util::ResultExt;

pub fn init(cx: &mut App) {
    cx.spawn(async move |cx| {
        let trusted_worktrees = TrustedWorktrees::new().await;
        cx.update(|cx| {
            let trusted_worktees_storage = TrustedWorktreesStorage(cx.new(|_| trusted_worktrees));
            cx.set_global(trusted_worktees_storage);
        })
        .log_err();
    })
    .detach();
}

pub struct Session {
    session_id: String,
    old_session_id: Option<String>,
    old_window_ids: Option<Vec<WindowId>>,
}

const SESSION_ID_KEY: &str = "session_id";
const SESSION_WINDOW_STACK_KEY: &str = "session_window_stack";
const TRUSTED_WORKSPACES_KEY: &str = "trusted_workspaces";
const TRUSTED_WORKSPACES_SEPARATOR: &str = "<|>";

impl Session {
    pub async fn new(session_id: String) -> Self {
        let old_session_id = KEY_VALUE_STORE.read_kvp(SESSION_ID_KEY).ok().flatten();

        KEY_VALUE_STORE
            .write_kvp(SESSION_ID_KEY.to_string(), session_id.clone())
            .await
            .log_err();

        let old_window_ids = KEY_VALUE_STORE
            .read_kvp(SESSION_WINDOW_STACK_KEY)
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<u64>>(&json).ok())
            .map(|vec| {
                vec.into_iter()
                    .map(WindowId::from)
                    .collect::<Vec<WindowId>>()
            });

        Self {
            session_id,
            old_session_id,
            old_window_ids,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: None,
            old_window_ids: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.session_id
    }
}

pub struct AppSession {
    session: Session,
    _serialization_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl AppSession {
    pub fn new(session: Session, cx: &Context<Self>) -> Self {
        let _subscriptions = vec![cx.on_app_quit(Self::app_will_quit)];

        let _serialization_task = cx.spawn(async move |_, cx| {
            let mut current_window_stack = Vec::new();
            loop {
                if let Some(windows) = cx.update(|cx| window_stack(cx)).ok().flatten()
                    && windows != current_window_stack
                {
                    store_window_stack(&windows).await;
                    current_window_stack = windows;
                }

                cx.background_executor()
                    .timer(Duration::from_millis(500))
                    .await;
            }
        });

        Self {
            session,
            _subscriptions,
            _serialization_task,
        }
    }

    fn app_will_quit(&mut self, cx: &mut Context<Self>) -> Task<()> {
        if let Some(window_stack) = window_stack(cx) {
            cx.background_spawn(async move { store_window_stack(&window_stack).await })
        } else {
            Task::ready(())
        }
    }

    pub fn id(&self) -> &str {
        self.session.id()
    }

    pub fn last_session_id(&self) -> Option<&str> {
        self.session.old_session_id.as_deref()
    }

    pub fn last_session_window_stack(&self) -> Option<Vec<WindowId>> {
        self.session.old_window_ids.clone()
    }
}

/// A collection of worktree absolute paths that are considered trusted.
/// This can be used when checking for this criteria before enabling certain features.
#[derive(Clone)]
pub struct TrustedWorktreesStorage(Entity<TrustedWorktrees>);

pub enum Event {
    TrustedWorktree(PathBuf),
    UntrustedWorktree(PathBuf),
}

/// A collection of absolute paths for trusted worktrees.
/// Such worktrees' local settings will be processed and applied.
///
/// Emits an event each time the worktree path checked and found not trusted,
/// or a certain worktree path had been trusted.
struct TrustedWorktrees {
    worktree_roots: HashSet<PathBuf>,
    serialization_task: Task<()>,
}

impl EventEmitter<Event> for TrustedWorktrees {}

impl TrustedWorktrees {
    async fn new() -> Self {
        Self {
            worktree_roots: KEY_VALUE_STORE
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

    fn trust_path(&mut self, abs_path: PathBuf, cx: &mut Context<'_, Self>) {
        debug_assert!(
            abs_path.is_absolute(),
            "Cannot trust non-absolute path {abs_path:?}"
        );
        let updated = self.worktree_roots.insert(abs_path.clone());
        if updated {
            let new_worktree_roots =
                self.worktree_roots
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
            cx.emit(Event::TrustedWorktree(abs_path));
        }
    }
}

impl Global for TrustedWorktreesStorage {}

impl TrustedWorktreesStorage {
    pub fn subscribe(
        &self,
        cx: &mut App,
        mut on_event: impl FnMut(&Event, &mut App) + 'static,
    ) -> Subscription {
        cx.subscribe(&self.0, move |_, e, cx| on_event(e, cx))
    }

    /// Adds a worktree absolute path to the trusted list.
    /// This will emit [`Event::TrustedWorktree`] event.
    pub fn trust_path(&self, abs_path: PathBuf, cx: &mut App) {
        self.0.update(cx, |trusted_worktrees, cx| {
            trusted_worktrees.trust_path(abs_path, cx)
        });
    }

    /// Checks whether a certain worktree absolute path is trusted.
    /// If not, emits [`Event::UntrustedWorktree`] event.
    pub fn can_trust_path(&self, abs_path: &Path, cx: &mut App) -> bool {
        debug_assert!(
            abs_path.is_absolute(),
            "Cannot check if trusting non-absolute path {abs_path:?}"
        );

        self.0.update(cx, |trusted_worktrees, cx| {
            let trusted_worktree_roots = &trusted_worktrees.worktree_roots;
            let can_trust = if trusted_worktree_roots.len() > 100 {
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
            if !can_trust {
                cx.emit(Event::UntrustedWorktree(abs_path.to_owned()));
            }

            can_trust
        })
    }
}

fn window_stack(cx: &App) -> Option<Vec<u64>> {
    Some(
        cx.window_stack()?
            .into_iter()
            .map(|window| window.window_id().as_u64())
            .collect(),
    )
}

async fn store_window_stack(windows: &[u64]) {
    if let Ok(window_ids_json) = serde_json::to_string(windows) {
        KEY_VALUE_STORE
            .write_kvp(SESSION_WINDOW_STACK_KEY.to_string(), window_ids_json)
            .await
            .log_err();
    }
}
