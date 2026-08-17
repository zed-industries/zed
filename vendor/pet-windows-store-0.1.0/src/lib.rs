// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod env_variables;
mod environment_locations;
mod environments;

use crate::env_variables::EnvVariables;
#[cfg(windows)]
use environments::list_store_pythons;
use pet_core::env::PythonEnv;
use pet_core::python_environment::{PythonEnvironment, PythonEnvironmentKind};
use pet_core::reporter::Reporter;
use pet_core::LocatorKind;
use pet_core::{
    os_environment::Environment, Locator, RefreshStatePersistence, RefreshStateSyncScope,
};
#[cfg(any(windows, test))]
use pet_fs::path::norm_case;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
#[cfg_attr(not(any(windows, test)), allow(dead_code))]
struct CachedStoreEnvironment {
    environment: PythonEnvironment,
    normalized_symlinks: Vec<PathBuf>,
}

impl CachedStoreEnvironment {
    #[cfg(any(windows, test))]
    fn from_environment(environment: PythonEnvironment) -> Self {
        let normalized_symlinks = environment
            .symlinks
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|path| normalize_for_comparison(path))
            .collect();

        Self {
            environment,
            normalized_symlinks,
        }
    }
}

#[cfg(any(windows, test))]
fn normalize_for_comparison(path: &Path) -> PathBuf {
    let normalized = norm_case(path);
    let path_str = normalized.to_string_lossy();
    if let Some(unc_path) = path_str.strip_prefix(r"\\?\UNC\") {
        PathBuf::from(format!(r"\\{unc_path}"))
    } else if let Some(path_without_prefix) = path_str.strip_prefix(r"\\?\") {
        PathBuf::from(path_without_prefix)
    } else {
        normalized
    }
}

pub fn is_windows_app_folder_in_program_files(path: &Path) -> bool {
    let path = path.to_str().unwrap_or_default().to_ascii_lowercase();
    path.get(1..)
        .is_some_and(|path| path.starts_with(":\\program files\\windowsapps"))
}

pub struct WindowsStore {
    pub env_vars: EnvVariables,
    #[allow(dead_code)]
    environments: Arc<RwLock<Option<Arc<Vec<CachedStoreEnvironment>>>>>,
}

impl WindowsStore {
    pub fn from(environment: &dyn Environment) -> WindowsStore {
        WindowsStore {
            env_vars: EnvVariables::from(environment),
            environments: Arc::new(RwLock::new(None)),
        }
    }
    #[cfg(windows)]
    fn find_with_cache(&self) -> Option<Arc<Vec<CachedStoreEnvironment>>> {
        // First check if we have cached results
        {
            let environments = self.environments.read().unwrap();
            if let Some(environments) = environments.clone() {
                return Some(environments);
            }
        }

        let envs = Arc::new(
            list_store_pythons(&self.env_vars)
                .unwrap_or_default()
                .into_iter()
                .map(CachedStoreEnvironment::from_environment)
                .collect::<Vec<_>>(),
        );
        self.environments.write().unwrap().replace(envs.clone());
        Some(envs)
    }
    #[cfg(windows)]
    fn clear(&self) {
        self.environments.write().unwrap().take();
    }

    fn sync_environments_from(&self, source: &WindowsStore) {
        let environments = source.environments.read().unwrap().clone();
        self.environments.write().unwrap().clone_from(&environments);
    }
}

impl Locator for WindowsStore {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::WindowsStore
    }
    fn refresh_state(&self) -> RefreshStatePersistence {
        RefreshStatePersistence::SyncedDiscoveryState
    }
    fn sync_refresh_state_from(&self, source: &dyn Locator, scope: &RefreshStateSyncScope) {
        let source = source
            .as_any()
            .downcast_ref::<WindowsStore>()
            .unwrap_or_else(|| {
                panic!(
                    "attempted to sync WindowsStore state from {:?}",
                    source.get_kind()
                )
            });

        match scope {
            RefreshStateSyncScope::Full => self.sync_environments_from(source),
            RefreshStateSyncScope::GlobalFiltered(kind)
                if self.supported_categories().contains(kind) =>
            {
                self.sync_environments_from(source)
            }
            RefreshStateSyncScope::GlobalFiltered(_) | RefreshStateSyncScope::Workspace => {}
        }
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::WindowsStore]
    }

    #[cfg(windows)]
    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        use pet_core::python_environment::PythonEnvironmentBuilder;
        use pet_virtualenv::is_virtualenv;

        // A virtual environment created from a Windows Store Python may still have an
        // executable path or symlink chain that resolves back to the base Store install.
        // Even in that case, the environment itself is a virtualenv and should not be
        // classified as a Windows Store environment here.
        if is_virtualenv(env) {
            return None;
        }
        // Normalize paths to handle \\?\ prefix differences
        let list_of_possible_exes: Vec<PathBuf> = vec![env.executable.clone()]
            .into_iter()
            .chain(env.symlinks.clone().unwrap_or_default())
            .map(|p| normalize_for_comparison(&p))
            .collect();
        if let Some(environments) = self.find_with_cache() {
            for found_env in environments.iter() {
                if !found_env.normalized_symlinks.is_empty() {
                    // Check if we have found this exe.
                    if list_of_possible_exes
                        .iter()
                        .any(|exe| found_env.normalized_symlinks.contains(exe))
                    {
                        // Its possible the env discovery was not aware of the symlink
                        // E.g. if we are asked to resolve `../WindowsApp/python.exe`
                        // We will have no idea, hence this will get spawned, and then exe
                        // might be something like `../WindowsApp/PythonSoftwareFoundation.Python.3.10...`
                        // However the env found by the locator will almost never contain python.exe nor python3.exe
                        // See README.md
                        // As a result, we need to add those symlinks here.
                        let builder = PythonEnvironmentBuilder::from_environment(
                            found_env.environment.clone(),
                        )
                        .symlinks(env.symlinks.clone());
                        return Some(builder.build());
                    }
                }
            }
        }
        None
    }

    #[cfg(unix)]
    fn try_from(&self, _env: &PythonEnv) -> Option<PythonEnvironment> {
        None
    }

    #[cfg(windows)]
    fn find(&self, reporter: &dyn Reporter) {
        self.clear();
        if let Some(environments) = self.find_with_cache() {
            environments
                .iter()
                .for_each(|e| reporter.report_environment(&e.environment))
        }
    }

    #[cfg(unix)]
    fn find(&self, _reporter: &dyn Reporter) {
        //
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::os_environment::EnvironmentApi;

    fn cached_environment(name: &str) -> CachedStoreEnvironment {
        CachedStoreEnvironment::from_environment(PythonEnvironment {
            name: Some(name.to_string()),
            ..Default::default()
        })
    }

    #[test]
    fn windows_store_reports_kind_supported_categories_and_refresh_state() {
        let environment = EnvironmentApi::new();
        let locator = WindowsStore::from(&environment);

        assert_eq!(locator.get_kind(), LocatorKind::WindowsStore);
        assert_eq!(
            locator.supported_categories(),
            vec![PythonEnvironmentKind::WindowsStore]
        );
        assert_eq!(
            locator.refresh_state(),
            RefreshStatePersistence::SyncedDiscoveryState
        );
    }

    #[test]
    fn is_windows_app_folder_in_program_files_handles_windowsapps_paths_and_short_inputs() {
        assert!(is_windows_app_folder_in_program_files(Path::new(
            r"C:\Program Files\WindowsApps\PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0"
        )));
        assert!(!is_windows_app_folder_in_program_files(Path::new(
            r"C:\Users\User\AppData\Local\Microsoft\WindowsApps"
        )));
        assert!(!is_windows_app_folder_in_program_files(Path::new("")));
    }

    #[test]
    fn cached_store_environment_normalizes_symlinks_once() {
        let cached = CachedStoreEnvironment::from_environment(PythonEnvironment {
            symlinks: Some(vec![PathBuf::from(r"\\?\C:\Users\User\python.exe")]),
            ..Default::default()
        });

        assert_eq!(
            cached.normalized_symlinks,
            vec![PathBuf::from(r"C:\Users\User\python.exe")]
        );
    }

    #[test]
    fn cached_store_environment_normalizes_extended_unc_symlinks() {
        let cached = CachedStoreEnvironment::from_environment(PythonEnvironment {
            symlinks: Some(vec![PathBuf::from(r"\\?\UNC\server\share\python.exe")]),
            ..Default::default()
        });

        assert_eq!(
            cached.normalized_symlinks,
            vec![PathBuf::from(r"\\server\share\python.exe")]
        );
    }

    #[cfg(windows)]
    #[test]
    fn try_from_matches_cached_normalized_symlink() {
        let environment = EnvironmentApi::new();
        let locator = WindowsStore::from(&environment);
        let symlink =
            PathBuf::from(r"C:\Users\User\AppData\Local\Microsoft\WindowsApps\python3.11.exe");
        let store_environment = PythonEnvironment {
            kind: Some(PythonEnvironmentKind::WindowsStore),
            symlinks: Some(vec![PathBuf::from(format!(r"\\?\{}", symlink.display()))]),
            ..Default::default()
        };
        locator.environments.write().unwrap().replace(Arc::new(vec![
            CachedStoreEnvironment::from_environment(store_environment),
        ]));

        let mut env = PythonEnv::new(symlink.clone(), None, None);
        env.symlinks = Some(vec![symlink]);

        assert!(locator.try_from(&env).is_some());
    }

    #[cfg(windows)]
    #[test]
    fn try_from_normalizes_incoming_extended_prefix_symlink() {
        let environment = EnvironmentApi::new();
        let locator = WindowsStore::from(&environment);
        let symlink =
            PathBuf::from(r"C:\Users\User\AppData\Local\Microsoft\WindowsApps\python3.11.exe");
        let store_environment = PythonEnvironment {
            kind: Some(PythonEnvironmentKind::WindowsStore),
            symlinks: Some(vec![symlink.clone()]),
            ..Default::default()
        };
        locator.environments.write().unwrap().replace(Arc::new(vec![
            CachedStoreEnvironment::from_environment(store_environment),
        ]));

        let mut env = PythonEnv::new(symlink.clone(), None, None);
        env.symlinks = Some(vec![PathBuf::from(format!(r"\\?\{}", symlink.display()))]);

        assert!(locator.try_from(&env).is_some());
    }

    #[test]
    fn test_full_refresh_sync_replaces_store_cache() {
        let environment = EnvironmentApi::new();
        let shared = WindowsStore::from(&environment);
        let refreshed = WindowsStore::from(&environment);

        shared
            .environments
            .write()
            .unwrap()
            .replace(Arc::new(vec![cached_environment("stale")]));
        refreshed
            .environments
            .write()
            .unwrap()
            .replace(Arc::new(vec![cached_environment("fresh")]));

        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Full);

        let result = shared.environments.read().unwrap().clone().unwrap();
        assert_eq!(result[0].environment.name.as_deref(), Some("fresh"));
    }

    #[test]
    fn test_workspace_scope_does_not_replace_store_cache() {
        let environment = EnvironmentApi::new();
        let shared = WindowsStore::from(&environment);
        let refreshed = WindowsStore::from(&environment);

        shared
            .environments
            .write()
            .unwrap()
            .replace(Arc::new(vec![cached_environment("stale")]));
        refreshed
            .environments
            .write()
            .unwrap()
            .replace(Arc::new(vec![cached_environment("fresh")]));

        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Workspace);

        let result = shared.environments.read().unwrap().clone().unwrap();
        assert_eq!(result[0].environment.name.as_deref(), Some("stale"));
    }

    #[test]
    fn test_global_filtered_scope_syncs_only_for_windows_store_kind() {
        let environment = EnvironmentApi::new();
        let shared = WindowsStore::from(&environment);
        let refreshed = WindowsStore::from(&environment);

        shared
            .environments
            .write()
            .unwrap()
            .replace(Arc::new(vec![cached_environment("stale")]));
        refreshed
            .environments
            .write()
            .unwrap()
            .replace(Arc::new(vec![cached_environment("fresh")]));

        shared.sync_refresh_state_from(
            &refreshed,
            &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::WindowsStore),
        );
        let result = shared.environments.read().unwrap().clone().unwrap();
        assert_eq!(result[0].environment.name.as_deref(), Some("fresh"));

        shared
            .environments
            .write()
            .unwrap()
            .replace(Arc::new(vec![cached_environment("stale")]));
        shared.sync_refresh_state_from(
            &refreshed,
            &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::Conda),
        );
        let result = shared.environments.read().unwrap().clone().unwrap();
        assert_eq!(result[0].environment.name.as_deref(), Some("stale"));
    }

    #[test]
    fn test_sync_full_when_source_has_none_cache() {
        let environment = EnvironmentApi::new();
        let shared = WindowsStore::from(&environment);
        let refreshed = WindowsStore::from(&environment);

        // shared has some data, refreshed has None
        shared
            .environments
            .write()
            .unwrap()
            .replace(Arc::new(vec![cached_environment("existing")]));

        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Full);

        // After syncing from source with None cache, shared should also be None
        assert!(shared.environments.read().unwrap().is_none());
    }

    #[test]
    fn test_sync_is_idempotent() {
        let environment = EnvironmentApi::new();
        let shared = WindowsStore::from(&environment);
        let refreshed = WindowsStore::from(&environment);

        refreshed
            .environments
            .write()
            .unwrap()
            .replace(Arc::new(vec![cached_environment("fresh")]));

        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Full);
        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Full);

        let result = shared.environments.read().unwrap().clone().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].environment.name.as_deref(), Some("fresh"));
    }

    #[test]
    fn is_windows_app_folder_case_insensitive() {
        assert!(is_windows_app_folder_in_program_files(Path::new(
            r"C:\PROGRAM FILES\WINDOWSAPPS\Something"
        )));
        assert!(is_windows_app_folder_in_program_files(Path::new(
            r"d:\program files\windowsapps\something"
        )));
    }

    #[test]
    fn is_windows_app_folder_rejects_single_char_input() {
        assert!(!is_windows_app_folder_in_program_files(Path::new("C")));
        assert!(!is_windows_app_folder_in_program_files(Path::new("/")));
    }

    #[test]
    fn normalize_for_comparison_strips_extended_prefix() {
        assert_eq!(
            normalize_for_comparison(Path::new(r"\\?\C:\foo\bar.exe")),
            PathBuf::from(r"C:\foo\bar.exe")
        );
    }

    #[test]
    fn normalize_for_comparison_strips_extended_unc_prefix() {
        assert_eq!(
            normalize_for_comparison(Path::new(r"\\?\UNC\server\share\file.exe")),
            PathBuf::from(r"\\server\share\file.exe")
        );
    }

    #[test]
    fn normalize_for_comparison_preserves_plain_path() {
        let path = Path::new(r"C:\Users\User\python.exe");
        assert_eq!(normalize_for_comparison(path), PathBuf::from(path));
    }

    #[test]
    fn cached_environment_with_empty_symlinks() {
        let cached = CachedStoreEnvironment::from_environment(PythonEnvironment {
            symlinks: Some(vec![]),
            ..Default::default()
        });
        assert!(cached.normalized_symlinks.is_empty());
    }

    #[test]
    fn cached_environment_with_no_symlinks() {
        let cached = CachedStoreEnvironment::from_environment(PythonEnvironment {
            symlinks: None,
            ..Default::default()
        });
        assert!(cached.normalized_symlinks.is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn try_from_returns_none_on_unix() {
        let environment = EnvironmentApi::new();
        let locator = WindowsStore::from(&environment);
        let env = PythonEnv::new(PathBuf::from("/usr/bin/python3"), None, None);

        assert!(locator.try_from(&env).is_none());
    }
}
