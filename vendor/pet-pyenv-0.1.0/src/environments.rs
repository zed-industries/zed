// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use lazy_static::lazy_static;
use pet_core::{
    arch::Architecture,
    manager::EnvManager,
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind},
};
use pet_python_utils::executable::find_executables;
use pet_python_utils::version;
use regex::Regex;
use std::path::Path;

lazy_static! {
    // Stable Versions = like 3.10.10
    static ref PURE_PYTHON_VERSION: Regex = Regex::new(r"^(\d+\.\d+\.\d+)$")
        .expect("error parsing Version regex for Python Version in pyenv");
    // Dev Versions = like 3.10-dev
    static ref DEV_PYTHON_VERSION: Regex = Regex::new(r"^(\d+\.\d+-.*)$")
        .expect("error parsing Version regex for Dev Python Version in pyenv");
    // Alpha, rc Versions = like 3.10.0a3
    static ref BETA_PYTHON_VERSION: Regex = Regex::new(r"^(\d+\.\d+.\d+\w\d+)")
        .expect("error parsing Version regex for Alpha Python Version in pyenv");
    // win32 versions, rc Versions = like 3.11.0a-win32
    static ref WIN32_PYTHON_VERSION: Regex = Regex::new(r"^(\d+\.\d+.\d+\w\d+)-win32")
        .expect("error parsing Version regex for Win32 Python Version in pyenv");
}

pub fn get_generic_python_environment(
    executable: &Path,
    path: &Path,
    manager: &Option<EnvManager>,
) -> Option<PythonEnvironment> {
    let file_name = path.file_name()?.to_string_lossy().to_string();
    // If we can get the version from the header files, thats more accurate.
    let version = version::from_header_files(path).or_else(|| get_version(&file_name));

    let arch = if file_name.ends_with("-win32") {
        Some(Architecture::X86)
    } else {
        None
    };

    Some(
        PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::Pyenv))
            .executable(Some(executable.to_path_buf()))
            .version(version)
            .prefix(Some(path.to_path_buf()))
            .manager(manager.clone())
            .arch(arch)
            .symlinks(Some(find_executables(path)))
            .build(),
    )
}

pub fn get_virtual_env_environment(
    executable: &Path,
    path: &Path,
    manager: &Option<EnvManager>,
) -> Option<PythonEnvironment> {
    let version = version::from_pyvenv_cfg(path)?;
    Some(
        PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::PyenvVirtualEnv))
            .executable(Some(executable.to_path_buf()))
            .version(Some(version))
            .prefix(Some(path.to_path_buf()))
            .manager(manager.clone())
            .symlinks(Some(find_executables(path)))
            .build(),
    )
}

fn get_version(folder_name: &str) -> Option<String> {
    // Stable Versions = like 3.10.10
    match PURE_PYTHON_VERSION.captures(folder_name) {
        Some(captures) => captures.get(1).map(|version| version.as_str().to_string()),
        None => {
            // Dev Versions = like 3.10-dev
            match DEV_PYTHON_VERSION.captures(folder_name) {
                Some(captures) => captures.get(1).map(|version| version.as_str().to_string()),
                None => {
                    // Alpha, rc Versions = like 3.10.0a3
                    match BETA_PYTHON_VERSION.captures(folder_name) {
                        Some(captures) => {
                            captures.get(1).map(|version| version.as_str().to_string())
                        }
                        None => {
                            // win32 versions, rc Versions = like 3.11.0a-win32
                            match WIN32_PYTHON_VERSION.captures(folder_name) {
                                Some(captures) => {
                                    captures.get(1).map(|version| version.as_str().to_string())
                                }
                                None => None,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    // get_version tests
    #[test]
    fn get_version_parses_stable_version() {
        assert_eq!(get_version("3.10.10"), Some("3.10.10".to_string()));
        assert_eq!(get_version("3.12.0"), Some("3.12.0".to_string()));
        assert_eq!(get_version("2.7.18"), Some("2.7.18".to_string()));
    }

    #[test]
    fn get_version_parses_dev_version() {
        assert_eq!(get_version("3.10-dev"), Some("3.10-dev".to_string()));
        assert_eq!(get_version("3.13-dev"), Some("3.13-dev".to_string()));
    }

    #[test]
    fn get_version_parses_alpha_rc_version() {
        assert_eq!(get_version("3.10.0a3"), Some("3.10.0a3".to_string()));
        assert_eq!(get_version("3.12.0b1"), Some("3.12.0b1".to_string()));
    }

    #[test]
    fn get_version_returns_none_for_multi_letter_prerelease() {
        // Known limitation: BETA_PYTHON_VERSION regex uses \w (single char) so multi-letter
        // pre-release tags like "rc" are not captured. Real pyenv installs can have rc versions
        // (e.g. 3.13.0rc1), but version detection falls back to header files in that case.
        assert_eq!(get_version("3.11.0rc2"), None);
    }

    #[test]
    fn get_version_parses_win32_version() {
        assert_eq!(get_version("3.11.0a4-win32"), Some("3.11.0a4".to_string()));
    }

    #[test]
    fn get_version_returns_none_for_non_version_strings() {
        assert_eq!(get_version("mambaforge-4.10.1-4"), None);
        assert_eq!(get_version("pypy3.9-7.3.15"), None);
        assert_eq!(get_version("my-virtual-env"), None);
        assert_eq!(get_version(""), None);
    }

    #[test]
    fn get_version_returns_none_for_partial_version() {
        assert_eq!(get_version("3.10"), None);
    }

    // get_generic_python_environment tests
    #[test]
    fn get_generic_python_environment_with_stable_version_folder() {
        let root = tempdir().unwrap();
        let env_path = root.path().join("3.12.0");
        let bin_dir = if cfg!(windows) {
            env_path.join("Scripts")
        } else {
            env_path.join("bin")
        };
        fs::create_dir_all(&bin_dir).unwrap();
        let exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };
        fs::write(&exe, b"").unwrap();

        let result = get_generic_python_environment(&exe, &env_path, &None).unwrap();

        assert_eq!(result.kind, Some(PythonEnvironmentKind::Pyenv));
        assert_eq!(
            result.executable.as_ref().unwrap().file_name(),
            exe.file_name()
        );
        assert_eq!(result.version, Some("3.12.0".to_string()));
        assert_eq!(
            result.prefix.as_ref().unwrap().file_name(),
            env_path.file_name()
        );
        assert!(result.manager.is_none());
    }

    #[test]
    fn get_generic_python_environment_with_win32_folder_sets_x86_arch() {
        let root = tempdir().unwrap();
        let env_path = root.path().join("3.11.0a4-win32");
        let bin_dir = if cfg!(windows) {
            env_path.join("Scripts")
        } else {
            env_path.join("bin")
        };
        fs::create_dir_all(&bin_dir).unwrap();
        let exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };
        fs::write(&exe, b"").unwrap();

        let result = get_generic_python_environment(&exe, &env_path, &None).unwrap();

        assert_eq!(result.arch, Some(Architecture::X86));
    }

    #[test]
    fn get_generic_python_environment_with_non_win32_folder_has_no_arch() {
        let root = tempdir().unwrap();
        let env_path = root.path().join("3.12.0");
        let bin_dir = if cfg!(windows) {
            env_path.join("Scripts")
        } else {
            env_path.join("bin")
        };
        fs::create_dir_all(&bin_dir).unwrap();
        let exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };
        fs::write(&exe, b"").unwrap();

        let result = get_generic_python_environment(&exe, &env_path, &None).unwrap();

        assert!(result.arch.is_none());
    }

    #[test]
    fn get_generic_python_environment_includes_manager_when_provided() {
        let root = tempdir().unwrap();
        let env_path = root.path().join("3.12.0");
        let bin_dir = if cfg!(windows) {
            env_path.join("Scripts")
        } else {
            env_path.join("bin")
        };
        fs::create_dir_all(&bin_dir).unwrap();
        let exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };
        fs::write(&exe, b"").unwrap();

        let mgr = EnvManager::new(
            PathBuf::from("/usr/bin/pyenv"),
            pet_core::manager::EnvManagerType::Pyenv,
            Some("2.4.0".to_string()),
        );
        let result = get_generic_python_environment(&exe, &env_path, &Some(mgr.clone())).unwrap();

        assert_eq!(result.manager, Some(mgr));
    }

    #[test]
    fn get_generic_python_environment_with_unrecognized_folder_name() {
        let root = tempdir().unwrap();
        let env_path = root.path().join("mambaforge-4.10.1-4");
        let bin_dir = if cfg!(windows) {
            env_path.join("Scripts")
        } else {
            env_path.join("bin")
        };
        fs::create_dir_all(&bin_dir).unwrap();
        let exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };
        fs::write(&exe, b"").unwrap();

        let result = get_generic_python_environment(&exe, &env_path, &None).unwrap();

        assert_eq!(result.kind, Some(PythonEnvironmentKind::Pyenv));
        // No version extractable from folder name and no header files
        assert!(result.version.is_none());
    }

    // get_virtual_env_environment tests
    #[test]
    fn get_virtual_env_returns_none_without_pyvenv_cfg() {
        let root = tempdir().unwrap();
        let env_path = root.path().join("my-venv");
        let bin_dir = if cfg!(windows) {
            env_path.join("Scripts")
        } else {
            env_path.join("bin")
        };
        fs::create_dir_all(&bin_dir).unwrap();
        let exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };
        fs::write(&exe, b"").unwrap();

        let result = get_virtual_env_environment(&exe, &env_path, &None);

        assert!(result.is_none());
    }

    #[test]
    fn get_virtual_env_returns_env_with_pyvenv_cfg() {
        let root = tempdir().unwrap();
        let env_path = root.path().join("my-venv");
        let bin_dir = if cfg!(windows) {
            env_path.join("Scripts")
        } else {
            env_path.join("bin")
        };
        fs::create_dir_all(&bin_dir).unwrap();
        let exe = if cfg!(windows) {
            bin_dir.join("python.exe")
        } else {
            bin_dir.join("python")
        };
        fs::write(&exe, b"").unwrap();
        fs::write(
            env_path.join("pyvenv.cfg"),
            "version = 3.12.0\nhome = /usr/bin\n",
        )
        .unwrap();

        let result = get_virtual_env_environment(&exe, &env_path, &None).unwrap();

        assert_eq!(result.kind, Some(PythonEnvironmentKind::PyenvVirtualEnv));
        assert_eq!(result.version, Some("3.12.0".to_string()));
        assert_eq!(
            result.executable.as_ref().unwrap().file_name(),
            exe.file_name()
        );
        assert_eq!(
            result.prefix.as_ref().unwrap().file_name(),
            env_path.file_name()
        );
    }
}
