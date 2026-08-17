// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use std::ffi::OsStr;
use std::{
    fs,
    path::{Path, PathBuf},
};

lazy_static! {
    static ref WINDOWS_EXE: Regex =
        Regex::new(r"python(\d+\.?)*\.exe$").expect("error parsing Windows executable regex");
    static ref UNIX_EXE: Regex =
        Regex::new(r"python(\d+\.?)*$").expect("error parsing Unix executable regex");
}

/// Checks if a path is a broken symlink (symlink that points to a non-existent target).
/// Returns true if the path is a symlink and its target does not exist.
pub fn is_broken_symlink(path: &Path) -> bool {
    // First check if it's a symlink using symlink_metadata (doesn't follow symlinks)
    if let Ok(metadata) = fs::symlink_metadata(path) {
        if metadata.file_type().is_symlink() {
            // Now check if the target exists using regular metadata (follows symlinks)
            // If this fails or returns false for exists(), then it's broken
            return !path.exists();
        }
    }
    false
}

/// Result of looking for an executable in an environment path.
#[derive(Debug, Clone)]
pub enum ExecutableResult {
    /// A valid executable was found
    Found(PathBuf),
    /// An executable path exists but is broken (e.g., broken symlink)
    Broken(PathBuf),
    /// No executable was found
    NotFound,
}

#[cfg(windows)]
pub fn find_executable(env_path: &Path) -> Option<PathBuf> {
    [
        env_path.join("Scripts").join("python.exe"),
        env_path.join("Scripts").join("python3.exe"),
        env_path.join("bin").join("python.exe"),
        env_path.join("bin").join("python3.exe"),
        env_path.join("python.exe"),
        env_path.join("python3.exe"),
    ]
    .into_iter()
    .find(|path| path.is_file())
}

#[cfg(unix)]
pub fn find_executable(env_path: &Path) -> Option<PathBuf> {
    [
        env_path.join("bin").join("python"),
        env_path.join("bin").join("python3"),
        env_path.join("python"),
        env_path.join("python3"),
    ]
    .into_iter()
    .find(|path| path.is_file())
}

/// Finds an executable in the environment path, including broken symlinks.
/// This is useful for detecting virtual environments that have broken Python executables.
#[cfg(windows)]
pub fn find_executable_or_broken(env_path: &Path) -> ExecutableResult {
    let candidates = [
        env_path.join("Scripts").join("python.exe"),
        env_path.join("Scripts").join("python3.exe"),
        env_path.join("bin").join("python.exe"),
        env_path.join("bin").join("python3.exe"),
        env_path.join("python.exe"),
        env_path.join("python3.exe"),
    ];

    // First try to find a valid executable
    if let Some(path) = candidates.iter().find(|path| path.is_file()) {
        return ExecutableResult::Found(path.clone());
    }

    // Then check for broken symlinks
    if let Some(path) = candidates.iter().find(|path| is_broken_symlink(path)) {
        return ExecutableResult::Broken(path.clone());
    }

    ExecutableResult::NotFound
}

/// Finds an executable in the environment path, including broken symlinks.
/// This is useful for detecting virtual environments that have broken Python executables.
#[cfg(unix)]
pub fn find_executable_or_broken(env_path: &Path) -> ExecutableResult {
    let candidates = [
        env_path.join("bin").join("python"),
        env_path.join("bin").join("python3"),
        env_path.join("python"),
        env_path.join("python3"),
    ];

    // First try to find a valid executable
    if let Some(path) = candidates.iter().find(|path| path.is_file()) {
        return ExecutableResult::Found(path.clone());
    }

    // Then check for broken symlinks
    if let Some(path) = candidates.iter().find(|path| is_broken_symlink(path)) {
        return ExecutableResult::Broken(path.clone());
    }

    ExecutableResult::NotFound
}

pub fn find_executables<T: AsRef<Path>>(env_path: T) -> Vec<PathBuf> {
    let mut env_path = env_path.as_ref().to_path_buf();
    // Never find exes in pyenv shims folder, they are not valid exes.
    // Pyenv can be installed at custom locations (e.g., ~/.pl/pyenv via PYENV_ROOT),
    // not just ~/.pyenv, so we check for any path ending with "shims" that has a
    // parent directory containing "pyenv".
    if is_pyenv_shims_dir(&env_path) {
        return vec![];
    }
    let mut python_executables = vec![];
    if cfg!(windows) {
        // Only windows can have a Scripts folder
        let bin = "Scripts";
        if env_path.join(bin).exists() {
            env_path = env_path.join(bin);
        }
    }
    let bin = "bin"; // Windows can have bin as well, https://github.com/microsoft/vscode-python/issues/24792
    if env_path.join(bin).exists() {
        env_path = env_path.join(bin);
    }

    // If we have python.exe or python3.exe, then enumerator files in this directory
    // We might have others like python 3.10 and python 3.11
    // If we do not find python or python3, then do not enumerate, as its very expensive.
    // This fn gets called from a number of places, e.g. to look scan all folders that are in PATH variable,
    // & a few others, and scanning all of those dirs is every expensive.
    let python_exe = if cfg!(windows) {
        "python.exe"
    } else {
        "python"
    };
    let python3_exe = if cfg!(windows) {
        "python3.exe"
    } else {
        "python3"
    };

    // On linux /home/linuxbrew/.linuxbrew/bin does not contain a `python` file
    // If you install python@3.10, then only a python3.10 exe is created in that bin directory.
    // As a compromise, we only enumerate if this is a bin directory and there are no python exes
    // Else enumerating entire directories is very expensive.
    if env_path.join(python_exe).exists()
        || env_path.join(python3_exe).exists()
        || env_path.ends_with(bin)
    {
        // Enumerate this directory and get all `python` & `pythonX.X` files.
        if let Ok(entries) = fs::read_dir(env_path) {
            for entry in entries.filter_map(Result::ok) {
                let file = entry.path();
                if file.is_file() && is_python_executable_name(&file) {
                    python_executables.push(file);
                }
            }
        }
    }

    // Ensure the exe `python` is first, instead of `python3.10`
    python_executables.sort();
    python_executables
}

pub fn is_python_executable_name(exe: &Path) -> bool {
    let name = exe
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase();
    if !name.starts_with("python") {
        return false;
    }
    // Regex to match pythonX.X.exe
    if cfg!(windows) {
        WINDOWS_EXE.is_match(&name)
    } else {
        UNIX_EXE.is_match(&name)
    }
}

/// Checks if the given path is a pyenv shims directory.
/// Pyenv shims are not valid Python executables - they are wrapper scripts that
/// redirect to the actual Python installation based on pyenv configuration.
/// Pyenv can be installed at custom locations via PYENV_ROOT (e.g., ~/.pl/pyenv),
/// not just the default ~/.pyenv location.
fn is_pyenv_shims_dir(path: &Path) -> bool {
    // Must end with "shims"
    if !path.ends_with("shims") {
        return false;
    }

    // Check if parent directory name contains "pyenv" (case-insensitive)
    // This handles: ~/.pyenv/shims, ~/.pl/pyenv/shims, /opt/pyenv/shims, etc.
    if let Some(parent) = path.parent() {
        if let Some(parent_name) = parent.file_name() {
            if let Some(name_str) = parent_name.to_str() {
                return name_str.to_lowercase().contains("pyenv");
            }
        }
    }
    false
}

pub fn should_search_for_environments_in_path<P: AsRef<Path>>(path: &P) -> bool {
    // Never search in the .git folder
    // Never search in the node_modules folder
    // Mostly copied from https://github.com/github/gitignore/blob/main/Python.gitignore
    let folders_to_ignore = [
        "node_modules",
        ".cargo",
        ".devcontainer",
        ".github",
        ".git",
        ".tox",
        ".nox",
        ".hypothesis",
        ".ipynb_checkpoints",
        ".eggs",
        ".coverage",
        ".cache",
        ".pyre",
        ".ptype",
        ".pytest_cache",
        ".vscode",
        "__pycache__",
        "__pypackages__",
        ".mypy_cache",
        "cython_debug",
        "env.bak",
        "venv.bak",
        "Scripts", // If the folder ends bin/scripts, then ignore it, as the parent is most likely an env.
        "bin", // If the folder ends bin/scripts, then ignore it, as the parent is most likely an env.
    ];
    for folder in folders_to_ignore.iter() {
        if path.as_ref().ends_with(folder) {
            trace!("Ignoring folder: {:?}", path.as_ref());
            return false;
        }
    }

    true
}

#[cfg(target_os = "windows")]
pub fn new_silent_command(program: impl AsRef<OsStr>) -> std::process::Command {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut command = std::process::Command::new(program);
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

#[cfg(not(target_os = "windows"))]
pub fn new_silent_command(program: impl AsRef<OsStr>) -> std::process::Command {
    std::process::Command::new(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_python_executable_test() {
        #[cfg(unix)]
        assert!(is_python_executable_name(PathBuf::from("python").as_path()));
        #[cfg(unix)]
        assert!(is_python_executable_name(
            PathBuf::from("python3").as_path()
        ));
        #[cfg(unix)]
        assert!(is_python_executable_name(
            PathBuf::from("python3.1").as_path()
        ));
        #[cfg(unix)]
        assert!(is_python_executable_name(
            PathBuf::from("python3.10").as_path()
        ));
        #[cfg(unix)]
        assert!(is_python_executable_name(
            PathBuf::from("python4.10").as_path()
        ));

        #[cfg(windows)]
        assert!(is_python_executable_name(
            PathBuf::from("python.exe").as_path()
        ));
        #[cfg(windows)]
        assert!(is_python_executable_name(
            PathBuf::from("python3.exe").as_path()
        ));
        #[cfg(windows)]
        assert!(is_python_executable_name(
            PathBuf::from("python3.1.exe").as_path()
        ));
        #[cfg(windows)]
        assert!(is_python_executable_name(
            PathBuf::from("python3.10.exe").as_path()
        ));
        #[cfg(windows)]
        assert!(is_python_executable_name(
            PathBuf::from("python4.10.exe").as_path()
        ));
    }
    #[test]
    fn is_not_python_executable_test() {
        #[cfg(unix)]
        assert!(!is_python_executable_name(
            PathBuf::from("pythonw").as_path()
        ));
        #[cfg(unix)]
        assert!(!is_python_executable_name(
            PathBuf::from("pythonw3").as_path()
        ));

        #[cfg(windows)]
        assert!(!is_python_executable_name(
            PathBuf::from("pythonw.exe").as_path()
        ));
        #[cfg(windows)]
        assert!(!is_python_executable_name(
            PathBuf::from("pythonw3.exe").as_path()
        ));
        #[cfg(windows)]
        assert!(!is_python_executable_name(
            PathBuf::from("python.exe.__target__").as_path()
        ));
        #[cfg(windows)]
        assert!(!is_python_executable_name(
            PathBuf::from("python3.exe.__target__").as_path()
        ));
        #[cfg(windows)]
        assert!(!is_python_executable_name(
            PathBuf::from("python3.12.exe.__target__").as_path()
        ));
    }

    #[test]
    fn embedded_python_executables_are_not_python_names() {
        // DCC tools like Maya and Houdini embed Python under non-standard names.
        // These should NOT match is_python_executable_name (they are excluded from
        // automatic discovery), but they can still be resolved via the resolve API
        // when a user explicitly provides the path (see issue #375).
        #[cfg(windows)]
        {
            assert!(!is_python_executable_name(
                PathBuf::from("mayapy.exe").as_path()
            ));
            assert!(!is_python_executable_name(
                PathBuf::from("hython.exe").as_path()
            ));
            assert!(!is_python_executable_name(
                PathBuf::from("bpy.exe").as_path()
            ));
        }
        #[cfg(unix)]
        {
            assert!(!is_python_executable_name(
                PathBuf::from("mayapy").as_path()
            ));
            assert!(!is_python_executable_name(
                PathBuf::from("hython").as_path()
            ));
            assert!(!is_python_executable_name(PathBuf::from("bpy").as_path()));
        }
    }

    #[test]
    fn test_is_pyenv_shims_dir() {
        // Standard pyenv location
        assert!(is_pyenv_shims_dir(
            PathBuf::from("/home/user/.pyenv/shims").as_path()
        ));

        // Custom pyenv location (issue #238)
        assert!(is_pyenv_shims_dir(
            PathBuf::from("/home/user/.pl/pyenv/shims").as_path()
        ));

        // Other custom locations
        assert!(is_pyenv_shims_dir(
            PathBuf::from("/opt/pyenv/shims").as_path()
        ));
        assert!(is_pyenv_shims_dir(
            PathBuf::from("/usr/local/pyenv/shims").as_path()
        ));

        // pyenv-win style (parent contains "pyenv")
        assert!(is_pyenv_shims_dir(
            PathBuf::from("/home/user/.pyenv/pyenv-win/shims").as_path()
        ));

        // Not pyenv shims (should return false)
        assert!(!is_pyenv_shims_dir(
            PathBuf::from("/home/user/.pyenv/versions/3.10.0/bin").as_path()
        ));
        assert!(!is_pyenv_shims_dir(PathBuf::from("/usr/bin").as_path()));
        assert!(!is_pyenv_shims_dir(
            PathBuf::from("/home/user/shims").as_path()
        )); // "shims" but parent is not pyenv
        assert!(!is_pyenv_shims_dir(
            PathBuf::from("/home/user/project/shims").as_path()
        ));
    }

    #[test]
    fn test_is_broken_symlink_regular_file() {
        // A regular file should not be detected as a broken symlink
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("pet_test_regular_file.txt");
        fs::write(&test_file, "test").unwrap();

        assert!(!is_broken_symlink(&test_file));

        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn test_is_broken_symlink_nonexistent() {
        // A non-existent path should not be detected as a broken symlink
        let nonexistent = PathBuf::from("/this/path/does/not/exist/python");
        assert!(!is_broken_symlink(&nonexistent));
    }

    #[test]
    #[cfg(unix)]
    fn test_is_broken_symlink_unix() {
        use std::os::unix::fs::symlink;

        let temp_dir = std::env::temp_dir();
        let target = temp_dir.join("pet_test_symlink_target_nonexistent");
        let link = temp_dir.join("pet_test_broken_symlink");

        // Clean up any previous test artifacts
        let _ = fs::remove_file(&link);
        let _ = fs::remove_file(&target);

        // Create a symlink to a non-existent target
        symlink(&target, &link).unwrap();

        // The symlink should be detected as broken
        assert!(is_broken_symlink(&link));

        // Clean up
        let _ = fs::remove_file(&link);
    }

    #[test]
    #[cfg(unix)]
    fn test_is_broken_symlink_valid_symlink() {
        use std::os::unix::fs::symlink;

        let temp_dir = std::env::temp_dir();
        let target = temp_dir.join("pet_test_symlink_target_exists");
        let link = temp_dir.join("pet_test_valid_symlink");

        // Clean up any previous test artifacts
        let _ = fs::remove_file(&link);
        let _ = fs::remove_file(&target);

        // Create the target file
        fs::write(&target, "test").unwrap();

        // Create a symlink to the existing target
        symlink(&target, &link).unwrap();

        // The symlink should NOT be detected as broken
        assert!(!is_broken_symlink(&link));

        // Clean up
        let _ = fs::remove_file(&link);
        let _ = fs::remove_file(&target);
    }

    #[test]
    fn test_find_executable_or_broken_not_found() {
        let temp_dir = std::env::temp_dir().join("pet_test_empty_env");
        let _ = fs::create_dir_all(&temp_dir);

        match find_executable_or_broken(&temp_dir) {
            ExecutableResult::NotFound => (),
            other => panic!("Expected NotFound, got {:?}", other),
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_find_executable_or_broken_found() {
        let temp_dir = std::env::temp_dir().join("pet_test_valid_env");
        #[cfg(windows)]
        let bin_dir = temp_dir.join("Scripts");
        #[cfg(unix)]
        let bin_dir = temp_dir.join("bin");

        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&bin_dir).unwrap();

        #[cfg(windows)]
        let python_exe = bin_dir.join("python.exe");
        #[cfg(unix)]
        let python_exe = bin_dir.join("python");

        fs::write(&python_exe, "fake python").unwrap();

        match find_executable_or_broken(&temp_dir) {
            ExecutableResult::Found(path) => assert_eq!(path, python_exe),
            other => panic!("Expected Found, got {:?}", other),
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    #[cfg(unix)]
    fn test_find_executable_or_broken_broken_symlink() {
        use std::os::unix::fs::symlink;

        let temp_dir = std::env::temp_dir().join("pet_test_broken_env");
        let bin_dir = temp_dir.join("bin");

        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&bin_dir).unwrap();

        let python_exe = bin_dir.join("python");
        let nonexistent_target = PathBuf::from("/nonexistent/python3.10");

        // Create a broken symlink
        symlink(&nonexistent_target, &python_exe).unwrap();

        match find_executable_or_broken(&temp_dir) {
            ExecutableResult::Broken(path) => assert_eq!(path, python_exe),
            other => panic!("Expected Broken, got {:?}", other),
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
