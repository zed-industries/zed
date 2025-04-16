use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use futures::StreamExt;
use gpui::{App, Entity, Task};
use language::OffsetRangeExt;
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::{
    Project,
    search::{SearchQuery, SearchResult},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp, fmt::Write, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownString;
use util::paths::PathMatcher;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RegexSearchToolInput {
    /// A regex pattern to search for in the entire project. Note that the regex
    /// will be parsed by the Rust `regex` crate.
    pub regex: String,

    /// Optional starting position for paginated results (0-based).
    /// When not provided, starts from the beginning.
    #[serde(default)]
    pub offset: u32,

    /// Whether the regex is case-sensitive. Defaults to false (case-insensitive).
    #[serde(default)]
    pub case_sensitive: bool,
}

impl RegexSearchToolInput {
    /// Which page of search results this is.
    pub fn page(&self) -> u32 {
        1 + (self.offset / RESULTS_PER_PAGE)
    }
}

const RESULTS_PER_PAGE: u32 = 20;

pub struct RegexSearchTool;

impl Tool for RegexSearchTool {
    fn name(&self) -> String {
        "regex_search".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./regex_search_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Regex
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<RegexSearchToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<RegexSearchToolInput>(input.clone()) {
            Ok(input) => {
                let page = input.page();
                let regex_str = MarkdownString::inline_code(&input.regex);
                let case_info = if input.case_sensitive {
                    " (case-sensitive)"
                } else {
                    ""
                };

                if page > 1 {
                    format!("Get page {page} of search results for regex {regex_str}{case_info}")
                } else {
                    format!("Search files for regex {regex_str}{case_info}")
                }
            }
            Err(_) => "Search with regex".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> ToolResult {
        const CONTEXT_LINES: u32 = 2;

        let (offset, regex, case_sensitive) =
            match serde_json::from_value::<RegexSearchToolInput>(input) {
                Ok(input) => (input.offset, input.regex, input.case_sensitive),
                Err(err) => return Task::ready(Err(anyhow!(err))).into(),
            };

        let query = match SearchQuery::regex(
            &regex,
            false,
            case_sensitive,
            false,
            false,
            PathMatcher::default(),
            PathMatcher::default(),
            None,
        ) {
            Ok(query) => query,
            Err(error) => return Task::ready(Err(error)).into(),
        };

        let results = project.update(cx, |project, cx| project.search(query, cx));

        cx.spawn(async move|cx|  {
            futures::pin_mut!(results);

            let mut output = String::new();
            let mut skips_remaining = offset;
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
                            if matches_found >= RESULTS_PER_PAGE {
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
                    offset + 1,
                    offset + matches_found,
                    offset + RESULTS_PER_PAGE,
                ))
            } else {
                Ok(format!("Found {matches_found} matches:\n{output}"))
            }
        }).into()
    }
}
