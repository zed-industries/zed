// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use env_variables::EnvVariables;
use environment_locations::list_environments;
use lazy_static::lazy_static;
use log::trace;
use manager::PoetryManager;
use pet_core::{
    env::PythonEnv,
    os_environment::Environment,
    python_environment::{PythonEnvironment, PythonEnvironmentKind},
    reporter::Reporter,
    Configuration, Locator, LocatorKind, LocatorResult, RefreshStatePersistence,
    RefreshStateSyncScope,
};
use pet_virtualenv::is_virtualenv;
use regex::Regex;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use telemetry::report_missing_envs;

pub mod config;
pub mod env_variables;
mod environment;
pub mod environment_locations;
mod environment_locations_spawn;
pub mod manager;
mod pyproject_toml;
mod telemetry;

lazy_static! {
    static ref POETRY_ENV_NAME_PATTERN: Regex = Regex::new(r"^.+-[A-Za-z0-9_-]{8}-py\d+\.\d+$")
        .expect("Error generating RegEx for poetry environment name pattern");
}

/// Check if a path looks like a Poetry environment in the cache directory
/// Poetry cache environments have names like: {name}-{hash}-py{version}
/// and are located in cache directories containing "pypoetry/virtualenvs"
fn is_poetry_cache_environment(path: &Path) -> bool {
    // Check if the environment is in a directory that looks like Poetry's virtualenvs cache
    // Common patterns:
    // - Linux: ~/.cache/pypoetry/virtualenvs/
    // - macOS: ~/Library/Caches/pypoetry/virtualenvs/
    // - Windows: %LOCALAPPDATA%\pypoetry\Cache\virtualenvs\
    if has_poetry_cache_components(path) {
        // Further validate by checking if the directory name matches Poetry's naming pattern
        // Pattern: {name}-{8-char-hash}-py{version}
        if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
            // Check for Poetry's hash-based naming: name-XXXXXXXX-py<major>.<minor>
            // The hash is 8 characters of base64url encoding
            if POETRY_ENV_NAME_PATTERN.is_match(dir_name) {
                return true;
            }
        }
    }

    false
}

fn has_poetry_cache_components(path: &Path) -> bool {
    let mut found_pypoetry = false;

    for component in path.components() {
        let Some(component) = component.as_os_str().to_str() else {
            return false;
        };

        if component.eq_ignore_ascii_case("pypoetry") {
            found_pypoetry = true;
            continue;
        }

        if found_pypoetry && component.eq_ignore_ascii_case("virtualenvs") {
            return true;
        }
    }

    false
}

/// Check if a .venv directory is an in-project Poetry environment
/// This is for the case when virtualenvs.in-project = true is set.
/// We check if the parent directory has Poetry configuration files.
fn is_in_project_poetry_environment(path: &Path) -> bool {
    // Check if this is a .venv directory
    let dir_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    if dir_name != ".venv" {
        return false;
    }

    // Check if the parent directory has Poetry configuration
    if let Some(parent) = path.parent() {
        // Check for poetry.toml - a local Poetry configuration file
        // Its presence indicates this project uses Poetry
        let poetry_toml = parent.join("poetry.toml");
        if poetry_toml.is_file() {
            trace!(
                "Found in-project Poetry environment: {:?} with poetry.toml at {:?}",
                path,
                poetry_toml
            );
            return true;
        }

        // Check if pyproject.toml contains Poetry configuration
        let pyproject_toml = parent.join("pyproject.toml");
        if pyproject_toml.is_file() {
            if let Ok(contents) = std::fs::read_to_string(&pyproject_toml) {
                // Look for [tool.poetry] or poetry as build backend
                if contents.contains("[tool.poetry]")
                    || contents.contains("poetry.core.masonry.api")
                    || contents.contains("poetry-core")
                {
                    trace!(
                        "Found in-project Poetry environment: {:?} with pyproject.toml at {:?}",
                        path,
                        pyproject_toml
                    );
                    return true;
                }
            }
        }
    }

    false
}

pub trait PoetryLocator: Send + Sync {
    fn find_and_report_missing_envs(
        &self,
        reporter: &dyn Reporter,
        poetry_executable: Option<PathBuf>,
    ) -> Option<()>;
}

pub struct Poetry {
    pub workspace_directories: Arc<RwLock<Vec<PathBuf>>>,
    pub env_vars: EnvVariables,
    pub poetry_executable: Arc<RwLock<Option<PathBuf>>>,
    search_result: Arc<RwLock<Option<LocatorResult>>>,
}

impl Poetry {
    pub fn new(environment: &dyn Environment) -> Self {
        Poetry {
            search_result: Arc::new(RwLock::new(None)),
            workspace_directories: Arc::new(RwLock::new(vec![])),
            env_vars: EnvVariables::from(environment),
            poetry_executable: Arc::new(RwLock::new(None)),
        }
    }
    fn clear(&self) {
        self.search_result.write().unwrap().take();
    }
    pub fn from(environment: &dyn Environment) -> Poetry {
        Poetry::new(environment)
    }

    pub fn sync_search_result_from(&self, source: &Poetry) {
        let search_result = source.search_result.read().unwrap().clone();
        self.search_result
            .write()
            .unwrap()
            .clone_from(&search_result);
    }

    pub fn merge_search_result_from(&self, source: &Poetry) {
        let source_search_result = source.search_result.read().unwrap().clone();
        let Some(source_search_result) = source_search_result else {
            return;
        };

        let mut merged = self
            .search_result
            .read()
            .unwrap()
            .clone()
            .unwrap_or(LocatorResult {
                managers: vec![],
                environments: vec![],
            });
        merged.managers.extend(source_search_result.managers);
        merged.managers.sort();
        merged.managers.dedup();

        merged
            .environments
            .extend(source_search_result.environments);
        merged.environments.sort();
        merged.environments.dedup();

        self.search_result.write().unwrap().replace(merged);
    }

    fn find_with_cache(&self) -> Option<LocatorResult> {
        // First check if we have cached results
        {
            let search_result = self.search_result.read().unwrap();
            if let Some(result) = search_result.clone() {
                return Some(result);
            }
        }

        // First find the manager
        let manager = manager::PoetryManager::find(
            self.poetry_executable.read().unwrap().clone(),
            &self.env_vars,
        );
        trace!("Poetry Manager {:?}", manager);
        let mut result = LocatorResult {
            managers: vec![],
            environments: vec![],
        };
        if let Some(manager) = &manager {
            result.managers.push(manager.to_manager());
        }

        let workspace_dirs = self.workspace_directories.read().unwrap().clone();
        let envs = list_environments(&self.env_vars, &workspace_dirs, manager).unwrap_or_default();
        result.environments.extend(envs.clone());

        // Having a value in the search result means that we have already searched for environments
        self.search_result.write().unwrap().replace(result.clone());

        if result.managers.is_empty() && result.environments.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

impl PoetryLocator for Poetry {
    fn find_and_report_missing_envs(
        &self,
        reporter: &dyn Reporter,
        poetry_executable: Option<PathBuf>,
    ) -> Option<()> {
        let user_provided_poetry_exe = poetry_executable.is_some();
        let manager = PoetryManager::find(poetry_executable.clone(), &self.env_vars)?;
        let poetry_executable = manager.executable.clone();

        let workspace_dirs = self.workspace_directories.read().unwrap().clone();
        let environments_using_spawn = environment_locations_spawn::list_environments(
            &poetry_executable,
            &workspace_dirs,
            &manager,
        );

        let result = self.search_result.read().unwrap().clone();
        let _ = report_missing_envs(
            reporter,
            &poetry_executable,
            workspace_dirs,
            &self.env_vars,
            &environments_using_spawn,
            result,
            user_provided_poetry_exe,
        );

        Some(())
    }
}

impl Locator for Poetry {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::Poetry
    }
    fn refresh_state(&self) -> RefreshStatePersistence {
        RefreshStatePersistence::SyncedDiscoveryState
    }
    fn sync_refresh_state_from(&self, source: &dyn Locator, scope: &RefreshStateSyncScope) {
        let source = source.as_any().downcast_ref::<Poetry>().unwrap_or_else(|| {
            panic!(
                "attempted to sync Poetry state from {:?}",
                source.get_kind()
            )
        });
        match scope {
            RefreshStateSyncScope::Full => self.sync_search_result_from(source),
            RefreshStateSyncScope::GlobalFiltered(kind)
                if self.supported_categories().contains(kind) =>
            {
                self.sync_search_result_from(source)
            }
            RefreshStateSyncScope::Workspace => self.merge_search_result_from(source),
            RefreshStateSyncScope::GlobalFiltered(_) => {}
        }
    }
    fn configure(&self, config: &Configuration) {
        let mut ws_dirs = self.workspace_directories.write().unwrap();
        ws_dirs.clear();
        if let Some(workspace_directories) = &config.workspace_directories {
            if !workspace_directories.is_empty() {
                ws_dirs.extend(workspace_directories.clone());
            }
        }
        self.poetry_executable
            .write()
            .unwrap()
            .clone_from(&config.poetry_executable);
    }

    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::Poetry]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if !is_virtualenv(env) {
            return None;
        }

        // First, check if the environment is in our cache
        if let Some(result) = self.find_with_cache() {
            for found_env in result.environments {
                if let Some(symlinks) = &found_env.symlinks {
                    if symlinks.contains(&env.executable) {
                        return Some(found_env.clone());
                    }
                }
            }
        }

        // Fallback: Check if the path looks like a Poetry environment
        // This handles cases where the environment wasn't discovered during find()
        // (e.g., workspace directories not configured, or pyproject.toml not found)
        if let Some(prefix) = &env.prefix {
            if is_poetry_cache_environment(prefix) {
                trace!(
                    "Identified Poetry environment by cache path pattern: {:?}",
                    prefix
                );
                return environment::create_poetry_env(
                    prefix,
                    prefix.clone(), // We don't have the project directory, use prefix
                    None,           // No manager available in this fallback case
                );
            }

            // Check for in-project .venv Poetry environment
            if is_in_project_poetry_environment(prefix) {
                trace!("Identified in-project Poetry environment: {:?}", prefix);
                // For in-project .venv, the project directory is the parent
                let project_dir = prefix.parent().unwrap_or(prefix).to_path_buf();
                return environment::create_poetry_env(
                    prefix,
                    project_dir,
                    None, // No manager available in this fallback case
                );
            }
        }

        None
    }

    fn find(&self, reporter: &dyn Reporter) {
        self.clear();
        if let Some(result) = self.find_with_cache() {
            for manager in result.managers {
                reporter.report_manager(&manager.clone());
            }
            for found_env in result.environments {
                reporter.report_environment(&found_env);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::os_environment::EnvironmentApi;

    fn path_from_components(components: &[&str]) -> PathBuf {
        let mut path = PathBuf::new();
        for component in components {
            path.push(component);
        }
        path
    }

    #[test]
    fn test_poetry_cache_environment_requires_exact_cache_components() {
        let path = path_from_components(&[
            "home",
            "user",
            ".cache",
            "pypoetry",
            "virtualenvs",
            "project-1a2b3c4d-py3.11",
        ]);

        assert!(is_poetry_cache_environment(&path));
    }

    #[test]
    fn test_poetry_cache_environment_allows_windows_cache_component() {
        let path = path_from_components(&[
            "Users",
            "user",
            "AppData",
            "Local",
            "pypoetry",
            "Cache",
            "virtualenvs",
            "project-1a2b3c4d-py3.11",
        ]);

        assert!(is_poetry_cache_environment(&path));
    }

    #[test]
    fn test_poetry_cache_environment_rejects_substring_cache_components() {
        let path = path_from_components(&[
            "Users",
            "pypoetry_user",
            "virtualenvs_backup",
            "project-1a2b3c4d-py3.11",
        ]);

        assert!(!is_poetry_cache_environment(&path));
    }

    #[test]
    fn test_poetry_cache_environment_requires_ordered_cache_components() {
        let path = path_from_components(&[
            "home",
            "user",
            ".cache",
            "virtualenvs",
            "pypoetry",
            "project-1a2b3c4d-py3.11",
        ]);

        assert!(!is_poetry_cache_environment(&path));
    }

    #[test]
    fn test_poetry_cache_environment_allows_mixed_case_cache_components() {
        let path = path_from_components(&[
            "Users",
            "user",
            "AppData",
            "Local",
            "PyPoetry",
            "Cache",
            "VirtualEnvs",
            "project-1a2b3c4d-py3.11",
        ]);

        assert!(is_poetry_cache_environment(&path));
    }

    #[test]
    fn test_poetry_cache_environment_requires_poetry_env_name() {
        let path = path_from_components(&[
            "home",
            "user",
            ".cache",
            "pypoetry",
            "virtualenvs",
            "not-a-poetry-env",
        ]);

        assert!(!is_poetry_cache_environment(&path));
    }

    #[test]
    fn test_sync_search_result_from_replaces_cached_result() {
        let environment = EnvironmentApi::new();
        let target = Poetry::from(&environment);
        let source = Poetry::from(&environment);

        target
            .search_result
            .write()
            .unwrap()
            .replace(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("stale".to_string()),
                    kind: Some(PythonEnvironmentKind::Poetry),
                    ..Default::default()
                }],
            });

        source
            .search_result
            .write()
            .unwrap()
            .replace(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("fresh".to_string()),
                    kind: Some(PythonEnvironmentKind::Poetry),
                    ..Default::default()
                }],
            });

        target.sync_search_result_from(&source);

        let result = target.search_result.read().unwrap().clone();
        assert_eq!(
            result.unwrap().environments[0].name.as_deref(),
            Some("fresh")
        );
    }

    #[test]
    fn test_workspace_scope_merges_search_results() {
        let environment = EnvironmentApi::new();
        let target = Poetry::from(&environment);
        let source = Poetry::from(&environment);

        target
            .search_result
            .write()
            .unwrap()
            .replace(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("existing".to_string()),
                    kind: Some(PythonEnvironmentKind::Poetry),
                    ..Default::default()
                }],
            });

        source
            .search_result
            .write()
            .unwrap()
            .replace(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("workspace".to_string()),
                    kind: Some(PythonEnvironmentKind::Poetry),
                    ..Default::default()
                }],
            });

        target.sync_refresh_state_from(&source, &RefreshStateSyncScope::Workspace);

        let result = target.search_result.read().unwrap().clone().unwrap();
        let mut names = result
            .environments
            .iter()
            .map(|environment| environment.name.clone().unwrap())
            .collect::<Vec<String>>();
        names.sort();

        assert_eq!(names, vec!["existing".to_string(), "workspace".to_string()]);
    }

    #[test]
    fn test_clear_preserves_configured_poetry_executable() {
        let environment = EnvironmentApi::new();
        let poetry = Poetry::from(&environment);
        let configured = PathBuf::from("/configured/poetry");

        poetry.configure(&Configuration {
            poetry_executable: Some(configured.clone()),
            ..Default::default()
        });
        poetry
            .search_result
            .write()
            .unwrap()
            .replace(LocatorResult {
                managers: vec![],
                environments: vec![],
            });

        poetry.clear();

        assert_eq!(
            poetry.poetry_executable.read().unwrap().clone(),
            Some(configured)
        );
        assert!(poetry.search_result.read().unwrap().is_none());
    }

    #[test]
    fn test_configure_clears_poetry_executable_when_unset() {
        let environment = EnvironmentApi::new();
        let poetry = Poetry::from(&environment);

        poetry.configure(&Configuration {
            poetry_executable: Some(PathBuf::from("/configured/poetry")),
            ..Default::default()
        });
        poetry.configure(&Configuration::default());

        assert!(poetry.poetry_executable.read().unwrap().is_none());
    }
}
