use anyhow::bail;
use collections::HashMap;
use serde::Deserialize;
use util::ResultExt;

use crate::static_source::{Definition, DefinitionProvider};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct TaskOptions {
    cwd: Option<String>,
    #[serde(default)]
    env: HashMap<String, String>,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeTaskDefinition {
    label: String,
    #[serde(flatten)]
    command: Option<Command>,
    #[serde(flatten)]
    other_attributes: HashMap<String, serde_json_lenient::Value>,
    options: Option<TaskOptions>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum Command {
    Npm { script: String },
    Shell { command: String, args: Vec<String> },
    Gulp { task: String },
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
        vscode_format::{Command, VsCodeTaskDefinition},
        VsCodeTaskFile,
    };

    #[test]
    fn can_deserialize_ts_tasks() {
        static TYPESCRIPT_TASKS: &'static str = include_str!("../test_data/typescript.json");
        let vscode_definitions: VsCodeTaskFile =
            serde_json_lenient::from_str(&TYPESCRIPT_TASKS).unwrap();
        assert_eq!(
            vscode_definitions.tasks,
            vec![
                VsCodeTaskDefinition {
                    label: "gulp: tests".to_string(),
                    command: Some(Command::Npm {
                        script: "foo".to_string()
                    }),
                    other_attributes: Default::default(),
                    options: None,
                },
                VsCodeTaskDefinition {
                    label: "tsc: watch ./src".to_string(),
                    command: Some(Command::Npm {
                        script: "foo".to_string()
                    }),
                    other_attributes: Default::default(),
                    options: None,
                },
                VsCodeTaskDefinition {
                    label: "npm: build:compiler".to_string(),
                    command: Some(Command::Npm {
                        script: "foo".to_string()
                    }),
                    other_attributes: Default::default(),
                    options: None,
                },
                VsCodeTaskDefinition {
                    label: "npm: build:tests".to_string(),
                    command: Some(Command::Npm {
                        script: "foo".to_string()
                    }),
                    other_attributes: Default::default(),
                    options: None,
                }
            ]
        );
    }

    #[test]
    fn can_deserialize_rust_analyzer_tasks() {
        static RUSTANALYZER_TASKS: &'static str = include_str!("../test_data/rust-analyzer.json");
    }
}
