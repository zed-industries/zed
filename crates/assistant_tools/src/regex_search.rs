use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use futures::StreamExt;
use gpui::{App, Entity, Task};
use language::OffsetRangeExt;
use language_model::LanguageModelRequestMessage;
use project::{search::SearchQuery, Project};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp, fmt::Write, sync::Arc};
use util::paths::PathMatcher;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RegexSearchToolInput {
    /// A regex pattern to search for in the entire project. Note that the regex
    /// will be parsed by the Rust `regex` crate.
    pub regex: String,
}

pub struct RegexSearchTool;

impl Tool for RegexSearchTool {
    fn name(&self) -> String {
        "regex-search".into()
    }

    fn description(&self) -> String {
        include_str!("./regex_search_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(RegexSearchToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        const CONTEXT_LINES: u32 = 2;

        let input = match serde_json::from_value::<RegexSearchToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let query = match SearchQuery::regex(
            &input.regex,
            false,
            false,
            false,
            PathMatcher::default(),
            PathMatcher::default(),
            None,
        ) {
            Ok(query) => query,
            Err(error) => return Task::ready(Err(error)),
        };

        let results = project.update(cx, |project, cx| project.search(query, cx));
        cx.spawn(|cx| async move {
            futures::pin_mut!(results);

            let mut output = String::new();
            while let Some(project::search::SearchResult::Buffer { buffer, ranges }) =
                results.next().await
            {
                if ranges.is_empty() {
                    continue;
                }

                buffer.read_with(&cx, |buffer, cx| {
                    if let Some(path) = buffer.file().map(|file| file.full_path(cx)) {
                        writeln!(output, "### Found matches in {}:\n", path.display()).unwrap();
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
                            while let Some(next_range) = ranges.peek() {
                                if range.end.row >= next_range.start.row {
                                    range.end = next_range.end;
                                    ranges.next();
                                } else {
                                    break;
                                }
                            }

                            writeln!(output, "```").unwrap();
                            output.extend(buffer.text_for_range(range));
                            writeln!(output, "\n```\n").unwrap();
                        }
                    }
                })?;
            }

            if output.is_empty() {
                Ok("No matches found".into())
            } else {
                Ok(output)
            }
        })
    }
}
