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

    // variable takes precedence over constant
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
async fn test_fuzzy_score(cx: &mut TestAppContext) {
    // first character sensitive over sort_text and sort_kind
    {
        let completions = vec![
            CompletionBuilder::variable("element_type", None, "7ffffffe"),
            CompletionBuilder::constant("ElementType", None, "7fffffff"),
        ];
        let matches =
            filter_and_sort_matches("Elem", &completions, SnippetSortOrder::default(), cx).await;
        assert_eq!(
            matches
                .iter()
                .map(|m| m.string.as_str())
                .collect::<Vec<_>>(),
            vec!["ElementType", "element_type"]
        );
        assert!(matches[0].score > matches[1].score);
    }

    // fuzzy takes over sort_text and sort_kind
    {
        let completions = vec![
            CompletionBuilder::function("onAbort?", None, "12"),
            CompletionBuilder::function("onAuxClick?", None, "12"),
            CompletionBuilder::variable("onPlay?", None, "12"),
            CompletionBuilder::variable("onLoad?", None, "12"),
            CompletionBuilder::variable("onDrag?", None, "12"),
            CompletionBuilder::function("onPause?", None, "10"),
            CompletionBuilder::function("onPaste?", None, "10"),
            CompletionBuilder::function("onAnimationEnd?", None, "12"),
            CompletionBuilder::function("onAbortCapture?", None, "12"),
            CompletionBuilder::constant("onChange?", None, "12"),
            CompletionBuilder::constant("onWaiting?", None, "12"),
            CompletionBuilder::function("onCanPlay?", None, "12"),
        ];
        let matches =
            filter_and_sort_matches("ona", &completions, SnippetSortOrder::default(), cx).await;
        for i in 0..4 {
            assert!(matches[i].string.to_lowercase().starts_with("ona"));
        }
    }

    // plain fuzzy prefix match
    {
        let completions = vec![
            CompletionBuilder::function("set_text", None, "7fffffff"),
            CompletionBuilder::function("set_placeholder_text", None, "7fffffff"),
            CompletionBuilder::function("set_text_style_refinement", None, "7fffffff"),
            CompletionBuilder::function("set_context_menu_options", None, "7fffffff"),
            CompletionBuilder::function("select_to_next_word_end", None, "7fffffff"),
            CompletionBuilder::function("select_to_next_subword_end", None, "7fffffff"),
            CompletionBuilder::function("set_custom_context_menu", None, "7fffffff"),
            CompletionBuilder::function("select_to_end_of_excerpt", None, "7fffffff"),
            CompletionBuilder::function("select_to_start_of_excerpt", None, "7fffffff"),
            CompletionBuilder::function("select_to_start_of_next_excerpt", None, "7fffffff"),
            CompletionBuilder::function("select_to_end_of_previous_excerpt", None, "7fffffff"),
        ];
        let matches =
            filter_and_sort_matches("set_text", &completions, SnippetSortOrder::Top, cx).await;
        assert_eq!(matches[0].string, "set_text");
        assert_eq!(matches[1].string, "set_text_style_refinement");
        assert_eq!(matches[2].string, "set_placeholder_text");
    }

    // fuzzy filter text over label, sort_text and sort_kind
    {
        // Case 1: "awa"
        let completions = vec![
            CompletionBuilder::method("await", Some("await"), "7fffffff"),
            CompletionBuilder::method("await.ne", Some("ne"), "80000010"),
            CompletionBuilder::method("await.eq", Some("eq"), "80000010"),
            CompletionBuilder::method("await.or", Some("or"), "7ffffff8"),
            CompletionBuilder::method("await.zip", Some("zip"), "80000006"),
            CompletionBuilder::method("await.xor", Some("xor"), "7ffffff8"),
            CompletionBuilder::method("await.and", Some("and"), "80000006"),
            CompletionBuilder::method("await.map", Some("map"), "80000006"),
        ];

        test_for_each_prefix("await", &completions, cx, |matches| {
            // for each prefix, first item should always be one with lower sort_text
            assert_eq!(matches[0].string, "await");
        })
        .await;
    }
}

#[gpui::test]
async fn test_sort_text(cx: &mut TestAppContext) {
    // sort text takes precedance over sort_kind, when fuzzy is same
    {
        let completions = vec![
            CompletionBuilder::variable("unreachable", None, "80000000"),
            CompletionBuilder::function("unreachable!(…)", None, "7fffffff"),
            CompletionBuilder::function("unchecked_rem", None, "80000010"),
            CompletionBuilder::function("unreachable_unchecked", None, "80000020"),
        ];

        test_for_each_prefix("unreachabl", &completions, cx, |matches| {
            // for each prefix, first item should always be one with lower sort_text
            assert_eq!(matches[0].string, "unreachable!(…)");
            assert_eq!(matches[1].string, "unreachable");

            // fuzzy score should match for first two items as query is common prefix
            assert_eq!(matches[0].score, matches[1].score);
        })
        .await;

        let matches =
            filter_and_sort_matches("unreachable", &completions, SnippetSortOrder::Top, cx).await;
        // exact match comes first
        assert_eq!(matches[0].string, "unreachable");
        assert_eq!(matches[1].string, "unreachable!(…)");

        // fuzzy score should match for first two items as query is common prefix
        assert_eq!(matches[0].score, matches[1].score);
    }
}

#[gpui::test]
async fn test_sort_snippet(cx: &mut TestAppContext) {
    let completions = vec![
        CompletionBuilder::constant("println", None, "7fffffff"),
        CompletionBuilder::snippet("println!(…)", None, "80000000"),
    ];
    let matches = filter_and_sort_matches("prin", &completions, SnippetSortOrder::Top, cx).await;

    // snippet take precedence over sort_text and sort_kind
    assert_eq!(matches[0].string, "println!(…)");
}

#[gpui::test]
async fn test_sort_exact(cx: &mut TestAppContext) {
    // sort_text takes over if no exact match
    let completions = vec![
        CompletionBuilder::function("into", None, "80000004"),
        CompletionBuilder::function("try_into", None, "80000004"),
        CompletionBuilder::snippet("println", None, "80000004"),
        CompletionBuilder::function("clone_into", None, "80000004"),
        CompletionBuilder::function("into_searcher", None, "80000000"),
        CompletionBuilder::snippet("eprintln", None, "80000004"),
    ];
    let matches =
        filter_and_sort_matches("int", &completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0].string, "into_searcher");

    // exact match takes over sort_text
    let completions = vec![
        CompletionBuilder::function("into", None, "80000004"),
        CompletionBuilder::function("try_into", None, "80000004"),
        CompletionBuilder::function("clone_into", None, "80000004"),
        CompletionBuilder::function("into_searcher", None, "80000000"),
        CompletionBuilder::function("split_terminator", None, "7fffffff"),
        CompletionBuilder::function("rsplit_terminator", None, "7fffffff"),
    ];
    let matches =
        filter_and_sort_matches("into", &completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0].string, "into");
}

#[gpui::test]
async fn test_sort_positions(cx: &mut TestAppContext) {
    // positions take precedence over fuzzy score and sort_text
    let completions = vec![
        CompletionBuilder::function("rounded-full", None, "15788"),
        CompletionBuilder::variable("rounded-t-full", None, "15846"),
        CompletionBuilder::variable("rounded-b-full", None, "15731"),
        CompletionBuilder::function("rounded-tr-full", None, "15866"),
    ];

    let matches = filter_and_sort_matches(
        "rounded-full",
        &completions,
        SnippetSortOrder::default(),
        cx,
    )
    .await;
    assert_eq!(matches[0].string, "rounded-full");

    let matches =
        filter_and_sort_matches("roundedfull", &completions, SnippetSortOrder::default(), cx).await;
    assert_eq!(matches[0].string, "rounded-full");
}

#[gpui::test]
async fn test_fuzzy_over_sort_positions(cx: &mut TestAppContext) {
    let completions = vec![
        CompletionBuilder::variable("lsp_document_colors", None, "7fffffff"), // 0.29 fuzzy score
        CompletionBuilder::function(
            "language_servers_running_disk_based_diagnostics",
            None,
            "7fffffff",
        ), // 0.168 fuzzy score
        CompletionBuilder::function("code_lens", None, "7fffffff"),           // 3.2 fuzzy score
        CompletionBuilder::variable("lsp_code_lens", None, "7fffffff"),       // 3.2 fuzzy score
        CompletionBuilder::function("fetch_code_lens", None, "7fffffff"),     // 3.2 fuzzy score
    ];

    let matches =
        filter_and_sort_matches("lens", &completions, SnippetSortOrder::default(), cx).await;

    assert_eq!(matches[0].string, "code_lens");
    assert_eq!(matches[1].string, "lsp_code_lens");
    assert_eq!(matches[2].string, "fetch_code_lens");
}

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

    fn method(label: &str, filter_text: Option<&str>, sort_text: &str) -> Completion {
        Self::new(label, filter_text, sort_text, CompletionItemKind::METHOD)
    }

    fn variable(label: &str, filter_text: Option<&str>, sort_text: &str) -> Completion {
        Self::new(label, filter_text, sort_text, CompletionItemKind::VARIABLE)
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
            label: CodeLabel::plain(label.to_string(), filter_text),
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
        .map(|(id, completion)| StringMatchCandidate::new(id, completion.label.filter_text()))
        .collect();
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let background_executor = cx.executor();
    let matches = fuzzy::match_strings(
        &candidates,
        query,
        query.chars().any(|c| c.is_uppercase()),
        false,
        100,
        &cancel_flag,
        background_executor,
    )
    .await;
    CompletionsMenu::sort_string_matches(matches, Some(query), snippet_sort_order, completions)
}
