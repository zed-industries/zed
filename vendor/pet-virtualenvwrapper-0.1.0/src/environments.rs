// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::{env_variables::EnvVariables, environment_locations::get_work_on_home_path};
use pet_core::env::PythonEnv;
use pet_fs::path::norm_case;
use pet_virtualenv::is_virtualenv;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn is_under_work_on_home(executable: &Path, work_on_home_dir: &Path) -> bool {
    if executable.starts_with(work_on_home_dir) {
        return true;
    }

    if let (Ok(executable), Ok(work_on_home_dir)) = (
        fs::canonicalize(executable),
        fs::canonicalize(work_on_home_dir),
    ) {
        return norm_case(executable).starts_with(norm_case(work_on_home_dir));
    }

    false
}

pub fn is_virtualenvwrapper(env: &PythonEnv, environment: &EnvVariables) -> bool {
    if env.prefix.is_none() {
        return false;
    }

    // For environment to be a virtualenvwrapper based it has to follow these two rules:
    // 1. It should be in a sub-directory under the WORKON_HOME
    // 2. It should be a valid virtualenv environment
    if let Some(work_on_home_dir) = get_work_on_home_path(environment) {
        if is_under_work_on_home(&env.executable, &work_on_home_dir) && is_virtualenv(env) {
            return true;
        }
    }

    false
}

pub fn get_project(env: &PythonEnv) -> Option<PathBuf> {
    let project_file = env.prefix.clone()?.join(".project");
    let contents = fs::read_to_string(project_file).ok()?;
    let project_folder = norm_case(PathBuf::from(contents.trim().to_string()));
    if fs::metadata(&project_folder).is_ok() {
        Some(norm_case(&project_folder))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::Path,
        sync::atomic::{AtomicUsize, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    #[cfg(windows)]
    use std::os::windows::fs::symlink_dir;

    static NEXT_TEST_DIR_ID: AtomicUsize = AtomicUsize::new(0);

    fn create_test_dir(name: &str) -> PathBuf {
        let id = NEXT_TEST_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "pet-virtualenvwrapper-{name}-{}-{timestamp}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).unwrap();
        directory
    }

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

    #[test]
    fn is_virtualenvwrapper_requires_prefix_inside_workon_home_and_valid_virtualenv() {
        let workon_home = create_test_dir("workon-home");
        let prefix = workon_home.join("wrapped-env");
        let executable = create_virtualenv(&prefix);
        let env = PythonEnv::new(executable, Some(prefix.clone()), None);
        let env_variables = EnvVariables {
            home: None,
            workon_home: Some(workon_home.to_string_lossy().to_string()),
        };

        assert!(is_virtualenvwrapper(&env, &env_variables));

        fs::remove_dir_all(workon_home).unwrap();
    }

    #[test]
    fn is_virtualenvwrapper_rejects_env_without_prefix_or_outside_workon_home() {
        let workon_home = create_test_dir("workon-home");
        let outside_prefix = create_test_dir("outside-env").join("wrapped-env");
        let executable = create_virtualenv(&outside_prefix);
        let env_variables = EnvVariables {
            home: None,
            workon_home: Some(workon_home.to_string_lossy().to_string()),
        };

        let no_prefix_env = PythonEnv::new(executable.clone(), None, None);
        let outside_env = PythonEnv::new(executable, Some(outside_prefix.clone()), None);

        assert!(!is_virtualenvwrapper(&no_prefix_env, &env_variables));
        assert!(!is_virtualenvwrapper(&outside_env, &env_variables));

        fs::remove_dir_all(workon_home).unwrap();
        fs::remove_dir_all(outside_prefix.parent().unwrap()).unwrap();
    }

    #[test]
    fn is_virtualenvwrapper_rejects_non_virtualenv_under_workon_home() {
        let workon_home = create_test_dir("workon-home");
        let prefix = workon_home.join("not-a-virtualenv");
        let scripts_dir = prefix.join(if cfg!(windows) { "Scripts" } else { "bin" });
        fs::create_dir_all(&scripts_dir).unwrap();
        let executable = scripts_dir.join(if cfg!(windows) {
            "python.exe"
        } else {
            "python"
        });
        fs::write(&executable, b"").unwrap();
        let env = PythonEnv::new(executable, Some(prefix), None);
        let env_variables = EnvVariables {
            home: None,
            workon_home: Some(workon_home.to_string_lossy().to_string()),
        };

        assert!(!is_virtualenvwrapper(&env, &env_variables));

        fs::remove_dir_all(workon_home).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn is_virtualenvwrapper_accepts_env_under_symlinked_workon_home() {
        let real_workon_home = create_test_dir("real-workon-home");
        let linked_parent = create_test_dir("linked-parent");
        let linked_workon_home = linked_parent.join("linked-workon-home");
        if let Err(error) = symlink_dir(&real_workon_home, &linked_workon_home) {
            eprintln!(
                "Skipping symlinked WORKON_HOME test because symlink creation failed: {error:?}"
            );
            fs::remove_dir_all(real_workon_home).unwrap();
            fs::remove_dir_all(linked_parent).unwrap();
            return;
        }

        let prefix = real_workon_home.join("wrapped-env");
        let executable = create_virtualenv(&prefix);
        let env = PythonEnv::new(executable, Some(prefix), None);
        let env_variables = EnvVariables {
            home: None,
            workon_home: Some(linked_workon_home.to_string_lossy().to_string()),
        };

        assert!(is_virtualenvwrapper(&env, &env_variables));

        fs::remove_dir_all(linked_workon_home).unwrap();
        fs::remove_dir_all(linked_parent).unwrap();
        fs::remove_dir_all(real_workon_home).unwrap();
    }

    #[test]
    fn get_project_reads_existing_project_path_from_project_file() {
        let project_root = fs::canonicalize(create_test_dir("project-root")).unwrap();
        let prefix = create_test_dir("wrapped-env");
        let executable = create_virtualenv(&prefix);
        fs::write(
            prefix.join(".project"),
            format!(" {} \n", project_root.display()),
        )
        .unwrap();
        let env = PythonEnv::new(executable, Some(prefix.clone()), None);

        assert_eq!(get_project(&env), Some(norm_case(project_root.clone())));

        fs::remove_dir_all(project_root).unwrap();
        fs::remove_dir_all(prefix).unwrap();
    }

    #[test]
    fn get_project_returns_none_for_missing_prefix_or_missing_project_path() {
        let prefix = create_test_dir("wrapped-env");
        let executable = create_virtualenv(&prefix);
        fs::write(
            prefix.join(".project"),
            prefix.join("missing").display().to_string(),
        )
        .unwrap();
        let missing_project_env = PythonEnv::new(executable.clone(), Some(prefix.clone()), None);
        let no_prefix_env = PythonEnv::new(executable, None, None);

        assert_eq!(get_project(&missing_project_env), None);
        assert_eq!(get_project(&no_prefix_env), None);

        fs::remove_dir_all(prefix).unwrap();
    }
}

// pub fn list_python_environments(path: &PathBuf) -> Option<Vec<PythonEnv>> {
//     let mut python_envs: Vec<PythonEnv> = vec![];
//     for venv_dir in fs::read_dir(path)
//         .ok()?
//         .filter_map(Result::ok)
//         .map(|e| e.path())
//     {
//         if fs::metadata(&venv_dir).is_err() {
//             continue;
//         }
//         if let Some(executable) = find_executable(&venv_dir) {
//             python_envs.push(PythonEnv::new(
//                 executable.clone(),
//                 Some(venv_dir.clone()),
//                 version::from_pyvenv_cfg(&venv_dir),
//             ));
//         }
//     }

//     Some(python_envs)
// }
