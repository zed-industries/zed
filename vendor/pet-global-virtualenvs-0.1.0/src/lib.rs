// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_conda::utils::is_conda_env;
use pet_fs::path::{expand_path, norm_case};
use std::{fs, path::PathBuf};

fn get_global_virtualenv_dirs(
    work_on_home_env_var: Option<String>,
    xdg_data_home: Option<String>,
    user_home: Option<PathBuf>,
) -> Vec<PathBuf> {
    let mut venv_dirs: Vec<PathBuf> = vec![];

    if let Some(work_on_home) = work_on_home_env_var {
        let work_on_home = norm_case(expand_path(PathBuf::from(work_on_home)));
        if work_on_home.exists() {
            venv_dirs.push(work_on_home);
        }
    }

    // Used by pipenv (https://github.com/pypa/pipenv/blob/main/pipenv/utils/shell.py#L184)
    if let Some(xdg_data_home) = xdg_data_home.map(|d| PathBuf::from(d).join("virtualenvs")) {
        if xdg_data_home.exists() {
            venv_dirs.push(xdg_data_home);
        }
    }
    if let Some(home) = user_home {
        for dir in [
            PathBuf::from("envs"),
            PathBuf::from(".direnv"),
            PathBuf::from(".venvs"), // Used by pipenv, https://pipenv.pypa.io/en/latest/virtualenv.html
            PathBuf::from(".virtualenvs"), // Used by pipenv (https://github.com/pypa/pipenv/blob/main/pipenv/utils/shell.py#L184), and also default location for virtualenvwrapper, https://virtualenvwrapper.readthedocs.io/en/latest/install.html#location-of-environments
            PathBuf::from(".local").join("share").join("virtualenvs"), // Used by pipenv (https://github.com/pypa/pipenv/blob/main/pipenv/utils/shell.py#L184)
        ] {
            let venv_dir = home.join(dir);
            if venv_dir.exists() {
                venv_dirs.push(venv_dir);
            }
        }
        if cfg!(target_os = "linux") {
            // https://virtualenvwrapper.readthedocs.io/en/latest/index.html
            // Default recommended location for virtualenvwrapper
            let envs = home.join("Envs");
            if envs.exists() {
                venv_dirs.push(envs);
            }
        }
    }

    venv_dirs
}

pub fn list_global_virtual_envs_paths(
    virtual_env_env_var: Option<String>,
    work_on_home_env_var: Option<String>,
    xdg_data_home: Option<String>,
    user_home: Option<PathBuf>,
) -> Vec<PathBuf> {
    let mut python_envs: Vec<PathBuf> = vec![];

    if let Some(virtual_env) = virtual_env_env_var {
        let virtual_env = norm_case(expand_path(PathBuf::from(virtual_env)));
        if virtual_env.exists() {
            python_envs.push(virtual_env);
        }
    }

    for root_dir in &get_global_virtualenv_dirs(work_on_home_env_var, xdg_data_home, user_home) {
        if let Ok(dirs) = fs::read_dir(root_dir) {
            python_envs.append(
                &mut dirs
                    .filter_map(Result::ok)
                    .map(|e| e.path())
                    .filter(|p| !is_conda_env(p))
                    .collect(),
            )
        }
    }

    python_envs.sort();
    python_envs.dedup();

    python_envs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn create_test_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "pet-global-virtualenvs-{name}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).unwrap();
        directory
    }

    #[test]
    fn global_virtualenv_dirs_include_existing_configured_and_default_locations() {
        let root = create_test_dir("dirs");
        let work_on_home = root.join("workon-home");
        let xdg_virtualenvs = root.join("xdg-data").join("virtualenvs");
        let local_virtualenvs = root.join(".local").join("share").join("virtualenvs");
        let missing_work_on_home = root.join("missing-workon-home");
        fs::create_dir_all(&work_on_home).unwrap();
        fs::create_dir_all(&xdg_virtualenvs).unwrap();
        fs::create_dir_all(root.join("envs")).unwrap();
        fs::create_dir_all(root.join(".direnv")).unwrap();
        fs::create_dir_all(root.join(".venvs")).unwrap();
        fs::create_dir_all(root.join(".virtualenvs")).unwrap();
        fs::create_dir_all(&local_virtualenvs).unwrap();

        #[cfg(target_os = "linux")]
        {
            fs::create_dir_all(root.join("Envs")).unwrap();
        }

        let mut dirs = get_global_virtualenv_dirs(
            Some(missing_work_on_home.to_string_lossy().to_string()),
            Some(root.join("xdg-data").to_string_lossy().to_string()),
            Some(root.clone()),
        );
        dirs.sort();

        #[cfg(not(target_os = "linux"))]
        let expected_dirs = vec![
            root.join(".direnv"),
            root.join(".local").join("share").join("virtualenvs"),
            root.join(".venvs"),
            root.join(".virtualenvs"),
            root.join("envs"),
            xdg_virtualenvs,
        ];

        #[cfg(target_os = "linux")]
        let mut expected_dirs = vec![
            root.join(".direnv"),
            root.join(".local").join("share").join("virtualenvs"),
            root.join(".venvs"),
            root.join(".virtualenvs"),
            root.join("envs"),
            xdg_virtualenvs,
            root.join("Envs"),
        ];

        #[cfg(target_os = "linux")]
        {
            expected_dirs.sort();
        }

        assert_eq!(dirs, expected_dirs);

        let dirs = get_global_virtualenv_dirs(
            Some(work_on_home.to_string_lossy().to_string()),
            None,
            None,
        );

        assert_eq!(dirs, vec![norm_case(work_on_home)]);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn global_virtualenv_paths_include_virtual_env_var_and_non_conda_children_only() {
        let root = create_test_dir("envs");
        let virtual_env = root.join("active-venv");
        let work_on_home = root.join("workon-home");
        let venv = work_on_home.join("plain-venv");
        let conda_env = work_on_home.join("conda-env");
        fs::create_dir_all(&virtual_env).unwrap();
        fs::create_dir_all(&venv).unwrap();
        fs::create_dir_all(conda_env.join("conda-meta")).unwrap();

        let mut python_envs = list_global_virtual_envs_paths(
            Some(virtual_env.to_string_lossy().to_string()),
            Some(work_on_home.to_string_lossy().to_string()),
            None,
            Some(root.clone()),
        );
        python_envs.sort();

        assert_eq!(python_envs, vec![norm_case(virtual_env), norm_case(venv)]);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn global_virtualenv_paths_are_deduplicated_and_ignore_missing_inputs() {
        let root = create_test_dir("dedupe-envs");
        let work_on_home = root.join("workon-home");
        let venv = work_on_home.join("plain-venv");
        fs::create_dir_all(&venv).unwrap();

        let python_envs = list_global_virtual_envs_paths(
            Some(venv.to_string_lossy().to_string()),
            Some(work_on_home.to_string_lossy().to_string()),
            Some(root.join("missing-xdg-data").to_string_lossy().to_string()),
            Some(root.join("missing-home")),
        );

        assert_eq!(python_envs, vec![norm_case(venv)]);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn global_virtualenv_paths_include_xdg_and_default_home_children() {
        let root = create_test_dir("xdg-and-home-envs");
        let xdg_data_home = root.join("xdg-data");
        let xdg_venv = xdg_data_home.join("virtualenvs").join("xdg-venv");
        let default_venv = root.join(".virtualenvs").join("default-venv");
        fs::create_dir_all(&xdg_venv).unwrap();
        fs::create_dir_all(&default_venv).unwrap();

        let python_envs = list_global_virtual_envs_paths(
            None,
            None,
            Some(xdg_data_home.to_string_lossy().to_string()),
            Some(root.clone()),
        );

        assert_eq!(python_envs, vec![default_venv, xdg_venv]);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn global_virtualenv_dirs_returns_empty_when_all_none() {
        let dirs = get_global_virtualenv_dirs(None, None, None);
        assert!(dirs.is_empty());
    }

    #[test]
    fn global_virtualenv_dirs_skips_nonexistent_work_on_home() {
        let root = create_test_dir("skip-work-on");
        let nonexistent = root.join("does-not-exist");

        let dirs =
            get_global_virtualenv_dirs(Some(nonexistent.to_string_lossy().to_string()), None, None);
        assert!(dirs.is_empty());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn global_virtualenv_dirs_skips_nonexistent_xdg_virtualenvs() {
        let root = create_test_dir("skip-xdg");
        let xdg = root.join("xdg-missing");

        let dirs = get_global_virtualenv_dirs(None, Some(xdg.to_string_lossy().to_string()), None);
        assert!(dirs.is_empty());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn list_global_virtual_envs_all_none_returns_empty() {
        let paths = list_global_virtual_envs_paths(None, None, None, None);
        assert!(paths.is_empty());
    }

    #[test]
    fn list_global_virtual_envs_virtual_env_var_nonexistent_is_skipped() {
        let root = create_test_dir("skip-virtual-env");
        let nonexistent = root.join("nonexistent-venv");

        let paths = list_global_virtual_envs_paths(
            Some(nonexistent.to_string_lossy().to_string()),
            None,
            None,
            None,
        );
        assert!(paths.is_empty());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn list_global_virtual_envs_skips_empty_dirs() {
        let root = create_test_dir("empty-dirs");
        let work_on_home = root.join("workon");
        fs::create_dir_all(&work_on_home).unwrap();

        // workon dir exists but has no children
        let paths = list_global_virtual_envs_paths(
            None,
            Some(work_on_home.to_string_lossy().to_string()),
            None,
            None,
        );
        assert!(paths.is_empty());

        fs::remove_dir_all(root).unwrap();
    }
}
