// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use conda_info::CondaInfo;
use env_variables::EnvVariables;
use environment_locations::{
    get_conda_dir_from_exe, get_conda_environment_paths, get_conda_envs_from_environment_txt,
    get_environments,
};
use environments::{get_conda_environment_info, CondaEnvironment};
use log::error;
use manager::{get_mamba_manager, is_mamba_executable, CondaManager};
use pet_core::{
    cache::LocatorCache,
    env::PythonEnv,
    os_environment::Environment,
    python_environment::{PythonEnvironment, PythonEnvironmentKind},
    reporter::Reporter,
    Locator, LocatorKind, RefreshStatePersistence, RefreshStateSyncScope,
};
use pet_fs::path::norm_case;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread,
    time::SystemTime,
};
use telemetry::{get_conda_rcs_and_env_dirs, report_missing_envs};
use utils::{is_conda_env, is_conda_install};

mod conda_info;
pub mod conda_rc;
pub mod env_variables;
pub mod environment_locations;
pub mod environments;
pub mod manager;
pub mod package;
mod telemetry;
pub mod utils;

pub trait CondaLocator: Send + Sync {
    fn find_and_report(&self, reporter: &dyn Reporter, path: &Path);
    fn find_and_report_missing_envs(
        &self,
        reporter: &dyn Reporter,
        conda_executable: Option<PathBuf>,
    ) -> Option<()>;
    fn get_info_for_telemetry(&self, conda_executable: Option<PathBuf>) -> CondaTelemetryInfo;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CondaTelemetryInfo {
    pub can_spawn_conda: bool,
    pub conda_rcs: Vec<PathBuf>,
    pub env_dirs: Vec<PathBuf>,
    pub environments_txt: Option<PathBuf>,
    pub environments_txt_exists: Option<bool>,
    pub user_provided_env_found: Option<bool>,
    pub environments_from_txt: Vec<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FileFingerprint {
    modified: SystemTime,
    len: u64,
}

impl FileFingerprint {
    fn from_metadata(metadata: fs::Metadata) -> Option<Self> {
        Some(Self {
            modified: metadata.modified().ok()?,
            len: metadata.len(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CondaEnvironmentFingerprint {
    conda_meta: FileFingerprint,
    history: Option<FileFingerprint>,
}

impl CondaEnvironmentFingerprint {
    fn from_prefix(prefix: &Path) -> Option<Self> {
        let conda_meta = prefix.join("conda-meta");
        let history = match fs::metadata(conda_meta.join("history")) {
            Ok(metadata) => Some(FileFingerprint::from_metadata(metadata)?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(_) => return None,
        };

        Some(Self {
            conda_meta: FileFingerprint::from_metadata(fs::metadata(conda_meta).ok()?)?,
            history,
        })
    }
}

#[derive(Clone)]
struct CondaEnvironmentDetails {
    environment: PythonEnvironment,
    conda_dir: Option<PathBuf>,
}

#[derive(Clone)]
struct CachedCondaEnvironment {
    fingerprint: CondaEnvironmentFingerprint,
    details: CondaEnvironmentDetails,
}

type CondaEnvironmentInfoCache = Arc<RwLock<HashMap<PathBuf, CachedCondaEnvironment>>>;

pub struct Conda {
    pub environments: Arc<LocatorCache<PathBuf, PythonEnvironment>>,
    pub managers: Arc<LocatorCache<PathBuf, CondaManager>>,
    pub mamba_managers: Arc<LocatorCache<PathBuf, CondaManager>>,
    pub env_vars: EnvVariables,
    conda_executable: Arc<RwLock<Option<PathBuf>>>,
    environment_info_cache: CondaEnvironmentInfoCache,
}

impl Conda {
    pub fn from(env: &dyn Environment) -> Conda {
        Self::with_environment_info_cache(env, Arc::new(RwLock::new(HashMap::new())))
    }

    pub fn from_shared_environment_cache(env: &dyn Environment, source: &Conda) -> Conda {
        Self::with_environment_info_cache(env, source.environment_info_cache.clone())
    }

    fn with_environment_info_cache(
        env: &dyn Environment,
        environment_info_cache: CondaEnvironmentInfoCache,
    ) -> Conda {
        Conda {
            environments: Arc::new(LocatorCache::new()),
            managers: Arc::new(LocatorCache::new()),
            mamba_managers: Arc::new(LocatorCache::new()),
            env_vars: EnvVariables::from(env),
            conda_executable: Arc::new(RwLock::new(None)),
            environment_info_cache,
        }
    }

    fn clear(&self) {
        self.environments.clear();
        self.managers.clear();
        self.mamba_managers.clear();
    }

    fn get_environment_details(&self, path: &Path) -> Option<CondaEnvironmentDetails> {
        self.get_or_load_environment_details(path, || {
            let environment = get_conda_environment_info(path, &None)?;
            let conda_dir = environment.conda_dir.clone();
            Some(CondaEnvironmentDetails {
                environment: environment.to_python_environment(None),
                conda_dir,
            })
        })
    }

    fn get_or_load_environment_details<F>(
        &self,
        path: &Path,
        load: F,
    ) -> Option<CondaEnvironmentDetails>
    where
        F: FnOnce() -> Option<CondaEnvironmentDetails>,
    {
        let cache_key = norm_case(path);
        let fingerprint_before = CondaEnvironmentFingerprint::from_prefix(path);
        if let Some(fingerprint) = &fingerprint_before {
            if let Some(cached) = self
                .environment_info_cache
                .read()
                .expect("conda environment info cache lock poisoned")
                .get(&cache_key)
                .filter(|cached| &cached.fingerprint == fingerprint)
            {
                let mut details = cached.details.clone();
                details.environment.prefix = Some(path.to_path_buf());
                return Some(details);
            }
        }

        let Some(details) = load() else {
            self.environment_info_cache
                .write()
                .expect("conda environment info cache lock poisoned")
                .remove(&cache_key);
            return None;
        };
        let fingerprint_after = CondaEnvironmentFingerprint::from_prefix(path);
        let mut cache = self
            .environment_info_cache
            .write()
            .expect("conda environment info cache lock poisoned");
        if fingerprint_before.is_some() && fingerprint_before == fingerprint_after {
            cache.insert(
                cache_key,
                CachedCondaEnvironment {
                    fingerprint: fingerprint_after.expect("fingerprint checked as present"),
                    details: details.clone(),
                },
            );
        } else {
            cache.remove(&cache_key);
        }

        Some(details)
    }
}

impl CondaLocator for Conda {
    fn find_and_report_missing_envs(
        &self,
        reporter: &dyn Reporter,
        conda_executable: Option<PathBuf>,
    ) -> Option<()> {
        // Look for environments that we couldn't find without spawning conda.
        let user_provided_conda_exe = conda_executable.is_some();
        // Try the provided executable first (could be conda or mamba for backwards compat),
        // then fall back to mamba/micromamba found on PATH if conda is unavailable.
        let conda_info = CondaInfo::from(conda_executable).or_else(|| {
            let mamba_exe = manager::find_mamba_binary(&self.env_vars);
            CondaInfo::from(mamba_exe)
        })?;
        let environments_map = self.environments.clone_map();
        let new_envs = conda_info
            .envs
            .clone()
            .into_iter()
            .filter(|p| !environments_map.contains_key(p))
            .collect::<Vec<PathBuf>>();
        if new_envs.is_empty() {
            return None;
        }
        let environments = environments_map
            .into_values()
            .collect::<Vec<PythonEnvironment>>();

        let _ = report_missing_envs(
            reporter,
            &self.env_vars,
            &new_envs,
            &environments,
            &conda_info,
            user_provided_conda_exe,
        );

        Some(())
    }

    fn get_info_for_telemetry(&self, conda_executable: Option<PathBuf>) -> CondaTelemetryInfo {
        let can_spawn_conda = CondaInfo::from(conda_executable).is_some();
        let environments = self.environments.values();
        let (conda_rcs, env_dirs) = get_conda_rcs_and_env_dirs(&self.env_vars, &environments);
        let mut environments_txt = None;
        let mut environments_txt_exists = None;
        if let Some(ref home) = self.env_vars.home {
            let file = Path::new(&home).join(".conda").join("environments.txt");
            environments_txt_exists = Some(file.exists());
            environments_txt = Some(file);
        }

        let conda_exe = &self.conda_executable.read().unwrap().clone();
        let envs_found = get_conda_environment_paths(&self.env_vars, conda_exe);
        let mut user_provided_env_found = None;
        if let Some(conda_dir) = get_conda_dir_from_exe(conda_exe) {
            let conda_dir = norm_case(conda_dir);
            user_provided_env_found = Some(envs_found.contains(&conda_dir));
        }

        CondaTelemetryInfo {
            can_spawn_conda,
            conda_rcs,
            env_dirs,
            user_provided_env_found,
            environments_txt,
            environments_txt_exists,
            environments_from_txt: get_conda_envs_from_environment_txt(&self.env_vars),
        }
    }

    fn find_and_report(&self, reporter: &dyn Reporter, conda_dir: &Path) {
        if !is_conda_install(conda_dir) {
            return;
        }
        if let Some(manager) = CondaManager::from(conda_dir) {
            if let Some(conda_dir) = manager.conda_dir.clone() {
                // Keep track to search again later.
                // Possible we'll find environments in other directories created using this manager
                self.managers.insert(conda_dir.clone(), manager.clone());

                // Also check for a mamba/micromamba manager in the same directory and report it.
                let _ = self
                    .mamba_managers
                    .get_or_insert_with(conda_dir.clone(), || {
                        let mgr = get_mamba_manager(&conda_dir);
                        if let Some(ref m) = mgr {
                            reporter.report_manager(&m.to_manager());
                        }
                        mgr
                    });

                // Find all the environments in the conda install folder. (under `envs` folder)
                for conda_env in
                    get_conda_environments(&get_environments(&conda_dir), &manager.clone().into())
                {
                    // If reported earlier, no point processing this again.
                    if self.environments.contains_key(&conda_env.prefix) {
                        continue;
                    }

                    // Get the right manager for this conda env.
                    // Possible the manager is different from the one we got from the conda_dir.
                    let manager = conda_env
                        .clone()
                        .conda_dir
                        .and_then(|p| CondaManager::from(&p))
                        .unwrap_or(manager.clone());
                    let env = conda_env.to_python_environment(Some(manager.to_manager()));
                    self.environments
                        .insert(conda_env.prefix.clone(), env.clone());
                    reporter.report_manager(&manager.to_manager());
                    reporter.report_environment(&env);
                }
            }
        }
    }
}

impl Conda {
    fn get_manager(&self, conda_dir: &Path) -> Option<CondaManager> {
        self.managers
            .get_or_insert_with(conda_dir.to_path_buf(), || CondaManager::from(conda_dir))
    }
}

impl Locator for Conda {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::Conda
    }
    fn refresh_state(&self) -> RefreshStatePersistence {
        RefreshStatePersistence::SyncedDiscoveryState
    }
    fn sync_refresh_state_from(&self, source: &dyn Locator, scope: &RefreshStateSyncScope) {
        let source = source.as_any().downcast_ref::<Conda>().unwrap_or_else(|| {
            panic!("attempted to sync Conda state from {:?}", source.get_kind())
        });

        match scope {
            RefreshStateSyncScope::Full => {
                // Full refresh: replace all caches entirely.
                self.environments.clear();
                self.environments
                    .insert_many(source.environments.clone_map());

                self.managers.clear();
                self.managers.insert_many(source.managers.clone_map());

                self.mamba_managers.clear();
                self.mamba_managers
                    .insert_many(source.mamba_managers.clone_map());
            }
            RefreshStateSyncScope::GlobalFiltered(kind)
                if self.supported_categories().contains(kind) =>
            {
                // Filtered refresh: merge discoveries without clearing existing
                // caches. Today find() exhaustively discovers all conda
                // environments, but a filtered scope should not assume that and
                // must not drop entries found by a previous full refresh.
                // Trade-off: deleted environments may linger until the next Full
                // refresh, but that is preferable to silently losing live entries.
                self.environments
                    .insert_many(source.environments.clone_map());
                self.managers.insert_many(source.managers.clone_map());
                self.mamba_managers
                    .insert_many(source.mamba_managers.clone_map());
            }
            RefreshStateSyncScope::GlobalFiltered(_) | RefreshStateSyncScope::Workspace => {}
        }
    }
    fn configure(&self, config: &pet_core::Configuration) {
        self.conda_executable
            .write()
            .unwrap()
            .clone_from(&config.conda_executable);
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::Conda]
    }
    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        // Possible we do not have the prefix, but this exe is in the bin directory and its a conda env or root conda install.
        let mut prefix = env.prefix.clone();
        if prefix.is_none() {
            if let Some(parent_dir) = &env.executable.parent() {
                if is_conda_env(parent_dir) {
                    // This is a conda env (most likely root conda env as the exe is in the same directory (generally on windows))
                    prefix = Some(parent_dir.to_path_buf());
                } else if parent_dir.ends_with("bin") || parent_dir.ends_with("Scripts") {
                    if let Some(parent_dir) = parent_dir.parent() {
                        if is_conda_env(parent_dir) {
                            // This is a conda env
                            prefix = Some(parent_dir.to_path_buf());
                        }
                    }
                }
            }
        }

        let path = prefix.as_ref()?;
        if !is_conda_env(path) {
            return None;
        }

        if let Some(cached_env) = self.environments.get(path) {
            return Some(cached_env);
        }

        let details = self.get_environment_details(path)?;
        let mut environment = details.environment;
        if let Some(conda_dir) = details.conda_dir {
            if let Some(manager) = self.get_manager(&conda_dir) {
                environment.manager = Some(manager.to_manager());
            } else {
                error!(
                    "Unable to find Conda Manager for env (even though we have a conda_dir): {:?}",
                    environment
                );
            }
        } else {
            error!("Unable to find Conda Manager for env: {:?}", environment);
        }

        self.environments.insert(path.clone(), environment.clone());
        Some(environment)
    }
    fn find(&self, reporter: &dyn Reporter) {
        // Discovery outputs are rebuilt on every refresh. The separate environment info cache
        // survives and is invalidated by conda metadata fingerprints.
        self.clear();

        let env_vars = self.env_vars.clone();
        let executable = self.conda_executable.read().unwrap().clone();
        thread::scope(|s| {
            if let Some(ref exe) = executable {
                if is_mamba_executable(exe) {
                    if let Some(mamba_dir) = get_conda_dir_from_exe(&executable) {
                        if let Some(mamba_mgr) = get_mamba_manager(&mamba_dir) {
                            self.mamba_managers.insert(mamba_dir, mamba_mgr.clone());
                            reporter.report_manager(&mamba_mgr.to_manager());
                        }
                    }
                }
            }

            let possible_conda_envs = get_conda_environment_paths(&env_vars, &executable);
            let active_prefixes: HashSet<PathBuf> =
                possible_conda_envs.iter().map(norm_case).collect();
            for path in possible_conda_envs {
                s.spawn(move || {
                    let details = self.get_environment_details(&path)?;
                    let prefix = path.clone();

                    let Some(conda_dir) = details.conda_dir else {
                        error!(
                            "Unable to find Conda Manager for the Conda env: {:?}",
                            details.environment
                        );
                        self.environments
                            .insert(prefix, details.environment.clone());
                        reporter.report_environment(&details.environment);
                        return None;
                    };

                    if self.environments.contains_key(&prefix) {
                        return None;
                    }

                    let manager = self
                        .managers
                        .get_or_insert_with(conda_dir.clone(), || CondaManager::from(&conda_dir));

                    let mut environment = details.environment;
                    if let Some(manager) = manager {
                        environment.manager = Some(manager.to_manager());
                        self.environments
                            .insert(prefix.clone(), environment.clone());
                        reporter.report_manager(&manager.to_manager());
                        reporter.report_environment(&environment);

                        let _ = self
                            .mamba_managers
                            .get_or_insert_with(conda_dir.clone(), || {
                                let mgr = get_mamba_manager(&conda_dir);
                                if let Some(ref m) = mgr {
                                    reporter.report_manager(&m.to_manager());
                                }
                                mgr
                            });
                    } else {
                        error!(
                            "Unable to find Conda Manager for Conda env (even though we have a conda_dir {:?}): Env Details = {:?}",
                            conda_dir, environment
                        );
                        self.environments
                            .insert(prefix.clone(), environment.clone());
                        reporter.report_environment(&environment);
                    }
                    Some(())
                });
            }

            self.environment_info_cache
                .write()
                .expect("conda environment info cache lock poisoned")
                .retain(|prefix, _| active_prefixes.contains(prefix));
        });
    }
}
fn get_conda_environments(
    paths: &Vec<PathBuf>,
    manager: &Option<CondaManager>,
) -> Vec<CondaEnvironment> {
    paths
        .par_iter()
        .filter_map(|path| get_conda_environment_info(path, manager))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::os_environment::EnvironmentApi;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn test_details(prefix: &Path, load: usize) -> CondaEnvironmentDetails {
        CondaEnvironmentDetails {
            environment: PythonEnvironment::new(
                None,
                Some(PythonEnvironmentKind::Conda),
                Some(prefix.to_path_buf()),
                None,
                Some(load.to_string()),
            ),
            conda_dir: None,
        }
    }

    #[test]
    fn environment_info_cache_is_shared_and_invalidated_by_history_changes() {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        let prefix = std::env::temp_dir().join(format!(
            "pet-conda-environment-cache-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        let conda_meta = prefix.join("conda-meta");
        let history = conda_meta.join("history");
        fs::create_dir_all(&conda_meta).unwrap();
        fs::write(&history, "initial history").unwrap();

        let environment = EnvironmentApi::new();
        let shared = Conda::from(&environment);
        let loads = AtomicUsize::new(0);
        let first = shared
            .get_or_load_environment_details(&prefix, || {
                let load = loads.fetch_add(1, Ordering::Relaxed) + 1;
                Some(test_details(&prefix, load))
            })
            .unwrap();
        assert_eq!(first.environment.version.as_deref(), Some("1"));

        let refresh = Conda::from_shared_environment_cache(&environment, &shared);
        let cached = refresh
            .get_or_load_environment_details(&prefix, || {
                panic!("unchanged metadata should reuse the shared cache")
            })
            .unwrap();
        assert_eq!(cached.environment.version.as_deref(), Some("1"));
        assert_eq!(loads.load(Ordering::Relaxed), 1);

        fs::write(&history, "updated history with a different length").unwrap();
        let refreshed = refresh
            .get_or_load_environment_details(&prefix, || {
                let load = loads.fetch_add(1, Ordering::Relaxed) + 1;
                Some(test_details(&prefix, load))
            })
            .unwrap();
        assert_eq!(refreshed.environment.version.as_deref(), Some("2"));
        assert_eq!(loads.load(Ordering::Relaxed), 2);

        fs::remove_dir_all(prefix).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn environment_info_cache_normalizes_windows_keys() {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        let prefix = std::env::temp_dir().join(format!(
            "pet-conda-environment-cache-case-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        let conda_meta = prefix.join("conda-meta");
        fs::create_dir_all(&conda_meta).unwrap();
        fs::write(conda_meta.join("history"), "history").unwrap();

        let alternate_separators = PathBuf::from(prefix.to_string_lossy().replace('\\', "/"));
        let environment = EnvironmentApi::new();
        let locator = Conda::from(&environment);
        let loads = AtomicUsize::new(0);

        locator
            .get_or_load_environment_details(&prefix, || {
                loads.fetch_add(1, Ordering::Relaxed);
                Some(test_details(&prefix, 1))
            })
            .unwrap();
        let cached = locator
            .get_or_load_environment_details(&alternate_separators, || {
                panic!("equivalent Windows paths should reuse the cache")
            })
            .unwrap();

        assert_eq!(loads.load(Ordering::Relaxed), 1);
        assert_eq!(cached.environment.prefix, Some(alternate_separators));

        fs::remove_dir_all(prefix).unwrap();
    }
}
