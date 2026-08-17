// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::path::{Path, PathBuf};

/// conda-meta must exist as this contains a mandatory `history` file.
pub fn is_conda_install(path: &Path) -> bool {
    if (path.join("condabin").exists() || path.join("envs").exists())
        && path.join("conda-meta").exists()
    {
        // For https://github.com/microsoft/vscode-python/issues/24247
        // Possible the env has a condabin or envs folder but its not the install directory.
        // & in fact its just a regular conda env.
        // Easy way is to check if the grand parent folder is a conda install directory.
        if let Some(parent) = path.parent() {
            if let Some(parent) = parent.parent() {
                // If the grand parent is a conda install directory,
                // then this is definitely not a conda install dir.
                if (parent.join("condabin").exists() || parent.join("envs").exists())
                    && parent.join("conda-meta").exists()
                {
                    return false;
                }
            }
        }

        return true;
    }

    false
}

/// conda-meta must exist as this contains a mandatory `history` file.
/// The root conda installation folder is also a conda environment (its the base environment).
/// Exclude Pixi environments to avoid misidentifying them as conda environments.
pub fn is_conda_env(path: &Path) -> bool {
    path.join("conda-meta").is_dir() && !path.join("conda-meta").join("pixi").is_file()
}

/// Only used in tests, noop in production.
///
/// Change the root of the path to a new root.
/// Lets assume some config file is located in the root directory /etc/config/config.toml.
/// We cannot test this unless we create such a file on the root of the filesystem.
/// Thats very risky and not recommended (ideally we want to create stuff in separate test folders).
/// The solution is to change the root of the path to a test folder.
pub fn change_root_of_path(path: &Path, new_root: &Option<PathBuf>) -> PathBuf {
    if cfg!(windows) {
        return path.to_path_buf();
    }
    if let Some(new_root) = new_root {
        // This only applies in tests.
        // We need this, as the root folder cannot be mocked.
        // Strip the first `/` (this path is only for testing purposes)
        new_root.join(&path.to_string_lossy()[1..])
    } else {
        path.to_path_buf()
    }
}
