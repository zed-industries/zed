// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use lazy_static::lazy_static;
use log::warn;
use pet_core::arch::Architecture;
use regex::Regex;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

lazy_static! {
    // Sample => python-3.12.2-hdf0ec26_0_cpython.json
    static ref PYTHON_VERSION: Regex = Regex::new("^python-([\\d+\\.*]*)-.*.json$")
        .expect("error parsing Version regex for Python Package Version in conda");
    // Sample => conda-23.1.0-py310hca03da5_0.json
    static ref CONDA_VERSION: Regex = Regex::new("^conda-([\\d+\\.*]*)-.*.json$")
        .expect("error parsing Version regex for Conda Package Version in conda");
    // Sample => +defaults::python-3.10.9-hc0d8a6c_1
    // Sample => +conda-forge/osx-arm64::python-3.12.2-hdf0ec26_0_cpython
    static ref PYTHON_VERSION_IN_HISTORY: Regex = Regex::new(".*python-([\\d+\\.*]*)-(.*)")
        .expect("error parsing Version regex for Python Package Version in conda");
    // Sample => +defaults::conda-23.1.0-py310hca03da5_0
    static ref CONDA_VERSION_IN_HISTORY: Regex = Regex::new(".*conda-([\\d+\\.*]*)-(.*)")
        .expect("error parsing Version regex for Conda Package Version in conda");
}

use std::{fmt, fs};

#[derive(Debug, Clone, PartialEq)]
pub enum Package {
    Conda,
    Python,
}

impl Package {
    pub fn to_name(&self) -> &str {
        match self {
            Package::Conda => "conda",
            Package::Python => "python",
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Package::Conda => write!(f, "Conda"),
            Package::Python => write!(f, "Python"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CondaPackageInfo {
    pub package: Package,
    #[allow(dead_code)]
    pub path: PathBuf,
    pub version: String,
    pub arch: Option<Architecture>,
}

impl CondaPackageInfo {
    pub fn from(path: &Path, package: &Package) -> Option<Self> {
        let history = fs::read_to_string(path.join("conda-meta").join("history")).ok();
        Self::from_history(path, package, history.as_deref())
    }

    pub(crate) fn from_history(
        path: &Path,
        package: &Package,
        history: Option<&str>,
    ) -> Option<Self> {
        get_conda_package_info(path, package, history)
    }
}

#[derive(Deserialize, Debug)]
struct CondaMetaPackageStructure {
    channel: Option<String>,
    version: Option<String>,
}

/// Get the details of a conda package from the 'conda-meta' directory.
fn get_conda_package_info(
    path: &Path,
    name: &Package,
    history: Option<&str>,
) -> Option<CondaPackageInfo> {
    if let Some(info) =
        history.and_then(|history| get_conda_package_info_from_history(path, name, history))
    {
        Some(info)
    } else {
        warn!(
            "Unable to find conda package {} in {:?}, trying slower approach",
            name, path
        );

        get_conda_package_info_from_package_json(path, name)
    }
}

fn get_conda_package_info_from_history(
    path: &Path,
    name: &Package,
    history_contents: &str,
) -> Option<CondaPackageInfo> {
    // conda-meta is in the root of the conda installation folder
    let path = path.join("conda-meta");
    let package_entry = format!(":{}-", name.to_name());

    // Filter to only include lines that:
    // 1. Start with '+' (installed packages, not '-' for removed packages)
    // 2. Contain the package entry (e.g., ":python-")
    //
    // We need the LAST matching entry because conda appends to history chronologically.
    // When a package is upgraded, the old version is removed (-) and new version installed (+).
    // The last '+' entry represents the currently installed version.
    //
    // Sample history for Python upgrade from 3.9.18 to 3.9.21:
    //   +defaults::python-3.9.18-h123456_0     <- initial install
    //   ...
    //   -defaults::python-3.9.18-h123456_0     <- removed during upgrade
    //   +defaults::python-3.9.21-h789abc_0     <- current version (we want this)
    let line = history_contents
        .lines()
        .rfind(|line| line.starts_with('+') && line.contains(&package_entry))?;

    // Sample entry in the history file
    // +conda-forge/osx-arm64::psutil-5.9.8-py312he37b823_0
    // +conda-forge/osx-arm64::python-3.12.2-hdf0ec26_0_cpython
    // +conda-forge/osx-arm64::python_abi-3.12-4_cp312
    let regex = get_package_version_history_regex(name);
    if let Some(captures) = regex.captures(line) {
        if let Some(version) = captures.get(1) {
            if let Some(hash) = captures.get(2) {
                let package_path = format!(
                    "{}-{}-{}.json",
                    name.to_name(),
                    version.as_str(),
                    hash.as_str()
                );
                let package_path = path.join(package_path);
                let mut arch: Option<Architecture> = None;
                // Sample contents
                // {
                //   "build": "h966fe2a_2",
                //   "build_number": 2,
                //   "channel": "https://repo.anaconda.com/pkgs/main/win-64",
                //   "constrains": [],
                // }
                // 32bit channel is https://repo.anaconda.com/pkgs/main/win-32/
                // 64bit channel is "channel": "https://repo.anaconda.com/pkgs/main/osx-arm64",
                if let Ok(contents) = read_to_string(&package_path) {
                    if let Ok(js) = serde_json::from_str::<CondaMetaPackageStructure>(&contents) {
                        if let Some(channel) = js.channel {
                            if channel.ends_with("64") {
                                arch = Some(Architecture::X64);
                            } else if channel.ends_with("32") {
                                arch = Some(Architecture::X86);
                            }
                        }
                        if let Some(version) = js.version {
                            return Some(CondaPackageInfo {
                                package: name.clone(),
                                path: package_path,
                                version,
                                arch,
                            });
                        } else {
                            warn!(
                                "Unable to find version for package {} in {:?}",
                                name, package_path
                            );
                        }
                    }
                }
            }
        }
    }
    None
}

fn get_conda_package_info_from_package_json(
    path: &Path,
    name: &Package,
) -> Option<CondaPackageInfo> {
    let package_name = format!("{}-", name.to_name());
    let regex = get_package_version_regex(name);
    let path = path.join("conda-meta");

    // Fallback, slower approach of enumerating all files.
    let entries = fs::read_dir(path).ok()?;
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        if file_name.starts_with(&package_name) && file_name.ends_with(".json") {
            if let Some(captures) = regex.captures(&file_name) {
                if let Some(version) = captures.get(1) {
                    let mut arch: Option<Architecture> = None;
                    // Sample contents
                    // {
                    //   "build": "h966fe2a_2",
                    //   "build_number": 2,
                    //   "channel": "https://repo.anaconda.com/pkgs/main/win-64",
                    //   "constrains": [],
                    // }
                    // 32bit channel is https://repo.anaconda.com/pkgs/main/win-32/
                    // 64bit channel is "channel": "https://repo.anaconda.com/pkgs/main/osx-arm64",
                    if let Ok(contents) = read_to_string(&path) {
                        if let Ok(js) = serde_json::from_str::<CondaMetaPackageStructure>(&contents)
                        {
                            if let Some(channel) = js.channel {
                                if channel.ends_with("64") {
                                    arch = Some(Architecture::X64);
                                } else if channel.ends_with("32") {
                                    arch = Some(Architecture::X86);
                                }
                            }
                        }
                    }
                    return Some(CondaPackageInfo {
                        package: name.clone(),
                        path: path.clone(),
                        version: version.as_str().to_string(),
                        arch,
                    });
                }
            }
        }
    }
    None
}

fn get_package_version_regex(package: &Package) -> &Regex {
    match package {
        Package::Conda => &CONDA_VERSION,
        Package::Python => &PYTHON_VERSION,
    }
}
fn get_package_version_history_regex(package: &Package) -> &Regex {
    match package {
        Package::Conda => &CONDA_VERSION_IN_HISTORY,
        Package::Python => &PYTHON_VERSION_IN_HISTORY,
    }
}
