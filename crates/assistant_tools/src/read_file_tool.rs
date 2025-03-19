use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use itertools::Itertools;
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadFileToolInput {
    /// The relative path of the file to read.
    ///
    /// This path should never be absolute, and the first component
    /// of the path should always be a root directory in a project.
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - directory1
    /// - directory2
    ///
    /// If you wanna access `file.txt` in `directory1`, you should use the path `directory1/file.txt`.
    /// If you wanna access `file.txt` in `directory2`, you should use the path `directory2/file.txt`.
    /// </example>
    pub path: Arc<Path>,

    /// Optional line number to start reading on (1-based index)
    #[serde(default)]
    pub start_line: Option<usize>,

    /// Optional line number to end reading on (1-based index)
    #[serde(default)]
    pub end_line: Option<usize>,
}

pub struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> String {
        "read-file".into()
    }

    fn description(&self) -> String {
        include_str!("./read_file_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(ReadFileToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<ReadFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!("Path not found in project")));
        };

        cx.spawn(|mut cx| async move {
            let buffer = cx
                .update(|cx| {
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx))
                })?
                .await?;

            let result = buffer.read_with(&cx, |buffer, _cx| {
                if buffer
                    .file()
                    .map_or(false, |file| file.disk_state().exists())
                {
                    let text = buffer.text();
                    let string = if input.start_line.is_some() || input.end_line.is_some() {
                        let start = input.start_line.unwrap_or(1);
                        let lines = text.split('\n').skip(start - 1);
                        if let Some(end) = input.end_line {
                            let count = end.saturating_sub(start);
                            Itertools::intersperse(lines.take(count), "\n").collect()
                        } else {
                            Itertools::intersperse(lines, "\n").collect()
                        }
                    } else {
                        text
                    };

                    Ok(string)
                } else {
                    Err(anyhow!("File does not exist"))
                }
            })??;

            action_log.update(&mut cx, |log, cx| {
                log.buffer_read(buffer, cx);
            })?;

            anyhow::Ok(result)
        })
    }
}
