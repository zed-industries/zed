// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![cfg(unix)]

mod common;
use pet_python_utils::version;
use std::path::PathBuf;

use common::resolve_test_path;

#[cfg(unix)]
#[test]
fn version_from_sys_prefix() {
    let path: PathBuf = resolve_test_path(&["unix", "pyvenv_cfg", ".venv"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.12.1");

    let path: PathBuf = resolve_test_path(&["unix", "pyvenv_cfg", ".venv", "bin"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.12.1");
}

#[cfg(unix)]
#[test]
fn version_from_sys_prefix_using_version_info_format() {
    let path: PathBuf = resolve_test_path(&["unix", "pyvenv_cfg", "hatch_env"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.9.6.final.0");

    let path: PathBuf = resolve_test_path(&["unix", "pyvenv_cfg", "hatch_env", "bin"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.9.6.final.0");
}

#[cfg(unix)]
#[test]
fn no_version_without_pyvenv_cfg_and_without_headers() {
    let path: PathBuf = resolve_test_path(&["unix", "pyvenv_cfg", "python3.9.9_without_headers"]);
    let version = version::from_prefix(&path);
    assert!(version.is_none());

    let path: PathBuf =
        resolve_test_path(&["unix", "pyvenv_cfg", "python3.9.9_without_headers", "bin"]);
    let version = version::from_prefix(&path);
    assert!(version.is_none());

    let path: PathBuf = resolve_test_path(&[
        "unix",
        "pyvenv_cfg",
        "python3.9.9_without_headers",
        "bin",
        "python",
    ]);
    let version = version::from_prefix(&path);
    assert!(version.is_none());
}

#[cfg(unix)]
#[test]
fn no_version_for_invalid_paths() {
    let path: PathBuf = resolve_test_path(&["unix_1234"]);
    let version = version::from_prefix(&path);
    assert!(version.is_none());
}

#[cfg(unix)]
#[test]
fn version_from_header_files() {
    let path: PathBuf = resolve_test_path(&["unix", "headers", "python3.9.9"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.9.9");

    let path: PathBuf = resolve_test_path(&["unix", "headers", "python3.9.9", "bin"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.9.9");

    let path: PathBuf = resolve_test_path(&["unix", "headers", "python3.10-dev", "bin"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.10.14+");

    let path: PathBuf = resolve_test_path(&["unix", "headers", "python3.13", "bin"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.13.0a5");
}

#[cfg(unix)]
#[test]
fn version_from_build_details_json() {
    // Final release: micro version, no pre-release suffix.
    let path: PathBuf = resolve_test_path(&["unix", "build_details", "python3.14"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.14.2");

    let path: PathBuf = resolve_test_path(&["unix", "build_details", "python3.14", "bin"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.14.2");

    // Alpha pre-release: serial appended as `aN`.
    let path: PathBuf = resolve_test_path(&["unix", "build_details", "python3.15a"]);
    let version = version::from_prefix(&path).unwrap();
    assert_eq!(version, "3.15.0a1");

    // Direct accessor returns the same value.
    let direct =
        version::from_build_details(&resolve_test_path(&["unix", "build_details", "python3.14"]))
            .unwrap();
    assert_eq!(direct, "3.14.2");
}
