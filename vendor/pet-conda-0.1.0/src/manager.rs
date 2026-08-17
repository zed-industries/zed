// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    conda_info::CondaInfo,
    env_variables::EnvVariables,
    environments::get_conda_installation_used_to_create_conda_env,
    package::CondaPackageInfo,
    utils::{is_conda_env, is_conda_install},
};
use log::trace;
use pet_core::{manager::EnvManager, manager::EnvManagerType};
use std::{
    env,
    path::{Path, PathBuf},
};

fn get_conda_executable(path: &Path) -> Option<PathBuf> {
    #[cfg(windows)]
    let relative_path_to_conda_exe = vec![
        PathBuf::from("Scripts").join("conda.exe"),
        PathBuf::from("Scripts").join("conda.bat"),
        PathBuf::from("bin").join("conda.exe"),
        PathBuf::from("bin").join("conda.bat"),
    ];
    #[cfg(unix)]
    let relative_path_to_conda_exe = vec![PathBuf::from("bin").join("conda")];

    for relative_path in relative_path_to_conda_exe {
        let exe = path.join(&relative_path);
        if exe.exists() {
            return Some(exe);
        }
    }

    None
}

fn get_mamba_executable(path: &Path) -> Option<PathBuf> {
    #[cfg(windows)]
    let relative_paths = vec![
        PathBuf::from("Scripts").join("mamba.exe"),
        PathBuf::from("Scripts").join("mamba.bat"),
        PathBuf::from("Scripts").join("micromamba.exe"),
        PathBuf::from("Scripts").join("micromamba.bat"),
        PathBuf::from("bin").join("mamba.exe"),
        PathBuf::from("bin").join("mamba.bat"),
        PathBuf::from("bin").join("micromamba.exe"),
        PathBuf::from("bin").join("micromamba.bat"),
    ];
    #[cfg(unix)]
    let relative_paths = vec![
        PathBuf::from("bin").join("mamba"),
        PathBuf::from("bin").join("micromamba"),
    ];

    for relative_path in relative_paths {
        let exe = path.join(&relative_path);
        if exe.exists() {
            return Some(exe);
        }
    }

    None
}

/// Specifically returns the file names that are valid for 'conda' on windows
#[cfg(windows)]
fn get_conda_bin_names() -> Vec<&'static str> {
    vec!["conda.exe", "conda.bat"]
}

/// Specifically returns the file names that are valid for 'conda' on linux/Mac
#[cfg(unix)]
fn get_conda_bin_names() -> Vec<&'static str> {
    vec!["conda"]
}

/// Specifically returns the file names that are valid for 'mamba'/'micromamba' on windows
#[cfg(windows)]
fn get_mamba_bin_names() -> Vec<&'static str> {
    vec!["mamba.exe", "mamba.bat", "micromamba.exe", "micromamba.bat"]
}

/// Specifically returns the file names that are valid for 'mamba'/'micromamba' on linux/Mac
#[cfg(unix)]
fn get_mamba_bin_names() -> Vec<&'static str> {
    vec!["mamba", "micromamba"]
}

/// Find the conda binary on the PATH environment variable
pub fn find_conda_binary(env_vars: &EnvVariables) -> Option<PathBuf> {
    let paths = env_vars.path.clone()?;
    for path in env::split_paths(&paths) {
        for bin in get_conda_bin_names() {
            let conda_path = path.join(bin);
            if conda_path.is_file() || conda_path.is_symlink() {
                return Some(conda_path);
            }
        }
    }
    None
}

/// Find a mamba or micromamba binary on the PATH environment variable
pub fn find_mamba_binary(env_vars: &EnvVariables) -> Option<PathBuf> {
    let paths = env_vars.path.clone()?;
    for path in env::split_paths(&paths) {
        for bin in get_mamba_bin_names() {
            let mamba_path = path.join(bin);
            if mamba_path.is_file() || mamba_path.is_symlink() {
                return Some(mamba_path);
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct CondaManager {
    pub executable: PathBuf,
    pub version: Option<String>,
    pub conda_dir: Option<PathBuf>,
    pub manager_type: EnvManagerType,
}

impl CondaManager {
    pub fn to_manager(&self) -> EnvManager {
        EnvManager {
            tool: self.manager_type,
            executable: self.executable.clone(),
            version: self.version.clone(),
        }
    }
    pub fn from(path: &Path) -> Option<CondaManager> {
        if !is_conda_env(path) {
            return None;
        }

        // If this environment is in a folder named `envs`, then the parent directory of `envs` is the root conda install folder.
        if let Some(parent) = path.ancestors().nth(2) {
            if is_conda_install(parent) {
                if let Some(manager) =
                    get_conda_manager(parent).or_else(|| get_mamba_manager(parent))
                {
                    return Some(manager);
                }
            }
        }

        // Possible this is a conda environment in some other location
        // Such as global env folders location configured via condarc file
        // Or a conda env created using `-p` flag.
        // Get the conda install folder from the history file.
        // Or its in a location such as `~/.conda/envs` or `~/miniconda3/envs` where the conda install folder is not a parent of this path.
        if let Some(conda_install_folder) = get_conda_installation_used_to_create_conda_env(path) {
            get_conda_manager(&conda_install_folder)
                .or_else(|| get_mamba_manager(&conda_install_folder))
        } else {
            // If this is a conda env and the parent is `.conda/envs`, then this is definitely NOT a root conda install folder.
            // Hence never use conda installs from these env paths.
            if let Some(parent) = path.parent() {
                if parent.ends_with(".conda/envs") || parent.ends_with(".conda\\envs") {
                    trace!(
                        "Parent path ends with .conda/envs, not a root conda install folder: {:?}",
                        parent
                    );
                    return None;
                }
            }

            if let Some(manager) = get_conda_manager(path).or_else(|| get_mamba_manager(path)) {
                Some(manager)
            } else {
                trace!("No conda or mamba manager found for path: {:?}", path);
                None
            }
        }
    }
    pub fn from_info(
        executable: &Path,
        info: &CondaInfo,
        manager_type: EnvManagerType,
    ) -> Option<CondaManager> {
        Some(CondaManager {
            executable: executable.to_path_buf(),
            version: Some(info.conda_version.clone()),
            conda_dir: info.conda_prefix.clone(),
            manager_type,
        })
    }
}

fn get_conda_manager(path: &Path) -> Option<CondaManager> {
    let conda_exe = get_conda_executable(path)?;
    if let Some(conda_pkg) = CondaPackageInfo::from(path, &crate::package::Package::Conda) {
        Some(CondaManager {
            executable: conda_exe,
            version: Some(conda_pkg.version),
            conda_dir: Some(path.to_path_buf()),
            manager_type: EnvManagerType::Conda,
        })
    } else {
        None
    }
}

/// Checks whether a given executable path refers to a mamba or micromamba binary.
pub fn is_mamba_executable(exe: &Path) -> bool {
    if let Some(name) = exe.file_name().and_then(|n| n.to_str()) {
        let name = name.to_lowercase();
        name.starts_with("mamba") || name.starts_with("micromamba")
    } else {
        false
    }
}

pub(crate) fn get_mamba_manager(path: &Path) -> Option<CondaManager> {
    let mamba_exe = get_mamba_executable(path)?;
    // We cannot reliably determine the mamba/micromamba version from package metadata alone.
    // The conda package version in conda-meta is the conda version, not the mamba version.
    // Determining the mamba version would require spawning the mamba process.
    Some(CondaManager {
        executable: mamba_exe,
        version: None,
        conda_dir: Some(path.to_path_buf()),
        manager_type: EnvManagerType::Mamba,
    })
}
