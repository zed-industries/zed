use collections::HashMap;
use gpui::SharedString;
use serde::Deserialize;
use util::ResultExt as _;

use crate::{
    DebugScenario, DebugTaskFile, EnvVariableReplacer, TcpArgumentsTemplate, VariableName,
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
    other_attributes: serde_json::Value,
}

impl VsCodeDebugTaskDefinition {
    fn try_to_zed(self, replacer: &EnvVariableReplacer) -> anyhow::Result<DebugScenario> {
        let label = replacer.replace(&self.name).into();
        // TODO based on grep.app results it seems that vscode supports whitespace-splitting this field (ugh)
        let definition = DebugScenario {
            label,
            build: None,
            adapter: task_type_to_adapter_name(&self.r#type),
            // TODO host?
            tcp_connection: self.port.map(|port| TcpArgumentsTemplate {
                port: Some(port),
                host: None,
                timeout: None,
            }),
            config: replacer.replace_value(self.other_attributes),
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
            ("file".to_owned(), VariableName::Filename.to_string()), // TODO other interesting variables?
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
        "rdbg" => "Ruby",
        _ => task_type,
    }
    .to_owned()
    .into()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{DebugScenario, DebugTaskFile, TcpArgumentsTemplate};

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
                config: json!({
                    "showDevDebugOutput": false,
                }),
                tcp_connection: Some(TcpArgumentsTemplate {
                    port: Some(17),
                    host: None,
                    timeout: None,
                }),
                build: None
            }])
        );
    }
}
