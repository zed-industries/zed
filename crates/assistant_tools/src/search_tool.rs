use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
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
    pub path: Option<String>,

    /// When specified, this filters the output based on the contents of the files or code symbols.
    ///
    /// - If the "output" parameter is "symbols", then this search query will be sent to a language server to filter which the code symbols (such as identifiers, types, etc.) will be included in the output.
    /// - If the "output" parameter is "text", then this query will be interpreted as a regex, and only text snippets matching that regex will be included.
    /// - If the "output" parameter is "paths", then this query will be interpreted as a regex, and only files whose text contents match that regex will be included.
    #[serde(default)]
    pub query: Option<String>,

    /// Whether the query should match case-sensitively. Defaults to false (case-insensitive).
    #[serde(default)]
    pub query_case_sensitive: bool,

    /// The desired format for the output.
    pub output: Output,

    /// Optional position (1-based index) to start reading on, if you want to read a subset of the contents.
    /// When reading a file, this refers to a line number in the file (e.g. 1 is the first line).
    /// When reading a directory, this refers to the number of the directory entry (e.g. 1 is the first entry).
    /// For paginated results, this represents the starting item (1-based).
    ///
    /// Defaults to 1.
    #[serde(default)]
    pub start: Option<u32>,

    /// Optional position (1-based index) to end reading on, if you want to read a subset of the contents.
    /// When reading a file, this refers to a line number in the file (e.g. 1 is the first line).
    /// When reading a directory, this refers to the number of the directory entry (e.g. 1 is the first entry).
    /// For paginated results, this represents how many items to include (starting from the start position).
    ///
    /// Defaults to reading until the end of the file or directory, or a reasonable limit for paginated results.
    #[serde(default)]
    pub end: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Output {
    /// Output matching file paths only.
    /// If no path is specified, this outputs all the paths in the project matching
    /// the query (or all paths in the project, if there is no query specified), limited
    /// based on start and/or end (if they are specified).
    ///
    /// <example>
    /// To list the paths of all Rust files in the project:
    /// {
    ///     "path": "**/*.rs",
    ///     "output": "paths"
    /// }
    /// </example>
    ///
    /// <example>
    /// To list the paths of all files in the project which contain the string "TODO":
    /// {
    ///     "query": "TODO",
    ///     "output": "paths"
    /// }
    /// </example>
    Paths,

    /// Output matching arbitrary text regions within files, including their line numbers.
    /// If no path is specified, this outputs text found in every file in the project
    /// matching the query (a query should always be specified when using "output": "text"
    /// and no path). If no query is specified, but a path is specified, reads the entire
    /// contents of that path.
    ///
    /// Output is always limited based on start and/or end (if they are specified).
    ///
    /// <example>
    /// To find all occurrences of "TODO" in all files (including paths and line numbers):
    /// {
    ///     "query": "TODO",
    ///     "output": "text"
    /// }
    ///
    /// To read the first 5 lines of an individual file:
    /// {
    ///     "path": "path/to/file.txt",
    ///     "output": "text"
    ///     "end": 5
    /// }
    ///
    /// To read all the entries in a directory:
    /// {
    ///     "path": "path/to/directory/",
    ///     "output": "text"
    /// }
    /// </example>
    Text,

    /// Output matching code symbols (such as identifiers, types, etc.) within files, including their line numbers.
    /// If no path is specified, outputs symbols found across the entire project.
    ///
    /// <example>
    /// To find all functions with "search" in their name:
    /// {
    ///     "query": "search",
    ///     "output": "symbols"
    /// }
    /// </example>
    Symbols,

    /// Output error and warning diagnostics for files matching the `path` glob.
    /// If no path is specified, outputs a summary of diagnostics found across the entire project.
    /// If query is specified, it is treated as a regex, and only shows individual diagnostics
    /// which match that regex.
    ///
    /// <example>
    /// To find all diagnostics in Rust files:
    /// {
    ///     "path_glob": "**/*.rs",
    ///     "output": "diagnostics"
    /// }
    /// </example>
    ///
    /// <example>
    /// To find diagnostics containing the word "unused":
    /// {
    ///     "query": "unused",
    ///     "output": "diagnostics"
    /// }
    /// </example>
    ///
    /// <example>
    /// To find a summary of all errors and warnings in the project:
    /// {
    ///     "output": "diagnostics"
    /// }
    /// </example>
    Diagnostics,
}

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

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<SearchToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        let todo = todo!(); // TODO render ui_text
        // match serde_json::from_value::<SearchToolInput>(input.clone()) {
        //     Ok(input) => {
        //         // Don't show any pattern if not specified
        //         let path_pattern = input.path.as_deref().map(MarkdownString::inline_code);
        //         let case_info = if input.query_case_sensitive {
        //             " (case-sensitive)"
        //         } else {
        //             ""
        //         };

        //         match input.output {
        //             Output::Paths => match path_pattern {
        //                 Some(pattern) => format!("Find paths matching {}{}", pattern, case_info),
        //                 None => format!("Find all paths{}", case_info),
        //             },
        //             Output::Text => {
        //                 if let Some(search_regex) = &input.query {
        //                     let search_pattern = MarkdownString::inline_code(search_regex);
        //                     match path_pattern {
        //                         Some(pattern) => format!(
        //                             "Search for {} in files matching {}{}",
        //                             search_pattern, pattern, case_info
        //                         ),
        //                         None => format!("Search for {}{}", search_pattern, case_info),
        //                     }
        //                 } else {
        //                     match path_pattern {
        //                         Some(pattern) => {
        //                             format!("Search in files matching {}{}", pattern, case_info)
        //                         }
        //                         None => format!("Search in all files{}", case_info),
        //                     }
        //                 }
        //             }
        //             Output::Symbols => {
        //                 if let Some(search_regex) = &input.query {
        //                     let search_pattern = MarkdownString::inline_code(search_regex);
        //                     match path_pattern {
        //                         Some(pattern) => format!(
        //                             "Find symbols matching {} in files matching {}{}",
        //                             search_pattern, pattern, case_info
        //                         ),
        //                         None => {
        //                             format!("Find symbols matching {}{}", search_pattern, case_info)
        //                         }
        //                     }
        //                 } else {
        //                     match path_pattern {
        //                         Some(pattern) => format!(
        //                             "Find symbols in files matching {}{}",
        //                             pattern, case_info
        //                         ),
        //                         None => format!("Find all symbols{}", case_info),
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     Err(_) => "Unified search".to_string(),
        // }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> ToolResult {
        let output = match serde_json::from_value::<SearchToolInput>(input) {
            Ok(input) => match input.output {
                Output::Paths => {
                    let todo = todo!(); // TODO process paths
                }
                Output::Text => text_search(
                    project,
                    input.path,
                    action_log,
                    input.query,
                    input.query_case_sensitive,
                    input.start,
                    input.end,
                    cx,
                ),
                Output::Symbols => match &input.path {
                    Some(path_glob) => {
                        let todo = todo!(); // TODO go get all the symbols in all those files
                    }
                    None => cx.spawn(async move |cx| {
                        project_symbols(
                            project,
                            &input.query.unwrap_or_default(),
                            input.start,
                            input.end,
                            cx,
                        )
                        .await
                    }),
                },
                Output::Diagnostics => todo!(),
            },
            Err(err) => Task::ready(Err(anyhow!(err))),
        };

        ToolResult { output }
    }
}

fn text_search(
    project: Entity<Project>,
    path_glob: Option<String>,
    action_log: Entity<ActionLog>,
    query: Option<String>,
    case_sensitive: bool,
    start: Option<u32>,
    end: Option<u32>,
    cx: &mut App,
) -> Task<Result<String>> {
    const MATCH_WHOLE_WORD: bool = false;
    const INCLUDE_IGNORED: bool = false;
    let files_to_exclude = PathMatcher::default();
    let Ok(files_to_include) = PathMatcher::new(path_glob.iter()) else {
        return Task::ready(Err(anyhow!(
            "Invalid path glob: {}",
            path_glob.unwrap_or_default()
        )));
    };

    // If a query regex is provided, create a search query for filtering files by content.
    // If it's not a valid regex, assume the model wanted an exact match.
    match query {
        Some(query_str) => {
            let Ok(query) = SearchQuery::regex(
                &query_str,
                MATCH_WHOLE_WORD,
                case_sensitive,
                INCLUDE_IGNORED,
                false,
                files_to_include.clone(),
                files_to_exclude.clone(),
                None, // buffers
            )
            .or_else({
                let query_str = &query_str;
                move |_| {
                    SearchQuery::text(
                        query_str,
                        MATCH_WHOLE_WORD,
                        case_sensitive,
                        INCLUDE_IGNORED,
                        files_to_include,
                        files_to_exclude,
                        None, // buffers
                    )
                }
            }) else {
                return Task::ready(Err(anyhow!("Invalid query regex: {query_str}")));
            };

            let results = project.update(cx, |project, cx| project.search(query, cx));

            cx.spawn(async move|cx|  {
                const CONTEXT_LINES: u32 = 2;

                futures::pin_mut!(results);

                let mut output = String::new();
                let start = start.unwrap_or(1);
                let mut skips_remaining = start.saturating_sub(1);
                let mut total_results_desired = end.map(|end| end.saturating_sub(start)).unwrap_or(u32::MAX);
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

                                // We'd already found all the matches we were asked to find, and we just found one more.
                                if matches_found >= total_results_desired {
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
                        "Showing matches {}-{} (there were more matches found; adjust start and/or end to others):\n{output}",
                        start,
                        start + matches_found,
                    ))
                } else {
                    Ok(format!("Found {matches_found} matches:\n{output}"))
                }
            })
        }
        None => {
            let todo = todo!(); // TODO don't actually do a search, just filter all the paths.
        }
    }
}

async fn project_symbols(
    project: Entity<Project>,
    query: &str,
    start: Option<u32>,
    end: Option<u32>,
    cx: &mut AsyncApp,
) -> anyhow::Result<String> {
    let symbols = project
        .update(cx, |project, cx| project.symbols(query, cx))?
        .await?;

    // We report a different error later on if there was a query.
    if symbols.is_empty() && query.is_empty() {
        return Err(anyhow!(
            "The language server found no code symbols in this project."
        ));
    }

    let mut symbols_by_path: IndexMap<PathBuf, Vec<Symbol>> = IndexMap::default();

    for symbol in symbols {
        if let Some(worktree_path) = project.read_with(cx, |project, cx| {
            project
                .worktree_for_id(symbol.path.worktree_id, cx)
                .map(|worktree| PathBuf::from(worktree.read(cx).root_name()))
        })? {
            let path = worktree_path.join(&symbol.path.path);
            symbols_by_path.entry(path).or_default().push(symbol);
        }
    }

    if symbols_by_path.is_empty() {
        Err(anyhow!(
            "The language server found no code symbols in this project when filtering by query {query:?}."
        ))
    } else {
        render_symbols_by_path(symbols_by_path, project, cx).await
    }
}

async fn render_symbols_by_path(
    symbols_by_path: impl IntoIterator<Item = (PathBuf, Vec<Symbol>)>,
    project: Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<String> {
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
