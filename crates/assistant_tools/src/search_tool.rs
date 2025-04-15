use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool};
use collections::IndexMap;
use futures::StreamExt;
use gpui::{App, AsyncApp, Entity, Task};
use language::{OffsetRangeExt, OutlineItem, ParseStatus, Point};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::{
    Project, Symbol,
    search::{SearchQuery, SearchResult},
};
use regex::{Regex, RegexBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp, convert::TryFrom, fmt::Write, path::PathBuf, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownString;
use util::paths::PathMatcher;
use worktree::Snapshot;

// No helper traits needed

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchToolInput {
    /// Only paths matching this regex will be considered for the output.
    ///
    /// If this paramter is omitted, all files will be considered.
    ///
    /// <example>
    /// To find all Markdown files, use ".*\\.md$"
    /// To find files in a specific directory, use "src/zed\\.dev/.*"
    /// </example>
    pub path_regex: Option<String>,

    /// Whether path_regex should do case-senstive matches. Defaults to false.
    ///
    /// <example>
    /// Set to `true` to make path pattern matching case-sensitive.
    /// For instance, "SRC" would not match "src" when this is true.
    /// </example>
    #[serde(default)]
    pub path_regex_case_sensitive: bool,

    /// Only files containing this regex will be included in the output.
    ///
    /// - If the "output" parameter is "symbols", then only code symbols (such as identifiers, types, etc.) matching this regex will be included.
    /// - If the "output" parameter is "text", then only text snippets matching this regex will be included.
    /// - If the "output" parameter is "paths", then only files whose contents match this regex will be included.
    ///
    /// If this parameter is omitted, then no filtering based on file contents will occur.
    #[serde(default)]
    pub file_contents_regex: Option<String>,

    /// Whether the regex is case-sensitive. Defaults to false (case-insensitive).
    #[serde(default)]
    pub file_contents_regex_case_sensitive: bool,

    /// The desired format for the output.
    pub output: Output,

    /// Optional starting position for paginated results (0-based).
    /// When not provided, starts from the beginning.
    #[serde(default)]
    pub offset: u32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Output {
    /// Output matching file paths only.
    Paths,
    /// Output matching arbitrary text regions within files, including their line numbers.
    Text,
    /// Output matching code symbols (such as identifiers, types, etc.) within files, including their line numbers.
    Symbols,
}

// Different search modes have different pagination limits
const PATHS_RESULTS_PER_PAGE: usize = 50;
const TEXT_RESULTS_PER_PAGE: usize = 20;
const SYMBOLS_RESULTS_PER_PAGE: u32 = 100;

pub struct SearchTool;

impl Tool for SearchTool {
    fn name(&self) -> String {
        "search".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./search_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::SearchCode
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> serde_json::Value {
        json_schema_for::<SearchToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<SearchToolInput>(input.clone()) {
            Ok(input) => {
                let path_pattern =
                    MarkdownString::inline_code(input.path_regex.as_deref().unwrap_or(".*"));
                let case_info = if input.path_regex_case_sensitive
                    || input.file_contents_regex_case_sensitive
                {
                    " (case-sensitive)"
                } else {
                    ""
                };

                match input.output {
                    Output::Paths => {
                        format!("Find paths matching {}{}", path_pattern, case_info)
                    }
                    Output::Text => {
                        if let Some(search_regex) = &input.file_contents_regex {
                            let search_pattern = MarkdownString::inline_code(search_regex);
                            format!(
                                "Search for {} in files matching {}{}",
                                search_pattern, path_pattern, case_info
                            )
                        } else {
                            format!("Search in files matching {}{}", path_pattern, case_info)
                        }
                    }
                    Output::Symbols => {
                        if let Some(search_regex) = &input.file_contents_regex {
                            let search_pattern = MarkdownString::inline_code(search_regex);
                            format!(
                                "Find symbols matching {} in files matching {}{}",
                                search_pattern, path_pattern, case_info
                            )
                        } else {
                            format!(
                                "Find symbols in files matching {}{}",
                                path_pattern, case_info
                            )
                        }
                    }
                }
            }
            Err(_) => "Unified search".to_string(),
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
        let input = match serde_json::from_value::<SearchToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        match input.output {
            Output::Paths => search_paths(input, project, cx),
            Output::Text => search_text(input, project, cx),
            Output::Symbols => search_symbols(input, project, action_log, cx),
        }
    }
}

fn search_paths(
    input: SearchToolInput,
    project: Entity<Project>,
    cx: &mut App,
) -> Task<Result<String>> {
    let path_regex = match RegexBuilder::new(input.path_regex.as_deref().unwrap_or(".*"))
        .case_insensitive(!input.path_regex_case_sensitive)
        .build()
    {
        Ok(regex) => regex,
        Err(err) => return Task::ready(Err(anyhow!("Invalid path regex: {}", err))),
    };

    let snapshots: Vec<Snapshot> = project
        .read(cx)
        .worktrees(cx)
        .map(|worktree| worktree.read(cx).snapshot())
        .collect();

    cx.background_executor().spawn(async move {
        let mut matches = Vec::new();

        for worktree in snapshots {
            let root_name = worktree.root_name();

            // Don't consider ignored entries
            for entry in worktree.entries(false, 0) {
                let path_string = entry.path.to_string_lossy().to_string();

                if path_regex.is_match(&path_string) {
                    matches.push(
                        PathBuf::from(root_name)
                            .join(&entry.path)
                            .to_string_lossy()
                            .to_string(),
                    );
                }
            }
        }

        if matches.is_empty() {
            Ok(format!("No paths in the project matched the regex {:?}", input.path_regex))
        } else {
            // Sort to group entries in the same directory together
            matches.sort();

            let total_matches = matches.len();
            let response = if total_matches > PATHS_RESULTS_PER_PAGE + input.offset as usize {
                let paginated_matches: Vec<_> = matches
                    .into_iter()
                    .skip(input.offset as usize)
                    .take(PATHS_RESULTS_PER_PAGE)
                    .collect();

                format!(
                    "Found {} total matches. Showing results {}-{} (provide 'offset' parameter for more results):\n\n{}",
                    total_matches,
                    input.offset + 1,
                    input.offset as usize + paginated_matches.len(),
                    paginated_matches.join("\n")
                )
            } else {
                let displayed_matches: Vec<_> = matches
                    .into_iter()
                    .skip(input.offset as usize)
                    .collect();

                format!(
                    "Found {} total matches:\n\n{}",
                    total_matches,
                    displayed_matches.join("\n")
                )
            };

            Ok(response)
        }
    })
}

fn search_text(
    input: SearchToolInput,
    project: Entity<Project>,
    cx: &mut App,
) -> Task<Result<String>> {
    const CONTEXT_LINES: u32 = 2;

    let search_regex = match &input.file_contents_regex {
        Some(regex) => regex.clone(),
        None => {
            return Task::ready(Err(anyhow!(
                "file_contents_regex is required for text search mode"
            )));
        }
    };

    // We don't have a specific_file field anymore, so always use path_regex
    // Create a query based on the path regex
    let path_matcher = match PathMatcher::new([input.path_regex.as_deref().unwrap_or(".*")]) {
        Ok(matcher) => matcher,
        Err(err) => return Task::ready(Err(anyhow!("Invalid file path: {}", err))),
    };

    let query = match SearchQuery::regex(
        &search_regex,
        false,
        input.file_contents_regex_case_sensitive,
        false,
        false,
        path_matcher,
        PathMatcher::default(),
        None,
    ) {
        Ok(query) => query,
        Err(error) => return Task::ready(Err(error)),
    };

    let results = project.update(cx, |project, cx| project.search(query, cx));

    cx.spawn(async move|cx|  {
        futures::pin_mut!(results);

        let mut output = String::new();
        let mut skips_remaining = input.offset;
        let mut matches_found = 0;
        let mut has_more_matches = false;

        while let Some(SearchResult::Buffer { buffer, ranges }) = results.next().await {
            if ranges.is_empty() {
                continue;
            }

            buffer.read_with(cx, |buffer, cx| -> Result<(), anyhow::Error> {
                if let Some(path) = buffer.file().map(|file| file.full_path(cx)) {
                    let mut file_header_written = false;
                    let mut ranges = ranges
                        .into_iter()
                        .map(|range| {
                            let mut point_range = range.to_point(buffer);
                            point_range.start.row =
                                point_range.start.row.saturating_sub(CONTEXT_LINES);
                            point_range.start.column = 0;
                            point_range.end.row = cmp::min(
                                buffer.max_point().row,
                                point_range.end.row + CONTEXT_LINES,
                            );
                            point_range.end.column = buffer.line_len(point_range.end.row);
                            point_range
                        })
                        .peekable();

                    while let Some(mut range) = ranges.next() {
                        if skips_remaining > 0 {
                            skips_remaining -= 1;
                            continue;
                        }

                        // We'd already found a full page of matches, and we just found one more.
                        if matches_found >= TEXT_RESULTS_PER_PAGE {
                            has_more_matches = true;
                            return Ok(());
                        }

                        while let Some(next_range) = ranges.peek() {
                            if range.end.row >= next_range.start.row {
                                range.end = next_range.end;
                                ranges.next();
                            } else {
                                break;
                            }
                        }

                        if !file_header_written {
                            writeln!(output, "\n## Matches in {}", path.display())?;
                            file_header_written = true;
                        }

                        let start_line = range.start.row + 1;
                        let end_line = range.end.row + 1;
                        writeln!(output, "\n### Lines {start_line}-{end_line}\n```")?;
                        output.extend(buffer.text_for_range(range));
                        output.push_str("\n```\n");

                        matches_found += 1;
                    }
                }

                Ok(())
            })??;
        }

        if matches_found == 0 {
            Ok("No matches found".to_string())
        } else if has_more_matches {
            Ok(format!(
                "Showing matches {}-{} (there were more matches found; use offset: {} to see next page):\n{output}",
                input.offset + 1,
                input.offset + u32::try_from(matches_found).unwrap_or(0),
                input.offset + u32::try_from(TEXT_RESULTS_PER_PAGE).unwrap_or(0),
            ))
        } else {
            Ok(format!("Found {matches_found} matches:\n{output}"))
        }
    })
}

fn search_symbols(
    input: SearchToolInput,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    cx: &mut App,
) -> Task<Result<String>> {
    // Create regex for filtering symbols if file_contents_regex is provided
    let regex = match &input.file_contents_regex {
        Some(regex_str) => {
            match RegexBuilder::new(regex_str)
                .case_insensitive(!input.file_contents_regex_case_sensitive)
                .build()
            {
                Ok(regex) => Some(regex),
                Err(err) => return Task::ready(Err(anyhow!("Invalid regex: {}", err))),
            }
        }
        None => None,
    };

    // We don't use specific_file anymore - use path_regex directly if it's a single file
    if let Some(path) = &input.path_regex {
        let path_string = path.clone();
        return cx.spawn(async move |cx| {
            file_outline(project, path_string, action_log, regex, input.offset, cx).await
        });
    }

    // Otherwise, get project-wide symbols filtered by path_regex
    let path_regex = match RegexBuilder::new(input.path_regex.as_deref().unwrap_or(".*"))
        .case_insensitive(!input.path_regex_case_sensitive)
        .build()
    {
        Ok(regex) => regex,
        Err(err) => return Task::ready(Err(anyhow!("Invalid path regex: {}", err))),
    };

    cx.spawn(async move |cx| project_symbols(project, path_regex, regex, input.offset, cx).await)
}

async fn file_outline(
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
                .ok_or_else(|| anyhow!("Path {} not found in project", path))
        })??;

        project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))?
            .await?
    };

    action_log.update(cx, |action_log, cx| {
        action_log.buffer_read(buffer.clone(), cx);
    })?;

    // Wait until the buffer has been fully parsed, so that we can read its outline
    let mut parse_status = buffer.read_with(cx, |buffer, _| buffer.parse_status())?;
    while *parse_status.borrow() != ParseStatus::Idle {
        parse_status.changed().await?;
    }

    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
    let Some(outline) = snapshot.outline(None) else {
        return Err(anyhow!(
            "No outline information available for file: {}",
            path
        ));
    };

    let mut output = String::new();
    writeln!(&mut output, "# Symbols in {}\n", path).ok();

    render_outline(
        &mut output,
        outline
            .items
            .into_iter()
            .map(|item| item.to_point(&snapshot)),
        regex,
        offset,
        SYMBOLS_RESULTS_PER_PAGE,
    )
}

async fn project_symbols(
    project: Entity<Project>,
    path_regex: Regex,
    name_regex: Option<Regex>,
    offset: u32,
    cx: &mut AsyncApp,
) -> anyhow::Result<String> {
    const SYMBOLS_RESULTS_PER_PAGE_USIZE: usize = 100;
    let symbols = project
        .update(cx, |project, cx| project.symbols("", cx))?
        .await?;

    if symbols.is_empty() {
        return Err(anyhow!("No symbols found in project."));
    }

    let mut symbols_by_path: IndexMap<PathBuf, Vec<&Symbol>> = IndexMap::default();

    for symbol in symbols
        .iter()
        // Apply both path and name filters
        .filter(|symbol| {
            // Convert path to string for regex matching
            let path_string = symbol.path.path.to_string_lossy().to_string();
            let path_matches = path_regex.is_match(&path_string);

            let name_matches = if let Some(regex) = &name_regex {
                regex.is_match(&symbol.name)
            } else {
                true
            };

            path_matches && name_matches
        })
        .skip(offset as usize)
        // Take 1 more than RESULTS_PER_PAGE so we can tell if there are more results
        .take(SYMBOLS_RESULTS_PER_PAGE_USIZE + 1)
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
        return Ok("No symbols found matching the criteria.".to_string());
    }

    let mut symbols_rendered: usize = 0;
    let mut has_more_symbols = false;
    let mut output = String::new();

    'outer: for (file_path, file_symbols) in symbols_by_path {
        if symbols_rendered > 0 {
            output.push('\n');
        }

        writeln!(&mut output, "## {}", file_path.display()).ok();

        for symbol in file_symbols {
            if symbols_rendered >= SYMBOLS_RESULTS_PER_PAGE_USIZE {
                has_more_symbols = true;
                break 'outer;
            }

            write!(&mut output, "  {} ", symbol.label.text()).ok();

            // Convert to 1-based line numbers for display
            let start_line = symbol.range.start.0.row + 1;
            let end_line = symbol.range.end.0.row + 1;

            if start_line == end_line {
                writeln!(&mut output, "[L{}]", start_line).ok();
            } else {
                writeln!(&mut output, "[L{}-{}]", start_line, end_line).ok();
            }

            symbols_rendered += 1;
        }
    }

    Ok(if has_more_symbols {
        format!(
            "{output}\n\nShowing symbols {}-{} (more symbols were found; use offset: {} to see next page)",
            offset + 1,
            offset + u32::try_from(symbols_rendered).unwrap_or(0),
            offset + 100,
        )
    } else {
        format!(
            "{output}\n\nShowing symbols {}-{} (total symbols: {})",
            offset + 1,
            offset + u32::try_from(symbols_rendered).unwrap_or(0),
            offset + u32::try_from(symbols_rendered).unwrap_or(0),
        )
    })
}

fn render_outline(
    output: &mut String,
    items: impl IntoIterator<Item = OutlineItem<Point>>,
    regex: Option<Regex>,
    offset: u32,
    results_per_page: u32,
) -> anyhow::Result<String> {
    let mut items = items.into_iter().skip(offset as usize);

    let entries = items
        .by_ref()
        .filter(|item| {
            regex
                .as_ref()
                .map_or(true, |regex| regex.is_match(&item.text))
        })
        .take(results_per_page as usize)
        .collect::<Vec<_>>();

    let has_more = items.next().is_some();

    let entries_rendered = render_symbol_entries(output, entries);

    // Calculate pagination information
    let page_start = offset + 1;
    let page_end = offset + u32::try_from(entries_rendered).unwrap_or(0);
    let total_symbols = if has_more {
        format!("more than {}", page_end)
    } else {
        page_end.to_string()
    };

    // Add pagination information
    if has_more {
        writeln!(output, "\nShowing symbols {page_start}-{page_end} (there were more symbols found; use offset: {page_end} to see next page)",
        )
    } else {
        writeln!(
            output,
            "\nShowing symbols {page_start}-{page_end} (total symbols: {total_symbols})",
        )
    }
    .ok();

    Ok(output.clone())
}

fn render_symbol_entries(
    output: &mut String,
    items: impl IntoIterator<Item = OutlineItem<Point>>,
) -> u32 {
    let mut entries_rendered = 0;

    for item in items {
        // Indent based on depth ("#" for level 0, "##" for level 1, etc.)
        for _ in 0..=item.depth {
            output.push('#');
        }
        output.push(' ');
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
