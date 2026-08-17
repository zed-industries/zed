// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod common;

#[cfg(unix)]
#[test]
fn finds_manager_from_root_env() {
    use common::resolve_test_path;
    use pet_conda::manager::CondaManager;

    let path = resolve_test_path(&["unix", "anaconda3-2023.03"]);

    let manager = CondaManager::from(&path).unwrap();

    assert_eq!(manager.executable, path.join("bin").join("conda"));
    assert_eq!(manager.version, Some("23.1.0".into()));
}

#[cfg(unix)]
#[test]
fn finds_manager_from_root_within_an_env() {
    use common::resolve_test_path;
    use pet_conda::manager::CondaManager;

    let conda_dir = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    let path = resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "env_python_3"]);

    let manager = CondaManager::from(&path).unwrap();

    assert_eq!(manager.executable, conda_dir.join("bin").join("conda"));
    assert_eq!(manager.version, Some("23.1.0".into()));

    // Try a conda env without Python
    let path = resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "without_python"]);

    let manager = CondaManager::from(&path).unwrap();

    assert_eq!(manager.executable, conda_dir.join("bin").join("conda"));
    assert_eq!(manager.version, Some("23.1.0".into()));
}

#[cfg(unix)]
#[test]
fn does_not_find_conda_env_for_bogus_dirs() {
    use common::resolve_test_path;
    use pet_conda::manager::CondaManager;

    let path = resolve_test_path(&["unix", "bogus_directory"]);

    assert!(CondaManager::from(&path).is_none());
}

/// Test that find_conda_binary finds conda from the PATH environment variable.
/// This is important for discovering conda installations on mapped drives and
/// other non-standard locations (fixes https://github.com/microsoft/python-environment-tools/issues/194).
#[cfg(unix)]
#[test]
fn finds_conda_binary_from_path() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::env_variables::EnvVariables;
    use pet_conda::manager::find_conda_binary;
    use std::collections::HashMap;

    let anaconda_bin = resolve_test_path(&["unix", "anaconda3-2023.03", "bin"]);
    let path_value = anaconda_bin.to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("PATH".to_string(), path_value);

    let env = create_test_environment(vars, None, vec![], None);
    let env_vars = EnvVariables::from(&env);

    let conda_binary = find_conda_binary(&env_vars);

    assert!(conda_binary.is_some());
    assert_eq!(
        conda_binary.unwrap(),
        resolve_test_path(&["unix", "anaconda3-2023.03", "bin", "conda"])
    );
}

/// Test that find_conda_binary also works when conda is in the condabin directory
/// (common on Windows with Miniforge/Anaconda where condabin is added to PATH).
#[cfg(unix)]
#[test]
fn finds_conda_binary_from_condabin_path() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::env_variables::EnvVariables;
    use pet_conda::manager::find_conda_binary;
    use std::collections::HashMap;

    let anaconda_condabin = resolve_test_path(&["unix", "anaconda3-2023.03", "condabin"]);
    let path_value = anaconda_condabin.to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("PATH".to_string(), path_value);

    let env = create_test_environment(vars, None, vec![], None);
    let env_vars = EnvVariables::from(&env);

    let conda_binary = find_conda_binary(&env_vars);

    assert!(conda_binary.is_some());
    assert_eq!(
        conda_binary.unwrap(),
        resolve_test_path(&["unix", "anaconda3-2023.03", "condabin", "conda"])
    );
}

/// Test that find_conda_binary returns None when conda is not on PATH.
#[cfg(unix)]
#[test]
fn does_not_find_conda_binary_when_not_on_path() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::env_variables::EnvVariables;
    use pet_conda::manager::find_conda_binary;
    use std::collections::HashMap;

    // Use a path that doesn't have conda
    let some_other_path = resolve_test_path(&["unix", "bogus_directory"]);
    let path_value = some_other_path.to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("PATH".to_string(), path_value);

    let env = create_test_environment(vars, None, vec![], None);
    let env_vars = EnvVariables::from(&env);

    let conda_binary = find_conda_binary(&env_vars);

    assert!(conda_binary.is_none());
}

// ==================== Mamba/Micromamba Tests ====================

/// Test is_mamba_executable correctly identifies mamba binaries.
#[test]
fn is_mamba_executable_identifies_mamba_binaries() {
    use pet_conda::manager::is_mamba_executable;
    use std::path::Path;

    // Cross-platform: forward-slash paths work on all platforms
    assert!(is_mamba_executable(Path::new("/usr/bin/mamba")));
    assert!(is_mamba_executable(Path::new("/usr/bin/micromamba")));
    assert!(is_mamba_executable(Path::new("/opt/miniforge3/bin/mamba")));

    // Should NOT match conda or python
    assert!(!is_mamba_executable(Path::new("/usr/bin/conda")));
    assert!(!is_mamba_executable(Path::new("/usr/bin/python")));

    // Windows-specific paths with backslashes (only valid on Windows)
    #[cfg(windows)]
    {
        assert!(is_mamba_executable(Path::new("C:\\Scripts\\mamba.exe")));
        assert!(is_mamba_executable(Path::new(
            "C:\\Scripts\\micromamba.exe"
        )));
        assert!(!is_mamba_executable(Path::new("C:\\Scripts\\conda.exe")));
    }
}

/// Test that get_mamba_manager finds a mamba manager from a conda install with mamba.
#[cfg(unix)]
#[test]
fn finds_mamba_manager_from_miniforge_root() {
    use common::resolve_test_path;
    use pet_conda::manager::CondaManager;
    use pet_core::manager::EnvManagerType;

    let path = resolve_test_path(&["unix", "miniforge3-mamba"]);
    let manager = CondaManager::from(&path).unwrap();

    // Should find conda first (conda takes precedence)
    assert_eq!(manager.executable, path.join("bin").join("conda"));
    assert_eq!(manager.version, Some("24.1.0".into()));
    assert_eq!(manager.manager_type, EnvManagerType::Conda);
}

/// Test that CondaManager::from falls back to mamba when only micromamba is available.
#[cfg(unix)]
#[test]
fn finds_mamba_manager_when_only_micromamba_available() {
    use common::resolve_test_path;
    use pet_conda::manager::CondaManager;
    use pet_core::manager::EnvManagerType;

    let path = resolve_test_path(&["unix", "micromamba-only"]);
    let manager = CondaManager::from(&path).unwrap();

    // No conda binary, should fall back to micromamba
    assert_eq!(manager.executable, path.join("bin").join("micromamba"));
    assert_eq!(manager.version, None); // version is unknown for mamba
    assert_eq!(manager.manager_type, EnvManagerType::Mamba);
}

/// Test that find_mamba_binary finds mamba from the PATH environment variable.
#[cfg(unix)]
#[test]
fn finds_mamba_binary_from_path() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::env_variables::EnvVariables;
    use pet_conda::manager::find_mamba_binary;
    use std::collections::HashMap;

    let miniforge_bin = resolve_test_path(&["unix", "miniforge3-mamba", "bin"]);
    let path_value = miniforge_bin.to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("PATH".to_string(), path_value);

    let env = create_test_environment(vars, None, vec![], None);
    let env_vars = EnvVariables::from(&env);

    let mamba_binary = find_mamba_binary(&env_vars);

    assert!(mamba_binary.is_some());
    assert_eq!(
        mamba_binary.unwrap(),
        resolve_test_path(&["unix", "miniforge3-mamba", "bin", "mamba"])
    );
}

/// Test that find_mamba_binary finds micromamba when mamba is not available.
#[cfg(unix)]
#[test]
fn finds_micromamba_binary_from_path() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::env_variables::EnvVariables;
    use pet_conda::manager::find_mamba_binary;
    use std::collections::HashMap;

    let micromamba_bin = resolve_test_path(&["unix", "micromamba-only", "bin"]);
    let path_value = micromamba_bin.to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("PATH".to_string(), path_value);

    let env = create_test_environment(vars, None, vec![], None);
    let env_vars = EnvVariables::from(&env);

    let mamba_binary = find_mamba_binary(&env_vars);

    assert!(mamba_binary.is_some());
    assert_eq!(
        mamba_binary.unwrap(),
        resolve_test_path(&["unix", "micromamba-only", "bin", "micromamba"])
    );
}

/// Test that find_mamba_binary returns None when mamba is not on PATH.
#[cfg(unix)]
#[test]
fn does_not_find_mamba_binary_when_not_on_path() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::env_variables::EnvVariables;
    use pet_conda::manager::find_mamba_binary;
    use std::collections::HashMap;

    // Use the anaconda path which has conda but no mamba
    let anaconda_bin = resolve_test_path(&["unix", "anaconda3-2023.03", "bin"]);
    let path_value = anaconda_bin.to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("PATH".to_string(), path_value);

    let env = create_test_environment(vars, None, vec![], None);
    let env_vars = EnvVariables::from(&env);

    let mamba_binary = find_mamba_binary(&env_vars);

    assert!(mamba_binary.is_none());
}

/// Test that CondaManager::from finds mamba manager from a child env
/// in a micromamba-only installation (no conda binary).
#[cfg(unix)]
#[test]
fn finds_mamba_manager_from_child_env_in_miniforge() {
    use common::resolve_test_path;
    use pet_conda::manager::CondaManager;
    use pet_core::manager::EnvManagerType;

    let path = resolve_test_path(&["unix", "miniforge3-mamba", "envs", "myenv"]);
    let manager = CondaManager::from(&path).unwrap();

    // Should find the conda manager from the parent install dir
    let conda_dir = resolve_test_path(&["unix", "miniforge3-mamba"]);
    assert_eq!(manager.executable, conda_dir.join("bin").join("conda"));
    assert_eq!(manager.manager_type, EnvManagerType::Conda);
}
