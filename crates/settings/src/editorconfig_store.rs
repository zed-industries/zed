use anyhow::{Context as _, Result};
use collections::{BTreeMap, BTreeSet, HashMap, HashSet, btree_map};
use ec4rs::{ConfigParser, PropertiesSource, Section};
use fs::Fs;
use futures::StreamExt;
use gpui::{AsyncApp, Context, EventEmitter, Task, WeakEntity};
use paths::EDITORCONFIG_NAME;
use smallvec::SmallVec;
use std::{path::Path, str::FromStr, sync::Arc};
use util::{ResultExt as _, rel_path::RelPath};

use crate::{InvalidSettingsError, WorktreeId, watch_config_file};

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

#[derive(Default)]
pub struct EditorconfigStore {
    /// External editorconfig files shared across multiple worktrees
    external_configs: BTreeMap<Arc<Path>, (Option<Editorconfig>, Task<()>)>,
    worktree_editorconfig_state: BTreeMap<WorktreeId, WorktreeEditorconfigState>,
}

impl EventEmitter<()> for EditorconfigStore {}

#[derive(Default)]
pub struct WorktreeEditorconfigState {
    internal_configs: BTreeMap<Arc<RelPath>, (String, Option<Editorconfig>)>,
    external_config_paths: Option<BTreeSet<Arc<Path>>>,
    external_configs_loading_task: Option<Task<()>>,
}

impl EditorconfigStore {
    pub(crate) fn set_local_internal_configs(
        &mut self,
        root_id: WorktreeId,
        directory_path: Arc<RelPath>,
        editorconfig_contents: Option<&str>,
    ) -> std::result::Result<(), InvalidSettingsError> {
        match editorconfig_contents {
            None => {
                if let Some(state) = self.worktree_editorconfig_state.get_mut(&root_id) {
                    state.internal_configs.remove(&directory_path);
                }
            }
            Some(editorconfig_contents) => {
                let state = self.worktree_editorconfig_state.entry(root_id).or_default();
                match state.internal_configs.entry(directory_path.clone()) {
                    btree_map::Entry::Vacant(v) => match editorconfig_contents.parse() {
                        Ok(new_contents) => {
                            v.insert((editorconfig_contents.to_owned(), Some(new_contents)));
                        }
                        Err(e) => {
                            v.insert((editorconfig_contents.to_owned(), None));
                            return Err(InvalidSettingsError::Editorconfig {
                                message: e.to_string(),
                                path: directory_path
                                    .join(RelPath::unix(EDITORCONFIG_NAME).unwrap()),
                            });
                        }
                    },
                    btree_map::Entry::Occupied(mut o) => {
                        if o.get().0 != editorconfig_contents {
                            match editorconfig_contents.parse() {
                                Ok(new_contents) => {
                                    o.insert((
                                        editorconfig_contents.to_owned(),
                                        Some(new_contents),
                                    ));
                                }
                                Err(e) => {
                                    o.insert((editorconfig_contents.to_owned(), None));
                                    return Err(InvalidSettingsError::Editorconfig {
                                        message: e.to_string(),
                                        path: directory_path
                                            .join(RelPath::unix(EDITORCONFIG_NAME).unwrap()),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn remove_worktree(&mut self, root_id: WorktreeId) {
        if let Some(removed) = self.worktree_editorconfig_state.remove(&root_id) {
            let paths_in_use: HashSet<_> = self
                .worktree_editorconfig_state
                .values()
                .flat_map(|w| w.external_config_paths.iter().flatten())
                .collect();
            for path in removed.external_config_paths.iter().flatten() {
                if !paths_in_use.contains(path) {
                    self.external_configs.remove(path);
                }
            }
        }
    }

    pub fn local_internal_configs(
        &self,
        root_id: WorktreeId,
    ) -> impl '_ + Iterator<Item = (Arc<RelPath>, String, Option<Editorconfig>)> {
        self.worktree_editorconfig_state
            .get(&root_id)
            .into_iter()
            .flat_map(|state| {
                state
                    .internal_configs
                    .iter()
                    .map(|(path, (content, parsed_content))| {
                        (path.clone(), content.clone(), parsed_content.clone())
                    })
            })
    }

    pub fn load_external_configs(
        &mut self,
        worktree_id: WorktreeId,
        worktree_path: Arc<Path>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) {
        let Some(state) = self.worktree_editorconfig_state.get(&worktree_id) else {
            return;
        };
        if state.external_config_paths.is_some() {
            return;
        }

        // We don't always traverse up to look for external editor configs, only when there exists some internal config for the worktree
        // We can use a better heuristic here to figure out when to/not to traverse
        if state.internal_configs.is_empty() {
            return;
        }

        if let Some((_, Some(parsed))) = state.internal_configs.get(RelPath::empty()) {
            if parsed.is_root {
                return;
            }
        }

        let task = cx.spawn(async move |this, cx| {
            let external_configs =
                Self::reload_external_config_chain(&this, worktree_path, &fs, &cx).await;
            // todo(smit): need to think more on this case
            if external_configs.is_empty() {
                return;
            }
            this.update(cx, |this, cx| {
                let state = this
                    .worktree_editorconfig_state
                    .entry(worktree_id)
                    .or_default();
                state.external_config_paths = Some(
                    external_configs
                        .iter()
                        .map(|(path, _)| path.clone())
                        .collect(),
                );
                for (dir_path, config) in external_configs {
                    if this.external_configs.contains_key(&dir_path) {
                        continue;
                    }
                    let editorconfig_path = dir_path.join(EDITORCONFIG_NAME);
                    let watcher_task = Self::watch_external_config(
                        fs.clone(),
                        dir_path.clone(),
                        editorconfig_path,
                        cx,
                    );
                    this.external_configs
                        .insert(dir_path, (config, watcher_task));
                }
            })
            .ok();
        });

        self.worktree_editorconfig_state
            .entry(worktree_id)
            .or_default()
            .external_configs_loading_task = Some(task);
    }

    async fn reload_external_config_chain(
        this: &WeakEntity<Self>,
        worktree_path: Arc<Path>,
        fs: &Arc<dyn Fs>,
        cx: &AsyncApp,
    ) -> Vec<(Arc<Path>, Option<Editorconfig>)> {
        let cached_configs: HashMap<Arc<Path>, Option<Editorconfig>> = this
            .read_with(cx, |this, _| {
                this.external_configs
                    .iter()
                    .map(|(path, (config, _))| (path.clone(), config.clone()))
                    .collect()
            })
            .unwrap_or_default();

        let mut external_configs = Vec::new();
        let mut current = worktree_path.parent().map(|p| p.to_path_buf());

        while let Some(dir) = current {
            let dir_path: Arc<Path> = Arc::from(dir.as_path());
            if let Some(cached) = cached_configs.get(&dir_path) {
                let is_root = cached.as_ref().is_some_and(|c| c.is_root);
                external_configs.push((dir_path, None));
                if is_root {
                    break;
                }
            } else {
                let editorconfig_path = dir.join(EDITORCONFIG_NAME);
                if let Ok(content) = fs.load(&editorconfig_path).await {
                    match content.parse::<Editorconfig>() {
                        Ok(parsed) => {
                            let is_root = parsed.is_root;
                            external_configs.push((dir_path, Some(parsed)));
                            if is_root {
                                break;
                            }
                        }
                        Err(err) => {
                            log::warn!(
                                "Failed to parse external editorconfig at {:?}: {}",
                                editorconfig_path,
                                err
                            );
                        }
                    }
                }
            }
            current = dir.parent().map(|p| p.to_path_buf());
        }

        external_configs
    }

    fn watch_external_config(
        fs: Arc<dyn Fs>,
        dir_path: Arc<Path>,
        editorconfig_path: std::path::PathBuf,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let mut config_rx =
            watch_config_file(cx.background_executor(), fs, editorconfig_path.clone());

        cx.spawn(async move |this, cx| {
            while let Some(content) = config_rx.next().await {
                let parsed = if content.is_empty() {
                    None
                } else {
                    match content.parse::<Editorconfig>() {
                        Ok(parsed) => Some(parsed),
                        Err(err) => {
                            log::warn!(
                                "Failed to parse external editorconfig at {:?}: {}",
                                editorconfig_path,
                                err
                            );
                            None
                        }
                    }
                };

                let dir_path = dir_path.clone();
                this.update(cx, |this, cx| {
                    if let Some(entry) = this.external_configs.get_mut(&dir_path) {
                        entry.0 = parsed;
                    }
                    cx.emit(());
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
        let state = self.worktree_editorconfig_state.get(&for_worktree);
        let empty_path: Arc<RelPath> = RelPath::empty().into();
        let internal_root_config_is_root = state
            .and_then(|state| state.internal_configs.get(&empty_path))
            .and_then(|(_, parsed)| parsed.as_ref())
            .is_some_and(|ec| ec.is_root);

        if !internal_root_config_is_root {
            if let Some(state) = state {
                for path in state.external_config_paths.iter().flatten() {
                    if let Some((Some(parsed_editorconfig), _)) = self.external_configs.get(path) {
                        if parsed_editorconfig.is_root {
                            properties = EditorconfigProperties::new();
                        }
                        for section in parsed_editorconfig.sections.clone() {
                            section
                                .apply_to(&mut properties, for_path.as_std_path())
                                .log_err()?;
                        }
                    }
                }
            }
        }

        for (directory_with_config, _, parsed_editorconfig) in
            self.local_internal_configs(for_worktree)
        {
            if !for_path.starts_with(&directory_with_config) {
                properties.use_fallbacks();
                return Some(properties);
            }
            let parsed_editorconfig = parsed_editorconfig?;
            if parsed_editorconfig.is_root {
                properties = EditorconfigProperties::new();
            }
            for section in parsed_editorconfig.sections {
                section
                    .apply_to(&mut properties, for_path.as_std_path())
                    .log_err()?;
            }
        }

        properties.use_fallbacks();
        Some(properties)
    }
}
