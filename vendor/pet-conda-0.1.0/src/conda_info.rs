// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use log::{error, trace, warn};
use pet_fs::path::resolve_symlink;
use pet_python_utils::executable::new_silent_command;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct CondaInfo {
    pub executable: PathBuf,
    pub envs: Vec<PathBuf>,
    pub conda_prefix: Option<PathBuf>,
    pub conda_version: String,
    pub envs_dirs: Vec<PathBuf>,
    pub config_files: Vec<PathBuf>,
    pub rc_path: Option<PathBuf>,
    pub sys_rc_path: Option<PathBuf>,
    pub user_rc_path: Option<PathBuf>,
    pub root_prefix: Option<PathBuf>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CondaInfoJson {
    pub envs: Option<Vec<PathBuf>>,
    pub conda_prefix: Option<PathBuf>,
    pub conda_version: Option<String>,
    pub envs_dirs: Option<Vec<PathBuf>>,
    /// This is an alias for envs_dirs
    pub envs_path: Option<Vec<PathBuf>>,
    pub config_files: Option<Vec<PathBuf>>,
    pub rc_path: Option<PathBuf>,
    pub user_rc_path: Option<PathBuf>,
    pub sys_rc_path: Option<PathBuf>,
    pub root_prefix: Option<PathBuf>,
}

impl CondaInfo {
    pub fn from(executable: Option<PathBuf>) -> Option<CondaInfo> {
        // let using_default = executable.is_none() || executable == Some("conda".into());
        // Possible we got a symlink to the conda exe, first try to resolve that.
        let executable = if cfg!(windows) {
            executable.clone().unwrap_or("conda".into())
        } else {
            let executable = executable.unwrap_or("conda".into());
            resolve_symlink(&executable).unwrap_or(executable)
        };

        let result = new_silent_command(&executable)
            .arg("info")
            .arg("--json")
            .output();
        trace!("Executing Conda: {:?} info --json -a", executable);
        match result {
            Ok(output) => {
                if output.status.success() {
                    let output = String::from_utf8_lossy(&output.stdout).to_string();
                    match serde_json::from_str::<CondaInfoJson>(output.trim()) {
                        Ok(info) => {
                            let envs_path = info
                                .envs_path
                                .unwrap_or_default()
                                .drain(..)
                                .collect::<Vec<PathBuf>>();
                            let mut envs_dirs = info
                                .envs_dirs
                                .unwrap_or_default()
                                .drain(..)
                                .collect::<Vec<PathBuf>>();
                            envs_dirs.extend(envs_path);
                            let info = CondaInfo {
                                executable: executable.clone(),
                                envs: info.envs.unwrap_or_default().drain(..).collect(),
                                conda_prefix: info.conda_prefix,
                                root_prefix: info.root_prefix,
                                rc_path: info.rc_path,
                                sys_rc_path: info.sys_rc_path,
                                user_rc_path: info.user_rc_path,
                                envs_dirs,
                                conda_version: info.conda_version.unwrap_or_default(),
                                config_files: info
                                    .config_files
                                    .unwrap_or_default()
                                    .drain(..)
                                    .collect(),
                            };
                            Some(info)
                        }
                        Err(err) => {
                            error!(
                                "Conda Execution for {:?} produced an output {:?} that could not be parsed as JSON",
                                executable, err,
                            );
                            None
                        }
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    // No point logging the message if conda is not installed or a custom conda exe wasn't provided.
                    if executable.to_string_lossy() != "conda" {
                        warn!(
                            "Failed to get conda info using  {:?} ({:?}) {}",
                            executable,
                            output.status.code().unwrap_or_default(),
                            stderr
                        );
                    }
                    None
                }
            }
            Err(err) => {
                // No point logging the message if conda is not installed or a custom conda exe wasn't provided.
                if executable.to_string_lossy() != "conda" {
                    warn!("Failed to execute conda info {:?}", err);
                }
                None
            }
        }
    }
}
