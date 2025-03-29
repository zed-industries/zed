use std::fmt::Write;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language::{Anchor, BufferSnapshot, Outline};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use util::markdown::MarkdownString;

const LANGUAGE_SERVER_RETRIES: [Duration; 4] = [
    Duration::from_millis(100),
    Duration::from_millis(500),
    Duration::from_millis(1000),
    Duration::from_millis(2000),
];

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OutlineToolInput {
    /// The relative path of the source code file to read and get the outline for.
    /// This tool should only be used on source code files, never on any other type of file.
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
    /// If you want to access `file.md` in `directory1`, you should use the path `directory1/file.md`.
    /// If you want to access `file.md` in `directory2`, you should use the path `directory2/file.md`.
    /// </example>
    pub path: String,
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
                let path = MarkdownString::inline_code(&input.path);
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
            return Task::ready(Err(anyhow!("Path {} not found in project", &input.path)));
        };

        cx.spawn(async move |cx| {
            let buffer = cx
                .update(|cx| {
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx))
                })?
                .await?;

            action_log.update(cx, |action_log, cx| {
                action_log.buffer_read(buffer.clone(), cx);
            })?;

            // If we just opened the buffer, language server information might not be available
            // right away. When that happens, we poll it with increasing delays until either it
            // succeeds or we give up and time out.
            for retry_delay in LANGUAGE_SERVER_RETRIES {
                let (outline, snapshot) = buffer.read_with(cx, |buffer, _cx| {
                    let snapshot = buffer.snapshot();
                    (snapshot.outline(None), snapshot)
                })?;

                if snapshot.is_empty() {
                    return Err(anyhow!("This file is empty."));
                }

                if let Some(outline) = outline {
                    let string = to_outline_string(outline, &snapshot);

                    if !string.is_empty() {
                        return Ok(string);
                    }
                }

                log::info!(
                    "Outline information not available yet for {}. Retrying in {:?}.",
                    &input.path,
                    retry_delay
                );

                cx.background_executor().timer(retry_delay).await;
            }

            Err(anyhow!(
                "Timed out waiting for the language server to provide outline information on this file."
            ))
        })
    }
}

fn to_outline_string(outline: Outline<Anchor>, snapshot: &BufferSnapshot) -> String {
    let mut buf = String::new();

    for item in outline.items.iter() {
        let point_item = item.to_point(snapshot);
        // Add heading based on depth. (Don't use indentation, because models are
        // more likely to treat # for headings as semantic than spaces.)
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
