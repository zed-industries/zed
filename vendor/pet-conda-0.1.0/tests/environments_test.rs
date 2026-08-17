// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![cfg(unix)]

mod common;
use common::resolve_test_path;
use pet_conda::environments::CondaEnvironment;
use pet_core::arch::Architecture;

#[cfg(unix)]
#[test]
fn find_root_conda_env() {
    let path = resolve_test_path(&["unix", "anaconda3-2023.03"]);

    let env = CondaEnvironment::from(&path, &None).unwrap();

    assert_eq!(env.prefix, path.clone());
    assert_eq!(env.arch, Some(Architecture::X64));
    assert_eq!(env.conda_dir, Some(path.clone()));
    assert_eq!(
        env.executable,
        Some(path.clone().join("bin").join("python"))
    );
    assert_eq!(env.version, Some("3.10.9".into()));
}

#[cfg(unix)]
#[test]
fn find_root_conda_env_without_history_file() {
    let path = resolve_test_path(&["unix", "anaconda3-2023.03-without-history"]);

    let env = CondaEnvironment::from(&path, &None).unwrap();

    assert_eq!(env.prefix, path.clone());
    assert_eq!(env.arch, Some(Architecture::X64));
    assert_eq!(env.conda_dir, Some(path.clone()));
    assert_eq!(
        env.executable,
        Some(path.clone().join("bin").join("python"))
    );
    assert_eq!(env.version, Some("3.10.9".into()));
}

#[cfg(unix)]
#[test]
fn find_conda_env() {
    let conda_dir = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    let path = resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "env_python_3"]);

    let env = CondaEnvironment::from(&path, &None).unwrap();

    assert_eq!(env.prefix, path.clone());
    assert_eq!(env.arch, Some(Architecture::X64));
    assert_eq!(env.conda_dir, Some(conda_dir.clone()));
    assert_eq!(
        env.executable,
        Some(path.clone().join("bin").join("python"))
    );
    assert_eq!(env.version, Some("3.12.2".into()));
}

#[cfg(unix)]
#[test]
fn find_conda_env_without_python() {
    let conda_dir = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    let path = resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "without_python"]);

    let env = CondaEnvironment::from(&path, &None).unwrap();

    assert_eq!(env.prefix, path.clone());
    assert_eq!(env.arch, None);
    assert_eq!(env.conda_dir, Some(conda_dir.clone()));
    assert_eq!(env.executable, None);
    assert_eq!(env.version, None);
}
