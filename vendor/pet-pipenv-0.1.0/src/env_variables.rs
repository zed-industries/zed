// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use pet_core::os_environment::Environment;
use std::path::PathBuf;

#[derive(Debug, Clone)]
// NOTE: Do not implement Default trait, as we do not want to ever forget to set the values.
// Lets be explicit, this way we never miss a value (in Windows or Unix).
pub struct EnvVariables {
    #[allow(dead_code)]
    pub pipenv_max_depth: u16,
    pub pipenv_pipfile: String,
    /// User's home directory
    pub home: Option<PathBuf>,
    /// Maps to env var `WORKON_HOME` - custom directory for virtual environments
    pub workon_home: Option<PathBuf>,
    pub xdg_data_home: Option<String>,
    /// Maps to env var `PATH`
    pub path: Option<String>,
}

impl EnvVariables {
    pub fn from(env: &dyn Environment) -> Self {
        EnvVariables {
            pipenv_max_depth: env
                .get_env_var("PIPENV_MAX_DEPTH".to_string())
                .map(|s| s.parse::<u16>().ok().unwrap_or(3))
                .unwrap_or(3),
            pipenv_pipfile: env
                .get_env_var("PIPENV_PIPFILE".to_string())
                .unwrap_or("Pipfile".to_string()),
            home: env.get_user_home(),
            xdg_data_home: env.get_env_var("XDG_DATA_HOME".to_string()),
            workon_home: env
                .get_env_var("WORKON_HOME".to_string())
                .map(PathBuf::from),
            path: env.get_env_var("PATH".to_string()),
        }
    }
}
