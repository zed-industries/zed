//! User-global `AGENTS.md` support.
//!
//! Loads `~/.config/zed/AGENTS.md` (or the platform equivalent) into an
//! in-memory global, watches the file for changes, and surfaces read errors
//! through a caller-supplied notifier (so the host application can present
//! them with the same UI it uses for settings/keymap errors).
//!
//! Empty or whitespace-only files are treated as "no user `AGENTS.md`".
//! Read errors are also treated as "no user `AGENTS.md`" for the purpose of
//! the system prompt, but the error itself is exposed via [`UserAgentsMd::error`]
//! and forwarded to the notifier.
//!
//! The file is read in full, mirroring how project rules / repo `AGENTS.md`
//! files are loaded by the native agent today.

use std::sync::Arc;

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, BorrowAppContext, Global, SharedString, Task};
use settings::watch_config_file;

/// In-memory snapshot of the user-global `AGENTS.md` file.
///
/// Stored as a `Global` so that the system prompt builder can read it
/// synchronously without having to plumb extra state through every callsite.
#[derive(Debug, Default, Clone)]
pub struct UserAgentsMd {
    /// The trimmed contents of the file, or `None` if the file is missing,
    /// empty, whitespace-only, or could not be read.
    content: Option<SharedString>,
    /// The most recent read error, if any. Cleared when the file is read
    /// successfully (including reads that return missing/empty content).
    error: Option<SharedString>,
}

impl Global for UserAgentsMd {}

impl UserAgentsMd {
    pub fn global(cx: &App) -> Option<&Self> {
        cx.try_global::<UserAgentsMd>()
    }

    /// The trimmed `AGENTS.md` content, if any.
    pub fn content(&self) -> Option<&SharedString> {
        self.content.as_ref()
    }

    /// The most recent read error, if the file could not be read.
    pub fn error(&self) -> Option<&SharedString> {
        self.error.as_ref()
    }
}

/// Outcome of a single read of the user-global `AGENTS.md` file. Used by the
/// notifier callback in [`init`] to decide whether to show or dismiss UI.
#[derive(Debug, Clone)]
pub enum UserAgentsMdStatus {
    /// The file was read and is non-empty (after trimming).
    Loaded,
    /// The file is missing, empty, or whitespace-only.
    Empty,
    /// The file could not be read.
    Error(SharedString),
}

/// Initialize the user-global `AGENTS.md` watcher.
///
/// Starts a background task that watches [`paths::agents_file`] for changes
/// and updates the [`UserAgentsMd`] global accordingly. The `on_status_change`
/// callback is invoked on the foreground thread whenever a new read completes,
/// so callers can show or dismiss notifications matching the
/// settings/keymap-error UI.
///
/// Calling this more than once replaces the previous watcher. The watcher
/// task is stored in the global so it lives for the lifetime of the app.
pub fn init(
    fs: Arc<dyn Fs>,
    cx: &mut App,
    on_status_change: impl Fn(UserAgentsMdStatus, &mut App) + 'static,
) {
    cx.set_global(UserAgentsMd::default());
    let task = spawn_watcher(fs, cx, on_status_change);
    cx.set_global(UserAgentsMdWatcher(task));
}

struct UserAgentsMdWatcher(#[allow(dead_code)] Task<()>);

impl Global for UserAgentsMdWatcher {}

fn spawn_watcher(
    fs: Arc<dyn Fs>,
    cx: &mut App,
    on_status_change: impl Fn(UserAgentsMdStatus, &mut App) + 'static,
) -> Task<()> {
    let path = paths::agents_file().clone();
    let (mut rx, watcher_task) = watch_config_file(cx.background_executor(), fs.clone(), path);

    cx.spawn(async move |cx| {
        // Keep the file watcher task alive for as long as this task runs.
        let _watcher_task = watcher_task;

        // `watch_config_file` swallows file-open errors (it emits an empty
        // string when the file is missing or unreadable), so we probe the
        // path on each event to tell "missing / empty" apart from "exists but
        // failed to read". This mirrors how `settings.json` is watched, with
        // the extra probe being the only addition: settings.json doesn't need
        // to surface read errors because invalid JSON is reported separately,
        // but for AGENTS.md a raw read error is the only signal we get.
        while let Some(raw) = rx.next().await {
            let trimmed = raw.trim();
            let status = if !trimmed.is_empty() {
                UserAgentsMdStatus::Loaded
            } else if let Some(error) = probe_read_error(fs.as_ref(), paths::agents_file()).await {
                UserAgentsMdStatus::Error(error)
            } else {
                UserAgentsMdStatus::Empty
            };

            let new_content = if matches!(status, UserAgentsMdStatus::Loaded) {
                Some(SharedString::from(trimmed.to_string()))
            } else {
                None
            };
            let new_error = if let UserAgentsMdStatus::Error(message) = &status {
                Some(message.clone())
            } else {
                None
            };

            cx.update(|cx| {
                cx.update_global::<UserAgentsMd, _>(|state, _| {
                    state.content = new_content;
                    state.error = new_error;
                });
                on_status_change(status, cx);
            });
        }
    })
}

async fn probe_read_error(fs: &dyn Fs, path: &std::path::Path) -> Option<SharedString> {
    match fs.load(path).await {
        Ok(_) => None,
        Err(err) => {
            if let Some(io_err) = err.downcast_ref::<std::io::Error>()
                && io_err.kind() == std::io::ErrorKind::NotFound
            {
                return None;
            }
            Some(SharedString::from(format!("{err:#}")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use std::cell::RefCell;
    use std::rc::Rc;

    async fn init_test(
        cx: &mut TestAppContext,
    ) -> (Arc<FakeFs>, Rc<RefCell<Vec<UserAgentsMdStatus>>>) {
        cx.executor().allow_parking();
        let fs = FakeFs::new(cx.executor());
        // FakeFs requires the parent directory to exist before insert_file.
        let config_dir = paths::agents_file()
            .parent()
            .expect("AGENTS.md path should have a parent")
            .to_path_buf();
        fs.create_dir(&config_dir).await.unwrap();

        let status_history: Rc<RefCell<Vec<UserAgentsMdStatus>>> = Rc::new(RefCell::new(vec![]));
        let history_clone = status_history.clone();
        cx.update(|cx| {
            init(fs.clone(), cx, move |status, _cx| {
                history_clone.borrow_mut().push(status);
            });
        });
        (fs, status_history)
    }

    #[gpui::test]
    async fn loads_initial_content(cx: &mut TestAppContext) {
        let path = paths::agents_file();
        let (fs, history) = init_test(cx).await;
        fs.insert_file(path, b"be concise".to_vec()).await;

        cx.run_until_parked();
        cx.update(|cx| {
            assert_eq!(
                UserAgentsMd::global(cx)
                    .and_then(|s| s.content().cloned())
                    .as_deref(),
                Some("be concise"),
            );
            assert!(
                UserAgentsMd::global(cx)
                    .and_then(|s| s.error().cloned())
                    .is_none()
            );
        });
        assert!(matches!(
            history.borrow().last(),
            Some(UserAgentsMdStatus::Loaded)
        ));
    }

    #[gpui::test]
    async fn empty_file_is_ignored(cx: &mut TestAppContext) {
        let path = paths::agents_file();
        let (fs, history) = init_test(cx).await;
        fs.insert_file(path, b"   \n  \t".to_vec()).await;

        cx.run_until_parked();
        cx.update(|cx| {
            assert!(
                UserAgentsMd::global(cx)
                    .and_then(|s| s.content().cloned())
                    .is_none()
            );
        });
        assert!(matches!(
            history.borrow().last(),
            Some(UserAgentsMdStatus::Empty)
        ));
    }

    #[gpui::test]
    async fn reacts_to_file_changes(cx: &mut TestAppContext) {
        let path = paths::agents_file();
        let (fs, _history) = init_test(cx).await;
        fs.insert_file(path, b"first".to_vec()).await;
        cx.run_until_parked();
        cx.update(|cx| {
            assert_eq!(
                UserAgentsMd::global(cx)
                    .and_then(|s| s.content().cloned())
                    .as_deref(),
                Some("first"),
            );
        });

        fs.insert_file(path, b"second".to_vec()).await;
        cx.run_until_parked();
        cx.update(|cx| {
            assert_eq!(
                UserAgentsMd::global(cx)
                    .and_then(|s| s.content().cloned())
                    .as_deref(),
                Some("second"),
            );
        });
    }
}
