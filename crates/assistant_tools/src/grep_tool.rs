use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use futures::StreamExt;
use gpui::{AnyWindowHandle, App, Entity, Task};
use language::{OffsetRangeExt, ParseStatus, Point};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::{
    Project,
    search::{SearchQuery, SearchResult},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp, fmt::Write, sync::Arc};
use ui::IconName;
use util::RangeExt;
use util::markdown::MarkdownInlineCode;
use util::paths::PathMatcher;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GrepToolInput {
    /// A regex pattern to search for in the entire project. Note that the regex
    /// will be parsed by the Rust `regex` crate.
    ///
    /// Do NOT specify a path here! This will only be matched against the code **content**.
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
                let regex_str = MarkdownInlineCode(&input.regex);
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
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        const CONTEXT_LINES: u32 = 2;
        const MAX_ANCESTOR_LINES: u32 = 10;

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

        cx.spawn(async move |cx|  {
            futures::pin_mut!(results);

            let mut output = String::new();
            let mut skips_remaining = input.offset;
            let mut matches_found = 0;
            let mut has_more_matches = false;

            'outer: while let Some(SearchResult::Buffer { buffer, ranges }) = results.next().await {
                if ranges.is_empty() {
                    continue;
                }

                let (Some(path), mut parse_status) = buffer.read_with(cx, |buffer, cx| {
                    (buffer.file().map(|file| file.full_path(cx)), buffer.parse_status())
                })? else {
                    continue;
                };


                while *parse_status.borrow() != ParseStatus::Idle {
                    parse_status.changed().await?;
                }

                let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

                let mut ranges = ranges
                    .into_iter()
                    .map(|range| {
                        let matched = range.to_point(&snapshot);
                        let matched_end_line_len = snapshot.line_len(matched.end.row);
                        let full_lines = Point::new(matched.start.row, 0)..Point::new(matched.end.row, matched_end_line_len);
                        let symbols = snapshot.symbols_containing(matched.start, None);

                        if let Some(ancestor_node) = snapshot.syntax_ancestor(full_lines.clone()) {
                            let full_ancestor_range = ancestor_node.byte_range().to_point(&snapshot);
                            let end_row = full_ancestor_range.end.row.min(full_ancestor_range.start.row + MAX_ANCESTOR_LINES);
                            let end_col = snapshot.line_len(end_row);
                            let capped_ancestor_range = Point::new(full_ancestor_range.start.row, 0)..Point::new(end_row, end_col);

                            if capped_ancestor_range.contains_inclusive(&full_lines) {
                                return (capped_ancestor_range, Some(full_ancestor_range), symbols)
                            }
                        }

                        let mut matched = matched;
                        matched.start.column = 0;
                        matched.start.row =
                            matched.start.row.saturating_sub(CONTEXT_LINES);
                        matched.end.row = cmp::min(
                            snapshot.max_point().row,
                            matched.end.row + CONTEXT_LINES,
                        );
                        matched.end.column = snapshot.line_len(matched.end.row);

                        (matched, None, symbols)
                    })
                    .peekable();

                let mut file_header_written = false;

                while let Some((mut range, ancestor_range, parent_symbols)) = ranges.next(){
                    if skips_remaining > 0 {
                        skips_remaining -= 1;
                        continue;
                    }

                    // We'd already found a full page of matches, and we just found one more.
                    if matches_found >= RESULTS_PER_PAGE {
                        has_more_matches = true;
                        break 'outer;
                    }

                    while let Some((next_range, _, _)) = ranges.peek() {
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

                    let end_row = range.end.row;
                    output.push_str("\n### ");

                    if let Some(parent_symbols) = &parent_symbols {
                        for symbol in parent_symbols {
                            write!(output, "{} › ", symbol.text)?;
                        }
                    }

                    if range.start.row == end_row {
                        writeln!(output, "L{}", range.start.row + 1)?;
                    } else {
                        writeln!(output, "L{}-{}", range.start.row + 1, end_row + 1)?;
                    }

                    output.push_str("```\n");
                    output.extend(snapshot.text_for_range(range));
                    output.push_str("\n```\n");

                    if let Some(ancestor_range) = ancestor_range {
                        if end_row < ancestor_range.end.row {
                            let remaining_lines = ancestor_range.end.row - end_row;
                            writeln!(output, "\n{} lines remaining in ancestor node. Read the file to see all.", remaining_lines)?;
                        }
                    }

                    matches_found += 1;
                }
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
    use language::{Language, LanguageConfig, LanguageMatcher};
    use project::{FakeFs, Project};
    use settings::SettingsStore;
    use unindent::Unindent;
    use util::path;

    #[gpui::test]
    async fn test_grep_tool_with_include_pattern(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

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
        cx.executor().allow_parking();

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

    /// Helper function to set up a syntax test environment
    async fn setup_syntax_test(cx: &mut TestAppContext) -> Entity<Project> {
        use unindent::Unindent;
        init_test(cx);
        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor().clone());

        // Create test file with syntax structures
        fs.insert_tree(
            "/root",
            serde_json::json!({
                "test_syntax.rs": r#"
                    fn top_level_function() {
                        println!("This is at the top level");
                    }

                    mod feature_module {
                        pub mod nested_module {
                            pub fn nested_function(
                                first_arg: String,
                                second_arg: i32,
                            ) {
                                println!("Function in nested module");
                                println!("{first_arg}");
                                println!("{second_arg}");
                            }
                        }
                    }

                    struct MyStruct {
                        field1: String,
                        field2: i32,
                    }

                    impl MyStruct {
                        fn method_with_block() {
                            let condition = true;
                            if condition {
                                println!("Inside if block");
                            }
                        }

                        fn long_function() {
                            println!("Line 1");
                            println!("Line 2");
                            println!("Line 3");
                            println!("Line 4");
                            println!("Line 5");
                            println!("Line 6");
                            println!("Line 7");
                            println!("Line 8");
                            println!("Line 9");
                            println!("Line 10");
                            println!("Line 11");
                            println!("Line 12");
                        }
                    }

                    trait Processor {
                        fn process(&self, input: &str) -> String;
                    }

                    impl Processor for MyStruct {
                        fn process(&self, input: &str) -> String {
                            format!("Processed: {}", input)
                        }
                    }
                "#.unindent().trim(),
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        project.update(cx, |project, _cx| {
            project.languages().add(rust_lang().into())
        });

        project
    }

    #[gpui::test]
    async fn test_grep_top_level_function(cx: &mut TestAppContext) {
        let project = setup_syntax_test(cx).await;

        // Test: Line at the top level of the file
        let input = serde_json::to_value(GrepToolInput {
            regex: "This is at the top level".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        let expected = r#"
            Found 1 matches:

            ## Matches in root/test_syntax.rs

            ### fn top_level_function › L1-3
            ```
            fn top_level_function() {
                println!("This is at the top level");
            }
            ```
            "#
        .unindent();
        assert_eq!(result, expected);
    }

    #[gpui::test]
    async fn test_grep_function_body(cx: &mut TestAppContext) {
        let project = setup_syntax_test(cx).await;

        // Test: Line inside a function body
        let input = serde_json::to_value(GrepToolInput {
            regex: "Function in nested module".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        let expected = r#"
            Found 1 matches:

            ## Matches in root/test_syntax.rs

            ### mod feature_module › pub mod nested_module › pub fn nested_function › L10-14
            ```
                    ) {
                        println!("Function in nested module");
                        println!("{first_arg}");
                        println!("{second_arg}");
                    }
            ```
            "#
        .unindent();
        assert_eq!(result, expected);
    }

    #[gpui::test]
    async fn test_grep_function_args_and_body(cx: &mut TestAppContext) {
        let project = setup_syntax_test(cx).await;

        // Test: Line with a function argument
        let input = serde_json::to_value(GrepToolInput {
            regex: "second_arg".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        let expected = r#"
            Found 1 matches:

            ## Matches in root/test_syntax.rs

            ### mod feature_module › pub mod nested_module › pub fn nested_function › L7-14
            ```
                    pub fn nested_function(
                        first_arg: String,
                        second_arg: i32,
                    ) {
                        println!("Function in nested module");
                        println!("{first_arg}");
                        println!("{second_arg}");
                    }
            ```
            "#
        .unindent();
        assert_eq!(result, expected);
    }

    #[gpui::test]
    async fn test_grep_if_block(cx: &mut TestAppContext) {
        use unindent::Unindent;
        let project = setup_syntax_test(cx).await;

        // Test: Line inside an if block
        let input = serde_json::to_value(GrepToolInput {
            regex: "Inside if block".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        let expected = r#"
            Found 1 matches:

            ## Matches in root/test_syntax.rs

            ### impl MyStruct › fn method_with_block › L26-28
            ```
                    if condition {
                        println!("Inside if block");
                    }
            ```
            "#
        .unindent();
        assert_eq!(result, expected);
    }

    #[gpui::test]
    async fn test_grep_long_function_top(cx: &mut TestAppContext) {
        use unindent::Unindent;
        let project = setup_syntax_test(cx).await;

        // Test: Line in the middle of a long function - should show message about remaining lines
        let input = serde_json::to_value(GrepToolInput {
            regex: "Line 5".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        let expected = r#"
            Found 1 matches:

            ## Matches in root/test_syntax.rs

            ### impl MyStruct › fn long_function › L31-41
            ```
                fn long_function() {
                    println!("Line 1");
                    println!("Line 2");
                    println!("Line 3");
                    println!("Line 4");
                    println!("Line 5");
                    println!("Line 6");
                    println!("Line 7");
                    println!("Line 8");
                    println!("Line 9");
                    println!("Line 10");
            ```

            3 lines remaining in ancestor node. Read the file to see all.
            "#
        .unindent();
        assert_eq!(result, expected);
    }

    #[gpui::test]
    async fn test_grep_long_function_bottom(cx: &mut TestAppContext) {
        use unindent::Unindent;
        let project = setup_syntax_test(cx).await;

        // Test: Line in the long function
        let input = serde_json::to_value(GrepToolInput {
            regex: "Line 12".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let result = run_grep_tool(input, project.clone(), cx).await;
        let expected = r#"
            Found 1 matches:

            ## Matches in root/test_syntax.rs

            ### impl MyStruct › fn long_function › L41-45
            ```
                    println!("Line 10");
                    println!("Line 11");
                    println!("Line 12");
                }
            }
            ```
            "#
        .unindent();
        assert_eq!(result, expected);
    }

    async fn run_grep_tool(
        input: serde_json::Value,
        project: Entity<Project>,
        cx: &mut TestAppContext,
    ) -> String {
        let tool = Arc::new(GrepTool);
        let action_log = cx.new(|_cx| ActionLog::new(project.clone()));
        let task = cx.update(|cx| tool.run(input, &[], project, action_log, None, cx));

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

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_outline_query(include_str!("../../languages/src/rust/outline.scm"))
        .unwrap()
    }
}
