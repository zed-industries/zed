// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(windows)]
use environments::get_registry_pythons;
use pet_conda::{utils::is_conda_env, CondaLocator};
use pet_core::{
    env::PythonEnv,
    python_environment::{PythonEnvironment, PythonEnvironmentKind},
    reporter::Reporter,
    Locator, LocatorKind, LocatorResult, RefreshStatePersistence, RefreshStateSyncScope,
};
use pet_virtualenv::is_virtualenv;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod environments;

/// Cached output of a full registry walk. Holds the registry-derived
/// `LocatorResult` plus the conda install dirs discovered via the
/// registry. The conda dirs are kept separately because
/// `conda_locator.find_and_report` writes its findings directly to the
/// reporter (not into `LocatorResult`); without remembering the dirs,
/// every cache hit in `find()` would silently drop registry-only conda
/// notifications (issue #454).
struct CachedRegistryWalk {
    #[cfg_attr(not(windows), allow(dead_code))]
    result: LocatorResult,
    #[cfg_attr(not(windows), allow(dead_code))]
    conda_install_dirs: Vec<PathBuf>,
}

pub struct WindowsRegistry {
    #[allow(dead_code)]
    conda_locator: Arc<dyn CondaLocator>,
    #[allow(dead_code)]
    search_result: Arc<Mutex<Option<Arc<CachedRegistryWalk>>>>,
}

impl WindowsRegistry {
    pub fn from(conda_locator: Arc<dyn CondaLocator>) -> WindowsRegistry {
        WindowsRegistry {
            conda_locator,
            search_result: Arc::new(Mutex::new(None)),
        }
    }
    #[cfg(windows)]
    fn find_with_cache(
        &self,
        reporter: Option<&dyn Reporter>,
    ) -> Option<(Arc<CachedRegistryWalk>, bool)> {
        // Quick cache check, then drop the lock before doing any expensive
        // work. Holding `search_result`'s mutex across `get_registry_pythons`
        // would serialize all callers behind one Defender-intercepted
        // registry walk and create lock-order risk against any locks the
        // reporter / conda locator take downstream (issue #454).
        //
        // Returns `(walk, did_walk)` so the caller can replay
        // notifications when we did NOT walk (cache hit) and avoid
        // double-reporting when we did (the walk already reported
        // inline).
        {
            let cached = self
                .search_result
                .lock()
                .expect("search_result mutex poisoned");
            if let Some(cached) = cached.as_ref() {
                return Some((Arc::clone(cached), false));
            }
        }

        let outcome = get_registry_pythons(&self.conda_locator, &reporter);
        let cached_walk = Arc::new(CachedRegistryWalk {
            result: outcome.result,
            conda_install_dirs: outcome.conda_install_dirs,
        });

        // If any worker thread panicked, the result is potentially partial.
        // Skip persisting it so the next refresh can retry the walk instead
        // of replaying a stale empty/partial cache forever (issue #454).
        if !outcome.had_panic {
            let mut cached = self
                .search_result
                .lock()
                .expect("search_result mutex poisoned");
            // Re-check under the lock: another caller may have populated
            // the cache while we were walking. Prefer their value so all
            // callers observe the same identity. We still report
            // `did_walk = true` because OUR reporter (if any) already
            // received inline notifications during the walk.
            if let Some(existing) = cached.as_ref() {
                return Some((Arc::clone(existing), true));
            }
            cached.replace(Arc::clone(&cached_walk));
        }

        Some((cached_walk, true))
    }

    fn sync_search_result_from(&self, source: &WindowsRegistry) {
        let search_result = source
            .search_result
            .lock()
            .expect("search_result mutex poisoned")
            .clone();
        self.search_result
            .lock()
            .expect("search_result mutex poisoned")
            .clone_from(&search_result);
    }
}

impl Locator for WindowsRegistry {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::WindowsRegistry
    }
    fn refresh_state(&self) -> RefreshStatePersistence {
        RefreshStatePersistence::SyncedDiscoveryState
    }
    fn sync_refresh_state_from(&self, source: &dyn Locator, scope: &RefreshStateSyncScope) {
        let source = source
            .as_any()
            .downcast_ref::<WindowsRegistry>()
            .unwrap_or_else(|| {
                panic!(
                    "attempted to sync WindowsRegistry state from {:?}",
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
            RefreshStateSyncScope::GlobalFiltered(_) | RefreshStateSyncScope::Workspace => {}
        }
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![
            PythonEnvironmentKind::WindowsRegistry,
            PythonEnvironmentKind::Conda,
        ]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        // Assume we create a virtual env from a python install,
        // Then the exe in the virtual env bin will be a symlink to the homebrew python install.
        // Hence the first part of the condition will be true, but the second part will be false.
        if is_virtualenv(env) {
            return None;
        }
        // We need to check this here, as its possible to install
        // a Python environment via an Installer that ends up in Windows Registry
        // However that environment is a conda environment.
        if let Some(env_path) = &env.prefix {
            if is_conda_env(env_path) {
                return None;
            }
        }
        #[cfg(windows)]
        {
            // Populate (or reuse) the registry-walk cache so we can answer
            // `try_from` queries for installs that are only discoverable
            // via HKLM/HKCU. The walk has reporter-only side effects
            // (notably `conda_locator.find_and_report(...)`) and
            // `try_from` can't supply a reporter, but that's safe now:
            // `find_with_cache` records every conda install dir on the
            // cached `CachedRegistryWalk`, and a later `find(reporter)`
            // cache hit replays those via `conda_locator.find_and_report`
            // (#454). Without this `try_from` would never identify
            // registry-only Pythons before the first `find()` runs.
            if let Some((cached, _did_walk)) = self.find_with_cache(None) {
                for found_env in &cached.result.environments {
                    if let Some(ref python_executable_path) = found_env.executable {
                        if python_executable_path == &env.executable {
                            return Some(found_env.clone());
                        }
                    }
                }
            }
        }
        None
    }

    #[cfg(windows)]
    fn find(&self, reporter: &dyn Reporter) {
        // We no longer reset `search_result` here: the cache may have been
        // populated via `sync_refresh_state_from` between refreshes, and
        // `find()` is invoked on transient locators per refresh, so on the
        // first refresh the cache is empty by construction. Re-clearing
        // forced every refresh to re-walk both registry hives, each of
        // which is intercepted by Windows Defender (issue #454).
        //
        // `find_with_cache` returns `(cached, did_walk)`. When it walked,
        // `get_registry_pythons` already reported entries inline to our
        // reporter, so we must NOT replay (that would double-report).
        // When it did NOT walk — either the first cache check hit, or
        // another thread populated the cache while we were entering — we
        // own the replay: registry environments and managers, plus a
        // re-invocation of `conda_locator.find_and_report` for each
        // cached conda install dir (those notifications go straight to
        // the reporter and aren't part of `LocatorResult`, so they would
        // otherwise silently disappear after the first refresh, #454).
        if let Some((cached, did_walk)) = self.find_with_cache(Some(reporter)) {
            if !did_walk {
                for manager in &cached.result.managers {
                    reporter.report_manager(manager);
                }
                for env in &cached.result.environments {
                    reporter.report_environment(env);
                }
                for conda_dir in &cached.conda_install_dirs {
                    self.conda_locator.find_and_report(reporter, conda_dir);
                }
            }
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
    use pet_conda::Conda;
    use pet_core::os_environment::EnvironmentApi;
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    use tempfile::TempDir;

    fn create_virtualenv(prefix: &Path) -> PathBuf {
        let scripts_dir = prefix.join(if cfg!(windows) { "Scripts" } else { "bin" });
        fs::create_dir_all(&scripts_dir).unwrap();
        fs::write(
            scripts_dir.join(if cfg!(windows) {
                "activate.bat"
            } else {
                "activate"
            }),
            b"",
        )
        .unwrap();
        let executable = scripts_dir.join(if cfg!(windows) {
            "python.exe"
        } else {
            "python"
        });
        fs::write(&executable, b"").unwrap();
        executable
    }

    fn create_locator() -> WindowsRegistry {
        let environment = EnvironmentApi::new();
        WindowsRegistry::from(Arc::new(Conda::from(&environment)))
    }

    fn wrap_cached(result: LocatorResult) -> Arc<CachedRegistryWalk> {
        Arc::new(CachedRegistryWalk {
            result,
            conda_install_dirs: vec![],
        })
    }

    #[test]
    fn test_windows_registry_reports_kind_categories_and_refresh_state() {
        let locator = create_locator();

        assert_eq!(locator.get_kind(), LocatorKind::WindowsRegistry);
        assert_eq!(
            locator.supported_categories(),
            vec![
                PythonEnvironmentKind::WindowsRegistry,
                PythonEnvironmentKind::Conda
            ]
        );
        assert_eq!(
            locator.refresh_state(),
            RefreshStatePersistence::SyncedDiscoveryState
        );
    }

    #[test]
    fn test_full_refresh_sync_replaces_registry_cache() {
        let shared = create_locator();
        let refreshed = create_locator();

        shared
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("stale".to_string()),
                    ..Default::default()
                }],
            }));
        refreshed
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("fresh".to_string()),
                    ..Default::default()
                }],
            }));

        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Full);

        let result = shared.search_result.lock().unwrap().clone().unwrap();
        assert_eq!(result.result.environments[0].name.as_deref(), Some("fresh"));
    }

    #[test]
    fn test_workspace_scope_does_not_replace_registry_cache() {
        let shared = create_locator();
        let refreshed = create_locator();

        shared
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("stale".to_string()),
                    ..Default::default()
                }],
            }));
        refreshed
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("fresh".to_string()),
                    ..Default::default()
                }],
            }));

        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Workspace);

        let result = shared.search_result.lock().unwrap().clone().unwrap();
        assert_eq!(result.result.environments[0].name.as_deref(), Some("stale"));
    }

    #[test]
    fn test_global_filtered_scope_syncs_supported_kinds_only() {
        let shared = create_locator();
        let refreshed = create_locator();

        shared
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("stale".to_string()),
                    ..Default::default()
                }],
            }));
        refreshed
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("fresh".to_string()),
                    ..Default::default()
                }],
            }));

        shared.sync_refresh_state_from(
            &refreshed,
            &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::WindowsRegistry),
        );
        let result = shared.search_result.lock().unwrap().clone().unwrap();
        assert_eq!(result.result.environments[0].name.as_deref(), Some("fresh"));

        shared
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("stale".to_string()),
                    ..Default::default()
                }],
            }));

        shared.sync_refresh_state_from(
            &refreshed,
            &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::Venv),
        );
        let result = shared.search_result.lock().unwrap().clone().unwrap();
        assert_eq!(result.result.environments[0].name.as_deref(), Some("stale"));
    }

    #[test]
    fn test_try_from_rejects_virtualenv_before_registry_lookup() {
        let temp_dir = TempDir::new().unwrap();
        let prefix = temp_dir.path().to_path_buf();
        let executable = create_virtualenv(&prefix);
        let env = PythonEnv::new(executable, Some(prefix), None);
        let locator = create_locator();

        assert!(locator.try_from(&env).is_none());
    }

    #[test]
    fn test_try_from_rejects_conda_prefix_before_registry_lookup() {
        let temp_dir = TempDir::new().unwrap();
        let prefix = temp_dir.path().to_path_buf();
        fs::create_dir_all(prefix.join("conda-meta")).unwrap();
        let executable = prefix.join("python.exe");
        fs::write(&executable, b"").unwrap();
        let env = PythonEnv::new(executable, Some(prefix), None);
        let locator = create_locator();

        assert!(locator.try_from(&env).is_none());
    }

    /// `find()` must NOT clear the cache before populating it. The previous
    /// implementation called `self.clear()` first, which forced every
    /// `refresh` RPC to re-walk both registry hives — a Defender-intercepted
    /// hot path tracked in #454. This test pins down the new contract:
    /// pre-populate the cache, run `find()`, and assert (a) the original
    /// entries survived (i.e. the cache was not cleared) and (b) the
    /// reporter was notified with the cached environments and managers,
    /// so cached results are still observable to refresh consumers.
    #[cfg(windows)]
    #[test]
    fn test_find_reuses_cached_results_within_locator_lifetime() {
        use pet_core::manager::EnvManager;
        use pet_core::python_environment::PythonEnvironment;
        use pet_core::reporter::Reporter;
        use pet_core::telemetry::TelemetryEvent;
        use std::sync::Mutex;

        #[derive(Default)]
        struct RecordingReporter {
            environments: Mutex<Vec<String>>,
            managers: Mutex<Vec<PathBuf>>,
        }
        impl Reporter for RecordingReporter {
            fn report_manager(&self, manager: &EnvManager) {
                self.managers
                    .lock()
                    .unwrap()
                    .push(manager.executable.clone());
            }
            fn report_environment(&self, env: &PythonEnvironment) {
                self.environments
                    .lock()
                    .unwrap()
                    .push(env.name.clone().unwrap_or_default());
            }
            fn report_telemetry(&self, _event: &TelemetryEvent) {}
        }

        let locator = create_locator();
        let cached_manager = EnvManager::new(
            PathBuf::from("C:\\fake\\python.exe"),
            pet_core::manager::EnvManagerType::Conda,
            None,
        );
        let cached = LocatorResult {
            managers: vec![cached_manager.clone()],
            environments: vec![PythonEnvironment {
                name: Some("cached".to_string()),
                ..Default::default()
            }],
        };
        locator
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(cached.clone()));

        let reporter = RecordingReporter::default();
        locator.find(&reporter);

        // (a) The cache must still be populated and unchanged.
        let after = locator
            .search_result
            .lock()
            .unwrap()
            .clone()
            .expect("cache must remain populated after find()");
        assert_eq!(
            after.result.environments.len(),
            1,
            "find() must not clear the cache before populating",
        );
        assert_eq!(after.result.environments[0].name.as_deref(), Some("cached"));
        // (b) The cached entries must have been replayed to the reporter
        // — otherwise WindowsRegistry discoveries would silently
        // disappear on every refresh after the first.
        assert_eq!(
            reporter.environments.lock().unwrap().as_slice(),
            &["cached".to_string()],
            "find() must replay cached environments to the reporter on a cache hit",
        );
        assert_eq!(
            reporter.managers.lock().unwrap().as_slice(),
            &[PathBuf::from("C:\\fake\\python.exe")],
            "find() must replay cached managers to the reporter on a cache hit",
        );
    }

    /// Smoke test: on a fresh locator (empty cache), `find()` runs the new
    /// parallel walk through HKLM and HKCU and never panics or deadlocks.
    /// The discovered environment list may legitimately be empty on a CI
    /// runner without any Python registry installs — we only assert the
    /// cache was populated (i.e. the walk completed and `Some(_)` was
    /// stored), not its contents.
    #[cfg(windows)]
    #[test]
    fn test_find_on_fresh_locator_completes_parallel_walk() {
        use pet_core::manager::EnvManager;
        use pet_core::python_environment::PythonEnvironment;
        use pet_core::reporter::Reporter;
        use pet_core::telemetry::TelemetryEvent;

        struct NoopReporter;
        impl Reporter for NoopReporter {
            fn report_manager(&self, _manager: &EnvManager) {}
            fn report_environment(&self, _env: &PythonEnvironment) {}
            fn report_telemetry(&self, _event: &TelemetryEvent) {}
        }

        let locator = create_locator();
        assert!(
            locator.search_result.lock().unwrap().is_none(),
            "freshly built locator must start with an empty cache",
        );

        locator.find(&NoopReporter);

        assert!(
            locator.search_result.lock().unwrap().is_some(),
            "find() must populate the cache after walking both hives",
        );
    }

    #[test]
    fn test_try_from_returns_none_for_plain_env_without_cache() {
        // A non-virtualenv, non-conda env returns None when no matching
        // executable is found in the registry. On Unix the registry lookup
        // is compiled out, so this always returns None after the guards.
        let temp_dir = TempDir::new().unwrap();
        let prefix = temp_dir.path().to_path_buf();
        let executable = prefix.join(if cfg!(windows) {
            "python.exe"
        } else {
            "python"
        });
        fs::create_dir_all(&prefix).unwrap();
        fs::write(&executable, b"").unwrap();
        let env = PythonEnv::new(executable, Some(prefix), None);
        let locator = create_locator();

        assert!(locator.try_from(&env).is_none());
    }

    #[test]
    fn test_sync_full_when_source_has_none_cache() {
        let shared = create_locator();
        let refreshed = create_locator();

        shared
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("existing".to_string()),
                    ..Default::default()
                }],
            }));

        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Full);

        assert!(shared.search_result.lock().unwrap().is_none());
    }

    #[test]
    fn test_sync_is_idempotent() {
        let shared = create_locator();
        let refreshed = create_locator();

        refreshed
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("fresh".to_string()),
                    ..Default::default()
                }],
            }));

        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Full);
        shared.sync_refresh_state_from(&refreshed, &RefreshStateSyncScope::Full);

        let result = shared.search_result.lock().unwrap().clone().unwrap();
        assert_eq!(result.result.environments.len(), 1);
        assert_eq!(result.result.environments[0].name.as_deref(), Some("fresh"));
    }

    #[test]
    fn test_global_filtered_conda_kind_syncs() {
        let shared = create_locator();
        let refreshed = create_locator();

        shared
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("stale".to_string()),
                    ..Default::default()
                }],
            }));
        refreshed
            .search_result
            .lock()
            .unwrap()
            .replace(wrap_cached(LocatorResult {
                managers: vec![],
                environments: vec![PythonEnvironment {
                    name: Some("fresh".to_string()),
                    ..Default::default()
                }],
            }));

        shared.sync_refresh_state_from(
            &refreshed,
            &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::Conda),
        );
        let result = shared.search_result.lock().unwrap().clone().unwrap();
        assert_eq!(result.result.environments[0].name.as_deref(), Some("fresh"));
    }
}
