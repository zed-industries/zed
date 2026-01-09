use anyhow::{Context as _, Result};
use collections::{BTreeMap, HashMap, HashSet, btree_map};
use ec4rs::{ConfigParser, PropertiesSource, Section};
use fs::Fs;
use futures::StreamExt;
use gpui::{Context, EventEmitter, Task};
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
    worktree_configs: BTreeMap<WorktreeId, WorktreeEditorconfigs>,
}

impl EventEmitter<()> for EditorconfigStore {}

#[derive(Default)]
pub struct WorktreeEditorconfigs {
    /// Ordered from closest to filesystem root to closest to worktree root.
    /// None means external configs haven't been loaded yet for worktree.
    external_config_paths: Option<Vec<Arc<Path>>>,
    internal_configs: BTreeMap<Arc<RelPath>, (String, Option<Editorconfig>)>,
    loading_task: Option<Task<()>>,
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
                if let Some(worktree_configs) = self.worktree_configs.get_mut(&root_id) {
                    worktree_configs.internal_configs.remove(&directory_path);
                }
            }
            Some(editorconfig_contents) => {
                let worktree_configs = self.worktree_configs.entry(root_id).or_default();
                match worktree_configs
                    .internal_configs
                    .entry(directory_path.clone())
                {
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
        if let Some(removed) = self.worktree_configs.remove(&root_id) {
            let paths_in_use: HashSet<_> = self
                .worktree_configs
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
        self.worktree_configs
            .get(&root_id)
            .into_iter()
            .flat_map(|worktree_configs| {
                worktree_configs
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
        worktree_abs_path: Arc<Path>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) {
        let Some(worktree_configs) = self.worktree_configs.get(&worktree_id) else {
            return;
        };
        if worktree_configs.external_config_paths.is_some() {
            return;
        }
        if worktree_configs.internal_configs.is_empty() {
            return;
        }
        if let Some((_, Some(parsed))) = worktree_configs
            .internal_configs
            .get(RelPath::empty())
        {
            if parsed.is_root {
                return;
            }
        }

        let cached_configs: HashMap<Arc<Path>, Option<Editorconfig>> = self
            .external_configs
            .iter()
            .map(|(path, (config, _))| (path.clone(), config.clone()))
            .collect();

        let task = cx.spawn(async move |this, cx| {
            let mut external_configs_to_load: Vec<(Arc<Path>, Option<Editorconfig>)> = Vec::new();

            let mut current = worktree_abs_path.parent().map(|p| p.to_path_buf());

            while let Some(dir) = current {
                let dir_path: Arc<Path> = Arc::from(dir.as_path());
                if let Some(cached) = cached_configs.get(&dir_path) {
                    let is_root = cached.as_ref().is_some_and(|c| c.is_root);
                    external_configs_to_load.push((dir_path, None));
                    if is_root {
                        break;
                    }
                } else {
                    let editorconfig_path = dir.join(EDITORCONFIG_NAME);
                    if let Ok(content) = fs.load(&editorconfig_path).await {
                        match content.parse::<Editorconfig>() {
                            Ok(parsed) => {
                                let is_root = parsed.is_root;
                                external_configs_to_load.push((dir_path, Some(parsed)));
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

            external_configs_to_load.reverse();

            if external_configs_to_load.is_empty() {
                return;
            }

            this.update(cx, |this, cx| {
                let worktree_config = this.worktree_configs.entry(worktree_id).or_default();
                worktree_config.external_config_paths = Some(
                    external_configs_to_load
                        .iter()
                        .map(|(path, _)| path.clone())
                        .collect(),
                );

                for (dir_path, config) in external_configs_to_load {
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

        self.worktree_configs
            .entry(worktree_id)
            .or_default()
            .loading_task = Some(task);
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
        let worktree_configs = self.worktree_configs.get(&for_worktree);
        let empty_path: Arc<RelPath> = RelPath::empty().into();
        let root_has_root_true = worktree_configs
            .and_then(|configs| configs.internal_configs.get(&empty_path))
            .and_then(|(_, parsed)| parsed.as_ref())
            .is_some_and(|ec| ec.is_root);

        if !root_has_root_true {
            if let Some(configs) = worktree_configs {
                for path in configs.external_config_paths.iter().flatten() {
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
