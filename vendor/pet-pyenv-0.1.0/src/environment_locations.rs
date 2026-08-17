// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::env_variables::EnvVariables;
use pet_fs::path::norm_case;
use std::path::PathBuf;

#[cfg(windows)]
pub fn get_home_pyenv_dir(env_vars: &EnvVariables) -> Option<PathBuf> {
    let home = env_vars.home.clone()?;
    Some(norm_case(home.join(".pyenv").join("pyenv-win")))
}

#[cfg(unix)]
pub fn get_home_pyenv_dir(env_vars: &EnvVariables) -> Option<PathBuf> {
    let home = env_vars.home.clone()?;
    Some(norm_case(home.join(".pyenv")))
}

pub fn get_binary_from_known_paths(env_vars: &EnvVariables) -> Option<PathBuf> {
    for known_path in &env_vars.known_global_search_locations {
        let exe = if cfg!(windows) {
            // pyenv-win provides pyenv.bat, not pyenv.exe
            known_path.join("pyenv.bat")
        } else {
            known_path.join("pyenv")
        };
        if exe.is_file() {
            return Some(norm_case(exe));
        }
    }
    None
}

pub fn get_pyenv_dir(env_vars: &EnvVariables) -> Option<PathBuf> {
    // Check if the pyenv environment variables exist: PYENV on Windows, PYENV_ROOT on Unix.
    // They contain the path to pyenv's installation folder.
    // If they don't exist, use the default path: ~/.pyenv/pyenv-win on Windows, ~/.pyenv on Unix.
    // If the interpreter path starts with the path to the pyenv folder, then it is a pyenv environment.
    // See https://github.com/pyenv/pyenv#locating-the-python-installation for general usage,
    // And https://github.com/pyenv-win/pyenv-win for Windows specifics.

    match &env_vars.pyenv_root {
        Some(dir) => Some(PathBuf::from(dir)),
        None => env_vars.pyenv.as_ref().map(PathBuf::from),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn make_env_vars(
        home: Option<PathBuf>,
        pyenv_root: Option<String>,
        pyenv: Option<String>,
        known_paths: Vec<PathBuf>,
    ) -> EnvVariables {
        EnvVariables {
            home,
            root: None,
            path: None,
            pyenv_root,
            pyenv,
            known_global_search_locations: known_paths,
        }
    }

    // get_pyenv_dir tests
    #[test]
    fn get_pyenv_dir_prefers_pyenv_root_over_pyenv() {
        let env = make_env_vars(
            None,
            Some("/custom/pyenv-root".to_string()),
            Some("/other/pyenv".to_string()),
            vec![],
        );
        assert_eq!(
            get_pyenv_dir(&env),
            Some(PathBuf::from("/custom/pyenv-root"))
        );
    }

    #[test]
    fn get_pyenv_dir_falls_back_to_pyenv_env_var() {
        let env = make_env_vars(None, None, Some("/fallback/pyenv".to_string()), vec![]);
        assert_eq!(get_pyenv_dir(&env), Some(PathBuf::from("/fallback/pyenv")));
    }

    #[test]
    fn get_pyenv_dir_returns_none_when_no_env_vars() {
        let env = make_env_vars(None, None, None, vec![]);
        assert_eq!(get_pyenv_dir(&env), None);
    }

    // get_home_pyenv_dir tests
    #[test]
    fn get_home_pyenv_dir_returns_none_without_home() {
        let env = make_env_vars(None, None, None, vec![]);
        assert_eq!(get_home_pyenv_dir(&env), None);
    }

    #[test]
    fn get_home_pyenv_dir_returns_expected_path_with_home() {
        let home = tempdir().unwrap();
        let env = make_env_vars(Some(home.path().to_path_buf()), None, None, vec![]);
        let result = get_home_pyenv_dir(&env).unwrap();
        let path_str = result.to_string_lossy();
        if cfg!(windows) {
            assert!(
                path_str.contains(".pyenv"),
                "Expected .pyenv in path: {}",
                path_str
            );
            assert!(
                path_str.contains("pyenv-win"),
                "Expected pyenv-win in path: {}",
                path_str
            );
        } else {
            assert!(result.ends_with(".pyenv"));
        }
    }

    // get_binary_from_known_paths tests
    #[test]
    fn get_binary_from_known_paths_finds_pyenv_binary() {
        let dir = tempdir().unwrap();
        let bin_name = if cfg!(windows) { "pyenv.bat" } else { "pyenv" };
        let exe = dir.path().join(bin_name);
        fs::write(&exe, b"").unwrap();

        let env = make_env_vars(None, None, None, vec![dir.path().to_path_buf()]);
        let result = get_binary_from_known_paths(&env);
        assert!(result.is_some());
    }

    #[test]
    fn get_binary_from_known_paths_returns_none_when_not_found() {
        let dir = tempdir().unwrap();
        let env = make_env_vars(None, None, None, vec![dir.path().to_path_buf()]);
        assert!(get_binary_from_known_paths(&env).is_none());
    }

    #[test]
    fn get_binary_from_known_paths_returns_none_for_empty_paths() {
        let env = make_env_vars(None, None, None, vec![]);
        assert!(get_binary_from_known_paths(&env).is_none());
    }
}
