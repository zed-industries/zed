use anyhow::{Context, Result};
use collections::HashMap;
use convert_case::{Case, Casing};
use std::{cmp::Reverse, ops::Range, sync::LazyLock};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Query, QueryMatch};

fn migrate(text: &str, patterns: MigrationPatterns, query: &Query) -> Result<Option<String>> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_json::LANGUAGE.into())?;
    let syntax_tree = parser
        .parse(&text, None)
        .context("failed to parse settings")?;

    let mut cursor = tree_sitter::QueryCursor::new();
    let mut matches = cursor.matches(query, syntax_tree.root_node(), text.as_bytes());

    let mut edits = vec![];
    while let Some(mat) = matches.next() {
        if let Some((_, callback)) = patterns.get(mat.pattern_index) {
            edits.extend(callback(&text, &mat, query));
        }
    }

    edits.sort_by_key(|(range, _)| (range.start, Reverse(range.end)));
    edits.dedup_by(|(range_b, _), (range_a, _)| {
        range_a.contains(&range_b.start) || range_a.contains(&range_b.end)
    });

    if edits.is_empty() {
        Ok(None)
    } else {
        let mut new_text = text.to_string();
        for (range, replacement) in edits.iter().rev() {
            new_text.replace_range(range.clone(), replacement);
        }
        if new_text == text {
            log::error!(
                "Edits computed for configuration migration do not cause a change: {:?}",
                edits
            );
            Ok(None)
        } else {
            Ok(Some(new_text))
        }
    }
}

pub fn migrate_keymap(text: &str) -> Result<Option<String>> {
    let transformed_text = migrate(
        text,
        KEYMAP_MIGRATION_TRANSFORMATION_PATTERNS,
        &KEYMAP_MIGRATION_TRANSFORMATION_QUERY,
    )?;
    let replacement_text = migrate(
        &transformed_text.as_ref().unwrap_or(&text.to_string()),
        KEYMAP_MIGRATION_REPLACEMENT_PATTERNS,
        &KEYMAP_MIGRATION_REPLACEMENT_QUERY,
    )?;
    Ok(replacement_text.or(transformed_text))
}

pub fn migrate_settings(text: &str) -> Result<Option<String>> {
    migrate(
        &text,
        SETTINGS_MIGRATION_PATTERNS,
        &SETTINGS_MIGRATION_QUERY,
    )
}

pub fn migrate_edit_prediction_provider_settings(text: &str) -> Result<Option<String>> {
    migrate(
        &text,
        &[(
            SETTINGS_NESTED_KEY_VALUE_PATTERN,
            replace_edit_prediction_provider_setting,
        )],
        &EDIT_PREDICTION_SETTINGS_MIGRATION_QUERY,
    )
}

type MigrationPatterns = &'static [(
    &'static str,
    fn(&str, &QueryMatch, &Query) -> Option<(Range<usize>, String)>,
)];

const KEYMAP_MIGRATION_TRANSFORMATION_PATTERNS: MigrationPatterns = &[
    (ACTION_ARRAY_PATTERN, replace_array_with_single_string),
    (
        ACTION_ARGUMENT_OBJECT_PATTERN,
        replace_action_argument_object_with_single_value,
    ),
    (ACTION_STRING_PATTERN, rename_string_action),
    (CONTEXT_PREDICATE_PATTERN, rename_context_key),
    (
        ACTION_STRING_OF_ARRAY_PATTERN,
        replace_first_string_of_array,
    ),
];

static KEYMAP_MIGRATION_TRANSFORMATION_QUERY: LazyLock<Query> = LazyLock::new(|| {
    Query::new(
        &tree_sitter_json::LANGUAGE.into(),
        &KEYMAP_MIGRATION_TRANSFORMATION_PATTERNS
            .iter()
            .map(|pattern| pattern.0)
            .collect::<String>(),
    )
    .unwrap()
});

const ACTION_ARRAY_PATTERN: &str = r#"(document
    (array
   	    (object
            (pair
                key: (string (string_content) @name)
                value: (
                    (object
                        (pair
                            key: (string)
                            value: ((array
                                . (string (string_content) @action_name)
                                . (string (string_content) @argument)
                                .)) @array
                        )
                    )
                )
            )
        )
    )
    (#eq? @name "bindings")
)"#;

fn replace_array_with_single_string(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let array_ix = query.capture_index_for_name("array")?;
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let argument_ix = query.capture_index_for_name("argument")?;

    let action_name = contents.get(
        mat.nodes_for_capture_index(action_name_ix)
            .next()?
            .byte_range(),
    )?;
    let argument = contents.get(
        mat.nodes_for_capture_index(argument_ix)
            .next()?
            .byte_range(),
    )?;

    let replacement = TRANSFORM_ARRAY.get(&(action_name, argument))?;
    let replacement_as_string = format!("\"{replacement}\"");
    let range_to_replace = mat.nodes_for_capture_index(array_ix).next()?.byte_range();

    Some((range_to_replace, replacement_as_string))
}

static TRANSFORM_ARRAY: LazyLock<HashMap<(&str, &str), &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        // activate
        (
            ("workspace::ActivatePaneInDirection", "Up"),
            "workspace::ActivatePaneUp",
        ),
        (
            ("workspace::ActivatePaneInDirection", "Down"),
            "workspace::ActivatePaneDown",
        ),
        (
            ("workspace::ActivatePaneInDirection", "Left"),
            "workspace::ActivatePaneLeft",
        ),
        (
            ("workspace::ActivatePaneInDirection", "Right"),
            "workspace::ActivatePaneRight",
        ),
        // swap
        (
            ("workspace::SwapPaneInDirection", "Up"),
            "workspace::SwapPaneUp",
        ),
        (
            ("workspace::SwapPaneInDirection", "Down"),
            "workspace::SwapPaneDown",
        ),
        (
            ("workspace::SwapPaneInDirection", "Left"),
            "workspace::SwapPaneLeft",
        ),
        (
            ("workspace::SwapPaneInDirection", "Right"),
            "workspace::SwapPaneRight",
        ),
        // menu
        (
            ("app_menu::NavigateApplicationMenuInDirection", "Left"),
            "app_menu::ActivateMenuLeft",
        ),
        (
            ("app_menu::NavigateApplicationMenuInDirection", "Right"),
            "app_menu::ActivateMenuRight",
        ),
        // vim push
        (("vim::PushOperator", "Change"), "vim::PushChange"),
        (("vim::PushOperator", "Delete"), "vim::PushDelete"),
        (("vim::PushOperator", "Yank"), "vim::PushYank"),
        (("vim::PushOperator", "Replace"), "vim::PushReplace"),
        (
            ("vim::PushOperator", "DeleteSurrounds"),
            "vim::PushDeleteSurrounds",
        ),
        (("vim::PushOperator", "Mark"), "vim::PushMark"),
        (("vim::PushOperator", "Indent"), "vim::PushIndent"),
        (("vim::PushOperator", "Outdent"), "vim::PushOutdent"),
        (("vim::PushOperator", "AutoIndent"), "vim::PushAutoIndent"),
        (("vim::PushOperator", "Rewrap"), "vim::PushRewrap"),
        (
            ("vim::PushOperator", "ShellCommand"),
            "vim::PushShellCommand",
        ),
        (("vim::PushOperator", "Lowercase"), "vim::PushLowercase"),
        (("vim::PushOperator", "Uppercase"), "vim::PushUppercase"),
        (
            ("vim::PushOperator", "OppositeCase"),
            "vim::PushOppositeCase",
        ),
        (("vim::PushOperator", "Register"), "vim::PushRegister"),
        (
            ("vim::PushOperator", "RecordRegister"),
            "vim::PushRecordRegister",
        ),
        (
            ("vim::PushOperator", "ReplayRegister"),
            "vim::PushReplayRegister",
        ),
        (
            ("vim::PushOperator", "ReplaceWithRegister"),
            "vim::PushReplaceWithRegister",
        ),
        (
            ("vim::PushOperator", "ToggleComments"),
            "vim::PushToggleComments",
        ),
        // vim switch
        (("vim::SwitchMode", "Normal"), "vim::SwitchToNormalMode"),
        (("vim::SwitchMode", "Insert"), "vim::SwitchToInsertMode"),
        (("vim::SwitchMode", "Replace"), "vim::SwitchToReplaceMode"),
        (("vim::SwitchMode", "Visual"), "vim::SwitchToVisualMode"),
        (
            ("vim::SwitchMode", "VisualLine"),
            "vim::SwitchToVisualLineMode",
        ),
        (
            ("vim::SwitchMode", "VisualBlock"),
            "vim::SwitchToVisualBlockMode",
        ),
        (
            ("vim::SwitchMode", "HelixNormal"),
            "vim::SwitchToHelixNormalMode",
        ),
        // vim resize
        (("vim::ResizePane", "Widen"), "vim::ResizePaneRight"),
        (("vim::ResizePane", "Narrow"), "vim::ResizePaneLeft"),
        (("vim::ResizePane", "Shorten"), "vim::ResizePaneDown"),
        (("vim::ResizePane", "Lengthen"), "vim::ResizePaneUp"),
    ])
});

const ACTION_STRING_OF_ARRAY_PATTERN: &str = r#"(document
    (array
   	    (object
            (pair
                key: (string (string_content) @name)
                value: (
                    (object
                        (pair
                            key: (string)
                            value: ((array
                                . (string (string_content) @action_name)
                                )
                            )
                        )
                    )
                )
            )
        )
    )
    (#eq? @name "bindings")
)"#;

// ["editor::GoToPrevHunk", { "center_cursor": true }] -> ["editor::GoToPreviousHunk", { "center_cursor": true }]
fn replace_first_string_of_array(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let action_name = contents.get(
        mat.nodes_for_capture_index(action_name_ix)
            .next()?
            .byte_range(),
    )?;
    let replacement = STRING_OF_ARRAY_REPLACE.get(action_name)?;
    let range_to_replace = mat
        .nodes_for_capture_index(action_name_ix)
        .next()?
        .byte_range();
    Some((range_to_replace, replacement.to_string()))
}

static STRING_OF_ARRAY_REPLACE: LazyLock<HashMap<&str, &str>> =
    LazyLock::new(|| HashMap::from_iter([("editor::GoToPrevHunk", "editor::GoToPreviousHunk")]));

const ACTION_ARGUMENT_OBJECT_PATTERN: &str = r#"(document
    (array
        (object
            (pair
                key: (string (string_content) @name)
                value: (
                    (object
                        (pair
                            key: (string)
                            value: ((array
                                . (string (string_content) @action_name)
                                . (object
                                    (pair
                                    key: (string (string_content) @action_key)
                                    value: (_)  @argument))
                                . ) @array
                            ))
                        )
                    )
                )
            )
        )
        (#eq? @name "bindings")
)"#;

/// [ "editor::FoldAtLevel", { "level": 1 } ] -> [ "editor::FoldAtLevel", 1 ]
fn replace_action_argument_object_with_single_value(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let array_ix = query.capture_index_for_name("array")?;
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let action_key_ix = query.capture_index_for_name("action_key")?;
    let argument_ix = query.capture_index_for_name("argument")?;

    let action_name = contents.get(
        mat.nodes_for_capture_index(action_name_ix)
            .next()?
            .byte_range(),
    )?;
    let action_key = contents.get(
        mat.nodes_for_capture_index(action_key_ix)
            .next()?
            .byte_range(),
    )?;
    let argument = contents.get(
        mat.nodes_for_capture_index(argument_ix)
            .next()?
            .byte_range(),
    )?;

    let new_action_name = UNWRAP_OBJECTS.get(&action_name)?.get(&action_key)?;

    let range_to_replace = mat.nodes_for_capture_index(array_ix).next()?.byte_range();
    let replacement = format!("[\"{}\", {}]", new_action_name, argument);
    Some((range_to_replace, replacement))
}

/// "ctrl-k ctrl-1": [ "editor::PushOperator", { "Object": {} } ] -> [ "editor::vim::PushObject", {} ]
static UNWRAP_OBJECTS: LazyLock<HashMap<&str, HashMap<&str, &str>>> = LazyLock::new(|| {
    HashMap::from_iter([
        (
            "editor::FoldAtLevel",
            HashMap::from_iter([("level", "editor::FoldAtLevel")]),
        ),
        (
            "vim::PushOperator",
            HashMap::from_iter([
                ("Object", "vim::PushObject"),
                ("FindForward", "vim::PushFindForward"),
                ("FindBackward", "vim::PushFindBackward"),
                ("Sneak", "vim::PushSneak"),
                ("SneakBackward", "vim::PushSneakBackward"),
                ("AddSurrounds", "vim::PushAddSurrounds"),
                ("ChangeSurrounds", "vim::PushChangeSurrounds"),
                ("Jump", "vim::PushJump"),
                ("Digraph", "vim::PushDigraph"),
                ("Literal", "vim::PushLiteral"),
            ]),
        ),
    ])
});

const KEYMAP_MIGRATION_REPLACEMENT_PATTERNS: MigrationPatterns = &[(
    ACTION_ARGUMENT_SNAKE_CASE_PATTERN,
    action_argument_snake_case,
)];

static KEYMAP_MIGRATION_REPLACEMENT_QUERY: LazyLock<Query> = LazyLock::new(|| {
    Query::new(
        &tree_sitter_json::LANGUAGE.into(),
        &KEYMAP_MIGRATION_REPLACEMENT_PATTERNS
            .iter()
            .map(|pattern| pattern.0)
            .collect::<String>(),
    )
    .unwrap()
});

const ACTION_STRING_PATTERN: &str = r#"(document
    (array
        (object
            (pair
                key: (string (string_content) @name)
                value: (
                    (object
                        (pair
                            key: (string)
                            value: (string (string_content) @action_name)
                        )
                    )
                )
            )
        )
    )
    (#eq? @name "bindings")
)"#;

fn rename_string_action(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let action_name_range = mat
        .nodes_for_capture_index(action_name_ix)
        .next()?
        .byte_range();
    let action_name = contents.get(action_name_range.clone())?;
    let new_action_name = STRING_REPLACE.get(&action_name)?;
    Some((action_name_range, new_action_name.to_string()))
}

/// "ctrl-k ctrl-1": "inline_completion::ToggleMenu" -> "edit_prediction::ToggleMenu"
static STRING_REPLACE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        (
            "inline_completion::ToggleMenu",
            "edit_prediction::ToggleMenu",
        ),
        ("editor::NextInlineCompletion", "editor::NextEditPrediction"),
        (
            "editor::PreviousInlineCompletion",
            "editor::PreviousEditPrediction",
        ),
        (
            "editor::AcceptPartialInlineCompletion",
            "editor::AcceptPartialEditPrediction",
        ),
        ("editor::ShowInlineCompletion", "editor::ShowEditPrediction"),
        (
            "editor::AcceptInlineCompletion",
            "editor::AcceptEditPrediction",
        ),
        (
            "editor::ToggleInlineCompletions",
            "editor::ToggleEditPrediction",
        ),
        (
            "editor::GoToPrevDiagnostic",
            "editor::GoToPreviousDiagnostic",
        ),
        ("editor::ContextMenuPrev", "editor::ContextMenuPrevious"),
        ("search::SelectPrevMatch", "search::SelectPreviousMatch"),
        ("file_finder::SelectPrev", "file_finder::SelectPrevious"),
        ("menu::SelectPrev", "menu::SelectPrevious"),
        ("editor::TabPrev", "editor::Backtab"),
        ("pane::ActivatePrevItem", "pane::ActivatePreviousItem"),
        ("vim::MoveToPrev", "vim::MoveToPrevious"),
        ("vim::MoveToPrevMatch", "vim::MoveToPreviousMatch"),
    ])
});

const CONTEXT_PREDICATE_PATTERN: &str = r#"(document
    (array
        (object
            (pair
                key: (string (string_content) @name)
                value: (string (string_content) @context_predicate)
            )
        )
    )
    (#eq? @name "context")
)"#;

fn rename_context_key(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let context_predicate_ix = query.capture_index_for_name("context_predicate")?;
    let context_predicate_range = mat
        .nodes_for_capture_index(context_predicate_ix)
        .next()?
        .byte_range();
    let old_predicate = contents.get(context_predicate_range.clone())?.to_string();
    let mut new_predicate = old_predicate.to_string();
    for (old_key, new_key) in CONTEXT_REPLACE.iter() {
        new_predicate = new_predicate.replace(old_key, new_key);
    }
    if new_predicate != old_predicate {
        Some((context_predicate_range, new_predicate.to_string()))
    } else {
        None
    }
}

/// "context": "Editor && inline_completion && !showing_completions" -> "Editor && edit_prediction && !showing_completions"
pub static CONTEXT_REPLACE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("inline_completion", "edit_prediction"),
        (
            "inline_completion_requires_modifier",
            "edit_prediction_requires_modifier",
        ),
    ])
});

const ACTION_ARGUMENT_SNAKE_CASE_PATTERN: &str = r#"(document
    (array
        (object
            (pair
                key: (string (string_content) @name)
                value: (
                    (object
                        (pair
                            key: (string)
                            value: ((array
                                . (string (string_content) @action_name)
                                . (object
                                    (pair
                                    key: (string (string_content) @argument_key)
                                    value: (_)  @argument_value))
                                . ) @array
                            ))
                        )
                    )
                )
            )
        )
    (#eq? @name "bindings")
)"#;

fn to_snake_case(text: &str) -> String {
    text.to_case(Case::Snake)
}

fn action_argument_snake_case(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let array_ix = query.capture_index_for_name("array")?;
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let argument_key_ix = query.capture_index_for_name("argument_key")?;
    let argument_value_ix = query.capture_index_for_name("argument_value")?;
    let action_name = contents.get(
        mat.nodes_for_capture_index(action_name_ix)
            .next()?
            .byte_range(),
    )?;

    let replacement_key = ACTION_ARGUMENT_SNAKE_CASE_REPLACE.get(action_name)?;
    let argument_key = contents.get(
        mat.nodes_for_capture_index(argument_key_ix)
            .next()?
            .byte_range(),
    )?;

    if argument_key != *replacement_key {
        return None;
    }

    let argument_value_node = mat.nodes_for_capture_index(argument_value_ix).next()?;
    let argument_value = contents.get(argument_value_node.byte_range())?;

    let new_key = to_snake_case(argument_key);
    let new_value = if argument_value_node.kind() == "string" {
        format!("\"{}\"", to_snake_case(argument_value.trim_matches('"')))
    } else {
        argument_value.to_string()
    };

    let range_to_replace = mat.nodes_for_capture_index(array_ix).next()?.byte_range();
    let replacement = format!(
        "[\"{}\", {{ \"{}\": {} }}]",
        action_name, new_key, new_value
    );

    Some((range_to_replace, replacement))
}

pub static ACTION_ARGUMENT_SNAKE_CASE_REPLACE: LazyLock<HashMap<&str, &str>> =
    LazyLock::new(|| {
        HashMap::from_iter([
            ("vim::NextWordStart", "ignorePunctuation"),
            ("vim::NextWordEnd", "ignorePunctuation"),
            ("vim::PreviousWordStart", "ignorePunctuation"),
            ("vim::PreviousWordEnd", "ignorePunctuation"),
            ("vim::MoveToNext", "partialWord"),
            ("vim::MoveToPrev", "partialWord"),
            ("vim::Down", "displayLines"),
            ("vim::Up", "displayLines"),
            ("vim::EndOfLine", "displayLines"),
            ("vim::StartOfLine", "displayLines"),
            ("vim::FirstNonWhitespace", "displayLines"),
            ("pane::CloseActiveItem", "saveIntent"),
            ("vim::Paste", "preserveClipboard"),
            ("vim::Word", "ignorePunctuation"),
            ("vim::Subword", "ignorePunctuation"),
            ("vim::IndentObj", "includeBelow"),
        ])
    });

const SETTINGS_MIGRATION_PATTERNS: MigrationPatterns = &[
    (SETTINGS_STRING_REPLACE_QUERY, replace_setting_name),
    (
        SETTINGS_NESTED_KEY_VALUE_PATTERN,
        replace_edit_prediction_provider_setting,
    ),
    (
        SETTINGS_NESTED_KEY_VALUE_PATTERN,
        replace_tab_close_button_setting_key,
    ),
    (
        SETTINGS_NESTED_KEY_VALUE_PATTERN,
        replace_tab_close_button_setting_value,
    ),
    (
        SETTINGS_REPLACE_IN_LANGUAGES_QUERY,
        replace_setting_in_languages,
    ),
];

static SETTINGS_MIGRATION_QUERY: LazyLock<Query> = LazyLock::new(|| {
    Query::new(
        &tree_sitter_json::LANGUAGE.into(),
        &SETTINGS_MIGRATION_PATTERNS
            .iter()
            .map(|pattern| pattern.0)
            .collect::<String>(),
    )
    .unwrap()
});

static EDIT_PREDICTION_SETTINGS_MIGRATION_QUERY: LazyLock<Query> = LazyLock::new(|| {
    Query::new(
        &tree_sitter_json::LANGUAGE.into(),
        SETTINGS_NESTED_KEY_VALUE_PATTERN,
    )
    .unwrap()
});

const SETTINGS_STRING_REPLACE_QUERY: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @name)
            value: (_)
        )
    )
)"#;

fn replace_setting_name(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let setting_capture_ix = query.capture_index_for_name("name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_capture_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_name_range.clone())?;
    let new_setting_name = SETTINGS_STRING_REPLACE.get(&setting_name)?;
    Some((setting_name_range, new_setting_name.to_string()))
}

pub static SETTINGS_STRING_REPLACE: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from_iter([
            (
                "show_inline_completions_in_menu",
                "show_edit_predictions_in_menu",
            ),
            ("show_inline_completions", "show_edit_predictions"),
            (
                "inline_completions_disabled_in",
                "edit_predictions_disabled_in",
            ),
            ("inline_completions", "edit_predictions"),
        ])
    });

const SETTINGS_NESTED_KEY_VALUE_PATTERN: &str = r#"
(object
  (pair
    key: (string (string_content) @parent_key)
    value: (object
        (pair
            key: (string (string_content) @setting_name)
            value: (_) @setting_value
        )
    )
  )
)
"#;

fn replace_edit_prediction_provider_setting(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let parent_object_capture_ix = query.capture_index_for_name("parent_key")?;
    let parent_object_range = mat
        .nodes_for_capture_index(parent_object_capture_ix)
        .next()?
        .byte_range();
    let parent_object_name = contents.get(parent_object_range.clone())?;

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_range.clone())?;

    if parent_object_name == "features" && setting_name == "inline_completion_provider" {
        return Some((setting_range, "edit_prediction_provider".into()));
    }

    None
}

fn replace_tab_close_button_setting_key(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let parent_object_capture_ix = query.capture_index_for_name("parent_key")?;
    let parent_object_range = mat
        .nodes_for_capture_index(parent_object_capture_ix)
        .next()?
        .byte_range();
    let parent_object_name = contents.get(parent_object_range.clone())?;

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_range.clone())?;

    if parent_object_name == "tabs" && setting_name == "always_show_close_button" {
        return Some((setting_range, "show_close_button".into()));
    }

    None
}

fn replace_tab_close_button_setting_value(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let parent_object_capture_ix = query.capture_index_for_name("parent_key")?;
    let parent_object_range = mat
        .nodes_for_capture_index(parent_object_capture_ix)
        .next()?
        .byte_range();
    let parent_object_name = contents.get(parent_object_range.clone())?;

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_name_range.clone())?;

    let setting_value_ix = query.capture_index_for_name("setting_value")?;
    let setting_value_range = mat
        .nodes_for_capture_index(setting_value_ix)
        .next()?
        .byte_range();
    let setting_value = contents.get(setting_value_range.clone())?;

    if parent_object_name == "tabs" && setting_name == "always_show_close_button" {
        match setting_value {
            "true" => {
                return Some((setting_value_range, "\"always\"".to_string()));
            }
            "false" => {
                return Some((setting_value_range, "\"hover\"".to_string()));
            }
            _ => {}
        }
    }

    None
}

const SETTINGS_REPLACE_IN_LANGUAGES_QUERY: &str = r#"
(object
  (pair
    key: (string (string_content) @languages)
    value: (object
    (pair
        key: (string)
        value: (object
            (pair
                key: (string (string_content) @setting_name)
                value: (_) @value
            )
        )
    ))
  )
)
(#eq? @languages "languages")
"#;

fn replace_setting_in_languages(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let setting_capture_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_capture_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_name_range.clone())?;
    let new_setting_name = LANGUAGE_SETTINGS_REPLACE.get(&setting_name)?;

    Some((setting_name_range, new_setting_name.to_string()))
}

static LANGUAGE_SETTINGS_REPLACE: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from_iter([
            ("show_inline_completions", "show_edit_predictions"),
            (
                "inline_completions_disabled_in",
                "edit_predictions_disabled_in",
            ),
        ])
    });

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_migrate_keymap(input: &str, output: Option<&str>) {
        let migrated = migrate_keymap(&input).unwrap();
        pretty_assertions::assert_eq!(migrated.as_deref(), output);
    }

    fn assert_migrate_settings(input: &str, output: Option<&str>) {
        let migrated = migrate_settings(&input).unwrap();
        pretty_assertions::assert_eq!(migrated.as_deref(), output);
    }

    #[test]
    fn test_replace_array_with_single_string() {
        assert_migrate_keymap(
            r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["workspace::ActivatePaneInDirection", "Up"]
                    }
                }
            ]
            "#,
            Some(
                r#"
            [
                {
                    "bindings": {
                        "cmd-1": "workspace::ActivatePaneUp"
                    }
                }
            ]
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_action_argument_object_with_single_value() {
        assert_migrate_keymap(
            r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["editor::FoldAtLevel", { "level": 1 }]
                    }
                }
            ]
            "#,
            Some(
                r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["editor::FoldAtLevel", 1]
                    }
                }
            ]
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_action_argument_object_with_single_value_2() {
        assert_migrate_keymap(
            r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["vim::PushOperator", { "Object": { "some" : "value" } }]
                    }
                }
            ]
            "#,
            Some(
                r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["vim::PushObject", { "some" : "value" }]
                    }
                }
            ]
            "#,
            ),
        )
    }

    #[test]
    fn test_rename_string_action() {
        assert_migrate_keymap(
            r#"
                [
                    {
                        "bindings": {
                            "cmd-1": "inline_completion::ToggleMenu"
                        }
                    }
                ]
            "#,
            Some(
                r#"
                [
                    {
                        "bindings": {
                            "cmd-1": "edit_prediction::ToggleMenu"
                        }
                    }
                ]
            "#,
            ),
        )
    }

    #[test]
    fn test_rename_context_key() {
        assert_migrate_keymap(
            r#"
                [
                    {
                        "context": "Editor && inline_completion && !showing_completions"
                    }
                ]
            "#,
            Some(
                r#"
                [
                    {
                        "context": "Editor && edit_prediction && !showing_completions"
                    }
                ]
            "#,
            ),
        )
    }

    #[test]
    fn test_string_of_array_replace() {
        assert_migrate_keymap(
            r#"
                [
                    {
                        "bindings": {
                            "ctrl-p": ["editor::GoToPrevHunk", { "center_cursor": true }],
                            "ctrl-q": ["editor::GoToPrevHunk"],
                            "ctrl-q": "editor::GoToPrevHunk", // should remain same
                        }
                    }
                ]
            "#,
            Some(
                r#"
                [
                    {
                        "bindings": {
                            "ctrl-p": ["editor::GoToPreviousHunk", { "center_cursor": true }],
                            "ctrl-q": ["editor::GoToPreviousHunk"],
                            "ctrl-q": "editor::GoToPrevHunk", // should remain same
                        }
                    }
                ]
            "#,
            ),
        )
    }

    #[test]
    fn test_action_argument_snake_case() {
        // First performs transformations, then replacements
        assert_migrate_keymap(
            r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["vim::PushOperator", { "Object": { "around": false } }],
                        "cmd-3": ["pane::CloseActiveItem", { "saveIntent": "saveAll" }],
                        "cmd-2": ["vim::NextWordStart", { "ignorePunctuation": true }],
                        "cmd-4": ["task::Spawn", { "task_name": "a b" }] // should remain as it is
                    }
                }
            ]
            "#,
            Some(
                r#"
            [
                {
                    "bindings": {
                        "cmd-1": ["vim::PushObject", { "around": false }],
                        "cmd-3": ["pane::CloseActiveItem", { "save_intent": "save_all" }],
                        "cmd-2": ["vim::NextWordStart", { "ignore_punctuation": true }],
                        "cmd-4": ["task::Spawn", { "task_name": "a b" }] // should remain as it is
                    }
                }
            ]
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_setting_name() {
        assert_migrate_settings(
            r#"
                {
                    "show_inline_completions_in_menu": true,
                    "show_inline_completions": true,
                    "inline_completions_disabled_in": ["string"],
                    "inline_completions": { "some" : "value" }
                }
            "#,
            Some(
                r#"
                {
                    "show_edit_predictions_in_menu": true,
                    "show_edit_predictions": true,
                    "edit_predictions_disabled_in": ["string"],
                    "edit_predictions": { "some" : "value" }
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_nested_string_replace_for_settings() {
        assert_migrate_settings(
            r#"
                {
                    "features": {
                        "inline_completion_provider": "zed"
                    },
                }
            "#,
            Some(
                r#"
                {
                    "features": {
                        "edit_prediction_provider": "zed"
                    },
                }
            "#,
            ),
        )
    }

    #[test]
    fn test_replace_settings_in_languages() {
        assert_migrate_settings(
            r#"
                {
                    "languages": {
                        "Astro": {
                            "show_inline_completions": true
                        }
                    }
                }
            "#,
            Some(
                r#"
                {
                    "languages": {
                        "Astro": {
                            "show_edit_predictions": true
                        }
                    }
                }
            "#,
            ),
        )
    }
}
