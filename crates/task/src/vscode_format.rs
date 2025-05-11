use anyhow::bail;
use collections::HashMap;
use serde::Deserialize;
use util::ResultExt;

use crate::{EnvVariableReplacer, TaskTemplate, TaskTemplates, VariableName};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct TaskOptions {
    cwd: Option<String>,
    #[serde(default)]
    env: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct VsCodeTaskDefinition {
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

impl VsCodeTaskDefinition {
    fn into_zed_format(self, replacer: &EnvVariableReplacer) -> anyhow::Result<TaskTemplate> {
        if self.other_attributes.contains_key("dependsOn") {
            bail!("Encountered unsupported `dependsOn` key during deserialization");
        }
        // `type` might not be set in e.g. tasks that use `dependsOn`; we still want to deserialize the whole object though (hence command is an Option),
        // as that way we can provide more specific description of why deserialization failed.
        // E.g. if the command is missing due to `dependsOn` presence, we can check other_attributes first before doing this (and provide nice error message)
        // catch-all if on value.command presence.
        let Some(command) = self.command else {
            bail!("Missing `type` field in task");
        };

        let (command, args) = match command {
            Command::Npm { script } => ("npm".to_owned(), vec!["run".to_string(), script]),
            Command::Shell { command, args } => (command, args),
            Command::Gulp { task } => ("gulp".to_owned(), vec![task]),
        };
        // Per VSC docs, only `command`, `args` and `options` support variable substitution.
        let command = replacer.replace(&command);
        let args = args.into_iter().map(|arg| replacer.replace(&arg)).collect();
        let mut ret = TaskTemplate {
            label: self.label,
            command,
            args,
            ..Default::default()
        };
        if let Some(options) = self.options {
            ret.cwd = options.cwd.map(|cwd| replacer.replace(&cwd));
            ret.env = options.env;
        }
        Ok(ret)
    }
}

/// [`VsCodeTaskFile`] is a superset of Code's task definition format.
#[derive(Debug, Deserialize, PartialEq)]
pub struct VsCodeTaskFile {
    tasks: Vec<VsCodeTaskDefinition>,
}

impl TryFrom<VsCodeTaskFile> for TaskTemplates {
    type Error = anyhow::Error;

    fn try_from(value: VsCodeTaskFile) -> Result<Self, Self::Error> {
        let replacer = EnvVariableReplacer::new(HashMap::from_iter([
            (
                "workspaceFolder".to_owned(),
                VariableName::WorktreeRoot.to_string(),
            ),
            ("file".to_owned(), VariableName::File.to_string()),
            ("lineNumber".to_owned(), VariableName::Row.to_string()),
            (
                "selectedText".to_owned(),
                VariableName::SelectedText.to_string(),
            ),
        ]));
        let templates = value
            .tasks
            .into_iter()
            .filter_map(|vscode_definition| vscode_definition.into_zed_format(&replacer).log_err())
            .collect();
        Ok(Self(templates))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        TaskTemplate, TaskTemplates, VsCodeTaskFile,
        vscode_format::{Command, VsCodeTaskDefinition},
    };

    use super::EnvVariableReplacer;

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
    fn test_variable_substitution() {
        let replacer = EnvVariableReplacer::new(Default::default());
        assert_eq!(replacer.replace("Food"), "Food");
        // Unknown variables are left in tact.
        assert_eq!(
            replacer.replace("$PATH is an environment variable"),
            "$PATH is an environment variable"
        );
        assert_eq!(replacer.replace("${PATH}"), "${PATH}");
        assert_eq!(replacer.replace("${PATH:food}"), "${PATH:food}");
        // And now, the actual replacing
        let replacer = EnvVariableReplacer::new(HashMap::from_iter([(
            "PATH".to_owned(),
            "ZED_PATH".to_owned(),
        )]));
        assert_eq!(replacer.replace("Food"), "Food");
        assert_eq!(
            replacer.replace("$PATH is an environment variable"),
            "${ZED_PATH} is an environment variable"
        );
        assert_eq!(replacer.replace("${PATH}"), "${ZED_PATH}");
        assert_eq!(replacer.replace("${PATH:food}"), "${ZED_PATH:food}");
    }

    #[test]
    fn can_deserialize_ts_tasks() {
        const TYPESCRIPT_TASKS: &str = include_str!("../test_data/typescript.json");
        let vscode_definitions: VsCodeTaskFile =
            serde_json_lenient::from_str(TYPESCRIPT_TASKS).unwrap();

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
            .zip(expected)
            .for_each(|(lhs, rhs)| compare_without_other_attributes(lhs.clone(), rhs));

        let expected = vec![
            TaskTemplate {
                label: "gulp: tests".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "build:tests:notypecheck".to_string()],
                ..Default::default()
            },
            TaskTemplate {
                label: "tsc: watch ./src".to_string(),
                command: "node".to_string(),
                args: vec![
                    "${ZED_WORKTREE_ROOT}/node_modules/typescript/lib/tsc.js".to_string(),
                    "--build".to_string(),
                    "${ZED_WORKTREE_ROOT}/src".to_string(),
                    "--watch".to_string(),
                ],
                ..Default::default()
            },
            TaskTemplate {
                label: "npm: build:compiler".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "build:compiler".to_string()],
                ..Default::default()
            },
            TaskTemplate {
                label: "npm: build:tests".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "build:tests:notypecheck".to_string()],
                ..Default::default()
            },
        ];

        let tasks: TaskTemplates = vscode_definitions.try_into().unwrap();
        assert_eq!(tasks.0, expected);
    }

    #[test]
    fn can_deserialize_rust_analyzer_tasks() {
        const RUST_ANALYZER_TASKS: &str = include_str!("../test_data/rust-analyzer.json");
        let vscode_definitions: VsCodeTaskFile =
            serde_json_lenient::from_str(RUST_ANALYZER_TASKS).unwrap();
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
            .zip(expected)
            .for_each(|(lhs, rhs)| compare_without_other_attributes(lhs.clone(), rhs));
        let expected = vec![
            TaskTemplate {
                label: "Build Extension in Background".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "watch".to_string()],
                ..Default::default()
            },
            TaskTemplate {
                label: "Build Extension".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "build".to_string()],
                ..Default::default()
            },
            TaskTemplate {
                label: "Build Server".to_string(),
                command: "cargo build --package rust-analyzer".to_string(),
                ..Default::default()
            },
            TaskTemplate {
                label: "Build Server (Release)".to_string(),
                command: "cargo build --release --package rust-analyzer".to_string(),
                ..Default::default()
            },
            TaskTemplate {
                label: "Pretest".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "pretest".to_string()],
                ..Default::default()
            },
        ];
        let tasks: TaskTemplates = vscode_definitions.try_into().unwrap();
        assert_eq!(tasks.0, expected);
    }
}
