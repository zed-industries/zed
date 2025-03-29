use std::fmt::Write;
use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language::{Anchor, OutlineItem};
use language_model::LanguageModelRequestMessage;
use project::Project;
use rope::Point;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use util::markdown::MarkdownString;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OutlineToolInput {
    /// The relative path of the source code file to read. This tool should only
    /// be used on source code files, never on any other type of file.
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
    /// If you want to access `file.txt` in `directory1`, you should use the path `directory1/file.txt`.
    /// If you want to access `file.txt` in `directory2`, you should use the path `directory2/file.txt`.
    /// </example>
    pub path: Arc<Path>,
}

pub struct OutlineTool;

impl Tool for OutlineTool {
    fn name(&self) -> String {
        "outline-tool".into()
    }

    fn needs_confirmation(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./outline_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Eye
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(OutlineToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<OutlineToolInput>(input.clone()) {
            Ok(input) => {
                let path = MarkdownString::inline_code(&input.path.display().to_string());
                format!("Read outline for {path}")
            }
            Err(_) => "Read outline".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<OutlineToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!(
                "Path {} not found in project",
                &input.path.display()
            )));
        };

        cx.spawn(async move |cx| {
            let buffer = cx
                .update(|cx| {
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx))
                })?
                .await?;

            action_log.update(cx, |log, cx| {
                log.buffer_read(buffer.clone(), cx);
            })?;

            let string = buffer.read_with(cx, |buffer, _cx| {
                let snapshot = buffer.snapshot();

                to_outline_string(
                    snapshot
                        .outline(None)
                        .iter()
                        .flat_map(|outline| outline.items.iter())
                        .map(|item| (item, item.to_point(&snapshot))),
                )
            })?;

            if string.is_empty() {
                Err(anyhow!("There is no outline available for this file."))
            } else {
                Ok(string)
            }
        })
    }
}

fn to_outline_string<'a>(
    items: impl IntoIterator<Item = (&'a OutlineItem<Anchor>, OutlineItem<Point>)>,
) -> String {
    let mut buf = String::new();

    for (item, point_item) in items {
        // Add heading based on depth.
        // (Don't use indentation because models are more likely
        // to treat # for headings as semantic than spaces.)
        write!(buf, "{} {} ", "#".repeat(item.depth), item.text).ok();

        // Convert to 1-based line numbers.
        let start_line = point_item.range.start.row as usize + 1;
        let end_line = point_item.range.end.row as usize + 1;

        if start_line == end_line {
            writeln!(buf, "[L{}]", start_line).ok();
        } else {
            writeln!(buf, "[L{}-{}]", start_line, end_line).ok();
        }
    }

    buf
}
