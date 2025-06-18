use crate::*;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use dap::{
    DebugRequest, StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
    adapters::{DebugAdapter, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition},
};
use gpui::AsyncApp;
use language::LanguageName;
use serde_json::json;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};
use task::{DebugScenario, ZedDebugConfig};

#[derive(Default, Debug)]
pub(crate) struct DartDebugAdapter;

impl DartDebugAdapter {
    const ADAPTER_NAME: &'static str = "Dart";
    const FLUTTER_EXECUTABLE_NAME: &'static str = "flutter";
    const DART_EXECUTABLE_NAME: &'static str = "dart";
}

#[async_trait(?Send)]
impl DebugAdapter for DartDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(LanguageName::new("Dart"))
    }

    fn dap_schema(&self) -> serde_json::Value {
        json!({
            "properties": {
                "request": {
                    "type": "string",
                    "enum": ["launch", "attach"],
                    "description": "Specifies 'launch' to start a new app, or 'attach' to connect to a running one."
                },
                "program": {
                    "type": "string",
                    "description": "Path to the main Dart entry point (e.g., lib/main.dart)."
                },
                "cwd": {
                    "type": "string",
                    "description": "Working directory for the Dart/Flutter process. Defaults to project root."
                },
                "args": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Arguments passed directly to the Dart program."
                },
                "toolArgs": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Arguments for the 'flutter' or 'dart' tool command (e.g., ['--flavor', 'dev'])."
                },
                "customTool": {
                    "type": "string",
                    "description": "Optional: custom tool (e.g., wrapper script) to run instead of 'flutter' or 'dart'."
                },
                "customToolReplacesArgs": {
                    "type": "integer",
                    "description": "Number of default arguments to remove when 'customTool' is used."
                },
                "vmServiceUri": {
                    "type": "string",
                    "description": "(Attach only) Dart VM Service URI for an already running application."
                },
                "vmServiceInfoFile": {
                    "type": "string",
                    "description": "(Attach only) Path to a file containing the Dart VM Service URI."
                },
                "noDebug": {
                    "type": "boolean",
                    "description": "(Launch only) If true, launches without debugging (run mode). Defaults to false (debug mode).",
                    "default": false
                },
                "flutterMode": {
                    "type": "string",
                    "enum": ["run", "test"],
                    "description": "If 'test', uses 'flutter test --machine' semantics. Defaults to 'run'.",
                    "default": "run"
                },
                "env": {
                    "type": "object",
                    "additionalProperties": { "type": "string" },
                    "description": "Environment variables for the launched program."
                },
                "console": {
                    "type": "string",
                    "enum": ["debugConsole", "terminal"],
                    "description": "Where to launch the debug target: internal console or terminal.",
                    "default": "debugConsole"
                },
                "enableDartDevelopmentService": {
                    "type": "boolean",
                    "description": "Whether to enable Dart Development Service (DDS).",
                    "default": true
                },
                "debugExternalPackageLibraries": {
                    "type": "boolean",
                    "description": "Whether to debug external package libraries.",
                    "default": false
                },
                "debugSdkLibraries": {
                    "type": "boolean",
                    "description": "Whether to debug SDK libraries.",
                    "default": false
                },
                "evaluateGettersInDebugViews": {
                    "type": "boolean",
                    "description": "Whether to evaluate getters in debug views.",
                    "default": true
                },
                "evaluateToStringInDebugViews": {
                    "type": "boolean",
                    "description": "Whether to evaluate toString() in debug views.",
                    "default": true
                }
            },
            "required": ["request"],
            "allOf": [
                {
                    "if": { "properties": { "request": { "const": "launch" } } },
                    "then": { "required": ["program"] }
                }
            ]
        })
    }

    async fn request_kind(
        &self,
        config: &serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        let map = config
            .as_object()
            .ok_or_else(|| anyhow!("Debug configuration must be a valid JSON object"))?;

        let request_str = map
            .get("request")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                anyhow!(
                    "The 'request' field is required in debug.json and must be a string ('launch' or 'attach')"
                )
            })?;

        match request_str {
            "launch" => {
                if !map.contains_key("program")
                    || map.get("program").and_then(|v| v.as_str()).is_none()
                {
                    return Err(anyhow!(
                        "The 'program' field (a string path to the main Dart file) is required for a 'launch' request"
                    ));
                }
                Ok(StartDebuggingRequestArgumentsRequest::Launch)
            }
            "attach" => {
                let has_uri = map.get("vmServiceUri").and_then(|v| v.as_str()).is_some();
                let has_info_file = map
                    .get("vmServiceInfoFile")
                    .and_then(|v| v.as_str())
                    .is_some();

                if !has_uri && !has_info_file {
                    return Err(anyhow!(
                        "For 'attach' request, either 'vmServiceUri' or 'vmServiceInfoFile' must be provided"
                    ));
                }

                Ok(StartDebuggingRequestArgumentsRequest::Attach)
            }
            _ => Err(anyhow!(
                "Invalid 'request' value: '{}'. It must be either 'launch' or 'attach'.",
                request_str
            )),
        }
    }

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let mut dap_config_map = serde_json::Map::new();

        dap_config_map.insert(
            "request".to_string(),
            match &zed_scenario.request {
                DebugRequest::Launch(_) => json!("launch"),
                DebugRequest::Attach(_) => json!("attach"),
            },
        );
        dap_config_map.insert("name".to_string(), json!(zed_scenario.label.as_ref()));

        match &zed_scenario.request {
            DebugRequest::Launch(launch_config) => {
                dap_config_map.insert("program".to_string(), json!(launch_config.program));
                if let Some(cwd_path) = &launch_config.cwd {
                    dap_config_map.insert("cwd".to_string(), json!(cwd_path.to_string_lossy()));
                }
                if !launch_config.args.is_empty() {
                    dap_config_map.insert("args".to_string(), json!(launch_config.args));
                }
                if !launch_config.env.is_empty() {
                    dap_config_map.insert("env".to_string(), launch_config.env_json());
                }
            }
            DebugRequest::Attach(_attach_config) => {
                // Attach-specific fields are handled via task_definition.config in get_binary
            }
        }

        if let Some(stop_on_entry) = zed_scenario.stop_on_entry {
            dap_config_map.insert("stopOnEntry".to_string(), json!(stop_on_entry));
        }

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            config: serde_json::Value::Object(dap_config_map),
            build: None,
            tcp_connection: None,
        })
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        task_definition: &DebugTaskDefinition,
        _user_installed_path: Option<PathBuf>,
        _cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let is_flutter_project = self.is_flutter_project(delegate).await;

        let tool_executable_path = if is_flutter_project {
            delegate
                .which(OsStr::new(Self::FLUTTER_EXECUTABLE_NAME))
                .await
                .map(|p| p.to_string_lossy().into_owned())
                .ok_or_else(|| {
                    anyhow!(
                        "'{}' command not found in PATH. Ensure Flutter SDK is installed and 'bin' directory is in PATH.",
                        Self::FLUTTER_EXECUTABLE_NAME
                    )
                })?
        } else {
            delegate
                .which(OsStr::new(Self::DART_EXECUTABLE_NAME))
                .await
                .map(|p| p.to_string_lossy().into_owned())
                .ok_or_else(|| {
                    anyhow!(
                        "'{}' command not found in PATH. Ensure Dart SDK is installed and 'bin' directory is in PATH.",
                        Self::DART_EXECUTABLE_NAME
                    )
                })?
        };

        let mut adapter_args = vec!["debug_adapter".to_string()];

        if is_flutter_project {
            if let Some(mode_val) = task_definition.config.get("flutterMode") {
                if mode_val.as_str() == Some("test") {
                    adapter_args.push("--test".to_string());
                }
            }
        }

        let executable = if let Some(custom_tool) = task_definition
            .config
            .get("customTool")
            .and_then(|v| v.as_str())
        {
            if let Some(replace_count) = task_definition
                .config
                .get("customToolReplacesArgs")
                .and_then(|v| v.as_u64())
            {
                let remove_count = std::cmp::min(replace_count as usize, adapter_args.len());
                adapter_args.drain(0..remove_count);
            }

            custom_tool.to_string()
        } else {
            tool_executable_path
        };

        if let Some(tool_args_val) = task_definition.config.get("toolArgs") {
            if let Some(tool_args_array) = tool_args_val.as_array() {
                for arg_val in tool_args_array {
                    if let Some(arg_str) = arg_val.as_str() {
                        adapter_args.push(arg_str.to_string());
                    }
                }
            }
        }

        let dap_request_args = StartDebuggingRequestArguments {
            request: self.request_kind(&task_definition.config).await?,
            configuration: task_definition.config.clone(),
        };

        let cwd = task_definition
            .config
            .get("cwd")
            .and_then(|v| v.as_str())
            .map(PathBuf::from)
            .or_else(|| Some(delegate.worktree_root_path().to_path_buf()));

        let mut envs = HashMap::default();
        if let Some(env_val) = task_definition.config.get("env") {
            if let Some(env_obj) = env_val.as_object() {
                for (k, v) in env_obj {
                    if let Some(s) = v.as_str() {
                        envs.insert(k.clone(), s.to_string());
                    }
                }
            }
        }

        Ok(DebugAdapterBinary {
            command: Some(executable),
            arguments: adapter_args,
            cwd,
            envs,
            connection: None,
            request_args: dap_request_args,
        })
    }
}

impl DartDebugAdapter {
    async fn is_flutter_project(&self, delegate: &Arc<dyn DapDelegate>) -> bool {
        let pubspec_path = delegate.worktree_root_path().join("pubspec.yaml");

        if !delegate.fs().is_file(&pubspec_path).await {
            return false;
        }

        if let Ok(contents) = delegate.fs().load(&pubspec_path).await {
            return contents.contains("flutter:")
                || contents.contains("flutter_test:")
                || contents.contains("sdk: flutter");
        }

        false
    }
}
