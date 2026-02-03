use anyhow::{Context as _, Result};
use collections::{BTreeMap, BTreeSet, HashSet};
use ec4rs::{ConfigParser, PropertiesSource, Section};
use fs::Fs;
use futures::StreamExt;
use gpui::{Context, EventEmitter, Task};
use paths::EDITORCONFIG_NAME;
use smallvec::SmallVec;
use std::{path::Path, str::FromStr, sync::Arc};
use util::{ResultExt as _, rel_path::RelPath};

use crate::{InvalidSettingsError, LocalSettingsPath, WorktreeId, watch_config_file};

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
    ExternalConfigChanged {
        path: LocalSettingsPath,
        content: Option<String>,
        affected_worktree_ids: Vec<WorktreeId>,
    },
}

impl EventEmitter<EditorconfigEvent> for EditorconfigStore {}

#[derive(Default)]
pub struct EditorconfigStore {
    external_configs: BTreeMap<Arc<Path>, (String, Option<Editorconfig>)>,
    worktree_state: BTreeMap<WorktreeId, EditorconfigWorktreeState>,
    local_external_config_watchers: BTreeMap<Arc<Path>, Task<()>>,
    local_external_config_discovery_tasks: BTreeMap<WorktreeId, Task<()>>,
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
                    self.local_external_config_watchers.remove(abs_path);
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
                                    rel_path.join(RelPath::unix(EDITORCONFIG_NAME).unwrap()),
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

    pub(crate) fn remove_for_worktree(&mut self, root_id: WorktreeId) {
        self.local_external_config_discovery_tasks.remove(&root_id);
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
                self.local_external_config_watchers.remove(path);
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
                        if fs.load(&path).await.is_ok() {
                            paths.push(dir_path);
                        }
                        current = dir.parent().map(|p| p.to_path_buf());
                    }
                    paths
                };

                this.update(cx, |this, cx| {
                    for dir_path in discovered_paths {
                        // We insert it here so that watchers can send events to appropriate worktrees.
                        // external_config_paths gets populated again in set_configs.
                        this.worktree_state
                            .entry(worktree_id)
                            .or_default()
                            .external_config_paths
                            .insert(dir_path.clone());
                        match this.local_external_config_watchers.entry(dir_path.clone()) {
                            std::collections::btree_map::Entry::Occupied(_) => {
                                if let Some(existing_config) = this.external_configs.get(&dir_path)
                                {
                                    cx.emit(EditorconfigEvent::ExternalConfigChanged {
                                        path: LocalSettingsPath::OutsideWorktree(dir_path),
                                        content: Some(existing_config.0.clone()),
                                        affected_worktree_ids: vec![worktree_id],
                                    });
                                } else {
                                    log::error!("Watcher exists for {dir_path:?} but no config found in external_configs");
                                }
                            }
                            std::collections::btree_map::Entry::Vacant(entry) => {
                                let watcher =
                                    Self::watch_local_external_config(fs.clone(), dir_path, cx);
                                entry.insert(watcher);
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

    fn watch_local_external_config(
        fs: Arc<dyn Fs>,
        dir_path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let config_path = dir_path.join(EDITORCONFIG_NAME);
        let (mut config_rx, watcher_task) =
            watch_config_file(cx.background_executor(), fs, config_path);

        cx.spawn(async move |this, cx| {
            let _watcher_task = watcher_task;
            while let Some(content) = config_rx.next().await {
                let content = Some(content).filter(|c| !c.is_empty());
                let dir_path = dir_path.clone();
                this.update(cx, |this, cx| {
                    let affected_worktree_ids: Vec<WorktreeId> = this
                        .worktree_state
                        .iter()
                        .filter_map(|(id, state)| {
                            state
                                .external_config_paths
                                .contains(&dir_path)
                                .then_some(*id)
                        })
                        .collect();

                    cx.emit(EditorconfigEvent::ExternalConfigChanged {
                        path: LocalSettingsPath::OutsideWorktree(dir_path),
                        content,
                        affected_worktree_ids,
                    });
                })
                .ok();
            }
        })
    }

    pub fn properties(
        &self,
        for_worktree: WorktreeId,
        for_path: &RelPath,
    ) -> Option<EditorconfigProperties> {
        let mut properties = EditorconfigProperties::new();
        let state = self.worktree_state.get(&for_worktree);
        let empty_path: Arc<RelPath> = RelPath::empty().into();
        let internal_root_config_is_root = state
            .and_then(|state| state.internal_configs.get(&empty_path))
            .and_then(|data| data.1.as_ref())
            .is_some_and(|ec| ec.is_root);

        if !internal_root_config_is_root {
            for (_, _, parsed_editorconfig) in self.external_configs(for_worktree) {
                if let Some(parsed_editorconfig) = parsed_editorconfig {
                    if parsed_editorconfig.is_root {
                        properties = EditorconfigProperties::new();
                    }
                    for section in &parsed_editorconfig.sections {
                        section
                            .apply_to(&mut properties, for_path.as_std_path())
                            .log_err()?;
                    }
                }
            }
        }

        for (directory_with_config, _, parsed_editorconfig) in self.internal_configs(for_worktree) {
            if directory_with_config > for_path {
                break;
            }
            if !for_path.starts_with(directory_with_config) {
                continue;
            }
            let parsed_editorconfig = parsed_editorconfig?;
            if parsed_editorconfig.is_root {
                properties = EditorconfigProperties::new();
            }
            for section in &parsed_editorconfig.sections {
                section
                    .apply_to(&mut properties, for_path.as_std_path())
                    .log_err()?;
            }
        }

        properties.use_fallbacks();
        Some(properties)
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
