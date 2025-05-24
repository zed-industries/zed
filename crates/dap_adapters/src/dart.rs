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

    async fn dap_schema(&self) -> serde_json::Value {
        json!({
            "properties": {
                "request": {
                    "type": "string",
                    "enum": ["launch", "attach"],
                    "description": "Request type: 'launch' to start a new app, 'attach' to connect to a running one"
                },
                "program": {
                    "type": "string",
                    "description": "Path to the main Dart entry point (e.g., lib/main.dart)"
                },
                "cwd": {
                    "type": "string",
                    "description": "Working directory for the Dart/Flutter process"
                },
                "args": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Arguments passed to the Dart program"
                },
                "toolArgs": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Arguments for the 'flutter' or 'dart' tool command"
                },
                "customTool": {
                    "type": "string",
                    "description": "Custom tool to run instead of 'flutter' or 'dart'"
                },
                "customToolReplacesArgs": {
                    "type": "integer",
                    "description": "Number of default arguments to remove when using customTool",
                    "default": 0
                },
                "vmServiceUri": {
                    "type": "string",
                    "description": "Dart VM Service URI for attach (e.g., ws://127.0.0.1:8181/ws)"
                },
                "vmServiceInfoFile": {
                    "type": "string",
                    "description": "Path to file containing the Dart VM Service URI"
                },
                "noDebug": {
                    "type": "boolean",
                    "description": "Launch without debugging (run mode)",
                    "default": false
                },
                "flutterMode": {
                    "type": "string",
                    "enum": ["run", "test"],
                    "description": "Flutter execution mode",
                    "default": "run"
                },
                "env": {
                    "type": "object",
                    "additionalProperties": { "type": "string" },
                    "description": "Environment variables for the launched program"
                },
                "console": {
                    "type": "string",
                    "enum": ["debugConsole", "terminal"],
                    "description": "Where to launch the debug target",
                    "default": "debugConsole"
                },
                "stopOnEntry": {
                    "type": "boolean",
                    "description": "Automatically stop after launch",
                    "default": false
                }
            },
            "required": ["request"],
            "allOf": [
                {
                    "if": { "properties": { "request": { "const": "launch" } } },
                    "then": { "required": ["program"] }
                },
                {
                    "if": { "properties": { "request": { "const": "attach" } } },
                    "then": {
                        "anyOf": [
                            { "required": ["vmServiceUri"] },
                            { "required": ["vmServiceInfoFile"] }
                        ]
                    }
                }
            ]
        })
    }

    fn validate_config(
        &self,
        config: &serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        let map = config
            .as_object()
            .ok_or_else(|| anyhow!("Debug configuration must be a valid JSON object"))?;

        let request_str = map.get("request").and_then(|v| v.as_str()).ok_or_else(|| {
            anyhow!("'request' field is required and must be 'launch' or 'attach'")
        })?;

        match request_str {
            "launch" => {
                if !map.contains_key("program")
                    || map.get("program").and_then(|v| v.as_str()).is_none()
                {
                    return Err(anyhow!("'program' field is required for launch requests"));
                }
                Ok(StartDebuggingRequestArgumentsRequest::Launch)
            }
            "attach" => {
                let has_uri = map.get("vmServiceUri").and_then(|v| v.as_str()).is_some();
                let has_file = map
                    .get("vmServiceInfoFile")
                    .and_then(|v| v.as_str())
                    .is_some();

                if !has_uri && !has_file {
                    return Err(anyhow!(
                        "Attach requests require either 'vmServiceUri' or 'vmServiceInfoFile'"
                    ));
                }

                Ok(StartDebuggingRequestArgumentsRequest::Attach)
            }
            _ => Err(anyhow!("'request' must be either 'launch' or 'attach'")),
        }
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let mut config = serde_json::Map::new();

        config.insert(
            "request".to_string(),
            match &zed_scenario.request {
                DebugRequest::Launch(_) => json!("launch"),
                DebugRequest::Attach(_) => json!("attach"),
            },
        );
        config.insert("name".to_string(), json!(zed_scenario.label.as_ref()));

        match &zed_scenario.request {
            DebugRequest::Launch(launch_config) => {
                config.insert("program".to_string(), json!(launch_config.program));
                if let Some(cwd_path) = &launch_config.cwd {
                    config.insert("cwd".to_string(), json!(cwd_path.to_string_lossy()));
                }
                if !launch_config.args.is_empty() {
                    config.insert("args".to_string(), json!(launch_config.args));
                }
                if !launch_config.env.is_empty() {
                    config.insert("env".to_string(), launch_config.env_json());
                }
            }
            DebugRequest::Attach(_) => {
                // attach-specific fields are handled via task_definition.config
            }
        }

        if let Some(stop_on_entry) = zed_scenario.stop_on_entry {
            config.insert("stopOnEntry".to_string(), json!(stop_on_entry));
        }

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            config: serde_json::Value::Object(config),
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
                        "'{}' not found in PATH. Install Flutter SDK and add to PATH",
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
                        "'{}' not found in PATH. Install Dart SDK and add to PATH",
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

        let mut processed_config = task_definition.config.clone();
        self.process_dart_defines(&mut processed_config, delegate)
            .await?;

        let dap_request_args = StartDebuggingRequestArguments {
            request: self.validate_config(&processed_config)?,
            configuration: processed_config,
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
            command: executable,
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

    async fn process_dart_defines(
        &self,
        config: &mut serde_json::Value,
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<()> {
        if let Some(tool_args) = config.get("toolArgs").and_then(|v| v.as_array()).cloned() {
            let mut new_tool_args = Vec::new();

            for arg in tool_args {
                if let Some(arg_str) = arg.as_str() {
                    if arg_str.starts_with("--dart-define-from-file=") {
                        let file_path = &arg_str[24..];
                        let full_path = if std::path::Path::new(file_path).is_relative() {
                            delegate.worktree_root_path().join(file_path)
                        } else {
                            std::path::PathBuf::from(file_path)
                        };

                        if let Ok(content) = delegate.fs().load(&full_path).await {
                            match serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
                                &content,
                            ) {
                                Ok(json_map) => {
                                    for (key, value) in json_map {
                                        let value_str = match value {
                                            serde_json::Value::String(s) => s,
                                            serde_json::Value::Number(n) => n.to_string(),
                                            serde_json::Value::Bool(b) => b.to_string(),
                                            _ => value.to_string(),
                                        };
                                        new_tool_args.push(serde_json::Value::String(format!(
                                            "--dart-define={}={}",
                                            key, value_str
                                        )));
                                    }
                                }
                                Err(_) => {
                                    new_tool_args.push(arg);
                                }
                            }
                        } else {
                            new_tool_args.push(arg);
                        }
                    } else {
                        new_tool_args.push(arg);
                    }
                }
            }

            if let Some(config_obj) = config.as_object_mut() {
                config_obj.insert(
                    "toolArgs".to_string(),
                    serde_json::Value::Array(new_tool_args),
                );
            }
        }

        Ok(())
    }
}
