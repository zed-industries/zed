// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

use env_variables::EnvVariables;
use environments::{get_generic_python_environment, get_virtual_env_environment};
use log::trace;
use manager::PyEnvInfo;
use pet_conda::{utils::is_conda_env, CondaLocator};
use pet_core::{
    env::PythonEnv,
    manager::{EnvManager, EnvManagerType},
    os_environment::Environment,
    python_environment::{PythonEnvironment, PythonEnvironmentKind},
    reporter::Reporter,
    Locator, LocatorKind, RefreshStatePersistence,
};
use pet_python_utils::executable::find_executable;

pub mod env_variables;
mod environment_locations;
mod environments;
mod manager;

pub struct PyEnv {
    pub env_vars: EnvVariables,
    pub conda_locator: Arc<dyn CondaLocator>,
    manager: Arc<Mutex<Option<EnvManager>>>,
    versions_dir: Arc<Mutex<Option<PathBuf>>>,
}

impl PyEnv {
    pub fn from(
        environment: &dyn Environment,
        conda_locator: Arc<dyn CondaLocator>,
    ) -> impl Locator {
        PyEnv {
            env_vars: EnvVariables::from(environment),
            conda_locator,
            manager: Arc::new(Mutex::new(None)),
            versions_dir: Arc::new(Mutex::new(None)),
        }
    }
    fn clear(&self) {
        self.manager.lock().expect("manager mutex poisoned").take();
        self.versions_dir
            .lock()
            .expect("versions_dir mutex poisoned")
            .take();
    }
    fn get_manager_versions_dir(&self) -> (Option<EnvManager>, Option<PathBuf>) {
        let mut managers = self.manager.lock().expect("manager mutex poisoned");
        let mut versions = self
            .versions_dir
            .lock()
            .expect("versions_dir mutex poisoned");
        if managers.is_none() || versions.is_none() {
            let pyenv_info = PyEnvInfo::from(&self.env_vars);
            trace!("PyEnv Info {:?}", pyenv_info);
            if let Some(ref exe) = pyenv_info.exe {
                let version = pyenv_info.version.clone();
                let manager = EnvManager::new(exe.clone(), EnvManagerType::Pyenv, version);
                managers.replace(manager);
            } else {
                managers.take();
            }
            if let Some(version_path) = &pyenv_info.versions {
                versions.replace(version_path.clone());
            } else {
                versions.take();
            }
        }
        (managers.clone(), versions.clone())
    }
}

impl Locator for PyEnv {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::PyEnv
    }
    fn refresh_state(&self) -> RefreshStatePersistence {
        RefreshStatePersistence::SelfHydratingCache
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![
            PythonEnvironmentKind::Pyenv,
            PythonEnvironmentKind::PyenvVirtualEnv,
            PythonEnvironmentKind::Conda,
        ]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if let Some(prefix) = &env.prefix {
            if is_conda_env(prefix) {
                return None;
            }
        }
        // Possible this is a root conda env (hence parent directory is conda install dir).
        if is_conda_env(env.executable.parent()?) {
            return None;
        }
        // Possible this is a conda env (hence parent directory is Scripts/bin dir).
        if is_conda_env(env.executable.parent()?.parent()?) {
            return None;
        }

        // Env path must exists,
        // If exe is Scripts/python.exe or bin/python.exe
        // Then env path is parent of Scripts or bin
        // & in pyenv case thats a directory inside `versions` folder.

        let (manager, versions) = self.get_manager_versions_dir();
        if let Some(versions) = versions {
            if env.executable.starts_with(versions) {
                let env_path = env.prefix.clone()?;
                if let Some(env) = get_virtual_env_environment(&env.executable, &env_path, &manager)
                {
                    return Some(env);
                } else if let Some(env) =
                    get_generic_python_environment(&env.executable, &env_path, &manager)
                {
                    return Some(env);
                }
            }
        }
        None
    }

    fn find(&self, reporter: &dyn Reporter) {
        self.clear();

        let (manager, versions) = self.get_manager_versions_dir();

        if let Some(manager) = &manager {
            reporter.report_manager(manager);
        }

        if let Some(versions) = versions {
            let conda_locator = self.conda_locator.clone();
            thread::scope(|s| {
                if let Ok(reader) = fs::read_dir(versions) {
                    for path in reader.filter_map(Result::ok).map(|e| e.path()) {
                        let conda_locator = conda_locator.clone();
                        let manager = manager.clone();
                        let path = path.clone();
                        s.spawn(move || {
                            if let Some(executable) = find_executable(&path) {
                                if is_conda_env(&path) {
                                    conda_locator.find_and_report(reporter, &path);
                                } else if let Some(env) =
                                    get_virtual_env_environment(&executable, &path, &manager)
                                {
                                    reporter.report_environment(&env)
                                } else if let Some(env) =
                                    get_generic_python_environment(&executable, &path, &manager)
                                {
                                    reporter.report_environment(&env)
                                }
                            }
                        });
                    }
                }
            });
        } else {
            trace!("PyEnv versions directory not found");
        }
    }
}
