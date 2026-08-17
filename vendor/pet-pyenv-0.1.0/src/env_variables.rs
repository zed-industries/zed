// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_core::os_environment::Environment;
use std::path::PathBuf;

#[derive(Debug, Clone)]
// NOTE: Do not implement Default trait, as we do not want to ever forget to set the values.
// Lets be explicit, this way we never miss a value (in Windows or Unix).
pub struct EnvVariables {
    pub home: Option<PathBuf>,
    pub root: Option<PathBuf>,
    pub path: Option<String>,
    pub pyenv_root: Option<String>,
    pub pyenv: Option<String>,
    pub known_global_search_locations: Vec<PathBuf>,
}

impl EnvVariables {
    pub fn from(env: &dyn Environment) -> Self {
        EnvVariables {
            home: env.get_user_home(),
            root: env.get_root(),
            path: env.get_env_var("PATH".to_string()),
            pyenv_root: env.get_env_var("PYENV_ROOT".to_string()),
            pyenv: env.get_env_var("PYENV".to_string()),
            known_global_search_locations: env.get_know_global_search_locations(),
        }
    }
}
