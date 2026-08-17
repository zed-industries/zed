// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![cfg(unix)]

mod common;
use pet_python_utils::executable;
use std::path::PathBuf;

use common::resolve_test_path;

#[cfg(unix)]
#[test]
fn find_executables() {
    // .venv
    let path: PathBuf = resolve_test_path(&["unix", "executables", ".venv"]);
    let mut executables = executable::find_executables(path.clone());
    executables.sort();

    assert_eq!(
        executables,
        vec![
            resolve_test_path(&["unix", "executables", ".venv", "bin", "python"]),
            resolve_test_path(&["unix", "executables", ".venv", "bin", "python3"]),
        ]
    );

    // Python3.9.9
    let path: PathBuf = resolve_test_path(&["unix", "executables", "python3.9.9"]);
    let mut executables = executable::find_executables(path.clone());
    executables.sort();

    assert_eq!(
        executables,
        vec![
            resolve_test_path(&["unix", "executables", "python3.9.9", "bin", "python3"]),
            resolve_test_path(&["unix", "executables", "python3.9.9", "bin", "python3.9.9"]),
        ]
    );

    // Conda without Python.
    let path: PathBuf = resolve_test_path(&["unix", "executables", "conda_without_python"]);
    let executables = executable::find_executables(path.clone());

    assert_eq!(executables.len(), 0);

    // Bogus dir
    let path: PathBuf = resolve_test_path(&["unix_bogus_dir"]);
    let executables = executable::find_executables(path.clone());

    assert_eq!(executables.len(), 0);
}
