// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use env_logger::Builder;
use log::{trace, LevelFilter};
use pet_core::{
    manager::EnvManager,
    python_environment::{PythonEnvironment, PythonEnvironmentKind},
    reporter::Reporter,
    telemetry::{get_telemetry_event_name, TelemetryEvent},
};
use pet_jsonrpc::send_message;
use serde::{Deserialize, Serialize};

pub struct JsonRpcReporter {
    report_only: Option<PythonEnvironmentKind>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
struct TelemetryData {
    event: String,
    data: TelemetryEvent,
}

impl Reporter for JsonRpcReporter {
    fn report_telemetry(&self, event: &TelemetryEvent) {
        let event = TelemetryData {
            event: get_telemetry_event_name(event).to_string(),
            data: event.clone(),
        };
        trace!("Telemetry event {:?}", event.event);
        send_message("telemetry", Some(event))
    }
    fn report_manager(&self, manager: &EnvManager) {
        trace!("Reporting Manager {:?}", manager);
        send_message("manager", manager.into())
    }

    fn report_environment(&self, env: &PythonEnvironment) {
        if !should_report_environment(self.report_only, env) {
            trace!(
                "Skip Reporting Environment ({:?}) {:?} due to refresh request to report only {:?}",
                env.kind,
                env.executable
                    .clone()
                    .unwrap_or(env.prefix.clone().unwrap_or_default()),
                self.report_only
            );
            return;
        }
        trace!("Reporting Environment {:?}", env);
        send_message("environment", env.into())
    }
}

fn should_report_environment(
    report_only: Option<PythonEnvironmentKind>,
    env: &PythonEnvironment,
) -> bool {
    match report_only {
        Some(kind) => env.kind == Some(kind),
        None => true,
    }
}

pub fn create_reporter(report_only: Option<PythonEnvironmentKind>) -> impl Reporter {
    JsonRpcReporter { report_only }
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
    Builder::new()
        .format(|_, record| {
            let level = match record.level() {
                log::Level::Debug => LogLevel::Debug,
                log::Level::Error => LogLevel::Error,
                log::Level::Info => LogLevel::Info,
                log::Level::Warn => LogLevel::Warning,
                _ => LogLevel::Debug,
            };
            let payload = Log {
                message: format!("{}", record.args()).to_string(),
                level,
            };
            send_message("log", payload.into());
            Ok(())
        })
        .filter(None, log_level)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use pet_core::telemetry::TelemetryEvent;
    use serde_json::json;
    use std::{collections::BTreeMap, path::PathBuf};

    fn create_environment(kind: PythonEnvironmentKind) -> PythonEnvironment {
        PythonEnvironment::new(
            Some(PathBuf::from("/tmp/.venv/bin/python")),
            Some(kind),
            Some(PathBuf::from("/tmp/.venv")),
            None,
            Some("3.12.0".to_string()),
        )
    }

    #[test]
    fn environment_filter_allows_all_without_requested_kind() {
        let environment = create_environment(PythonEnvironmentKind::Venv);

        assert!(should_report_environment(None, &environment));
    }

    #[test]
    fn environment_filter_allows_matching_requested_kind() {
        let environment = create_environment(PythonEnvironmentKind::Poetry);

        assert!(should_report_environment(
            Some(PythonEnvironmentKind::Poetry),
            &environment
        ));
    }

    #[test]
    fn environment_filter_rejects_non_matching_requested_kind() {
        let environment = create_environment(PythonEnvironmentKind::Venv);

        assert!(!should_report_environment(
            Some(PythonEnvironmentKind::Poetry),
            &environment
        ));
    }

    #[test]
    fn telemetry_data_serializes_event_name_and_payload() {
        let event = TelemetryEvent::RefreshPerformance(
            pet_core::telemetry::refresh_performance::RefreshPerformance {
                total: 10,
                locators: BTreeMap::new(),
                breakdown: BTreeMap::new(),
            },
        );
        let payload = TelemetryData {
            event: get_telemetry_event_name(&event).to_string(),
            data: event,
        };

        let value = serde_json::to_value(payload).unwrap();

        assert_eq!(value["event"], json!("RefreshPerformance"));
        assert_eq!(value["data"]["refreshPerformance"]["total"], json!(10));
    }

    #[test]
    fn refresh_progress_serializes_privacy_safe_fields() {
        use pet_core::telemetry::refresh_progress::{
            RefreshProgress, RefreshProgressPhase, RefreshProgressStatus,
        };

        let event = TelemetryEvent::RefreshProgress(RefreshProgress {
            refresh_id: 42,
            phase: RefreshProgressPhase::Locators,
            status: RefreshProgressStatus::Completed,
            elapsed_ms: 15,
            phase_elapsed_ms: None,
            locator_name: Some("Conda".to_string()),
            locator_elapsed_ms: Some(10),
        });
        let payload = TelemetryData {
            event: get_telemetry_event_name(&event).to_string(),
            data: event,
        };

        assert_eq!(
            serde_json::to_value(payload).unwrap(),
            json!({
                "event": "RefreshProgress",
                "data": {
                    "refreshProgress": {
                        "refreshId": 42,
                        "phase": "locators",
                        "status": "completed",
                        "elapsedMs": 15,
                        "locatorName": "Conda",
                        "locatorElapsedMs": 10
                    }
                }
            })
        );
    }
    #[test]
    fn log_payload_uses_camel_case_fields_and_level_renames() {
        let payload = Log {
            message: "hello".to_string(),
            level: LogLevel::Warning,
        };

        assert_eq!(
            serde_json::to_value(payload).unwrap(),
            json!({ "message": "hello", "level": "warning" })
        );
    }
}
