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
            sort_key: (2, "foo_bar_baz"),
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
            sort_key: (1, "foo_bar_qux"),
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
            sort_key: (2, "floorf64"),
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
            sort_key: (2, "floorf32"),
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
            sort_key: (2, "floorf16"),
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
            sort_key: (2, "floorf128"),
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
            sort_key: (2, "foo_bar_baz"),
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
            sort_key: (1, "foo_bar_qux"),
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
            sort_key: (2, "ElementType"),
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
            sort_key: (1, "element_type"),
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
            sort_key: (2, "simd_select"),
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
            sort_key: (0, "while let"),
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
            sort_key: (2, "ElementType"),
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
            sort_key: (1, "element_type"),
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
            sort_key: (2, "REPLACEMENT_CHARACTER"),
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
            sort_key: (2, "ElementType"),
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
            sort_key: (1, "element_type"),
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
            sort_key: (2, "unreachable"),
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
            sort_key: (2, "unreachable!(…)"),
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
            sort_key: (2, "unchecked_rem"),
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
            sort_key: (2, "unreachable_unchecked"),
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
            sort_key: (3, "unreachable"),
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
            sort_key: (3, "unreachable!(…)"),
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
            sort_key: (3, "unreachable_unchecked"),
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
            sort_key: (2, "unreachable"),
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
            sort_key: (2, "unreachable!(…)"),
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
            sort_key: (2, "unreachable_unchecked"),
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
            sort_key: (3, "unreachable"),
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
            sort_key: (3, "unreachable!(…)"),
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
            sort_key: (3, "unreachable_unchecked"),
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
            sort_key: (2, "unreachable"),
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
            sort_key: (2, "unreachable!(…)"),
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
            sort_key: (2, "unreachable_unchecked"),
        },
    ];
    CompletionsMenu::sort_matches(&mut matches, query, SnippetSortOrder::default());
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "unreachable",
        "Perfect fuzzy match should be preferred over others"
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
            sort_key: (3, "var"), // function
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
            sort_key: (1, "var"), // variable
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
            sort_key: (3, "var"), // function
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
            sort_key: (2, "var"), // constant
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
fn test_sort_matches_jsx_event_handler(_cx: &mut TestAppContext) {
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
            sort_key: (3, "onCut?"),
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
            sort_key: (3, "onPlay?"),
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
            sort_key: (3, "color?"),
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
            sort_key: (3, "defaultValue?"),
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
            sort_key: (3, "style?"),
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
            sort_key: (3, "className?"),
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
            sort_key: (3, "onAbort?"),
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
            sort_key: (3, "onAuxClick?"),
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
            sort_key: (3, "onPlay?"),
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
            sort_key: (3, "onLoad?"),
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
            sort_key: (3, "onDrag?"),
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
            sort_key: (3, "onPause?"),
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
            sort_key: (3, "onPaste?"),
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
            sort_key: (3, "onAnimationEnd?"),
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
            sort_key: (3, "onAbortCapture?"),
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
            sort_key: (3, "onChange?"),
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
            sort_key: (3, "onWaiting?"),
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
            sort_key: (3, "onCanPlay?"),
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
            sort_key: (3, "onAnimationStart?"),
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
            sort_key: (3, "onAuxClickCapture?"),
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
            sort_key: (3, "onStalled?"),
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
            sort_key: (3, "onPlaying?"),
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
            sort_key: (3, "onDragEnd?"),
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
            sort_key: (3, "onInvalid?"),
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
            sort_key: (3, "onDragOver?"),
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
            sort_key: (3, "onDragExit?"),
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
            sort_key: (3, "onAnimationIteration?"),
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
            sort_key: (3, "onRateChange?"),
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
            sort_key: (3, "onLoadStart?"),
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
            sort_key: (3, "onDragStart?"),
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
            sort_key: (3, "onDragLeave?"),
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
            sort_key: (3, "onDragEnter?"),
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
            sort_key: (3, "onAnimationEndCapture?"),
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
            sort_key: (3, "onLoadedData?"),
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
            sort_key: (2, "println"),
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
            sort_key: (2, "println!(…)"),
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
            sort_key: (3, "set_text"),
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
            sort_key: (3, "set_placeholder_text"),
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
            sort_key: (3, "set_text_style_refinement"),
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
            sort_key: (3, "set_context_menu_options"),
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
            sort_key: (3, "select_to_next_word_end"),
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
            sort_key: (3, "select_to_next_subword_end"),
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
            sort_key: (3, "set_custom_context_menu"),
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
            sort_key: (3, "select_to_end_of_excerpt"),
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
            sort_key: (3, "select_to_start_of_excerpt"),
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
            sort_key: (3, "select_to_start_of_next_excerpt"),
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
            sort_key: (3, "select_to_end_of_previous_excerpt"),
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
            sort_key: (3, "select_to_beginning"),
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
            sort_key: (3, "set_collapse_matches"),
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
            sort_key: (3, "set_autoindent"),
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
            sort_key: (3, "set_all_diagnostics_active"),
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
            sort_key: (3, "select_to_end_of_line"),
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
            sort_key: (3, "select_all"),
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
            sort_key: (3, "select_line"),
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
            sort_key: (3, "select_left"),
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
            sort_key: (3, "select_down"),
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
            sort_key: (0, "await"),
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
            sort_key: (3, "await.ne"),
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
            sort_key: (3, "await.eq"),
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
            sort_key: (3, "await.or"),
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
            sort_key: (3, "await.zip"),
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
            sort_key: (3, "await.xor"),
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
            sort_key: (3, "await.and"),
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
            sort_key: (3, "await.map"),
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
            sort_key: (3, "await.take"),
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
            "await.xor",
            "await.take",
            "await.and",
            "await.map",
            "await.zip",
            "await.eq",
            "await.ne"
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
            sort_key: (0, "await"),
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
            sort_key: (3, "await.ne"),
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
            sort_key: (3, "await.eq"),
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
            sort_key: (3, "await.or"),
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
            sort_key: (3, "await.zip"),
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
            sort_key: (3, "await.xor"),
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
            sort_key: (3, "await.and"),
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
            sort_key: (3, "await.map"),
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
            sort_key: (3, "await.take"),
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
            "await.xor",
            "await.take",
            "await.and",
            "await.map",
            "await.zip",
            "await.eq",
            "await.ne"
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
            sort_key: (3, "__init__"),
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
            sort_key: (3, "__init__"),
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
            sort_key: (3, "__instancecheck__"),
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
            sort_key: (3, "__init_subclass__"),
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
            sort_key: (3, "__instancecheck__"),
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
            sort_key: (3, "__init_subclass__"),
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
            sort_key: (3, "__init__"),
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
            sort_key: (3, "__init__"),
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
            sort_key: (3, "__init_subclass__"),
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
            sort_key: (3, "__init_subclass__"),
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
            sort_key: (3, "__init__"),
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
            sort_key: (3, "__init__"),
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
            sort_key: (3, "__init_subclass__"),
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
            sort_key: (3, "__init_subclass__"),
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
            sort_key: (3, "__init__"),
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
            sort_key: (3, "__init__"),
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
            sort_key: (3, "__init_subclass__"),
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
            sort_key: (3, "__init_subclass__"),
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
