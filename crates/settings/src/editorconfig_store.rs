use anyhow::{Context as _, Result};
use collections::{BTreeMap, HashMap, HashSet, btree_map};
use ec4rs::{ConfigParser, PropertiesSource, Section};
use fs::Fs;
use paths::EDITORCONFIG_NAME;
use smallvec::SmallVec;
use std::{path::Path, str::FromStr, sync::Arc};
use util::{ResultExt as _, rel_path::RelPath};

use crate::{InvalidSettingsError, WorktreeId};

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

#[derive(Clone, Default)]
pub struct EditorconfigStore {
    /// These can be shared across multiple worktrees.
    pub external_configs: BTreeMap<Arc<Path>, Editorconfig>,
    pub worktree_configs: BTreeMap<WorktreeId, WorktreeEditorconfigs>,
}

#[derive(Clone, Default)]
pub struct WorktreeEditorconfigs {
    /// Ordered from closest to filesystem root to closest to worktree root.
    pub external_config_paths: Vec<Arc<Path>>,
    pub internal_configs: BTreeMap<Arc<RelPath>, (String, Option<Editorconfig>)>,
    pub external_configs_loaded: bool,
}

impl EditorconfigStore {
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

    pub fn local_external_configs(&self) -> HashMap<Arc<Path>, Editorconfig> {
        self.external_configs
            .iter()
            .map(|(path, config)| (path.clone(), config.clone()))
            .collect()
    }

    pub fn set_local_external_configs(&mut self, new_configs: Vec<(Arc<Path>, Editorconfig)>) {
        for (path, config) in new_configs {
            self.external_configs.insert(path, config);
        }
    }

    pub fn set_external_paths_for_worktree(
        &mut self,
        worktree_id: WorktreeId,
        external_config_paths: Vec<Arc<Path>>,
    ) {
        self.worktree_configs
            .entry(worktree_id)
            .or_default()
            .external_config_paths = external_config_paths;
    }

    pub fn should_load_external_configs(&self, worktree_id: WorktreeId) -> bool {
        let Some(worktree_configs) = self.worktree_configs.get(&worktree_id) else {
            return false;
        };
        if worktree_configs.external_configs_loaded {
            return false;
        }
        if worktree_configs.internal_configs.is_empty() {
            return false;
        }
        let empty_path: Arc<RelPath> = RelPath::empty().into();
        if let Some((_, Some(parsed))) = worktree_configs.internal_configs.get(&empty_path) {
            if parsed.is_root {
                return false;
            }
        }
        true
    }

    pub fn mark_external_configs_loaded(&mut self, worktree_id: WorktreeId) {
        if let Some(configs) = self.worktree_configs.get_mut(&worktree_id) {
            configs.external_configs_loaded = true;
        }
    }

    pub fn set_local_internal_configs(
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

    pub fn clear_worktree(&mut self, root_id: WorktreeId) -> Option<WorktreeEditorconfigs> {
        self.worktree_configs.remove(&root_id)
    }

    pub async fn load_external_configs(
        fs: &Arc<dyn Fs>,
        worktree_abs_path: Arc<Path>,
        cached_configs: HashMap<Arc<Path>, Editorconfig>,
    ) -> (Vec<Arc<Path>>, Vec<(Arc<Path>, Editorconfig)>) {
        let mut external_paths = Vec::new();
        let mut new_configs = Vec::new();

        let mut current = worktree_abs_path.parent().map(|p| p.to_path_buf());

        while let Some(dir) = current {
            let dir_path: Arc<Path> = Arc::from(dir.as_path());
            if let Some(cached) = cached_configs.get(&dir_path) {
                let is_root = cached.is_root;
                external_paths.push(dir_path);
                if is_root {
                    break;
                }
            } else {
                let editorconfig_path = dir.join(EDITORCONFIG_NAME);
                if let Ok(content) = fs.load(&editorconfig_path).await {
                    match content.parse::<Editorconfig>() {
                        Ok(parsed) => {
                            let is_root = parsed.is_root;
                            external_paths.push(dir_path.clone());
                            new_configs.push((dir_path, parsed));
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

        external_paths.reverse();
        new_configs.reverse();

        (external_paths, new_configs)
    }

    pub fn clear_orphaned_external_configs(&mut self, removed: &WorktreeEditorconfigs) {
        let paths_in_use: HashSet<_> = self
            .worktree_configs
            .values()
            .flat_map(|w| w.external_config_paths.iter())
            .collect();
        for path in &removed.external_config_paths {
            if !paths_in_use.contains(path) {
                self.external_configs.remove(path);
            }
        }
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
                for path in &configs.external_config_paths {
                    if let Some(parsed_editorconfig) = self.external_configs.get(path) {
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
