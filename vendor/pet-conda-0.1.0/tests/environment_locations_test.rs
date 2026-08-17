// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod common;

#[cfg(unix)]
#[test]
fn non_existent_envrionments_txt() {
    use common::{create_env_variables, resolve_test_path};
    use pet_conda::environment_locations::get_conda_envs_from_environment_txt;

    let root = resolve_test_path(&["unix", "root_empty"]);
    let home = resolve_test_path(&["unix", "bogus directory"]);
    let env = create_env_variables(home, root);

    let environments = get_conda_envs_from_environment_txt(&env);

    assert!(environments.is_empty());
}

#[cfg(unix)]
#[test]
fn list_conda_envs_in_install_location() {
    use common::resolve_test_path;
    use pet_conda::environment_locations::get_environments;

    let path = resolve_test_path(&["unix", "anaconda3-2023.03"]);

    let mut locations = get_environments(&path);
    locations.sort();

    assert_eq!(
        locations,
        vec![
            resolve_test_path(&["unix", "anaconda3-2023.03"]),
            resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "env_python_3"]),
            resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "myenv"]),
            resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "without_python"]),
        ]
    );
}

/// Test that when get_environments is called with a child environment under the `envs` folder,
/// it also discovers the parent conda install (base environment) and all sibling environments.
/// This is the fix for https://github.com/microsoft/python-environment-tools/issues/236
/// where the base conda environment wasn't discovered when only child envs were listed
/// in environments.txt (e.g., from Homebrew Cask installs like /opt/homebrew/Caskroom/miniforge/base).
#[cfg(unix)]
#[test]
fn list_conda_envs_discovers_base_from_child_env() {
    use common::resolve_test_path;
    use pet_conda::environment_locations::get_environments;

    // Call get_environments with a child environment path (not the install directory)
    let child_env_path = resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "myenv"]);

    let mut locations = get_environments(&child_env_path);
    locations.sort();

    // Should discover not only the child env, but also the base env (conda install dir)
    // and all sibling environments
    assert_eq!(
        locations,
        vec![
            resolve_test_path(&["unix", "anaconda3-2023.03"]),
            resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "env_python_3"]),
            resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "myenv"]),
            resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "without_python"]),
        ]
    );
}

/// Test that get_environments works correctly with an env_python_3 child environment
/// (another sibling to verify the fix works for any child env under envs folder).
#[cfg(unix)]
#[test]
fn list_conda_envs_discovers_base_from_another_child_env() {
    use common::resolve_test_path;
    use pet_conda::environment_locations::get_environments;

    // Call get_environments with a different child environment path
    let child_env_path = resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "env_python_3"]);

    let mut locations = get_environments(&child_env_path);
    locations.sort();

    // Should discover the base env and all sibling environments
    assert_eq!(
        locations,
        vec![
            resolve_test_path(&["unix", "anaconda3-2023.03"]),
            resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "env_python_3"]),
            resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "myenv"]),
            resolve_test_path(&["unix", "anaconda3-2023.03", "envs", "without_python"]),
        ]
    );
}

/// Test that get_known_conda_install_locations discovers conda installations from PATH
/// when no explicit conda_executable is provided. This is important for discovering
/// conda installations on mapped drives and other non-standard locations.
/// Fixes https://github.com/microsoft/python-environment-tools/issues/194
#[cfg(unix)]
#[test]
fn discovers_conda_install_from_path() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::env_variables::EnvVariables;
    use pet_conda::environment_locations::get_known_conda_install_locations;
    use std::collections::HashMap;

    // Set up PATH to include the conda bin directory (simulating conda on a mapped drive)
    let anaconda_bin = resolve_test_path(&["unix", "anaconda3-2023.03", "bin"]);
    let path_value = anaconda_bin.to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("PATH".to_string(), path_value);

    let env = create_test_environment(vars, None, vec![], None);
    let env_vars = EnvVariables::from(&env);

    // Call get_known_conda_install_locations without an explicit conda_executable
    let locations = get_known_conda_install_locations(&env_vars, &None);

    // The anaconda3-2023.03 install should be discovered from PATH
    let expected_conda_install = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    assert!(
        locations.contains(&expected_conda_install),
        "Expected {:?} to be in {:?}",
        expected_conda_install,
        locations
    );
}

/// Test that get_known_conda_install_locations discovers conda installations from condabin in PATH.
/// This simulates the typical Windows Miniforge/Anaconda setup where condabin is added to PATH.
/// Fixes https://github.com/microsoft/python-environment-tools/issues/194
#[cfg(unix)]
#[test]
fn discovers_conda_install_from_condabin_in_path() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::env_variables::EnvVariables;
    use pet_conda::environment_locations::get_known_conda_install_locations;
    use std::collections::HashMap;

    // Set up PATH to include the condabin directory (typical Miniforge/Anaconda setup on Windows)
    let anaconda_condabin = resolve_test_path(&["unix", "anaconda3-2023.03", "condabin"]);
    let path_value = anaconda_condabin.to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("PATH".to_string(), path_value);

    let env = create_test_environment(vars, None, vec![], None);
    let env_vars = EnvVariables::from(&env);

    // Call get_known_conda_install_locations without an explicit conda_executable
    let locations = get_known_conda_install_locations(&env_vars, &None);

    // The anaconda3-2023.03 install should be discovered from PATH via condabin
    let expected_conda_install = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    assert!(
        locations.contains(&expected_conda_install),
        "Expected {:?} to be in {:?}",
        expected_conda_install,
        locations
    );
}

/// Test that when an explicit conda_executable is provided, PATH lookup is skipped.
/// This ensures we don't do unnecessary work when the user has configured a conda path.
#[cfg(unix)]
#[test]
fn skips_path_lookup_when_conda_executable_provided() {
    use common::{create_test_environment, resolve_test_path};
    use pet_conda::env_variables::EnvVariables;
    use pet_conda::environment_locations::get_known_conda_install_locations;
    use std::collections::HashMap;

    // Set up PATH to include a conda directory
    let anaconda_bin = resolve_test_path(&["unix", "anaconda3-2023.03", "bin"]);
    let path_value = anaconda_bin.to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("PATH".to_string(), path_value);

    let env = create_test_environment(vars, None, vec![], None);
    let env_vars = EnvVariables::from(&env);

    // Provide an explicit conda_executable
    let conda_executable = Some(resolve_test_path(&[
        "unix",
        "anaconda3-2023.03",
        "bin",
        "conda",
    ]));

    // Call get_known_conda_install_locations with an explicit conda_executable
    let locations = get_known_conda_install_locations(&env_vars, &conda_executable);

    // The conda install should still be discovered (from the explicit path, not PATH)
    let expected_conda_install = resolve_test_path(&["unix", "anaconda3-2023.03"]);
    assert!(
        locations.contains(&expected_conda_install),
        "Expected {:?} to be in {:?}",
        expected_conda_install,
        locations
    );
}
