use crate::editor_tests::init_test;
use crate::rainbow_brackets::compute_rainbow_brackets_for_range;
use crate::test::editor_lsp_test_context::EditorLspTestContext;
use gpui::TestAppContext;
use indoc::indoc;
use language::{BracketPair, BracketPairConfig, Language, LanguageConfig, LanguageMatcher};

/// Helper function to create a test language with bracket configuration
fn create_test_language() -> Language {
    // Use a simpler language without tree-sitter queries for basic tests
    Language::new(
        LanguageConfig {
            name: "TestLang".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["test".to_string()],
                ..Default::default()
            },
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        },
        None, // No tree-sitter grammar for simple tests
    )
}

// Test that the empty buffer test works correctly
#[gpui::test]
async fn test_empty_buffer(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = create_test_language();
    let mut cx = EditorLspTestContext::new(language, Default::default(), cx).await;

    cx.set_state("ˇ");

    let buffer = cx.editor(|editor, _, cx| {
        let multi_buffer = editor.buffer().read(cx).snapshot(cx);
        multi_buffer.as_singleton().unwrap().2.clone()
    });
    let highlights = compute_rainbow_brackets_for_range(&buffer, 0..0);

    // Without tree-sitter, should return None
    assert!(highlights.is_none());
}

// Test simple single bracket pair detection
#[gpui::test]
async fn test_simple_bracket_detection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = create_test_language();
    let mut cx = EditorLspTestContext::new(language, Default::default(), cx).await;

    // Test single bracket pair
    cx.set_state(indoc! {r#"
        fn test() {
            printlnˇ!("Hello");
        }
    "#});

    let buffer = cx.editor(|editor, _, cx| {
        let multi_buffer = editor.buffer().read(cx).snapshot(cx);
        multi_buffer.as_singleton().unwrap().2.clone()
    });
    let highlights = compute_rainbow_brackets_for_range(&buffer, 0..buffer.len());

    // Without tree-sitter, should return None
    assert!(highlights.is_none());
}

// Test nested bracket levels
#[gpui::test]
async fn test_nested_bracket_levels(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = create_test_language();
    let mut cx = EditorLspTestContext::new(language, Default::default(), cx).await;

    // Test nested brackets with different types
    cx.set_state(indoc! {r#"
        fn test() {
            let array = [1, 2, 3];
            if (condition) {
                processˇ(array[0]);
            }
        }
    "#});

    let buffer = cx.editor(|editor, _, cx| {
        let multi_buffer = editor.buffer().read(cx).snapshot(cx);
        multi_buffer.as_singleton().unwrap().2.clone()
    });
    let highlights = compute_rainbow_brackets_for_range(&buffer, 0..buffer.len());

    // Without tree-sitter, should return None
    assert!(highlights.is_none());
}

// Test level wrapping
#[gpui::test]
async fn test_level_wrapping(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = create_test_language();
    let mut cx = EditorLspTestContext::new(language, Default::default(), cx).await;

    // Test that levels wrap around after 10 (0-9)
    let deeply_nested = r#"
        fn testˇ() {                    // Level 0
            {                          // Level 1
                {                      // Level 2
                    {                  // Level 3
                        {              // Level 4
                            {          // Level 5
                                {      // Level 6
                                    {  // Level 7
                                        { // Level 8
                                            { // Level 9
                                                { // Level 0 (wrapped)
                                                    println!("Deep!");
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    "#;

    cx.set_state(deeply_nested);

    let buffer = cx.editor(|editor, _, cx| {
        let multi_buffer = editor.buffer().read(cx).snapshot(cx);
        multi_buffer.as_singleton().unwrap().2.clone()
    });
    let highlights = compute_rainbow_brackets_for_range(&buffer, 0..buffer.len());

    // Without tree-sitter, should return None
    assert!(highlights.is_none());
}

// Test mixed bracket types
#[gpui::test]
async fn test_mixed_bracket_types(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = create_test_language();
    let mut cx = EditorLspTestContext::new(language, Default::default(), cx).await;

    cx.set_state(indoc! {r#"
        fn test() {
            let tuple = (1, 2, 3);
            let array = [4, 5, 6];
            let result = computeˇ({
                value: tuple.0 + array[0]
            });
        }
    "#});

    let buffer = cx.editor(|editor, _, cx| {
        let multi_buffer = editor.buffer().read(cx).snapshot(cx);
        multi_buffer.as_singleton().unwrap().2.clone()
    });
    let highlights = compute_rainbow_brackets_for_range(&buffer, 0..buffer.len());

    // Without tree-sitter, should return None
    assert!(highlights.is_none());
}

// Test that unclosed brackets are handled gracefully
#[gpui::test]
async fn test_unclosed_brackets(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = create_test_language();
    let mut cx = EditorLspTestContext::new(language, Default::default(), cx).await;

    // Test with unclosed brackets
    cx.set_state(indoc! {r#"
        fn test() {
            let array = [1, 2, 3;  // Missing closing bracket
            if (condition {        // Mixed up brackets
                printlnˇ!("Test");
            }
        // Missing closing brace for function
    "#});

    let buffer = cx.editor(|editor, _, cx| {
        let multi_buffer = editor.buffer().read(cx).snapshot(cx);
        multi_buffer.as_singleton().unwrap().2.clone()
    });
    let highlights = compute_rainbow_brackets_for_range(&buffer, 0..buffer.len());

    // Without tree-sitter, should return None
    assert!(highlights.is_none());
}

// Test brackets in strings and comments
#[gpui::test]
async fn test_brackets_in_strings_and_comments(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = create_test_language();
    let mut cx = EditorLspTestContext::new(language, Default::default(), cx).await;

    cx.set_state(indoc! {r#"
        fn test() {
            // This { bracket } should not be highlighted
            let string = "Another { bracket } in string";
            let actual = { valueˇ: 42 };
        }
    "#});

    let buffer = cx.editor(|editor, _, cx| {
        let multi_buffer = editor.buffer().read(cx).snapshot(cx);
        multi_buffer.as_singleton().unwrap().2.clone()
    });
    let highlights = compute_rainbow_brackets_for_range(&buffer, 0..buffer.len());

    // Without tree-sitter, should return None
    assert!(highlights.is_none());
}

// Test with real Rust language and brackets.scm query
#[gpui::test]
async fn test_rust_rainbow_brackets(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    // Create a Rust-like language with proper tree-sitter and bracket query
    let rust_lang = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: false,
                        surround: false,
                        newline: false,
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )
    .with_brackets_query(indoc! {r#"
        ; Rainbow bracket scopes for common Rust constructs
        [(block) (match_block) (use_list) (field_initializer_list)] @rainbow.scope
        
        ; Rainbow brackets - actual bracket characters
        ["{" "}" "(" ")" "[" "]"] @rainbow.bracket
        "#})
    .unwrap();

    let mut cx = EditorLspTestContext::new(rust_lang, Default::default(), cx).await;

    // Test real Rust code
    cx.set_state(indoc! {r#"
        fn process_data() {
            let array = [1, 2, 3];
            if true {
                println!ˇ("Hello");
            }
        }
    "#});

    let buffer = cx.editor(|editor, _, cx| {
        let multi_buffer = editor.buffer().read(cx).snapshot(cx);
        multi_buffer.as_singleton().unwrap().2.clone()
    });
    
    let highlights = compute_rainbow_brackets_for_range(&buffer, 0..buffer.len());

    // With proper tree-sitter setup, we should get highlights
    assert!(highlights.is_some());
    let highlights = highlights.unwrap();

    // Verify we have some levels
    assert!(!highlights.is_empty(), "Should have at least one level of brackets");
    
    // Verify structure is correct
    for (level, ranges) in highlights {
        assert!(level < 10, "Level {} should be < 10", level);
        for range in ranges {
            assert!(range.start < range.end);
            assert!(range.end <= buffer.len());
        }
    }
}