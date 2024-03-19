use anyhow::bail;
use collections::HashMap;
use serde::Deserialize;
use util::ResultExt;

use crate::static_source::{Definition, DefinitionProvider};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct TaskOptions {
    cwd: Option<String>,
    #[serde(default)]
    env: HashMap<String, String>,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeTaskDefinition {
    label: String,
    #[serde(flatten)]
    command: Option<Command>,
    #[serde(flatten)]
    other_attributes: HashMap<String, serde_json_lenient::Value>,
    options: Option<TaskOptions>,
}

#[derive(Clone, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum Command {
    Npm {
        script: String,
    },
    Shell {
        command: String,
        #[serde(default)]
        args: Vec<String>,
    },
    Gulp {
        task: String,
    },
}

impl TryFrom<VsCodeTaskDefinition> for Definition {
    type Error = anyhow::Error;

    fn try_from(value: VsCodeTaskDefinition) -> Result<Self, Self::Error> {
        if value.other_attributes.contains_key("dependsOn") {
            bail!("Encountered dependsOn key during deserialization");
        }
        let Some(command) = value.command else {
            bail!("Missing `type` field in task");
        };

        let (command, args) = match command {
            Command::Npm { script } => ("npm".to_owned(), vec!["run".to_string(), script]),
            Command::Shell { command, args } => (command, args),
            Command::Gulp { task } => ("gulp".to_owned(), vec![task]),
        };
        let mut ret = Self {
            label: value.label,
            command,
            args,
            ..Default::default()
        };
        if let Some(options) = value.options {
            ret.cwd = options.cwd;
            ret.env = options.env;
        }
        Ok(ret)
    }
}
// https://github.com/microsoft/TypeScript/blob/main/.vscode/tasks.json
#[derive(Debug, Deserialize, PartialEq)]
/// TODO: docs for this
pub struct VsCodeTaskFile {
    tasks: Vec<VsCodeTaskDefinition>,
}

impl TryFrom<VsCodeTaskFile> for DefinitionProvider {
    type Error = anyhow::Error;

    fn try_from(value: VsCodeTaskFile) -> Result<Self, Self::Error> {
        let definitions = value
            .tasks
            .into_iter()
            .filter_map(|vscode_definition| vscode_definition.try_into().log_err())
            .collect();
        Ok(Self(definitions))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        static_source::{Definition, DefinitionProvider},
        vscode_format::{Command, VsCodeTaskDefinition},
        VsCodeTaskFile,
    };

    fn compare_without_other_attributes(lhs: VsCodeTaskDefinition, rhs: VsCodeTaskDefinition) {
        assert_eq!(
            VsCodeTaskDefinition {
                other_attributes: Default::default(),
                ..lhs
            },
            VsCodeTaskDefinition {
                other_attributes: Default::default(),
                ..rhs
            },
        );
    }
    #[test]
    fn can_deserialize_ts_tasks() {
        static TYPESCRIPT_TASKS: &'static str = include_str!("../test_data/typescript.json");
        let vscode_definitions: VsCodeTaskFile =
            serde_json_lenient::from_str(&TYPESCRIPT_TASKS).unwrap();

        let expected = vec![
            VsCodeTaskDefinition {
                label: "gulp: tests".to_string(),
                command: Some(Command::Npm {
                    script: "build:tests:notypecheck".to_string(),
                }),
                other_attributes: Default::default(),
                options: None,
            },
            VsCodeTaskDefinition {
                label: "tsc: watch ./src".to_string(),
                command: Some(Command::Shell {
                    command: "node".to_string(),
                    args: vec![
                        "${workspaceFolder}/node_modules/typescript/lib/tsc.js".to_string(),
                        "--build".to_string(),
                        "${workspaceFolder}/src".to_string(),
                        "--watch".to_string(),
                    ],
                }),
                other_attributes: Default::default(),
                options: None,
            },
            VsCodeTaskDefinition {
                label: "npm: build:compiler".to_string(),
                command: Some(Command::Npm {
                    script: "build:compiler".to_string(),
                }),
                other_attributes: Default::default(),
                options: None,
            },
            VsCodeTaskDefinition {
                label: "npm: build:tests".to_string(),
                command: Some(Command::Npm {
                    script: "build:tests:notypecheck".to_string(),
                }),
                other_attributes: Default::default(),
                options: None,
            },
        ];

        assert_eq!(vscode_definitions.tasks.len(), expected.len());
        vscode_definitions
            .tasks
            .iter()
            .zip(expected.into_iter())
            .for_each(|(lhs, rhs)| compare_without_other_attributes(lhs.clone(), rhs));

        let expected = vec![
            Definition {
                label: "gulp: tests".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "build:tests:notypecheck".to_string()],
                ..Default::default()
            },
            Definition {
                label: "tsc: watch ./src".to_string(),
                command: "node".to_string(),
                args: vec![
                    "${workspaceFolder}/node_modules/typescript/lib/tsc.js".to_string(),
                    "--build".to_string(),
                    "${workspaceFolder}/src".to_string(),
                    "--watch".to_string(),
                ],
                ..Default::default()
            },
            Definition {
                label: "npm: build:compiler".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "build:compiler".to_string()],
                ..Default::default()
            },
            Definition {
                label: "npm: build:tests".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "build:tests:notypecheck".to_string()],
                ..Default::default()
            },
        ];

        let tasks: DefinitionProvider = vscode_definitions.try_into().unwrap();
        assert_eq!(tasks.0, expected);
    }

    #[test]
    fn can_deserialize_rust_analyzer_tasks() {
        static RUST_ANALYZER_TASKS: &'static str = include_str!("../test_data/rust-analyzer.json");
        let vscode_definitions: VsCodeTaskFile =
            serde_json_lenient::from_str(&RUST_ANALYZER_TASKS).unwrap();
        let expected = vec![
            VsCodeTaskDefinition {
                label: "Build Extension in Background".to_string(),
                command: Some(Command::Npm {
                    script: "watch".to_string(),
                }),
                options: None,
                other_attributes: Default::default(),
            },
            VsCodeTaskDefinition {
                label: "Build Extension".to_string(),
                command: Some(Command::Npm {
                    script: "build".to_string(),
                }),
                options: None,
                other_attributes: Default::default(),
            },
            VsCodeTaskDefinition {
                label: "Build Server".to_string(),
                command: Some(Command::Shell {
                    command: "cargo build --package rust-analyzer".to_string(),
                    args: Default::default(),
                }),
                options: None,
                other_attributes: Default::default(),
            },
            VsCodeTaskDefinition {
                label: "Build Server (Release)".to_string(),
                command: Some(Command::Shell {
                    command: "cargo build --release --package rust-analyzer".to_string(),
                    args: Default::default(),
                }),
                options: None,
                other_attributes: Default::default(),
            },
            VsCodeTaskDefinition {
                label: "Pretest".to_string(),
                command: Some(Command::Npm {
                    script: "pretest".to_string(),
                }),
                options: None,
                other_attributes: Default::default(),
            },
            VsCodeTaskDefinition {
                label: "Build Server and Extension".to_string(),
                command: None,
                options: None,
                other_attributes: Default::default(),
            },
            VsCodeTaskDefinition {
                label: "Build Server (Release) and Extension".to_string(),
                command: None,
                options: None,
                other_attributes: Default::default(),
            },
        ];
        assert_eq!(vscode_definitions.tasks.len(), expected.len());
        vscode_definitions
            .tasks
            .iter()
            .zip(expected.into_iter())
            .for_each(|(lhs, rhs)| compare_without_other_attributes(lhs.clone(), rhs));
        let expected = vec![
            Definition {
                label: "Build Extension in Background".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "watch".to_string()],
                ..Default::default()
            },
            Definition {
                label: "Build Extension".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "build".to_string()],
                ..Default::default()
            },
            Definition {
                label: "Build Server".to_string(),
                command: "cargo build --package rust-analyzer".to_string(),
                ..Default::default()
            },
            Definition {
                label: "Build Server (Release)".to_string(),
                command: "cargo build --release --package rust-analyzer".to_string(),
                ..Default::default()
            },
            Definition {
                label: "Pretest".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "pretest".to_string()],
                ..Default::default()
            },
        ];
        let tasks: DefinitionProvider = vscode_definitions.try_into().unwrap();
        assert_eq!(tasks.0, expected);
    }
}
