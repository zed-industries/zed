// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};

/// Telemetry sent when
/// 1. We are able to spawn conda
/// 2. We have found some new envs after spawning conda
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Copy)]
pub struct MissingCondaEnvironments {
    /// Total number of missing conda envs.
    pub missing: u16,
    /// Total number of env_dirs not found even after parsing the conda_rc files.
    /// This will tell us that we are either unable to parse some of the conda_rc files or there are other
    /// env_dirs that we are not able to find.
    pub env_dirs_not_found: Option<u16>,
    /// Whether the user provided a conda executable.
    pub user_provided_conda_exe: Option<bool>,
    /// Whether the root prefix returned by conda was not found by us.
    pub root_prefix_not_found: Option<bool>,
    /// Whether the conda prefix returned by conda was not found by us.
    pub conda_prefix_not_found: Option<bool>,
    /// Whether we found a conda manager or not.
    pub conda_manager_not_found: Option<bool>,
    /// Whether we failed to find the system rc path.
    pub sys_rc_not_found: Option<bool>,
    /// Whether we failed to find the user rc path.
    pub user_rc_not_found: Option<bool>,
    /// Number of config files (excluding sys and user rc) that were not found.
    pub other_rc_not_found: Option<u16>,
    /// Number of conda envs that were not found by us, and the envs belong to env_dirs in the sys config rc.
    pub missing_env_dirs_from_sys_rc: Option<u16>,
    /// Number of conda envs that were not found by us, and the envs belong to env_dirs in the user config rc.
    pub missing_env_dirs_from_user_rc: Option<u16>,
    /// Number of conda envs that were not found by us, and the envs belong to env_dirs in the other config rc.
    pub missing_env_dirs_from_other_rc: Option<u16>,
    /// Number of conda envs that were not found by us, and the envs belong to env_dirs in the sys config rc.
    pub missing_from_sys_rc_env_dirs: Option<u16>,
    /// Number of conda envs that were not found by us, and the envs belong to env_dirs in the user config rc.
    pub missing_from_user_rc_env_dirs: Option<u16>,
    /// Number of conda envs that were not found by us, and the envs belong to env_dirs in the other config rc.
    pub missing_from_other_rc_env_dirs: Option<u16>,
}

impl std::fmt::Display for MissingCondaEnvironments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Missing Conda Environments ({}): ", self.missing).unwrap_or_default();
        if self.user_provided_conda_exe.unwrap_or_default() {
            writeln!(f, "   User Provided Conda Exe").unwrap_or_default();
        }
        if self.root_prefix_not_found.unwrap_or_default() {
            writeln!(f, "   Root Prefix not found").unwrap_or_default();
        }
        if self.conda_prefix_not_found.unwrap_or_default() {
            writeln!(f, "   Conda Prefix not found").unwrap_or_default();
        }
        if self.conda_manager_not_found.unwrap_or_default() {
            writeln!(f, "   Conda Manager not found").unwrap_or_default();
        }
        if self.sys_rc_not_found.unwrap_or_default() {
            writeln!(f, "   Sys conda_rc not found").unwrap_or_default();
        }
        if self.user_rc_not_found.unwrap_or_default() {
            writeln!(f, "   User conda_rc not found").unwrap_or_default();
        }
        if self.other_rc_not_found.unwrap_or_default() > 0 {
            writeln!(
                f,
                "   Other conda_rc not found ({})",
                self.other_rc_not_found.unwrap_or_default()
            )
            .unwrap_or_default();
        }
        if self.missing_env_dirs_from_sys_rc.unwrap_or_default() > 0 {
            writeln!(
                f,
                "   Missing env_dirs from sys conda_rc ({})",
                self.missing_env_dirs_from_sys_rc.unwrap_or_default()
            )
            .unwrap_or_default();
        }
        if self.missing_env_dirs_from_user_rc.unwrap_or_default() > 0 {
            writeln!(
                f,
                "   Missing env_dirs from user conda_rc ({})",
                self.missing_env_dirs_from_user_rc.unwrap_or_default()
            )
            .unwrap_or_default();
        }
        if self.missing_env_dirs_from_other_rc.unwrap_or_default() > 0 {
            writeln!(
                f,
                "   Missing env_dirs from other conda_rc ({})",
                self.missing_env_dirs_from_other_rc.unwrap_or_default()
            )
            .unwrap_or_default();
        }
        if self.missing_from_sys_rc_env_dirs.unwrap_or_default() > 0 {
            writeln!(
                f,
                "   Missing envs from env_dirs in sys conda_rc ({})",
                self.missing_from_sys_rc_env_dirs.unwrap_or_default()
            )
            .unwrap_or_default();
        }
        if self.missing_from_user_rc_env_dirs.unwrap_or_default() > 0 {
            writeln!(
                f,
                "   Missing envs from env_dirs in user conda_rc ({})",
                self.missing_env_dirs_from_user_rc.unwrap_or_default()
            )
            .unwrap_or_default();
        }
        if self.missing_from_other_rc_env_dirs.unwrap_or_default() > 0 {
            writeln!(
                f,
                "   Missing envs from env_dirs in other conda_rc ({})",
                self.missing_from_other_rc_env_dirs.unwrap_or_default()
            )
            .unwrap_or_default();
        }
        Ok(())
    }
}
