use std::fmt::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool};
use collections::IndexMap;
use gpui::{App, AsyncApp, Entity, Task};
use language::{CodeLabel, Language, LanguageRegistry};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use lsp::SymbolKind;
use project::{DocumentSymbol, Project, Symbol};
use regex::{Regex, RegexBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use util::markdown::MarkdownString;

use crate::code_symbol_iter::{CodeSymbolIterator, Entry};
use crate::schema::json_schema_for;

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

    /// Whether the regex is case-sensitive. Defaults to false (case-insensitive).
    ///
    /// <example>
    /// Set to `true` to make regex matching case-sensitive.
    /// </example>
    #[serde(default)]
    pub case_sensitive: bool,

    /// Optional starting position for paginated results (0-based).
    /// When not provided, starts from the beginning.
    #[serde(default)]
    pub offset: u32,
}

impl CodeSymbolsInput {
    /// Which page of search results this is.
    pub fn page(&self) -> u32 {
        1 + (self.offset / RESULTS_PER_PAGE)
    }
}

const RESULTS_PER_PAGE: u32 = 2000;

pub struct CodeSymbolsTool;

impl Tool for CodeSymbolsTool {
    fn name(&self) -> String {
        "code_symbols".into()
    }

    fn needs_confirmation(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./code_symbols_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Code
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> serde_json::Value {
        json_schema_for::<CodeSymbolsInput>(format)
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

        let regex = match input.regex {
            Some(regex_str) => match RegexBuilder::new(&regex_str)
                .case_insensitive(!input.case_sensitive)
                .build()
            {
                Ok(regex) => Some(regex),
                Err(err) => return Task::ready(Err(anyhow!("Invalid regex: {err}"))),
            },
            None => None,
        };

        cx.spawn(async move |cx| match input.path {
            Some(path) => file_outline(project, path, action_log, regex, input.offset, cx).await,
            None => project_symbols(project, regex, input.offset, cx).await,
        })
    }
}

pub async fn file_outline(
    project: Entity<Project>,
    path: String,
    action_log: Entity<ActionLog>,
    regex: Option<Regex>,
    offset: u32,
    cx: &mut AsyncApp,
) -> anyhow::Result<String> {
    let buffer = {
        let project_path = project.read_with(cx, |project, cx| {
            project
                .find_project_path(&path, cx)
                .ok_or_else(|| anyhow!("Path {path} not found in project"))
        })??;

        project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))?
            .await?
    };

    action_log.update(cx, |action_log, cx| {
        action_log.buffer_read(buffer.clone(), cx);
    })?;

    let symbols = project
        .update(cx, |project, cx| project.document_symbols(&buffer, cx))?
        .await?;

    if symbols.is_empty() {
        return Err(
            if buffer.read_with(cx, |buffer, _| buffer.snapshot().is_empty())? {
                anyhow!("This file is empty.")
            } else {
                anyhow!("No outline information available for this file.")
            },
        );
    }

    let language = buffer.read_with(cx, |buffer, _| buffer.language().cloned())?;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone())?;

    render_outline(&symbols, language, language_registry, regex, offset).await
}

async fn project_symbols(
    project: Entity<Project>,
    regex: Option<Regex>,
    offset: u32,
    cx: &mut AsyncApp,
) -> anyhow::Result<String> {
    let symbols = project
        .update(cx, |project, cx| project.symbols("", cx))?
        .await?;

    if symbols.is_empty() {
        return Err(anyhow!("No symbols found in project."));
    }

    let mut symbols_by_path: IndexMap<PathBuf, Vec<&Symbol>> = IndexMap::default();

    for symbol in symbols
        .iter()
        .filter(|symbol| {
            if let Some(regex) = &regex {
                regex.is_match(&symbol.name)
            } else {
                true
            }
        })
        .skip(offset as usize)
        // Take 1 more than RESULTS_PER_PAGE so we can tell if there are more results.
        .take((RESULTS_PER_PAGE as usize).saturating_add(1))
    {
        if let Some(worktree_path) = project.read_with(cx, |project, cx| {
            project
                .worktree_for_id(symbol.path.worktree_id, cx)
                .map(|worktree| PathBuf::from(worktree.read(cx).root_name()))
        })? {
            let path = worktree_path.join(&symbol.path.path);
            symbols_by_path.entry(path).or_default().push(symbol);
        }
    }

    // If no symbols matched the filter, return early
    if symbols_by_path.is_empty() {
        return Err(anyhow!("No symbols found matching the criteria."));
    }

    let mut symbols_rendered = 0;
    let mut has_more_symbols = false;
    let mut output = String::new();

    'outer: for (file_path, file_symbols) in symbols_by_path {
        if symbols_rendered > 0 {
            output.push('\n');
        }

        writeln!(&mut output, "{}", file_path.display()).ok();

        for symbol in file_symbols {
            if symbols_rendered >= RESULTS_PER_PAGE {
                has_more_symbols = true;
                break 'outer;
            }

            write!(&mut output, "  {} ", symbol.label.text()).ok();

            // Convert to 1-based line numbers for display
            let start_line = symbol.range.start.0.row as usize + 1;
            let end_line = symbol.range.end.0.row as usize + 1;

            if start_line == end_line {
                writeln!(&mut output, "[L{}]", start_line).ok();
            } else {
                writeln!(&mut output, "[L{}-{}]", start_line, end_line).ok();
            }

            symbols_rendered += 1;
        }
    }

    Ok(if symbols_rendered == 0 {
        "No symbols found in the requested page.".to_string()
    } else if has_more_symbols {
        format!(
            "{output}\nShowing symbols {}-{} (more symbols were found; use offset: {} to see next page)",
            offset + 1,
            offset + symbols_rendered,
            offset + RESULTS_PER_PAGE,
        )
    } else {
        output
    })
}

async fn render_outline(
    symbols: &[DocumentSymbol],
    language: Option<Arc<Language>>,
    registry: Arc<LanguageRegistry>,
    regex: Option<Regex>,
    offset: u32,
) -> Result<String> {
    const RESULTS_PER_PAGE_USIZE: usize = RESULTS_PER_PAGE as usize;
    let entries = CodeSymbolIterator::new(symbols, regex.clone())
        .skip(offset as usize)
        // Take 1 more than RESULTS_PER_PAGE so we can tell if there are more results.
        .take(RESULTS_PER_PAGE_USIZE.saturating_add(1))
        .collect::<Vec<Entry>>();
    let has_more = entries.len() > RESULTS_PER_PAGE_USIZE;

    // Get language-specific labels, if available
    let labels = match &language {
        Some(lang) => {
            let entries_for_labels: Vec<(String, SymbolKind)> = entries
                .iter()
                .take(RESULTS_PER_PAGE_USIZE)
                .map(|entry| (entry.name.clone(), entry.kind))
                .collect();

            let lang_name = lang.name();
            if let Some(lsp_adapter) = registry.lsp_adapters(&lang_name).first().cloned() {
                lsp_adapter
                    .labels_for_symbols(&entries_for_labels, lang)
                    .await
                    .ok()
            } else {
                None
            }
        }
        None => None,
    };

    let mut output = String::new();

    let entries_rendered = match &labels {
        Some(label_list) => render_entries(
            &mut output,
            entries
                .into_iter()
                .take(RESULTS_PER_PAGE_USIZE)
                .zip(label_list.iter())
                .map(|(entry, label)| (entry, label.as_ref())),
        ),
        None => render_entries(
            &mut output,
            entries
                .into_iter()
                .take(RESULTS_PER_PAGE_USIZE)
                .map(|entry| (entry, None)),
        ),
    };

    // Calculate pagination information
    let page_start = offset + 1;
    let page_end = offset + entries_rendered;
    let total_symbols = if has_more {
        format!("more than {}", page_end)
    } else {
        page_end.to_string()
    };

    // Add pagination information
    if has_more {
        writeln!(&mut output, "\nShowing symbols {page_start}-{page_end} (there were more symbols found; use offset: {page_end} to see next page)",
        )
    } else {
        writeln!(
            &mut output,
            "\nShowing symbols {page_start}-{page_end} (total symbols: {total_symbols})",
        )
    }
    .ok();

    Ok(output)
}

fn render_entries<'a>(
    output: &mut String,
    entries: impl IntoIterator<Item = (Entry, Option<&'a CodeLabel>)>,
) -> u32 {
    let mut entries_rendered = 0;

    for (entry, label) in entries {
        // Indent based on depth ("" for level 0, "  " for level 1, etc.)
        for _ in 0..entry.depth {
            output.push_str("  ");
        }

        match label {
            Some(label) => {
                output.push_str(label.text());
            }
            None => {
                write_symbol_kind(output, entry.kind).ok();
                output.push_str(&entry.name);
            }
        }

        // Add position information - convert to 1-based line numbers for display
        let start_line = entry.start_line + 1;
        let end_line = entry.end_line + 1;

        if start_line == end_line {
            writeln!(output, " [L{}]", start_line).ok();
        } else {
            writeln!(output, " [L{}-{}]", start_line, end_line).ok();
        }
        entries_rendered += 1;
    }

    entries_rendered
}

// We may not have a language server adapter to have language-specific
// ways to translate SymbolKnd into a string. In that situation,
// fall back on some reasonable default strings to render.
fn write_symbol_kind(buf: &mut String, kind: SymbolKind) -> Result<(), fmt::Error> {
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
