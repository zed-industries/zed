use std::fmt::Write;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::{DocumentSymbol, Project};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use util::markdown::MarkdownString;

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

        cx.spawn(async move |cx| {
            let buffer = {
                let project_path = project.read_with(cx, |project, cx| {
                    project
                        .find_project_path(&input.path, cx)
                        .ok_or_else(|| anyhow!("Path {} not found in project", &input.path))
                })??;

                project.update(cx, |project, cx| project.open_buffer(project_path, cx))?.await?
            };

            action_log.update(cx, |action_log, cx| {
                action_log.buffer_read(buffer.clone(), cx);
            })?;

            // Check if the file is empty
            if buffer.read_with(cx, |buffer, _| buffer.snapshot().is_empty())? {
                return Err(anyhow!("This file is empty."));
            }

            // Request document symbols from the language server
            let symbols = project
                .update(cx, |project, cx| project.document_symbols(&buffer, cx))?
                .await?;

            if symbols.is_empty() {
                return Err(anyhow!("No outline information available for this file."));
            }

            // Convert the document symbols to a hierarchical outline
            let outline = render_outline(&symbols);

            Ok(outline)
        })
    }
}

fn render_outline(symbols: &[DocumentSymbol]) -> String {
    let mut output = String::new();
    render_symbols(symbols, 1, &mut output);
    output
}

fn render_symbols(symbols: &[DocumentSymbol], depth: usize, output: &mut String) {
    for symbol in symbols {
        // Add heading based on depth (# for level 1, ## for level 2, etc.)
        write!(output, "{} ", "#".repeat(depth)).ok();
        
        // The outline panel doesn't modify the text or add symbol type information
        // in its rendering; the text already includes that information from the
        // language server
        write!(output, "{} ", symbol.name).ok();

        // Convert to 1-based line numbers for display
        let start_line = symbol.range.start.0.row as usize + 1;
        let end_line = symbol.range.end.0.row as usize + 1;

        if start_line == end_line {
            writeln!(output, "[L{}]", start_line).ok();
        } else {
            writeln!(output, "[L{}-{}]", start_line, end_line).ok();
        }

        // Recursively process children with increased depth
        if !symbol.children.is_empty() {
            render_symbols(&symbol.children, depth + 1, output);
        }
    }
}