use std::fmt::Write;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language::LanguageRegistry;
use language_model::LanguageModelRequestMessage;
use project::{DocumentSymbol, Project, Symbol};
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
        "code-symbols".into()
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
                let mut symbols_by_file: HashMap<String, Vec<&Symbol>> = HashMap::new();

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

                    // Add all symbols for this file with their labels
                    for symbol in file_symbols {
                        // Use the symbol's existing label instead of debug formatting the kind
                        let kind_str = format!("{} ", symbol.label.text());

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

            // Get the language for this buffer
            let language = buffer.read_with(cx, |buffer, _| buffer.language().cloned())?;
            let language_registry = project.read_with(cx, |project, _| project.languages().clone())?;

            // Convert the document symbols to a hierarchical outline
            let outline = render_outline(&symbols, language, language_registry).await?;
            
            Ok(outline)
        })
    }
}

// Avoid async recursion by splitting into an async function that gets labels,
// and a non-async function that formats the output using those labels
async fn render_outline(
    symbols: &[DocumentSymbol],
    language: Option<Arc<language::Language>>,
    language_registry: Arc<LanguageRegistry>,
) -> Result<String> {
    // Collect all symbols (flattened) to get labels for them all at once
    let mut all_symbols = Vec::new();
    collect_symbols_recursive(symbols, &mut all_symbols);

    // Create a list of symbol name/kind pairs for generating labels
    let label_params: Vec<(String, _)> = all_symbols
        .iter()
        .map(|symbol| (symbol.name.clone(), symbol.kind))
        .collect();
    
    // Get labels for the symbols if we have a language with an adapter
    let labels = if let Some(language) = &language {
        let lsp_adapter = language_registry
            .lsp_adapters(&language.name())
            .first()
            .cloned();

        if let Some(lsp_adapter) = lsp_adapter {
            match lsp_adapter.labels_for_symbols(&label_params, language).await {
                Ok(labels) => labels,
                Err(_) => vec![None; label_params.len()],
            }
        } else {
            vec![None; label_params.len()]
        }
    } else {
        vec![None; label_params.len()]
    };

    // Format output with the retrieved labels
    let mut output = String::new();
    let mut symbol_index = 0;
    render_symbols(symbols, 1, &mut output, &labels, &mut symbol_index);
    Ok(output)
}

// Helper function to collect all symbols in a flattened list
fn collect_symbols_recursive(symbols: &[DocumentSymbol], all_symbols: &mut Vec<DocumentSymbol>) {
    for symbol in symbols {
        all_symbols.push(symbol.clone());
        collect_symbols_recursive(&symbol.children, all_symbols);
    }
}

// Non-async function to format symbols with their labels
fn render_symbols(
    symbols: &[DocumentSymbol],
    depth: usize,
    output: &mut String,
    labels: &[Option<language::CodeLabel>],
    symbol_index: &mut usize,
) {
    for symbol in symbols {
        // Get the current symbol's index
        let current_index = *symbol_index;
        *symbol_index += 1;

        // Add heading based on depth (# for level 1, ## for level 2, etc.)
        let kind_str = if let Some(Some(label)) = labels.get(current_index) {
            label.text().to_string()
        } else {
            format!("{:?}", symbol.kind)
        };
        
        let display_text = format!("{} {}", kind_str, symbol.name);

        write!(output, "{} {}", "#".repeat(depth), display_text).ok();

        // Convert to 1-based line numbers for display
        let start_line = symbol.range.start.0.row as usize + 1;
        let end_line = symbol.range.end.0.row as usize + 1;

        if start_line == end_line {
            writeln!(output, " [L{}]", start_line).ok();
        } else {
            writeln!(output, " [L{}-{}]", start_line, end_line).ok();
        }

        // Recursively process children with increased depth
        if !symbol.children.is_empty() {
            render_symbols(&symbol.children, depth + 1, output, labels, symbol_index);
        }
    }
}