use std::fmt::Write;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::{DocumentSymbol, Project};
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use util::markdown::MarkdownString;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CodeSymbolsInput {
    /// The relative path of the source code file to read and get the symbols for.
    /// This tool should only be used on source code files, never on any other type of file.
    ///
    /// This path should never be absolute, and the first component
    /// of the path should always be a root directory in a project.
    ///
    /// If no path is specified, this tool returns a flat list of all symbols in the project
    /// instead of a hierarchical outline of a specific file.
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
    #[serde(default)]
    pub path: Option<String>,
    
    /// Optional regex pattern to filter symbols by name.
    /// When provided, only symbols whose names match this pattern will be included in the results.
    ///
    /// <example>
    /// To find only symbols that contain the word "test", use the regex pattern "test".
    /// To find methods that start with "get_", use the regex pattern "^get_".
    /// </example>
    #[serde(default)]
    pub regex: Option<String>,
}

pub struct CodeSymbolsTool;

impl Tool for CodeSymbolsTool {
    fn name(&self) -> String {
        "outline-tool".into()
    }

    fn needs_confirmation(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./code_symbols_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Eye
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(CodeSymbolsInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<CodeSymbolsInput>(input.clone()) {
            Ok(input) => match &input.path {
                Some(path) => {
                    let path = MarkdownString::inline_code(path);
                    format!("Read outline for {path}")
                }
                None => "List all project symbols".to_string(),
            },
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
        let input = match serde_json::from_value::<CodeSymbolsInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        // If no path is specified, list all project symbols instead of a file outline
        if input.path.is_none() {
            return cx.spawn(async move |cx| {
                let symbols = project
                    .update(cx, |project, cx| project.symbols("", cx))?
                    .await?;

                if symbols.is_empty() {
                    return Err(anyhow!("No symbols found in project."));
                }

                // If regex is provided, prepare it for filtering
                let regex_filter = match input.regex {
                    Some(regex_str) => {
                        match Regex::new(&regex_str) {
                            Ok(re) => Some(re),
                            Err(err) => return Err(anyhow!("Invalid regex pattern: {}", err)),
                        }
                    },
                    None => None,
                };

                // Group symbols by file path
                use std::collections::HashMap;
                let mut symbols_by_file: HashMap<String, Vec<&project::Symbol>> = HashMap::new();
                
                // First, filter and group symbols by file
                for symbol in &symbols {
                    // Skip this symbol if it doesn't match the regex filter
                    if let Some(re) = &regex_filter {
                        if !re.is_match(&symbol.name) {
                            continue;
                        }
                    }
                    
                    let worktree_name = project.read_with(cx, |project, cx| {
                        project
                            .worktree_for_id(symbol.path.worktree_id, cx)
                            .map(|worktree| worktree.read(cx).root_name().to_string())
                            .unwrap_or_default()
                    })?;

                    let path = format!("{}/{}", worktree_name, symbol.path.path.to_string_lossy());
                    symbols_by_file.entry(path).or_default().push(symbol);
                }
                
                // If no symbols matched the filter, return early
                if symbols_by_file.is_empty() {
                    return Err(anyhow!("No symbols found matching the criteria."));
                }

                // Now render the grouped symbols
                let mut output = String::new();
                for (file_path, file_symbols) in symbols_by_file {
                    // Extract the filename from the path for the heading
                    let filename = file_symbols[0].path.path.file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                        
                    // Add a heading for the file
                    writeln!(&mut output, "# File: {} ({})", filename, file_path).ok();
                    
                    // Add all symbols for this file
                    for symbol in file_symbols {
                        let kind_str = format!("{:?} ", symbol.kind);
                        
                        // Convert to 1-based line numbers for display
                        let start_line = symbol.range.start.0.row as usize + 1;
                        let end_line = symbol.range.end.0.row as usize + 1;
                        
                        // Write the symbol with indentation
                        if start_line == end_line {
                            writeln!(
                                &mut output,
                                "## {}{} [L{}]",
                                kind_str, symbol.name, start_line
                            )
                            .ok();
                        } else {
                            writeln!(
                                &mut output,
                                "## {}{} [L{}-{}]",
                                kind_str, symbol.name, start_line, end_line
                            )
                            .ok();
                        }
                    }
                    
                    // Add a blank line between files for readability
                    writeln!(&mut output).ok();
                }
                
                Ok(output)
            });
        }

        // Handle the case with a specified path (existing file outline functionality)
        cx.spawn(async move |cx| {
            let buffer = {
                let project_path = project.read_with(cx, |project, cx| {
                    project
                        .find_project_path(input.path.as_ref().unwrap(), cx)
                        .ok_or_else(|| {
                            anyhow!("Path {} not found in project", input.path.as_ref().unwrap())
                        })
                })??;

                project
                    .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                    .await?
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
        let display_text = format!("{:?} {}", symbol.kind, symbol.name);

        write!(output, "{} {}", "#".repeat(depth), display_text).ok();

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
