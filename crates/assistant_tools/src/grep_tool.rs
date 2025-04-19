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
pub struct GrepToolInput {
    /// A regex pattern to search for in the entire project. Note that the regex
    /// will be parsed by the Rust `regex` crate.
    pub regex: String,

    /// A glob pattern for the paths of files to include in the search.
    /// Supports standard glob patterns like "**/*.rs" or "src/**/*.ts".
    /// If omitted, all files in the project will be searched.
    pub include_pattern: Option<String>,

    /// Optional starting position for paginated results (0-based).
    /// When not provided, starts from the beginning.
    #[serde(default)]
    pub offset: u32,

    /// Whether the regex is case-sensitive. Defaults to false (case-insensitive).
    #[serde(default)]
    pub case_sensitive: bool,
}

impl GrepToolInput {
    /// Which page of search results this is.
    pub fn page(&self) -> u32 {
        1 + (self.offset / RESULTS_PER_PAGE)
    }
}

const RESULTS_PER_PAGE: u32 = 20;

pub struct GrepTool;

impl Tool for GrepTool {
    fn name(&self) -> String {
        "grep".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./grep_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Regex
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<GrepToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<GrepToolInput>(input.clone()) {
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

        let input = match serde_json::from_value::<GrepToolInput>(input) {
            Ok(input) => input,
            Err(error) => {
                return Task::ready(Err(anyhow!("Failed to parse input: {}", error))).into();
            }
        };

        let include_matcher = match PathMatcher::new(
            input
                .include_pattern
                .as_ref()
                .into_iter()
                .collect::<Vec<_>>(),
        ) {
            Ok(matcher) => matcher,
            Err(error) => {
                return Task::ready(Err(anyhow!("invalid include glob pattern: {}", error))).into();
            }
        };

        let query = match SearchQuery::regex(
            &input.regex,
            false,
            input.case_sensitive,
            false,
            false,
            include_matcher,
            PathMatcher::default(), // For now, keep it simple and don't enable an exclude pattern.
            true, // Always match file include pattern against *full project paths* that start with a project root.
            None,
        ) {
            Ok(query) => query,
            Err(error) => return Task::ready(Err(error)).into(),
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
                    input.offset + 1,
                    input.offset + matches_found,
                    input.offset + RESULTS_PER_PAGE,
                ))
            } else {
                Ok(format!("Found {matches_found} matches:\n{output}"))
            }
        }).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assistant_tool::Tool;
    use gpui::{AppContext, TestAppContext};
    use project::{FakeFs, Project};
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_grep_tool_with_include_pattern(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root",
            serde_json::json!({
                "src": {
                    "main.rs": "fn main() {\n    println!(\"Hello, world!\");\n}",
                    "utils": {
                        "helper.rs": "fn helper() {\n    println!(\"I'm a helper!\");\n}",
                    },
                },
                "tests": {
                    "test_main.rs": "fn test_main() {\n    assert!(true);\n}",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        // Test with include pattern for Rust files inside the root of the project
        let input = serde_json::to_value(GrepToolInput {
            regex: "println".to_string(),
            include_pattern: Some("root/**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        assert!(result.contains("main.rs"), "Should find matches in main.rs");
        assert!(
            result.contains("helper.rs"),
            "Should find matches in helper.rs"
        );
        assert!(
            !result.contains("test_main.rs"),
            "Should not include test_main.rs even though it's a .rs file (because it doesn't have the pattern)"
        );

        // Test with include pattern for src directory only
        let input = serde_json::to_value(GrepToolInput {
            regex: "fn".to_string(),
            include_pattern: Some("root/**/src/**".to_string()),
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        assert!(
            result.contains("main.rs"),
            "Should find matches in src/main.rs"
        );
        assert!(
            result.contains("helper.rs"),
            "Should find matches in src/utils/helper.rs"
        );
        assert!(
            !result.contains("test_main.rs"),
            "Should not include test_main.rs as it's not in src directory"
        );

        // Test with empty include pattern (should default to all files)
        let input = serde_json::to_value(GrepToolInput {
            regex: "fn".to_string(),
            include_pattern: None,
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        assert!(result.contains("main.rs"), "Should find matches in main.rs");
        assert!(
            result.contains("helper.rs"),
            "Should find matches in helper.rs"
        );
        assert!(
            result.contains("test_main.rs"),
            "Should include test_main.rs"
        );
    }

    #[gpui::test]
    async fn test_grep_tool_with_case_sensitivity(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root",
            serde_json::json!({
                "case_test.txt": "This file has UPPERCASE and lowercase text.\nUPPERCASE patterns should match only with case_sensitive: true",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        // Test case-insensitive search (default)
        let input = serde_json::to_value(GrepToolInput {
            regex: "uppercase".to_string(),
            include_pattern: Some("**/*.txt".to_string()),
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        assert!(
            result.contains("UPPERCASE"),
            "Case-insensitive search should match uppercase"
        );

        // Test case-sensitive search
        let input = serde_json::to_value(GrepToolInput {
            regex: "uppercase".to_string(),
            include_pattern: Some("**/*.txt".to_string()),
            offset: 0,
            case_sensitive: true,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        assert!(
            !result.contains("UPPERCASE"),
            "Case-sensitive search should not match uppercase"
        );

        // Test case-sensitive search
        let input = serde_json::to_value(GrepToolInput {
            regex: "LOWERCASE".to_string(),
            include_pattern: Some("**/*.txt".to_string()),
            offset: 0,
            case_sensitive: true,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;

        assert!(
            !result.contains("lowercase"),
            "Case-sensitive search should match lowercase"
        );

        // Test case-sensitive search for lowercase pattern
        let input = serde_json::to_value(GrepToolInput {
            regex: "lowercase".to_string(),
            include_pattern: Some("**/*.txt".to_string()),
            offset: 0,
            case_sensitive: true,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        assert!(
            result.contains("lowercase"),
            "Case-sensitive search should match lowercase text"
        );
    }

    async fn run_grep_tool(
        input: serde_json::Value,
        project: Entity<Project>,
        cx: &mut TestAppContext,
    ) -> String {
        let tool = Arc::new(GrepTool);
        let action_log = cx.new(|_cx| ActionLog::new(project.clone()));
        let task = cx.update(|cx| tool.run(input, &[], project, action_log, cx));

        match task.output.await {
            Ok(result) => result,
            Err(e) => panic!("Failed to run grep tool: {}", e),
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }
}
