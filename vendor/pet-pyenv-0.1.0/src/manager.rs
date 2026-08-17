// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::{
    env_variables::EnvVariables,
    environment_locations::{get_binary_from_known_paths, get_home_pyenv_dir, get_pyenv_dir},
};
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::{fs, path::PathBuf};

lazy_static! {
    // Sample /opt/homebrew/Cellar/pyenv/2.4.0/libexec/pyenv
    static ref PYENV_VERSION_FROM_PATH: Regex = Regex::new(r"pyenv/((\d+\.?)*)/")
        .expect("error parsing Version regex for PyEnv Version from Path in pyenv");
    // Sample /opt/homebrew/Cellar/pyenv/2.4.0/libexec/pyenv
    static ref PYENV_VERSION_FROM_VERSION_FILE: Regex = Regex::new(r"(\d+\.\d+\.\d+)")
        .expect("error parsing Version regex for PyEnv Version from Version File in pyenv");
}

#[derive(Debug)]
pub struct PyEnvInfo {
    #[allow(dead_code)]
    pub exe: Option<PathBuf>,
    pub versions: Option<PathBuf>,
    pub version: Option<String>,
}

impl PyEnvInfo {
    pub fn from(environment: &EnvVariables) -> PyEnvInfo {
        get_pyenv_info(environment)
    }
}

fn get_pyenv_info(environment: &EnvVariables) -> PyEnvInfo {
    let mut pyenv = PyEnvInfo {
        exe: None,
        versions: None,
        version: None,
    };
    if let Some(dir) = get_pyenv_dir(environment) {
        let versions = dir.join("versions");
        if versions.exists() {
            pyenv.versions = Some(versions);
        }
        let exe = dir.join("bin").join("pyenv");
        if exe.exists() {
            pyenv.exe = Some(exe);
        }
    }
    if let Some(exe) = get_binary_from_known_paths(environment) {
        pyenv.exe = Some(exe);
    }

    if pyenv.exe.is_none() || pyenv.versions.is_none() {
        if let Some(path) = get_home_pyenv_dir(environment) {
            if pyenv.exe.is_none() {
                let exe = path.join("bin").join("pyenv");
                if exe.exists() {
                    pyenv.exe = Some(exe);
                }
            }
            if pyenv.versions.is_none() {
                let versions = path.join("versions");
                if versions.exists() {
                    pyenv.versions = Some(versions);
                }
            }
        }
    }

    // Get the version of the pyenv manager
    if let Some(ref exe) = pyenv.exe {
        pyenv.version = get_pyenv_manager_version(exe, environment);
    }

    pyenv
}

#[cfg(windows)]
fn get_pyenv_manager_version(
    _pyenv_binary_path: &Path,
    environment: &EnvVariables,
) -> Option<String> {
    // In windows, the version is stored in the `.pyenv/.version` file

    // Try env var path first, then fall back to home directory
    let pyenv_dir = get_pyenv_dir(environment)
        .or_else(|| get_home_pyenv_dir(environment)?.parent().map(PathBuf::from))?;

    let mut version_file = pyenv_dir.join(".version");
    if !version_file.exists() {
        // We might have got the path `~/.pyenv/pyenv-win`
        version_file = pyenv_dir.parent()?.join(".version");
        if !version_file.exists() {
            return None;
        }
    }
    let version = fs::read_to_string(version_file).ok()?;
    Some(
        PYENV_VERSION_FROM_VERSION_FILE
            .captures(&version)?
            .get(1)?
            .as_str()
            .to_string(),
    )
}

#[cfg(unix)]
fn get_pyenv_manager_version(pyenv_exe: &Path, _environment: &EnvVariables) -> Option<String> {
    let real_path = fs::read_link(pyenv_exe).ok()?;
    // Look for version in path
    // Sample /opt/homebrew/Cellar/pyenv/2.4.0/libexec/pyenv
    Some(
        PYENV_VERSION_FROM_PATH
            .captures(real_path.to_str().unwrap_or_default())?
            .get(1)?
            .as_str()
            .to_string(),
    )
}
