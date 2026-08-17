// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RefreshProgressPhase {
    Locators,
    Path,
    GlobalVirtualEnvs,
    Workspaces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RefreshProgressStatus {
    Started,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshProgress {
    pub refresh_id: u64,
    pub phase: RefreshProgressPhase,
    pub status: RefreshProgressStatus,
    pub elapsed_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_elapsed_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator_elapsed_ms: Option<u128>,
}
