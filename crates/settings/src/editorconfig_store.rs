use anyhow::{Context as _, Result};
use collections::{BTreeMap, BTreeSet, HashSet};
use ec4rs::{
    ConfigParser, PropertyKey as _, Section,
    property::{
        EndOfLine, FinalNewline, IndentSize, IndentStyle, MaxLineLen, TabWidth, TrimTrailingWs,
    },
};
use fs::Fs;
use futures::channel::mpsc;
use gpui::{Context, EventEmitter, Task};
use paths::EDITORCONFIG_NAME;
use smallvec::SmallVec;
use std::{path::Path, str::FromStr, sync::Arc};
use util::{ResultExt, rel_path::RelPath};

use crate::{
    InvalidSettingsError, LocalSettingsPath, WorktreeId, editorconfig_watcher::EditorconfigWatcher,
};

pub type EditorconfigProperties = ec4rs::Properties;

#[derive(Clone)]
pub struct Editorconfig {
    pub is_root: bool,
    pub sections: SmallVec<[Section; 5]>,
}

impl FromStr for Editorconfig {
    type Err = anyhow::Error;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let parser = ConfigParser::new_buffered(contents.as_bytes())
            .context("creating editorconfig parser")?;
        let is_root = parser.is_root;
        let sections = parser
            .collect::<Result<SmallVec<_>, _>>()
            .context("parsing editorconfig sections")?;
        Ok(Self { is_root, sections })
    }
}

#[derive(Clone, Debug)]
pub enum EditorconfigEvent {
    ConfigChanged {
        path: LocalSettingsPath,
        content: Option<String>,
        affected_worktree_ids: Vec<WorktreeId>,
    },
}

impl EventEmitter<EditorconfigEvent> for EditorconfigStore {}

#[derive(Default)]
pub struct EditorconfigStore {
    external_configs: BTreeMap<Arc<Path>, (String, Option<Editorconfig>)>,
    local_config_watchers: BTreeMap<(WorktreeId, Arc<RelPath>), (Arc<Path>, LocalConfigWatcher)>,
    worktree_state: BTreeMap<WorktreeId, EditorconfigWorktreeState>,
    local_external_config_watchers: BTreeMap<Arc<Path>, ExternalConfigWatcher>,
    local_external_config_discovery_tasks: BTreeMap<WorktreeId, Task<()>>,
}

struct LocalConfigWatcher {
    _task: Task<()>,
    reload: mpsc::UnboundedSender<()>,
}

struct ExternalConfigWatcher {
    watcher: LocalConfigWatcher,
    worktree_ids: BTreeSet<WorktreeId>,
}

#[derive(Default)]
struct EditorconfigWorktreeState {
    internal_configs: BTreeMap<Arc<RelPath>, (String, Option<Editorconfig>)>,
    external_config_paths: BTreeSet<Arc<Path>>,
}

impl EditorconfigStore {
    pub(crate) fn set_configs(
        &mut self,
        worktree_id: WorktreeId,
        path: LocalSettingsPath,
        content: Option<&str>,
    ) -> std::result::Result<(), InvalidSettingsError> {
        match (&path, content) {
            (LocalSettingsPath::InWorktree(rel_path), None) => {
                if let Some(state) = self.worktree_state.get_mut(&worktree_id) {
                    state.internal_configs.remove(rel_path);
                }
            }
            (LocalSettingsPath::OutsideWorktree(abs_path), None) => {
                if let Some(state) = self.worktree_state.get_mut(&worktree_id) {
                    state.external_config_paths.remove(abs_path);
                }
                let still_in_use = self
                    .worktree_state
                    .values()
                    .any(|state| state.external_config_paths.contains(abs_path));
                if !still_in_use {
                    self.external_configs.remove(abs_path);
                }
            }
            (LocalSettingsPath::InWorktree(rel_path), Some(content)) => {
                let state = self.worktree_state.entry(worktree_id).or_default();
                let should_update = state
                    .internal_configs
                    .get(rel_path)
                    .map_or(true, |entry| entry.0 != content);
                if should_update {
                    let parsed = match content.parse::<Editorconfig>() {
                        Ok(parsed) => Some(parsed),
                        Err(e) => {
                            state
                                .internal_configs
                                .insert(rel_path.clone(), (content.to_owned(), None));
                            return Err(InvalidSettingsError::Editorconfig {
                                message: e.to_string(),
                                path: LocalSettingsPath::InWorktree(
                                    rel_path
                                        .join(RelPath::from_unix_str(EDITORCONFIG_NAME).unwrap())
                                        .into(),
                                ),
                            });
                        }
                    };
                    state
                        .internal_configs
                        .insert(rel_path.clone(), (content.to_owned(), parsed));
                }
            }
            (LocalSettingsPath::OutsideWorktree(abs_path), Some(content)) => {
                let state = self.worktree_state.entry(worktree_id).or_default();
                state.external_config_paths.insert(abs_path.clone());
                let should_update = self
                    .external_configs
                    .get(abs_path)
                    .map_or(true, |entry| entry.0 != content);
                if should_update {
                    let parsed = match content.parse::<Editorconfig>() {
                        Ok(parsed) => Some(parsed),
                        Err(e) => {
                            self.external_configs
                                .insert(abs_path.clone(), (content.to_owned(), None));
                            return Err(InvalidSettingsError::Editorconfig {
                                message: e.to_string(),
                                path: LocalSettingsPath::OutsideWorktree(
                                    abs_path.join(EDITORCONFIG_NAME).into(),
                                ),
                            });
                        }
                    };
                    self.external_configs
                        .insert(abs_path.clone(), (content.to_owned(), parsed));
                }
            }
        }
        Ok(())
    }

    pub fn remove_for_worktree(&mut self, root_id: WorktreeId) {
        self.local_config_watchers
            .retain(|(worktree_id, _), _| *worktree_id != root_id);
        self.local_external_config_discovery_tasks.remove(&root_id);
        self.local_external_config_watchers.retain(|_, watcher| {
            watcher.worktree_ids.remove(&root_id);
            !watcher.worktree_ids.is_empty()
        });
        let Some(removed) = self.worktree_state.remove(&root_id) else {
            return;
        };
        let paths_in_use: HashSet<_> = self
            .worktree_state
            .values()
            .flat_map(|w| w.external_config_paths.iter())
            .collect();
        for path in removed.external_config_paths.iter() {
            if !paths_in_use.contains(path) {
                self.external_configs.remove(path);
            }
        }
    }

    fn internal_configs(
        &self,
        root_id: WorktreeId,
    ) -> impl '_ + Iterator<Item = (&RelPath, &str, Option<&Editorconfig>)> {
        self.worktree_state
            .get(&root_id)
            .into_iter()
            .flat_map(|state| {
                state
                    .internal_configs
                    .iter()
                    .map(|(path, data)| (path.as_ref(), data.0.as_str(), data.1.as_ref()))
            })
    }

    fn external_configs(
        &self,
        worktree_id: WorktreeId,
    ) -> impl '_ + Iterator<Item = (&Path, &str, Option<&Editorconfig>)> {
        self.worktree_state
            .get(&worktree_id)
            .into_iter()
            .flat_map(|state| {
                state.external_config_paths.iter().filter_map(|path| {
                    self.external_configs
                        .get(path)
                        .map(|entry| (path.as_ref(), entry.0.as_str(), entry.1.as_ref()))
                })
            })
    }

    pub fn local_editorconfig_settings(
        &self,
        worktree_id: WorktreeId,
    ) -> impl '_ + Iterator<Item = (LocalSettingsPath, &str, Option<&Editorconfig>)> {
        let external = self
            .external_configs(worktree_id)
            .map(|(path, content, parsed)| {
                (
                    LocalSettingsPath::OutsideWorktree(path.into()),
                    content,
                    parsed,
                )
            });
        let internal = self
            .internal_configs(worktree_id)
            .map(|(path, content, parsed)| {
                (LocalSettingsPath::InWorktree(path.into()), content, parsed)
            });
        external.chain(internal)
    }

    pub fn discover_local_external_configs_chain(
        &mut self,
        worktree_id: WorktreeId,
        worktree_path: Arc<Path>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) {
        // We should only have one discovery task per worktree.
        if self
            .local_external_config_discovery_tasks
            .contains_key(&worktree_id)
        {
            return;
        }

        let task = cx.spawn({
            let fs = fs.clone();
            async move |this, cx| {
                let discovered_paths = {
                    let mut paths = Vec::new();
                    let mut current = worktree_path.parent().map(|p| p.to_path_buf());
                    while let Some(dir) = current {
                        let dir_path: Arc<Path> = Arc::from(dir.as_path());
                        let path = dir.join(EDITORCONFIG_NAME);
                        if fs.load(&path).await.is_ok() || fs.read_link(&path).await.is_ok() {
                            paths.push(dir_path);
                        }
                        current = dir.parent().map(|p| p.to_path_buf());
                    }
                    paths
                };

                this.update(cx, |this, cx| {
                    for dir_path in discovered_paths {
                        match this.local_external_config_watchers.entry(dir_path.clone()) {
                            std::collections::btree_map::Entry::Occupied(mut entry) => {
                                entry.get_mut().worktree_ids.insert(worktree_id);
                                entry.get().watcher.reload.unbounded_send(()).log_err();
                            }
                            std::collections::btree_map::Entry::Vacant(entry) => {
                                let watcher =
                                    Self::watch_local_external_config(fs.clone(), dir_path, cx);
                                entry.insert(ExternalConfigWatcher {
                                    watcher,
                                    worktree_ids: BTreeSet::from_iter([worktree_id]),
                                });
                            }
                        }
                    }
                })
                .ok();
            }
        });

        self.local_external_config_discovery_tasks
            .insert(worktree_id, task);
    }

    pub fn watch_local_config(
        &mut self,
        worktree_id: WorktreeId,
        worktree_path: Arc<Path>,
        directory: Arc<RelPath>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) {
        let key = (worktree_id, directory.clone());
        if let Some((watched_worktree_path, watcher)) = self.local_config_watchers.get(&key)
            && *watched_worktree_path == worktree_path
        {
            watcher.reload.unbounded_send(()).log_err();
            return;
        }

        let (reload, mut reloads) = mpsc::unbounded();
        let config_path = worktree_path
            .join(directory.as_std_path())
            .join(EDITORCONFIG_NAME);
        let task = cx.spawn({
            let worktree_path = worktree_path.clone();
            async move |this, cx| {
                let mut watcher = EditorconfigWatcher::new(fs.clone(), config_path.clone());
                let mut discovered_parents = false;
                loop {
                    let content = watcher.load().await.log_err();
                    let discover_parents = !discovered_parents
                        && (content.as_ref().is_some_and(Option::is_some)
                            || fs.read_link(&config_path).await.is_ok()
                            || fs.is_file(&config_path).await);
                    if this
                        .update(cx, |this, cx| {
                            if discover_parents {
                                this.discover_local_external_configs_chain(
                                    worktree_id,
                                    worktree_path.clone(),
                                    fs.clone(),
                                    cx,
                                );
                                discovered_parents = true;
                            }
                            if let Some(content) = content {
                                cx.emit(EditorconfigEvent::ConfigChanged {
                                    path: LocalSettingsPath::InWorktree(directory.clone()),
                                    content,
                                    affected_worktree_ids: vec![worktree_id],
                                });
                            }
                        })
                        .is_err()
                    {
                        break;
                    }
                    watcher.changed(&mut reloads).await;
                }
            }
        });
        // Replacing the entry drops any watcher still bound to a former worktree location
        self.local_config_watchers.insert(
            key,
            (
                worktree_path,
                LocalConfigWatcher {
                    _task: task,
                    reload,
                },
            ),
        );
    }

    pub fn update_worktree_path(
        &mut self,
        worktree_id: WorktreeId,
        worktree_path: Arc<Path>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) {
        let moved_directories = self
            .local_config_watchers
            .iter()
            .filter(|((id, _), (watched_worktree_path, _))| {
                *id == worktree_id && *watched_worktree_path != worktree_path
            })
            .map(|((_, directory), _)| directory.clone())
            .collect::<Vec<_>>();
        for directory in moved_directories {
            self.watch_local_config(
                worktree_id,
                worktree_path.clone(),
                directory,
                fs.clone(),
                cx,
            );
        }
    }

    pub fn watches_local_config(&self, worktree_id: WorktreeId, directory: Arc<RelPath>) -> bool {
        self.local_config_watchers
            .contains_key(&(worktree_id, directory))
    }

    pub fn retain_local_config_watchers(
        &mut self,
        worktree_id: WorktreeId,
        mut retain: impl FnMut(&RelPath) -> bool,
        cx: &mut Context<Self>,
    ) {
        self.local_config_watchers.retain(|(id, directory), _| {
            if *id != worktree_id || retain(directory) {
                return true;
            }
            cx.emit(EditorconfigEvent::ConfigChanged {
                path: LocalSettingsPath::InWorktree(directory.clone()),
                content: None,
                affected_worktree_ids: vec![worktree_id],
            });
            false
        });
    }

    fn watch_local_external_config(
        fs: Arc<dyn Fs>,
        dir_path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> LocalConfigWatcher {
        let (reload, mut reloads) = mpsc::unbounded();
        let task = cx.spawn(async move |this, cx| {
            let mut watcher = EditorconfigWatcher::new(fs, dir_path.join(EDITORCONFIG_NAME));
            loop {
                if let Some(content) = watcher.load().await.log_err()
                    && this
                        .update(cx, |this, cx| {
                            let affected_worktree_ids = this
                                .local_external_config_watchers
                                .get(&dir_path)
                                .map(|watcher| watcher.worktree_ids.iter().copied().collect())
                                .unwrap_or_default();
                            cx.emit(EditorconfigEvent::ConfigChanged {
                                path: LocalSettingsPath::OutsideWorktree(dir_path.clone()),
                                content,
                                affected_worktree_ids,
                            });
                        })
                        .is_err()
                {
                    break;
                }
                watcher.changed(&mut reloads).await;
            }
        });
        LocalConfigWatcher {
            _task: task,
            reload,
        }
    }

    pub fn properties(
        &self,
        for_worktree: WorktreeId,
        for_path: &RelPath,
    ) -> Option<EditorconfigProperties> {
        let state = self.worktree_state.get(&for_worktree);
        let has_internal_configs = state.is_some_and(|state| !state.internal_configs.is_empty());
        let has_external_configs = self.external_configs(for_worktree).next().is_some();
        if !has_internal_configs && !has_external_configs {
            return None;
        }

        let mut properties = EditorconfigProperties::new();
        let internal_root_config_is_root = state
            .and_then(|state| state.internal_configs.get(RelPath::empty()))
            .and_then(|data| data.1.as_ref())
            .is_some_and(|ec| ec.is_root);

        let std_path = for_path.as_std_path();

        if !internal_root_config_is_root {
            for (_, _, parsed_editorconfig) in self.external_configs(for_worktree) {
                if let Some(parsed_editorconfig) = parsed_editorconfig {
                    if parsed_editorconfig.is_root {
                        properties = EditorconfigProperties::new();
                    }
                    for section in &parsed_editorconfig.sections {
                        apply_relevant_properties(section, std_path, &mut properties);
                    }
                }
            }
        }

        if let Some(state) = state {
            let mut internal_configs: SmallVec<[&Editorconfig; 8]> = SmallVec::new();

            for ancestor in for_path.ancestors() {
                if let Some((_, parsed)) = state.internal_configs.get(ancestor) {
                    let config = parsed.as_ref()?;
                    internal_configs.push(config);
                    if config.is_root {
                        break;
                    }
                }
            }

            for config in internal_configs.into_iter().rev() {
                if config.is_root {
                    properties = EditorconfigProperties::new();
                }
                for section in &config.sections {
                    apply_relevant_properties(section, std_path, &mut properties);
                }
            }
        }

        properties.use_fallbacks();
        Some(properties)
    }
}

fn apply_relevant_properties(
    section: &Section,
    std_path: &Path,
    properties: &mut EditorconfigProperties,
) {
    if !section.applies_to(std_path) {
        return;
    }
    let relevant_keys = [
        IndentStyle::key(),
        IndentSize::key(),
        TabWidth::key(),
        EndOfLine::key(),
        MaxLineLen::key(),
        FinalNewline::key(),
        TrimTrailingWs::key(),
    ];
    for (key, value) in section.props().iter() {
        if relevant_keys.contains(&key) {
            properties.insert_raw_for_key(key, value.clone());
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl EditorconfigStore {
    pub fn test_state(&self) -> (Vec<WorktreeId>, Vec<Arc<Path>>, Vec<Arc<Path>>) {
        let worktree_ids: Vec<_> = self.worktree_state.keys().copied().collect();
        let external_paths: Vec<_> = self.external_configs.keys().cloned().collect();
        let watcher_paths: Vec<_> = self
            .local_external_config_watchers
            .keys()
            .cloned()
            .collect();
        (worktree_ids, external_paths, watcher_paths)
    }

    pub fn external_config_paths_for_worktree(&self, worktree_id: WorktreeId) -> Vec<Arc<Path>> {
        self.worktree_state
            .get(&worktree_id)
            .map(|state| state.external_config_paths.iter().cloned().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{AppContext as _, TestAppContext};
    use serde_json::json;
    use util::path;

    #[gpui::test]
    async fn test_external_config_watcher_survives_clearing_and_removal(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/parent"),
            json!({
                ".editorconfig": "root = true\n[*]\nindent_size = 2\n",
                "worktree_a": {},
                "worktree_b": {},
            }),
        )
        .await;
        let store = cx.new(|_| EditorconfigStore::default());
        let _subscription = cx.update(|cx| {
            cx.subscribe(&store, |store, event: &EditorconfigEvent, cx| {
                let EditorconfigEvent::ConfigChanged {
                    path,
                    content,
                    affected_worktree_ids,
                } = event;
                store.update(cx, |store, _| {
                    for worktree_id in affected_worktree_ids {
                        store
                            .set_configs(*worktree_id, path.clone(), content.as_deref())
                            .expect("external config applies");
                    }
                });
            })
        });
        let first_worktree = WorktreeId::from_usize(1);
        let second_worktree = WorktreeId::from_usize(2);
        let parent: Arc<Path> = Path::new(path!("/parent")).into();
        let config_path = Path::new(path!("/parent/.editorconfig"));
        let file_path = RelPath::from_unix_str("file.txt").expect("valid relative path");
        let indent_size = |worktree_id, cx: &TestAppContext| {
            store.read_with(cx, |store, _| {
                store
                    .properties(worktree_id, file_path)
                    .and_then(|properties| properties.get::<IndentSize>().ok())
            })
        };
        store.update(cx, |store, cx| {
            store.discover_local_external_configs_chain(
                first_worktree,
                Path::new(path!("/parent/worktree_a")).into(),
                fs.clone(),
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(indent_size(first_worktree, cx), Some(IndentSize::Value(2)));

        fs.write(config_path, &[0xff])
            .await
            .expect("invalid UTF-8 is written");
        cx.run_until_parked();
        assert_eq!(indent_size(first_worktree, cx), Some(IndentSize::Value(2)));

        fs.write(config_path, b"")
            .await
            .expect("external config empties");
        cx.run_until_parked();
        assert_eq!(indent_size(first_worktree, cx), None);
        store.read_with(cx, |store, _| {
            assert!(store.external_configs.is_empty());
            assert_eq!(store.local_external_config_watchers.len(), 1);
        });

        store.update(cx, |store, cx| {
            store.discover_local_external_configs_chain(
                second_worktree,
                Path::new(path!("/parent/worktree_b")).into(),
                fs.clone(),
                cx,
            );
        });
        cx.run_until_parked();
        store.read_with(cx, |store, _| {
            assert!(store.external_configs.is_empty());
            assert_eq!(store.local_external_config_watchers.len(), 1);
            assert_eq!(
                store
                    .local_external_config_watchers
                    .get(&parent)
                    .expect("shared watcher exists")
                    .worktree_ids,
                BTreeSet::from_iter([first_worktree, second_worktree]),
            );
        });

        fs.write(config_path, b"root = true\n[*]\nindent_size = 6\n")
            .await
            .expect("external config refills");
        cx.run_until_parked();
        assert_eq!(indent_size(first_worktree, cx), Some(IndentSize::Value(6)));
        assert_eq!(indent_size(second_worktree, cx), Some(IndentSize::Value(6)));

        fs.remove_file(config_path, Default::default())
            .await
            .expect("external config is deleted");
        cx.run_until_parked();
        assert_eq!(indent_size(first_worktree, cx), None);
        assert_eq!(indent_size(second_worktree, cx), None);
        store.update(cx, |store, _| store.remove_for_worktree(first_worktree));
        cx.run_until_parked();
        store.read_with(cx, |store, _| {
            assert!(store.external_configs.is_empty());
            assert_eq!(store.local_external_config_watchers.len(), 1);
            assert_eq!(
                store
                    .local_external_config_watchers
                    .get(&parent)
                    .expect("surviving watcher exists")
                    .worktree_ids,
                BTreeSet::from_iter([second_worktree]),
            );
        });

        fs.atomic_write(
            config_path.to_path_buf(),
            "root = true\n[*]\nindent_size = 8\n".to_owned(),
        )
        .await
        .expect("external config is recreated");
        cx.run_until_parked();
        assert_eq!(indent_size(first_worktree, cx), None);
        assert_eq!(indent_size(second_worktree, cx), Some(IndentSize::Value(8)));
        store.update(cx, |store, _| store.remove_for_worktree(second_worktree));
        cx.run_until_parked();
        store.read_with(cx, |store, _| {
            assert!(store.external_configs.is_empty());
            assert!(store.local_external_config_discovery_tasks.is_empty());
            assert!(store.local_external_config_watchers.is_empty());
            assert!(store.worktree_state.is_empty());
        });
        assert!(fs.watched_paths().is_empty());
    }

    #[test]
    fn test_properties_resolution_and_updates() {
        let mut store = EditorconfigStore::default();
        let worktree_id = WorktreeId::from_usize(1);
        let root = LocalSettingsPath::InWorktree(Arc::from(RelPath::empty()));
        let file_path = RelPath::from_unix_str("src/main.rs").unwrap();

        assert_eq!(store.properties(worktree_id, file_path), None);

        store
            .set_configs(
                worktree_id,
                root.clone(),
                Some("root = true\n\n[*]\nindent_size = 4\n"),
            )
            .unwrap();
        let properties = store.properties(worktree_id, file_path).unwrap();
        assert_eq!(
            properties.get::<IndentSize>().ok(),
            Some(IndentSize::Value(4))
        );

        store
            .set_configs(
                worktree_id,
                root.clone(),
                Some("root = true\n\n[*]\nindent_size = 8\n"),
            )
            .unwrap();
        let properties = store.properties(worktree_id, file_path).unwrap();
        assert_eq!(
            properties.get::<IndentSize>().ok(),
            Some(IndentSize::Value(8))
        );

        store.set_configs(worktree_id, root, None).unwrap();
        assert_eq!(store.properties(worktree_id, file_path), None);
    }

    #[test]
    fn test_nested_config_overrides_and_irrelevant_keys() {
        let mut store = EditorconfigStore::default();
        let worktree_id = WorktreeId::from_usize(1);
        let file_path = RelPath::from_unix_str("src/main.rs").unwrap();

        store
            .set_configs(
                worktree_id,
                LocalSettingsPath::InWorktree(Arc::from(RelPath::empty())),
                Some(
                    "root = true\n\n[*]\nindent_size = 4\nmax_line_length = 100\ncurly_bracket_next_line = true\n",
                ),
            )
            .unwrap();
        store
            .set_configs(
                worktree_id,
                LocalSettingsPath::InWorktree(Arc::from(RelPath::from_unix_str("src").unwrap())),
                Some("[*.rs]\nindent_size = 2\n"),
            )
            .unwrap();

        let properties = store.properties(worktree_id, file_path).unwrap();
        assert_eq!(
            properties.get::<IndentSize>().ok(),
            Some(IndentSize::Value(2))
        );
        assert_eq!(
            properties.get::<MaxLineLen>().ok(),
            Some(MaxLineLen::Value(100))
        );
        assert_eq!(
            properties
                .get_raw_for_key("curly_bracket_next_line")
                .into_option(),
            None
        );

        let other_file = RelPath::from_unix_str("src/main.js").unwrap();
        let properties = store.properties(worktree_id, other_file).unwrap();
        assert_eq!(
            properties.get::<IndentSize>().ok(),
            Some(IndentSize::Value(4))
        );
    }

    #[test]
    fn test_no_properties_after_worktree_removal() {
        let mut store = EditorconfigStore::default();
        let worktree_id = WorktreeId::from_usize(1);
        let file_path = RelPath::from_unix_str("src/main.rs").unwrap();

        store
            .set_configs(
                worktree_id,
                LocalSettingsPath::InWorktree(Arc::from(RelPath::empty())),
                Some("root = true\n\n[*]\nindent_size = 4\n"),
            )
            .unwrap();
        assert!(store.properties(worktree_id, file_path).is_some());

        store.remove_for_worktree(worktree_id);
        assert_eq!(store.properties(worktree_id, file_path), None);
    }
}
