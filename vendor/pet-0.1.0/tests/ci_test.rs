// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{path::PathBuf, sync::Once};

use common::{does_version_match, resolve_test_path};
use lazy_static::lazy_static;
use log::{error, trace, warn};
use pet::{
    find::identify_python_executables_using_locators,
    locators::identify_python_environment_using_locators, resolve::resolve_environment,
};
use pet_core::{
    arch::Architecture,
    env::PythonEnv,
    python_environment::{PythonEnvironment, PythonEnvironmentKind},
};
use pet_env_var_path::get_search_paths_from_env_variables;
use pet_poetry::Poetry;
use pet_reporter::{cache::CacheReporter, collect};
use regex::Regex;
use serde::Deserialize;

lazy_static! {
    static ref PYTHON_VERSION: Regex = Regex::new("([\\d+\\.?]*).*")
        .expect("error parsing Version regex for Python Version in test");
}

static INIT: Once = Once::new();

/// Setup function that is only run once, even if called multiple times.
fn setup() {
    INIT.call_once(|| {
        env_logger::builder()
            .filter(None, log::LevelFilter::Trace)
            .init();
    });
}

mod common;

#[cfg_attr(
    any(
        feature = "ci",
        feature = "ci-jupyter-container",
        feature = "ci-homebrew-container",
        feature = "ci-poetry-global",
        feature = "ci-poetry-project",
        feature = "ci-poetry-custom",
    ),
    test
)]
#[allow(dead_code)]
/// Verification 1
/// For each discovered enviornment verify the accuracy of sys.prefix and sys.version
/// by spawning the Python executable
/// Verification 2:
/// For each enviornment, given the executable verify we can get the exact same information
/// Using the `locator.try_from` method (without having to find all environments).
/// I.e. we should be able to get the same information using only the executable.
/// Verification 3:
/// Similarly for each environment use one of the known symlinks and verify we can get the same information.
/// Verification 4 & 5:
/// Similarly for each environment use resolve method and verify we get the exact same information.
fn verify_validity_of_discovered_envs() {
    use pet::{find::find_and_report_envs, locators::create_locators};
    use pet_conda::Conda;
    use pet_core::{os_environment::EnvironmentApi, Configuration};
    use std::{env, sync::Arc};

    setup();

    let workspace_dir = PathBuf::from(env::var("GITHUB_WORKSPACE").unwrap_or_default());
    let reporter = Arc::new(collect::create_reporter());
    let environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&environment));
    let poetry_locator = Arc::new(Poetry::from(&environment));
    let config = Configuration {
        workspace_directories: Some(vec![workspace_dir.clone()]),
        ..Default::default()
    };
    let locators = create_locators(conda_locator.clone(), poetry_locator.clone(), &environment);
    for locator in locators.iter() {
        locator.configure(&config);
    }

    // Find all environments on this machine.
    find_and_report_envs(
        &CacheReporter::new(reporter.clone()),
        Default::default(),
        &locators,
        &environment,
        None,
        None,
    );

    let environments = reporter.environments.lock().unwrap().clone();
    // let mut threads = vec![];
    for environment in environments {
        if environment.executable.is_none() {
            continue;
        }
        // Verification 1
        // For each enviornment verify the accuracy of sys.prefix and sys.version
        // by spawning the Python executable
        let e = environment.clone();
        // threads.push(thread::spawn(move || {
        verify_validity_of_interpreter_info(e);
        // }));
        let e = environment.clone();
        // threads.push(thread::spawn(move || {
        for exe in &e.clone().symlinks.unwrap_or_default() {
            // Verification 2:
            // For each enviornment, given the executable verify we can get the exact same information
            // Using the `locator.try_from` method (without having to find all environments).
            // I.e. we should be able to get the same information using only the executable.
            //
            // Verification 3:
            // Similarly for each environment use one of the known symlinks and verify we can get the same information.
            verify_we_can_get_same_env_info_using_from_with_exe(exe, environment.clone());
            // Verification 4 & 5:
            // Similarly for each environment use resolve method and verify we get the exact same information.
            verify_we_can_get_same_env_info_using_resolve_with_exe(exe, environment.clone());
            // Verification 6:
            // Given the exe, verify we can use the `find` method in JSON RPC to get the details, without spawning Python.
            verify_we_can_get_same_env_info_using_find_with_exe(exe, environment.clone());
        }
        // }));
    }
    // for thread in threads {
    //     thread.join().unwrap();
    // }
}

#[cfg(unix)]
#[cfg(target_os = "linux")]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// On linux we create a virtualenvwrapper environment named `venv_wrapper_env1`
fn check_if_virtualenvwrapper_exists() {
    use pet::{find::find_and_report_envs, locators::create_locators};
    use pet_conda::Conda;
    use pet_core::os_environment::EnvironmentApi;
    use std::sync::Arc;

    setup();
    let reporter = Arc::new(collect::create_reporter());
    let environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&environment));
    let poetry_locator = Arc::new(Poetry::from(&environment));

    find_and_report_envs(
        &CacheReporter::new(reporter.clone()),
        Default::default(),
        &create_locators(conda_locator.clone(), poetry_locator.clone(), &environment),
        &environment,
        None,
        None,
    );

    let environments = reporter.environments.lock().unwrap().clone();

    assert!(
        environments.iter().any(
            |env| env.kind == Some(PythonEnvironmentKind::VirtualEnvWrapper)
                && env.executable.is_some()
                && env.prefix.is_some()
                && env.name == Some("venv_wrapper_env1".to_string())
                && env
                    .executable
                    .clone()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .contains("venv_wrapper_env1")
        ),
        "Virtualenvwrapper environment not found, found: {environments:?}"
    );
}

#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
fn check_if_pipenv_exists() {
    use pet::{find::find_and_report_envs, locators::create_locators};
    use pet_conda::Conda;
    use pet_core::os_environment::EnvironmentApi;
    use std::{env, sync::Arc};

    setup();
    let reporter = Arc::new(collect::create_reporter());
    let environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&environment));
    let poetry_locator = Arc::new(Poetry::from(&environment));

    find_and_report_envs(
        &CacheReporter::new(reporter.clone()),
        Default::default(),
        &create_locators(conda_locator.clone(), poetry_locator.clone(), &environment),
        &environment,
        None,
        None,
    );

    let environments = reporter.environments.lock().unwrap().clone();

    let workspace_dir = PathBuf::from(env::var("GITHUB_WORKSPACE").unwrap_or_default());
    environments
        .iter()
        .find(|env| {
            env.kind == Some(PythonEnvironmentKind::Pipenv)
                && env.project == Some(workspace_dir.clone())
        })
        .unwrap_or_else(|| panic!("Pipenv environment not found, found {environments:?}"));
}

#[cfg(unix)]
#[cfg(target_os = "linux")]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// On linux we create a virtualenvwrapper environment named `venv_wrapper_env1`
fn check_if_pyenv_virtualenv_exists() {
    use pet::{find::find_and_report_envs, locators::create_locators};
    use pet_conda::Conda;
    use pet_core::os_environment::EnvironmentApi;
    use std::sync::Arc;

    setup();
    let reporter = Arc::new(collect::create_reporter());
    let environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&environment));
    let poetry_locator = Arc::new(Poetry::from(&environment));
    trace!("Checking for pyenv-virtualenv");

    find_and_report_envs(
        &CacheReporter::new(reporter.clone()),
        Default::default(),
        &create_locators(conda_locator.clone(), poetry_locator.clone(), &environment),
        &environment,
        None,
        None,
    );

    let environments = reporter.environments.lock().unwrap().clone();

    assert!(
        environments.iter().any(
            |env| env.kind == Some(PythonEnvironmentKind::PyenvVirtualEnv)
                && env.executable.is_some()
                && env.prefix.is_some()
                && env.manager.is_some()
                && env
                    .executable
                    .clone()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .contains("pyenv-virtualenv-env1")
        ),
        "pyenv-virtualenv environment not found, found: {environments:?}"
    );
}

fn verify_validity_of_interpreter_info(environment: PythonEnvironment) {
    let run_command = get_python_run_command(&environment);
    let interpreter_info = match get_python_interpreter_info(&run_command) {
        Some(info) => info,
        None => {
            // Conda base environments on CI runners can return empty output due to
            // activation issues. Skip gracefully for Conda; fail for everything else.
            if environment.kind == Some(PythonEnvironmentKind::Conda) {
                warn!(
                    "Skipping interpreter info validation for Conda env {:?} (command returned no output)",
                    environment
                );
                return;
            }
            panic!(
                "Failed to get interpreter info for {:?} (command returned no output)",
                environment
            );
        }
    };

    // Home brew has too many syminks, unfortunately its not easy to test in CI.
    if environment.kind != Some(PythonEnvironmentKind::Homebrew) {
        let expected_executable = environment.executable.clone().unwrap();

        // Ensure the executable is in one of the identified symlinks
        assert!(
            environment
                .symlinks
                .clone()
                .unwrap_or_default()
                .contains(&expected_executable),
            "Executable mismatch for {:?}",
            environment.clone()
        );
    }
    // If this is a conda env, then the manager, prefix and a few things must exist.
    if environment.kind == Some(PythonEnvironmentKind::Conda) {
        assert!(environment.manager.is_some());
        assert!(environment.prefix.is_some());
        if environment.executable.is_some() {
            // Version must exist in this case.
            assert!(environment.version.is_some());
        }
    }
    if let Some(prefix) = environment.clone().prefix {
        if (interpreter_info.clone().executable == "/usr/local/python/current/bin/python"
            && (prefix.to_str().unwrap() == "/usr/local/python/current"
                && interpreter_info.clone().sys_prefix == "/usr/local/python/3.10.13")
            || (prefix.to_str().unwrap() == "/usr/local/python/3.10.13"
                && interpreter_info.clone().sys_prefix == "/usr/local/python/current"))
            || (interpreter_info.clone().executable == "/home/codespace/.python/current/bin/python"
                && (prefix.to_str().unwrap() == "/home/codespace/.python/current"
                    && interpreter_info.clone().sys_prefix == "/usr/local/python/3.10.13")
                || (prefix.to_str().unwrap() == "/usr/local/python/3.10.13"
                    && interpreter_info.clone().sys_prefix == "/home/codespace/.python/current"))
        {
            // known issue https://github.com/microsoft/python-environment-tools/issues/64
        } else {
            assert_eq!(
                prefix.to_str().unwrap(),
                interpreter_info.clone().sys_prefix,
                "Prefix mismatch for {:?}",
                environment.clone()
            );
        }
    }
    if let Some(arch) = environment.clone().arch {
        let expected_arch = if interpreter_info.clone().is64_bit {
            Architecture::X64
        } else {
            Architecture::X86
        };
        assert_eq!(
            arch,
            expected_arch,
            "Architecture mismatch for {:?}",
            environment.clone()
        );
    }
    if let Some(version) = environment.clone().version {
        let expected_version = &interpreter_info.clone().sys_version;
        if !does_version_match(&version, expected_version) {
            // For pyenv environments, the version may come from the directory name
            // which can be out of sync with the binary during CPython version transitions
            // (e.g. pyenv installs pre-built binaries without header files).
            // Verify at least major.minor matches.
            // See https://github.com/microsoft/python-environment-tools/issues/371
            if environment.kind == Some(PythonEnvironmentKind::Pyenv) {
                let parts: Vec<&str> = version.splitn(3, '.').collect();
                let major_minor = if parts.len() >= 2 {
                    format!("{}.{}", parts[0], parts[1])
                } else {
                    version.clone()
                };
                assert!(
                    expected_version.starts_with(&major_minor),
                    "Version mismatch (even major.minor differs) for (expected {:?} to start with {:?}) for {:?}",
                    expected_version,
                    major_minor,
                    environment.clone()
                );
                warn!(
                    "Pyenv patch version mismatch: detected={:?}, actual starts with {:?}",
                    version, expected_version
                );
            } else {
                panic!(
                    "Version mismatch for (expected {:?} to start with {:?}) for {:?}",
                    expected_version,
                    version,
                    environment.clone()
                );
            }
        }
    }
}

fn verify_we_can_get_same_env_info_using_from_with_exe(
    executable: &PathBuf,
    environment: PythonEnvironment,
) {
    // Assume we were given a path to the exe, then we use the `locator.try_from` method.
    // We should be able to get the exct same information back given only the exe.
    //
    // Note: We will not not use the old locator objects, as we do not want any cached information.
    // Hence create the locators all over again.
    use pet::locators::create_locators;
    use pet_conda::Conda;
    use pet_core::{os_environment::EnvironmentApi, Configuration};
    use std::{env, sync::Arc};

    let workspace_dir = PathBuf::from(env::var("GITHUB_WORKSPACE").unwrap_or_default());
    let os_environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&os_environment));
    let poetry_locator = Arc::new(Poetry::from(&os_environment));
    let mut config = Configuration::default();
    let search_paths = vec![workspace_dir.clone()];
    config.workspace_directories = Some(search_paths.clone());
    let locators = create_locators(
        conda_locator.clone(),
        poetry_locator.clone(),
        &os_environment,
    );
    for locator in locators.iter() {
        locator.configure(&config);
    }
    let global_env_search_paths: Vec<PathBuf> =
        get_search_paths_from_env_variables(&os_environment);

    let env = PythonEnv::new(executable.clone(), None, None);
    let resolved =
        identify_python_environment_using_locators(&env, &locators, &global_env_search_paths)
            .unwrap_or_else(|| {
                panic!("Failed to resolve environment using `resolve` for {environment:?}")
            });
    trace!(
        "For exe {:?} we got Environment = {:?}, To compare against {:?}",
        executable,
        resolved,
        environment
    );

    compare_environments(
        resolved,
        environment,
        format!("try_from using exe {executable:?}").as_str(),
    );
}

fn verify_we_can_get_same_env_info_using_find_with_exe(
    executable: &PathBuf,
    environment: PythonEnvironment,
) {
    // Assume we were given a path to the exe, then we use the `locator.try_from` method.
    // We should be able to get the exct same information back given only the exe.
    //
    // Note: We will not not use the old locator objects, as we do not want any cached information.
    // Hence create the locators all over again.
    use pet::locators::create_locators;
    use pet_conda::Conda;
    use pet_core::{os_environment::EnvironmentApi, Configuration};
    use std::{env, sync::Arc};

    let workspace_dir = PathBuf::from(env::var("GITHUB_WORKSPACE").unwrap_or_default());
    let os_environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&os_environment));
    let poetry_locator = Arc::new(Poetry::from(&os_environment));
    let mut config = Configuration::default();
    let search_paths = vec![workspace_dir.clone()];
    config.workspace_directories = Some(search_paths.clone());
    let locators = create_locators(
        conda_locator.clone(),
        poetry_locator.clone(),
        &os_environment,
    );
    for locator in locators.iter() {
        locator.configure(&config);
    }
    let global_env_search_paths: Vec<PathBuf> =
        get_search_paths_from_env_variables(&os_environment);

    let collect_reporter = Arc::new(collect::create_reporter());
    let reporter = CacheReporter::new(collect_reporter.clone());
    identify_python_executables_using_locators(
        vec![executable.clone()],
        &locators,
        &reporter,
        &global_env_search_paths,
    );

    let envs = collect_reporter.environments.lock().unwrap().clone();
    if envs.is_empty() {
        panic!("Failed to find Python environment {executable:?}, details => {environment:?}");
    }
    trace!(
        "For exe {:?} we got Environment = {:?}, To compare against {:?}",
        executable,
        envs[0],
        environment
    );

    compare_environments(
        envs[0].clone(),
        environment,
        format!("find using exe {executable:?}").as_str(),
    );
}

fn compare_environments(actual: PythonEnvironment, expected: PythonEnvironment, method: &str) {
    let mut actual = actual.clone();
    let mut expected = expected.clone();

    assert_eq!(
        actual.kind,
        expected.clone().kind,
        "Category mismatch when using {method} for {expected:?} and {actual:?}"
    );

    // if env.kind != environment.clone().kind {
    //     error!(
    //         "Category mismatch when using {} for {:?} and {:?}",
    //         method, environment, env
    //     );
    // }

    if let (Some(version), Some(expected_version)) =
        (expected.clone().version, actual.clone().version)
    {
        if !does_version_match(&version, &expected_version) {
            // Pyenv directory name can differ from actual binary version at patch level.
            // See https://github.com/microsoft/python-environment-tools/issues/371
            if expected.kind == Some(PythonEnvironmentKind::Pyenv) {
                let parts: Vec<&str> = version.splitn(3, '.').collect();
                let major_minor = if parts.len() >= 2 {
                    format!("{}.{}", parts[0], parts[1])
                } else {
                    version.clone()
                };
                assert!(
                    expected_version.starts_with(&major_minor),
                    "Version mismatch (even major.minor differs) when using {} for (expected {:?} to start with {:?}) for env = {:?} and environment = {:?}",
                    method,
                    expected_version,
                    major_minor,
                    actual.clone(),
                    expected.clone()
                );
            } else {
                panic!(
                    "Version mismatch when using {} for (expected {:?} to start with {:?}) for env = {:?} and environment = {:?}",
                    method,
                    expected_version,
                    version,
                    actual.clone(),
                    expected.clone()
                );
            }
        }
        // if !does_version_match(&version, &expected_version) {
        //     error!("Version mismatch when using {} for (expected {:?} to start with {:?}) for env = {:?} and environment = {:?}",
        //     method,
        //     expected_version,
        //     version,
        //     env.clone(),
        //     environment.clone()
        //     );
        // }
    }
    // We have compared the versions, now ensure they are treated as the same
    // So that we can compare the objects easily
    actual.version = expected.clone().version;

    if let Some(prefix) = expected.clone().prefix {
        if (actual.clone().executable
            == Some(PathBuf::from("/usr/local/python/current/bin/python"))
            && (prefix.to_str().unwrap() == "/usr/local/python/current"
                && actual.clone().prefix == Some(PathBuf::from("/usr/local/python/3.10.13")))
            || (prefix.to_str().unwrap() == "/usr/local/python/3.10.13"
                && actual.clone().prefix == Some(PathBuf::from("/usr/local/python/current"))))
            || (actual.clone().executable
                == Some(PathBuf::from("/home/codespace/.python/current/bin/python"))
                && (prefix.to_str().unwrap() == "/home/codespace/.python/current"
                    && actual.clone().prefix == Some(PathBuf::from("/usr/local/python/3.10.13")))
                || (prefix.to_str().unwrap() == "/usr/local/python/3.10.13"
                    && actual.clone().prefix
                        == Some(PathBuf::from("/home/codespace/.python/current"))))
        {
            // known issue https://github.com/microsoft/python-environment-tools/issues/64
            actual.prefix = expected.clone().prefix;
        }
    }
    // known issue
    actual.symlinks = Some(
        actual
            .clone()
            .symlinks
            .unwrap_or_default()
            .iter()
            .filter(|p| {
                // This is in the path, but not easy to figure out, unless we add support for codespaces or CI.
                !(p.starts_with("/Users/runner/hostedtoolcache/Python")
                    && p.to_string_lossy().contains("arm64"))
            })
            .map(|p| p.to_path_buf())
            .collect::<Vec<PathBuf>>(),
    );
    expected.symlinks = Some(
        expected
            .clone()
            .symlinks
            .unwrap_or_default()
            .iter()
            .filter(|p| {
                // This is in the path, but not easy to figure out, unless we add support for codespaces or CI.
                !(p.starts_with("/Users/runner/hostedtoolcache/Python")
                    && p.to_string_lossy().contains("arm64"))
            })
            .map(|p| p.to_path_buf())
            .collect::<Vec<PathBuf>>(),
    );

    // if we know the arch, then verify it
    if expected.arch.as_ref().is_some()
        && actual.arch.as_ref().is_some()
        && actual.arch.as_ref() != expected.arch.as_ref()
    {
        error!(
            "Arch mismatch when using {} for {:?} and {:?}",
            method, expected, actual
        );
    }
    actual.arch = expected.clone().arch;

    // if we know the prefix, then verify it
    if expected.prefix.as_ref().is_some()
        && actual.prefix.as_ref().is_some()
        && actual.prefix.as_ref() != expected.prefix.as_ref()
    {
        error!(
            "Prefirx mismatch when using {} for {:?} and {:?}",
            method, expected, actual
        );
    }
    actual.prefix = expected.clone().prefix;

    assert_eq!(
        actual, expected,
        "Environment mismatch when using {method} for {expected:?}"
    );

    // if env != environment {
    //     error!(
    //         "Environment mismatch when using {} for {:?} and {:?}",
    //         method, environment, env
    //     );
    // }
}

fn verify_we_can_get_same_env_info_using_resolve_with_exe(
    executable: &PathBuf,
    environment: PythonEnvironment,
) {
    // Assume we were given a path to the exe, then we use the `locator.try_from` method.
    // We should be able to get the exct same information back given only the exe.
    //
    // Note: We will not not use the old locator objects, as we do not want any cached information.
    // Hence create the locators all over again.
    use pet::locators::create_locators;
    use pet_conda::Conda;
    use pet_core::{os_environment::EnvironmentApi, Configuration};
    use std::{env, sync::Arc};

    let workspace_dir = PathBuf::from(env::var("GITHUB_WORKSPACE").unwrap_or_default());
    let os_environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&os_environment));
    let poetry_locator = Arc::new(Poetry::from(&os_environment));
    let config = Configuration {
        workspace_directories: Some(vec![workspace_dir.clone()]),
        ..Default::default()
    };
    let locators = create_locators(
        conda_locator.clone(),
        poetry_locator.clone(),
        &os_environment,
    );
    for locator in locators.iter() {
        locator.configure(&config);
    }

    let env = resolve_environment(executable, &locators, &os_environment).unwrap_or_else(|| {
        panic!("Failed to resolve environment using `resolve` for {environment:?}")
    });
    trace!(
        "For exe {:?} we got Environment = {:?}, To compare against {:?}",
        executable,
        env,
        environment
    );
    if env.resolved.is_none() {
        error!(
            "Failed to resolve environment using `resolve` for {:?} in {:?}",
            executable, environment
        );
        return;
    }
    compare_environments(
        env.resolved.unwrap(),
        environment,
        format!("resolve using exe {executable:?}").as_str(),
    );
}

#[cfg(unix)]
#[cfg(target_os = "linux")]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// On linux we /bin/python, /usr/bin/python and /usr/local/python are all separate environments.
fn verify_bin_usr_bin_user_local_are_separate_python_envs() {
    use pet::{find::find_and_report_envs, locators::create_locators};
    use pet_conda::Conda;
    use pet_core::os_environment::EnvironmentApi;
    use std::sync::Arc;

    setup();
    let reporter = Arc::new(collect::create_reporter());
    let environment = EnvironmentApi::new();
    let conda_locator = Arc::new(Conda::from(&environment));
    let poetry_locator = Arc::new(Poetry::from(&environment));

    find_and_report_envs(
        &CacheReporter::new(reporter.clone()),
        Default::default(),
        &create_locators(conda_locator.clone(), poetry_locator.clone(), &environment),
        &environment,
        None,
        None,
    );

    let environments = reporter.environments.lock().unwrap().clone();

    // Python env /bin/python cannot have symlinks in /usr/bin or /usr/local
    // Python env /usr/bin/python cannot have symlinks /bin or /usr/local
    // Python env /usr/local/bin/python cannot have symlinks in /bin or /usr/bin
    let bins = ["/bin", "/usr/bin", "/usr/local/bin"];
    for bin in bins.iter() {
        if let Some(bin_python) = environments.iter().find(|e| {
            e.executable.clone().is_some()
                && e.executable
                    .clone()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .starts_with(bin)
        }) {
            // If the exe is in /bin, then we can never have any symlinks to other folders such as /usr/bin or /usr/local
            let other_bins = bins
                .iter()
                .filter(|b| *b != bin)
                .map(|b| PathBuf::from(*b))
                .collect::<Vec<PathBuf>>();
            if let Some(symlinks) = &bin_python.symlinks {
                for symlink in symlinks.iter() {
                    let parent_of_symlink = symlink.parent().unwrap().to_path_buf();
                    if other_bins.contains(&parent_of_symlink) {
                        panic!(
                            "Python environment {bin_python:?} cannot have a symlinks in {other_bins:?}"
                        );
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn get_conda_exe() -> &'static str {
    // On CI we expect conda to be in the current path.
    "conda"
}

#[derive(Deserialize, Clone)]
struct InterpreterInfo {
    sys_prefix: String,
    #[allow(dead_code)]
    executable: String,
    sys_version: String,
    is64_bit: bool,
    // version_info: (u16, u16, u16, String, u16),
}

fn get_python_run_command(env: &PythonEnvironment) -> Vec<String> {
    if env.clone().kind == Some(PythonEnvironmentKind::Conda) {
        if env.executable.is_none() {
            panic!("Conda environment without executable");
        }
        let conda_exe = match env.manager.clone() {
            Some(manager) => manager.executable.to_str().unwrap_or_default().to_string(),
            None => get_conda_exe().to_string(),
        };
        if let Some(name) = env.name.clone() {
            vec![
                conda_exe,
                "run".to_string(),
                "-n".to_string(),
                name,
                "python".to_string(),
            ]
        } else if let Some(prefix) = env.prefix.clone() {
            vec![
                conda_exe,
                "run".to_string(),
                "-p".to_string(),
                prefix.to_str().unwrap_or_default().to_string(),
                "python".to_string(),
            ]
        } else {
            panic!("Conda environment without name or prefix")
        }
    } else {
        vec![env
            .executable
            .clone()
            .expect("Python environment without executable")
            .to_str()
            .unwrap()
            .to_string()]
    }
}

fn get_python_interpreter_info(cli: &[String]) -> Option<InterpreterInfo> {
    let mut cli = cli.to_owned();
    cli.push(
        resolve_test_path(&["interpreterInfo.py"])
            .to_str()
            .unwrap_or_default()
            .to_string(),
    );
    let output = std::process::Command::new(cli.first().expect("empty cli"))
        .args(&cli[1..])
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute command {cli:?}"));
    let exit_status = output.status;
    let stdout = String::from_utf8(output.stdout).unwrap_or_default();
    let stderr = String::from_utf8(output.stderr).unwrap_or_default();
    trace!("Get Interpreter Info: {:?} => {:?}", cli, stdout);
    if let Some((_, json_part)) = stdout.split_once("503bebe7-c838-4cea-a1bc-0f2963bcb657") {
        match serde_json::from_str(json_part) {
            Ok(info) => Some(info),
            Err(e) => {
                warn!(
                    "Failed to parse interpreter info for {cli:?}: {e}, exit: {exit_status}, stdout: {stdout:?}, stderr: {stderr:?}"
                );
                None
            }
        }
    } else {
        warn!(
            "Failed to get interpreter info for {cli:?}: marker not found in output, exit: {exit_status}, stdout: {stdout:?}, stderr: {stderr:?}"
        );
        None
    }
}
