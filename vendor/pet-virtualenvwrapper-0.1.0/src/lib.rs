// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use env_variables::EnvVariables;
use environments::{get_project, is_virtualenvwrapper};
use pet_core::{
    env::PythonEnv,
    os_environment::Environment,
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentKind},
    reporter::Reporter,
    Locator, LocatorKind,
};
use pet_python_utils::executable::find_executables;
use pet_python_utils::version;

mod env_variables;
mod environment_locations;
mod environments;

pub struct VirtualEnvWrapper {
    pub env_vars: EnvVariables,
}

impl VirtualEnvWrapper {
    pub fn from(environment: &dyn Environment) -> VirtualEnvWrapper {
        VirtualEnvWrapper {
            env_vars: EnvVariables::from(environment),
        }
    }
}

impl Locator for VirtualEnvWrapper {
    fn get_kind(&self) -> LocatorKind {
        LocatorKind::VirtualEnvWrapper
    }
    fn supported_categories(&self) -> Vec<PythonEnvironmentKind> {
        vec![PythonEnvironmentKind::VirtualEnvWrapper]
    }

    fn try_from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if !is_virtualenvwrapper(env, &self.env_vars) {
            return None;
        }
        let version = match env.version {
            Some(ref v) => Some(v.clone()),
            None => match &env.prefix {
                Some(prefix) => version::from_creator_for_virtual_env(prefix),
                None => None,
            },
        };
        let mut symlinks = vec![];
        let mut name = None;
        if let Some(ref prefix) = env.prefix {
            symlinks.append(&mut find_executables(prefix));
            name = prefix.file_name().and_then(|f| f.to_str());
        }

        Some(
            PythonEnvironmentBuilder::new(Some(PythonEnvironmentKind::VirtualEnvWrapper))
                .name(name.map(String::from))
                .executable(Some(env.executable.clone()))
                .version(version)
                .prefix(env.prefix.clone())
                .project(get_project(env))
                .symlinks(Some(symlinks))
                .build(),
        )
    }

    fn find(&self, _reporter: &dyn Reporter) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    struct TestEnvironment {
        user_home: Option<PathBuf>,
        env_vars: HashMap<String, String>,
    }

    impl Environment for TestEnvironment {
        fn get_user_home(&self) -> Option<PathBuf> {
            self.user_home.clone()
        }

        fn get_root(&self) -> Option<PathBuf> {
            None
        }

        fn get_env_var(&self, key: String) -> Option<String> {
            self.env_vars.get(&key).cloned()
        }

        fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
            vec![]
        }
    }

    fn create_test_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "pet-virtualenvwrapper-{name}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).unwrap();
        directory
    }

    fn create_virtualenv(prefix: &Path) -> PathBuf {
        let scripts_dir = prefix.join(if cfg!(windows) { "Scripts" } else { "bin" });
        fs::create_dir_all(&scripts_dir).unwrap();
        fs::write(
            scripts_dir.join(if cfg!(windows) {
                "activate.bat"
            } else {
                "activate"
            }),
            b"",
        )
        .unwrap();
        let executable = scripts_dir.join(if cfg!(windows) {
            "python.exe"
        } else {
            "python"
        });
        fs::write(&executable, b"").unwrap();
        executable
    }

    #[test]
    fn virtualenvwrapper_reports_kind_and_supported_category() {
        let locator = VirtualEnvWrapper {
            env_vars: EnvVariables {
                home: None,
                workon_home: None,
            },
        };

        assert_eq!(locator.get_kind(), LocatorKind::VirtualEnvWrapper);
        assert_eq!(
            locator.supported_categories(),
            vec![PythonEnvironmentKind::VirtualEnvWrapper]
        );
    }

    #[test]
    fn from_reads_environment_variables() {
        let workon_home = create_test_dir("workon-home");
        let mut env_vars = HashMap::new();
        env_vars.insert(
            "WORKON_HOME".to_string(),
            workon_home.to_string_lossy().to_string(),
        );
        let environment = TestEnvironment {
            user_home: Some(workon_home.clone()),
            env_vars,
        };

        let locator = VirtualEnvWrapper::from(&environment);

        assert_eq!(locator.env_vars.home, Some(workon_home.clone()));
        assert_eq!(
            locator.env_vars.workon_home,
            Some(workon_home.to_string_lossy().to_string())
        );

        fs::remove_dir_all(workon_home).unwrap();
    }

    #[test]
    fn try_from_builds_virtualenvwrapper_environment() {
        let workon_home = create_test_dir("workon-home");
        let prefix = workon_home.join("wrapped-env");
        let executable = create_virtualenv(&prefix);
        let project_root = create_test_dir("project-root");
        fs::write(prefix.join(".project"), project_root.display().to_string()).unwrap();
        let locator = VirtualEnvWrapper {
            env_vars: EnvVariables {
                home: None,
                workon_home: Some(workon_home.to_string_lossy().to_string()),
            },
        };
        let env = PythonEnv::new(
            executable.clone(),
            Some(prefix.clone()),
            Some("3.12.1".to_string()),
        );

        let virtualenvwrapper_env = locator.try_from(&env).unwrap();

        assert_eq!(
            virtualenvwrapper_env.kind,
            Some(PythonEnvironmentKind::VirtualEnvWrapper)
        );
        assert_eq!(virtualenvwrapper_env.name, Some("wrapped-env".to_string()));
        assert_eq!(
            virtualenvwrapper_env
                .executable
                .as_ref()
                .map(pet_fs::path::norm_case),
            Some(pet_fs::path::norm_case(executable))
        );
        assert_eq!(virtualenvwrapper_env.version, Some("3.12.1".to_string()));
        assert_eq!(
            virtualenvwrapper_env.prefix,
            Some(pet_fs::path::norm_case(prefix.clone()))
        );
        assert_eq!(
            virtualenvwrapper_env.project,
            Some(pet_fs::path::norm_case(project_root.clone()))
        );
        assert!(virtualenvwrapper_env
            .symlinks
            .iter()
            .flatten()
            .any(|symlink| symlink.file_name()
                == virtualenvwrapper_env
                    .executable
                    .as_ref()
                    .unwrap()
                    .file_name()));

        fs::remove_dir_all(workon_home).unwrap();
        fs::remove_dir_all(project_root).unwrap();
    }

    #[test]
    fn try_from_returns_none_for_non_virtualenvwrapper_env() {
        let prefix = create_test_dir("standalone-env");
        let workon_home = create_test_dir("workon-home");
        let executable = create_virtualenv(&prefix);
        let locator = VirtualEnvWrapper {
            env_vars: EnvVariables {
                home: None,
                workon_home: Some(workon_home.to_string_lossy().to_string()),
            },
        };
        let env = PythonEnv::new(executable, Some(prefix.clone()), None);

        assert!(locator.try_from(&env).is_none());

        fs::remove_dir_all(prefix).unwrap();
        fs::remove_dir_all(workon_home).unwrap();
    }
}
