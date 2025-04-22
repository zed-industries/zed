use anyhow::anyhow;
use collections::HashMap;
use serde::Deserialize;
use util::ResultExt as _;

use crate::{
    AttachRequest, DebugArgs, DebugArgsRequest, EnvVariableReplacer, TaskTemplate, TaskTemplates,
    TaskType, TcpArgumentsTemplate, VariableName,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
enum Request {
    Launch,
    Attach,
}

// TODO support preLaunchTask linkage with other tasks
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct VsCodeDebugTaskDefinition {
    r#type: String,
    name: String,
    request: Request,

    #[serde(default)]
    program: Option<String>,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: HashMap<String, Option<String>>,
    // TODO envFile?
    #[serde(default)]
    cwd: Option<String>,
    #[serde(default)]
    port: Option<u16>,
    #[serde(default)]
    stop_on_entry: Option<bool>,
    #[serde(flatten)]
    other_attributes: HashMap<String, serde_json_lenient::Value>,
}

impl VsCodeDebugTaskDefinition {
    fn try_to_zed(self, replacer: &EnvVariableReplacer) -> anyhow::Result<TaskTemplate> {
        let label = replacer.replace(&self.name);
        let command = match &self.request {
            Request::Launch => self.program.clone().ok_or_else(|| {
                anyhow!("debug task launch configuration does not define a program")
            })?,
            Request::Attach => Default::default(),
        };

        let command = replacer.replace(&command);
        // TODO based on grep.app results it seems that vscode supports whitespace-splitting this field (ugh)
        let args = self
            .args
            .into_iter()
            .map(|arg| replacer.replace(&arg))
            .collect();
        let cwd = self.cwd.map(|cwd| replacer.replace(&cwd));
        let template = TaskTemplate {
            label,
            command,
            args,
            cwd,
            // TODO support unsetting envs like vscode
            env: self
                .env
                .into_iter()
                .filter_map(|(k, v)| Some((k, v?)))
                .collect(),
            task_type: TaskType::Debug(DebugArgs {
                request: match self.request {
                    Request::Launch { .. } => DebugArgsRequest::Launch,
                    Request::Attach => DebugArgsRequest::Attach(AttachRequest { process_id: None }),
                },
                adapter: task_type_to_adapter_name(self.r#type),
                // TODO host?
                tcp_connection: self.port.map(|port| TcpArgumentsTemplate {
                    port: Some(port),
                    host: None,
                    timeout: None,
                }),
                stop_on_entry: self.stop_on_entry,
                // TODO
                initialize_args: None,
                locator: None,
            }),
            ..Default::default()
        };
        Ok(template)
    }
}

/// blah
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeDebugTaskFile {
    version: String,
    configurations: Vec<VsCodeDebugTaskDefinition>,
}

impl TryFrom<VsCodeDebugTaskFile> for TaskTemplates {
    type Error = anyhow::Error;

    fn try_from(file: VsCodeDebugTaskFile) -> Result<Self, Self::Error> {
        let replacer = EnvVariableReplacer::new(HashMap::from_iter([
            (
                "workspaceFolder".to_owned(),
                VariableName::WorktreeRoot.to_string(),
            ),
            // TODO other interesting variables?
        ]));
        let templates = file
            .configurations
            .into_iter()
            .filter_map(|config| config.try_to_zed(&replacer).log_err())
            .collect::<Vec<_>>();
        Ok(TaskTemplates(templates))
    }
}

// TODO figure out how to make JsDebugAdapter::ADAPTER_NAME et al available here
fn task_type_to_adapter_name(task_type: String) -> String {
    match task_type.as_str() {
        "node" => "JavaScript".to_owned(),
        "go" => "Delve".to_owned(),
        "php" => "PHP".to_owned(),
        "cppdbg" | "lldb" => "CodeLLDB".to_owned(),
        "debugpy" => "Debugpy".to_owned(),
        _ => task_type,
    }
}

#[cfg(test)]
mod tests {
    use collections::HashMap;

    use crate::{
        DebugArgs, DebugArgsRequest, TaskTemplate, TaskTemplates, TaskType, TcpArgumentsTemplate,
    };

    use super::VsCodeDebugTaskFile;

    #[test]
    fn test_parsing_vscode_launch_json() {
        let raw = r#"
            {
                "version": "0.2.0",
                "configurations": [
                    {
                        "name": "Debug my JS app",
                        "request": "launch",
                        "type": "node",
                        "program": "${workspaceFolder}/xyz.js",
                        "showDevDebugOutput": false,
                        "stopOnEntry": true,
                        "args": ["--foo", "${workspaceFolder}/thing"],
                        "cwd": "${workspaceFolder}/sub",
                        "env": {
                            "X": "Y"
                        },
                        "port": 17
                    },
                ]
            }
        "#;
        let parsed: VsCodeDebugTaskFile =
            serde_json_lenient::from_str(&raw).expect("deserializing launch.json");
        let zed = TaskTemplates::try_from(parsed).expect("converting to Zed debug templates");
        pretty_assertions::assert_eq!(
            zed,
            TaskTemplates(vec![TaskTemplate {
                label: "Debug my JS app".into(),
                command: "${ZED_WORKTREE_ROOT}/xyz.js".into(),
                args: vec!["--foo".into(), "${ZED_WORKTREE_ROOT}/thing".into()],
                cwd: Some("${ZED_WORKTREE_ROOT}/sub".into()),
                env: HashMap::from_iter([("X".into(), "Y".into())]),
                task_type: TaskType::Debug(DebugArgs {
                    request: DebugArgsRequest::Launch,
                    adapter: "JavaScript".into(),
                    tcp_connection: Some(TcpArgumentsTemplate {
                        port: Some(17),
                        host: None,
                        timeout: None,
                    }),
                    stop_on_entry: Some(true),
                    initialize_args: None,
                    locator: None,
                }),
                ..Default::default()
            }])
        );
    }
}
