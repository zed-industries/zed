use crate::code_context_menus::{CompletionsMenu, SortableMatch};
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
                string: "foo_baz_qux".to_string(),
            },
            is_snippet: false,
            sort_text: Some("7ffffffe"),
            sort_key: (1, "foo_baz_qux"),
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
    CompletionsMenu::sort_matches(&mut matches, query);
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "foo_baz_qux",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string.as_str(),
        "foo_bar_baz",
        "Match order not expected"
    );
    assert_eq!(
        matches[2].string_match.string.as_str(),
        "floorf128",
        "Match order not expected"
    );
    assert_eq!(
        matches[3].string_match.string.as_str(),
        "floorf16",
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
    CompletionsMenu::sort_matches(&mut matches, query);
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "foo_baz_qux",
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
    CompletionsMenu::sort_matches(&mut matches, query);
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
    CompletionsMenu::sort_matches(&mut matches, query);
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
    CompletionsMenu::sort_matches(&mut matches, query);
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
fn test_sort_matches_unreachable(_cx: &mut TestAppContext) {
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
            is_snippet: false,
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
    CompletionsMenu::sort_matches(&mut matches, query);
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "unreachable!(…)",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string.as_str(),
        "unreachable",
        "Match order not expected"
    );
    assert_eq!(
        matches[2].string_match.string.as_str(),
        "unchecked_rem",
        "Match order not expected"
    );

    // Case 2: "unreach"
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
            is_snippet: false,
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
    CompletionsMenu::sort_matches(&mut matches, query);
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "unreachable!(…)",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string.as_str(),
        "unreachable",
        "Match order not expected"
    );
    assert_eq!(
        matches[2].string_match.string.as_str(),
        "unchecked_rem",
        "Match order not expected"
    );

    // Case 3: "unreachable"
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
    CompletionsMenu::sort_matches(&mut matches, query);
    assert_eq!(
        matches[0].string_match.string.as_str(),
        "unreachable!(…)",
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.string.as_str(),
        "unreachable",
        "Match order not expected"
    );
    assert_eq!(
        matches[2].string_match.string.as_str(),
        "unchecked_rem",
        "Match order not expected"
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
    CompletionsMenu::sort_matches(&mut matches, query);
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
    CompletionsMenu::sort_matches(&mut matches, query);
    assert_eq!(
        matches[0].string_match.candidate_id, 1,
        "Match order not expected"
    );
    assert_eq!(
        matches[1].string_match.candidate_id, 0,
        "Match order not expected"
    );
}
