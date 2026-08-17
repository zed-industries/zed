// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use pet_core::os_environment::Environment;
use std::path::PathBuf;

#[derive(Debug, Clone)]
// NOTE: Do not implement Default trait, as we do not want to ever forget to set the values.
// Lets be explicit, this way we never miss a value (in Windows or Unix).
pub struct EnvVariables {
    #[allow(dead_code)]
    pub home: Option<PathBuf>,
    #[allow(dead_code)]
    pub root: Option<PathBuf>,
    #[allow(dead_code)]
    pub path: Option<String>,
    pub homebrew_prefix: Option<String>,
    #[allow(dead_code)]
    pub known_global_search_locations: Vec<PathBuf>,
}

impl EnvVariables {
    pub fn from(env: &dyn Environment) -> Self {
        EnvVariables {
            home: env.get_user_home(),
            root: env.get_root(),
            path: env.get_env_var("PATH".to_string()),
            homebrew_prefix: env.get_env_var("HOMEBREW_PREFIX".to_string()),
            known_global_search_locations: env.get_know_global_search_locations(),
        }
    }
}
