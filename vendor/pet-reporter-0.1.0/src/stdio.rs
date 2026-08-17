// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use env_logger::Builder;
use log::LevelFilter;
use pet_core::{
    manager::{EnvManager, EnvManagerType},
    python_environment::{PythonEnvironment, PythonEnvironmentKind},
    reporter::Reporter,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct StdioReporter {
    print_list: bool,
    managers: Arc<Mutex<HashMap<EnvManagerType, u16>>>,
    environments: Arc<Mutex<HashMap<Option<PythonEnvironmentKind>, u16>>>,
    environment_paths: Arc<Mutex<HashMap<Option<PythonEnvironmentKind>, Vec<PythonEnvironment>>>>,
    kind: Option<PythonEnvironmentKind>,
}

pub struct Summary {
    pub managers: HashMap<EnvManagerType, u16>,
    pub environments: HashMap<Option<PythonEnvironmentKind>, u16>,
    pub environment_paths: HashMap<Option<PythonEnvironmentKind>, Vec<PythonEnvironment>>,
}

impl StdioReporter {
    pub fn get_summary(&self) -> Summary {
        let managers = self.managers.lock().expect("managers mutex poisoned");
        let environments = self
            .environments
            .lock()
            .expect("environments mutex poisoned");
        let environment_paths = self
            .environment_paths
            .lock()
            .expect("environment_paths mutex poisoned");
        Summary {
            managers: managers.clone(),
            environments: environments.clone(),
            environment_paths: environment_paths.clone(),
        }
    }
}
impl Reporter for StdioReporter {
    fn report_telemetry(&self, _event: &pet_core::telemetry::TelemetryEvent) {
        //
    }
    fn report_manager(&self, manager: &EnvManager) {
        let mut managers = self.managers.lock().expect("managers mutex poisoned");
        let count = managers.get(&manager.tool).unwrap_or(&0) + 1;
        managers.insert(manager.tool, count);
        if self.print_list {
            println!("{manager}")
        }
    }

    fn report_environment(&self, env: &PythonEnvironment) {
        if self.kind.is_some() && env.kind != self.kind {
            return;
        }
        let mut environments = self
            .environments
            .lock()
            .expect("environments mutex poisoned");
        let count = environments.get(&env.kind).unwrap_or(&0) + 1;
        environments.insert(env.kind, count);

        // Store the environment details for verbose reporting
        let mut environment_paths = self
            .environment_paths
            .lock()
            .expect("environment_paths mutex poisoned");
        let paths = environment_paths.entry(env.kind).or_default();
        paths.push(env.clone());

        if self.print_list {
            println!("{env}")
        }
    }
}

pub fn create_reporter(print_list: bool, kind: Option<PythonEnvironmentKind>) -> StdioReporter {
    StdioReporter {
        print_list,
        managers: Arc::new(Mutex::new(HashMap::new())),
        environments: Arc::new(Mutex::new(HashMap::new())),
        environment_paths: Arc::new(Mutex::new(HashMap::new())),
        kind,
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
pub enum LogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub message: String,
    pub level: LogLevel,
}

pub fn initialize_logger(log_level: LevelFilter) {
    Builder::new().filter(None, log_level).init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_environment(kind: PythonEnvironmentKind, executable: &str) -> PythonEnvironment {
        PythonEnvironment::new(
            Some(PathBuf::from(executable)),
            Some(kind),
            Some(PathBuf::from("/tmp/env")),
            None,
            Some("3.12.0".to_string()),
        )
    }

    #[test]
    fn stdio_reporter_counts_managers_and_environments() {
        let reporter = create_reporter(false, None);
        let manager = EnvManager::new(
            PathBuf::from("/tmp/conda"),
            EnvManagerType::Conda,
            Some("24.1.0".to_string()),
        );
        let environment = create_environment(PythonEnvironmentKind::Venv, "/tmp/.venv/bin/python");

        reporter.report_manager(&manager);
        reporter.report_manager(&manager);
        reporter.report_environment(&environment);
        reporter.report_environment(&environment);

        let summary = reporter.get_summary();
        assert_eq!(summary.managers.get(&EnvManagerType::Conda), Some(&2));
        assert_eq!(
            summary.environments.get(&Some(PythonEnvironmentKind::Venv)),
            Some(&2)
        );
        assert_eq!(
            summary
                .environment_paths
                .get(&Some(PythonEnvironmentKind::Venv))
                .unwrap()
                .as_slice(),
            &[environment.clone(), environment]
        );
    }

    #[test]
    fn stdio_reporter_filters_environments_by_requested_kind() {
        let reporter = create_reporter(false, Some(PythonEnvironmentKind::Poetry));
        let poetry_environment =
            create_environment(PythonEnvironmentKind::Poetry, "/tmp/poetry/bin/python");
        let venv_environment =
            create_environment(PythonEnvironmentKind::Venv, "/tmp/.venv/bin/python");

        reporter.report_environment(&venv_environment);
        reporter.report_environment(&poetry_environment);

        let summary = reporter.get_summary();
        assert!(!summary
            .environments
            .contains_key(&Some(PythonEnvironmentKind::Venv)));
        assert_eq!(
            summary
                .environments
                .get(&Some(PythonEnvironmentKind::Poetry)),
            Some(&1)
        );
    }
}
