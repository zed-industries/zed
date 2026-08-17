// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use lazy_static::lazy_static;
use log::{error, trace};
use pet_core::python_environment::PythonEnvironment;
use pet_python_utils::executable::new_silent_command;
use regex::Regex;
use std::{path::PathBuf, time::SystemTime};

use crate::{environment::create_poetry_env, manager::PoetryManager};

lazy_static! {
    static ref SANITIZE_NAME: Regex = Regex::new("[ $`!*@\"\\\r\n\t]")
        .expect("Error generating RegEx for poetry file path hash generator");
}

pub fn list_environments(
    executable: &PathBuf,
    workspace_dirs: &Vec<PathBuf>,
    manager: &PoetryManager,
) -> Vec<PythonEnvironment> {
    let mut envs = vec![];
    for workspace_dir in workspace_dirs {
        if let Some(workspace_envs) = get_environments(executable, workspace_dir) {
            for workspace_env in workspace_envs {
                if let Some(env) =
                    create_poetry_env(&workspace_env, workspace_dir.clone(), Some(manager.clone()))
                {
                    envs.push(env);
                }
            }
        }
    }
    envs
}

fn get_environments(executable: &PathBuf, workspace_dir: &PathBuf) -> Option<Vec<PathBuf>> {
    let start = SystemTime::now();
    let result = new_silent_command(executable)
        .arg("env")
        .arg("list")
        .arg("--full-path")
        .current_dir(workspace_dir)
        .output();
    trace!(
        "Executed Poetry ({}ms): {:?} env list --full-path for {:?}",
        start.elapsed().unwrap_or_default().as_millis(),
        executable,
        workspace_dir
    );
    match result {
        Ok(output) => {
            if output.status.success() {
                let output = String::from_utf8_lossy(&output.stdout).to_string();
                Some(
                    output
                        .lines()
                        .map(|line|
                        // Remove the '(Activated)` suffix from the line
                        line.trim_end_matches(" (Activated)").trim())
                        .filter(|line| !line.is_empty())
                        .map(|line|
                        // Remove the '(Activated)` suffix from the line
                        PathBuf::from(line.trim_end_matches(" (Activated)").trim()))
                        .collect::<Vec<PathBuf>>(),
                )
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                trace!(
                    "Failed to get Poetry Envs using exe {:?} ({:?}) {}",
                    executable,
                    output.status.code().unwrap_or_default(),
                    stderr
                );
                None
            }
        }
        Err(err) => {
            error!("Failed to execute Poetry env list {:?}", err);
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct PoetryConfig {
    pub cache_dir: Option<PathBuf>,
    pub virtualenvs_in_project: Option<bool>,
    pub virtualenvs_path: Option<PathBuf>,
}

pub fn get_config(executable: &PathBuf, workspace_dir: &PathBuf) -> PoetryConfig {
    let cache_dir = get_config_path(executable, workspace_dir, "cache-dir");
    let virtualenvs_path = get_config_path(executable, workspace_dir, "virtualenvs.path");
    let virtualenvs_in_project =
        get_config_bool(executable, workspace_dir, "virtualenvs.in-project");
    PoetryConfig {
        cache_dir,
        virtualenvs_in_project,
        virtualenvs_path,
    }
}

fn get_config_bool(executable: &PathBuf, workspace_dir: &PathBuf, setting: &str) -> Option<bool> {
    match get_config_value(executable, workspace_dir, setting) {
        Some(output) => {
            let output = output.trim();
            if output.starts_with("true") {
                Some(true)
            } else if output.starts_with("false") {
                Some(false)
            } else {
                None
            }
        }
        None => None,
    }
}
fn get_config_path(
    executable: &PathBuf,
    workspace_dir: &PathBuf,
    setting: &str,
) -> Option<PathBuf> {
    get_config_value(executable, workspace_dir, setting).map(|output| PathBuf::from(output.trim()))
}

fn get_config_value(
    executable: &PathBuf,
    workspace_dir: &PathBuf,
    setting: &str,
) -> Option<String> {
    let start = SystemTime::now();
    let result = std::process::Command::new(executable)
        .arg("config")
        .arg(setting)
        .current_dir(workspace_dir)
        .output();
    trace!(
        "Executed Poetry ({}ms): {executable:?} config {setting} {workspace_dir:?}",
        start.elapsed().unwrap_or_default().as_millis(),
    );
    match result {
        Ok(output) => {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                trace!(
                    "Failed to get Poetry config {setting} using exe {executable:?} in {workspace_dir:?}, due to ({}) {}",
                    output.status.code().unwrap_or_default(),
                    stderr
                );
                None
            }
        }
        Err(err) => {
            error!("Failed to execute Poetry env list {:?}", err);
            None
        }
    }
}
