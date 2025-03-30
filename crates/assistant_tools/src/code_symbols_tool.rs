use std::cmp;
use std::fmt::{self, Write};
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language::{CodeLabel, Language, LanguageRegistry};
use language_model::LanguageModelRequestMessage;
use lsp::SymbolKind;
use project::{DocumentSymbol, Project, Symbol};
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use util::markdown::MarkdownString;

use crate::code_symbol_iter::{CodeSymbolIterator, Entry};

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

    /// Optional starting position for paginated results (0-based).
    /// When not provided, starts from the beginning.
    #[serde(default)]
    pub offset: Option<u32>,
}

impl CodeSymbolsInput {
    /// Which page of search results this is.
    pub fn page(&self) -> u32 {
        1 + (self.offset.unwrap_or(0) / RESULTS_PER_PAGE)
    }
}

const RESULTS_PER_PAGE: u32 = 2000;

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
            Ok(input) => {
                let page = input.page();

                match &input.path {
                    Some(path) => {
                        let path = MarkdownString::inline_code(path);
                        if page > 1 {
                            format!("List page {page} of code symbols for {path}")
                        } else {
                            format!("List code symbols for {path}")
                        }
                    }
                    None => {
                        if page > 1 {
                            format!("List page {page} of project symbols")
                        } else {
                            "List all project symbols".to_string()
                        }
                    }
                }
            }
            Err(_) => "List code symbols".to_string(),
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
                    Some(regex_str) => match Regex::new(&regex_str) {
                        Ok(re) => Some(re),
                        Err(err) => return Err(anyhow!("Invalid regex pattern: {}", err)),
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

                let offset = input.offset.unwrap_or(0);
                let mut skips_remaining = offset;
                let mut symbols_rendered = 0;
                let mut has_more_symbols = false;
                let mut output = String::new();

                for (file_path, file_symbols) in symbols_by_file {
                    // Track symbols in this file
                    let mut file_symbols_rendered = 0;
                    let mut file_header_written = false;

                    // Process symbols for this file
                    for symbol in file_symbols {
                        if skips_remaining > 0 {
                            skips_remaining -= 1;
                            continue;
                        }

                        // Check if we've already rendered a full page
                        if symbols_rendered >= RESULTS_PER_PAGE {
                            has_more_symbols = true;
                            break;
                        }

                        // Write file header only when we're going to include symbols from this file
                        if !file_header_written {
                            // Extract the filename from the path for the heading
                            let filename = symbol
                                .path
                                .path
                                .file_name()
                                .map(|f| f.to_string_lossy().to_string())
                                .unwrap_or_else(|| "unknown".to_string());

                            // Add a heading for the file
                            writeln!(&mut output, "# File: {} ({})", filename, file_path).ok();
                            file_header_written = true;
                        }

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

                        symbols_rendered += 1;
                        file_symbols_rendered += 1;
                    }

                    // Add a blank line between files for readability if we rendered symbols from this file
                    if file_symbols_rendered > 0 {
                        writeln!(&mut output).ok();
                    }

                    // Check if we need to stop after this file
                    if has_more_symbols {
                        break;
                    }
                }

                if symbols_rendered == 0 {
                    Ok("No symbols found in the requested page.".to_string())
                } else if has_more_symbols {
                    let result = format!(
                        "{}Showing symbols {}-{} (there were more symbols found; use offset: {} to see next page)",
                        output,
                        offset + 1,
                        offset + symbols_rendered,
                        offset + RESULTS_PER_PAGE,
                    );
                    Ok(result)
                } else {
                    let total = offset + symbols_rendered;
                    let result = format!("{}Found {} total symbols", output, total);
                    Ok(result)
                }
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
            let language_registry =
                project.read_with(cx, |project, _| project.languages().clone())?;

            // Convert the document symbols to a hierarchical outline with pagination
            let outline = render_outline(&symbols, language, language_registry, &input).await?;

            Ok(outline)
        })
    }
}

async fn render_outline(
    symbols: &[DocumentSymbol],
    language: Option<Arc<Language>>,
    registry: Arc<LanguageRegistry>,
    input: &CodeSymbolsInput,
) -> Result<String> {
    let regex = match &input.regex {
        Some(regex_str) => match Regex::new(regex_str) {
            Ok(regex) => Some(regex),
            Err(err) => return Err(anyhow!("Invalid regex: {err}")),
        },
        None => None,
    };
    let offset = input.offset.unwrap_or(0);
    let entries = CodeSymbolIterator::new(symbols, None)
        .skip(offset as usize)
        // Get 1 more than RESULTS_PER_PAGE so we can tell if there are more results.
        .take((RESULTS_PER_PAGE as usize).saturating_add(1));
    let labels = match language.and_then(|lang| registry.lsp_adapters(&lang.name()).first()) {
        Some(lsp_adapter) => lsp_adapter
            .labels_for_symbols(entries.clone(), lang)
            .await
            .ok(),
        None => None,
    };

    let mut output = String::new();
    let mut has_more = match labels {
        Some(labels) => render_entries(&mut output, entries.zip(labels.iter())),
        None => render_entries(&mut output, entries.map(|entry| (entry, None))),
    };

    if has_more {
        writeln!(&mut output, "\nShowing symbols {}-{} (there were more symbols found; use offset: {} to see next page)",
            offset + 1,
            page_end,
            page_end
        )
    } else {
        writeln!(
            &mut output,
            "\nShowing symbols {}-{} (total symbols: {})",
            offset + 1,
            page_end,
            total_symbols
        )
    }
    .ok();

    Ok(output)
}

fn gather_symbols(
    symbols: impl IntoIterator<Item = DocumentSymbol>,
    predicate: impl Clone + Fn(&DocumentSymbol) -> bool,
    all_symbols: &mut Vec<DocumentSymbol>,
) {
    for symbol in symbols.into_iter().filter(|symbol| predicate(&symbol)) {
        all_symbols.push(symbol);
    }
}

// If we don't know the symbol kind,
fn write_symbol_kind(buf: &mut String, kind: lsp::SymbolKind) -> Result<(), fmt::Error> {
    match kind {
        SymbolKind::FILE => write!(buf, "file "),
        SymbolKind::MODULE => write!(buf, "module "),
        SymbolKind::NAMESPACE => write!(buf, "namespace "),
        SymbolKind::PACKAGE => write!(buf, "package "),
        SymbolKind::CLASS => write!(buf, "class "),
        SymbolKind::METHOD => write!(buf, "method "),
        SymbolKind::PROPERTY => write!(buf, "property "),
        SymbolKind::FIELD => write!(buf, "field "),
        SymbolKind::CONSTRUCTOR => write!(buf, "constructor "),
        SymbolKind::ENUM => write!(buf, "enum "),
        SymbolKind::INTERFACE => write!(buf, "interface "),
        SymbolKind::FUNCTION => write!(buf, "function "),
        SymbolKind::VARIABLE => write!(buf, "variable "),
        SymbolKind::CONSTANT => write!(buf, "constant "),
        SymbolKind::STRING => write!(buf, "string "),
        SymbolKind::NUMBER => write!(buf, "number "),
        SymbolKind::BOOLEAN => write!(buf, "boolean "),
        SymbolKind::ARRAY => write!(buf, "array "),
        SymbolKind::OBJECT => write!(buf, "object "),
        SymbolKind::KEY => write!(buf, "key "),
        SymbolKind::NULL => write!(buf, "null "),
        SymbolKind::ENUM_MEMBER => write!(buf, "enum member "),
        SymbolKind::STRUCT => write!(buf, "struct "),
        SymbolKind::EVENT => write!(buf, "event "),
        SymbolKind::OPERATOR => write!(buf, "operator "),
        SymbolKind::TYPE_PARAMETER => write!(buf, "type parameter "),
        _ => Ok(()),
    }
}

/// Only renders at most RESULTS_PER_PAGE entries, and returns whether the iterator
/// had more entries left to render afterwards.
fn render_entries(entries: impl IntoIterator<(Entry, Option<&CodeLabel>)>, output: &mut String) {
    let mut entries_rendered = 0;

    for (entry, code_label) in entries {
        if entries_rendered >= RESULTS_PER_PAGE {
            // We were about to render more than a page; instead, stop here
            // and return that there were more entries to render.
            return true;
        }
        // Add heading based on depth (# for level 1, ## for level 2, etc.)
        write!(output, "{} ", "#".repeat(entry.depth)).ok();

        if let Some(code_label) = code_label {
            output.push_str(code_label.text());
        } else {
            write_symbol_kind(output, entry.kind).ok();
            output.push_str(entry.name.as_str());
        }

        // Convert to 1-based line numbers for display
        let start_line = entry.range.start.0.row as usize + 1;
        let end_line = entry.range.end.0.row as usize + 1;

        if start_line == end_line {
            writeln!(output, " [L{}]", start_line).ok();
        } else {
            writeln!(output, " [L{}-{}]", start_line, end_line).ok();
        }

        entries_rendered += 1;
    }

    false
}
