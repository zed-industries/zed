//! Project-wide storage of configurations available, similar to task inventory but for run/debug configurations.

use std::{
    borrow::Cow,
    path::PathBuf,
    sync::Arc,
};

use collections::{HashMap, VecDeque};
use configuration::{ConfigurationTemplate, ConfigurationTemplates, ResolvedConfiguration};
use gpui::{App, AppContext, Entity};
use settings::{InvalidSettingsError, parse_json_with_comments};
use util::rel_path::RelPath;
use worktree::WorktreeId;

/// Inventory of configurations for the project
pub struct ConfigurationInventory {
    /// Recently executed configurations (LRU cache)
    last_scheduled_configurations: VecDeque<(ConfigurationSourceKind, ResolvedConfiguration)>,
    /// Configurations from settings files
    templates_from_settings: ConfigurationInventoryFor<ConfigurationTemplate>,
}

impl std::fmt::Debug for ConfigurationInventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigurationInventory")
            .field("last_scheduled_configurations", &self.last_scheduled_configurations)
            .field("templates_from_settings", &self.templates_from_settings)
            .finish()
    }
}

#[derive(Debug)]
struct ConfigurationInventoryFor<T> {
    global: HashMap<PathBuf, Vec<T>>,
    worktree: HashMap<WorktreeId, HashMap<Arc<RelPath>, Vec<T>>>,
}

impl Default for ConfigurationInventoryFor<ConfigurationTemplate> {
    fn default() -> Self {
        Self {
            global: HashMap::default(),
            worktree: HashMap::default(),
        }
    }
}

impl ConfigurationInventoryFor<ConfigurationTemplate> {
    fn worktree_configurations(
        &self,
        worktree: WorktreeId,
    ) -> impl '_ + Iterator<Item = (ConfigurationSourceKind, ConfigurationTemplate)> {
        self.worktree
            .get(&worktree)
            .into_iter()
            .flatten()
            .flat_map(|(directory, templates)| {
                templates.iter().map(move |template| (directory, template))
            })
            .map(move |(directory, template)| {
                (
                    ConfigurationSourceKind::Worktree {
                        id: worktree,
                        directory_in_worktree: directory.clone(),
                        id_base: Cow::Owned(format!(
                            "local worktree configurations from directory {directory:?}"
                        )),
                    },
                    template.clone(),
                )
            })
    }

    fn global_configurations(&self) -> impl '_ + Iterator<Item = (ConfigurationSourceKind, ConfigurationTemplate)> {
        self.global.iter().flat_map(|(file_path, templates)| {
            templates.iter().map(|template| {
                (
                    ConfigurationSourceKind::AbsPath {
                        id_base: Cow::Borrowed("global configurations.json"),
                        abs_path: file_path.clone(),
                    },
                    template.clone(),
                )
            })
        })
    }
}

/// Kind of source for configurations
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConfigurationSourceKind {
    /// User-created one-off configurations
    UserInput,
    /// Configurations from .zed/configurations.json
    Worktree {
        id: WorktreeId,
        directory_in_worktree: Arc<RelPath>,
        id_base: Cow<'static, str>,
    },
    /// Global configurations from ~/.config/zed/configurations.json
    AbsPath {
        id_base: Cow<'static, str>,
        abs_path: PathBuf,
    },
}

impl ConfigurationInventory {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {
            last_scheduled_configurations: VecDeque::new(),
            templates_from_settings: ConfigurationInventoryFor::default(),
        })
    }
    
    fn new_inner() -> Self {
        Self {
            last_scheduled_configurations: VecDeque::new(),
            templates_from_settings: ConfigurationInventoryFor::default(),
        }
    }

    /// List all available configurations
    pub fn list_configurations(
        &self,
        worktree: Option<WorktreeId>,
    ) -> Vec<(ConfigurationSourceKind, ConfigurationTemplate)> {
        let mut configurations = Vec::new();

        // Add configurations from worktree
        if let Some(worktree_id) = worktree {
            configurations.extend(self.templates_from_settings.worktree_configurations(worktree_id));
        }

        // Add global configurations
        configurations.extend(self.templates_from_settings.global_configurations());

        configurations
    }

    /// Get recently used configurations sorted by LRU
    pub fn used_configurations(&self) -> impl Iterator<Item = &ResolvedConfiguration> {
        self.last_scheduled_configurations.iter().map(|(_, config)| config)
    }

    /// Register that a configuration was scheduled
    pub fn configuration_scheduled(&mut self, source: ConfigurationSourceKind, resolved: ResolvedConfiguration) {
        const MAX_RECENT_CONFIGURATIONS: usize = 50;

        // Remove if already exists to update position
        self.last_scheduled_configurations.retain(|(_, config)| config.id != resolved.id);

        // Add to front
        self.last_scheduled_configurations.push_front((source, resolved));

        // Trim to max size
        if self.last_scheduled_configurations.len() > MAX_RECENT_CONFIGURATIONS {
            self.last_scheduled_configurations.truncate(MAX_RECENT_CONFIGURATIONS);
        }
    }

    /// Update configurations from a settings file
    pub fn update_configurations(
        &mut self,
        location: ConfigurationSettingsLocation,
        configurations: Vec<ConfigurationTemplate>,
    ) {
        match location {
            ConfigurationSettingsLocation::Global(path) => {
                if configurations.is_empty() {
                    self.templates_from_settings.global.remove(&path);
                } else {
                    self.templates_from_settings.global.insert(path, configurations);
                }
            }
            ConfigurationSettingsLocation::Worktree {
                worktree_id,
                directory_in_worktree,
            } => {
                let worktree_configs = self.templates_from_settings.worktree.entry(worktree_id).or_default();
                if configurations.is_empty() {
                    worktree_configs.remove(&directory_in_worktree);
                    if worktree_configs.is_empty() {
                        self.templates_from_settings.worktree.remove(&worktree_id);
                    }
                } else {
                    worktree_configs.insert(directory_in_worktree, configurations);
                }
            }
        }
    }

    /// Clear configurations for a specific worktree
    pub fn clear_worktree(&mut self, worktree_id: WorktreeId) {
        self.templates_from_settings.worktree.remove(&worktree_id);
        self.last_scheduled_configurations.retain(|(source, _)| {
            !matches!(source, ConfigurationSourceKind::Worktree { id, .. } if id == &worktree_id)
        });
    }

    /// Delete a previously used configuration from history
    pub fn delete_previously_used(&mut self, id: &configuration::ConfigurationId) {
        self.last_scheduled_configurations.retain(|(_, config)| &config.id != id);
    }
}

/// Location of configuration settings file
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigurationSettingsLocation {
    Global(PathBuf),
    Worktree {
        worktree_id: WorktreeId,
        directory_in_worktree: Arc<RelPath>,
    },
}

/// Parse configurations from JSON
pub fn parse_configuration_file(
    content: String,
) -> Result<Vec<ConfigurationTemplate>, InvalidSettingsError> {
    let json_value = parse_json_with_comments::<serde_json::Value>(&content)
        .map_err(|err| InvalidSettingsError::InvalidConfigurationFile(err.to_string()))?;
    let configurations: ConfigurationTemplates = serde_json::from_value(json_value)
        .map_err(|err| InvalidSettingsError::InvalidConfigurationFile(err.to_string()))?;
    Ok(configurations.0)
}
