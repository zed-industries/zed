// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::build_details::BuildDetails;
use crate::headers::{self, Headers};
use log::{trace, warn};
use pet_core::pyvenv_cfg::PyVenvCfg;
use pet_fs::path::resolve_symlink;
use std::{
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

pub fn from_header_files(prefix: &Path) -> Option<String> {
    Headers::get_version(prefix)
}
pub fn from_pyvenv_cfg(prefix: &Path) -> Option<String> {
    PyVenvCfg::find(prefix).and_then(|cfg| cfg.version)
}
/// Reads the Python version from a `build-details.json` file ([PEP 739])
/// installed by Python 3.14+ in the platform-independent stdlib directory.
///
/// [PEP 739]: https://peps.python.org/pep-0739/
pub fn from_build_details(prefix: &Path) -> Option<String> {
    BuildDetails::find(prefix).map(|bd| bd.version_string())
}
pub fn from_creator_for_virtual_env(prefix: &Path) -> Option<String> {
    if let Some(version) = Headers::get_version(prefix) {
        return Some(version);
    }
    let mut bin = "bin";
    let mut executable = prefix.join(bin).join("python");
    if cfg!(windows) && !executable.exists() {
        bin = "Scripts";
        executable = prefix.join(bin).join("python.exe");
    }

    // Determine who created this virtual environment, and get version of that environment.
    // Note, its unlikely conda envs were used to create virtual envs, thats a very bad idea (known to cause issues and not reccomended).
    // Hence do not support conda envs when getting versio of the parent env.
    if let Some(mut creator_executable) = get_python_exe_used_to_create_venv(executable) {
        // Possible we got resolved to the same bin directory but python3.10
        if creator_executable.starts_with(prefix) {
            creator_executable = resolve_symlink(&creator_executable)?;
        }
        let parent_dir = creator_executable.parent()?;
        if parent_dir.file_name().unwrap_or_default() != bin {
            trace!("Creator of virtual environment found, but the creator of {:?} is located in {:?} , instead of a {:?} directory", prefix, creator_executable, bin);
            None
        } else {
            // Assume the python environment used to create this virtual env is a regular install of Python.
            // Try to get the version of that environment.
            let sys_root = parent_dir.parent()?;
            let pyver = if let Some(pyvenvcfg) = PyVenvCfg::find(prefix) {
                match (pyvenvcfg.version_major, pyvenvcfg.version_minor) {
                    (Some(major), Some(minor)) => Some((major, minor)),
                    _ => None,
                }
            } else {
                None
            };
            // Prefer build-details.json (Python 3.14+) over header parsing — it's
            // a single small file read vs. recursively scanning include/Headers.
            BuildDetails::find_with_hint(sys_root, pyver)
                .map(|bd| bd.version_string())
                .or_else(|| headers::get_version(sys_root, pyver))
        }
    } else if cfg!(windows) {
        // Only on windows is it difficult to get the creator of the virtual environment.
        get_version_from_pyvenv_if_pyvenv_cfg_and_exe_created_same_time(prefix)
    } else {
        None
    }
}

pub fn from_prefix(prefix: &Path) -> Option<String> {
    if let Some(version) = from_pyvenv_cfg(prefix) {
        Some(version)
    } else if let Some(version) = from_build_details(prefix) {
        Some(version)
    } else {
        from_header_files(prefix)
    }
}

/// When creating virtual envs using `python -m venv` or the like,
/// The executable in the new environment ends up being a symlink to the python executable used to create the env.
/// Using this information its possible to determine the version of the Python environment used to create the env.
fn get_python_exe_used_to_create_venv<T: AsRef<Path>>(executable: T) -> Option<PathBuf> {
    let parent_dir = executable.as_ref().parent()?;
    if cfg!(windows) {
        if parent_dir.file_name().unwrap_or_default() != "bin"
            && parent_dir.file_name().unwrap_or_default() != "Scripts"
        {
            warn!("Attempted to determine creator of virtual environment, but the env executable ({:?}) is not in the expected location.", executable.as_ref());
            return None;
        }
    } else if parent_dir.file_name().unwrap_or_default() != "bin" {
        warn!("Attempted to determine creator of virtual environment, but the env executable ({:?}) is not in the expected location.", executable.as_ref());
        return None;
    }

    let symlink = resolve_symlink(&executable)?;
    if symlink.is_file() {
        Some(symlink)
    } else {
        None
    }
}

/// Use pyvenv.cfg to get the version of the virtual environment in windows.
/// If the creation/modified dates of the pyvenv.cfg and the Scripts/python.exe are in the same period (few minutes apart)
/// Then we can use the pyvenv.cfg to get the version of the virtual environment.
fn get_version_from_pyvenv_if_pyvenv_cfg_and_exe_created_same_time(
    prefix: &Path,
) -> Option<String> {
    let cfg = PyVenvCfg::find(prefix)?;
    let pyvenv_cfg = prefix.join("pyvenv.cfg");
    if !pyvenv_cfg.exists() {
        return None;
    }
    let cfg_metadata = pyvenv_cfg.metadata().ok()?;
    let mut bin = prefix.join("Scripts");
    if !bin.exists() {
        bin = prefix.join("bin");
    }
    let exe_metadata = bin.join("python.exe").metadata().ok()?;
    let cfg_modified = cfg_metadata
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();
    let exe_modified = exe_metadata
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();
    // If they are just a few minutes apart,
    // then we can assume the version in the pyvenv.cfg is the version of the virtual environment.
    if cfg_modified.abs_diff(exe_modified) < 60 {
        trace!(
            "Using pyvenv.cfg to get version of virtual environment {:?}",
            prefix
        );
        cfg.version
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    fn write_file(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut f = fs::File::create(path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
    }

    /// `pyvenv.cfg` must take priority over `build-details.json` when both
    /// exist in the same prefix; otherwise we'd report the *base* interpreter's
    /// version for a venv whose stdlib is shared with its base.
    #[test]
    fn from_prefix_prefers_pyvenv_cfg_over_build_details() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write_file(
            &prefix.join("pyvenv.cfg"),
            "home = /usr/bin\nversion = 3.11.5\n",
        );
        write_file(
            &prefix
                .join("lib")
                .join("python3.14")
                .join("build-details.json"),
            r#"{
                "schema_version": "1.0",
                "language": {
                    "version": "3.14",
                    "version_info": {
                        "major": 3,
                        "minor": 14,
                        "micro": 1,
                        "releaselevel": "final",
                        "serial": 0
                    }
                }
            }"#,
        );

        assert_eq!(from_prefix(prefix), Some("3.11.5".to_string()));
    }

    /// `build-details.json` should win over header parsing when both exist —
    /// it's cheaper to read and is the authoritative source on Python 3.14+.
    #[test]
    fn from_prefix_prefers_build_details_over_headers() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        // Conflicting sources: headers say 3.13.0, build-details says 3.14.1.
        write_file(
            &prefix.join("include").join("patchlevel.h"),
            "#define PY_VERSION              \"3.13.0\"\n",
        );
        write_file(
            &prefix
                .join("lib")
                .join("python3.14")
                .join("build-details.json"),
            r#"{
                "schema_version": "1.0",
                "language": {
                    "version": "3.14",
                    "version_info": {
                        "major": 3,
                        "minor": 14,
                        "micro": 1,
                        "releaselevel": "final",
                        "serial": 0
                    }
                }
            }"#,
        );

        assert_eq!(from_prefix(prefix), Some("3.14.1".to_string()));
    }
}
