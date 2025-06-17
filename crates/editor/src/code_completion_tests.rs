use crate::{code_context_menus::CompletionsMenu, editor_settings::SnippetSortOrder};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::TestAppContext;
use language::CodeLabel;
use lsp::{CompletionItem, CompletionItemKind, LanguageServerId};
use project::{Completion, CompletionSource};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use text::Anchor;

#[gpui::test]
async fn test_sort_kind(cx: &mut TestAppContext) {
    let completions = vec![
        CompletionBuilder::function("floorf128", None, "80000000"),
        CompletionBuilder::constant("foo_bar_baz", None, "80000000"),
        CompletionBuilder::variable("foo_bar_qux", None, "80000000"),
    ];
    let matches =
        filter_and_sort_matches("foo", &completions, SnippetSortOrder::default(), cx).await;

    // variable takes precedance over constant
    // constant take precedence over function
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string.as_str())
            .collect::<Vec<_>>(),
        vec!["foo_bar_qux", "foo_bar_baz", "floorf128"]
    );

    // fuzzy score should match for first two items as query is common prefix
    assert_eq!(matches[0].score, matches[1].score);
}

#[gpui::test]
async fn test_fuzzy_for_first_character_case(cx: &mut TestAppContext) {
    let completions = vec![
        CompletionBuilder::variable("element_type", None, "7ffffffe"),
        CompletionBuilder::constant("ElementType", None, "7fffffff"),
    ];
    let matches =
        filter_and_sort_matches("Elem", &completions, SnippetSortOrder::default(), cx).await;

    // first character case takes precedance over sort_text and sort_kind
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string.as_str())
            .collect::<Vec<_>>(),
        vec!["ElementType", "element_type"]
    );

    // fuzzy score for matched case item should be higher
    assert!(matches[0].score > matches[1].score);
}

#[gpui::test]
async fn test_sort_text(cx: &mut TestAppContext) {
    let completions = vec![
        CompletionBuilder::function("unreachable", None, "80000000"),
        CompletionBuilder::function("unreachable!(…)", None, "7fffffff"),
        CompletionBuilder::function("unchecked_rem", None, "80000000"),
        CompletionBuilder::function("unreachable_unchecked", None, "80000000"),
    ];

    test_for_each_prefix("unreachable", &completions, cx, |matches| {
        // for each prefix, first item should always be one with lower sort_text
        assert_eq!(matches[0].string, "unreachable!(…)");

        // fuzzy score should match for first two items as query is common prefix
        assert_eq!(matches[0].score, matches[1].score);
    })
    .await;
}

// #[gpui::test]
// async fn test_sort_matches_for_jsx_event_handler(cx: &mut TestAppContext) {
//     // Case 1: "on"
//     let completions = vec![
//         CompletionBuilder::function("onCut?", "12"),
//         CompletionBuilder::function("onPlay?", "12"),
//         CompletionBuilder::function("color?", "12"),
//         CompletionBuilder::function("defaultValue?", "12"),
//         CompletionBuilder::function("style?", "12"),
//         CompletionBuilder::function("className?", "12"),
//     ];
//     let matches =
//         filter_and_sort_matches("on", &completions, SnippetSortOrder::default(), cx).await;
//     assert_eq!(matches[0], "onCut?");
//     assert_eq!(matches[1], "onPlay?");

//     // Case 2: "ona"
//     let completions = vec![
//         CompletionBuilder::function("onAbort?", "12"),
//         CompletionBuilder::function("onAuxClick?", "12"),
//         CompletionBuilder::function("onPlay?", "12"),
//         CompletionBuilder::function("onLoad?", "12"),
//         CompletionBuilder::function("onDrag?", "12"),
//         CompletionBuilder::function("onPause?", "12"),
//         CompletionBuilder::function("onPaste?", "12"),
//         CompletionBuilder::function("onAnimationEnd?", "12"),
//         CompletionBuilder::function("onAbortCapture?", "12"),
//         CompletionBuilder::function("onChange?", "12"),
//         CompletionBuilder::function("onWaiting?", "12"),
//         CompletionBuilder::function("onCanPlay?", "12"),
//     ];
//     let matches =
//         filter_and_sort_matches("ona", &completions, SnippetSortOrder::default(), cx).await;
//     assert_eq!(matches[0], "onAbort?");
//     assert_eq!(matches[1], "onAbortCapture?");
// }

// #[gpui::test]
// async fn test_sort_matches_for_snippets(cx: &mut TestAppContext) {
//     // Case 1: "prin"
//     let completions = vec![
//         CompletionBuilder::constant("println", "80000000"),
//         CompletionBuilder::snippet("println!(…)", "80000000"),
//     ];
//     let matches = filter_and_sort_matches("prin", &completions, SnippetSortOrder::Top, cx).await;
//     assert_eq!(matches[0], "println!(…)");
// }

// #[gpui::test]
// async fn test_sort_matches_for_exact_match(cx: &mut TestAppContext) {
//     // Case 1: "set_text"
//     let completions = vec![
//         CompletionBuilder::function("set_text", "7fffffff"),
//         CompletionBuilder::function("set_placeholder_text", "7fffffff"),
//         CompletionBuilder::function("set_text_style_refinement", "7fffffff"),
//         CompletionBuilder::function("set_context_menu_options", "7fffffff"),
//         CompletionBuilder::function("select_to_next_word_end", "7fffffff"),
//         CompletionBuilder::function("select_to_next_subword_end", "7fffffff"),
//         CompletionBuilder::function("set_custom_context_menu", "7fffffff"),
//         CompletionBuilder::function("select_to_end_of_excerpt", "7fffffff"),
//         CompletionBuilder::function("select_to_start_of_excerpt", "7fffffff"),
//         CompletionBuilder::function("select_to_start_of_next_excerpt", "7fffffff"),
//         CompletionBuilder::function("select_to_end_of_previous_excerpt", "7fffffff"),
//     ];
//     let matches =
//         filter_and_sort_matches("set_text", &completions, SnippetSortOrder::Top, cx).await;
//     assert_eq!(matches[0], "set_text");
//     assert_eq!(matches[1], "set_text_style_refinement");
//     assert_eq!(matches[2], "set_context_menu_options");
// }

// #[gpui::test]
// async fn test_sort_matches_for_prefix_matches(cx: &mut TestAppContext) {
//     // Case 1: "set"
//     let completions = vec![
//         CompletionBuilder::function("select_to_beginning", "7fffffff"),
//         CompletionBuilder::function("set_collapse_matches", "7fffffff"),
//         CompletionBuilder::function("set_autoindent", "7fffffff"),
//         CompletionBuilder::function("set_all_diagnostics_active", "7fffffff"),
//         CompletionBuilder::function("select_to_end_of_line", "7fffffff"),
//         CompletionBuilder::function("select_all", "7fffffff"),
//         CompletionBuilder::function("select_line", "7fffffff"),
//         CompletionBuilder::function("select_left", "7fffffff"),
//         CompletionBuilder::function("select_down", "7fffffff"),
//     ];
//     let matches = filter_and_sort_matches("set", &completions, SnippetSortOrder::Top, cx).await;
//     assert_eq!(matches[0], "set_all_diagnostics_active");
//     assert_eq!(matches[1], "set_autoindent");
//     assert_eq!(matches[2], "set_collapse_matches");
// }

// // convert this test to filter test where filter is happening right

// // #[gpui::test]
// // async fn test_sort_matches_for_await(cx: &mut TestAppContext) {
// //     // Case 1: "awa"
// //     let completions = vec![
// //         CompletionBuilder::keyword("await", "7fffffff"),
// //         CompletionBuilder::function("await.ne", "80000010"),
// //         CompletionBuilder::function("await.eq", "80000010"),
// //         CompletionBuilder::function("await.or", "7ffffff8"),
// //         CompletionBuilder::function("await.zip", "80000006"),
// //         CompletionBuilder::function("await.xor", "7ffffff8"),
// //         CompletionBuilder::function("await.and", "80000006"),
// //         CompletionBuilder::function("await.map", "80000006"),
// //         CompletionBuilder::function("await.take", "7ffffff8"),
// //     ];
// //     let matches = sort_matches("awa", &completions, SnippetSortOrder::Top, cx).await;
// //     assert_eq!(matches[0], "await");

// //     // Case 2: "await"
// //     let completions = vec![
// //         CompletionBuilder::keyword("await", "7fffffff"),
// //         CompletionBuilder::function("await.ne", "80000010"),
// //         CompletionBuilder::function("await.eq", "80000010"),
// //         CompletionBuilder::function("await.or", "7ffffff8"),
// //         CompletionBuilder::function("await.zip", "80000006"),
// //         CompletionBuilder::function("await.xor", "7ffffff8"),
// //         CompletionBuilder::function("await.and", "80000006"),
// //         CompletionBuilder::function("await.map", "80000006"),
// //         CompletionBuilder::function("await.take", "7ffffff8"),
// //     ];
// //     let matches = sort_matches("await", &completions, SnippetSortOrder::Top, cx).await;
// //     assert_eq!(matches[0], "await");
// // }

// #[gpui::test]
// async fn test_sort_matches_for_python_init(cx: &mut TestAppContext) {
//     // Case 1: "__in"
//     let completions = vec![
//         CompletionBuilder::function("__init__", "05.0003.__init__"),
//         CompletionBuilder::function("__init__", "05.0003"),
//         CompletionBuilder::function("__instancecheck__", "05.0005.__instancecheck__"),
//         CompletionBuilder::function("__init_subclass__", "05.0004.__init_subclass__"),
//         CompletionBuilder::function("__instancecheck__", "05.0005"),
//         CompletionBuilder::function("__init_subclass__", "05.0004"),
//     ];
//     let matches = filter_and_sort_matches("__in", &completions, SnippetSortOrder::Top, cx).await;
//     assert_eq!(matches[0], "__init__");
//     assert_eq!(matches[1], "__init__");

//     // // Case 2: "__ini"
//     // let completions = vec![
//     //     CompletionBuilder::function("__init__", "05.0004.__init__"),
//     //     CompletionBuilder::function("__init__", "05.0004"),
//     //     CompletionBuilder::function("__init_subclass__", "05.0003.__init_subclass__"),
//     //     CompletionBuilder::function("__init_subclass__", "05.0003"),
//     // ];
//     // let matches = sort_matches("__ini", &completions, SnippetSortOrder::Top, cx).await;
//     // assert_eq!(matches[0], "__init__");
//     // assert_eq!(matches[1], "__init__");

//     // Case 3: "__init"
//     let completions = vec![
//         CompletionBuilder::function("__init__", "05.0000.__init__"),
//         CompletionBuilder::function("__init__", "05.0000"),
//         CompletionBuilder::function("__init_subclass__", "05.0001.__init_subclass__"),
//         CompletionBuilder::function("__init_subclass__", "05.0001"),
//     ];
//     let matches = filter_and_sort_matches("__init", &completions, SnippetSortOrder::Top, cx).await;
//     assert_eq!(matches[0], "__init__");
//     assert_eq!(matches[1], "__init__");

//     // Case 4: "__init_"
//     let completions = vec![
//         CompletionBuilder::function("__init__", "11.9999.__init__"),
//         CompletionBuilder::function("__init__", "11.9999"),
//         CompletionBuilder::function("__init_subclass__", "05.0000.__init_subclass__"),
//         CompletionBuilder::function("__init_subclass__", "05.0000"),
//     ];
//     let matches = filter_and_sort_matches("__init_", &completions, SnippetSortOrder::Top, cx).await;
//     // assert_eq!(matches[0], "__init__");
//     // assert_eq!(matches[1], "__init__");
// }

// #[gpui::test]
// async fn test_sort_matches_for_rust_into(cx: &mut TestAppContext) {
//     // Case 1: "int"
//     let completions = vec![
//         CompletionBuilder::function("into", "80000004"),
//         CompletionBuilder::function("try_into", "80000004"),
//         CompletionBuilder::snippet("println", "80000004"),
//         CompletionBuilder::function("clone_into", "80000004"),
//         CompletionBuilder::function("into_searcher", "80000000"),
//         CompletionBuilder::snippet("eprintln", "80000004"),
//     ];
//     let matches =
//         filter_and_sort_matches("int", &completions, SnippetSortOrder::default(), cx).await;
//     // assert_eq!(matches[0], "into");

//     // Case 2: "into"
//     let completions = vec![
//         CompletionBuilder::function("into", "80000004"),
//         CompletionBuilder::function("try_into", "80000004"),
//         CompletionBuilder::function("clone_into", "80000004"),
//         CompletionBuilder::function("into_searcher", "80000000"),
//         CompletionBuilder::function("split_terminator", "7fffffff"),
//         CompletionBuilder::function("rsplit_terminator", "7fffffff"),
//     ];
//     let matches =
//         filter_and_sort_matches("into", &completions, SnippetSortOrder::default(), cx).await;
//     // assert_eq!(matches[0], "into");
// }

// #[gpui::test]
// async fn test_sort_matches_for_variable_over_function(cx: &mut TestAppContext) {
//     // Case 1: "serial"
//     let completions = vec![
//         CompletionBuilder::function("serialize", "80000000"),
//         CompletionBuilder::function("serialize", "80000000"),
//         CompletionBuilder::variable("serialization_key", "7ffffffe"),
//         CompletionBuilder::function("serialize_version", "80000000"),
//         CompletionBuilder::function("deserialize", "80000000"),
//     ];
//     let matches =
//         filter_and_sort_matches("serial", &completions, SnippetSortOrder::default(), cx).await;
//     assert_eq!(matches[0], "serialization_key");
//     assert_eq!(matches[1], "serialize");
//     assert_eq!(matches[2], "serialize");
//     assert_eq!(matches[3], "serialize_version");
//     assert_eq!(matches[4], "deserialize");
// }

// #[gpui::test]
// async fn test_sort_matches_for_local_methods_over_library(cx: &mut TestAppContext) {
//     // Case 1: "setis"
//     let completions = vec![
//         CompletionBuilder::variable("setISODay", "16"),
//         CompletionBuilder::variable("setISOWeek", "16"),
//         CompletionBuilder::variable("setISOWeekYear", "16"),
//         CompletionBuilder::function("setISOWeekYear", "16"),
//         CompletionBuilder::variable("setIsRefreshing", "11"),
//         CompletionBuilder::function("setFips", "16"),
//     ];
//     let matches =
//         filter_and_sort_matches("setis", &completions, SnippetSortOrder::default(), cx).await;
//     assert_eq!(matches[0], "setIsRefreshing");
//     assert_eq!(matches[1], "setISODay");
//     assert_eq!(matches[2], "setISOWeek");
// }

// #[gpui::test]
// async fn test_sort_matches_for_prioritize_not_exact_match(cx: &mut TestAppContext) {
//     // Case 1: "item"
//     let completions = vec![
//         CompletionBuilder::function("Item", "16"),
//         CompletionBuilder::variable("Item", "16"),
//         CompletionBuilder::variable("items", "11"),
//         CompletionBuilder::function("ItemText", "16"),
//     ];
//     let matches =
//         filter_and_sort_matches("item", &completions, SnippetSortOrder::default(), cx).await;
//     assert_eq!(matches[0], "items");
//     assert_eq!(matches[1], "Item");
//     assert_eq!(matches[2], "Item");
//     assert_eq!(matches[3], "ItemText");
// }

// #[gpui::test]
// async fn test_sort_matches_for_tailwind_classes(cx: &mut TestAppContext) {
//     let completions = vec![
//         CompletionBuilder::function("rounded-full", "15788"),
//         CompletionBuilder::variable("rounded-t-full", "15846"),
//         CompletionBuilder::variable("rounded-b-full", "15731"),
//         CompletionBuilder::function("rounded-tr-full", "15866"),
//     ];
//     // Case 1: "rounded-full"
//     let matches = filter_and_sort_matches(
//         "rounded-full",
//         &completions,
//         SnippetSortOrder::default(),
//         cx,
//     )
//     .await;
//     assert_eq!(matches[0], "rounded-full");
//     // Case 2: "roundedfull"
//     let matches =
//         filter_and_sort_matches("roundedfull", &completions, SnippetSortOrder::default(), cx).await;
//     assert_eq!(matches[0], "rounded-full");
// }

// #[gpui::test]
// async fn test_sort_matches_for_special_characters(cx: &mut TestAppContext) {
//     // Case 1: "-"
//     let completions_case1 = vec![
//         CompletionBuilder::snippet("do .. end", "0001"),
//         CompletionBuilder::keyword("and", "0002"),
//         CompletionBuilder::keyword("break", "0003"),
//         CompletionBuilder::keyword("else", "0004"),
//         CompletionBuilder::snippet("elseif .. then", "0005"),
//         CompletionBuilder::keyword("end", "0006"),
//         CompletionBuilder::keyword("false", "0007"),
//     ];
//     let matches =
//         filter_and_sort_matches("-", &completions_case1, SnippetSortOrder::default(), cx).await;
//     assert_eq!(matches[0], "do .. end");
//     assert_eq!(matches[1], "and");
//     assert_eq!(matches[2], "else");

//     // Case 2: "--"
//     let completions_case2 = vec![
//         CompletionBuilder::snippet("#region", "0001"),
//         CompletionBuilder::snippet("#endregion", "0002"),
//     ];
//     let matches =
//         filter_and_sort_matches("--", &completions_case2, SnippetSortOrder::default(), cx).await;
//     assert_eq!(matches[0], "#region");
//     assert_eq!(matches[1], "#endregion");

//     // Case 3: "---"
//     let completions_case3 = vec![CompletionBuilder::snippet("@param;@return", "0001")];
//     let matches =
//         filter_and_sort_matches("---", &completions_case3, SnippetSortOrder::default(), cx).await;
//     assert_eq!(matches[0], "@param;@return");
// }

async fn test_for_each_prefix<F>(
    target: &str,
    completions: &Vec<Completion>,
    cx: &mut TestAppContext,
    mut test_fn: F,
) where
    F: FnMut(Vec<StringMatch>),
{
    for i in 1..=target.len() {
        let prefix = &target[..i];
        let matches =
            filter_and_sort_matches(prefix, completions, SnippetSortOrder::default(), cx).await;
        test_fn(matches);
    }
}

struct CompletionBuilder;

impl CompletionBuilder {
    fn constant(label: &str, filter_text: Option<&str>, sort_text: &str) -> Completion {
        Self::new(label, filter_text, sort_text, CompletionItemKind::CONSTANT)
    }

    fn function(label: &str, filter_text: Option<&str>, sort_text: &str) -> Completion {
        Self::new(label, filter_text, sort_text, CompletionItemKind::FUNCTION)
    }

    fn variable(label: &str, filter_text: Option<&str>, sort_text: &str) -> Completion {
        Self::new(label, filter_text, sort_text, CompletionItemKind::VARIABLE)
    }

    fn keyword(label: &str, filter_text: Option<&str>, sort_text: &str) -> Completion {
        Self::new(label, filter_text, sort_text, CompletionItemKind::KEYWORD)
    }

    fn snippet(label: &str, filter_text: Option<&str>, sort_text: &str) -> Completion {
        Self::new(label, filter_text, sort_text, CompletionItemKind::SNIPPET)
    }

    fn new(
        label: &str,
        filter_text: Option<&str>,
        sort_text: &str,
        kind: CompletionItemKind,
    ) -> Completion {
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
                    filter_text: filter_text.map(|text| text.to_string()),
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

async fn filter_and_sort_matches(
    query: &str,
    completions: &Vec<Completion>,
    snippet_sort_order: SnippetSortOrder,
    cx: &mut TestAppContext,
) -> Vec<StringMatch> {
    let candidates: Arc<[StringMatchCandidate]> = completions
        .iter()
        .enumerate()
        .map(|(id, completion)| StringMatchCandidate::new(id, &completion.filter_text()))
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
    CompletionsMenu::sort_string_matches(matches, Some(query), snippet_sort_order, &completions)
}
