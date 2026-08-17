// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![cfg(unix)]

mod common;
use common::resolve_test_path;
use pet_conda::utils;
use std::path::PathBuf;

#[cfg(unix)]
#[test]
fn is_conda_install() {
    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    assert!(utils::is_conda_install(&path));

    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03-without-history"]);
    assert!(utils::is_conda_install(&path));
}

#[cfg(unix)]
#[test]
fn is_not_conda_install() {
    let path: PathBuf = resolve_test_path(&["unix", "some bogus directory"]);
    assert!(!utils::is_conda_install(&path));

    // Conda env is not an install location.
    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "env_python_3"]);
    assert!(!utils::is_conda_install(&path));
}

#[cfg(unix)]
#[test]
fn is_conda_env() {
    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    assert!(utils::is_conda_env(&path));

    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03-without-history"]);
    assert!(utils::is_conda_env(&path));

    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "env_python_3"]);
    assert!(utils::is_conda_env(&path));
}

#[cfg(unix)]
#[test]
fn is_not_conda_env() {
    let path: PathBuf = resolve_test_path(&["unix", "some bogus directory"]);
    assert!(!utils::is_conda_env(&path));

    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    assert!(utils::is_conda_env(&path));

    let path: PathBuf = resolve_test_path(&["unix", "anaconda3-2023.03-without-history"]);
    assert!(utils::is_conda_env(&path));
}
