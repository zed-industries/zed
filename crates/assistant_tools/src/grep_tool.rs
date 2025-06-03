use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use futures::StreamExt;
use gpui::{AnyWindowHandle, App, Entity, Task};
use language::{OffsetRangeExt, ParseStatus, Point};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
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
        _request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        const CONTEXT_LINES: u32 = 2;
        const MAX_ANCESTOR_LINES: u32 = 10;

        let input = match serde_json::from_value::<GrepToolInput>(input) {
            Ok(input) => input,
            Err(error) => {
                return Task::ready(Err(anyhow!("Failed to parse input: {error}"))).into();
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
                return Task::ready(Err(anyhow!("invalid include glob pattern: {error}"))).into();
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
                Ok("No matches found".to_string().into())
            } else if has_more_matches {
                Ok(format!(
                    "Showing matches {}-{} (there were more matches found; use offset: {} to see next page):\n{output}",
                    input.offset + 1,
                    input.offset + matches_found,
                    input.offset + RESULTS_PER_PAGE,
                ).into())
            } else {
                Ok(format!("Found {matches_found} matches:\n{output}").into())
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
    use language_model::fake_provider::FakeLanguageModel;
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
        let model = Arc::new(FakeLanguageModel::default());
        let task =
            cx.update(|cx| tool.run(input, Arc::default(), project, action_log, model, None, cx));

        match task.output.await {
            Ok(result) => {
                if cfg!(windows) {
                    result.content.as_str().unwrap().replace("root\\", "root/")
                } else {
                    result.content.as_str().unwrap().to_string()
                }
            }
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

    #[gpui::test]
    async fn test_grep_security_boundaries(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        // This test verifies two critical security requirements for the grep tool:
        // 1. It should NEVER search files outside the project worktree boundaries
        // 2. It should respect file exclusion settings (e.g., .git, node_modules, .env files)
        //
        // CURRENT STATUS:
        // ✓ Requirement 1 is working correctly - files outside project are not searched
        // ✗ Requirement 2 is BROKEN - excluded files are being returned in search results
        //
        // EXPECTED BEHAVIOR ONCE FIXED:
        // The grep tool should use the project's file exclusion settings (likely from
        // ProjectSettings or gitignore rules) to filter out sensitive files. This means:
        // - .git directories should never be searched
        // - node_modules should be excluded by default
        // - Files matching .gitignore patterns should be excluded
        // - Hidden files (starting with .) should follow project settings
        // - Binary files and build artifacts should be excluded
        // - Files matching the private_files setting should be excluded
        //
        // The test is intentionally written to fail until this security issue is resolved.

        let fs = FakeFs::new(cx.executor().clone());

        // Create a comprehensive test structure with:
        // - Multiple worktree roots (/project1, /project2)
        // - Files outside the project (/outside, /home) that should NEVER be searched
        // - Various file types that should be excluded by default
        fs.insert_tree(
            "/",
            serde_json::json!({
                // === PROJECT WORKTREE 1 ===
                "project1": {
                    // Normal files - should be included in search
                    "main.rs": "fn main() { println!(\"Hello from project1\"); }",
                    "lib.rs": "fn library_function() { /* project1 lib */ }",
                    "README.md": "# Project 1\nfn in_markdown() { }",

                    // Files that should be excluded by default
                    ".git": {
                        "config": "fn git_config() { /* should be excluded */ }",
                        "HEAD": "fn git_head() { /* should be excluded */ }"
                    },
                    "node_modules": {
                        "dep.rs": "fn node_module() { /* should be excluded */ }",
                        "package.json": "{ \"name\": \"fn_in_json\" }"
                    },
                    "target": {
                        "debug": {
                            "build.rs": "fn build_artifact() { /* should be excluded */ }"
                        }
                    },
                    ".env": "SECRET_KEY=abc123\nfn env_secrets() { /* should be excluded */ }",
                    ".env.local": "LOCAL_SECRET=xyz789\nfn local_env() { /* should be excluded */ }",
                    ".hidden.rs": "fn hidden_function() { /* should be excluded */ }",
                    ".DS_Store": "fn ds_store() { /* should be excluded */ }",
                    "private.pem": "fn private_key() { /* private_files setting */ }",
                    "server.key": "fn server_key() { /* private_files setting */ }",
                    "client.cert": "fn client_cert() { /* private_files setting */ }",
                    "secrets.yml": "fn secrets_yaml() { /* private_files setting */ }"
                },

                // === PROJECT WORKTREE 2 ===
                "project2": {
                    // Normal files - should be included
                    "app.rs": "fn app_function() { println!(\"Hello from project2\"); }",
                    "util.rs": "fn utility_function() { /* project2 util */ }",

                    // More excluded files
                    ".git": {
                        "hooks": {
                            "pre-commit": "fn pre_commit() { /* should be excluded */ }"
                        }
                    },
                    "target": {
                        "release": {
                            "final.rs": "fn release_build() { /* should be excluded */ }"
                        }
                    },
                    "Cargo.lock": "fn in_lock_file() { /* often excluded */ }",
                    ".gitignore": "target/\nfn in_gitignore() { }",
                    "api.key": "fn api_key() { /* private_files setting */ }",
                    "database.crt": "fn database_cert() { /* private_files setting */ }"
                },

                // === OUTSIDE PROJECT - CRITICAL: These should NEVER be searched ===
                "outside": {
                    "secret.rs": "fn secret_function() { /* OUTSIDE PROJECT */ }",
                    "config": {
                        "passwords.txt": "fn password() { /* OUTSIDE PROJECT */ }"
                    }
                },
                "home": {
                    "user": {
                        "private.rs": "fn private_data() { /* OUTSIDE PROJECT */ }",
                        ".ssh": {
                            "id_rsa": "fn ssh_key() { /* OUTSIDE PROJECT */ }"
                        }
                    }
                },
                "etc": {
                    "passwd": "fn system_file() { /* OUTSIDE PROJECT */ }"
                }
            }),
        )
        .await;

        // Create project with two worktree roots
        let project = Project::test(
            fs.clone(),
            [path!("/project1").as_ref(), path!("/project2").as_ref()],
            cx,
        )
        .await;

        // === TEST 1: Basic search to check file inclusion/exclusion ===
        println!("\n=== TEST 1: Testing file inclusion/exclusion rules ===");

        let input = serde_json::to_value(GrepToolInput {
            regex: "fn".to_string(),
            include_pattern: None,
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let results = run_grep_tool(input, project.clone(), cx).await;
        let result_paths = extract_paths_from_results(&results);

        println!("Found {} files with matches", result_paths.len());

        // Define expected behavior
        let should_include = vec![
            "project1/main.rs",
            "project1/lib.rs",
            "project1/README.md",
            "project2/app.rs",
            "project2/util.rs",
        ];

        let should_exclude_patterns = vec![
            (".git", "Git repository files"),
            ("node_modules", "Node.js dependencies"),
            ("target", "Build artifacts"),
            (".env", "Environment files with secrets (private_files)"),
            (".hidden", "Hidden files"),
            (".DS_Store", "macOS system files"),
            ("outside", "Files outside project roots"),
            ("/home/", "User home directory"),
            ("/etc/", "System files"),
            (".pem", "Private key files (private_files)"),
            (".key", "Key files (private_files)"),
            (".cert", "Certificate files (private_files)"),
            (".crt", "Certificate files (private_files)"),
            ("secrets.yml", "Secrets configuration (private_files)"),
        ];

        // Check included files
        println!("\nChecking files that SHOULD be included:");
        for path in &should_include {
            let found = result_paths.iter().any(|p| p.contains(path));
            println!("  {} {}", if found { "✓" } else { "✗" }, path);
            assert!(found, "Expected file '{}' was not found in results", path);
        }

        // Check excluded files
        println!("\nChecking patterns that SHOULD be excluded:");
        let mut security_issues = Vec::new();
        for (pattern, description) in &should_exclude_patterns {
            let found = result_paths.iter().any(|p| p.contains(pattern));
            if found {
                println!("  ✗ {} ({})", pattern, description);
                security_issues.push(format!("{} ({})", pattern, description));
            } else {
                println!("  ✓ {} ({})", pattern, description);
            }
        }

        // Report security issues
        if !security_issues.is_empty() {
            println!("\n⚠️  SECURITY ISSUES FOUND:");
            println!("The grep tool is exposing the following sensitive files:");
            for issue in &security_issues {
                println!("  - {}", issue);
            }
        }

        // === TEST 2: Verify project boundary enforcement ===
        println!("\n=== TEST 2: Testing project boundary enforcement ===");

        let boundary_input = serde_json::to_value(GrepToolInput {
            regex: "OUTSIDE PROJECT|secret_function|private_data|ssh_key|system_file".to_string(),
            include_pattern: None,
            offset: 0,
            case_sensitive: false,
        })
        .unwrap();

        let boundary_results = run_grep_tool(boundary_input, project.clone(), cx).await;
        let boundary_paths = extract_paths_from_results(&boundary_results);

        let outside_project_paths: Vec<_> = boundary_paths
            .iter()
            .filter(|path| !path.starts_with("project1/") && !path.starts_with("project2/"))
            .collect();

        if outside_project_paths.is_empty() {
            println!("✓ Project boundaries are properly enforced");
            println!("  No files outside /project1 and /project2 were searched");
        } else {
            println!("✗ CRITICAL: Project boundaries were violated!");
            println!("  The following paths outside the project were searched:");
            for path in &outside_project_paths {
                println!("    - {}", path);
            }
        }

        // === TEST 3: Try to escape project with malicious include patterns ===
        println!("\n=== TEST 3: Testing escape attempts with include patterns ===");

        let escape_attempts = vec![
            ("../outside/**/*.rs", "Relative path escape"),
            ("/outside/**/*.rs", "Absolute path escape"),
            ("../../**/*.rs", "Multiple parent directory escape"),
            ("/home/**/*", "Absolute path to home directory"),
        ];

        for (pattern, description) in escape_attempts {
            let escape_input = serde_json::to_value(GrepToolInput {
                regex: "fn".to_string(),
                include_pattern: Some(pattern.to_string()),
                offset: 0,
                case_sensitive: false,
            })
            .unwrap();

            let escape_results = run_grep_tool(escape_input, project.clone(), cx).await;
            let escape_paths = extract_paths_from_results(&escape_results);

            let escaped = escape_paths
                .iter()
                .any(|p| !p.starts_with("project1/") && !p.starts_with("project2/"));

            if escaped {
                println!("  ✗ {} - ESCAPED PROJECT BOUNDARY!", description);
            } else {
                println!("  ✓ {} - properly contained", description);
            }

            assert!(
                !escaped,
                "CRITICAL: Include pattern '{}' allowed escaping project boundaries!",
                pattern
            );
        }

        // === TEST 4: Verify private_files setting is respected ===
        println!("\n=== TEST 4: Testing private_files setting enforcement ===");

        // Test specifically for files that should be excluded by the private_files setting
        let private_files_patterns = vec![
            ".env",
            ".env.local",
            ".pem",
            ".key",
            ".cert",
            ".crt",
            "secrets.yml",
        ];

        let mut private_files_found = Vec::new();
        for pattern in &private_files_patterns {
            if result_paths.iter().any(|p| p.contains(pattern)) {
                private_files_found.push(pattern);
            }
        }

        if !private_files_found.is_empty() {
            println!("✗ SECURITY ISSUE: Files matching private_files setting are exposed:");
            for pattern in &private_files_found {
                println!("  - Files matching '{}'", pattern);
            }
        } else {
            println!("✓ No private files found (but this might be due to other exclusions)");
        }

        // === FINAL ASSERTIONS ===

        // This assertion documents the current BROKEN behavior
        // It SHOULD pass but currently fails, indicating the security issue
        assert!(
            !result_paths.iter().any(|p| p.contains(".env")),
            "SECURITY ISSUE: .env files containing secrets are being exposed in search results!"
        );

        // Additional assertion for private_files
        assert!(
            private_files_found.is_empty(),
            "SECURITY ISSUE: Files matching private_files setting are exposed: {:?}",
            private_files_found
        );

        // This assertion verifies project boundaries are respected (currently passing)
        assert!(
            outside_project_paths.is_empty(),
            "CRITICAL: Files outside project boundaries were searched!"
        );
    }

    // Helper function to extract file paths from grep results
    fn extract_paths_from_results(results: &str) -> Vec<String> {
        results
            .lines()
            .filter(|line| line.starts_with("## Matches in "))
            .map(|line| {
                line.strip_prefix("## Matches in ")
                    .unwrap()
                    .trim()
                    .to_string()
            })
            .collect()
    }
}
