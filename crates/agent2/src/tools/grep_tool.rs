use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use futures::StreamExt;
use gpui::{App, Entity, SharedString, Task};
use language::{OffsetRangeExt, ParseStatus, Point};
use project::{
    Project, WorktreeSettings,
    search::{SearchQuery, SearchResult},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{cmp, fmt::Write, sync::Arc};
use util::RangeExt;
use util::markdown::MarkdownInlineCode;
use util::paths::PathMatcher;

/// Searches the contents of files in the project with a regular expression
///
/// - Prefer this tool to path search when searching for symbols in the project, because you won't need to guess what path it's in.
/// - Supports full regex syntax (eg. "log.*Error", "function\\s+\\w+", etc.)
/// - Pass an `include_pattern` if you know how to narrow your search on the files system
/// - Never use this tool to search for paths. Only search file contents with this tool.
/// - Use this tool when you need to find files containing specific patterns
/// - Results are paginated with 20 matches per page. Use the optional 'offset' parameter to request subsequent pages.
/// - DO NOT use HTML entities solely to escape characters in the tool parameters.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GrepToolInput {
    /// A regex pattern to search for in the entire project. Note that the regex will be parsed by the Rust `regex` crate.
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

pub struct GrepTool {
    project: Entity<Project>,
}

impl GrepTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for GrepTool {
    type Input = GrepToolInput;
    type Output = String;

    fn name() -> &'static str {
        "grep"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Search
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
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
            Err(_) => "Search with regex".into(),
        }
        .into()
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        const CONTEXT_LINES: u32 = 2;
        const MAX_ANCESTOR_LINES: u32 = 10;

        let include_matcher = match PathMatcher::new(
            input
                .include_pattern
                .as_ref()
                .into_iter()
                .collect::<Vec<_>>(),
        ) {
            Ok(matcher) => matcher,
            Err(error) => {
                return Task::ready(Err(anyhow!("invalid include glob pattern: {error}")));
            }
        };

        // Exclude global file_scan_exclusions and private_files settings
        let exclude_matcher = {
            let global_settings = WorktreeSettings::get_global(cx);
            let exclude_patterns = global_settings
                .file_scan_exclusions
                .sources()
                .iter()
                .chain(global_settings.private_files.sources().iter());

            match PathMatcher::new(exclude_patterns) {
                Ok(matcher) => matcher,
                Err(error) => {
                    return Task::ready(Err(anyhow!("invalid exclude pattern: {error}")));
                }
            }
        };

        let query = match SearchQuery::regex(
            &input.regex,
            false,
            input.case_sensitive,
            false,
            false,
            include_matcher,
            exclude_matcher,
            true, // Always match file include pattern against *full project paths* that start with a project root.
            None,
        ) {
            Ok(query) => query,
            Err(error) => return Task::ready(Err(error)),
        };

        let results = self
            .project
            .update(cx, |project, cx| project.search(query, cx));

        let project = self.project.downgrade();
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

                let Ok((Some(path), mut parse_status)) = buffer.read_with(cx, |buffer, cx| {
                    (buffer.file().map(|file| file.full_path(cx)), buffer.parse_status())
                }) else {
                    continue;
                };

                // Check if this file should be excluded based on its worktree settings
                if let Ok(Some(project_path)) = project.read_with(cx, |project, cx| {
                    project.find_project_path(&path, cx)
                })
                    && cx.update(|cx| {
                        let worktree_settings = WorktreeSettings::get(Some((&project_path).into()), cx);
                        worktree_settings.is_path_excluded(&project_path.path)
                            || worktree_settings.is_path_private(&project_path.path)
                    }).unwrap_or(false) {
                        continue;
                    }

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

                    for symbol in parent_symbols {
                        write!(output, "{} › ", symbol.text)?;
                    }

                    if range.start.row == end_row {
                        writeln!(output, "L{}", range.start.row + 1)?;
                    } else {
                        writeln!(output, "L{}-{}", range.start.row + 1, end_row + 1)?;
                    }

                    output.push_str("```\n");
                    output.extend(snapshot.text_for_range(range));
                    output.push_str("\n```\n");

                    if let Some(ancestor_range) = ancestor_range
                        && end_row < ancestor_range.end.row {
                            let remaining_lines = ancestor_range.end.row - end_row;
                            writeln!(output, "\n{} lines remaining in ancestor node. Read the file to see all.", remaining_lines)?;
                        }

                    matches_found += 1;
                }
            }

            if matches_found == 0 {
                Ok("No matches found".into())
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
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ToolCallEventStream;

    use super::*;
    use gpui::{TestAppContext, UpdateGlobal};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use project::{FakeFs, Project, WorktreeSettings};
    use serde_json::json;
    use settings::SettingsStore;
    use unindent::Unindent;
    use util::path;

    #[gpui::test]
    async fn test_grep_tool_with_include_pattern(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
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
        let input = GrepToolInput {
            regex: "println".to_string(),
            include_pattern: Some("root/**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        };

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
        let input = GrepToolInput {
            regex: "fn".to_string(),
            include_pattern: Some("root/**/src/**".to_string()),
            offset: 0,
            case_sensitive: false,
        };

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
        let input = GrepToolInput {
            regex: "fn".to_string(),
            include_pattern: None,
            offset: 0,
            case_sensitive: false,
        };

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

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            serde_json::json!({
                "case_test.txt": "This file has UPPERCASE and lowercase text.\nUPPERCASE patterns should match only with case_sensitive: true",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        // Test case-insensitive search (default)
        let input = GrepToolInput {
            regex: "uppercase".to_string(),
            include_pattern: Some("**/*.txt".to_string()),
            offset: 0,
            case_sensitive: false,
        };

        let result = run_grep_tool(input, project.clone(), cx).await;
        assert!(
            result.contains("UPPERCASE"),
            "Case-insensitive search should match uppercase"
        );

        // Test case-sensitive search
        let input = GrepToolInput {
            regex: "uppercase".to_string(),
            include_pattern: Some("**/*.txt".to_string()),
            offset: 0,
            case_sensitive: true,
        };

        let result = run_grep_tool(input, project.clone(), cx).await;
        assert!(
            !result.contains("UPPERCASE"),
            "Case-sensitive search should not match uppercase"
        );

        // Test case-sensitive search
        let input = GrepToolInput {
            regex: "LOWERCASE".to_string(),
            include_pattern: Some("**/*.txt".to_string()),
            offset: 0,
            case_sensitive: true,
        };

        let result = run_grep_tool(input, project.clone(), cx).await;

        assert!(
            !result.contains("lowercase"),
            "Case-sensitive search should match lowercase"
        );

        // Test case-sensitive search for lowercase pattern
        let input = GrepToolInput {
            regex: "lowercase".to_string(),
            include_pattern: Some("**/*.txt".to_string()),
            offset: 0,
            case_sensitive: true,
        };

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

        let fs = FakeFs::new(cx.executor());

        // Create test file with syntax structures
        fs.insert_tree(
            path!("/root"),
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
        let input = GrepToolInput {
            regex: "This is at the top level".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        };

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
        let input = GrepToolInput {
            regex: "Function in nested module".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        };

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
        let input = GrepToolInput {
            regex: "second_arg".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        };

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
        let input = GrepToolInput {
            regex: "Inside if block".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        };

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
        let input = GrepToolInput {
            regex: "Line 5".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        };

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
        let input = GrepToolInput {
            regex: "Line 12".to_string(),
            include_pattern: Some("**/*.rs".to_string()),
            offset: 0,
            case_sensitive: false,
        };

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
        input: GrepToolInput,
        project: Entity<Project>,
        cx: &mut TestAppContext,
    ) -> String {
        let tool = Arc::new(GrepTool { project });
        let task = cx.update(|cx| tool.run(input, ToolCallEventStream::test().0, cx));

        match task.await {
            Ok(result) => {
                if cfg!(windows) {
                    result.replace("root\\", "root/")
                } else {
                    result
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
        .with_outline_query(include_str!("../../../languages/src/rust/outline.scm"))
        .unwrap()
    }

    #[gpui::test]
    async fn test_grep_security_boundaries(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            path!("/"),
            json!({
                "project_root": {
                    "allowed_file.rs": "fn main() { println!(\"This file is in the project\"); }",
                    ".mysecrets": "SECRET_KEY=abc123\nfn secret() { /* private */ }",
                    ".secretdir": {
                        "config": "fn special_configuration() { /* excluded */ }"
                    },
                    ".mymetadata": "fn custom_metadata() { /* excluded */ }",
                    "subdir": {
                        "normal_file.rs": "fn normal_file_content() { /* Normal */ }",
                        "special.privatekey": "fn private_key_content() { /* private */ }",
                        "data.mysensitive": "fn sensitive_data() { /* private */ }"
                    }
                },
                "outside_project": {
                    "sensitive_file.rs": "fn outside_function() { /* This file is outside the project */ }"
                }
            }),
        )
        .await;

        cx.update(|cx| {
            use gpui::UpdateGlobal;
            use project::WorktreeSettings;
            use settings::SettingsStore;
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<WorktreeSettings>(cx, |settings| {
                    settings.file_scan_exclusions = Some(vec![
                        "**/.secretdir".to_string(),
                        "**/.mymetadata".to_string(),
                    ]);
                    settings.private_files = Some(vec![
                        "**/.mysecrets".to_string(),
                        "**/*.privatekey".to_string(),
                        "**/*.mysensitive".to_string(),
                    ]);
                });
            });
        });

        let project = Project::test(fs.clone(), [path!("/project_root").as_ref()], cx).await;

        // Searching for files outside the project worktree should return no results
        let result = run_grep_tool(
            GrepToolInput {
                regex: "outside_function".to_string(),
                include_pattern: None,
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);
        assert!(
            paths.is_empty(),
            "grep_tool should not find files outside the project worktree"
        );

        // Searching within the project should succeed
        let result = run_grep_tool(
            GrepToolInput {
                regex: "main".to_string(),
                include_pattern: None,
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);
        assert!(
            paths.iter().any(|p| p.contains("allowed_file.rs")),
            "grep_tool should be able to search files inside worktrees"
        );

        // Searching files that match file_scan_exclusions should return no results
        let result = run_grep_tool(
            GrepToolInput {
                regex: "special_configuration".to_string(),
                include_pattern: None,
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);
        assert!(
            paths.is_empty(),
            "grep_tool should not search files in .secretdir (file_scan_exclusions)"
        );

        let result = run_grep_tool(
            GrepToolInput {
                regex: "custom_metadata".to_string(),
                include_pattern: None,
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);
        assert!(
            paths.is_empty(),
            "grep_tool should not search .mymetadata files (file_scan_exclusions)"
        );

        // Searching private files should return no results
        let result = run_grep_tool(
            GrepToolInput {
                regex: "SECRET_KEY".to_string(),
                include_pattern: None,
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);
        assert!(
            paths.is_empty(),
            "grep_tool should not search .mysecrets (private_files)"
        );

        let result = run_grep_tool(
            GrepToolInput {
                regex: "private_key_content".to_string(),
                include_pattern: None,
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);

        assert!(
            paths.is_empty(),
            "grep_tool should not search .privatekey files (private_files)"
        );

        let result = run_grep_tool(
            GrepToolInput {
                regex: "sensitive_data".to_string(),
                include_pattern: None,
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);
        assert!(
            paths.is_empty(),
            "grep_tool should not search .mysensitive files (private_files)"
        );

        // Searching a normal file should still work, even with private_files configured
        let result = run_grep_tool(
            GrepToolInput {
                regex: "normal_file_content".to_string(),
                include_pattern: None,
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);
        assert!(
            paths.iter().any(|p| p.contains("normal_file.rs")),
            "Should be able to search normal files"
        );

        // Path traversal attempts with .. in include_pattern should not escape project
        let result = run_grep_tool(
            GrepToolInput {
                regex: "outside_function".to_string(),
                include_pattern: Some("../outside_project/**/*.rs".to_string()),
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);
        assert!(
            paths.is_empty(),
            "grep_tool should not allow escaping project boundaries with relative paths"
        );
    }

    #[gpui::test]
    async fn test_grep_with_multiple_worktree_settings(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        // Create first worktree with its own private files
        fs.insert_tree(
            path!("/worktree1"),
            json!({
                ".zed": {
                    "settings.json": r#"{
                        "file_scan_exclusions": ["**/fixture.*"],
                        "private_files": ["**/secret.rs"]
                    }"#
                },
                "src": {
                    "main.rs": "fn main() { let secret_key = \"hidden\"; }",
                    "secret.rs": "const API_KEY: &str = \"secret_value\";",
                    "utils.rs": "pub fn get_config() -> String { \"config\".to_string() }"
                },
                "tests": {
                    "test.rs": "fn test_secret() { assert!(true); }",
                    "fixture.sql": "SELECT * FROM secret_table;"
                }
            }),
        )
        .await;

        // Create second worktree with different private files
        fs.insert_tree(
            path!("/worktree2"),
            json!({
                ".zed": {
                    "settings.json": r#"{
                        "file_scan_exclusions": ["**/internal.*"],
                        "private_files": ["**/private.js", "**/data.json"]
                    }"#
                },
                "lib": {
                    "public.js": "export function getSecret() { return 'public'; }",
                    "private.js": "const SECRET_KEY = \"private_value\";",
                    "data.json": "{\"secret_data\": \"hidden\"}"
                },
                "docs": {
                    "README.md": "# Documentation with secret info",
                    "internal.md": "Internal secret documentation"
                }
            }),
        )
        .await;

        // Set global settings
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<WorktreeSettings>(cx, |settings| {
                    settings.file_scan_exclusions =
                        Some(vec!["**/.git".to_string(), "**/node_modules".to_string()]);
                    settings.private_files = Some(vec!["**/.env".to_string()]);
                });
            });
        });

        let project = Project::test(
            fs.clone(),
            [path!("/worktree1").as_ref(), path!("/worktree2").as_ref()],
            cx,
        )
        .await;

        // Wait for worktrees to be fully scanned
        cx.executor().run_until_parked();

        // Search for "secret" - should exclude files based on worktree-specific settings
        let result = run_grep_tool(
            GrepToolInput {
                regex: "secret".to_string(),
                include_pattern: None,
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;
        let paths = extract_paths_from_results(&result);

        // Should find matches in non-private files
        assert!(
            paths.iter().any(|p| p.contains("main.rs")),
            "Should find 'secret' in worktree1/src/main.rs"
        );
        assert!(
            paths.iter().any(|p| p.contains("test.rs")),
            "Should find 'secret' in worktree1/tests/test.rs"
        );
        assert!(
            paths.iter().any(|p| p.contains("public.js")),
            "Should find 'secret' in worktree2/lib/public.js"
        );
        assert!(
            paths.iter().any(|p| p.contains("README.md")),
            "Should find 'secret' in worktree2/docs/README.md"
        );

        // Should NOT find matches in private/excluded files based on worktree settings
        assert!(
            !paths.iter().any(|p| p.contains("secret.rs")),
            "Should not search in worktree1/src/secret.rs (local private_files)"
        );
        assert!(
            !paths.iter().any(|p| p.contains("fixture.sql")),
            "Should not search in worktree1/tests/fixture.sql (local file_scan_exclusions)"
        );
        assert!(
            !paths.iter().any(|p| p.contains("private.js")),
            "Should not search in worktree2/lib/private.js (local private_files)"
        );
        assert!(
            !paths.iter().any(|p| p.contains("data.json")),
            "Should not search in worktree2/lib/data.json (local private_files)"
        );
        assert!(
            !paths.iter().any(|p| p.contains("internal.md")),
            "Should not search in worktree2/docs/internal.md (local file_scan_exclusions)"
        );

        // Test with `include_pattern` specific to one worktree
        let result = run_grep_tool(
            GrepToolInput {
                regex: "secret".to_string(),
                include_pattern: Some("worktree1/**/*.rs".to_string()),
                offset: 0,
                case_sensitive: false,
            },
            project.clone(),
            cx,
        )
        .await;

        let paths = extract_paths_from_results(&result);

        // Should only find matches in worktree1 *.rs files (excluding private ones)
        assert!(
            paths.iter().any(|p| p.contains("main.rs")),
            "Should find match in worktree1/src/main.rs"
        );
        assert!(
            paths.iter().any(|p| p.contains("test.rs")),
            "Should find match in worktree1/tests/test.rs"
        );
        assert!(
            !paths.iter().any(|p| p.contains("secret.rs")),
            "Should not find match in excluded worktree1/src/secret.rs"
        );
        assert!(
            paths.iter().all(|p| !p.contains("worktree2")),
            "Should not find any matches in worktree2"
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
