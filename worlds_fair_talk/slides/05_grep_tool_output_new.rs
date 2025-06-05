#[gpui::test]
async fn test_grep_function_args_and_body(cx: &mut TestAppContext) {
    let project = setup_syntax_test(cx).await;

    // Test: Line with a function argument
    let input = serde_json::to_value(GrepToolInput {
        regex: "fn run(".to_string(),
        include_pattern: Some("**/*.rs".to_string()),
        offset: 0,
        case_sensitive: false,
    })
    .unwrap();

    let result = run_grep_tool(input, project.clone(), cx).await;
    let expected = r#"
        Found 1 matches:

        ## Matches in crates/assistant_tool/src/assistant_tool.rs

        ### trait AssistantTool › fn run › L238-241

        ```rs
        /// Runs the tool with the provided input.
        fn run(
            self: Arc<Self>,
            input: serde_json::Value,
            request: Arc<LanguageModelRequest>,
            project: Entity<Project>,
            action_log: Entity<ActionLog>,
            model: Arc<dyn LanguageModel>,
            window: Option<AnyWindowHandle>,
            cx: &mut App,
        ) -> ToolResult;
        ```
        "#
    .unindent();
    assert_eq!(result, expected);
}
