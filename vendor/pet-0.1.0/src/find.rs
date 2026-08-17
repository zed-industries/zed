// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use log::{trace, warn};
use pet_conda::utils::is_conda_env;
use pet_core::env::PythonEnv;
use pet_core::os_environment::Environment;
use pet_core::python_environment::PythonEnvironmentKind;
use pet_core::reporter::Reporter;
use pet_core::telemetry::refresh_progress::{
    RefreshProgress, RefreshProgressPhase, RefreshProgressStatus,
};
use pet_core::telemetry::TelemetryEvent;
use pet_core::{Configuration, Locator, LocatorKind};
use pet_env_var_path::get_search_paths_from_env_variables;
use pet_global_virtualenvs::list_global_virtual_envs_paths;
use pet_pixi::is_pixi_env;
use pet_python_utils::executable::{
    find_executable, find_executables, should_search_for_environments_in_path,
};
use pet_venv::try_environment_from_venv_dir;
use pet_virtualenv::is_virtualenv_dir;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::{sync::Arc, thread};
use tracing::{info_span, instrument};

use crate::locators::identify_python_environment_using_locators;

pub struct Summary {
    pub total: Duration,
    pub locators: BTreeMap<LocatorKind, Duration>,
    pub breakdown: BTreeMap<&'static str, Duration>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchScope {
    /// Search for environments in global space.
    Global(PythonEnvironmentKind),
    /// Search for environments in workspace folder.
    Workspace,
}

fn report_refresh_progress(
    reporter: &dyn Reporter,
    refresh_id: Option<u64>,
    refresh_start: Instant,
    phase: RefreshProgressPhase,
    status: RefreshProgressStatus,
    phase_elapsed: Option<Duration>,
    locator: Option<(String, Option<Duration>)>,
) {
    let Some(refresh_id) = refresh_id else {
        return;
    };

    let (locator_name, locator_elapsed_ms) = locator.map_or((None, None), |(name, elapsed)| {
        (Some(name), elapsed.map(|duration| duration.as_millis()))
    });
    reporter.report_telemetry(&TelemetryEvent::RefreshProgress(RefreshProgress {
        refresh_id,
        phase,
        status,
        elapsed_ms: refresh_start.elapsed().as_millis(),
        phase_elapsed_ms: phase_elapsed.map(|duration| duration.as_millis()),
        locator_name,
        locator_elapsed_ms,
    }));
}
#[instrument(skip(reporter, configuration, locators, environment), fields(search_scope = ?search_scope))]
pub fn find_and_report_envs(
    reporter: &dyn Reporter,
    configuration: Configuration,
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    environment: &dyn Environment,
    search_scope: Option<SearchScope>,
    refresh_id: Option<u64>,
) -> Arc<Mutex<Summary>> {
    let summary = Arc::new(Mutex::new(Summary {
        total: Duration::from_secs(0),
        locators: BTreeMap::new(),
        breakdown: BTreeMap::new(),
    }));
    let refresh_start = Instant::now();

    // From settings
    let environment_directories = configuration.environment_directories.unwrap_or_default();
    let workspace_directories = configuration.workspace_directories.unwrap_or_default();
    let executables = configuration.executables.unwrap_or_default();
    let search_global = match search_scope {
        Some(SearchScope::Global(_)) => true,
        Some(SearchScope::Workspace) => false,
        _ => true,
    };
    let search_kind = match search_scope {
        Some(SearchScope::Global(kind)) => Some(kind),
        _ => None,
    };

    thread::scope(|s| {
        // 1. Find using known global locators.
        s.spawn(|| {
            // Find in all the finders
            let _span = info_span!("locators_phase").entered();
            let start = Instant::now();
            report_refresh_progress(
                reporter,
                refresh_id,
                refresh_start,
                RefreshProgressPhase::Locators,
                RefreshProgressStatus::Started,
                None,
                None,
            );
            if search_global {
                thread::scope(|s| {
                    for locator in locators.iter() {
                        if let Some(kind) = &search_kind {
                            if !locator.supported_categories().contains(kind) {
                                trace!(
                                    "Skipping locator: {:?} as it does not support {:?} (required by refresh command)",
                                    locator.get_kind(),
                                    kind
                                );
                                continue;
                            }
                        }

                        let locator = locator.clone();
                        let summary = summary.clone();
                        s.spawn(move || {
                            let locator_name = format!("{:?}", locator.get_kind());
                            let _span = info_span!("locator_find", locator = %locator_name).entered();
                            let start = Instant::now();
                            report_refresh_progress(
                                reporter,
                                refresh_id,
                                refresh_start,
                                RefreshProgressPhase::Locators,
                                RefreshProgressStatus::Started,
                                None,
                                Some((locator_name.clone(), None)),
                            );
                            trace!("Searching using locator: {:?}", locator.get_kind());
                            locator.find(reporter);
                            let elapsed = start.elapsed();
                            trace!(
                                "Completed searching using locator: {:?} in {:?}",
                                locator.get_kind(),
                                elapsed
                            );
                            summary
                                .lock()
                                .unwrap()
                                .locators
                                .insert(locator.get_kind(), elapsed);
                            report_refresh_progress(
                                reporter,
                                refresh_id,
                                refresh_start,
                                RefreshProgressPhase::Locators,
                                RefreshProgressStatus::Completed,
                                None,
                                Some((locator_name, Some(elapsed))),
                            );
                        });
                    }
                });
            }
            let elapsed = start.elapsed();
            summary
                .lock()
                .unwrap()
                .breakdown
                .insert("Locators", elapsed);
            report_refresh_progress(
                reporter,
                refresh_id,
                refresh_start,
                RefreshProgressPhase::Locators,
                RefreshProgressStatus::Completed,
                Some(elapsed),
                None,
            );
        });
        // Step 2: Search in PATH variable
        s.spawn(|| {
            let _span = info_span!("path_search_phase").entered();
            let start = Instant::now();
            report_refresh_progress(
                reporter,
                refresh_id,
                refresh_start,
                RefreshProgressPhase::Path,
                RefreshProgressStatus::Started,
                None,
                None,
            );
            if search_global {
                let global_env_search_paths: Vec<PathBuf> =
                    get_search_paths_from_env_variables(environment);

                trace!(
                    "Searching for environments in global folders: {:?}",
                    global_env_search_paths
                );
                find_python_environments(
                    &global_env_search_paths,
                    reporter,
                    locators,
                    false,
                    &global_env_search_paths,
                );
            }
            let elapsed = start.elapsed();
            summary.lock().unwrap().breakdown.insert("Path", elapsed);
            report_refresh_progress(
                reporter,
                refresh_id,
                refresh_start,
                RefreshProgressPhase::Path,
                RefreshProgressStatus::Completed,
                Some(elapsed),
                None,
            );
        });
        // Step 3: Search in some global locations for virtual envs.
        // Convert to Arc<[PathBuf]> for O(1) cloning in thread spawns
        let environment_directories: Arc<[PathBuf]> = environment_directories.into();
        let environment_directories_for_step3 = environment_directories.clone();
        let summary_for_step3 = summary.clone();
        s.spawn(move || {
            let _span = info_span!("global_virtualenvs_phase").entered();
            let start = Instant::now();
            report_refresh_progress(
                reporter,
                refresh_id,
                refresh_start,
                RefreshProgressPhase::GlobalVirtualEnvs,
                RefreshProgressStatus::Started,
                None,
                None,
            );
            if search_global {
                let mut possible_environments = vec![];

                // These are directories that contain environments, hence enumerate these directories.
                for directory in environment_directories_for_step3.iter() {
                    if let Ok(reader) = fs::read_dir(directory) {
                        possible_environments.append(
                            &mut reader
                                .filter_map(Result::ok)
                                // Use path().is_dir() instead of file_type().is_dir() to follow symlinks
                                // See: https://github.com/microsoft/python-environment-tools/issues/196
                                .filter(|d| d.path().is_dir())
                                .map(|p| p.path())
                                .collect(),
                        );
                    }
                }

                let search_paths: Vec<PathBuf> = [
                    list_global_virtual_envs_paths(
                        environment.get_env_var("VIRTUAL_ENV".into()),
                        environment.get_env_var("WORKON_HOME".into()),
                        environment.get_env_var("XDG_DATA_HOME".into()),
                        environment.get_user_home(),
                    ),
                    possible_environments,
                ]
                .concat();
                let global_env_search_paths: Vec<PathBuf> =
                    get_search_paths_from_env_variables(environment);

                trace!(
                    "Searching for environments in global venv folders: {:?}",
                    search_paths
                );

                find_python_environments(
                    &search_paths,
                    reporter,
                    locators,
                    false,
                    &global_env_search_paths,
                );
            }
            let elapsed = start.elapsed();
            summary_for_step3
                .lock()
                .unwrap()
                .breakdown
                .insert("GlobalVirtualEnvs", elapsed);
            report_refresh_progress(
                reporter,
                refresh_id,
                refresh_start,
                RefreshProgressPhase::GlobalVirtualEnvs,
                RefreshProgressStatus::Completed,
                Some(elapsed),
                None,
            );
        });
        // Step 4: Find in workspace folders too.
        // This can be merged with step 2 as well, as we're only look for environments
        // in some folders.
        // However we want step 2 to happen faster, as that list of generally much smaller.
        // This list of folders generally map to workspace folders
        // & users can have a lot of workspace folders and can have a large number fo files/directories
        // that could the discovery.
        let summary_for_step4 = summary.clone();
        s.spawn(move || {
            let _span = info_span!("workspace_search_phase").entered();
            let start = Instant::now();
            report_refresh_progress(
                reporter,
                refresh_id,
                refresh_start,
                RefreshProgressPhase::Workspaces,
                RefreshProgressStatus::Started,
                None,
                None,
            );
            thread::scope(|s| {
                // Find environments in the workspace folders.
                if !workspace_directories.is_empty() {
                    trace!(
                        "Searching for environments in workspace folders: {:?}",
                        workspace_directories
                    );
                    // Convert to Arc<[PathBuf]> for O(1) cloning in thread spawns
                    let global_env_search_paths: Arc<[PathBuf]> =
                        get_search_paths_from_env_variables(environment).into();
                    for workspace_folder in workspace_directories {
                        let global_env_search_paths = global_env_search_paths.clone();
                        let environment_directories = environment_directories.clone();
                        s.spawn(move || {
                            find_python_environments_in_workspace_folder_recursive(
                                &workspace_folder,
                                reporter,
                                locators,
                                &global_env_search_paths,
                                &environment_directories,
                            );
                        });
                    }
                }
                // Find the python exes provided.
                if !executables.is_empty() {
                    trace!("Searching for environment executables: {:?}", executables);
                    let global_env_search_paths: Vec<PathBuf> =
                        get_search_paths_from_env_variables(environment);
                    identify_python_executables_using_locators(
                        executables,
                        locators,
                        reporter,
                        &global_env_search_paths,
                    );
                }
            });

            let elapsed = start.elapsed();
            summary_for_step4
                .lock()
                .unwrap()
                .breakdown
                .insert("Workspaces", elapsed);
            report_refresh_progress(
                reporter,
                refresh_id,
                refresh_start,
                RefreshProgressPhase::Workspaces,
                RefreshProgressStatus::Completed,
                Some(elapsed),
                None,
            );
        });
    });
    summary.lock().expect("summary mutex poisoned").total = refresh_start.elapsed();

    summary
}

#[instrument(skip(reporter, locators, global_env_search_paths, environment_directories), fields(workspace = %workspace_folder.display()))]
pub fn find_python_environments_in_workspace_folder_recursive(
    workspace_folder: &PathBuf,
    reporter: &dyn Reporter,
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    global_env_search_paths: &[PathBuf],
    environment_directories: &[PathBuf],
) {
    // When searching in a directory, give preference to some paths.
    let mut paths_to_search_first = vec![
        // Possible this is a virtual env
        workspace_folder.to_path_buf(),
        // Optimize for finding these first.
        workspace_folder.join(".venv"),
        workspace_folder.join(".conda"),
        workspace_folder.join(".virtualenv"),
        workspace_folder.join("venv"),
    ];

    // Add all subdirectories of .pixi/envs/**
    if let Ok(reader) = fs::read_dir(workspace_folder.join(".pixi").join("envs")) {
        reader
            .filter_map(Result::ok)
            // Use path().is_dir() instead of file_type().is_dir() to follow symlinks
            .filter(|d| d.path().is_dir())
            .map(|p| p.path())
            .for_each(|p| paths_to_search_first.push(p));
    }

    // Possible this is an environment.
    find_python_environments_in_paths_with_locators(
        &paths_to_search_first,
        locators,
        reporter,
        true,
        global_env_search_paths,
    );

    // If this is a virtual env folder, no need to scan this.
    // Note: calling is_pixi_env after is_conda_env is redundant but kept for consistency.
    if is_virtualenv_dir(workspace_folder)
        || is_conda_env(workspace_folder)
        || is_pixi_env(workspace_folder)
    {
        return;
    }
    if let Ok(reader) = fs::read_dir(workspace_folder) {
        for folder in reader
            .filter_map(Result::ok)
            // Use path().is_dir() instead of file_type().is_dir() to follow symlinks
            .filter(|d| d.path().is_dir())
            .map(|p| p.path())
            .filter(|p| {
                // If this directory is a sub directory or is in the environment_directories, then do not search in this directory.
                if environment_directories.contains(p) {
                    return true;
                }
                if environment_directories.iter().any(|d| p.starts_with(d)) {
                    return true;
                }
                should_search_for_environments_in_path(p)
            })
            .filter(|p| !paths_to_search_first.contains(p))
        {
            find_python_environments(&[folder], reporter, locators, true, &[]);
        }
    }
}

fn find_python_environments(
    paths: &[PathBuf],
    reporter: &dyn Reporter,
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    is_workspace_folder: bool,
    global_env_search_paths: &[PathBuf],
) {
    if paths.is_empty() {
        return;
    }
    thread::scope(|s| {
        for item in paths {
            let locators = locators.clone();
            let item = item.clone();
            s.spawn(move || {
                find_python_environments_in_paths_with_locators(
                    &[item],
                    &locators,
                    reporter,
                    is_workspace_folder,
                    global_env_search_paths,
                );
            });
        }
    });
}

fn find_python_environments_in_paths_with_locators(
    paths: &[PathBuf],
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    reporter: &dyn Reporter,
    is_workspace_folder: bool,
    global_env_search_paths: &[PathBuf],
) {
    for path in paths {
        let executables = if is_workspace_folder {
            // If we're in a workspace folder, then we only need to look for bin/python or bin/python.exe
            // As workspace folders generally have either virtual env or conda env or the like.
            // They will not have environments that will ONLY have a file like `bin/python3`.
            // I.e. bin/python will almost always exist.

            // Paths like /Library/Frameworks/Python.framework/Versions/3.10/bin can end up in the current PATH variable.
            // Hence do not just look for files in a bin directory of the path.
            if let Some(executable) = find_executable(path) {
                vec![executable]
            } else {
                // No valid executable found. Check if this is a broken venv.
                // If so, report it with an error instead of silently skipping.
                if let Some(broken_env) = try_environment_from_venv_dir(path) {
                    if broken_env.error.is_some() {
                        reporter.report_environment(&broken_env);
                    }
                }
                vec![]
            }
        } else {
            // Paths like /Library/Frameworks/Python.framework/Versions/3.10/bin can end up in the current PATH variable.
            // Hence do not just look for files in a bin directory of the path.
            find_executables(path)
                .into_iter()
                .filter(|p| {
                    // Exclude python2 on macOS
                    if std::env::consts::OS == "macos" {
                        return p.to_str().unwrap_or_default() != "/usr/bin/python2";
                    }
                    true
                })
                .collect::<Vec<PathBuf>>()
        };

        identify_python_executables_using_locators(
            executables,
            locators,
            reporter,
            global_env_search_paths,
        );
    }
}

#[instrument(skip(locators, reporter, global_env_search_paths), fields(executable_count = executables.len()))]
pub fn identify_python_executables_using_locators(
    executables: Vec<PathBuf>,
    locators: &Arc<Vec<Arc<dyn Locator>>>,
    reporter: &dyn Reporter,
    global_env_search_paths: &[PathBuf],
) {
    for exe in executables.into_iter() {
        let executable = exe.clone();
        let env = PythonEnv::new(exe.to_owned(), None, None);
        if let Some(env) =
            identify_python_environment_using_locators(&env, locators, global_env_search_paths)
        {
            if let Some(manager) = &env.manager {
                reporter.report_manager(manager);
            }
            reporter.report_environment(&env);
            continue;
        } else {
            warn!("Unknown Python Env {:?}", executable);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::{manager::EnvManager, python_environment::PythonEnvironment};
    use std::fs;
    use std::path::PathBuf;
    use std::sync::Mutex as StdMutex;
    use tempfile::TempDir;

    /// Test that `path().is_dir()` properly follows symlinks to directories.
    /// This is the fix for https://github.com/microsoft/python-environment-tools/issues/196
    ///
    /// The issue was that `DirEntry::file_type().is_dir()` returns false for symlinks
    /// to directories on Unix, causing symlinked virtual environments to be missed.
    #[test]
    #[cfg(unix)]
    fn test_symlinked_directory_is_detected() {
        use std::os::unix::fs::symlink;

        // Create temporary directories
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let target_dir = tmp.path().join("actual_venv");
        let container_dir = tmp.path().join("envs");
        let symlink_dir = container_dir.join("linked_venv");

        // Create the target directory (simulating a venv)
        fs::create_dir_all(&target_dir).expect("Failed to create target dir");
        fs::create_dir_all(&container_dir).expect("Failed to create container dir");

        // Create a symlink from envs/linked_venv -> actual_venv
        symlink(&target_dir, &symlink_dir).expect("Failed to create symlink");

        // Verify the symlink was created
        assert!(symlink_dir.exists(), "Symlink should exist");

        // Test that path().is_dir() follows the symlink
        let entries: Vec<_> = fs::read_dir(&container_dir)
            .expect("Failed to read dir")
            .filter_map(Result::ok)
            .collect();

        assert_eq!(entries.len(), 1, "Should have one entry");

        let entry = &entries[0];

        // This is the OLD behavior that caused the bug:
        // file_type().is_dir() does NOT follow symlinks
        let file_type_is_dir = entry.file_type().is_ok_and(|ft| ft.is_dir());
        assert!(
            !file_type_is_dir,
            "file_type().is_dir() should return false for symlinks (this is the bug)"
        );

        // This is the NEW behavior that fixes the bug:
        // path().is_dir() DOES follow symlinks
        let path_is_dir = entry.path().is_dir();
        assert!(
            path_is_dir,
            "path().is_dir() should return true for symlinks to directories"
        );
    }

    /// Test that regular directories still work with the fix
    #[test]
    fn test_regular_directory_is_detected() {
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let container_dir = tmp.path().join("envs");
        let sub_dir = container_dir.join("my_venv");

        fs::create_dir_all(&sub_dir).expect("Failed to create dirs");

        let entries: Vec<_> = fs::read_dir(&container_dir)
            .expect("Failed to read dir")
            .filter_map(Result::ok)
            .filter(|d| d.path().is_dir())
            .collect();

        assert_eq!(entries.len(), 1, "Should detect the regular directory");
        assert!(
            entries[0].path().ends_with("my_venv"),
            "Should be the my_venv directory"
        );
    }

    /// Test that files are not incorrectly detected as directories
    #[test]
    fn test_file_is_not_detected_as_directory() {
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let container_dir = tmp.path().join("envs");
        let file_path = container_dir.join("some_file.txt");

        fs::create_dir_all(&container_dir).expect("Failed to create dirs");
        fs::write(&file_path, "test content").expect("Failed to write file");

        let dirs: Vec<_> = fs::read_dir(&container_dir)
            .expect("Failed to read dir")
            .filter_map(Result::ok)
            .filter(|d| d.path().is_dir())
            .collect();

        assert!(dirs.is_empty(), "Should not detect files as directories");
    }

    /// Test symlinked directory scenario matching the original issue:
    /// User has ~/envs with symlinks to venvs in other locations
    #[test]
    #[cfg(unix)]
    fn test_symlinked_venv_in_envs_directory() {
        use std::os::unix::fs::symlink;

        let tmp = TempDir::new().expect("Failed to create temp dir");

        // Simulate user's actual venv location
        let project_dir = tmp.path().join("projects").join("myproject");
        let actual_venv = project_dir.join(".venv");

        // Simulate ~/envs directory with symlink
        let envs_dir = tmp.path().join("envs");
        let symlinked_venv = envs_dir.join("myproject_venv");

        // Create the actual venv structure
        fs::create_dir_all(actual_venv.join("bin")).expect("Failed to create venv");
        fs::write(actual_venv.join("bin").join("python"), "").expect("Failed to create python");
        fs::write(actual_venv.join("pyvenv.cfg"), "home = /usr/bin")
            .expect("Failed to create pyvenv.cfg");

        // Create envs directory with symlink
        fs::create_dir_all(&envs_dir).expect("Failed to create envs dir");
        symlink(&actual_venv, &symlinked_venv).expect("Failed to create symlink");

        // The fix ensures this symlinked directory is discovered
        let discovered: Vec<_> = fs::read_dir(&envs_dir)
            .expect("Failed to read envs dir")
            .filter_map(Result::ok)
            .filter(|d| d.path().is_dir()) // The fix: using path().is_dir()
            .map(|d| d.path())
            .collect();

        assert_eq!(discovered.len(), 1, "Should discover the symlinked venv");
        assert_eq!(
            discovered[0], symlinked_venv,
            "Should be the symlinked venv path"
        );

        // Verify it's actually a venv by checking for pyvenv.cfg
        assert!(
            discovered[0].join("pyvenv.cfg").exists(),
            "Symlink should point to a valid venv"
        );
    }

    /// CRITICAL TEST: Verify that path().is_dir() does NOT resolve symlinks to their target paths.
    /// This ensures we use the symlink path (e.g., ~/envs/myenv) not the deep target path
    /// (e.g., /some/deep/path/to/actual/venv).
    ///
    /// This is important because:
    /// 1. Users expect to see the symlink path in their environment list
    /// 2. We don't want to accidentally traverse into deep filesystem locations
    /// 3. The symlink path is the "user-facing" path they configured
    #[test]
    #[cfg(unix)]
    fn test_symlink_path_is_preserved_not_resolved() {
        use std::os::unix::fs::symlink;

        let tmp = TempDir::new().expect("Failed to create temp dir");

        // Create a "deep" target directory structure
        let deep_target = tmp
            .path()
            .join("deep")
            .join("nested")
            .join("path")
            .join("venv");
        fs::create_dir_all(&deep_target).expect("Failed to create deep target");

        // Create a container with a symlink pointing to the deep target
        let container_dir = tmp.path().join("envs");
        let symlink_path = container_dir.join("my_venv");
        fs::create_dir_all(&container_dir).expect("Failed to create container");
        symlink(&deep_target, &symlink_path).expect("Failed to create symlink");

        // Get the discovered paths using the same pattern as our fix
        let discovered: Vec<PathBuf> = fs::read_dir(&container_dir)
            .expect("Failed to read dir")
            .filter_map(Result::ok)
            .filter(|d| d.path().is_dir()) // This follows symlink to CHECK if it's a dir
            .map(|d| d.path()) // But this returns the SYMLINK path, not the target
            .collect();

        assert_eq!(discovered.len(), 1);

        // CRITICAL: The path should be the symlink, NOT the resolved target
        assert_eq!(
            discovered[0], symlink_path,
            "Should return the symlink path, not the deep target"
        );

        // Verify we did NOT get the deep target path
        assert_ne!(
            discovered[0], deep_target,
            "Should NOT resolve to the deep target path"
        );

        // The path should NOT contain the deep nested structure
        assert!(
            !discovered[0].to_string_lossy().contains("deep/nested"),
            "Path should not contain the deep nested target structure"
        );

        // Extra verification: fs::canonicalize WOULD resolve it (showing the difference)
        // Note: We canonicalize both paths for comparison because on macOS /var is a
        // symlink to /private/var, so canonicalize resolves that too.
        let resolved = fs::canonicalize(&discovered[0]).expect("Should resolve");
        let canonical_target = fs::canonicalize(&deep_target).expect("Should resolve target");
        assert_eq!(
            resolved, canonical_target,
            "canonicalize() would resolve to target, but path() does not"
        );
    }
    struct EmptyEnvironment;

    impl Environment for EmptyEnvironment {
        fn get_user_home(&self) -> Option<PathBuf> {
            None
        }
        fn get_root(&self) -> Option<PathBuf> {
            None
        }
        fn get_env_var(&self, _key: String) -> Option<String> {
            None
        }
        fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
            Vec::new()
        }
    }

    struct NoopCondaLocator;

    impl Locator for NoopCondaLocator {
        fn get_kind(&self) -> LocatorKind {
            LocatorKind::Conda
        }
        fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
            vec![PythonEnvironmentKind::Conda]
        }
        fn try_from(&self, _env: &PythonEnv) -> Option<PythonEnvironment> {
            None
        }
        fn find(&self, _reporter: &dyn Reporter) {}
    }

    #[derive(Default)]
    struct ProgressReporter {
        events: StdMutex<Vec<TelemetryEvent>>,
    }

    impl Reporter for ProgressReporter {
        fn report_manager(&self, _manager: &EnvManager) {}
        fn report_environment(&self, _env: &PythonEnvironment) {}
        fn report_telemetry(&self, event: &TelemetryEvent) {
            self.events.lock().unwrap().push(event.clone());
        }
    }

    #[test]
    fn refresh_progress_reports_phases_and_locator_timing() {
        let reporter = ProgressReporter::default();
        let locators: Arc<Vec<Arc<dyn Locator>>> = Arc::new(vec![Arc::new(NoopCondaLocator)]);

        find_and_report_envs(
            &reporter,
            Configuration::default(),
            &locators,
            &EmptyEnvironment,
            Some(SearchScope::Global(PythonEnvironmentKind::Conda)),
            Some(42),
        );

        let events = reporter.events.lock().unwrap();
        let progress = events
            .iter()
            .filter_map(|event| match event {
                TelemetryEvent::RefreshProgress(progress) => Some(progress),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(progress.len(), 10);
        assert!(progress.iter().all(|progress| progress.refresh_id == 42));
        for phase in [
            RefreshProgressPhase::Locators,
            RefreshProgressPhase::Path,
            RefreshProgressPhase::GlobalVirtualEnvs,
            RefreshProgressPhase::Workspaces,
        ] {
            assert_eq!(
                progress
                    .iter()
                    .filter(|progress| {
                        progress.phase == phase
                            && progress.status == RefreshProgressStatus::Started
                            && progress.locator_name.is_none()
                    })
                    .count(),
                1
            );
            assert_eq!(
                progress
                    .iter()
                    .filter(|progress| {
                        progress.phase == phase
                            && progress.status == RefreshProgressStatus::Completed
                            && progress.locator_name.is_none()
                            && progress.phase_elapsed_ms.is_some()
                    })
                    .count(),
                1
            );
        }

        let locator_progress = progress
            .iter()
            .filter(|progress| progress.locator_name.as_deref() == Some("Conda"))
            .collect::<Vec<_>>();
        assert_eq!(locator_progress.len(), 2);
        assert!(locator_progress.iter().any(|progress| {
            progress.status == RefreshProgressStatus::Started
                && progress.locator_elapsed_ms.is_none()
        }));
        assert!(locator_progress.iter().any(|progress| {
            progress.status == RefreshProgressStatus::Completed
                && progress.locator_elapsed_ms.is_some()
        }));
    }
}
