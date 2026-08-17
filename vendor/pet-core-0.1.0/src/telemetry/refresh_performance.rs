// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Telemetry with metrics for finding all environments as a result of refresh.
/// All durations are in milliseconds.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone)]
pub struct RefreshPerformance {
    /// Total time taken to find all envs.
    pub total: u128,
    /// Breakdown of Global VirtualEnvs, Path, Workspace and the locators.
    pub breakdown: BTreeMap<String, u128>,
    /// Breakdown of each individual locators such as conda, pyenv, etc.
    pub locators: BTreeMap<String, u128>,
}
