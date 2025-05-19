use crate::{
    code_context_menus::{CompletionsMenu, SortableMatch},
    editor_settings::SnippetSortOrder,
};
use fuzzy::StringMatch;
use gpui::TestAppContext;

#[gpui::test]
fn test_sort_matches_local_variable_over_global_variable(_cx: &mut TestAppContext) {
    // Case 1: "foo"
    let query: Option<&str> = Some("foo");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2727272727272727,
                positions: vec![],
                string: "foo_bar_baz".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 2,
            sort_label: "foo_bar_baz",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2727272727272727,
                positions: vec![],
                string: "foo_bar_qux".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffffe"),
            sort_kind: 1,
            sort_label: "foo_bar_qux",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.22499999999999998,
                positions: vec![],
                string: "floorf64".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "floorf64",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.22499999999999998,
                positions: vec![],
                string: "floorf32".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "floorf32",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.22499999999999998,
                positions: vec![],
                string: "floorf16".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "floorf16",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2,
                positions: vec![],
                string: "floorf128".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "floorf128",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "foo_bar_qux",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string.as_str(),
        "foo_bar_baz",
        "Match order not expected"
    );
    assert_eq!(
        matches[2].string_match.string.as_str(),
        "floorf16",
        "Match order not expected"
    );
    assert_eq!(
        matches[3].string_match.string.as_str(),
        "floorf32",
        "Match order not expected"
    );

    // Case 2: "foobar"
    let query: Option<&str> = Some("foobar");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.4363636363636364,
                positions: vec![],
                string: "foo_bar_baz".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 2,
            sort_label: "foo_bar_baz",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.4363636363636364,
                positions: vec![],
                string: "foo_bar_qux".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffffe"),
            sort_kind: 1,
            sort_label: "foo_bar_qux",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "foo_bar_qux",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string.as_str(),
        "foo_bar_baz",
        "Match order not expected"
    );
}

#[gpui::test]
fn test_sort_matches_local_variable_over_global_enum(_cx: &mut TestAppContext) {
    // Case 1: "ele"
    let query: Option<&str> = Some("ele");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2727272727272727,
                positions: vec![],
                string: "ElementType".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 2,
            sort_label: "ElementType",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.25,
                positions: vec![],
                string: "element_type".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffffe"),
            sort_kind: 1,
            sort_label: "element_type",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.16363636363636364,
                positions: vec![],
                string: "simd_select".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "simd_select",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.16,
                positions: vec![],
                string: "while let".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 0,
            sort_label: "while let",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "element_type",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string.as_str(),
        "ElementType",
        "Match order not expected"
    );

    // Case 2: "eleme"
    let query: Option<&str> = Some("eleme");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.4545454545454546,
                positions: vec![],
                string: "ElementType".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 2,
            sort_label: "ElementType",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.41666666666666663,
                positions: vec![],
                string: "element_type".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffffe"),
            sort_kind: 1,
            sort_label: "element_type",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.04714285714285713,
                positions: vec![],
                string: "REPLACEMENT_CHARACTER".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "REPLACEMENT_CHARACTER",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "element_type",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string.as_str(),
        "ElementType",
        "Match order not expected"
    );

    // Case 3: "Elem"
    let query: Option<&str> = Some("Elem");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.36363636363636365,
                positions: vec![],
                string: "ElementType".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 2,
            sort_label: "ElementType",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.0003333333333333333,
                positions: vec![],
                string: "element_type".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffffe"),
            sort_kind: 1,
            sort_label: "element_type",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "ElementType",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string.as_str(),
        "element_type",
        "Match order not expected"
    );
}

#[gpui::test]
fn test_sort_matches_for_unreachable(_cx: &mut TestAppContext) {
    // Case 1: "unre"
    let query: Option<&str> = Some("unre");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.36363636363636365,
                positions: vec![],
                string: "unreachable".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "unreachable",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.26666666666666666,
                positions: vec![],
                string: "unreachable!(…)".to_string(),
            },
            is_snippet: true,
            sort_text: Some("7fffffff"),
            sort_kind: 2,
            sort_label: "unreachable!(…)",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.24615384615384617,
                positions: vec![],
                string: "unchecked_rem".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "unchecked_rem",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.19047619047619047,
                positions: vec![],
                string: "unreachable_unchecked".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "unreachable_unchecked",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "unreachable!(…)",
        "Match order not expected"
    );

    // Case 2: "unrea"
    let query: Option<&str> = Some("unrea");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.4545454545454546,
                positions: vec![],
                string: "unreachable".to_string(),
            },
            is_snippet: true,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "unreachable",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.3333333333333333,
                positions: vec![],
                string: "unreachable!(…)".to_string(),
            },
            is_snippet: true,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "unreachable!(…)",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.23809523809523808,
                positions: vec![],
                string: "unreachable_unchecked".to_string(),
            },
            is_snippet: true,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "unreachable_unchecked",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "unreachable!(…)",
        "Match order not expected"
    );

    // Case 3: "unreach"
    let query: Option<&str> = Some("unreach");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.6363636363636364,
                positions: vec![],
                string: "unreachable".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "unreachable",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.4666666666666667,
                positions: vec![],
                string: "unreachable!(…)".to_string(),
            },
            is_snippet: true,
            sort_text: Some("7fffffff"),
            sort_kind: 2,
            sort_label: "unreachable!(…)",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.3333333333333333,
                positions: vec![],
                string: "unreachable_unchecked".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "unreachable_unchecked",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "unreachable!(…)",
        "Match order not expected"
    );

    // Case 4: "unreachabl"
    let query: Option<&str> = Some("unreachable");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.9090909090909092,
                positions: vec![],
                string: "unreachable".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "unreachable",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.6666666666666666,
                positions: vec![],
                string: "unreachable!(…)".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "unreachable!(…)",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.47619047619047616,
                positions: vec![],
                string: "unreachable_unchecked".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "unreachable_unchecked",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "unreachable!(…)",
        "Match order not expected"
    );

    // Case 5: "unreachable"
    let query: Option<&str> = Some("unreachable");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 1.0,
                positions: vec![],
                string: "unreachable".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "unreachable",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.7333333333333333,
                positions: vec![],
                string: "unreachable!(…)".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 2,
            sort_label: "unreachable!(…)",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.5238095238095237,
                positions: vec![],
                string: "unreachable_unchecked".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "unreachable_unchecked",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "unreachable!(…)",
        "LSP should take over even when fuzzy perfect matches"
    );
}

#[gpui::test]
fn test_sort_matches_variable_and_constants_over_function(_cx: &mut TestAppContext) {
    // Case 1: "var" as variable
    let query: Option<&str> = Some("var");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 1.0,
                positions: vec![],
                string: "var".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "var", // function
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1,
                score: 1.0,
                positions: vec![],
                string: "var".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 1,
            sort_label: "var", // variable
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.candidate_id, 1,
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.candidate_id, 0,
        "Match order not expected"
    );

    // Case 2:  "var" as constant
    let query: Option<&str> = Some("var");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 1.0,
                positions: vec![],
                string: "var".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "var", // function
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1,
                score: 1.0,
                positions: vec![],
                string: "var".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 2,
            sort_label: "var", // constant
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.candidate_id, 1,
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.candidate_id, 0,
        "Match order not expected"
    );
}

#[gpui::test]
fn test_sort_matches_for_jsx_event_handler(_cx: &mut TestAppContext) {
    // Case 1: "on"
    let query: Option<&str> = Some("on");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.3333333333333333,
                positions: vec![],
                string: "onCut?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onCut?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2857142857142857,
                positions: vec![],
                string: "onPlay?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onPlay?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.25,
                positions: vec![],
                string: "color?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "color?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.25,
                positions: vec![],
                string: "defaultValue?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "defaultValue?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.25,
                positions: vec![],
                string: "style?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "style?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.20,
                positions: vec![],
                string: "className?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "className?",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string, "onCut?",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string, "onPlay?",
        "Match order not expected"
    );

    // Case 2: "ona"
    let query: Option<&str> = Some("ona");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.375,
                positions: vec![],
                string: "onAbort?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onAbort?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2727272727272727,
                positions: vec![],
                string: "onAuxClick?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onAuxClick?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.23571428571428565,
                positions: vec![],
                string: "onPlay?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onPlay?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.23571428571428565,
                positions: vec![],
                string: "onLoad?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onLoad?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.23571428571428565,
                positions: vec![],
                string: "onDrag?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onDrag?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.22499999999999998,
                positions: vec![],
                string: "onPause?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onPause?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.22499999999999998,
                positions: vec![],
                string: "onPaste?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onPaste?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2,
                positions: vec![],
                string: "onAnimationEnd?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onAnimationEnd?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2,
                positions: vec![],
                string: "onAbortCapture?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onAbortCapture?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.1833333333333333,
                positions: vec![],
                string: "onChange?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onChange?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.18,
                positions: vec![],
                string: "onWaiting?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onWaiting?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.18,
                positions: vec![],
                string: "onCanPlay?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onCanPlay?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.1764705882352941,
                positions: vec![],
                string: "onAnimationStart?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onAnimationStart?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.16666666666666666,
                positions: vec![],
                string: "onAuxClickCapture?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onAuxClickCapture?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.16499999999999998,
                positions: vec![],
                string: "onStalled?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onStalled?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.16499999999999998,
                positions: vec![],
                string: "onPlaying?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onPlaying?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.16499999999999998,
                positions: vec![],
                string: "onDragEnd?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onDragEnd?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.15000000000000002,
                positions: vec![],
                string: "onInvalid?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onInvalid?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.15,
                positions: vec![],
                string: "onDragOver?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onDragOver?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.15,
                positions: vec![],
                string: "onDragExit?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onDragExit?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.14285714285714285,
                positions: vec![],
                string: "onAnimationIteration?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onAnimationIteration?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.13846153846153847,
                positions: vec![],
                string: "onRateChange?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onRateChange?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.13749999999999996,
                positions: vec![],
                string: "onLoadStart?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onLoadStart?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.13749999999999996,
                positions: vec![],
                string: "onDragStart?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onDragStart?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.13749999999999996,
                positions: vec![],
                string: "onDragLeave?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onDragLeave?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.13749999999999996,
                positions: vec![],
                string: "onDragEnter?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onDragEnter?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.13636363636363635,
                positions: vec![],
                string: "onAnimationEndCapture?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onAnimationEndCapture?",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.12692307692307692,
                positions: vec![],
                string: "onLoadedData?".to_string(),
            },
            is_snippet: false,
            sort_text: Some("12"),
            sort_kind: 3,
            sort_label: "onLoadedData?",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches
            .iter()
            .take(12)
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "onAbort?",
            "onAuxClick?",
            "onAbortCapture?",
            "onAnimationEnd?",
            "onAnimationStart?",
            "onAuxClickCapture?",
            "onAnimationIteration?",
            "onAnimationEndCapture?",
            "onDrag?",
            "onLoad?",
            "onPlay?",
            "onPaste?",
        ]
    );
}

#[gpui::test]
fn test_sort_matches_for_snippets(_cx: &mut TestAppContext) {
    // Case 1: "prin"
    let query: Option<&str> = Some("prin");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2,
                positions: vec![],
                string: "println".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "println",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.2,
                positions: vec![],
                string: "println!(…)".to_string(),
            },
            is_snippet: true,
            sort_text: Some("80000000"),
            sort_kind: 2,
            sort_label: "println!(…)",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::Top);
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "println!(…)",
        "Match order not expected"
    );
}

#[gpui::test]
fn test_sort_matches_for_exact_match(_cx: &mut TestAppContext) {
    // Case 1: "set_text"
    let query: Option<&str> = Some("set_text");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 1.0,
                positions: vec![],
                string: "set_text".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "set_text",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.32000000000000006,
                positions: vec![],
                string: "set_placeholder_text".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "set_placeholder_text",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.32,
                positions: vec![],
                string: "set_text_style_refinement".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "set_text_style_refinement",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.16666666666666666,
                positions: vec![],
                string: "set_context_menu_options".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "set_context_menu_options",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.08695652173913043,
                positions: vec![],
                string: "select_to_next_word_end".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_to_next_word_end",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.07692307692307693,
                positions: vec![],
                string: "select_to_next_subword_end".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_to_next_subword_end",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.06956521739130435,
                positions: vec![],
                string: "set_custom_context_menu".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "set_custom_context_menu",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.06,
                positions: vec![],
                string: "select_to_end_of_excerpt".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_to_end_of_excerpt",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.055384615384615386,
                positions: vec![],
                string: "select_to_start_of_excerpt".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_to_start_of_excerpt",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.0464516129032258,
                positions: vec![],
                string: "select_to_start_of_next_excerpt".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_to_start_of_next_excerpt",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.04363636363636363,
                positions: vec![],
                string: "select_to_end_of_previous_excerpt".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_to_end_of_previous_excerpt",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::Top);
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "set_text",
            "set_text_style_refinement",
            "set_placeholder_text",
            "set_context_menu_options",
            "set_custom_context_menu",
            "select_to_next_word_end",
            "select_to_next_subword_end",
            "select_to_end_of_excerpt",
            "select_to_start_of_excerpt",
            "select_to_start_of_next_excerpt",
            "select_to_end_of_previous_excerpt",
        ]
    );
}

#[gpui::test]
fn test_sort_matches_for_prefix_matches(_cx: &mut TestAppContext) {
    // Case 1: "set"
    let query: Option<&str> = Some("set");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.12631578947368421,
                positions: vec![],
                string: "select_to_beginning".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_to_beginning",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.15000000000000002,
                positions: vec![],
                string: "set_collapse_matches".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "set_collapse_matches",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.21428571428571427,
                positions: vec![],
                string: "set_autoindent".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "set_autoindent",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.11538461538461539,
                positions: vec![],
                string: "set_all_diagnostics_active".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "set_all_diagnostics_active",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.1142857142857143,
                positions: vec![],
                string: "select_to_end_of_line".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_to_end_of_line",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.15000000000000002,
                positions: vec![],
                string: "select_all".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_all",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.13636363636363635,
                positions: vec![],
                string: "select_line".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_line",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.13636363636363635,
                positions: vec![],
                string: "select_left".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_left",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.13636363636363635,
                positions: vec![],
                string: "select_down".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "select_down",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::Top);
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "set_autoindent",
            "set_collapse_matches",
            "set_all_diagnostics_active",
            "select_all",
            "select_down",
            "select_left",
            "select_line",
            "select_to_beginning",
            "select_to_end_of_line",
        ]
    );
}

#[gpui::test]
fn test_sort_matches_for_await(_cx: &mut TestAppContext) {
    // Case 1: "awa"
    let query: Option<&str> = Some("awa");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.6000000000000001,
                positions: vec![],
                string: "await".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 0,
            sort_label: "await",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 35,
                score: 0.375,
                positions: vec![],
                string: "await.ne".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000010"),
            sort_kind: 3,
            sort_label: "await.ne",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 34,
                score: 0.375,
                positions: vec![],
                string: "await.eq".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000010"),
            sort_kind: 3,
            sort_label: "await.eq",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 18,
                score: 0.375,
                positions: vec![],
                string: "await.or".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffff8"),
            sort_kind: 3,
            sort_label: "await.or",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 21,
                score: 0.3333333333333333,
                positions: vec![],
                string: "await.zip".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000006"),
            sort_kind: 3,
            sort_label: "await.zip",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 20,
                score: 0.3333333333333333,
                positions: vec![],
                string: "await.xor".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffff8"),
            sort_kind: 3,
            sort_label: "await.xor",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 15,
                score: 0.3333333333333333,
                positions: vec![],
                string: "await.and".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000006"),
            sort_kind: 3,
            sort_label: "await.and",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 9,
                score: 0.3333333333333333,
                positions: vec![],
                string: "await.map".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000006"),
            sort_kind: 3,
            sort_label: "await.map",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 47,
                score: 0.30000000000000004,
                positions: vec![],
                string: "await.take".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffff8"),
            sort_kind: 3,
            sort_label: "await.take",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::Top);
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "await",
            "await.or",
            "await.eq",
            "await.ne",
            "await.xor",
            "await.take",
            "await.and",
            "await.map",
            "await.zip"
        ]
    );
    // Case 2: "await"
    let query: Option<&str> = Some("await");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 1.0,
                positions: vec![],
                string: "await".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 0,
            sort_label: "await",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 35,
                score: 0.625,
                positions: vec![],
                string: "await.ne".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000010"),
            sort_kind: 3,
            sort_label: "await.ne",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 34,
                score: 0.625,
                positions: vec![],
                string: "await.eq".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000010"),
            sort_kind: 3,
            sort_label: "await.eq",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 18,
                score: 0.625,
                positions: vec![],
                string: "await.or".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffff8"),
            sort_kind: 3,
            sort_label: "await.or",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 21,
                score: 0.5555555555555556,
                positions: vec![],
                string: "await.zip".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000006"),
            sort_kind: 3,
            sort_label: "await.zip",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 20,
                score: 0.5555555555555556,
                positions: vec![],
                string: "await.xor".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffff8"),
            sort_kind: 3,
            sort_label: "await.xor",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 15,
                score: 0.5555555555555556,
                positions: vec![],
                string: "await.and".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000006"),
            sort_kind: 3,
            sort_label: "await.and",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 9,
                score: 0.5555555555555556,
                positions: vec![],
                string: "await.map".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000006"),
            sort_kind: 3,
            sort_label: "await.map",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 47,
                score: 0.5,
                positions: vec![],
                string: "await.take".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffff8"),
            sort_kind: 3,
            sort_label: "await.take",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::Top);
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "await",
            "await.or",
            "await.eq",
            "await.ne",
            "await.xor",
            "await.take",
            "await.and",
            "await.map",
            "await.zip"
        ]
    );
}

#[gpui::test]
fn test_sort_matches_for_python_init(_cx: &mut TestAppContext) {
    // Case 1: "__in"
    let query: Option<&str> = Some("__in");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 211,
                score: 0.5,
                positions: vec![],
                string: "__init__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0003.__init__"),
            sort_kind: 3,
            sort_label: "__init__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.5,
                positions: vec![],
                string: "__init__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0003"),
            sort_kind: 3,
            sort_label: "__init__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 215,
                score: 0.23529411764705882,
                positions: vec![],
                string: "__instancecheck__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0005.__instancecheck__"),
            sort_kind: 3,
            sort_label: "__instancecheck__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 213,
                score: 0.23529411764705882,
                positions: vec![],
                string: "__init_subclass__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0004.__init_subclass__"),
            sort_kind: 3,
            sort_label: "__init_subclass__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 4,
                score: 0.23529411764705882,
                positions: vec![],
                string: "__instancecheck__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0005"),
            sort_kind: 3,
            sort_label: "__instancecheck__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 2,
                score: 0.23529411764705882,
                positions: vec![],
                string: "__init_subclass__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0004"),
            sort_kind: 3,
            sort_label: "__init_subclass__",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::Top);
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "__init__",
            "__init__",
            "__init_subclass__",
            "__init_subclass__",
            "__instancecheck__",
            "__instancecheck__",
        ]
    );
    // Case 2: "__ini"
    let query: Option<&str> = Some("__ini");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 9,
                score: 0.625,
                positions: vec![],
                string: "__init__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0004.__init__"),
            sort_kind: 3,
            sort_label: "__init__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.625,
                positions: vec![],
                string: "__init__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0004"),
            sort_kind: 3,
            sort_label: "__init__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 10,
                score: 0.29411764705882354,
                positions: vec![],
                string: "__init_subclass__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0003.__init_subclass__"),
            sort_kind: 3,
            sort_label: "__init_subclass__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1,
                score: 0.29411764705882354,
                positions: vec![],
                string: "__init_subclass__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0003"),
            sort_kind: 3,
            sort_label: "__init_subclass__",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::Top);
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "__init__",
            "__init__",
            "__init_subclass__",
            "__init_subclass__",
        ]
    );
    // Case 3: "__init"
    let query: Option<&str> = Some("__init");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 7,
                score: 0.75,
                positions: vec![],
                string: "__init__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0000.__init__"),
            sort_kind: 3,
            sort_label: "__init__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.75,
                positions: vec![],
                string: "__init__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0000"),
            sort_kind: 3,
            sort_label: "__init__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 8,
                score: 0.3529411764705882,
                positions: vec![],
                string: "__init_subclass__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0001.__init_subclass__"),
            sort_kind: 3,
            sort_label: "__init_subclass__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1,
                score: 0.3529411764705882,
                positions: vec![],
                string: "__init_subclass__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0001"),
            sort_kind: 3,
            sort_label: "__init_subclass__",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::Top);
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "__init__",
            "__init__",
            "__init_subclass__",
            "__init_subclass__",
        ]
    );
    // Case 4: "__init_"
    let query: Option<&str> = Some("__init_");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 4,
                score: 0.875,
                positions: vec![],
                string: "__init__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("11.9999.__init__"),
            sort_kind: 3,
            sort_label: "__init__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.875,
                positions: vec![],
                string: "__init__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("11.9999"),
            sort_kind: 3,
            sort_label: "__init__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 5,
                score: 0.4117647058823529,
                positions: vec![],
                string: "__init_subclass__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0000.__init_subclass__"),
            sort_kind: 3,
            sort_label: "__init_subclass__",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1,
                score: 0.4117647058823529,
                positions: vec![],
                string: "__init_subclass__".to_string(),
            },
            is_snippet: false,
            sort_text: Some("05.0000"),
            sort_kind: 3,
            sort_label: "__init_subclass__",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::Top);
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "__init__",
            "__init__",
            "__init_subclass__",
            "__init_subclass__",
        ]
    );
}

#[gpui::test]
fn test_sort_matches_for_rust_into(_cx: &mut TestAppContext) {
    // Case 1: "int"
    let query: Option<&str> = Some("int");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 67,
                score: 0.75,
                positions: vec![],
                string: "into".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000004"),
            sort_kind: 3,
            sort_label: "into",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 68,
                score: 0.30000000000000004,
                positions: vec![],
                string: "try_into".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000004"),
            sort_kind: 3,
            sort_label: "try_into",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 108,
                score: 0.2571428571428571,
                positions: vec![],
                string: "println".to_string(),
            },
            is_snippet: true,
            sort_text: Some("80000004"),
            sort_kind: 3,
            sort_label: "println",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 73,
                score: 0.24,
                positions: vec![],
                string: "clone_into".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000004"),
            sort_kind: 3,
            sort_label: "clone_into",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1,
                score: 0.23076923076923078,
                positions: vec![],
                string: "into_searcher".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "into_searcher",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 109,
                score: 0.22499999999999998,
                positions: vec![],
                string: "eprintln".to_string(),
            },
            is_snippet: true,
            sort_text: Some("80000004"),
            sort_kind: 3,
            sort_label: "eprintln",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "into",
        "Match order not expected"
    );
    // Case 2: "into"
    let query: Option<&str> = Some("into");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 65,
                score: 1.0,
                positions: vec![],
                string: "into".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000004"),
            sort_kind: 3,
            sort_label: "into",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 66,
                score: 0.4,
                positions: vec![],
                string: "try_into".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000004"),
            sort_kind: 3,
            sort_label: "try_into",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 71,
                score: 0.32,
                positions: vec![],
                string: "clone_into".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000004"),
            sort_kind: 3,
            sort_label: "clone_into",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 0,
                score: 0.3076923076923077,
                positions: vec![],
                string: "into_searcher".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "into_searcher",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 27,
                score: 0.09,
                positions: vec![],
                string: "split_terminator".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "split_terminator",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 28,
                score: 0.08470588235294117,
                positions: vec![],
                string: "rsplit_terminator".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7fffffff"),
            sort_kind: 3,
            sort_label: "rsplit_terminator",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "into",
        "Match order not expected"
    );
}

#[gpui::test]
fn test_sort_matches_for_variable_over_function(_cx: &mut TestAppContext) {
    // Case 1: "serial"
    let query: Option<&str> = Some("serial");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 33,
                score: 0.6666666666666666,
                positions: vec![],
                string: "serialize".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "serialize",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 32,
                score: 0.6666666666666666,
                positions: vec![],
                string: "serialize".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "serialize",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 103,
                score: 0.3529411764705882,
                positions: vec![],
                string: "serialization_key".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffffe"),
            sort_kind: 1,
            sort_label: "serialization_key",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 18,
                score: 0.3529411764705882,
                positions: vec![],
                string: "serialize_version".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "serialize_version",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 65,
                score: 0.32727272727272727,
                positions: vec![],
                string: "deserialize".to_string(),
            },
            is_snippet: false,
            sort_text: Some("80000000"),
            sort_kind: 3,
            sort_label: "deserialize",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "serialization_key",
            "serialize",
            "serialize",
            "serialize_version",
            "deserialize"
        ]
    );
}

#[gpui::test]
fn test_sort_matches_for_local_methods_over_library(_cx: &mut TestAppContext) {
    // Case 1: "setis"
    let query: Option<&str> = Some("setis");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1200,
                score: 0.5555555555555556,
                positions: vec![],
                string: "setISODay".to_string(),
            },
            is_snippet: false,
            sort_text: Some("16"),
            sort_kind: 1,
            sort_label: "setISODay",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1216,
                score: 0.5,
                positions: vec![],
                string: "setISOWeek".to_string(),
            },
            is_snippet: false,
            sort_text: Some("16"),
            sort_kind: 1,
            sort_label: "setISOWeek",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1232,
                score: 0.3571428571428571,
                positions: vec![],
                string: "setISOWeekYear".to_string(),
            },
            is_snippet: false,
            sort_text: Some("16"),
            sort_kind: 1,
            sort_label: "setISOWeekYear",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1217,
                score: 0.3571428571428571,
                positions: vec![],
                string: "setISOWeekYear".to_string(),
            },
            is_snippet: false,
            sort_text: Some("16"),
            sort_kind: 3,
            sort_label: "setISOWeekYear",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 53,
                score: 0.3333333333333333,
                positions: vec![],
                string: "setIsRefreshing".to_string(),
            },
            is_snippet: false,
            sort_text: Some("11"),
            sort_kind: 1,
            sort_label: "setIsRefreshing",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1180,
                score: 0.2571428571428571,
                positions: vec![],
                string: "setFips".to_string(),
            },
            is_snippet: false,
            sort_text: Some("16"),
            sort_kind: 3,
            sort_label: "setFips",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "setIsRefreshing",
            "setISODay",
            "setISOWeek",
            "setISOWeekYear",
            "setISOWeekYear",
            "setFips"
        ]
    );
}

#[gpui::test]
fn test_sort_matches_for_priotize_not_exact_match(_cx: &mut TestAppContext) {
    // Case 1: "item"
    let query: Option<&str> = Some("item");
    let mut matches: Vec<SortableMatch<'_>> = vec![
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1115,
                score: 1.0,
                positions: vec![],
                string: "Item".to_string(),
            },
            is_snippet: false,
            sort_text: Some("16"),
            sort_kind: 3,
            sort_label: "Item",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1108,
                score: 1.0,
                positions: vec![],
                string: "Item".to_string(),
            },
            is_snippet: false,
            sort_text: Some("16"),
            sort_kind: 1,
            sort_label: "Item",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 26,
                score: 0.8,
                positions: vec![],
                string: "items".to_string(),
            },
            is_snippet: false,
            sort_text: Some("11"),
            sort_kind: 1,
            sort_label: "items",
        },
        SortableMatch {
            string_match: StringMatch {
                candidate_id: 1138,
                score: 0.5,
                positions: vec![],
                string: "ItemText".to_string(),
            },
            is_snippet: false,
            sort_text: Some("16"),
            sort_kind: 3,
            sort_label: "ItemText",
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches
            .iter()
            .map(|m| m.string_match.string.as_str())
            .collect::<Vec<&str>>(),
        vec!["items", "Item", "Item", "ItemText"]
    );
}
