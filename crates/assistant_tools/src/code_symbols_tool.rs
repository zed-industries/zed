use std::fmt::Write;
use std::path::PathBuf;
use std::sync::Arc;

use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use collections::IndexMap;
use gpui::{App, AsyncApp, Entity, Task};
use language::{OutlineItem, ParseStatus, Point};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::{Project, Symbol};
use regex::{Regex, RegexBuilder};
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

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./code_symbols_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Code
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
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
    ) -> ToolResult {
        let input = match serde_json::from_value::<CodeSymbolsInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let regex = match input.regex {
            Some(regex_str) => match RegexBuilder::new(&regex_str)
                .case_insensitive(!input.case_sensitive)
                .build()
            {
                Ok(regex) => Some(regex),
                Err(err) => return Task::ready(Err(anyhow!("Invalid regex: {err}"))).into(),
            },
            None => None,
        };

        cx.spawn(async move |cx| match input.path {
            Some(path) => file_outline(project, path, action_log, regex, cx).await,
            None => project_symbols(project, regex, input.offset, cx).await,
        })
        .into()
    }
}

pub async fn file_outline(
    project: Entity<Project>,
    path: String,
    action_log: Entity<ActionLog>,
    regex: Option<Regex>,
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

    // Wait until the buffer has been fully parsed, so that we can read its outline.
    let mut parse_status = buffer.read_with(cx, |buffer, _| buffer.parse_status())?;
    while *parse_status.borrow() != ParseStatus::Idle {
        parse_status.changed().await?;
    }

    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
    let Some(outline) = snapshot.outline(None) else {
        return Err(anyhow!("No outline information available for this file."));
    };

    render_outline(
        outline
            .items
            .into_iter()
            .map(|item| item.to_point(&snapshot)),
        regex,
        0,
        usize::MAX,
    )
    .await
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
    items: impl IntoIterator<Item = OutlineItem<Point>>,
    regex: Option<Regex>,
    offset: usize,
    results_per_page: usize,
) -> Result<String> {
    let mut items = items.into_iter().skip(offset);

    let entries = items
        .by_ref()
        .filter(|item| {
            regex
                .as_ref()
                .is_none_or(|regex| regex.is_match(&item.text))
        })
        .take(results_per_page)
        .collect::<Vec<_>>();
    let has_more = items.next().is_some();

    let mut output = String::new();
    let entries_rendered = render_entries(&mut output, entries);

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

fn render_entries(
    output: &mut String,
    items: impl IntoIterator<Item = OutlineItem<Point>>,
) -> usize {
    let mut entries_rendered = 0;

    for item in items {
        // Indent based on depth ("" for level 0, "  " for level 1, etc.)
        for _ in 0..item.depth {
            output.push(' ');
        }
        output.push_str(&item.text);

        // Add position information - convert to 1-based line numbers for display
        let start_line = item.range.start.row + 1;
        let end_line = item.range.end.row + 1;

        if start_line == end_line {
            writeln!(output, " [L{}]", start_line).ok();
        } else {
            writeln!(output, " [L{}-{}]", start_line, end_line).ok();
        }
        entries_rendered += 1;
    }

    entries_rendered
}
