// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::env_variables::EnvVariables;
use pet_fs::path::norm_case;
use std::path::PathBuf;

#[cfg(windows)]
fn get_default_virtualenvwrapper_path(env_vars: &EnvVariables) -> Option<PathBuf> {
    // In Windows, the default path for WORKON_HOME is %USERPROFILE%\Envs.
    // If 'Envs' is not available we should default to '.virtualenvs'. Since that
    // is also valid for windows.

    if let Some(user_home) = &env_vars.home {
        let home = user_home.join("Envs");
        if home.exists() {
            return Some(norm_case(home));
        }
        let home = user_home.join(".virtualenvs");
        if home.exists() {
            return Some(norm_case(home));
        }
        let home = user_home.join("virtualenvs");
        if home.exists() {
            return Some(norm_case(home));
        }
    }
    None
}

#[cfg(unix)]
fn get_default_virtualenvwrapper_path(env_vars: &EnvVariables) -> Option<PathBuf> {
    if let Some(home) = &env_vars.home {
        let home = home.join(".virtualenvs");
        if home.exists() {
            return Some(norm_case(&home));
        }
    }
    None
}

pub fn get_work_on_home_path(environment: &EnvVariables) -> Option<PathBuf> {
    // The WORKON_HOME variable contains the path to the root directory of all virtualenvwrapper environments.
    // If the interpreter path belongs to one of them then it is a virtualenvwrapper type of environment.
    if let Some(work_on_home) = &environment.workon_home {
        let work_on_home = norm_case(PathBuf::from(work_on_home));
        if work_on_home.exists() {
            return Some(work_on_home);
        }
    }
    get_default_virtualenvwrapper_path(environment)
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
            "pet-virtualenvwrapper-{name}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).unwrap();
        directory
    }

    #[test]
    fn workon_home_path_prefers_existing_workon_home_env_var() {
        let workon_home = create_test_dir("workon-home");
        let env_variables = EnvVariables {
            home: None,
            workon_home: Some(workon_home.to_string_lossy().to_string()),
        };

        assert_eq!(
            get_work_on_home_path(&env_variables),
            Some(norm_case(&workon_home))
        );

        fs::remove_dir_all(workon_home).unwrap();
    }

    #[test]
    fn workon_home_path_falls_back_to_default_home_location() {
        let user_home = create_test_dir("home");

        #[cfg(windows)]
        let default_home = user_home.join("Envs");
        #[cfg(unix)]
        let default_home = user_home.join(".virtualenvs");

        fs::create_dir_all(&default_home).unwrap();
        let env_variables = EnvVariables {
            home: Some(user_home.clone()),
            workon_home: None,
        };

        assert_eq!(
            get_work_on_home_path(&env_variables),
            Some(norm_case(default_home))
        );

        fs::remove_dir_all(user_home).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn workon_home_path_falls_back_to_dot_virtualenvs_on_windows() {
        let user_home = create_test_dir("windows-home");
        let default_home = user_home.join(".virtualenvs");
        fs::create_dir_all(&default_home).unwrap();
        let env_variables = EnvVariables {
            home: Some(user_home.clone()),
            workon_home: None,
        };

        assert_eq!(
            get_work_on_home_path(&env_variables),
            Some(norm_case(default_home))
        );

        fs::remove_dir_all(user_home).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn workon_home_path_supports_legacy_virtualenvs_without_dot_on_windows() {
        let user_home = create_test_dir("windows-home-legacy");
        let default_home = user_home.join("virtualenvs");
        fs::create_dir_all(&default_home).unwrap();
        let env_variables = EnvVariables {
            home: Some(user_home.clone()),
            workon_home: None,
        };

        assert_eq!(
            get_work_on_home_path(&env_variables),
            Some(norm_case(default_home))
        );

        fs::remove_dir_all(user_home).unwrap();
    }

    #[test]
    fn workon_home_path_returns_none_when_no_candidate_exists() {
        let workon_home = create_test_dir("missing-workon-home");
        let env_variables = EnvVariables {
            home: None,
            workon_home: Some(workon_home.join("missing").to_string_lossy().to_string()),
        };

        assert_eq!(get_work_on_home_path(&env_variables), None);

        fs::remove_dir_all(workon_home).unwrap();
    }
}
