// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use log::trace;
use pet_core::manager::{EnvManager, EnvManagerType};
use std::{env, path::PathBuf};

use crate::env_variables::EnvVariables;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PipenvManager {
    pub executable: PathBuf,
}

impl PipenvManager {
    pub fn find(executable: Option<PathBuf>, env_variables: &EnvVariables) -> Option<Self> {
        // If an explicit executable path is provided, check if it exists
        if let Some(executable) = executable {
            if executable.is_file() {
                return Some(PipenvManager { executable });
            }
        }

        // Search in common installation locations
        if let Some(home) = &env_variables.home {
            let mut search_paths = vec![
                // pip install --user pipenv on Linux/macOS
                home.join(".local").join("bin").join("pipenv"),
                // pipx install pipenv
                home.join(".local")
                    .join("pipx")
                    .join("venvs")
                    .join("pipenv")
                    .join("bin")
                    .join("pipenv"),
            ];

            if std::env::consts::OS == "windows" {
                // pip install --user pipenv on Windows
                search_paths.push(
                    home.join("AppData")
                        .join("Roaming")
                        .join("Python")
                        .join("Scripts")
                        .join("pipenv.exe"),
                );
                // Another common Windows location
                search_paths.push(
                    home.join("AppData")
                        .join("Local")
                        .join("Programs")
                        .join("Python")
                        .join("Scripts")
                        .join("pipenv.exe"),
                );
                // pipx on Windows
                search_paths.push(
                    home.join(".local")
                        .join("pipx")
                        .join("venvs")
                        .join("pipenv")
                        .join("Scripts")
                        .join("pipenv.exe"),
                );
            }

            for executable in search_paths {
                if executable.is_file() {
                    return Some(PipenvManager { executable });
                }
            }

            // Look for pipenv in current PATH
            if let Some(env_path) = &env_variables.path {
                for each in env::split_paths(env_path) {
                    let executable = each.join("pipenv");
                    if executable.is_file() {
                        return Some(PipenvManager { executable });
                    }
                    if std::env::consts::OS == "windows" {
                        let executable = each.join("pipenv.exe");
                        if executable.is_file() {
                            return Some(PipenvManager { executable });
                        }
                    }
                }
            }
        }

        trace!("Pipenv exe not found");
        None
    }

    pub fn to_manager(&self) -> EnvManager {
        EnvManager {
            executable: self.executable.clone(),
            version: None,
            tool: EnvManagerType::Pipenv,
        }
    }
}
