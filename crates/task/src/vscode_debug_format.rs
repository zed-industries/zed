use collections::HashMap;
use serde::Deserialize;
use util::ResultExt as _;

use crate::{
    DebugScenario, DebugTaskFile, EnvVariableReplacer, TcpArgumentsTemplate, VariableName,
};

// TODO support preLaunchTask linkage with other tasks
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct VsCodeDebugTaskDefinition {
    r#type: String,
    name: String,
    #[serde(default)]
    port: Option<u16>,
    #[serde(flatten)]
    other_attributes: serde_json::Value,
}

impl VsCodeDebugTaskDefinition {
    fn try_to_zed(mut self, replacer: &EnvVariableReplacer) -> anyhow::Result<DebugScenario> {
        let label = replacer.replace(&self.name);
        let mut config = replacer.replace_value(self.other_attributes);
        let adapter = task_type_to_adapter_name(&self.r#type);
        if let Some(config) = config.as_object_mut()
            && adapter == "JavaScript"
        {
            config.insert("type".to_owned(), self.r#type.clone().into());
            if let Some(port) = self.port.take() {
                config.insert("port".to_owned(), port.into());
            }
        }
        let definition = DebugScenario {
            label: label.into(),
            build: None,
            adapter: adapter.into(),
            tcp_connection: self.port.map(|port| TcpArgumentsTemplate {
                port: Some(port),
                host: None,
                timeout: None,
            }),
            config,
        };
        Ok(definition)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeDebugTaskFile {
    #[serde(default)]
    version: Option<String>,
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
            (
                "relativeFile".to_owned(),
                VariableName::RelativeFile.to_string(),
            ),
            ("file".to_owned(), VariableName::File.to_string()),
        ]));
        let templates = file
            .configurations
            .into_iter()
            .filter_map(|config| config.try_to_zed(&replacer).log_err())
            .collect::<Vec<_>>();
        Ok(DebugTaskFile(templates))
    }
}

fn task_type_to_adapter_name(task_type: &str) -> String {
    match task_type {
        "pwa-node" | "node" | "node-terminal" | "chrome" | "pwa-chrome" | "edge" | "pwa-edge"
        | "msedge" | "pwa-msedge" => "JavaScript",
        "go" => "Delve",
        "php" => "Xdebug",
        "cppdbg" | "lldb" => "CodeLLDB",
        "debugpy" => "Debugpy",
        "rdbg" => "rdbg",
        _ => task_type,
    }
    .to_owned()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{DebugScenario, DebugTaskFile};

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
            serde_json_lenient::from_str(raw).expect("deserializing launch.json");
        let zed = DebugTaskFile::try_from(parsed).expect("converting to Zed debug templates");
        pretty_assertions::assert_eq!(
            zed,
            DebugTaskFile(vec![DebugScenario {
                label: "Debug my JS app".into(),
                adapter: "JavaScript".into(),
                config: json!({
                    "request": "launch",
                    "program": "${ZED_WORKTREE_ROOT}/xyz.js",
                    "showDevDebugOutput": false,
                    "stopOnEntry": true,
                    "args": [
                        "--foo",
                        "${ZED_WORKTREE_ROOT}/thing",
                    ],
                    "cwd": "${ZED_WORKTREE_ROOT}/${FOO}/sub",
                    "env": {
                        "X": "Y",
                    },
                    "type": "node",
                    "port": 17,
                }),
                tcp_connection: None,
                build: None
            }])
        );
    }
}
