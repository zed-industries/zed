// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_core::{manager::EnvManager, python_environment::PythonEnvironment, reporter::Reporter};
use std::sync::{Arc, Mutex};

/// Used to just collect the environments and managers and will not report anytihng anywhere.
pub struct CollectReporter {
    pub managers: Arc<Mutex<Vec<EnvManager>>>,
    pub environments: Arc<Mutex<Vec<PythonEnvironment>>>,
}

impl Default for CollectReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectReporter {
    pub fn new() -> CollectReporter {
        CollectReporter {
            managers: Arc::new(Mutex::new(vec![])),
            environments: Arc::new(Mutex::new(vec![])),
        }
    }
}
impl Reporter for CollectReporter {
    fn report_telemetry(&self, _event: &pet_core::telemetry::TelemetryEvent) {
        //
    }
    fn report_manager(&self, manager: &EnvManager) {
        self.managers
            .lock()
            .expect("managers mutex poisoned")
            .push(manager.clone());
    }

    fn report_environment(&self, env: &PythonEnvironment) {
        self.environments
            .lock()
            .expect("environments mutex poisoned")
            .push(env.clone());
    }
}

pub fn create_reporter() -> CollectReporter {
    CollectReporter::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::{
        manager::EnvManagerType, python_environment::PythonEnvironmentKind,
        telemetry::TelemetryEvent,
    };
    use std::{path::PathBuf, time::Duration};

    #[test]
    fn collect_reporter_accumulates_managers_and_environments() {
        let reporter = create_reporter();
        let manager = EnvManager::new(
            PathBuf::from("/tmp/conda"),
            EnvManagerType::Conda,
            Some("24.1.0".to_string()),
        );
        let environment = PythonEnvironment::new(
            Some(PathBuf::from("/tmp/.venv/bin/python")),
            Some(PythonEnvironmentKind::Venv),
            Some(PathBuf::from("/tmp/.venv")),
            Some(manager.clone()),
            Some("3.12.0".to_string()),
        );

        reporter.report_manager(&manager);
        reporter.report_environment(&environment);
        reporter.report_telemetry(&TelemetryEvent::SearchCompleted(Duration::from_secs(1)));

        assert_eq!(reporter.managers.lock().unwrap().as_slice(), &[manager]);
        assert_eq!(
            reporter.environments.lock().unwrap().as_slice(),
            &[environment]
        );
    }

    #[test]
    fn default_collect_reporter_starts_empty() {
        let reporter = CollectReporter::default();

        assert!(reporter.managers.lock().unwrap().is_empty());
        assert!(reporter.environments.lock().unwrap().is_empty());
    }
}
