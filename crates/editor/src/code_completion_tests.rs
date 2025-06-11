use crate::{code_context_menus::CompletionsMenu, editor_settings::SnippetSortOrder};
use fuzzy::StringMatchCandidate;
use gpui::TestAppContext;
use language::CodeLabel;
use lsp::{CompletionItem, CompletionItemKind, LanguageServerId};
use project::{Completion, CompletionSource};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use text::Anchor;

struct CompletionBuilder;

impl CompletionBuilder {
    fn constant(label: &str, sort_text: &str) -> Completion {
        Self::new(label, sort_text, CompletionItemKind::CONSTANT)
    }

    fn variable(label: &str, sort_text: &str) -> Completion {
        Self::new(label, sort_text, CompletionItemKind::VARIABLE)
    }

    fn keyword(label: &str, sort_text: &str) -> Completion {
        Self::new(label, sort_text, CompletionItemKind::KEYWORD)
    }

    fn snippet(label: &str, sort_text: &str) -> Completion {
        Self::new(label, sort_text, CompletionItemKind::SNIPPET)
    }

    fn new(label: &str, sort_text: &str, kind: CompletionItemKind) -> Completion {
        Completion {
            replace_range: Anchor::MIN..Anchor::MAX,
            new_text: label.to_string(),
            label: CodeLabel {
                text: label.to_string(),
                runs: Default::default(),
                filter_range: 0..label.len(),
            },
            documentation: None,
            source: CompletionSource::Lsp {
                insert_range: None,
                server_id: LanguageServerId(0),
                lsp_completion: Box::new(CompletionItem {
                    label: label.to_string(),
                    kind: Some(kind),
                    sort_text: Some(sort_text.to_string()),
                    ..Default::default()
                }),
                lsp_defaults: None,
                resolved: false,
            },
            icon_path: None,
            insert_text_mode: None,
            confirm: None,
        }
    }
}

async fn sort_matches(
    query: &str,
    completions: Vec<Completion>,
    snippet_sort_order: SnippetSortOrder,
    cx: &mut TestAppContext,
) -> Vec<String> {
    let candidates: Arc<[StringMatchCandidate]> = completions
        .iter()
        .enumerate()
        .map(|(id, completion)| StringMatchCandidate::new(id, &completion.label.text))
        .collect();
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let background_executor = cx.executor();
    let matches = fuzzy::match_strings(
        &candidates,
        query,
        query.chars().any(|c| c.is_uppercase()),
        100,
        &cancel_flag,
        background_executor,
    )
    .await;
    let sorted_matches = CompletionsMenu::sort_string_matches(
        matches,
        Some(query),
        snippet_sort_order,
        &completions,
    );
    sorted_matches.into_iter().map(|m| m.string).collect()
}

#[gpui::test]
async fn test_sort_matches_local_variable_over_global_variable(cx: &mut TestAppContext) {
    // Case 1: "foo"
    let completions = vec![
        CompletionBuilder::constant("foo_bar_baz", "7fffffff"),
        CompletionBuilder::variable("foo_bar_qux", "7ffffffe"),
        CompletionBuilder::constant("floorf64", "80000000"),
        CompletionBuilder::constant("floorf32", "80000000"),
        CompletionBuilder::constant("floorf16", "80000000"),
        CompletionBuilder::constant("floorf128", "80000000"),
    ];
    let matches = sort_matches("foo", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "foo_bar_qux");
    assert_eq!(matches[1], "foo_bar_baz");
    assert_eq!(matches[2], "floorf16");
    assert_eq!(matches[3], "floorf32");

    // Case 2: "foobar"
    let completions = vec![
        CompletionBuilder::constant("foo_bar_baz", "7fffffff"),
        CompletionBuilder::variable("foo_bar_qux", "7ffffffe"),
    ];
    let matches = sort_matches("foobar", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "foo_bar_qux");
    assert_eq!(matches[1], "foo_bar_baz");
}

#[gpui::test]
async fn test_sort_matches_local_variable_over_global_enum(cx: &mut TestAppContext) {
    // Case 1: "ele"
    let completions = vec![
        CompletionBuilder::constant("ElementType", "7fffffff"),
        CompletionBuilder::variable("element_type", "7ffffffe"),
        CompletionBuilder::constant("simd_select", "80000000"),
        CompletionBuilder::keyword("while let", "7fffffff"),
    ];
    let matches = sort_matches("ele", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "element_type");
    assert_eq!(matches[1], "ElementType");

    // Case 2: "eleme"
    let completions = vec![
        CompletionBuilder::constant("ElementType", "7fffffff"),
        CompletionBuilder::variable("element_type", "7ffffffe"),
        CompletionBuilder::constant("REPLACEMENT_CHARACTER", "80000000"),
    ];
    let matches = sort_matches("eleme", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "element_type");
    assert_eq!(matches[1], "ElementType");

    // Case 3: "Elem"
    let completions = vec![
        CompletionBuilder::constant("ElementType", "7fffffff"),
        CompletionBuilder::variable("element_type", "7ffffffe"),
    ];
    let matches = sort_matches("Elem", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "ElementType");
    assert_eq!(matches[1], "element_type");
}

#[gpui::test]
async fn test_sort_matches_for_unreachable(cx: &mut TestAppContext) {
    // Case 1: "unre"
    let completions = vec![
        CompletionBuilder::constant("unreachable", "80000000"),
        CompletionBuilder::snippet("unreachable!(…)", "7fffffff"),
        CompletionBuilder::constant("unchecked_rem", "80000000"),
        CompletionBuilder::constant("unreachable_unchecked", "80000000"),
    ];
    let matches = sort_matches("unre", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "unreachable!(…)");

    // Case 2: "unrea"
    let completions = vec![
        CompletionBuilder::snippet("unreachable", "80000000"),
        CompletionBuilder::snippet("unreachable!(…)", "7fffffff"),
        CompletionBuilder::snippet("unreachable_unchecked", "80000000"),
    ];
    let matches = sort_matches("unrea", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "unreachable!(…)");

    // Case 3: "unreach"
    let completions = vec![
        CompletionBuilder::constant("unreachable", "80000000"),
        CompletionBuilder::snippet("unreachable!(…)", "7fffffff"),
        CompletionBuilder::constant("unreachable_unchecked", "80000000"),
    ];
    let matches = sort_matches("unreach", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "unreachable!(…)");

    // Case 4: "unreachabl"
    let completions = vec![
        CompletionBuilder::snippet("unreachable", "80000000"),
        CompletionBuilder::constant("unreachable!(…)", "7fffffff"),
        CompletionBuilder::constant("unreachable_unchecked", "80000000"),
    ];
    let matches = sort_matches("unreachable", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "unreachable!(…)");

    // Case 5: "unreachable"
    let completions = vec![
        CompletionBuilder::constant("unreachable", "80000000"),
        CompletionBuilder::constant("unreachable!(…)", "7fffffff"),
        CompletionBuilder::constant("unreachable_unchecked", "80000000"),
    ];
    let matches = sort_matches("unreachable", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "unreachable!(…)");
}

#[gpui::test]
async fn test_sort_matches_variable_and_constants_over_function(cx: &mut TestAppContext) {
    // Case 1: "var" as variable
    let completions = vec![
        CompletionBuilder::new("var", "7fffffff", CompletionItemKind::FUNCTION),
        CompletionBuilder::variable("var", "7fffffff"),
    ];
    let matches = sort_matches("var", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "var");
    assert_eq!(matches[1], "var");

    // Case 2: "var" as constant
    let completions = vec![
        CompletionBuilder::new("var", "7fffffff", CompletionItemKind::FUNCTION),
        CompletionBuilder::constant("var", "7fffffff"),
    ];
    let matches = sort_matches("var", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "var");
    assert_eq!(matches[1], "var");
}

#[gpui::test]
async fn test_sort_matches_for_jsx_event_handler(cx: &mut TestAppContext) {
    // Case 1: "on"
    let completions = vec![
        CompletionBuilder::new("onCut?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onPlay?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("color?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("defaultValue?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("style?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("className?", "12", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("on", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "onCut?");
    assert_eq!(matches[1], "onPlay?");

    // Case 2: "ona"
    let completions = vec![
        CompletionBuilder::new("onAbort?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onAuxClick?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onPlay?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onLoad?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onDrag?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onPause?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onPaste?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onAnimationEnd?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onAbortCapture?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onChange?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onWaiting?", "12", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("onCanPlay?", "12", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("ona", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "onAbort?");
    assert_eq!(matches[1], "onAuxClick?");
}

#[gpui::test]
async fn test_sort_matches_for_snippets(cx: &mut TestAppContext) {
    // Case 1: "prin"
    let completions = vec![
        CompletionBuilder::constant("println", "80000000"),
        CompletionBuilder::snippet("println!(…)", "80000000"),
    ];
    let matches = sort_matches("prin", completions, SnippetSortOrder::Top, cx).await;
    assert_eq!(matches[0], "println!(…)");
}

#[gpui::test]
async fn test_sort_matches_for_exact_match(cx: &mut TestAppContext) {
    // Case 1: "set_text"
    let completions = vec![
        CompletionBuilder::new("set_text", "7fffffff", CompletionItemKind::FUNCTION),
        CompletionBuilder::new(
            "set_placeholder_text",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "set_text_style_refinement",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "set_context_menu_options",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "select_to_next_word_end",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "select_to_next_subword_end",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "set_custom_context_menu",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "select_to_end_of_excerpt",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "select_to_start_of_excerpt",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "select_to_start_of_next_excerpt",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "select_to_end_of_previous_excerpt",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
    ];
    let matches = sort_matches("set_text", completions, SnippetSortOrder::Top, cx).await;
    assert_eq!(matches[0], "set_text");
    assert_eq!(matches[1], "set_text_style_refinement");
    assert_eq!(matches[2], "set_placeholder_text");
}

#[gpui::test]
async fn test_sort_matches_for_prefix_matches(cx: &mut TestAppContext) {
    // Case 1: "set"
    let completions = vec![
        CompletionBuilder::new(
            "select_to_beginning",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "set_collapse_matches",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new("set_autoindent", "7fffffff", CompletionItemKind::FUNCTION),
        CompletionBuilder::new(
            "set_all_diagnostics_active",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "select_to_end_of_line",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new("select_all", "7fffffff", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("select_line", "7fffffff", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("select_left", "7fffffff", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("select_down", "7fffffff", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("set", completions, SnippetSortOrder::Top, cx).await;
    assert_eq!(matches[0], "set_autoindent");
    assert_eq!(matches[1], "set_collapse_matches");
    assert_eq!(matches[2], "set_all_diagnostics_active");
}

#[gpui::test]
async fn test_sort_matches_for_await(cx: &mut TestAppContext) {
    // Case 1: "awa"
    let completions = vec![
        CompletionBuilder::keyword("await", "7fffffff"),
        CompletionBuilder::new("await.ne", "80000010", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.eq", "80000010", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.or", "7ffffff8", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.zip", "80000006", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.xor", "7ffffff8", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.and", "80000006", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.map", "80000006", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.take", "7ffffff8", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("awa", completions, SnippetSortOrder::Top, cx).await;
    assert_eq!(matches[0], "await");

    // Case 2: "await"
    let completions = vec![
        CompletionBuilder::keyword("await", "7fffffff"),
        CompletionBuilder::new("await.ne", "80000010", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.eq", "80000010", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.or", "7ffffff8", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.zip", "80000006", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.xor", "7ffffff8", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.and", "80000006", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.map", "80000006", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("await.take", "7ffffff8", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("await", completions, SnippetSortOrder::Top, cx).await;
    assert_eq!(matches[0], "await");
}

#[gpui::test]
async fn test_sort_matches_for_python_init(cx: &mut TestAppContext) {
    // Case 1: "__in"
    let completions = vec![
        CompletionBuilder::new("__init__", "05.0003.__init__", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("__init__", "05.0003", CompletionItemKind::FUNCTION),
        CompletionBuilder::new(
            "__instancecheck__",
            "05.0005.__instancecheck__",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new(
            "__init_subclass__",
            "05.0004.__init_subclass__",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new("__instancecheck__", "05.0005", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("__init_subclass__", "05.0004", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("__in", completions, SnippetSortOrder::Top, cx).await;
    assert_eq!(matches[0], "__init__");
    assert_eq!(matches[1], "__init__");

    // Case 2: "__ini"
    let completions = vec![
        CompletionBuilder::new("__init__", "05.0004.__init__", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("__init__", "05.0004", CompletionItemKind::FUNCTION),
        CompletionBuilder::new(
            "__init_subclass__",
            "05.0003.__init_subclass__",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new("__init_subclass__", "05.0003", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("__ini", completions, SnippetSortOrder::Top, cx).await;
    assert_eq!(matches[0], "__init__");
    assert_eq!(matches[1], "__init__");

    // Case 3: "__init"
    let completions = vec![
        CompletionBuilder::new("__init__", "05.0000.__init__", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("__init__", "05.0000", CompletionItemKind::FUNCTION),
        CompletionBuilder::new(
            "__init_subclass__",
            "05.0001.__init_subclass__",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new("__init_subclass__", "05.0001", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("__init", completions, SnippetSortOrder::Top, cx).await;
    assert_eq!(matches[0], "__init__");
    assert_eq!(matches[1], "__init__");

    // Case 4: "__init_"
    let completions = vec![
        CompletionBuilder::new("__init__", "11.9999.__init__", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("__init__", "11.9999", CompletionItemKind::FUNCTION),
        CompletionBuilder::new(
            "__init_subclass__",
            "05.0000.__init_subclass__",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new("__init_subclass__", "05.0000", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("__init_", completions, SnippetSortOrder::Top, cx).await;
    assert_eq!(matches[0], "__init__");
    assert_eq!(matches[1], "__init__");
}

#[gpui::test]
async fn test_sort_matches_for_rust_into(cx: &mut TestAppContext) {
    // Case 1: "int"
    let completions = vec![
        CompletionBuilder::new("into", "80000004", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("try_into", "80000004", CompletionItemKind::FUNCTION),
        CompletionBuilder::snippet("println", "80000004"),
        CompletionBuilder::new("clone_into", "80000004", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("into_searcher", "80000000", CompletionItemKind::FUNCTION),
        CompletionBuilder::snippet("eprintln", "80000004"),
    ];
    let matches = sort_matches("int", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "into");

    // Case 2: "into"
    let completions = vec![
        CompletionBuilder::new("into", "80000004", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("try_into", "80000004", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("clone_into", "80000004", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("into_searcher", "80000000", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("split_terminator", "7fffffff", CompletionItemKind::FUNCTION),
        CompletionBuilder::new(
            "rsplit_terminator",
            "7fffffff",
            CompletionItemKind::FUNCTION,
        ),
    ];
    let matches = sort_matches("into", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "into");
}

#[gpui::test]
async fn test_sort_matches_for_variable_over_function(cx: &mut TestAppContext) {
    // Case 1: "serial"
    let completions = vec![
        CompletionBuilder::new("serialize", "80000000", CompletionItemKind::FUNCTION),
        CompletionBuilder::new("serialize", "80000000", CompletionItemKind::FUNCTION),
        CompletionBuilder::variable("serialization_key", "7ffffffe"),
        CompletionBuilder::new(
            "serialize_version",
            "80000000",
            CompletionItemKind::FUNCTION,
        ),
        CompletionBuilder::new("deserialize", "80000000", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("serial", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "serialization_key");
    assert_eq!(matches[1], "serialize");
    assert_eq!(matches[2], "serialize");
    assert_eq!(matches[3], "serialize_version");
    assert_eq!(matches[4], "deserialize");
}

#[gpui::test]
async fn test_sort_matches_for_local_methods_over_library(cx: &mut TestAppContext) {
    // Case 1: "setis"
    let completions = vec![
        CompletionBuilder::variable("setISODay", "16"),
        CompletionBuilder::variable("setISOWeek", "16"),
        CompletionBuilder::variable("setISOWeekYear", "16"),
        CompletionBuilder::new("setISOWeekYear", "16", CompletionItemKind::FUNCTION),
        CompletionBuilder::variable("setIsRefreshing", "11"),
        CompletionBuilder::new("setFips", "16", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("setis", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "setIsRefreshing");
    assert_eq!(matches[1], "setISODay");
    assert_eq!(matches[2], "setISOWeek");
}

#[gpui::test]
async fn test_sort_matches_for_prioritize_not_exact_match(cx: &mut TestAppContext) {
    // Case 1: "item"
    let completions = vec![
        CompletionBuilder::new("Item", "16", CompletionItemKind::FUNCTION),
        CompletionBuilder::variable("Item", "16"),
        CompletionBuilder::variable("items", "11"),
        CompletionBuilder::new("ItemText", "16", CompletionItemKind::FUNCTION),
    ];
    let matches = sort_matches("item", completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0], "items");
    assert_eq!(matches[1], "Item");
    assert_eq!(matches[2], "Item");
    assert_eq!(matches[3], "ItemText");
}
