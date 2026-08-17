// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! WinPython environment locator for Windows.
//!
//! WinPython is a portable Python distribution for Windows that is commonly used
//! in scientific and educational environments. This locator detects WinPython
//! installations by looking for characteristic directory structures and marker files.

use lazy_static::lazy_static;
use log::trace;
use pet_core::{
    env::PythonEnv,
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind},
    reporter::Reporter,
    Locator, LocatorKind, RefreshStatePersistence, RefreshStateSyncScope,
};
use pet_fs::path::norm_case;
use pet_python_utils::executable::find_executables;
use pet_virtualenv::is_virtualenv;
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

/// Environment variable users can set to point at a WinPython installation
/// (or a directory that contains one or more `WPy*` installations).
///
/// Multiple paths can be provided, separated by the platform path separator
/// (`;` on Windows, `:` elsewhere).
#[cfg(windows)]
const WINPYTHON_HOME_ENV_VAR: &str = "WINPYTHON_HOME";

lazy_static! {
    /// Regex to match WinPython top-level directory names.
    /// Examples: WPy64-31300, WPy32-3900, WPy-31100, WPy64-31300Qt5
    static ref WINPYTHON_DIR_REGEX: Regex =
        Regex::new(r"(?i)^WPy(64|32)?-?\d+").expect("error parsing WinPython directory regex");

    /// Regex to match Python folder within WinPython.
    /// Examples: python-3.13.0.amd64, python-3.9.0, python-3.10.5.amd64
    static ref PYTHON_FOLDER_REGEX: Regex =
        Regex::new(r"(?i)^python-\d+\.\d+\.\d+(\.(amd64|win32))?$")
            .expect("error parsing Python folder regex");
}

/// Marker files that indicate a WinPython installation.
const WINPYTHON_MARKER_FILES: &[&str] = &[".winpython", "winpython.ini"];

pub struct WinPython {
    /// Cached discovery result. Populated lazily by `find_with_cache()` and
    /// cleared at the start of each `find()` (refresh). Mirrors the pattern
    /// used by `WindowsStore` and `WindowsRegistry`.
    cached_environments: Arc<Mutex<Option<Arc<Vec<PythonEnvironment>>>>>,
}

impl WinPython {
    pub fn new() -> WinPython {
        WinPython {
            cached_environments: Arc::new(Mutex::new(None)),
        }
    }

    #[cfg(windows)]
    fn find_with_cache(&self) -> Arc<Vec<PythonEnvironment>> {
        {
            let cache = self
                .cached_environments
                .lock()
                .expect("cached_environments mutex poisoned");
            if let Some(envs) = cache.clone() {
                return envs;
            }
        }

        let envs = Arc::new(discover_environments(self));
        self.cached_environments
            .lock()
            .expect("cached_environments mutex poisoned")
            .replace(envs.clone());
        envs
    }

    #[cfg(windows)]
    fn clear(&self) {
        self.cached_environments
            .lock()
            .expect("cached_environments mutex poisoned")
            .take();
    }

    fn sync_cached_environments_from(&self, source: &WinPython) {
        let cache = source
            .cached_environments
            .lock()
            .expect("cached_environments mutex poisoned")
            .clone();
        self.cached_environments
            .lock()
            .expect("cached_environments mutex poisoned")
            .clone_from(&cache);
    }
}

impl Default for WinPython {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a directory is a WinPython installation root by looking for marker files.
fn is_winpython_root(path: &Path) -> bool {
    for marker in WINPYTHON_MARKER_FILES {
        if path.join(marker).exists() {
            return true;
        }
    }
    false
}

/// Check if a directory name matches the WinPython naming pattern.
fn is_winpython_dir_name(name: &str) -> bool {
    WINPYTHON_DIR_REGEX.is_match(name)
}

/// Check if a directory name matches the Python folder naming pattern within WinPython.
fn is_python_folder_name(name: &str) -> bool {
    PYTHON_FOLDER_REGEX.is_match(name)
}

/// Given a Python executable path, try to find the WinPython root directory.
/// Returns (winpython_root, python_folder) if found.
fn find_winpython_root(executable: &Path) -> Option<(PathBuf, PathBuf)> {
    // Typical structure:
    // WPy64-31300/python-3.13.0.amd64/python.exe
    // or
    // WPy64-31300/python-3.13.0.amd64/Scripts/python.exe (unlikely but possible)

    let mut current = executable.parent()?;

    // Walk up the directory tree looking for WinPython markers
    for _ in 0..5 {
        // Check if current directory has WinPython marker files
        if is_winpython_root(current) {
            // Find the python folder within this WinPython root
            if let Some(python_folder) = find_python_folder_in_winpython(current) {
                return Some((current.to_path_buf(), python_folder));
            }
        }

        // Check if parent directory name matches WinPython pattern
        if let Some(name) = current.file_name() {
            let name_str = name.to_string_lossy();
            if is_winpython_dir_name(&name_str) {
                // This might be the WinPython root
                if let Some(python_folder) = find_python_folder_in_winpython(current) {
                    return Some((current.to_path_buf(), python_folder));
                }
            }
        }

        // Move to parent directory
        current = current.parent()?;
    }

    None
}

/// Find the Python installation folder within a WinPython root directory.
fn find_python_folder_in_winpython(winpython_root: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(winpython_root).ok()?;

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if is_python_folder_name(&name_str) {
                    // Verify this folder contains python.exe
                    let python_exe = path.join(if cfg!(windows) {
                        "python.exe"
                    } else {
                        "python"
                    });
                    if python_exe.exists() {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

/// Get the version from the Python folder name.
/// Example: "python-3.13.0.amd64" -> "3.13.0"
fn version_from_folder_name(folder_name: &str) -> Option<String> {
    let name = folder_name.to_lowercase();
    if let Some(stripped) = name.strip_prefix("python-") {
        // Remove architecture suffix if present
        let version_part = stripped
            .strip_suffix(".amd64")
            .or_else(|| stripped.strip_suffix(".win32"))
            .unwrap_or(stripped);
        Some(version_part.to_string())
    } else {
        None
    }
}

/// Get the display name for a WinPython installation.
fn get_display_name(winpython_root: &Path, version: Option<&str>) -> Option<String> {
    let folder_name = winpython_root.file_name()?.to_string_lossy().to_string();

    if let Some(ver) = version {
        Some(format!("WinPython {ver}"))
    } else {
        Some(format!("WinPython ({folder_name})"))
    }
}

impl Locator for WinPython {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::WinPython
    }

    fn refresh_state(&self) -> RefreshStatePersistence {
        RefreshStatePersistence::SyncedDiscoveryState
    }

    fn sync_refresh_state_from(&self, source: &dyn Locator, scope: &RefreshStateSyncScope) {
        let source = source
            .as_any()
            .downcast_ref::<WinPython>()
            .unwrap_or_else(|| {
                panic!(
                    "attempted to sync WinPython state from {:?}",
                    source.get_kind()
                )
            });

        match scope {
            RefreshStateSyncScope::Full => self.sync_cached_environments_from(source),
            RefreshStateSyncScope::GlobalFiltered(kind)
                if self.supported_categories().contains(kind) =>
            {
                self.sync_cached_environments_from(source)
            }
            RefreshStateSyncScope::GlobalFiltered(_) | RefreshStateSyncScope::Workspace => {}
        }
    }

    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::WinPython]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        // WinPython is Windows-only
        if cfg!(not(windows)) {
            return None;
        }

        // Don't identify virtual environments as WinPython
        if is_virtualenv(env) {
            return None;
        }

        // Try to find the WinPython root from the executable path
        let (winpython_root, python_folder) = find_winpython_root(&env.executable)?;

        trace!(
            "Found WinPython installation at {:?} (python folder: {:?})",
            winpython_root,
            python_folder
        );

        Some(build_environment(
            env.executable.clone(),
            winpython_root,
            python_folder,
            env.version.clone(),
            env.symlinks.clone(),
        ))
    }

    #[cfg(windows)]
    fn find(&self, reporter: &dyn Reporter) {
        self.clear();
        for env in self.find_with_cache().iter() {
            reporter.report_environment(env);
        }
    }

    #[cfg(not(windows))]
    fn find(&self, _reporter: &dyn Reporter) {
        // WinPython is Windows-only.
    }
}

/// Build a `PythonEnvironment` for a discovered WinPython install. Shared by
/// `try_from()` and the search-path discovery walk.
fn build_environment(
    executable: PathBuf,
    winpython_root: PathBuf,
    python_folder: PathBuf,
    explicit_version: Option<String>,
    extra_symlinks: Option<Vec<PathBuf>>,
) -> PythonEnvironment {
    // Normalize the canonical executable too so it matches its corresponding
    // entry in `symlinks` on Windows (where path comparisons are case-insensitive
    // but `PathBuf` equality is not).
    let executable = norm_case(&executable);

    // Get version from folder name first; fall back to a caller-supplied value.
    let version = python_folder
        .file_name()
        .and_then(|n| version_from_folder_name(&n.to_string_lossy()))
        .or(explicit_version);

    // Collect all Python executables in the installation. We normalize *every*
    // candidate before comparing so case-only or separator-only differences on
    // Windows don't slip past `contains()` and produce duplicates after dedup.
    let mut symlinks: Vec<PathBuf> = Vec::new();
    let mut seen: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    let push =
        |p: PathBuf, symlinks: &mut Vec<PathBuf>, seen: &mut std::collections::HashSet<PathBuf>| {
            let normed = norm_case(&p);
            if seen.insert(normed.clone()) {
                symlinks.push(normed);
            }
        };

    push(executable.clone(), &mut symlinks, &mut seen);
    if let Some(extra) = extra_symlinks {
        for s in extra {
            push(s, &mut symlinks, &mut seen);
        }
    }

    // Add executables from the python folder root.
    for exe in find_executables(&python_folder) {
        push(exe, &mut symlinks, &mut seen);
    }

    // Add python* (but not pip*) executables from Scripts/.
    let scripts_dir = python_folder.join("Scripts");
    if scripts_dir.exists() {
        for exe in find_executables(&scripts_dir) {
            let exe_name = exe.file_name().map(|n| n.to_string_lossy().to_lowercase());
            if exe_name
                .as_ref()
                .is_some_and(|n| n.starts_with("python") && !n.contains("pip"))
            {
                push(exe, &mut symlinks, &mut seen);
            }
        }
    }

    symlinks.sort();

    let display_name = get_display_name(&winpython_root, version.as_deref());

    PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::WinPython))
        .display_name(display_name)
        .executable(Some(executable))
        .version(version)
        .prefix(Some(python_folder))
        .symlinks(Some(symlinks))
        .build()
}

/// Walk the (now narrow) set of search paths and return every WinPython
/// install we can identify. Called by `find_with_cache()`; never called
/// directly so callers always go through the cache.
#[cfg(windows)]
fn discover_environments(locator: &WinPython) -> Vec<PythonEnvironment> {
    discover_environments_in(locator, get_winpython_search_paths())
}

/// Testable variant of [`discover_environments`] that takes the list of
/// search paths as input rather than reading environment variables.
#[cfg(windows)]
fn discover_environments_in(
    locator: &WinPython,
    search_paths: Vec<PathBuf>,
) -> Vec<PythonEnvironment> {
    let mut found: Vec<PythonEnvironment> = Vec::new();
    let mut seen_executables: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

    for search_path in search_paths {
        if !search_path.exists() {
            continue;
        }
        trace!("Searching for WinPython in {:?}", search_path);

        // The search path itself may *be* a WinPython install (when supplied
        // via WINPYTHON_HOME pointing directly at, e.g., `D:\WPy64-31300`).
        if is_winpython_root(&search_path)
            || search_path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(is_winpython_dir_name)
        {
            collect_install(&search_path, locator, &mut found, &mut seen_executables);
            continue;
        }

        // Otherwise treat it as a directory that may contain WinPython installs.
        if let Ok(entries) = fs::read_dir(&search_path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let is_match = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(is_winpython_dir_name)
                    || is_winpython_root(&path);
                if is_match {
                    collect_install(&path, locator, &mut found, &mut seen_executables);
                }
            }
        }
    }

    found
}

#[cfg(windows)]
fn collect_install(
    winpython_root: &Path,
    locator: &WinPython,
    found: &mut Vec<PythonEnvironment>,
    seen_executables: &mut std::collections::HashSet<PathBuf>,
) {
    let Some(python_folder) = find_python_folder_in_winpython(winpython_root) else {
        return;
    };
    let python_exe = python_folder.join("python.exe");
    if !python_exe.exists() {
        return;
    }
    let normed_exe = norm_case(&python_exe);
    if !seen_executables.insert(normed_exe.clone()) {
        return;
    }
    let env = PythonEnv::new(python_exe.clone(), Some(python_folder.clone()), None);
    if let Some(found_env) = locator.try_from(&env) {
        found.push(found_env);
    }
}

/// Return paths in which to look for WinPython installations.
///
/// Historically PET scanned drive roots (`C:\`, `D:\`, `E:\`), `Program Files`,
/// `~/Downloads`, `~/Desktop`, and `~/Documents` on every refresh. Each of
/// those is a Defender hot-spot on Windows, so on every refresh PET incurred
/// hundreds of `is_dir()` stat syscalls just to *not* find a WinPython
/// install. WinPython is a niche portable distribution, so we now restrict
/// discovery to:
///
/// * `%USERPROFILE%\WinPython` — the most common opt-in convention.
/// * Whatever the user puts in the `WINPYTHON_HOME` environment variable
///   (multiple paths separated by `;`). Each entry can either *be* a
///   WinPython install or *contain* one or more.
///
/// Users with WinPython installed elsewhere (e.g. `D:\WPy64-31300`) can set
/// `WINPYTHON_HOME=D:\WPy64-31300` (or the parent directory) to opt in.
#[cfg(windows)]
fn get_winpython_search_paths() -> Vec<PathBuf> {
    use std::env;
    build_search_paths(
        env::var("USERPROFILE").ok(),
        env::var(WINPYTHON_HOME_ENV_VAR).ok(),
    )
}

/// Pure helper that builds the search-path list from user-profile and
/// WINPYTHON_HOME values. Extracted from `get_winpython_search_paths` so
/// tests can pin down the policy without mutating process env vars (which
/// races between parallel tests).
#[cfg(any(windows, test))]
fn build_search_paths(userprofile: Option<String>, winpython_home: Option<String>) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut seen: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    let mut push_unique = |p: PathBuf, paths: &mut Vec<PathBuf>| {
        // Normalize for both dedup and the actual scan path so case-only,
        // separator-only, or `\\?\`-prefix differences on Windows don't
        // produce duplicate scans, and so non-normalized inputs (e.g. mixed
        // separators in `WINPYTHON_HOME`) get resolved consistently.
        let normed = norm_case(&p);
        if seen.insert(normed.clone()) {
            paths.push(normed);
        }
    };

    // 1. Conventional location: %USERPROFILE%\WinPython
    if let Some(home) = userprofile {
        if !home.is_empty() {
            push_unique(PathBuf::from(&home).join("WinPython"), &mut paths);
        }
    }

    // 2. Opt-in via environment variable. Supports multiple paths separated
    //    by the platform's path separator (`;` on Windows).
    if let Some(extra) = winpython_home {
        for entry in std::env::split_paths(&extra) {
            if entry.as_os_str().is_empty() {
                continue;
            }
            push_unique(entry, &mut paths);
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_is_winpython_dir_name() {
        assert!(is_winpython_dir_name("WPy64-31300"));
        assert!(is_winpython_dir_name("WPy32-3900"));
        assert!(is_winpython_dir_name("WPy-31100"));
        assert!(is_winpython_dir_name("WPy64-31300Qt5"));
        assert!(is_winpython_dir_name("wpy64-31300")); // case insensitive

        assert!(!is_winpython_dir_name("Python"));
        assert!(!is_winpython_dir_name("python-3.13.0"));
        assert!(!is_winpython_dir_name("random-folder"));
    }

    #[test]
    fn test_is_python_folder_name() {
        assert!(is_python_folder_name("python-3.13.0.amd64"));
        assert!(is_python_folder_name("python-3.9.0"));
        assert!(is_python_folder_name("python-3.10.5.amd64"));
        assert!(is_python_folder_name("python-3.8.0.win32"));
        assert!(is_python_folder_name("Python-3.13.0.amd64")); // case insensitive

        assert!(!is_python_folder_name("python"));
        assert!(!is_python_folder_name("python3"));
        assert!(!is_python_folder_name("WPy64-31300"));
    }

    #[test]
    fn test_version_from_folder_name() {
        assert_eq!(
            version_from_folder_name("python-3.13.0.amd64"),
            Some("3.13.0".to_string())
        );
        assert_eq!(
            version_from_folder_name("python-3.9.0"),
            Some("3.9.0".to_string())
        );
        assert_eq!(
            version_from_folder_name("python-3.8.0.win32"),
            Some("3.8.0".to_string())
        );
        assert_eq!(
            version_from_folder_name("Python-3.10.5.amd64"),
            Some("3.10.5".to_string())
        );

        assert_eq!(version_from_folder_name("python"), None);
        assert_eq!(version_from_folder_name("not-python-3.9.0"), None);
    }

    #[test]
    fn test_get_display_name() {
        // Use a simple directory name that works on all platforms
        let path = PathBuf::from("WPy64-31300");
        assert_eq!(
            get_display_name(&path, Some("3.13.0")),
            Some("WinPython 3.13.0".to_string())
        );
        assert_eq!(
            get_display_name(&path, None),
            Some("WinPython (WPy64-31300)".to_string())
        );
    }

    #[test]
    fn test_is_winpython_root_with_marker() {
        let dir = tempdir().unwrap();
        let winpython_marker = dir.path().join(".winpython");
        File::create(&winpython_marker).unwrap();

        assert!(is_winpython_root(dir.path()));
    }

    #[test]
    fn test_is_winpython_root_with_ini_marker() {
        let dir = tempdir().unwrap();
        let winpython_ini = dir.path().join("winpython.ini");
        File::create(&winpython_ini).unwrap();

        assert!(is_winpython_root(dir.path()));
    }

    #[test]
    fn test_is_winpython_root_without_marker() {
        let dir = tempdir().unwrap();
        assert!(!is_winpython_root(dir.path()));
    }

    #[test]
    #[cfg(windows)]
    fn test_find_python_folder_in_winpython() {
        let dir = tempdir().unwrap();
        let python_folder = dir.path().join("python-3.13.0.amd64");
        fs::create_dir_all(&python_folder).unwrap();

        // Create python.exe
        let python_exe = python_folder.join("python.exe");
        File::create(&python_exe).unwrap();

        let result = find_python_folder_in_winpython(dir.path());
        assert!(result.is_some());
        assert_eq!(result.unwrap(), python_folder);
    }

    #[test]
    fn test_find_python_folder_missing_exe() {
        let dir = tempdir().unwrap();
        let python_folder = dir.path().join("python-3.13.0.amd64");
        fs::create_dir_all(&python_folder).unwrap();

        // No python.exe created
        let result = find_python_folder_in_winpython(dir.path());
        assert!(result.is_none());
    }

    #[test]
    #[cfg(windows)]
    fn test_find_winpython_root_with_marker() {
        let dir = tempdir().unwrap();

        // Create WinPython structure with marker
        let winpython_root = dir.path().join("WPy64-31300");
        fs::create_dir_all(&winpython_root).unwrap();
        File::create(winpython_root.join(".winpython")).unwrap();

        let python_folder = winpython_root.join("python-3.13.0.amd64");
        fs::create_dir_all(&python_folder).unwrap();
        let python_exe = python_folder.join("python.exe");
        File::create(&python_exe).unwrap();

        let result = find_winpython_root(&python_exe);
        assert!(result.is_some());
        let (root, folder) = result.unwrap();
        assert_eq!(root, winpython_root);
        assert_eq!(folder, python_folder);
    }

    #[test]
    #[cfg(windows)]
    fn test_find_winpython_root_by_dir_name() {
        let dir = tempdir().unwrap();

        // Create WinPython structure without marker (relying on dir name)
        let winpython_root = dir.path().join("WPy64-31300");
        fs::create_dir_all(&winpython_root).unwrap();

        let python_folder = winpython_root.join("python-3.13.0.amd64");
        fs::create_dir_all(&python_folder).unwrap();
        let python_exe = python_folder.join("python.exe");
        File::create(&python_exe).unwrap();

        let result = find_winpython_root(&python_exe);
        assert!(result.is_some());
        let (root, folder) = result.unwrap();
        assert_eq!(root, winpython_root);
        assert_eq!(folder, python_folder);
    }

    #[test]
    fn test_find_winpython_root_not_winpython() {
        let dir = tempdir().unwrap();

        // Create a regular Python structure (not WinPython)
        let python_folder = dir.path().join("some-random-folder");
        fs::create_dir_all(&python_folder).unwrap();

        #[cfg(windows)]
        let python_exe = python_folder.join("python.exe");
        #[cfg(not(windows))]
        let python_exe = python_folder.join("python");

        File::create(&python_exe).unwrap();

        let result = find_winpython_root(&python_exe);
        assert!(result.is_none());
    }

    #[test]
    fn test_winpython_locator_kind() {
        let locator = WinPython::new();
        assert_eq!(locator.get_kind(), LocatorKind::WinPython);
    }

    #[test]
    fn test_winpython_supported_categories() {
        let locator = WinPython::new();
        let categories = locator.supported_categories();
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0], PythonEnvironmentKind::WinPython);
    }

    #[test]
    fn test_winpython_refresh_state_is_synced_discovery_state() {
        let locator = WinPython::new();
        assert_eq!(
            locator.refresh_state(),
            RefreshStatePersistence::SyncedDiscoveryState
        );
    }

    /// `find_with_cache` should populate and reuse the cache; `clear` resets it.
    /// Pre-populates the cache directly so the test never touches the real
    /// filesystem or `WINPYTHON_HOME`, keeping it deterministic regardless of
    /// the host machine's WinPython state.
    #[test]
    #[cfg(windows)]
    fn test_find_with_cache_reuses_results_until_cleared() {
        let locator = WinPython::new();
        // Seed the cache with an empty Vec so `find_with_cache` short-circuits
        // without calling `discover_environments`.
        locator
            .cached_environments
            .lock()
            .unwrap()
            .replace(Arc::new(Vec::new()));

        let first = locator.find_with_cache();
        let second = locator.find_with_cache();
        // Same Arc allocation indicates the cache was reused.
        assert!(Arc::ptr_eq(&first, &second));

        locator.clear();
        assert!(locator.cached_environments.lock().unwrap().is_none());
    }

    /// `WINPYTHON_HOME` is the only opt-in for non-default locations now.
    /// We do *not* scan `C:\`, `D:\`, `E:\`, `Program Files`, `~/Downloads`,
    /// `~/Desktop`, or `~/Documents` — that was the pre-#453 behavior and
    /// caused Defender-induced p90 latency on Windows refreshes.
    #[test]
    #[cfg(windows)]
    fn test_search_paths_exclude_drive_roots_and_program_files() {
        let paths = build_search_paths(Some(r"C:\Users\test".to_string()), None);

        // `norm_case` may rewrite casing or representation when the path
        // happens to exist on the test host; compare against the same
        // normalization to keep the assertion machine-independent.
        let expected = norm_case(PathBuf::from(r"C:\Users\test\WinPython"));
        assert_eq!(paths, vec![expected]);

        for p in &paths {
            let s = p.to_string_lossy().to_lowercase();
            // No drive roots like "c:\", "d:\", "e:\".
            assert!(
                !(s.len() == 3 && s.ends_with(":\\")),
                "search paths must not include drive roots: {s}"
            );
            // No `Program Files` style entries.
            assert!(
                !s.contains("program files"),
                "search paths must not include Program Files: {s}"
            );
            // No `Downloads`, `Desktop`, `Documents`.
            for banned in ["downloads", "desktop", "documents"] {
                assert!(
                    !s.ends_with(&format!("\\{banned}")),
                    "search paths must not include user {banned}: {s}"
                );
            }
        }
    }

    /// Missing USERPROFILE is OK — we just produce no default entry.
    #[test]
    fn test_search_paths_no_userprofile() {
        let paths = build_search_paths(None, None);
        assert!(paths.is_empty(), "expected no paths, got {paths:?}");
    }

    /// `WINPYTHON_HOME` accepts multiple paths separated by the platform's
    /// path separator (`;` on Windows, `:` on Unix).
    #[test]
    fn test_winpython_home_supports_multiple_paths() {
        #[cfg(windows)]
        let joined = r"D:\WPy64-31300;E:\custom".to_string();
        #[cfg(not(windows))]
        let joined = "/opt/wpy:/srv/wpy".to_string();

        let paths = build_search_paths(None, Some(joined));

        #[cfg(windows)]
        let expected = vec![
            norm_case(PathBuf::from(r"D:\WPy64-31300")),
            norm_case(PathBuf::from(r"E:\custom")),
        ];
        #[cfg(not(windows))]
        let expected = vec![
            norm_case(PathBuf::from("/opt/wpy")),
            norm_case(PathBuf::from("/srv/wpy")),
        ];

        assert_eq!(paths, expected);
    }

    /// Duplicate entries across USERPROFILE and WINPYTHON_HOME are deduped.
    #[test]
    fn test_search_paths_deduplicate() {
        #[cfg(windows)]
        let home = r"C:\Users\test".to_string();
        #[cfg(not(windows))]
        let home = "/home/test".to_string();

        let default_path = PathBuf::from(&home).join("WinPython");
        let extra = default_path.to_string_lossy().to_string();

        let paths = build_search_paths(Some(home), Some(extra));
        assert_eq!(paths, vec![norm_case(default_path)]);
    }

    /// Build a minimal on-disk WinPython tree under `parent`:
    ///
    /// ```text
    /// parent/<name>/
    ///   .winpython
    ///   python-3.13.0.amd64/
    ///     python.exe
    /// ```
    ///
    /// Returns the WinPython root and the python.exe path.
    #[cfg(windows)]
    fn make_winpython_tree(parent: &Path, name: &str) -> (PathBuf, PathBuf) {
        let root = parent.join(name);
        fs::create_dir_all(&root).unwrap();
        File::create(root.join(".winpython")).unwrap();

        let python_folder = root.join("python-3.13.0.amd64");
        fs::create_dir_all(&python_folder).unwrap();
        let python_exe = python_folder.join(if cfg!(windows) {
            "python.exe"
        } else {
            "python"
        });
        File::create(&python_exe).unwrap();

        (root, python_exe)
    }

    /// `discover_environments_in` should find an install when the search path
    /// is a *parent directory* containing one or more WPy* directories.
    #[test]
    #[cfg(windows)]
    fn test_discover_environments_in_parent_dir() {
        let dir = tempdir().unwrap();
        let (_root, python_exe) = make_winpython_tree(dir.path(), "WPy64-31300");

        let locator = WinPython::new();
        let envs = discover_environments_in(&locator, vec![dir.path().to_path_buf()]);

        assert_eq!(envs.len(), 1, "expected one env, got {envs:?}");
        let env = &envs[0];
        assert_eq!(env.kind, Some(PythonEnvironmentKind::WinPython));
        assert_eq!(
            env.executable.as_deref(),
            Some(norm_case(&python_exe).as_path())
        );
    }

    /// `discover_environments_in` should also accept a path that *is itself*
    /// a WinPython install (the `WINPYTHON_HOME=D:\WPy64-31300` shape).
    #[test]
    #[cfg(windows)]
    fn test_discover_environments_in_direct_install() {
        let dir = tempdir().unwrap();
        let (root, _python_exe) = make_winpython_tree(dir.path(), "WPy64-31300");

        let locator = WinPython::new();
        let envs = discover_environments_in(&locator, vec![root]);

        assert_eq!(envs.len(), 1);
    }

    /// Two WPy* directories under the same parent should both be discovered,
    /// and we should not double-report when the same install is reachable
    /// via two different search paths (dedup by normalized executable).
    #[test]
    #[cfg(windows)]
    fn test_discover_environments_in_dedups() {
        let dir = tempdir().unwrap();
        let (root_a, _) = make_winpython_tree(dir.path(), "WPy64-31300");
        make_winpython_tree(dir.path(), "WPy64-31200");

        let locator = WinPython::new();
        let envs = discover_environments_in(
            &locator,
            // Pass parent twice + a direct install to exercise dedup.
            vec![dir.path().to_path_buf(), dir.path().to_path_buf(), root_a],
        );

        assert_eq!(envs.len(), 2, "expected 2 unique envs, got {envs:?}");
    }

    /// Non-existent search paths are skipped silently.
    #[test]
    #[cfg(windows)]
    fn test_discover_environments_in_ignores_missing_paths() {
        let locator = WinPython::new();
        let envs =
            discover_environments_in(&locator, vec![PathBuf::from(r"Z:\definitely\not\here")]);
        assert!(envs.is_empty());
    }

    /// Drive `find_with_cache`'s cached-hit path and the report loop
    /// (the body of `Locator::find` minus the `clear()`).
    #[test]
    #[cfg(windows)]
    fn test_find_with_cache_iteration_reports_each_environment() {
        use pet_core::manager::EnvManager;
        use pet_core::telemetry::TelemetryEvent;
        use std::sync::Mutex;

        struct Capture {
            envs: Mutex<Vec<PythonEnvironment>>,
        }
        impl Reporter for Capture {
            fn report_manager(&self, _: &EnvManager) {}
            fn report_environment(&self, env: &PythonEnvironment) {
                self.envs.lock().unwrap().push(env.clone());
            }
            fn report_telemetry(&self, _: &TelemetryEvent) {}
        }

        let dir = tempdir().unwrap();
        let (root, _) = make_winpython_tree(dir.path(), "WPy64-31300");

        let locator = WinPython::new();
        let env = PythonEnv::new(
            root.join("python-3.13.0.amd64").join("python.exe"),
            Some(root.join("python-3.13.0.amd64")),
            None,
        );
        let py_env = locator.try_from(&env).expect("try_from should succeed");

        // Drive `find_with_cache` straight to its cached-hit branch and the
        // report loop without invoking `discover_environments` (which would
        // read real env vars).
        locator
            .cached_environments
            .lock()
            .unwrap()
            .replace(Arc::new(vec![py_env]));

        let reporter = Capture {
            envs: Mutex::new(Vec::new()),
        };
        for env in locator.find_with_cache().iter() {
            reporter.report_environment(env);
        }

        let captured = reporter.envs.lock().unwrap();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].kind, Some(PythonEnvironmentKind::WinPython));
    }

    /// `sync_refresh_state_from(Full)` copies the source's cached envs.
    #[test]
    fn test_sync_refresh_state_full_scope_copies_cache() {
        let source = WinPython::new();
        source
            .cached_environments
            .lock()
            .unwrap()
            .replace(Arc::new(Vec::new()));

        let target = WinPython::new();
        assert!(target.cached_environments.lock().unwrap().is_none());

        target.sync_refresh_state_from(&source, &RefreshStateSyncScope::Full);
        assert!(target.cached_environments.lock().unwrap().is_some());
    }

    /// `sync_refresh_state_from(GlobalFiltered(WinPython))` also syncs.
    #[test]
    fn test_sync_refresh_state_matching_filtered_scope_copies_cache() {
        let source = WinPython::new();
        source
            .cached_environments
            .lock()
            .unwrap()
            .replace(Arc::new(Vec::new()));

        let target = WinPython::new();
        target.sync_refresh_state_from(
            &source,
            &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::WinPython),
        );
        assert!(target.cached_environments.lock().unwrap().is_some());
    }

    /// `GlobalFiltered` for an unrelated kind and `Workspace` are no-ops.
    #[test]
    fn test_sync_refresh_state_other_scopes_are_no_op() {
        let source = WinPython::new();
        source
            .cached_environments
            .lock()
            .unwrap()
            .replace(Arc::new(Vec::new()));

        let target = WinPython::new();
        target.sync_refresh_state_from(
            &source,
            &RefreshStateSyncScope::GlobalFiltered(PythonEnvironmentKind::Conda),
        );
        assert!(target.cached_environments.lock().unwrap().is_none());

        target.sync_refresh_state_from(&source, &RefreshStateSyncScope::Workspace);
        assert!(target.cached_environments.lock().unwrap().is_none());
    }
}
