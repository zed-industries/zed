use std::path::PathBuf;

use anyhow::anyhow;
use collections::HashMap;
use gpui::SharedString;
use serde::Deserialize;
use util::ResultExt as _;

use crate::{
    AttachRequest, DebugRequest, DebugScenario, DebugTaskFile, EnvVariableReplacer, LaunchRequest,
    TcpArgumentsTemplate, VariableName, debug_format,
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
    fn try_to_zed(self, replacer: &EnvVariableReplacer) -> anyhow::Result<DebugScenario> {
        let label = replacer.replace(&self.name).into();
        // TODO based on grep.app results it seems that vscode supports whitespace-splitting this field (ugh)
        let definition = DebugScenario {
            label,
            build: None,
            request: match self.request {
                Request::Launch => {
                    // let cwd = self.cwd.map(|cwd| PathBuf::from(replacer.replace(&cwd)));
                    // let program = self.program.ok_or_else(|| {
                    //     anyhow!("vscode debug launch configuration does not define a program")
                    // })?;
                    // let program = replacer.replace(&program);
                    // let args = self
                    //     .args
                    //     .into_iter()
                    //     .map(|arg| replacer.replace(&arg))
                    //     .collect();
                    // let env = self
                    //     .env
                    //     .into_iter()
                    //     .filter_map(|(k, v)| v.map(|v| (k, v)))
                    //     .collect();
                    // DebugRequest::Launch(LaunchRequest {
                    //     program,
                    //     cwd,
                    //     args,
                    //     env,
                    // })
                    Some(debug_format::Request::Launch)
                }
                Request::Attach => Some(debug_format::Request::Attach),
            },
            adapter: task_type_to_adapter_name(&self.r#type),
            // TODO host?
            tcp_connection: self.port.map(|port| TcpArgumentsTemplate {
                port: Some(port),
                host: None,
                timeout: None,
            }),
            stop_on_entry: self.stop_on_entry,
            // TODO
            config: serde_json::Value::Null,
        };
        Ok(definition)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeDebugTaskFile {
    version: String,
    configurations: Vec<VsCodeDebugTaskDefinition>,
}

impl TryFrom<VsCodeDebugTaskFile> for DebugTaskFile {
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
        Ok(DebugTaskFile(templates))
    }
}

// todo(debugger) figure out how to make JsDebugAdapter::ADAPTER_NAME et al available here
fn task_type_to_adapter_name(task_type: &str) -> SharedString {
    match task_type {
        "node" => "JavaScript",
        "go" => "Delve",
        "php" => "PHP",
        "cppdbg" | "lldb" => "CodeLLDB",
        "debugpy" => "Debugpy",
        _ => task_type,
    }
    .to_owned()
    .into()
}

#[cfg(test)]
mod tests {

    use collections::FxHashMap;

    use crate::{DebugRequest, DebugScenario, DebugTaskFile, LaunchRequest, TcpArgumentsTemplate};

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
                        "cwd": "${workspaceFolder}/${env:FOO}/sub",
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
        let zed = DebugTaskFile::try_from(parsed).expect("converting to Zed debug templates");
        pretty_assertions::assert_eq!(
            zed,
            DebugTaskFile(vec![DebugScenario {
                label: "Debug my JS app".into(),
                adapter: "JavaScript".into(),
                stop_on_entry: Some(true),
                initialize_args: None,
                tcp_connection: Some(TcpArgumentsTemplate {
                    port: Some(17),
                    host: None,
                    timeout: None,
                }),
                request: Some(DebugRequest::Launch(LaunchRequest {
                    program: "${ZED_WORKTREE_ROOT}/xyz.js".into(),
                    args: vec!["--foo".into(), "${ZED_WORKTREE_ROOT}/thing".into()],
                    cwd: Some("${ZED_WORKTREE_ROOT}/${FOO}/sub".into()),
                    env: FxHashMap::from_iter([("X".into(), "Y".into())])
                })),
                build: None
            }])
        );
    }
}
