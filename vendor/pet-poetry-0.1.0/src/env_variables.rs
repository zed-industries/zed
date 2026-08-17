// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_core::os_environment::Environment;
use std::path::PathBuf;

#[derive(Debug, Clone)]
// NOTE: Do not implement Default trait, as we do not want to ever forget to set the values.
// Lets be explicit, this way we never miss a value (in Windows or Unix).
pub struct EnvVariables {
    pub home: Option<PathBuf>,
    /// Only used in tests, None in production.
    pub root: Option<PathBuf>,
    /// Maps to env var `APPDATA`
    pub app_data: Option<PathBuf>,
    /// Maps to env var `POETRY_VIRTUALENVS_PATH`
    pub poetry_virtualenvs_path: Option<PathBuf>,
    /// Maps to env var `POETRY_HOME`
    pub poetry_home: Option<PathBuf>,
    /// Maps to env var `POETRY_CONFIG_DIR`
    pub poetry_config_dir: Option<PathBuf>,
    /// Maps to env var `POETRY_CACHE_DIR`
    pub poetry_cache_dir: Option<PathBuf>,
    /// Maps to env var `POETRY_VIRTUALENVS_IN_PROJECT`
    pub poetry_virtualenvs_in_project: Option<bool>,
    /// Maps to env var `PATH`
    pub path: Option<String>,
}

impl EnvVariables {
    pub fn from(env: &dyn Environment) -> Self {
        let mut poetry_home = None;
        let home = env.get_user_home();
        if let (Some(home), Some(poetry_home_value)) =
            (&home, &env.get_env_var("POETRY_HOME".to_string()))
        {
            if poetry_home_value.starts_with('~') {
                poetry_home = Some(PathBuf::from(
                    poetry_home_value.replace('~', home.to_str().unwrap()),
                ));
            } else {
                poetry_home = Some(PathBuf::from(poetry_home_value));
            }
        }

        EnvVariables {
            home,
            path: env.get_env_var("PATH".to_string()),
            root: env.get_root(),
            app_data: env.get_env_var("APPDATA".to_string()).map(PathBuf::from),
            poetry_virtualenvs_path: env
                .get_env_var("POETRY_VIRTUALENVS_PATH".to_string())
                .map(PathBuf::from),
            poetry_cache_dir: env
                .get_env_var("POETRY_CACHE_DIR".to_string())
                .map(PathBuf::from),
            poetry_config_dir: env
                .get_env_var("POETRY_CONFIG_DIR".to_string())
                .map(PathBuf::from),
            poetry_virtualenvs_in_project: env
                .get_env_var("POETRY_VIRTUALENVS_IN_PROJECT".to_string())
                .map(|v| v == "1" || v.to_lowercase() == "true"),
            poetry_home,
        }
    }
}
