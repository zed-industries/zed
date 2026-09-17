//! User-global `AGENTS.md` support.
//!
//! Loads `~/.config/zed/AGENTS.md` (or the platform equivalent) into an
//! in-memory global, watches the file for changes, and surfaces read errors
//! through a caller-supplied notifier (so the host application can present
//! them with the same UI it uses for settings/keymap errors).
//!
//! Empty or whitespace-only files are treated as "no user `AGENTS.md`".
//! Read errors are also treated as "no user `AGENTS.md`" for the purpose of
//! the system prompt, but the error itself is exposed via
//! [`UserAgentsMdState::Error`] and forwarded to the notifier.
//!
//! The file is read in full, mirroring how project rules / repo `AGENTS.md`
//! files are loaded by the native agent today.

use std::sync::Arc;

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, BorrowAppContext, Global, SharedString, Task};
use settings::watch_config_file;

/// In-memory state of the user-global `AGENTS.md` file.
#[derive(Debug, Default, Clone)]
pub enum UserAgentsMdState {
    /// The file is missing, empty, or whitespace-only.
    #[default]
    Empty,
    /// The file was loaded successfully; carries its trimmed contents.
    Loaded(SharedString),
    /// The file exists but could not be read; carries the error message.
    Error(SharedString),
}

impl UserAgentsMdState {
    /// The trimmed `AGENTS.md` content, if the file was loaded successfully.
    pub fn content(&self) -> Option<&SharedString> {
        match self {
            Self::Loaded(content) => Some(content),
            Self::Empty | Self::Error(_) => None,
        }
    }

    /// The most recent read error, if the file exists but could not be read.
    pub fn error(&self) -> Option<&SharedString> {
        match self {
            Self::Error(message) => Some(message),
            Self::Empty | Self::Loaded(_) => None,
        }
    }
}

/// Global wrapper that owns the current [`UserAgentsMdState`] plus the watcher
/// task responsible for keeping it up to date.
///
/// Holding the [`Task`] in a `_watcher` field (matching the
/// `_settings_files_watcher` pattern in `SettingsStore`) ties the watcher's
/// lifetime to the data it produces: replacing or removing the global cancels
/// the watcher.
pub struct UserAgentsMd {
    state: UserAgentsMdState,
    _watcher: Task<()>,
}

impl Global for UserAgentsMd {}

impl UserAgentsMd {
    pub fn global(cx: &App) -> Option<&Self> {
        cx.try_global::<UserAgentsMd>()
    }

    pub fn state(&self) -> &UserAgentsMdState {
        &self.state
    }

    /// Convenience accessor for the trimmed `AGENTS.md` content.
    pub fn content(&self) -> Option<&SharedString> {
        self.state.content()
    }

    /// Convenience accessor for the most recent read error.
    pub fn error(&self) -> Option<&SharedString> {
        self.state.error()
    }
}

/// Initialize the user-global `AGENTS.md` watcher.
///
/// Starts a background task that watches [`paths::agents_file`] for changes
/// and updates the [`UserAgentsMd`] global accordingly. The `on_change`
/// callback is invoked on the foreground thread whenever a new read completes,
/// so callers can show or dismiss notifications matching the
/// settings/keymap-error UI.
///
/// Calling this more than once replaces the previous global, which drops the
/// previous watcher task and cancels it.
pub fn init(
    fs: Arc<dyn Fs>,
    cx: &mut App,
    on_change: impl Fn(&UserAgentsMdState, &mut App) + 'static,
) {
    let watcher = spawn_watcher(fs, cx, on_change);
    cx.set_global(UserAgentsMd {
        state: UserAgentsMdState::default(),
        _watcher: watcher,
    });
}

fn spawn_watcher(
    fs: Arc<dyn Fs>,
    cx: &mut App,
    on_change: impl Fn(&UserAgentsMdState, &mut App) + 'static,
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
            let new_state = if !trimmed.is_empty() {
                UserAgentsMdState::Loaded(SharedString::from(trimmed.to_string()))
            } else if let Some(error) = probe_read_error(fs.as_ref(), paths::agents_file()).await {
                UserAgentsMdState::Error(error)
            } else {
                UserAgentsMdState::Empty
            };

            cx.update(|cx| {
                cx.update_global::<UserAgentsMd, _>(|md, _| {
                    md.state = new_state.clone();
                });
                on_change(&new_state, cx);
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
    ) -> (Arc<FakeFs>, Rc<RefCell<Vec<UserAgentsMdState>>>) {
        cx.executor().allow_parking();
        let fs = FakeFs::new(cx.executor());
        // FakeFs requires the parent directory to exist before insert_file.
        let config_dir = paths::agents_file()
            .parent()
            .expect("AGENTS.md path should have a parent")
            .to_path_buf();
        fs.create_dir(&config_dir).await.unwrap();

        let history: Rc<RefCell<Vec<UserAgentsMdState>>> = Rc::new(RefCell::new(vec![]));
        let history_clone = history.clone();
        cx.update(|cx| {
            init(fs.clone(), cx, move |state, _cx| {
                history_clone.borrow_mut().push(state.clone());
            });
        });
        (fs, history)
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
                    .and_then(|md| md.content().cloned())
                    .as_deref(),
                Some("be concise"),
            );
            assert!(
                UserAgentsMd::global(cx)
                    .and_then(|md| md.error().cloned())
                    .is_none()
            );
        });
        assert!(matches!(
            history.borrow().last(),
            Some(UserAgentsMdState::Loaded(_))
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
                    .and_then(|md| md.content().cloned())
                    .is_none()
            );
        });
        assert!(matches!(
            history.borrow().last(),
            Some(UserAgentsMdState::Empty)
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
                    .and_then(|md| md.content().cloned())
                    .as_deref(),
                Some("first"),
            );
        });

        fs.insert_file(path, b"second".to_vec()).await;
        cx.run_until_parked();
        cx.update(|cx| {
            assert_eq!(
                UserAgentsMd::global(cx)
                    .and_then(|md| md.content().cloned())
                    .as_deref(),
                Some("second"),
            );
        });
    }
}
