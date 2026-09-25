use anyhow::{Context as _, Result};
use collections::{BTreeMap, BTreeSet, VecDeque};
use fs::{Fs, PathEvent, Watcher};
use futures::{Stream, StreamExt as _, channel::mpsc, future};
use std::{
    path::{Component, Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::Duration,
};

pub(super) struct EditorconfigWatcher {
    fs: Arc<dyn Fs>,
    path: PathBuf,
    dependencies: BTreeSet<PathBuf>,
    watches: BTreeMap<PathBuf, DirectoryWatch>,
}

struct DirectoryWatch {
    inode: u64,
    events: Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>,
    _watcher: Arc<dyn Watcher>,
}

impl EditorconfigWatcher {
    pub fn new(fs: Arc<dyn Fs>, path: PathBuf) -> Self {
        Self {
            fs,
            path,
            dependencies: BTreeSet::new(),
            watches: BTreeMap::new(),
        }
    }

    pub async fn load(&mut self) -> Result<Option<String>> {
        let mut dependencies = BTreeSet::new();
        let target = loop {
            dependencies.clear();

            let resolved = self.resolve(&mut dependencies).await;
            let logical_path = resolved
                .as_ref()
                .map(|(logical, _)| logical)
                .unwrap_or(&self.path)
                .clone();
            let changed = self.update_watches(&dependencies, &logical_path).await?;
            self.dependencies = dependencies.clone();
            // Recheck the chain after subscribing so a retarget during watch setup
            // cannot leave the loaded content paired with watches for the old target
            if !changed {
                break resolved?.1;
            }
        };

        match self.fs.load(&self.path).await {
            Ok(content) => Ok(Some(content).filter(|content| !content.is_empty())),
            Err(error) => {
                // RealFs metadata for a dangling link describes the link itself
                // rather than reporting absence. Check the resolved target instead
                match self.fs.metadata(&target).await {
                    Ok(None) => Ok(None),
                    _ => {
                        Err(error).with_context(|| format!("loading EditorConfig {:?}", self.path))
                    }
                }
            }
        }
    }

    async fn resolve(&self, dependencies: &mut BTreeSet<PathBuf>) -> Result<(PathBuf, PathBuf)> {
        let mut remaining = self
            .path
            .components()
            .map(|component| component.as_os_str().to_owned())
            .collect::<VecDeque<_>>();
        let mut resolved = PathBuf::new();
        let mut logical_path = None;
        let mut symlinks = 0;
        while let Some(component) = remaining.pop_front() {
            if Path::new(&component).components().next() == Some(Component::ParentDir) {
                resolved.pop();
                continue;
            }
            resolved.push(&component);
            if remaining.is_empty() && logical_path.is_none() {
                logical_path = Some(resolved.clone());
                dependencies.insert(resolved.clone());
            }
            if let Ok(target) = self.fs.read_link(&resolved).await {
                dependencies.insert(resolved.clone());
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
        dependencies.insert(resolved.clone());
        Ok((logical_path.context("empty EditorConfig path")?, resolved))
    }

    async fn update_watches(
        &mut self,
        dependencies: &BTreeSet<PathBuf>,
        logical_path: &Path,
    ) -> Result<bool> {
        let mut directories = BTreeMap::new();
        for path in dependencies {
            let mut found_directory = false;
            for directory in path.ancestors().skip(1) {
                // Ancestor-only removal events must reach us even when an entire
                // target tree disappears. Stop at the shared path instead of
                // subscribing to unrelated ancestors of the config
                if found_directory && logical_path.starts_with(directory) {
                    break;
                }
                if let Some(metadata) = self.fs.metadata(directory).await?
                    && metadata.is_dir
                {
                    let canonical = self.fs.canonicalize(directory).await?;
                    directories.insert(canonical, metadata.inode);
                    found_directory = true;
                    if logical_path.starts_with(directory) {
                        break;
                    }
                }
            }
        }

        let mut changed = false;
        self.watches.retain(|directory, watch| {
            let keep = directories.get(directory) == Some(&watch.inode);
            changed |= !keep;
            keep
        });
        for (directory, inode) in directories {
            if let std::collections::btree_map::Entry::Vacant(entry) =
                self.watches.entry(directory.clone())
            {
                let (events, watcher) = self.fs.watch(&directory, Duration::from_millis(100)).await;
                entry.insert(DirectoryWatch {
                    inode,
                    events,
                    _watcher: watcher,
                });
                changed = true;
            }
        }
        Ok(changed)
    }

    pub async fn changed(&mut self, reloads: &mut mpsc::UnboundedReceiver<()>) {
        loop {
            let events = self
                .watches
                .values_mut()
                .map(|watch| watch.events.next())
                .collect::<Vec<_>>();
            if events.is_empty() {
                reloads.next().await;
                return;
            }
            let events = match future::select(reloads.next(), future::select_all(events)).await {
                future::Either::Left(_) => return,
                future::Either::Right(((events, _, _), _)) => events,
            };
            let Some(events) = events else {
                return;
            };

            if events.iter().any(|event| {
                self.dependencies.iter().any(|dependency| {
                    if self.fs.is_path_case_sensitive(dependency) {
                        dependency.starts_with(&event.path)
                    } else {
                        Path::new(&dependency.to_string_lossy().to_lowercase())
                            .starts_with(event.path.to_string_lossy().to_lowercase())
                    }
                })
            }) {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::{FakeFs, RemoveOptions};
    use futures::FutureExt as _;
    use gpui::TestAppContext;
    use serde_json::json;
    use util::path;

    const INITIAL_CONTENT: &str = "[*]\nindent_size = 2\n";
    const UPDATED_CONTENT: &str = "[*]\nindent_size = 4\n";

    fn assert_watched_paths(fs: &FakeFs, expected: &[&str]) {
        let mut paths = fs.watched_paths();
        paths.sort();
        assert_eq!(
            paths,
            expected.iter().map(PathBuf::from).collect::<Vec<_>>()
        );
    }

    async fn assert_reload_and_settle(
        watcher: &mut EditorconfigWatcher,
        reloads: &mut mpsc::UnboundedReceiver<()>,
        expected: Option<&str>,
        cx: &mut TestAppContext,
    ) {
        watcher.changed(reloads).await;
        // Overlapping directory watches can queue the same filesystem change
        for _ in 0..=watcher.watches.len() {
            assert_eq!(
                watcher.load().await.expect("config reloads").as_deref(),
                expected,
            );
            cx.run_until_parked();
            if watcher.changed(reloads).now_or_never().is_none() {
                return;
            }
        }
        panic!("filesystem events must settle after bounded reloads");
    }

    #[gpui::test]
    async fn resolves_relative_symlink_chains_and_releases_watches_on_drop(
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "links": {},
                "store": {
                    "release": {},
                    "shared": { "editorconfig": INITIAL_CONTENT },
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
        let mut watcher =
            EditorconfigWatcher::new(fs.clone(), path!("/worktree/.editorconfig").into());
        let (_reload_sender, mut reloads) = mpsc::unbounded();

        assert_eq!(
            watcher.load().await.expect("config loads").as_deref(),
            Some(INITIAL_CONTENT),
        );
        assert_eq!(
            watcher.dependencies,
            BTreeSet::from_iter([
                path!("/links/package").into(),
                path!("/store/release/editorconfig").into(),
                path!("/store/shared/editorconfig").into(),
                path!("/worktree/.editorconfig").into(),
            ]),
        );
        assert_watched_paths(
            &fs,
            &[
                path!("/links"),
                path!("/store"),
                path!("/store/release"),
                path!("/store/shared"),
                path!("/worktree"),
            ],
        );

        fs.write(
            Path::new(path!("/store/shared/editorconfig")),
            UPDATED_CONTENT.as_bytes(),
        )
        .await
        .expect("target updates");
        watcher.changed(&mut reloads).await;
        assert_eq!(
            watcher.load().await.expect("config reloads").as_deref(),
            Some(UPDATED_CONTENT),
        );

        drop(watcher);
        cx.run_until_parked();
        assert!(fs.watched_paths().is_empty());
    }

    #[gpui::test]
    async fn reports_invalid_utf8_without_signaling_removal(cx: &mut TestAppContext) {
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
        let mut watcher =
            EditorconfigWatcher::new(fs.clone(), path!("/worktree/.editorconfig").into());
        let (_reload_sender, mut reloads) = mpsc::unbounded();
        let target = Path::new(path!("/target/editorconfig"));

        assert_eq!(
            watcher.load().await.expect("config loads").as_deref(),
            Some(INITIAL_CONTENT),
        );
        fs.write(target, &[0xff])
            .await
            .expect("invalid UTF-8 is written");
        watcher.changed(&mut reloads).await;
        let error = watcher
            .load()
            .await
            .expect_err("invalid UTF-8 must not clear accepted content with Ok(None)");
        assert!(error.downcast_ref::<std::string::FromUtf8Error>().is_some());
        assert_watched_paths(&fs, &[path!("/target"), path!("/worktree")]);

        fs.write(target, UPDATED_CONTENT.as_bytes())
            .await
            .expect("valid content is restored");
        watcher.changed(&mut reloads).await;
        assert_eq!(
            watcher.load().await.expect("config reloads").as_deref(),
            Some(UPDATED_CONTENT),
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
        let mut watcher = EditorconfigWatcher::new(fs.clone(), config.into());
        let (_reload_sender, mut reloads) = mpsc::unbounded();

        assert_eq!(
            watcher.load().await.expect("dangling target is absent"),
            None
        );
        assert_eq!(
            fs.read_link(config).await.expect("physical link remains"),
            PathBuf::from(path!("../target/editorconfig")),
        );
        assert_watched_paths(&fs, &[path!("/target"), path!("/worktree")]);

        fs.write(target, INITIAL_CONTENT.as_bytes())
            .await
            .expect("target is created");
        watcher.changed(&mut reloads).await;
        assert_eq!(
            watcher.load().await.expect("config loads").as_deref(),
            Some(INITIAL_CONTENT),
        );

        fs.remove_file(config, Default::default())
            .await
            .expect("physical link is deleted");
        watcher.changed(&mut reloads).await;
        assert_eq!(watcher.load().await.expect("deleted link is absent"), None);
        assert!(fs.read_link(config).await.is_err());
        assert_eq!(
            fs.load(target).await.expect("target remains"),
            INITIAL_CONTENT
        );
        assert_eq!(watcher.dependencies, BTreeSet::from_iter([config.into()]));
        assert_watched_paths(&fs, &[path!("/worktree")]);
    }

    #[gpui::test]
    async fn rearms_replaced_target_directories_and_recovers_missing_parents(
        cx: &mut TestAppContext,
    ) {
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
        let mut watcher =
            EditorconfigWatcher::new(fs.clone(), path!("/worktree/.editorconfig").into());
        let (_reload_sender, mut reloads) = mpsc::unbounded();
        let target_root = Path::new(path!("/target"));
        let package = Path::new(path!("/target/package"));
        let parent = Path::new(path!("/target/package/config"));
        let target = Path::new(path!("/target/package/config/editorconfig"));
        let remove_options = RemoveOptions {
            recursive: true,
            ..Default::default()
        };

        assert_eq!(
            watcher.load().await.expect("config loads").as_deref(),
            Some(INITIAL_CONTENT),
        );
        let original_inode = watcher
            .watches
            .get(parent)
            .expect("target is watched")
            .inode;
        fs.remove_dir(package, remove_options)
            .await
            .expect("target parents are removed");
        fs.write(target, UPDATED_CONTENT.as_bytes())
            .await
            .expect("target parents are replaced before reloading");
        assert_reload_and_settle(&mut watcher, &mut reloads, Some(UPDATED_CONTENT), cx).await;
        assert_ne!(
            watcher
                .watches
                .get(parent)
                .expect("target is rearmed")
                .inode,
            original_inode,
        );
        assert_watched_paths(
            &fs,
            &[
                path!("/target"),
                path!("/target/package"),
                path!("/target/package/config"),
                path!("/worktree"),
            ],
        );

        fs.remove_dir(target_root, remove_options)
            .await
            .expect("entire target subtree is removed");
        assert_reload_and_settle(&mut watcher, &mut reloads, None, cx).await;
        assert_watched_paths(&fs, &[path!("/"), path!("/worktree")]);

        let mut expected_watches = vec![path!("/worktree")];
        for directory in [target_root, package, parent] {
            fs.create_dir(directory).await.expect("parent is recreated");
            assert_reload_and_settle(&mut watcher, &mut reloads, None, cx).await;
            expected_watches.push(directory.to_str().expect("test path is UTF-8"));
            expected_watches.sort();
            assert_watched_paths(&fs, &expected_watches);
        }
        fs.write(target, INITIAL_CONTENT.as_bytes())
            .await
            .expect("target is restored");
        assert_reload_and_settle(&mut watcher, &mut reloads, Some(INITIAL_CONTENT), cx).await;
    }

    #[gpui::test]
    async fn ignores_old_target_events_after_retargeting(cx: &mut TestAppContext) {
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
        let mut watcher = EditorconfigWatcher::new(fs.clone(), config.into());
        let (_reload_sender, mut reloads) = mpsc::unbounded();
        assert_eq!(
            watcher.load().await.expect("first target loads").as_deref(),
            Some(INITIAL_CONTENT),
        );

        fs.insert_symlink(config, path!("../target/second").into())
            .await;
        watcher.changed(&mut reloads).await;
        assert_eq!(
            watcher
                .load()
                .await
                .expect("second target loads")
                .as_deref(),
            Some(UPDATED_CONTENT),
        );
        assert_eq!(
            watcher.dependencies,
            BTreeSet::from_iter([path!("/target/second").into(), config.into()]),
        );

        fs.write(Path::new(path!("/target/first")), b"obsolete")
            .await
            .expect("old target updates");
        cx.run_until_parked();
        assert!(watcher.changed(&mut reloads).now_or_never().is_none());

        fs.write(
            Path::new(path!("/target/second")),
            INITIAL_CONTENT.as_bytes(),
        )
        .await
        .expect("current target updates");
        watcher.changed(&mut reloads).await;
        assert_eq!(
            watcher
                .load()
                .await
                .expect("current target reloads")
                .as_deref(),
            Some(INITIAL_CONTENT),
        );
    }
}
