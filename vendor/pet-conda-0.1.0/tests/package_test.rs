// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![cfg(unix)]

mod common;
use pet_conda::package::{self, CondaPackageInfo};
use std::path::PathBuf;

use common::resolve_test_path;

#[cfg(unix)]
#[test]
fn empty_result_for_bogus_paths() {
    let path: PathBuf = resolve_test_path(&["unix", "bogus_path"]);
    let pkg = CondaPackageInfo::from(&path, &package::Package::Conda);

    assert!(pkg.is_none());
}

#[cfg(unix)]
#[test]
fn get_conda_package_info() {
    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    let pkg = CondaPackageInfo::from(&path, &package::Package::Conda).unwrap();

    assert_eq!(pkg.package, package::Package::Conda);
    assert_eq!(pkg.version, "23.1.0".to_string());
    assert_eq!(
        pkg.path,
        resolve_test_path(&[
            "unix",
            "anaconda3-2023.03",
            "conda-meta",
            "conda-23.1.0-py310hca03da5_0.json"
        ])
    );
}

#[cfg(unix)]
#[test]
fn get_python_package_info() {
    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    let pkg = CondaPackageInfo::from(&path, &package::Package::Python).unwrap();

    assert_eq!(pkg.package, package::Package::Python);
    assert_eq!(pkg.version, "3.10.9".to_string());
    assert_eq!(
        pkg.path,
        resolve_test_path(&[
            "unix",
            "anaconda3-2023.03",
            "conda-meta",
            "python-3.10.9-hc0d8a6c_1.json"
        ])
    );
}

#[cfg(unix)]
#[test]
fn get_conda_package_info_without_history() {
    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03-without-history"]);
    let pkg = CondaPackageInfo::from(&path, &package::Package::Conda).unwrap();

    assert_eq!(pkg.package, package::Package::Conda);
    assert_eq!(pkg.version, "23.1.0".to_string());
    assert_eq!(
        pkg.path,
        resolve_test_path(&[
            "unix",
            "anaconda3-2023.03-without-history",
            "conda-meta",
            "conda-23.1.0-py310hca03da5_0.json"
        ])
    );
}

#[cfg(unix)]
#[test]
fn get_python_package_info_without_history() {
    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03-without-history"]);
    let pkg = CondaPackageInfo::from(&path, &package::Package::Python).unwrap();

    assert_eq!(pkg.package, package::Package::Python);
    assert_eq!(pkg.version, "3.10.9".to_string());
    assert_eq!(
        pkg.path,
        resolve_test_path(&[
            "unix",
            "anaconda3-2023.03-without-history",
            "conda-meta",
            "python-3.10.9-hc0d8a6c_1.json"
        ])
    );
}

/// Test that when Python is upgraded, we get the current (last installed) version,
/// not the original (first installed) version.
/// This is a regression test for https://github.com/microsoft/python-environment-tools/issues/239
///
/// The history file contains:
///   +defaults::python-3.9.18-h1a28f6b_0  (initial install)
///   -defaults::python-3.9.18-h1a28f6b_0  (removed during upgrade)
///   +defaults::python-3.9.21-h789abc_0   (current version)
///
/// We should detect version 3.9.21, not 3.9.18.
#[cfg(unix)]
#[test]
fn get_python_package_info_after_upgrade() {
    let path: PathBuf = resolve_test_path(&["unix", "conda_env_with_python_upgrade"]);
    let pkg = CondaPackageInfo::from(&path, &package::Package::Python).unwrap();

    assert_eq!(pkg.package, package::Package::Python);
    // Should be 3.9.21 (current version), NOT 3.9.18 (original version)
    assert_eq!(pkg.version, "3.9.21".to_string());
    assert_eq!(
        pkg.path,
        resolve_test_path(&[
            "unix",
            "conda_env_with_python_upgrade",
            "conda-meta",
            "python-3.9.21-h789abc_0.json"
        ])
    );
}
