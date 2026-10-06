use anyhow::{Context as _, Result};
use collections::VecDeque;
use fs::{Fs, fs_watcher::poll_interval};
use futures::{StreamExt as _, channel::mpsc, future};
use gpui::BackgroundExecutor;
use std::{
    path::{Component, Path, PathBuf},
    sync::Arc,
};

/// Checks an EditorConfig path on the file watcher's polling interval rather
/// than subscribing to its directories, which can be as broad as a home
/// directory. Each check resolves the symlink chain again, so changes to any
/// link or to the target are found without watching them.
pub(super) struct EditorconfigWatcher {
    fs: Arc<dyn Fs>,
    executor: BackgroundExecutor,
    path: PathBuf,
    observed: Option<Observation>,
    delivered: Option<Option<String>>,
}

#[derive(Debug, PartialEq)]
pub(super) struct Update {
    /// Whether a file or link exists at the config path, which can warrant
    /// parent discovery even when it yields no settings
    pub present: bool,
    /// Content to apply, or `None` when the read failed or the content is
    /// unchanged and no reload was requested
    pub content: Option<Option<String>>,
}

#[derive(Clone, PartialEq)]
struct Observation {
    present: bool,
    // A failed read keeps only its message so that a persistent failure
    // compares equal from one check to the next
    content: Result<Option<String>, String>,
}

impl EditorconfigWatcher {
    pub fn new(fs: Arc<dyn Fs>, executor: BackgroundExecutor, path: PathBuf) -> Self {
        Self {
            fs,
            executor,
            path,
            observed: None,
            delivered: None,
        }
    }

    pub async fn load(&mut self) -> Update {
        let observation = self
            .executor
            .spawn(observe(self.fs.clone(), self.path.clone()))
            .await;
        self.record(observation, true)
    }

    /// Waits for a reload request or for a check that observes something
    /// different. Checks run in the background, so unchanged intervals never
    /// wake the caller.
    pub async fn changed(&mut self, reloads: &mut mpsc::UnboundedReceiver<()>) -> Update {
        let changed = self.executor.spawn({
            let fs = self.fs.clone();
            let path = self.path.clone();
            let executor = self.executor.clone();
            let observed = self.observed.clone();
            async move {
                loop {
                    executor.timer(poll_interval()).await;
                    let observation = observe(fs.clone(), path.clone()).await;
                    if observed.as_ref() != Some(&observation) {
                        break observation;
                    }
                }
            }
        });
        match future::select(reloads.next(), changed).await {
            future::Either::Left((_, superseded)) => {
                // Otherwise the pending check keeps running while the reload loads
                drop(superseded);
                self.load().await
            }
            future::Either::Right((observation, _)) => self.record(observation, false),
        }
    }

    fn record(&mut self, observation: Observation, requested: bool) -> Update {
        let content = match &observation.content {
            // Owners that joined since the last delivery rely on reloads to
            // receive content that has not changed
            Ok(content) => {
                (requested || self.delivered.as_ref() != Some(content)).then(|| content.clone())
            }
            Err(error) => {
                // Keep the accepted settings, reporting each failure once
                if self
                    .observed
                    .as_ref()
                    .is_none_or(|observed| observed.content != observation.content)
                {
                    log::error!("{error}");
                }
                // A requested load may serve an owner that joined since the
                // last delivery, so the next successful check must reach it
                if requested {
                    self.delivered = None;
                }
                None
            }
        };
        if let Some(content) = &content {
            self.delivered = Some(content.clone());
        }
        let update = Update {
            present: observation.present,
            content,
        };
        self.observed = Some(observation);
        update
    }
}

async fn observe(fs: Arc<dyn Fs>, path: PathBuf) -> Observation {
    let present = fs.read_link(&path).await.is_ok() || fs.is_file(&path).await;
    let content = load(fs.as_ref(), &path)
        .await
        .map_err(|error| format!("{error:#}"));
    Observation { present, content }
}

async fn load(fs: &dyn Fs, path: &Path) -> Result<Option<String>> {
    let target = resolve(fs, path).await?;
    match fs.load(path).await {
        Ok(content) => Ok(Some(content).filter(|content| !content.is_empty())),
        Err(error) => {
            // RealFs metadata for a dangling link describes the link itself
            // rather than reporting absence. Check the resolved target instead
            match fs.metadata(&target).await {
                Ok(None) => Ok(None),
                _ => Err(error).with_context(|| format!("loading EditorConfig {path:?}")),
            }
        }
    }
}

async fn resolve(fs: &dyn Fs, path: &Path) -> Result<PathBuf> {
    let mut remaining = path
        .components()
        .map(|component| component.as_os_str().to_owned())
        .collect::<VecDeque<_>>();
    let mut resolved = PathBuf::new();
    let mut symlinks = 0;
    while let Some(component) = remaining.pop_front() {
        if Path::new(&component).components().next() == Some(Component::ParentDir) {
            resolved.pop();
            continue;
        }
        resolved.push(&component);
        if let Ok(target) = fs.read_link(&resolved).await {
            symlinks += 1;
            anyhow::ensure!(symlinks <= 40, "too many symlinks in EditorConfig path");
            resolved.pop();
            if target.is_absolute() {
                resolved.clear();
            }
            for component in target.components().rev() {
                remaining.push_front(component.as_os_str().to_owned());
            }
        }
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::{FakeFs, RemoveOptions};
    use gpui::TestAppContext;
    use serde_json::json;
    use std::{pin::pin, task::Poll};
    use util::path;

    const INITIAL_CONTENT: &str = "[*]\nindent_size = 2\n";
    const UPDATED_CONTENT: &str = "[*]\nindent_size = 4\n";

    fn delivers(content: &str) -> Update {
        Update {
            present: true,
            content: Some(Some(content.to_owned())),
        }
    }

    /// Lets one polling interval elapse while waiting for a change, and
    /// returns what the watcher reported
    async fn poll_once(
        watcher: &mut EditorconfigWatcher,
        reloads: &mut mpsc::UnboundedReceiver<()>,
        cx: &mut TestAppContext,
    ) -> Option<Update> {
        let mut changed = pin!(watcher.changed(reloads));
        assert!(futures::poll!(changed.as_mut()).is_pending());
        cx.run_until_parked();
        assert!(
            futures::poll!(changed.as_mut()).is_pending(),
            "changes are only found by the next check",
        );
        cx.executor().advance_clock(poll_interval());
        cx.run_until_parked();
        match futures::poll!(changed.as_mut()) {
            Poll::Ready(update) => Some(update),
            Poll::Pending => None,
        }
    }

    #[gpui::test]
    async fn polls_link_chains_without_watching_or_listing_directories(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "links": {},
                "store": {
                    "release": {},
                    "shared": {
                        "editorconfig": INITIAL_CONTENT,
                        "unrelated": {},
                    },
                },
                "worktree": {},
            }),
        )
        .await;
        for (link, target) in [
            (path!("/links/package"), path!("../store/release")),
            (
                path!("/store/release/editorconfig"),
                path!("../shared/editorconfig"),
            ),
            (
                path!("/worktree/.editorconfig"),
                path!("../links/package/editorconfig"),
            ),
        ] {
            fs.insert_symlink(link, target.into()).await;
        }
        let mut watcher = EditorconfigWatcher::new(
            fs.clone(),
            cx.executor(),
            path!("/worktree/.editorconfig").into(),
        );
        let (_reload_sender, mut reloads) = mpsc::unbounded();
        let read_dir_calls = fs.read_dir_call_count();

        assert_eq!(watcher.load().await, delivers(INITIAL_CONTENT));
        assert_eq!(poll_once(&mut watcher, &mut reloads, cx).await, None);
        let unrelated = Path::new(path!("/store/shared/unrelated"));
        for index in 0..32 {
            fs.write(&unrelated.join(format!("{index}.txt")), b"unrelated")
                .await
                .expect("unrelated file is written");
        }
        assert_eq!(poll_once(&mut watcher, &mut reloads, cx).await, None);

        fs.write(
            Path::new(path!("/store/shared/editorconfig")),
            UPDATED_CONTENT.as_bytes(),
        )
        .await
        .expect("target updates");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(delivers(UPDATED_CONTENT)),
        );
        assert!(fs.watch_calls().is_empty());
        assert_eq!(fs.read_dir_call_count(), read_dir_calls);
    }

    #[gpui::test]
    async fn reload_requests_deliver_unchanged_content_immediately(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/worktree"),
            json!({ ".editorconfig": INITIAL_CONTENT }),
        )
        .await;
        let mut watcher = EditorconfigWatcher::new(
            fs.clone(),
            cx.executor(),
            path!("/worktree/.editorconfig").into(),
        );
        let (reload_sender, mut reloads) = mpsc::unbounded();

        assert_eq!(watcher.load().await, delivers(INITIAL_CONTENT));
        assert_eq!(poll_once(&mut watcher, &mut reloads, cx).await, None);

        reload_sender
            .unbounded_send(())
            .expect("reload is requested");
        let mut changed = pin!(watcher.changed(&mut reloads));
        assert!(futures::poll!(changed.as_mut()).is_pending());
        cx.run_until_parked();
        assert_eq!(
            futures::poll!(changed.as_mut()),
            Poll::Ready(delivers(INITIAL_CONTENT)),
        );
    }

    #[gpui::test]
    async fn keeps_accepted_content_through_persistent_read_failures(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "target": { "editorconfig": INITIAL_CONTENT },
                "worktree": {},
            }),
        )
        .await;
        fs.insert_symlink(
            path!("/worktree/.editorconfig"),
            path!("../target/editorconfig").into(),
        )
        .await;
        let mut watcher = EditorconfigWatcher::new(
            fs.clone(),
            cx.executor(),
            path!("/worktree/.editorconfig").into(),
        );
        let (_reload_sender, mut reloads) = mpsc::unbounded();
        let target = Path::new(path!("/target/editorconfig"));

        assert_eq!(watcher.load().await, delivers(INITIAL_CONTENT));
        fs.write(target, &[0xff])
            .await
            .expect("invalid UTF-8 is written");
        // A failed read never delivers `Some(None)`, which would clear the
        // accepted settings, and repeated failures are not reported again
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(Update {
                present: true,
                content: None,
            }),
        );
        assert_eq!(poll_once(&mut watcher, &mut reloads, cx).await, None);
        assert_eq!(poll_once(&mut watcher, &mut reloads, cx).await, None);

        fs.write(target, UPDATED_CONTENT.as_bytes())
            .await
            .expect("valid content is restored");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(delivers(UPDATED_CONTENT)),
        );
    }

    #[gpui::test]
    async fn distinguishes_dangling_targets_from_deleted_links(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/"), json!({ "target": {}, "worktree": {} }))
            .await;
        let config = Path::new(path!("/worktree/.editorconfig"));
        let target = Path::new(path!("/target/editorconfig"));
        fs.insert_symlink(config, path!("../target/editorconfig").into())
            .await;
        let mut watcher = EditorconfigWatcher::new(fs.clone(), cx.executor(), config.into());
        let (_reload_sender, mut reloads) = mpsc::unbounded();

        assert_eq!(
            watcher.load().await,
            Update {
                present: true,
                content: Some(None),
            },
        );
        fs.write(target, INITIAL_CONTENT.as_bytes())
            .await
            .expect("target is created");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(delivers(INITIAL_CONTENT)),
        );

        fs.remove_file(config, Default::default())
            .await
            .expect("physical link is deleted");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(Update {
                present: false,
                content: Some(None),
            }),
        );
        assert_eq!(
            fs.load(target).await.expect("target remains"),
            INITIAL_CONTENT
        );
    }

    #[gpui::test]
    async fn recovers_replaced_target_directories_and_missing_parents(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "target": { "package": { "config": { "editorconfig": INITIAL_CONTENT } } },
                "worktree": {},
            }),
        )
        .await;
        fs.insert_symlink(
            path!("/worktree/.editorconfig"),
            path!("../target/package/config/editorconfig").into(),
        )
        .await;
        let mut watcher = EditorconfigWatcher::new(
            fs.clone(),
            cx.executor(),
            path!("/worktree/.editorconfig").into(),
        );
        let (_reload_sender, mut reloads) = mpsc::unbounded();
        let target_root = Path::new(path!("/target"));
        let package = Path::new(path!("/target/package"));
        let parent = Path::new(path!("/target/package/config"));
        let target = Path::new(path!("/target/package/config/editorconfig"));
        let remove_options = RemoveOptions {
            recursive: true,
            ..Default::default()
        };

        assert_eq!(watcher.load().await, delivers(INITIAL_CONTENT));
        fs.remove_dir(package, remove_options)
            .await
            .expect("target parents are removed");
        fs.write(target, UPDATED_CONTENT.as_bytes())
            .await
            .expect("target parents are replaced before the next check");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(delivers(UPDATED_CONTENT)),
        );

        fs.remove_dir(target_root, remove_options)
            .await
            .expect("entire target subtree is removed");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(Update {
                present: true,
                content: Some(None),
            }),
        );
        for directory in [target_root, package, parent] {
            fs.create_dir(directory).await.expect("parent is recreated");
            assert_eq!(poll_once(&mut watcher, &mut reloads, cx).await, None);
        }
        fs.write(target, INITIAL_CONTENT.as_bytes())
            .await
            .expect("target is restored");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(delivers(INITIAL_CONTENT)),
        );
    }

    #[gpui::test]
    async fn follows_retargeted_links_and_ignores_old_targets(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "target": {
                    "first": INITIAL_CONTENT,
                    "second": UPDATED_CONTENT,
                },
                "worktree": {},
            }),
        )
        .await;
        let config = Path::new(path!("/worktree/.editorconfig"));
        fs.insert_symlink(config, path!("../target/first").into())
            .await;
        let mut watcher = EditorconfigWatcher::new(fs.clone(), cx.executor(), config.into());
        let (_reload_sender, mut reloads) = mpsc::unbounded();

        assert_eq!(watcher.load().await, delivers(INITIAL_CONTENT));
        fs.insert_symlink(config, path!("../target/second").into())
            .await;
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(delivers(UPDATED_CONTENT)),
        );

        fs.write(Path::new(path!("/target/first")), b"obsolete")
            .await
            .expect("old target updates");
        assert_eq!(poll_once(&mut watcher, &mut reloads, cx).await, None);

        fs.write(
            Path::new(path!("/target/second")),
            INITIAL_CONTENT.as_bytes(),
        )
        .await
        .expect("current target updates");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(delivers(INITIAL_CONTENT)),
        );
    }

    #[gpui::test]
    async fn reports_presence_changes_without_reapplying_absent_content(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/"), json!({ "shared": {}, "worktree": {} }))
            .await;
        let config = Path::new(path!("/worktree/.editorconfig"));
        let mut watcher = EditorconfigWatcher::new(fs.clone(), cx.executor(), config.into());
        let (_reload_sender, mut reloads) = mpsc::unbounded();
        let present_without_content = Some(Update {
            present: true,
            content: None,
        });

        assert_eq!(
            watcher.load().await,
            Update {
                present: false,
                content: Some(None),
            },
        );
        assert_eq!(poll_once(&mut watcher, &mut reloads, cx).await, None);

        fs.write(config, b"")
            .await
            .expect("empty config is created");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            present_without_content,
        );
        fs.remove_file(config, Default::default())
            .await
            .expect("empty config is deleted");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(Update {
                present: false,
                content: None,
            }),
        );

        fs.insert_symlink(config, path!("../shared/editorconfig").into())
            .await;
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            present_without_content,
        );
        fs.write(
            Path::new(path!("/shared/editorconfig")),
            INITIAL_CONTENT.as_bytes(),
        )
        .await
        .expect("link target is created");
        assert_eq!(
            poll_once(&mut watcher, &mut reloads, cx).await,
            Some(delivers(INITIAL_CONTENT)),
        );
    }
}
