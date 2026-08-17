// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use pet_core::os_environment::Environment;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn get_search_paths_from_env_variables(environment: &dyn Environment) -> Vec<PathBuf> {
    let search_paths = environment
        .get_know_global_search_locations()
        .into_iter()
        .map(normalize_search_path)
        .collect::<HashSet<PathBuf>>();

    // Exclude files from this folder, as they would have been discovered elsewhere (windows_store)
    // Also the exe is merely a pointer to another file.
    let user_home = environment.get_user_home();
    search_paths
        .into_iter()
        .filter(|search_path| !is_windows_apps_path(search_path, user_home.as_ref()))
        .collect()
}

fn is_windows_apps_path(search_path: &Path, user_home: Option<&PathBuf>) -> bool {
    if let Some(home) = user_home {
        let apps_path = home
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("WindowsApps");
        if search_path.starts_with(apps_path) {
            return true;
        }
    }

    let components = search_path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>();

    components.windows(4).any(|components| {
        components[0].eq_ignore_ascii_case("AppData")
            && components[1].eq_ignore_ascii_case("Local")
            && components[2].eq_ignore_ascii_case("Microsoft")
            && components[3].eq_ignore_ascii_case("WindowsApps")
    })
}

/// Normalizes a search path for deduplication purposes.
///
/// On Unix: Uses fs::canonicalize to resolve symlinks. This is important for merged-usr
/// systems where /bin, /sbin, /usr/sbin are symlinks to /usr/bin - we don't want to
/// report the same Python installation multiple times.
/// See: https://github.com/microsoft/python-environment-tools/pull/200
///
/// On Windows: Uses norm_case (GetLongPathNameW) to normalize case WITHOUT resolving
/// directory junctions. This is important for tools like Scoop that use junctions
/// (e.g., python\current -> python\3.13.3). Using fs::canonicalize would resolve
/// the junction, causing symlink tracking to fail when the shim points to the
/// junction path but executables are discovered from the resolved path.
/// See: https://github.com/microsoft/python-environment-tools/issues/187
fn normalize_search_path(path: PathBuf) -> PathBuf {
    #[cfg(unix)]
    {
        std::fs::canonicalize(&path).unwrap_or(path)
    }

    #[cfg(windows)]
    {
        pet_fs::path::norm_case(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    struct TestEnvironment {
        user_home: Option<PathBuf>,
        global_search_locations: Vec<PathBuf>,
    }

    impl Environment for TestEnvironment {
        fn get_user_home(&self) -> Option<PathBuf> {
            self.user_home.clone()
        }

        fn get_root(&self) -> Option<PathBuf> {
            None
        }

        fn get_env_var(&self, _key: String) -> Option<String> {
            None
        }

        fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
            self.global_search_locations.clone()
        }
    }

    fn create_test_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "pet-env-var-path-{name}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).unwrap();
        directory
    }

    #[test]
    fn search_paths_are_deduplicated_and_windows_apps_paths_are_filtered() {
        let home = create_test_dir("home");
        let regular_path = home.join("Python");
        let windows_apps_path = home
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("WindowsApps");
        fs::create_dir_all(&regular_path).unwrap();
        fs::create_dir_all(&windows_apps_path).unwrap();

        let environment = TestEnvironment {
            user_home: Some(home.clone()),
            global_search_locations: vec![
                regular_path.clone(),
                regular_path.clone(),
                windows_apps_path,
            ],
        };

        let mut search_paths = get_search_paths_from_env_variables(&environment);
        search_paths.sort();

        assert_eq!(search_paths, vec![normalize_search_path(regular_path)]);

        fs::remove_dir_all(home).unwrap();
    }

    #[test]
    fn search_paths_are_preserved_when_home_is_unknown() {
        let environment = TestEnvironment {
            user_home: None,
            global_search_locations: vec![
                PathBuf::from("/usr/bin"),
                PathBuf::from(if cfg!(windows) {
                    r"C:\Users\User\AppData\Local\Microsoft\WindowsApps"
                } else {
                    "/Users/user/AppData/Local/Microsoft/WindowsApps"
                }),
            ],
        };

        assert_eq!(
            get_search_paths_from_env_variables(&environment),
            vec![normalize_search_path(PathBuf::from("/usr/bin"))]
        );
    }

    #[test]
    fn windows_apps_path_detection_is_case_insensitive_by_components() {
        let path = PathBuf::from(if cfg!(windows) {
            r"C:\Users\User\appdata\LOCAL\microsoft\WINDOWSAPPS"
        } else {
            "/Users/user/appdata/LOCAL/microsoft/WINDOWSAPPS"
        });

        assert!(is_windows_apps_path(&path, None));
    }

    #[test]
    fn windows_apps_path_detection_rejects_partial_component_matches() {
        let path = PathBuf::from(if cfg!(windows) {
            r"C:\Users\User\AppDataBackup\Local\Microsoft\WindowsApps"
        } else {
            "/Users/user/AppDataBackup/Local/Microsoft/WindowsApps"
        });

        assert!(!is_windows_apps_path(&path, None));
    }

    // ── additional coverage ───────────────────────────────────────

    #[test]
    fn search_paths_returns_empty_for_empty_locations() {
        let environment = TestEnvironment {
            user_home: None,
            global_search_locations: vec![],
        };

        let result = get_search_paths_from_env_variables(&environment);
        assert!(result.is_empty());
    }

    #[test]
    fn search_paths_returns_empty_when_all_are_windows_apps() {
        let home = create_test_dir("all-apps");
        let apps1 = home
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("WindowsApps");
        let apps2 = home
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("WindowsApps")
            .join("subdir");
        fs::create_dir_all(&apps1).unwrap();
        fs::create_dir_all(&apps2).unwrap();

        let environment = TestEnvironment {
            user_home: Some(home.clone()),
            global_search_locations: vec![apps1, apps2],
        };

        let result = get_search_paths_from_env_variables(&environment);
        assert!(result.is_empty());

        fs::remove_dir_all(home).unwrap();
    }

    #[test]
    fn is_windows_apps_path_matches_with_known_home() {
        let home = create_test_dir("known-home");
        let apps_path = home
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("WindowsApps");
        fs::create_dir_all(&apps_path).unwrap();

        assert!(is_windows_apps_path(&apps_path, Some(&home)));

        fs::remove_dir_all(home).unwrap();
    }

    #[test]
    fn is_windows_apps_path_matches_subdirectory_of_apps() {
        let home = create_test_dir("apps-subdir");
        let apps_subdir = home
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("WindowsApps")
            .join("PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0");
        // No need to create on disk - starts_with is a path comparison

        assert!(is_windows_apps_path(&apps_subdir, Some(&home)));

        fs::remove_dir_all(home).unwrap();
    }

    #[test]
    fn is_windows_apps_path_rejects_unrelated_path() {
        let path = PathBuf::from(if cfg!(windows) {
            r"C:\Python312"
        } else {
            "/usr/local/bin"
        });

        assert!(!is_windows_apps_path(&path, None));
    }

    #[test]
    fn normalize_search_path_handles_non_existent_path() {
        let non_existent = PathBuf::from(if cfg!(windows) {
            r"C:\this\path\does\not\exist"
        } else {
            "/this/path/does/not/exist"
        });

        // On Unix, canonicalize fails for non-existent paths so it returns original
        // On Windows, norm_case returns the path as-is
        let result = normalize_search_path(non_existent.clone());
        // Should not panic and should return something
        assert!(!result.as_os_str().is_empty());
    }

    #[test]
    fn normalize_search_path_handles_existing_path() {
        let temp = create_test_dir("normalize");

        let result = normalize_search_path(temp.clone());
        // On all platforms, an existing path should normalize successfully
        assert!(result.exists());

        fs::remove_dir_all(temp).unwrap();
    }
}
