// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::sync::Once;

use pet_poetry::Poetry;
use pet_reporter::{cache::CacheReporter, collect};

mod common;

static INIT: Once = Once::new();

/// Setup function that is only run once, even if called multiple times.
fn setup() {
    INIT.call_once(|| {
        env_logger::builder()
            .filter(None, log::LevelFilter::Trace)
            .init();
    });
}

#[cfg_attr(any(feature = "ci-poetry-global", feature = "ci-poetry-custom"), test)]
#[allow(dead_code)]
/// This is a test with Poetry for current directory with Python 3.12 and 3.11 and envs are created in regular global cache directory
fn verify_ci_poetry_global() {
    use pet::{find::find_and_report_envs, locators::create_locators};
    use pet_conda::Conda;
    use pet_core::{
        manager::EnvManagerType,
        os_environment::EnvironmentApi,
        python_environment::{PythonEnvironment, PythonEnvironmentKind},
        Configuration,
    };
    use std::{env, path::PathBuf, sync::Arc};

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

    find_and_report_envs(
        &CacheReporter::new(reporter.clone()),
        Default::default(),
        &locators,
        &environment,
        None,
        None,
    );

    let environments = reporter.environments.lock().unwrap().clone();

    // On CI the poetry manager is installed using wsl, and the path isn't available on windows
    if std::env::consts::OS != "windows" {
        reporter
            .managers
            .lock()
            .unwrap()
            .clone()
            .iter()
            .find(|m| m.tool == EnvManagerType::Poetry)
            .expect("Poetry manager not found");
    }

    let poetry_envs = environments
        .iter()
        .filter(|e| {
            e.kind == Some(PythonEnvironmentKind::Poetry)
                && e.project == Some(workspace_dir.clone())
        })
        .collect::<Vec<&PythonEnvironment>>();

    assert_eq!(poetry_envs.len(), 2);

    poetry_envs
        .iter()
        .find(|e| e.version.clone().unwrap_or_default().starts_with("3.12"))
        .expect("Python 3.12 not found");
    poetry_envs
        .iter()
        .find(|e| e.version.clone().unwrap_or_default().starts_with("3.11"))
        .expect("Python 3.12 not found");
}

#[cfg_attr(feature = "ci-poetry-project", test)]
#[allow(dead_code)]
/// This is a test with Poetry for current directory with Python 3.11 and created as .venv in project directory.
fn verify_ci_poetry_project() {
    use pet::{find::find_and_report_envs, locators::create_locators};
    use pet_conda::Conda;
    use pet_core::{
        manager::EnvManagerType,
        os_environment::EnvironmentApi,
        python_environment::{PythonEnvironment, PythonEnvironmentKind},
        Configuration,
    };
    use std::{env, path::PathBuf, sync::Arc};

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

    find_and_report_envs(
        &CacheReporter::new(reporter.clone()),
        Default::default(),
        &locators,
        &environment,
        None,
        None,
    );

    let environments = reporter.environments.lock().unwrap().clone();

    // On CI the poetry manager is installed using wsl, and the path isn't available on windows
    if std::env::consts::OS != "windows" {
        reporter
            .managers
            .lock()
            .unwrap()
            .clone()
            .iter()
            .find(|m| m.tool == EnvManagerType::Poetry)
            .expect("Poetry manager not found");
    }

    let poetry_envs = environments
        .iter()
        .filter(|e| {
            e.kind == Some(PythonEnvironmentKind::Poetry)
                && e.project == Some(workspace_dir.clone())
        })
        .collect::<Vec<&PythonEnvironment>>();

    assert_eq!(poetry_envs.len(), 1);

    assert!(
        poetry_envs[0]
            .version
            .clone()
            .unwrap_or_default()
            .starts_with("3.11"),
        "Python 3.11 not found"
    );
    assert_eq!(
        poetry_envs[0].prefix.clone().unwrap_or_default(),
        workspace_dir.join(".venv")
    );
}
