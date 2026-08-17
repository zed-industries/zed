// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};

/// Telemetry sent when
/// 1. We are able to spawn poetry
/// 2. We have found some new envs after spawning poetry
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Copy)]
pub struct MissingPoetryEnvironments {
    /// Total number of missing envs.
    pub missing: u16,
    /// Total number of missing envs, where the envs are created in the virtualenvs_path directory.
    pub missing_in_path: u16,
    /// Whether the user provided a executable.
    pub user_provided_poetry_exe: Option<bool>,
    /// Whether we managed to find the poetry exe or not.
    pub poetry_exe_not_found: Option<bool>,
    /// Whether we failed to find the global config file.
    pub global_config_not_found: Option<bool>,
    /// Whether the cache-dir returned by Poetry exe was not found by us
    /// This indicated the fact that we are unable to parse the poetry config file or something else.
    pub cache_dir_not_found: Option<bool>,
    /// Whether the cache-dir we found is different from what is returned by Poetry exe
    pub cache_dir_is_different: Option<bool>,
    /// Whether the virtualenvs path returned by Poetry exe was not found by us
    /// This indicated the fact that we are unable to parse the poetry config file or something else.
    pub virtualenvs_path_not_found: Option<bool>,
    /// Whether the virtualenvs_path we found is different from what is returned by Poetry exe
    pub virtualenvs_path_is_different: Option<bool>,
    /// Whether the virtualenvs.in-project setting value is differnt from what is returned by Poetry exe
    pub in_project_is_different: Option<bool>,
}
