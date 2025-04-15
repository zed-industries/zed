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
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp, convert::TryFrom, fmt::Write, path::PathBuf, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownString;
use util::paths::PathMatcher;
use worktree::Snapshot;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchToolInput {
    /// Only paths matching this glob pattern will be considered for the output.
    ///
    /// If this parameter is omitted, all files will be considered.
    ///
    /// <example>
    /// To find all Markdown files, use "**/*.md"
    /// To find files in a specific directory, use "src/zed.dev/**"
    /// </example>
    pub path_glob: Option<String>,

    /// When specified, this filters the output based on the contents of the files or code symbols.
    ///
    /// - If the "output" parameter is "symbols", then this search query be sent to the language server to filter which the code symbols (such as identifiers, types, etc.) will be included in the output.
    /// - If the "output" parameter is "text", then this query will be interpreted as a regex, adn only text snippets matching that regex will be included.
    /// - If the "output" parameter is "paths", then this query will be interpreted as a regex, adn only files whose text contents match that regex will be included.
    #[serde(default)]
    pub query: Option<String>,

    /// Whether the regex is case-sensitive. Defaults to false (case-insensitive).
    #[serde(default)]
    pub contents_regex_case_sensitive: bool,

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
    /// If no path_glob is specified, outputs all the paths in the project.
    Paths,
    /// Output matching arbitrary text regions within files, including their line numbers.
    /// If no path_glob is specified, outputs text found in any file in the project.
    Text,
    /// Output matching code symbols (such as identifiers, types, etc.) within files, including their line numbers.
    /// If no path_glob is specified, outputs symbols found across the entire project.
    Symbols,
}

// Different search modes have different pagination limits
const PATHS_RESULTS_PER_PAGE: usize = 50;
const TEXT_RESULTS_PER_PAGE: usize = 20;
const SYMBOLS_LINES_PER_PAGE: u32 = 1000;

pub struct SearchTool;

impl Tool for SearchTool {
    fn name(&self) -> String {
        "search_project".into()
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
                // Don't show any pattern if not specified
                let path_pattern = input.path_glob.as_deref().map(MarkdownString::inline_code);
                let case_info = if input.contents_regex_case_sensitive {
                    " (case-sensitive)"
                } else {
                    ""
                };

                match input.output {
                    Output::Paths => match path_pattern {
                        Some(pattern) => format!("Find paths matching {}{}", pattern, case_info),
                        None => format!("Find all paths{}", case_info),
                    },
                    Output::Text => {
                        if let Some(search_regex) = &input.query {
                            let search_pattern = MarkdownString::inline_code(search_regex);
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Search for {} in files matching {}{}",
                                    search_pattern, pattern, case_info
                                ),
                                None => format!("Search for {}{}", search_pattern, case_info),
                            }
                        } else {
                            match path_pattern {
                                Some(pattern) => {
                                    format!("Search in files matching {}{}", pattern, case_info)
                                }
                                None => format!("Search in all files{}", case_info),
                            }
                        }
                    }
                    Output::Symbols => {
                        if let Some(search_regex) = &input.query {
                            let search_pattern = MarkdownString::inline_code(search_regex);
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find symbols matching {} in files matching {}{}",
                                    search_pattern, pattern, case_info
                                ),
                                None => {
                                    format!("Find symbols matching {}{}", search_pattern, case_info)
                                }
                            }
                        } else {
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find symbols in files matching {}{}",
                                    pattern, case_info
                                ),
                                None => format!("Find all symbols{}", case_info),
                            }
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
            Output::Text => search_text(input, project, action_log.clone(), cx),
            Output::Symbols => search_symbols(input, project, action_log, cx),
        }
    }
}

fn search_paths(
    input: SearchToolInput,
    project: Entity<Project>,
    cx: &mut App,
) -> Task<Result<String>> {
    // Clone the path_glob to avoid borrowing issues with the async closure
    let path_glob_option = input.path_glob.clone();
    let query_option = input.query.clone();
    let case_sensitive = input.contents_regex_case_sensitive;

    // Create the path matcher based on the provided glob pattern or use a matcher that matches everything
    let path_matcher = if let Some(glob) = path_glob_option.as_deref() {
        match PathMatcher::new([glob]) {
            Ok(matcher) => matcher,
            Err(err) => return Task::ready(Err(anyhow!("Invalid glob pattern: {}", err))),
        }
    } else {
        // When no glob pattern is provided, match all files
        match PathMatcher::new(["*"]) {
            Ok(matcher) => matcher,
            Err(err) => {
                return Task::ready(Err(anyhow!(
                    "Failed to create default path matcher: {}",
                    err
                )));
            }
        }
    };

    // If a query regex is provided, create a search query for filtering files by content
    let regex_query = if let Some(regex) = &query_option {
        match SearchQuery::regex(
            regex,
            false,
            case_sensitive,
            false,
            false,
            path_matcher.clone(),
            PathMatcher::default(),
            None,
        ) {
            Ok(query) => Some(query),
            Err(error) => return Task::ready(Err(error)),
        }
    } else {
        None
    };

    let snapshots: Vec<Snapshot> = project
        .read(cx)
        .worktrees(cx)
        .map(|worktree| worktree.read(cx).snapshot())
        .collect();

    // Create a copy of path_glob for use in the async closure
    let path_glob_for_error = path_glob_option.clone();

    // If we need to filter by content, use the search functionality
    if let Some(query) = regex_query {
        let results = project.update(cx, |project, cx| project.search(query, cx));

        return cx.spawn(async move |cx| {
            futures::pin_mut!(results);

            let mut filtered_paths = Vec::new();

            while let Some(SearchResult::Buffer { buffer, ranges }) = results.next().await {
                if !ranges.is_empty() {
                    if let Some(path) = buffer.read_with(cx, |buffer, cx| {
                        buffer.file().map(|file| file.full_path(cx).to_string_lossy().to_string())
                    })? {
                        filtered_paths.push(path);
                    }
                }
            }

            if filtered_paths.is_empty() {
                return Ok(format!("No paths in the project matched the glob {:?} and content regex {:?}",
                                 path_glob_for_error, query_option));
            }

            // Sort to group entries in the same directory together
            filtered_paths.sort();

            let total_matches = filtered_paths.len();
            let response = if total_matches > PATHS_RESULTS_PER_PAGE + input.offset as usize {
                let paginated_matches: Vec<_> = filtered_paths
                    .into_iter()
                    .skip(input.offset as usize)
                    .take(PATHS_RESULTS_PER_PAGE)
                    .collect();

                format!(
                    "Found {} paths matching the content regex. Showing results {}-{} (provide 'offset' parameter for more results):\n\n{}",
                    total_matches,
                    input.offset + 1,
                    input.offset as usize + paginated_matches.len(),
                    paginated_matches.join("\n")
                )
            } else {
                let displayed_matches: Vec<_> = filtered_paths
                    .into_iter()
                    .skip(input.offset as usize)
                    .collect();

                format!(
                    "Found {} paths matching the content regex:\n\n{}",
                    total_matches,
                    displayed_matches.join("\n")
                )
            };

            Ok(response)
        });
    }

    // If no content regex, just filter by path glob as before
    cx.background_executor().spawn(async move {
        let mut matches = Vec::new();

        for worktree in snapshots {
            let root_name = worktree.root_name();

            // Don't consider ignored entries
            for entry in worktree.entries(false, 0) {
                if path_matcher.is_match(&entry.path) {
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
            Ok(format!("No paths in the project matched the glob {:?}", path_glob_for_error))
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
    action_log: Entity<ActionLog>,
    cx: &mut App,
) -> Task<Result<String>> {
    const CONTEXT_LINES: u32 = 2;
    /// If the model requests to read a file whose size exceeds this, then
    /// the tool will return an outline instead of the full file contents
    const MAX_FILE_SIZE_TO_READ: usize = 16384;

    // If no query is provided and path_glob points to a specific file, read the file contents
    if input.query.is_none()
        && input.path_glob.as_ref().map_or(false, |glob| {
            !glob.contains('*') && !glob.contains('?') && !glob.contains('[')
        })
    {
        let file_path = input.path_glob.unwrap();

        return cx.spawn(async move |cx| {
            let Some(project_path) = project.read_with(cx, |project, cx| {
                project.find_project_path(&file_path, cx)
            })? else {
                return Err(anyhow!("Path {} not found in project", &file_path));
            };

            let buffer = project.update(cx, |project, cx| {
                project.open_buffer(project_path, cx)
            })?.await?;

            // Check file size to see if it's too big
            let file_size = buffer.read_with(cx, |buffer, _cx| buffer.text().len())?;

            if file_size <= MAX_FILE_SIZE_TO_READ {
                // File is small enough, so return its contents
                let result = buffer.read_with(cx, |buffer, _cx| buffer.text())?;
                Ok(format!("Contents of {}:\n\n```\n{}\n```", file_path, result))
            } else {
                // File is too big, so get its outline instead
                let outline = render_file_outline(project, file_path.clone(), action_log.clone(), None, 0, cx).await?;

                Ok(format!("The file '{}' was too big to read all at once. Here is an outline of its symbols:\n\n{}\n\nTry searching for specific content by providing a regex query.", file_path, outline))
            }
        });
    }

    let search_regex = match &input.query {
        Some(regex) => regex.clone(),
        None => {
            return Task::ready(Err(anyhow!(
                "Either provide a specific file path in path_glob or a regex query to search for text"
            )));
        }
    };

    // Create a query based on the path glob or use a matcher that matches everything
    let path_matcher = if let Some(glob) = input.path_glob.as_deref() {
        match PathMatcher::new([glob]) {
            Ok(matcher) => matcher,
            Err(err) => return Task::ready(Err(anyhow!("Invalid glob pattern: {}", err))),
        }
    } else {
        // When no glob pattern is provided, match all files
        match PathMatcher::new(["*"]) {
            Ok(matcher) => matcher,
            Err(err) => {
                return Task::ready(Err(anyhow!(
                    "Failed to create default path matcher: {}",
                    err
                )));
            }
        }
    };

    let query = match SearchQuery::regex(
        dbg!(&search_regex),
        false,
        input.contents_regex_case_sensitive,
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
    // Check if path_glob is a specific file path
    if let Some(path) = &input.path_glob {
        // If the glob pattern doesn't contain wildcards, assume it's a specific file path
        if !path.contains('*') && !path.contains('?') && !path.contains('[') {
            let path_string = path.clone();
            return cx.spawn(async move |cx| {
                render_file_outline(
                    project,
                    path_string,
                    action_log,
                    input.query,
                    input.offset,
                    cx,
                )
                .await
            });
        }
    }

    // Otherwise, get project-wide symbols filtered by path_glob
    let path_matcher = if let Some(glob) = input.path_glob.as_deref() {
        match PathMatcher::new([glob]) {
            Ok(matcher) => Some(matcher),
            Err(err) => return Task::ready(Err(anyhow!("Invalid glob pattern: {}", err))),
        }
    } else {
        None
    };

    cx.spawn(async move |cx| {
        project_symbols(project, path_matcher, &input.query.unwrap_or_default(), cx).await
    })
}

async fn render_file_outline(
    project: Entity<Project>,
    path: String,
    action_log: Entity<ActionLog>,
    query: Option<String>,
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

    // Get buffer text for showing content at each symbol
    let buffer_text = buffer.read_with(cx, |buffer, _| buffer.text())?;

    render_outline(
        &mut output,
        outline
            .items
            .into_iter()
            .map(|item| item.to_point(&snapshot)),
        query,
        offset,
        u32::MAX, // No symbol limit, just use line limit
        Some((&buffer_text, path.clone())),
        SYMBOLS_LINES_PER_PAGE,
    )
}

async fn project_symbols(
    project: Entity<Project>,
    path_matcher: Option<PathMatcher>,
    query: &str,
    cx: &mut AsyncApp,
) -> anyhow::Result<String> {
    let symbols = project
        .update(cx, |project, cx| project.symbols(query, cx))?
        .await?;

    if symbols.is_empty() {
        return Err(anyhow!("No symbols found in project."));
    }

    let mut symbols_by_path: IndexMap<PathBuf, Vec<&Symbol>> = IndexMap::default();

    for symbol in symbols.iter().filter(|symbol| {
        path_matcher
            .as_ref()
            .map(|matcher| matcher.is_match(&symbol.path.path))
            .unwrap_or(true)
    }) {
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
    let mut output = String::new();
    let mut lines_shown = 0;
    let mut skipped_symbols = 0;

    for (file_path, file_symbols) in symbols_by_path {
        if symbols_rendered > 0 {
            output.push('\n');
        }

        writeln!(&mut output, "## {}", file_path.display()).ok();

        // We'll need to read the file's content to display snippets
        let file_path_str = file_path.to_string_lossy().to_string();
        let project_path = project.read_with(cx, |project, cx| {
            project.find_project_path(&file_path_str, cx)
        })?;

        if let Some(project_path) = project_path {
            // Get file content if possible
            let buffer_task =
                project.update(cx, |project, cx| project.open_buffer(project_path, cx))?;
            if let Ok(buffer) = buffer_task.await {
                if let Ok(file_text) = buffer.read_with(cx, |buffer, _| buffer.text()) {
                    // Process symbols for this file
                    for symbol in file_symbols {
                        // Convert to 0-based line numbers for slicing the content
                        let start_line = symbol.range.start.0.row;
                        let end_line = symbol.range.end.0.row;

                        // Convert to 1-based line numbers for display
                        let display_start = start_line + 1;
                        let display_end = end_line + 1;

                        // Write the symbol header
                        write!(&mut output, "  {} ", symbol.label.text()).ok();

                        if display_start == display_end {
                            writeln!(&mut output, "[L{}]", display_start).ok();
                        } else {
                            writeln!(&mut output, "[L{}-{}]", display_start, display_end).ok();
                        }

                        // Increment count even if we don't show content due to lines limit
                        symbols_rendered += 1;

                        // Check if we still have line budget to show content
                        if lines_shown < SYMBOLS_LINES_PER_PAGE as usize {
                            // Add the code content with indentation
                            let lines: Vec<&str> = file_text.split('\n').collect();
                            let start_idx = start_line as usize;
                            let end_idx = (end_line as usize) + 1;

                            if start_idx < lines.len() {
                                let actual_end = end_idx.min(lines.len());
                                let line_count = actual_end - start_idx;

                                // Only show content if we have enough line budget
                                if lines_shown + line_count <= SYMBOLS_LINES_PER_PAGE as usize {
                                    writeln!(&mut output, "```").ok();
                                    for line in &lines[start_idx..actual_end] {
                                        writeln!(&mut output, "    {}", line).ok();
                                    }
                                    writeln!(&mut output, "```\n").ok();

                                    lines_shown += line_count;
                                } else {
                                    // We've hit our limit, note that we're skipping content
                                    skipped_symbols += 1;
                                }
                            }
                        } else {
                            // We've hit our line limit
                            skipped_symbols += 1;
                        }
                    }
                }
            }
        } else {
            // Fall back to just showing the symbols without content if we can't open the file
            for symbol in file_symbols {
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
    }

    // Add information about lines shown and skipped symbols
    if skipped_symbols > 0 {
        writeln!(
            &mut output,
            "\nShowing {} symbols with {} lines of content. {} symbols' content was not shown due to the {} line limit.",
            symbols_rendered,
            lines_shown,
            skipped_symbols,
            SYMBOLS_LINES_PER_PAGE
        ).ok();
    } else {
        writeln!(
            &mut output,
            "\nShowing {} symbols with {} lines of content.",
            symbols_rendered, lines_shown
        )
        .ok();
    }

    Ok(output)
}

fn render_outline(
    output: &mut String,
    items: impl IntoIterator<Item = OutlineItem<Point>>,
    query: Option<String>,
    offset: u32,
    results_per_page: u32,
    file_content: Option<(&str, String)>,
    max_lines_per_page: u32,
) -> anyhow::Result<String> {
    let mut items = items.into_iter().skip(offset as usize);

    let entries = items
        .by_ref()
        .filter(|item| {
            query
                .as_ref()
                .map_or(true, |query| item.text.contains(query))
        })
        .take(results_per_page as usize)
        .collect::<Vec<_>>();

    let has_more = items.next().is_some();

    // Track content lines shown
    let mut content_lines = 0;

    let entries_rendered = render_symbol_entries(
        output,
        entries,
        file_content,
        max_lines_per_page,
        &mut content_lines,
    );

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
        writeln!(output, "\nShowing symbols {page_start}-{page_end} with {content_lines} lines of content (there were more symbols found; use offset: {page_end} to see next page)",
        )
    } else {
        writeln!(
            output,
            "\nShowing symbols {page_start}-{page_end} with {content_lines} lines of content (total symbols: {total_symbols})",
        )
    }
    .ok();

    Ok(output.clone())
}

fn render_symbol_entries(
    output: &mut String,
    items: impl IntoIterator<Item = OutlineItem<Point>>,
    file_content: Option<(&str, String)>,
    max_lines_per_page: u32,
    offset: &mut u32,
) -> u32 {
    let mut entries_rendered = 0;
    let mut lines_shown: u32 = 0;

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

        // Add file content if available
        if let Some((content, _)) = &file_content {
            // Convert to 0-based line numbers for content
            let content_start = item.range.start.row as usize;
            let content_end = item.range.end.row as usize + 1; // +1 to include the end line

            // Check if we still have line budget
            if lines_shown < max_lines_per_page {
                // Split content into lines and get the relevant section
                let lines: Vec<&str> = content.split('\n').collect();

                if content_start < lines.len() {
                    let actual_end = content_end.min(lines.len());
                    let line_count = (actual_end - content_start) as u32;

                    // Make sure we don't exceed the maximum lines per page
                    if lines_shown + line_count <= max_lines_per_page {
                        writeln!(output, "```").ok();
                        for line in &lines[content_start..actual_end] {
                            writeln!(output, "    {}", line).ok();
                        }
                        writeln!(output, "```\n").ok();

                        lines_shown += line_count;
                    }
                }
            }
        }
    }

    // Update the offset with lines shown
    *offset = lines_shown;

    entries_rendered
}
