use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool};
use collections::IndexMap;
use futures::StreamExt;
use gpui::{App, AsyncApp, Entity, Task};
use language::{
    BufferSnapshot, Location, OffsetRangeExt, OutlineItem, ParseStatus, Point, ToPoint,
};
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
    /// - If the "output" parameter is "text", then this query will be interpreted as a regex, and only text snippets matching that regex will be included.
    /// - If the "output" parameter is "paths", then this query will be interpreted as a regex, and only files whose text contents match that regex will be included.
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
    /// Output the *definitions* of matching code symbols (such as identifiers, types, etc.)
    /// within files, including line numbers. If no path_glob is specified, outputs
    /// the definitions of matching code symbols found across the entire project.
    ///
    /// This is different from "declarations" in that the "definition" is where the code symbol
    /// is first *assigned*, even if it was given its name earlier. For example, in C, there
    /// might be a "declaration" of `int a;` and then later a "definition" of `a = 5;`
    ///
    /// <example>
    /// Using "output": "definitions" and a "query" of "abc" would output the `abc = 5;` line of this C code:
    ///
    /// ```c
    /// int abc;
    /// int xyz;
    /// abc = 5;
    /// xyz = 6;
    /// ```
    /// </example>
    Definitions,
    /// Output the *declarations* of matching code symbols (such as identifiers, types, etc.)
    /// within files, including line numbers. If no path_glob is specified, outputs
    /// the declarations of matching code symbols found across the entire project.
    ///
    /// This is different from "definition" in that the "declaration" is where the code symbol
    /// is first given a name, even if it's not assigned a value until later. For example, in C,
    /// there might be a "declaration" of `int a;` and then later a "definition" of `a = 5;`
    ///
    /// <example>
    /// Using "output": "declarations" and a "query" of "abc" would output the `int abc;` line of this C code:
    ///
    /// ```c
    /// int abc;
    /// int xyz;
    /// abc = 5;
    /// xyz = 6;
    /// ```
    /// </example>
    Declarations,
    /// Output the *implementations* of matching code symbols (such as identifiers, types, etc.)
    /// within files, including line numbers. If no path_glob is specified, outputs
    /// the implementations of matching code symbols found across the entire project.
    ///
    /// As an example, in a Java code base you might use this to query for method which is
    /// defined in an interface or abstract class, and potentially implemented in multiple derived
    /// classes. This "implementations" output type would tell you about those multiple implementations
    /// in the derived classes.
    Implementations,
    /// Output the *type definitions* of matching code symbols (such as identifiers, types, etc.)
    /// within files, including line numbers. If no path_glob is specified, outputs
    /// the type definitions of matching code symbols found across the entire project.
    ///
    /// <example>
    /// Using "output": "types" and a "query" of "abc" would output the `enum direction { N, E, S, W };` line of this C code:
    ///
    /// ```c
    /// enum direction { N, E, S, W };
    ///
    /// int x = 5;
    /// enum direction abc = N;
    /// ```
    /// </example>
    Types,
    /// Output the *references* of matching code symbols (such as identifiers, types, etc.)
    /// within files, including line numbers. If no path_glob is specified, outputs
    /// the type definitions of matching code symbols found across the entire project.
    References,
}

// Different search modes have different pagination limits
const PATHS_RESULTS_PER_PAGE: usize = 50;
const TEXT_RESULTS_PER_PAGE: usize = 20;
const SYMBOLS_RESULTS_PER_PAGE: u32 = 100;

pub struct SearchTool;

#[derive(Debug, Clone, Copy)]
enum SymbolInfoType {
    Definition,
    Declaration,
    Implementation,
    TypeDefinition,
    References,
}

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
                    Output::Definitions => {
                        if let Some(search_regex) = &input.query {
                            let search_pattern = MarkdownString::inline_code(search_regex);
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find definitions of {} in files matching {}{}",
                                    search_pattern, pattern, case_info
                                ),
                                None => {
                                    format!("Find definitions of {}{}", search_pattern, case_info)
                                }
                            }
                        } else {
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find definitions in files matching {}{}",
                                    pattern, case_info
                                ),
                                None => format!("Find all definitions{}", case_info),
                            }
                        }
                    }
                    Output::Declarations => {
                        if let Some(search_regex) = &input.query {
                            let search_pattern = MarkdownString::inline_code(search_regex);
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find declarations of {} in files matching {}{}",
                                    search_pattern, pattern, case_info
                                ),
                                None => {
                                    format!("Find declarations of {}{}", search_pattern, case_info)
                                }
                            }
                        } else {
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find declarations in files matching {}{}",
                                    pattern, case_info
                                ),
                                None => format!("Find all declarations{}", case_info),
                            }
                        }
                    }
                    Output::Implementations => {
                        if let Some(search_regex) = &input.query {
                            let search_pattern = MarkdownString::inline_code(search_regex);
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find implementations of {} in files matching {}{}",
                                    search_pattern, pattern, case_info
                                ),
                                None => {
                                    format!(
                                        "Find implementations of {}{}",
                                        search_pattern, case_info
                                    )
                                }
                            }
                        } else {
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find implementations in files matching {}{}",
                                    pattern, case_info
                                ),
                                None => format!("Find all implementations{}", case_info),
                            }
                        }
                    }
                    Output::Types => {
                        if let Some(search_regex) = &input.query {
                            let search_pattern = MarkdownString::inline_code(search_regex);
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find type definitions of {} in files matching {}{}",
                                    search_pattern, pattern, case_info
                                ),
                                None => {
                                    format!(
                                        "Find type definitions of {}{}",
                                        search_pattern, case_info
                                    )
                                }
                            }
                        } else {
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find type definitions in files matching {}{}",
                                    pattern, case_info
                                ),
                                None => format!("Find all type definitions{}", case_info),
                            }
                        }
                    }
                    Output::References => {
                        if let Some(search_regex) = &input.query {
                            let search_pattern = MarkdownString::inline_code(search_regex);
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find references to {} in files matching {}{}",
                                    search_pattern, pattern, case_info
                                ),
                                None => {
                                    format!("Find references to {}{}", search_pattern, case_info)
                                }
                            }
                        } else {
                            match path_pattern {
                                Some(pattern) => format!(
                                    "Find references in files matching {}{}",
                                    pattern, case_info
                                ),
                                None => format!("Find all references{}", case_info),
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
            Output::Text => search_text(input, project, cx),
            Output::Symbols => search_symbols(input, project, action_log, cx),
            Output::Definitions => {
                search_symbol_info(input, project, action_log, SymbolInfoType::Definition, cx)
            }
            Output::Declarations => {
                search_symbol_info(input, project, action_log, SymbolInfoType::Declaration, cx)
            }
            Output::Implementations => search_symbol_info(
                input,
                project,
                action_log,
                SymbolInfoType::Implementation,
                cx,
            ),
            Output::Types => search_symbol_info(
                input,
                project,
                action_log,
                SymbolInfoType::TypeDefinition,
                cx,
            ),
            Output::References => {
                search_symbol_info(input, project, action_log, SymbolInfoType::References, cx)
            }
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

    let snapshots: Vec<Snapshot> = project
        .read(cx)
        .worktrees(cx)
        .map(|worktree| worktree.read(cx).snapshot())
        .collect();

    // Create a copy of path_glob for use in the async closure
    let path_glob_for_error = path_glob_option.clone();

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
    cx: &mut App,
) -> Task<Result<String>> {
    const CONTEXT_LINES: u32 = 2;

    let search_regex = match &input.query {
        Some(regex) => regex.clone(),
        None => {
            return Task::ready(Err(anyhow!(
                "file_contents_regex is required for text search mode"
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
        &search_regex,
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
                file_outline(
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

async fn file_symbol_info(
    project: Entity<Project>,
    path: String,
    query: Option<String>,
    action_log: Entity<ActionLog>,
    info_type: SymbolInfoType,
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

    // Wait until the buffer has been fully parsed
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

    // Filter outline items based on query if provided
    let filtered_items: Vec<_> = outline
        .items
        .into_iter()
        .filter(|item| query.as_ref().map_or(true, |q| item.text.contains(q)))
        .collect();

    // If there are no matching items, return early
    if filtered_items.is_empty() {
        return Err(anyhow!("No symbols matching query found in file: {}", path));
    }

    // Process each filtered symbol to get the requested information
    let mut results = String::new();
    let mut items_processed = 0;

    for item in filtered_items
        .into_iter()
        .skip(offset as usize)
        .take(SYMBOLS_RESULTS_PER_PAGE as usize)
    {
        let point_item = item.to_point(&snapshot);
        let position = point_item.range.start;

        dbg!(&item.text);

        match dbg!(info_type) {
            SymbolInfoType::Definition => {
                let definitions = project
                    .update(cx, |project, cx| project.definition(&buffer, position, cx))?
                    .await?;

                if !definitions.is_empty() {
                    write_symbol_locations(
                        &mut results,
                        &item.text,
                        definitions.into_iter().map(|link| link.target),
                        cx,
                    )?;
                    items_processed += 1;
                }
            }
            SymbolInfoType::Declaration => {
                let declarations = project
                    .update(cx, |project, cx| project.declaration(&buffer, position, cx))?
                    .await?;

                if !declarations.is_empty() {
                    write_symbol_locations(
                        &mut results,
                        &item.text,
                        declarations.into_iter().map(|link| link.target),
                        cx,
                    )?;
                    items_processed += 1;
                }
            }
            SymbolInfoType::Implementation => {
                let implementations = project
                    .update(cx, |project, cx| {
                        project.implementation(&buffer, position, cx)
                    })?
                    .await?;

                if !implementations.is_empty() {
                    write_symbol_locations(
                        &mut results,
                        &item.text,
                        implementations.into_iter().map(|link| link.target),
                        cx,
                    )?;
                    items_processed += 1;
                }
            }
            SymbolInfoType::TypeDefinition => {
                let type_defs = project
                    .update(cx, |project, cx| {
                        project.type_definition(&buffer, position, cx)
                    })?
                    .await?;

                if !type_defs.is_empty() {
                    write_symbol_locations(
                        &mut results,
                        &item.text,
                        type_defs.into_iter().map(|link| link.target),
                        cx,
                    )?;
                    items_processed += 1;
                }
            }
            SymbolInfoType::References => {
                let references = project
                    .update(cx, |project, cx| project.references(&buffer, position, cx))?
                    .await?;

                if !references.is_empty() {
                    write_symbol_locations(&mut results, &item.text, references, cx)?;
                    items_processed += 1;
                }
            }
        }
    }

    if results.is_empty() {
        Err(anyhow!(
            "No {} found for symbols in {}",
            format!("{:?}", info_type).to_lowercase(),
            path
        ))
    } else {
        Ok(format!(
            "Found {} items with {}:\n\n{}",
            items_processed,
            format!("{:?}", info_type).to_lowercase(),
            results
        ))
    }
}

async fn file_outline(
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

    render_outline(
        &mut output,
        outline
            .items
            .into_iter()
            .map(|item| item.to_point(&snapshot)),
        query,
        offset,
        SYMBOLS_RESULTS_PER_PAGE,
    )
}

fn search_symbol_info(
    input: SearchToolInput,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    info_type: SymbolInfoType,
    cx: &mut App,
) -> Task<Result<String>> {
    // Check if path_glob is a specific file path
    if let Some(path) = &input.path_glob {
        // If the glob pattern doesn't contain wildcards, assume it's a specific file path
        if !path.contains('*') && !path.contains('?') && !path.contains('[') {
            let path_string = path.clone();
            return cx.spawn(async move |cx| {
                file_symbol_info(
                    project,
                    path_string,
                    input.query,
                    action_log,
                    info_type,
                    input.offset,
                    cx,
                )
                .await
            });
        }
    }

    // Create a path matcher for filtering symbols
    let path_matcher = if let Some(glob) = input.path_glob.as_deref() {
        match PathMatcher::new([glob]) {
            Ok(matcher) => Some(matcher),
            Err(err) => return Task::ready(Err(anyhow!("Invalid glob pattern: {}", err))),
        }
    } else {
        None
    };

    cx.spawn(async move |cx| {
        project_symbol_info(
            project,
            path_matcher,
            &input.query.unwrap_or_default(),
            info_type,
            input.offset,
            cx,
        )
        .await
    })
}

async fn project_symbol_info(
    project: Entity<Project>,
    path_matcher: Option<PathMatcher>,
    query: &str,
    info_type: SymbolInfoType,
    offset: u32,
    cx: &mut AsyncApp,
) -> anyhow::Result<String> {
    // First get all the project symbols
    let symbols = project
        .update(cx, |project, cx| project.symbols(query, cx))?
        .await?;

    if symbols.is_empty() {
        return Err(anyhow!("No matching symbols found in project."));
    }

    // Filter symbols by path if matcher is provided
    let filtered_symbols: Vec<_> = symbols
        .iter()
        .filter(|symbol| {
            path_matcher
                .as_ref()
                .map(|matcher| matcher.is_match(&symbol.path.path))
                .unwrap_or(true)
        })
        .collect();

    if filtered_symbols.is_empty() {
        return Err(anyhow!("No symbols found matching the criteria."));
    }

    // Apply pagination
    let page_symbols = filtered_symbols
        .into_iter()
        .skip(offset as usize)
        .take(SYMBOLS_RESULTS_PER_PAGE as usize);

    let mut result = String::new();
    let mut processed_count = 0;

    for symbol in page_symbols {
        dbg!(&symbol.name);
        // Get the project path for the symbol
        let project_path = project.read_with(cx, |project, cx| {
            if let Some(worktree) = project.worktree_for_id(symbol.path.worktree_id, cx) {
                let worktree_path = worktree.read(cx).root_name().to_string();
                Some(
                    project
                        .find_project_path(
                            &format!("{}/{}", worktree_path, symbol.path.path.to_string_lossy()),
                            cx,
                        )
                        .unwrap(),
                )
            } else {
                None
            }
        })?;

        let Some(project_path) = project_path else {
            continue;
        };

        // Open the buffer to process the symbol
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))?
            .await?;

        dbg!(&symbol);

        // Convert from symbol range to position
        let position = buffer.read_with(cx, |_, _| Point {
            row: symbol.range.start.0.row,
            column: symbol.range.start.0.column,
        })?;

        match dbg!(info_type) {
            SymbolInfoType::Definition => {
                dbg!(&position);
                let definitions = project
                    .update(cx, |project, cx| project.definition(&buffer, position, cx))?
                    .await?;

                dbg!(&definitions);
                if !definitions.is_empty() {
                    write_symbol_locations(
                        &mut result,
                        &symbol.label.text().to_string(),
                        definitions.into_iter().map(|link| link.target),
                        cx,
                    )?;
                    processed_count += 1;
                }
            }
            SymbolInfoType::Declaration => {
                dbg!(&position);
                let declarations = project
                    .update(cx, |project, cx| project.declaration(&buffer, position, cx))?
                    .await?;

                dbg!(&declarations);
                if !declarations.is_empty() {
                    write_symbol_locations(
                        &mut result,
                        &symbol.label.text().to_string(),
                        declarations.into_iter().map(|link| link.target),
                        cx,
                    )?;
                    processed_count += 1;
                }
            }
            SymbolInfoType::Implementation => {
                let implementations = project
                    .update(cx, |project, cx| {
                        project.implementation(&buffer, position, cx)
                    })?
                    .await?;

                if !implementations.is_empty() {
                    write_symbol_locations(
                        &mut result,
                        &symbol.label.text().to_string(),
                        implementations.into_iter().map(|link| link.target),
                        cx,
                    )?;
                    processed_count += 1;
                }
            }
            SymbolInfoType::TypeDefinition => {
                let type_defs = project
                    .update(cx, |project, cx| {
                        project.type_definition(&buffer, position, cx)
                    })?
                    .await?;

                if !type_defs.is_empty() {
                    write_symbol_locations(
                        &mut result,
                        &symbol.label.text().to_string(),
                        type_defs.into_iter().map(|link| link.target),
                        cx,
                    )?;
                    processed_count += 1;
                }
            }
            SymbolInfoType::References => {
                let references = project
                    .update(cx, |project, cx| project.references(&buffer, position, cx))?
                    .await?;

                if !references.is_empty() {
                    write_symbol_locations(
                        &mut result,
                        &symbol.label.text().to_string(),
                        references,
                        cx,
                    )?;
                    processed_count += 1;
                }
            }
        }
    }

    if processed_count == 0 {
        Err(anyhow!(
            "No {} found for any of the symbols.",
            format!("{:?}", info_type).to_lowercase()
        ))
    } else {
        Ok(format!(
            "Found {} for {} symbols:\n\n{}",
            format!("{:?}", info_type).to_lowercase(),
            processed_count,
            result
        ))
    }
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

    for (file_path, file_symbols) in symbols_by_path {
        if symbols_rendered > 0 {
            output.push('\n');
        }

        writeln!(&mut output, "## {}", file_path.display()).ok();

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

    Ok(output)
}

fn render_outline(
    output: &mut String,
    items: impl IntoIterator<Item = OutlineItem<Point>>,
    query: Option<String>,
    offset: u32,
    results_per_page: u32,
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

fn write_symbol_locations(
    output: &mut String,
    symbol_name: &str,
    locations: impl IntoIterator<Item = Location>,
    cx: &mut AsyncApp,
) -> anyhow::Result<()> {
    writeln!(output, "### Symbol: {}", symbol_name).ok();

    let mut location_count = 0;

    for location in locations {
        location
            .buffer
            .read_with(cx, |buffer, _cx| {
                if let Some(target_path) = buffer
                    .file()
                    .and_then(|file| file.path().as_os_str().to_str())
                {
                    let snapshot = buffer.snapshot();
                    let start = location.range.start.to_point(&snapshot);
                    let end = location.range.end.to_point(&snapshot);

                    // Convert to 1-based line/column numbers for display
                    let start_line = start.row + 1;
                    let start_col = start.column + 1;
                    let end_line = end.row + 1;
                    let end_col = end.column + 1;

                    if start_line == end_line {
                        writeln!(output, "- {}:{}:{}", target_path, start_line, start_col).ok();
                    } else {
                        writeln!(
                            output,
                            "- {}:{}:{}-{}:{}",
                            target_path, start_line, start_col, end_line, end_col
                        )
                        .ok();
                    }

                    // Add code excerpt
                    write_code_excerpt(output, &snapshot, start, end);
                    location_count += 1;
                }
            })
            .ok();
    }

    if location_count > 0 {
        writeln!(output).ok();
    }

    Ok(())
}

fn write_code_excerpt(output: &mut String, snapshot: &BufferSnapshot, start: Point, end: Point) {
    const MAX_CONTEXT_LINES: u32 = 3;
    const MAX_LINE_LEN: u32 = 200;

    let start_row = start.row.saturating_sub(MAX_CONTEXT_LINES);
    let end_row = cmp::min(snapshot.max_point().row, end.row + MAX_CONTEXT_LINES);

    writeln!(output, "```").ok();

    for row in start_row..=end_row {
        let row_start = Point::new(row, 0);
        let row_end = if row < snapshot.max_point().row {
            Point::new(row + 1, 0)
        } else {
            Point::new(row, u32::MAX)
        };

        // Add line number prefix
        write!(output, "{}: ", row + 1).ok();

        // Add line content with truncation if too long
        let line_content = snapshot
            .text_for_range(row_start..row_end)
            .take(MAX_LINE_LEN as usize)
            .collect::<String>();

        output.push_str(&line_content);

        if row_end.column > MAX_LINE_LEN {
            output.push_str("…");
        }

        output.push('\n');
    }

    writeln!(output, "```").ok();
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
