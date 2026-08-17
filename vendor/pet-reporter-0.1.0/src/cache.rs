// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::environment::get_environment_key;
use pet_core::{manager::EnvManager, python_environment::PythonEnvironment, reporter::Reporter};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, RwLock},
};

/// Poorly named, needs to be renamed,
/// The purpose of this reporter was to act as a cache, but since then
/// the requirements of caching have changed and this is no longer a cache.
/// This is merely a decorator class that ensures we do not report the same env/manager more than once.
pub struct CacheReporter {
    reporter: Arc<dyn Reporter>,
    reported_managers: Arc<RwLock<HashMap<PathBuf, EnvManager>>>,
    reported_environments: Arc<RwLock<HashMap<PathBuf, PythonEnvironment>>>,
}

impl CacheReporter {
    pub fn new(reporter: Arc<dyn Reporter>) -> Self {
        Self {
            reporter,
            reported_managers: Arc::new(RwLock::new(HashMap::new())),
            reported_environments: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
impl Reporter for CacheReporter {
    fn report_telemetry(&self, event: &pet_core::telemetry::TelemetryEvent) {
        self.reporter.report_telemetry(event);
    }
    fn report_manager(&self, manager: &EnvManager) {
        // First check with read lock
        {
            let reported_managers = self.reported_managers.read().unwrap();
            if reported_managers.contains_key(&manager.executable) {
                return;
            }
        }
        // Insert with write lock
        let mut reported_managers = self.reported_managers.write().unwrap();
        if !reported_managers.contains_key(&manager.executable) {
            reported_managers.insert(manager.executable.clone(), manager.clone());
            self.reporter.report_manager(manager);
        }
    }

    fn report_environment(&self, env: &PythonEnvironment) {
        if let Some(key) = get_environment_key(env) {
            // First check with read lock
            {
                let reported_environments = self.reported_environments.read().unwrap();
                if reported_environments.contains_key(&key) {
                    return;
                }
            }
            // Insert with write lock
            let mut reported_environments = self.reported_environments.write().unwrap();
            if !reported_environments.contains_key(&key) {
                reported_environments.insert(key.clone(), env.clone());
                self.reporter.report_environment(env);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::{
        manager::EnvManagerType, python_environment::PythonEnvironmentKind,
        telemetry::TelemetryEvent,
    };
    use std::{sync::Mutex, time::Duration};

    #[derive(Default)]
    struct RecordingReporter {
        managers: Mutex<Vec<EnvManager>>,
        environments: Mutex<Vec<PythonEnvironment>>,
        telemetry_count: Mutex<usize>,
    }

    impl Reporter for RecordingReporter {
        fn report_telemetry(&self, _event: &pet_core::telemetry::TelemetryEvent) {
            *self.telemetry_count.lock().unwrap() += 1;
        }

        fn report_manager(&self, manager: &EnvManager) {
            self.managers.lock().unwrap().push(manager.clone());
        }

        fn report_environment(&self, env: &PythonEnvironment) {
            self.environments.lock().unwrap().push(env.clone());
        }
    }

    #[test]
    fn cache_reporter_dedupes_managers_by_executable() {
        let inner = Arc::new(RecordingReporter::default());
        let reporter = CacheReporter::new(inner.clone());
        let manager = EnvManager::new(
            PathBuf::from("/tmp/conda"),
            EnvManagerType::Conda,
            Some("24.1.0".to_string()),
        );

        reporter.report_manager(&manager);
        reporter.report_manager(&manager);

        assert_eq!(inner.managers.lock().unwrap().as_slice(), &[manager]);
    }

    #[test]
    fn cache_reporter_dedupes_environments_by_environment_key() {
        let inner = Arc::new(RecordingReporter::default());
        let reporter = CacheReporter::new(inner.clone());
        let environment = PythonEnvironment::new(
            Some(PathBuf::from("/tmp/.venv/bin/python")),
            Some(PythonEnvironmentKind::Venv),
            Some(PathBuf::from("/tmp/.venv")),
            None,
            Some("3.12.0".to_string()),
        );

        reporter.report_environment(&environment);
        reporter.report_environment(&environment);

        assert_eq!(
            inner.environments.lock().unwrap().as_slice(),
            &[environment]
        );
    }

    #[test]
    fn cache_reporter_ignores_environments_without_a_key() {
        let inner = Arc::new(RecordingReporter::default());
        let reporter = CacheReporter::new(inner.clone());
        let environment = PythonEnvironment::default();

        reporter.report_environment(&environment);

        assert!(inner.environments.lock().unwrap().is_empty());
    }

    #[test]
    fn cache_reporter_forwards_telemetry() {
        let inner = Arc::new(RecordingReporter::default());
        let reporter = CacheReporter::new(inner.clone());

        reporter.report_telemetry(&TelemetryEvent::SearchCompleted(Duration::from_secs(1)));

        assert_eq!(*inner.telemetry_count.lock().unwrap(), 1);
    }
}
