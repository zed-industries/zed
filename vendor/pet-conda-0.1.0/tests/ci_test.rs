// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![cfg(unix)]

use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Once;

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

#[cfg(unix)]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// We should detect the conda install along with the base env
fn detect_conda_root() {
    use std::sync::Arc;

    use pet_conda::Conda;
    use pet_core::{
        manager::EnvManagerType, os_environment::EnvironmentApi,
        python_environment::PythonEnvironmentKind, Locator,
    };
    use pet_reporter::{cache::CacheReporter, collect};

    setup();
    let env = EnvironmentApi::new();

    let reporter = Arc::new(collect::create_reporter());
    let conda = Conda::from(&env);
    conda.find(&CacheReporter::new(reporter.clone()));

    let environments = reporter.environments.lock().unwrap().clone();
    let managers = reporter.managers.lock().unwrap().clone();

    let info = get_conda_info();
    let conda_dir = PathBuf::from(info.conda_prefix.clone());
    let manager = &managers[0];
    assert_eq!(manager.executable, conda_dir.join("bin").join("conda"));
    assert_eq!(manager.tool, EnvManagerType::Conda);
    assert_eq!(manager.version, info.conda_version.into());

    let env = &environments
        .iter()
        .find(|e| e.name == Some("base".into()))
        .unwrap();
    assert_eq!(env.prefix, conda_dir.clone().into());
    assert_eq!(env.name, Some("base".into()));
    assert_eq!(env.kind, Some(PythonEnvironmentKind::Conda));
    assert_eq!(env.executable, Some(conda_dir.join("bin").join("python")));
    assert_eq!(env.version, Some(get_version(&info.python_version)));

    assert_eq!(env.manager, Some(manager.clone()));
}

#[cfg(unix)]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// Given the path to the root directory, detect the manager and base env
fn detect_conda_root_from_path() {
    use pet_conda::Conda;
    use pet_core::{
        env::PythonEnv, manager::EnvManagerType, os_environment::EnvironmentApi,
        python_environment::PythonEnvironmentKind, Locator,
    };
    use std::path::PathBuf;

    setup();
    let env = EnvironmentApi::new();
    let info = get_conda_info();
    let conda_dir = PathBuf::from(info.conda_prefix.clone());
    let exe = conda_dir.join("bin").join("python");
    let conda = Conda::from(&env);

    let python_env = PythonEnv::new(exe, Some(conda_dir.clone()), None);
    let env = conda.try_from(&python_env).unwrap();

    assert!(env.manager.is_some());

    let manager = env.manager.unwrap();
    assert_eq!(manager.executable, conda_dir.join("bin").join("conda"));
    assert_eq!(manager.tool, EnvManagerType::Conda);
    assert_eq!(manager.version, info.conda_version.into());

    assert_eq!(env.prefix, conda_dir.clone().into());
    assert_eq!(env.name, Some("base".into()));
    assert_eq!(env.kind, Some(PythonEnvironmentKind::Conda));
    assert_eq!(env.executable, Some(conda_dir.join("bin").join("python")));
    assert_eq!(env.version, Some(get_version(&info.python_version)));
}

#[cfg(unix)]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// When a new env is created detect that too
fn detect_new_conda_env() {
    use pet_conda::Conda;
    use pet_core::{
        os_environment::EnvironmentApi, python_environment::PythonEnvironmentKind, Locator,
    };
    use pet_reporter::{cache::CacheReporter, collect};
    use std::{path::PathBuf, sync::Arc};

    setup();
    let env_name = "env_with_python";
    create_conda_env(
        &CondaCreateEnvNameOrPath::Name(env_name.into()),
        Some("3.10".into()),
    );
    let env = EnvironmentApi::new();

    let conda = Conda::from(&env);
    let reporter = Arc::new(collect::create_reporter());
    conda.find(&CacheReporter::new(reporter.clone()));

    let environments = reporter.environments.lock().unwrap().clone();
    let managers = reporter.managers.lock().unwrap().clone();

    let manager = &managers[0];

    let info = get_conda_info();
    let conda_dir = PathBuf::from(info.conda_prefix.clone());
    let env = environments
        .iter()
        .find(|x| x.name == Some(env_name.into()))
        .unwrap_or_else(|| panic!("New Environment not created, detected envs {environments:?}"));

    let prefix = conda_dir.clone().join("envs").join(env_name);
    assert_eq!(env.prefix, prefix.clone().into());
    assert_eq!(env.name, Some(env_name.into()));
    assert_eq!(env.kind, Some(PythonEnvironmentKind::Conda));
    assert_eq!(env.executable, prefix.join("bin").join("python").into());
    assert!(
        env.version.clone().unwrap_or_default().starts_with("3.10"),
        "Expected 3.10, but got Version: {:?}",
        env.version
    );

    assert_eq!(env.manager, Some(manager.clone()));
}

#[cfg(unix)]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// Identify the manager and conda env given the path to a conda env inside the `envs` directory
fn detect_conda_env_from_path() {
    use pet_conda::Conda;
    use pet_core::{
        env::PythonEnv, manager::EnvManagerType, os_environment::EnvironmentApi,
        python_environment::PythonEnvironmentKind, Locator,
    };
    use std::path::PathBuf;

    setup();
    let env = EnvironmentApi::new();
    let info = get_conda_info();
    let env_name = "env_with_python2";
    create_conda_env(
        &CondaCreateEnvNameOrPath::Name(env_name.into()),
        Some("3.10".into()),
    );
    let conda_dir = PathBuf::from(info.conda_prefix.clone());
    let prefix = conda_dir.join("envs").join(env_name);
    let exe = prefix.join("bin").join("python");
    let conda = Conda::from(&env);

    let python_env = PythonEnv::new(exe.clone(), Some(prefix.clone()), None);
    let env = conda.try_from(&python_env).unwrap();

    assert!(env.manager.is_some());

    let manager = env.manager.unwrap();
    assert_eq!(manager.executable, conda_dir.join("bin").join("conda"));
    assert_eq!(manager.tool, EnvManagerType::Conda);
    assert_eq!(manager.version, info.conda_version.into());

    assert_eq!(env.prefix, prefix.clone().into());
    assert_eq!(env.name, Some(env_name.into()));
    assert_eq!(env.kind, Some(PythonEnvironmentKind::Conda));
    assert_eq!(env.executable, exe.clone().into());
    assert!(
        env.version.clone().unwrap_or_default().starts_with("3.10"),
        "Expected 3.10, but got Version: {:?}",
        env.version
    );
}

#[cfg(unix)]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// Detect envs created without Python
fn detect_new_conda_env_without_python() {
    use pet_conda::Conda;
    use pet_core::{
        os_environment::EnvironmentApi, python_environment::PythonEnvironmentKind, Locator,
    };
    use pet_reporter::{cache::CacheReporter, collect};
    use std::{path::PathBuf, sync::Arc};

    setup();
    let env_name = "env_without_python";
    create_conda_env(&CondaCreateEnvNameOrPath::Name(env_name.into()), None);
    let env = EnvironmentApi::new();

    let conda = Conda::from(&env);
    let reporter = Arc::new(collect::create_reporter());
    conda.find(&CacheReporter::new(reporter.clone()));

    let environments = reporter.environments.lock().unwrap().clone();
    let managers = reporter.managers.lock().unwrap().clone();

    let manager = &managers[0];

    let info = get_conda_info();
    let conda_dir = PathBuf::from(info.conda_prefix.clone());
    let env = environments
        .iter()
        .find(|x| x.name == Some(env_name.into()))
        .unwrap_or_else(|| panic!("New Environment not created, detected envs {environments:?}"));

    let prefix = conda_dir.clone().join("envs").join(env_name);
    assert_eq!(env.prefix, prefix.clone().into());
    assert_eq!(env.name, Some(env_name.into()));
    assert_eq!(env.kind, Some(PythonEnvironmentKind::Conda));
    assert!(env.executable.is_none());
    assert!(env.version.is_none());

    assert_eq!(env.manager, Some(manager.clone()));
}

#[cfg(unix)]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// Detect envs created without Python in a custom directory using the -p flag
fn detect_new_conda_env_created_with_p_flag_without_python() {
    use std::sync::Arc;

    use common::resolve_test_path;
    use pet_conda::Conda;
    use pet_core::{
        os_environment::EnvironmentApi, python_environment::PythonEnvironmentKind, Locator,
    };
    use pet_reporter::{cache::CacheReporter, collect};

    setup();
    let env_name = "env_without_python3";
    let prefix = resolve_test_path(&["unix", env_name]);
    create_conda_env(&CondaCreateEnvNameOrPath::Path(prefix.clone()), None);
    let env = EnvironmentApi::new();

    let conda = Conda::from(&env);
    let reporter = Arc::new(collect::create_reporter());
    conda.find(&CacheReporter::new(reporter.clone()));

    let environments = reporter.environments.lock().unwrap().clone();
    let managers = reporter.managers.lock().unwrap().clone();

    let manager = &managers[0];

    let env = environments
        .iter()
        .find(|x| x.prefix == Some(prefix.clone()))
        .unwrap_or_else(|| {
            panic!("New Environment ({prefix:?}) not created, detected envs {environments:?}")
        });

    assert_eq!(env.prefix, prefix.clone().into());
    assert_eq!(env.name, None);
    assert_eq!(env.kind, Some(PythonEnvironmentKind::Conda));
    assert!(env.executable.is_none());
    assert!(env.version.is_none());

    assert_eq!(env.manager, Some(manager.clone()));
}

#[cfg(unix)]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// Detect envs created Python in a custom directory using the -p flag
fn detect_new_conda_env_created_with_p_flag_with_python() {
    use std::sync::Arc;

    use common::resolve_test_path;
    use pet_conda::Conda;
    use pet_core::{
        os_environment::EnvironmentApi, python_environment::PythonEnvironmentKind, Locator,
    };
    use pet_reporter::{cache::CacheReporter, collect};

    setup();
    let env_name = "env_with_python3";
    let prefix = resolve_test_path(&["unix", env_name]);
    let exe = prefix.join("bin").join("python");
    create_conda_env(
        &CondaCreateEnvNameOrPath::Path(prefix.clone()),
        Some("3.10".into()),
    );
    let env = EnvironmentApi::new();

    let conda = Conda::from(&env);
    let reporter = Arc::new(collect::create_reporter());
    conda.find(&CacheReporter::new(reporter.clone()));

    let environments = reporter.environments.lock().unwrap().clone();
    let managers = reporter.managers.lock().unwrap().clone();

    let manager = &managers[0];

    let env = environments
        .iter()
        .find(|x| x.prefix == Some(prefix.clone()))
        .unwrap_or_else(|| panic!("New Environment not created, detected envs {environments:?}"));

    assert_eq!(env.prefix, prefix.clone().into());
    assert_eq!(env.name, None);
    assert_eq!(env.kind, Some(PythonEnvironmentKind::Conda));
    assert_eq!(env.executable, exe.into());
    assert!(
        env.version.clone().unwrap_or_default().starts_with("3.10"),
        "Expected 3.10, but got Version: {:?}",
        env.version
    );

    assert_eq!(env.manager, Some(manager.clone()));
}

#[cfg(unix)]
#[cfg_attr(feature = "ci", test)]
#[allow(dead_code)]
// Install mamba into the conda base env and verify both Conda and Mamba managers are reported
fn detect_mamba_manager_alongside_conda() {
    use std::sync::Arc;

    use pet_conda::Conda;
    use pet_core::{
        manager::EnvManagerType, os_environment::EnvironmentApi,
        python_environment::PythonEnvironmentKind, Locator,
    };
    use pet_reporter::{cache::CacheReporter, collect};

    setup();

    // Install mamba into the base conda environment
    let output = std::process::Command::new(get_conda_exe())
        .args(["install", "-n", "base", "mamba", "-c", "conda-forge", "-y"])
        .output()
        .expect("Failed to install mamba");
    println!(
        "Mamba install stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "Mamba install stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.status.success(),
        "Failed to install mamba into conda base env"
    );

    let env = EnvironmentApi::new();
    let reporter = Arc::new(collect::create_reporter());
    let conda = Conda::from(&env);
    conda.find(&CacheReporter::new(reporter.clone()));

    let managers = reporter.managers.lock().unwrap().clone();

    let info = get_conda_info();
    let conda_dir = PathBuf::from(info.conda_prefix.clone());

    // Verify we have a Conda manager
    let conda_manager = managers
        .iter()
        .find(|m| m.tool == EnvManagerType::Conda)
        .unwrap_or_else(|| panic!("Conda manager not found in managers: {managers:?}"));
    assert_eq!(
        conda_manager.executable,
        conda_dir.join("bin").join("conda")
    );

    // Verify we have a Mamba manager
    let mamba_manager = managers
        .iter()
        .find(|m| m.tool == EnvManagerType::Mamba)
        .unwrap_or_else(|| panic!("Mamba manager not found in managers: {managers:?}"));
    assert_eq!(
        mamba_manager.executable,
        conda_dir.join("bin").join("mamba")
    );

    // Environments should still be reported as Conda kind
    let environments = reporter.environments.lock().unwrap().clone();
    let base_env = environments
        .iter()
        .find(|e| e.name == Some("base".into()))
        .unwrap_or_else(|| panic!("Base env not found in environments: {environments:?}"));
    assert_eq!(base_env.kind, Some(PythonEnvironmentKind::Conda));
    // The environment manager should be Conda (not Mamba)
    assert_eq!(
        base_env.manager.as_ref().map(|m| m.tool),
        Some(EnvManagerType::Conda)
    );
}

#[derive(Deserialize)]
struct CondaInfo {
    conda_version: String,
    conda_prefix: String,
    python_version: String,
    #[allow(dead_code)]
    envs: Vec<String>,
}

fn get_conda_exe() -> &'static str {
    // On CI we expect conda to be in the current path.
    "conda"
}

fn get_conda_info() -> CondaInfo {
    // Spawn `conda --version` to get the version of conda as a string
    let output = std::process::Command::new(get_conda_exe())
        .args(["info", "--json"])
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8(output.stdout).unwrap();
    let conda_info: CondaInfo = serde_json::from_str(&output).unwrap();
    conda_info
}

enum CondaCreateEnvNameOrPath {
    Name(String),
    Path(PathBuf),
}

fn create_conda_env(mode: &CondaCreateEnvNameOrPath, python_version: Option<String>) {
    let mut cli: Vec<String> = vec!["create".to_string()];
    match mode {
        CondaCreateEnvNameOrPath::Name(name) => {
            cli.push("-n".to_string());
            cli.push(name.to_string());
        }
        CondaCreateEnvNameOrPath::Path(path) => {
            cli.push("-p".to_string());
            cli.push(path.to_str().unwrap().to_string());
        }
    }
    if let Some(ref python_version) = python_version {
        cli.push(format!("python={}", python_version.as_str()));
    }
    cli.push("-y".to_string());
    // Spawn `conda --version` to get the version of conda as a string
    println!("Creating conda env with: {:?} {:?}", get_conda_exe(), cli);
    let _ = std::process::Command::new(get_conda_exe())
        .args(cli)
        .output()
        .expect("Failed to execute command");
}

fn get_version(value: &str) -> String {
    // Regex to extract just the d.d.d version from the full version string
    let re = regex::Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let captures = re.captures(value).unwrap();
    captures.get(0).unwrap().as_str().to_string()
}
